# [M] A close racing a refused one-crossing drop

## Goal

In Go and Kotlin, a `close()` that meets another thread's one-crossing drop of a checked
object reports that drop's outcome: it returns once the handle is closed, and fails as
borrowed once Rust refused the drop and the handle reopened. A `Drop` whose callback closes
the same object, on its own thread or on one it then joins, still returns at once.

## Plan

Found in review of [#906](https://github.com/kixel-ai/rust2/pull/906). Since
[#838](https://github.com/kixel-ai/rust2/pull/838) (Go) and #906 (Kotlin), a checked close no
call holds claims the handle `CLOSED | CLOSING`, then drops and frees it in one crossing,
restoring it open if Rust refuses the drop under a borrow of its own, such as a lent
`Draft`. A second close meanwhile returns at once, though the first may yet be refused.
Waiting it out instead, as #906 first did, hangs when the `Drop`'s callback hands the
close to another thread and joins it, since the deciding close cannot finish meanwhile.

The second close can wait only until Rust commits the drop: Rust refuses before it takes
the value, and runs any callback only after. Recommended: the glue tells the binding when
it commits, before the destructor runs, such as by clearing `CLOSING` in the handle's state
through a JNI field write in Kotlin and a store through a pointer in Go; waiting closers
then return once it clears. That splits `rt::Object::close` into its check and commit and
the destructor. Measure the object-return row before and after, since the one crossing
was the point. Model a refused drop against a second close, and a drop handing its close
to a joined thread, in `ffi/tests/hold.rs`.
