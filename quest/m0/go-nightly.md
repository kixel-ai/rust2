# [S] The parked Go translator leaves the merge gate

## Goal

`just ci` no longer selects the `go` module: [rust2-go](/go/README.md)'s check and tests
run in the nightly's `just gate` only. Since the translator was parked on 2026-11-07
([its epic](/quest/m4/go/README.md)), about 20 merges that touched no file under `go/`
ran it anyway, because `sh/dispatch.sh`'s `go`
scope also matches `driver/`, the toolchain pin, and `ts/fixtures/codec/`.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ nightly only. Rejected:
keeping it in the gate; deleting `go/`, which the maintainer rejected on 2026-11-07 and
which stays rejected.

- Take `go` out of the modules `just ci` selects in `sh/dispatch.sh`, keep it in
  `just gate`, and update `sh/dispatch.test.py`.
- A change under `go/` itself then merges untested by the gate. Recommended: such a diff
  still selects `go`, so only the shared inputs stop triggering it.
- The nightly's failure issue is how a break in `go/` from a `driver/` change is found; the
  epic accepts that it can rot between nightlies.

Public API: none.
