# [XS] Confirm the queue's warm export

## Goal

The merge queue's `ffi` module serves the moq stand-in's crates from main's mbx bundle,
and bench/ffi/README.md's "The export's cache" records its time against the groups before.

## Plan

The cause, found 2026-11-26 with a throwaway `mbx explain --last` step in
[#1050](https://github.com/kixel-ai/rust2/pull/1050)'s `ffi` job: the export's own `cargo
check` never ran through mbx. ffi/justfile ran the exporter with `cargo run`, which hands it,
as `$CARGO`, the cargo mbx's shim chains to; the driver runs that cargo with
`RUSTC_WRAPPER` empty and counts on the shim to set its own, so no cache wrapped any crate of
the check, moq's included, and the bundle never held them. The 143 and 144 hits read before
were the exporter's own build. Under the shim, the recipes now run the exporter through a cargo
runner that unsets `$CARGO`, so its cargo is the shim on `PATH`; on desktop the check then
served all 332 crates from a warm store in 7.3 s, against 23.2 s without it.

Once a main push has saved a bundle with that fix, read the next merge groups' `ffi` job
logs (`gh api repos/kixel-ai/rust2/actions/jobs/<id>/logs`): the stand-in's export should
print its own `mbx[cache]` line with moq's crates hit, and moq-net's check, from its
`Checking` line to moq-congestion's, take well under the 7.1 s of group 45816910267. Record
the check-and-test step's minutes over at least 5 groups beside the 16.1 to 29.2, median
20.8, of the 23 groups on mbx 1.29.2, and its 16.5 in that group.
