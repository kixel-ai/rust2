# [M] Debug of std's other iterators

## Goal

[rust2-ts](/ts/README.md) translates `Debug` of the std iterators that still have no
mapping, writing what std's does instead of being refused: `str`'s iterators other than
`Chars`, `Bytes`, `CharIndices`, and the escapes (`Split` and its forms, `Lines`,
`SplitWhitespace`, `Matches`, `MatchIndices`), a `BTreeSet`'s set operations, and each
`ExtractIf`. `iter::repeat`, `repeat_n`, and `once_with`, which are not translated at all
yet, translate with their `Debug`. Proven by differential tests against native Rust.

## Plan

The sources, `Option`'s, `Result`'s, and an array's iterators, a slice's parts, and a hash
set's operations landed on `iter_debug` in `src/lower/fmt.rs`; extend it the same way.

What is left writes private state the runtime does not keep in std's shape:

- `str`'s searcher iterators write their searcher's fields: a `CharSearcher`'s
  `finger` and `finger_back` in UTF-8 bytes, a `StrSearcher`'s `TwoWaySearcher` state
  (`crit_pos`, `period`, `byteset`, `memory`), and an empty needle's flags. The runtime
  searches in UTF-16 code units by a `Pattern`, so it would track std's state beside its
  own, or search as std does.
- A `BTreeSet`'s `Difference` and `Intersection` write which strategy std picked
  (`Stitch`, `Search`, `Iterate`, `Answer`) and its iterators, `Search` the whole other
  set; `Union` and `SymmetricDifference` their `MergeIterInner`'s peeked item. The runtime
  already picks std's strategy, so its state may map over.
- `Vec`'s, `LinkedList`'s, and a B-tree's `ExtractIf` write the item they peek at; a hash
  map's and set's write `ExtractIf { .. }` once their `extract_if` translates.
