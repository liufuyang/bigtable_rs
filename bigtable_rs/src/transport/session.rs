//! Session type — the runtime holding-state for one Bigtable Session's
//! lifecycle. SESSION_SPEC §1–§8, §10.
//!
//! Wire I/O is abstracted behind `VRpcSender`; PR #5 will wire this to
//! the real tonic bidi stream. Everything here is spec-driven behavior
//! + concurrency discipline — the wire is intentionally out of scope.

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use arc_swap::ArcSwapOption;
use async_trait::async_trait;
use parking_lot::Mutex;
use tokio_util::sync::CancellationToken;

use crate::session_proto::{close_session_request, CloseSessionRequest, SessionRefreshConfig};
use crate::transport::close_reason::{CloseReason, CloseReasonSlot};
use crate::transport::debug_tag::{DebugTag, DebugTagRing};
use crate::transport::errors::SessionError;
use crate::transport::heartbeat::{
    run_watchdog, system_now_nanos, ActivePredicate, HeartbeatWatchdog, OnMissCallback,
    DEFAULT_HEARTBEAT_INTERVAL,
};
use crate::transport::hooks::SessionHooks;
use crate::transport::state::{is_state, not_state, AtomicSessionState, SessionState};
use crate::transport::vrpc::{DrainOutcome, RpcId, RpcIdGenerator, Slot};
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::PeerInfo as PbPeerInfo;

const DEFAULT_DEBUG_TAG_CAP: usize = 64;
const GOAWAY_CLOSE_TIMEOUT: Duration = Duration::from_secs(30);

/// Abstracted wire — PR #5 supplies a tonic-bidi-stream impl; tests use
/// `MockVRpcSender` (see `#[cfg(test)]` below).
///
/// `close_session` semantics: send the `CloseSessionRequest` frame; do
/// not wait for the server EOF (Session's `close()` handles that).
#[async_trait]
pub(crate) trait VRpcSender: Send + Sync {
    async fn close_session(&self, req: CloseSessionRequest) -> Result<(), SessionError>;
}

/// Session — all the state a single Bigtable Session's lifecycle owns.
pub(crate) struct Session {
    state: AtomicSessionState,
    /// State the session was in immediately before its final transition
    /// to Closed. Lets consumers distinguish client-initiated clean-close
    /// (prev == WaitServerClose) from a server-driven close.
    prev_state_at_close: AtomicI32,

    hooks: SessionHooks,
    slot: Slot,
    heartbeat: Arc<HeartbeatWatchdog>,
    close_reason: CloseReasonSlot,

    peer_info: ArcSwapOption<PbPeerInfo>,
    refresh_config: ArcSwapOption<SessionRefreshConfig>,
    rpc_ids: RpcIdGenerator,

    debug_tags: Arc<DebugTagRing>,
    log_name: String,

    /// Stops long-running background tasks (heartbeat, read loop when
    /// PR #5 lands). Cancelled on close/force_close.
    stop: CancellationToken,

    /// Optional wire binding. Populated by `attach_sender` once PR #5
    /// wires the stream. Absent in unit tests (close still works —
    /// falls back to internal-only lifecycle transitions).
    ///
    /// Stored behind a plain `Mutex` (not `ArcSwap`) because `ArcSwap`
    /// requires `Sized` and this is a `dyn` trait object. Contention is
    /// negligible (write happens once at attach, reads happen a handful
    /// of times per close).
    sender: Mutex<Option<Arc<dyn VRpcSender>>>,
}

impl std::fmt::Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Session")
            .field("log_name", &self.log_name)
            .field("state", &self.state.load())
            .field("close_reason", &self.close_reason.get())
            .field("has_peer_info", &self.peer_info.load().is_some())
            .finish()
    }
}

