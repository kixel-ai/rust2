# [S] C++ names that escape alike stay apart

## Goal

Two Rust names in one C++ scope that escape to the same C++ name, such as the enum
variants `Unix` and `Unix_` (both `unix_`), or `Delete` and `Delete_` (both `delete_`),
export to distinct names or fail the export with a span at the second item, never a
header with a duplicate member that fails at the consumer's compile. This covers enum
values, record fields, methods, and parameters.

## Plan

Found by the review of [kixel-ai/rust2#873](https://github.com/kixel-ai/rust2/pull/873),
which added `unix`, `linux`, and `i386` to `cpp::escape`'s list, since GNU dialects
predefine them as macros. The collision was already possible for keywords.

`cpp::escape` cannot simply append another `_` to a name that already ends in one, as
`abi::escape` does for C: it also escapes C names, which C's clash rule keeps unique and
which may end in `_` (`audio_encoder_finish_`), and every Rust name ending in `_`, such as
a field `type_`, would change in C++. Recommended: register each scope's escaped names as
the C++ generator already does for a member named as its class, and give the later
collision a trailing `_`. Refuse with a span only where that rule cannot apply.

Public API: C++ names of the colliding items only.
