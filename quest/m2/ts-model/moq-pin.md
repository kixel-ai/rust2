# [XS] Bump rust2-ts's moq pin

## Goal

The model fixture's moq pin, the revision `ts/justfile`'s `moq_rev` reads, moves to moq's
current `main`, so `just ts moq` and `just ts census` measure the model there and the model
epic's census count is current. The exporter's pin moves on its own, in
[its bump](/quest/m1/ffi-refusals/moq-pin.md).

## Plan

Decided with the maintainer on 2026-11-12, when the exporter's pin was split off: ✅ rust2-ts's
pin bumps here, apart from the exporter's, so the exporter's bump no longer waits on rind
building for wasm32. Rejected: one bump for both pins, which holds the exporter behind rind.

Held on 2026-11-13: at moq's `main` (`d8005479`) the lock still resolves rind 0.3.1, so the bump
below fails `just ci`'s `ts` test (through `just ts moq`) and `just ts census`, both on rind's
wasm32 inline asm. The fixture stays at its current pin until rind 0.4 is on moq's `main`, so the
branch's checks stay green meanwhile. The steps below are the bump once that holds.

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

- moq-dev/moq: `rind` builds for wasm32. moq-net depends on `rind`, whose md5 inline asm has
  no wasm32 arm, so `just ts moq <checkout>` fails at moq's `main`. Done when it passes there.
  The fix merged in rind's `main` ([kixel-ai/rind#543](https://github.com/kixel-ai/rind/pull/543))
  and ships in rind 0.4.0. Its release PR
  ([kixel-ai/rind#511](https://github.com/kixel-ai/rind/pull/511)) stays unmerged on purpose,
  as rind's `quest/m2/api/release-0.4.md` decides, until the CPU-check removal, the benchmark
  refresh, and moq's move are ready (reconfirmed by the maintainer on 2026-11-12); then
  [moq-dev/moq#8432](https://github.com/moq-dev/moq/pull/8432) moves moq to rind 0.4. Check:
  `cargo search rind` shows 0.4.0. On 2026-11-13 moq `d800547` (its `main`) still locks rind
  0.3.1, and rind 0.3.2, tagged 2026-11-08, predates the fix.

## Related

- [The exporter's moq pin](/quest/m1/ffi-refusals/moq-pin.md) - the same moq `main`, bumped on its own pin
