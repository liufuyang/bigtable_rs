//! Debug-tag ring buffer per Session. Non-blocking recorder for
//! observability of transient events (GOAWAY-before-Start, cancelled-drain,
//! heartbeat miss). Read by later `debugview` code (PR #11).

use parking_lot::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum DebugTag {
    VRPCCancelledDrained,
    GoawayBeforeStart,
    GoawayAfterClose,
    HeartbeatMissed,
    OpenWrongState,
    UnknownResponse,
    ForceCloseNeverStarted,
    ReadLoopPanic,
}

impl DebugTag {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            DebugTag::VRPCCancelledDrained => "vrpc-cancelled-drained",
            DebugTag::GoawayBeforeStart => "goaway-before-start",
            DebugTag::GoawayAfterClose => "goaway-after-close",
            DebugTag::HeartbeatMissed => "heartbeat-missed",
            DebugTag::OpenWrongState => "open-wrong-state",
            DebugTag::UnknownResponse => "unknown-response",
            DebugTag::ForceCloseNeverStarted => "force-close-never-started",
            DebugTag::ReadLoopPanic => "readloop-panic",
        }
    }
}

/// Fixed-capacity per-Session ring. Records are dropped on overflow
/// (non-blocking recording is the point — the alternative would be
/// stalling a lifecycle path on a full buffer, worst possible failure
/// mode for a debug facility).
#[derive(Debug)]
pub(crate) struct DebugTagRing {
    inner: Mutex<Inner>,
}

#[derive(Debug)]
struct Inner {
    // (tag, monotonic seq) — seq lets the debugview later render events
    // in insertion order even after wraparound.
    slots: Vec<Option<(DebugTag, u64)>>,
    head: usize,
    next_seq: u64,
    dropped: u64,
}

impl DebugTagRing {
    pub(crate) fn with_capacity(cap: usize) -> Self {
        assert!(cap > 0, "DebugTagRing capacity must be > 0");
        Self {
            inner: Mutex::new(Inner {
                slots: vec![None; cap],
                head: 0,
                next_seq: 0,
                dropped: 0,
            }),
        }
    }

    pub(crate) fn record(&self, tag: DebugTag) {
        let mut g = self.inner.lock();
        let seq = g.next_seq;
        g.next_seq = g.next_seq.wrapping_add(1);
        let head = g.head;
        let cap = g.slots.len();
        if g.slots[head].is_some() {
            g.dropped = g.dropped.wrapping_add(1);
        }
        g.slots[head] = Some((tag, seq));
        g.head = (head + 1) % cap;
    }

    pub(crate) fn snapshot(&self) -> Vec<(DebugTag, u64)> {
        let g = self.inner.lock();
        let mut out: Vec<_> = g.slots.iter().flatten().copied().collect();
        out.sort_by_key(|(_, seq)| *seq);
        out
    }

    pub(crate) fn dropped(&self) -> u64 {
        self.inner.lock().dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_in_order_below_capacity() {
        let r = DebugTagRing::with_capacity(4);
        r.record(DebugTag::GoawayBeforeStart);
        r.record(DebugTag::HeartbeatMissed);
        let snap = r.snapshot();
        assert_eq!(snap.len(), 2);
        assert_eq!(snap[0].0, DebugTag::GoawayBeforeStart);
        assert_eq!(snap[1].0, DebugTag::HeartbeatMissed);
    }

    #[test]
    fn wraparound_counts_drops() {
        let r = DebugTagRing::with_capacity(2);
        r.record(DebugTag::GoawayBeforeStart);
        r.record(DebugTag::HeartbeatMissed);
        r.record(DebugTag::VRPCCancelledDrained);
        assert_eq!(r.dropped(), 1);
        let snap = r.snapshot();
        assert_eq!(snap.len(), 2);
        // Newest two win; seq ordering preserved.
        assert!(snap.iter().any(|(t, _)| *t == DebugTag::HeartbeatMissed));
        assert!(snap
            .iter()
            .any(|(t, _)| *t == DebugTag::VRPCCancelledDrained));
    }
}
