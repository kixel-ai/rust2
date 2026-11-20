# rust2-ffi

A rustc driver that exports a crate's `pub` API as a C ABI and a C header, with no
attributes in the Rust source.
Every language's generator but Python's builds on the C ABI.

It targets facades written for the exporter's mapping rules. A stock crate's
whole public API is not generally exportable: generic and borrow signatures can
be refused, and the census also lists API items left out by those rules.
[Writing a facade](FACADE.md) goes from a crate to a facade with no refusals, and says
what each refusal asks you to change.

```bash
rust2-ffi (--out dir | --census dir | --api file) [--manifest-path Cargo.toml] [--target triple]... [--with file]... [--baseline api.json] [-- cargo args]
```

It runs `cargo check` on the package with itself as `RUSTC_WRAPPER`, for each `--target`
or else the host, whatever cargo's config names, through
the front end it shares with [rust2-ts](../ts/README.md) in [`driver/`](../driver). A build
script's probe of the compiler through that wrapper, as `rustversion` and `autocfg` make,
runs the `rustc` cargo resolved, the one the glue builds with, rather than the exporter's
nightly: an item a build script enables on nightly alone is exported only when that `rustc`
is a nightly. The package is the facade: once rustc has checked its library, it walks every `pub` item
reachable from the crate root, following `pub use` re-exports of other crates' items and
modules, and exports it by fixed rules, or refuses it with a compile error at its span.
So a facade such as moq-ffi exports several crates into one library by re-exporting what
it ships; a type a signature names that no path from its root reaches exports by its
public path in its own crate ([Rules](#rules)). The prefix of every C name
is the library's crate name, Cargo's `[lib] name`, which defaults to the package's name in
snake case. `--out` writes a crate of `extern "C"` functions over the facade, its runtime
beside it, and the header in `include/<prefix>.h`; `cargo build` on that crate makes a
static and a shared library for those targets alone: its build script refuses another, on
which an item the target compiles would be missing, or one it leaves out left over. So a
`cfg`-gated item works on its targets and on no other ([Targets](#targets)), and the
layouts, `usize` included, are the target's the crate builds for. It builds `--locked` against the `Cargo.lock` written beside
the manifest: the facade's, with what the crate adds of its own resolved in. If that moves
any package the exports build with from the version the check resolved, the export fails.
Its manifest declares the facade's `rust-version`. That crate enables the facade's features as the check did;
a feature named in a dependency, `--features <dependency>/<feature>`, is an error, since
the facade enables what its dependencies build with. It resolves them as the check did,
wherever it is built: its manifest carries, as a `[patch]`, each patch or `[replace]` cargo applied
to what the library builds with on any platform, read off cargo's resolution, wherever
it was declared (the manifest, cargo's config, `--config`, or the environment), keyed by
its source's URL.
A `[replace]` from the source it replaces, which no `[patch]` can carry, is an error.
Beside the C header, `include/<prefix>.hpp` holds the same API in C++, over a header per crate,
`kotlin/<prefix>.kt` holds it in Kotlin, over JNI functions the crate builds with
`--features kotlin`, and `kotlin/<prefix>.pro` the R8 and ProGuard rules keeping what
those functions look up by name. `python/<prefix>.py` holds it in Python, over a CPython
extension the crate builds with `--features python`, `swift/<prefix>.swift` holds it in
Swift, over the C header as a clang module, `go/` holds it in Go, a package per crate
over the C ABI through cgo, and `dart/<prefix>.dart` holds it in Dart, over the C ABI through
`dart:ffi` and what the crate builds with `--features dart`. When `wasm32-wasip1` is among
the targets, `web/` holds it in TypeScript for the web, ES modules over the `.wasm` the
crate builds with `--target wasm32-wasip1 --features web`.
An export again rewrites only the files whose content changed, and removes from
`include/<prefix>/`, `go/`, and `web/` only what it no longer writes, so a build of an unchanged
export stays fresh. The crate's version, `0.0.0+<hash>`, hashes its Rust sources and
manifest, so exports that differ build apart in a target directory they share.

It reads no config: the package is in the current directory unless `--manifest-path` says
where, and an item the rules cannot export fails the export at its span. Nothing leaves an
item out but the source itself: `#[doc(hidden)]`, or a `cfg` that leaves it out of a build.

## Rules

Every `pub` item reachable from the facade's root, by its shortest path, is exported or
refused. `#[doc(hidden)]` items are no part of the API and are left out; type aliases
export nothing of their own. A re-export of the standard library's item is refused, since
its types map by role. A `pub use` of a whole crate, as in `pub use rustls;` or
`pub use web_transport_proto as proto;`, is refused once at that `pub use`, since it puts
every release of that crate in the facade's API; a glob of its root, `pub use dep::*;`,
re-exports its items instead, which export.

A type an export's signature names that no path from the root reaches, as moq-net's
`dial_tcp` returns kio's `Pending`, exports by its shortest public path in its own crate,
`<prefix>::<crate>::<path>`, through that crate's modules and re-exports, with what its own
signatures name in turn. This holds whether or not the facade re-exports anything else of
that crate. The glue depends on that crate directly, as the facade resolves it; a crate
another of its name shares, the facade's root names, or another registry serves is not
reached. A crate only signatures reach keeps the roles of one no export is: its error in a
`Result` crosses as its message, and its type behind an `Arc` as an opaque handle, refused
bare. Its errors, the types implementing `std::error::Error`, are never reached, so an
exported error holding one is an object, as it was. A type it cannot reach, such as one no
public path names, or an error taken by value, is held by value as an opaque handle instead.

| Rust | C |
| --- | --- |
| a struct whose fields are all `pub`, not `#[non_exhaustive]`, which map, with no `Drop` of its own | a record: a struct, copied across |
| an enum with variants whose fields map, with no `Drop` of its own | a C enum without fields, or a struct of a tag and a union |
| any other type that is `Copy` and `Send`, with no padding under any order of its fields, such as moq's `stats::Traffic` | an opaque value: a struct of its bytes, copied across, which only Rust makes, reads, or changes; its `pub` fields are a getter and a setter, and a `&mut self` method writes back the caller's copy alone, one returning a future or a `Result` refused, since a language raising the error would lose the change; one no export returns is left out, since C could never make one |
| any other type | an object: an opaque handle to an `Arc`, freed with `_free`; one that is `Copy`, such as std's `Instant`, whose padding a `rustc` may place anywhere, is copied out of its handle where Rust takes it by value, and a closure neither takes nor returns one by value; one that is not `Send`, such as kio's `clock::Entered`, stays on the thread that made it, as [the C ABI](#the-c-abi)'s threads say |
| a type whose only generic parameters are lifetimes, such as `Path<'a>` | an object holding its `'static` form |
| a generic type, at each instance it infers | that instance as any other type, with each method whose impl and own bounds it meets; any other instance is refused |
| a `pub` field of an object or opaque value | a getter returning a copy, and a setter assigning in place, as `x.field = value` does |
| a `pub` trait | a callback interface: a table of C functions and their data; a trait C cannot implement, such as one with a blanket impl, an associated type, or a supertrait but a marker, is left out, since C never passes one, and a signature taking one from C is refused |
| a type parameter or `impl` bounded by `Fn`, `FnMut`, or `FnOnce` alone, taking what a callback takes, objects by reference, or opaque values, and returning a scalar, a `String`, an opaque value, an `Option` of one, or a `Result` of one and a `String` | a C function and its `void *` data; one a lifetime bound or the return lets outlive the call, as kio's `clock::drive` and a spawned task's are, adds a `free` Rust calls once it drops it; a closure of any other signature is refused |
| a type parameter or `impl` taken by value and bounded by `Future` or `IntoFuture`, whose every bound `Pin<Box<dyn Future<Output = ()> + Send>>` meets, as kio's `clock::timeout` takes, or taken as `Pin<&mut F>` by a poll, as kio's `Waiter::poll_future` takes | `<prefix>_foreign_future`: a future of `()` C implements, which Rust takes as that type, or which the future the poll makes owns and pins; a future of another output is refused |
| a type parameter or `impl` bounded by `Read` and `Seek` alone, behind `&mut`, which no lifetime bound or return lets outlive the call, as moq-mux's `container::mp4::repair` takes | `<prefix>_foreign_reader`: a `read` and a `seek` C implements and their data, lent for the call; any other reader is refused |
| `fn` and inherent `pub` methods | functions named through their type |
| an object's `Deref` to an exported type, followed as far as the chain reaches | the target's `pub` methods and fields, a record's too, on the object, as auto-deref finds them: a method the object or a nearer target has, exported or not, hides a method, and a field a field; a `&mut self` method or a setter only while every `Deref` on the way is a `DerefMut`; and the census names each by the object's path |
| a `()` parameter of a function, not of a callback's method | nothing: C passes no argument for it |
| `Result<T, E>` | `bool f(..., T *out, E *err)`; an `E` of another crate that implements `Display`, such as `anyhow::Error` or `io::Error`, is `String`, its message; a `()` error, as once_cell's `race::OnceBool::set` returns, is refused, having no value to raise |
| a returned tuple `(A, B)` | one out pointer each: `void f(..., A *out0, B *out1)` |
| a tuple inside another type, or as a field or parameter | `<prefix>_tuple_<A>_<B>`: a struct of fields `_0`, `_1`, and so on; of references only returned |
| `async fn`, or a return that implements `Future` | a future: `f` makes it, `f_poll` polls it, `f_future_free` drops it, and `f_start` makes it and polls it once, returning a handle only if it pends; a function that is not `async` runs at the call, as in Rust, and when its future borrows the arguments, the future owns them as an `async fn`'s does, holding each in place until it drops |
| a function returning `Poll<T>` that polls through a `&Context`, a `&mut Context`, or a reference to an object exactly one associated function makes from a `Waker` alone, of a type with no generic parameters, such as kio's `Waiter`, which may be the method's own `&self`, as `Waiter::poll_future`'s is | a future of `T`, as an `async fn`'s: each poll calls the function again with the poll's context, or with an object made from its waker and kept until the next poll, so what it registers stays registered; each other parameter taken by value must be `Copy`; a method polling through its own `&self` is a function without one |
| `poll_x`, when `x` exports as a future | left out: the future is its poll |
| `bool`, integers but 128-bit, floats, `char` | `bool`, `<stdint.h>` types, `float`, `double`, a `uint32_t` code point |
| `u128` | `<prefix>_u128`: its low and high 64 bits |
| `NonZero<T>` of an integer but 128-bit | `T`, in every binding but Swift, whose runtime `NonZero<T>` holds no zero; a zero passed in aborts in C and C++, and Kotlin, Python, Go, and Dart check it first and raise their error. Its option and list are its own, `<prefix>_option_non_zero_<T>` beside `T`'s |
| `[u8; N]` | `<prefix>_array_u8_<N>`: a struct of its `N` bytes, `items`, copied across; an empty one is refused |
| `[T; N]` of any other `T`, returned or a constant's value | `Vec<T>` of its `N` items; taken, it is refused |
| `&str`, `String`, `&[u8]`, `Vec<u8>`, `bytes::Bytes` | `<prefix>_buf`: an owned buffer returned; passed in, a view, or a buffer Rust returned, which Rust takes back without a copy unless it sits behind a pointer or in an array; `<prefix>_buf_clone` shares one the caller keeps, which a `Vec<u8>`, `String`, or `PathBuf` parameter then copies, since only a sole owner's allocation becomes a `Vec` |
| `&Path`, `PathBuf` | `<prefix>_buf` of the path's bytes as the OS holds them: on Unix any, on Windows WTF-8, which holds a name that is not UTF-16, else UTF-8 |
| `Duration` | `<prefix>_duration`: its seconds and nanoseconds; a billion nanoseconds or more passed in aborts |
| `SocketAddrV4`, `SocketAddrV6` | `<prefix>_socket_addr_v4`, `<prefix>_socket_addr_v6`: its IP, port, and for IPv6 its flow label and scope, so each holds its family alone |
| `SocketAddr` | `<prefix>_socket_addr`: `v6`, and a union `addr` of the two |
| `Ipv4Addr`, `Ipv6Addr` | `<prefix>_ipv4_addr`, `<prefix>_ipv6_addr`: its bytes in network order |
| `IpAddr` | `<prefix>_ip_addr`: `v6`, and a union `ip` of the two |
| `url::Url` | an object `<prefix>_Url`, made by `<prefix>_Url_parse`, which fails with the parse error's message, and printed by `_to_string` |
| `Arc<T>` of another crate's `Send` and `Sync` type that no other rule maps, such as `Arc<rustls::ServerConfig>`, or `Arc<dyn Trait>` and `&'static dyn Trait` of another crate's trait without generic arguments, whose auto traits are `Send` and `Sync` but those the trait implies, borrowing nothing C lends, as rustls's `Arc<dyn ResolvesServerCert>` | an opaque object named by `T`'s crate and name, such as `<prefix>_rustls_ServerConfig`, with `_clone` and `_free` and none of `T`'s API: it only passes back to Rust |
| another crate's `Send` and `Sync` type taken or returned by value or lent, with no type parameters, that no other rule maps, that is no iterator or future, and that is not reached by its public path, such as one only a `#[doc(hidden)]` module names, or a standard library type no other rule maps, such as `io::Error`, named by its full path, `<prefix>_std_io_Error` | an opaque object named as the `Arc` form is, with `_free` and none of `T`'s API, holding the value: Rust takes it back by value, or borrows it; a type also named behind an `Arc` is that form, and refused by value |
| a type implementing `PartialEq` and `Hash` | beside `<T>_eq`, `uint64_t <T>_hash`, its `Hash` through std's hasher with its fixed keys, so equal values hash alike in every binding |
| a type implementing `PartialOrd` with itself | `int8_t <T>_partial_cmp(a, b)`: -1, 0, or 1 as `a` is less than, equal to, or greater than `b`, or 2 when the two are unordered; with `Ord`, `int8_t <T>_cmp(a, b)` too, -1, 0, or 1. Every binding's `<` and its kin call `_partial_cmp`, as Rust's do, and its total order, where it has one, `_cmp` |
| `std::time::Instant` | an object `<prefix>_Instant`, which only Rust makes, copied into each call that takes it and compared by `_eq` and ordered by `_cmp`, with none of its methods |
| `tracing::Level` | `<prefix>_Level`: a C enum of its five levels, `TRACE` to `ERROR`, ordered by tracing's `Ord`, the more verbose after the less |
| `tracing::level_filters::LevelFilter` | `Option<tracing::Level>`, whose `None` is `OFF` |
| `serde_json::Value` | an object `<prefix>_Json`, made by `<prefix>_Json_parse` from JSON text, which fails with serde_json's message, and read back by `_to_text`; its contents are the caller's JSON library's to read |
| `Vec<T>`, `&[T]`, and `Box<[T]>` of any other `T` | `<prefix>_vec_<T>`: an array of `T`'s C values, viewed passed in and owned returned, a slice as a clone; of references only returned; a boxed slice of bytes is refused |
| `BTreeSet<T>`, `BTreeMap<K, V>`, and `serde_json::Map` | as `Vec<T>`, `Vec<(K, V)>`, and `Vec<(String, serde_json::Value)>`: its items in order, from which Rust builds it, keeping the last of a key |
| a returned iterator, such as `slice::Iter` or an `impl Iterator`, that no other rule maps, outside a future, made of core's and alloc's iterators over a slice, string, `Option`, `Result`, or collection, or serde_json's `map::Iter`, `IntoIter`, `Keys`, or `Values`, through `rev`, `chain`, `flatten`, `fuse`, `enumerate`, `peekable`, `copied`, `cloned`, or `filter`, with no trait object inside; a closure or function inside, such as `filter`'s predicate or `split`'s pattern, must drop nothing, and a `filter` only items whose drop runs nothing | what `collect::<Vec<_>>()` makes of it: every item, taken at the call, so a closure inside runs on every item, not only those a caller stopping early would reach |
| a returned `impl Trait`, or one a returned future outputs or a collected iterator yields, other than that future or iterator itself, whose hidden type is an object | that object's handle, which keeps the `impl Trait` itself, so every lifetime it captures must be `'static`: one capturing a borrow, as `impl Clone + '_` does, is refused, and `+ use<>` captures none; one hiding any other type, such as an `Option`, is refused, since the glue cannot name it |
| `Waker`, taken by value | `<prefix>_waker`, as a future's poll takes it |
| a returned `Waker` or `&Waker`, or a returned `Context`, which is only a waker lent for a call | `<prefix>_waker` of a clone of the waker, or the context's: the caller's, to wake through Rust and drop once |
| `&Context` or `&mut Context` of a function returning no `Poll` and no future | `<prefix>_waker`, which the context is made of for the call |
| `Range<T>`, `Bound<T>` of an integer | `<prefix>_range_<T>`, `<prefix>_bound_<T>`: a `<prefix>_bound` kind and a value |
| `impl RangeBounds<T>` of an integer | `<prefix>_bounds_<T>`: its start and end bounds |
| `Option<T>` | a null handle or buffer, or `<prefix>_option_<T>` |
| a `pub const` | a `#define` if a `bool` or integer, else a function returning it |
| `impl Into<X>`, `impl AsRef<X>`, or a bound implying one, as a supertrait does | `X`, or `&X` and then `String` or `Vec<T>` where `X` is `str` or `[T]` |
| `impl Into<X>`, where `X` does not map | what a `From` implementation into `X` takes: a source parameter by its bounds as above, so `impl<B: Into<Bytes>> From<B> for Timed<Bytes>` takes `Bytes`, and by each `Into<Y>` whose `Y` does not map, what converts into `Y` in turn, each `Y` once and refusing a chain longer than the crate's recursion limit; any other source as it is, so `impl<'a, V> From<&'a V> for Timed<&'a V>` takes `&V`. Two sources C passes as one buffer, such as `String` and `Vec<u8>`, are refused, unless they are forms of one value, as `&str` and `String` are |
| `impl IntoIterator<Item = X>`, where `X` is no type parameter | `Vec<X>`, or `&[Y]` where `X` is `&Y` |
| `impl io::Read`, not bounded by `Seek` | `&[u8]` |
| `impl Iterator<Item = X>` taken | `Vec<X>`, whose items Rust iterates; for `X` of `&str` or `&[u8]`, an array of buffers C lends for the call; an iterator of other references, or one a future or a callback takes, is refused |
| another type parameter | `serde_json::Value` for a serde `Serialize`, `Deserialize`, or `Deserializer` bound, as a derived `deserialize` takes, else the first of `String`, `Bytes`, and `Vec<u8>` meeting its bounds, or a foreign implementation of its one exported trait; else an instance of the function at each type the exports implement a trait bounding it for, through a blanket impl at each type implementing what the impl requires, as `&X` where that meets its bounds too and no bound, its own or one a supertrait implies, makes it outlive the call, so C keeps its handle, and, as an argument of a generic type, at each argument of an instance that meets its bounds and makes each generic type the signature names an instance |
| a const parameter of a function | an instance of the function at each value an instance of a generic type the signature passes it to has there, where each generic type the signature then names is an instance |
| a const argument | a `bool` or an integer below 128 bits; any other is refused |

A generic type infers the instances it exports at, with no config: the one at its
defaults, if each type and const parameter has one, as `frame::Buffer` at `N = 8`; the
one each inherent impl is of, such as `impl Pair<u32>`; each choice, for each parameter,
among its default and the types the exports implement a trait bounding it for, one of
the exports', which are `()`, a primitive, `String`, or an exported type or an instance
of one, such as `impl Seed<u64> for Wrap<u32>`; and each instance a signature, `pub`
field, or constant names at any choice its bounds allow, whether or not the rest of it
maps, which may name more. Inference finishes before any type is mapped, so mapping
never adds an instance. An instance that holds an instance of a type inside it, such as
`Cell<Cell<u8>>`, is refused, as it could name ever deeper ones.

A field holding a reference, as `&'a mut ClientConfig` in rustls's
`DangerousClientConfig` is, is read through it, lent where it is not `Clone`, and has no
setter, since no borrow C holds outlives a call. A returned `Cow<'_, str>` or
`Cow<'_, [u8]>` crosses as the string or bytes it owns, a copy C would make of a borrow
anyway; a `Cow` anywhere else is refused, as C would hand Rust its form. A future owns
its output before it ends, so it may return a `Cow` only on its own, in an `Option`, or
as a tuple's element, and an error's only as `Cow<str>`, whose message crosses. Another crate's
type that only refused items name, as `std::net::TcpListener` in moq-tokio's
`server::Streams` is, is one no export returns, so those items are left out with it.

A record or enum holds only values and `Copy` handles: one with a field that does not map,
or that holds itself, is an object instead, and the census lists it. A borrowed return is
returned as a clone, so it must be `Clone`, and a handle in it is a new handle to the
clone, but for an object a `&self` method of an object returns borrowed from its receiver
alone, or an object's `pub` field holds, bare or as an `Option`'s `Some`, through a `Box` too: that one, if not `Clone`, is a lent handle, which holds its receiver and checks it
on each use, on one thread at a time where the receiver is not `Sync`. A handle outlives every call, so every lifetime an
object takes is `'static`, as is every lifetime that must outlive one; any other lifetime
is the call's. A signature may not lend C's borrow for one that is `'static`, but for a
`&str` or `&[u8]`, which Rust interns, nor return an object that borrows, but for an
exclusive lend: a `&mut self` method of an object returning an object, bare or as a
`Result`'s `Ok`, that borrows the receiver's lifetime alone and is not `Copy`, as a lock's
guard does, which its handle holds with the receiver's exclusive borrow; a copy would
carry the borrow past it. Each of its `Clone`, `PartialEq`, `Hash`, `PartialOrd`, `Ord`,
`Debug`, `Display`, `Deref`, and `DerefMut` must hold at every lifetime, as the binding calls them on the
`'static` form, and its `Deref` target must name no lifetime. So must each of the
method's own bounds naming the receiver's lifetime, as the binding's lend gives the call a
borrow of any lifetime. A parameter is a reference
only at its top or as `Some` of an `Option`, lent as the reference alone is, so
`Option<&T>` is a null or borrowed handle, a null or viewed buffer, or an option of a
value, and not inside a `Box` or any other type. A `&mut` of a value, but a `str` or a
slice, is in and out, and not inside an `Option`, a callback, or a future, nor borrowed
by what the call returns; a `&mut` of another crate's `Arc` is refused. Only a bare
returned `&Object` is lent, never one inside what a call returns nor its error. What C could never hold, an
object no export C can call returns or lends to a closure, a generic type no export
instantiates, a type parameter bounded by another crate's trait no type C holds implements,
and whatever takes one, is left out and listed in the census. Everything
else is refused, and the census counts why.

## Targets

One binding serves every target its package ships to, such as one Kotlin file for each
Android ABI and the desktops, though their APIs differ by `cfg`. Each `--target` is
checked in turn, and the binding holds every item any of them has. On each target, every
item that target compiles works exactly, and no other does:

- A function, method, or constructor only some targets have is declared everywhere. On
  another target, the library defines it to fail, and the binding checks the library's
  target before calling it, raising the language's own error, as for a closed object:
  `UnsupportedOperationException` in Kotlin, `NotImplementedError` in Python, a
  `CheckError` thrown in Swift, an error matching `errors.ErrUnsupported` returned in Go,
  and `UnsupportedError` in Dart. C and C++ abort with the message. A type's std traits
  are its methods here, and its `Default` a constructor, except where the language's own
  protocol cannot fail: Swift traps in a `description`, `debugDescription`, or `==` that
  does not throw, and Go's `String` and `GoString` return the error's text.
- So does a variant only some targets have, when a value holding it crosses into a target
  without it, except that Swift traps and Go panics, as a value's conversion there returns
  no error. A record's fields cross whole, so one whose fields differ between targets is
  refused, as below.
- A type only some targets have is declared everywhere, and only those targets' items
  make or take it.
- A constant only some targets have keeps its value everywhere, since reading it calls
  nothing.
- An item whose definition differs between two targets, such as a function whose
  signature does, a record with a field only one has, a plain enum whose variant has
  another value, or a constant of another value, is refused at its span, with a note at the other's, naming both targets.

The crate's build script sets `cfg(rust2_ffi_target = "<triple>")` for the target it
builds for, which gates each item, and refuses a target it was not exported for.
`<prefix>_target()` returns the library's target as its index among the header's,
listed in its comment, which each binding reads once when it loads.

A target's check may need its own C toolchain, as moq's `aws-lc-sys` and `ring` do even
for a check, which only its own host may have. `--api file` writes one target's API, with
the spans of its items, instead of exporting; an export elsewhere adds it with
`--with file`, as though it had checked that target too. Both must check the same
package and features, from the same sources: a staged span's file is relative
to the workspace's root or cargo's home, so a checkout at another path keeps it, and is
dropped for the other target's span where the file differs.

## The API lock

Each export writes its API, merged across every target it serves, to `api.json` in `--out`,
with the staged API and each binding's name table. `--baseline file` fails the export, writing nothing, where its
API breaks the one in `file`, with an error at each item that names how:

- an item removed, or no longer on one of the baseline's targets, and a target the baseline
  serves that the export does not;
- a function's receiver, parameter, return, error, or asyncness changed, a constant's type
  changed, a trait method added or changed, or a trait impl such as `Clone` dropped;
- a record's or variant's field added, removed, moved, or of another type, a variant
  removed or of another value or, in an enum with fields, at another place, which is its C
  tag, one added to an enum another crate matches exhaustively, and an enum gaining its
  first variant with fields or losing its last, which changes it between a C enum and a
  struct;
- whether a type's handle is checked at run time, which gives or takes a status from each
  call taking it, and makes it throw in Swift: borrowing it exclusively or moving it out in
  an added method makes it checked;
- the library's name, which every C name starts with;
- a C name that now names another item: since a name depends on every item exported
  ([The C ABI](#the-c-abi)), adding `audio::encoder_finish` renames `audio_encoder::finish`
  to `moq_audio_encoder_finish_`, and a C caller of the old name would call the new item.
  A tuple's struct, known by its Rust spelling, is renamed the same way when another
  tuple's elements run together into its name.

An item is known by its kind and Rust path. An addition passes: a new
item, method, trait impl, or a variant of a `#[non_exhaustive]` enum. A constant's value and
an item's docs may change. The check runs once, on the API every language is generated from;
it compares C's names and the C++, Kotlin, Python, Swift, Go, and Dart generators' own
public names. A clash that renames a kept class, function, or member reports its language
and old and new names at the item's span. An error type's exception class is known by its
error type and the type the language holds it as, since Kotlin's `UShortArray` and
`List<UShort>` may share one error type: one added beside another adds a class, and a
native type respelled or gone breaks nothing while a class still bears its old name. Staged `--api` files have no binding names; a
baseline must be a complete export. An older baseline without name tables still has its
structure checked, reporting each break before the error that it lacks names, so its
re-export with this version accepts no break that check would have caught. A binding
rename that follows from a C rename is reported once, as the C rename.

The baseline is a lockfile, not config: the consumer commits it beside `Cargo.lock`, the
export reads it only to compare, and the export is the same with or without it. It refuses a
baseline that is the `api.json` it would write. To accept a
break, export without `--baseline` and commit the `api.json` it writes. Its spans are relative
to the workspace and cargo's home, as a staged API's are, so a removed item's error points
where it was while its file is unchanged.

The lock compares the API and the names each generator gives it, not the signatures each
language renders, so a generator change alone, such as a Kotlin type for a Rust one, passes
it. rust2 locks those too: [`surface/`](surface) holds each binding's API as its language
renders it, for the api fixture and moq's stand-in, and `just ci` fails where a generator
renders another. `just ffi surface` rewrites them; a PR changing any binding's API commits
the diff. Each is the language's own reading where one needs no build: Python's `ast` for
the `.pyi` a type checker sees, `go doc -all` for Go, `tsc`'s declarations for the web
binding. C, C++, Kotlin, Swift, and Dart are an outline of the generated source: each
public declaration up to its body, without comments or private members. The files are x86_64
Linux's exports, as the merge queue runs on, since a header names its targets.

## The C ABI

- **Handles** are `Arc`s, and a call borrows the value as Rust's receiver or parameter
  does, checked at run time: shared for `&T`, exclusive for `&mut T`, and moved out for
  `T`, after which the handle is only freed. A `Copy` object, one whose bytes its value
  decides are initialized, as an `Option<u32>` in it does, is copied out instead, leaving
  its handle as it was. A
  value that is not `Sync` is shared on one thread at a time, and held exclusively by a
  future, which may be polled on any thread. A borrow Rust would refuse at compile time,
  or a moved-out handle, fails the call before any Rust code runs, so it changes nothing,
  and frees each buffer the caller handed it, as a call that ran would:
  a function whose handles can fail a check takes a trailing `<prefix>_status *status`,
  through which it writes the `<PREFIX>_STATUS_*` that failed, returning a zeroed value,
  and `<prefix>_status_message` names it. A call that passes its checks leaves `*status`
  as it was, so a caller zeroes it before each call it reads it after. A NULL `status` aborts with the message, naming
  the function, as C++ passes; each other binding raises its language's own error, as
  below. A null handle aborts. Only a type some export can misuse is checked: one that is
  not `Sync`, or that an export borrows as `&mut`, moves out, or lends a closure; a call
  borrowing none of those takes no `status`, and neither does `_free`. A checked type also
  has `_drop`, which drops the value at once, as a move out would, and leaves the handle to
  `_free`: a later call fails with `<PREFIX>_STATUS_MOVED`, a second drop does nothing,
  and one while a call or future borrows the value fails with
  `<PREFIX>_STATUS_BORROWED`. A record's or data enum's `_eq`, `_to_string`, and `_debug`
  take a `status` too when it holds a checked handle. A setter borrows as `&mut T` and takes
  its value as `T`, so it writes the value the handle holds, or the one Rust lent it,
  while a getter's copy is C's own: a nested field is got, set, and set back. A type
  whose C++ copies share one value has `_borrowed`, true while a call or future borrows
  it, so moving it out would fail.
- **Threads**: a value that is not `Send`, an object or a future, stays on the thread that
  made it, which alone borrows, moves, polls, or drops it. A future is such a value if it
  holds one, as a poll function's holds its receiver and parameters, or makes one, as its
  output, which its last poll makes on its thread. A call or poll on another
  fails with `<PREFIX>_STATUS_NOT_SEND` before any Rust code runs, so such a future's poll
  takes a trailing `status` too. A `_free` or `_drop` elsewhere drops nothing there: it
  queues the drop to the value's thread, which runs it at its next check of one of its
  values or as it exits. A thread running a loop that may call nothing for a while
  passes `<prefix>_thread_wake` a waker, which such a free wakes so the loop schedules
  `<prefix>_thread_drain` there. One freed once its thread exited leaks, printing why, since
  nothing may drop it. A function that makes such a value, taking none or only in an
  `Option`, has a twin `<name>_homed` of the same signature, as do its future's start,
  `<name>_start_homed`, and `Default`'s, `<T>_default_homed`: with no such value passed,
  it runs on a home, a thread Rust owns, which the value then stays on, and each later
  call on it, from any thread, runs there while the caller waits, as does a call on a
  value that is not `Send` it returns; with one passed, it runs as the function does. The first present of
  a call's receiver and its parameters that are handles of such values, bare or in an
  `Option`, sends it to that value's home, if it lives on one; a value made on a thread
  the caller owns is not sent, so a call on it from another thread fails its check. Values from different homes, or one
  nested in a list or a record, are not sent: the call runs on the first's home, where
  any other fails its check with `<PREFIX>_STATUS_NOT_SEND`. A home waiting on a call into
  Dart, or on another home, runs meanwhile what that call asks of its own values. A
  binding whose threads move calls the twins: Go and Dart for every such value, Kotlin
  and Swift for such a future made from none, which a pool's thread then polls.
- **Opaque values** are `typedef struct <T> { uintN_t opaque[n]; } <T>;`, the bytes of a
  `Copy` type with private fields, in units of its alignment up to 8 bytes: C copies one
  as any struct, and passes `&self` as a `const <T> *` and `&mut self`, a field's setter
  included, as a `<T> *` Rust writes what the call left back through, so only that copy
  changes; it never makes or reads one itself, and frees none. Its fields fill it under
  any order a `rustc` may give them, with a byte for an enum's tag where no field has a
  niche to hold it, and a variant without data in another's niche counted as that niche's
  whole scalar, which it writes, so it has no padding whichever `rustc` builds the glue,
  which asserts each target's size: `None` of an `Option<NonZero<u64>>` fills its 8 bytes.
  They must also fill a model layout at both 4- and 8-byte pointers, each primitive
  aligned to its size and a `repr`'s field order kept, so every target decides alike. Any
  other `Copy` type, such as std's `Instant`, padded on a 64-bit target, or an
  `Option<u32>` or `Option<&[u8]>`, whose `None` leaves bytes uninitialized, is an object
  instead, so no byte Rust leaves uninitialized reaches a language, which may serialize
  what it holds. Its size and alignment are the largest of the export's targets, so one
  header serves them all, a few bytes past a 32-bit target's own. A change of size is an
  ABI change, as a record's field change is.
- **In and out**: a `&mut` of a value is two parameters, the value passed in as one taken
  by value is, and a `<name>_out` pointer, through which Rust writes what the call left in
  it on every path, its errors included. C keeps what it passed, and frees what Rust wrote
  as a returned value. C++ takes a reference, Swift an `inout`, and Go a pointer, each
  assigned what Rust wrote; Kotlin, Python, and Dart take a `Mut` holding the value, whose
  `value` Rust's write replaces, before any error raises.
- **Lent handles** borrow from the handle a call lent them from: each use holds that one
  shared and fails with `<PREFIX>_STATUS_OUTLIVED` once it was since borrowed exclusively,
  moved out, or freed, which Rust's borrow checker would refuse. Borrowing a lent handle
  exclusively, or moving it out, fails with `<PREFIX>_STATUS_LENT`, as Rust lent it.
- **Exclusive lends**, a `&mut self` method's returned object that borrows its receiver,
  such as a guard, are owned handles that hold the receiver borrowed exclusively until the
  value drops: by `_drop`, by `_free` of the last handle, or by a call it moves into, as
  `commit(self)` does, which gives the borrow back once the call returns. Until then the
  receiver fails each call with `<PREFIX>_STATUS_EXCLUSIVE` or `<PREFIX>_STATUS_BORROWED`,
  and its `_free` leaves the value to the lend. A clone shares the borrow, which the last
  copy gives back. A call that could keep the value past it or carry its borrow out fails
  with `<PREFIX>_STATUS_LENT`, as does a move into anything but a plain call: one taking an
  object whose lifetime is `'static`, or ties it to another argument or the return, itself
  or through the signature's bounds, and a field's getter or setter where the field holds
  what borrows.
- **Interned strings**: a `&str` or `&[u8]` a function needs for `'static`, as
  `Path::from_static` and `CertificateDer::from_slice` do, is leaked once per distinct content and reused for every later equal one, so a
  caller passing ever-new strings grows the set without bound.
- **Arrays** of a `Vec` are a pointer, a length, and an owner, as a buffer is: passed
  in, Rust copies each element out of C's array, or for a slice of a fixed-width integer
  other than `u8`, which is bytes, or `u128`, or of a float, borrows C's array in place for the call, and a binding passes its language's
  array of those numbers as one block; returned, `<prefix>_vec_<T>_free` frees
  the array and its first `len` elements, so a binding that takes the elements sets
  `len` to 0 first. A null pointer is `None`, so C passes a non-null one for `Some` of
  an empty array.
- **Values** are copied in from the caller's memory, which Rust never frees, and lowered
  into memory the caller frees once with the type's `_free`.
- **Futures** run no runtime: C polls with a `<prefix>_waker` and polls again when woken.
  A poll that finishes writes the result through the poll's own out pointers, so a
  cancelled read never loses a value. The future reuses the last waker when given the same
  one again, and owns what it borrowed: the handle it was called on stays borrowed until
  it finishes or is freed. `_start` is the call and a first poll with no waker in one:
  a future ready at once costs one call and no handle, and one that pends writes its
  handle through its `_future` out pointer, which must not be null: the last parameter, or
  the one before a checked call's `status`. A null waker wakes nothing
  and costs no allocation, and a future is one allocation.
- **Wakers** move into Rust, which calls `drop` once it lets go of the last clone, so a
  state machine polled with a waker, such as moq's `Driver`s through kio's `Waiter`, runs
  from the caller's event loop. Rust may call `wake` and `drop` from any thread, as a
  runtime's workers do. A waker Rust returns is the caller's, which may wake it from any
  thread any number of times, then drop it once, or pass it back into Rust, which wraps
  it as any waker.
- **Foreign futures** are a `poll`, a `free`, and their `data`. Rust calls `poll` from any
  thread, one call at a time, with a `<prefix>_waker` of its own each time, which C owns
  and drops once, until it returns true; C wakes the latest waker once the future can
  progress. Rust calls `free` once it drops the future, done or not, which may be before
  any poll. A null `poll` aborts.
- **Callbacks** are a struct of a table and its data. Rust calls `free` once it drops the
  implementation, and may call the others from any thread. A callback takes scalars and
  buffers lent for the call, and returns a scalar.
- **Closures** are a function and its data, which Rust calls from any thread, one call
  at a time. One lent for a call Rust calls only during the call; one Rust keeps comes
  with a `free`, which Rust calls once it drops the closure, done or not, maybe before
  any call. One takes what a callback takes, objects by reference, each lent as a handle
  to the value where it is for that call, and opaque values, copied: C borrows a lent
  handle no further than Rust lent it, and moving out of it, freeing it, or keeping a
  borrow past the call aborts. What it returns is Rust's to own: an opaque value, copied
  as Rust copies it without running its `Clone`, or a buffer `<prefix>_buf_copy` made; a
  `Result` returns false and writes its error's buffer.
- **Readers** are a `read`, a `seek`, and their data, lent for the call, which Rust calls
  from any thread, one call at a time. `read` writes at most the length it was given,
  `seek` takes `SEEK_SET`, whose offset's bits are a `uint64_t` that may pass
  `INT64_MAX`, `SEEK_CUR`, or `SEEK_END`, and either fails by returning false
  with a buffer `<prefix>_buf_copy` made, which Rust raises as an `io::Error` of that
  message. A null function, or a read past the length, aborts.
- **The contract** is a hash of the header's declarations, its docs included: the header
  defines it as `<PREFIX>_CONTRACT`, and `<prefix>_contract()` returns it from the library
  built with the header. A binding checks the two match before any other call, so a header
  and a library from different builds refuse to mix.
- **Panics** abort, as an `extern "C"` function's do.
- **Must-use returns**: a function returning a handle, a buffer, a waker, a value holding
  any of those or a `Vec`, such as a record with a `String` field, a poll, or a fallible
  call's `bool` is `<PREFIX>_MUST_USE`, C23's and C++17's `[[nodiscard]]`, or
  GCC's and Clang's `warn_unused_result` before C23, since dropping it leaks or loses an
  error. Docs are `/** */`, which Doxygen reads.
- **Android**: the build script links the library with 16 KB pages, which Google Play
  requires of a 64-bit library from 2027-02-01 and an NDK before r28 does not do itself. A
  panic or a failed check that aborts goes to logcat as `rust2-ffi` and to bionic's abort
  message, which shows in the tombstone, `ApplicationExitInfo`, and Play vitals, since an
  app's stderr goes nowhere.
- **Names** are the prefix, then the item's path from the facade's root, each segment in
  Rust's case, joined by `_`: `track::Producer::append_group` is
  `moq_track_Producer_append_group`, and moq-net's `session::Stats` and `Session::stats`
  are `moq_session_Stats` and `moq_Session_stats`. Items of other crates are named by
  where the facade re-exports them, so two of one name need two paths. A field's getter
  and setter add `_get` and `_set` to its path, as SWIG's do, so they stay apart from a
  method of the field's name and from a `set_` method, which Rust names verb first:
  moq-tokio's `Backoff::initial` field, which moq's facade re-exports in `tokio`, is
  `moq_tokio_Backoff_initial_get` and `moq_tokio_Backoff_initial_set` beside the method
  `moq_tokio_Backoff_initial`. An enum variant's
  payload is `<enum>_<Variant>`, and its tag and a constant are the name in upper case.
  Joining still loses where segments split, and the runtime shares the prefix, so every C
  name is registered, and only names that clash change. An item whose name matches another's,
  or one the runtime or a type's own functions take, such as a method `free` beside the
  type's `_free`, takes a trailing `_`, as do the names made from it: `audio::encoder_finish`
  and `audio_encoder::finish` are `moq_audio_encoder_finish` and `moq_audio_encoder_finish_`.
  The runtime's names keep theirs first, then names made from an item, then items' own, each
  by its kind and Rust path. A name depends on every item exported, so one an upstream
  change adds can lengthen another's.

  An instance is named by its generic item's name, then each argument but those at their
  defaults after the last that is not, so the instance at its defaults is named as the
  item. Each argument takes the last segment of its C name, such as `Extra` for
  `mux::catalog::hang::Extra`, so `mux::catalog::Producer<mux::catalog::hang::Extra>` is
  `moq_mux_catalog_Producer_Extra`. Where two instances of one item would still share a name,
  each argument that tells them apart takes one more segment at a time, until they
  differ: the api fixture's `Counter::seeded` is `api_Counter_seeded_Point` at its
  `Point`, and `api_Counter_seeded_polar_Point` at `polar::Point`. The names depend on
  every instance the item exports at, so one an upstream change adds can lengthen another's
  name. A type instance's methods are named through it, and a function instance appends
  its arguments' names, a reference's by what it borrows and a const's by its value: the
  api fixture's `drain` is `api_drain_4` at `Batch` and `api_drain_2` at `Batch<2>`.

## C++

`<prefix>.hpp` is header-only C++ over the C header, for C++17, 20, and 23, with
exceptions and RTTI on or off: it throws nothing and reads no type information. It
includes a header per crate whose items the facade exports, `<prefix>/<crate>.hpp`, which
a translation unit may include alone to pay only for that crate and those it names: moq's
`#include "moq/net.hpp"` parses in under half the memory of `moq.hpp`. A crate's header is
named as [its Go package](#go) is, so none is the shared `rt`. Each includes the shared
`<prefix>/rt.hpp`, which holds the runtime and declares every type, so a class declares a
method that belongs to a later crate, as an instance naming that crate's type does, and
that crate's header defines it; and the headers of the crates its items name, each before
it in Cargo's order, so none includes another in a cycle. An item's crate is the one Go's
packages place it in.

- **Names** are the C names without the prefix, in a namespace of it: `moq_track_Producer`
  is `moq::track_Producer`, and its method `moq_track_Producer_append_group` is
  `moq::track_Producer::append_group`. A keyword, or a macro GNU dialects predefine
  (`unix`, `linux`, `i386`, as in moq-tokio's `tokio::unix_`, and each of them followed by
  underscores, so `unix_` becomes `unix__`), takes a trailing `_` (`counter::new_`),
  as does a member named as its class or one of its fields, and a parameter named as a
  member or as anything in the namespace, which it would hide.
- **Objects** are classes owning their handle through a `std::unique_ptr` whose deleter
  calls `_free`, so they move and never copy, and a `Clone` type's `clone()` crosses into
  Rust for a value of its own. A type implementing `Hash` has `hash()`, Rust's, by which
  `std::hash` hashes it. Every type implementing `PartialOrd` has `<`, `<=`, `>`, and `>=`,
  through Rust's `partial_cmp`, and from C++20 `<=>`: a `std::weak_ordering` through `cmp`
  with `Ord`, since Rust's `Equal` promises agreement with `==` alone, and else a
  `std::partial_ordering`. A type whose copies may share one value, by the rule below,
  owns it through a `std::shared_ptr` instead: a copy shares the handle without crossing
  into Rust, and the last copy frees it, on whichever thread drops it. `&self` methods are
  `const`, and `self` methods are `&&`, so a call that moves the value out reads
  `std::move(c).into_label()`. A handle taken by value is moved into, by a parameter taken
  by value; moving out of a shared copy another copy holds, or a pending future borrows,
  moves out a clone, leaving that copy whole, as Rust leaves a clone. A call on a
  moved-from object, or one a call moved the value out of, aborts; a dangling reference
  to an object is C++'s own undefined behavior. Under Clang with `-Wconsumed`, such a call
  on a local is a compile error: each class carries Clang's typestate attributes, behind
  macros other compilers see as empty. The abort stays for a move Clang cannot see, as
  through a reference. Under `-Wdangling`, a view of a temporary `buf` is a compile
  error through `[[clang::lifetimebound]]`; an object lent from a receiver keeps its
  run-time check, since Clang does not track a returned handle class.
  No export keeps a view or reference past its call, so none carries
  `[[clang::lifetime_capture_by]]`; the first that does adds it.
- **Checks**: a call passes no `status`, so a borrow or move Rust's rules refuse aborts
  with its message, naming the function, as C's does with a NULL one: so does a use or
  poll of a value that is not `Send` on another thread than the one that made it, and
  one destroyed on another drops on its own thread: a job posted to the executor of the
  thread's latest `then` or `via` drops it, while that task lives and if the job runs on
  the thread, else its next call or its exit does.
- **Sharing** stands in for `clone` where no call can tell the two apart but by counting
  an `Arc`'s or `Rc`'s clones, which the exporter decides at compile time. A type's copies share when it is `Clone`; `Sync`,
  since one that is not is borrowed on one thread at a time, which copies on two threads
  would contend for; borrowed mutably by no export, through `&mut self`, a `&mut`
  parameter, or a closure lent one; and its clone owns no state that changes behind `&`.
  For that last, what a field reaches through an `Arc`, an `Rc`, or a shared reference,
  the clone shares, and any other field must clone by a derived `Clone` or std's, with no
  `Drop` of its own, and hold only such values. So a `Mutex`, `OnceLock`, or atomic it
  owns, even behind a `Box`, keeps copies apart, as does a raw pointer, or a `Clone` or
  `Drop` written by hand, which may count clones. std's `Clone` of a `Copy` type copies
  it. The one difference: every C++ copy of a shared object is one Rust value, so
  `Arc::strong_count`, `Rc::strong_count`, or a weak count of what it holds counts one for
  all of them, where Rust's clones would count one each. Of moq's types, `path`, `timestamp`, and `url`
  share, and its producers and consumers, whose `Clone` counts them, cross.
- **Records** are structs, and **enums** an `enum class`, or for one with data a
  `std::variant` of a struct per variant, `<enum>_<variant>`. An enum's methods and
  traits are free functions taking it first. `PartialEq` is `==`, `Display` `to_string()`,
  `Debug` `debug()`, and `Default` `default_()`. An **opaque value** is a trivially
  copyable class holding its C struct, with no public constructor, so only Rust makes one;
  a `&mut self` method is not `const`, and assigns `*this` what Rust wrote back.
- **Bytes** are passed as a `view`, which any contiguous bytes convert to, and returned as
  a `buf`, which owns Rust's buffer without a copy, so it outlives the object it came
  from. From C++20, either converts to `std::span<const std::uint8_t>`. A `view` or span
  lives only as long as the bytes it views, and a `view` lent to a callback only for the
  call; reading one past that is C++'s own undefined behavior, which Clang's `-Wdangling`
  refuses for a view or `str()` of a temporary `buf`, and from Clang 23 `-Wlifetime-safety`
  for one kept past its `buf` across statements: `buf` is a `[[gsl::Owner]]`, and `view`,
  `slice`, and `items` are `[[gsl::Pointer]]`s. A `Vec<T>` is a `std::vector`. A list
  parameter of a fixed-width integer other than `u8` (bytes) or `u128` or of a float, as
  `&[f32]` or `Vec<u64>`, is a `slice<T>`, which Rust reads in place and copies only if it
  keeps it: a pointer and a count, or any contiguous range of `T`, such as a
  `std::vector`, a `std::array`, a C array, a braced list, or from C++20 a
  `std::span<const T>`, which it converts back to. One whose
  items hold bytes is taken as `items` of `view`s, which points into a `std::vector` of them
  in place and converts any other range whose items convert and outlive the call, such as a
  returned `std::vector<buf>`, but not a range of temporaries. A `std::vector` moved in
  converts with each item moved, so a returned one passes back even when its items hold a
  `Copy` object C++ cannot copy. One whose items also move a
  handle is taken as `taken`, a `std::vector` of them moved in as it is, or any other such
  range moved in, each handle moved out of it. A socket address is a `socket_addr`, an IP
  address an `ip_addr`, and a `u128` a `u128`, each the C struct. `Option<T>` is `std::optional<T>`, a tuple `std::tuple`, and `Result<T, E>` a `result<T, E>`, whose `value()` aborts on an
  error. A `[u8; N]` is a `std::array<std::uint8_t, N>`. A `result`, `future`, `task`, or
  `buf` is `[[nodiscard]]`, as Rust's `Result` and futures are `#[must_use]`: dropping
  one unread loses an error, cancels a call, or reads bytes for nothing.
- **Traits** are abstract classes, passed as a `std::unique_ptr` that Rust deletes.
- **Closures** are any callable, taken by a template parameter, which Rust calls from any
  thread, one call at a time. One lent for a call Rust calls only during the call; one Rust
  keeps it moves or copies into a copy of its own, which it destroys once done with it,
  maybe before any call. An object it lends as `&T` is a `const` reference to its class, and as `&mut T` an `api::borrow<T>`, which reaches its methods
  through `->` but neither moves nor copies. Moving out of either, or keeping a copy past
  the call, aborts. It returns bytes as anything a `view` takes, copied for Rust, and a
  `Result` as a `result`, or anything with its `operator bool`, `operator*`, and
  `error()`, such as `std::expected`, whose error is anything a `view` takes.
- **Readers**, a `Read + Seek` lent for the call, are any value with `read(uint8_t *,
  size_t)` and `seek(int64_t, std::ios_base::seekdir)`, as `seekg` takes them, each
  returning a `result` of the bytes read or the position, whose error is anything a `view`
  takes, which Rust raises as an `io::Error` of that message. Not an `std::istream`, whose
  failures keep no message. A seek from the start past `INT64_MAX`, as Rust's
  `SeekFrom::Start` takes a `u64`, calls `seek_start(uint64_t)` instead, and fails in a
  reader without one. Rust calls them only during the call, from any thread, one at a
  time; a read past the size it was given aborts.
- **Futures** are move-only `future<...>`s, freed when destroyed, which cancels them and
  loses nothing. `poll(waker)` polls once; `std::move(f).get()` blocks the thread;
  `std::move(f).then(exec, callback)` polls on an executor and calls back there, returning
  a `task` that cancels when destroyed, after which the callback has finished or never
  runs and the executor is never called again: a `task` dropped before its executor starts
  the job never runs, though one dropped at once may lose that race to an executor on
  another thread; and from C++20, `co_await std::move(f).via(exec)` resumes the coroutine
  on the executor. An executor is any callable taking a `void()` job to run once, later, on any
  thread: never inline, since a wake may come from inside Rust. Each wake that lands while
  the future polls polls it again, and a future has one poller at a time, since awaiting
  it moves it.
- A **future C++ implements** is any value with `bool poll(const api::waker &)`, such as a
  binding's `future` of `void`, which Rust takes by value, polls from any thread until it
  returns true, and destroys once done with it. One a poll borrows pinned, as
  `Waiter::poll_future` does, the future that poll makes owns.
- A **waker** is a shared callable, which Rust may call from any thread. One Rust returns
  wakes through Rust and drops with its last copy.
- **The contract** is checked when the program loads, before any other call: a header
  and a library from different builds abort.

## Kotlin

`<prefix>.kt` is Kotlin for the JVM and Android over the crate's JNI functions, which
`--features kotlin` builds into the same library, loaded by `System.loadLibrary`. It needs
kotlinx-coroutines. Each value crosses JNI as a scalar, a handle as a `Long`, bytes passed
in as a `ByteArray`, bytes returned as their owner, a block of numbers as its primitive
array, and anything else encoded into a `ByteArray` as its C struct holds it. A record a
call takes or returns directly, or in an `Option`, crosses as its scalars, a nested
record's included, and an opaque value as its 64-bit words, as JNI parameters in, after a
`Boolean` saying whether an `Option` is `Some` and zeros for `None`, and through a
`LongArray` each thread keeps out, with only a record's other fields encoded;
an opaque value of one word returns as that `Long`. Past the 255 slots a JVM method takes,
the rest cross encoded.

- **Names** are in a package of the prefix: types in PascalCase without it, members in
  camelCase without their type's name (`moq.TrackProducer.appendGroup`). Pascal case
  loses where segments split, so two types whose names would match each keep a `_` at
  every split after the leading segments they share: moq's `tokio::ConnectError` and
  `tokio::connect::Error` are `TokioConnectError` and `TokioConnect_Error`. Two that
  still match, as segments apart only by case do, take a trailing `_` after the first by C
  name, as does a type named as the runtime or a generated class. Any other top-level name,
  such as a function's or an exception's, takes a trailing `_` where a type, the runtime, or
  an earlier name has it. A keyword is quoted in backticks, and a member named as one Kotlin
  gives its type takes a trailing `_`. A constructor such as `new` stays a companion function: `Counter.new(1)`.
- **Objects** are classes owning their handle, `AutoCloseable`, which each call holds
  until it returns, so the handle is freed only once no call holds it; a call after
  `close()` throws. Of a checked type, `close()` drops the value through Rust's own
  `_drop`, as Rust's `drop` would, and throws instead while a call borrows the value, as
  Rust refuses that drop, leaving the value to a later `close()`. Of any other type,
  `close()` frees the handle, and throws while a call holds it. A returned object waits in
  a nursery of the last 64 its thread returned, where `close()` frees it without the GC's
  registration, a phantom reference linked into a list under a lock. One never closed is
  registered once pushed out, by 64 later returns on its thread or with its nursery, which
  the thread holds only weakly, so a collection reaching it while it is not returning, as a
  full one does, drops it; then
  the GC frees it once it collects it, on a daemon thread a phantom reference queue feeds. A `self`
  method, or passing the object by value, moves its value out and closes it once the call
  returns. `Clone` is `clone()`, `PartialEq` is `equals`, and `Hash` is `hashCode`, Rust's
  hash folded as `Long.hashCode` folds it, or 0 without one. `PartialOrd` is
  `partialCompareTo`, null when unordered, and `Ord` makes it `Comparable`, whose
  `compareTo`, and so `<`, is Rust's `cmp`; an enum class's `compareTo` stays Kotlin's
  own by declaration order, which Kotlin makes final. A value that is not `Send`
  throws an `IllegalStateException` when used on another thread than the one that made
  it, and closed or collected on another, it drops on its own at its next call or as it
  exits. A future that is not `Send` made from none lives on a home, so a coroutine a
  dispatcher resumes on any thread polls it; one a value's method makes stays on that
  value's thread.
  A field is a `val`, `firstGet`, and its setter a function, `firstSet(value)`, named as
  the C ones, since setting an object moves it in and closes its handle, which an
  assignment would hide.
- **Checks**: a borrow or move Rust's rules refuse, such as a `&mut` borrow of an object
  another call borrows, or a move out of one a closure was lent, throws an
  `IllegalStateException` naming the function and the rule, before any Rust code runs, so the call changed nothing.
- **Records** are data classes, compared by their fields, and **enums** an `enum class`,
  or for one with data a sealed class of a data class per variant. `Display` is
  `toString()`, `Debug` `debug()`, and `Default` a companion's `default()`. A companion's
  function is `@JvmStatic`, so Java calls it on the class, as Kotlin does, but for
  `default()`, whose name Java reserves; one whose JVM name a field's getter has, as
  `getX()` beside `x`, takes a trailing `_` on the JVM alone. An **opaque
  value** is a class with an internal constructor, so only Rust makes one: of no more than
  8 bytes on every target, with no padding, fields of integers, `bool`s, and `char`s alone, nested or not, and a
  derived or std `PartialEq`, a value class over a `Long`, compared by its bits, which
  allocates nothing unless Kotlin boxes it, as in a `List` or a nullable; otherwise a class of a `Long` for each 64-bit word of its bytes, which allocates
  nothing where the JIT sees it does not escape, whose `==` is Rust's, and whose `hashCode` is Rust's `Hash`, or 0
  without one, since none Kotlin computes agrees with its `==`. Either is shared by reference, so it never changes: a `&mut
  self` method, a field's setter included, returns the value as the call left it,
  `x = x.add(o)`, and beside Rust's own return, a `Pair`, or each element of its tuple,
  `val (y, n) = x.take()`.
- **Bytes** are a `Bytes`. Passed in, Rust's own are shared with no copy, and Kotlin's,
  which `Bytes(ByteArray)` and `Bytes(ByteBuffer)` hold without a copy, are copied once
  into a buffer Rust takes, at a call's top level or in a record; those Rust lends a
  closure or trait method are copied. A direct `ByteBuffer`'s, as a `MediaCodec` output
  is, are copied straight from its memory, from its position until its limit when made;
  a heap one's array is shared, or copied when made if it is read-only. Writing a direct
  buffer's bytes while a call copies them, or reading them while one copies into it, is a
  race whose bytes are undefined; a buffer Android made inaccessible, as `MediaCodec` does
  on releasing it, throws `IllegalStateException` before Rust reads it. A `Bytes` is
  read only through `size`, `get`, `slice`, `copyInto`, and `toByteArray()`, so no view of
  Rust's memory escapes and no read meets it freed; `copyInto(ByteBuffer)` copies into a
  buffer at its position once, as `put` does, straight into a direct one's memory, such as
  a `MediaCodec` input's. Up to 1 KiB are copied, which costs less than the GC's cleanup;
  longer ones stay in Rust's buffer, each read a JNI call that passes the buffer's owner,
  which JNI keeps reachable until the call returns, and are freed once the GC finds no
  `Bytes` reaching them, slices included. The GC sees none of Rust's buffers, so at most
  64 MiB are lent at once, freed or not; past that, longer ones are copied too, and that
  garbage in Kotlin's heap runs the GC, which frees them. A `Bytes` equals and hashes by its contents, wherever each keeps them, so a
  record holding bytes does too; Rust's lent ones are copied across in one call to compare.
  Strings are `String`, passed in as their UTF-16, which Rust encodes once into UTF-8 before
  the call reads any argument, so one holding a lone surrogate, which no Rust text holds,
  throws `IllegalArgumentException` before anything moves; a `char` is an `Int`, which
  throws `IllegalArgumentException` passed in unless it is a Unicode scalar value; `usize`
  `ULong` and `isize` `Long`, which throw `IllegalArgumentException` passed in past the
  library's target's range, as a 32-bit one's is, and a `NonZero<T>` `T`'s type, which throws
  `IllegalArgumentException` passed in zero; `Option<T>` is `T?`, a tuple `Pair` or
  `Triple`, or else a generated `Tuple<n>`, and also `Tuple2` or `Tuple3` when it holds a
  numeric array or a float anywhere, records included, since `Pair` and `Triple` cannot
  compare them as Rust does (so a `BTreeMap<String, f64>` is a `List<Tuple2<String, Double>>`), `u128` `java.math.BigInteger`, which throws `IllegalArgumentException` passed in out of its range, and an `Err` a thrown `<E>Exception`
  holding it, whose message is its `Display`, a checked `Exception` each function that throws it declares with `@Throws`, so Java can catch it. A `[u8; N]` is a generated `ArrayU8x<N>`,
  which copies the `ByteArray` it is made of and throws `IllegalArgumentException` unless
  it holds exactly `N` bytes.
- **Std values** no Kotlin or Java type holds exactly are the runtime's own, each holding
  every Rust value and made only of valid ones, with conversions to and from the
  idiomatic type that throw `IllegalArgumentException` where the other cannot hold a value
  exactly. A `Duration` is a `Duration` of `secs: ULong` and `nanos: UInt`, made from and
  converted to a `kotlin.time.Duration` (`Duration(5.seconds)`, `toKotlinDuration()`),
  which is never negative or infinite in Rust and drops what is below a millisecond past
  about 146 years. A path is a `PathBuf` of the OS's bytes, any on Unix and WTF-8 on
  Windows, made from them or from a `String`, and converted to one or to a
  `java.nio.file.Path` by `toText()` and `toPath()` (Android's `java.nio.file` starts at API 26), which throw where it is not UTF-8;
  its `toString()` shows what is not as U+FFFD, as Rust's `display` does. An `IpAddr`
  is a sealed `IpAddr` of an `Ipv4Addr`, a value class of its bits, or an `Ipv6Addr`,
  each kept of its family, so an IPv4-mapped address stays IPv6, and a `SocketAddr` a
  sealed `SocketAddr` of a `SocketAddrV4`, a value class of its address and port, or a
  `SocketAddrV6`, which keeps its flow label and scope; each prints as Rust does, and
  converts to and from `java.net`'s, which has no unresolved address in Rust, no scope in
  an `IpAddr`, and no flow label in Java. A `Vec<T>`, `&[T]`, or `Box<[T]>` of a
  fixed-width integer other than `u8` (bytes) or `u128`, or of a float, is Kotlin's primitive array of it: `ByteArray`,
  `ShortArray`, `IntArray`, or `LongArray` for `i8` through `i64`, `UShortArray`,
  `UIntArray`, or `ULongArray` for `u16` through `u64`, which a caller uses under
  `@OptIn(ExperimentalUnsignedTypes::class)`, and `FloatArray` or `DoubleArray`. It
  crosses whole: passed or returned directly, in one JNI array copy, and inside another
  value, in one bulk copy into or out of its encoding. A record, variant, or tuple holding
  one, or a float anywhere, compares and hashes it by its contents and its floats by IEEE
  `==`, as Rust does: NaN equals nothing, itself included, and -0.0 equals 0.0. Any other `Vec<T>` or `&[T]` is
  a `List`, and passing a `Vec` by value closes each object in it once the call returns.
- **Traits** are interfaces; once Rust drops an implementation, it is closed if
  `AutoCloseable`.
- **Closures** are function types, which Rust calls from any thread: one lent for a call
  only during the call, and one Rust keeps, as `Spawned.new` does, until it drops it,
  when it deletes its global reference and the GC may collect it. An object it lends is
  closed once the call returns. One returning a `Result` returns its value and fails by
  throwing the `<E>Exception` a function's `Err` throws, as `StringException("…")`; any
  other exception aborts, as a callback's does.
- A **reader** is a `java.nio.channels.SeekableByteChannel`, which a `FileChannel` or
  `Files.newByteChannel` is, and the one standard interface with both reads and absolute
  seeks. Rust reads and seeks it only during the call, from any thread, one call at a
  time: each read fills a `ByteArray` Kotlin allocates, which Rust copies out of, so no
  view of Rust's memory escapes to a channel that keeps its buffer. A seek from the start
  past `Long.MAX_VALUE`, as Rust's `SeekFrom::Start` takes a `u64`, calls `seekStart` of a
  channel that is also a `StartSeekable`. An `IOException` it throws, or a seek before the
  start or past `Long.MAX_VALUE` it cannot take, is Rust's `io::Error` of its message; any
  other exception aborts.
- **Futures** are `suspend fun`s, which resume on the coroutine's dispatcher, as C++ resumes
  on its executor, and never inside a wake: an `Unconfined` one resumes on `Default`. Cancelling the coroutine frees the future and
  loses nothing. Each call is its own future, so it has one poller. The call polls it once
  before suspending, and makes the waker it parks on only if that poll pends.
- A **future Kotlin implements** is a `suspend () -> Unit`, which Rust's first poll
  launches in the calling coroutine's context, on its dispatcher, or on `Default` for one
  that does not dispatch. It runs on a job of its own, so the caller never waits on it,
  which the caller's job cancelling cancels, as does Rust dropping it; one dropped before it
  starts never runs. Once cancelled, it ends unfinished, so a cancelled call reports the
  cancel even if the lambda returns. One that throws otherwise aborts. A poll borrowing
  one, as `Waiter.pollFuture` does, is a `suspend fun` taking it, whose future owns it.
- A **waker** is a `Waker`, which Rust may call from any thread. One Rust returns wakes
  through Rust, and the garbage collector drops it.
- A function taking a **JNI env**, `&mut jni::Env`, exports to Kotlin alone, since no
  other language has one to pass: its Kotlin function leaves the env out and passes its
  calling thread's, as `tokioTlsInitAndroid(context)` does for moq's `init_android`. A
  `jni::objects::JObject` it takes is an `Any?`, the reference Kotlin passed, which Rust
  holds only during the call. A Java exception its Rust leaves pending is what Kotlin
  gets, as from any native method, in place of what the call returned, which drops
  first. Any other JNI type, an env taken any other way, a `JObject` with no env to hold
  it under, two envs, one lent to a future or a trait Kotlin implements, or one beside a
  `&mut` value, whose change the exception would lose, is refused.
- **The contract** is checked when the library loads, before any other call: a Kotlin
  file and a library from different builds throw. That first call also notes the JVM and
  `_Native` for the calls Rust makes into Kotlin, so the library defines no `JNI_OnLoad`,
  and the crate it exports may define its own, as moq-ffi's does.
- **Shrinking**: `<prefix>.pro` keeps what the JNI glue finds by name, `_Native`, its native
  methods, and the Kotlin functions Rust calls, while R8 strips and renames the rest. An
  Android library ships it as a consumer rule (`consumerProguardFiles`), which its AAR
  carries to each app that shrinks it.

## Python

`<prefix>.py` is Python over a CPython extension that `--features python` builds into the
same library on the stable ABI, so one build loads in CPython 3.11 and later, but not in a
free-threaded build (3.13t, 3.14t), which the module refuses at import with an `ImportError`. The module
loads the library from beside itself: `lib<prefix>.so`, `lib<prefix>.dylib`, or
`<prefix>.dll`. Its glue converts each Python value to the C value and calls the C ABI
function in-process, so a call is one crossing, which releases the GIL while Rust runs,
so Rust may call Python back from any thread meanwhile. A leaf keeps it: a read or write of
a primitive `pub` field of a `Send` object reached without a `Deref`, or a `PartialEq` or
`Hash` derived, or std's, down to primitives, which the export proves runs no code but
std's and the glue's, waits on no lock, and calls no Python, so it cannot deadlock on the
GIL (`Abi::leaf` in `src/abi.rs` gives the argument). A field read costs 26 ns that way,
against 63 releasing it ([the benchmark](../bench/ffi/README.md#leaf-calls)).
Each function, method, and property but an `async def` is the extension's own, bound in
the module or set on its class as it loads, so a call enters it with no Python frame
between; the Python declares them under `typing.TYPE_CHECKING`, for type checkers. They
take arguments by position or keyword, raise as a `def` would, and carry their
docstrings and signatures. The module passes mypy and pyright, both strict, so a typed
caller imports it with no ignores.

- **Names** are in the module: types in PascalCase without the prefix, members in
  snake_case without their type's name (`moq.TrackProducer.append_group`), and two types
  whose names would match kept apart as Kotlin's are (`TokioConnect_Error`), as are
  top-level names that clash. A keyword, a member Python gives every such class, or a
  method named as a field takes a trailing `_`. A member or top-level name may be a
  builtin's (`bytes`, `int`): the module reads each builtin through an alias (`_bytes`).
  Constructors such as `new` stay static methods: `Counter.new(1)`.
- **Objects** are classes owning their handle, freed once their last reference goes,
  which the caller's reference keeps through a call, so there is no `close()`. A `self`
  method, or passing the object by value, moves its value out, after which a call raises
  `ValueError`. `Clone` is `clone()`, and `PartialEq` is `==`, unhashable unless the type implements `Hash`, which `hash()` hashes it by. `PartialOrd` is `<`, `<=`, `>`, and `>=`, through Rust's `partial_cmp`, each false when the two are unordered. An object, as
  an opaque value and a record, takes weak references (`weakref.ref`). A field is a
  property, `first_get`, and its setter a method, `first_set(value)`, named as the C ones,
  since setting an object moves it in and closes it, which an assignment would hide.
- **Checks**: a borrow or move Rust's rules refuse raises, naming the function and the rule,
  before any Rust code runs, so the call changed nothing: `ValueError` for a value moved out, or a lent object that outlived the one it
  borrows from, as for a closed object, and `RuntimeError` for any other, as for a value
  that is not `Send` used on another thread than the one that made it. Freed on another,
  as the collector or the last reference there frees it, such a value drops on its own
  thread: on the next turn of the event loop that ran there when a call made a value that
  is not `Send`, though the loop calls nothing into Rust, else at its next call or as it
  exits.
- **Records** are slotted classes, written out as `@dataclass(slots=True,
  weakref_slot=True)` would make them, so no `dataclass` runs at import: an `__init__`
  taking each field, `__match_args__`, a dataclass's `repr`, and `__eq__` comparing the
  fields, a float by IEEE `==` even against itself, as Rust does. `dataclasses.fields`,
  `replace`, and `asdict` take one, as its `__dataclass_fields__` is made on its first read,
  and so does `copy.replace`. **Enums** are an `Enum` of upper-case members, or for one
  with data a class of such a record per variant, `Event.Moved(p)`. `Display` is `str()`, `Debug` `repr()`, and `Default` a static
  `default()`. Constants are module or class attributes, made anew on each read unless a
  literal. An **opaque value** is a class of the runtime's `Value`, which holds its bytes
  in place, so only Rust makes or reads one: making one raises `TypeError`. It is shared
  by reference, so it never changes: a `&mut self` method, a field's setter included,
  returns the value as the call left it, `x = x.add(o)`, and beside Rust's own return, a
  `tuple`, with each element of a tuple Rust returns, `x, n = x.take()`.
- **Bytes** passed in are any buffer, `bytes | bytearray | memoryview`: `bytes` is read in
  place, and others, which Python may change while Rust reads, are copied. Bytes Rust
  returns, or a record holds, are a read-only `memoryview` lending Rust's buffer until it
  and its views are released. Those Rust lends a closure or trait method are copied into
  `bytes`. Strings are `str`, `u128` an `int`, `Option<T>` `T | None`, a tuple a `tuple`, `Duration`
  the runtime's `Duration`, a `NamedTuple` of `secs` and `nanos` as Rust holds them, which
  raises `OverflowError` or `ValueError` passed in out of their range; its `from_timedelta`
  and `to_timedelta` raise rather than lose anything, as a `timedelta` holds only whole
  microseconds up to 999,999,999 days. A zero passed for a `NonZero` raises `ValueError`. A `Range` is a `range`, and an `Err` a raised `<E>Exception`
  holding it in `error`, whose `str` is its `Display`. An enum's, or an object's whose
  type is an enum, raises a subclass per variant, `ErrorException.Timeout`, which
  `ErrorException(error)` makes too, so `except ErrorException.Timeout` catches that
  variant alone; one of a variant a `#[non_exhaustive]` enum's crate adds after the export
  raises `ErrorException` itself. A `[u8; N]` is a generated
  `ArrayU8x<N>`, a `bytes` that raises `ValueError` unless it holds exactly `N`; a
  parameter takes any `bytes` of `N`.
- A **path** is a `str`, or any path passed in, which crosses as `os.fsencode` encodes it, so
  one that is not UTF-8 keeps its bytes. A `SocketAddr` is the `socket` module's tuple,
  `(host, port)` or `(host, port, flowinfo, scope_id)`, of that family for `V4` or `V6`.
  An `IpAddr` is an `ipaddress.IPv4Address` or `IPv6Address`, and an `Ipv4Addr` or
  `Ipv6Addr` the one of its family; one passed in with a scope raises `ValueError`.
  A `Vec<T>` or `&[T]` Rust returns is a `list`, and passed in is any iterable other than a `str`,
  annotated `Iterable`, or `Sequence` in a record, which keeps it. Passing a `Vec` by value closes
  each object in it.
  A `Vec<T>`, `&[T]`, or `Box<[T]>` of a fixed-width integer or float, `u8` (bytes), `u128`,
  `usize`, and `isize` aside, Rust returns as an `array.array` of its typecode, filled in one copy; the extension
  fails to load where a typecode is not exactly as wide as its Rust type. Passed in, it is
  any buffer of exactly those numbers in native byte order, one dimension and C-contiguous, such as an
  `array.array`, a `memoryview` cast to their format, or a numpy array, copied once, since
  another thread may write it while Rust reads; a buffer of another kind, width, or byte order raises
  `TypeError`. A record compares one by its numbers, numpy's included.
  An integer out of its Rust type's range raises `OverflowError`.
- **Traits** are abstract classes an implementation subclasses; once Rust drops one, its
  `close()` is called if it has one.
- **Closures** are any callable, which Rust calls from any thread, one call at a time,
  taking the GIL: one lent for a call only during the call, and one Rust keeps until it
  drops it, when it lets go of its reference. An object it lends is closed once that call
  of it returns. What it returns Rust owns: a `str` is copied, and an opaque value's
  bytes, so the caller's stays its own. One returning a `Result`
  fails by raising the exception of its error, such as `StringException(message)`, whose
  `error` Rust takes. Any other exception a closure or trait method raises prints its
  traceback and aborts, since Rust has nowhere to unwind it to.
- A **reader** is a `ForeignReader`: anything with `readinto` and `seek`, as a binary
  file such as `open(path, "rb")` or `io.BytesIO` has, which Rust reads and seeks only
  during the call, from any thread, one call at a time, taking the GIL. `readinto` fills
  a `memoryview` of Rust's own buffer in place, so a read copies nothing; a view of it
  kept past the read, such as a slice, aborts, as it would write the buffer once it is
  gone. An `OSError` fails Rust's read or seek with its message, and any other exception
  aborts. A value without both methods raises `TypeError` at the call.
- **Futures** are `async def`s, which run nothing until awaited, or for a function that is
  not `async`, an awaitable the call returns. They resume on the running event loop and
  never inside a wake: a wake from any thread queues the future's token and writes a byte
  to a socket the loop reads, without the GIL, or during a call on the loop's own thread,
  writes nothing and schedules the loop's dispatch once the call returns. Cancelling the task frees the future and
  loses nothing. The call polls it once before suspending, and a future is awaited once,
  as a coroutine is. Futures and wakers run on asyncio's event loop alone: an await or a
  `Waker` where none runs, as under trio, raises `RuntimeError` saying so.
- An **awaitable Python implements** is an `Awaitable[None]`, such as a coroutine, passed
  on a running event loop, where Rust's first poll runs it as a task. Rust dropping it
  cancels the task, or closes a coroutine it never started. One that raises, or that
  anything but Rust cancels, prints its traceback and aborts. A value that is not
  awaitable, or a future of another loop, raises at the call. A poll lent one pinned,
  as `Waiter.poll_future`, is an `async def` taking it, whose future owns it.
- A **waker** is a `Waker(callback)`, which Rust may wake from any thread, calling
  `callback` on its event loop; `wake()` wakes it from any thread. One Rust returns is a
  `Waker` too, which wakes through Rust and drops with its last reference.
- **The contract** is checked when the module imports, before any other call: a Python
  file and a library from different builds raise `ImportError`.

## Swift

`swift/<prefix>.swift` is Swift over the C ABI, for Apple's platforms and Linux. It imports
the header as the clang module `<Prefix>FFI`, which `swift/module.modulemap` declares
beside `swift/<prefix>_atomics.h`, the atomics its runtime needs that Swift's standard
library has only from macOS 15: a build puts `swift/` on its import path and links the
static library. It imports swift-collections' `BasicContainers` for `UniqueArray`, which
`just ffi swift-collections` builds at the version the tests pin, 1.7.5. It needs Swift 6.2
and, for `Swift.Duration`, macOS 13 or iOS 16, and builds in either language mode. Every
value of a Rust type crosses as one Swift type that holds it exactly, and back unchanged.

- **Names** are as Kotlin's, at the module's top level: types in PascalCase without the
  prefix, members in camelCase without their type's name (`TrackProducer.appendGroup`). A
  keyword is quoted in backticks, and a member named as one Swift gives its type, such as
  `init`, or as a field or member before it, takes a trailing `_`. Names that clash are
  kept apart as Kotlin's are. Parameters have no labels but an optional borrowed object's,
  so a call reads as Rust's, and a
  constructor such as `new` stays a static function: `Counter.new(1)`.
- **Objects** are noncopyable structs (`~Copyable`) owning their handle, freed in `deinit`,
  so there is no `close()`, and Swift's compiler checks each move and borrow Rust makes: a
  `self` method is `consuming`, a `&mut self` one `mutating`, so its object must be a
  `var`, and a parameter is `consuming` by value, `borrowing` for `&T`, and `inout` for
  `&mut T`. A call on a moved object, or an object lent to a closure kept past its call,
  fails to compile. A tuple holding one is a generated struct, `TupleStringCounter`, whose
  fields `_0`, `_1`, ... are its elements, and an array of them swift-collections'
  `UniqueArray`. Since an optional holds one only by consuming it, an `Option<&T>` of one
  is an overload with and without it, labelled by its name, as a defaulted argument is:
  `clamped()`, `clamped(low:)`, `clamped(high:)` and `clamped(low:high:)`, up to two per
  function; `Option<&mut T>` is `inout T?`. `Clone` is `clone()`, and `PartialEq` is `==`, which Swift 6.4 makes
  `Equatable` unless it throws, as below, and `Hashable` with `Hash`, whose `hash(into:)`
  combines Rust's hash; `PartialOrd` is `<`, `<=`, `>`, and `>=`, through Rust's
  `partial_cmp`, and with `Ord` makes it `Comparable` unless it throws; `Display` is `description` and `Debug`
  `debugDescription`, which Swift conforms no noncopyable type to yet. A `Copy` object,
  which Rust copies where it takes one by value, and an object thrown as an error, which
  `Swift.Error` requires be copyable, are final classes instead, whose handle a call
  holds until it returns, and moving its value out closes it, after which a call throws. A
  field is a computed property, `firstGet`, and its setter a function, `firstSet(value)`,
  named as the C ones, since setting an object moves it in, which an assignment would hide.
  An object is `Sendable` where Rust lets it cross threads, checking each borrow at run
  time: a class if its type is `Send` and `Sync`, since its holders share it, and a
  noncopyable struct if `Send`, since it only moves. A record, enum, or tuple struct is
  `Sendable` only if each handle it holds is, so Swift's compiler keeps the others on one
  task, and a use on another thread anyway, as after a suspension that resumed elsewhere,
  throws a `CheckError`. A type thrown as an error is `Sendable` unchecked, since
  `Swift.Error` requires it. A future that is not `Send` made from none lives on a home, so
  any executor awaits it.
- **Checks**: a borrow or move Rust's rules refuse at run time, such as a borrow of a value
  that is not `Sync` on a second thread, throws a `CheckError`, holding the function and
  the rule, before any Rust code runs, so the call changed nothing, but that a call
  consuming an object took it, as its parameter consumes it whether or not the value moves,
  and frees it as a scope's end would. So every function whose objects can fail a check `throws`, and a
  field read stays a property whose getter throws, except one of a noncopyable value,
  whose getter Swift cannot let throw, which is a method, `firstGet()`. Of a type whose
  checks can fail, or a record or enum holding one, `description` and `debugDescription`
  are getters that throw, and `clone()`, `==`, and `hash(into:)` throw, so it conforms to
  none of `CustomStringConvertible`, `CustomDebugStringConvertible`, `Equatable`, and
  `Hashable`, whose
  requirements cannot throw. A closed class anywhere in a call's arguments throws too. A
  copy a closure returns, which has no caller to throw to, traps instead.
- **Records** are structs and **enums** Swift enums, with a variant's data as associated
  values, all `Sendable`. `PartialEq` is `==`, through Rust's, and `Hash` `hash(into:)`,
  making it `Hashable`; `Display` is `description`,
  `Debug` `debugDescription`, each throwing as an object's does when it holds a checked
  one, and `Default` a static `default()`. An **opaque value** is a struct holding its C
  struct, with no public initializer, so only Rust makes one; a `&mut self` method is
  `mutating`, and assigns `self` what Rust wrote back.
- **Bytes** are the runtime's `Bytes`, a `RandomAccessCollection` of `UInt8` whose slices
  are `Bytes` too. Bytes Rust returns lend its buffer until Swift releases the last `Bytes`
  or slice reading it, and pass back to Rust as a second owner of it, with no copy, except
  as an array's items or behind a pointer, which Rust only reads and so copies. Swift's
  own, made with `Bytes(data)` or from any sequence of bytes, Rust copies once where it
  keeps them. `bytes.data` is a `Data`: Swift's as they are, and Rust's copied, so nothing
  writes into a buffer Rust shares, and `bytes.withSpan` lends them in place as a
  `Span<UInt8>`.
- **Slices** a call borrows, `&[u8]` and `&[T]` of a fixed-width integer or float, are
  `borrowing Swift.Span<T>`, which Rust reads in place: an array's `span`, or before
  Apple's 26 releases its slice's (`array[...]`), or `Bytes` lent by `withSpan`, whose
  closure is synchronous. An `async` call takes `Bytes` copied to a slice that lives
  across the `await`, as `let s = Array(bytes)[...]` and then `try await f(s.span)`.
  Bytes Rust lends a closure or trait method are a `Span<UInt8>` of Rust's own memory,
  which the compiler keeps from outliving the call. A `span` property on `Bytes` waits on Swift's lifetime dependencies, still
  experimental, and on `Data` keeping short bytes inline.
  Strings are `String`, copied, `char` `Unicode.Scalar`, `usize` `UInt`, `Option<T>` `T?`,
  a tuple a Swift tuple, though a tuple of one is its element, `u128` a `U128` of its
  `high` and `low` halves until the deployment target allows `UInt128`, and `Duration` the
  runtime's `RustDuration` of its `secs` and `nanos`, whose initializer throws a
  `CheckError` for a second's nanoseconds or more. `duration` is it as a `Swift.Duration`,
  which holds every one, and `RustDuration(_:)` makes one from a `Swift.Duration`,
  throwing for one negative, past Rust's longest, or with a part below a nanosecond. A
  `NonZero<T>` is the runtime's `NonZero<T>` of `T`'s integer, its `value`, made with
  `try NonZero(3)`, whose initializer throws a `CheckError` for a zero, so no call
  taking one checks it and a type holding one keeps its protocols. An `Err` is thrown: an exported type as itself, which conforms to
  `Swift.Error`, and any other in a generated `<T>Error` holding it, whose `init(error:)`
  makes one for a closure to throw back. A `[u8; N]` is a
  generated `ArrayU8x<N>`, a collection holding its bytes inline in C's struct, made by a
  failable initializer from exactly `N` bytes, until the deployment target allows
  `InlineArray`.
- A **path** is the runtime's `PathBuf` of its `bytes`, a `Bytes` sharing Rust's buffer as
  bytes do, so one that is not UTF-8 keeps them; `PathBuf("a/b")`, or a string literal,
  makes one, and `string` is its text, or nil if it is not UTF-8. Each address type is the
  runtime's of its name, as std's: `Ipv4Addr` and `Ipv6Addr` of their `bits`, as
  `to_bits` gives them, or `octets`, `IpAddr` an enum of `.v4` and `.v6`, `SocketAddrV4`
  and `SocketAddrV6` of their IP, port, and IPv6's flow label and scope, and `SocketAddr`
  an enum of either. A `Vec<T>`, `&[T]`, or
  `Box<[T]>` is an array, or a `UniqueArray` of objects; one of a fixed-width integer
  other than `u8` (bytes) or `u128`, or of a float, is copied across whole, with no call per item.
- **Traits** are `Sendable` protocols, which Rust releases once it drops an implementation.
- **Closures** are `@Sendable` function types, which Rust calls from any thread, one call
  at a time. One lent for a call Rust calls only during the call; one Rust keeps is
  `@escaping`, boxed until Rust drops it. An object it lends is `borrowing`, or `inout`
  for `&mut T`, so it cannot outlive the call; assigning another to the `inout` one traps,
  since Rust's stays its own. One returning
  `Result<T, String>` `throws`: Rust reads its error's `description`, so a thrown
  `StringError` returns its string as it was.
- A **reader** is `inout some SeekableReader`, the runtime's protocol of
  `read(into:)` and `seek(offset:from:)` from a `SeekOrigin`, as swift-system's
  `FileDescriptor` reads and seeks, lent for the call and written back once it returns,
  so a struct's position stays. A seek from the start past `Int64.max`, as Rust's
  `SeekFrom::Start` takes a `UInt64`, calls `seek(start:)`, which by default throws. Rust
  calls it from any thread, one call at a time, and reads a thrown error's `description`
  as its `io::Error`'s message.
- **Futures** are `async throws` functions that run on their caller's executor
  (`nonisolated(nonsending)`), so a call ready at its first poll never leaves the caller's
  actor, and one that suspends resumes there, never inside a wake. Cancelling the task frees the future, losing nothing, and throws
  `CancellationError`. Each call is its own future, so it has one poller. The call makes
  and polls it in one crossing, `_start`, before suspending, and makes the signal it parks
  on only if that poll pends; the signal latches a wake that lands before it parks.
- A **future Swift implements** is an `@escaping @Sendable () async -> Void`, which Rust's
  first poll runs as a task of its own. Rust dropping it cancels the task, and one dropped
  before it starts never runs. A poll lent one, such as `Waiter.pollFuture`, is a static
  `async throws` function whose future owns it.
- A **waker** is a `@Sendable () -> Void`, which Rust may call from any thread. One Rust
  returns wakes through Rust and drops once Swift releases it.
- **The contract** is checked once, at the first call into the library: a Swift file and a
  library from different builds trap.
- **Every body may inline into its caller.** Each function, initializer, subscript, and
  computed property is `@inlinable`, and what they name `@usableFromInline`, so a
  consumer's optimizer in another module can inline a call and reach the C function
  directly, with no flag of its own. Where it declines, the call goes to the module's
  copy, which Swift may merge with like wrappers into one that calls the C function
  through a pointer, whose cost follows where the linker places it. Their cold paths, a
  failed check's error and a handle's free, stay out of line, so a call more often looks
  worth inlining. The module's `.swiftmodule` holds the bodies, about 5 times its size
  without them.

## Go

`go/` holds a Go package per crate whose items the facade exports, over the C header
through cgo, for Go 1.24 or later, and the runtime they share, `go/rt`. Each includes
`<prefix>.h` from `go/rt`, so `go/` is a whole module, as a release's zip holds it. The
runtime links each exported target's static library from beside itself, as
`rt/lib<prefix>_<goos>_<goarch>.a`, by a `#cgo <goos>,<goarch> LDFLAGS` line naming it and the
system libraries it needs, as rustc's `--print native-static-libs` prints them, which the
check of each target reads from its crates' `#[link]` attributes and build scripts. So a
build needs no `CGO_LDFLAGS`, and on a platform with no library, `rt/platform.go` fails to
compile. Go links Windows through MinGW, so only `x86_64-pc-windows-gnu` and
`aarch64-pc-windows-gnullvm` are Windows targets it links, and iOS the device's library, as
Go builds the simulator's from the same `ios/arm64`. A library a crate ships rather than the
system, as winapi's `winapi_<dll>` import libraries for MinGW, is not found where the Go
links: the release builder exports and builds with `WINAPI_NO_BUNDLED_LIBRARIES=1`, so
winapi links the system's; Linux and macOS lines exclude Android
and iOS, which Go also counts as `linux` and `darwin`. The export puts no library there:
copy it in, as [the release builder](release/README.md) does, without the LLVM bitcode
(`.llvmbc`, `__LLVM`) that Go refuses, from 1.26.11 and 1.27.4, in an archive a `#cgo` line
names. A crate's package is its name without the prefix and its
`_`: moq-mux's is `mux`, and serde_json's `serdejson`, but keeps the prefix where its
strip leaves no leading letter (`moq_2d` is `moq2d`). One that a Go keyword, `C`, `main`,
the runtime, or an earlier crate's package takes gains a trailing `_`, as a crate `rt` is
`rt_`. Cargo keeps crates from depending on each other in a cycle, so their packages
never import each other in one, as Go requires,
while Rust's modules may, so a package per module could not hold moq. An instance of a
generic type, or a function naming one, belongs to the last of its own and its arguments'
crates in Cargo's order, which can name them all: moq's
`mux::catalog::Producer<hang::catalog::Extra>` is in `mux`. The packages' import path is
the module of the module `go/` is in, as Go finds it: that of a `go.mod` in `go/` itself,
which the export keeps as it is, as a release's names its module path, or else of the
`go.mod` above `--out`, with the path to `go/`, each as `go mod edit -json` reads it; or else
`<prefix>`, the module of the `go.mod` the export writes in `go/`, which a caller requires and
replaces with that directory. The export needs `go` on PATH whenever it reads a go.mod
other than one in its own form, `module <path>` and `go 1.24`.

- **Names** are the C names without the prefix and the segments outside the item's crate,
  such as the facade's module re-exporting it, in Pascal case: `moq_track_Producer` is
  `net.TrackProducer`, its method `moq_track_Producer_append_group` is
  `TrackProducer.AppendGroup`, and `moq_mux_codec_opus_Config` is `mux.CodecOpusConfig`. Two
  types of a package whose names would match are kept apart as Kotlin's are, after the
  segments outside the crate: moq-tokio's `ConnectError` and `connect::Error` are
  `tokio.ConnectError` and `tokio.Connect_Error`. A name still shared after that keeps the
  module segment before it: the facade's `tokio::connect::Config`, `tokio::listen::Config`,
  and `tokio::quic::Config` are `tokio.ConnectConfig`, `tokio.ListenConfig`, and
  `tokio.QuicConfig`. An
  instance in another crate's package keeps its whole path. A function of a type that takes no receiver is a function of
  the package named through its type (`CounterSeededPoint`), but `new`, which is
  `NewCounter`. A constant keeps its upper case (`LIMIT`), and one that is not a `bool` or
  integer is a function of that name (`GREETINGS()`). A function of the package named as a
  type, which Go's case cannot keep apart as C's does, is `New` and that name when it
  returns that type, or an `Option` of it (moq's `clock::interval` is
  `kio.NewClockInterval`), and otherwise takes a trailing `_` (`mux::codec::opus::config`
  is `mux.CodecOpusConfig_`).
  A method named `Close` or `Error`, or as a field or a method before it, and a parameter
  named as a Go keyword, a predeclared name, or a package the code uses, take a trailing
  `_` too, as does a variant's or error's name or any other in a package that another
  there, or the runtime's, takes.
- **Objects** are pointers to structs owning their handle, each an `io.Closer`, which
  each call holds until it returns, so the handle is freed only once no call holds it. Of
  a checked type, `Close()` drops the value through Rust's own `_drop`, as Rust's `drop`
  would, and returns an error matching `rt.ErrBorrowed` instead while a call on another
  goroutine borrows the value, as Rust refuses that drop, leaving the value to a later
  `Close()`. Of any other type, `Close()` frees the handle, and returns an error matching
  `rt.ErrBorrowed` while a call holds it; one no call holds, as soon after its return,
  drops and frees in one crossing. A second `Close()` does nothing. A returned object waits
  in a nursery of the last 64 its P returned, where `Close()` frees it without the GC's
  registration (`runtime.AddCleanup` and its `Stop`), which costs more than the rest of a
  return and close. One never closed is registered once pushed out, by 64 later returns or
  with its nursery once the GC drops that from its `sync.Pool`, and freed once the GC
  collects it after. A call that meets
  a closed object, as its receiver or anywhere in its arguments, returns an error matching
  `rt.ErrClosed`, so every such call returns an `error`; `String()` and `GoString()`,
  which return none, return the error's text. A `self` method, or passing the object by
  value, moves its value out and closes it once the call returns. `Clone` is `Clone()`, and a `pub` field a getter, `FirstGet()`,
  and a setter, `FirstSet(v)`.
- **Checks**: a borrow or move Rust's rules refuse returns an error naming the function
  and the rule, before any Rust code runs, so the call changed nothing: one matching `rt.ErrClosed` for a value moved out, or a lent
  object that outlived the one it borrows from, and `rt.ErrBorrowed` for any other. So
  every function whose objects can fail a check returns an `error`. `String()` and
  `GoString()` return a failed check's text, having no error to return; what a closure
  returns has no caller to return one to, so it aborts with the message. A checked call
  goes through a C function of the package's preamble that keeps the status in a C local
  and returns it beside the value, so the call allocates nothing for it: cgo moves to the
  heap any Go local whose address C gets, since C may call back into Go, which can move the
  goroutine's stack.
- **Threads**: a goroutine moves between threads, so a value that is not `Send` is made on
  a home, a thread Rust owns, which runs each call on it, from any goroutine, while the
  goroutine's thread waits, and drops it there once closed or collected.
- **Records** are structs, and **plain enums** an `int32` type with a constant per
  variant (`ShapeSquare`). An enum with data is an interface each variant's struct
  implements (`EventMoved{V0: p}`), sealed so only the package's do; its methods are the
  interface's. Positional fields are `V0`, `V1`, and so on. `Display` is `String()`,
  `Debug` `GoString()`, `PartialEq` `Equal(other)`, `PartialOrd` `PartialCompare(other)`,
  -1, 0, or 1 and false when unordered, and `Ord` `Compare(other)`, the shape
  `slices.SortFunc` takes, all through Rust's, and `Default` a
  function of the package, `PointDefault()`. An object's exported `Handle_` field, and a
  trait's `<Trait>Vtable_` function, are how one generated package reaches another's,
  since cgo gives each its own C types: callers use neither. An **opaque value** is a
  struct of its bytes, `Bytes_`, and `Made_`, true once Rust made it: a zero value panics
  where Rust would read it, and `==` does not compile, since bytes alike need not be
  equal, so `Equal`, if it has one, is Rust's. A `&mut self` method has a pointer
  receiver, which it assigns what Rust wrote back.
- **Bytes** a call takes are a `[]byte`, and text a `string`, each read in place: Go's
  memory, pinned where C reads it through other memory. Bytes Rust passes a callback are
  copied into a `[]byte`. Bytes anywhere else, such as those Rust returns or a record
  holds, are an `rt.Bytes`, made from Go's with `rt.NewBytes`. One of 512 B or more from Rust
  reads its buffer in place through `Len`, `At`, `Slice`, `CopyTo`, `Read`, and
  `WriteTo`, and frees it once the GC finds no `Bytes` reaching it, so no Go code holds a
  slice of freed memory; a smaller one is copied, which costs less than lending it. Up to
  16 buffers lent one after another, within 64 KiB, share one GC cleanup and free
  together, so a `Bytes` kept long keeps at most 64 KiB of the others.
  The GC sees none of Rust's buffers, so at most 64 MiB are lent at once, freed or not;
  past that, larger ones are copied too, and that garbage in Go's heap runs the GC, which
  frees them. On glibc, each thread's malloc arena keeps what is freed into it for that
  thread to reuse, and a goroutine moves between threads, so reading lent bytes at full
  speed can hold up to 64 MiB of RSS per thread the reads ran on. A global allocator that
  returns freed memory, such as [mimalloc](https://crates.io/crates/mimalloc), declared in
  the exported crate, holds it near what is lent. `Copy()` copies it out. Text Rust returns is copied once into a `string`, and Rust's
  freed at the call. A path is a `string`
  of the OS's bytes, which need not be UTF-8. `Option<T>` is `*T`, or for an object its
  pointer, nil for `None`. A tuple is `rt.Tuple2[A, B]` and so on, but a tuple of one is its
  element, and one returned whole is the function's results. A `[u8; N]` is a `[N]byte`.
- **Lossless types**: each holds every value of its Rust type, so one passed in and
  returned is the same value, and none panics. Where Go's own type does not, the
  runtime's crosses, with conversions to and from Go's that return an error matching
  `rt.ErrInvalid` for a value the other has no room for. A `u128` is an `rt.U128{Hi, Lo}`,
  converted by `Big()` and `rt.U128Of(*big.Int)`. A `Duration` is an `rt.Duration`, whose
  zero value is zero and whose `Secs()` and `Nanos()` are Rust's: `rt.NewDuration(secs,
  nanos)` carries nanoseconds past a second into the seconds, as Rust's `Duration::new`
  does, `rt.DurationOf(time.Duration)` fails on a negative one, and `Std()` fails past
  `time.Duration`'s 292 years. An address is the runtime's mirror of `std::net`'s:
  `rt.Ipv4Addr` and `rt.Ipv6Addr` their octets, `rt.SocketAddrV4` and `rt.SocketAddrV6`
  structs of their fields, flow label and scope too, and `rt.IpAddr` and `rt.SocketAddr`
  one of either family, made by `rt.IpAddrFromV4` and so on and read by `V4()` and `V6()`,
  whose zero value is IPv4's `0.0.0.0`, port 0. Each converts to and from `net/netip`'s,
  `Netip()` and `rt.<Type>Of`, which fail on the wrong family, a zone an `Ipv6Addr` or
  `IpAddr` has no room for, or a flow label `netip` has none for; a scope is a zone of its
  number, and a zone converts by its number or its interface's name. Every one compares
  with `==`. A `string` that is not UTF-8, a `rune` that is not a Unicode scalar value,
  a path that is not WTF-8 on Windows, or a zero for a `NonZero`, fails the call taking it,
  anywhere in its arguments, before it is made, with an error matching `rt.ErrInvalid`, so
  every such call returns an `error`, on every target for a path too, `Equal` of a value
  holding text too, and `String()` and `GoString()` the error's text; what a closure or
  trait Go implements returns Rust has no caller to return one to, so it aborts.
- **Lists**: a `Vec<T>`, `&[T]`, or `Box<[T]>` is a `[]T`; one of a
  fixed-width integer other than `u8` (bytes) or `u128`, or of a float, is lent in place, pinned for the call, as bytes are, and
  one Rust returns is copied out whole.
- **Errors**: a `Result` returns an `error` last. An exported error type is returned as
  itself, implementing `error` with its `Display`, else its `Debug`, and any other in a struct
  `<T>Error{Value}`, such as `rt.StringError`, in the package of the last crate a type
  in it belongs to, or `rt` for none. Ranges and bounds are `rt` types too
  (`rt.RangeU64`).
- **Traits** are interfaces. Once Rust drops an implementation, it is closed if it is an
  `io.Closer`.
- **Closures** are funcs, which Rust calls from any thread: one lent for a call only
  during the call, and one Rust keeps, as moq's `clock::drive` does, until Rust drops it,
  which releases it to the GC. An object it lends by reference is closed once the func
  returns. A `Result` returns an `error`
  last, whose `Error()` crosses as Rust's `String`. What it returns crosses as Rust's: a
  `string` copied, and an opaque value as its bytes, so the func keeps its own; a zero one
  panics.
- **Readers**, `Read + Seek` behind `&mut`, are an `io.ReadSeeker`, such as an `*os.File`,
  which Rust reads and seeks only during the call, one call at a time, and never closes.
  A read of bytes and an error hands Rust the bytes, then the error on the next read, as
  Rust's `Read` takes one or the other; `io.EOF` ends it, and a read of nothing and no
  error reads again, giving up with `io.ErrNoProgress` after 100. A seek from the start
  past `math.MaxInt64`, as Rust's `SeekFrom::Start` takes a `uint64`, calls `SeekStart` of
  a reader that is also an `rt.StartSeeker`, as `io.Copy` uses an `io.WriterTo`, and fails
  in any other. Their errors cross as their messages.
- **Futures** are calls taking a `context.Context` first, which block until the future is
  ready. The call makes and polls it in one crossing, `_start`, before it waits, and
  makes the waker it waits on only if that poll pends; a wake from any thread signals the waiting goroutine, which polls
  again, so a wake never polls inside Rust, and one that lands before the wait is kept. A
  context done before the call, or while it waits, returns its error and frees the future,
  which cancels it in Rust and loses nothing: the next call takes what it would have.
  Each call is its own future, so it has one poller, and its context bounds only it.
- A **future Go implements** is a `func(context.Context)`, which Rust's first poll runs on
  a goroutine of its own. Its context derives from the call's, `context.Background()` for
  a call without one, so it carries the caller's values and deadline, and the caller's
  cancel reaches it; Rust dropping it cancels it too, and one dropped before it starts
  never runs. Once its context is done, it ends unfinished, so a cancelled call reports
  the cancel even if the function returns. A poll taking one pinned through a waiter's own
  method, as kio's `Waiter::poll_future` does, is a call of the package taking it
  (`WaiterPollFuture`), whose future owns it.
- A **waker** is a `func()`, which Rust may call from any thread. One Rust returns wakes
  through Rust, and the garbage collector drops it.
- **The contract** is checked when `rt` loads, before any other call: Go packages and a
  library from different builds panic.

## Dart

`<prefix>.dart` is the library `package:<prefix>/<prefix>.dart` over the C ABI through
`dart:ffi`, tested on Dart 3.13. Each C function is an `@Native` external, which finds
the shared library the crate builds with `--features dart` as the package's native asset:
the library's own URI, mapped to the library by a build hook or by `--native-assets`.

- **Names** are the C names without the prefix: a type in Pascal case and a member in
  camel case, so `moq_track_Producer` is `TrackProducer`, and its method
  `moq_track_Producer_append_group` is `TrackProducer.appendGroup`, and a constant in
  upper case (`LIMIT`). A function of a type
  that returns it is a constructor, which may share its name with a field: an object's
  `new` is its unnamed one (`Counter('c', 1)`), and any other a named one
  (`Position.group`). A type's other functions are static members. Two types whose
  names would match are kept apart as Kotlin's are (`TokioConnect_Error`). A reserved word takes
  a trailing `_` (`Point.default_()`), and names that clash, such as one the runtime
  declares or a data enum's variant class, are kept apart as Kotlin's are. The glue's C
  names are `<prefix>_dart_`, with a `_` more for each C name of the export already there.
- **Objects** are final classes owning their handle. Of a checked type, `close()` drops
  the value through Rust's own `_drop`, as Rust's `drop` would, and frees the handle in
  the same crossing, and a `close()` while a call borrows the value, from a closure that
  call runs, throws a `StateError` instead, as Rust refuses that drop. Of any other type,
  `close()` frees the handle, and throws while a call holds it. A second `close()` does
  nothing. A returned object waits in the isolate's nursery of the last 64 returned, where
  `close()` frees it without the garbage collector's registration (a finalizable handle of
  Dart's C API), which costs more than the rest of a return and close. One never closed is
  registered once 64 later returns push it out, and freed once the garbage collector finds
  it unreachable after; one still in the nursery when the isolate exits is freed then. Any
  call that meets
  a closed object, as its receiver or anywhere in its arguments, throws a `StateError`
  before Rust sees it. A `self` method, or passing the object by value, moves its value
  out. `Clone` is `clone()`, and a `pub`
  field a getter and a setter.
- **Checks**: a borrow or move Rust's rules refuse throws a `StateError` naming the
  function and the rule, before any Rust code runs, so the call changed nothing.
- **Records** are final classes with a named, required parameter per field, and
  positional fields are `$0`, `$1`, and so on. **Plain enums** are Dart enums, and an enum
  with data a sealed class with a final class per variant (`EventMoved`), so a `switch`
  over it is exhaustive. `Display` is `toString()`, `Debug` `debug()`, `PartialEq` `==`, and
  `Hash` `hashCode`, all through Rust's; without `Hash`, `hashCode` throws an
  `UnsupportedError`, since no hash Dart computes agrees with Rust's `==`. `PartialOrd` is
  `<`, `<=`, `>`, `>=`, and `partialCompareTo`, null when unordered, through Rust's
  `partial_cmp`, and `Ord` makes it `Comparable`, whose `compareTo` is Rust's `cmp`. `Default` is a
  constructor, `default_()`. An
  **opaque value** is a final class of its bytes as `int`s, with a private constructor,
  so only Rust makes one. It never changes: a `&mut self` method, a field's setter
  included, returns the value as the call left it, `x = x.add(o)`, and beside Rust's own
  return, a record, with each element of a tuple Rust returns, `final (y, n) = x.take()`. A plain enum's `==` is Dart's, by its value.
- **Integers** are `int`, but for `u64` and `usize`, which are the runtime's `U64`: an
  extension type on the `int` of its 64 bits, costing nothing over it, which compares
  unsigned and converts, checked, to and from an `int` (`U64(5)`, `toInt()`) or a
  `BigInt`; `U64.fromBits` takes any `int`'s bits. Being an extension type, it prints as
  its `int` does, signed: `toRadixString` and `toBigInt` print it unsigned. A narrower
  integer passed in throws a `RangeError` when out of its range, as do a `usize` or
  `isize` past 32 bits on a 32-bit target, and a zero passed for a `NonZero` throws an
  `ArgumentError`. A `char` is a `String` of one Unicode scalar, and any other throws an
  `ArgumentError`.
- **Bytes** a call takes are a `Uint8List`, and text a `String`, which throws an
  `ArgumentError` passed in holding a lone surrogate, since UTF-8 has no form of one and
  Dart's encoder would replace it. Rust keeps Dart's own
  with one copy from 1 KiB, into a buffer Rust takes, and with two below, since allocating
  Rust's buffer costs more there than the second copy; a lent `Uint8List` Rust returned
  passes back as a second owner of Rust's buffer, with no copy. Bytes Rust returns are a `Uint8List` owned by Dart's memory
  manager: up to 64 KiB copied, with Rust's freed at the call, and a longer one an
  unmodifiable view of Rust's buffer, freed once neither it nor any view of it is
  reachable, so no view outlives its memory. A lent view costs a finalizer and charges the
  garbage collector its length, which costs more than a copy up to about 64 KiB. Text Rust
  returns is copied into a `String`. A path is the runtime's `PathBuf` of its bytes,
  crossing as bytes do, so one that is not UTF-8 crosses whole: `PathBuf(String)` makes
  one, and `toFilePath()` is its `String`, throwing a `FormatException` where it is not
  UTF-8. `Option<T>` is `T?`, a tuple a Dart record, and a `Vec<T>` or `&[T]` a
  `List<T>`, except one of a fixed-width integer other than `u8` (bytes) or `u128` or of a float, or a `Box<[T]>` of one, which
  is the `dart:typed_data` list of its width, such as a `Uint32List` or `Float64List`:
  copied whole into the call's memory passed in, and out of Rust's returned; a `u64`'s is
  a `List<U64>`, a `Uint64List` read as `U64`s. A `u128` is a `BigInt`, which throws an
  `ArgumentError` passed in out of its range. A `[u8; N]` is a generated `ArrayU8x<N>`,
  which copies the list it is made of and throws an `ArgumentError` unless it holds
  exactly `N` bytes. `Duration` is the runtime's `RustDuration`, its `U64` seconds and
  nanoseconds below one, which throws a `RangeError` made of a second's nanoseconds or
  more: `RustDuration.from` takes a `Duration`, throwing an `ArgumentError` for a negative
  one, and `toDuration()` gives one, throwing a `RangeError` for part of a microsecond or
  one past `Duration`'s longest. Addresses are the runtime's, as Rust's: `Ipv4Addr` and
  `Ipv6Addr` of their octets, each an `IpAddr`, a sealed class; and `SocketAddrV4` and
  `SocketAddrV6`, with its flow label and scope, each a `SocketAddr`, whose port, label,
  and scope throw a `RangeError` out of range. So the type holds only its family, and each
  crosses whole. `IpAddr.from`, `Ipv4Addr.from`, and `Ipv6Addr.from` take a `dart:io`
  `InternetAddress`, throwing an `ArgumentError` for a Unix one, one of the other family,
  or an IPv6 one with a scope, which Rust's has none of; `toInternetAddress()` gives one.
- **Errors**: a `Result`'s error is thrown. An exported error type is thrown as itself,
  implementing `Exception`, and any other in a final class `<T>Exception` holding it as
  `error`, such as `StringException`.
- **Traits** are abstract classes. Rust calls an implementation's methods on the isolate's
  thread, and once it drops one, calls its `close()`, which does nothing unless overridden.
  A call that throws before Rust sees it gives Rust none of its implementations.
- **Closures** are Dart functions, which Rust calls on the isolate's thread: one lent for
  a call only during the call, and one Rust keeps until it drops it, which then lets go of
  it. An object it lends by reference lives only for that call of the closure. What it
  returns Rust takes: a `String` copied, and an opaque value copied. One whose Rust returns `Result<T, String>` fails by throwing
  `StringException`, the binding's own form of that error; anything else it throws aborts.
- A **reader** Rust reads and seeks, a `Read + Seek` behind `&mut`, is a `SeekableReader`:
  `readInto` and `seek`, which Rust calls only during the call, on the isolate's thread.
  `SeekableReader.file` reads a `RandomAccessFile` through its synchronous methods, as
  Rust's calls are. A seek from `SeekFrom.start` takes a `u64`'s bits, as `U64.bits` holds
  them, so one past `i64`'s range is negative, which a file refuses. Rust's buffer is never
  `readInto`'s: it fills a list of Dart's own, which Rust copies. An `Exception` either
  throws fails the call into Rust as an `io::Error` of its text, and anything else it
  throws aborts.
- **Futures** are `Cancelable<T>`, a `Future<T>` completed on the isolate that made it:
  an extension type on Dart's own future, which `await` takes its shortest path through.
  The call makes and polls it in one crossing, `_start`, returning it complete if that
  poll finished, and makes the state that polls it again, and a waker, only if it pends. A wake, from any
  thread, posts to the isolate's port, and its event loop polls again, so a wake never
  polls inside Rust, and one that lands during a poll is kept. `cancel()` frees the
  future unless it finished, which cancels it in Rust and loses nothing: the next call
  takes what it would have, and it completes with `Cancelled`. A pending future keeps
  the isolate alive, as a timer does.
- A **future Dart implements** is any `Future<void>`, already running as Dart's futures
  are, which Rust polls until it completes, or which a poll owns, as
  `Waiter.pollFuture` does. Rust dropping it cancels it if it is a `Cancelable` still
  running, and one that fails while Rust holds it aborts.
- A **waker** is a `void Function()`, which runs on the isolate's event loop. One Rust
  returns is a `Waker`, an object whose `call()` wakes through Rust and whose `close()`
  drops it; one never closed is dropped once the garbage collector finds it unreachable,
  and the isolate's own waker keeps the isolate alive until then.
- **Threads**: Dart runs code only on its isolate's thread, so Rust reaches it from any
  other through the isolate's port, and waits for its event loop to run the call. While
  Rust holds an implementation, a closure, or a reader of the isolate, each call into Rust, a
  future's poll and `cancel()` included, runs on a thread of its own, acting as the
  caller's for the handles it borrows, while the isolate's thread runs what Rust calls
  into Dart meanwhile. Any other call runs on the isolate's thread, since nothing it waits
  on can call into Dart. A leaf, as Python keeps the GIL across, is a leaf call (`isLeaf`)
  to the C function on the isolate's thread, with no hop and no arena: a field read costs
  13 ns, against 31, and against 2 us while Rust holds a closure. A call that holds only
  its receiver holds it with no arena either. An isolate may move between threads, so a
  value that is not `Send` is made on a home, a thread Rust owns, which runs each call on
  it.
- **The contract** is checked at the first call, before any other: a Dart file and a
  library from different builds throw a `StateError`.

## IR

[`src/ir.rs`](src/ir.rs) is the API as each generator reads it, with no language decided:
records, enums, objects, callbacks, functions, and constants of every export, each by
its Rust path, its span, and its docs. A type is in the exact form Rust takes or returns it
(`&str` apart from `String`, `Box<dyn Trait>` apart from `impl Trait`), so a generator can
borrow where Rust borrows. [`src/abi.rs`](src/abi.rs) decides the C ABI from it once,
names and signatures, so [`src/c.rs`](src/c.rs), [`src/cpp.rs`](src/cpp.rs),
[`src/kotlin.rs`](src/kotlin.rs), [`src/python.rs`](src/python.rs),
[`src/swift.rs`](src/swift.rs), [`src/go.rs`](src/go.rs), [`src/dart.rs`](src/dart.rs), and
[`src/glue.rs`](src/glue.rs)
agree; the runtime the glue includes is [`runtime/rt.rs`](runtime/rt.rs), the one the C++
header includes [`runtime/rt.hpp`](runtime/rt.hpp), the ones Kotlin and its JNI glue
include [`runtime/rt.kt`](runtime/rt.kt) and [`runtime/jni.rs`](runtime/jni.rs), the ones
Python and its extension include [`runtime/rt.py`](runtime/rt.py),
[`runtime/native.py`](runtime/native.py), and [`runtime/python.rs`](runtime/python.rs),
the one the Swift includes [`runtime/rt.swift`](runtime/rt.swift), the ones the Go's
runtime package and each crate's include [`runtime/rt.go`](runtime/rt.go) and
[`runtime/ffi_cgo.go`](runtime/ffi_cgo.go), and the ones the Dart and its glue include
[`runtime/rt.dart`](runtime/rt.dart) and [`runtime/dart.rs`](runtime/dart.rs).

## Census

`--census dir` exports nothing. It writes `<package>.md` for the facade, which counts what exports and ranks every refusal by construct and by the type it names,
lists each refused item at its span, a binding's own refusals such as Swift's included,
each of which fails the export, and what else it left out, such as what C can never hold
and the traits C cannot implement, and why. It fails on an `instance not inferred` refusal,
which is a bug in the exporter's inference
rather than in the crate.

## TypeScript (web)

`web/` is TypeScript over the C ABI of the crate built for `wasm32-wasip1` with `--features
web`, tested on Node 24 and Deno 2, which run it as it is, and nightly in headless Chromium
and Firefox, bundled by Bun. Each file is an ES module a bundler takes as it is, importing
its neighbors by their `.ts` names; the `.wasm` needs no tool run on it after `cargo build`,
and no wasm-bindgen. The binding serves
`wasm32-wasip1` alone: an item another target has and it lacks is left out, not declared to
fail. `load(source)` of `index.ts` compiles and instantiates the module, from its bytes, a
`Response`, a `WebAssembly.Module`, or its URL, before any other call, and checks its
contract and target there. Without a source it loads `<prefix>.wasm` beside `index.ts`,
through `new URL("./<prefix>.wasm", import.meta.url)`, which Vite and webpack follow into
the bundle; a `file:` URL is read through Node's file system, where `process.getBuiltinModule`
finds one, since Node's `fetch` refuses the scheme. A runtime with source phase imports,
Node 24 and Deno, may pass `import source module from "<package>/<prefix>.wasm"` instead.

`web/` is an npm package too, since Node strips no type under `node_modules`
([nodejs#57215](https://github.com/nodejs/node/issues/57215)): beside each `.ts` the
export writes its `.js`, the types stripped by [oxc](https://oxc.rs) and each import named
by its `.js` file, and its `.d.ts`, declared from that file alone, as JSR requires too, so
every export states its type. Its `package.json` names it as the facade's package and
version, exports `index` as the package and each other module by its name, such as
`<package>/Counter` and `<package>/rt`, and the module as `<package>/<prefix>.wasm`, and
ships the JavaScript and declarations with the module `cargo build` puts beside them.
[The release builder](release/README.md) packs it for npm and JSR. `tests/fixtures.rs`
packs the api fixture's with npm, installs it into a project of its own, type-checks
`tests/api.ts` against the declarations alone, and runs it on Node, Deno, and Bun.

- **The platform** is the web's: the module's std is `wasm32-wasip1`'s, whose imports are a
  detail between the generated module and its binding, which implements each with the web
  API for it, so nothing of WASI ships. Its clocks are `performance.now`, from
  `performance.timeOrigin` for the real-time one; its randomness `crypto.getRandomValues`;
  its stdout and stderr `console.log` and `console.error`, a line at a time; its
  environment and arguments empty; and its exit an abort. What else the module would
  import, such as the file system, sockets, or another module's function, is refused at
  export, at the span of each call in the crate's own code that reaches it, naming the
  imports it reaches and the path down to them. The export walks MIR from each exported
  function, and each one a crate's `.init_array` static runs when the module loads,
  through every call, `dyn` implementation, function pointer, static, and drop it can
  reach, std's included, and into C and whatever Rust the C calls back, so a new std needs
  no change here: every crate builds with `-Zalways-encode-mir`, std's from the dev
  shell's MIR sysroot or else `-Zbuild-std`, which doubles the export's peak memory, as
  [measured](../bench/ffi/README.md#the-exports-build-cost); a native export reads no MIR.
  A C function or static no linked library defines, which would fail the module's link, is
  refused the same way, as is a call into wasm-bindgen, whose imports on `wasm32-wasip1`
  compile to stubs that panic. `thread::Builder::spawn` and a socket's `connect` and `bind`
  import nothing there and return an error, as Rust's own build for the target does, so
  they are not refused; `thread::spawn` panics on that error, which aborts the module. The
  walk checks the generated crate, so unlike the export's other refusals it runs once
  `out/` is written, and a refused export leaves that `out/` behind, failed, as a refused
  `Cargo.lock` does. `load` still refuses a module importing anything else. Measured on
  the api fixture, stripped, with LTO at `opt-level = "s"`, that std costs 24 KB over
  `wasm32-unknown-unknown`'s (3%), 9.5 KB gzipped, and the glue 2 KB more;
  `wasm32-unknown-unknown` has no clock, so `Instant::now` panics there.
- **The API** is rust2-ts's shape: each type is a module of its own, `web/<Type>.ts`, which
  exports the type by its name and its functions, each taking `self` first, so a caller
  writes `import * as Counter from "./web/Counter.ts"` and `Counter.get(c)`, and a bundler
  drops every function no caller names. `index.ts` holds the free functions, the constants,
  and `load`. Names are the C names without the prefix: a type in Pascal case, and a
  function in camel case, a constant's in upper case; a reserved word is exported as is
  from a local with a trailing `$`, so `Counter.new("c", 1)` and `Point.default()` work
  through the module's namespace.
- **Values**: a record is a plain object of its fields in camel case, positional ones `_0`,
  `_1`, and so on; an opaque value an `rt.Opaque`, its bytes, which only Rust makes. A plain
  enum is a union of its variants' names, and an enum with data a union of objects tagged
  by `tag`. A tuple is an array, `Option<T>` is `T | undefined`, or `T | rt.None` where `T`
  can already be `undefined`, `Result<T, E>` an `rt.Result`, `{ ok: true, value }` or `{ ok:
  false, error }`, and an error is that value, never thrown. `u8` through `u32`, `usize`,
  `isize`, `f32`, `f64`, and `char` are numbers, each checked as it crosses in; `u64`, `i64`,
  and `u128` are rust2-ts's `rt.u64` and so on, a number while a safe integer and 32-bit
  limbs past it, converted from the `bigint` a wasm `i64` is at the boundary, which costs a
  getter 9% over returning that `bigint` as it is. `String` and `&str` are strings, bytes a
  `Uint8Array`, or an `rt.Bytes` where a call returns them (below), a `Vec` of numbers the
  typed array holding them exactly, any other `Vec` an array, a path an `rt.PathBuf` of its
  bytes, and `Duration`, the addresses, and the ranges plain objects. A `&mut` value is an `rt.Place`, whose `v` the call writes back, and a
  value's `&mut self` method returns it first, as Dart's does.
- **Memory**: each call lowers what it passes onto a stack of the binding's in linear
  memory, which it gives back once the call returns, and reads what Rust returns at the
  layouts C's wasm32 ABI gives the header's structs, passing a struct of one scalar as it,
  any other through a pointer to a copy, and one returned through a pointer passed first.
  Text Rust returns is copied out, and its buffer freed at once, so no view outlives a
  growth of memory, which detaches it; what Rust lends a closure or trait is copied before
  any call it makes can grow memory.
- **Bytes** a call returns, its own result or inside an `Option`, `rt.Result`, or `rt.Call`,
  are an `rt.Bytes`, the runtime's own type: `length`, `at(i)`, iteration, and `slice()`,
  which copies them into the idiomatic `Uint8Array`. Past 64 bytes it lends Rust's buffer
  in place, read through memory as it is at each read, so a growth detaches nothing a
  caller holds; up to 64 it holds a copy, since an engine keeps a typed array that short
  in its own heap, which costs less than a lend. Disposing it, as `using` does, frees Rust's
  buffer once no call reading it in place is running, and a read after throws `Bytes is
  closed`, whichever it holds. Lent bytes wait in the nursery objects do (below), and
  one never disposed then waits for the `FinalizationRegistry`,
  which runs only once the collector does for its own heap, which no lent buffer grows: so
  past a quarter of memory lent and not yet freed, or once memory has grown by a third
  since none were, a call copies instead, so bytes nobody disposes grow memory by no more
  than a third, even a slice keeping a larger buffer whole. Held lent bytes therefore
  turn later returns into copies once memory grows a third, until they are disposed. Passed to a call, an `rt.Bytes` crosses
  in place for one that reads it, as a second owner of Rust's buffer for one that takes a
  `Bytes`, and copied by Rust for one that takes a `Vec`, which owns its bytes alone. Bytes
  inside another value, such as a record's field or a list's item, are copied into a
  `Uint8Array`, since only a caller holding them alone disposes them. No view of Rust's buffer reaches a caller, as a `Uint8Array` over memory
  would: it detaches once memory grows, writes what Rust's other owners of the bytes read,
  and through a `subarray` or its `buffer` outlives the buffer it views. On Node 24, a lent
  payload costs 0.2 µs a call at any length, against a copy's 0.5 µs at 100 B, 1.1 µs at
  1 KiB, and 97 µs at 1 MiB; [the benchmark](../bench/ffi/README.md#lending-returned-bytes)
  has each engine's.
- **Objects** are an `rt.Handle`, disposed by `[Symbol.dispose]`, as `using` does; where the engine lacks that symbol, as stable Safari does, `rt` defines it as `Symbol.for("Symbol.dispose")`, which esbuild's `using` falls back to too. Of a
  checked type, disposing it drops the value through Rust's own `_drop`, which fails while
  a call borrows it; of any other, it throws while a call holds it. The handle is freed once
  disposed and no call holds it, so one disposed while a later argument lowers fails the
  call's check rather than reach Rust freed. A call that meets a disposed handle throws
  `<Type> is closed`. A returned object waits in a nursery of the last 64 objects and lent
  bytes returned, where disposing it frees it without a `FinalizationRegistry` entry, which
  costs more than the rest of a return and its disposal. One never disposed registers once
  pushed out, by 64 later returns or once a collection finds the nursery's sentinel, which
  then pushes out all it holds, and the registry frees it once the collector finds it
  unreachable after. On Node 24, a returned object disposed costs 0.18 µs, against 0.36
  µs with the registry entry and 0.1 µs for the C calls alone.
- **Traits** are interfaces JavaScript implements, with an optional `close()` Rust calls
  once it drops an implementation, and **closures** are functions: each is a C function
  the module imports from `rust2`, whose data is a token the binding finds the value by, so
  JavaScript never writes the module's function table. Its std traits are functions of its
  module: `toString`, `debug`, an object's `clone`, `eq`, with `Hash` `hash`, `partialCmp`
  from `PartialOrd`, undefined when unordered, `cmp` from `Ord`, the shape
  `Array.prototype.sort` takes, and `default`.
- **Futures** are an `rt.Call`, a `Promise` with `cancel()`: the call polls once as it is
  made, and each wake polls again on a microtask, never inside the wake. Cancelling frees
  the future in Rust, losing nothing, and rejects with `rt.Cancelled`. A future JavaScript
  implements is any promise, a waker a function, and a reader an `rt.Reader`.
- **Failure**: a run-time check that fails throws an `Error` naming the C function and the
  check, as each binding raises. Rust's panic aborts the module, as do a throw out of a
  closure, trait, or reader, since Rust has nowhere to unwind to: the message goes to
  `console.error`, and every later call throws, since nothing of Rust's state is sound.
- **Threads**: none. wasm32 runs Rust on JavaScript's one thread, with no shared memory and
  no atomics, so no check of a value that is not `Send` or `Sync` can fail.

## Tests

```bash
just ffi test     # export the fixtures, build them, and drive api from C, C++, Kotlin, Python, Swift, Go, Dart, and TypeScript on Node, Deno, and Bun
just ffi moq      # export moq's crates at the pinned revision, build them, and run moq/moq.c, moq/moq.cpp, moq/moq.kt, moq/moq.py, moq/moq.swift, moq/moq.go, and moq/moq.dart
just ffi moq-c    # the same export, built in debug, running only moq/moq.c: `just ci`'s check on moq
just ffi moq-cpp  # the same export, running only moq/moq.cpp
just ffi moq-swift  # the same export, running only moq/moq.swift
just ffi moq-api <target>  # stage moq's API on another target into target/moq-api, which the exports above merge
just ffi census   # census each into target/census
just ffi shapes   # round-trip every sampled type shape in every language but KNOWN's, as the nightly does
just ffi corpus   # census, export, and bind each crate of corpus/crates.txt through a facade, as the nightly does; it runs their build scripts
just ffi corpus-update  # take corpus/crates.txt afresh from crates.io's top crates, and resolve corpus/Cargo.lock
just ffi protocol # run longer protocol scripts from more seeds through the C ABI and every binding, as the nightly does
nix develop .#bench --command just ffi browsers  # api's TypeScript in headless Chromium, Firefox, and on Linux WebKit, lowering `using` three ways for WebKit, as the nightly runs it
nix develop .#android --command just ffi android  # the Kotlin test, shrunk by R8, under CheckJNI on the Android device adb reaches
nix develop .#bench --command just ffi bench  # the binding benchmark: bench/ffi/README.md's cases from Rust, and through the export from C, C++, Kotlin, Python, Swift, Go, and Dart
nix develop .#bench --command just ffi codspeed  # after bench-build, each single-threaded case once under CodSpeed's CPU simulation, as the CodSpeed workflow runs it
nix develop .#sanitize --command just ffi sanitize  # api's C and C++ scripts under ASan with UBSan, TSan, and Valgrind, its Swift under TSan, its Go under the race detector and cgocheck2, its Python looped on a debug CPython, and its C ABI under Miri
```

The `api` fixture covers each rule once, and re-exports the `dep` fixture's types, which
export into the same library. The fixture's binding serves the host and wasm32, whose
items the host's library lacks, and each language's test checks that the host's items work
and that wasm32's fail. Each language's script is written at test time by
`tests/fixtures/transcript` from the fixture's `examples/transcript.rs`, whose `main` is a
typed subset of Rust checked against the export's `api.json`: an item it cannot resolve, or
a construct a language's writer cannot spell, fails the test naming it. Each function the
example defines is a hook each language's test writes by hand under the same name, for what
tests its idiom, such as futures and readers, or prints a value no two bindings shape alike.
A new rule's case is a few lines of `main`, which every language then runs.
`tests/api.c` runs the script through the C, and its transcript must match the native one
line for line; then it breaks each runtime check, which must abort with its message. `tests/api.cpp`
runs the script through the C++ header at C++17, 20, and 23, with exceptions and RTTI off
at 17 and 23, builds it under Clang's `-Wconsumed -Wdangling -Werror`, each case of
`tests/api_strict.cpp` failing there, then checks its futures: a wake that lands during a poll, a cancel between a
wake and its poll that loses nothing, and a coroutine resumed or destroyed. `tests/api.kt`
runs the script through the Kotlin on the JVM under CheckJNI, where a warning fails it, checks its futures the same way, that a call
after `close()` throws, that `Bytes` and their slices keep Rust's buffer, past the object or
record they came from, until the GC frees it, that calls racing `close()` or the GC
read live values, that the GC frees an object never closed left in an idle nursery or
pushed out of one, and that each object of many threads' returns, closes, and drops is
freed once, and breaks each runtime check, which must throw or abort with its message. `just ffi android` runs
the same on ART: the Kotlin and `tests/api.kt`, shrunk by R8 into an app's dex with only the
generated rules keeping what JNI looks up, against the library built by the NDK for the
device's ABI, which ANDROID_HOME and ANDROID_NDK_HOME locate, under ART's CheckJNI. It also
reads the library's segment alignment with `readelf`, which must be 16 KB, and checks that a
failed check and a panic reach logcat and bionic's abort message, which the tombstone shows.
Nightly runs it on an x86_64 emulator at API 24, the oldest moq's Android library supports, at
API 35, and at API 35 on an image whose kernel pages by 16 KB, and on a 32-bit x86 one at API
24, as moq's armv7 build is 32-bit.
`tests/api.py` runs the script through the Python, in CPython's development mode with
warnings as errors, checks its futures the same way, what frees an object or bytes nobody
closed, and the values Python's types bound, and breaks each runtime check, which must raise or
abort with its message; the Python with another contract must refuse the library. `api_free_threaded` imports it under
nixpkgs' free-threaded CPython, which the dev shell names in `PYTHON_FREE_THREADED`, and checks the `ImportError`.
`api_py_types` checks the Python with mypy and pyright, both strict, at CPython 3.11, as
`just ffi moq` checks moq's.
`tests/api.swift` runs the script through the Swift, built as a module in Swift 6's
language mode and type-checked in Swift 5's, checks its futures the same way and that
releasing frees, and breaks each runtime check, which must trap or abort with its message,
and each move or borrow the compiler checks, which must fail to compile.
It runs with nixpkgs' Swift on Linux, and nightly with Apple's on macOS.
`tests/api.go` runs the script through the Go after `go vet`, failing any run that leaves a
goroutine Go's `goroutineleak` profile finds, checks its futures the same
way, plus a context done before the call, a deadline, and a handle closed during a call,
checks that bytes cross both ways, that a `Bytes` keeps Rust's buffer while only a sub-view
or a reader reaches it and that the GC frees it once none does, that the GC frees an object
never closed once 64 later returns push it out of its nursery, that each object of many
goroutines' returns, closes, and drops is freed once, and that each kind of call meeting a closed object, including one racing
`Close`, returns `rt.ErrClosed`, and breaks
each runtime check, which must panic with its message; a runtime package from another
build must fail its contract check. The `target` test exports a package for the host, for wasm32, and for both into one
binding, which builds for each, and staged with `--api` and `--with`; a build for a target
not exported fails, and items that differ between the two are refused, naming both.
The shape tests (`tests/fixtures/shapes.rs`) sample value types from the IR's `Type`, each
leaf alone and in each wrapper, then nested to depth 3 from a fixed seed, and emit a crate
of `make_N(seed)`, `echo_N(v)`, and `digest_N(&v)` per shape, with the shape in each
position a binding maps apart: a closure's parameter and return, a future, a `Result`'s
value and error, a `&mut` written back, beside other parameters, an `Option<&T>`, an
object's methods, and a trait a caller implements. A digest hashes floats by their bits,
so a NaN's payload counts, and the sampler draws signaling and quiet NaNs. Its census
decides what exports, so a shape refused by design is left out rather than failed. Each
language's driver, the web's on Node included, is a generated list of these calls whose
printed digests must match the crate's own, run natively, or as wasm under Node's WASI
for the web, where `usize` is 32 bits. The gate runs a subset; `just ffi shapes` runs
every sampled shape and, as the nightly does, more drawn from the day's seed, which it
prints and `RUST2_SHAPES_SEED` replays. `KNOWN` lists what a binding still gets wrong,
each with the quest that fixes it; the driver skips those calls, so the fix deletes its
entry, and the nightly fails on an entry that matches no call of the fixed set. A new IR
`Type` variant fails `ir_covered` until the sampler covers it.
The corpus (`tests/fixtures/corpus.rs`) exports stock crates as a consumer would: each
crate of `corpus/crates.txt`, the top crates on crates.io by downloads that are libraries
building as they are, pinned by `corpus/Cargo.lock`, through a facade of its own, `pub use
<crate>::*;`. Each is censused, then, where the census refuses nothing, exported, and
its glue, C and C++ programs including its headers (compiled, not linked, since a crate
may call a platform library the host lacks), its Kotlin, Python, Swift, Go, and Dart
built; a panic, an error, or a binding that does not build fails it, a refusal not.
Its `KNOWN` lists the crates it fails on today, each with the quest that fixes it or why
none can, which it skips. The nightly runs it in four shards, `CORPUS_SHARD=i/4`, and each writes a report
of every crate's refusals and failures.
The protocol tests run random sequences of borrows, exclusive lends, moves, drops, frees,
polls, and cancels over the api fixture's checked types, on the thread that made a value
and another, against `tests/protocol/model.rs`: an executable model of what this README
says each step returns, the status a check fails with included. A seed makes a script of
steps and the model's transcript of their outcomes, shaped by what a driver's surface can
run, and each driver runs the script through its surface and prints the outcomes back, which
must match line for line, with the counts of the script's waker where it polls with one.
`tests/protocol.c` runs them through the C ABI, `tests/miri/protocol.rs` under Miri, and
`just ffi sanitize` runs the C driver under ASan and TSan. Each binding runs them through
its own wrapper too, from `tests/protocol.cpp`, `.swift`, `.py`, `.go`, `.kt`, `.dart`, and
`.ts`: its `Profile` in the model leaves out what the binding cannot write, such as a free
where its collector frees or a failed check where it aborts, and adds what the binding does
its own way, such as a close that refuses before Rust's check, a future made only as it is
awaited, or one made on a home. `protocol_profiles` checks that each flag keeps its
scripts to it. The gate runs 500 scripts of 100 steps through the C ABI, failing unless
they meet every status and make every kind of handle, and 40 to 500 through each binding;
`just ffi protocol`, run nightly, runs 20,000 of 400 steps through the C ABI and 1,000 to
5,000 through each binding.
`tests/borrow.rs` and `tests/hold.rs` model the borrow check and the bindings' hold counts
under loom, and `tests/future.rs` and `tests/thread.rs` model the runtime's futures and
threads, each mounting the runtime's own source over loom's primitives. The futures' model
races a wake from another thread against a poll, a change of waker, a start, and a free:
each C waker drops once and is never woken after, a pending poll is always woken, and a
future that is not `Send` refuses a poll elsewhere and drops on its own thread. The threads'
model races a free elsewhere against its thread's check, wake, drain, and exit, and lends
calls to homes that call each other: each value drops once on its own thread, or leaks once
that thread exited, and each home goes idle once no value or call holds it. Each explores
every interleaving, or for homes those within a preemption bound, in the gate.
The `reject` fixture's expected errors, each with its notes, are `fixtures/reject/expected.txt`.
The `lock` test exports the `lock` fixture for wasm32, staged, and the host, then again
against that export's `api.json`: its `added` feature only adds and must pass, and its `next`
feature breaks the API twice for each kind of break, once in an item only wasm32 has, each
error matching `fixtures/lock/expected.txt`.
`just ffi sanitize`, run nightly, rebuilds the `api` export with std under AddressSanitizer
and then ThreadSanitizer, runs `tests/api.c` and `tests/api.cpp` against it, the first time
with UndefinedBehaviorSanitizer too, and `tests/api.swift` against the second, and matches
their output to the native transcript; it runs `tests/api.c` and `tests/api.cpp` against the
uninstrumented library under Valgrind's memcheck, `tests/api.go` under Go's race detector
and cgocheck2, and `tests/api.py`'s checks on a debug build of the newest CPython, then its
`leaks` case, which loops them and fails unless the interpreter's references and memory
blocks end where they began; and it drives the C ABI from `tests/miri/api.rs` and the
protocol scripts from `tests/miri/protocol.rs` under Miri with strict provenance. The
protocol scripts' C driver runs under each sanitizer too.
moq's crates export through `target/moq-ffi`, an empty library on each that `just ffi moq`
writes in place of moq-ffi's remainder. The export serves the host and each target
`just ffi moq-api` staged in `target/moq-api` from the same moq sources, and fails on a
staged API from other sources. Nightly stages every target moq ships a binding to but the
host on the host with its C toolchain, Android's in the Android shell from the runner's NDK,
and runs `just ffi moq` on Linux over all of them, so an item two targets define
differently fails it. `moq/moq.c` publishes and subscribes to a moq-net
track in one process, polling each read, scopes an origin to moq-pattern's patterns and
hands it to a client to publish, polls its driver until the origin's last owner drops, reads an OpusHead into hang's catalog through moq-mux,
sets and reads a field of the catalog lent to C through `catalog::Producer::mutate`, and
serves a broadcast from a moq-tokio server bound to port 0 to a client that reads it over
QUIC, both on moq-tokio's fallback runtime, whose threads wake C's. `moq/moq.cpp` does the
same through the C++ header, short of the catalog's field, at C++17 without exceptions or
RTTI, as OBS builds it, serving on an executor's thread and blocking on each read, and
`moq/moq.kt` through the Kotlin, serving in a coroutine and suspending on each read,
`moq/moq.py` through the Python, running the origin's driver and the server as asyncio
tasks and awaiting each read, `moq/moq.swift` through the Swift, serving in a task and
awaiting each read, `moq/moq.go` through the Go, running the origin's driver on a
goroutine of its own and serving on another, each read blocking under a context, and
`moq/moq.dart` through the Dart, polling the origin's driver from the isolate's event loop
on each wake and at its deadline's `Timer`, and awaiting each read.

The C++ builds take `CXX`, or `c++`, with every warning an error and `-Wshadow`, which
stands in for MSVC's warnings that a name hides another. On Windows they take MSVC's `cl`
from a developer environment, at `/W4` with `/permissive-`, and `/std:c++latest` for 23.
They link what rustc prints a program linking the library needs. Nightly runs
`just ffi test cpp` and `just ffi moq-cpp` with Clang on Linux and macOS and with MSVC on
Windows.
