# [S] Kotlin variant classes never shadow a name their enum uses

## Goal

[rust2-ffi](/ffi/README.md)'s Kotlin builds for an enum whose data variant is named as
any type or object its sealed class's body names. A variant class is nested in the
sealed class, so inside that body its simple name wins over a builtin, the runtime, or
another exported type. On [#743](https://github.com/kixel-ai/rust2/pull/743)'s head, a
`Debug` enum with variants `Double(f64)` and `String(String)` fails to compile: each
field's type resolves to its own variant class, and `debug()`'s `Native.Writer()` and
`String` resolve to variants named `Native` and `String`. The common Rust shape
`enum Event { Point(Point) }` hits the same, its field typed as the variant itself.
Proven by an api fixture enum with such variants, built and compared in Kotlin.

## Plan

[#743](https://github.com/kixel-ai/rust2/pull/743) qualified the names its generated
`equals` and `hashCode` use (`kotlin.Any`, `kotlin.Boolean`, `kotlin.Int`, and
`<prefix>.Native`); the rest of a sealed class's body still names types bare.

Qualify every type and object the generated Kotlin names, builtins as
`kotlin.*` and the binding's own as `<prefix>.*`, rather than renaming the variant, so a
variant keeps its Rust name. Renaming a colliding variant with a trailing `_`, as
top-level types are, is smaller but renames a variant whenever a type of its name is added. A qualifier is still shadowed by a field or variable named
as it, such as a field `kotlin` or one named as the prefix, so the change also keeps
generated locals and fields apart from those two names.

