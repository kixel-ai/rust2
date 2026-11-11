# [S] A warm gate

## Goal

The land gate's `just ci` builds through mbx, as moq's does, so a run rebuilds only what
changed.

## Plan

Decided with the maintainer on 2026-11-01: mbx in a per-repository cache mount, as moq's
gate has. PR code can write to it, a risk moq already accepts; mbx keys each compilation on
its compiler, target, and inputs. Local worktrees share it too, where the dev shell already
can.

Public API: none.

## Waiting

- Maintainer: move the land host's mbx pin from 1.28.0 to 1.29.2 or later, then rerun `mbx setup` wherever a shim exists, since a shim runs the mbx its `mbx-target` file names. Check: `mbx --version` on the land host prints 1.29.2 or later, and the shim's `mbx-target` names that store path. 1.28.0 prints no real paths for files in a build script's `OUT_DIR` ([jdx/mr-boxington#896](https://github.com/jdx/mr-boxington/issues/896)).
- Maintainer: give rust2's land instance moq's per-repository mbx profile, `id = 9`, `command = "just ci"`, `cpuQuota = 800`, `memoryHigh = "8G"`, `memorySwapMax = "1G"`, with `enable = lib.mkDefault false` until the next item. Check: the host config's `rust2` entry has these values.
- Maintainer: enable that profile once [#426](https://github.com/moq-dev/rust2/pull/426) has merged. Check: a gate run's log shows `mbx[cache]: object cache: ...`.
