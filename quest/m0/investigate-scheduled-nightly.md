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

- Series: [`scheduled Nightly`](https://github.com/kixel-ai/rust2/actions/runs/45863659128/attempts/1), in jobs.
- 2026-11-26: 7 jobs against a baseline of 0 jobs, +7 jobs, at [`b2f468c`](https://github.com/kixel-ai/rust2/commit/b2f468c2363c7dc5c01a2eb581d24f4f5e264295). Run 45863659128 attempt 1 failed, failing Newest nightly, Shapes in every language, Packages, Export moq-net to C++ (MSVC), Bench the bindings, Export to C under sanitizers and Miri, Export to Kotlin on Android.