impl Session {
    /// Constructor. `hooks` is single-writer-at-construction per §10 —
    /// it MUST be finalized before any lifecycle method is called.
    pub(crate) fn new(log_name: impl Into<String>, hooks: SessionHooks) -> Arc<Self> {
        let now = system_now_nanos();
        let interval = DEFAULT_HEARTBEAT_INTERVAL;
        // Initial grace = one interval so a stream that never receives a
        // frame trips within `interval` after construction.
        let initial_deadline = now.saturating_add(interval.as_nanos() as u64);
        let heartbeat = Arc::new(HeartbeatWatchdog::new(interval, initial_deadline));
        Arc::new(Self {
            state: AtomicSessionState::new(SessionState::New),
            prev_state_at_close: AtomicI32::new(-1),
            hooks,
            slot: Slot::new(),
            heartbeat,
            close_reason: CloseReasonSlot::new(),
            peer_info: ArcSwapOption::from(None),
            refresh_config: ArcSwapOption::from(None),
            rpc_ids: RpcIdGenerator::default(),
            debug_tags: Arc::new(DebugTagRing::with_capacity(DEFAULT_DEBUG_TAG_CAP)),
            log_name: log_name.into(),
            stop: CancellationToken::new(),
            sender: Mutex::new(None),
        })
    }

    pub(crate) fn log_name(&self) -> &str {
        &self.log_name
    }

    pub(crate) fn state(&self) -> SessionState {
        self.state.load()
    }

    pub(crate) fn close_reason(&self) -> CloseReason {
        self.close_reason.get()
    }

    pub(crate) fn peer_info(&self) -> Option<Arc<PbPeerInfo>> {
        self.peer_info.load_full()
    }

    pub(crate) fn refresh_config(&self) -> Option<Arc<SessionRefreshConfig>> {
        self.refresh_config.load_full()
    }

    pub(crate) fn debug_tags(&self) -> Arc<DebugTagRing> {
        Arc::clone(&self.debug_tags)
    }

    pub(crate) fn heartbeat(&self) -> Arc<HeartbeatWatchdog> {
        Arc::clone(&self.heartbeat)
    }

    pub(crate) fn stop_token(&self) -> CancellationToken {
        self.stop.clone()
    }

    /// PR #5 binds a real wire; tests can inject a mock.
    pub(crate) fn attach_sender(&self, s: Arc<dyn VRpcSender>) {
        *self.sender.lock() = Some(s);
    }

    /// Assign a fresh monotonic `RpcId`.
    pub(crate) fn next_rpc_id(&self) -> RpcId {
        self.rpc_ids.next()
    }

    /// Records a debug tag (§8, §6). Non-blocking; on ring overflow the
    /// oldest tag is dropped and `debug_tags().dropped()` reflects it.
    pub(crate) fn record_tag(&self, tag: DebugTag) {
        self.debug_tags.record(tag);
    }

    /// New → Starting; fires `on_start` (§4). Returns false if the
    /// session was already past New (idempotent-friendly for callers
    /// that don't want to check state first).
    pub(crate) fn start(self: &Arc<Self>) -> bool {
        let (_, ok) = self
            .state
            .try_transition(SessionState::Starting, is_state([SessionState::New]));
        if !ok {
            return false;
        }
        // §4: fire OnStart BEFORE spawning background loops so hook
        // ordering is enforced by construction — even a lightning-fast
        // handshake can't sneak OnActive in before OnStart.
        self.hooks.fire_on_start();

        self.spawn_heartbeat_loop();
        true
    }

    /// Called after the server confirms OpenSession. Stores PeerInfo
    /// SYNCHRONOUSLY before transitioning to Ready and firing OnActive
    /// — SESSION_SPEC §3 requires observers see a Ready session with
    /// populated PeerInfo.
    pub(crate) fn mark_ready(self: &Arc<Self>, peer: Option<PbPeerInfo>) -> bool {
        if let Some(p) = peer {
            self.peer_info.store(Some(Arc::new(p)));
        }
        let (_, ok) = self
            .state
            .try_transition(SessionState::Ready, is_state([SessionState::Starting]));
        if !ok {
            self.record_tag(DebugTag::OpenWrongState);
            return false;
        }
        // §4: OnActive is fired only AFTER PeerInfo is stamped so every
        // observer sees the session at Ready with routing info populated.
        self.hooks.fire_on_active();
        true
    }

    /// Reset the heartbeat watchdog on any recognized wire frame.
    pub(crate) fn reset_heartbeat(&self) {
        self.heartbeat.reset(system_now_nanos());
    }

    /// Server-driven interval update (`SessionParametersResponse`
    /// handler in PR #5 calls this).
    pub(crate) fn set_heartbeat_interval(&self, interval: Duration) {
        self.heartbeat.set_interval(interval, system_now_nanos());
    }

    pub(crate) fn set_refresh_config(&self, cfg: SessionRefreshConfig) {
        self.refresh_config.store(Some(Arc::new(cfg)));
    }

