//! Go names: items resolved by their paths from the crate root, and every Rust item's
//! package-level Go name.

use std::collections::{HashSet, VecDeque};

use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{CRATE_DEF_ID, DefId, LOCAL_CRATE};
use rustc_middle::ty::{self, GenericArgKind, Instance, InstanceKind, ShimKind, Ty, TyCtxt};
use rustc_span::Span;

/// Go's keywords and predeclared identifiers, which a package-level name must not shadow.
pub const PREDECLARED: &[&str] = &[
	"break",
	"case",
	"chan",
	"const",
	"continue",
	"default",
	"defer",
	"else",
	"fallthrough",
	"for",
	"func",
	"go",
	"goto",
	"if",
	"import",
	"interface",
	"map",
	"package",
	"range",
	"return",
	"select",
	"struct",
	"switch",
	"type",
	"var",
	"bool",
	"byte",
	"complex64",
	"complex128",
	"error",
	"float32",
	"float64",
	"int",
	"int8",
	"int16",
	"int32",
	"int64",
	"rune",
	"string",
	"uint",
	"uint8",
	"uint16",
	"uint32",
	"uint64",
	"uintptr",
	"true",
	"false",
	"iota",
	"nil",
	"append",
	"cap",
	"clear",
	"close",
	"complex",
	"copy",
	"delete",
	"imag",
	"len",
	"make",
	"max",
	"min",
	"new",
	"panic",
	"print",
	"println",
	"real",
	"recover",
	"any",
	"comparable",
];

/// The crate's `pub` API: each item a path of `pub` names from the crate root reaches,
/// through its modules, re-exports, and those of the crates it re-exports modules of, by
/// its shortest such path, the first in definition order of those as short. A module is
/// walked rather than listed, and an alias of a type with the alias's own parameters
/// stands for the type where no other path reaches it. Each comes with the span of the
/// name that exports it.
pub fn public(tcx: TyCtxt<'_>) -> Vec<(String, DefId, Span)> {
	let root = CRATE_DEF_ID.to_def_id();
	let mut items = Vec::new();
	let mut aliased = Vec::new();
	let mut seen = HashSet::from([root]);
	let mut modules = VecDeque::from([(String::new(), root)]);
	while let Some((prefix, module)) = modules.pop_front() {
		let children = match module.as_local() {
			Some(local) => tcx.module_children_local(local),
			None => tcx.module_children(module),
		};
		for child in children {
			let Res::Def(kind, def_id) = child.res else {
				continue;
			};
			if !child.vis.is_public() || matches!(kind, DefKind::Ctor(..)) || !seen.insert(def_id) {
				continue;
			}
			let (name, span) = (child.ident.name, child.ident.span);
			let path = match prefix.is_empty() {
				true => name.to_string(),
				false => format!("{prefix}::{name}"),
			};
			match kind {
				DefKind::Mod => modules.push_back((path, def_id)),
				DefKind::TyAlias => aliased.push((path, def_id, span)),
				_ => items.push((path, def_id, span)),
			}
		}
	}
	for (path, alias, span) in aliased {
		let identity = ty::GenericArgs::identity_for_item(tcx, alias);
		match *tcx.type_of(alias).instantiate_identity().skip_norm_wip().kind() {
			ty::Adt(adt, args)
				if args == identity && super::library::mapped(tcx, adt.did()).is_none() && seen.insert(adt.did()) =>
			{
				items.push((path, adt.did(), span))
			}
			_ => items.push((path, alias, span)),
		}
	}
	items
}

/// The item a path names in another crate, from its root through modules.
pub fn resolve_in(tcx: TyCtxt<'_>, krate: rustc_hir::def_id::CrateNum, path: &[&str]) -> Option<DefId> {
	let mut item = krate.as_def_id();
	for segment in path {
		item = tcx.module_children(item).iter().find_map(|child| match child.res {
			Res::Def(kind, def_id)
				if child.ident.as_str() == *segment && !matches!(kind, DefKind::Ctor(..) | DefKind::Macro(_)) =>
			{
				Some(def_id)
			}
			_ => None,
		})?;
	}
	Some(item)
}

/// A root's exported Go name: its path in camel case, `harness::run` as `HarnessRun`.
pub fn exported(path: &str) -> String {
	path.split("::")
		.flat_map(|segment| segment.split('_'))
		.map(|word| {
			let mut chars = word.chars();
			match chars.next() {
				Some(first) => first.to_uppercase().chain(chars).collect(),
				None => String::new(),
			}
		})
		.collect()
}

/// A constant's or static's exported Go name: its path as [`exported`] names it, but each
/// upper-case word of its own name capitalized as Go names constants, `limits::MAX_LEN`
/// as `LimitsMaxLen`.
pub fn exported_value(path: &str) -> String {
	let (owner, name) = path.rsplit_once("::").map_or(("", path), |(o, n)| (o, n));
	let words: Vec<String> = name
		.split('_')
		.map(|word| match word.chars().any(|c| c.is_lowercase()) {
			true => word.to_string(),
			false => word.to_lowercase(),
		})
		.collect();
	format!("{}{}", exported(owner), exported(&words.join("_")))
}

/// Whether Go exports `name` as written: an ASCII identifier with a leading uppercase
/// letter, so no Unicode letter or mark Rust allows and Go does not can reach Go's output.
pub fn is_exported(name: &str) -> bool {
	name.starts_with(|c: char| c.is_ascii_uppercase()) && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// `name` as a Go identifier: anything but letters, digits, and `_` becomes `_`.
pub fn ident(name: &str) -> String {
	let mut out: String = name
		.chars()
		.map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
		.collect();
	while out.contains("__") {
		out = out.replace("__", "_");
	}
	let out = out.trim_matches('_').to_string();
	if out.is_empty() || out.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("x{out}");
	}
	out
}

