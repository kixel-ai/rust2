# [S] moq's Kotlin compiles in kotlinc's default heap

## Goal

The Kotlin the exporter writes for moq compiles under kotlinc's and Gradle's default heaps, so no consumer needs a JVM flag, and `ffi/justfile` drops the `JAVA_OPTS="-Xmx2g -Xms128M"` its moq recipes set.

## Plan

- Kept by the maintainer on 2026-11-14 ([kixel-ai/rust2#730](https://github.com/kixel-ai/rust2/pull/730#issuecomment-6881950120)), rather than dropped.
- Measured on 2026-11-15 ([kixel-ai/rust2#833](https://github.com/kixel-ai/rust2/pull/833)), kotlinc 2.4.21 on JDK 21, moq at the exporter's pin:
  - `moq.kt` is 3.9 MB, 90,822 lines. It needs more than 528 MiB of heap and compiles in 544 MiB, with or without `moq/moq.kt`.
  - The heap tracks the whole module, not the largest file, so the recommended split into files does not help. With all files in one kotlinc run, max live heap after GC is 676 MB as one file and 664 MB as 7 files, and one file per declaration (1,062 files) runs out at 2 GB.
  - A heap dump at the out-of-memory error (628 MiB) shows why. The JVM backend keeps every class's ASM writer until output: about 70 MiB of labels, frames, and symbol tables, plus most of the 156 MiB of `byte[]`. The light syntax tree also stays for line numbers: 49 MiB of 689,000 markers. IR and FIR take about 80 MiB more. The peak is in codegen, after FIR, of 10.4 MB of class files in 1,700 classes.
  - KDoc, 1.3 MB of the 3.9 MB, costs nothing measurable, and nor does inlining `block` once in the runtime's `hold(handle?, block)` instead of twice.
  - Stock Gradle 8.14 gives its daemon `-Xmx512m`, and the Kotlin daemon inherits it. A stock `kotlin("jvm")` build of `moq.kt` passed once, and an identical rebuild ran out of memory after 7 minutes of GC.
  - moq's own Kotlin build, the one consumer that compiles `moq.kt`, sets `org.gradle.jvmargs=-Xmx2g` in `kt/gradle.properties`, as Android Studio's new-project template does with 2 GB. Apps take its published library and never compile `moq.kt`. Only `ffi/justfile`'s raw kotlinc runs at the launcher's 512 MiB default.
- What would meet the Goal is about a quarter less code for kotlinc to hold, so the build clears 512 MiB with room as moq grows: fewer classes and methods, or less bytecode per method. The generic families moq exports once per instance, such as `MuxContainerProducer{Container,Fmp4Wire,LegacyWire}{Video,Audio,Text,Json,Binary}Config`, are the bulk of it.

## Waiting

- Maintainer: whether to keep this quest now that its plan is refuted. Options:
  1. (Recommended) Delete it. The consumer that compiles `moq.kt` already gives Gradle 2 GB, and the flag in `ffi/justfile` only sizes this repository's own test runs.
  2. Rewrite it to shrink the generated Kotlin by about a quarter, resized to L. It needs a design for the generic families or the per-method code, with an uncertain payoff.
