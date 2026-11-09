# rust2-go

A rustc driver that translates a Rust crate to pure Go: no cgo, one Go package, with the
semantics of the Rust source. It is a spike, measuring whether Go users are better served by
a translation than by the C ABI [rust2-ffi](../ffi/README.md) exports, on moq's codec.

```bash
rust2-go --out dir [--manifest-path Cargo.toml] [-- cargo args]
```

It translates the library of the package whose `Cargo.toml` is in the current directory,
or wherever `--manifest-path` says, to a Go package named after the crate. There is no
config: the crate's `pub` API is what Go callers reach, each item by the shortest path the
crate exports it at, in camel case, through its re-exports and the modules of other crates
it re-exports. A function is exported, `harness::run` as `HarnessRun`, a type is declared
with its `pub` inherent methods, `Counter::new` as `CounterNew`, and a type alias is a Go
alias, `type Byte = Value[uint8]`. An alias that is the only path to a type and passes it
just its own parameters names the type instead. A trait comes along as its impls are called.
A constant or static, associated ones included, is named with each upper-case word of its
own name capitalized, `MAX_LEN` as `MaxLen` and `Counter::MAX` as `CounterMax`. A
constant of an integer, `bool`, `char`, or `&str` is a Go `const`, a `&str` as a Go string,
and any other a function returning a fresh value at each call, as each use of Rust's is.
A static is one Go variable that translated code and Go callers share. Go has no read-only
variable, so Go callers must not write to a static, nor through what a constant or static
points to. A generic function, method, or associated constant, and a type with `pub`
methods that only a specialized alias reaches are refused.

It runs `cargo check` on the host with itself as `RUSTC_WRAPPER`, through the front end
it shares with [rust2-ts](../ts/README.md) in [`driver/`](../driver), building std from
source so every std function has MIR. From the `pub` API it translates every function
instance reached, monomorphized, and every type they name, into one `.go` file per crate
and the runtime in `runtime/`, then runs `gofmt`. What it can't translate is a compile
error at its span, and nothing is written.

## How Rust becomes Go

| Rust | Go |
| --- | --- |
| a function instance | a function, its MIR blocks structured into loops and `if`s as rust2-ts structures them, a `goto` leaving a block where JS would `break` it, its locals as variables but a temporary assigned and read once, which folds into its reader |
| `u8`..`u64`, `i8`..`i64`, `usize`, `isize` | `uint8`..`uint64`, `int8`..`int64`, `uint`, `int` (64-bit hosts) |
| an overflow check, a bounds check | native Rust's panic message, as a `RustPanic` |
| a struct, a tuple | a struct, generic where Rust's is, its `pub` fields exported (`count` as `Count`) and a tuple's as `F0` and so on; `tuple2[A, B]` and so on |
| an enum | a struct of its tag and a field per variant with fields; one of over four words, which Go would keep in memory, as its tag and each variant's fields, a variable, parameter, and result each, a field of over four words behind a pointer: an argument's the callee only reads, a result's the caller passes for the callee to write |
| `&T`, `Box<T>` | `*T` |
| `&[T]`, `&str`, `Vec<T>` | `[]T`, `[]uint8`, `[]T` |
| a constant | its value at each use; a reference's target, one package-level variable |
| a static | one package-level variable |
| a closure | a struct of its captures |
| `dyn Display`, `dyn Debug` | a `func(*Formatter) fmtResult` |
| a `tracing` event | nothing, as with no subscriber |

std translates from its own MIR, down to bindings for what Go does itself: a `Vec`'s
growth, a slice's iterator, UTF-8 validation, and the formatting machinery, which
`runtime/rt_fmt.go` ports from core so `{:?}` and `{:#?}` print as Rust's do.

A `Vec<u8>`'s storage is the runtime's own while only the translation reaches it: it
grows to twice its capacity without zeroing, through the runtime's `growslice`, and an
outgrown buffer is recycled for the next grow. Own storage is told apart by its shape, a
capacity that is a power of two of at least 8 and never filled. A root function, which
Go calls, takes Go's storage it is passed off that shape, a byte less capacity or a copy
when full, so it is never recycled, and clips own storage it hands back to Go, in its
result or behind an argument's `&mut`, to its length, so Go never sees past it and it is
never recycled either; Go's `append` to such a slice copies. It works on a copy of each
container an argument owns, a `Box`'s pointee or a `Vec`'s elements, so Go code still
holding one never sees what the translation moved or grew out of it: a `Box` Go passes
by value is left as it was, as Rust's move would leave it unread. Only roots are
exported. A root that reaches byte storage refuses a function pointer or trait object in
its signature, since Go code could fill it with a function that keeps what it is handed
or reads a place the translation is still writing; a raw pointer, which could see what
handing in copied; and a reference anywhere in an argument but its outermost type,
which the translation could move out of what handing back walks.

A panic is a Go panic of `RustPanic` carrying native Rust's message. As with
`panic = "abort"`, no cleanup runs on the way out, so a caller that recovers one must
drop what the translation held.

## Refused

`fixtures/reject/expected.txt` lists them with their spans: 128-bit integers, floats, a
`Drop` impl, a trait object of any trait but `Display` and `Debug`, a union, a type
generic over a constant, such as an array's by-value iterator, a `pub` item that is
generic, a `static mut`, a static with interior mutability, such as an atomic, or of another
language, a reference into part of a static, and, in the signature of a root that
reaches byte storage, a function pointer, trait object, raw pointer, or reference
inside an argument.

## Tests

`just go test` translates each fixture, runs `go vet` on it, and runs it against native
Rust on the same inputs, failing on any difference in output or panic message:
`fixtures/codec` is rust2-ts's harness over moq's codec, `fixtures/ints` every
integer width's arithmetic, overflow, division by zero, and shifts, `fixtures/order`
calls, reads, and overflow checks a folded temporary could reorder or skip, and
`fixtures/enums` enums held as their parts, whose pointers a callee could write through
or see changed, and `fixtures/hand` byte storage made, grown, and shrunk each way, which
`tests/hand/hand_test.go` then holds onto from Go as Go code may. `fixtures/items` also
checks its exported constants and statics as `tests/exports` reads them from Go.

`just go bench` times the codec's workloads translated against the same crate exported by
rust2-ffi and called through cgo: per-call varints and frame headers, and `stream/`, a
group's whole stream, the bar the translation is held to.
