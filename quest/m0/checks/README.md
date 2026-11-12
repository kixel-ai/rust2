# [S] Checks at moq's cost

## Goal

The merge queue's build and agents' local checks cost what moq's do: `just ci` runs only what a
change touches, builds warm, and runs the differential fixtures optimized, while a nightly
`just gate` runs the full suite. Measured against the 2026-11-01 audit's baseline: about
33 CPU-hours a day in the gate (65 CPU-minutes a run) and 100 to 130 locally.

## Plan

Decided with the maintainer on 2026-11-01, from an audit of the gate's and agents' checks
(nixos2 and the host's systemd accounting): copy how moq works. `just ci` before every push
stays the rule, and costs less now that it selects by the diff
([#421](https://github.com/moq-dev/rust2/pull/421)); CONTRIBUTING.md and the skills
keep their rules. All of it is m0, since every agent pays the cost now.

Its children have landed; what remains is the measurement: report the gate's and local CPU
against the baseline above, cold and warm, and record any gap as a new quest.
The CPU numbers come from the runner host's systemd accounting, which an agent without
access to nixos2 cannot read; queue wall time from the Actions API stands in until then.

The first warm queue build that compiled,
[run 43265518031](https://github.com/moq-dev/rust2/actions/runs/43265518031) for
[#715](https://github.com/moq-dev/rust2/pull/715), logged `mbx[cache]` with 0 hits,
55 misses, and 10 bypassed (`unsupported-crate-type`), storing 66.2 MiB: it seeded an empty
store, so it shows the store fills, not that it serves. Its Build took 7m18s, against 9m23s
to 23m46s for the three queue builds before it. The only later queue build,
[run 43286222929](https://github.com/moq-dev/rust2/actions/runs/43286222929) for #718,
compiled nothing (23 s, diff-selected checks only). The measurement starts at the next
queue build that compiles Rust: if its `object cache` line still shows 0 hits, the warm
gate does not work and that is the gap quest; otherwise compare its CPU with the baseline.

## Waiting

- Maintainer: read the queue builds' CPU from nixos2's systemd accounting (the baseline's
  source), cold and warm, since agents cannot reach the host; done when this Plan records
  them. Considered: Actions timing alone, which gives wall time, not CPU.
