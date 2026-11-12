# [S] A warm gate

## Goal

The merge queue's `just ci` builds through mbx, as moq's does, so a run rebuilds only what
changed.

## Plan

Decided with the maintainer on 2026-11-01: mbx in a per-repository cache mount, as moq's
gate has. PR code can write to it, a risk moq already accepts; mbx keys each compilation on
its compiler, target, and inputs. Local worktrees share it too, where the dev shell already
can.

The gate is no longer a `quest land` check: rust2 lands through GitHub's merge queue, whose
build (`.github/workflows/queue-build.yml`) runs on the host's `rust2-queue` runner. That
runner already has a persistent home and mbx's cache environment and job-boundary GC from
the host's `ci-runners.nix`, but nothing installs mbx or its Cargo shim, so the dev shell's
`mbx setup --status` finds no shim and the build is cold: a 2026-11-12 queue build logged
no `mbx` line and 379 `Compiling` lines. Install them in the queue build as moq's
`.github/actions/rust-cache` does on its `moq-queue` runner: `jdx/mr-boxington-action` at
mbx 1.29.2 or later (1.28.0 prints no real paths for files in a build script's `OUT_DIR`,
[jdx/mr-boxington#896](https://github.com/jdx/mr-boxington/issues/896)), then
`mbx setup --yes`, failing the job when no shim results, with its guard that only main's
merge queue writes the persistent store. [#426](https://github.com/moq-dev/rust2/pull/426)
already puts the shim ahead of the pinned toolchain in the dev shell. Done when a queue
build's log shows `mbx[cache]: object cache: ...`.

Public API: none.
