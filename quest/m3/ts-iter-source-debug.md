# [M] Debug of a str's searches

## Goal

[rust2-ts](/ts/README.md) translates `Debug` of `str`'s searching iterators, which it
refuses now: `Split` and its forms, `Matches`, `MatchIndices`, `Lines`, and
`SplitWhitespace`, writing what std's does fresh, `{:#?}`, and after reads from either end.
Proven by differential tests against native Rust.

## Plan

std writes each searcher's private state, which the runtime does not keep in std's shape:
a `CharSearcher`'s `finger` and `finger_back` in UTF-8 bytes, a `StrSearcher`'s
`TwoWaySearcher` (`crit_pos`, `period`, `byteset`, `position`, `memory`) or an empty
needle's flags, and a closure's or a char set's `CharIndices`. The runtime searches in
UTF-16 code units by a `Pattern`, so it would track std's state beside its own, replay
std's search when written, or search as std does; measure the cost to plain searches
before choosing. `Lines` and `SplitWhitespace` wrap a `SplitInclusive` and a `Split`, so
they follow from those.

`SplitAsciiWhitespace` has no mapping at all yet, and std writes it as a slice's `Split` of
the bytes left.
