# [L] C# and Unity bindings

## Goal

[rust2-ffi](/ffi/README.md) writes `csharp/<prefix>.cs`, the crate's API in C# over the C
ABI, which runs under .NET (NativeAOT included) and under Unity's IL2CPP on iOS and
Android. It follows every generator's rules
([the exporter epic](/quest/m1/ffi-export/README.md)), the `api` fixture's script runs
through it with a transcript matching native Rust's, and it is on
[the binding benchmark](/bench/ffi/README.md).

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ planned in m3 before a
consumer asks ([m3](/quest/m3/README.md#plan)); moq ships no C# today.

- Prior art, all annotation-based: csbindgen (reads `extern "C" fn`), Interoptopus,
  BoltFFI, Diplomat, and uniffi-bindgen-cs. None exports a `pub` API as it is.
- Recommended: blittable signatures only, called through function pointers
  (`delegate* unmanaged[Cdecl]`) so no marshalling stub runs, with records as
  `[StructLayout(LayoutKind.Sequential)]` structs at the C layouts, bytes as
  `ReadOnlySpan<byte>` in and a lent `ReadOnlyMemory<byte>` owner out, futures as
  `ValueTask` on the caller's `SynchronizationContext`, and handles as `SafeHandle`s with
  `IDisposable`. Confirm each against the C# language level and IL2CPP limits of Unity's
  current LTS, where callbacks into managed code need `[MonoPInvokeCallback]` and iOS
  links the library statically (`__Internal`); where the two runtimes need different
  code, the generator writes one file both compile.
- Unity cannot run in this repository's CI. Open: how Unity is proven. Recommended: the
  .NET test in the gate, compiled also at Unity's language level, and a nightly IL2CPP
  build without the editor if one can be made headless; report what is possible.

Public API: the generated C# API.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the C ABI, its lossless types, and its checks are settled before a new generator copies them
