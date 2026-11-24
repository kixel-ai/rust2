# [M] Investigate job CI / CI / Build, job CI / CI / ffi, and 1 more

## Goal

Kixel Quest measured what the Plan lists and filed it here on its own. For each finding:

1. Diagnose its cause, starting from the culprit's diff and sessions where it has them.
2. Have a second session, one that did not write the diagnosis, try to refute it, and diagnose again when it does.
3. Fix it, accept it with the reason recorded in this Plan, or improve on it further. A fix carries the measurement that shows it.

Then look past the finding: make the related improvements the diagnosis turns up where they fit here, and add the rest as follow-up quests.

## Plan

Each finding keeps the comment Kixel Quest finds it by, so a later run that sees it again adds its sighting here instead of filing it anew.

### `job CI / CI / Build`: flake

<!-- kixel.quest finding {"series":"job CI / CI / Build","culprit":null} -->

- Series: [`job CI / CI / Build`](https://github.com/kixel-ai/rust2/actions/runs/45505016245/attempts/1), in %.
- A flake: fix it at the root cause now, or disable it until it is fixed and record that here. Never retry it, wait it out, or loosen what it checks. This quest stays open until it is fixed and enabled again.
- 2026-11-24: 50 % against a baseline of 0 %, +50 %, at [`ec577fb`](https://github.com/kixel-ai/rust2/commit/ec577fba8a850be31e0b6a6c4293ea6a96372617). Failed in run 45505016245 attempt 1 and passed in run 45505016245 attempt 2 of CI, failing 1 of the 2 times it ran on its commit.

### `job CI / CI / ffi`: flake

<!-- kixel.quest finding {"series":"job CI / CI / ffi","culprit":null} -->

- Series: [`job CI / CI / ffi`](https://github.com/kixel-ai/rust2/actions/runs/45505016245/attempts/1), in %.
- A flake: fix it at the root cause now, or disable it until it is fixed and record that here. Never retry it, wait it out, or loosen what it checks. This quest stays open until it is fixed and enabled again.
- 2026-11-24: 50 % against a baseline of 0 %, +50 %, at [`ec577fb`](https://github.com/kixel-ai/rust2/commit/ec577fba8a850be31e0b6a6c4293ea6a96372617). Failed in run 45505016245 attempt 1 and passed in run 45505016245 attempt 2 of CI, failing 1 of the 2 times it ran on its commit.

### `job CI / CI / ts`: flake

<!-- kixel.quest finding {"series":"job CI / CI / ts","culprit":null} -->

- Series: [`job CI / CI / ts`](https://github.com/kixel-ai/rust2/actions/runs/45505016245/attempts/1), in %.
- A flake: fix it at the root cause now, or disable it until it is fixed and record that here. Never retry it, wait it out, or loosen what it checks. This quest stays open until it is fixed and enabled again.
- 2026-11-24: 50 % against a baseline of 0 %, +50 %, at [`ec577fb`](https://github.com/kixel-ai/rust2/commit/ec577fba8a850be31e0b6a6c4293ea6a96372617). Failed in run 45505016245 attempt 1 and passed in run 45505016245 attempt 2 of CI, failing 1 of the 2 times it ran on its commit.
