# [S] Kotlin returns a fallible call's value directly

## Goal

A [rust2-ffi](/ffi/README.md) Kotlin call that returns a `Result` or a tuple returns its
value with no `ByteArray` on the success path, and a returned object takes no lock every
other object shares. On [the binding benchmark](/bench/ffi/README.md) Kotlin's write costs
230 ns of CPU time against C's 148, and its object return 210 against C's 25.

## Plan

Found by the 2026-11-12 audit; neither cost is another quest's:
[records](/quest/m1/ffi-speed/kotlin-records.md) covers records only, and
[object ownership](/quest/m1/ffi-speed/object-ownership.md) Go and Dart only.

- Scalar and handle `Result` successes now return directly, errors use a kept native
  exception factory, and cleanup uses striped intrusive lists. The benchmark includes
  four-thread object returns.
- Finish tuple returns and non-scalar `Result` successes: these still return encoded
  `ByteArray` values through `Native.Reader` (`ffi/src/kotlin.rs`). Preserve mutable
  argument writebacks, error ownership, and moved-argument cleanup on every path.
- Add differential coverage for each direct-return path and measure the single-thread
  and contended benchmark before considering this quest complete.

Public API: none; the JNI functions between the generated Kotlin and its glue change.
