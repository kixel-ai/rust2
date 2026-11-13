# m1: consumers waiting

## Goal

The work moq is waiting on: the exporter that replaces the UniFFI generator forks moq
builds its bindings with today.

## Plan

Decided with the maintainer on 2026-10-25, from the
[spike](https://github.com/moq-dev/rust2/pull/41): the [exporter](/ffi/README.md), which
turns a crate's `pub` API into bindings with no attributes, replaces the UniFFI fork,
whose feature quests are folded into the [exporter's](/quest/m1/ffi-export/README.md).

Decided 2026-11-02 (maintainer, moq's /quest-unblock): the fork is not published to crates.io. moq skips it and switches straight to the exporter ([moq's switch](https://github.com/moq-dev/moq/blob/main/quest/m3/ffi-exporter.md)), so publishing would only commit permanent `rust2-*` names to code already slated for retirement. moq keeps its own pinned generator forks until the exporter is ready.

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ the fork is deleted now,
since moq never builds on it and its only consumers here were the binding benchmark's
baseline and nightly jobs. Rejected: keeping it until moq switches; keeping it as is.

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ exporter work that
changes no generated API is [its own epic](/quest/m1/ffi-speed/README.md), so it does not
hold moq's pin open. Rejected: keeping it in the exporter's epic.
