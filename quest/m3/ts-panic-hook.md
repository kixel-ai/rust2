# [L] A panic hook

## Goal

[rust2-ts](/ts/README.md) translates `panic::set_hook`, `take_hook`, and the
`PanicHookInfo` a hook is given: its payload, `payload_as_str`, `location`, and `Display`,
with every panic the translation makes running the hook as native Rust's does under
`panic=abort`, proven by a differential test.

## Plan

Split from the PR that translated the rest of `std::panic`, which refuses both functions
with a span. A hook reads each panic's `Location`, so every panic needs one, the runtime's
own too, such as an index out of bounds or an overflow, at the location std's
`#[track_caller]` reports. The differential harness sets its own hook natively, so a case
that sets one must leave the harness able to read the message.

The crate's own `#[track_caller]` functions already take their caller's `rt.Location`
(`ts/src/lower/caller.rs`); std's that map to the runtime, such as `unwrap`, `expect`,
indexing, and `RefCell::borrow_mut`, take none, as nothing reads it yet. A prototype
passing a hoisted `rt.Location` constant as a last argument to each of the codec's checked
operations, indexes, unwraps, and ranges measured within `just ts bench`'s noise in every
engine, but grew the codec's bundle by 7.7% raw and 5.5% gzipped, and moq sets no hook, so
nothing reads them there. Which calls carry one is open in the quest's PR: always, or only
where a `set_hook` is reachable from the roots. A panic inside std's own code, not under
`#[track_caller]`, is at a path in the native build's sysroot, which translation must name
as that build does.

Public API: the translator's output, and the runtime's `Panic`.
