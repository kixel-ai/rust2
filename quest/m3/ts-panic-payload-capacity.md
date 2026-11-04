# [S] std's panic messages with std's capacity

## Goal

A panic hook that downcasts the `String` payload of a panic std's own code formats,
such as `slice index starts at {} but ends at {}`, reads std's capacity from it.
std's `panic!` writes the message into an empty `String` piece by piece, while the
runtime's sites in `ts/runtime/rs/` pass one concatenated string with `owned` true,
which it writes in one push: exact only for a message of one piece, as
`panic_display`'s.

## Plan

Pass each such site's message as the `Arguments` std's format string makes, through
`panicFmt`, so the hook writes it as std does, and extend the `lang/panics` fixture's
`sized` hook to every case.
