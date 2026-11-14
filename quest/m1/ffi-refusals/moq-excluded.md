# [M] What the excludes hide at moq's main

## Goal

Each construct `just ffi census <checkout>` still refuses at moq's `main` behind an
exclude in `ffi/moq/ffi.toml` exports by a rule, or is left out as what C can never hold,
beyond `NonZero`, which exports, and the types
[not-send](/quest/m1/ffi-export/not-send.md) exports. Measured on 2026-11-13 at moq
`98e54e7`, with #721's reach of other crates' types and the rules for another crate's
value, a non-`async` function's borrowing future, and a lent object field in place: 23
items beyond not-send's, 13 of them in crates #721 newly reaches.

## Plan

[The epic](/quest/m1/ffi-refusals/README.md)'s decisions apply: rust2 asks moq for no
source change, and what C can never hold is left out and listed. Measure again first.

The maintainer kept this quest on 2026-11-13 and accepted each recommendation below as its plan.

- `std::io::Error` taken by value (`accept::Failure::classify`, `accept::Health::failed`)
  and `std::net::IpAddr` (`quic::demux::Datagram::local`). Decided: `IpAddr` crosses
  as `SocketAddr` already does, by family; `io::Error` as an opaque handle, as another
  crate's value now is, with the standard library no longer exempt.
- Trait objects: `quic::Controller`'s `Arc<dyn moq_congestion::Factory + Send + Sync>`
  field and `tls::Serve::resolver`'s `Arc<dyn rustls::server::ResolvesServerCert>`.
  Decided: an opaque handle to the `Arc`, as another crate's `Arc` already is.
- Generic parameters: `server::Request::within` (`impl Future`),
  `websocket::Listener::with_protocols` (`impl IntoIterator<Item: AsRef<str>>`),
  `tcp::TlsAcceptor::offered` (`impl Iterator`), and `tcp::TlsAcceptor::accept` (an IO
  stream). Decided: `AsRef<str>` items as strings and an `Iterator` parameter as the
  array of its items, as returns already are; a foreign future as #705's is; the IO
  stream left out as what C can never hold, since no export returns one.
- `server::Config::websocket`, a borrowed `Option` of a listener that is not `Clone`.
  Decided: lent as `Some` of a handle, as a bare field now is.
- `server::Listener::accept_with`'s future is not `Send`. Left out today as what C can
  never hold, since it takes a `qmux::Session` no export returns once the excludes go;
  confirm the census lists it so.

- Reached since #721 exports other crates' types with their API: rustls's
  `ClientConfig` fields of `Arc<dyn …>` (5) and `client::Resumption::store`, under the
  trait-object decision above. Four more constructs, decided by the maintainer on 2026-11-13:
  - A `'static` borrow parameter (`CertificateDer::from_slice`, `Der::from_slice`):
    ✅ exported by interning by content (rejected: left out and listed). The binding keeps
    one process-wide table keyed by the bytes' contents; the first call with given bytes
    copies them into a leaked Rust allocation, and later calls with equal bytes reuse it.
    Never key by C's pointer, which C may free or reuse. The generated docs on such a
    parameter say memory grows with each distinct value and is never freed. This is sound
    for a shared `&'static [u8]`: the copy is immutable and never freed, and equal bytes
    differ only by address. Refuse with a span any other `'static` borrow, such as a
    `&'static mut` (interning would alias it) or a non-byte element type.
  - `rustls::ConfigBuilder<ServerConfig, …>`, both by value and behind an `Arc`:
    ✅ left out and listed (rejected: a handle everywhere).
  - `qmux::Session::connect` and `accept`, bounded by qmux's `Transport`: ✅ left out and
    listed (rejected: a callback interface).
  - `moq_sock::demux::Discard::ALL` (`[Discard; 4]`): ✅ crosses as the `Vec` of its
    elements, as a returned slice does (rejected: left out).

Public API: the generated API of each item that newly exports.
