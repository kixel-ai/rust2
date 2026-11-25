# [M] The release builder compiles each target's dependencies once, on the builds left over

## Goal

Each build leg of `.github/workflows/bindings.yml` compiles a target's dependency graph once
per distinct flag set, not once per package, now also for the Swift, Go, Android, and
Apple C builds, and the build job's `timeout-minutes` falls below 120 only if every leg of
moq's stand-in then finishes with a third to spare. Each package's contents match main's
build of the same source, shown by diffing a Check's packages on the branch against one on
main; `reproduce` alone shows only that two builds of the branch agree.

## Plan

Found by [#994](https://github.com/kixel-ai/rust2/pull/994), which raised the build job's
timeout to 180 minutes. As of its Check (run 45338225999), moq's stand-in's `windows` leg
took 101 minutes and `apple-darwin` 78, and on main's nightly both had run past 90 since
[#964](https://github.com/kixel-ai/rust2/pull/964) and
[#966](https://github.com/kixel-ai/rust2/pull/966) added a JVM library and an abi3t wheel
per host. An Android build of moq takes about 8 minutes, and it is mostly the dependencies.

[#1000](https://github.com/kixel-ai/rust2/pull/1000) builds both wheels, the Dart and JVM
libraries, and the C library off Apple in `WHEELS_DIR` (`target-python`), and set the cap to
120 minutes. Left: `target-c` on Apple, `target-swift`, `target-go`, and `target-android`.
Cargo keeps one artifact per feature set and profile, so a shared directory rebuilds only the
crates whose features or profile differ. Measure which of those builds share, and whether
`line_tables`' `--config` or a package's environment, such as a deployment target, keeps two
apart. Note that `staticlib`'s comment says cargo does not rebuild for a changed deployment
target, so those builds must stay apart, or the variable must reach the fingerprint. Apple's C
library keeps its install name in the flags every crate builds with, so it shares only if
that moves to the generated crate's own `--` arguments.
