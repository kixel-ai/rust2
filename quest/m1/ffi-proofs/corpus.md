# [M] Export a corpus of stock crates nightly

## Goal

A nightly job censuses and exports a pinned corpus of popular crates.io crates through a
facade each. A panic in rust2-ffi, an error with no span, or a binding of a crate with no
refusals that does not compile fails the run. The two crashes reproduced on 2026-11-18
are fixed: once_cell's `race::OnceBool::set -> Result<(), ()>` hits `unreachable!("Unit is
never returned")` at `ffi/src/cpp.rs:489`, and rustc-hash fails with no span ("two members
of FxHasher are both FxHasher::default_ in C++").

## Plan

Found by the 2026-11-18 language audit. [#762](https://github.com/kixel-ai/rust2/pull/762)
censused the top 300 crates on 2026-11-13 and only its scope note landed: 68 crates that
build natively failed in the exporter (44 on an `owned_box` lang-item error, 23 on an
ambiguous package selector, 1 on a missed instance in futures-util), and 3 with no
refusals failed to generate. Both break AGENTS.md's rule that an unsupported construct is
an error with a span.

Recommended: start from #762's corpus and harness, pinned by version so the nightly is
reproducible, and triage each failure class into a fix in this PR or a refusal with a
span. A class too large for this PR becomes a follow-up quest. Keep the job out of the
merge gate.

Public API: none, beyond the fixes.
