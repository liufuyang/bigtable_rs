//! Heartbeat watchdog. SESSION_SPEC §7, §8.
//!
//! Deadline in wall-nanos, extended by every recognized wire frame in
//! either direction (Send, Recv, heartbeat, SessionRefreshConfig, error).
//! **Unknown frame types explicitly do NOT reset** — that guard is enforced
//! at the caller (`Session::handle_session_response`) not here.
//!
//! Watchdog is ARMED only while a vRPC is in-flight (§7); the loop
//! consults an `is_active` callback each tick before deciding whether to
//! evaluate the deadline. Idle sessions receive no heartbeats and must not
//! be torn down.
//!
//! Miss sequence (§8):
//!  1. record `DebugTag::HeartbeatMissed`
//!  2. deterministic log BEFORE force-close (so the marker isn't lost)
//!  3. invoke `on_miss` callback (Session wires this to ForceClose)
//!  4. loop returns; no respawn

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

use crate::transport::debug_tag::{DebugTag, DebugTagRing};

pub(crate) const DEFAULT_HEARTBEAT_INTERVAL: Duration = Duration::from_millis(100);

/// Callback returning whether the watchdog should evaluate the deadline
/// this tick. Wired to `Slot::is_occupied()` on the Session so idle
/// sessions skip the check entirely.
pub(crate) type ActivePredicate = Arc<dyn Fn() -> bool + Send + Sync>;

/// Callback fired on miss. Wired to `Session::force_close` at the call
/// site so heartbeat.rs has zero knowledge of Session internals.
pub(crate) type OnMissCallback = Arc<dyn Fn() + Send + Sync>;

#[derive(Debug)]
pub(crate) struct HeartbeatWatchdog {
    /// Interval in nanos; server-negotiable via SessionParametersResponse
    /// (SESSION_SPEC §7). Default 100ms.
    interval_nanos: AtomicU64,
    /// `wall_now_nanos + interval` at last frame — a single miss trips
    /// the watchdog. Bumped by `reset()` from any frame handler.
    next_deadline_nanos: AtomicU64,
    /// Nudges the loop to re-evaluate when interval/deadline changes
    /// while it's sleeping.
    wake: Notify,
}

impl HeartbeatWatchdog {
    /// Constructor: takes the initial deadline (wall-nanos). Session
    /// stamps `now + initial_grace` at construction so a stream that
    /// never receives frames trips within one interval.
    pub(crate) fn new(interval: Duration, initial_deadline_nanos: u64) -> Self {
        Self {
            interval_nanos: AtomicU64::new(interval.as_nanos() as u64),
            next_deadline_nanos: AtomicU64::new(initial_deadline_nanos),
            wake: Notify::new(),
        }
    }

    pub(crate) fn interval(&self) -> Duration {
        Duration::from_nanos(self.interval_nanos.load(Ordering::Acquire))
    }

    /// Extends the deadline to `now_nanos + interval`. Hot path — 1
    /// atomic store + 1 non-blocking wake (§7).
    pub(crate) fn reset(&self, now_nanos: u64) {
        let interval = self.interval_nanos.load(Ordering::Acquire);
        self.next_deadline_nanos
            .store(now_nanos.saturating_add(interval), Ordering::Release);
        // Wake nudges the loop's sleep short-circuit — bursts coalesce
        // to a single pending wake via Notify::notify_one semantics.
        self.wake.notify_one();
    }

    /// Server-driven interval change (SessionParametersResponse handler
    /// calls this). Also re-stamps the deadline so the timer sees the
    /// new cadence immediately.
    pub(crate) fn set_interval(&self, interval: Duration, now_nanos: u64) {
        let nanos = interval.as_nanos() as u64;
        self.interval_nanos.store(nanos, Ordering::Release);
        self.next_deadline_nanos
            .store(now_nanos.saturating_add(nanos), Ordering::Release);
        self.wake.notify_one();
    }
}

/// Runs the watchdog loop until `stop` cancels. Uses `tokio::time::sleep`
/// so tests using `tokio::time::pause()` + `advance()` can drive the loop
/// deterministically. Wall-nanos are consulted via `now_nanos()` — swap
/// this in tests for a fake clock.
pub(crate) async fn run_watchdog(
    watchdog: Arc<HeartbeatWatchdog>,
    stop: CancellationToken,
    active: ActivePredicate,
    on_miss: OnMissCallback,
    debug_tags: Arc<DebugTagRing>,
    now_nanos: Arc<dyn Fn() -> u64 + Send + Sync>,
) {
    loop {
        let interval = watchdog.interval();
        let remaining = {
            let now = now_nanos();
            let deadline = watchdog.next_deadline_nanos.load(Ordering::Acquire);
            deadline.saturating_sub(now)
        };
        let sleep_for = Duration::from_nanos(remaining).max(Duration::from_nanos(1));

        tokio::select! {
            _ = stop.cancelled() => return,
            _ = watchdog.wake.notified() => continue,
            _ = tokio::time::sleep(sleep_for) => {}
        }

        if stop.is_cancelled() {
            return;
        }
        if !active() {
            // §7: idle sessions receive no heartbeats. Re-check after one
            // interval so a freshly-started vRPC is picked up.
            tokio::select! {
                _ = stop.cancelled() => return,
                _ = watchdog.wake.notified() => {},
                _ = tokio::time::sleep(interval) => {}
            }
            continue;
        }

        let now = now_nanos();
        let deadline = watchdog.next_deadline_nanos.load(Ordering::Acquire);
        if now < deadline {
            // A frame extended the deadline while we slept; re-arm.
            continue;
        }

        // §8 ordered miss sequence.
        debug_tags.record(DebugTag::HeartbeatMissed);
        log::warn!(
            "bigtable session heartbeat MISSED — forcing close (interval={:?})",
            interval
        );
        on_miss();
        return;
    }
}

