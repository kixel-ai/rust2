# [M] A constant only some targets have exists only there

## Goal

Every binding gives a `pub const` that only some of an export's targets have, such as a
`usize` constant under `#[cfg(target_pointer_width = "64")]`, exactly where its library
has it: absent from what a build for any other target compiles, or where one package
serves several targets at run time, an error on reading it there, as a function the
library's target lacks is. No binding truncates its value or fails to build for a target
that lacks it.

## Plan

- Found 2026-11-23 in review round 1 of React Native's binding, by its api fixture's
  `WORD: usize = usize::MAX` on 64-bit targets alone. Every generator ignores
  `Constant::on`:
  - Go writes `const WORD uint = 18446744073709551615`, which no 32-bit `GOARCH` compiles,
    and Swift a `UInt` literal no ILP32 target such as watchOS's `arm64_32` does.
  - C's `#define API_WORD ((size_t)18446744073709551615ULL)` truncates on a 32-bit target.
  - React Native's one package, which serves 64-bit and 32-bit ABIs at once, leaves out a
    constant only some of its targets have, so `api.WORD` is missing from a package that
    serves armv7.
  - C#'s property already throws on a 32-bit runtime, through `checked`.
- Compile-time bindings, such as C's `#if`, Go's build constraints, and Swift's `#if arch`,
  gate the constant by the targets in `on`. Run-time ones, such as React Native and C#,
  read it through a check of `c_target()`, as `lacks` does.
- The api fixture's `WORD` checks each binding.
