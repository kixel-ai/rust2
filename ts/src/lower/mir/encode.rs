//! Byte arrays used only by constant reads and appends, held as scalar bytes.
//! This runs after temporary folding, when a slice's nonescaping use is visible.

use rustc_data_structures::fx::{FxHashMap, FxHashSet};

use super::{visit_expr, visit_stmt};
use crate::lower::body::Body;
use crate::ts::{Expr, Key, Stmt, StmtKind};
use rust2_driver::cfg::Cond;

fn call<'e>(e: &'e Expr, name: &str) -> Option<&'e [Expr]> {
	match e.unlocated() {
		Expr::Call(f, args) if matches!(f.unlocated(), Expr::Rs(n) if n == name) => Some(args),
		_ => None,
	}
}

fn append(e: &Expr) -> Option<(&str, &Expr, &Expr)> {
	for name in ["byteVecExtendFromSlice", "bytesMutExtend"] {
		if let Some([target, slice]) = call(e, name)
			&& let Some([array]) = call(slice, "byteSliceFrom")
		{
			return Some((name, target, array));
		}
	}
	None
}

pub(super) fn literal(e: &Expr) -> Option<&[Expr]> {
	match e.unlocated() {
		Expr::Call(f, values) if matches!(f.unlocated(), Expr::Member(base, Key::Name(n)) if n == "of" && matches!(base.unlocated(), Expr::Ident(n) if n == "Uint8Array")) => {
			Some(values)
		}
		_ => None,
	}
}

fn integer(e: &Expr) -> Option<(&str, &Expr, usize, bool)> {
	if let Expr::Call(f, args) = e.unlocated()
		&& args.is_empty()
		&& let Expr::Member(inner, Key::Name(method)) = f.unlocated()
		&& method == "reverse"
		&& let Some((kind, value, n, le)) = integer(inner)
	{
		return Some((kind, value, n, !le));
	}
	let Expr::Call(f, args) = e.unlocated() else {
		return None;
	};
	let Expr::Rs(name) = f.unlocated() else { return None };
	let [value] = args.as_slice() else { return None };
	let (kind, le) = name
		.strip_suffix("ToBeBytes")
		.map(|k| (k, false))
		.or_else(|| name.strip_suffix("ToLeBytes").map(|k| (k, true)))?;
	let n = match kind {
		"u8" | "i8" => 1,
		"u16" | "i16" => 2,
		"u32" | "i32" | "usize" | "isize" | "f32" => 4,
		"u64" | "i64" | "f64" => 8,
		"u128" | "i128" => 16,
		_ => return None,
	};
	Some((kind, value, n, le))
}

fn size(e: &Expr) -> Option<usize> {
	literal(e).map(<[_]>::len).or_else(|| integer(e).map(|(_, _, n, _)| n))
}

fn ident(e: &Expr, name: &str) -> bool {
	matches!(e.unlocated(), Expr::Ident(n) if n == name)
}

fn index(e: &Expr) -> Option<(&Expr, usize)> {
	let Expr::Index(base, at) = e.unlocated() else {
		return None;
	};
	let Expr::Lit(at) = at.unlocated() else { return None };
	Some((base, at.parse().ok()?))
}

#[derive(Default)]
struct Uses {
	total: usize,
	allowed: usize,
	/// One past the highest fixed byte read.
	end: usize,
	writes: bool,
}

/// Every use of each name, in one walk. Only an append that is a whole expression
/// statement, which `list` rewrites, consumes an array; one anywhere else escapes it.
fn uses(body: &[Stmt], out: &mut FxHashMap<String, Uses>) {
	for stmt in body {
		if let StmtKind::Expr(e) = &stmt.kind
			&& let Some((_, _, array)) = append(e)
			&& let Expr::Ident(n) = array.unlocated()
		{
			out.entry(n.clone()).or_default().allowed += 1;
		}
		match &stmt.kind {
			StmtKind::If { then, els, .. } => {
				uses(then, out);
				uses(els, out);
			}
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
				uses(body, out)
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					uses(body, out);
				}
				uses(default, out);
			}
			_ => {}
		}
	}
}