    /// Drains the slot after a matching response arrives. Fires
    /// `on_slot_drained` per §2 (this is a request-path drain, NOT a
    /// teardown-path drain).
    pub(crate) fn drain_slot(&self, rpc_id: RpcId) -> DrainOutcome {
        let (rpc, outcome) = self.slot.drain_by_response(rpc_id);
        if let Some(_r) = rpc {
            self.hooks.fire_on_slot_drained();
            if outcome == DrainOutcome::CancelledDrained {
                self.record_tag(DebugTag::VRPCCancelledDrained);
            }
        }
        outcome
    }

    /// Marks the current in-flight RPC as cancelled (caller's ctx fired).
    /// Slot stays occupied — the eventual server response drains it as
    /// `VRPCCancelledDrained` (§2).
    pub(crate) fn mark_cancelled(&self) -> Option<RpcId> {
        self.slot.mark_cancelled()
    }

    /// Teardown path — releases the slot WITHOUT firing on_slot_drained
    /// (§2 explicit rule). Delivers `err` to any waiting task via the
    /// deliver channel.
    pub(crate) fn cancel_active_rpcs(&self, err: SessionError) {
        if let Some(rpc) = self.slot.take() {
            // Deliver via try_send — the awaiter is on the other end
            // with a cap-1 buffer, and if the awaiter is gone the send
            // silently drops which is exactly what we want.
            rpc.cancel.cancel();
            let _ = rpc
                .deliver
                .try_send(crate::transport::vrpc::VRpcResult::Transport(err));
            // Deliberately NOT firing on_slot_drained — §2.
        }
    }

    /// Graceful close (§5). Ready → Closing → drain → Send
    /// CloseSessionRequest → WaitServerClose. Idempotent.
    ///
    /// PR #2 does not wait for the server EOF (that's driven by the
    /// read loop landing in PR #5); this method sends the request and
    /// advances state.
    pub(crate) async fn close(self: &Arc<Self>, req: CloseSessionRequest) {
        let (_, transitioned) = self.state.try_transition(
            SessionState::Closing,
            is_state([
                SessionState::New,
                SessionState::Starting,
                SessionState::Ready,
            ]),
        );
        if transitioned {
            self.hooks.fire_on_closing();
        }
        if self.state.load() != SessionState::Closing {
            // Already past Closing (WaitServerClose / Closed) — nothing
            // to do (§5: idempotent).
            return;
        }
        self.close_reason.set_once(close_reason_from_req(&req));

        // No wire yet? PR #5 will call the sender. In its absence, jump
        // to WaitServerClose so the state progression completes. Clone
        // the Arc out of the lock so we drop the mutex before .await.
        let sender = self.sender.lock().as_ref().map(Arc::clone);
        if let Some(sender) = sender {
            if let Err(e) = sender.close_session(req).await {
                // Send failed — degenerates to ForceClose per Go.
                self.force_close_internal(Some(CloseSessionRequest {
                    reason: close_session_request::CloseSessionReason::Error as i32,
                    description: format!("send CloseSessionRequest failed: {e}"),
                }));
                return;
            }
        }
        self.state.try_transition(
            SessionState::WaitServerClose,
            is_state([SessionState::Closing]),
        );
    }

    /// Immediate close (§5, §8). Skips Closing, sends no
    /// CloseSessionRequest, cancels active RPCs. Idempotent.
    pub(crate) fn force_close(self: &Arc<Self>, req: Option<CloseSessionRequest>) {
        self.force_close_internal(req);
    }

    fn force_close_internal(self: &Arc<Self>, req: Option<CloseSessionRequest>) {
        let (prev, ok) = self
            .state
            .try_transition(SessionState::Closed, not_state([SessionState::Closed]));
        if !ok {
            return;
        }
        self.prev_state_at_close
            .store(prev as i32, Ordering::Release);
        if prev == SessionState::New {
            self.record_tag(DebugTag::ForceCloseNeverStarted);
        }
        // ForceClose skips Ready → Closing, so onClosing fires here as
        // the safety net — closingOnce dedupes with any earlier fire.
        self.hooks.fire_on_closing();
        let reason = req
            .as_ref()
            .map(close_reason_from_req)
            .unwrap_or(CloseReason::Unknown);
        self.close_reason.set_once(reason);

        let err = match &req {
            Some(r) => match close_session_reason(r) {
                close_session_request::CloseSessionReason::MissedHeartbeat => {
                    SessionError::HeartbeatMissed
                }
                close_session_request::CloseSessionReason::Goaway => SessionError::GoAway,
                _ => SessionError::Closed(self.close_reason.get()),
            },
            None => SessionError::Closed(self.close_reason.get()),
        };
        self.cancel_active_rpcs(err);
        self.stop.cancel();
        self.hooks.fire_on_close();
    }

