//! Lifecycle hooks. SESSION_SPEC §4.
//!
//! Fixed ordering `OnStart → OnActive → OnClosing → OnClose`; each fires
//! exactly once over the Session's lifetime. `OnSlotDrained` fires per
//! drain event (SESSION_SPEC §2), not once — it's ordered separately.

use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) type HookFn = Box<dyn Fn() + Send + Sync>;

#[derive(Default)]
pub(crate) struct SessionHooks {
    pub(crate) on_start: Option<HookFn>,
    pub(crate) on_active: Option<HookFn>,
    pub(crate) on_closing: Option<HookFn>,
    pub(crate) on_close: Option<HookFn>,
    pub(crate) on_slot_drained: Option<HookFn>,

    // Once-guards for the four ordered hooks. AtomicBool with AcqRel
    // matches the Java sync-context/Go sync.Once semantics here — the
    // publish side (whoever wins the CAS) sees a happens-before to any
    // Acquire-load observer of `true`.
    started: AtomicBool,
    activated: AtomicBool,
    closing_fired: AtomicBool,
    closed_fired: AtomicBool,
}

impl std::fmt::Debug for SessionHooks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionHooks")
            .field("has_on_start", &self.on_start.is_some())
            .field("has_on_active", &self.on_active.is_some())
            .field("has_on_closing", &self.on_closing.is_some())
            .field("has_on_close", &self.on_close.is_some())
            .field("has_on_slot_drained", &self.on_slot_drained.is_some())
            .field("started", &self.started.load(Ordering::Acquire))
            .field("activated", &self.activated.load(Ordering::Acquire))
            .field("closing_fired", &self.closing_fired.load(Ordering::Acquire))
            .field("closed_fired", &self.closed_fired.load(Ordering::Acquire))
            .finish()
    }
}

impl SessionHooks {
    pub(crate) fn fire_on_start(&self) {
        if self
            .started
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            if let Some(h) = &self.on_start {
                h();
            }
        }
    }

    pub(crate) fn fire_on_active(&self) {
        if self
            .activated
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            if let Some(h) = &self.on_active {
                h();
            }
        }
    }

    pub(crate) fn fire_on_closing(&self) {
        if self
            .closing_fired
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            if let Some(h) = &self.on_closing {
                h();
            }
        }
    }

    /// Safety-net: fires OnClosing before OnClose if some path forgot. See
    /// Go `notifyClosed → notifyClosing` — the ordering contract holds
    /// even when a caller skipped Closing (e.g. ForceClose from NEW).
    pub(crate) fn fire_on_close(&self) {
        if self
            .closed_fired
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            self.fire_on_closing();
            if let Some(h) = &self.on_close {
                h();
            }
        }
    }

    pub(crate) fn fire_on_slot_drained(&self) {
        if let Some(h) = &self.on_slot_drained {
            h();
        }
    }
}

/// Builder that accepts individual hooks without exposing SessionHooks'
/// private once-guards.
#[derive(Default)]
pub(crate) struct SessionHooksBuilder {
    pub(crate) on_start: Option<HookFn>,
    pub(crate) on_active: Option<HookFn>,
    pub(crate) on_closing: Option<HookFn>,
    pub(crate) on_close: Option<HookFn>,
    pub(crate) on_slot_drained: Option<HookFn>,
}

impl SessionHooksBuilder {
    pub(crate) fn on_start<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_start = Some(Box::new(f));
        self
    }
    pub(crate) fn on_active<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_active = Some(Box::new(f));
        self
    }
    pub(crate) fn on_closing<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_closing = Some(Box::new(f));
        self
    }
    pub(crate) fn on_close<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }
    pub(crate) fn on_slot_drained<F: Fn() + Send + Sync + 'static>(mut self, f: F) -> Self {
        self.on_slot_drained = Some(Box::new(f));
        self
    }
    pub(crate) fn build(self) -> SessionHooks {
        SessionHooks {
            on_start: self.on_start,
            on_active: self.on_active,
            on_closing: self.on_closing,
            on_close: self.on_close,
            on_slot_drained: self.on_slot_drained,
            ..SessionHooks::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    fn counter() -> (Arc<AtomicUsize>, HookFn) {
        let c = Arc::new(AtomicUsize::new(0));
        let cc = Arc::clone(&c);
        (
            c,
            Box::new(move || {
                cc.fetch_add(1, Ordering::AcqRel);
            }),
        )
    }

    #[test]
    fn each_lifecycle_hook_fires_at_most_once() {
        let (start_c, start) = counter();
        let (active_c, active) = counter();
        let (closing_c, closing) = counter();
        let (close_c, close) = counter();
        let hooks = SessionHooks {
            on_start: Some(start),
            on_active: Some(active),
            on_closing: Some(closing),
            on_close: Some(close),
            ..SessionHooks::default()
        };
        hooks.fire_on_start();
        hooks.fire_on_start();
        hooks.fire_on_active();
        hooks.fire_on_active();
        hooks.fire_on_closing();
        hooks.fire_on_closing();
        hooks.fire_on_close();
        hooks.fire_on_close();
        assert_eq!(start_c.load(Ordering::Acquire), 1);
        assert_eq!(active_c.load(Ordering::Acquire), 1);
        assert_eq!(closing_c.load(Ordering::Acquire), 1);
        assert_eq!(close_c.load(Ordering::Acquire), 1);
    }

    #[test]
    fn on_slot_drained_fires_each_call() {
        let (c, drained) = counter();
        let hooks = SessionHooks {
            on_slot_drained: Some(drained),
            ..SessionHooks::default()
        };
        hooks.fire_on_slot_drained();
        hooks.fire_on_slot_drained();
        hooks.fire_on_slot_drained();
        assert_eq!(c.load(Ordering::Acquire), 3);
    }

    #[test]
    fn close_safety_net_fires_closing_first() {
        use parking_lot::Mutex;
        let order = Arc::new(Mutex::new(Vec::<&'static str>::new()));
        let o1 = Arc::clone(&order);
        let o2 = Arc::clone(&order);
        let hooks = SessionHooks {
            on_closing: Some(Box::new(move || o1.lock().push("closing"))),
            on_close: Some(Box::new(move || o2.lock().push("close"))),
            ..SessionHooks::default()
        };
        hooks.fire_on_close(); // Skipped Closing intentionally.
        assert_eq!(*order.lock(), vec!["closing", "close"]);
    }

    #[test]
    fn concurrent_fire_wins_exactly_one() {
        let (c, hook) = counter();
        let hooks = Arc::new(SessionHooks {
            on_active: Some(hook),
            ..SessionHooks::default()
        });
        let mut handles = vec![];
        for _ in 0..8 {
            let h = Arc::clone(&hooks);
            handles.push(std::thread::spawn(move || h.fire_on_active()));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(c.load(Ordering::Acquire), 1);
    }

    #[test]
    fn missing_hooks_are_no_ops() {
        let hooks = SessionHooks::default();
        hooks.fire_on_start();
        hooks.fire_on_active();
        hooks.fire_on_closing();
        hooks.fire_on_close();
        hooks.fire_on_slot_drained();
    }
}
