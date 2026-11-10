# [XS] Waiting: Swift 6.4 in nixpkgs-unstable

## Goal

rust2-ffi's Swift builds and runs on aarch64-linux: `api_swift` runs there, and
`just ffi moq` and `just ffi bench` stop skipping the Swift.

## Plan

Decided with the maintainer on 2026-10-26, from
[#93](https://github.com/moq-dev/rust2/pull/93): swiftc 6.3.3 is marked broken on
aarch64-linux in the pinned nixpkgs, where its gold link fails, so the default dev shell
leaves Swift out there. Bump the `nixpkgs` input in its own change, then drop
`api_swift`'s `ignore` in `ffi/tests/fixtures.rs`, the Swift skips in `just ffi moq` and
`just ffi bench` (`ffi/justfile`), and the default dev shell's "not on aarch64-linux"
comment in `flake.nix` that [#653](https://github.com/moq-dev/rust2/pull/653) added. Keep
that shell's `swiftc.meta.broken` guard, which covers any later broken Swift. Never allow
the broken package.

Public API: none.

## Waiting

- NixOS/nixpkgs: the latest `nixpkgs-unstable` carries Swift 6.4.0
  ([NixOS/nixpkgs#574002](https://github.com/NixOS/nixpkgs/pull/574002), merged to
  `staging` on 2026-10-22 and to `master` on 2026-11-07 through
  [NixOS/nixpkgs#580394](https://github.com/NixOS/nixpkgs/pull/580394), not in
  `nixpkgs-unstable` on 2026-11-09). Check that
  `nix eval .#devShells.aarch64-linux.default.nativeBuildInputs` lists Swift after a bump.
