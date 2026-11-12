# [S] Waiting: type-check in process with tsrs

## Goal

Once [tsrs](https://blog.cloudflare.com/typescript-rust-port/), the Rust port of
TypeScript, is a stable Rust library, rust2-ts
type-checks its output and bindings in process through it, with no `tsc` on `PATH`,
failing at the Rust span as today, and measures the translation's time against running `tsc`.

## Plan

Decided with the maintainer on 2026-10-27, on
[the bindings PR](https://github.com/moq-dev/rust2/pull/127): translation runs the
flake's `tsc` 7 for now. tsrs was experimental then, and its reported speed over
TypeScript 7 is the vendor's own.

- Its checks must agree with `tsc`'s on every fixture, or the switch waits.

Public API: none, unless the translator drops its `tsc` requirement.

## Waiting

- oxc-project/tsrs: [tsrs](https://github.com/oxc-project/tsrs) publishes a stable release
  as a crate. On 2026-11-12 its latest release is v0.25.0, and crates.io has its
  `tsrs_core` crate only as 0.24.0 and `7.1.0-beta.tsrs1`; its stable release targets
  TypeScript 7.1 with full conformance. Check its releases and `cargo info tsrs_core`.
