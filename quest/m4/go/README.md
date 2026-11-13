# moq in pure Go

## Goal

[rust2-go](/go/README.md) translates moq-net to pure Go, with no cgo, close enough to the
exporter's [cgo bindings](/ffi/README.md#go) in speed that Go users can take it
in their place: `go get` installs it and it cross-compiles.

## Plan

Decided with the maintainer on 2026-11-01: Go users avoid cgo, which breaks `go get`'s
plain install and cross-compiling, so a pure-Go moq is worth researching. The cgo
exporter stays moq's Go route until a translation matches it.

Decided with the maintainer on 2026-11-01, settling
[#410](https://github.com/kixel-ai/rust2/pull/410):

- **Continue as research,** speed first, then
  [coverage](/quest/m4/go/coverage.md). If speed cannot bring the translated crate
  within 2x of cgo on a realistic whole-crate workload (a group's stream end to end,
  decided on 2026-11-02 in [#428](https://github.com/kixel-ai/rust2/pull/428)), delete
  `go/` and this epic, keeping the measurements in the PR that does.
- **Share rust2-ts's front end piece by piece,** as Go needs each piece, starting with
  the control-flow structuring, now `driver/src/cfg.rs`. No shared crate lands ahead
  of a Go use. About a quarter of `ts/src` would serve Go, nearly all of it inside files
  whose lowering returns TypeScript's AST.

Decided with the maintainer on 2026-11-07, settling
[kixel-ai/rust2#643](https://github.com/kixel-ai/rust2/pull/643):

- **Speed met the bar.** With the translation owning a `Vec<u8>`'s storage and clipping
  it at escape, every `stream/` workload of `just go bench` runs within 2x of cgo, the
  worst at 2.07x CPU time, which counts as noise.
- ✅ **Parked until a consumer needs pure Go.** None has asked, and the cgo exporter
  stays moq's Go route. This epic and its children don't start until one does; each
  child's `Waiting` item holds it. When one does, first weigh running moq compiled to
  WASM inside Go on a pure-Go runtime such as [wazero](https://wazero.io) (no cgo, a
  plain `go get`; unmeasured for moq-net's I/O) against finishing this translation.
  Rejected: splitting [coverage](/quest/m4/go/coverage.md) into a quest per construct
  now; deleting `go/`, whose translator and measurements are kept.

The spike ([kixel-ai/rust2#410](https://github.com/kixel-ai/rust2/pull/410)) translated
moq's codec held to rust2-ts's bar: 35,273 codec cases and 124,193 integer cases match
native Rust, panics included. On 2026-11-01 it ran 2 to 113 times slower than cgo on
batched loops, which #643 above superseded. Stripped, it adds 324 KB to a Go binary where
cgo adds 1.15 MB and a libc dependency.

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ `go/` left the merge gate for the nightly: `just ci` selects it only for a diff
  under `go/`. Rejected: keeping it in the gate; deleting `go/`, which stays rejected.
- ✅ The four Go API quests planned from
  [#614](https://github.com/kixel-ai/rust2/pull/614)'s follow-ups are folded in here as
  notes, since none can start while the epic is parked and each rewrote the same
  `go/fixtures/items`. Their full text is in git history, last at `bf5ca258`, under
  `quest/m4/go/`. When the epic resumes, plan them again in this order:
  - `runtime-names.md` first, since it settles the package boundary the others name
    against: the runtime becomes its own Go package, so a crate's `Result` or
    `Formatter` cannot collide with it. The audit found its cost understated: generated
    code names 76 unexported runtime functions and 19 unexported types that would have
    to be exported.
  - `enums.md`: an exported tag, a constant, and a constructor per variant, so another
    package can build and match a `pub enum`. Its scope must add the runtime's own
    `Option`, `Result`, and `Ordering`, which have an unexported `tag` and no exported
    constructor (`go/runtime/rt.go`).
  - `trait-impls.md`: `Display` as `String()`, `Debug` as `GoString()`, `PartialEq` as
    `Equal`, and a naming rule for every other trait's methods.
  - `alias-methods.md`: a private generic type's methods reached only through a
    specialized alias (`pub type Byte = Value<u8>`), which #614 refuses.
