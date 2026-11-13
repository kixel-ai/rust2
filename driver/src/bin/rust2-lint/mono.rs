//! The mono budget: what a generic's copies cost the crate's codegen.
//!
//! A generic costs the optimized MIR statements of every copy the crate instantiates,
//! as rustc's own size estimate counts them, with its closures and async bodies: each is
//! part of the copy of the function that defines it. A generic fails when that total
//! exceeds its budget, unless every copy is thin enough to only forward.

use rustc_ast::token::Delimiter;
use rustc_ast::{LitKind, MetaItemKind};
use rustc_attr_ir::{AttrArgs, AttrItem, Attribute};
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_hir::def::DefKind;
use rustc_middle::mono::{CollectionMode, MonoItem};
use rustc_middle::ty::{GenericArgs, Instance, InstanceKind, TyCtxt};
use rustc_span::def_id::DefId;
use rustc_span::{Span, Symbol};

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
	copies: FxHashMap<GenericArgs<'tcx>, u64>,
}

impl Cost<'_> {
	fn statements(&self) -> u64 {
		self.copies.values().sum()
	}

	fn largest(&self) -> u64 {
		self.copies.values().copied().max().unwrap_or(0)
	}
}

/// Fail on every generic the crate instantiates whose copies cost more than `budget`
/// statements, or than its own attribute allows.
pub fn check(tcx: TyCtxt<'_>, budget: u64) {
	let instances = instances(tcx);
	let mut costs: FxHashMap<DefId, Cost<'_>> = FxHashMap::default();
	for &instance in &instances {
		let generic = tcx.typeck_root_def_id(instance.def_id());
		let generics = tcx.generics_of(generic);
		if !generics.requires_monomorphization(tcx) {
			continue;
		}
		let copy = instance.args.truncate_to(tcx, generics);
		*costs.entry(generic).or_default().copies.entry(copy).or_default() += tcx.size_estimate(instance) as u64;
	}
	let mut over: Vec<(DefId, Cost<'_>, u64)> = costs
		.into_iter()
		.filter(|(_, cost)| cost.largest() > THIN)
		.filter_map(|(generic, cost)| {
			let limit = match limit(tcx, generic) {
				Some(Ok(limit)) => limit,
				Some(Err(())) => return None,
				None => budget,
			};
			(cost.statements() > limit).then_some((generic, cost, limit))
		})
		.collect();
	if over.is_empty() {
		return;
	}
	over.sort_by_cached_key(|(generic, cost, _)| (std::cmp::Reverse(cost.statements()), tcx.def_path_str(*generic)));
	let callers = callers(tcx, &instances, over.iter().map(|(generic, _, _)| *generic).collect());
	for (generic, cost, limit) in over {
		let mut err = tcx.dcx().struct_span_err(
			tcx.def_span(generic),
			format!(
				"`{}` costs {} MIR statements in {} copies of up to {} each, over its mono budget of {limit}",
				tcx.def_path_str(generic),
				cost.statements(),
				cost.copies.len(),
				cost.largest(),
			),
		);
		let mut callers: Vec<_> = callers.get(&generic).into_iter().flatten().collect();
		callers.sort_by_cached_key(|(caller, (copies, _))| (std::cmp::Reverse(*copies), tcx.def_path_str(**caller)));
		for (caller, (copies, span)) in callers.iter().take(CALLERS) {
			err.span_note(*span, format!("`{}` instantiates {copies}", tcx.def_path_str(**caller)));
		}
		if callers.len() > CALLERS {
			err.note(format!("and {} more callers", callers.len() - CALLERS));
		}
		err.help(format!(
			"make it forward to a non-generic function, or give it its own budget with `#[cfg_attr(rust2, {}(limit = <statements>, reason = \"<why>\"))]`",
			ATTRIBUTE.join("::"),
		));
		err.emit();
	}
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

/// For each of `generics`, the functions outside it that instantiate it: how many
/// copies each, and where the first is.
fn callers<'tcx>(
	tcx: TyCtxt<'tcx>,
	instances: &FxHashSet<Instance<'tcx>>,
	generics: FxHashSet<DefId>,
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
			if generic == caller || !generics.contains(&generic) {
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
	let attr = tcx.tool_attrs(generic, &ATTRIBUTE.map(Symbol::intern)).next()?;
	Some(parse(attr).map_err(|err| {
		tcx.dcx().span_err(attr.span, err);
	}))
}

/// Fail on every `rust2` attribute in the crate but a well-formed budget on a generic
/// function: a crate that instantiates the function reads it, but only this one can say
/// where it is wrong.
pub fn check_attributes(tcx: TyCtxt<'_>) {
	let [tool, name] = ATTRIBUTE.map(Symbol::intern);
	for owner in tcx.hir_crate_items(()).owners() {
		for (local_id, attrs) in tcx.hir_attr_map(owner).map.iter() {
			let tools = attrs.iter().filter_map(|attr| match attr {
				Attribute::Tool(item) => Some(&**item),
				_ => None,
			});
			for attr in tools.filter(|attr| attr.path.segments.first() == Some(&tool)) {
				let path = &attr.path.segments;
				if **path != [tool, name] {
					let path: Vec<&str> = path.iter().map(Symbol::as_str).collect();
					tcx.dcx()
						.span_err(attr.span, format!("unknown attribute `{}`", path.join("::")));
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
					tcx.dcx().span_err(attr.span, err);
				}
			}
		}
	}
}

/// The limit a budget attribute sets.
fn parse(attr: &AttrItem) -> Result<u64, String> {
	let mut limit = None;
	let mut reason = false;
	let list = match &attr.args {
		AttrArgs::Delimited(args) if args.delim == Delimiter::Parenthesis => {
			MetaItemKind::list_from_tokens(args.tokens.clone())
		}
		_ => None,
	};
	for item in list.unwrap_or_default() {
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
