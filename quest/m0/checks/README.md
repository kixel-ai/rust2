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

Its children have landed; what remains is the measurement: once a queue build logs
`mbx[cache]: object cache: ...`, confirming the warm gate, report the gate's and local CPU
against the baseline above, cold and warm, and record any gap as a new quest.
