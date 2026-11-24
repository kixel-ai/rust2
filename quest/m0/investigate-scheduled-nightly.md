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

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/45453172701/attempts/1), in jobs.
- 2026-11-24: 6 jobs against a baseline of 0 jobs, +6 jobs, at [`9423091`](https://github.com/kixel-ai/rust2/commit/94230918ff0a434caf353ca19d47a6f29d933e36). Run 45453172701 attempt 1 failed, failing Shapes in every language, Export to C under sanitizers and Miri, Export moq-net to C++ (MSVC), Export to Kotlin on Android, Packages, Newest nightly.
