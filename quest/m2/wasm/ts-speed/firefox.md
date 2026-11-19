# [S] Find Firefox's uniform slowdown

## Goal

One Firefox profile of the translation's slowest rows (`time/*`, `parts/*`, varint encode)
finds what makes SpiderMonkey uniformly slower than V8 on the translation, and the fix
lands, or the PR reports why it cannot and what it costs the gate.

## Plan

Found by the 2026-11-18 audit: Firefox runs `time/*` 17 to 29 times rust2-wasm, `parts/*`
up to 50, and varint encode 4 to 5, against far smaller gaps in Chrome, which points to
one SpiderMonkey-specific cause. Land after [the 64-bit](/quest/m2/wasm/ts-speed/int64.md)
and [slice](/quest/m2/wasm/ts-speed/slices.md) changes if their PRs are open, so the
profile reads current code.

The string codec's loops (`ts/runtime/rs/utf8.ts`) take Firefox 1.4 to 1.9 times
its native `TextEncoder` and `TextDecoder`, which Chrome's gain outweighs. A microbenchmark
there put much of it on `rs.bytes`: a view into the shared buffer cost SpiderMonkey 20 to
60 ns where a 25-byte encode took 30. `rs.bytes` backs every short byte allocation, so it
may be part of the uniform cause.

Public API: none.
