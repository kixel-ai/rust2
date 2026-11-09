# [M] u64 wrapping arithmetic near a plain add in V8

## Goal

[rust2-ts](/ts/README.md)'s `u64` wrapping arithmetic (`rs.u64WrappingAdd` and its kin)
costs close to a plain `+` while the value stays a safe integer, so the codec bench's
`parts/prefix-writes` runs near its hand-written plain-array route in Node and Chrome.
Exact for every input, proven by differential tests against native Rust past 2^53 and at
the wrap.

## Plan

Found by [#656](https://github.com/moq-dev/rust2/pull/656). Its 12-13x came from an
unbundled harness. The 2026-11-08 nightly (minified bundles, every workload in one worker)
puts translated over hand-written for `parts/prefix-writes` at 16, 256, and 4096 items at:

- Node: 7.5x, 4.0x, 2.9x
- Chrome: 3.9x, 2.7x, 1.9x
- Firefox: 6.7x, 2.2x, 2.0x
- Bun: 2.5x, 1.7x, 1.7x

Measured in Node 24 (V8 13.6) on the bench's minified bundle, after every other workload
had run, at 4096 items, in CPU µs per call:

- The plain array: 370.
- The translation: 1030.
- With `rs.u64WrappingAdd` swapped for a bare `+`: 485.
- With only its two `typeof` tests kept: 760.

The cost is not the operation's instructions. A plain `+` lets TurboFan peel the inner
loop, which hoists the array's map, elements, and copy-on-write checks out of it, so each
item costs a load, an add, and a store. Any branch whose other side has never run
compiles to a branch to a deopt. That deopt leaves the loop without a loop-exit mark, so
`LoopFinder::HasMarkedExits` refuses to peel the loop, and every check stays in it. The
`u64` fast path needs two such branches: the `typeof` tests and the 2^53 bound. Both are
data-dependent, so neither can be folded away before peeling runs.

No exact runtime formulation got past that:

- Testing only the bound an unsigned sum or difference can cross gains 17% with
  `parts` alone, but 0-5% with every workload loaded.
- `(n | 0) === n || n <= MAX_SAFE` lets V8 fold the bound for small integers, but only
  after peeling. It is 2.5x slower for values past 2^32, such as moq's timestamps.
- `Number.isSafeInteger`, `Math.abs`, and an int32 pre-test don't fold before peeling
  either.
- Warming the slow path into a real call makes the loop 1.7x slower.
- V8's `--additive-safe-int-feedback` changes nothing.

Inlining the fast path at each call site only gives each `+` its own type feedback. The
bound branch still blocks peeling.

## Waiting

- Maintainer: decide the quest's fate, since the runtime can't reach its Goal in V8.
  Options:
  1. Delete it.
  2. Keep it waiting on V8 to peel loops with unmarked deopt exits, which needs a V8 bug
     filed outside moq-dev.
  3. Rewrite it toward translator range analysis, which proves a sum safe only where the
     source bounds its operands. That never covers a `Vec<u64>`'s elements, as here.

  Recommend 1: Bun is already at 1.7x, and the rest is a V8 limit no translator output
  can avoid. Check: this file is deleted, or its Goal is rewritten.
