# [L] Generate the differential tests from the transcript

## Goal

The differential part of each language's `ffi/tests/api.*`, the script whose output must
match `examples/transcript.rs` line for line, is generated for all eight languages from
the transcript written once as data. What stays hand-written is only what tests a
language's idiom or the protocol: futures and cancel, close and GC, thread checks, and
each broken runtime check. The PR reports the hand-written lines before and after.

## Plan

[The epic](/quest/m1/ffi-rules/README.md) holds the measurements and the decision.

Recommended: a small step language (call, field read, expected value) the shape tests'
driver generator can reuse, rather than a second generator per language; the generated
drivers are written at test time, not committed.

Public API: none.
