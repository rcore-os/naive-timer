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
//! assert_eq!(timer.next(), Some(Duration::from_secs(1)));
//!
//! timer.expire(Duration::from_millis(1000));
//! assert_eq!(event.load(Ordering::SeqCst), true);
//! assert_eq!(timer.next(), None);
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
//!
//! # Limitations
//!
//! For simplicity, **timer cancellation** is not supported.
//!
//! The callback function should check the current time `now` and its own information,
//! to decide whether it is still a valid event.

#![no_std]
#![deny(missing_docs)]
#![deny(warnings)]

use alloc::boxed::Box;
use alloc::collections::BinaryHeap;
use core::cmp::Ordering;
use core::time::Duration;

extern crate alloc;

/// A naive timer.
#[derive(Default)]
pub struct Timer {
    events: BinaryHeap<Event>,
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
        let event = Event {
            deadline,
            callback: Box::new(callback),
        };
        self.events.push(event);
    }

    /// Expire timers.
    ///
    /// Given the current time `now`, trigger and remove all expired timers.
    pub fn expire(&mut self, now: Duration) {
        while let Some(t) = self.events.peek() {
            if t.deadline > now {
                break;
            }
            let event = self.events.pop().unwrap();
            (event.callback)(now);
        }
    }

    /// Get next timer.
    pub fn next(&self) -> Option<Duration> {
        self.events.peek().map(|e| e.deadline)
    }
}

struct Event {
    deadline: Duration,
    callback: Callback,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.deadline.eq(&other.deadline)
    }
}

impl Eq for Event {}

// BinaryHeap is a max-heap. So we need to reverse the order.
impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.deadline.partial_cmp(&self.deadline)
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        other.deadline.cmp(&self.deadline)
    }
}
