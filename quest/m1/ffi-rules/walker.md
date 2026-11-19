# [M] Spike a shared lift and lower walker on Go

## Goal

A time-boxed spike: one walker over the IR emits lift and lower instructions, as
wit-bindgen's `Bindgen` does, and the Go generator implements them, with every test
passing. The PR measures a [#710](https://github.com/kixel-ai/rust2/pull/710)-sized rule
on it, files and lines in the Go generator before and after, and recommends whether every
generator moves to it. A no-go deletes the spike and keeps its numbers in the PR.

## Plan

Deferred by [the exporter epic](/quest/m1/ffi-export/README.md#plan) until its four `[L]`
children landed; they have. [The epic](/quest/m1/ffi-rules/README.md) holds the decision
and the per-rule measurements: the walker cuts generator duplication for rules that touch
several positions (#710 added 130 to 260 lines to each generator), and barely helps a new
value type, whose cost is mostly per-language runtime code.

Public API: none.

## Required

- [The transcript as data](/quest/m1/ffi-rules/transcript.md) - the tests a walker change must pass no longer need porting by hand
