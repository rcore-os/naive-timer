# naive-timer

[![Crate](https://img.shields.io/crates/v/naive-timer.svg)](https://crates.io/crates/naive-timer)
[![Docs](https://docs.rs/naive-timer/badge.svg)](https://docs.rs/naive-timer)
[![Actions Status](https://github.com/rcore-os/naive-timer/workflows/CI/badge.svg)](https://github.com/rcore-os/naive-timer/actions)
[![Coverage Status](https://coveralls.io/repos/github/rcore-os/naive-timer/badge.svg)](https://coveralls.io/github/rcore-os/naive-timer)

A minimal naive timer for embedded platforms in Rust (no_std + alloc).

## Example

```rust
let mut timer = naive_timer::Timer::default();
let event = Arc::new(AtomicBool::new(false));

// add a timer with callback
timer.add(Duration::from_secs(1), {
    let event = event.clone();
    move |_now| event.store(true, Ordering::SeqCst)
});

// expire timers (usually from timer interrupt)
timer.expire(Duration::from_millis(1000));
assert_eq!(event.load(Ordering::SeqCst), true);
assert_eq!(timer.next(), None);
```

## License

The code in this repository is licensed under the MIT License.
