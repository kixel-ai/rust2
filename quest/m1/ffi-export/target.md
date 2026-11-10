# [L] Export for a target

## Goal

`rust2-ffi --target <triple>` exports the `pub` API the facade has on that target, so an
item under `cfg(target_os = "android")`, `cfg(windows)`, or `cfg(target_arch = "wasm32")`
is in that target's bindings and in no other's, and the glue crate builds for the target
with no item missing or left over. Proven by a fixture whose API differs by target,
exported and built for the host and at least one other target, and by moq's census on each
target moq ships.

## Plan

Found in the 2026-11-09 quest audit: the check runs on the host whatever cargo's config
names (`ffi/src/cli.rs`'s `Check` sets `target: Some(host_tuple())`), and `--target` is
refused ("the export builds for the host"). So a Linux host exports moq-tokio's `unix`
module into bindings shipped to Windows, which `ffi/moq/ffi.toml` hides today with its
`tokio::unix::Error` exclude, and no host can export for wasm32, which
[rust2-wasm](/quest/m2/wasm/README.md) needs.

Decided with the maintainer on 2026-11-09: ✅ m1, in the exporter epic, since
[the refusals epic](/quest/m1/ffi-export/refusals/README.md) deletes the excludes that
hide target-gated items. Rejected: m2 with the wasm backend, leaving the exporter host-only.

- The check needs the target's std and, for a build script or proc macro, the host's, as
  cargo already splits them; rust2-ts checks for `wasm32-unknown-unknown` through the same
  front end ([`driver/`](/driver)).
- The C ABI's layouts are the target's: `usize`, pointer width, and struct layout come from
  that target's check, including a 32-bit one (`ffi/runtime/jni.rs` passes `usize` as
  `u64` and casts, which truncates silently on a 32-bit Android ABI).
- Open: how a binding several targets share is exported when their APIs differ, such as one
  Swift package for iOS and macOS or one Kotlin file for every Android ABI. Recommended:
  export once per target, and where a shared binding is asked for, refuse an item whose
  export differs between its targets, at the item's span, naming both. Report first what
  differs in moq at the pin between the targets it ships.

Public API: the `--target` flag, and each target's exports.
