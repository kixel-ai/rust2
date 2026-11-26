# [XS] Confirm the queue's warm export

## Goal

The merge queue's `ffi` module serves the moq stand-in's crates from main's mbx bundle,
and bench/ffi/README.md's "The export's cache" records its time against the groups before.

## Plan

`just ffi standin` named its moq checkout through `realpath`, and CI's `ffi/target` is a link
into a store mbx names afresh in every run, so cargo hashed moq's crates, path dependencies
outside the stand-in's workspace, by a new path in every merge group, and mbx never served
them: the export checked moq-net for 12.5 s in every group, before mbx 1.31 and after. It
now names the checkout as spelled, `/home/runner/work/rust2/rust2/ffi/.work/moq` in every
run; on **desktop** a store filled through a symlink whose target then moved served the
whole export, 332 hits.

Once a main push has saved a bundle with the stand-in at that path, read the next merge
groups' `ffi` job logs (`gh api repos/kixel-ai/rust2/actions/jobs/<id>/logs`): moq-net's
check should take well under a second, and the stand-in's export and `moq-c`'s glue build
less than the 49 and 97 s recorded. Record the check-and-test step's minutes over at least 5
groups beside the 16.1 to 29.2, median 20.8, of the 23 groups on mbx 1.29.2. If moq's
crates still compile, `mbx explain --last` in a rerun of the job, or `MBX_BYPASS_LOG`, says
why.

Read 2026-11-26: the first merge group to read a bundle saved with the stand-in at that path
is 45816910267 (the `ffi` job 139899005997, bundle from the push of `7b94c699`); the
groups before it read bundles from before #1039, and every other group since skipped
`ffi`. In it moq's crates still compiled: moq-net's check, from its `Checking` line to
moq-congestion's, took 7.1 s (8.4 s before the fix), the stand-in's export ran 12:12:25 to
12:13:28 with 144 hits (143 before), and the check and test step took 16.5 minutes, within
the 16.1 to 29.2 of mbx 1.29.2. Every bundle read that day (04:07, 04:38, 05:32, 12:09 UTC) and the one the push of
`7b94c699` saved hold the same 956 actions and 5469 objects (3.1 GiB), though the push of `7b94c699` compiled moq
afresh at the new path, and its job reports only 4.8 MiB stored: the export may be saving
none of what that run compiled. Next: run `mbx explain --last` from
`ffi/.work/moq-ffi` after the stand-in's export in a pull request's `ffi` job (it reads
main's bundle), or `MBX_BYPASS_LOG`, to see why, and file it with mbx if it is the export;
then read at least 5 groups that read such a bundle.
