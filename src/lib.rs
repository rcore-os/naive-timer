//! A minimal naive timer for embedded (no_std) platforms.

#![no_std]
#![feature(map_first_last)]
#![deny(missing_docs)]
#![deny(warnings)]

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::time::Duration;

extern crate alloc;

/// A naive timer.
#[derive(Default)]
pub struct Timer {
    events: BTreeMap<Duration, Callback>,
}

/// The type of callback function.
type Callback = Box<dyn FnOnce(Duration) + Send + Sync + 'static>;

impl Timer {
    /// Add a timer.
    ///
    /// The `callback` will be called on timer expired after `deadline`.
    pub fn add(
        &mut self,
        deadline: Duration,
        callback: impl FnOnce(Duration) + Send + Sync + 'static,
    ) {
        let old = self.events.insert(deadline, Box::new(callback));
        assert!(old.is_none(), "exist a timer with deadline {:?}", deadline);
    }

    /// Expire timers.
    ///
    /// Given the current time `now`, trigger and remove all expired timers.
    pub fn expire(&mut self, now: Duration) {
        while let Some(entry) = self.events.first_entry() {
            if *entry.key() > now {
                return;
            }
            let (_, callback) = entry.remove_entry();
            callback(now);
        }
    }
}
