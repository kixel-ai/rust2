# [M] Python methods without a wrapper frame

## Goal

A method call on an object the exporter's [Python](/ffi/README.md) generates enters the
extension directly, with no Python wrapper frame in between, saving about 25 ns of a call's
104 ns. `just ffi bench` reports the call against main and
[BoltFFI's](/bench/ffi/README.md#boltffi-comparison) in the same run. The Python API and the
GIL release during a call stay as they are.

## Plan

Decided with the maintainer on 2026-11-03: planned from the alternatives
[#560](https://github.com/moq-dev/rust2/pull/560) measured and dropped. That PR found the wrapper
frame costs about 25 ns; native methods (a `PyType_FromSpec` method table, or whatever
CPython offers that is faster) would remove it. It changes how the generated classes are
built, so keep what callers see the same: signatures, keyword arguments, defaults,
docstrings, and the type stubs. Holding the GIL stays rejected, since a call blocking on a
lock a Rust thread holds while calling Python back would deadlock.

moq's Python bindings are the consumer. Public API: none.

## Related

- [Python calls skip the loop lookup](/quest/m3/ffi-python-loop-lookup.md) - the other per-call cost left after #564
