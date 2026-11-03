# [S] A B-tree's extract_if

## Goal

[rust2-ts](/ts/README.md) translates `BTreeMap::extract_if` and `BTreeSet::extract_if`,
which it refuses now: each entry in the range its closure returns true for, taken out as it
is read in key order, the rest kept once it drops, and `Debug` of their `ExtractIf`, which
writes the entry it reads next as `peek`. Proven by differential tests against native Rust.

## Plan

The runtime's B-tree keeps its keys in linked leaves, which a removal while a `BTreeIter`
reads them may rebalance; the iterator has to survive that as std's cursor does. A map's
closure takes `&mut V`, lent as a place where JS copies the value, as `retain`'s is. No
consumer calls them yet; they are planned here as part of std.
