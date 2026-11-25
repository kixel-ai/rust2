# [M] Investigate cost/by-model/model:claude-opus-5-5 (USD), friction/split/category:waiting (h), and 1 more

## Goal

Kixel Quest measured what the Plan lists and filed it here on its own. For each finding:

1. Diagnose its cause, starting from the culprit's diff and sessions where it has them.
2. Have a second session, one that did not write the diagnosis, try to refute it, and diagnose again when it does.
3. Fix it, accept it with the reason recorded in this Plan, or improve on it further. A fix carries the measurement that shows it.

Then look past the finding: make the related improvements the diagnosis turns up where they fit here, and add the rest as follow-up quests.

## Plan

Each finding keeps the comment Kixel Quest finds it by, so a later run that sees it again adds its sighting here instead of filing it anew.

### `cost/by-model/model:claude-opus-5-5 (USD)`: bottleneck

<!-- kixel.quest finding {"series":"cost/by\u002dmodel/model:claude\u002dopus\u002d5\u002d5 (USD)","culprit":null} -->

- Series: [`cost/by-model/model:claude-opus-5-5 (USD)`](https://kixel.quest/factory#bottlenecks-1431364399), in USD.
- 2026-11-25: 5.71 USD against a baseline of 5.71 USD, +0 USD (+0%). The largest share of cost, 99.8% of its measure over the 30 days to 2026-11-25; too few measurements for a trend.

### `friction/split/category:waiting (h)`: bottleneck

<!-- kixel.quest finding {"series":"friction/split/category:waiting (h)","culprit":null} -->

- Series: [`friction/split/category:waiting (h)`](https://kixel.quest/factory#bottlenecks-1431364399), in h.
- 2026-11-25: 0.07462 h against a baseline of 0.07462 h, +0 h (+0%). The largest share of friction, 61.89% of its measure over the 30 days to 2026-11-25; too few measurements for a trend.

### `host/gate-cpu/command:just ci (h)`: bottleneck

<!-- kixel.quest finding {"series":"host/gate\u002dcpu/command:just ci (h)","culprit":null} -->

- Series: [`host/gate-cpu/command:just ci (h)`](https://kixel.quest/factory#bottlenecks-1431364399), in h.
- 2026-11-25: 0.2084 h against a baseline of 0.2084 h, +0 h (+0%). The largest share of host time, 36.54% of its measure over the 30 days to 2026-11-25; too few measurements for a trend.
