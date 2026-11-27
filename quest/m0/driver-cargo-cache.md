# [XS] Raise the driver's cargo cache bypass with mbx

## Goal

A consumer that runs a rust2 driver as a cargo subcommand under mbx's shim reads its check's
crates from the cache, or mbx documents why it cannot.

## Plan

The callers that run a `Wrap::Workspace` driver are fixed: ffi/justfile's `exporter`
([#1050](https://github.com/kixel-ai/rust2/pull/1050)) and test recipes, and ffi/release/package.py's
exporter run each run it without the `$CARGO` that `cargo run` and `cargo test` set, under mbx's
shim. ts and go check with `Wrap::All`, so mbx defers to their `RUSTC_WRAPPER` and never caches
them, `$CARGO` or not; caching them means moving them to `Wrap::Workspace`, which is not
planned. driver/justfile's tests start no driver through `$CARGO`.

What remains is option (c), decided 2026-11-26 (maintainer, /quest-run): raise it in
[jdx/mr-boxington#1015](https://github.com/jdx/mr-boxington/discussions/1015)'s discussion. A
driver started as a cargo subcommand (`cargo rust2-...`) gets the same `$CARGO` and probably
skips the cache too; no caller of ours can unset it there, so only mbx can fix it. That is
unverified. The comment's draft and evidence are in the PR that rewrote this quest.

## Waiting

- Maintainer: post the drafted comment in jdx/mr-boxington#1015's discussion, which is outside
  our orgs (mbx's CONTRIBUTING takes proposals in Discussions and allows AI help if disclosed).
  Check: the comment exists. Then delete this quest, or rewrite it to mbx's answer.
