# [M] The C# binding built and run under Unity's IL2CPP

## Goal

A nightly job builds the `api` fixture's C# test as a Unity player with IL2CPP, headless,
for Linux, and runs it: its transcript matches native Rust's and its checks pass, as
`ffi/tests/fixtures.rs`'s `api_csharp` does on .NET. Today the gate proves Unity only by
compiling the binding at C# 9 against .NET Standard 2.1 with `UNITY_5_3_OR_NEWER` and
stub attributes (`ffi/tests/unity.cs`), and running that half of it on CoreCLR.

## Plan

- Recommended: GameCI's `unityci/editor` image for the current LTS with the Linux IL2CPP
  module, `-batchmode -nographics -buildTarget StandaloneLinux64` and a build script that
  sets `ScriptingImplementation.IL2CPP`, on the nightly's self-hosted runner. The player
  loads the fixture's library from `Plugins/`, and its `Main` is the test's, called from
  a `RuntimeInitializeOnLoadMethod` that quits with the exit code.
- iOS (`__Internal`, static linking) and Android stay unproven until a device job exists;
  the Linux player exercises the same IL2CPP callbacks (`MonoPInvokeCallback`).
- Decided 2026-11-23 (maintainer, /quest-run): ✅ keep the license wait, deferred
  (rejected: dropping it). The maintainer's Unity account holds only a Unity Personal seat,
  not activated, and no Pro serial, and GameCI's docs say a Personal `.ulf` comes only from
  activating in the Unity Hub desktop app, never from the web dashboard; so the license is
  a Personal one, activated in Hub on a desktop.

## Waiting

- Maintainer: activate a Unity Personal license in the Unity Hub desktop app (Preferences >
  Licenses > Add > Get a free personal license), then set `UNITY_LICENSE` (the `.ulf` file's
  contents), `UNITY_EMAIL`, and `UNITY_PASSWORD` as kixel-ai/rust2 repository secrets.
  Then sign out of Unity Hub: since Hub 3.7.0 that is how a Personal seat returns, and a
  desktop still holding it blocks CI's activation.
  An agent cannot take it: activation signs in to the maintainer's Unity ID in a desktop
  app. Considered: a build without activation, which the editor refuses even in batch mode.
  Check:
  `gh api repos/kixel-ai/rust2/actions/secrets --jq '[.secrets[].name]'` lists the three.

## Related

- [C# in the shapes, protocol, and corpus tests](/quest/m3/csharp-tests.md)
