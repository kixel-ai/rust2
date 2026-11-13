# Dead public items: preliminary driver experiment

Status: incomplete spike, 2026-11-13. No deletion or visibility change is proposed.
The full rust2/moq inventory and the go/no-go comparison remain in
[the quest](/quest/m3/dead-pub.md).

The proposed subtraction of resolved uses from public definitions is insufficient.
A throwaway driver using the pinned nightly's HIR paths and type-dependent method
resolution misses a required implementation reached by generic trait dispatch.
This is evidence against that collector, not against the feasibility of a fuller
compiler analysis. It does not establish that narrow-and-retry yields the same list.

## Reproduction

Toolchain: rustc 1.101.0-nightly, nightly-2026-10-15, with rustc-dev.
The experiment used `quest gate` for compilation and execution. Save the driver
below as `probe.rs`, compile it with that nightly, then run:

```sh
rustc probe.rs -o probe -L "$(rustc --print sysroot)/lib" -C rpath=yes
./probe dispatch.rs -o dispatch
./dispatch
```

The source is:

```rust
pub trait Action { fn run(&self) -> u32; }
pub struct Live;
impl Action for Live { fn run(&self) -> u32 { 42 } }
pub fn invoke<T: Action>(value: T) -> u32 { value.run() }
pub fn crate_only() -> u32 { 7 }
pub fn dead() -> u32 { 9 }
#[cfg(test)] mod tests { #[test] fn uses() { assert_eq!(super::dead(), 9); } }
fn main() { assert_eq!(invoke(Live), 42); assert_eq!(crate_only(), 7); }
```

The unmodified executable succeeds, including the assertion that dispatch returns
42. The probe emits `PUBLIC dispatch::<Live as Action>::run`, but its `USE` lines
include only `dispatch::Action::run` for the method call. There is no `USE` for
`dispatch::<Live as Action>::run`. The flat subtraction therefore misclassifies
that implementation as unused.

Independently replacing the entire `impl Action for Live` body with `{}` and
compiling with ordinary rustc fails with:

```text
error[E0046]: not all trait items implemented, missing: `run`
```

This matters even before mutation testing behavior: the suggested deletion fails
type checking. Restricting the inventory to syntactically explicit `pub` skips
trait implementation methods but does not account for their implicit references.
The collector below deliberately uses compiler visibility so this gap is visible.

The fixture's `dead` function also shows why test and production graphs must stay
separate: it has no production reference and a test reference. That classification
has not yet been validated by running a test-mode inventory.

## Coverage and next experiment

The collector enumerates public functions, associated functions, structs, enums,
traits, type aliases, constants, statics, and modules. It traverses resolved HIR
paths and resolved method-call expressions after analysis. It does not inventory
every kind of public item or claim completeness for these kinds.

Missing coverage includes fields, variants, macros and their invocation provenance,
type-relative paths outside method calls, trait obligations and implementation
selection, implicit drop calls, and a root-based liveness graph. Counting a path in
an otherwise dead body as a live use also hides dead subsystems. A presentation path
is not a cross-crate identity: a real collector must join crate-disambiguated DefIds,
including re-exports, rather than these display strings.

Recommendation: revise the collector before building a production tool. First
validate obligation and implicit-use edges with compiler mutation fixtures. Then
collect the full corpus, retain Cargo target and feature identities for each
invocation, separate test roots, and validate all suggestions together with the
same targets/features. Separate published API from workspace-only findings.
Neither an absent reference in this probe nor a successful host build proves an
item is unused under other configurations.

## Prior art

