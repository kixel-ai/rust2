# [S] Python calls skip the loop lookup

## Goal

A call into the exporter's [Python](/ffi/README.md) while an asyncio loop runs costs what a
call outside a loop does, unless the thread has a future waiting to be woken. `just ffi
bench` shows the in-loop call back at main's cost before
[#564](https://github.com/moq-dev/rust2/pull/564), and the wake tests #564 added still pass.

## Plan

Decided with the maintainer on 2026-11-03: planned from #564's follow-up. #564 resumes a
same-thread wake without the socket pair by looking up the running loop on every call made
while one runs, about +14 ns (104 to 118 ns). Most of those calls wake nothing. Skip the
lookup unless something on the thread is waiting; #564's PR body explains why remembering
only the thread that last ran the loop is unsafe (a stale loop can deadlock a blocking call).

moq's Python calls run inside a loop, so moq pays this cost. Public API: none.
