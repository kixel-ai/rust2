# [XS] A warm gate

## Goal

The land gate's `just ci` builds through mbx, as moq's does, so a run rebuilds only what
changed: measured on nixos2, root clippy falls from 399 to 83 CPU-seconds warm, and uniffi's
test build from 291 to 20.

## Plan

Decided with the maintainer on 2026-11-01: mbx in a per-repository cache mount, as moq's
gate has. PR code can write to it, a risk moq already accepts; mbx keys each compilation on
its compiler, target, and inputs. Local worktrees share it too, where the dev shell already
can.

The dev shell puts mbx's Cargo shim ahead of its toolchains wherever `mbx setup` installed
one. mbx must first print real paths in diagnostics: through 1.29.1, an error in a build
script's `OUT_DIR` names the file by mbx's placeholder (`${out_dir}`, `${target}`), which
fails `uniffi_uitests`'s trybuild snapshots
([jdx/mr-boxington#896](https://github.com/jdx/mr-boxington/issues/896)). Decided with the
maintainer on 2026-11-02: wait for the upstream fix rather than keep that test off mbx.

Once a release fixes it, kixelated/nixos's `quest-land` module moves its mbx pin to that
release (the version in `packages/mbx.nix` and `mbxCheck`'s `prepare` stamp), and the `rust2`
land instance takes moq-pro's profile (maintainer, 2026-11-02):

```nix
rust2 = mbxCheck // {
  id = 9;
  enable = lib.mkDefault false;
  # check.yml's Check runs the same recipe; it needs no base.
  command = "just ci";
  cpuQuota = 800;
  memoryHigh = "8G";
  memorySwapMax = "1G";
};
```

Check it by reading a gate run's log for mbx's cache summary.

Public API: none.
