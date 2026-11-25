# [M] Export as fast on Apple as on Linux

## Goal

The exporter's pass over a facade takes about as long on a macOS host as on Linux.
Historical CI measurements of `rust2-ffi`'s check of moq's stand-in, `Checking moq-ffi` to `Finished` in the
`Export` step of the stand-in's build legs, takes 25 s on `ubuntu-24.04` and 5 s on
Windows, but 9 to 10 min on every `macos-26` leg and about 16 on `macos-15-intel`. That is
9 to 16 minutes of each Apple leg of every Nightly, and the same wait for anyone exporting
on a Mac.

## Plan

[#998](https://github.com/kixel-ai/rust2/pull/998) supplies the finalized type-name
cache and the one-pass Dart constructor scan. The earlier Linux 16 s to 3.5 s profile
used #1000's alternative implementation, which its merge dropped, so it is not a
baseline for this quest. The CI figures above measure `Checking moq-ffi` to `Finished`,
including rustc's check; they are not the isolated warm exporter measurement.

- First measure the current merged exporter on both Linux and macOS, using the same
  command and timing boundaries, and profile Linux again. Owner scans remain in
  `Abi::export`, `homed`, and `status`; distinguish shared costs from host-specific ones.
- Profile the export of the stand-in on a Mac (`sample` or `samply` on the `rustc` the
  driver runs as for `moq-ffi`), and export for `aarch64-apple-darwin` from Linux, to tell
  an Apple host's cost (allocator, file system) from an Apple target's (trait solving
  over the `security-framework` and `core-foundation` types the export may walk).
- Fix the cause; a macOS allocator alone points at the driver's allocation rate.

## Hosts

- macos - the slowdown is only on Apple hosts
