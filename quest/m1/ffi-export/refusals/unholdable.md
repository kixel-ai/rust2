# [S] What C can never hold is left out and listed

## Goal

An object no export returns or lends to a closure, and a generic type no export
instantiates, leave the bindings without failing the export, and the census keeps listing
each as unreachable: 71 objects at moq `ffcb7cd9`, mostly errors that appear only in
`From` impls (`DecodeError`, `BoundsExceeded`, `clock::OutOfRange`), and 26 generic types,
such as moq-net's `transport::poll` futures and moq-mux's `Export<T>`s.

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): ✅ left out and kept listed
in the census, since C could never get one. Rejected: refusing them, which would make moq
add constructors or returns it has no use for.

An object lent to a closure, as the api fixture's `walk` lends `Cursor`, reaches C as a
handle for the call and stays in the bindings. The census already counts it held, so the
71 exclude such objects; a callback trait takes no objects.

Some leave the list once the functions that return them export (`Driver`,
`announce::Announce`); the census section shows which remain.
