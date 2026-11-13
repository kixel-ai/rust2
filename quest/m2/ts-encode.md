# [M] Multi-byte encodes write in place

## Goal

[rust2-ts](/ts/README.md) translates `to_be_bytes`, `to_le_bytes`, and a literal byte
array that flow only into `extend_from_slice` or `put_slice` as direct stores, with no
typed array made per write. On the 2026-11-12 nightly
([run 43246857664](https://github.com/moq-dev/rust2/actions/runs/43246857664), Chromium) a
QUIC varint encode costs 13.6 ns at one byte, 56.2 at two, and 194 at eight, against
wasm's 4.84, 9.14, and 9.65, and a video frame encode 289 against 33.7.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ before the gate, in m2,
so [the measurement](/quest/m2/wasm/routes.md) judges the translator with its known
losses fixed.

- The cause: `ts/runtime/rs/int.ts`'s `beBytes` makes a `Uint8Array` per call, which
  generated code wraps in a slice for `rs.byteVecExtendFromSlice`
  (`ts/fixtures/codec/expected/coding/varint.ts`'s `writeQuic`). The decode side is
  already scalar.
- Scalar-replace the bytes where the array does not escape: one more escape analysis in
  `ts/src/lower/`, beside the parts representation. An array that does escape keeps
  today's form.
- Expected output changes across the codec fixtures; the differential tests against
  native Rust must not.
- Report every encode row before and after, in Chrome and Firefox.

Public API: none.
