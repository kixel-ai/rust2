# [M] Exclusive pinned cores for bench mode

## Goal

`swole bench` can reserve exclusive physical core(s) on Linux—pinning benchmark processes to dedicated core(s) with each core's SMT sibling held idle and the `performance` frequency governor active (coordinating with host leases / `quest gate` where present), and cleanly restoring core reservations and governor state on exit—reducing shared-host wall-clock noise at its source.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include exclusive core reservation for `swole bench` in the `swole` epic to cut shared-host timing noise and agent turn time.

Recommendations for the implementer:

- Read CPU topology from `/sys/devices/system/cpu/cpu*/topology/thread_siblings_list` to pair each physical core with its SMT sibling(s) and keep the sibling idle during measurement.
- Integrate cleanly with host scheduling/lease coordination where present (such as `quest gate` / dotfiles' `ci-lease` or a lockfile under `/run` or `~/.cache/swole/` so concurrent `swole bench` runs claim disjoint physical cores rather than contending on the same core).
- When the host permits cpufreq control (or via a minimal helper/capability), switch the reserved core's scaling governor to `performance` and restore the original governor on drop/signal (including `SIGINT`/`SIGTERM`); when cpufreq is read-only, probe once at setup, report the active governor in the fingerprint/header, and never silently claim exclusivity it doesn't hold.
- Verify on a loaded multi-core Linux host with an A/A `swole bench HEAD` run comparing unpinned shared execution against exclusive-core pinned execution and recording the interval tightening in the PR.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - integrates into `swole bench` job execution

## Hosts

- linux - CPU affinity, SMT sibling topology, and cpufreq governor
