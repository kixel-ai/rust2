# [S] Dart: an implementation given to a call that throws first

## Goal

An implementation Dart gives to a call that throws before it reaches Rust, such as on a
closed object or an integer out of range in another argument, is closed when the call
throws rather than leaked with its relay, proven in `ffi/tests/api.dart`.

## Plan

`_n.give` makes the relay before the call lowers its other arguments, and only Rust's drop
frees it. A leaked relay also keeps the isolate serving every later call on a worker
thread, about 8 µs a call
([moq-dev/rust2#467](https://github.com/moq-dev/rust2/pull/467)).
