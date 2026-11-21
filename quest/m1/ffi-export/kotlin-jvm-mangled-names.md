# [M] Kotlin: Java calls a member Kotlin mangles by the JVM name the lock records

## Goal

Every generated Kotlin function and property that plain Java can call has the JVM name the
lock's `/ jvm` entry records, including those Kotlin mangles for an inline class, and one
Java cannot call records no JVM name, so the lock never holds a name the class lacks.

## Plan

Found by CodeRabbit on [kixel-ai/rust2#934](https://github.com/kixel-ai/rust2/pull/934#discussion_r4652081539).
Since [#919](https://github.com/kixel-ai/rust2/pull/919), `Gen::function` in
`ffi/src/kotlin.rs` writes the name it computes to the `/ jvm` lock entry and emits
`@JvmName` only where that name differs from the declared one. Kotlin mangles the JVM name of
a function taking an inline class (`UInt`, `ULong`, `UShort`, `UByte`, an opaque `value
class`) and of a class member returning one, so `ClashStore.count(): UInt` is
`count-pVg5ArA` on the JVM while the lock records `count`. A hyphenated name is not a Java
identifier, so Java cannot call any such member, and the lock's break check watches a name
that does not exist.

Checked with kotlinc and javap on 2026-11-21: a top-level function returning `UInt` is not
mangled; a member is. `@JvmName` on a final member or a property getter pins the name
exactly, mangled or not, and Java calls it. `@JvmName` on a member of a `value class` is a
compile error, and such a member compiles to a static `name-hash(bits)`.

Recommendation: emit `@JvmName` on every function and getter Kotlin would mangle, with the
name `Gen::jvm_named` gives it, so the lock's entry is the method's name and `Caller.java`
calls one, such as `ClashStore.count()`. A member of an opaque value class records no
`/ jvm` entry, as Java cannot call it. Rejected: recording no JVM name for every mangled
member, which keeps unsigned APIs out of Java's reach.
