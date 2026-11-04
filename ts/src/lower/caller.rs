//! `#[track_caller]`, as rustc's codegen passes it: each function that requires its
//! caller's `panic::Location` takes it as a last parameter, and each call the crate
//! writes of one passes the location of the call, or, in a function that takes one
//! itself, the one its own caller passed. A function pointer to one, or a trait object's
//! method that only its impl tracks, passes the function's own location, as rustc's shim
//! does. A call through std's code passes a location in std's source, which translation
//! can't name, so it is refused.

use std::sync::OnceLock;

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{Instance, InstanceKind, TyCtxt};
use rustc_span::{RemapPathScopeComponents, SourceFile, Span};

use super::Translator;
use super::body::Body;
use super::library::{self, Std};
use crate::ts::{Expr, Key, Param, Stmt, StmtKind, Type};

/// The `rt.Location` rustc gives a call at `span`: where the outermost macro call it came
/// from is, in the file as the native build names it, by line and column from 1.
pub fn location(tcx: TyCtxt<'_>, span: Span) -> Expr {
	let topmost = span.ctxt().outer_expn().expansion_cause().unwrap_or(span);
	let pos = tcx.sess.source_map().lookup_char_pos(topmost.lo());
	object(file(tcx, &pos.file), pos.line, pos.col_display + 1)
}

/// Where the toolchain's prebuilt std remaps the crates.io crates it is built from, as
/// `hashbrown`: a directory the toolchain's build picked, which nothing else names.
const STD_DEPS: &str = "/cargo/registry/25cdd57fae9f0462";

/// The name of `file` as the session names it, or for std's own source, as the toolchain's
/// prebuilt std does, which the native build links: by the commit it was built from.
fn file(tcx: TyCtxt<'_>, file: &SourceFile) -> String {
	let name = file
		.name
		.display(RemapPathScopeComponents::MACRO)
		.to_string_lossy()
		.into_owned();
	if !super::of_std(tcx, file.cnum) {
		return name;
	}
	let Some((_, rest)) = name.split_once("/library/") else {
		return name;
	};
	match rest.strip_prefix("vendor/") {
		Some(dep) => format!("{STD_DEPS}/{dep}"),
		None => format!("/rustc/{}/library/{rest}", commit(tcx)),
	}
}

