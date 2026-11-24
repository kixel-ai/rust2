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

- `ffi/surface/surface.py` renders only the surfaces a check asks for. `just ci` keeps
  catching a snapshot no surface writes anymore, through a cheap inventory of every
  surface's output paths checked against the files on disk; the full render stays nightly.
- Fine-grained diff dispatch in `sh/dispatch.sh` within `ffi/` and `ts/`: a change to one
  language's emitter, runtime, or tests runs that language's tests, and anything shared
  runs all. Test the map on executed coverage: for each changed file, the selected tests must
  include every test whose coverage executes it, and the check fails on any one missing,
  not only on the selector's output, as #960's `dispatch.test.py` did.
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

Decided 2026-11-23 (maintainer, /quest-run): ✅ keep this quest (rejected: dropping it).

Decided 2026-11-23 (maintainer, /quest-run): ✅ switch AGENTS.md's pre-push rule from
`just ci` to `just check` plus the tests a change touches, as #960 proposed, only once
[hosted CI](/quest/m0/ci-hosted.md)'s remote PR CI runs reliably (rejected: switching now;
keeping `just ci` for good). Until then `just ci` is the only full check a PR gets before
the queue.

## Related

- [Hosted CI](/quest/m0/ci-hosted.md) - the rest of #960
