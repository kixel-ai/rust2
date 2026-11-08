# [L] Generic parameters and traits export

## Goal

The census at the moq pin refuses no item of moq's own for a generic parameter, type
argument, trait object, supertrait, associated type, unmet bound, callback or closure
signature, or blanket impl: 51 items at moq `ffcb7cd9`.

## Plan

Ranked first by moq-ffi's hand-written wrapper: `Client::connect` and `Server::accept`
take `transport::poll::Boxable`, which moq-ffi's `session.rs` and `server.rs` wrap by hand.
Then `impl Future` and `IntoFuture` parameters (`clock::timeout`, `Waiter::poll_future`),
`io::Read` parameters, trait objects, associated types (`mux::catalog::Stream`,
`transport::poll::Session`), supertraits, `time::Driver`'s callback signature, and
`clock::drive`'s kept closure.

A generic exports at the instances the facade names, as generic types already do; a trait a
language implements crosses as the vtable the exporter already gives callback traits.
Where a construct has no C shape, the census keeps refusing it with its span, and moq's
signature is what changes: report it to the maintainer rather than add config.
