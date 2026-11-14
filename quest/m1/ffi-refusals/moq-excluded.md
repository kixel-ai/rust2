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
  trait-object decision above. And four constructs no decision covers yet, listed under
  Waiting: `CertificateDer::from_slice` and `Der::from_slice` (a `'static` borrow,
  `&'a [u8]` of a handle's `'static` lifetime), the two `rustls::ConfigBuilder<ServerConfig,
  …>` instances (`ServerConfig` by value, which an `Arc` elsewhere holds),
  `qmux::Session::connect` and `accept` (a type parameter bounded by qmux's `Transport`),
  and `moq_sock::demux::Discard::ALL` (`[Discard; 4]`, an array of a non-`u8` element).

Public API: the generated API of each item that newly exports.

## Waiting

- Maintainer: a `'static` borrow parameter (`CertificateDer::from_slice`,
  `Der::from_slice`). Recommended: left out and listed as what C can never hold, since C
  lends only for the call; rejected: copy into a leaked buffer, which never frees.
- Maintainer: a type both by value and in an `Arc` (`ConfigBuilder<ServerConfig, …>`).
  Recommended: left out and listed, as a type argument no handle can name; rejected: a
  second, by-value handle, which `Arc::try_unwrap` would need at run time.
- Maintainer: a type parameter bounded by another crate's trait (`qmux::Session::connect`,
  `accept`, over `Transport`). Recommended: left out and listed, since no export returns a
  `Transport`; rejected: a callback trait for it, which C would implement over raw IO.
- Maintainer: an array of a non-`u8` element (`Discard::ALL`). Recommended: crosses as the
  `Vec` of its elements, as a returned slice does; rejected: left out.
