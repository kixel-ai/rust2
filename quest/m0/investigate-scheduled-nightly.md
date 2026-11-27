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

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/46036149452/attempts/1), in jobs.
- 2026-11-27: 10 jobs against a baseline of 0 jobs, +10 jobs, at [`6148d50`](https://github.com/kixel-ai/rust2/commit/6148d502544699d2db19bc7fd51b7932a2216a4f). Run 46036149452 attempt 1 failed, failing Packages, Shapes in every language, Export moq-net to C++ (MSVC), Bench the bindings, Export to C under sanitizers and Miri, Stage moq's API (Windows), Newest nightly, Export to Kotlin on Android, Binding packages / moq stand-in / build (apple-darwin, macos-26, aarch64-apple-darwin), Binding packages / api fixture / build (apple-catalyst, macos-26, aarch64-apple-ios-macabi).