    /// Handles a server-initiated GOAWAY. SESSION_SPEC §6 — 7 steps in
    /// exact order. See `goaway.rs` for the driver (kept there so
    /// session.rs doesn't need to know the 30s-close-timeout details).
    pub(crate) fn is_active(&self) -> bool {
        self.slot.is_occupied()
    }

    /// Starts the heartbeat watchdog task. Uses the runtime's current
    /// dispatcher; the task ends when `self.stop` is cancelled OR the
    /// watchdog fires miss.
    fn spawn_heartbeat_loop(self: &Arc<Self>) {
        let watchdog = Arc::clone(&self.heartbeat);
        let stop = self.stop.clone();
        let debug_tags = Arc::clone(&self.debug_tags);
        let s_active = Arc::downgrade(self);
        let active: ActivePredicate =
            Arc::new(move || s_active.upgrade().map(|s| s.is_active()).unwrap_or(false));
        let s_miss = Arc::downgrade(self);
        let on_miss: OnMissCallback = Arc::new(move || {
            if let Some(s) = s_miss.upgrade() {
                s.force_close(Some(CloseSessionRequest {
                    reason: close_session_request::CloseSessionReason::MissedHeartbeat as i32,
                    description: "client terminated session due to missed server heartbeats"
                        .to_string(),
                }));
            }
        });
        let now_fn: Arc<dyn Fn() -> u64 + Send + Sync> = Arc::new(system_now_nanos);
        tokio::spawn(async move {
            run_watchdog(watchdog, stop, active, on_miss, debug_tags, now_fn).await;
        });
    }

    // === access for goaway.rs / vrpc handlers ===

    pub(crate) fn hooks(&self) -> &SessionHooks {
        &self.hooks
    }

    pub(crate) fn state_atomic(&self) -> &AtomicSessionState {
        &self.state
    }

    pub(crate) fn close_reason_slot(&self) -> &CloseReasonSlot {
        &self.close_reason
    }

    pub(crate) fn slot(&self) -> &Slot {
        &self.slot
    }
}

fn close_reason_from_req(req: &CloseSessionRequest) -> CloseReason {
    match close_session_reason(req) {
        close_session_request::CloseSessionReason::Goaway => CloseReason::GoAway,
        close_session_request::CloseSessionReason::MissedHeartbeat => CloseReason::MissedHeartbeat,
        close_session_request::CloseSessionReason::Error => {
            CloseReason::Error(req.description.clone())
        }
        close_session_request::CloseSessionReason::User => CloseReason::User,
        // Downsize + Unset land here; treat as User (client-initiated,
        // benign) so the debug view doesn't bucket them under "Unknown".
        _ => CloseReason::User,
    }
}

fn close_session_reason(req: &CloseSessionRequest) -> close_session_request::CloseSessionReason {
    close_session_request::CloseSessionReason::try_from(req.reason)
        .unwrap_or(close_session_request::CloseSessionReason::Unset)
}

