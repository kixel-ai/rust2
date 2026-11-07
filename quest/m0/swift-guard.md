# [XS] Leave out a broken Swift on aarch64

## Goal

The default dev shell leaves Swift out wherever nixpkgs marks `swiftc` broken, so it
evaluates on aarch64-linux and the nightly's aarch64 jobs pass again. Today they have
failed since 2026-10-29 on `Refusing to evaluate package 'swiftc-6.3.3'`.

## Plan

The guard `!pkgs.swift.meta.broken` (`flake.nix`, the default shell's
`nativeBuildInputs`) is always true: nixpkgs' `swift` wrapper copies only `platforms` and
`badPlatforms` from `swiftc`, not `broken`,
so the shell still pulls in the broken compiler. Guard on `pkgs.swiftc.meta.broken`
instead. Found while reviewing [#617](https://github.com/moq-dev/rust2/pull/617), whose
nixpkgs bump does not change it. The guard came in with the comment's "not on
aarch64-linux" note; name its PR under `## Caused by`.

[Swift 6.4 in nixpkgs-unstable](/quest/m2/uniffi-swift-arm.md) still restores Swift on
aarch64 once nixpkgs ships it.

Decided with the maintainer on 2026-11-07: ✅ m0 (rejected: fold into the m2 waiting
quest, drop), since a red nightly hides new failures.
