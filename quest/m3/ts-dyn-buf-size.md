# [S] A Buf object's size

## Goal

[rust2-ts](/ts/README.md)'s `dyn Buf` objects hold only the methods some call dispatches
to, so each buffer coerced to one adds a few functions to the output and not all of `Buf`'s.

## Plan

Found writing `dyn Buf`: an object holds every vtable entry, as the crate's own traits'
do, so the stock fixture's six buffers add about 5,000 lines of `bytes`' provided methods
translated from MIR. A crate's calls through `dyn Buf` name the methods it needs, but an
object passed to another crate may be called by methods this one never calls, so the
set has to be the dependents' too, or each method made on first call.

Public API: the translator's output.