fn tally(body: &[Stmt]) -> FxHashMap<String, Uses> {
	let mut out = FxHashMap::<String, Uses>::default();
	uses(body, &mut out);
	let mark = |e: &Expr, out: &mut FxHashMap<String, Uses>| {
		visit_expr(e, &mut |e| {
			if let Expr::Ident(n) = e {
				out.entry(n.clone()).or_default().writes = true;
			}
		})
	};
	for stmt in body {
		visit_stmt(stmt, &mut |root| {
			visit_expr(root, &mut |e| match e {
				Expr::Ident(n) => out.entry(n.clone()).or_default().total += 1,
				Expr::Index(..) => {
					if let Some((base, i)) = index(e)
						&& let Expr::Ident(n) = base.unlocated()
					{
						let uses = out.entry(n.clone()).or_default();
						uses.allowed += 1;
						uses.end = uses.end.max(i + 1);
					}
				}
				// A write or closure capture keeps its storage, even if all its reads qualify.
				Expr::Assign(lhs, _) | Expr::Postfix(lhs, _) => mark(lhs, &mut out),
				Expr::Arrow(..) | Expr::Lambda(..) => mark(e, &mut out),
				_ => {}
			});
		});
	}
	out
}

/// Every read must consume the whole array in an append, or read a fixed byte.
fn confined(uses: &FxHashMap<String, Uses>, name: &str, len: usize) -> bool {
	uses.get(name)
		.is_some_and(|u| u.total == u.allowed && u.end <= len && !u.writes)
}

/// Scalar-replace nonescaping byte arrays and append their bytes directly.
pub(super) fn lower(body: &mut Vec<Stmt>, b: &mut Body<'_, '_, '_>) -> bool {
	let mut candidates = false;
	let mut counts = FxHashMap::<String, usize>::default();
	for stmt in body.iter() {
		visit_stmt(stmt, &mut |root| {
			visit_expr(root, &mut |e| {
				candidates |= size(e).is_some();
				if let Expr::Ident(name) = e {
					*counts.entry(name.clone()).or_default() += 1;
				}
			})
		});
	}
	if !candidates {
		return false;
	}
	let mut immutable = FxHashSet::default();
	names(body, &mut immutable, true);
	fuse(body, &counts, &immutable);
	let original = body.clone();
	let uses = tally(&original);
	list(body, &uses, &mut immutable, &mut FxHashMap::default(), b);
	let mut spills = FxHashSet::default();
	collect_spills(body, &original, &mut spills);
	let mut reads = FxHashMap::<String, usize>::default();
	for stmt in body.iter() {
		visit_stmt(stmt, &mut |root| {
			visit_expr(root, &mut |e| {
				if let Expr::Ident(name) = e {
					*reads.entry(name.clone()).or_default() += 1;
				}
			})
		});
	}
	unused(body, &reads, &spills);
	!spills.is_empty() || body.len() != original.len()
}

fn names(body: &[Stmt], out: &mut FxHashSet<String>, immutable: bool) {
	for stmt in body {
		match &stmt.kind {
			StmtKind::Decl { name, mutable, .. } if !immutable || !mutable => {
				out.insert(name.clone());
			}
			StmtKind::If { then, els, .. } => {
				names(then, out, immutable);
				names(els, out, immutable);
			}
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
				names(body, out, immutable)
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					names(body, out, immutable);
				}
				names(default, out, immutable);
			}
			_ => {}
		}
	}
}

fn collect_spills(body: &[Stmt], original: &[Stmt], out: &mut FxHashSet<String>) {
	let mut old = FxHashSet::default();
	names(original, &mut old, false);
	names(body, out, false);
	out.retain(|n| !old.contains(n));
}

