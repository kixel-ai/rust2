//! Drop glue for a value whose JS form carries no `[Symbol.dispose]` that drops it: a
//! `Result`, an enum, a tuple, or an instance of a generic type, holding something that
//! drops, and a std type holding one. The crate's types, tuples, and std's enums drop by
//! rustc's `drop_in_place` shim, which runs a `Drop` impl and then drops each field,
//! switching on the variant; std's other types by the runtime, given what drops what
//! they hold.

use rustc_middle::ty::{self, EarlyBinder, Instance, Ty, TypeVisitableExt};
use rustc_span::Span;

use super::{Env, Fx};
use crate::lower::Translator;
use crate::lower::body::Body;
use crate::program::{SymKind, Symbol};
use crate::ts::{self, Expr, Param, StmtKind, SymId, Type};

/// The function dropping a `ty`, made the first time, or `None` for a type over
/// parameters, whose glue only an instance can say.
pub(crate) fn shim<'tcx>(t: &mut Translator<'_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Option<SymId> {
	if let Some(&sym) = t.drop_shims.get(&ty) {
		return Some(sym);
	}
	if ty.has_param() || ty.has_aliases() {
		return None;
	}
	let tcx = t.tcx;
	let sym = t.program.sym(Symbol {
		module: Vec::new(),
		base: "drop".into(),
		suffix: crate::lower::names::ty(tcx, ty),
		owner: None,
		kind: SymKind::Function,
		order: 0,
		span,
		instance: ty::print::with_no_trimmed_paths!(ty.to_string()),
		name: String::new(),
		alias: None,
	});
	t.drop_shims.insert(ty, sym);
	let rustc = match ty.kind() {
		ty::Tuple(_) => true,
		ty::Adt(adt, _) => adt.is_enum() || t.crates.translates(adt.did()),
		_ => false,
	};
	// The shim takes a pointer, which is the value in JS.
	let param_ty = t.ty(ty, span);
	let (params, body) = match rustc {
		true => {
			let instance = Instance::resolve_drop_glue(tcx, ty);
			let body = tcx.instance_mir(instance.def).clone();
			let mir = instance.instantiate_mir_and_normalize_erasing_regions(
				tcx,
				t.typing_env_pub(),
				EarlyBinder::bind(tcx, body),
			);
			let mut b = Body::new(t);
			let mut fx = Fx::new(&mut b, &mir, Env::Shim, span);
			let params = fx.params();
			let body = fx.lower();
			let params = params
				.into_iter()
				.map(|p| Param {
					ty: Some(param_ty.clone()),
					..p
				})
				.collect();
			(params, body)
		}
		false => {
			let param = Param {
				name: "value".into(),
				ty: Some(param_ty),
			};
			let dropped = t.drop_value(Expr::ident("value"), ty, span);
			(
				vec![param],
				dropped.map(|e| StmtKind::Expr(e).at(None)).into_iter().collect(),
			)
		}
	};
	t.program.items.push(ts::Item::Function(ts::Function {
		sym,
		params,
		ret: Type::Builtin("void"),
		body,
		loc: None,
		doc: None,
	}));
	Some(sym)
}

/// `value` dropped by its glue.
pub(crate) fn call(sym: SymId, value: Expr) -> Expr {
	Expr::Sym(sym).call(vec![value])
}
