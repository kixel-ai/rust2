# [M] The rest of str's searches

## Goal

[rust2-ts](/ts/README.md) translates `str`'s pattern searches and text iterators still
refused, each by every pattern std takes, searching in std's order from each end, proven by
differential tests against native Rust.

## Plan

Found translating the [std calls moq's main added to the model](https://github.com/moq-dev/rust2/pull/434),
by a census of a crate calling each by a `char`. These refuse as unsupported calls:

- `find`, `strip_prefix`, `strip_suffix`, `trim_matches`, and `replace`;
- the iterators `rsplit`, `rsplit_terminator`, `splitn`, `rsplitn`, `split_inclusive`,
  `matches`, `rmatches`, and `rmatch_indices`;
- `char_indices`, `lines`, and `split_whitespace`.

The runtime's `Searcher` in `rt/str.ts` already searches from both ends and `Split` holds
std's `allow_trailing_empty`, so most map onto them as `split`, `split_terminator`, and
`rsplit_once` do. A predicate's calls are visible, so each must call it on the chars std's
does, in order. No consumer calls them yet; they are planned here as part of std.

Public API: the translator's output.