fn stable(e: &Expr, immutable: &FxHashSet<String>) -> bool {
	match e.unlocated() {
		Expr::Lit(_) => true,
		Expr::Ident(n) => immutable.contains(n),
		Expr::Binary(l, _, r) => stable(l, immutable) && stable(r, immutable),
		Expr::Unary(_, e) | Expr::As(e, _) => stable(e, immutable),
		_ => false,
	}
}

fn bytes(e: &Expr, immutable: &mut FxHashSet<String>, b: &mut Body<'_, '_, '_>, out: &mut Vec<Stmt>) -> Vec<Expr> {
	let Some(values) = literal(e) else {
		let (kind, value, n, le) = integer(e).expect("a byte array");
		let value = match kind {
			"f32" => Expr::rs("f32ToBits", vec![value.clone()]),
			"f64" => Expr::rs("f64ToBits", vec![value.clone()]),
			_ => value.clone(),
		};
		let value = if stable(&value, immutable) {
			value
		} else {
			b.spill(value, "word", out)
		};
		// A wide integer's 32-bit words, each taken once. Every byte is then a pure
		// function of a fresh const, so it needs no snapshot.
		let words: Vec<Expr> = match n {
			..=4 => vec![value],
			_ => (0..n / 4)
				.map(|k| {
					b.spill(
						Expr::rs("intWord", vec![value.clone(), Expr::lit((32 * k).to_string())]),
						"word",
						out,
					)
				})
				.collect(),
		};
		for word in &words {
			if let Expr::Ident(name) = word {
				immutable.insert(name.clone());
			}
		}
		return (0..n)
			.map(|i| {
				let shift = 8 * if le { i } else { n - 1 - i };
				let word = words[shift / 32].clone();
				let word = match shift % 32 {
					0 => word,
					shift => word.binary(">>>", Expr::lit(shift.to_string())),
				};
				word.binary("&", Expr::lit("255"))
			})
			.collect();
	};
	// Snapshot elements before any later effect, except pure expressions of immutable values.
	values
		.iter()
		.cloned()
		.map(|v| {
			if stable(&v, immutable) {
				v
			} else {
				let value = b.spill(v, "byte", out);
				if let Expr::Ident(name) = &value {
					immutable.insert(name.clone());
				}
				value
			}
		})
		.collect()
}

fn list(
	body: &mut Vec<Stmt>,
	uses: &FxHashMap<String, Uses>,
	immutable: &mut FxHashSet<String>,
	arrays: &mut FxHashMap<String, Vec<Expr>>,
	b: &mut Body<'_, '_, '_>,
) {
	let mut out = Vec::new();
	for mut stmt in std::mem::take(body) {
		if let StmtKind::Decl {
			mutable: false,
			name,
			init: Some(init),
			..
		} = &stmt.kind
			&& let Some(n) = size(init)
			&& confined(uses, name, n)
		{
			let mut init = init.clone();
			replace(&mut init, arrays);
			arrays.insert(name.clone(), bytes(&init, immutable, b, &mut out));
			continue;
		}
		match &mut stmt.kind {
			StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => replace(e, arrays),
			StmtKind::If { cond, then, els } => {
				replace(cond, arrays);
				list(then, uses, immutable, arrays, b);
				list(els, uses, immutable, arrays, b);
			}
			StmtKind::While { cond, body, .. } | StmtKind::ForOf { iter: cond, body, .. } => {
				replace(cond, arrays);
				list(body, uses, immutable, arrays, b);
			}
			StmtKind::Block { body, .. } => list(body, uses, immutable, arrays, b),
			StmtKind::Switch {
				disc, cases, default, ..
			} => {
				replace(disc, arrays);
				for (value, body) in cases {
					replace(value, arrays);
					list(body, uses, immutable, arrays, b);
				}
				list(default, uses, immutable, arrays, b);
			}
			_ => {}
		}
		if let StmtKind::Expr(e) = &stmt.kind
			&& let Some((kind, target, array)) = append(e)
			&& (size(array).is_some() || matches!(array.unlocated(), Expr::Ident(n) if arrays.contains_key(n)))
		{
			let target = if stable(target, immutable) {
				target.clone()
			} else {
				b.spill(target.clone(), "dst", &mut out)
			};
			let values = match array.unlocated() {
				Expr::Ident(n) => arrays[n].clone(),
				_ => bytes(array, immutable, b, &mut out),
			};
			let len = Expr::lit(values.len().to_string());
			let mutable = kind == "bytesMutExtend";
			out.push(
				StmtKind::Expr(Expr::rs(
					if mutable { "bytesMutReserve" } else { "byteVecReserve" },
					vec![
						target.clone(),
						Expr::As(Box::new(len.clone()), crate::ts::Type::Rs("usize", Vec::new())),
					],
				))
				.at(stmt.loc),
			);
			let pos = target.clone().member("length");
			let pos = if mutable {
				target.clone().member("start").binary("+", pos)
			} else {
				pos
			};
			let pos = b.spill(pos, "pos", &mut out);
			for (i, value) in values.into_iter().enumerate() {
				out.push(
					StmtKind::Expr(
						target
							.clone()
							.member("data")
							.index(pos.clone().binary("+", Expr::lit(i.to_string())))
							.assign(value),
					)
					.at(stmt.loc),
				);
			}
			out.push(
				StmtKind::Expr(
					target
						.clone()
						.member("length")
						.assign(target.clone().member("length").binary("+", len)),
				)
				.at(stmt.loc),
			);
		} else {
			out.push(stmt);
		}
	}
	*body = out;
}

