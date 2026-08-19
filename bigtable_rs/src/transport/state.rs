//! Session state machine. SESSION_SPEC §1.
//!
//! Six states, strictly forward. `try_transition` is CAS-driven so multiple
//! threads racing on the same transition are safe — only one wins, the losers
//! see the winner's value on the next `load()`.

use std::sync::atomic::{AtomicU8, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub(crate) enum SessionState {
    New = 0,
    Starting = 1,
    Ready = 2,
    Closing = 3,
    WaitServerClose = 4,
    Closed = 5,
}

impl SessionState {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            SessionState::New => "New",
            SessionState::Starting => "Starting",
            SessionState::Ready => "Ready",
            SessionState::Closing => "Closing",
            SessionState::WaitServerClose => "WaitServerClose",
            SessionState::Closed => "Closed",
        }
    }

    fn from_u8(v: u8) -> SessionState {
        match v {
            0 => SessionState::New,
            1 => SessionState::Starting,
            2 => SessionState::Ready,
            3 => SessionState::Closing,
            4 => SessionState::WaitServerClose,
            _ => SessionState::Closed,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AtomicSessionState(AtomicU8);

impl AtomicSessionState {
    pub(crate) fn new(initial: SessionState) -> Self {
        Self(AtomicU8::new(initial as u8))
    }

    pub(crate) fn load(&self) -> SessionState {
        SessionState::from_u8(self.0.load(Ordering::Acquire))
    }

    /// Attempts to move to `to` iff the predicate `ok(current)` holds.
    /// Returns `(prev_state, applied)`. Retries on CAS loss with a still-
    /// valid current state — matches Go `transitionTo` semantics.
    pub(crate) fn try_transition(
        &self,
        to: SessionState,
        ok: impl Fn(SessionState) -> bool,
    ) -> (SessionState, bool) {
        loop {
            let prev = self.load();
            if !ok(prev) {
                return (prev, false);
            }
            // AcqRel: transitions publish state changes that other observers
            // read via Acquire; we also want the failing branch to see the
            // freshest value so the retry loop terminates.
            if self
                .0
                .compare_exchange(prev as u8, to as u8, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return (prev, true);
            }
        }
    }

    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self.load(), SessionState::Closed)
    }
}

/// Predicate: current state matches any of `allowed`.
pub(crate) fn is_state<const N: usize>(
    allowed: [SessionState; N],
) -> impl Fn(SessionState) -> bool {
    move |s| allowed.contains(&s)
}

/// Predicate: current state matches none of `forbidden`.
pub(crate) fn not_state<const N: usize>(
    forbidden: [SessionState; N],
) -> impl Fn(SessionState) -> bool {
    move |s| !forbidden.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordering_is_monotonic() {
        assert!(SessionState::New < SessionState::Starting);
        assert!(SessionState::Starting < SessionState::Ready);
        assert!(SessionState::Ready < SessionState::Closing);
        assert!(SessionState::Closing < SessionState::WaitServerClose);
        assert!(SessionState::WaitServerClose < SessionState::Closed);
    }

    #[test]
    fn legal_forward_transition() {
        let s = AtomicSessionState::new(SessionState::New);
        let (prev, ok) = s.try_transition(SessionState::Starting, is_state([SessionState::New]));
        assert!(ok);
        assert_eq!(prev, SessionState::New);
        assert_eq!(s.load(), SessionState::Starting);
    }

    #[test]
    fn illegal_transition_rejected_and_state_unchanged() {
        let s = AtomicSessionState::new(SessionState::New);
        // Predicate mandates Ready as precondition — currently New.
        let (prev, ok) = s.try_transition(SessionState::Closing, is_state([SessionState::Ready]));
        assert!(!ok);
        assert_eq!(prev, SessionState::New);
        assert_eq!(s.load(), SessionState::New);
    }

    #[test]
    fn not_state_predicate_blocks_terminal() {
        let s = AtomicSessionState::new(SessionState::Closed);
        let (_, ok) = s.try_transition(SessionState::Closed, not_state([SessionState::Closed]));
        assert!(!ok);
    }

    #[test]
    fn full_happy_path() {
        let s = AtomicSessionState::new(SessionState::New);
        assert!(
            s.try_transition(SessionState::Starting, is_state([SessionState::New]))
                .1
        );
        assert!(
            s.try_transition(SessionState::Ready, is_state([SessionState::Starting]))
                .1
        );
        assert!(
            s.try_transition(SessionState::Closing, is_state([SessionState::Ready]))
                .1
        );
        assert!(
            s.try_transition(
                SessionState::WaitServerClose,
                is_state([SessionState::Closing])
            )
            .1
        );
        assert!(
            s.try_transition(
                SessionState::Closed,
                is_state([SessionState::WaitServerClose])
            )
            .1
        );
        assert!(s.is_terminal());
    }

    #[test]
    fn concurrent_transition_one_winner() {
        use std::sync::Arc;
        let s = Arc::new(AtomicSessionState::new(SessionState::New));
        let s1 = Arc::clone(&s);
        let s2 = Arc::clone(&s);
        let h1 = std::thread::spawn(move || {
            s1.try_transition(SessionState::Starting, is_state([SessionState::New]))
                .1
        });
        let h2 = std::thread::spawn(move || {
            s2.try_transition(SessionState::Starting, is_state([SessionState::New]))
                .1
        });
        let (a, b) = (h1.join().unwrap(), h2.join().unwrap());
        assert!(a ^ b, "exactly one thread should win the CAS");
        assert_eq!(s.load(), SessionState::Starting);
    }
}