/// Timeout for GOAWAY-driven graceful close, matching Go's 30s bounded ctx
/// in `handleGoAway`. Exposed for goaway.rs.
pub(crate) fn goaway_close_timeout() -> Duration {
    GOAWAY_CLOSE_TIMEOUT
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::transport::hooks::SessionHooksBuilder;
    use crate::transport::vrpc::{InFlightRpc, RpcId, VRpcResult};
    use parking_lot::Mutex;
    use std::sync::atomic::AtomicUsize;
    use tokio::sync::mpsc;

    pub(crate) struct MockVRpcSender {
        pub(crate) close_calls: Mutex<Vec<CloseSessionRequest>>,
        pub(crate) fail: bool,
    }

    impl MockVRpcSender {
        pub(crate) fn new() -> Self {
            Self {
                close_calls: Mutex::new(vec![]),
                fail: false,
            }
        }
    }

    #[async_trait]
    impl VRpcSender for MockVRpcSender {
        async fn close_session(&self, req: CloseSessionRequest) -> Result<(), SessionError> {
            self.close_calls.lock().push(req);
            if self.fail {
                Err(SessionError::Wire("mock fail".into()))
            } else {
                Ok(())
            }
        }
    }

    fn seq() -> (Arc<Mutex<Vec<&'static str>>>, SessionHooks) {
        let order = Arc::new(Mutex::new(Vec::<&'static str>::new()));
        let (o1, o2, o3, o4, o5) = (
            Arc::clone(&order),
            Arc::clone(&order),
            Arc::clone(&order),
            Arc::clone(&order),
            Arc::clone(&order),
        );
        let hooks = SessionHooksBuilder::default()
            .on_start(move || o1.lock().push("start"))
            .on_active(move || o2.lock().push("active"))
            .on_closing(move || o3.lock().push("closing"))
            .on_close(move || o4.lock().push("close"))
            .on_slot_drained(move || o5.lock().push("drained"))
            .build();
        (order, hooks)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn happy_path_start_ready_close_orders_hooks() {
        let (order, hooks) = seq();
        let s = Session::new("test-session", hooks);
        assert!(s.start());
        assert_eq!(s.state(), SessionState::Starting);
        assert!(s.mark_ready(None));
        assert_eq!(s.state(), SessionState::Ready);
        s.close(CloseSessionRequest {
            reason: close_session_request::CloseSessionReason::User as i32,
            description: "test".into(),
        })
        .await;
        assert_eq!(s.state(), SessionState::WaitServerClose);
        let o = order.lock().clone();
        assert_eq!(o, vec!["start", "active", "closing"]);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn force_close_from_new_fires_closing_then_close() {
        let (order, hooks) = seq();
        let s = Session::new("test", hooks);
        s.force_close(None);
        let o = order.lock().clone();
        // start/active did NOT run (skipped states), but the ordering
        // among fired hooks is preserved: closing precedes close.
        assert_eq!(o, vec!["closing", "close"]);
        assert_eq!(s.state(), SessionState::Closed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn peer_info_populated_before_on_active() {
        // §3: OnActive observer must see PeerInfo populated.
        let seen = Arc::new(AtomicUsize::new(0));
        let seen_c = Arc::clone(&seen);
        let s: Arc<Session> = Arc::new_cyclic(|weak_s: &std::sync::Weak<Session>| {
            let weak_s = weak_s.clone();
            let now = system_now_nanos();
            let interval = DEFAULT_HEARTBEAT_INTERVAL;
            let heartbeat = Arc::new(HeartbeatWatchdog::new(
                interval,
                now.saturating_add(interval.as_nanos() as u64),
            ));
            let hooks = SessionHooksBuilder::default()
                .on_active(move || {
                    if let Some(s) = weak_s.upgrade() {
                        if s.peer_info().is_some() {
                            seen_c.fetch_add(1, Ordering::AcqRel);
                        }
                    }
                })
                .build();
            Session {
                state: AtomicSessionState::new(SessionState::New),
                prev_state_at_close: AtomicI32::new(-1),
                hooks,
                slot: Slot::new(),
                heartbeat,
                close_reason: CloseReasonSlot::new(),
                peer_info: ArcSwapOption::from(None),
                refresh_config: ArcSwapOption::from(None),
                rpc_ids: RpcIdGenerator::default(),
                debug_tags: Arc::new(DebugTagRing::with_capacity(DEFAULT_DEBUG_TAG_CAP)),
                log_name: "peer-check".into(),
                stop: CancellationToken::new(),
                sender: Mutex::new(None),
            }
        });
        assert!(s.start());
        let peer = PbPeerInfo {
            application_frontend_id: 42,
            application_frontend_subzone: "sz".into(),
            ..PbPeerInfo::default()
        };
        assert!(s.mark_ready(Some(peer)));
        assert_eq!(
            seen.load(Ordering::Acquire),
            1,
            "on_active must have observed PeerInfo"
        );
    }

    #[tokio::test(flavor = "current_thread")]
    async fn close_is_idempotent() {
        let s = Session::new("t", SessionHooks::default());
        assert!(s.start());
        assert!(s.mark_ready(None));
        s.close(CloseSessionRequest::default()).await;
        // Second call must not panic or backtrack state.
        s.close(CloseSessionRequest::default()).await;
        let st = s.state();
        assert!(matches!(
            st,
            SessionState::WaitServerClose | SessionState::Closed
        ));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn force_close_is_idempotent() {
        let s = Session::new("t", SessionHooks::default());
        s.force_close(None);
        s.force_close(None);
        assert_eq!(s.state(), SessionState::Closed);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn cancel_active_rpcs_does_not_fire_on_slot_drained() {
        // §2 per-hook table: teardown MUST NOT fire on_slot_drained.
        let drained_count = Arc::new(AtomicUsize::new(0));
        let dc = Arc::clone(&drained_count);
        let hooks = SessionHooksBuilder::default()
            .on_slot_drained(move || {
                dc.fetch_add(1, Ordering::AcqRel);
            })
            .build();
        let s = Session::new("t", hooks);
        // Claim a slot.
        let (tx, mut rx) = mpsc::channel(1);
        s.slot()
            .claim(InFlightRpc {
                rpc_id: RpcId(1),
                method: "test",
                cancel: CancellationToken::new(),
                deliver: tx,
                cancelled: false,
            })
            .unwrap();
        s.cancel_active_rpcs(SessionError::HeartbeatMissed);
        assert_eq!(
            drained_count.load(Ordering::Acquire),
            0,
            "cancel_active_rpcs is a teardown path; must NOT fire on_slot_drained"
        );
        // Awaiter observes the transport error.
        assert!(matches!(rx.recv().await, Some(VRpcResult::Transport(_))));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn request_path_drain_fires_on_slot_drained() {
        // §2 per-hook table: request-path drain MUST fire on_slot_drained.
        let drained_count = Arc::new(AtomicUsize::new(0));
        let dc = Arc::clone(&drained_count);
        let hooks = SessionHooksBuilder::default()
            .on_slot_drained(move || {
                dc.fetch_add(1, Ordering::AcqRel);
            })
            .build();
        let s = Session::new("t", hooks);
        let (tx, _rx) = mpsc::channel(1);
        s.slot()
            .claim(InFlightRpc {
                rpc_id: RpcId(1),
                method: "test",
                cancel: CancellationToken::new(),
                deliver: tx,
                cancelled: false,
            })
            .unwrap();
        let outcome = s.drain_slot(RpcId(1));
        assert_eq!(outcome, DrainOutcome::Delivered);
        assert_eq!(drained_count.load(Ordering::Acquire), 1);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn drain_with_wrong_rpc_id_is_silent() {
        // §2 last bullet: unmatched rpc_id is DROPPED.
        let drained_count = Arc::new(AtomicUsize::new(0));
        let dc = Arc::clone(&drained_count);
        let hooks = SessionHooksBuilder::default()
            .on_slot_drained(move || {
                dc.fetch_add(1, Ordering::AcqRel);
            })
            .build();
        let s = Session::new("t", hooks);
        let (tx, _rx) = mpsc::channel(1);
        s.slot()
            .claim(InFlightRpc {
                rpc_id: RpcId(1),
                method: "t",
                cancel: CancellationToken::new(),
                deliver: tx,
                cancelled: false,
            })
            .unwrap();
        let outcome = s.drain_slot(RpcId(999));
        assert_eq!(outcome, DrainOutcome::Unmatched);
        assert!(s.slot().is_occupied(), "unmatched drain must keep slot");
        assert_eq!(drained_count.load(Ordering::Acquire), 0);
    }

    #[tokio::test(flavor = "current_thread")]
    async fn close_wire_send_failure_degrades_to_force_close() {
        let s = Session::new("t", SessionHooks::default());
        s.attach_sender(Arc::new(MockVRpcSender {
            close_calls: Mutex::new(vec![]),
            fail: true,
        }));
        assert!(s.start());
        assert!(s.mark_ready(None));
        s.close(CloseSessionRequest::default()).await;
        assert_eq!(s.state(), SessionState::Closed);
    }

    #[test]
    fn close_reason_from_req_maps_heartbeat() {
        let req = CloseSessionRequest {
            reason: close_session_request::CloseSessionReason::MissedHeartbeat as i32,
            description: "".into(),
        };
        assert_eq!(close_reason_from_req(&req), CloseReason::MissedHeartbeat);
    }

    #[test]
    fn close_reason_from_req_maps_goaway() {
        let req = CloseSessionRequest {
            reason: close_session_request::CloseSessionReason::Goaway as i32,
            description: "".into(),
        };
        assert_eq!(close_reason_from_req(&req), CloseReason::GoAway);
    }
}
