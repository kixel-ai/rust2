# [L] Measured test work cut

## Goal

`just ci` does less work for the same coverage, each cut backed by a measurement: the
`ffi` module's time on a diff that touches one binding, and the whole suite's, drop by
what the PR reports, and no diff skips a test that the code it changes runs under.

## Plan

Planned from [#960](https://github.com/kixel-ai/rust2/pull/960), alongside
[hosted CI](/quest/m0/ci-hosted.md). Decided 2026-11-23 (maintainer, /quest-run): ✅ split
#960 into quests (rejected: taking it over as one PR; leaving it). Each item is worth its
place only where a benchmark against [#971](https://github.com/kixel-ai/rust2/pull/971)'s
optimized exporter shows a gain; drop one that shows none and say so.

- `ffi/surface/surface.py` renders only the surfaces a check asks for; the full stale-file
  check, which catches a snapshot no surface writes anymore, stays in the nightly.
- Fine-grained diff dispatch in `sh/dispatch.sh` within `ffi/` and `ts/`: a change to one
  language's emitter, runtime, or tests runs that language's tests, and anything shared
  runs all. Test the map on executed coverage, that a selected test set exercises the files
  the diff changes, not only on the selector's output, as #960's `dispatch.test.py` did.
  Cover the languages that exist now, which #960 predates, such as C# and Python ABI3T.
- Parallel binding generation in `ffi/src/glue.rs` and `web.rs`, the C++ and Go fixture
  tests in `ffi/tests/fixtures.rs`, and `surface.py`'s extraction, each only where measured
  faster, on a bounded pool instead of #960's thread per item.

Dropped from #960, and why:

- `[profile.dev] opt-level = 1` in `ffi/` and `ts/`: superseded for the exporter by #971,
  which optimizes `rust2-ffi` and `rust2-driver` alone. Workspace-wide, it also optimizes
  every dependency, which was never measured.
- `.github/` changes as lint-only: actionlint shows a workflow parses, not that its build
  commands still work.

## Waiting

- Maintainer: whether agents run `just check` plus the tests their change touches before a
  push, instead of `just ci`, as #960 proposed. It changes AGENTS.md's rule, so it is the
  maintainer's call. Options: keep `just ci`; or switch once
  [hosted CI](/quest/m0/ci-hosted.md) runs every PR remotely. Recommended: switch only once
  hosted CI has run reliably on PRs, since until then `just ci` is the only full check a PR
  gets before the queue. Check: the decision is recorded in this Plan.

## Related

- [Hosted CI](/quest/m0/ci-hosted.md) - the rest of #960