/// The commit the wrapped rustc was built from, as `rustc -vV` says.
fn commit(tcx: TyCtxt<'_>) -> &'static str {
	static COMMIT: OnceLock<Option<String>> = OnceLock::new();
	let commit = COMMIT.get_or_init(|| {
		let rustc = crate::driver::RUSTC.get()?;
		let out = std::process::Command::new(rustc).arg("-vV").output().ok()?;
		let text = String::from_utf8(out.stdout).ok()?;
		text.lines()
			.find_map(|line| line.strip_prefix("commit-hash: "))
			.map(str::to_string)
	});
	match commit {
		Some(commit) => commit,
		None => tcx.dcx().fatal("rust2-ts: the wrapped rustc's `-vV` names no commit, which std's files are named by"),
	}
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

/// Each runtime function that takes where a panic it makes is, as an `at?: Location` last
/// parameter, by how many parameters come before it.
fn located() -> &'static rustc_data_structures::fx::FxHashMap<&'static str, usize> {
	static LOCATED: OnceLock<rustc_data_structures::fx::FxHashMap<&'static str, usize>> = OnceLock::new();
	LOCATED.get_or_init(|| {
		(crate::program::RUNTIME.iter())
			.flat_map(|&(_, text)| signatures(text))
			.collect()
	})
}

/// The functions `text` exports whose last parameter is `at?: Location`, and its index.
fn signatures(text: &'static str) -> impl Iterator<Item = (&'static str, usize)> {
	["export function ", "export const "].into_iter().flat_map(move |keyword| {
		text.match_indices(keyword).filter_map(move |(at, _)| {
			let rest = &text[at + keyword.len()..];
			let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
			let (name, rest) = rest.split_at(end);
			let params = &rest[rest.find('(')? + 1..];
			let params = split(params);
			let last = params.last()?;
			(last.trim() == "at?: Location").then_some((name, params.len() - 1))
		})
	})
}

/// The parameters of the list `text` starts, up to its closing parenthesis.
fn split(text: &str) -> Vec<&str> {
	let mut params = Vec::new();
	let (mut depth, mut start, mut prev) = (0usize, 0, ' ');
	for (i, c) in text.char_indices() {
		match c {
			'(' | '[' | '{' | '<' => depth += 1,
			// The `>` of an arrow type's `=>` closes nothing.
			'>' if prev == '=' => {}
			')' | ']' | '}' | '>' if depth > 0 => depth -= 1,
			')' => {
				if !text[start..i].trim().is_empty() {
					params.push(&text[start..i]);
				}
				return params;
			}
			',' if depth == 0 => {
				params.push(&text[start..i]);
				start = i + 1;
			}
			_ => {}
		}
		prev = c;
	}
	params
}

/// Pass `at` to each call in `stmts` of a runtime function that takes where it panics,
/// which was given nowhere yet.
pub fn locate(stmts: &mut [Stmt], at: &Expr) {
	for stmt in stmts {
		stmt_exprs(stmt, &mut |e| locate_expr(e, at));
	}
}

/// Pass `at` as [`locate`] does, within `e`.
pub fn locate_expr(e: &mut Expr, at: &Expr) {
	walk(e, &mut |e| {
		if let Some((args, index)) = unlocated(e) {
			args.resize_with(index, Expr::undefined);
			args.push(at.clone());
		}
	});
}

/// Whether `stmts` call a runtime function that takes where it panics without saying where.
pub fn unlocated_in(stmts: &mut [Stmt]) -> bool {
	let mut found = false;
	for stmt in stmts {
		stmt_exprs(stmt, &mut |e| walk(e, &mut |e| found |= unlocated(e).is_some()));
	}
	found
}

/// The arguments of `e` if it calls a runtime function that takes where it panics and is
/// given none, and where that goes among them.
fn unlocated(e: &mut Expr) -> Option<(&mut Vec<Expr>, usize)> {
	let Expr::Call(callee, args) = e else {
		return None;
	};
	let name = match &**callee {
		Expr::Rt(name) => name,
		Expr::Instantiate(f, _) if let Expr::Rt(name) = &**f => name,
		_ => return None,
	};
	let index = *located().get(name.as_str())?;
	(args.len() <= index).then_some((args, index))
}

/// Call `f` on each expression `stmt` holds directly, and in its blocks.
fn stmt_exprs(stmt: &mut Stmt, f: &mut dyn FnMut(&mut Expr)) {
	match &mut stmt.kind {
		StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => f(e),
		StmtKind::If { cond, then, els } => {
			f(cond);
			then.iter_mut().chain(els).for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::While { cond: e, body, .. } | StmtKind::ForOf { iter: e, body, .. } => {
			f(e);
			body.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Block { body, .. } => body.iter_mut().for_each(|s| stmt_exprs(s, f)),
		StmtKind::Switch {
			disc, cases, default, ..
		} => {
			f(disc);
			for (value, body) in cases {
				f(value);
				body.iter_mut().for_each(|s| stmt_exprs(s, f));
			}
			default.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Return(None)
		| StmtKind::Decl { init: None, .. }
		| StmtKind::Declare { .. }
		| StmtKind::Break(_)
		| StmtKind::Continue(_) => {}
	}
}

/// Call `f` on `e` and every expression within it, arrow bodies included, innermost first.
fn walk(e: &mut Expr, f: &mut dyn FnMut(&mut Expr)) {
	match e {
		Expr::Ident(_) | Expr::This | Expr::Sym(_) | Expr::Rt(_) | Expr::Lit(_) | Expr::Str(_) => {}
		Expr::Array(items) => items.iter_mut().for_each(|e| walk(e, f)),
		Expr::Object(props) => props.iter_mut().for_each(|(_, e)| walk(e, f)),
		Expr::Member(e, _)
		| Expr::Unary(_, e)
		| Expr::Located(_, e)
		| Expr::As(e, _)
		| Expr::Instantiate(e, _)
		| Expr::Lambda(_, e)
		| Expr::Postfix(e, _) => walk(e, f),
		Expr::Index(a, b) | Expr::Binary(a, _, b) | Expr::Assign(a, b) => {
			walk(a, f);
			walk(b, f);
		}
		Expr::Call(callee, args) | Expr::New(callee, args) => {
			walk(callee, f);
			args.iter_mut().for_each(|e| walk(e, f));
		}
		Expr::Cond(c, a, b) => {
			walk(c, f);
			walk(a, f);
			walk(b, f);
		}
		Expr::Arrow(_, _, body) => body.iter_mut().for_each(|s| stmt_exprs(s, &mut |e| walk(e, f))),
	}
	f(e);
}
