# [XS] Bump rust2-ts's moq pin

## Goal

The model fixture's moq pin, the revision `ts/justfile`'s `moq_rev` reads, moves to moq's
current `main`, so `just ts moq` and `just ts census` measure the model there and the model
epic's census count is current. The exporter's pin moved on its own, to moq
`1aa86d5e6` in [kixel-ai/rust2#881](https://github.com/kixel-ai/rust2/pull/881).

## Plan

Decided with the maintainer on 2026-11-12, when the exporter's pin was split off: ✅ rust2-ts's
pin bumps here, apart from the exporter's, so the exporter's bump no longer waits on rind
building for wasm32. Rejected: one bump for both pins, which holds the exporter behind rind.

Held on 2026-11-13: at moq's `main` (`d8005479`) the lock resolved rind 0.3.1, so the bump below
failed `just ci`'s `ts` test (through `just ts moq`) and `just ts census`, both on rind's wasm32
inline asm. The fixture stays at its current pin until rind 0.4 is on moq's `main`, so the
branch's checks stay green meanwhile. The maintainer settled it on 2026-11-14: ✅ keep the bump
held until rind 0.4 is published and moq moves to it (rejected: push the bump to `d8005479` now
as a red draft; pin an older moq revision now). The steps below are the bump once that holds.

- Set `rev` in `ts/fixtures/model/Cargo.toml` to moq's `main`, then run `cargo update -p moq-net`
  there, as its comment says.
- Run `just ts moq <checkout>` and `just ts census`. Regenerate `expected/`, and any `ts/bench`
  file the new moq API changes, as the last bump ([#390](https://github.com/kixel-ai/rust2/pull/390))
  did for this pin.
- Update the census count in the [model epic](/quest/m2/ts-model/README.md) from the census at
  the new pin.
- Touches no `ffi/` file: the exporter's bump owns those.

Public API: none. The translator's output changes only where moq's API does.

## Waiting

- Maintainer: publish rind 0.4, which carries rind's wasm32 fix
  ([kixel-ai/rind#543](https://github.com/kixel-ai/rind/pull/543)); moq-net depends on `rind`,
  whose md5 inline asm in 0.3 has no wasm32 arm, so `just ts moq <checkout>` fails at moq's `main`.
  The release PR, now [kixel-ai/rind#1058](https://github.com/kixel-ai/rind/pull/1058) (proposing
  rind 0.4.0) after [kixel-ai/rind#511](https://github.com/kixel-ai/rind/pull/511) closed unmerged
  on 2026-11-25, stays unmerged on purpose, as rind's `quest/m2/api/release-0.4.md` decides, until
  its last requirements and moq's move are ready. Both benchmark refreshes before 0.4 have landed
  ([kixel-ai/rind#619](https://github.com/kixel-ai/rind/pull/619) on 2026-11-19, and the second,
  decided in [kixel-ai/rind#949](https://github.com/kixel-ai/rind/pull/949), in
  [kixel-ai/rind#991](https://github.com/kixel-ai/rind/pull/991) on 2026-11-24). An agent cannot
  merge it: rind's release PR merges on the maintainer's approval. Check: `cargo search rind`
  shows 0.4.0; on 2026-11-26 it showed 0.3.2.
- moq-dev/moq: [moq-dev/moq#8432](https://github.com/moq-dev/moq/pull/8432) moves moq to rind 0.4
  once it is published. Check: moq's `main` `Cargo.lock` names rind 0.4.x and `just ts moq` passes
  there; on 2026-11-26 moq `9943f1e` locked rind 0.3.2, which predates the fix.
