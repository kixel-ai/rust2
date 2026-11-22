# [S] Export with an optimized rust2-ffi in the merge queue

## Goal

The merge queue's `ffi` module exports moq's stand-in with a rust2-ffi built with
optimizations, so the facade's own check, which no cache serves, takes seconds rather than
minutes, and the queue's median merge group is shorter for it.

## Plan

Measured 2026-11-22 on **desktop** at a load of about 19 (`bench/ffi/README.md`'s "The
export's cache"): with every other crate served from mbx's cache, the stand-in's native
export took 132 to 142 s by the dev build that `cargo run` makes, nearly all of it the
facade's check on one thread, and 19 to 32 s by a release build. The queue exports twice
(native and web), and the binding tests export more.

Measure both sides before choosing: what building rust2-ffi optimized adds to the queue
when rust2-ffi changed, which mbx cannot serve, against the time each export saves.
Recommended: `opt-level = 1` for the ffi workspace's own package in its dev profile,
which keeps `cargo run` and `cargo test` on one build; else `--release` in the recipes
that export.
