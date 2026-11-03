# [S] Gate moq-net's translation

## Goal

A change that breaks moq-net's translation fails in the merge queue, not the next nightly:
`just ci` for a `ts` change translates moq-net at the pin and type-checks it, as
`just ts moq` does. Today only `nightly.yml` runs `just ts moq`, so
[moq-dev/rust2#519](https://github.com/moq-dev/rust2/pull/519) landed a panic on moq-net
that a bench run found hours later.

## Plan

`just ts check` already checks out the pin for the bench's type-check, so what this adds is
building moq-net's dependencies through the driver and translating it. Measure that in CPU,
cold and warm, against [the checks budget](/quest/m0/checks/README.md) before adding it;
if it costs too much, consider running it only for changes under `ts/src` or `ts/runtime`.

Public API: none.

## Related

- [Checks at moq's cost](/quest/m0/checks/README.md) - the gate's CPU budget this spends from
