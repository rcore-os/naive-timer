//! A minimal naive timer for embedded (no_std) platforms.
//!
//! # Example: single thread
//! ```
//! use alloc::sync::Arc;
//! use core::time::Duration;
//! use core::sync::atomic::{AtomicBool, Ordering};
//! use naive_timer::Timer;
//! extern crate alloc;
//!
//! let mut timer = Timer::default();
//! let event = Arc::new(AtomicBool::new(false));
//!
//! timer.add(Duration::from_secs(1), {
//!     let event = event.clone();
//!     move |now| event.store(true, Ordering::SeqCst)
//! });
//!
//! timer.expire(Duration::from_millis(999));
//! assert_eq!(event.load(Ordering::SeqCst), false);
//!
//! timer.expire(Duration::from_millis(1000));
//! assert_eq!(event.load(Ordering::SeqCst), true);
//! ```
//!
//! # Example: ticks and wakeup
//! ```
//! use alloc::sync::Arc;
//! use core::time::Duration;
//! use core::sync::atomic::{AtomicU64, Ordering};
//! use std::time::{SystemTime, UNIX_EPOCH};
//! use naive_timer::Timer;
//! extern crate alloc;
//!
//! /// Get current time in `Duration`.
//! fn now() -> Duration {
//!     SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
//! }
//!
//! let mut timer = Timer::default();
//!
//! // add timer to wake me up
//! let thread = std::thread::current();
//! timer.add(now() + Duration::from_millis(25), move |_| thread.unpark());
//!
//! // generate ticks (5 times per 10ms)
//! // spawn a thread to emulate timer interrupt
//! let handle = std::thread::spawn(move || {
//!     for _ in 0..5 {
//!         std::thread::sleep(Duration::from_millis(10));
//!         timer.expire(now());
//!     }
//! });
//!
//! // wait for wakeup
//! let t0 = now();
//! std::thread::park();
//! let sleep_time = now() - t0;
//! assert!(sleep_time > Duration::from_millis(30));
//! assert!(sleep_time < Duration::from_millis(40));
//!
//! // join thread
//! handle.join().unwrap();
//! ```

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
