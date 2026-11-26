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
