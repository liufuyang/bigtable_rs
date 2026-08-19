//! vRPC primitives — RpcId, the one-in-flight slot, in-flight bookkeeping.
//! SESSION_SPEC §2, §10.
//!
//! Design constraints from §10:
//! - Slot's lock is INNERMOST — held only across pointer/state assignments.
//!   No I/O, no chan ops, no other lock nesting. That's why the mutex is
//!   `parking_lot::Mutex` (never crosses `.await`).
//! - `deliver` is a cap-1 buffered channel per vRPC (`tokio::mpsc::channel(1)`);
//!   the buffer defends against a ctx-done-vs-response tick race in the
//!   awaiter, matching Go's `chan vrpcResult` with cap 1.
//!
//! §2 semantics:
//! - Multiplex limit is EXACTLY 1. A losing `claim()` returns `SlotBusy` so
//!   the caller sees `AttemptState::Uncommitted` and can retry on another
//!   session.
//! - `mark_cancelled()` KEEPS the slot occupied — Go's `markCancelled` sets
//!   a cancelled flag on the current in-flight RPC without releasing it.
//!   The eventual server response drains it as `VRPCCancelledDrained`.
//! - `release()` (called via `drain_slot` in session.rs) is the sole
//!   "slot became free" event; it fires `on_slot_drained` (except from
//!   the `cancel_active_rpcs` teardown path).

use std::sync::atomic::{AtomicU64, Ordering};

use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::session_proto::{ErrorResponse, VirtualRpcResponse};
use crate::transport::errors::SessionError;

/// Monotonic, per-Session vRPC identifier. Generated from an owning
/// `AtomicU64` on the Session; unique within a session lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct RpcId(pub(crate) i64);

impl RpcId {
    pub(crate) fn value(self) -> i64 {
        self.0
    }
}

/// Monotonic counter that hands out `RpcId`s. Wraps an `AtomicU64` so
/// callers can hold it as a field on Session (single-writer isn't
/// required; concurrent `next()`s are fine).
#[derive(Debug, Default)]
pub(crate) struct RpcIdGenerator(AtomicU64);

impl RpcIdGenerator {
    pub(crate) fn next(&self) -> RpcId {
        // `Relaxed` is fine — ordering across `next()` calls only matters
        // for uniqueness, which the atomic guarantees. No happens-before
        // constraint on payload publication (that runs under `slotMu`).
        let n = self.0.fetch_add(1, Ordering::Relaxed);
        // rpc_id is int64 on the wire; overflow after 2^63 monotonically
        // increasing IDs is not a real-world concern within a single
        // Session's lifetime.
        RpcId(n as i64)
    }
}

/// Result carried on the per-vRPC `deliver` channel. Exactly one variant
/// is set. PR #3's retry oracle reads `error()` to classify.
#[derive(Debug)]
pub(crate) enum VRpcResult {
    Response(VirtualRpcResponse),
    ServerError(ErrorResponse),
    Transport(SessionError),
}

/// In-flight vRPC bookkeeping. Published under `Slot`'s mutex.
pub(crate) struct InFlightRpc {
    pub(crate) rpc_id: RpcId,
    pub(crate) method: &'static str,
    /// Fires when the caller's context is cancelled. `mark_cancelled()`
    /// sets it; the awaiting task observes it via `.cancelled()`.
    pub(crate) cancel: CancellationToken,
    /// Cap-1 buffered: defends against caller-ctx-done vs response tick
    /// race (SESSION_SPEC §10). Only the drain path sends into this;
    /// the awaiter reads it.
    pub(crate) deliver: mpsc::Sender<VRpcResult>,
    /// True once `mark_cancelled` fired. The slot remains OCCUPIED until
    /// the server response arrives (or teardown runs); a bookkeeping-only
    /// flag consumed by drain-site logic to tag as `VRPCCancelledDrained`.
    pub(crate) cancelled: bool,
}

impl std::fmt::Debug for InFlightRpc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InFlightRpc")
            .field("rpc_id", &self.rpc_id)
            .field("method", &self.method)
            .field("cancelled", &self.cancelled)
            .finish()
    }
}

/// Outcome of `drain_by_response`, telling the caller whether the drain
/// corresponded to a cancelled request (→ tag `VRPCCancelledDrained`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DrainOutcome {
    /// Normal delivery — response matched the current in-flight RPC.
    Delivered,
    /// Delivered but the caller had cancelled earlier — bookkeeping only.
    CancelledDrained,
    /// The rpc_id didn't match anything in-flight; caller should DROP the
    /// response silently (SESSION_SPEC §2 last bullet).
    Unmatched,
}

/// The one-in-flight slot. `Mutex` is short — never held across `.await`;
/// callers move I/O outside the lock.
#[derive(Debug, Default)]
pub(crate) struct Slot {
    inner: Mutex<Option<InFlightRpc>>,
}

impl Slot {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Attempts to install `rpc` as the sole in-flight RPC. Returns
    /// `Err(SessionError::SlotBusy)` when a prior RPC has not drained —
    /// caller surfaces this as `AttemptState::Uncommitted`.
    pub(crate) fn claim(&self, rpc: InFlightRpc) -> Result<(), SessionError> {
        let mut g = self.inner.lock();
        if g.is_some() {
            return Err(SessionError::SlotBusy);
        }
        *g = Some(rpc);
        Ok(())
    }

