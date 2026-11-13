# [M] Owned buffers are taken, not copied

## Goal

A byte payload a [rust2-ffi](/ffi/README.md) binding passes to Rust is copied at most
once, and a payload Rust lent can be passed back with no copy: a buffer that arrives with
an owner is Rust's own and is taken, and `<prefix>_buf_clone` gives a binding a second
owner of a lent one. Kotlin's and Dart's writes on
[the binding benchmark](/bench/ffi/README.md) lose an allocation and a copy each, and a
forwarder's read-then-write moves no payload bytes in any language.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a child of the exporter
epic, since it changes what a buffer's `owner` means on the way in.

Today, in `ffi/src/glue.rs`'s `lift`, a `Bytes` argument is `Bytes::copy_from_slice` of the
buffer, and a `Vec<u8>` or `String` is a `to_vec` or `to_owned`, whatever the buffer's
owner. Kotlin's glue has already copied the `ByteArray` into a `Vec` by then
(`ffi/runtime/jni.rs`'s `bytes`), and Dart's copies into a buffer the call frees, so each
payload is copied twice. The header calls a buffer passed in "a view Rust reads during
the call and never frees", so its owner is never read.

- A buffer passed in with a non-null owner is taken: its bytes become the `Bytes`, `Vec`,
  or `String` with no copy, and the caller no longer owns it. A null owner still lends,
  and Rust copies.
- The JNI and Dart glue build the buffer once in Rust and hand it over.
- `<prefix>_buf_clone` bumps the owner's count, so a binding keeps a lent payload while
  passing it on.
- A C caller that leaves garbage in `owner` breaks; the contract hash changes, so a stale
  header fails at load.

Prove it in every language's `api` test with a payload read and written back, checked for
identity of the storage where the language can see it, and re-measure the bench's write
and throughput rows.

Public API: the C ABI's meaning of an input buffer's owner (breaking), `<prefix>_buf_clone`,
and each binding's way to pass a lent payload on.
