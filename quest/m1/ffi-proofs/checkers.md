# [M] Run every language's own checkers

## Goal

The nightly sanitize job, or the gate where a check is fast, adds:

- UBSan with ASan on the C and C++ tests, and Valgrind memcheck on both against the
  uninstrumented library;
- Go's `GOEXPERIMENT=cgocheck2`, and an empty `goroutineleak` profile after the futures
  and cancel cases;
- CheckJNI on HotSpot (`-Xcheck:jni`) and on ART;
- the Swift test under ThreadSanitizer;
- Bun as a runtime of the web binding's test, beside Node and Deno;
- a 32-bit ART run (x86 emulator at API 24), since moq stages armv7 and only 64-bit ABIs
  run today;
- the Python test looped under a debug CPython, with `sys.gettotalrefcount()` and
  `sys.getallocatedblocks()` flat across iterations, and a run on CPython 3.15.

## Plan

Found by the 2026-11-18 language audit. `just ffi sanitize` covers C, C++, Go's race
detector, and Miri; nothing runs the others. The Python runtime does about 3,000 lines of
manual reference counting.

Each check that finds a bug gets the fix in this PR, or a quest. Say how much each added
gate check costs.

Public API: none.