fn replace(e: &mut Expr, arrays: &FxHashMap<String, Vec<Expr>>) {
	if let Some((base, i)) = index(e)
		&& let Expr::Ident(n) = base.unlocated()
		&& let Some(values) = arrays.get(n)
	{
		*e = values[i].clone();
		return;
	}
	match e {
		Expr::Member(e, _)
		| Expr::Unary(_, e)
		| Expr::Postfix(e, _)
		| Expr::Located(_, e)
		| Expr::As(e, _)
		| Expr::Instantiate(e, _) => replace(e, arrays),
		Expr::Index(a, b) | Expr::Binary(a, _, b) | Expr::Assign(a, b) => {
			replace(a, arrays);
			replace(b, arrays);
		}
		Expr::Call(f, args) | Expr::New(f, args) => {
			replace(f, arrays);
			for e in args {
				replace(e, arrays);
			}
		}
		Expr::Array(values) => {
			for e in values {
				replace(e, arrays);
			}
		}
		Expr::Object(props) => {
			for (_, e) in props {
				replace(e, arrays);
			}
		}
		Expr::Cond(c, a, b) => {
			replace(c, arrays);
			replace(a, arrays);
			replace(b, arrays);
		}
		_ => {}
	}
}

// Retain evaluation of unused array elements, including calls that can panic.
fn unused(body: &mut Vec<Stmt>, reads: &FxHashMap<String, usize>, spills: &FxHashSet<String>) {
	body.retain(|stmt| !matches!(&stmt.kind, StmtKind::Decl { mutable: false, name, init: Some(init), .. } if spills.contains(name) && !reads.contains_key(name) && init.pure()));
	for stmt in body {
		match &mut stmt.kind {
			StmtKind::Decl {
				mutable: false,
				name,
				init: Some(init),
				..
			} if spills.contains(name) && !reads.contains_key(name) => {
				stmt.kind = StmtKind::Expr(init.clone());
			}
			StmtKind::If { then, els, .. } => {
				unused(then, reads, spills);
				unused(els, reads, spills);
			}
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
				unused(body, reads, spills)
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					unused(body, reads, spills);
				}
				unused(default, reads, spills);
			}
			_ => {}
		}
	}
}

/// Whether evaluating `e` runs no code that could rebind a local, as a closure can.
fn inert(e: &Expr) -> bool {
	e.pure() || literal(e).is_some_and(|v| v.iter().all(inert)) || integer(e).is_some_and(|(_, v, _, _)| inert(v))
}

