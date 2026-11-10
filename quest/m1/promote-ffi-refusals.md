# [XS] Promote the refusals epic out of ffi-export

## Goal

`quest/m1/ffi-export/refusals/` is its own epic at `quest/m1/ffi-refusals/`,
so no epic in this tree holds another. The exporter's README names it as a
blocker instead of a child.

## Plan

Decided 2026-11-09 (user, /quest-plan in
[moq-dev/quest#594](https://github.com/moq-dev/quest/pull/594)): epics no
longer nest, since a nested epic means its parent is too large, and every
epic gets a branch its quests land on. ✅ Promote each nested epic to a
sibling in its milestone (rejected: merge its quests into the parent).

`quest mv` refuses to move a quest with an open PR, since the PR's branch is
named after the quest's path, so this waits for them. Start no new quest
under the old path first. Then run
`quest mv quest/m1/ffi-export/refusals quest/m1/ffi-refusals`.

## Waiting

- moq-dev/rust2: the open PRs under `quest/m1/ffi-export/refusals/` land or
  close ([#683](https://github.com/moq-dev/rust2/pull/683),
  [#679](https://github.com/moq-dev/rust2/pull/679),
  [#678](https://github.com/moq-dev/rust2/pull/678),
  [#676](https://github.com/moq-dev/rust2/pull/676),
  [#675](https://github.com/moq-dev/rust2/pull/675) and
  [#670](https://github.com/moq-dev/rust2/pull/670) on 2026-11-09). Check
  with `gh pr list -R moq-dev/rust2 --search "head:quest/m1/ffi-export/refusals/"`
  printing nothing.
