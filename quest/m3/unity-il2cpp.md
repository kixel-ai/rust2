# [M] The C# binding built and run under Unity's IL2CPP

## Goal

A nightly job builds the `api` fixture's C# test as a Unity player with IL2CPP, headless,
for Linux, and runs it: its transcript matches native Rust's and its checks pass, as
`ffi/tests/fixtures.rs`'s `api_csharp` does on .NET. Today the gate proves Unity only by
compiling the binding at C# 9 against .NET Standard 2.1 with `UNITY_5_3_OR_NEWER` and
stub attributes (`ffi/tests/unity.cs`).

## Plan

- Recommended: GameCI's `unityci/editor` image for the current LTS with the Linux IL2CPP
  module, `-batchmode -nographics -buildTarget StandaloneLinux64` and a build script that
  sets `ScriptingImplementation.IL2CPP`, on the nightly's self-hosted runner. The player
  loads the fixture's library from `Plugins/`, and its `Main` is the test's, called from
  a `RuntimeInitializeOnLoadMethod` that quits with the exit code.
- iOS (`__Internal`, static linking) and Android stay unproven until a device job exists;
  the Linux player exercises the same IL2CPP callbacks (`MonoPInvokeCallback`).

## Waiting

- Maintainer: a Unity license the nightly can activate (a Personal license's `.ulf`, or a
  Pro serial with its account), stored as a repository secret. An agent cannot take it:
  activation signs in to the maintainer's Unity ID. Considered: a build without
  activation, which the editor refuses even in batch mode. Check: the secret `UNITY_LICENSE`
  exists in kixel-ai/rust2's settings.

## Related

- [C# in the shapes, protocol, and corpus tests](/quest/m3/csharp-tests.md)
