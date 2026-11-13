# [S] The export's own cost in the binding benchmark

## Goal

[The binding benchmark](/bench/ffi/README.md)'s report gains a Build table beside Size: how
long [rust2-ffi](/ffi/README.md) takes to export moq, and how long the crate it generates
takes to build, each with its peak RSS. A consumer pays both on every export, and no
benchmark times the tool itself.

## Plan

Decided with the maintainer on 2026-11-12, in `/quest-plan`: ✅ in the binding benchmark's
report, since `just ffi bench` already runs the export (rejected: a scenario in
[the build benchmark](/bench/build/README.md), which measures upstream's levers and takes
hours; CodSpeed walltime, which is noisy on shared runners, while rustc under simulation
takes hours).

- Time the export apart from moq's own build of the same crates, so the table shows what
  the exporter adds, not what cargo costs anyway.
- Report it like the rest of that report: medians over the rounds beside the load
  average, no threshold.

Public API: none.
