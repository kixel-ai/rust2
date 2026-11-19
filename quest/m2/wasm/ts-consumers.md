# [S] Both TypeScript routes work in Safari and strict projects

## Goal

- Both runtimes, rust2-ffi's web `rt.ts` and rust2-ts's, define `Symbol.dispose` as
  `Symbol.for("Symbol.dispose")` when the engine lacks it, at module evaluation before any
  consumer code runs, and key disposal on it. So a consumer's `using` runs the Rust drop on
  stable Safari, which lacks `Symbol.dispose`, whether native or downleveled: TypeScript's
  `__addDisposableResource` throws when `Symbol.dispose` is absent rather than falling back,
  while esbuild's helper falls back to `Symbol.for("Symbol.dispose")`, so a key fallback
  alone would not do. `ffi/tests/api.ts` and a translated fixture's caller use `using`, and
  the WebKit run below covers it native, downleveled by `tsc`, and downleveled by esbuild.
- `just ffi browsers` runs the web binding's test in WebKit too (Playwright's WebKit or
  WPE `cog` from nixpkgs), nightly beside Chromium and Firefox.
- rust2-ts's output type-checks under `noImplicitOverride` (`ts/src/check.rs:23`), which
  moq's tsconfig sets and the runtime fails today at `ts/runtime/rs/tokio.ts:580`.

## Plan

Found by the 2026-11-18 language audit: stable Safari still leaves `Symbol.dispose`
undefined (only [Technology Preview 250](https://webkit.org/blog/18191/release-notes-for-safari-technology-preview-250/)
enables it, [logtape#255](https://github.com/dahlia/logtape/issues/255)), so methods keyed
on it land under the string `"undefined"` (`ffi/runtime/rt.ts:164`, `:383`, `:818`,
`:1085`). Bun stands in for JavaScriptCore in the benchmark and ships the symbol, which hid
it. Safari, iOS's every browser included, is a browser moq's route must reach.

A child of this epic since both routes share the API and the gate reads both.

Public API: none; disposal works where it silently did not.
