# [M] A refused call frees the buffers it was given

## Goal

A call a failed run-time check refuses frees every buffer the caller handed to Rust, as one that ran does, so a refusal leaks nothing in any binding.

## Plan

- kixel-ai/rust2#744 made Rust take a buffer passed by value. kixel-ai/rust2#730 lifts a call's arguments in three passes: those holding no handle, then those holding a checked handle, then the reserved moves. A check that fails returns at once. The arguments not lifted yet, and the fields of the failing argument after the failing handle, keep buffers nobody frees. Only an argument holding both a checked handle and a buffer reaches this, such as a record of a moved object and a `String`. Swift hands every such buffer to Rust as a copy it allocated.
- Swift has the same gap before C runs. `Native.take` shares a buffer's owner, or copies the buffer into Rust's memory, for an argument, and an `Arena` does so for bytes nested in one. A later argument's closed class can then throw `CheckError`, and nothing reclaims what was shared, since unlike Go's `arena_` (`made`), Swift's `Arena` keeps no list of the buffers it gave.
- Reproduce it first: add a fixture function taking such a record or tuple by value, refuse it with a dropped handle from `api.c`, and check the buffer's drop count (`api_dropped`) and LeakSanitizer.
- Recommended fix: check every handle of an argument before lifting any of its buffers, as `reserve` already does for a moved one, so no failure can follow a take. The alternative, a failure path that frees each buffer not yet taken, has to track how far a record's lift got.
- Swift: track each buffer given to Rust in the arena until the call runs, and free them if it throws first, as Go's arena does; open each `Native.take` inside every throwing open, so nothing throws between a take and the call.
