# [M] The release builder compiles each target's dependencies once

## Goal

Each build leg of `.github/workflows/bindings.yml` compiles a target's dependency graph once,
not once per package, and every leg for moq's stand-in finishes well inside the 90 minutes it
had before, so the build job's `timeout-minutes` returns to that. Packages stay byte for byte
what they are, as `reproduce` checks.

## Plan

Found by [#994](https://github.com/kixel-ai/rust2/pull/994), which raised the build job's
timeout to 180 minutes. As of its Check (run 45338225999), moq's stand-in's `windows` leg
took 101 minutes and `apple-darwin` 78, and on main's nightly both had run past 90 since
[#964](https://github.com/kixel-ai/rust2/pull/964) and
[#966](https://github.com/kixel-ai/rust2/pull/966) added a JVM library and an abi3t wheel
per host. An Android build of moq takes about 8 minutes, and it is mostly the dependencies.

`build()` in `ffi/release/package.py` gives most packages their own `--target-dir`:
`target-python` and `target-python-abi3t` for the wheels, `target-c`, `target-go` or
`target-swift`, `target-dart`, and `target-android`, while the JVM library already shares
`target-python`. Cargo keeps one artifact per feature set and profile, so a shared directory
rebuilds only the crates whose features or profile differ. Measure which builds share, and
whether `line_tables`' `--config` or a package's environment, such as a deployment target,
keeps two apart. Note that `staticlib`'s comment says cargo does not rebuild for a changed
deployment target, so those builds must stay apart, or the variable must reach the
fingerprint.