    /// Releases the slot unconditionally. Used by teardown paths
    /// (`cancel_active_rpcs`, `handleClose`) — the caller decides whether
    /// to fire `on_slot_drained` (it MUST NOT on teardown per §2).
    pub(crate) fn take(&self) -> Option<InFlightRpc> {
        self.inner.lock().take()
    }

    /// Drains the slot iff the current in-flight RPC has `rpc_id`.
    /// Returns the drained `InFlightRpc` when matched. Unmatched IDs are
    /// dropped silently per §2 — caller sees `DrainOutcome::Unmatched`.
    pub(crate) fn drain_by_response(&self, rpc_id: RpcId) -> (Option<InFlightRpc>, DrainOutcome) {
        let mut g = self.inner.lock();
        match g.as_ref() {
            Some(rpc) if rpc.rpc_id == rpc_id => {
                let taken = g.take().expect("slot was Some in this branch");
                let outcome = if taken.cancelled {
                    DrainOutcome::CancelledDrained
                } else {
                    DrainOutcome::Delivered
                };
                (Some(taken), outcome)
            }
            Some(_) | None => (None, DrainOutcome::Unmatched),
        }
    }

    /// Marks the in-flight RPC (if any) as cancelled and fires its
    /// cancellation token. Does NOT release the slot — the eventual
    /// server response drains it (§2).
    pub(crate) fn mark_cancelled(&self) -> Option<RpcId> {
        let mut g = self.inner.lock();
        let rpc = g.as_mut()?;
        rpc.cancelled = true;
        rpc.cancel.cancel();
        Some(rpc.rpc_id)
    }

    /// Non-mutating snapshot of the current in-flight ID (for tests /
    /// debugview) — CAN return None even between mark_cancelled and
    /// eventual drain (there IS still an RPC in flight, but the caller
    /// is only inspecting the ID).
    pub(crate) fn current_rpc_id(&self) -> Option<RpcId> {
        self.inner.lock().as_ref().map(|r| r.rpc_id)
    }

    pub(crate) fn is_occupied(&self) -> bool {
        self.inner.lock().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    fn make_rpc(id: i64) -> (InFlightRpc, mpsc::Receiver<VRpcResult>) {
        let (tx, rx) = mpsc::channel(1);
        (
            InFlightRpc {
                rpc_id: RpcId(id),
                method: "test",
                cancel: CancellationToken::new(),
                deliver: tx,
                cancelled: false,
            },
            rx,
        )
    }

    #[test]
    fn rpc_id_generator_is_monotonic() {
        let g = RpcIdGenerator::default();
        assert_eq!(g.next(), RpcId(0));
        assert_eq!(g.next(), RpcId(1));
        assert_eq!(g.next(), RpcId(2));
    }

    #[test]
    fn first_claim_succeeds_second_fails() {
        let s = Slot::new();
        let (r1, _rx1) = make_rpc(1);
        let (r2, _rx2) = make_rpc(2);
        assert!(s.claim(r1).is_ok());
        assert!(matches!(s.claim(r2), Err(SessionError::SlotBusy)));
        assert!(s.is_occupied());
    }

    #[test]
    fn drain_by_matching_id_returns_delivered() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(9);
        s.claim(r).unwrap();
        let (drained, outcome) = s.drain_by_response(RpcId(9));
        assert!(drained.is_some());
        assert_eq!(outcome, DrainOutcome::Delivered);
        assert!(!s.is_occupied());
    }

    #[test]
    fn drain_by_wrong_id_returns_unmatched_and_keeps_slot() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(9);
        s.claim(r).unwrap();
        let (drained, outcome) = s.drain_by_response(RpcId(10));
        assert!(drained.is_none());
        assert_eq!(outcome, DrainOutcome::Unmatched);
        assert!(
            s.is_occupied(),
            "slot must remain occupied on unmatched drain"
        );
    }

    #[test]
    fn mark_cancelled_keeps_slot() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(3);
        s.claim(r).unwrap();
        assert_eq!(s.mark_cancelled(), Some(RpcId(3)));
        assert!(s.is_occupied(), "cancelled RPC still occupies the slot");
    }

    #[test]
    fn drain_after_cancel_reports_cancelled_drained() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(3);
        s.claim(r).unwrap();
        s.mark_cancelled();
        let (drained, outcome) = s.drain_by_response(RpcId(3));
        assert!(drained.is_some());
        assert_eq!(outcome, DrainOutcome::CancelledDrained);
    }

    #[test]
    fn take_releases_unconditionally() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(1);
        s.claim(r).unwrap();
        assert!(s.take().is_some());
        assert!(!s.is_occupied());
    }

    #[test]
    fn concurrent_claim_only_one_wins() {
        let s = Arc::new(Slot::new());
        let ok = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let mut handles = vec![];
        for i in 0..16 {
            let s = Arc::clone(&s);
            let ok = Arc::clone(&ok);
            handles.push(thread::spawn(move || {
                let (r, _rx) = make_rpc(i as i64);
                if s.claim(r).is_ok() {
                    ok.fetch_add(1, Ordering::AcqRel);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(ok.load(Ordering::Acquire), 1);
    }

    #[test]
    fn cancel_token_fires_on_mark_cancelled() {
        let s = Slot::new();
        let (r, _rx) = make_rpc(1);
        let tok = r.cancel.clone();
        s.claim(r).unwrap();
        assert!(!tok.is_cancelled());
        s.mark_cancelled();
        assert!(tok.is_cancelled());
    }
}
