# [M] Kotlin: plain Java reaches what Kotlin's callers reach

## Goal

Plain Java, compiling against the generated Kotlin's classes, calls every function, reads
every field, makes every value, boxes and unboxes every unsigned value, and implements every
callback interface a Kotlin caller can, an opaque value's members and the functions taking
one included, through types as checked as Kotlin's, so a Java caller of the Kotlin library
never needs reflection or a Kotlin shim, and never passes Rust bits it could forge.

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
- A callback interface's abstract method taking or returning an unsigned type, as `Log`'s
  `log(level: UByte, ...)`, is mangled (`log-0ky7B_Q(byte, String)`), so no Java class
  implements the interface, and Kotlin refuses `@JvmName` on an open member.
  Recommended: an interface that has one gains a nested `Java` interface of the same
  methods, each unsigned type as the primitive the JVM passes (`fun log(level: Byte,
  ...)`, `fun count(): Int`), which nothing mangles, and a companion
  `@JvmStatic fun of(java: Java): Log` wrapping one in the Kotlin interface by
  `toUByte()` and `toByte()`; Java writes `Log.of(new MyLog())`. Rejected: an abstract
  adapter class implementing `Log`, since a method whose only unsigned type is its return,
  `count(): UInt`, cannot share its Kotlin name with `count(): Int`, a conflicting
  overload; signed types in `Log` itself, which Kotlin's implementations would lose.
- A nullable or generic unsigned value is a `kotlin.UInt` box on the JVM, as
  `Berth.new(at: UInt?)` takes and `measured(f: () -> ULong)`'s closure returns, and Java
  can neither make one nor read its bits: `kotlin.UInt`'s only constructor and accessor are
  `box-impl(int)` and `unbox-impl()`. Recommended: the runtime gains an `Unsigned` object
  of `@JvmStatic` functions pinned by `@JvmName`, `box(int): UInt?` and `bits(UInt?): int`
  for each unsigned type, overloaded on the primitive, which a probe checked: Java's
  `Unsigned.bits(at(Unsigned.box(-2)))` read `4294967295` back, and a Java lambda
  returning `Unsigned.box(-2L)` passed to `measured` returned `u64::MAX`. Rejected: a
  `Java` overload of each such function, which multiplies the surface.
- An opaque value Kotlin holds in a value class reaches no Java caller: its members compile
  to statics such as `number-impl(long)`, and since
  [kixel-ai/rust2#945](https://github.com/kixel-ai/rust2/pull/945) a function taking one
  has a JVM name no Java identifier is (`take-opaque`), as Java would pass bits Rust reads
  unchecked, while one returning one hands Java its bare `long`. Recommended:
  `@JvmExposeBoxed` (experimental in Kotlin 2.2 and on, opted into by the generated file
  with `@OptIn(ExperimentalStdlibApi::class)`, which asks nothing of a consumer's build) on
  each function taking or returning one and on each member of its class, so Java holds the
  boxed class Rust made and calls `make()`, `v.next()`, and `take(V)` by name; a probe with
  kotlinc 2.4.21 compiled and ran exactly that from Java. A function returning one then
  returns the box, not the `long`. Rejected: a runtime handle type with a checked
  conversion, which adds a class per opaque value for what the box already is; a
  documented `long` contract, checked or not, which leaves type confusion between opaque
  types.

Prove each by a `Caller.java` call, and the callback by a Java class implementing one, as
#945 does for a mangled function.

