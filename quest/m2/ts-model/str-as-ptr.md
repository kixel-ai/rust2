# [S] A string's bytes through its pointer

## Goal

[rust2-ts](/ts/README.md) translates `str::as_ptr` as a read-only pointer to the string's
UTF-8, so `ptr::copy_nonoverlapping` and `slice::from_raw_parts` read what Rust reads,
proven by differential tests against native Rust. At the moq pin arrayvec's
`ArrayString` refuses it once in the model and 3 times in moq-net whole.

## Plan

A `&str` stays a JS string (decided on #574), so the bytes are an encoding of it; a pointer into
them must not let a write through it go unnoticed, which Rust forbids anyway for a `&str`.

Public API: the translator's output.
