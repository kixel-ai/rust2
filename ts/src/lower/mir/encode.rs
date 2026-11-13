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

fn count(e: &Expr, name: &str) -> usize {
	let mut n = 0;
	visit_expr(e, &mut |e| n += usize::from(matches!(e, Expr::Ident(s) if s == name)));
	n
}

fn index(e: &Expr) -> Option<(&Expr, usize)> {
	let Expr::Index(base, at) = e.unlocated() else {
		return None;
	};
	let Expr::Lit(at) = at.unlocated() else { return None };
	Some((base, at.parse().ok()?))
}

/// Every read must consume the whole array in an append, or read a fixed byte.
/// A write or closure capture keeps its storage, even if all its reads qualify.
fn confined(body: &[Stmt], name: &str, len: usize) -> bool {
	let (mut total, mut allowed, mut writes) = (0, 0, false);
	for stmt in body {
		visit_stmt(stmt, &mut |root| {
			total += count(root, name);
			if append(root).is_some_and(|(_, _, array)| ident(array, name)) {
				allowed += 1;
			}
			visit_expr(root, &mut |e| {
				if matches!(e, Expr::Index(..)) && index(e).is_some_and(|(base, i)| i < len && ident(base, name)) {
					allowed += 1;
				}
				match e {
					Expr::Assign(lhs, _) | Expr::Postfix(lhs, _) if count(lhs, name) != 0 => writes = true,
					Expr::Arrow(..) | Expr::Lambda(..) if count(e, name) != 0 => writes = true,
					_ => {}
				}
			});
		});
	}
	total == allowed && !writes
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
	fuse(body, &counts);
	let original = body.clone();
	list(body, &original, &mut FxHashMap::default(), b);
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

fn collect_spills(body: &[Stmt], original: &[Stmt], out: &mut FxHashSet<String>) {
	fn names(body: &[Stmt], out: &mut FxHashSet<String>) {
		for stmt in body {
			match &stmt.kind {
				StmtKind::Decl { name, .. } => {
					out.insert(name.clone());
				}
				StmtKind::If { then, els, .. } => {
					names(then, out);
					names(els, out);
				}
				StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
					names(body, out)
				}
				StmtKind::Switch { cases, default, .. } => {
					for (_, body) in cases {
						names(body, out);
					}
					names(default, out);
				}
				_ => {}
			}
		}
	}
	let mut old = FxHashSet::default();
	names(original, &mut old);
	names(body, out);
	out.retain(|n| !old.contains(n));
}

fn bytes(e: &Expr, b: &mut Body<'_, '_, '_>, out: &mut Vec<Stmt>) -> Vec<Expr> {
	let values = if let Some(values) = literal(e) {
		values.to_vec()
	} else {
		let (kind, value, n, le) = integer(e).expect("a byte array");
		let value = match kind {
			"f32" => Expr::rs("f32ToBits", vec![value.clone()]),
			"f64" => Expr::rs("f64ToBits", vec![value.clone()]),
			_ => value.clone(),
		};
		let value = b.spill(value, "word", out);
		(0..n)
			.map(|i| {
				let shift = 8 * if le { i } else { n - 1 - i };
				if n <= 4 {
					value
						.clone()
						.binary(">>>", Expr::lit(shift.to_string()))
						.binary("&", Expr::lit("255"))
				} else {
					Expr::rs("intByte", vec![value.clone(), Expr::lit(shift.to_string())])
				}
			})
			.collect()
	};
	// Snapshot every byte before any write or later effect, as the array did.
	values
		.into_iter()
		.map(|v| {
			if matches!(v.unlocated(), Expr::Lit(_)) {
				v
			} else {
				b.spill(v, "byte", out)
			}
		})
		.collect()
}

fn list(body: &mut Vec<Stmt>, original: &[Stmt], arrays: &mut FxHashMap<String, Vec<Expr>>, b: &mut Body<'_, '_, '_>) {
	let mut out = Vec::new();
	for mut stmt in std::mem::take(body) {
		if let StmtKind::Decl {
			mutable: false,
			name,
			init: Some(init),
			..
		} = &stmt.kind
			&& let Some(n) = size(init)
			&& confined(original, name, n)
		{
			let mut init = init.clone();
			replace(&mut init, arrays);
			arrays.insert(name.clone(), bytes(&init, b, &mut out));
			continue;
		}
		match &mut stmt.kind {
			StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => replace(e, arrays),
			StmtKind::If { cond, then, els } => {
				replace(cond, arrays);
				list(then, original, arrays, b);
				list(els, original, arrays, b);
			}
			StmtKind::While { cond, body, .. } | StmtKind::ForOf { iter: cond, body, .. } => {
				replace(cond, arrays);
				list(body, original, arrays, b);
			}
			StmtKind::Block { body, .. } => list(body, original, arrays, b),
			StmtKind::Switch {
				disc, cases, default, ..
			} => {
				replace(disc, arrays);
				for (value, body) in cases {
					replace(value, arrays);
					list(body, original, arrays, b);
				}
				list(default, original, arrays, b);
			}
			_ => {}
		}
		let eager = match &stmt.kind {
			StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => Some(e),
			_ => None,
		};
		if let Some(e) = eager
			&& let Some((kind, target, array)) = append(e)
			&& (size(array).is_some() || matches!(array.unlocated(), Expr::Ident(n) if arrays.contains_key(n)))
		{
			let target = b.spill(target.clone(), "dst", &mut out);
			let values = match array.unlocated() {
				Expr::Ident(n) => arrays[n].clone(),
				_ => bytes(array, b, &mut out),
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

			match &mut stmt.kind {
				StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => {
					*e = Expr::undefined();
					out.push(stmt);
				}
				_ => {}
			}
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

fn fuse(body: &mut Vec<Stmt>, counts: &FxHashMap<String, usize>) {
	// A slice temporary immediately consumed by an append may move past the read of
	// its destination's local, whose identity the slice expression cannot change.
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
		if call(init, "byteSliceFrom").is_none() {
			continue;
		}
		let StmtKind::Expr(next) = &body[i + 1].kind else {
			continue;
		};
		let matches = ["byteVecExtendFromSlice", "bytesMutExtend"]
			.iter()
			.any(|kind| matches!(call(next, kind), Some([target, slice]) if matches!(target.unlocated(), Expr::Ident(_)) && ident(slice, name)));
		if matches && counts.get(name) == Some(&1) {
			let (name, init) = (name.clone(), init.clone());
			super::subst_stmt(&mut body[i + 1], &name, &init);
			body.remove(i);
		}
	}

	for stmt in body {
		match &mut stmt.kind {
			StmtKind::If { then, els, .. } => {
				fuse(then, counts);
				fuse(els, counts);
			}
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } | StmtKind::Block { body, .. } => {
				fuse(body, counts)
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					fuse(body, counts);
				}
				fuse(default, counts);
			}
			_ => {}
		}
	}
}
