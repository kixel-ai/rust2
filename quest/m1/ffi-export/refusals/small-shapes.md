# [S] Small shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a small shape: 26 items at moq
`ffcb7cd9` use `u128` (`Timestamp::as_nanos`), an array, a const generic, unit outside a
return, `Result` outside a return, `!`, an enum without variants, or an object or future
that is not `Send`.

## Plan

Each is its own small mapping; what has no C shape, such as an object that is not `Send`
under the C ABI's any-thread calls, keeps its refusal and goes to the maintainer as moq's
signature to change.
