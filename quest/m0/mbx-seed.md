# [XS] Turn mbx's target seeding back on

## Goal

`flake.nix` no longer sets `MBX_TARGET_SEED`, and a fresh checkout's `just ffi surface`
passes with seeding on, locally and in the merge queue.

## Plan

mbx's seeding copies registry build units into every target directory it learned from
another checkout, nested ones such as `target/rust2-ffi` included, and creates them
without cargo's `CACHEDIR.TAG`; `cargo clean --target-dir` then refuses them. Once mbx
tags what it creates, or stops seeding a directory the checkout never built in, delete the
setting, bump the queue's mbx pin in `.github/workflows/queue-build.yml` to that release,
and check a fresh worktree whose first build seeds `target/rust2-ffi` from another.

## Waiting

- Maintainer: file the mbx issue drafted for this, and link it here
- jdx/mr-boxington: a release whose seeding writes `CACHEDIR.TAG` or skips nested target
  directories
