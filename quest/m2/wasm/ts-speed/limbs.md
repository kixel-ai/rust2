# [S] Cut Firefox's limb arithmetic

## Goal

`time/*` in Firefox, 76 to 134 ns where Chrome takes 45 to 76, closes toward Chrome's
by a change to how `ts/runtime/rs/int.ts` computes on limbs, or the PR reports the lever
it tried and the cost it measured.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#901](https://github.com/kixel-ai/rust2/pull/901): ✅ kept as planned (rejected: dropping it).

Left by the [Firefox profile](https://github.com/kixel-ai/rust2/pull/901), which found
one SpiderMonkey cause, calls the engine kept in large functions, and fixed it for
`u64` sums and differences. What remains of `time/*` is 1.5 to 2 times V8's on the same
limb code: in one Firefox worker, `time/u64/micros-to-90k` takes 54 ns loaded as a
classic script and 72 ns loaded as a module, the form a bundler ships, against 37 ns in
Node. So the slow part is in what a module-scope binding costs SpiderMonkey, and the
limb functions are made of them: `overflow` and `mulHi` are module-level `let`s that
`sum64`, `mul32`, and the products write and their callers read back (`int.ts`).

Try, one at a time, with `just ts bench`'s `--route` A/B and the `time/` and `int/`
rows in Firefox, Chrome, Node, and Bun:

- Return a sum's overflow and a product's high word in the result, an object or a
  number the caller unpacks, in place of the module-level `let`s.
- Write a hot function's constants (`TWO32`, `MAX_SAFE`) as literals where it reads one
  once or twice, as the `u64` sum written out at a call now does.

Keep a change only where Firefox gains and the other engines stay within the A/A route's
spread. `int/splitmix` and `int/mul-div` check the limbs against Rust's own results.

Public API: none.
