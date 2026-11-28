# [M] Verus proves kio's WithinCredit on the Rust, beside rust2-lean and Aeneas

## Goal

Verus proves, on kio's `lab/pure-machine` Rust at `8013885` with no extraction step, that one `send_credit::Session::step` from any session that is well formed and within its credit either is refused or keeps both properties, without a panic or overflow, for every `S` and `G`.
It also proves `warm_settle`'s `options_rank_below` with the statement [compare/Ranked.lean](/driver/lean/compare/Ranked.lean) proves.
`just driver verus-kio` reruns both proofs in kio's own `.#verus` shell at the pin, and [driver/lean/README.md](/driver/lean/README.md)'s kio comparison gains a table: rust2-lean, Aeneas, and Verus, each with the source changes it needed, proof lines per lemma, check time warm and cold, and toolchain size.

Non-goals: a Verus proof on moq-net's code, which [moq-dev/moq's quest/m5/verus-subscribe.md](https://github.com/moq-dev/moq/blob/main/quest/m5/verus-subscribe.md) plans; Verus in `just ci`; proving `WithinCredit` in Lean.

## Plan

Decided 2026-11-27 (maintainer, [kixel-ai/rust2#1061](https://github.com/kixel-ai/rust2/pull/1061)): ✅ prove `WithinCredit` over one step of `SendCredit` with Verus as a third reference point (rejected: no), first placed in kixel-ai/kio's quest tree.

Decided 2026-11-27 (headless `/quest-plan`, the maintainer's pre-approved recommendations, for review):

- ✅ This quest lives in rust2's m2 (rejected: moq-dev/moq, extending `quest/m4/send-credit-machine.md` to prove the ported machine; reopening kixel-ai/kio). kio was archived the same day it moved into moq as `rs/kio`, leaving `lab/pure-machine` as a read-only record. The measurement only compares if it runs on the source the Lean spike ran on. Proving moq-net's port instead would mix the port's changes into Verus's count and wait behind m4's release cut. rust2 already checks kio out at the pin in [compare/kio.sh](/driver/lean/compare/kio.sh). kixel-ai/kio#26 first proposed Verus as a reference point for rust2's verdict, and the result steers [lean-router.md](/quest/m2/lean-router.md)'s `Spec` tactic.
- ✅ Pin kio `8013885` (rejected: `073df54`). `lab/pure-machine` is identical at both, and `8013885` is the revision whose `nix develop .#verus` packages the Verus release that its `verify/` package pins.
- ✅ Run Verus from kio's own `.#verus` shell at the pin (rejected: packaging Verus in rust2's flake). This adds no toolchain to rust2, the same way the Aeneas numbers came from kio's own `.#prove` shell.
- ✅ Keep Verus's source changes as a patch in `driver/lean/compare/`, applied to the pinned checkout (rejected: vendoring `lab/pure-machine` into rust2). The patch's diff of the machine's own lines, apart from specs and proofs, is the "source changes Verus needed" count. Count it as kio#9's friction table counted Aeneas's rewrites, and the README counts it against rust2-lean's 0 and Aeneas's 5.
- ✅ Write the specs in Verus's attribute syntax, as kio's `src/log.rs` does, with a verify-only package depending on `vstd` (rejected: a `verus!` block around the machine). The file then stays plain Rust that kio's BFS, Kani, and rust2-lean still build, which is the point of proving on the Rust.
- ✅ The `SendCredit` statement is kio's Kani harness `keeps_invariant` (`src/send_credit/proofs.rs`), generalized from `Session<2, 3>` to every `S` and `G` (rejected: fixed at `Session<2, 3>`). A bounded shape is what Kani already proves, so generality is what Verus adds. `within_credit` alone is not inductive, so `well_formed` is in both the hypothesis and the conclusion, along with Kani's assumption that `arrivals` and `opened` are below `u8::MAX`.
- ✅ Also prove `options_rank_below` (rejected: `WithinCredit` only). Neither Lean proof covers `WithinCredit`, so only the same lemma gives proof lines comparable with rust2-lean's 141 and Aeneas's 116.
- ✅ The script runs on demand, like `lean-kio`, and never in `just ci` or Nightly (rejected: a Nightly row). The pin never moves, so a scheduled run would re-check a fixed result.
- ✅ The numbers live in the driver README (rejected: only in the PR body, where #1061 left the Lean numbers). Copy #1061's measured rows into the table so all three columns sit together.

Measure as #1061 did: on one host, with the same counter of non-blank lines outside comments, and the cold run including the shell's download. A construct Verus refuses gets rewritten in the patch and counted. Close the proof without `assume` and without a new `external_body`, as kio's Verus rules require. Name any trusted model of `std` that Verus needs, such as `Vec::push` on the outputs, in the table.

## Related

- [rust2-lean translates moq-net's production Router](/quest/m2/lean-router.md) - its `Spec` tactic targets the proof-line gap this quest measures against a third tool
