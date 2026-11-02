# [M] Dart: cheaper ready futures

## Goal

A read through the generated Dart whose future is ready at its first poll costs no more
than Python's or Go's, measured by `just ffi bench`'s frame written and read.

## Plan

Measured for the Dart generator on a loaded host, so read the ratios: a frame written and
read costs about 1.8 to 3 µs in Dart, against about 1 µs in Python and Go, while a handle
call costs 23 to 45 ns. The read's three crossings are the same as theirs. What Dart adds
is a `Completer` per call, two `_Arena`s, the frame's `NativeFinalizer`, and the microtask
each `await` takes.

Left to try:

- profiling the read with `dart --observe` before changing it;
- completing a ready `Cancelable` without a `Completer`, as a future already complete;
- one arena for the call and its first poll.
