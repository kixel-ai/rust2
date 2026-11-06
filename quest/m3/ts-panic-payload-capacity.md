# [M] std's panic messages hold std's capacity

## Goal

A panic hook in code [rust2-ts](/ts/README.md) generates reads the same `String` capacity
from std's own formatted panic messages (such as `slice index starts at {} but ends at
{}`) as native Rust, proven by the panics fixture's hook against native Rust.

## Plan

Found in #574, which gives `String` its capacity: the runtime passes std's formatted messages as one string, so their
capacity differs from std's, which builds them piece by piece (37 against 44 for that
message). About 64 runtime sites pass such a message. Build each as std's `format!` does,
without slowing the paths that never panic.

Public API: the runtime's panic sites, not their messages.
