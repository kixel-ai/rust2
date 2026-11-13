//! The mono budget: what a generic's copies cost the crate's codegen.
//!
//! A generic costs the optimized MIR statements of every copy the crate instantiates,
//! as rustc's own size estimate counts them, with its closures and async bodies: each is
//! part of the copy of the function that defines it. A generic fails when that total
//! exceeds its budget, unless every copy is thin enough to only forward.

use rustc_ast::LitKind;
use rustc_attr_ir::Attribute;
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_hir::def::DefKind;
use rustc_middle::mono::{CollectionMode, MonoItem};
use rustc_middle::ty::{GenericArgsRef, Instance, InstanceKind, TyCtxt};
use rustc_span::def_id::DefId;
use rustc_span::{FileName, Span, Symbol};

use std::path::PathBuf;

/// The statements a copy may cost and still only forward: pin a future and hand it to a
/// non-generic body, as an executor's entry point does in about 60. A generic whose every
/// copy is this thin passes whatever they total, since the only thinner fix is fewer callers.
pub const THIN: u64 = 100;

/// How many callers an error names.
const CALLERS: usize = 3;

/// The attribute that sets one generic's budget.
const ATTRIBUTE: [&str; 2] = ["rust2", "mono_budget"];

/// What one generic's copies cost.
#[derive(Default)]
struct Cost<'tcx> {
	/// The MIR statements of each copy, its closures' included, by the generic's arguments:
	/// a closure's own arguments start with them.
	copies: FxHashMap<GenericArgsRef<'tcx>, u64>,
}

impl Cost<'_> {
	fn statements(&self) -> u64 {
		self.copies.values().sum()
	}

	fn largest(&self) -> u64 {
		self.copies.values().copied().max().unwrap_or(0)
	}
}

/// The share of the crate's codegen, in percent, past which `--report` names a generic.
pub const SHARE: u64 = 1;

/// What the lint does with the generics' costs.
#[derive(Clone, Copy)]
pub enum Mode {
	/// Fail on every generic over its budget, the crate-wide one unless it sets its own.
	Check(u64),
	/// Warn on every generic past [`SHARE`] of the crate's codegen, failing none.
	Report,
}

