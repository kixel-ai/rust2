# [M] Find why ART's throughput falls off from 16 KiB frames

## Goal

[The binding benchmark](/bench/ffi/README.md#kotlin-on-art)'s Kotlin throughput on ART at
16 KiB to 1 MiB frames is explained by a profile, and cut where the binding causes it, with
before and after numbers in the README.

## Plan

Found by the ART measurement on 2026-11-20: on one API 35 x86_64 emulator, ART moves 100 B
and 1 KiB frames within 3x of HotSpot (52 against 149 MB/s, 411 against 1,195), but from
16 KiB on it holds near 650 MB/s against HotSpot's 6.7 to 9.3 GB/s. Each frame is written
from a `ByteArray` and read back as `Bytes` whose first byte is touched, so the suspects are
the copy into Rust (`GetByteArrayRegion`, which ART may route through a pinned or copied
array), a copy out on the read, and the emulator's memory bandwidth, which a native C
memcpy loop on the same emulator separates from the binding. Profile with `simpleperf` on
the emulator before changing anything.

Public API: none expected.

## Hosts

- linux - the Android emulator needs KVM
