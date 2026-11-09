# m1: consumers waiting

## Goal

The work moq is waiting on: the exporter that replaces the UniFFI fork moq
builds its bindings with today.

## Plan

Decided with the maintainer on 2026-10-25, from the
[spike](https://github.com/moq-dev/rust2/pull/41): the [exporter](/ffi/README.md), which
turns a crate's `pub` API into bindings with no attributes, replaces the UniFFI fork. The
fork stays in maintenance and keeps building moq's bindings until then; its feature quests
are folded into the [exporter's](/quest/m1/ffi-export/README.md). The
[binding benchmark](/bench/ffi/README.md) continues as the exporter's baseline.

Decided 2026-11-02 (maintainer, moq's /quest-unblock): the fork is not published to crates.io. moq skips it and switches straight to the exporter ([moq's switch](https://github.com/moq-dev/moq/blob/main/quest/m3/ffi-exporter.md)), so publishing would only commit permanent `rust2-*` names to code already slated for retirement. moq keeps its own pinned generator forks until the exporter is ready.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - moq's bindings exported from its crates' `pub` API, replacing the fork
