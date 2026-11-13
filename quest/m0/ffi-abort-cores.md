# [S] Expected aborts leave no core dump

## Goal

The broken-contract cases in [`ffi/tests/fixtures.rs`](/ffi/tests/fixtures.rs), which
spawn each binding's test program and assert it aborts, still assert the abort and its
message, but on Linux the kernel never hands the dying child to the host's core handler:
no core is written or stored, `systemd-coredump` does not start, and the journal gets no
crash entry.

## Plan

Found by the 2026-11-13 write audit of the `nixos` CI host: of one boot's 6,007 cores,
5,957 were these expected aborts (C's `api-test zero`, C++17/20/23, Python, Dart, Kotlin
on `java`, Swift, Go), about 220 GB a day before that host stopped storing cores, and
fedora stored 5,116 more that day. Every one ran with `RLIMIT_CORE` unlimited. Each crash
also logs a CRIT journal entry, and journald syncs on every CRIT entry: about 35 GB a day
of writes on `nixos` that survive `Storage=none`. Agents' `just test` caused 75% of them,
the queue and the land fixers the rest. No single PR caused it: the tests abort by design.

What disabling the core does, measured on fedora (kernel 7.3, systemd 262, a piped
`core_pattern`) by aborting a child at each limit:

- `RLIMIT_CORE` at 0, set in the parent's `pre_exec`, survives `execve`. No core is
  written, but the kernel still pipes the crash to `systemd-coredump`, which logs
  "Resource limits disable core dumping" and then, at priority 2 (CRIT), "terminated
  abnormally without generating a coredump". A limit of 1 behaves the same. So this
  ends the core writes but neither the journald syncs nor the per-crash
  `systemd-coredump` unit.
- `prctl(PR_SET_DUMPABLE, 0)` stops the kernel before it calls the handler, but `execve`
  resets it, so a parent's `pre_exec` cannot set it. The child has to set it on itself.

Each test program already routes its broken cases through one `broken(case)` entry, so
marking itself non-dumpable there is one call per language on Linux: `prctl` in C and
C++, `ctypes` in Python, `Glibc.prctl` in Swift, `syscall`/`unix.Prctl` in Go, and FFI in
Dart. Kotlin runs on JDK 21, whose foreign-function API is still a preview there, so the
JVM cannot make the call without `--enable-preview`.
[#730](https://github.com/kixel-ai/rust2/pull/730) turns many Kotlin, Python, Dart and
Swift aborts into raised errors, but what a closure returns, C and C++ checks, and Rust
panics still abort, so this quest is needed either way; expect to merge with it in the
test programs and `fixtures.rs`.

Decided with the maintainer on 2026-11-13:

- Mechanism: ✅ each test program marks itself non-dumpable (`prctl(PR_SET_DUMPABLE, 0)`)
  at its `broken(case)` entry on Linux, and the Kotlin JVM's aborting cases, which cannot
  call `prctl` on JDK 21 without a preview flag, spawn with `RLIMIT_CORE` at 0 (rejected:
  `RLIMIT_CORE` at 0 for every case from one spawn helper, since every crash would still
  start `systemd-coredump` and log a CRIT entry; an `LD_PRELOAD` shim, for the build
  artifact it adds). Kotlin keeps one CRIT entry per abort and no core.
- Milestone: ✅ m0 (rejected: m1), since every agent's `just test` pays this.

Verify on the real surface: run `just test` for the ffi fixtures on Linux and compare
`coredumpctl list` and `journalctl -p crit` over the run before and after; the
after-run should show no entries from the test programs.

## Hosts

- linux - the core handler and `prctl` are Linux's; macOS's default core limit is already 0

## Related

- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - turns many of these aborts into raised errors, leaving fewer to cover
