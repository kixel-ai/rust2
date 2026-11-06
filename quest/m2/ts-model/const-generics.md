# [M] Const generic types

## Goal

[rust2-ts](/ts/README.md) translates structs and enums with const parameters, such as
arrayvec's `ArrayString<CAP>` and `ArrayVec<T, N>`, so each instance's body reads its own
`N`, proven by differential tests against native Rust. At the moq pin it refuses once in
the model (`Path::join`'s `ArrayString`) and 8 times in moq-net whole.

## Plan

Functions with const parameters already translate per instance; types do not yet. Check
how the choice interacts with a type's class being shared across instances.

Public API: the translator's output.
