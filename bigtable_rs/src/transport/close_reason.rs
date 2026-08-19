//! Close-reason attribution. SESSION_SPEC §5.
//!
//! `GoAway`/`MissedHeartbeat`/`Error` must beat a late `StreamEnd:*` — the
//! slot is CAS-once so the first non-`Unknown` reason wins. Late stampers
//! (which are almost always `StreamEnd:*` from the read loop's EOF path)
//! see the CAS fail and become no-ops.

use parking_lot::Mutex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CloseReason {
    Unknown,
    GoAway,
    MissedHeartbeat,
    User,
    Error(String),
    StreamEnd(String),
}

impl CloseReason {
    fn is_unknown(&self) -> bool {
        matches!(self, CloseReason::Unknown)
    }

    pub(crate) fn as_label(&self) -> &str {
        match self {
            CloseReason::Unknown => "Unknown",
            CloseReason::GoAway => "GoAway",
            CloseReason::MissedHeartbeat => "MissedHeartbeat",
            CloseReason::User => "User",
            CloseReason::Error(_) => "Error",
            CloseReason::StreamEnd(_) => "StreamEnd",
        }
    }
}

/// One-writer-wins slot. The reason is guarded by a short `parking_lot::Mutex`
/// rather than an `Atomic*` because `CloseReason::Error(String)` isn't atomic
/// and the contention is negligible (write happens once, reads a handful of
/// times over a session's lifetime).
#[derive(Debug)]
pub(crate) struct CloseReasonSlot {
    inner: Mutex<CloseReason>,
}

impl Default for CloseReasonSlot {
    fn default() -> Self {
        Self {
            inner: Mutex::new(CloseReason::Unknown),
        }
    }
}

impl CloseReasonSlot {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Store `reason` iff the current value is `Unknown`. Returns `true` if
    /// this call stamped the value. Late callers see `false`.
    pub(crate) fn set_once(&self, reason: CloseReason) -> bool {
        // set_once semantics: only the first non-Unknown wins. Passing
        // Unknown itself is a caller bug (nothing to stamp) — we ignore it.
        if reason.is_unknown() {
            return false;
        }
        let mut guard = self.inner.lock();
        if guard.is_unknown() {
            *guard = reason;
            true
        } else {
            false
        }
    }

    pub(crate) fn get(&self) -> CloseReason {
        self.inner.lock().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_write_wins() {
        let slot = CloseReasonSlot::new();
        assert!(slot.set_once(CloseReason::GoAway));
        assert_eq!(slot.get(), CloseReason::GoAway);
    }

    #[test]
    fn late_stream_end_loses_to_goaway() {
        let slot = CloseReasonSlot::new();
        assert!(slot.set_once(CloseReason::GoAway));
        assert!(!slot.set_once(CloseReason::StreamEnd("Unavailable".into())));
        assert_eq!(slot.get(), CloseReason::GoAway);
    }

    #[test]
    fn late_stream_end_loses_to_missed_heartbeat() {
        let slot = CloseReasonSlot::new();
        assert!(slot.set_once(CloseReason::MissedHeartbeat));
        assert!(!slot.set_once(CloseReason::StreamEnd("EOF".into())));
        assert_eq!(slot.get(), CloseReason::MissedHeartbeat);
    }

    #[test]
    fn late_stream_end_loses_to_error() {
        let slot = CloseReasonSlot::new();
        assert!(slot.set_once(CloseReason::Error("boom".into())));
        assert!(!slot.set_once(CloseReason::StreamEnd("Cancelled".into())));
        assert_eq!(slot.get(), CloseReason::Error("boom".into()));
    }

    #[test]
    fn stream_end_wins_when_nothing_stamped_first() {
        let slot = CloseReasonSlot::new();
        assert!(slot.set_once(CloseReason::StreamEnd("EOF".into())));
        assert_eq!(slot.get(), CloseReason::StreamEnd("EOF".into()));
    }

    #[test]
    fn unknown_is_never_stamped() {
        let slot = CloseReasonSlot::new();
        assert!(!slot.set_once(CloseReason::Unknown));
        assert_eq!(slot.get(), CloseReason::Unknown);
    }

    #[test]
    fn labels_are_stable() {
        assert_eq!(CloseReason::GoAway.as_label(), "GoAway");
        assert_eq!(CloseReason::MissedHeartbeat.as_label(), "MissedHeartbeat");
        assert_eq!(CloseReason::Error("x".into()).as_label(), "Error");
        assert_eq!(CloseReason::StreamEnd("x".into()).as_label(), "StreamEnd");
    }
}
