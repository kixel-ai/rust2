//! `#[track_caller]`, as rustc's codegen passes it: each function that requires its
//! caller's `panic::Location` takes it as a last parameter, and each call the crate
//! writes of one passes the location of the call, or, in a function that takes one
//! itself, the one its own caller passed. A function pointer to one, or a trait object's
//! method that only its impl tracks, passes the function's own location, as rustc's shim
//! does. A call through std's code passes a location in std's source, which translation
//! can't name, so it is refused.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Instance, InstanceKind, TyCtxt};
use rustc_span::{RemapPathScopeComponents, Span};

use super::Translator;
use super::body::Body;
use super::library::{self, Std};
use crate::ts::{Expr, Key, Param, StmtKind, Type};

/// The `rt.Location` rustc gives a call at `span`: where the outermost macro call it came
/// from is, in the file as the session names it, by line and column from 1.
pub fn location(tcx: TyCtxt<'_>, span: Span) -> Expr {
	let topmost = span.ctxt().outer_expn().expansion_cause().unwrap_or(span);
	let pos = tcx.sess.source_map().lookup_char_pos(topmost.lo());
	let file = pos
		.file
		.name
		.display(RemapPathScopeComponents::MACRO)
		.to_string_lossy()
		.into_owned();
	object(file, pos.line, pos.col_display + 1)
}

/// The `rt.Location` of `file`, `line`, and `col`.
pub fn object(file: String, line: impl ToString, col: impl ToString) -> Expr {
	Expr::Object(vec![
		(Key::Name("file".into()), Expr::Str(file)),
		(Key::Name("line".into()), Expr::lit(line.to_string())),
		(Key::Name("col".into()), Expr::lit(col.to_string())),
	])
}

/// Whether a call of `instance` passes its caller's location: the crate's own, as std's
/// read it only to say where they panic, which translated panics do not carry.
pub fn tracks<'tcx>(t: &Translator<'_, 'tcx>, instance: Instance<'tcx>) -> bool {
	t.crates.translates(instance.def_id()) && instance.def.requires_caller_location(t.tcx)
}

/// Whether `def_id` is `Location::caller`, which gives the location its call passes.
pub fn is_caller(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
	library::inherent_std(tcx, def_id) == Some(Std::Location) && tcx.item_name(def_id).as_str() == "caller"
}

/// Whether a call of the crate's trait method `def_id` through a trait object passes its
/// caller's location, as its declaration's `#[track_caller]` says.
pub fn tracks_virtual(t: &Translator<'_, '_>, def_id: DefId) -> bool {
	t.crates.translates(def_id) && InstanceKind::Virtual(def_id, 0).requires_caller_location(t.tcx)
}

impl<'tcx> Body<'_, '_, 'tcx> {
	/// The location a call of `instance`, which takes its caller's, passes: the one the
	/// crate's call of it gave.
	pub fn caller_arg(&mut self, instance: Instance<'tcx>, span: Span) -> Expr {
		match self.site.take() {
			Some((at, location)) if at == instance => location,
			site => {
				self.site = site;
				let path = self.t.tcx.def_path_str(instance.def_id());
				self.err(
					span,
					format!(
						"a call through std's code of `{path}`, a `#[track_caller]` function, whose caller is in std's source"
					),
				)
			}
		}
	}

	/// The function `instance` as a pointer, which passes its own location, as rustc's
	/// shim does, or `None` if it takes none.
	pub fn reified(&mut self, instance: Instance<'tcx>, span: Span) -> Option<Expr> {
		let tcx = self.t.tcx;
		if !tracks(self.t, instance) {
			return None;
		}
		let sig = self.t.sig(instance.def_id(), instance.args);
		let params: Vec<Param> = (sig.inputs().iter())
			.map(|input| Param {
				name: self.fresh("a"),
				ty: Some(self.t.ty(*input, span)),
			})
			.collect();
		let site = (instance, location(tcx, tcx.def_span(instance.def_id())));
		let outer = self.site.replace(site);
		let mut body = Vec::new();
		let values = params.iter().map(|p| Expr::ident(&p.name)).collect();
		let call = self.call_def(instance.def_id(), instance.args, values, span, &mut body);
		self.site = outer;
		body.push(StmtKind::Return(Some(call.typed())).at(None));
		// Declared, so its `()` is `void`, which a value's type says is `undefined`.
		let ret = self.t.ret_ty(sig.output(), span);
		let arrow = Expr::Arrow(params, ret, body);
		Some(match sig.output().is_unit() {
			true => Expr::As(Box::new(arrow), self.t.fn_ty(sig.inputs(), sig.output(), span)),
			false => arrow,
		})
	}
}

/// Refuse `instance`, which std's code calls, if it takes its caller's location, which is
/// then in std's source.
pub fn refuse_std_call<'tcx>(t: &mut Translator<'_, 'tcx>, instance: Instance<'tcx>) {
	if tracks(t, instance) {
		let path = t.tcx.def_path_str(instance.def_id());
		let msg = format!("`{path}`, a `#[track_caller]` function std's code calls, whose caller is in std's source");
		t.err(t.tcx.def_span(instance.def_id()), msg);
	}
}

/// The type of the parameter a `#[track_caller]` function takes its caller's location by.
pub fn param_ty() -> Type {
	Type::Rt("Location", Vec::new())
}
