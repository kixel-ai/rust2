# [L] Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage

## Goal

`swole` unifies its CLI around three modes over the same compiled benchmark binary—`swole quick [<base>]`, `swole sim [<base>]`, and `swole bench [<base>]`—probes the host's measurement capabilities once at setup (hashing them into the fingerprint), collects every core Linux `perf` counter the host exposes at L2 (user) and L1 (user+kernel) via multi-pass event groups with zero kernel multiplexing, and always collects OS `rusage` metrics (user/sys CPU time, max RSS, minor/major page faults, voluntary/involuntary context switches) on Linux and macOS.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Three modes on one binary:
  - `swole quick [<base>]`: single-shot native execution at 1× speed, parallel across the PMU's cores, with `preload.rs` (fixed RNG/clock), `ADDR_NO_RANDOMIZE`, and padded launch. Collects core `perf` counters (`instructions:u`, `cycles`, `branches`, `branch-misses`, L1/LLC misses) + `rusage` (host-fingerprinted; fails loudly on hosts with no perf counters, directing the user to `swole sim` or `swole bench`).
  - `swole sim [<base>]`: deterministic single-shot Valgrind Callgrind (the CodSpeed replacement, cross-host, 0% noise, works on PMU-less VMs and CI).
  - `swole bench [<base>]`: native multi-iteration execution (standalone or paired alternating `<base>` vs head, carrying forward #1033's median + distribution-free 95% CI) collecting wall-clock + `rusage` + every `perf` counter tier the host's setup probe unlocks.
- ✅ Probe once at setup, never silently fall back or multiplex at runtime:
  - Read `perf_event_paranoid`, effective `CAP_PERFMON`, available PMUs under `/sys/bus/event_source/devices`, and the core PMU's hardware counter slot count (e.g. 6 on AMD Zen 5), and hash them into the native fingerprint. Print the active capability tier and metric set before running.
  - When the active metric set needs more events than hardware counter slots, run one pass per event group (leader + siblings pinned together) rather than letting the kernel multiplex and scale; any counter with `running != enabled` still fails the run loudly.
  - At L2 (`paranoid <= 2`): user-space instructions, cycles (and derived IPC), branches and mispredicts, L1d/L1i/LLC/dTLB/iTLB misses, CPU top-down frontend/backend stall counters where the PMU exports them, and software events (`task-clock`, `page-faults`, `context-switches`, `cpu-migrations`).
  - At L1 (`paranoid <= 1` or `CAP_PERFMON`): also measure kernel counters (`exclude_kernel = 0`) and report user vs kernel split, exposing real syscall and `io_uring` costs.
  - Where `perf` is unavailable (Linux `paranoid >= 3`, PMU-less VMs, macOS), `swole bench` decides at setup to collect wall-clock + OS `rusage` (`getrusage` / `CLOCK_THREAD_CPUTIME_ID` / macOS `proc_pid_rusage`) cleanly and prints why perf counters are absent.
- ✅ Privilege model: grant `CAP_PERFMON` to the `swole` binary (`setcap cap_perfmon=ep` or NixOS `security.wrappers`) on Linux hosts so the host keeps its default `perf_event_paranoid` while `swole` gets full perf access; document and ship the NixOS wrapper snippet.

Recommendations for the implementer:

- Emit all collected metrics in both the terminal comparison table (grouped concisely, hiding unchanged noise columns or summarizing cleanly) and Kixel Quest's `metrics.json` (respecting Kixel Quest's measure-name and payload limits; check whether a per-metric series naming convention needs any adjustment in `kixel.quest`).
- Keep `swole quick` and `swole sim` `--out` and `--allow-missing` behavior compatible with Kixel Quest's nightly and bisect workflows.
- Verify multi-pass counter groups against `perf stat` on a known microbenchmark (differential check that instructions, cycles, branches, and misses match stock `perf stat` within measurement noise and never multiplex).

Public API: CLI subcommands (`quick`, `sim`, `bench`, `fingerprint`), `metrics.json` measures, and harness-to-runner protocol.

## Hosts

- linux - verifying Linux perf capability tiers, multi-pass counter groups, and `CAP_PERFMON`
