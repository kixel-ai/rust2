# [XL] moq-net beyond the codec

## Goal

rust2-go translates moq-net's model, as rust2-ts does, and its differential tests cover
each construct below against native Rust.

## Plan

The spike refuses each of these with its span (`go/fixtures/reject/expected.txt`):

- **`Drop`:** a `Drop` impl runs where Rust drops the value, in drop order, with drop
  flags where a value moves on some paths only. Translation now skips cleanup blocks, so
  a panic runs no drops, as `panic = "abort"` does.
- **Floats:** `f32` and `f64`, whose arithmetic Go matches; their formatting does not.
- **128-bit integers:** Go has none, so a runtime struct of two `uint64`s.
- **Trait objects** of traits other than `Display` and `Debug`, and unions.
- **Const generics,** such as an array's by-value iterator.
- **Pointers:** a wide pointer's `==`, which compares address and length, and any
  pointer's ordering, which Go lacks; and a constant allocation referenced at two offsets
  or types, which needs one Go variable and references derived into it by its layout.
- **`Vec::capacity`,** which Go's growth would make differ from Rust's.
- **std's collections:** `HashMap`, `BTreeMap`, `VecDeque`, translated from std's MIR or
  bound to the runtime as rust2-ts's are. The reject fixture holds no case for them yet.
- **Statics and pointers the fixture also refuses:** a `static mut`, a static with
  interior mutability such as an atomic, and `NonNull` ([go/README.md](/go/README.md)).

The translation targets the host, 64-bit `usize`; a 32-bit Go target needs its own
translation.

Decided with the maintainer on 2026-11-06: coverage waits on speed, as the epic orders
them, and splits into one quest per construct above only once speed reaches its bar. A
failed speed result deletes `go/`, so planning the constructs earlier would be wasted.
The last coverage child reruns `just go bench` against speed's bar, every `stream/`
workload within 2x of cgo, since constructs such as `Vec::capacity` change what the
stream workload runs. Speed reached the bar on 2026-11-07 in
[moq-dev/rust2#643](https://github.com/moq-dev/rust2/pull/643), but the epic is parked
until a consumer needs pure Go, so the split waits for that too.

- ✅ Gate on speed, split later.
- Split now, each child requiring speed.
- Split now, ungated.

## Waiting

- Go consumers: one needs moq in pure Go, which [the epic](/quest/m4/go/README.md) is
  parked until; check with the maintainer before starting.
