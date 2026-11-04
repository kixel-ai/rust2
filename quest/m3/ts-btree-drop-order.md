# [S] A B-tree drops as std's does

## Goal

[rust2-ts](/ts/README.md)'s `BTreeMap` and `BTreeSet` drop their keys and values in
std's order, node by node, proven by differential tests against native Rust that log
each drop.

## Plan

The runtime drops in key order, decided with the maintainer on 2026-10-30 for speed
while its tree was not std's. With std's node layout from
[the searches](/quest/m3/ts-btree-search-order.md), std's order costs little; measure it on
the TS bench's `btree/` workloads.

Public API: the translator's output.

## Required

- [A B-tree's searches call Ord as std's do](/quest/m3/ts-btree-search-order.md) - std's node layout and search