fn fuse(body: &mut Vec<Stmt>, counts: &FxHashMap<String, usize>, immutable: &FxHashSet<String>) {
	// A slice temporary immediately consumed by an append may move past the read of
	// its destination's local, when the slice expression cannot rebind that local.
	for i in (0..body.len().saturating_sub(1)).rev() {
		let StmtKind::Decl {
			mutable: false,
			name,
			init: Some(init),
			..
		} = &body[i].kind
		else {
			continue;
		};
		let Some([array]) = call(init, "byteSliceFrom") else {
			continue;
		};
		let StmtKind::Expr(next) = &body[i + 1].kind else {
			continue;
		};
		let matches = ["byteVecExtendFromSlice", "bytesMutExtend"]
			.iter()
			.any(|kind| matches!(call(next, kind), Some([target, slice]) if matches!(target.unlocated(), Expr::Ident(t) if immutable.contains(t) || inert(array)) && ident(slice, name)));
		if matches && counts.get(name) == Some(&1) {
			let (name, init) = (name.clone(), init.clone());
			super::subst_stmt(&mut body[i + 1], &name, &init);
			body.remove(i);
		}
	}

	for stmt in body {
		match &mut stmt.kind {
			StmtKind::If { then, els, .. } => {
				fuse(then, counts, immutable);
				fuse(els, counts, immutable);
			}
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
				fuse(body, counts, immutable)
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					fuse(body, counts, immutable);
				}
				fuse(default, counts, immutable);
			}
			_ => {}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn snapshots_only_skip_immutable_values() {
		let decl = |name: &str, mutable| {
			StmtKind::Decl {
				name: name.into(),
				mutable,
				ty: None,
				init: Some(Expr::lit("1")),
			}
			.at(None)
		};
		let mut immutable = FxHashSet::default();
		names(&[decl("constant", false), decl("variable", true)], &mut immutable, true);
		assert!(stable(&Expr::ident("constant"), &immutable));
		assert!(stable(&Expr::lit("42"), &immutable));
		let derived = |name| {
			Expr::ident(name)
				.binary(">>>", Expr::lit("24"))
				.binary("&", Expr::lit("255"))
		};
		assert!(stable(&derived("constant"), &immutable));
		assert!(!stable(&derived("variable"), &immutable));
		assert!(!stable(&Expr::ident("variable"), &immutable));
		assert!(!stable(&Expr::ident("parameter"), &immutable));
		assert!(!stable(&Expr::ident("constant").member("data"), &immutable));
	}

	#[test]
	fn confinement_excludes_writes_and_captures() {
		let read = Expr::ident("bytes").index(Expr::lit("0"));
		let stmt = |e| StmtKind::Expr(e).at(None);
		let confined = |body: &[Stmt], name, len| confined(&tally(body), name, len);
		assert!(confined(&[stmt(read.clone())], "bytes", 2));
		assert!(!confined(
			&[stmt(Expr::ident("bytes").index(Expr::lit("2")))],
			"bytes",
			2
		));
		let extend = Expr::rs(
			"byteVecExtendFromSlice",
			vec![
				Expr::ident("out"),
				Expr::rs("byteSliceFrom", vec![Expr::ident("bytes")]),
			],
		);
		assert!(confined(&[stmt(extend.clone())], "bytes", 2));
		// An append `list` doesn't rewrite, as in a header or a return, escapes.
		let header = StmtKind::If {
			cond: extend.clone(),
			then: Vec::new(),
			els: Vec::new(),
		}
		.at(None);
		assert!(!confined(&[header], "bytes", 2));
		assert!(!confined(&[StmtKind::Return(Some(extend)).at(None)], "bytes", 2));
		for e in [
			read.clone().assign(Expr::lit("42")),
			Expr::Postfix(Box::new(read.clone()), "++"),
			Expr::Lambda(vec![], Box::new(read.clone())),
			Expr::Arrow(
				vec![],
				crate::ts::Type::Builtin("number"),
				vec![StmtKind::Return(Some(read)).at(None)],
			),
		] {
			assert!(!confined(&[stmt(e)], "bytes", 2));
		}
	}
}