[Deadwood 1.0.1](https://docs.rs/crate/deadwood-rs/1.0.1), released November 9,
adds public-item and test-only analysis to the prior art in the quest. Its own
limitations say resolution is syntactic and excludes macro expansion, method calls
and trait dispatch. It therefore does not replace the requested compiler-resolved
experiment. [cargo-workspace-unused-pub](https://github.com/cpg314/cargo-workspace-unused-pub)
uses SCIP and focuses on methods. Neither source establishes compiler-level
coverage for this quest.

## Corpus run

Inputs: rust2 `a37b3951` (the claim adds no code) and a clean moq checkout at
`52fa6b2f0ece29c44bfa2459ec93fc54b4b48221`. All commands use the pinned nightly,
explicit `RUSTC` and `RUSTDOC`, a workspace wrapper, `--locked --all-targets`,
and a separate scratch Cargo target directory. moq additionally uses `--workspace`.
These are host Cargo targets with default features, not a feature or platform matrix.
No source mutations were applied to either repository.

The four rust2 workspace checks passed. The following are cardinalities of the
prototype's display-string sets, including generated definitions and harness roots,
not counts of safe changes or an inventory of Rust identities:

| Workspace | Public definition strings | Reference strings | Definition strings absent from references |
| --- | ---: | ---: | ---: |
| driver | 85 | 468 | 44 |
| ffi | 694 | 2648 | 529 |
| ts | 1099 | 4068 | 340 |
| go | 152 | 1014 | 47 |

The apparent candidates include
`rust2_go::<driver::Translate as rustc_driver::Callbacks>::after_expansion`,
`runtime::<Logged<'_, T> as std::ops::Drop>::drop`, derived comparison methods,
and generated serde visitors. These are examples of missing implicit edges, not
recommended deletions. This experiment cannot determine the full safe set or
compare that set with narrow-and-retry.

An initial direct Cargo invocation selected stable Rust; its artifacts were excluded
from the experiment by selecting a new scratch directory after explicitly selecting
the nightly. That setup failure supplies no evidence about source compatibility.

The moq run did not complete. An initial probe query used an item's owner rather
than the enclosing body owner for an anonymous constant and caused an ICE at
`moq-sock`'s `demux::Counters`; the source below corrects that query. The resumed
run then stopped because pkg-config could not find GStreamer, GLib and GObject in
the rust2 shell. A command using moq's own development shell was queued, then
cancelled before completion when the experiment was stopped in favor of revising
the collector. There is no complete moq inventory or source-compatibility verdict.

## Throwaway collector

This is the diagnostic prototype, not a supported tool or a safe deletion list.
The Cargo guard limits output to selected workspace packages; direct rustc
invocations, including the fixture above, are always inspected. Cargo wrapper use
requires `RUSTC` to name the matching nightly explicitly.

```rust
#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Path, HirId, Expr, ExprKind};
use rustc_hir::def::{Res, DefKind};
struct Uses<'tcx> { tcx: TyCtxt<'tcx> }
impl<'tcx> Visitor<'tcx> for Uses<'tcx> {
 type NestedFilter = rustc_middle::hir::nested_filter::All;
 fn maybe_tcx(&mut self) -> TyCtxt<'tcx> { self.tcx }
 fn visit_path(&mut self, path: &Path<'tcx>, id: HirId) {
  if let Res::Def(_, def) = path.res { eprintln!("USE\t{}", name(self.tcx, def)); }
  intravisit::walk_path(self, path);
  let _ = id;
 }
 fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
  if matches!(expr.kind, ExprKind::MethodCall(..)) {
   if let Some(def) = self.tcx.typeck(self.tcx.hir_enclosing_body_owner(expr.hir_id)).type_dependent_def_id(expr.hir_id) {
    eprintln!("USE\t{}", name(self.tcx, def));
   }
  }
  intravisit::walk_expr(self, expr);
 }
}
fn name(tcx: TyCtxt<'_>, def: rustc_hir::def_id::DefId) -> String { let krate = tcx.crate_name(def.krate).to_string(); let path = tcx.def_path_str(def); format!("{}::{}", krate, path.strip_prefix(&format!("{krate}::")).unwrap_or(&path)) }
struct Probe;
impl Callbacks for Probe {
 fn after_analysis<'tcx>(&mut self, _: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
  if std::env::var_os("CARGO_MANIFEST_DIR").is_some() && std::env::var_os("CARGO_PRIMARY_PACKAGE").is_none() { return Compilation::Continue; }
  eprintln!("MODE\t{}\t{}", tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE), tcx.sess.opts.test);
  for id in tcx.iter_local_def_id() {
   if matches!(tcx.def_kind(id), DefKind::Fn | DefKind::AssocFn | DefKind::Struct | DefKind::Enum | DefKind::Trait | DefKind::TyAlias | DefKind::Const | DefKind::Static { .. } | DefKind::Mod) && tcx.visibility(id).is_public() {
    eprintln!("PUBLIC\t{}\t{:?}\t{:?}", name(tcx, id.to_def_id()), tcx.def_kind(id), tcx.def_span(id));
   }
  }
  tcx.hir_walk_toplevel_module(&mut Uses { tcx });
  Compilation::Continue
 }
}
fn main() {
 let mut args: Vec<String> = std::env::args().collect();
 if args.get(1).is_some_and(|s| s.ends_with("rustc")) { args.remove(1); }
 rustc_driver::compiler_entrypoint(&args, &mut Probe);
}
```