/// A field's Go name: `F<i>` for a tuple's, else its Rust name escaped as [`escaped`],
/// exported when the field is `pub` so Go callers in other packages reach it. Any other
/// name Go would export, or that starts with `_`, takes a leading `_`, which keeps the
/// mapping one to one; a `pub` one that is not ASCII with a leading lowercase letter is
/// refused where its type is declared.
pub fn field(field: &ty::FieldDef, index: usize) -> String {
	let name = field.name.as_str();
	if name.parse::<usize>().is_ok() || name.is_empty() {
		return format!("F{index}");
	}
	let name = escaped(name);
	let mut chars = name.chars();
	match chars.next() {
		Some(first) if first.is_ascii_lowercase() && field.vis.is_public() => {
			first.to_ascii_uppercase().to_string() + chars.as_str()
		}
		Some(first) if first.is_uppercase() || first == '_' => format!("_{name}"),
		_ => name,
	}
}

/// A field's or variant's name kept off Go's keywords and the enum's `tag`, by a trailing
/// `_`, which a name ending in one takes too, so two names never escape to one.
pub fn escaped(name: &str) -> String {
	match name == "tag" || name.ends_with('_') || PREDECLARED.contains(&name) {
		true => format!("{name}_"),
		false => name.to_string(),
	}
}

/// The base of a static's Go name, unexported: its path, with its crate when not the
/// translated one.
pub fn static_base(tcx: TyCtxt<'_>, def_id: DefId) -> String {
	let mut base = String::from("static");
	if def_id.krate != LOCAL_CRATE {
		base = format!("{base}_{}", tcx.crate_name(def_id.krate));
	}
	format!("{base}_{}", item_base(tcx, def_id))
}

/// The base of a function instance's Go name: its owner and name, its crate when not the
/// translated one, and its type arguments.
pub fn func_base<'tcx>(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> String {
	let def_id = instance.def_id();
	let mut parts = Vec::new();
	if def_id.krate != LOCAL_CRATE {
		parts.push(tcx.crate_name(def_id.krate).to_string());
	}
	parts.push(item_base(tcx, def_id));
	match instance.def {
		InstanceKind::Item(_) => {}
		InstanceKind::Shim(ShimKind::ClosureOnce { .. }) => parts.push("once".into()),
		_ => parts.push("shim".into()),
	}
	for arg in instance.args.iter() {
		if let GenericArgKind::Type(ty) = arg.kind() {
			parts.push(ty_short(tcx, ty));
		}
	}
	let mut name = parts.join("_");
	name.truncate(80);
	name
}

/// An item's name under the type or module that owns it.
pub fn item_base(tcx: TyCtxt<'_>, def_id: DefId) -> String {
	match tcx.def_kind(def_id) {
		DefKind::Closure => format!("{}_closure", item_base(tcx, tcx.parent(def_id))),
		DefKind::AssocFn | DefKind::AssocConst => {
			let parent = tcx.parent(def_id);
			let name = tcx.item_name(def_id).to_string();
			match tcx.def_kind(parent) {
				DefKind::Impl { of_trait } => {
					let self_ty = tcx.type_of(parent).instantiate_identity().skip_norm_wip();
					let mut s = ty_short(tcx, self_ty);
					if of_trait {
						let trait_id = tcx.impl_trait_id(parent);
						s.push('_');
						s.push_str(tcx.item_name(trait_id).as_str());
					}
					format!("{s}_{name}")
				}
				_ => format!("{}_{name}", tcx.item_name(parent)),
			}
		}
		DefKind::Ctor(..) => item_base(tcx, tcx.parent(def_id)),
		_ => {
			// Local free functions by their module path, so two modules' `read` stay apart.
			let path = tcx.def_path(def_id);
			path.data
				.iter()
				.filter_map(|d| d.data.get_opt_name())
				.map(|s| s.to_string())
				.collect::<Vec<_>>()
				.join("_")
		}
	}
}

/// A short name for a type, in names of functions instantiated with it.
pub fn ty_short<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> String {
	match ty.kind() {
		ty::Bool => "bool".into(),
		ty::Char => "char".into(),
		ty::Int(i) => i.name_str().into(),
		ty::Uint(u) => u.name_str().into(),
		ty::Float(f) => f.name_str().into(),
		ty::Str => "str".into(),
		ty::Never => "never".into(),
		ty::Adt(adt, args) => {
			let mut s = tcx.item_name(adt.did()).to_string();
			for arg in args.iter() {
				if let GenericArgKind::Type(t) = arg.kind() {
					s.push('_');
					s.push_str(&ty_short(tcx, t));
				}
			}
			s
		}
		ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => format!("ref_{}", ty_short(tcx, *inner)),
		ty::Slice(elem) => format!("slice_{}", ty_short(tcx, *elem)),
		ty::Array(elem, _) => format!("array_{}", ty_short(tcx, *elem)),
		ty::Tuple(tys) if tys.is_empty() => "unit".into(),
		ty::Tuple(tys) => format!(
			"tuple_{}",
			tys.iter().map(|t| ty_short(tcx, t)).collect::<Vec<_>>().join("_")
		),
		ty::FnDef(def_id, _) => tcx.item_name(*def_id).to_string(),
		ty::Closure(..) => "closure".into(),
		ty::FnPtr(..) => "fn".into(),
		ty::Dynamic(..) => "dyn".into(),
		_ => "ty".into(),
	}
}
