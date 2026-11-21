# [M] Kotlin: plain Java reaches what Kotlin's callers reach

## Goal

Plain Java, compiling against the generated Kotlin's classes, calls every function, reads
every field, and makes every value a Kotlin caller can, apart from a value class's members
and a function taking an opaque value, which stay Kotlin's, so a Java caller of the Kotlin
library never needs reflection or a Kotlin shim.

## Plan

Found by [kixel-ai/rust2#945](https://github.com/kixel-ai/rust2/pull/945), which pins the
JVM name of each function and getter Kotlin would mangle for an unsigned type. These gaps
remain, each checked with kotlinc 2.4 and javap on 2026-11-21:

- A companion function named `new` or `default`, as most constructors are, keeps that JVM
  name, which Java's grammar refuses, so Java makes no object through one.
  Recommended: [`Gen::jvm_named`](/ffi/src/kotlin.rs) counts Java's reserved words as
  taken, as it does `Object`'s final methods, so `new` is `new_` on the JVM alone; this
  breaks each such entry the lock's `/ jvm` key records once.
- A record's, variant's, or range's field of an unsigned type has a mangled getter, such
  as `getSeq-pVg5ArA`. Recommended: `@get:JvmName("getSeq")` on each, which a data class's
  constructor property takes; no lock entry is needed, as the getter's name follows from
  the field's, which the lock records.
- A data class whose constructor takes an unsigned type compiles it to a synthetic one
  taking a `DefaultConstructorMarker`, so Java makes no such record. Recommended: a
  companion `@JvmStatic` factory pinned by `@JvmName`, named as no Rust method takes,
  over making Java build the `kotlin.UInt` boxes `@JvmExposeBoxed` would take.
- The runtime's own types (`Duration`, `Ipv4Addr`, `SocketAddrV4`, `SocketAddrV6`) have
  members and constructors of unsigned types with the same mangled names. Recommended:
  the same pins in `ffi/runtime/types.kt`, which a value class's members cannot take, so
  `Ipv4Addr`'s and `SocketAddrV4`'s gain companion functions Java calls.

Prove each by a `Caller.java` call, as #945 does for a mangled function.

