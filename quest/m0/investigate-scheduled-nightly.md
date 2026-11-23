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

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/44915362458/attempts/1), in jobs.
- 2026-11-23: 5 jobs against a baseline of 0 jobs, +5 jobs, at [`ef21a6d`](https://github.com/kixel-ai/rust2/commit/ef21a6da0b105f1c6da8a6074257c6b6f4be0b6f). Run 44915362458 attempt 1 failed, failing Newest nightly, Export moq-net to C++ (MSVC), TypeScript package, Shapes in every language, Gate.
