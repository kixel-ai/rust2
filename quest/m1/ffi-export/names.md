# [M] No name clash fails an export

## Goal

Every [rust2-ffi](/ffi/README.md) generator names every exported item by a fixed rule
that cannot collide, so no export fails telling the crate's author to "rename one in the
source or exclude it in the config". A fixture whose items clash in each language today
(two types whose segments differ only by case, a member named as one the language gives
its type, a Dart name the runtime declares) exports and is called from each.

## Plan

Found in the 2026-11-09 quest audit: these errors are raised by the generators, outside
the census, so "the census lists no refusals" does not cover them, and once
[the refusals epic](/quest/m1/ffi-export/refusals/README.md) deletes `ffi.toml` their
advice names a config that no longer exists. The sites: `kotlin.rs` (two types whose
names match by case), `python.rs`, `cpp.rs`, `dart.rs`, `go.rs`, `swift.rs`, the C names'
own registry (`audio_encoder::finish` against `audio::encoder_finish`), and
`ffi/README.md`'s Dart section. AGENTS.md calls each source change a consumer needs a gap
to close.

Every generator runs on every export, so one language's clash fails all of them: a
crate that never ships Dart cannot export because of a Dart name.

- Extend the rule the generators already share for two types whose names match: a `_`
  where their paths split, one more segment at a time until they differ, and a trailing
  `_` against a name the language or runtime owns.
- A name depends on every item exported, so an upstream addition can lengthen another's
  name, as generic instances' already can. Keep that rule rather than add stable mangling.
- Open: the C names, where joining segments with `_` loses where they split. Recommended:
  the same one-more-segment rule with a doubled `_` at the split that differs, so
  `moq_audio_encoder_finish` stays for the item whose path is shorter.

Public API: the names of items that clash today, which fail to export now.
