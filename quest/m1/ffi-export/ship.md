# [S] Ship the exporter for moq to pin

## Goal

moq can pin [rust2-ffi](/ffi/README.md): the flake has an `ffi` package, the root
`README.md` lists the generators, and the rules every generator follows move from
[the epic](/quest/m1/ffi-export/README.md) into `ffi/README.md`, since the epic's file is
deleted with its last child. This is the epic's last child, and what
[moq's wait](https://github.com/moq-dev/moq/blob/main/quest/m3/rust2-exporter.md) checks.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ the closing work is this
child, which alone waits on a clean census. Rejected: the epic requiring
the refusals epic, since finished, which held back every other child.

Every sibling lands first, since each breaks a generated API moq would otherwise adopt and
then see change. A sibling planned later adds itself to `Required` here.
