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

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/45663277828/attempts/1), in jobs.
- 2026-11-25: 6 jobs against a baseline of 0 jobs, +6 jobs, at [`c33f92f`](https://github.com/kixel-ai/rust2/commit/c33f92f1fc7b1e23530ec0fb95d57f54d82a8b5d). Run 45663277828 attempt 1 failed, failing Packages, Shapes in every language, Export to Kotlin on Android, Newest nightly, Export moq-net to C++ (MSVC), Export to C under sanitizers and Miri.
