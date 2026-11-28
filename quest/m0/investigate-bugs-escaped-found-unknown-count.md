# [M] Investigate bugs/escaped/found:unknown (count) and ship/pickup (h)

## Goal

Kixel Quest measured what the Plan lists and filed it here on its own. For each finding:

1. Diagnose its cause, starting from the culprit's diff and sessions where it has them.
2. Have a second session, one that did not write the diagnosis, try to refute it, and diagnose again when it does.
3. Fix it, accept it with the reason recorded in this Plan, or improve on it further. A fix carries the measurement that shows it.

Then look past the finding: make the related improvements the diagnosis turns up where they fit here, and add the rest as follow-up quests.

## Plan

Each finding keeps the comment Kixel Quest finds it by, so a later run that sees it again adds its sighting here instead of filing it anew.

### `bugs/escaped/found:unknown (count)`: bottleneck

<!-- kixel.quest finding {"series":"bugs/escaped/found:unknown (count)","culprit":null} -->

- Series: [`bugs/escaped/found:unknown (count)`](https://kixel.quest/factory#bottlenecks-1431364399).
- 2026-11-28: 0 against a baseline of 0, +0. The largest share of escaped bugs, 35.29% of its measure over the 30 days to 2026-11-28; its last 7 days ran no different than the 30 before, median 0 over 7 days against 0 over 30, one-sided Mann-Whitney U, p = 0.004505.

### `ship/pickup (h)`: bottleneck

<!-- kixel.quest finding {"series":"ship/pickup (h)","culprit":null} -->

- Series: [`ship/pickup (h)`](https://kixel.quest/factory#bottlenecks-1431364399), in h.
- 2026-11-28: 2.628 h against a baseline of 2.628 h, +0 h (+0%). The largest share of time to ship, 33.94% of its phases over the 30 days to 2026-11-28; too few measurements for a trend.
