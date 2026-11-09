# [M] Poll and Context export

## Goal

A `poll_*` method of moq's with no async twin exports and takes a `<prefix>_waker`, as the
C ABI's future polls already do: 23 items at moq `ffcb7cd9` name `Poll` or `Context`, such
as `goaway::Consumer::poll`, plus 3 with a future borrowed from a function that is not
`async` or a `Waker` not taken by value (`Park::hold`, `Waiter::waker`).

## Plan

A `poll_*` with an async twin stays left out for the twin, as the census already counts.
Each language wraps the exported poll the way its generator wraps a future's, so no
generator gains a second waker path.
