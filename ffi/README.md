# rust2-ffi

A rustc driver that exports a crate's `pub` API as a C ABI and a C header, with no
attributes in the Rust source.
Every language's generator but Python's builds on the C ABI.

```bash
rust2-ffi --config ffi.toml (--out dir | --census dir) [--manifest-path Cargo.toml] [--target triple] [-- cargo args]
```

It runs `cargo check` on the package with itself as `RUSTC_WRAPPER`, for `--target` or
else the host, whatever cargo's config names, through
the front end it shares with [rust2-ts](../ts/README.md) in [`driver/`](../driver). The
package is the facade: once rustc has checked its library, it walks every `pub` item
reachable from the crate root, following `pub use` re-exports of other crates' items and
modules, and exports it by fixed rules, or refuses it with a compile error at its span.
So a facade such as moq-ffi exports several crates into one library by re-exporting what
it ships, and a type no path from its root reaches is refused. The prefix of every C name
is the library's crate name, Cargo's `[lib] name`, which defaults to the package's name in
snake case. `--out` writes a crate of `extern "C"` functions over the facade, its runtime
beside it, and the header in `include/<prefix>.h`; `cargo build` on that crate makes a
static and a shared library for that target alone: its build script refuses another, on
which an item the target compiles would be missing, or one it leaves out left over. So a
`cfg`-gated item is in its target's export and in no other's, and the layouts, `usize`
included, are that target's. That crate enables the facade's features as the check did;
a feature named in a dependency, `--features <dependency>/<feature>`, is an error, since
the facade enables what its dependencies build with. It resolves them as the check did,
wherever it is built: its manifest carries, as a `[patch]`, each patch or `[replace]` cargo applied
to what the library builds with on any platform, read off cargo's resolution, wherever
it was declared (the manifest, cargo's config, `--config`, or the environment), keyed by
its source's URL.
A `[replace]` from the source it replaces, which no `[patch]` can carry, is an error.
Beside the C header, `include/<prefix>.hpp` holds the same API in C++,
`kotlin/<prefix>.kt` holds it in Kotlin, over JNI functions the crate builds with
`--features kotlin`, and `kotlin/<prefix>.pro` the R8 and ProGuard rules keeping what
those functions look up by name. `python/<prefix>.py` holds it in Python, over a CPython
extension the crate builds with `--features python`, `swift/<prefix>.swift` holds it in
Swift, over the C header as a clang module, `go/` holds it in Go, a package per crate
over the C ABI through cgo, and `dart/<prefix>.dart` holds it in Dart, over the C ABI through
`dart:ffi` and what the crate builds with `--features dart`.

```toml
# ffi.toml, beside the package's Cargo.toml unless --manifest-path says otherwise.
# Paths from the facade's root left out: a module, an item, or one method or field. Each
# must name something exported, or what a `cfg` leaves out of this build, as moq-tokio's
# `unix` is on Windows. Nothing else leaves an item out. As in rustdoc, a path a
# method or constant shares with a field names the method, and `field@` the field.
# A `cfg`-stripped item is found through a glob re-export only while its source module
# keeps an exported item; under a glob of a module a `cfg` empties, the exclude fails as
# naming nothing exported.
exclude = ["track::Producer::poll_closed"]
```

## Rules

Every `pub` item reachable from the facade's root, by its shortest path, is exported or
refused. `#[doc(hidden)]` items are no part of the API and are left out; type aliases
export nothing of their own. A re-export of the standard library's item is refused, since
its types map by role. A `pub use` of a whole crate, as in `pub use rustls;` or
`pub use web_transport_proto as proto;`, is refused once at that `pub use`, since it puts
every release of that crate in the facade's API; a glob of its root, `pub use dep::*;`,
re-exports its items instead, which export. A type no path reaches is refused.

| Rust | C |
| --- | --- |
| a struct whose fields are all `pub`, not `#[non_exhaustive]`, which map, with no `Drop` of its own | a record: a struct, copied across |
| an enum whose fields map, with no `Drop` of its own | a C enum without fields, or a struct of a tag and a union |
| any other type, if `Send` | an object: an opaque handle to an `Arc`, freed with `_free` |
| a type whose only generic parameters are lifetimes, such as `Path<'a>` | an object holding its `'static` form |
| a generic type, at each instance it infers | that instance as any other type, with each method whose impl and own bounds it meets; any other instance is refused |
| a `pub` field of an object | a getter returning a copy, and a setter assigning in place, as `x.field = value` does |
| a `pub` trait | a callback interface: a table of C functions and their data; a trait C cannot implement, such as one with a blanket impl, an associated type, or a supertrait but a marker, is left out, since C never passes one, and a signature taking one from C is refused |
| a type parameter or `impl` bounded by `Fn`, `FnMut`, or `FnOnce`, which no lifetime bound or return lets outlive the call | a C function and its `void *` data, which Rust calls only during the call; any other closure is refused |
| `fn` and inherent `pub` methods | functions named through their type |
| `Result<T, E>` | `bool f(..., T *out, E *err)`; an `E` of another crate that implements `Display`, such as `anyhow::Error` or `io::Error`, is `String`, its message |
| a returned tuple `(A, B)` | one out pointer each: `void f(..., A *out0, B *out1)` |
| a tuple inside another type, or as a field or parameter | `<prefix>_tuple_<A>_<B>`: a struct of fields `_0`, `_1`, and so on; of references only returned |
| `async fn`, or a return that implements `Future` | a future: `f` makes it, `f_poll` polls it, `f_future_free` drops it; a function that is not `async` runs at the call, as in Rust |
| `poll_x`, when `x` exports as a future | left out: the future is its poll |
| `bool`, integers but 128-bit, floats, `char` | `bool`, `<stdint.h>` types, `float`, `double`, a `uint32_t` code point |
| `&str`, `String`, `&[u8]`, `Vec<u8>`, `bytes::Bytes` | `<prefix>_buf`: a view passed in, an owned buffer returned |
| `&Path`, `PathBuf` | `<prefix>_buf` of the path's bytes as the OS holds them on Unix, which need not be UTF-8, else UTF-8 |
| `Duration` | `<prefix>_duration` |
| `SocketAddr`, `SocketAddrV4`, `SocketAddrV6` | `<prefix>_socket_addr`: its IP's bytes, port, and whether it is IPv6; one of the other family passed to `V4` or `V6` aborts |
| `url::Url` | an object `<prefix>_Url`, made by `<prefix>_Url_parse`, which fails with the parse error's message, and printed by `_to_string` |
| `Arc<T>` of another crate's `Send` and `Sync` type that no other rule maps, such as `Arc<rustls::ServerConfig>` | an opaque object named by `T`'s crate and name, such as `<prefix>_rustls_ServerConfig`, with `_clone` and `_free` and none of `T`'s API: it only passes back to Rust |
| `serde_json::Value` | an object `<prefix>_Json`, made by `<prefix>_Json_parse` from JSON text, which fails with serde_json's message, and read back by `_to_text`; its contents are the caller's JSON library's to read |
| `Vec<T>` and `&[T]` of any other `T` | `<prefix>_vec_<T>`: an array of `T`'s C values, viewed passed in and owned returned, a slice as a clone; of references only returned |
| `BTreeSet<T>`, `BTreeMap<K, V>`, and `serde_json::Map` | as `Vec<T>`, `Vec<(K, V)>`, and `Vec<(String, serde_json::Value)>`: its items in order, from which Rust builds it, keeping the last of a key |
| a returned iterator, such as `slice::Iter` or an `impl Iterator`, that no other rule maps, outside a future, made of core's and alloc's iterators over a slice, string, `Option`, `Result`, or collection, through `rev`, `chain`, `flatten`, `fuse`, `enumerate`, `peekable`, `copied`, or `cloned`, with no closure, function, or trait object inside | what `collect::<Vec<_>>()` makes of it: every item, taken at the call |
| `Waker`, taken by value | `<prefix>_waker`, as a future's poll takes it |
| `Range<T>`, `Bound<T>` of an integer | `<prefix>_range_<T>`, `<prefix>_bound_<T>`: a `<prefix>_bound` kind and a value |
| `impl RangeBounds<T>` of an integer | `<prefix>_bounds_<T>`: its start and end bounds |
| `Option<T>` | a null handle or buffer, or `<prefix>_option_<T>` |
| a `pub const` | a `#define` if a `bool` or integer, else a function returning it |
| `impl Into<X>`, `impl AsRef<X>`, or a bound implying one, as a supertrait does | `X`, or `&X` and then `String` or `Vec<T>` where `X` is `str` or `[T]` |
| `impl Into<X>`, where `X` does not map | what a `From` implementation into `X` takes: a source parameter by its bounds as above, so `impl<B: Into<Bytes>> From<B> for Timed<Bytes>` takes `Bytes`, and by each `Into<Y>` whose `Y` does not map, what converts into `Y` in turn, each `Y` once and refusing a chain longer than the crate's recursion limit; any other source as it is, so `impl<'a, V> From<&'a V> for Timed<&'a V>` takes `&V`. Two sources C passes as one buffer, such as `String` and `Vec<u8>`, are refused, unless they are forms of one value, as `&str` and `String` are |
| `impl IntoIterator<Item = X>`, where `X` is no type parameter | `Vec<X>`, or `&[Y]` where `X` is `&Y` |
| `impl io::Read` | `&[u8]` |
| another type parameter | `serde_json::Value` for a serde `Serialize` or `Deserialize` bound, else the first of `String`, `Bytes`, and `Vec<u8>` meeting its bounds, or a foreign implementation of its one exported trait; else an instance of the function at each type the exports implement a trait bounding it for, through a blanket impl at each type implementing what the impl requires, as `&X` where that meets its bounds too and no bound, its own or one a supertrait implies, makes it outlive the call, so C keeps its handle, and, as an argument of a generic type, at each argument of an instance that meets its bounds and makes each generic type the signature names an instance |

A generic type infers the instances it exports at, with no config: the one at its
defaults, if each type parameter has one; the one each inherent impl is of, such as
`impl Pair<u32>`; each choice, for each parameter, among its default and the types the
exports implement a trait bounding it for, one of the exports', which are `()`, a
primitive, `String`, or an exported type or an instance of one, such as
`impl Seed<u64> for Wrap<u32>`; and each instance a signature, `pub` field, or constant
names at any choice its bounds allow, whether or not the rest of it maps, which may name
more. Inference finishes before any type is mapped, so mapping never adds an instance.
An instance that holds an instance of a type inside it, such as `Cell<Cell<u8>>`, is
refused, as it could name ever deeper ones.

A record or enum holds only values and `Copy` handles: one with a field that does not map,
or that holds itself, is an object instead, and the census lists it. A borrowed return is
returned as a clone, so it must be `Clone`, and a handle in it is a new handle to the
clone, but for an object a `&self` method of an object returns borrowed from its receiver
alone: that one, if not `Clone`, is a lent handle, which holds its receiver and checks it
on each use, on one thread at a time where the receiver is not `Sync`. A handle outlives every call, so every lifetime an
object takes is `'static`, as is every lifetime that must outlive one; any other lifetime
is the call's. A signature may not lend C's borrow for one that is `'static`, but for a
`&str`, which Rust interns, nor return an object that borrows. A parameter is a reference
only at its top or as `Some` of an `Option`, lent as the reference alone is, so
`Option<&T>` is a null or borrowed handle, a null or viewed buffer, or an option of a
value, and not inside a `Box` or any other type. A `&mut` of a value, but a `str` or a
slice, is in and out, and not inside an `Option`, a callback, or a future, nor borrowed
by what the call returns; a `&mut` of another crate's `Arc` is refused. Only a bare
returned `&Object` is lent, never one inside what a call returns nor its error. What C could never hold, an
object no export C can call returns or lends to a closure, a generic type no export
instantiates, and whatever takes one, is left out and listed in the census. Everything
else is refused, and the census counts why.

## The C ABI

- **Handles** are `Arc`s, and a call borrows the value as Rust's receiver or parameter
  does, checked at run time: shared for `&T`, exclusive for `&mut T`, and moved out for
  `T`, after which the handle is only freed. A `Copy` value is copied out instead, and a
  value that is not `Sync` is shared on one thread at a time, and held exclusively by a
  future, which may be polled on any thread. A borrow Rust would
  refuse at compile time aborts, naming the function, as does a null or moved-out handle:
  there is no Rust caller to return an error to. A setter borrows as `&mut T` and takes
  its value as `T`, so it writes the value the handle holds, or the one Rust lent it,
  while a getter's copy is C's own: a nested field is got, set, and set back. A type
  whose C++ copies share one value has `_borrowed`, true while a call or future borrows
  it, so moving it out would abort.
- **In and out**: a `&mut` of a value is two parameters, the value passed in as one taken
  by value is, and a `<name>_out` pointer, through which Rust writes what the call left in
  it on every path, its errors included. C keeps what it passed, and frees what Rust wrote
  as a returned value. C++ takes a reference, Swift an `inout`, and Go a pointer, each
  assigned what Rust wrote; Kotlin, Python, and Dart take a `Mut` holding the value, whose
  `value` Rust's write replaces, before any error raises.
- **Lent handles** borrow from the handle a call lent them from: each use holds that one
  shared and aborts, naming the function, once it was since borrowed exclusively, moved
  out, or freed, which Rust's borrow checker would refuse. Borrowing a lent handle
  exclusively aborts too, as Rust lent it shared.
- **Interned strings**: a `&str` a function needs for `'static`, as `Path::from_static`
  does, is leaked once per distinct content and reused for every later equal one, so a
  caller passing ever-new strings grows the set without bound.
- **Arrays** of a `Vec` are a pointer, a length, and an owner, as a buffer is: passed
  in, Rust copies each element out of C's array; returned, `<prefix>_vec_<T>_free` frees
  the array and its first `len` elements, so a binding that takes the elements sets
  `len` to 0 first. A null pointer is `None`, so C passes a non-null one for `Some` of
  an empty array.
- **Values** are copied in from the caller's memory, which Rust never frees, and lowered
  into memory the caller frees once with the type's `_free`.
- **Futures** run no runtime: C polls with a `<prefix>_waker` and polls again when woken.
  A poll that finishes writes the result through the poll's own out pointers, so a
  cancelled read never loses a value. The future reuses the last waker when given the same
  one again, and owns what it borrowed: the handle it was called on stays borrowed until
  it finishes or is freed.
- **Wakers** move into Rust, which calls `drop` once it lets go of the last clone, so a
  state machine polled with a waker, such as moq's `Driver`s through kio's `Waiter`, runs
  from the caller's event loop. Rust may call `wake` and `drop` from any thread, as a
  runtime's workers do.
- **Callbacks** are a struct of a table and its data. Rust calls `free` once it drops the
  implementation, and may call the others from any thread. A callback takes scalars and
  buffers lent for the call, and returns a scalar.
- **Closures** lent for a call are a function and its data, which Rust calls only during
  the call, from any thread. One takes what a callback takes, and objects by reference,
  each lent as a handle to the value where it is for that call: C borrows it no further
  than Rust lent it, and moving out of it, freeing it, or keeping a borrow past the call
  aborts.
- **The contract** is a hash of the header's declarations, its docs included: the header
  defines it as `<PREFIX>_CONTRACT`, and `<prefix>_contract()` returns it from the library
  built with the header. A binding checks the two match before any other call, so a header
  and a library from different builds refuse to mix.
- **Panics** abort, as an `extern "C"` function's do.
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
  name is registered, and two items claiming one, which only names Rust's conventions
  leave apart can, is a compile error at both spans: `audio_encoder::finish` and
  `audio::encoder_finish` are both `moq_audio_encoder_finish`.

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
  its arguments' names, a reference's by what it borrows. An `exclude` of a type or method
  by the generic type's path leaves it out of every instance, and by the instance's path
  (`"mux::catalog::Producer<moq::mux::catalog::hang::Extra>::stats"`) out of that one.

## C++

`<prefix>.hpp` is header-only C++ over the C header, for C++17, 20, and 23, with
exceptions and RTTI on or off: it throws nothing and reads no type information.

- **Names** are the C names without the prefix, in a namespace of it: `moq_track_Producer`
  is `moq::track_Producer`, and its method `moq_track_Producer_append_group` is
  `moq::track_Producer::append_group`. A keyword takes a trailing `_` (`counter::new_`),
  as does a member named as its class or one of its fields, and a parameter named as a
  member or as anything in the namespace, which it would hide.
- **Objects** are classes owning their handle through a `std::unique_ptr` whose deleter
  calls `_free`, so they move and never copy, and a `Clone` type's `clone()` crosses into
  Rust for a value of its own. A type whose copies may share one value, by the rule below,
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
  through a reference.
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
  `Debug` `debug()`, and `Default` `default_()`.
- **Bytes** are passed as a `view`, which any contiguous bytes convert to, and returned as
  a `buf`, which owns Rust's buffer without a copy, so it outlives the object it came
  from. From C++20, either converts to `std::span<const std::uint8_t>`. A `view` or span
  lives only as long as the bytes it views, and a `view` lent to a callback only for the
  call; reading one past that is C++'s own undefined behavior, which Clang's `-Wdangling`
  refuses for a view or `str()` of a temporary `buf`. A `Vec<T>` is a `std::vector`, of
  `view`s for bytes passed in, and a socket address `socket_addr`, the C struct. `Option<T>` is `std::optional<T>`, a
  tuple `std::tuple`, and `Result<T, E>` a `result<T, E>`, whose `value()` aborts on an
  error.
- **Traits** are abstract classes, passed as a `std::unique_ptr` that Rust deletes.
- **Closures** lent for a call are any callable, taken by a template parameter, which
  Rust calls only during the call, from any thread. An object it lends as `&T` is a
  `const` reference to its class, and as `&mut T` an `api::borrow<T>`, which reaches its
  methods through `->` but neither moves nor copies. Moving out of either, or keeping a
  copy past the call, aborts.
- **Futures** are move-only `future<...>`s, freed when destroyed, which cancels them and
  loses nothing. `poll(waker)` polls once; `std::move(f).get()` blocks the thread;
  `std::move(f).then(exec, callback)` polls on an executor and calls back there, returning
  a `task` that cancels when destroyed, after which the callback has finished or never
  runs and the executor is never called again; and from C++20, `co_await std::move(f).via(exec)` resumes the coroutine on the
  executor. An executor is any callable taking a `void()` job to run once, later, on any
  thread: never inline, since a wake may come from inside Rust. Each wake that lands while
  the future polls polls it again, and a future has one poller at a time, since awaiting
  it moves it.
- A **waker** is a shared callable, which Rust may call from any thread.
- **The contract** is checked when the program loads, before any other call: a header
  and a library from different builds abort.

## Kotlin

`<prefix>.kt` is Kotlin for the JVM and Android over the crate's JNI functions, which
`--features kotlin` builds into the same library, loaded by `System.loadLibrary`. It needs
kotlinx-coroutines. Each value crosses JNI as a scalar, a handle as a `Long`, bytes passed
in as a `ByteArray`, bytes returned as their owner, and anything else encoded into a
`ByteArray` as its C struct holds it.

- **Names** are in a package of the prefix: types in PascalCase without it, members in
  camelCase without their type's name (`moq.TrackProducer.appendGroup`). Pascal case
  loses where segments split, so two types whose names would match each keep a `_` at
  every split after the leading segments they share: moq's `tokio::ConnectError` and
  `tokio::connect::Error` are `TokioConnectError` and `TokioConnect_Error`. Two whose
  segments differ only by case still match, and fail the export. A keyword is
  quoted in backticks, and a member named as one Kotlin gives its type takes a trailing
  `_`. A constructor such as `new` stays a companion function: `Counter.new(1)`.
- **Objects** are classes owning their handle, `AutoCloseable`, which `close()` frees once
  no call holds it; a call after it throws. One never closed is closed once the GC
  collects it, on a daemon thread a phantom reference queue feeds. A `self` method, or passing the object by
  value, closes it once the call returns. `Clone` is `clone()`, and `PartialEq` is `equals`.
  A field is a `val`, `firstGet`, and its setter a function, `firstSet(value)`, named as
  the C ones, since setting an object moves it in and closes its handle, which an
  assignment would hide.
- **Records** are data classes, compared by their fields, and **enums** an `enum class`,
  or for one with data a sealed class of a data class per variant. `Display` is
  `toString()`, `Debug` `debug()`, and `Default` a companion's `default()`.
- **Bytes** passed in are a `ByteArray`, copied, as are those Rust lends a closure or trait
  method. Bytes Rust returns, or a record holds, are a `Bytes`, read only through `size`,
  `get`, `slice`, `copyInto`, and `toByteArray()`, so no view of Rust's memory escapes and
  no read meets it freed. Up to 1 KiB are copied, which costs less than the GC's cleanup;
  longer ones stay in Rust's buffer, each read a JNI call that passes the buffer's owner,
  which JNI keeps reachable until the call returns, and are freed once the GC finds no
  `Bytes` reaching them, slices included. The GC sees none of Rust's buffers, so at most
  64 MiB are lent at once, freed or not; past that, longer ones are copied too, and that
  garbage in Kotlin's heap runs the GC, which frees them. `Bytes(ByteArray)` holds
  Kotlin's for a record passed in. A `Bytes` equals and hashes by its contents, wherever each keeps them, so a
  record holding bytes does too; Rust's lent ones are copied across in one call to compare.
  Strings are `String`, `usize` `ULong` and `isize` `Long`, which abort past a 32-bit
  target's range, `Option<T>` `T?`, a tuple `Pair`,
  `Triple`, or else a generated `Tuple<n>`, though a tuple of one returned whole is its element, `Duration` `kotlin.time.Duration`, and an `Err` a thrown `<E>Exception`
  holding it, whose message is its `Display`.
- A **path** is a `String`: one Rust returns that is not UTF-8 throws. A `SocketAddr`,
  `SocketAddrV4`, or `SocketAddrV6` is a `java.net.InetSocketAddress`, which must be
  resolved and, for `V4` or `V6`, of that family; one Rust returns with an IPv6 flow
  label, which Java does not keep, throws. A `Vec<T>` or `&[T]` is a `List`, and passing
  a `Vec` by value closes each object in it once the call returns.
- **Traits** are interfaces; once Rust drops an implementation, it is closed if
  `AutoCloseable`.
- **Closures** lent for a call are function types, which Rust calls only during the call,
  from any thread. An object it lends is closed once the call returns.
- **Futures** are `suspend fun`s, which resume on the coroutine's dispatcher, as C++ resumes
  on its executor, and never inside a wake: an `Unconfined` one resumes on `Default`. Cancelling the coroutine frees the future and
  loses nothing. Each call is its own future, so it has one poller. The call polls it once
  before suspending, and makes the waker it parks on only if that poll pends.
- A **waker** is a `Waker`, which Rust may call from any thread.
- **The contract** is checked when the library loads, before any other call: a Kotlin
  file and a library from different builds throw.
- **Shrinking**: `<prefix>.pro` keeps what the JNI glue finds by name, `Native`, its native
  methods, and the Kotlin functions Rust calls, while R8 strips and renames the rest. An
  Android library ships it as a consumer rule (`consumerProguardFiles`), which its AAR
  carries to each app that shrinks it.

## Python

`<prefix>.py` is Python over a CPython extension that `--features python` builds into the
same library on the stable ABI, so one build loads in CPython 3.11 and later. The module
loads the library from beside itself: `lib<prefix>.so`, `lib<prefix>.dylib`, or
`<prefix>.dll`. Its glue converts each Python value to the C value and calls the C ABI
function in-process, so a call is one crossing, which releases the GIL while Rust runs.
Each function, method, and property but an `async def` is the extension's own, bound in
the module or set on its class as it loads, so a call enters it with no Python frame
between; the Python declares them under `typing.TYPE_CHECKING`, for type checkers. They
take arguments by position or keyword, raise as a `def` would, and carry their
docstrings and signatures. The module passes mypy and pyright, both strict, so a typed
caller imports it with no ignores.

- **Names** are in the module: types in PascalCase without the prefix, members in
  snake_case without their type's name (`moq.TrackProducer.append_group`), and two types
  whose names would match kept apart as Kotlin's are (`TokioConnect_Error`). A keyword, a
  member Python gives every such class, or a method named as a field takes a trailing `_`.
  Constructors such as `new` stay static methods: `Counter.new(1)`.
- **Objects** are classes owning their handle, freed once their last reference goes,
  which a call holds until it returns, so there is no `close()`. A `self` method, or
  passing the object by value, moves its value out and closes it once the call returns,
  after which a call raises `ValueError`. `Clone` is `clone()`, and `PartialEq` is `==`, unhashable. A field is a
  property, `first_get`, and its setter a method, `first_set(value)`, named as the C ones,
  since setting an object moves it in and closes it, which an assignment would hide.
- **Records** are slotted dataclasses, compared by their fields, and **enums** an `Enum`
  of upper-case members, or for one with data a class of a dataclass per variant,
  `Event.Moved(p)`. `Display` is `str()`, `Debug` `repr()`, and `Default` a static
  `default()`. Constants are module or class attributes, made anew on each read unless a
  literal.
- **Bytes** passed in are any buffer, `bytes | bytearray | memoryview`: `bytes` is read in
  place, and others, which Python may change while Rust reads, are copied. Bytes Rust
  returns, or a record holds, are a read-only `memoryview` lending Rust's buffer until it
  and its views are released. Those Rust lends a closure or trait method are copied into
  `bytes`. Strings are `str`, `Option<T>` `T | None`, a tuple a `tuple`, `Duration`
  `datetime.timedelta`, truncated to the microsecond, with `timedelta.max` for any longer
  and for `Duration::MAX`, a `Range` a `range`, and an `Err` a raised `<E>Exception`
  holding it in `error`, whose `str` is its `Display`.
- A **path** is a `str`, or any path passed in, which crosses as `os.fsencode` encodes it, so
  one that is not UTF-8 keeps its bytes. A `SocketAddr` is the `socket` module's tuple,
  `(host, port)` or `(host, port, flowinfo, scope_id)`, of that family for `V4` or `V6`.
  A `Vec<T>` or `&[T]` Rust returns is a `list`, and passed in is any iterable other than a `str`,
  annotated `Iterable`, or `Sequence` in a record, which keeps it. Passing a `Vec` by value closes
  each object in it.
  An integer out of its Rust type's range raises `OverflowError`.
- **Traits** are abstract classes an implementation subclasses; once Rust drops one, its
  `close()` is called if it has one.
- **Closures** lent for a call are any callable, which Rust calls only during the call,
  from any thread. An object it lends is closed once the call returns. An exception a
  closure or trait method raises prints its traceback and aborts, since Rust has nowhere
  to unwind it to.
- **Futures** are `async def`s, which run nothing until awaited, or for a function that is
  not `async`, an awaitable the call returns. They resume on the running event loop and
  never inside a wake: a wake from any thread queues the future's token and writes a byte
  to a socket the loop reads, without the GIL, or during a call on the loop's own thread,
  writes nothing and schedules the loop's dispatch once the call returns. Cancelling the task frees the future and
  loses nothing. The call polls it once before suspending, and a future is awaited once,
  as a coroutine is.
- A **waker** is a `Waker(callback)`, which Rust may wake from any thread, calling
  `callback` on its event loop.
- **The contract** is checked when the module imports, before any other call: a Python
  file and a library from different builds raise `ImportError`.

## Swift

`swift/<prefix>.swift` is Swift over the C ABI, for Apple's platforms and Linux. It imports
the header as the clang module `<Prefix>FFI`, which `swift/module.modulemap` declares
beside `swift/<prefix>_atomics.h`, the atomics its runtime needs that Swift's standard
library has only from macOS 15: a build puts `swift/` on its import path and links the
static library. It needs Swift 6 and, for `Swift.Duration`, macOS 13 or iOS 16, and builds
in either language mode.

- **Names** are as Kotlin's, at the module's top level: types in PascalCase without the
  prefix, members in camelCase without their type's name (`TrackProducer.appendGroup`). A
  keyword is quoted in backticks, and a member named as one Swift gives its type, such as
  `init`, takes a trailing `_`. Parameters have no labels, so a call reads as Rust's, and a
  constructor such as `new` stays a static function: `Counter.new(1)`.
- **Objects** are final classes owning their handle, freed in `deinit`, which a call
  holds off until it returns, so there is no `close()`. A `self` method, or passing the
  object by value, moves its value out and closes it once the call returns, after which a
  call traps. `Clone` is `clone()`, and `PartialEq`
  is `==`. A field is a computed property, `firstGet`, and its setter a function,
  `firstSet(value)`, named as the C ones, since setting an object moves it in and closes
  its handle, which an assignment would hide. An object is `Sendable`, since Rust checks each borrow at run time.
- **Records** are structs and **enums** Swift enums, with a variant's data as associated
  values, all `Sendable`. `PartialEq` is `==`, through Rust's; `Display` is `description`,
  `Debug` `debugDescription`, and `Default` a static `default()`.
- **Bytes** are `Data`. Passed in at a call's top level, Rust reads Swift's own; nested in
  another value, they are copied in. Bytes Rust returns lend its buffer until Swift
  releases the `Data`, though `Data` copies a few bytes inline, freeing Rust's at once.
  Strings are `String`, copied, `char` `Unicode.Scalar`, `usize` `UInt`, `Option<T>` `T?`,
  a tuple a Swift tuple, though a tuple of one is its element, and `Duration`
  `Swift.Duration`. An `Err` is thrown: an exported type as itself, which conforms to
  `Swift.Error`, and any other in a generated `<T>Error` holding it.
- A **path** is a `String`: one Rust returns that is not UTF-8 traps. A `SocketAddr`,
  `SocketAddrV4`, or `SocketAddrV6` is the runtime's `SocketAddr`, its IP's 4 or 16 bytes
  and its port, flow label, and scope, whose family Rust checks. A `Vec<T>` or `&[T]` is
  an array, and passing a `Vec` by value closes each object in it once the call returns.
- **Traits** are `Sendable` protocols, which Rust releases once it drops an implementation.
- **Closures** lent for a call are `@Sendable` function types, which Rust calls only during
  the call, from any thread. An object it lends is closed once the call returns.
- **Futures** are `async throws` functions, which resume on the task's executor, never
  inside a wake. Cancelling the task frees the future, losing nothing, and throws
  `CancellationError`. Each call is its own future, so it has one poller. The call polls it
  once before suspending, and makes the signal it parks on only if that poll pends; the
  signal latches a wake that lands before it parks.
- A **waker** is a `@Sendable () -> Void`, which Rust may call from any thread.
- **The contract** is checked once, at the first call into the library: a Swift file and a
  library from different builds trap.
## Go

`go/` holds a Go package per crate whose items the facade exports, over the C header
through cgo, for Go 1.24 or later, and the runtime they share, `go/rt`. Each includes
`<prefix>.h` from the `include/` beside `go/`, as the export lays them out, and links the
library through `CGO_LDFLAGS`. A crate's package is its name without the prefix and its
`_`: moq-mux's is `mux`, and serde_json's `serdejson`, but keeps the prefix where its
strip leaves no leading letter (`moq_2d` is `moq2d`). One that a Go keyword, `C`, `main`,
the runtime, or an earlier crate's package takes gains a trailing `_`, as a crate `rt` is
`rt_`. Cargo keeps crates from depending on each other in a cycle, so their packages
never import each other in one, as Go requires,
while Rust's modules may, so a package per module could not hold moq. An instance of a
generic type, or a function naming one, belongs to the last of its own and its arguments'
crates in Cargo's order, which can name them all: moq's
`mux::catalog::Producer<hang::catalog::Extra>` is in `mux`. The packages' import path is
the module of the `go.mod` above `--out`, as `go mod edit -json` reads it, with the path to
`go/`; or else `<prefix>`, the module of the `go.mod` the export writes in `go/`, which a
caller requires and replaces with that directory. The export needs `go` on PATH whenever it
reads a go.mod.

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
  A method named `Close` or `Error`, and a parameter named as a Go keyword, a predeclared
  name, or a package the code uses, take a trailing `_` too.
- **Objects** are pointers to structs owning their handle. `Close()` frees it once no call
  holds it, and a second `Close()` does nothing; one never closed is closed once the GC
  collects it, through `runtime.AddCleanup`. A call that meets a closed object, as its
  receiver or anywhere in its arguments, returns an error matching `rt.ErrClosed`, even when
  `Close` races it on another goroutine, so every such call returns an `error`; `String()`
  and `GoString()`, which return none, return its text. A `self` method, or passing the object by value, closes
  it once the call returns. `Clone` is `Clone()`, and a `pub` field a getter, `FirstGet()`,
  and a setter, `FirstSet(v)`.
- **Records** are structs, and **plain enums** an `int32` type with a constant per
  variant (`ShapeSquare`). An enum with data is an interface each variant's struct
  implements (`EventMoved{V0: p}`), sealed so only the package's do; its methods are the
  interface's. Positional fields are `V0`, `V1`, and so on. `Display` is `String()`,
  `Debug` `GoString()`, `PartialEq` `Equal(other)`, all through Rust's, and `Default` a
  function of the package, `PointDefault()`. An object's exported `Handle_` field, and a
  trait's `<Trait>Vtable_` function, are how one generated package reaches another's,
  since cgo gives each its own C types: callers use neither.
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
  element, and one returned whole is the function's results. `Duration` is
  `time.Duration`: a negative one passed in, or one Rust returns past its 292 years,
  panics. A socket address is a `netip.AddrPort`, whose zone names an IPv6 scope by its
  number or its interface's name; one Rust returns with an IPv6 flow label, which
  `netip` does not keep, panics. A `Vec<T>` or `&[T]` is a `[]T`.
- **Errors**: a `Result` returns an `error` last. An exported error type is returned as
  itself, implementing `error` with its `Display`, else its `Debug`, and any other in a struct
  `<T>Error{Value}`, such as `rt.StringError`, in the package of the last crate a type
  in it belongs to, or `rt` for none. Ranges and bounds are `rt` types too
  (`rt.RangeU64`).
- **Traits** are interfaces. Once Rust drops an implementation, it is closed if it is an
  `io.Closer`.
- **Closures** lent for a call are funcs, which Rust calls only during the call, from any
  thread. An object it lends is closed once the call returns.
- **Futures** are calls taking a `context.Context` first, which block until the future is
  ready. The call polls it once before it waits, and makes the waker it waits on only if
  that poll pends; a wake from any thread signals the waiting goroutine, which polls
  again, so a wake never polls inside Rust, and one that lands before the wait is kept. A
  context done before the call, or while it waits, returns its error and frees the future,
  which cancels it in Rust and loses nothing: the next call takes what it would have.
  Each call is its own future, so it has one poller, and its context bounds only it.
- A **waker** is a `func()`, which Rust may call from any thread.
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
  a trailing `_` (`Point.default_()`). An item whose Dart name clashes with another's, or
  with a name the runtime declares, fails the export: rename one in the source or exclude it.
- **Objects** are final classes owning their handle. `close()` frees it once no call
  holds it, and a second does nothing; one never closed is freed once the garbage
  collector finds it unreachable, through a finalizable handle of Dart's C API. Any call that meets a
  closed object, as its receiver or anywhere in its arguments, throws a `StateError`
  before Rust sees it, and moves none of the call's other arguments. An isolate cannot
  send an object to another, so no `close()` races a call. A `self` method, or passing the
  object by value, closes it once the call returns. `Clone` is `clone()`, and a `pub`
  field a getter and a setter.
- **Records** are final classes with a named, required parameter per field, and
  positional fields are `$0`, `$1`, and so on. **Plain enums** are Dart enums, and an enum
  with data a sealed class with a final class per variant (`EventMoved`), so a `switch`
  over it is exhaustive. `Display` is `toString()`, `Debug` `debug()`, `PartialEq` `==`, all
  through Rust's, with a `hashCode` that throws an `UnsupportedError`, since no hash Dart
  computes agrees with Rust's `==`, and `Default` a constructor, `default_()`. A plain enum's `==` is Dart's, by its value.
- **Integers** are `int`. A narrower one passed in throws a `RangeError` when out of its
  range, and a `u64` passes its 64 bits, so one past `i64`'s range is negative in Dart.
  A `char` is a `String` of one Unicode scalar.
- **Bytes** a call takes are a `Uint8List`, and text a `String`, each copied into a
  buffer the call frees. Bytes Rust returns are a `Uint8List` owned by Dart's memory
  manager: up to 64 KiB copied, with Rust's freed at the call, and a longer one an
  unmodifiable view of Rust's buffer, freed once neither it nor any view of it is
  reachable, so no view outlives its memory. A lent view costs a finalizer and charges the
  garbage collector its length, which costs more than a copy up to about 64 KiB. Text Rust
  returns is copied into a `String`. A path is a `String`, and one Rust returns that is
  not UTF-8 aborts, since a throw would leave the rest of a list or record being lifted
  unfreed. `Option<T>` is `T?`, a tuple a Dart record, and a `Vec<T>` or `&[T]` a
  `List<T>`. `Duration` is `Duration`, truncated to the microsecond: a negative one passed
  in throws an `ArgumentError`, and Rust's longest is Dart's. A socket address is a
  `SocketAddr` of an `InternetAddress`, a port, and IPv6's flow label and scope; one passed
  in of the wrong family throws an `ArgumentError`, and a port, label, or scope out of its
  range a `RangeError`.
- **Errors**: a `Result`'s error is thrown. An exported error type is thrown as itself,
  implementing `Exception`, and any other in a final class `<T>Exception` holding it as
  `error`, such as `StringException`.
- **Traits** are abstract classes. Rust calls an implementation's methods on the isolate's
  thread, and once it drops one, calls its `close()`, which does nothing unless overridden.
  A call that throws before Rust sees it gives Rust none of its implementations.
- **Closures** lent for a call are Dart functions, which Rust calls only during the call,
  on the isolate's thread. An object it lends lives only for that call of the closure.
- **Futures** are `Cancelable<T>`, a `Future<T>` completed on the isolate that made it:
  an extension type on Dart's own future, which `await` takes its shortest path through.
  The call polls it once, returning it complete if that poll finished, and makes a waker
  only if it pends. A wake, from any
  thread, posts to the isolate's port, and its event loop polls again, so a wake never
  polls inside Rust, and one that lands during a poll is kept. `cancel()` frees the
  future unless it finished, which cancels it in Rust and loses nothing: the next call
  takes what it would have, and it completes with `Cancelled`. A pending future keeps
  the isolate alive, as a timer does.
- A **waker** is a `void Function()`, which runs on the isolate's event loop.
- **Threads**: Dart runs code only on its isolate's thread, so Rust reaches it from any
  other through the isolate's port, and waits for its event loop to run the call. While
  Rust holds an implementation or a lent closure of the isolate, each call into Rust, a
  future's poll and `cancel()` included, runs on a thread of its own, acting as the
  caller's for the handles it borrows, while the isolate's thread runs what Rust calls
  into Dart meanwhile. Any other call runs on the isolate's thread, since nothing it waits
  on can call into Dart.
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
lists each refused item at its span, then the `exclude` list that exports the rest, and
what else it left out, such as what C can never hold and the traits C cannot implement, and why.
It ignores the config's `exclude`. It still fails on an `instance not inferred`
refusal, which is a bug in the exporter's inference rather than in the crate.

## Tests

```bash
just ffi test     # export the fixtures, build them, and drive api from C, C++, Kotlin, Python, Swift, Go, and Dart
just ffi moq      # export moq's crates at the pinned revision, build them, and run moq/moq.c, moq/moq.cpp, moq/moq.kt, moq/moq.py, moq/moq.swift, moq/moq.go, and moq/moq.dart
just ffi moq-cpp  # the same export, running only moq/moq.cpp
just ffi moq-swift  # the same export, running only moq/moq.swift
just ffi census   # census each into target/census
nix develop .#android --command just ffi android  # the Kotlin test, shrunk by R8, on the Android device adb reaches
nix develop .#bench --command just ffi bench  # frame writes and reads, through the export from C, C++, Kotlin, Python, Swift, Go, and Dart, and moq's rs/moq-c
```

The `api` fixture covers each rule once, and re-exports the `dep` fixture's types, which
export into the same library. `tests/api.c` runs the same script as the
fixture's `examples/transcript.rs`, and the two transcripts must match line for line;
then it breaks each runtime check, which must abort with its message. `tests/api.cpp`
runs the script through the C++ header at C++17, 20, and 23, with exceptions and RTTI off
at 17 and 23, builds it under Clang's `-Wconsumed -Wdangling -Werror`, each case of
`tests/api_strict.cpp` failing there, then checks its futures: a wake that lands during a poll, a cancel between a
wake and its poll that loses nothing, and a coroutine resumed or destroyed. `tests/api.kt`
runs the script through the Kotlin on the JVM, checks its futures the same way, that a call
after `close()` throws, that `Bytes` and their slices keep Rust's buffer, past the object or
record they came from, until the GC frees it, and that calls racing `close()` or the GC
read live values, and breaks each runtime check, which must throw or abort with its message. `just ffi android` runs
the same on ART: the Kotlin and `tests/api.kt`, shrunk by R8 into an app's dex with only the
generated rules keeping what JNI looks up, against the library built by the NDK for the
device's ABI, which ANDROID_HOME and ANDROID_NDK_HOME locate. Nightly runs it on an
x86_64 emulator at API 24, the oldest moq's Android library supports, and at API 35.
`tests/api.py` runs the script through the Python, in CPython's development mode with
warnings as errors, checks its futures the same way, what frees an object or bytes nobody
closed, and the values Python's types bound, and breaks each runtime check, which must raise or
abort with its message; the Python with another contract must refuse the library.
`api_py_types` checks the Python with mypy and pyright, both strict, at CPython 3.11, as
`just ffi moq` checks moq's.
`tests/api.swift` runs the script through the Swift, built as a module in Swift 6's
language mode and type-checked in Swift 5's, checks its futures the same way and that
releasing frees, and breaks each runtime check, which must trap or abort with its message.
It runs with nixpkgs' Swift on Linux, which builds none for aarch64-linux
([quest](/quest/m2/swift-arm.md)), and nightly with Apple's on macOS.
`tests/api.go` runs the script through the Go after `go vet`, checks its futures the same
way, plus a context done before the call, a deadline, and a handle closed during a call,
checks that bytes cross both ways, that a `Bytes` keeps Rust's buffer while only a sub-view
or a reader reaches it and that the GC frees it once none does, that the GC frees an object
never closed, and that each kind of call meeting a closed object, including one racing
`Close`, returns `rt.ErrClosed`, and breaks
each runtime check, which must panic with its message; a runtime package from another
build must fail its contract check. The `reject` fixture's expected errors, each with its notes, are `fixtures/reject/expected.txt`.
moq's crates export through `target/moq-ffi`, an empty library on each that `just ffi moq`
writes in place of moq-ffi's remainder. `moq/moq.c` publishes and subscribes to a moq-net
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
