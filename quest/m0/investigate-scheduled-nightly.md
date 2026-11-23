# [M] Investigate scheduled Nightly

## Goal

Kixel Quest measured what the Plan lists and filed it here on its own. For each finding:

1. Diagnose its cause, starting from the culprit's diff and sessions where it has them.
2. Have a second session, one that did not write the diagnosis, try to refute it, and diagnose again when it does.
3. Fix it, accept it with the reason recorded in this Plan, or improve on it further. A fix carries the measurement that shows it.

Then look past the finding: make the related improvements the diagnosis turns up where they fit here, and add the rest as follow-up quests.

## Plan

Each finding keeps the comment Kixel Quest finds it by, so a later run that sees it again adds its sighting here instead of filing it anew.

### `scheduled Nightly`: failure

<!-- kixel.quest finding {"series":"scheduled Nightly","culprit":null} -->

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/44764178649/attempts/1), in jobs.
- 2026-11-23: 11 jobs against a baseline of 0 jobs, +11 jobs, at [`31c21f8`](https://github.com/kixel-ai/rust2/commit/31c21f85a99afc8a9cfcab909c93b7adec8164a9). Run 44764178649 attempt 1 failed, failing Export to C under sanitizers and Miri, TypeScript package, Export moq-net to C++ (Apple Clang), Shapes in every language, Newest nightly, Gate, Export moq-net to C++ (MSVC), Binding packages / api fixture / web-consumer, Binding packages / api fixture / build (apple-ios, macos-26, aarch64-apple-ios), Binding packages / api fixture / build (linux-x64, ubuntu-24.04, x86\_64-unknown-linux-gnu), and 1 more.
