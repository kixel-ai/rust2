# [S] Find Firefox's uniform slowdown

## Goal

One Firefox profile of the translation's slowest rows (`time/*`, `parts/*`, varint encode)
finds what makes SpiderMonkey uniformly slower than V8 on the translation, and the fix
lands, or the PR reports why it cannot and what it costs the gate.

## Plan

Found by the 2026-11-18 audit: Firefox runs `time/*` 17 to 29 times rust2-wasm, `parts/*`
up to 50, and varint encode 4 to 5, against far smaller gaps in Chrome, which points to
one SpiderMonkey-specific cause. Land after [the slice](/quest/m2/wasm/ts-speed/slices.md)
change if its PR is open, so the profile reads current code. The 64-bit change left
`time/*` as it was, so the bigint cost is not the cause.

Public API: none.
