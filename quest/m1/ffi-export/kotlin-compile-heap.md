# [S] moq's Kotlin compiles in kotlinc's default heap

## Goal

The Kotlin the exporter writes for moq compiles under kotlinc's and Gradle's default heaps, so no consumer needs a JVM flag, and `ffi/justfile` drops the `JAVA_OPTS="-Xmx2g -Xms128M"` its moq recipes set.

## Plan

- Kept by the maintainer on 2026-11-14 ([kixel-ai/rust2#730](https://github.com/kixel-ai/rust2/pull/730#issuecomment-6881950120)), rather than dropped.
- `moq.kt` is one 3.5 MB file, and kotlinc 2.4.21 needs 512 to 640 MB of heap to compile it with `moq/moq.kt`, just past the wrapper's default `-Xmx512M`. Main also outgrew that default by kixel-ai/rust2#805, which set the flag for both moq recipes.
- Measure first: the heap the Android Gradle plugin's Kotlin daemon gets by default, and what compiling `moq.kt` takes there (`just ffi android` builds the api fixture the same way).
- Recommended fix: write one Kotlin file per exported type, or per Rust module, beside `_Native`'s, so peak heap tracks the largest file rather than the whole export. Measure kotlinc's peak on moq before and after.
