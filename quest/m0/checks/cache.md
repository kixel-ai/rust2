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

The dev shell already puts mbx's Cargo shim ahead of its toolchains wherever `mbx setup`
installed one. What remains is the host: kixelated/nixos's `rust2` land instance mounts and
configures an mbx cache, as its `moq-pro` instance does with `mbxCheck`. Check it by
reading a gate run's log for mbx's cache summary.

Through mbx 1.29.1, `uniffi_uitests` fails: the diagnostics of rustc runs that trybuild's
nested cargo starts through mbx's wrapper keep the `${target}` placeholder mbx stores paths
under, so trybuild cannot match its snapshots. `MBX_DISABLE=1` passes. Get it fixed in mbx
(jdx/mr-boxington), or decide with the maintainer to keep that test off mbx meanwhile.

Public API: none.
