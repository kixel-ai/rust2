# [M] Export as fast on Apple as on Linux

## Goal

The exporter's pass over a facade takes about as long on a macOS host as on Linux.
Today `rust2-ffi`'s check of moq's stand-in, `Checking moq-ffi` to `Finished` in the
`Export` step of the stand-in's build legs, takes 25 s on `ubuntu-24.04` and 5 s on
Windows, but 9 to 10 min on every `macos-26` leg and about 16 on `macos-15-intel`. That is
10 to 18 minutes of each Apple leg of every Nightly, and the same wait for anyone exporting
on a Mac.

## Plan

[#1000](https://github.com/kixel-ai/rust2/pull/1000) removed the two quadratic paths a
Linux profile showed: Linux's export fell from 16 s to 3.5 s, while Apple's fell only from
11.5 to 8.8 min, so what remains is Apple's alone. The Linux profile after it is flat, with
no exporter path above 6%. Its syscalls are a few thousand file reads and writes, mostly
the React Native binding's 1,750 files.

- Profile the export of the stand-in on a Mac (`sample` or `samply` on the `rustc` the
  driver runs as for `moq-ffi`), and export for `aarch64-apple-darwin` from Linux, to tell
  an Apple host's cost (allocator, file system) from an Apple target's (trait solving
  over the `security-framework` and `core-foundation` types the export may walk).
- Fix the cause; a macOS allocator alone points at the driver's allocation rate.

## Hosts

- macos - the slowdown is only on Apple hosts
