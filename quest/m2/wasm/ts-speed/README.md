# rust2-ts's second performance pass

## Goal

The translation's largest measured gaps against rust2-wasm, where the 2026-11-18 audit
found an untried lever, are cut or explained before [the gate](/quest/m2/wasm/README.md)
judges translation. It completes when every child has merged.

## Plan

Measured on the nightly benchmark ([run 44339014570](https://github.com/kixel-ai/rust2/actions/runs/44339014570),
Chromium 157 and Firefox 159): the translation wins cold start (5.7 ms against 8.0 in
Chrome) and size (26 KB gzipped against 66), wins varint decode, group encode and decode,
and hash maps in Chrome, and loses 64-bit math past 2^53 (`time/*` 6 to 12 times),
formatting (6.8), string encode (30), and non-byte slices (`parts/*` 2.5 to 7.4). Firefox
ran `parts/prefix-writes` up to 50 times slower, from calls it kept in large functions, which
[the Firefox profile](https://github.com/kixel-ai/rust2/pull/901) wrote out for a `u64`'s sums
and differences; the rest of its gap is [the limbs](/quest/m2/wasm/ts-speed/limbs.md).

Decided with the maintainer on 2026-11-18: ✅ the whole pass in m2, before the gate,
slice scalarization included (rejected: only the cheap levers in m2; all of it in m3
after the gate). This extends the 2026-11-12 performance pass
([m2](/quest/m2/README.md#plan)).

The audit found MIR still the right input: the `wasm2js` route loses to the translation on
nearly every Chrome row, ships 88 KB gzipped against 26, and starts in 18.7 ms against
5.7.
