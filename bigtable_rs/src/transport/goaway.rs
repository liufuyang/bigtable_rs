//! GOAWAY handler. SESSION_SPEC §6 — 7 ordered steps.
//!
//! `LastRpcIdAdmitted` is intentionally NOT consumed. SESSION_SPEC §6:
//! "DEPRECATED (not implemented in the Go client)"; any reintroduction
//! MUST land as paired code + spec + test with Java-parity verification.

use std::sync::Arc;
use std::time::Duration;

use crate::session_proto::{close_session_request, CloseSessionRequest, GoAwayResponse};
use crate::transport::close_reason::CloseReason;
use crate::transport::debug_tag::DebugTag;
use crate::transport::session::{goaway_close_timeout, Session};
use crate::transport::state::{not_state, SessionState};

/// Executes SESSION_SPEC §6's 7-step sequence in exact order. Called
/// from the read loop (PR #5) when a `SessionResponse::GoAway` frame
/// arrives.
pub(crate) fn handle_goaway(session: &Arc<Session>, resp: GoAwayResponse) {
    // Step 1: precondition. GOAWAY on a still-NEW session is a
    // protocol oddity — record and drop.
    let pre_state = session.state();
    if (pre_state as u8) < (SessionState::Starting as u8) {
        session.record_tag(DebugTag::GoawayBeforeStart);
        log::debug!(
            "bigtable: GOAWAY arrived while session was in {} (want >= Starting)",
            pre_state.as_str()
        );
        return;
    }

    // Step 2: Ready → Closing. Terminal states are tagged and ignored.
    let (_, transitioned) = session.state_atomic().try_transition(
        SessionState::Closing,
        not_state([
            SessionState::Closing,
            SessionState::WaitServerClose,
            SessionState::Closed,
        ]),
    );
    if !transitioned {
        session.record_tag(DebugTag::GoawayAfterClose);
        return;
    }

    // Step 3: fire OnClosing immediately — pool pulls the session out
    // of routing structures now, up to 30s earlier than the actual
    // stream close.
    session.hooks().fire_on_closing();

    // Step 4: "GoAway" wins the close-reason CAS. Beats any later
    // StreamEnd:* the read loop would otherwise stamp.
    session.close_reason_slot().set_once(CloseReason::GoAway);

    // Step 5: deterministic log line with reason + description.
    log::info!(
        "bigtable: received GOAWAY reason={:?} description={:?}",
        resp.reason,
        resp.description
    );

    // Step 6: in-flight vRPC is NOT cancelled. Java parity — if the
    // server sends the vRPC response before dropping the stream, the
    // RPC completes successfully. Only when the stream terminates does
    // handleClose → cancelActiveRPCs fail it. This grace period is what
    // makes GOAWAY safe for non-idempotent Apply.

    // Step 7: off-loop `close()` driver under a 30s bounded timeout.
    let s = Arc::clone(session);
    tokio::spawn(async move {
        let req = CloseSessionRequest {
            reason: close_session_request::CloseSessionReason::Goaway as i32,
            description: "session closing after server GOAWAY".into(),
        };
        let close_fut = s.close(req);
        if tokio::time::timeout(goaway_close_timeout(), close_fut)
            .await
            .is_err()
        {
            // 30s exceeded: force-close matches Go's fallback semantics.
            s.force_close(Some(CloseSessionRequest {
                reason: close_session_request::CloseSessionReason::Goaway as i32,
                description: "GOAWAY close timed out; force-close".into(),
            }));
        }
    });
    let _ = Duration::from_secs(0); // Silence unused import in case of future rewrites.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::hooks::SessionHooksBuilder;
    use parking_lot::Mutex;

    fn seq_hooks() -> (
        Arc<Mutex<Vec<&'static str>>>,
        crate::transport::hooks::SessionHooks,
    ) {
        let order = Arc::new(Mutex::new(vec![]));
        let (o1, o2, o3, o4) = (
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
            .build();
        (order, hooks)
    }

    #[tokio::test(flavor = "current_thread")]
    async fn goaway_before_start_is_tagged_and_dropped() {
        let (order, hooks) = seq_hooks();
        let s = Session::new("t", hooks);
        // Still in NEW.
        handle_goaway(
            &s,
            GoAwayResponse {
                reason: "server-drain".into(),
                description: "test".into(),
                last_rpc_id_admitted: 0,
            },
        );
        assert_eq!(s.state(), SessionState::New);
        assert_eq!(order.lock().clone(), Vec::<&'static str>::new());
        assert!(s
            .debug_tags()
            .snapshot()
            .iter()
            .any(|(t, _)| *t == DebugTag::GoawayBeforeStart));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn goaway_after_close_is_tagged_and_dropped() {
        let (_, hooks) = seq_hooks();
        let s = Session::new("t", hooks);
        assert!(s.start());
        assert!(s.mark_ready(None));
        s.force_close(None);
        handle_goaway(
            &s,
            GoAwayResponse {
                reason: "late".into(),
                ..GoAwayResponse::default()
            },
        );
        assert!(s
            .debug_tags()
            .snapshot()
            .iter()
            .any(|(t, _)| *t == DebugTag::GoawayAfterClose));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn goaway_transitions_ready_to_closing_and_stamps_reason() {
        let (order, hooks) = seq_hooks();
        let s = Session::new("t", hooks);
        assert!(s.start());
        assert!(s.mark_ready(None));
        assert_eq!(s.state(), SessionState::Ready);
        handle_goaway(
            &s,
            GoAwayResponse {
                reason: "drain".into(),
                description: "server draining".into(),
                last_rpc_id_admitted: 0,
            },
        );
        // Immediately after handle_goaway, state is Closing and
        // OnClosing has fired (steps 2 + 3).
        assert_eq!(s.state(), SessionState::Closing);
        assert_eq!(s.close_reason(), CloseReason::GoAway);
        let o = order.lock().clone();
        assert!(o.contains(&"closing"));
    }

    #[tokio::test(flavor = "current_thread")]
    async fn goaway_reason_beats_late_stream_end() {
        let (_, hooks) = seq_hooks();
        let s = Session::new("t", hooks);
        assert!(s.start());
        assert!(s.mark_ready(None));
        handle_goaway(&s, GoAwayResponse::default());
        // Simulate late StreamEnd stamp (would come from read loop EOF).
        assert!(!s
            .close_reason_slot()
            .set_once(CloseReason::StreamEnd("EOF".into())));
        assert_eq!(s.close_reason(), CloseReason::GoAway);
    }
}
