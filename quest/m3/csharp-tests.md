# [M] C# in the shapes, protocol, and corpus tests

## Goal

The C# binding runs every test the other bindings run, beyond the `api` fixture's
transcript: a driver per sampled shape in the [shape tests](/ffi/README.md#tests), gate
subset and nightly; the protocol scripts through the C# in the gate and nightly; each
corpus crate's C# compiled for .NET and at Unity's level; `moq/moq.cs`, run against moq's
export by `just ffi moq`; and the C# outline in `ffi/surface/`.

## Plan

- Recommended: copy the Go and Kotlin drivers' shape, compiling with the `csc` helper in
  `ffi/tests/fixtures.rs` (Roslyn against the SDK's reference pack, no MSBuild), so each
  adds seconds, not a restore.
- The surface outline is the generated file's public declarations, as Kotlin's and
  Swift's are, through `ffi/surface/surface.py`.

## Related

- [Unity IL2CPP build](/quest/m3/unity-il2cpp.md) - runs the same test under IL2CPP
