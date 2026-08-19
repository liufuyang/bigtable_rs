//! Session-level errors + AttemptState mapping. Consumed by PR #3's retry
//! oracle. SESSION_SPEC §9.
//!
//! `AttemptState` lives here (not `vrpc.rs`) so it can be re-exported from
//! `transport::` without pulling the vRPC slot module along; PR #3 will add
//! `RetryingVRpc` which reads it through `SessionError::attempt_state()`.

use std::fmt;
use thiserror::Error;

use crate::transport::close_reason::CloseReason;

/// Three-value retry classification per SESSION_SPEC §9. Client never
/// invents retryability from raw gRPC codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AttemptState {
    /// Never left the client (encode fail, session Closing, pool rejected,
    /// ctx dead before Send). Retry unconditionally.
    Uncommitted,
    /// Handed to transport, no server response observed (Send err, Recv
    /// err, ctx cancel mid-flight, heartbeat miss). Retry only if
    /// idempotent.
    TransportFailure,
    /// Server returned a definitive result (ErrorResponse or decode err
    /// on delivered response). Retry only if `RetryInfo` present, or
    /// code in narrow always-retryable set.
    ServerResult,
}

impl fmt::Display for AttemptState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            AttemptState::Uncommitted => "Uncommitted",
            AttemptState::TransportFailure => "TransportFailure",
            AttemptState::ServerResult => "ServerResult",
        })
    }
}

#[derive(Debug, Error)]
pub(crate) enum SessionError {
    #[error("bigtable: session not active")]
    NotActive,
    #[error("bigtable: session slot busy — prior vRPC not drained")]
    SlotBusy,
    #[error("bigtable: session unavailable: server heartbeat missed")]
    HeartbeatMissed,
    #[error("bigtable: session closed ({0:?})")]
    Closed(CloseReason),
    #[error("bigtable: session unavailable: server sent GOAWAY")]
    GoAway,
    #[error("bigtable: session wire error: {0}")]
    Wire(String),
    #[error("bigtable: session encode error: {0}")]
    Encode(String),
    #[error("bigtable: server returned error: {0}")]
    Server(String),
}

impl SessionError {
    /// Retry classification consumed by PR #3's retry oracle.
    pub(crate) fn attempt_state(&self) -> AttemptState {
        match self {
            // NotActive / SlotBusy / Encode fire before the request hit the
            // wire — always safe to retry.
            SessionError::NotActive => AttemptState::Uncommitted,
            SessionError::SlotBusy => AttemptState::Uncommitted,
            SessionError::Encode(_) => AttemptState::Uncommitted,
            // HeartbeatMissed / Wire / GoAway / Closed handed to transport,
            // no definitive server response. Retry iff idempotent (§9).
            SessionError::HeartbeatMissed => AttemptState::TransportFailure,
            SessionError::Wire(_) => AttemptState::TransportFailure,
            SessionError::GoAway => AttemptState::TransportFailure,
            SessionError::Closed(_) => AttemptState::TransportFailure,
            // Server frame delivered a definitive verdict.
            SessionError::Server(_) => AttemptState::ServerResult,
        }
    }
}

impl From<tonic::Status> for SessionError {
    fn from(s: tonic::Status) -> Self {
        SessionError::Wire(format!("{}: {}", s.code(), s.message()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classification_matches_spec_table() {
        assert_eq!(
            SessionError::NotActive.attempt_state(),
            AttemptState::Uncommitted
        );
        assert_eq!(
            SessionError::SlotBusy.attempt_state(),
            AttemptState::Uncommitted
        );
        assert_eq!(
            SessionError::Encode("bad".into()).attempt_state(),
            AttemptState::Uncommitted
        );
        assert_eq!(
            SessionError::HeartbeatMissed.attempt_state(),
            AttemptState::TransportFailure
        );
        assert_eq!(
            SessionError::Wire("boom".into()).attempt_state(),
            AttemptState::TransportFailure
        );
        assert_eq!(
            SessionError::GoAway.attempt_state(),
            AttemptState::TransportFailure
        );
        assert_eq!(
            SessionError::Closed(CloseReason::MissedHeartbeat).attempt_state(),
            AttemptState::TransportFailure
        );
        assert_eq!(
            SessionError::Server("verdict".into()).attempt_state(),
            AttemptState::ServerResult
        );
    }

    #[test]
    fn from_tonic_status_is_wire() {
        let e: SessionError = tonic::Status::unavailable("something").into();
        assert!(matches!(e, SessionError::Wire(_)));
        assert_eq!(e.attempt_state(), AttemptState::TransportFailure);
    }
}
