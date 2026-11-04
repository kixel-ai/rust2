# [S] Writes through a crate's DerefMut to a value JS copies

## Goal

[rust2-ts](/ts/README.md) translates a write through a crate's own `DerefMut` whose target
is a value JS copies, such as `*lock.lock() = Some(x)` through a lock guard to an
`Option`, proven by differential tests against native Rust. Today each is refused as
"`DerefMut::deref_mut` of a value JS copies, which is no path".

## Plan

moq-net's model writes this way six times at the pin, through kio's lock guards
(`model/broadcast.rs`, `model/track.rs`). The guard's `deref_mut` returns a `&mut` to a
field of what it guards, so the place is a path once the call is seen through, or an
`rs.Place` the impl returns; pick whichever keeps std's own guards' fast path.
