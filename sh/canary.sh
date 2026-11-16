#!/usr/bin/env bash
# Point ts/rust-toolchain.toml, the nightly every driver links, at the newest one
# rust-overlay names that ships each component and target the file lists, and print
# it: the nightly's canary, which finds the next bump's breaks before it starts.
# It rewrites flake.lock and the toolchain file in place; commit neither.
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"
nix flake update rust-overlay
# shellcheck disable=SC2016 # the Nix expression interpolates, not the shell
version=$(nix eval --raw --impure --expr '
  let
    flake = builtins.getFlake (toString ./.);
    pkgs = import flake.inputs.nixpkgs {
      system = builtins.currentSystem;
      overlays = [ (import flake.inputs.rust-overlay) ];
    };
    file = (builtins.fromTOML (builtins.readFile ./ts/rust-toolchain.toml)).toolchain;
  in
  # fromRustupToolchainFile, which builds the dev shell, uses the default profile
  # when the file names none: check the same one, so the two agree.
  (pkgs.rust-bin.selectLatestNightlyWith (t: t.${file.profile or "default"}.override {
    extensions = file.components;
    targets = file.targets;
  })).version')
channel="nightly-${version##*-nightly-}"
sed -i "s/^channel = \".*\"/channel = \"$channel\"/" ts/rust-toolchain.toml
grep -q "^channel = \"$channel\"$" ts/rust-toolchain.toml
echo "$channel"
