# [S] Every lang case reachable

## Goal

The `lang` differential reaches every case its harness lists, and adding a case past what
the selector byte encodes fails loudly instead of silently aliasing another.

## Plan

Found reviewing the cheaper differential on 2026-11-01: `harness::cases()` lists 256 cases,
so `borrows::leaks` is op 0xff, which `entry` routes to `fmt::Fault`. Neither its generated
inputs nor `caller.ts`'s `leaks` scripts run it. `op` truncates an index to `u8`, and `run`
picks a case by `op % cases.len()`, so a 257th case would alias case 0. The
[lang split](/quest/m0/checks/fixture-layout.md) changes the harness's layout, so settle the
encoding with it in mind.

Public API: none.