/// Wall-clock nanos from `std::time::SystemTime`. Production default;
/// tests inject a fake.
pub(crate) fn system_now_nanos() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn miss_fires_when_active_and_no_reset() {
        let clock = Arc::new(AtomicU64::new(1_000_000_000));
        let now_fn: Arc<dyn Fn() -> u64 + Send + Sync> = {
            let c = Arc::clone(&clock);
            Arc::new(move || c.load(Ordering::Acquire))
        };
        let interval = Duration::from_millis(100);
        let wd = Arc::new(HeartbeatWatchdog::new(
            interval,
            clock.load(Ordering::Acquire) + interval.as_nanos() as u64,
        ));
        let stop = CancellationToken::new();
        let active: ActivePredicate = Arc::new(|| true);
        let miss_count = Arc::new(AtomicUsize::new(0));
        let mc = Arc::clone(&miss_count);
        let on_miss: OnMissCallback = Arc::new(move || {
            mc.fetch_add(1, Ordering::AcqRel);
        });
        let tags = Arc::new(DebugTagRing::with_capacity(4));
        let tags_r = Arc::clone(&tags);

        let stop_c = stop.clone();
        let wd_c = Arc::clone(&wd);
        let handle = tokio::spawn(async move {
            run_watchdog(wd_c, stop_c, active, on_miss, tags_r, now_fn).await;
        });

        // Advance wall clock so the deadline is in the past, and advance
        // tokio's timer so `sleep()` completes.
        clock.fetch_add(500_000_000, Ordering::AcqRel);
        tokio::time::advance(Duration::from_millis(500)).await;

        // Give the loop a moment to observe the pass-deadline branch.
        tokio::task::yield_now().await;
        let _ = tokio::time::timeout(Duration::from_millis(50), handle).await;

        assert_eq!(
            miss_count.load(Ordering::Acquire),
            1,
            "miss should fire once"
        );
        assert!(tags
            .snapshot()
            .iter()
            .any(|(t, _)| *t == DebugTag::HeartbeatMissed));
        stop.cancel();
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn reset_prevents_miss() {
        let clock = Arc::new(AtomicU64::new(1_000_000_000));
        let now_fn: Arc<dyn Fn() -> u64 + Send + Sync> = {
            let c = Arc::clone(&clock);
            Arc::new(move || c.load(Ordering::Acquire))
        };
        let interval = Duration::from_millis(100);
        let wd = Arc::new(HeartbeatWatchdog::new(
            interval,
            clock.load(Ordering::Acquire) + interval.as_nanos() as u64,
        ));
        let stop = CancellationToken::new();
        let active: ActivePredicate = Arc::new(|| true);
        let miss_count = Arc::new(AtomicUsize::new(0));
        let mc = Arc::clone(&miss_count);
        let on_miss: OnMissCallback = Arc::new(move || {
            mc.fetch_add(1, Ordering::AcqRel);
        });
        let tags = Arc::new(DebugTagRing::with_capacity(4));

        let stop_c = stop.clone();
        let wd_c = Arc::clone(&wd);
        tokio::spawn(async move {
            run_watchdog(wd_c, stop_c, active, on_miss, tags, now_fn).await;
        });

        // Reset every 40ms of wall time for 200ms. The 100ms deadline
        // never elapses without a reset.
        for _ in 0..5 {
            tokio::time::advance(Duration::from_millis(40)).await;
            clock.fetch_add(40_000_000, Ordering::AcqRel);
            wd.reset(clock.load(Ordering::Acquire));
            tokio::task::yield_now().await;
        }

        stop.cancel();
        assert_eq!(
            miss_count.load(Ordering::Acquire),
            0,
            "reset must keep watchdog alive"
        );
    }

    #[tokio::test(flavor = "current_thread", start_paused = true)]
    async fn idle_session_never_trips() {
        let clock = Arc::new(AtomicU64::new(1_000_000_000));
        let now_fn: Arc<dyn Fn() -> u64 + Send + Sync> = {
            let c = Arc::clone(&clock);
            Arc::new(move || c.load(Ordering::Acquire))
        };
        let interval = Duration::from_millis(100);
        let wd = Arc::new(HeartbeatWatchdog::new(
            interval,
            clock.load(Ordering::Acquire),
        ));
        let stop = CancellationToken::new();
        // active=false — session is idle, no vRPC in flight.
        let active: ActivePredicate = Arc::new(|| false);
        let miss_count = Arc::new(AtomicUsize::new(0));
        let mc = Arc::clone(&miss_count);
        let on_miss: OnMissCallback = Arc::new(move || {
            mc.fetch_add(1, Ordering::AcqRel);
        });
        let tags = Arc::new(DebugTagRing::with_capacity(4));

        let stop_c = stop.clone();
        let wd_c = Arc::clone(&wd);
        tokio::spawn(async move {
            run_watchdog(wd_c, stop_c, active, on_miss, tags, now_fn).await;
        });

        // Advance a full second past the deadline.
        clock.fetch_add(1_000_000_000, Ordering::AcqRel);
        tokio::time::advance(Duration::from_secs(1)).await;
        tokio::task::yield_now().await;

        stop.cancel();
        assert_eq!(miss_count.load(Ordering::Acquire), 0);
    }

    #[test]
    fn set_interval_updates_deadline() {
        let wd = HeartbeatWatchdog::new(Duration::from_millis(100), 1_000);
        wd.set_interval(Duration::from_millis(500), 2_000);
        assert_eq!(wd.interval(), Duration::from_millis(500));
        assert_eq!(
            wd.next_deadline_nanos.load(Ordering::Acquire),
            2_000 + 500 * 1_000_000
        );
    }
}
