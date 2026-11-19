# [XS] Analyze the generated Dart package

## Goal

`dart analyze --fatal-infos` runs on the generated Dart library itself, for the `api`
fixture and moq, with no file-wide `ignore_for_file: type=lint`, and passes, as pub.dev
scores `lib/` with the analyzer.

## Plan

Found by the 2026-11-18 language audit: `just dart-program` (`ffi/justfile:394`)
analyzes only the test program's directory, and the generated file opens with
`ignore_for_file: type=lint` (`ffi/src/dart.rs:117`), so its lints never surface.

Decided with the maintainer on 2026-11-18: ✅ in this epic, since fixing lints can rename
or retype generated API (rejected: after the pin).

Public API: whatever renames the analyzer's fixes need, if any.

## Required

- [API surface lock](/quest/m1/ffi-export/api-surface.md) - its CI check shows this PR's break in every language's rendered API
