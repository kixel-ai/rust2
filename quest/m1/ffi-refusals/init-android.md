# [M] Export moq-tokio's init_android on Android

## Goal

`moq_tokio::tls::init_android(env: &mut jni::Env, context: JObject)`, which exists only on
Android, exports there, so `ffi/moq/ffi.toml` drops its exclude and the Android staging of
`just ffi moq-api` refuses nothing. Its `jni` types match no rule in
[rust2-ffi](/ffi/README.md); without the exclude, the export follows them into the whole
`jni` crate and refuses 185 items.

## Plan

`tls::Error::AndroidInit(jni::errors::Error)` needs nothing: `tls::Error` crosses as an
opaque error with its message, so its variants are no part of the binding.

moq-ffi calls `init_android` today from its own `JNI_OnLoad`, with the application
`Context` it finds through `ActivityThread.currentApplication()`, so apps need no setup. The
generated Kotlin glue defines `JNI_OnLoad` too, so the two cannot share a library once the
real moq-ffi replaces the stand-in.

Decided (maintainer, 2026-11-14,
[kixel-ai/rust2#810](https://github.com/kixel-ai/rust2/pull/810#issuecomment-6883266024)):
✅ a general rule (rejected: the glue calling `init_android` itself when the library loads,
which names one moq function in the exporter; moq keeping `init_android` out of its `pub`
API). A `pub fn` taking a `&mut jni::Env` exports to Kotlin alone, the glue passing the env
of the calling thread, and a `JObject` crosses as Kotlin's `Any`. Kotlin then calls
`tls.initAndroid(context)`, which apps must call themselves unless moq's Kotlin calls it for
them. This quest also resolves the `JNI_OnLoad` clash with moq-ffi's own Android code.
