# [M] What the excludes hide at moq's main

## Goal

Each construct `just ffi census <checkout>` still refuses at moq's `main` behind an
exclude in `ffi/moq/ffi.toml` exports by a rule, or is left out as what C can never hold,
beyond `NonZero`, which exports, and the types
[not-send](/quest/m1/ffi-export/not-send.md) exports. Measured on 2026-11-12 at moq
`04f63af`, with the rules for another crate's value, a non-`async` function's borrowing
future, and a lent object field in place: 11 items.

## Plan

[The epic](/quest/m1/ffi-refusals/README.md)'s decisions apply: rust2 asks moq for no
source change, and what C can never hold is left out and listed. Measure again first.

- `std::io::Error` taken by value (`accept::Failure::classify`, `accept::Health::failed`)
  and `std::net::IpAddr` (`quic::demux::Datagram::local`). Recommended: `IpAddr` crosses
  as `SocketAddr` already does, by family; `io::Error` as an opaque handle, as another
  crate's value now is, with the standard library no longer exempt.
- Trait objects: `quic::Controller`'s `Arc<dyn moq_congestion::Factory + Send + Sync>`
  field and `tls::Serve::resolver`'s `Arc<dyn rustls::server::ResolvesServerCert>`.
  Recommended: an opaque handle to the `Arc`, as another crate's `Arc` already is.
- Generic parameters: `server::Request::within` (`impl Future`),
  `websocket::Listener::with_protocols` (`impl IntoIterator<Item: AsRef<str>>`),
  `tcp::TlsAcceptor::offered` (`impl Iterator`), and `tcp::TlsAcceptor::accept` (an IO
  stream). Recommended: `AsRef<str>` items as strings and an `Iterator` parameter as the
  array of its items, as returns already are; a foreign future as #705's is; the IO
  stream left out as what C can never hold, since no export returns one.
- `server::Config::websocket`, a borrowed `Option` of a listener that is not `Clone`.
  Recommended: lent as `Some` of a handle, as a bare field now is.
- `server::Listener::accept_with`'s future is not `Send`. Left out today as what C can
  never hold, since it takes a `qmux::Session` no export returns once the excludes go;
  confirm the census lists it so.

Public API: the generated API of each item that newly exports.
