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
[the refusals epic](/quest/m1/ffi-refusals/README.md), which held back every other child.

Every sibling lands first, since each breaks a generated API moq would otherwise adopt and
then see change. A sibling planned later adds itself to `Required` here.

## Required

- [Refusals](/quest/m1/ffi-refusals/README.md) - the census lists no refusals at the moq pin, and `ffi.toml` is gone
- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - which generated functions throw
- [Lossless binding types](/quest/m1/ffi-export/lossless.md) - the types of durations, addresses, paths, and text
- [Copy types cross by value](/quest/m1/ffi-export/copy-values.md) - how a `Copy` type is held in every language
- [Not Send](/quest/m1/ffi-export/not-send.md) - the types and futures that export checked by thread
- [Refuse unsafe functions](/quest/m1/ffi-export/unsafe-fns.md) - a soundness hole closed before the pin
- [Export moq for every target](/quest/m1/ffi-export/moq-targets.md) - one binding across the targets moq ships
- [Fused async start](/quest/m1/ffi-export/async-start.md) - every future's C signature
- [Numeric slices](/quest/m1/ffi-export/numeric-slices.md) - the types of numeric slices and vectors
- [API lock](/quest/m1/ffi-export/api-lock.md) - the check moq's releases run on the exported names
