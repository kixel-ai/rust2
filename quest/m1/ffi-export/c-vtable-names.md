# [S] A callback's C vtable compiles as C++ whatever its methods are named

## Goal

The C header's `<callback>_vtable` names each member so the header compiles as C and as
C++, which includes it: a trait method named as a C++ keyword (`delete`), a macro GNU
dialects predefine, or the table's own `free` gets a member name of its own, and every
binding that names the member (the Rust glue, Go, Swift, Dart) follows.

## Plan

Found by [cpp-name-collisions](https://github.com/kixel-ai/rust2/pull/914)' validation: a
trait `Undo { fn delete(&self, ..) }` writes `uint8_t (*delete)(void *data, ...)`, which
g++ refuses in `api.h` before the C++ header is read. `c.rs` names the member
`method.name()` raw, while its parameters already take `abi::escape`.

Recommended: name the member `abi::escape(method.name())`, kept apart from `free` and the
other members as C keeps a type's names apart, and have each binding read the name from
one place in `abi` rather than respell it. Prove it with an api fixture trait whose
methods are `delete` and `free`, implemented in each language's test.
