# [S] Small shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a small shape: 26 items at moq
`ffcb7cd9` use `u128` (`Timestamp::as_nanos`), an array, a const generic, unit outside a
return, `Result` outside a return, `!`, or an enum without variants. The count includes
the objects and futures that are not `Send`, which
[not-send](/quest/m1/ffi-export/not-send.md) owns since 2026-11-09.

## Plan

Each is its own small mapping; what has no C shape keeps its refusal and goes to the
maintainer as moq's signature to change.