/// Fail on, or report, the generics the crate instantiates by what their copies cost.
/// `local` holds the directories of the packages a user can edit: a generic outside them,
/// from std or a dependency, is reported at the local function that instantiates it.
pub fn check(tcx: TyCtxt<'_>, mode: Mode, local: &[PathBuf]) {
	let instances = instances(tcx);
	let mut total = 0;
	let mut costs: FxHashMap<DefId, Cost<'_>> = FxHashMap::default();
	for &instance in &instances {
		let size = tcx.size_estimate(instance) as u64;
		total += size;
		let generic = tcx.typeck_root_def_id(instance.def_id());
		let generics = tcx.generics_of(generic);
		if !generics.requires_monomorphization(tcx) {
			continue;
		}
		let copy = instance.args.truncate_to(tcx, generics);
		*costs.entry(generic).or_default().copies.entry(copy).or_default() += size;
	}
	let mut over: Vec<(DefId, Cost<'_>, u64)> = costs
		.into_iter()
		.filter(|(_, cost)| cost.largest() > THIN)
		.filter_map(|(generic, cost)| {
			let limit = match mode {
				Mode::Report => total * SHARE / 100,
				Mode::Check(budget) => match limit(tcx, generic) {
					Some(Ok(limit)) => limit,
					Some(Err(())) => return None,
					None => budget,
				},
			};
			(cost.statements() > limit).then_some((generic, cost, limit))
		})
		.collect();
	if over.is_empty() {
		return;
	}
	over.sort_by_cached_key(|(generic, cost, _)| (std::cmp::Reverse(cost.statements()), tcx.def_path_str(*generic)));
	let callers = callers(tcx, &instances);
	let editable = |def: DefId| editable(tcx, def, local);
	for (generic, cost, limit) in over {
		let path = tcx.def_path_str(generic);
		let size = format!(
			"`{path}` costs {} MIR statements in {} copies of up to {} each",
			cost.statements(),
			cost.copies.len(),
			cost.largest(),
		);
		let (span, through) = match editable(generic) {
			true => (tcx.def_span(generic), None),
			false => match origin(&callers, generic, &editable) {
				Some((caller, span)) => (span, Some(caller)),
				None => (tcx.def_span(generic), None),
			},
		};
		let through = through.map(|caller| format!(", instantiated through `{}`", tcx.def_path_str(caller)));
		let through = through.unwrap_or_default();
		let mut err = match mode {
			Mode::Check(_) => tcx
				.dcx()
				.struct_span_err(span, format!("{size}{through}, over its mono budget of {limit}")),
			Mode::Report => tcx.dcx().struct_span_warn(
				span,
				format!(
					"{size}{through}: {}% of the crate's {total}",
					cost.statements() * 100 / total.max(1)
				),
			),
		};
		let mut direct: Vec<_> = callers.get(&generic).into_iter().flatten().collect();
		direct.sort_by_cached_key(|(caller, (copies, _))| (std::cmp::Reverse(*copies), tcx.def_path_str(**caller)));
		for (caller, (copies, span)) in direct.iter().take(CALLERS) {
			err.span_note(*span, format!("`{}` instantiates {copies}", tcx.def_path_str(**caller)));
		}
		if direct.len() > CALLERS {
			err.note(format!("and {} more callers", direct.len() - CALLERS));
		}
		if matches!(mode, Mode::Check(_)) {
			err.help(match editable(generic) {
				true => format!(
					"make it forward to a non-generic function, or give it its own budget with `#[cfg_attr(rust2, {}(limit = <statements>, reason = \"<why>\"))]`",
					ATTRIBUTE.join("::"),
				),
				false => "call it with fewer types, or through a non-generic function of your own".into(),
			});
		}
		err.emit();
	}
}

/// Whether `def` is defined in a package the user can edit.
fn editable(tcx: TyCtxt<'_>, def: DefId, local: &[PathBuf]) -> bool {
	if def.is_local() {
		return true;
	}
	match tcx.sess.source_map().span_to_filename(tcx.def_span(def)) {
		FileName::Real(name) => name
			.local_path()
			// Cargo names a workspace member's files from the workspace root, rustc's directory.
			.and_then(|path| std::path::absolute(path).ok())
			.is_some_and(|path| local.iter().any(|dir| path.starts_with(dir))),
		_ => false,
	}
}

/// The nearest editable function that instantiates `generic`, through any chain of
/// callers, and where it does: the place a user can change how often it is copied.
fn origin(
	callers: &FxHashMap<DefId, FxHashMap<DefId, (u64, Span)>>,
	generic: DefId,
	editable: &impl Fn(DefId) -> bool,
) -> Option<(DefId, Span)> {
	let mut seen = FxHashSet::from_iter([generic]);
	let mut next = vec![generic];
	while !next.is_empty() {
		let mut found: Option<(u64, DefId, Span)> = None;
		let mut up = Vec::new();
		for callee in std::mem::take(&mut next) {
			for (&caller, &(copies, span)) in callers.get(&callee).into_iter().flatten() {
				if editable(caller) {
					// The heaviest caller at the nearest depth, then the first by span.
					if found.is_none_or(|(most, _, at)| copies > most || (copies == most && span.lo() < at.lo())) {
						found = Some((copies, caller, span));
					}
				} else if seen.insert(caller) {
					up.push(caller);
				}
			}
		}
		if let Some((_, caller, span)) = found {
			return Some((caller, span));
		}
		next = up;
	}
	None
}

/// Every function body the crate codegens, once each, but the shims rustc makes.
fn instances(tcx: TyCtxt<'_>) -> FxHashSet<Instance<'_>> {
	tcx.collect_and_partition_mono_items(())
		.codegen_units
		.iter()
		.flat_map(|cgu| cgu.items().keys())
		.filter_map(|item| match *item {
			MonoItem::Fn(instance) if matches!(instance.def, InstanceKind::Item(_)) => Some(instance),
			_ => None,
		})
		.collect()
}

/// For each function the crate codegens, the functions outside it that instantiate it:
/// how many copies each, and where the first is.
fn callers<'tcx>(
	tcx: TyCtxt<'tcx>,
	instances: &FxHashSet<Instance<'tcx>>,
) -> FxHashMap<DefId, FxHashMap<DefId, (u64, Span)>> {
	let mut callers: FxHashMap<DefId, FxHashMap<DefId, (u64, Span)>> = FxHashMap::default();
	for &instance in instances {
		let caller = tcx.typeck_root_def_id(instance.def_id());
		// Collection already ran this query for every instance, so it is cached.
		let Ok((used, _)) = tcx.items_of_instance((instance, CollectionMode::UsedItems)) else {
			continue;
		};
		for item in used {
			let MonoItem::Fn(callee) = item.node else { continue };
			let generic = callee.def_id();
			if generic == caller {
				continue;
			}
			let (copies, _) = callers
				.entry(generic)
				.or_default()
				.entry(caller)
				.or_insert((0, item.span));
			*copies += 1;
		}
	}
	callers
}

/// The budget `generic`'s own attribute sets, if it has one.
fn limit(tcx: TyCtxt<'_>, generic: DefId) -> Option<Result<u64, ()>> {
	let attr = tcx.get_attrs_by_path(generic, &ATTRIBUTE.map(Symbol::intern)).next()?;
	Some(parse(attr).map_err(|err| {
		tcx.dcx().span_err(attr.span(), err);
	}))
}

/// Fail on every `rust2` attribute in the crate but a well-formed budget on a generic
/// function: a crate that instantiates the function reads it, but only this one can say
/// where it is wrong.
pub fn check_attributes(tcx: TyCtxt<'_>) {
	let [tool, name] = ATTRIBUTE.map(Symbol::intern);
	for owner in tcx.hir_crate_items(()).owners() {
		for (local_id, attrs) in tcx.hir_attr_map(owner).map.iter() {
			for attr in attrs.iter().filter(|attr| attr.path().first() == Some(&tool)) {
				let path = attr.path();
				if path.as_slice() != [tool, name] {
					let path: Vec<&str> = path.iter().map(Symbol::as_str).collect();
					tcx.dcx()
						.span_err(attr.span(), format!("unknown attribute `{}`", path.join("::")));
					continue;
				}
				let def = owner.def_id.to_def_id();
				let generic = local_id.as_u32() == 0
					&& matches!(tcx.def_kind(def), DefKind::Fn | DefKind::AssocFn)
					&& tcx.generics_of(def).requires_monomorphization(tcx);
				let checked = match generic {
					true => parse(attr).map(|_| ()),
					false => Err(format!("`{}` goes on a generic function", ATTRIBUTE.join("::"))),
				};
				if let Err(err) = checked {
					tcx.dcx().span_err(attr.span(), err);
				}
			}
		}
	}
}

/// The limit a budget attribute sets.
fn parse(attr: &Attribute) -> Result<u64, String> {
	let mut limit = None;
	let mut reason = false;
	for item in attr.meta_item_list().unwrap_or_default() {
		let arg = item
			.meta_item()
			.and_then(|meta| Some((meta.name()?, &meta.name_value_literal()?.kind)));
		match arg {
			Some((name, LitKind::Int(n, _))) if name.as_str() == "limit" && limit.is_none() => {
				limit = Some(u64::try_from(n.get()).ok());
			}
			Some((name, LitKind::Str(why, _))) if name.as_str() == "reason" && !reason => {
				reason = !why.as_str().trim().is_empty();
			}
			_ => {
				limit = None;
				break;
			}
		}
	}
	match (limit, reason) {
		(Some(Some(limit)), true) => Ok(limit),
		_ => Err(format!(
			"`{}` takes `limit = <statements>` and a non-empty `reason = \"<why>\"`",
			ATTRIBUTE.join("::")
		)),
	}
}
