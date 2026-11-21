# [M] C: no name the C header declares is one C or C++ reserves

## Goal

The C header declares no name C reserves (one starting with `__` or `_` and a capital) or
C++ reserves (one holding `__` anywhere), since every C++ program includes it, and neither
does the C++ header where it builds a name from a C one. The api fixture's strict C++ build
then fails on `-Wreserved-identifier` like any other warning, and `RESERVED_FROM_C` in
`ffi/tests/fixtures.rs` is gone.

## Plan

Found by [cpp-reserved-names](https://github.com/kixel-ai/rust2/pull/929), which turned the
warning on and cleared every reserved name the C++ generator makes of Rust's. The rest come
from C's naming, which appends `_` to resolve a clash and joins with `_`, so a clash makes
`__` (the api fixture's names are listed in `RESERVED_FROM_C`):

- A type renamed past another's name, joined to a member: the fixture's `Status`, apart
  from the runtime's status as `api_Status_`, gives `api_Status__eq`, `api_Status__debug`,
  and `API_STATUS__MOVED`.
- A function whose Rust name starts with `_`: `api__deprecated`, `api___all__`.
- A parameter's or callback member's escape and the names made of it: `new__`, `by__`,
  `by___data`, `work__`, `work___data`, `delete__`, `out__out`, and in C++ the local
  `c_out__out_` made of C's out parameter.
- A macro's family, which `abi::escape` keeps apart by appending more `_`s
  (`abi::unmacroed`): `EOF__` is `EOF___`.
- A Rust name C itself reserves, kept reserved by appending `_`: moq's
  `moq_mux_container_ts_Mpegts_deserialize` takes serde's `__deserializer` as
  `__deserializer_`.

Recommendation: give C the rule C++ now has in `ffi/src/cpp.rs` (`escape`, `stepped`): a
reserved name drops its leading `_`s and keeps one `_` per run, and a name that moves,
past a clash or a macro, steps along its stem's names `x`, `x_`, `x_2`, `x_3`, never making
`__`; and have each join of a name ending in `_` (`{type}_{member}`,
`{name}_data`, C++'s `c_{name}_`, `{name}_debug`, `{name}_op`) keep apart without `__`.
This changes C symbols only where a clash or a reserved Rust name made them, none of moq's.

## Waiting

- Maintainer: whether C symbols that hold `__` today, such as `api_Status__eq`, may be
  renamed, a break for a C caller of a clashing name (recommended: yes, moq's symbols hold
  none; the alternative leaves `-Wreserved-identifier` off for the C header in C++)
