//! Collections, as `rs/collections.ts` gives them: `HashMap`, `HashSet`, `BTreeMap`,
//! `BTreeSet`, `VecDeque`, `LinkedList`, and `BinaryHeap`, std's or hashbrown's, and
//! hashbrown's `HashTable`, with `Ordering` and `DefaultHasher`, which their keys need.
//!
//! A hash collection holds a key that is a JS primitive as its own JS map key, which JS
//! compares as Rust's `Eq` does for numbers, strings, and booleans, and finds any other
//! key through its translated or bound `Hash` and `Eq`, which `rs.hashing` makes it carry.
//! A B-tree orders every key by its translated or bound `Ord`. Each value a collection
//! stores is its own copy, as in Rust.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, GenericArgsRef, Instance, Ty};
use rustc_span::Span;

use super::Translator;
use super::body::Body;
use super::library::{self, Call, Std};
use super::num::Int;
use super::pointer::raw_pointee;
use crate::ts::{Expr, Param, Stmt, StmtKind, Type};

/// A call of a standard library function on these types, or its arguments back if it is
/// none of theirs.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	mut args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	let span = call.span;
	let self_ty = call.args.types().next();
	let self_std = self_ty.and_then(|t| b.t.std_of(t.peel_refs()));
	// An inherent method's type, or a trait method's `Self`.
	let (std, coll) = match library::inherent_std(tcx, call.def_id) {
		Some(std) => (std, impl_self_ty(b, call)),
		None => match (owner, name, self_std, self_ty) {
			("Default", "default", Some(std), Some(t))
			| ("Clone", "clone", Some(std), Some(t))
			| ("Clone", "clone_from", Some(std @ (Std::LinkedList | Std::VecDeque)), Some(t))
			| ("ToOwned", "clone_into", Some(std @ Std::VecDeque), Some(t))
			| ("Index" | "IndexMut", "index" | "index_mut", Some(std), Some(t))
			| ("Extend", "extend", Some(std), Some(t))
			| ("From", "from", Some(std), Some(t))
			| ("BitOr" | "BitAnd" | "Sub" | "BitXor", _, Some(std), Some(t)) => (std, t.peel_refs()),
			// Comparing and hashing a key of a std type.
			("Ord", "cmp", ..) | ("PartialOrd", "partial_cmp", ..) if let Some(t) = self_ty => {
				let t = peel(b, t);
				if !std_key(b, t) {
					return Err(args);
				}
				let f = cmp_fn(b, t, name == "partial_cmp", span);
				return Ok(Some(f.call(args)));
			}
			("Hash", "hash", ..) if let Some(t) = self_ty => {
				let t = peel(b, t);
				// Into `DefaultHasher`, whose state the runtime's functions write, or the crate's
				// own, which they write through its methods.
				let hasher = library::targ(call.args, 1);
				let own = called_hasher(b.t, hasher);
				if !std_key(b, t) || !(own || b.t.std_of(hasher) == Some(Std::Hasher)) {
					return Err(args);
				}
				let f = hash_fn(b, t, hasher, span);
				let mut args = args;
				if own && let Some(state) = args.pop() {
					args.push(own_state(b, hasher, state, span));
				}
				return Ok(Some(f.call(args)));
			}
			// `Ord`'s defaults, of a std type or the crate's, which compare by `PartialOrd`'s `<`,
			// `<=`, and `>`, and of `&mut`s to values JS copies, their `rs.Place`s, by what they
			// hold, as std's impls for a `&mut` forward them.
			("Ord", "max" | "min" | "clamp", ..) if let Some(t) = self_ty => {
				let held = mut_place(b, t);
				let t = peel_ord(b, held.unwrap_or(t), false);
				let own = matches!(t.kind(), ty::Adt(adt, _) if b.t.own(adt.did())) || pointee(b, t).is_some();
				if !(std_key(b, t) || own) || held.is_none() && Int::of(t).is_some() {
					return Err(args);
				}
				let tests: &[&str] = if name == "clamp" { &["le", "lt", "gt"] } else { &["lt"] };
				let mut values = args;
				for test in tests {
					let f = test_fn(b, t, test, span);
					values.push(match held {
						Some(_) => Expr::rs("cellBy", vec![f]),
						None => f,
					});
				}
				// What drops the operands it does not return.
				let Some(droppers) = b.t.droppers(self_ty, span) else {
					return Ok(Some(Expr::undefined()));
				};
				values.extend(droppers);
				let f = match name {
					"max" => "ordMax",
					"min" => "ordMin",
					_ => "ordClamp",
				};
				return Ok(Some(Expr::rs(f, values)));
			}
			// A std type's own comparisons, as a tuple's are, by its `partial_cmp`. The library
			// maps the integers' and `bool`'s.
			("PartialOrd", "lt" | "le" | "gt" | "ge", ..) if let Some(t) = self_ty => {
				let t = peel(b, t);
				if matches!(t.kind(), ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_)) || !std_key(b, t) {
					return Err(args);
				}
				if own_test(b, t, name) {
					return Ok(Some(test_fn(b, t, name, span).call(args)));
				}
				let order = cmp_fn(b, t, true, span).call(args);
				let test = match name {
					"lt" => library::Inherited::Lt,
					"le" => library::Inherited::Le,
					"gt" => library::Inherited::Gt,
					_ => library::Inherited::Ge,
				};
				return Ok(Some(test.test(b, order, out)));
			}
			("PartialEq", "eq" | "ne", ..) if let Some(t) = self_ty => {
				// The library maps the integers', `bool`'s, and byte slices'.
				let rhs = library::targ(call.args, 1);
				// A `Cow<[T]>` compares as the slice it derefs to: its `ne` is `!eq`, or the
				// slice's, which is too.
				let (t, rhs) = (cow_deref(b, t, &mut args, 0), cow_deref(b, rhs, &mut args, 1));
				let slice = |t: Ty<'tcx>| matches!(t.kind(), ty::Slice(_)) && bytes(b, t);
				if slice(t) && slice(rhs) {
					return Err(args);
				}
				let t = peel(b, t);
				// Bytes of any form against any other, as the runtime reads each.
				if bytes(b, t) && bytes(b, peel(b, rhs)) {
					let eq = Expr::rs("eqBytes", args);
					return Ok(Some(if name == "eq" { eq } else { eq.not() }));
				}
				// Sequences of other forms or items, whose `ne` is `!eq`.
				if let Some(eq) = seq_eq(b, t, rhs, span) {
					let eq = eq.call(args);
					return Ok(Some(if name == "eq" { eq } else { eq.not() }));
				}
				let comparable = matches!(b.t.std_of(t), Some(Std::ErrorKind | Std::HashMap | Std::HashSet));
				if matches!(t.kind(), ty::Bool | ty::Int(_) | ty::Uint(_))
					|| library::nothing(tcx, t)
					|| bytes(b, t)
					|| !(std_key(b, t) || comparable)
					|| !alike(b, t, rhs)
				{
					return Err(args);
				}
				// A `String` against text of any form, as `eqStr` reads each.
				let text = library::owned_string(tcx, t) || library::owned_string(tcx, peel(b, rhs));
				let eq = match name {
					_ if text => Expr::Rs("eqStr".into()),
					"eq" => eq_fn(b, t, span),
					_ => not_ne_fn(b, t, span),
				};
				let eq = match (eq, <[Expr; 2]>::try_from(args)) {
					(Expr::Rs(same), Ok([l, r])) if same == "same" => l.binary("===", r),
					(eq, Ok(args)) => eq.call(args.into()),
					(_, Err(args)) => return Err(args),
				};
				return Ok(Some(if name == "eq" { eq } else { eq.not() }));
			}
			("Borrow", "borrow", ..) | ("BorrowMut", "borrow_mut", ..) if let Some(t) = self_ty => {
				let by = library::targ(call.args, 1);
				// A `&mut` to a value JS copies is its `rs.Place`, which `borrow` reads.
				let held = mut_place(b, t).is_some_and(|inner| same_key(b, inner, by));
				let text = library::owned_string(tcx, peel(b, t)) && !library::owned_string(tcx, peel(b, by));
				let value = match arc_of(b, peel(b, t)) {
					// A `String`'s `str` is its text, and its `&mut str` the `String`, the place of it.
					_ if text && name == "borrow" && same_key(b, t, by) => args.pop().map(|v| v.member("v")),
					_ if held || same_key(b, t, by) => args.pop().map(|value| match name {
						"borrow" if held => b.t.pointee_value(t, value),
						// Given the place of its receiver, which a `&mut` holds, but where it lends
						// itself, as std's impl for any `T` does.
						"borrow_mut"
							if t.is_ref()
								&& tcx.erase_and_anonymize_regions(t) != tcx.erase_and_anonymize_regions(by) =>
						{
							let receiver = Ty::new_mut_ref(tcx, tcx.lifetimes.re_erased, t);
							b.t.pointee_value(receiver, value)
						}
						_ => value,
					}),
					Some(inner) if same_key(b, inner, by) => args.pop().map(|v| v.member("value")),
					_ => return Err(args),
				};
				return Ok(value);
			}
			("Hasher", _, Some(Std::Hasher), _) => return Ok(Some(hasher_call(b, name, args, span, out))),
			// A method std provides of the crate's own `Hasher`, which writes through the others.
			("Hasher", _, None, Some(h)) if own_hasher(b.t, h) => {
				let mut values = args;
				let state = values.remove(0);
				values.insert(0, own_state(b, h, state, span));
				return Ok(Some(hasher_call(b, name, values, span, out)));
			}
			("BuildHasher", _, Some(std @ (Std::RandomState | Std::BuildHasherDefault)), Some(t)) => {
				(std, t.peel_refs())
			}
			("BuildHasher", "hash_one", None, Some(s))
				if let Some(h) = built_hasher(b, s)
					&& own_hasher(b.t, h) =>
			{
				return Ok(Some(own_hash_one(b, call, s, h, args, out)));
			}
			// The discriminant derived `Hash` and `Ord` read of an enum.
			("intrinsics", "discriminant_value", ..) if let Some(t) = self_ty => {
				let value = args.pop();
				return Ok(value.map(|value| discriminant(b, t, value, span)));
			}
			_ => return Err(args),
		},
	};
	let by = lookup_ty(b, std, call, owner, name);
	// A `[u8; N]` key is a typed array, but a reference to an array or a slice, as a lookup
	// takes one, is an `rs.ByteSlice`, so a view of its bytes stands in. A range holds the
	// keys themselves.
	if let Some(by) = by
		&& !matches!(name, "range" | "range_mut")
		&& matches!(by.kind(), ty::Array(..) | ty::Slice(_))
		&& bytes(b, by)
	{
		// The key as held, through boxes and `Arc`s: a reference to an array is a slice already.
		let mut key = arg(coll, 0);
		while let Some(Std::Box | Std::Arc) = b.t.std_of(key) {
			key = arg(key, 0);
		}
		if let (true, Some(at)) = (super::byte_array(key), args.get_mut(1)) {
			*at = Expr::rs("byteSliceView", vec![std::mem::replace(at, Expr::undefined())]);
		}
	}
	// An array or a `Vec` key is a JS array, so the elements of an `rs.Slice` stand in.
	if let Some(by) = by
		&& !matches!(name, "range" | "range_mut")
		&& let ty::Slice(elem) = by.kind()
		&& !super::byte(*elem)
		&& let key = peel(b, arg(coll, 0))
		&& (key.is_array() || b.t.std_of(key) == Some(Std::Vec))
		&& let Some(at) = args.get_mut(1)
	{
		*at = super::library::items_of(std::mem::replace(at, Expr::undefined()), *elem);
	}
	// Elements of any form, an array, a slice, or a `Vec`, hash and compare alike, so each
	// finds the others. A reference key finds what it points at by that value's own impls,
	// whatever `alike` makes of its `PartialEq` against a reference.
	if let Some(by) = by
		&& let key = arg(coll, 0)
		&& !same_key(b, key, by)
		&& !alike(b, key, by)
	{
		// A `Cow`'s `Hash`, `Eq`, and `Ord` are what it borrows', so what it is looked up by,
		// or a range's bounds, stands in as it is, as JS reads it as an owned `Cow`. An `Arc`'s
		// are its value's, so the value it is looked up by stands in as an `Arc` of it, which
		// the collection only compares.
		// A B-tree searches by what the crate's own `Borrow` lends of each key, which
		// `btree` gives it.
		let lent = matches!(std, Std::BTreeMap | Std::BTreeSet) && own_borrow(b, key, by);
		let key = peel(b, key);
		let cow = cow_of(b, key).is_some_and(|borrowed| same_key(b, borrowed, by));
		let arc = arc_of(b, key).is_some_and(|inner| same_key(b, inner, by));
		match args.get_mut(1) {
			_ if cow || lent => {}
			Some(key) if arc && !matches!(name, "range" | "range_mut") => {
				let value = std::mem::replace(key, Expr::undefined());
				*key = Expr::New(Box::new(Expr::Rs("Arc".into())), vec![value]);
			}
			_ => {
				return Ok(Some(b.err(
					span,
					format!("`{owner}::{name}` of `{coll}` by `{by}`, which is not supported yet"),
				)));
			}
		}
	}
	// A slice or an array finds an `rs.Vec` key, and a `str` an `rs.String` or an
	// `rs.StrCow` key, which the runtime reads alike, as its type.
	if let Some(by) = by
		&& !matches!(name, "range" | "range_mut")
		&& let Some(at) = args.get_mut(1)
		&& (other_shape(b, arg(coll, 0), by) || text_key(b, arg(coll, 0), by))
	{
		let key = b.t.ty(arg(coll, 0), span);
		*at = as_type(std::mem::replace(at, Expr::undefined()), key);
	}
	let e = match std {
		Std::HashMap | Std::HashSet => hash(b, coll, call, owner, name, args, out),
		Std::HashTable => table(b, coll, call, owner, name, args, out),
		Std::TableEntry => table_entry(b, coll, call, name, args),
		Std::BTreeMap | Std::BTreeSet => btree(b, coll, call, owner, name, args),
		Std::VecDeque => deque(b, coll, call, owner, name, args),
		Std::LinkedList => list(b, coll, call, owner, name, args),
		Std::BinaryHeap => heap(b, coll, call, owner, name, args),
		Std::PeekMut if name == "pop" => Some(Expr::rs("peekMutPop", args)),
		Std::Entry => entry(b, coll, call, name, args),
		Std::Ordering => ordering(name, args),
		Std::Hasher if matches!(name, "new" | "default") => Some(Expr::rs("hasher", vec![])),
		Std::RandomState => Some(random_state(b, call, coll, name, args)),
		Std::BuildHasherDefault => Some(build_hasher_default(b, call, coll, name, args, out)),
		_ => return Err(args),
	};
	let Some(e) = e else {
		return Ok(None);
	};
	// The runtime drops what std's would rather than return or keep, given what drops it.
	let lost = discarded(std, owner, name).unwrap_or_default();
	let Some(mut droppers) = b.t.droppers(lost.iter().map(|&n| arg(coll, n)), span) else {
		return Ok(Some(Expr::undefined()));
	};
	// An `Option` of an element, key, or value that can be `undefined` tells `None` apart.
	if library::none_last(owner, name)
		&& let Some(none) = library::sentinel_option(b.t, call).and_then(|inner| b.t.none_arg(inner))
	{
		droppers.resize(lost.len(), Expr::undefined());
		droppers.push(none);
	}
	let e = match e {
		Expr::Call(f, mut args) if !droppers.is_empty() => {
			args.extend(droppers);
			Expr::Call(f, args)
		}
		e => e,
	};
	Ok(Some(e))
}

/// Whether the mappings of a collection of type `std` hand out the bytes it holds as
/// [`super::lend::lend`] makes them, in what each returns and hands a closure, and read
/// the bytes they are given in either form.
pub fn lending(std: Std) -> bool {
	matches!(
		std,
		Std::HashMap
			| Std::HashSet
			| Std::BTreeMap
			| Std::BTreeSet
			| Std::Entry
			| Std::VecDeque
			| Std::LinkedList
			| Std::BinaryHeap
	)
}

/// What a method of a collection of type `std` may drop rather than return or keep, as the
/// indices of the collection's type arguments whose runtime function takes what drops each,
/// after its own arguments: the key `insert` and `entry` are given if an equal one is there
/// already, the key `remove` takes out, the value `or_insert` is given if one is, and what
/// `clear`, `retain`, `truncate`, and a deque's `resize`, `resize_with`, `clone_from`, and
/// `clone_into` discard. An iterator, and `extend` and `from`, take
/// theirs where they map.
fn discarded(std: Std, owner: &str, name: &str) -> Option<&'static [usize]> {
	let map = matches!(std, Std::HashMap | Std::BTreeMap);
	let set = matches!(std, Std::HashSet | Std::BTreeSet);
	Some(match (owner, name) {
		(_, "insert" | "entry" | "remove") if map || set => &[0],
		(_, "or_insert_with" | "or_insert_with_key" | "or_default") if std == Std::Entry => &[],
		(_, "remove") if std == Std::Entry => &[0],
		(_, "or_insert") if std == Std::Entry => &[1],
		(_, "clear" | "retain" | "retain_mut") if map => &[0, 1],
		(_, "clear" | "retain" | "retain_mut") if set => &[0],
		(
			_,
			"clear" | "retain" | "retain_mut" | "truncate" | "resize" | "resize_with" | "clone_from" | "clone_into",
		) if std == Std::VecDeque => &[0],
		(_, "clear") if std == Std::LinkedList => &[0],
		(_, "clear" | "retain") if std == Std::BinaryHeap => &[0],
		_ => return None,
	})
}

/// `values` and then `flag`, which a runtime function defaults to `false` unless the
/// `rs.None` it takes last follows, where `none`.
fn flagged(mut values: Vec<Expr>, flag: bool, none: bool) -> Vec<Expr> {
	if flag || none {
		values.push(Expr::lit(flag.to_string()));
	}
	values
}

/// What drops each key, and each value of a map, of a collection of type `coll`, of those
/// that drop: `undefined` for one that drops nothing.
fn entry_droppers<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Vec<Expr> {
	let n = match b.t.std_of(coll) {
		Some(Std::HashMap | Std::BTreeMap) => 2,
		_ => 1,
	};
	(0..n).map(|i| b.t.dropper(arg(coll, i), span)).collect()
}

/// What a collection of type `coll` stores each of: a key, a `(key, value)`, or an item.
fn stored<'tcx>(b: &Body<'_, '_, 'tcx>, coll: Ty<'tcx>) -> Ty<'tcx> {
	match b.t.std_of(coll) {
		Some(Std::HashMap | Std::BTreeMap) => Ty::new_tup(b.t.tcx, &[arg(coll, 0), arg(coll, 1)]),
		_ => arg(coll, 0),
	}
}

/// The type a lookup of a map or set takes its key as: the `Q` of `get<Q>` where
/// `K: Borrow<Q>`, or of `Index<&Q>`.
fn lookup_ty<'tcx>(b: &Body<'_, '_, 'tcx>, std: Std, call: &Call<'tcx>, owner: &str, name: &str) -> Option<Ty<'tcx>> {
	if !matches!(std, Std::HashMap | Std::HashSet | Std::BTreeMap | Std::BTreeSet) {
		return None;
	}
	match (owner, name) {
		("Index", "index") => match library::targ(call.args, 1).kind() {
			ty::Ref(_, by, _) => Some(*by),
			_ => None,
		},
		(
			_,
			"get" | "get_mut" | "get_key_value" | "contains_key" | "contains" | "remove" | "remove_entry" | "take"
			| "range" | "range_mut" | "split_off",
		) => {
			let tcx = b.t.tcx;
			let imp = tcx.inherent_impl_of_assoc(call.def_id)?;
			call.args[tcx.generics_of(imp).count()..]
				.iter()
				.find_map(|a| a.as_type())
		}
		_ => None,
	}
}

/// Whether `<key as Borrow<by>>::borrow` is the crate's own, so a search by a `by` orders
/// what it lends of each key, as std's does.
fn own_borrow<'tcx>(b: &Body<'_, '_, 'tcx>, key: Ty<'tcx>, by: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let f = trait_fn(tcx, "Borrow", "borrow");
	Instance::try_resolve(tcx, b.t.typing_env_pub(), f, tcx.mk_args(&[key.into(), by.into()]))
		.ok()
		.flatten()
		.is_some_and(|instance| b.t.own(instance.def_id()))
}

/// Whether a key of type `by` is a JS value that hashes, compares, and orders as one of
/// type `key` does: the same type through references and boxes, or text either way, a JS
/// string or an `rs.String`, which the runtime's string functions take both of.
fn same_key<'tcx>(b: &Body<'_, '_, 'tcx>, key: Ty<'tcx>, by: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let (key, by) = (peel(b, key), peel(b, by));
	let text = |ty| library::string(tcx, ty) || library::owned_string(tcx, ty);
	tcx.erase_and_anonymize_regions(key) == tcx.erase_and_anonymize_regions(by) || (text(key) && text(by))
}

/// Whether a key of type `by` is a JS string that finds a `key` that is a `String` or a
/// `Cow<str>`, an object.
fn text_key<'tcx>(b: &Body<'_, '_, 'tcx>, key: Ty<'tcx>, by: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let key = peel(b, key);
	(library::owned_string(tcx, key) || library::cow_str(tcx, key)) && library::string(tcx, peel(b, by))
}

/// Whether a key of type `by` is a slice or an array that finds a `key` that is a `Vec`,
/// whose JS value is an `rs.Vec` of the array.
fn other_shape<'tcx>(b: &Body<'_, '_, 'tcx>, key: Ty<'tcx>, by: Ty<'tcx>) -> bool {
	b.t.std_of(peel(b, key)) == Some(Std::Vec)
		&& elements_of(b, peel(b, by)).is_some()
		&& b.t.std_of(peel(b, by)) != Some(Std::Vec)
}

/// `value` as a `ty`, through `unknown`.
fn as_type(value: Expr, ty: Type) -> Expr {
	Expr::As(Box::new(Expr::As(Box::new(value), Type::Builtin("unknown"))), ty)
}

/// Whether a `ty` compares to an `other` as to another `ty`: as keys do, as bytes do, or
/// by elements that do. Else the impl of `PartialEq<other>` is its own.
fn alike<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, other: Ty<'tcx>) -> bool {
	// A crate's type against a reference compares by an impl of the crate's own, which no
	// reference's forwards to, as do two references or `Box`es of one with their own.
	let (ty, other) = peel_pair(b, trait_fn(b.t.tcx, "PartialEq", "eq"), ty, other);
	if (pointee(b, ty).is_some() || pointee(b, other).is_some()) && (local(b, peel(b, ty)) || local(b, peel(b, other)))
	{
		return false;
	}
	let (ty, other) = (peel(b, ty), peel(b, other));
	same_key(b, ty, other)
		|| (bytes(b, ty) && bytes(b, other))
		|| matches!((elements_of(b, ty), elements_of(b, other)), (Some(t), Some(o)) if alike(b, t, o))
}

/// Whether `call` lends a value a collection or an atomic holds that JS copies, which its
/// mapping makes an `rs.Place` of: returned as `get_mut` and an entry's do, yielded as
/// `iter_mut`'s, or handed to a closure as `retain`'s. An atomic is that place itself.
pub fn place_call<'tcx>(b: &Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let Some(std) = library::inherent_std(tcx, call.def_id) else {
		return false;
	};
	let name = tcx.item_name(call.def_id);
	let coll = impl_self_ty(b, call);
	let (methods, value): (&[&str], usize) = match std {
		Std::Vec => (&["retain_mut", "dedup_by", "dedup_by_key", "pop_if", "extract_if"], 0),
		Std::VecDeque => (
			&[
				"get_mut",
				"front_mut",
				"back_mut",
				"iter_mut",
				"range_mut",
				"retain_mut",
			],
			0,
		),
		Std::LinkedList => (
			&[
				"front_mut",
				"back_mut",
				"push_front_mut",
				"push_back_mut",
				"iter_mut",
				"extract_if",
			],
			0,
		),
		Std::HashMap => (&["get_mut", "iter_mut", "values_mut", "retain", "extract_if"], 1),
		Std::HashTable => (&["find_mut", "iter_mut", "retain"], 0),
		Std::TableEntry => (&["get_mut", "into_mut", "and_modify"], 0),
		Std::BTreeMap => (
			&["get_mut", "iter_mut", "values_mut", "range_mut", "retain", "extract_if"],
			1,
		),
		Std::Entry if is_vacant(b, coll) => (&["insert"], 1),
		Std::Entry => (
			&[
				"or_insert",
				"or_insert_with",
				"or_insert_with_key",
				"or_default",
				"get_mut",
				"into_mut",
				"and_modify",
			],
			1,
		),
		Std::Atomic => (&["get_mut"], 0),
		_ => return false,
	};
	methods.contains(&name.as_str()) && b.t.needs_place(arg(coll, value))
}

/// The type an inherent method's impl is of, such as `HashMap<K, V, RandomState>`.
fn impl_self_ty<'tcx>(b: &Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> Ty<'tcx> {
	let tcx = b.t.tcx;
	let imp = tcx.inherent_impl_of_assoc(call.def_id).expect("an inherent method");
	let generics = tcx.generics_of(imp).count();
	let ty = tcx
		.type_of(imp)
		.instantiate(tcx, &call.args[..generics])
		.skip_norm_wip();
	tcx.instantiate_bound_regions_with_erased(ty::Binder::dummy(ty))
}

/// A collection's `n`th type argument: a key, value, or item type.
fn arg<'tcx>(coll: Ty<'tcx>, n: usize) -> Ty<'tcx> {
	let ty::Adt(_, args) = coll.kind() else {
		unreachable!("a collection is a struct")
	};
	args.types().nth(n).expect("a collection's type argument")
}

/// What a JS value of `ty` is: through references and boxes, which are their pointee,
/// up to a `&mut` that is an `rs.Place`, and a `NonZero`, which is its integer.
fn peel<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
	match pointee(b, ty) {
		Some(inner) => peel(b, inner),
		None => ty,
	}
}

/// What `ty`, a `&mut` that is an `rs.Place`, points at; not a raw pointer, which orders
/// by its address.
fn mut_place<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Ref(_, _, ty::Mutability::Mut) => b.t.place_pointee(ty),
		_ => None,
	}
}

/// What `ty` points at, if JS holds it as that: `peel`'s one step.
fn pointee<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Ref(_, inner, _) if b.t.place_pointee(ty).is_none() => Some(*inner),
		ty::Adt(adt, args) if matches!(Std::of(b.t.tcx, adt.did()), Some(Std::Box | Std::NonZero)) => {
			Some(args.type_at(0))
		}
		_ => None,
	}
}

/// `ty` through what `peel` peels, while std's impl of `f`, a comparison trait's or
/// `Hash`'s required method, forwards to what each points at: a crate's own impl on a
/// reference or a `Box` is where it stops. `args` makes `f`'s generic arguments of a type.
fn peel_by<'tcx>(
	b: &Body<'_, '_, 'tcx>,
	f: DefId,
	mut ty: Ty<'tcx>,
	args: impl Fn(Ty<'tcx>) -> GenericArgsRef<'tcx>,
) -> Ty<'tcx> {
	while let Some(inner) = pointee(b, ty)
		&& !own_impl(b, f, args(ty))
	{
		ty = inner;
	}
	ty
}

/// `ty` through what `peel_by` peels for `PartialOrd`, or for `Ord` unless `partial`.
pub fn peel_ord<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, partial: bool) -> Ty<'tcx> {
	let tcx = b.t.tcx;
	match partial {
		true => peel_by(b, trait_fn(tcx, "PartialOrd", "partial_cmp"), ty, |t| {
			tcx.mk_args(&[t.into(), t.into()])
		}),
		false => peel_by(b, trait_fn(tcx, "Ord", "cmp"), ty, |t| tcx.mk_args(&[t.into()])),
	}
}

/// `ty` through what `peel_by` peels for `PartialEq`.
fn peel_eq<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
	let tcx = b.t.tcx;
	peel_by(b, trait_fn(tcx, "PartialEq", "eq"), ty, |t| {
		tcx.mk_args(&[t.into(), t.into()])
	})
}

/// `ty` through what `peel_by` peels for `Hash` into a `hasher`.
fn peel_hash<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, hasher: Ty<'tcx>) -> Ty<'tcx> {
	let tcx = b.t.tcx;
	peel_by(b, trait_fn(tcx, "Hash", "hash"), ty, |t| {
		tcx.mk_args(&[t.into(), hasher.into()])
	})
}

/// Whether `f` with `args` resolves to an impl other than std's.
fn own_impl<'tcx>(b: &Body<'_, '_, 'tcx>, f: DefId, args: GenericArgsRef<'tcx>) -> bool {
	let tcx = b.t.tcx;
	Instance::try_resolve(tcx, b.t.typing_env_pub(), f, args)
		.ok()
		.flatten()
		.is_some_and(|instance| !library::is_std(tcx, instance.def_id()))
}

/// Whether `ty`, as `peel_by` leaves it, compares and hashes by the crate's own impls: of a
/// type of its own, or of a reference or a `Box` where `peel_by` stopped.
pub fn by_own<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	local(b, ty) || pointee(b, ty).is_some()
}

/// Evaluate `values` for their effects: a receiver or a capacity, which nothing here keeps.
fn discard(values: Vec<Expr>, out: &mut Vec<Stmt>) {
	for value in values {
		if !value.is_path() {
			out.push(crate::ts::StmtKind::Expr(value).at(None));
		}
	}
}

/// A method of `HashMap` or `HashSet`, or of a trait they implement.
fn hash<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let span = call.span;
	let set = b.t.std_of(coll) == Some(Std::HashSet);
	let key = arg(coll, 0);
	let value = if set { None } else { Some(arg(coll, 1)) };
	let none = library::sentinel_option(b.t, call).is_some();
	let mut values = args;
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => {
			discard(values, out);
			let state = new_state(b, coll, span);
			hash_new(b, coll, state, span)?
		}
		// Its own `BuildHasher`, which it keeps, or a `RandomState`, which nothing reads.
		(_, "with_hasher") => {
			let state = held_state(b, coll).and_then(|_| values.pop());
			discard(values, out);
			hash_new(b, coll, state, span)?
		}
		(_, "with_capacity" | "with_capacity_and_hasher") => {
			let n = values.remove(0);
			let state = held_state(b, coll).and_then(|_| values.pop());
			discard(values, out);
			let mut values = vec![n, hash_max(b, coll, span)];
			values.extend(hashing(b, coll, key, span)?);
			if let Some(state) = state {
				values.resize(3, Expr::undefined());
				values.push(state);
			}
			empty(b, Expr::rs("hashWithCapacity", values), coll, span)
		}
		(_, "hasher") if let Some(state) = held_state(b, coll) => {
			let ty = b.t.ty(state, span);
			Expr::As(Box::new(values.pop()?.member("s")), ty)
		}
		(_, "len") => Expr::rs("hashLen", values),
		(_, "is_empty") => Expr::rs("hashLen", values).binary("===", Expr::lit("0")),
		(_, "clear") => Expr::rs("hashClear", values),
		(_, "contains_key" | "contains") => Expr::rs("hashContains", values),
		(_, "get_mut") if !set && b.t.needs_place(arg(coll, 1)) => Expr::rs("hashGetMut", values),
		(_, "get" | "get_mut") if !set => Expr::rs("hashGet", values),
		(_, "get") => Expr::rs("hashKey", flagged(values, false, none)),
		(_, "take") => Expr::rs("hashKey", flagged(values, true, none)),
		(_, "replace") => Expr::rs("hashReplace", values),
		(_, "get_key_value") => Expr::rs("hashGetKeyValue", values),
		(_, "insert") if set => Expr::rs("hashAdd", values),
		(_, "insert") => Expr::rs("hashInsert", values),
		(_, "remove") if set => Expr::rs("hashDelete", values),
		(_, "remove") => Expr::rs("hashRemove", values),
		(_, "remove_entry") => Expr::rs("hashRemoveEntry", values),
		(_, "retain") if !set && b.t.needs_place(arg(coll, 1)) => Expr::rs("hashRetainMut", values),
		(_, "retain") => Expr::rs("hashRetain", values),
		(_, "extract_if") if set => Expr::rs("hashExtractKeysIf", values),
		(_, "extract_if") if b.t.needs_place(arg(coll, 1)) => Expr::rs("hashExtractIfMut", values),
		(_, "extract_if") => Expr::rs("hashExtractIf", values),
		(_, "iter_mut") if b.t.needs_place(arg(coll, 1)) => Expr::rs("hashIterMut", values),
		(_, "values_mut") if b.t.needs_place(arg(coll, 1)) => Expr::rs("hashValuesMut", values),
		(_, "iter" | "iter_mut") if !set => Expr::rs("hashIter", values),
		(_, "iter" | "keys") => Expr::rs("hashKeys", values),
		(_, "values" | "values_mut") => Expr::rs("hashValues", values),
		// What drops the half it yields, then the other, which the runtime drops as std's
		// does: before `next` returns, and after `fold` has the item.
		(_, "into_keys" | "into_values") => {
			let (kept, other) = if name == "into_keys" { (0, 1) } else { (1, 0) };
			let Some(mut droppers) = b.t.droppers([arg(coll, kept), arg(coll, other)], span) else {
				return Some(Expr::undefined());
			};
			if let Some(state) = state_dropper(b, coll, span) {
				droppers.resize(2, Expr::undefined());
				droppers.push(state);
			}
			let f = if name == "into_keys" { "hashKeys" } else { "hashValues" };
			Expr::rs(f, super::iter::tail(values, droppers))
		}
		(_, "drain") => {
			let droppers = entry_droppers(b, coll, span);
			Expr::rs(
				if set { "hashDrainKeys" } else { "hashDrain" },
				super::iter::tail(values, droppers),
			)
		}
		(_, "entry") => Expr::rs("entry", values),
		(_, name) if let Some(f) = set_op("hash", name) => Expr::rs(f, values),
		("BitOr" | "BitAnd" | "Sub" | "BitXor", _) => {
			let keys = set_op_keys(b, coll, "hash", owner, values, span)?;
			let state = new_state(b, coll, span);
			let target = hash_new(b, coll, state, span)?;
			let max = hash_max(b, coll, span);
			Expr::rs(
				"hashExtendKeys",
				vec![target, keys, Expr::undefined(), Expr::undefined(), max],
			)
		}
		(_, "capacity" | "reserve" | "shrink_to" | "shrink_to_fit" | "allocation_size") => {
			sized(b, coll, name, values, span)
		}
		// hashbrown's returns its own error, an enum of its variants.
		(_, "try_reserve") if !super::stock::as_std(b.t.tcx, call.def_id) => {
			values.push(hash_max(b, coll, span));
			Expr::rs("hashTryReserve", values)
		}
		("Index", "index") => Expr::rs("hashIndex", values),
		("Clone", "clone") => {
			let k = clone_fn(b, key, span);
			let v = match value {
				Some(v) => clone_fn(b, v, span),
				None => Expr::undefined(),
			};
			values.extend([k, v]);
			let s = held_state(b, coll).map(|s| clone_fn(b, s, span));
			Expr::rs("hashClone", super::iter::tail(values, s.into_iter().collect()))
		}
		("Extend", "extend") | ("From", "from") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			let target = match owner {
				"From" => {
					let state = new_state(b, coll, span);
					hash_new(b, coll, state, span)?
				}
				_ => values.pop()?,
			};
			hash_extend(b, coll, target, source, iter, span)
		}
		_ => return None,
	};
	Some(e)
}

/// What `name` of a hash collection or table of type `coll`, given `values`, reads or sets
/// of its capacity, as std guarantees it.
fn sized<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, name: &str, mut values: Vec<Expr>, span: Span) -> Expr {
	match name {
		"capacity" => Expr::rs("hashCapacity", values),
		"reserve" => {
			values.push(hash_max(b, coll, span));
			Expr::rs("hashReserve", values)
		}
		"shrink_to_fit" => {
			values.push(Expr::lit("0"));
			Expr::rs("hashShrinkTo", values)
		}
		"shrink_to" => Expr::rs("hashShrinkTo", values),
		_ => {
			values.push(super::iter::size_of(b, stored(b, coll), span));
			Expr::rs("hashAllocationSize", values)
		}
	}
}

/// A method of hashbrown's `HashTable`, or of a trait it implements: the runtime's map's of
/// its items, under the hash each is given. The runtime takes the `hasher` a method does,
/// and never calls it, as it never moves an item.
fn table<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let span = call.span;
	let item = arg(coll, 0);
	// What a `&mut` to an item is: its place where JS would copy it.
	let place = b.t.needs_place(item);
	let mut values = args;
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => {
			discard(values, out);
			empty(b, Expr::rs("hashNew", vec![]), coll, span)
		}
		(_, "with_capacity") => {
			values.push(hash_max(b, coll, span));
			empty(b, Expr::rs("hashWithCapacity", values), coll, span)
		}
		(_, "len") => Expr::rs("hashLen", values),
		(_, "is_empty") => Expr::rs("hashLen", values).binary("===", Expr::lit("0")),
		(_, "capacity" | "allocation_size") => sized(b, coll, name, values, span),
		// The hasher last.
		(_, "reserve" | "shrink_to" | "shrink_to_fit") => {
			let hasher = values.pop()?;
			match sized(b, coll, name, values, span) {
				Expr::Call(f, mut values) => {
					values.push(hasher);
					Expr::Call(f, values)
				}
				e => e,
			}
		}
		(_, "find_mut") if place => Expr::rs("tableFindMut", values),
		(_, "find" | "find_mut") => Expr::rs("tableFind", values),
		(_, "find_entry") => Expr::rs("tableFindEntry", values),
		(_, "entry") => Expr::rs("tableEntry", values),
		(_, "insert_unique") => Expr::rs("tableInsertUnique", values),
		(_, "clear") => {
			values.push(Expr::undefined());
			values.extend(b.t.droppers([item], span)?);
			Expr::rs("hashClear", values)
		}
		(_, "retain") => {
			values.extend(b.t.droppers([item], span)?);
			Expr::rs(if place { "tableRetainMut" } else { "tableRetain" }, values)
		}
		(_, "iter_mut") if place => Expr::rs("hashValuesMut", values),
		(_, "iter" | "iter_mut") => Expr::rs("hashValues", values),
		(_, "drain") => {
			values.extend(b.t.droppers([item], span)?);
			Expr::rs("tableDrain", values)
		}
		("Clone", "clone") => {
			values.extend([Expr::undefined(), clone_fn(b, item, span)]);
			Expr::rs("hashClone", values)
		}
		_ => return None,
	};
	Some(e)
}

/// A method of a `HashTable`'s `Entry`, `OccupiedEntry`, `VacantEntry`, or `AbsentEntry`.
fn table_entry<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let item = arg(coll, 0);
	let mut values = args;
	let e = match name {
		"insert" | "or_insert" => {
			values.extend(b.t.droppers([item], span)?);
			Expr::rs(
				if name == "insert" {
					"tableEntryInsert"
				} else {
					"tableOrInsert"
				},
				values,
			)
		}
		"or_insert_with" => {
			values.extend(uncalled(b, call)?);
			Expr::rs("tableOrInsertWith", values)
		}
		"and_modify" => {
			values.extend(uncalled(b, call)?);
			Expr::rs(
				if b.t.needs_place(item) {
					"tableAndModifyMut"
				} else {
					"tableAndModify"
				},
				values,
			)
		}
		// The entry is its item's place.
		"get_mut" | "into_mut" if b.t.needs_place(item) => values.pop()?,
		"get" | "get_mut" | "into_mut" => values.pop()?.member("v"),
		"remove" => Expr::rs("tableRemove", values),
		"into_table" => values.pop()?.member("t"),
		_ => return None,
	};
	Some(e)
}

/// The runtime's function of a set's operation or test `name`, of the runtime's `prefix`
/// for its type: `hashUnion` of a `HashSet`'s `union`.
fn set_op(prefix: &str, name: &str) -> Option<String> {
	let op = match name {
		"union" => "Union",
		"intersection" => "Intersection",
		"difference" => "Difference",
		"symmetric_difference" => "SymmetricDifference",
		"is_subset" => "IsSubset",
		"is_superset" => "IsSuperset",
		"is_disjoint" => "IsDisjoint",
		_ => return None,
	};
	Some(format!("{prefix}{op}"))
}

/// What the set operator of trait `owner` builds a set of type `coll` from, as std's: its
/// operation of the two sets in `values`, each key cloned.
fn set_op_keys<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	prefix: &str,
	owner: &str,
	values: Vec<Expr>,
	span: Span,
) -> Option<Expr> {
	let name = match owner {
		"BitOr" => "union",
		"BitAnd" => "intersection",
		"Sub" => "difference",
		_ => "symmetric_difference",
	};
	let keys = Expr::rs(set_op(prefix, name)?, values);
	let clone = clone_fn(b, arg(coll, 0), span);
	Some(match clone == Expr::undefined() {
		true => keys,
		false => Expr::rs("iterMap", vec![keys, clone]),
	})
}

/// `extend` of `target`, a hash collection of type `coll`, from `source`, an iterator of
/// type `iter`, or `collect` into an empty one.
fn hash_extend<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	target: Expr,
	source: Expr,
	iter: Ty<'tcx>,
	span: Span,
) -> Expr {
	let f = match b.t.std_of(coll) {
		Some(Std::HashSet) => "hashExtendKeys",
		_ => "hashExtend",
	};
	let tail = hash_extend_tail(b, coll, iter, span);
	Expr::rs(f, super::iter::tail(vec![target, source], tail))
}

/// What `hashExtend` and its kin take after the collection and the items, of an iterator of
/// type `iter`: what drops each entry, how it reads them, and the most entries it holds.
fn hash_extend_tail<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, iter: Ty<'tcx>, span: Span) -> Vec<Expr> {
	let mut tail = entry_droppers(b, coll, span);
	// By `for_each`.
	tail.push(super::iter::iter_take(b, iter, true, span));
	tail.push(hash_max(b, coll, span));
	tail
}

/// The most entries a hash collection of type `coll` holds on wasm32, past which hashbrown's
/// `reserve` panics: what its table of `(K, V)`, or `(K, ())`, takes as `capacity_to_buckets`
/// and `calculate_layout_for` allow, with 8-byte groups.
fn hash_max<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Expr {
	let (size, align) = match super::iter::layout(b, stored(b, coll), span) {
		Ok(layout) => layout,
		Err(e) => return e,
	};
	const GROUP: u64 = 8;
	let ctrl_align = align.max(GROUP);
	let fits = |cap: u64| -> bool {
		if cap == 0 {
			return true;
		}
		let buckets = if cap < 15 {
			let cap = cap.max(if size <= 1 { 7 } else { 3 });
			if cap < 4 {
				4
			} else if cap < 8 {
				8
			} else {
				16
			}
		} else {
			match cap.checked_mul(8).filter(|&n| n <= u64::from(u32::MAX)) {
				Some(n) => (n / 7).next_power_of_two(),
				None => return false,
			}
		};
		let len = (size * buckets).div_ceil(ctrl_align) * ctrl_align + buckets + GROUP;
		len <= u64::from(i32::MAX as u32) - (ctrl_align - 1)
	};
	// The greatest capacity that fits, of which every smaller one does.
	let (mut lo, mut hi) = (0, u64::from(u32::MAX));
	while lo < hi {
		let mid = lo + (hi - lo).div_ceil(2);
		if fits(mid) {
			lo = mid;
		} else {
			hi = mid - 1;
		}
	}
	Expr::lit(lo.to_string())
}

/// An empty hash collection of type `coll`, which finds its keys as their type needs and
/// holds `state`, its own `BuildHasher`, if given.
fn hash_new<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, state: Option<Expr>, span: Span) -> Option<Expr> {
	let key = arg(coll, 0);
	let mut values: Vec<Expr> = hashing(b, coll, key, span)?.into_iter().collect();
	if let Some(state) = state {
		values.resize(1, Expr::undefined());
		values.push(state);
	}
	Some(empty(b, Expr::rs("hashNew", values), coll, span))
}

/// The `BuildHasher` of a `HashMap<K, V, S, A>` or `HashSet<T, S, A>` of type `coll`.
fn state_of<'tcx>(b: &Body<'_, '_, 'tcx>, coll: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let n = match b.t.std_of(coll)? {
		Std::HashMap => 2,
		Std::HashSet => 1,
		_ => return None,
	};
	let ty::Adt(_, args) = coll.kind() else {
		return None;
	};
	args.types().nth(n)
}

/// The `BuildHasher` a hash collection of type `coll` holds, which `hasher` lends: any but
/// `RandomState`, whose keys nothing reads.
fn held_state<'tcx>(b: &Body<'_, '_, 'tcx>, coll: Ty<'tcx>) -> Option<Ty<'tcx>> {
	state_of(b, coll).filter(|&state| b.t.std_of(state) != Some(Std::RandomState))
}

/// `Default::default` of the `BuildHasher` a new hash collection of type `coll` holds, if
/// it holds one.
fn new_state<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Option<Expr> {
	let state = held_state(b, coll)?;
	let tcx = b.t.tcx;
	let default = trait_fn(tcx, "Default", "default");
	let mut body = Vec::new();
	let value = b.call_def(default, tcx.mk_args(&[state.into()]), Vec::new(), span, &mut body);
	if body.is_empty() {
		return Some(value);
	}
	body.push(StmtKind::Return(Some(value.typed())).at(None));
	Some(Expr::Arrow(Vec::new(), b.t.ty(state, span), body).call(Vec::new()))
}

/// What drops the `BuildHasher` a hash collection of type `coll` holds, if it drops.
fn state_dropper<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Option<Expr> {
	let state = held_state(b, coll)?;
	b.t.drops(state).then(|| b.t.dropper(state, span))
}

/// `new`, as an empty collection of type `coll`: TS infers a generic key's type, as of
/// `rs.cmp`, from no argument, only from the type it is asserted to be.
fn empty<'tcx>(b: &mut Body<'_, '_, 'tcx>, new: Expr, coll: Ty<'tcx>, span: Span) -> Expr {
	Expr::As(Box::new(new), b.t.ty(coll, span))
}

/// The `rs.Hashing` a hash collection of type `coll` finds a `key` that is a JS object
/// by, or `Some(None)` if every key is a JS primitive that compares as Rust's `Eq` does.
fn hashing<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, key: Ty<'tcx>, span: Span) -> Option<Option<Expr>> {
	let peeled = peel(b, key);
	let primitive = match peeled.kind() {
		ty::Bool | ty::Char | ty::Str => true,
		_ if library::nothing(b.t.tcx, peeled) => true,
		ty::Int(_) | ty::Uint(_) => !Int::of(peeled)?.big(),
		ty::Adt(..) if library::string(b.t.tcx, peeled) => true,
		// Unless a reference to it compares by its own `PartialEq`.
		ty::Adt(adt, _) if b.t.own(adt.did()) && fieldless(*adt) => peel_eq(b, key) == peeled && derived_eq(b, peeled),
		_ => false,
	};
	if primitive {
		return Some(None);
	}
	let Some(hasher) = hasher_ty(b, coll) else {
		b.t.err(span, format!("hashing by the hasher of `{coll}`"));
		return None;
	};
	// Text goes by itself, as a JS map key the engine hashes.
	if library::owned_string(b.t.tcx, peeled) || library::cow_str(b.t.tcx, peeled) {
		return Some(Some(Expr::Rs("HASH_TEXT".into())));
	}
	let mut values = vec![hash_fn(b, key, hasher, span), eq_fn(b, key, span)];
	// A fieldless enum is a JS string, and a bound value may be any primitive, which its
	// own `Eq` may tell apart where JS can't, and a `String` and an owned `Cow<str>` are
	// objects a `str` equals, as is the `Some` of an `Option` of one, and a `Reverse`, a
	// `Wrapping`, or a `Saturating` of one.
	let mut held = peeled;
	while let Some(inner) = option_of(b, held)
		.or_else(|| reverse_of(b, held))
		.or_else(|| overflowing_of(b, held))
	{
		held = peel(b, inner);
	}
	if library::cow_str(b.t.tcx, held)
		|| library::owned_string(b.t.tcx, held)
		|| matches!(held.kind(), ty::Adt(adt, _) if fieldless(*adt) || b.t.bound_drops(adt.did()).is_some())
	{
		values.push(Expr::lit("true"));
	}
	Some(Some(Expr::rs("hashing", values)))
}

fn fieldless(adt: ty::AdtDef<'_>) -> bool {
	adt.is_enum() && adt.variants().iter().all(|v| v.fields.is_empty())
}

/// Whether `ty` compares, orders, and hashes by its own impls, which translate or which its
/// binding's exports stand in for, rather than as the runtime does std's.
fn local<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	matches!(ty.kind(), ty::Adt(adt, _) if b.t.own(adt.did()) || b.t.bound_drops(adt.did()).is_some())
}

/// Whether `ty`'s `ne` reaches the crate's own `ne` of a type it holds: through tuples and
/// `Arc`s, but not slices, `Option`s, `Reverse`s, or cells, whose `ne` is `!eq`.
fn own_ne<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let ty = peel_eq(b, ty);
	match ty.kind() {
		ty::Tuple(tys) => tys.iter().any(|t| own_ne(b, t)),
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => own_ne(b, inner),
		_ if by_own(b, ty) => {
			let ne = trait_fn(tcx, "PartialEq", "ne");
			Instance::try_resolve(tcx, b.t.typing_env_pub(), ne, tcx.mk_args(&[ty.into(), ty.into()]))
				.ok()
				.flatten()
				.is_some_and(|instance| instance.def_id() != ne)
		}
		_ => false,
	}
}

/// Whether `ty`'s `PartialOrd::{test}`, `lt` or another of its tests, reaches the crate's
/// own `test` of a type it holds, which `partial_cmp` stands for only while that is the
/// default: through a tuple's last element, `Arc`s, `Reverse`s, and cells. In a program
/// with a panic hook, a `RefCell`'s own too, which borrows elsewhere than its `partial_cmp`.
fn own_test<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, test: &str) -> bool {
	let tcx = b.t.tcx;
	let ty = peel_ord(b, ty, true);
	match ty.kind() {
		ty::Tuple(tys) => tys.last().is_some_and(|&t| own_test(b, t, test)),
		ty::Adt(..) if let Some(inner) = arc_of(b, ty).or_else(|| reverse_of(b, ty)) => own_test(b, inner, test),
		ty::Adt(..) if let Some((inner, by)) = cell_of(b, ty) => {
			(by == "refCellBy" && b.t.hooked) || own_test(b, inner, test)
		}
		_ if by_own(b, ty) => {
			let f = trait_fn(tcx, "PartialOrd", test);
			Instance::try_resolve(tcx, b.t.typing_env_pub(), f, tcx.mk_args(&[ty.into(), ty.into()]))
				.ok()
				.flatten()
				.is_some_and(|instance| instance.def_id() != f)
		}
		_ => false,
	}
}

/// `PartialOrd::{test}` of `ty` as a function, `lt` or another of its tests: the crate's own
/// where `ty` reaches it, or else by `partial_cmp`, as the trait's defaults test it.
pub fn test_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, test: &str, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let ty = peel_ord(b, ty, true);
	// An integer's, which the runtime tests directly rather than through `partial_cmp`.
	if matches!(ty.kind(), ty::Char | ty::Int(_) | ty::Uint(_)) {
		return Expr::Rs(test.into());
	}
	if !own_test(b, ty, test) {
		let by = match test {
			"lt" => "cmpLt",
			"le" => "cmpLe",
			"gt" => "cmpGt",
			_ => "cmpGe",
		};
		return Expr::rs(by, vec![cmp_fn(b, ty, true, span)]);
	}
	match ty.kind() {
		// The fields before the last chain by `partial_cmp`, which no crate's type overrides.
		ty::Tuple(tys) if let Some((&last, init)) = tys.split_last() => {
			let cmps = init.iter().map(|&t| cmp_fn(b, t, true, span)).collect();
			let order = if matches!(test, "lt" | "le") { "-1" } else { "1" };
			let last = test_fn(b, last, test, span);
			Expr::rs("testTuple", vec![Expr::Array(cmps), last, Expr::lit(order)])
		}
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => Expr::rs("cmpArc", vec![test_fn(b, inner, test, span)]),
		ty::Adt(..) if let Some(inner) = reverse_of(b, ty) => {
			Expr::rs("cmpReverse", vec![test_fn(b, inner, test, span)])
		}
		ty::Adt(..) if let Some((inner, by)) = cell_of(b, ty) => {
			let f = test_fn(b, inner, test, span);
			cell_by(b, ty, by, f, test, span)
		}
		_ => {
			let f = trait_fn(tcx, "PartialOrd", test);
			b.fn_value(f, tcx.mk_args(&[ty.into(), ty.into()]), span)
		}
	}
}

/// Whether `ty`'s `PartialEq` is derived, so it compares as JS compares its tags.
fn derived_eq<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let eq = trait_fn(tcx, "PartialEq", "eq");
	Instance::try_resolve(tcx, b.t.typing_env_pub(), eq, tcx.mk_args(&[ty.into(), ty.into()]))
		.ok()
		.flatten()
		.and_then(|instance| tcx.impl_of_assoc(instance.def_id()))
		.is_some_and(|imp| tcx.is_automatically_derived(imp))
}

/// The `Hasher` a hash collection of type `coll` hashes a key with, as the runtime's own
/// hasher stands in for it: its `RandomState`'s, or std's `DefaultHasher` in place of any
/// other `BuildHasher`'s, which no key goes through.
fn hasher_ty<'tcx>(b: &Body<'_, '_, 'tcx>, coll: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = b.t.tcx;
	if held_state(b, coll).is_some() {
		let std = tcx
			.crates(())
			.iter()
			.find(|&&c| tcx.crate_name(c) == rustc_span::sym::std)?;
		let child = |module: DefId, name: &str| {
			let child = tcx.module_children(module).iter().find(|c| c.ident.as_str() == name)?;
			child.res.opt_def_id()
		};
		let hasher = child(child(std.as_def_id(), "hash")?, "DefaultHasher")?;
		return Some(tcx.type_of(hasher).instantiate_identity().skip_norm_wip());
	}
	built_hasher(b, state_of(b, coll)?).filter(|&hasher| b.t.std_of(hasher) == Some(Std::Hasher))
}

/// The `Hasher` a `BuildHasher` of type `state` builds.
fn built_hasher<'tcx>(b: &Body<'_, '_, 'tcx>, state: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = b.t.tcx;
	let hasher = trait_fn(tcx, "BuildHasher", "Hasher");
	let projection = Ty::new_projection(tcx, ty::IsRigid::No, hasher, [state]);
	tcx.try_normalize_erasing_regions(b.t.typing_env_pub(), ty::Unnormalized::new_wip(projection))
		.ok()
}

/// The function `item` of `Hasher`, which has no diagnostic name: the trait beside `Hash`.
fn hasher_fn(tcx: ty::TyCtxt<'_>, item: &str) -> DefId {
	let hash = tcx.get_diagnostic_item(rustc_span::sym::Hash).expect("a trait of std");
	let hasher = tcx
		.module_children(tcx.parent(hash))
		.iter()
		.find(|child| {
			child.ident.as_str() == "Hasher"
				&& child
					.res
					.opt_def_id()
					.is_some_and(|d| tcx.def_kind(d) == rustc_hir::def::DefKind::Trait)
		})
		.and_then(|child| child.res.opt_def_id())
		.expect("a trait of std");
	trait_fn_of(tcx, hasher, item)
}

/// Whether `h` is a `Hasher` of the crate's own, whose impl translates.
pub(super) fn own_hasher<'tcx>(t: &Translator<'_, 'tcx>, h: Ty<'tcx>) -> bool {
	t.translated(hasher_fn(t.tcx, "finish"), t.tcx.mk_args(&[h.into()]))
}

/// Whether the runtime writes into `h` by its methods, through an `rs.OwnHasher` of the
/// `&mut` to it: the crate's own hasher, or one std's impls forward.
pub(super) fn called_hasher<'tcx>(t: &Translator<'_, 'tcx>, h: Ty<'tcx>) -> bool {
	own_hasher(t, h) || forwards_hasher(t, h)
}

/// Whether `h` is a `&mut` or a box of a trait object, the crate's own `Hasher`, std's
/// `DefaultHasher`, or another such, to which std's impls forward each method.
fn forwards_hasher<'tcx>(t: &Translator<'_, 'tcx>, h: Ty<'tcx>) -> bool {
	let held = match *h.kind() {
		ty::Ref(_, held, ty::Mutability::Mut) => held,
		_ if let Some(held) = h.boxed_ty() => held,
		_ => return false,
	};
	held.is_trait() || own_hasher(t, held) || t.std_of(held) == Some(Std::Hasher) || forwards_hasher(t, held)
}

/// Each key of `rs.HasherImpl`, and the `Hasher` method it holds.
const HASHER_METHODS: [(&str, &str); 16] = [
	("write", "write"),
	("finish", "finish"),
	("u8", "write_u8"),
	("u16", "write_u16"),
	("u32", "write_u32"),
	("u64", "write_u64"),
	("u128", "write_u128"),
	("usize", "write_usize"),
	("i8", "write_i8"),
	("i16", "write_i16"),
	("i32", "write_i32"),
	("i64", "write_i64"),
	("i128", "write_i128"),
	("isize", "write_isize"),
	("prefix", "write_length_prefix"),
	("str", "write_str"),
];

/// `state`, the `&mut` to `h`, a `Hasher` of the crate's own, as the runtime's `Hash`
/// functions write into it: an `rs.OwnHasher` of it, which hands its methods the `&mut` as
/// it is, an `rs.Place` where JS copies `h`.
fn own_state<'tcx>(b: &mut Body<'_, '_, 'tcx>, h: Ty<'tcx>, state: Expr, span: Span) -> Expr {
	let methods = hasher_impl(b, h, span);
	Expr::New(Box::new(Expr::Rs("OwnHasher".into())), vec![state, methods])
}

/// The methods of `h`, a `Hasher` of the crate's own, as an `rs.HasherImpl`: `write`,
/// `finish`, and those of std's provided ones its impl writes, a module-level value beside
/// its `finish`, made once. Each takes the `&mut` to `h`, which `finish`, given `&self`,
/// reads where JS copies `h`. One that forwards to another hasher has each method, as
/// what it forwards to, a trait object's, may have any.
fn hasher_impl<'tcx>(b: &mut Body<'_, '_, 'tcx>, h: Ty<'tcx>, span: Span) -> Expr {
	if let Some(&sym) = b.t.hashers.get(&h) {
		return Expr::Sym(sym);
	}
	let tcx = b.t.tcx;
	let args = tcx.mk_args(&[h.into()]);
	let state_ty = Ty::new_mut_ref(tcx, tcx.lifetimes.re_erased, h);
	let state = b.t.ty(state_ty, span);
	let mut methods = Vec::new();
	if forwards_hasher(b.t, h) {
		for (key, item) in HASHER_METHODS {
			let f = hasher_fn(tcx, item);
			let sig = b.t.sig(f, args);
			let name = b.fresh("h");
			let receiver = match item {
				"finish" => b.t.pointee_value(state_ty, Expr::ident(&name)),
				_ => Expr::ident(&name),
			};
			let mut params = vec![Param {
				name,
				ty: Some(state.clone()),
			}];
			let mut values = vec![receiver];
			for &input in &sig.inputs()[1..] {
				let name = b.fresh("value");
				values.push(Expr::ident(&name));
				params.push(Param {
					name,
					ty: Some(b.t.ty(input, span)),
				});
			}
			let mut body = Vec::new();
			let call = b.call_def(f, args, values, span, &mut body);
			body.push(StmtKind::Return(Some(call)).at(None));
			let ret = b.t.ret_ty(sig.output(), span);
			methods.push((crate::ts::Key::Name(key.into()), Expr::Arrow(params, ret, body)));
		}
		let sym = b.t.program.sym(crate::program::Symbol {
			module: Vec::new(),
			base: "hasher".into(),
			suffix: super::names::ty(tcx, h),
			owner: None,
			kind: crate::program::SymKind::Value,
			order: 0,
			span,
			instance: String::new(),
			name: String::new(),
			alias: None,
		});
		b.t.program.items.push(crate::ts::Item::Value {
			sym,
			ty: Type::Rs("HasherImpl", vec![state]),
			value: Expr::Object(methods),
			doc: Some(format!("The methods of `{h}`, which forward to what it points at.")),
		});
		b.t.hashers.insert(h, sym);
		return Expr::Sym(sym);
	}
	for (key, item) in HASHER_METHODS {
		let f = hasher_fn(tcx, item);
		if matches!(item, "write" | "finish") || b.t.translated(f, args) {
			let mut method = b.fn_value(f, args, span);
			if item == "finish" && b.t.needs_place(h) {
				let param = Param {
					name: "h".into(),
					ty: Some(state.clone()),
				};
				method = Expr::Lambda(vec![param], Box::new(method.call(vec![Expr::ident("h").member("v")])));
			}
			methods.push((crate::ts::Key::Name(key.into()), method));
		}
	}
	let finish = Instance::expect_resolve(tcx, b.t.typing_env_pub(), hasher_fn(tcx, "finish"), args, span);
	let finish = b.t.instance(finish);
	let finish = &b.t.program.syms[finish];
	let sym = b.t.program.sym(crate::program::Symbol {
		module: finish.module.clone(),
		base: "hasher".into(),
		suffix: finish.suffix.clone(),
		owner: None,
		kind: crate::program::SymKind::Value,
		order: finish.order,
		span,
		instance: String::new(),
		name: String::new(),
		alias: None,
	});
	b.t.program.items.push(crate::ts::Item::Value {
		sym,
		ty: Type::Rs("HasherImpl", vec![state]),
		value: Expr::Object(methods),
		doc: Some(format!(
			"The methods of `{h}`'s impl, which the runtime's hashing calls."
		)),
	});
	b.t.hashers.insert(h, sym);
	Expr::Sym(sym)
}

/// `BuildHasher::hash_one` of `s`, whose impl builds `h`, a `Hasher` of the crate's own:
/// `build_hasher`, then `Hash::hash` into it and `finish`, then the hasher and the value
/// it takes drop, as std's locals do. A hasher JS copies is built into a place of its own,
/// which its `&mut` is, and drops as the value that place holds.
fn own_hash_one<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	s: Ty<'tcx>,
	h: Ty<'tcx>,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Expr {
	let (tcx, span) = (b.t.tcx, call.span);
	let Ok([state, value]) = <[Expr; 2]>::try_from(args) else {
		unreachable!("hash_one takes a state and a value")
	};
	let value_ty = library::targ(call.args, 1);
	let build = trait_fn(tcx, "BuildHasher", "build_hasher");
	let mut built = b.call_def(build, tcx.mk_args(&[s.into()]), vec![state], span, out);
	let place = b.t.needs_place(h);
	if place {
		built = Expr::Object(vec![(crate::ts::Key::Name("v".into()), built)]);
	}
	let hasher = own_state(b, h, built, span);
	let mut values = vec![hasher, value, hash_fn(b, value_ty, h, span)];
	let mut drops = [b.t.drop_fn(value_ty, span), b.t.drop_fn(h, span)];
	if place && let Some(drop) = drops[1].take() {
		let name = b.fresh("h");
		let held = Expr::ident(&name).member("v");
		let ty = Some(b.t.ty(Ty::new_mut_ref(tcx, tcx.lifetimes.re_erased, h), span));
		drops[1] = Some(Expr::Lambda(vec![Param { name, ty }], Box::new(drop.call(vec![held]))));
	}
	if let [value_drop, Some(hasher_drop)] = drops {
		values.extend([value_drop.unwrap_or_else(Expr::undefined), hasher_drop]);
	} else if let [Some(value_drop), None] = drops {
		values.push(value_drop);
	}
	Expr::rs("hashOne", values)
}

/// A method of `BTreeMap` or `BTreeSet`, or of a trait they implement.
fn btree<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let set = b.t.std_of(coll) == Some(Std::BTreeSet);
	let key = arg(coll, 0);
	let value = if set { None } else { Some(arg(coll, 1)) };
	let mut values = args;
	let mut ranged = None;
	if matches!(name, "range" | "range_mut") {
		let range = bounds(b, call, values.pop()?);
		let target = range_input(b, call).1;
		values.push(match other_shape(b, key, target) {
			true => as_type(range.bounds.clone(), Type::Rs("Bounds", vec![b.t.ty(key, span)])),
			false => range.bounds.clone(),
		});
		ranged = Some(range);
		// std checks the ends by `==` and `>`, which a key of std's alone does as its `Ord`
		// does, and one holding the crate's own or a bound type by that type's `PartialEq`
		// and `PartialOrd`.
		if target.walk().filter_map(|arg| arg.as_type()).any(|t| local(b, t)) {
			values.push(eq_fn(b, target, span));
			values.push(test_fn(b, target, "gt", span));
		}
	}
	// A key, or a range of keys, of a type each key lends by the crate's own `Borrow`, which
	// is searched by that type's `Ord` against what each key lends.
	if let Some(by) = b.t.std_of(coll).and_then(|std| lookup_ty(b, std, call, owner, name))
		&& own_borrow(b, key, by)
		&& let Some(at) = values.get_mut(1)
	{
		let tcx = b.t.tcx;
		let at = std::mem::replace(at, Expr::undefined());
		let cmp = cmp_fn(b, by, false, span);
		let borrow = b.fn_value(
			trait_fn(tcx, "Borrow", "borrow"),
			tcx.mk_args(&[key.into(), by.into()]),
			span,
		);
		values[1] = Expr::rs("by", vec![at, cmp, borrow]);
	}
	// It holds the range, reads its end at each key it reads, and checks each key against
	// the end by `<=` or `<`, which a key of the crate's own does by its `PartialOrd`.
	if name == "extract_if" {
		let f = values.pop()?;
		let range = bounds(b, call, values.pop()?);
		let tests = match key.walk().filter_map(|arg| arg.as_type()).any(|t| local(b, t)) {
			true => vec![test_fn(b, key, "le", span), test_fn(b, key, "lt", span)],
			false => vec![Expr::undefined(), Expr::undefined()],
		};
		let name = match set {
			true => "btreeExtractKeysIf",
			false if b.t.needs_place(arg(coll, 1)) => "btreeExtractIfMut",
			false => "btreeExtractIf",
		};
		return Some(range.held(|bounds, drop| {
			values.extend([bounds, f]);
			Expr::rs(name, super::iter::tail(values, [tests, vec![drop]].concat()))
		}));
	}
	let none = library::sentinel_option(b.t, call).is_some();
	let last = |values: Vec<Expr>| flagged(values, true, none);
	let first = |values: Vec<Expr>| flagged(values, false, none);
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => btree_new(b, coll, span),
		(_, "len") => Expr::rs("btreeLen", values),
		(_, "is_empty") => Expr::rs("btreeLen", values).binary("===", Expr::lit("0")),
		(_, "clear") => Expr::rs("btreeClear", values),
		(_, "contains_key" | "contains") => Expr::rs("btreeContains", values),
		(_, "get_mut") if !set && b.t.needs_place(arg(coll, 1)) => Expr::rs("btreeGetMut", values),
		(_, "get" | "get_mut") if !set => Expr::rs("btreeGet", values),
		(_, "get") => Expr::rs("btreeKey", first(values)),
		(_, "take") => Expr::rs("btreeKey", last(values)),
		(_, "replace") => Expr::rs("btreeReplace", values),
		(_, "get_key_value") => Expr::rs("btreeGetKeyValue", values),
		(_, "insert") if set => Expr::rs("btreeAdd", values),
		(_, "insert") => Expr::rs("btreeInsert", values),
		(_, "remove") if set => Expr::rs("btreeDelete", values),
		(_, "remove") => Expr::rs("btreeRemove", values),
		(_, "remove_entry") => Expr::rs("btreeRemoveEntry", values),
		(_, "retain") if !set && b.t.needs_place(arg(coll, 1)) => Expr::rs("btreeRetainMut", values),
		(_, "retain") => Expr::rs("btreeRetain", values),
		(_, "first_key_value") => Expr::rs("btreeFirst", values),
		(_, "last_key_value") => Expr::rs("btreeFirst", last(values)),
		(_, "pop_first") if set => Expr::rs("btreePopKey", first(values)),
		(_, "pop_last") if set => Expr::rs("btreePopKey", last(values)),
		(_, "pop_first") => Expr::rs("btreePop", values),
		(_, "pop_last") => Expr::rs("btreePop", last(values)),
		(_, "first") => Expr::rs("btreeFirstKey", first(values)),
		(_, "last") => Expr::rs("btreeFirstKey", last(values)),
		(_, "first_entry") => Expr::rs("btreeFirstEntry", values),
		(_, "last_entry") => Expr::rs("btreeFirstEntry", last(values)),
		(_, "iter_mut") if b.t.needs_place(arg(coll, 1)) => Expr::rs("btreeIterMut", values),
		(_, "values_mut") if b.t.needs_place(arg(coll, 1)) => Expr::rs("btreeValuesMut", values),
		(_, "iter" | "iter_mut") if !set => Expr::rs("btreeIter", values),
		(_, "iter" | "keys") => Expr::rs("btreeKeys", values),
		(_, "values" | "values_mut") => Expr::rs("btreeValues", values),
		// Each of which drops the half it does not yield as it reads, and whose `last` reads
		// the back.
		(_, "into_keys" | "into_values") => {
			let mut droppers = entry_droppers(b, coll, span);
			if name == "into_keys" && b.t.drops(coll) {
				droppers.push(Expr::lit("true"));
			}
			let f = if name == "into_keys" {
				"btreeKeys"
			} else {
				"btreeValues"
			};
			Expr::rs(f, super::iter::tail(values, droppers))
		}
		(_, "range") if set => Expr::rs("btreeRangeKeys", values),
		(_, "range_mut") if b.t.needs_place(arg(coll, 1)) => Expr::rs("btreeRangeMut", values),
		(_, "range" | "range_mut") => Expr::rs("btreeRange", values),
		(_, "entry") => Expr::rs("entry", values),
		(_, "split_off") => Expr::rs("btreeSplitOff", values),
		(_, name) if let Some(f) = set_op("btree", name) => Expr::rs(f, values),
		("BitOr" | "BitAnd" | "Sub" | "BitXor", _) => {
			let keys = set_op_keys(b, coll, "btree", owner, values, span)?;
			let target = btree_new(b, coll, span);
			// std's dedups what they yield by `!=`, which finds none equal where the key's
			// impls are derived or std's, and agree.
			let same = match own_ord(b, key, &mut Default::default()) {
				true => not_ne_fn(b, key, span),
				false => Expr::undefined(),
			};
			let mut rest = vec![same];
			rest.extend(entry_droppers(b, coll, span));
			Expr::rs("btreeFromSorted", super::iter::tail(vec![target, keys], rest))
		}
		("Index", "index") => Expr::rs("btreeIndex", values),
		("Clone", "clone") => {
			let k = clone_fn(b, key, span);
			let v = match value {
				Some(v) => clone_fn(b, v, span),
				None => Expr::undefined(),
			};
			values.extend([k, v]);
			Expr::rs("btreeClone", values)
		}
		("Extend", "extend") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			extend(b, coll, library::targ(call.args, 1), values.pop()?, source, iter, span)?
		}
		("From", "from") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			let trusted = super::iter::implements(b.t, iter, "TrustedLen");
			btree_collect(b, coll, source, iter, trusted, span)
		}
		_ => return None,
	};
	Some(match ranged {
		Some(ranged) => ranged.after(e),
		None => e,
	})
}

/// `collect` and `from` into an empty B-tree of type `coll`, from `source`, an iterator of
/// type `iter`. std's collects into a `Vec` first, reserved as for a `TrustedLen` iterator
/// if `trusted`, sorts it as std's stable sort does, a set's by `PartialOrd::lt` and a
/// map's by `Ord::cmp`, then builds the tree of each but the last of a run of keys `!=` finds
/// equal, which drop in order.
fn btree_collect<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	source: Expr,
	iter: Ty<'tcx>,
	trusted: bool,
	span: Span,
) -> Expr {
	let set = b.t.std_of(coll) == Some(Std::BTreeSet);
	let target = btree_new(b, coll, span);
	let key = arg(coll, 0);
	let elem = stored(b, coll);
	let own = own_ord(b, key, &mut Default::default());
	// Of keys whose impls are derived or std's, which agree, inserting each in turn and
	// keeping the last of equal ones builds the same tree faster, where none drops.
	if !own && !b.t.drops(coll) {
		let mut droppers = vec![Expr::lit("true"), Expr::undefined()];
		if !set {
			droppers.push(Expr::undefined());
		}
		// Into a `Vec` first.
		let folds = super::iter::implements(b.t, iter, "TrustedLen");
		droppers.push(super::iter::iter_take(b, iter, folds, span));
		droppers.push(super::iter::size_of(b, elem, span));
		droppers.push(Expr::lit(trusted.to_string()));
		let f = if set { "btreeExtendKeys" } else { "btreeExtend" };
		return Expr::rs(f, super::iter::tail(vec![target, source], droppers));
	}
	let entries = super::iter::to_vec(b, source, iter, elem, trusted, span).member("items");
	let mut rest = Vec::from(super::library::sort_of(b, elem, span));
	// A set's sort tests `lt`, and a map's `Ord::cmp`; std's sorts no zero-sized one.
	if set {
		rest.push(match own && !b.t.zero_sized(elem) {
			true => test_fn(b, key, "lt", span),
			false => Expr::undefined(),
		});
	}
	rest.push(match own {
		true => not_ne_fn(b, key, span),
		false => Expr::undefined(),
	});
	rest.extend(entry_droppers(b, coll, span));
	let f = if set { "btreeCollectKeys" } else { "btreeCollect" };
	Expr::rs(f, super::iter::tail(vec![target, entries], rest))
}

/// Whether ordering or comparing a `ty` may run the crate's own `Ord`, `PartialOrd`, or
/// `PartialEq`, rather than one derived, which may panic or disagree with itself.
fn own_ord<'tcx>(
	b: &Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	seen: &mut rustc_data_structures::fx::FxHashSet<Ty<'tcx>>,
) -> bool {
	let tcx = b.t.tcx;
	let ty = peel(b, ty);
	if !seen.insert(ty) {
		return false;
	}
	match ty.kind() {
		// A binding's, hand-written even where the Rust impl is derived.
		ty::Adt(adt, _) if b.t.bound_drops(adt.did()).is_some() => true,
		ty::Adt(adt, args) if b.t.own(adt.did()) => {
			let derived = |tr: &str, item: &str, args: &[ty::GenericArg<'tcx>]| {
				let f = trait_fn(tcx, tr, item);
				Instance::try_resolve(tcx, b.t.typing_env_pub(), f, tcx.mk_args(args))
					.ok()
					.flatten()
					.and_then(|instance| tcx.impl_of_assoc(instance.def_id()))
					.is_some_and(|imp| tcx.is_automatically_derived(imp))
			};
			!derived("Ord", "cmp", &[ty.into()])
				|| !derived("PartialOrd", "partial_cmp", &[ty.into(), ty.into()])
				|| !derived("PartialEq", "eq", &[ty.into(), ty.into()])
				|| adt
					.all_fields()
					.any(|f| own_ord(b, f.ty(tcx, args).skip_norm_wip(), seen))
		}
		// Which compare nothing they name.
		ty::RawPtr(..) | ty::FnPtr(..) => false,
		ty::Adt(adt, _) if adt.is_phantom_data() => false,
		// std's, by each type it holds, which decides whether to look deeper.
		_ => {
			let mut walk = ty.walk();
			walk.next();
			while let Some(arg) = walk.next() {
				if let Some(t) = arg.as_type() {
					walk.skip_current_subtree();
					if own_ord(b, t, seen) {
						return true;
					}
				}
			}
			false
		}
	}
}

/// An empty B-tree of type `coll`, ordered by its keys' `Ord`.
fn btree_new<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Expr {
	let cmp = cmp_fn(b, arg(coll, 0), false, span);
	empty(b, Expr::rs("btreeNew", vec![cmp]), coll, span)
}

/// An empty `VecDeque` of type `coll`.
fn deque_new<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Expr {
	let made = Expr::rs("dequeNew", deque_items(b, arg(coll, 0), span));
	empty(b, made, coll, span)
}

/// What a runtime function that makes a `VecDeque` of `item`s is given after its own
/// arguments: their size, by which std's grows it, and whether they are bytes, which it
/// holds in a `Uint8Array` its slices view.
fn deque_items<'tcx>(b: &mut Body<'_, '_, 'tcx>, item: Ty<'tcx>, span: Span) -> Vec<Expr> {
	let bytes = super::byte(item);
	super::iter::tail(
		vec![super::iter::size_of(b, item, span)],
		vec![Expr::lit(bytes.to_string())],
	)
}

/// A method of `VecDeque`, or of a trait it implements.
fn deque<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let item = arg(coll, 0);
	let mut values = args;
	let none = library::sentinel_option(b.t, call).is_some();
	let back = |values: Vec<Expr>| flagged(values, true, none);
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => deque_new(b, coll, span),
		(_, "with_capacity") => {
			values.extend(deque_items(b, item, span));
			empty(b, Expr::rs("dequeWithCapacity", values), coll, span)
		}
		(
			_,
			"capacity" | "reserve" | "reserve_exact" | "try_reserve" | "try_reserve_exact" | "shrink_to"
			| "shrink_to_fit",
		) => {
			if name == "shrink_to_fit" {
				values.push(Expr::lit("0"));
			}
			let f = match name {
				"capacity" => "dequeCapacity",
				"reserve" => "dequeReserve",
				"reserve_exact" => "dequeReserveExact",
				"try_reserve" => "dequeTryReserve",
				"try_reserve_exact" => "dequeTryReserveExact",
				_ => "dequeShrinkTo",
			};
			Expr::rs(f, values)
		}
		(_, "len") => values.pop()?.member("n"),
		(_, "is_empty") => values.pop()?.member("n").binary("===", Expr::lit("0")),
		(_, "clear") => Expr::rs("dequeClear", values),
		(_, "push_back") => Expr::rs("dequePushBack", values),
		(_, "push_front") => Expr::rs("dequePushFront", values),
		(_, "pop_back") => Expr::rs("dequePopBack", values),
		(_, "pop_front") => Expr::rs("dequePopFront", values),
		(_, "front_mut") if b.t.needs_place(item) => Expr::rs("dequeEndMut", values),
		(_, "back_mut") if b.t.needs_place(item) => Expr::rs("dequeEndMut", back(values)),
		(_, "get_mut") if b.t.needs_place(item) => Expr::rs("dequeGetMut", values),
		(_, "front" | "front_mut") => Expr::rs("dequeEnd", flagged(values, false, none)),
		(_, "back" | "back_mut") => Expr::rs("dequeEnd", back(values)),
		(_, "get" | "get_mut") => Expr::rs("dequeGet", values),
		(_, "swap_remove_front") => Expr::rs("dequeSwapRemove", flagged(values, false, none)),
		(_, "swap_remove_back") => Expr::rs("dequeSwapRemove", back(values)),
		(_, "truncate") => Expr::rs("dequeTruncate", values),
		(_, "append") => Expr::rs("dequeAppend", values),
		(_, "split_off") => Expr::rs("dequeSplitOff", values),
		// `resize` clones its value for each slot but the last.
		(_, "resize") => {
			values.push(clone_fn(b, item, span));
			Expr::rs("dequeResize", values)
		}
		(_, "resize_with") => Expr::rs("dequeResizeWith", values),
		(_, "swap") => Expr::rs("dequeSwap", values),
		(_, "rotate_left") => Expr::rs("dequeRotate", values),
		(_, "rotate_right") => Expr::rs("dequeRotate", flagged(values, true, false)),
		(_, "make_contiguous") if super::byte(item) => Expr::rs("dequeByteMakeContiguous", values),
		(_, "make_contiguous") if item.is_array() => {
			values.push(super::array_fn(item, "From"));
			Expr::rs("dequeArrayMakeContiguous", values)
		}
		(_, "make_contiguous") => Expr::rs("dequeMakeContiguous", values),
		(_, "binary_search") => {
			values.push(cmp_fn(b, item, false, span));
			Expr::rs("dequeBinarySearch", values)
		}
		(_, "binary_search_by") => Expr::rs("dequeBinarySearchBy", values),
		// `VecDeque<T, A>::binary_search_by_key::<B, F>`.
		(_, "binary_search_by_key") => {
			let key = library::targ(call.args, 2);
			values.push(cmp_fn(b, key, false, span));
			let drop = b.t.dropper(key, span);
			Expr::rs("dequeBinarySearchByKey", super::iter::tail(values, vec![drop]))
		}
		(_, "partition_point") => Expr::rs("dequePartitionPoint", values),
		(_, "retain_mut") if b.t.needs_place(item) => Expr::rs("dequeRetainMut", values),
		(_, "retain" | "retain_mut") => Expr::rs("dequeRetain", values),
		(_, "remove") => Expr::rs("dequeRemove", values),
		(_, "insert") => Expr::rs("dequeInsert", values),
		(_, "contains") => {
			values.push(eq_fn(b, item, span));
			Expr::rs("dequeContains", values)
		}
		(_, "iter_mut") if b.t.needs_place(item) => Expr::rs("dequeIterMut", values),
		(_, "iter" | "iter_mut") => Expr::rs("dequeIter", values),
		(_, "range" | "range_mut") => {
			let ranged = bounds(b, call, values.pop()?);
			values.push(ranged.bounds.clone());
			let f = match name == "range_mut" && b.t.needs_place(item) {
				true => "dequeRangeMut",
				false => "dequeRange",
			};
			ranged.after(Expr::rs(f, values))
		}
		// Each a copy of its part of the buffer, or with `_mut`, a view that writes into it;
		// bytes and arrays are views either way.
		(_, "as_slices" | "as_mut_slices") if super::byte(item) => Expr::rs("dequeByteSlices", values),
		(_, "as_slices" | "as_mut_slices") if item.is_array() => {
			values.push(super::array_fn(item, "From"));
			Expr::rs("dequeArraySlices", values)
		}
		(_, "as_slices" | "as_mut_slices") => Expr::rs("dequeAsSlices", values),
		(_, "drain") => {
			let ranged = bounds(b, call, values.pop()?);
			values.push(ranged.bounds.clone());
			let droppers = entry_droppers(b, coll, span);
			ranged.after(Expr::rs("dequeDrain", super::iter::tail(values, droppers)))
		}
		("Index" | "IndexMut", "index" | "index_mut") => Expr::rs("dequeIndex", values),
		("Clone", "clone") => {
			let clone = clone_fn(b, item, span);
			Expr::rs("dequeClone", super::iter::tail(values, vec![clone]))
		}
		// std's clears it, which drops its items, and extends it by clones of the source's,
		// keeping its capacity; `clone_into` is the target's `clone_from`.
		("Clone", "clone_from") | ("ToOwned", "clone_into") => {
			if name == "clone_into" {
				values.reverse();
			}
			values.push(clone_fn(b, item, span));
			Expr::rs("dequeCloneFrom", values)
		}
		// `From<Vec<T>>` keeps the vector's capacity, and `From<[T; N]>` holds exactly `N`.
		("From", "from") if b.t.std_of(library::targ(call.args, 1)) == Some(Std::Vec) => {
			let v = values.pop()?;
			let v = match super::byte(item) {
				true => Expr::rs("byteVecToVec", vec![v]),
				false => v,
			};
			let made = Expr::rs("dequeFromVec", [vec![v], deque_items(b, item, span)].concat());
			empty(b, made, coll, span)
		}
		("From", "from") if library::targ(call.args, 1).is_array() => {
			let a = values.pop()?;
			let made = Expr::rs("dequeFromArray", [vec![a], deque_items(b, item, span)].concat());
			empty(b, made, coll, span)
		}
		("Extend", "extend") | ("From", "from") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			let target = match owner {
				"From" => deque_new(b, coll, span),
				_ => values.pop()?,
			};
			extend(b, coll, library::targ(call.args, 1), target, source, iter, span)?
		}
		_ => return None,
	};
	Some(e)
}

/// An empty `LinkedList` of type `coll`.
fn list_new<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Expr {
	empty(b, Expr::rs("listNew", vec![]), coll, span)
}

/// A method of `LinkedList`, or of a trait it implements. Its node is the place of the
/// item it holds, which `_mut` methods lend of items JS copies.
fn list<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let item = arg(coll, 0);
	let places = b.t.needs_place(item);
	let mut values = args;
	let none = library::sentinel_option(b.t, call).is_some();
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => list_new(b, coll, span),
		(_, "len") => values.pop()?.member("n"),
		(_, "is_empty") => values.pop()?.member("n").binary("===", Expr::lit("0")),
		(_, "clear") => Expr::rs("listClear", values),
		(_, "push_back" | "push_back_mut") => Expr::rs("listPushBack", values),
		(_, "push_front" | "push_front_mut") => Expr::rs("listPushFront", values),
		(_, "pop_back") => Expr::rs("listPopBack", values),
		(_, "pop_front") => Expr::rs("listPopFront", values),
		(_, "front_mut") if places => Expr::rs("listEndMut", values),
		(_, "back_mut") if places => Expr::rs("listEndMut", flagged(values, true, false)),
		(_, "front" | "front_mut") => Expr::rs("listEnd", flagged(values, false, none)),
		(_, "back" | "back_mut") => Expr::rs("listEnd", flagged(values, true, none)),
		(_, "contains") => {
			values.push(eq_fn(b, item, span));
			Expr::rs("listContains", values)
		}
		(_, "iter_mut") if places => Expr::rs("listIterMut", values),
		(_, "iter" | "iter_mut") => Expr::rs("listIter", values),
		(_, "append") => Expr::rs("listAppend", values),
		(_, "split_off") => Expr::rs("listSplitOff", values),
		(_, "extract_if") if places => Expr::rs("listExtractIfMut", values),
		(_, "extract_if") => Expr::rs("listExtractIf", values),
		("Clone", "clone") => {
			values.push(clone_fn(b, item, span));
			Expr::rs("listClone", values)
		}
		// Each item by its `clone_from`.
		("Clone", "clone_from") => {
			let fns = library::clone_from_fns(b, item, span);
			Expr::rs("listCloneFrom", [values, fns].concat())
		}
		// std's specializes `extend` by another list to `append`.
		("Extend", "extend") if library::targ(call.args, 2) == coll => Expr::rs("listAppend", values),
		("Extend", "extend") | ("From", "from") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			let target = match owner {
				"From" => list_new(b, coll, span),
				_ => values.pop()?,
			};
			extend(b, coll, library::targ(call.args, 1), target, source, iter, span)?
		}
		_ => return None,
	};
	Some(match name {
		// What a `_mut` method of an item JS doesn't copy lends is the item itself.
		"push_back_mut" | "push_front_mut" if !places => e.member("v"),
		_ => e,
	})
}

/// `PartialOrd`'s tests of `item` that std's heap sifts make: `le` up, and `le`, `ge`, and
/// `lt` down.
fn heap_tests<'tcx>(b: &mut Body<'_, '_, 'tcx>, item: Ty<'tcx>, down: bool, span: Span) -> Vec<Expr> {
	let tests: &[&str] = if down { &["le", "ge", "lt"] } else { &["le"] };
	tests.iter().map(|test| test_fn(b, item, test, span)).collect()
}

/// A method of `BinaryHeap`, or of a trait it implements, each that sifts given the tests
/// its sifts make.
fn heap<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let item = arg(coll, 0);
	let bytes = super::byte(item);
	let mut values = args;
	let e = match (owner, name) {
		(_, "new") | ("Default", "default") => empty(b, Expr::rs("heapNew", vec![]), coll, span),
		// Its vector's, given the size of its elements, by which std's grows it.
		(_, "with_capacity") => {
			values.push(super::iter::size_of(b, item, span));
			empty(b, Expr::rs("heapWithCapacity", values), coll, span)
		}
		(
			_,
			"capacity" | "reserve" | "reserve_exact" | "try_reserve" | "try_reserve_exact" | "shrink_to"
			| "shrink_to_fit",
		) => {
			if name == "shrink_to_fit" {
				values.push(Expr::lit("0"));
			}
			values.push(super::iter::size_of(b, item, span));
			let f = match name {
				"capacity" => "vecCapacity",
				"reserve" => "vecReserve",
				"reserve_exact" => "vecReserveExact",
				"try_reserve" => "vecTryReserve",
				"try_reserve_exact" => "vecTryReserveExact",
				_ => "vecShrinkTo",
			};
			Expr::rs(f, values)
		}
		(_, "len") => values.pop()?.member("items").member("length"),
		(_, "is_empty") => values
			.pop()?
			.member("items")
			.member("length")
			.binary("===", Expr::lit("0")),
		// Indexing returns `undefined` for no element, where an `rs.None` must follow.
		(_, "peek") if library::sentinel_option(b.t, call).is_some() => Expr::rs("heapPeek", values),
		(_, "peek") => values.pop()?.member("items").index(Expr::lit("0")),
		(_, "push" | "pop" | "peek_mut" | "append" | "retain" | "into_sorted_vec") => {
			values.extend(heap_tests(b, item, !matches!(name, "push" | "pop"), span));
			let f = match name {
				"push" => "heapPush",
				"pop" => "heapPop",
				"peek_mut" => "heapPeekMut",
				"append" => "heapAppend",
				"retain" => "heapRetain",
				_ => return Some(heap_vec(Expr::rs("heapIntoSortedVec", values), bytes)),
			};
			if name == "append" {
				values.push(super::iter::size_of(b, item, span));
			}
			Expr::rs(f, values)
		}
		(_, "into_vec") => heap_vec(values.pop()?, bytes),
		(_, "as_slice") if bytes => Expr::rs(
			"byteSliceFrom",
			vec![
				Expr::ident("Uint8Array")
					.member("from")
					.call(vec![values.pop()?.member("items")]),
			],
		),
		(_, "as_slice") if item.is_array() => super::slice_items_from(values.pop()?.member("items"), item),
		(_, "as_slice") => values.pop()?.member("items"),
		(_, "clear") => Expr::rs("heapClear", values),
		(_, "drain") => {
			let droppers = entry_droppers(b, coll, span);
			Expr::rs("heapDrain", super::iter::tail(values, droppers))
		}
		(_, "iter") => Expr::rs("heapIter", values),
		("Clone", "clone") => {
			values.push(clone_fn(b, item, span));
			Expr::rs("heapClone", values)
		}
		// `From<Vec<T>>` rebuilds the vector in place, and `From<[T; N]>` collects the array
		// into one first.
		("From", "from") if b.t.std_of(library::targ(call.args, 1)) == Some(Std::Vec) => {
			let v = values.pop()?;
			let d = match bytes {
				true => Expr::rs("byteVecToVec", vec![v]),
				false => v,
			};
			Expr::rs("heapFrom", [vec![d], heap_tests(b, item, true, span)].concat())
		}
		("Extend", "extend") | ("From", "from") => {
			let (source, iter) = extend_source(b, coll, call, owner, values.pop()?)?;
			if owner == "From" {
				let trusted = super::iter::implements(b.t, iter, "TrustedLen");
				let d = super::iter::to_vec(b, source, iter, item, trusted, span);
				return Some(Expr::rs(
					"heapFrom",
					[vec![d], heap_tests(b, item, true, span)].concat(),
				));
			}
			extend(b, coll, library::targ(call.args, 1), values.pop()?, source, iter, span)?
		}
		_ => return None,
	};
	Some(e)
}

/// The `Vec` a heap, `h`, is, an `rs.ByteVec` of bytes, with its capacity.
pub fn heap_vec(h: Expr, bytes: bool) -> Expr {
	match bytes {
		true => Expr::rs("byteVecFromVec", vec![h]),
		false => h,
	}
}

/// What `extend` or `from` adds to a collection: an `rs.Iter` of what `source` holds, and
/// the type of the iterator std's makes of it.
fn extend_source<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	source: Expr,
) -> Option<(Expr, Ty<'tcx>)> {
	// `Extend<A>::extend::<T>` and `From<T>::from`.
	let ty = library::targ(call.args, if owner == "Extend" { 2 } else { 1 });
	let iter = super::iter::iter_of(b.t, ty)?;
	let Some(items) = super::iter::into_iter(b, ty, source, call.span) else {
		return Some((
			b.err(call.span, format!("`{owner}` from `{ty}`, which is not supported yet")),
			iter,
		));
	};
	// `Extend<&T>` and `Extend<(&K, &V)>`, whose items are `Copy`, copy each one.
	let elems: Vec<Ty<'tcx>> = match b.t.std_of(coll) {
		Some(Std::HashMap | Std::BTreeMap) => vec![arg(coll, 0), arg(coll, 1)],
		_ => vec![arg(coll, 0)],
	};
	let stored = match elems.as_slice() {
		[k, v] => Ty::new_tup(b.t.tcx, &[*k, *v]),
		_ => elems[0],
	};
	let borrowed = owner == "Extend" && library::targ(call.args, 1) != stored;
	// A `[u8; N]` is given as the `rs.ByteSlice` a `&[u8; N]` is, so it is made an array
	// whether or not the crate writes into one.
	if !borrowed || !elems.iter().any(|&t| b.t.needs_copy(t) || super::byte_array(t)) {
		return Some((items, iter));
	}
	let items = match super::lend::copy_fn(b, stored, call.span) {
		Some(copy) => Expr::rs("iterMap", vec![items, copy]),
		None => items,
	};
	Some((items, iter))
}

/// `extend` of `target`, a collection of type `coll`, `()`, or a tuple of them, by `items`,
/// an iterator of type `iter` of `item`s, copied already where they are borrowed, as its
/// own `Extend` reads and reserves for it, giving the collection back.
pub fn extend<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	item: Ty<'tcx>,
	target: Expr,
	items: Expr,
	iter: Ty<'tcx>,
	span: Span,
) -> Option<Expr> {
	// `()`'s reads each item by `for_each`, and a tuple's is its own.
	if coll.is_unit() {
		let drops = super::iter::iter_drops(b, iter, span);
		let each = Expr::Lambda(Vec::new(), Box::new(Expr::undefined()));
		return Some(Expr::rs(
			"iterForEach",
			super::iter::tail(vec![items, each], vec![drops]),
		));
	}
	if let ty::Tuple(_) = coll.kind() {
		return tuple_extend(b, coll, item, Some(target), items, iter, span);
	}
	let std = b.t.std_of(coll)?;
	if std == Std::String {
		return super::iter::string_extend(b, target, items, item, iter, span);
	}
	// Of a `TrustedLen` iterator into a vector, by `for_each`, reserving its upper bound first.
	let trusted = super::iter::implements(b.t, iter, "TrustedLen");
	let vec_reserve = |b: &mut Body<'_, '_, 'tcx>| {
		let take = super::iter::iter_take(b, iter, trusted, span);
		vec![
			take,
			super::iter::size_of(b, arg(coll, 0), span),
			Expr::lit(trusted.to_string()),
		]
	};
	Some(match std {
		Std::Vec if super::byte(arg(coll, 0)) => {
			let take = super::iter::iter_take(b, iter, trusted, span);
			Expr::rs(
				"iterExtendBytes",
				super::iter::tail(vec![target, items], vec![take, Expr::lit(trusted.to_string())]),
			)
		}
		Std::Vec => Expr::rs("iterExtend", super::iter::tail(vec![target, items], vec_reserve(b))),
		Std::VecDeque => {
			let take = super::iter::iter_take(b, iter, trusted, span);
			Expr::rs(
				"dequeExtend",
				super::iter::tail(vec![target, items], vec![take, Expr::lit(trusted.to_string())]),
			)
		}
		// By `for_each`, reserving nothing.
		Std::LinkedList => {
			let take = super::iter::iter_take(b, iter, true, span);
			Expr::rs("listExtend", super::iter::tail(vec![target, items], vec![take]))
		}
		// Into its vector, and then sifted.
		Std::BinaryHeap => {
			let mut args = vec![target, items];
			args.extend(heap_tests(b, arg(coll, 0), true, span));
			Expr::rs("heapExtend", super::iter::tail(args, vec_reserve(b)))
		}
		Std::HashMap | Std::HashSet => hash_extend(b, coll, target, items, iter, span),
		Std::BTreeMap | Std::BTreeSet => {
			let mut droppers = vec![Expr::lit("false")];
			droppers.extend(entry_droppers(b, coll, span));
			// By `for_each`.
			droppers.push(super::iter::iter_take(b, iter, true, span));
			let f = if std == Std::BTreeSet {
				"btreeExtendKeys"
			} else {
				"btreeExtend"
			};
			Expr::rs(f, super::iter::tail(vec![target, items], droppers))
		}
		_ => return None,
	})
}

/// What copies each borrowed `item` a collection of type `coll` is extended by, as
/// `Extend<&T>` and `Extend<(&K, &V)>` store a copy: `None` where `item` is what it stores,
/// a copy is the value itself, or `coll` is no collection of items.
pub fn copy_borrowed<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, item: Ty<'tcx>, span: Span) -> Option<Expr> {
	let Some(
		Std::Vec
		| Std::VecDeque
		| Std::LinkedList
		| Std::BinaryHeap
		| Std::HashMap
		| Std::HashSet
		| Std::BTreeMap
		| Std::BTreeSet,
	) = b.t.std_of(coll)
	else {
		return None;
	};
	let stored = stored(b, coll);
	if item == stored {
		return None;
	}
	super::lend::copy_fn(b, stored, span)
}

/// `extend` of `target`, a tuple of collections of type `tuple`, by `items`, an iterator of
/// type `iter` of `item`s, or without one, `collect` into the tuple of the collections'
/// defaults, as std's `Extend` and `FromIterator` of tuples are: a 1-tuple's collection by
/// its own `extend`, and a longer one's each by its `extend_reserve` and `extend_one`.
pub fn tuple_extend<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	tuple: Ty<'tcx>,
	item: Ty<'tcx>,
	target: Option<Expr>,
	items: Expr,
	iter: Ty<'tcx>,
	span: Span,
) -> Option<Expr> {
	let (ty::Tuple(colls), ty::Tuple(fields)) = (tuple.kind(), item.kind()) else {
		return None;
	};
	let unsupported = |b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, field: Ty<'tcx>| {
		b.err(
			span,
			format!("`Extend` of `{tuple}` by `{item}`, whose `{coll}` of `{field}` is not supported yet"),
		)
	};
	let (f, rest) = match (colls.as_slice(), fields.as_slice()) {
		([coll], [field]) => {
			let Some(mapped) = one_items(b, tuple, item, iter, target.is_none()) else {
				return Some(unsupported(b, *coll, *field));
			};
			let (c, each) = (b.fresh("c"), b.fresh("items"));
			let mut each_items = Expr::ident(&each);
			if let Some(copy) = copy_borrowed(b, *coll, *field, span) {
				each_items = Expr::rs("iterMap", vec![each_items, copy]);
			}
			let extended = match own_extend(b, *coll, *field, mapped, span) {
				Some((f, place)) => Some(Expr::rs(
					"ownExtend",
					super::iter::tail(vec![Expr::ident(&c), each_items, f], vec![Expr::lit(place.to_string())]),
				)),
				None => extend(b, *coll, *field, Expr::ident(&c), each_items, mapped, span),
			};
			let Some(extended) = extended else {
				return Some(unsupported(b, *coll, *field));
			};
			let params = [c, each].map(|name| Param { name, ty: None });
			let f = Expr::Lambda(params.into(), Box::new(extended));
			("tupleExtendOne", vec![items, f])
		}
		_ => {
			let mut parts = Vec::new();
			for (coll, field) in colls.iter().zip(fields.iter()) {
				match part(b, coll, field, span) {
					Some(part) => parts.push(part),
					None => return Some(unsupported(b, coll, field)),
				}
			}
			// By `for_each`.
			let take = super::iter::iter_take(b, iter, true, span);
			let args = super::iter::tail(vec![items, Expr::Array(parts)], vec![take]);
			("tupleExtend", args)
		}
	};
	let target = match target {
		Some(target) => target,
		None => {
			let defaults = default(b, tuple, span)?;
			empty(b, defaults, tuple, span)
		}
	};
	Some(Expr::rs(f, [vec![target], rest].concat()))
}

/// `Iterator::partition` of `items`, an iterator of `item`s, by `f`, into two defaults of
/// `coll`, as std's makes them, each extended as a tuple's `Extend` extends it: `flags` say
/// whether the iterator drops and whether `f` drops as its fold ends.
pub fn partition<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	item: Ty<'tcx>,
	items: Expr,
	f: Expr,
	flags: Vec<Expr>,
	span: Span,
) -> Option<Expr> {
	let part = part(b, coll, item, span)?;
	let left = default(b, coll, span)?;
	let left = empty(b, left, coll, span);
	let right = default(b, coll, span)?;
	let right = empty(b, right, coll, span);
	Some(Expr::rs(
		"iterPartition",
		super::iter::tail(vec![items, f, Expr::Array(vec![left, right]), part], flags),
	))
}

/// The `rs.Part` a tuple's `Extend` adds `item`s to a collection of type `coll` by, or to a
/// tuple of them, which is one too: `None` for any other.
fn part<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, item: Ty<'tcx>, span: Span) -> Option<Expr> {
	if coll.is_unit() {
		return Some(Expr::Rs("UNIT_PART".into()));
	}
	if let (ty::Tuple(colls), ty::Tuple(items)) = (coll.kind(), item.kind()) {
		let parts = colls.iter().zip(items.iter()).map(|(c, i)| part(b, c, i, span));
		return Some(Expr::rs("tuplePart", vec![Expr::Array(parts.collect::<Option<_>>()?)]));
	}
	// The crate's own, whose `extend_one` is `extend` of `Some(item)`.
	let Some(std) = b.t.std_of(coll) else {
		if overrides_defaults(b.t, coll, item) {
			return None;
		}
		let (f, place) = own_extend(b, coll, item, Ty::new_option(b.t.tcx, item), span)?;
		return Some(Expr::rs(
			"ownPart",
			super::iter::tail(vec![f], vec![Expr::lit(place.to_string())]),
		));
	};
	if std == Std::String {
		let part = match super::iter::text_items(b.t, item)? {
			false => "STRING_CHARS_PART",
			true if library::cow_str(b.t.tcx, item.peel_refs()) => "STRING_COW_PART",
			true => "STRING_PART",
		};
		return Some(Expr::Rs(part.into()));
	}
	let size = |b: &mut Body<'_, '_, 'tcx>| vec![super::iter::size_of(b, arg(coll, 0), span)];
	let part = match std {
		Std::Vec if super::byte(arg(coll, 0)) => Expr::rs("byteVecPart", Vec::new()),
		Std::Vec => Expr::rs("vecPart", super::iter::tail(Vec::new(), size(b))),
		Std::VecDeque => Expr::rs("dequePart", Vec::new()),
		Std::LinkedList => Expr::rs("listPart", Vec::new()),
		Std::BinaryHeap => {
			let le = heap_tests(b, arg(coll, 0), false, span);
			Expr::rs("heapPart", super::iter::tail(le, size(b)))
		}
		Std::HashMap | Std::HashSet => {
			let mut args = entry_droppers(b, coll, span);
			args.push(hash_max(b, coll, span));
			let f = if std == Std::HashSet {
				"hashKeysPart"
			} else {
				"hashPart"
			};
			Expr::rs(f, super::iter::tail(Vec::new(), args))
		}
		Std::BTreeMap | Std::BTreeSet => {
			let droppers = entry_droppers(b, coll, span);
			let f = if std == Std::BTreeSet {
				"btreeKeysPart"
			} else {
				"btreePart"
			};
			Expr::rs(f, super::iter::tail(Vec::new(), droppers))
		}
		_ => return None,
	};
	Some(match copy_borrowed(b, coll, item, span) {
		Some(copy) => Expr::rs("copiedPart", vec![part, copy]),
		None => part,
	})
}

/// The crate's own `Extend::extend` of a collection of type `coll` by an iterator of type
/// `iter` of `item`s, as a function of the collection and the iterator, and whether it
/// takes the collection as an `rs.Place`: `None` for a collection of std's, or one whose
/// `extend` doesn't translate. Stable Rust can't override `extend_reserve` or `extend_one`,
/// which keep their defaults: nothing, and `extend` of `Some(item)`.
fn own_extend<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	item: Ty<'tcx>,
	iter: Ty<'tcx>,
	span: Span,
) -> Option<(Expr, bool)> {
	let tcx = b.t.tcx;
	if !matches!(coll.kind(), ty::Adt(..)) || b.t.std_of(coll).is_some() {
		return None;
	}
	let extend = trait_fn_of(tcx, super::iter::core_trait(tcx, "Extend")?, "extend");
	let args = tcx.mk_args(&[coll.into(), item.into(), iter.into()]);
	if !b.t.translated(extend, args) {
		return None;
	}
	Some((b.fn_value(extend, args, span), b.t.needs_place(coll)))
}

/// Whether `coll`'s own `Extend<item>` overrides what a tuple's `Extend` calls of it, as
/// only a nightly crate can: `extend_one`, `extend_reserve`, or `extend_one_unchecked`.
fn overrides_defaults<'tcx>(t: &Translator<'_, 'tcx>, coll: Ty<'tcx>, item: Ty<'tcx>) -> bool {
	let tcx = t.tcx;
	let Some(extend) = super::iter::core_trait(tcx, "Extend") else {
		return true;
	};
	let args = tcx.mk_args(&[coll.into(), item.into()]);
	["extend_one", "extend_reserve", "extend_one_unchecked"]
		.into_iter()
		.any(|name| {
			let f = trait_fn_of(tcx, extend, name);
			!matches!(Instance::try_resolve(tcx, t.typing_env_pub(), f, args), Ok(Some(i)) if i.def_id() == f)
		})
}

/// The iterator std's `Extend` of a 1-tuple of type `tuple`, or its `FromIterator` if
/// `collect`, gives the collection's own `extend`, from an iterator of type `iter` of
/// `item`s: a `Map` of it to each item's field, typed as std's body makes it.
fn one_items<'tcx>(
	b: &Body<'_, '_, 'tcx>,
	tuple: Ty<'tcx>,
	item: Ty<'tcx>,
	iter: Ty<'tcx>,
	collect: bool,
) -> Option<Ty<'tcx>> {
	let tcx = b.t.tcx;
	let extend = trait_fn_of(tcx, super::iter::core_trait(tcx, "Extend")?, "extend");
	let by = match collect {
		true => trait_fn(tcx, "FromIterator", "from_iter"),
		false => extend,
	};
	let args = tcx.mk_args(&[tuple.into(), item.into(), iter.into()]);
	let args = library::callee_args(b.t, by, args, |f| f == extend)?;
	Some(args.type_at(2))
}

/// `Default::default` of a collection of type `coll` that a tuple's `Extend` takes, or of a
/// tuple of them.
fn default<'tcx>(b: &mut Body<'_, '_, 'tcx>, coll: Ty<'tcx>, span: Span) -> Option<Expr> {
	if coll.is_unit() {
		return Some(Expr::undefined());
	}
	if let ty::Tuple(colls) = coll.kind() {
		let defaults = colls.iter().map(|c| default(b, c, span));
		return Some(Expr::Array(defaults.collect::<Option<_>>()?));
	}
	let Some(std) = b.t.std_of(coll) else {
		let tcx = b.t.tcx;
		let default = trait_fn(tcx, "Default", "default");
		let args = tcx.mk_args(&[coll.into()]);
		return b
			.t
			.translated(default, args)
			.then(|| b.fn_value(default, args, span).call(Vec::new()));
	};
	Some(match std {
		Std::String => Expr::rs("stringNew", Vec::new()),
		Std::Vec if super::byte(arg(coll, 0)) => Expr::New(Box::new(Expr::Rs("ByteVec".into())), Vec::new()),
		Std::Vec => library::vec_new(false),
		Std::VecDeque => deque_new(b, coll, span),
		Std::LinkedList => list_new(b, coll, span),
		Std::BinaryHeap => empty(b, Expr::rs("heapNew", Vec::new()), coll, span),
		Std::HashMap | Std::HashSet => {
			let state = new_state(b, coll, span);
			hash_new(b, coll, state, span)?
		}
		Std::BTreeMap | Std::BTreeSet => btree_new(b, coll, span),
		_ => return None,
	})
}

/// A method of an `Entry`, `OccupiedEntry`, or `VacantEntry`.
fn entry<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	coll: Ty<'tcx>,
	call: &Call<'tcx>,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let value = arg(coll, 1);
	let mut values = args;
	// What these return a `&mut` to is a place where JS would copy it.
	let place = b.t.needs_place(value);
	let mutable = |f: &str| match place {
		true => format!("{f}Mut"),
		false => f.to_string(),
	};
	let e = match name {
		"or_insert" => Expr::rs(mutable("entryOrInsert"), values),
		// A vacant entry's.
		"insert" if is_vacant(b, coll) => Expr::rs(mutable("entryOrInsert"), values),
		"insert" => Expr::rs("entryInsert", values),
		"or_insert_with" | "or_insert_with_key" => {
			values.extend(uncalled(b, call)?);
			Expr::rs(mutable("entryOrInsertWith"), values)
		}
		"or_default" => {
			let tcx = b.t.tcx;
			let default = trait_fn(tcx, "Default", "default");
			values.push(b.fn_value(default, tcx.mk_args(&[value.into()]), span));
			Expr::rs(mutable("entryOrInsertWith"), values)
		}
		"and_modify" => {
			values.extend(uncalled(b, call)?);
			Expr::rs(mutable("entryAndModify"), values)
		}
		"key" => Expr::rs("entryKey", values),
		"into_key" => values.pop()?.member("k"),
		"get" => Expr::rs("entryGet", values),
		"get_mut" | "into_mut" => Expr::rs(mutable("entryGet"), values),
		"remove" => Expr::rs("entryRemove", values),
		"remove_entry" => Expr::rs("entryRemoveEntry", values),
		_ => return None,
	};
	Some(e)
}

/// What drops the closure an entry's `or_insert_with`, `or_insert_with_key`, or `and_modify`
/// takes last, which std's drops uncalled unless the entry is vacant or occupied as it needs.
fn uncalled<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> Option<Vec<Expr>> {
	b.t.droppers(call.args.types().last(), call.span)
}

/// Whether `entry`, an `Entry` of a map or, as `std` says, of a `HashTable`, is its variant
/// `index`: `Occupied` while the map holds its key. Each variant's entry is the `Entry` itself.
pub fn entry_test(std: Std, adt: ty::AdtDef<'_>, index: rustc_abi::VariantIdx, entry: Expr) -> Expr {
	let f = if std == Std::TableEntry {
		"tableOccupied"
	} else {
		"entryOccupied"
	};
	let occupied = Expr::rs(f, vec![entry]);
	match adt.variant(index).name.as_str() {
		"Occupied" => occupied,
		_ => occupied.not(),
	}
}

/// Whether `entry` is a `VacantEntry`, whose `insert` inserts and returns the value.
fn is_vacant<'tcx>(b: &Body<'_, '_, 'tcx>, entry: Ty<'tcx>) -> bool {
	matches!(entry.kind(), ty::Adt(adt, _) if b.t.tcx.item_name(adt.did()).as_str() == "VacantEntry")
}

/// An `Ordering` variant, which is its discriminant: `Less` is `-1`, `Equal` `0`, and
/// `Greater` `1`.
pub fn ordering_variant(index: rustc_abi::VariantIdx) -> Expr {
	Expr::lit((index.as_u32() as i32 - 1).to_string())
}

/// `Ordering`'s methods, on its discriminant.
fn ordering(name: &str, mut values: Vec<Expr>) -> Option<Expr> {
	let zero = || Expr::lit("0");
	let e = match name {
		"is_eq" => values.pop()?.binary("===", zero()),
		"is_ne" => values.pop()?.binary("!==", zero()),
		"is_lt" => values.pop()?.binary("<", zero()),
		"is_gt" => values.pop()?.binary(">", zero()),
		"is_le" => values.pop()?.binary("<=", zero()),
		"is_ge" => values.pop()?.binary(">=", zero()),
		"reverse" => Expr::rs("orderingReverse", values),
		"then" => Expr::rs("orderingThen", values),
		"then_with" => {
			let [o, f] = <[Expr; 2]>::try_from(values).ok()?;
			Expr::rs("orderingThen", vec![o, f, Expr::lit("true")])
		}
		_ => return None,
	};
	Some(e)
}

/// A function of `RandomState`, or of a trait it implements: making one, which `Default`
/// does too, its `Clone`, which is itself, and building its hasher, or hashing one value
/// with it as `BuildHasher::hash_one` does, dropping the value.
fn random_state<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	ty: Ty<'tcx>,
	name: &str,
	args: Vec<Expr>,
) -> Expr {
	let span = call.span;
	let mut values = args.into_iter();
	match (name, values.next()) {
		("new" | "default", _) => Expr::rs("randomState", vec![]),
		("clone", Some(state)) => state,
		("build_hasher", Some(state)) => Expr::rs("buildHasher", vec![state]),
		// It takes the value, which drops once hashed.
		("hash_one", Some(state)) if let Some(value) = values.next() => {
			let hasher = built_hasher(b, ty).expect("a RandomState builds a DefaultHasher");
			let value_ty = library::targ(call.args, 1);
			let built = Expr::rs("buildHasher", vec![state]);
			let mut args = vec![built, value, hash_fn(b, value_ty, hasher, span)];
			args.extend(b.t.drop_fn(value_ty, span));
			Expr::rs("hashOne", args)
		}
		_ => b.err(span, format!("`RandomState::{name}`, which is not supported yet")),
	}
}

/// A function of `BuildHasherDefault`, or of a trait it implements, which holds nothing:
/// making one, which `Default` and `Clone` do too, building its hasher by the hasher's own
/// `Default`, or hashing one value with it as `BuildHasher::hash_one` does, dropping the
/// value.
fn build_hasher_default<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	ty: Ty<'tcx>,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Expr {
	let (tcx, span) = (b.t.tcx, call.span);
	let ty::Adt(_, gargs) = ty.kind() else {
		unreachable!("a BuildHasherDefault")
	};
	let hasher = gargs.type_at(0);
	match name {
		"new" | "default" | "clone" => Expr::undefined(),
		"build_hasher" => {
			let default = trait_fn(tcx, "Default", "default");
			b.call_def(default, tcx.mk_args(&[hasher.into()]), Vec::new(), span, out)
		}
		"hash_one" if own_hasher(b.t, hasher) => own_hash_one(b, call, ty, hasher, args, out),
		"hash_one" if b.t.std_of(hasher) == Some(Std::Hasher) => {
			let Ok([_, value]) = <[Expr; 2]>::try_from(args) else {
				unreachable!("hash_one takes a state and a value")
			};
			let value_ty = library::targ(call.args, 1);
			let mut args = vec![Expr::rs("hasher", vec![]), value, hash_fn(b, value_ty, hasher, span)];
			args.extend(b.t.drop_fn(value_ty, span));
			Expr::rs("hashOne", args)
		}
		_ => b.err(
			span,
			format!("`BuildHasherDefault<{hasher}>::{name}`, which is not supported yet"),
		),
	}
}

/// `Hasher`'s methods: writing an integer or a string, and `finish`.
fn hasher_call(b: &mut Body<'_, '_, '_>, name: &str, args: Vec<Expr>, span: Span, out: &mut Vec<Stmt>) -> Expr {
	let mut values = args;
	// The runtime takes the value first, so a hasher that is not a path evaluates first.
	if let [state, value] = values.as_mut_slice()
		&& !state.is_path()
		&& !value.is_constant()
	{
		let saved = std::mem::replace(state, Expr::undefined());
		*state = b.spill(saved, "state", out);
	}
	let swapped = |values: Vec<Expr>| values.into_iter().rev().collect::<Vec<_>>();
	let f = match name {
		"finish" => return Expr::rs("hashFinish", values),
		"write" => "hashWrite",
		"write_str" => "hashStr",
		"write_length_prefix" => "hashPrefix",
		_ if let Some(f) = name.strip_prefix("write_").and_then(hash_int) => f,
		_ => return b.err(span, format!("`Hasher::{name}`, which is not supported yet")),
	};
	Expr::rs(f, swapped(values))
}

/// The runtime's `Hash::hash` of the integer type `rs` names, which calls its `Hasher`
/// method: `hash16` of a `u16`, `hashI16` of an `i16`.
fn hash_int(rs: &str) -> Option<&'static str> {
	Some(match rs {
		"u8" => "hash8",
		"u16" => "hash16",
		"u32" => "hash32",
		"u64" => "hash64",
		"u128" => "hash128",
		"usize" => "hashUsize",
		"i8" => "hashI8",
		"i16" => "hashI16",
		"i32" => "hashI32",
		"i64" => "hashI64",
		"i128" => "hashI128",
		"isize" => "hashIsize",
		_ => return None,
	})
}

/// The discriminant of `value`, an enum of the crate, as `discriminant_value` reads it.
fn discriminant<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let ty = ty.peel_refs();
	let ty::Adt(adt, _) = ty.kind() else {
		return b.err(span, format!("the discriminant of `{ty}`"));
	};
	if !adt.is_enum() || !b.t.own(adt.did()) {
		return b.err(span, format!("the discriminant of `{ty}`"));
	}
	let info = b.t.adt(*adt, span);
	let Some(int) = Int::of(ty.discriminant_ty(tcx)) else {
		return b.err(span, format!("the discriminant of `{ty}`"));
	};
	let table = adt
		.discriminants(tcx)
		.map(|(i, d)| {
			(
				crate::ts::Key::Name(adt.variant(i).name.to_string()),
				int.lit(int.fold(d.val as i128)),
			)
		})
		.collect();
	let tag = match info.repr {
		super::AdtRepr::Tags => value,
		_ => value.member("tag"),
	};
	Expr::Index(Box::new(Expr::Object(table)), Box::new(tag))
}

/// Whether the runtime compares and hashes values of `ty`, a std type, as its `Ord` and
/// `Hash` do, through what it holds.
fn std_key<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	// What a std type holds may be the crate's, which compares and hashes by its own impls.
	let held = |t: Ty<'tcx>| {
		let t = peel(b, t);
		std_key(b, t) || b.t.deferred(t) || local(b, t)
	};
	match ty.kind() {
		// A float is no key, but its `PartialEq` and `PartialOrd` are the runtime's.
		ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) | ty::Str => true,
		_ if library::nothing(b.t.tcx, ty) => true,
		// Which compares by where it points, and orders and hashes by none.
		_ if raw_pointee(b.t.tcx, ty).is_some() => true,
		ty::Tuple(tys) => tys.iter().all(held),
		_ if let Some(elem) = elements_of(b, ty) => held(elem),
		_ if bytes(b, ty) => true,
		ty::Adt(..)
			if let Some(inner) = option_of(b, ty)
				.or_else(|| arc_of(b, ty))
				.or_else(|| reverse_of(b, ty))
				.or_else(|| overflowing_of(b, ty))
				.or_else(|| cow_of(b, ty))
				.or_else(|| deque_of(b, ty))
				.or_else(|| list_of(b, ty)) =>
		{
			held(inner)
		}
		ty::Adt(..) if let Some(bound) = range_of(b, ty) => held(bound),
		ty::Adt(..) if let Some((inner, _)) = cell_of(b, ty) => held(inner),
		ty::Adt(_, args) if derived_std(b, ty) => args.types().all(held),
		ty::Adt(..) if b.t.std_of(ty).is_some_and(Std::record) => true,
		ty::Adt(..) => {
			library::string(b.t.tcx, ty)
				|| library::owned_string(b.t.tcx, ty)
				|| matches!(
					b.t.std_of(ty),
					Some(Std::Ordering | Std::Duration | Std::TypeId | Std::Location | Std::TryReserveError)
				)
		}
		_ => false,
	}
}

/// Whether `ty` is a `Result` or a `Poll`, whose derived `PartialEq`, `PartialOrd`, and `Ord`
/// lower from their MIR.
fn derived_std<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	matches!(b.t.std_of(ty), Some(Std::Result | Std::Poll))
}

/// What `ty` holds if it is an `Option`.
fn option_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	(b.t.std_of(ty) == Some(Std::Option)).then(|| arg(ty, 0))
}

/// What `ty` holds if it is a `VecDeque`.
fn deque_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	(b.t.std_of(ty) == Some(Std::VecDeque)).then(|| arg(ty, 0))
}

/// What `ty` holds if it is a `LinkedList`.
fn list_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	(b.t.std_of(ty) == Some(Std::LinkedList)).then(|| arg(ty, 0))
}

/// What `ty`'s bounds are if it is one of std's ranges that holds any.
fn range_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let range = matches!(
		b.t.std_of(ty),
		Some(Std::Range | Std::RangeInclusive | Std::RangeFrom | Std::RangeTo | Std::RangeToInclusive)
	);
	range.then(|| arg(ty, 0))
}

/// What `ty` holds if it is a `Reverse`.
fn reverse_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	(b.t.std_of(ty) == Some(Std::Reverse)).then(|| arg(ty, 0))
}

/// What `ty` holds if it is a `Wrapping` or a `Saturating`, which compares, orders, and
/// hashes as it, but for its `ne`, which is `!eq`.
fn overflowing_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	b.t.std_of(ty).and_then(Std::overflow).map(|_| arg(ty, 0))
}

/// What `ty` borrows if it is a `Cow` other than of bytes, a `[T]` or a `str`, which it
/// compares, orders, and hashes as. A `Cow<[u8]>` is bytes already.
fn cow_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	library::cow_borrowed(b.t.tcx, ty).filter(|t| !bytes(b, *t))
}

/// `ty`, the type of `args[n]`, as what it derefs to if it is a `Cow` other than of bytes,
/// whose value in `args` becomes that.
fn cow_deref<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, args: &mut [Expr], n: usize) -> Ty<'tcx> {
	let Some(borrowed) = cow_of(b, peel(b, ty)) else {
		return ty;
	};
	if let Some(at) = args.get_mut(n) {
		*at = library::cow_deref(std::mem::replace(at, Expr::undefined()), borrowed);
	}
	borrowed
}

/// The elements of `ty` if it is a `Vec`, a slice, or an array of any type but `u8`, whose
/// bytes are typed arrays: an `rs.Vec` or a JS array, which the runtime reads alike.
fn elements_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let elem = match ty.kind() {
		ty::Slice(elem) | ty::Array(elem, _) => *elem,
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Vec) => arg(ty, 0),
		_ => return None,
	};
	(!super::byte(elem)).then_some(elem)
}

/// Whether `ty` is a `Vec`, a slice, an array, or a `Cow` of bytes: a typed array, an
/// `rs.ByteSlice` behind a reference, or an `rs.ByteVec`, or either of the last two.
fn bytes<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let elem = match ty.kind() {
		ty::Slice(elem) | ty::Array(elem, _) => *elem,
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Vec) => arg(ty, 0),
		_ if let Some(elem) = library::cow_slice(b.t.tcx, ty) => elem,
		_ => return false,
	};
	super::byte(elem)
}

/// What `bounds` makes of the range a call takes: what an `rs.Bounds` reads, and, for a
/// range that drops, the range and what drops it.
pub struct Ranged {
	pub bounds: Expr,
	dropped: Option<(Expr, String, Expr)>,
}

impl Ranged {
	/// `e`, the call, which drops a range that drops once it returns: std drops it once it
	/// has read its bounds and searched by them, after which it does nothing JS can see
	/// before it returns.
	pub fn after(self, e: Expr) -> Expr {
		let Some((range, name, drop)) = self.dropped else {
			return e;
		};
		let call = Expr::Lambda(vec![Param { name, ty: None }], Box::new(e));
		Expr::rs("dropAfter", vec![range, call, drop])
	}

	/// `call` of the bounds and what drops the range, for a call whose result holds the
	/// range and drops it as it drops: a range that drops, read once, and else `undefined`.
	pub fn held(self, call: impl FnOnce(Expr, Expr) -> Expr) -> Expr {
		let Some((range, name, drop)) = self.dropped else {
			return call(self.bounds, Expr::undefined());
		};
		let dropper = Expr::Lambda(Vec::new(), Box::new(drop.call(vec![Expr::ident(&name)])));
		let e = call(self.bounds, dropper);
		Expr::Lambda(vec![Param { name, ty: None }], Box::new(e)).call(vec![range])
	}

	/// The bounds, for a call that checks more once `slice::range` has checked them within
	/// `len`, as `String`'s do its char boundaries: a range that drops drops between the two,
	/// as std's does, and the call takes the checked `{ start, end }`.
	pub fn within(self, len: Expr) -> Expr {
		let Some((range, name, drop)) = self.dropped else {
			return self.bounds;
		};
		let checked = Expr::rs("boundsChecked", vec![self.bounds, len]);
		let call = Expr::Lambda(vec![Param { name, ty: None }], Box::new(checked));
		Expr::rs("dropAfter", vec![range, call, drop])
	}
}

/// Makes `range`, the range `call` takes, what an `rs.Bounds` reads: std's range types
/// and a pair of `Bound`s are already, and any other `RangeBounds` becomes `rs.boundsOf` of
/// it and its `start_bound` and `end_bound`, which the runtime calls where std's does.
pub fn bounds<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, range: Expr) -> Ranged {
	let tcx = b.t.tcx;
	let span = call.span;
	let (input, target) = range_input(b, call);
	let ty = b.t.sig(call.def_id, call.args).inputs()[input];
	// One that drops is the parameter of what `Ranged::after` calls, so it is read once.
	let (range, dropped) = match b.t.drop_fn(ty, span) {
		Some(drop) => {
			let name = b.fresh("range");
			(Expr::ident(&name), Some((range, name, drop)))
		}
		None => (range, None),
	};
	// Each instance decides for a range it is generic over.
	let bounds = if plain_bounds(b.t, ty) || b.t.deferred(ty) {
		range
	} else {
		let args = tcx.mk_args(&[ty.into(), target.into()]);
		let start = b.fn_value(trait_fn(tcx, "RangeBounds", "start_bound"), args, span);
		let end = b.fn_value(trait_fn(tcx, "RangeBounds", "end_bound"), args, span);
		Expr::rs("boundsOf", vec![range, start, end])
	};
	Ranged { bounds, dropped }
}

/// Whether an `rs.Bounds` reads `ty` as it is: std's range types and a pair of `Bound`s.
pub fn plain_bounds<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> bool {
	match (ty.kind(), t.std_of(ty)) {
		(
			_,
			Some(
				Std::Range
				| Std::RangeInclusive
				| Std::RangeFrom
				| Std::RangeTo
				| Std::RangeToInclusive
				| Std::RangeFull,
			),
		) => true,
		(ty::Tuple(tys), _) => tys.len() == 2 && tys.iter().all(|b| t.std_of(b) == Some(Std::Bound)),
		_ => false,
	}
}

/// Which of its inputs is the range `call` takes, and the `T` of the `RangeBounds<T>` it
/// implements, from the function's own bound on its range's type.
fn range_input<'tcx>(b: &Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> (usize, Ty<'tcx>) {
	let tcx = b.t.tcx;
	let bounds = tcx.parent(trait_fn(tcx, "RangeBounds", "start_bound"));
	let sig = tcx
		.fn_sig(call.def_id)
		.instantiate_identity()
		.skip_norm_wip()
		.skip_binder();
	let (input, target) = tcx
		.param_env(call.def_id)
		.caller_bounds()
		.find_map(|clause| {
			let tr = clause.as_trait_clause()?.skip_binder().trait_ref;
			let input = sig.inputs().iter().position(|&ty| ty == tr.self_ty())?;
			(tr.def_id == bounds).then(|| (input, tr.args.type_at(1)))
		})
		.expect("a range's bound");
	(input, b.t.subst(call.args, target))
}

/// What `ty` holds if it is an `Arc`.
fn arc_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	(b.t.std_of(ty) == Some(Std::Arc)).then(|| arg(ty, 0))
}

/// What one of std's cells of type `ty` compares by, and the runtime's function that reads
/// it from each: a `Cell`'s and a `RefCell`'s value, which a `RefCell`'s borrows, or a
/// one-time cell's `Option`, which only `PartialEq` compares.
fn cell_of<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<(Ty<'tcx>, &'static str)> {
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	match b.t.std_of(ty)? {
		Std::Cell => Some((args.type_at(0), "cellBy")),
		// Not an `RwLock`, which compares by nothing.
		Std::RefCell if b.t.tcx.is_diagnostic_item(rustc_span::sym::RefCell, adt.did()) => {
			Some((args.type_at(0), "refCellBy"))
		}
		Std::OnceCell | Std::OnceLock => Some((Ty::new_option(b.t.tcx, args.type_at(0)), "cellBy")),
		_ => None,
	}
}

/// The runtime's `by` of `f`, as [`cell_of`] says a cell compares, for its `method`: given
/// where a `RefCell`'s borrows, in a program with a panic hook.
/// A `Cell`'s values are compared as copies, as its `get` gives them, since a comparison
/// may write the cell, and a write lands in an object it holds.
fn cell_by<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, by: &str, f: Expr, method: &str, span: Span) -> Expr {
	let mut values = vec![f];
	if by == "refCellBy" && b.t.hooked {
		values.extend(super::caller::cell_sites(b.t.tcx, method));
	}
	if let ty::Adt(_, args) = ty.kind()
		&& b.t.std_of(ty) == Some(Std::Cell)
	{
		values.extend(b.t.copy_fn(args.type_at(0), span));
	}
	Expr::rs(by, values)
}

/// `Ord::cmp` of `ty` as a function, or `PartialOrd::partial_cmp` if `partial`: the
/// runtime's for a std type, which calls the same of what it holds, or the crate's, of its
/// type or a reference to it.
pub fn cmp_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, partial: bool, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let ty = peel_ord(b, ty, partial);
	match ty.kind() {
		ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) => Expr::Rs("cmp".into()),
		// `None` for a NaN: only `PartialOrd`, since a float is no `Ord`.
		_ if partial && let Some(float) = super::float::Float::of(ty) => Expr::rs_of(float.name(), "partialCmp"),
		ty::Str => Expr::Rs("cmpStr".into()),
		ty::Adt(..) if library::string(tcx, ty) || library::owned_string(tcx, ty) => Expr::Rs("cmpStr".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Ordering) => Expr::Rs("cmp".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Duration) => Expr::Rs("durationCmp".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::TypeId) => Expr::Rs("typeIdCmp".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Location) => Expr::Rs("locationCmp".into()),
		_ if library::nothing(tcx, ty) => Expr::Rs("cmpNothing".into()),
		ty::Tuple(tys) => {
			let fns = tys.iter().map(|t| cmp_fn(b, t, partial, span)).collect();
			// What the tuple holds and what it orders as, which `tsc` cannot infer from
			// fields of which some are generic.
			let ordering = Type::Rs("Ordering", vec![]);
			let order = match partial {
				true => Type::Union(vec![ordering, Type::Builtin("undefined")]),
				false => ordering,
			};
			let cmp = Expr::Instantiate(Box::new(Expr::Rs("cmpTuple".into())), vec![b.t.ty(ty, span), order]);
			Expr::Call(Box::new(cmp), vec![Expr::Array(fns)])
		}
		ty::Adt(..) if let Some(inner) = option_of(b, ty) => {
			let cmp = cmp_fn(b, inner, partial, span);
			Expr::rs("cmpOption", [cmp].into_iter().chain(b.t.none_arg(inner)).collect())
		}
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => Expr::rs("cmpArc", vec![cmp_fn(b, inner, partial, span)]),
		ty::Adt(..) if let Some((inner, by)) = cell_of(b, ty) => {
			let cmp = cmp_fn(b, inner, partial, span);
			cell_by(b, ty, by, cmp, if partial { "partial_cmp" } else { "cmp" }, span)
		}
		// An `rs.Place` compares by its value, as std's impl on `&mut` does by what it points at.
		ty::Ref(..) if let Some(inner) = b.t.place_pointee(ty) => {
			Expr::rs("cellBy", vec![cmp_fn(b, inner, partial, span)])
		}
		ty::Adt(..) if let Some(inner) = reverse_of(b, ty) => {
			Expr::rs("cmpReverse", vec![cmp_fn(b, inner, partial, span)])
		}
		ty::Adt(..) if let Some(inner) = overflowing_of(b, ty) => cmp_fn(b, inner, partial, span),
		ty::Adt(..) if let Some(borrowed) = cow_of(b, ty) => {
			Expr::rs("cowBy", vec![cmp_fn(b, borrowed, partial, span)])
		}
		_ if let Some(elem) = elements_of(b, ty) => Expr::rs("cmpSlice", vec![cmp_fn(b, elem, partial, span)]),
		// A deque's is its iterator's, which compares each item by its own impls, as a slice's does.
		ty::Adt(..) if let Some(item) = deque_of(b, ty) => Expr::rs("cmpDeque", vec![cmp_fn(b, item, partial, span)]),
		// A list's is its iterator's too.
		ty::Adt(..) if let Some(item) = list_of(b, ty) => Expr::rs("cmpList", vec![cmp_fn(b, item, partial, span)]),
		// Whichever form it has, as the key it is or the operand a reference passes.
		_ if bytes(b, ty) => Expr::Rs("cmpBytes".into()),
		// A `Result`'s or a `Poll`'s derived impls lower from its MIR, by variant and then by
		// what it holds.
		_ if (by_own(b, ty) || derived_std(b, ty)) && partial => {
			let cmp = trait_fn(tcx, "PartialOrd", "partial_cmp");
			b.fn_value(cmp, tcx.mk_args(&[ty.into(), ty.into()]), span)
		}
		_ if by_own(b, ty) || derived_std(b, ty) => {
			let cmp = trait_fn(tcx, "Ord", "cmp");
			b.fn_value(cmp, tcx.mk_args(&[ty.into()]), span)
		}
		_ if raw_pointee(tcx, ty).is_some() => b.err(
			span,
			format!("ordering `{ty}` by its address, which JS has no counterpart of"),
		),
		// Each instance decides what its type parameters are.
		_ if b.t.deferred(ty) => Expr::undefined(),
		_ => b.err(span, format!("ordering `{ty}`, which is not supported yet")),
	}
}

/// `PartialEq::eq` of `ty`, as a function: the runtime's for a std type, or the crate's, of
/// its type or a reference to it.
pub fn eq_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let ty = peel_eq(b, ty);
	match ty.kind() {
		ty::Int(_) | ty::Uint(_)
			if let Some(int) = Int::of(ty)
				&& int.big() =>
		{
			int.func("eq")
		}
		ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) | ty::Str => Expr::Rs("same".into()),
		_ if library::nothing(tcx, ty) => Expr::Rs("same".into()),
		ty::Adt(..)
			if library::string(tcx, ty)
				|| matches!(b.t.std_of(ty), Some(Std::Ordering | Std::ErrorKind | Std::TypeId)) =>
		{
			Expr::Rs("same".into())
		}
		ty::Adt(..) if library::owned_string(tcx, ty) => Expr::Rs("eqStr".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Duration) => Expr::Rs("durationEq".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Location) => Expr::Rs("locationEq".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::TryReserveError) => Expr::Rs("tryReserveErrorEq".into()),
		ty::Tuple(tys) => {
			let fns = tys.iter().map(|t| eq_fn(b, t, span)).collect();
			Expr::rs("eqTuple", vec![Expr::Array(fns)])
		}
		_ if let Some(pointee) = raw_pointee(tcx, ty) => match b.t.incomparable(pointee) {
			Some(why) => b.err(span, why),
			None => b.t.same_fn(pointee).unwrap_or_else(|| Expr::Rs("same".into())),
		},
		ty::Adt(..) if let Some(inner) = option_of(b, ty) => match eq_fn(b, inner, span) {
			// `None` is no value of `inner`, so `===` tells it from each.
			Expr::Rs(same) if same == "same" => Expr::Rs(same),
			eq => Expr::rs("eqOption", [eq].into_iter().chain(b.t.none_arg(inner)).collect()),
		},
		ty::Adt(..) if let Some(inner) = reverse_of(b, ty).or_else(|| overflowing_of(b, ty)) => eq_fn(b, inner, span),
		// An owned `Cow<str>` derefs to its `rs.String`, which `eqStr` reads.
		ty::Adt(..) if library::cow_str(tcx, ty) => Expr::Rs("eqStr".into()),
		ty::Adt(..) if let Some(borrowed) = cow_of(b, ty) => Expr::rs("cowBy", vec![eq_fn(b, borrowed, span)]),
		ty::Adt(..) if let Some(bound) = range_of(b, ty) => Expr::rs("rangeEq", vec![eq_fn(b, bound, span)]),
		// A slice compares its elements by `!=`, as a deque does each of its halves.
		_ if let Some(elem) = elements_of(b, ty) => Expr::rs("eqSlice", vec![not_ne_fn(b, elem, span)]),
		ty::Adt(..) if let Some(item) = deque_of(b, ty) => Expr::rs("eqSeq", vec![not_ne_fn(b, item, span)]),
		// A list's iterator compares its items by `==`.
		ty::Adt(..) if let Some(item) = list_of(b, ty) => Expr::rs("eqList", vec![eq_fn(b, item, span)]),
		// A map compares its values by `==`, and a set holds none.
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::HashMap) => Expr::rs("hashEq", vec![eq_fn(b, arg(ty, 1), span)]),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::HashSet) => Expr::rs("hashEq", vec![]),
		_ if bytes(b, ty) => Expr::Rs("eqBytes".into()),
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => {
			let eq = eq_fn(b, inner, span);
			arc_eq(b, inner, eq)
		}
		// Whose `ne` is `!eq`, which compares what it holds by `eq`.
		ty::Adt(..) if let Some((inner, by)) = cell_of(b, ty) => {
			let eq = eq_fn(b, inner, span);
			cell_by(b, ty, by, eq, "eq", span)
		}
		ty::Ref(..) if let Some(inner) = b.t.place_pointee(ty) => Expr::rs("cellBy", vec![eq_fn(b, inner, span)]),
		_ if by_own(b, ty) || derived_std(b, ty) || b.t.std_of(ty).is_some_and(Std::record) => {
			let eq = trait_fn(tcx, "PartialEq", "eq");
			b.fn_value(eq, tcx.mk_args(&[ty.into(), ty.into()]), span)
		}
		_ if b.t.deferred(ty) => Expr::undefined(),
		_ => b.err(span, format!("comparing `{ty}` as a key, which is not supported yet")),
	}
}

/// `!<ty as PartialEq>::ne`, as a function: `eq_fn`'s, but through the crate's own `ne` of
/// `ty` or of what a tuple or an `Arc` holds, which std's `ne` of each calls.
fn not_ne_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	let ty = peel_eq(b, ty);
	match ty.kind() {
		_ if !own_ne(b, ty) => eq_fn(b, ty, span),
		// `!(a.0 != b.0 || ...)`: each field's `!ne` in turn.
		ty::Tuple(tys) => {
			let fns = tys.iter().map(|t| not_ne_fn(b, t, span)).collect();
			Expr::rs("eqTuple", vec![Expr::Array(fns)])
		}
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => {
			let eq = not_ne_fn(b, inner, span);
			arc_eq(b, inner, eq)
		}
		_ => impl_eq(b, "ne", ty, ty, [ty, ty], span),
	}
}

/// `PartialEq::eq` of `Arc`s of `inner`, as a function, which compares what they hold by
/// `eq`.
fn arc_eq<'tcx>(b: &Body<'_, '_, 'tcx>, inner: Ty<'tcx>, eq: Expr) -> Expr {
	let mut values = vec![eq];
	// std's `Arc` finds itself equal before comparing a value that is `Eq`.
	if super::iter::implements(b.t, inner, "Eq") {
		values.push(Expr::lit("true"));
	}
	Expr::rs("eqArc", values)
}

/// `PartialEq::eq` of sequences of types `t` and `u`, as a function, where the runtime's of
/// keys does not compare them: a `VecDeque` against another sequence, or items that are no
/// key or compare by their own `PartialEq<U>`. Else `None`.
fn seq_eq<'tcx>(b: &mut Body<'_, '_, 'tcx>, t: Ty<'tcx>, u: Ty<'tcx>, span: Span) -> Option<Expr> {
	let (t, u) = (peel(b, t), peel(b, u));
	let (Some(ti), Some(ui)) = (seq_item(b, t), seq_item(b, u)) else {
		return None;
	};
	let deque = [t, u].into_iter().any(|t| b.t.std_of(t) == Some(Std::VecDeque));
	if !deque && alike(b, t, u) && std_key(b, t) {
		return None;
	}
	Some(Expr::rs("eqSeq", vec![item_eq(b, ti, ui, span)]))
}

/// What a slice, an array, a `Vec`, or a `VecDeque` of type `ty` holds, bytes included.
fn seq_item<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Slice(item) | ty::Array(item, _) => Some(*item),
		ty::Adt(..) if matches!(b.t.std_of(ty), Some(Std::Vec | Std::VecDeque)) => Some(arg(ty, 0)),
		_ => None,
	}
}

/// What std's `!=` of a `t` and a `u` that a sequence holds finds equal, as a function:
/// `!<t as PartialEq<u>>::ne`, which `eq` stands for unless the crate overrides it.
fn item_eq<'tcx>(b: &mut Body<'_, '_, 'tcx>, t: Ty<'tcx>, u: Ty<'tcx>, span: Span) -> Expr {
	pair_eq(b, t, u, true, span)
}

/// `<t as PartialEq<u>>::eq` as a function, which `==` of a `t` and a `u` calls, or with
/// `by_ne`, what `!=` of them finds equal: `!ne`, which `eq` stands for unless the crate
/// overrides it.
pub fn pair_eq<'tcx>(b: &mut Body<'_, '_, 'tcx>, t: Ty<'tcx>, u: Ty<'tcx>, by_ne: bool, span: Span) -> Expr {
	let tcx = b.t.tcx;
	if let Some(eq) = seq_eq(b, t, u, span) {
		return eq;
	}
	let (pt, pu) = (peel(b, t), peel(b, u));
	if b.t.deferred(pt) || b.t.deferred(pu) {
		return Expr::undefined();
	}
	if alike(b, t, u) {
		return match by_ne {
			true => not_ne_fn(b, pt, span),
			false => eq_fn(b, pt, span),
		};
	}
	let (pt, pu) = peel_pair(b, trait_fn(tcx, "PartialEq", "eq"), t, u);
	let ne = trait_fn(tcx, "PartialEq", "ne");
	let own = by_ne
		&& Instance::try_resolve(tcx, b.t.typing_env_pub(), ne, tcx.mk_args(&[pt.into(), pu.into()]))
			.ok()
			.flatten()
			.is_some_and(|instance| instance.def_id() != ne);
	impl_eq(b, if own { "ne" } else { "eq" }, pt, pu, [t, u], span)
}

/// `<t as PartialOrd<u>>::partial_cmp` as a function, which std's `partial_cmp` of a `t`
/// and a `u` calls: through what std's forwards through, `cmp_fn`'s where std's
/// impl compares one type, and else the crate's own.
pub fn pair_partial_cmp<'tcx>(b: &mut Body<'_, '_, 'tcx>, t: Ty<'tcx>, u: Ty<'tcx>, span: Span) -> Expr {
	let tcx = b.t.tcx;
	if b.t.deferred(peel(b, t)) || b.t.deferred(peel(b, u)) {
		return Expr::undefined();
	}
	let f = trait_fn(tcx, "PartialOrd", "partial_cmp");
	let (pt, pu) = peel_pair(b, f, t, u);
	if !own_impl(b, f, tcx.mk_args(&[pt.into(), pu.into()])) && same_key(b, pt, pu) {
		return cmp_fn(b, pt, true, span);
	}
	impl_fn(b, "PartialOrd", "partial_cmp", pt, pu, [t, u], span)
}

/// `t` and `u` through what `peel` peels, one of each at a time, while std's impl of `f`,
/// `PartialEq::eq` or `PartialOrd::partial_cmp`, forwards through both: a crate's own impl
/// of two references or `Box`es is where they stop.
fn peel_pair<'tcx>(b: &Body<'_, '_, 'tcx>, f: DefId, t: Ty<'tcx>, u: Ty<'tcx>) -> (Ty<'tcx>, Ty<'tcx>) {
	match (pointee(b, t), pointee(b, u)) {
		(Some(t_in), Some(u_in)) if !own_impl(b, f, b.t.tcx.mk_args(&[t.into(), u.into()])) => {
			peel_pair(b, f, t_in, u_in)
		}
		_ => (t, u),
	}
}

/// `<t as PartialEq<u>>::{name}`, `eq` or `ne`, as `impl_fn` makes it. A `ne` returns
/// what `eq` would.
pub fn impl_eq<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	name: &str,
	t: Ty<'tcx>,
	u: Ty<'tcx>,
	held: [Ty<'tcx>; 2],
	span: Span,
) -> Expr {
	impl_fn(b, "PartialEq", name, t, u, held, span)
}

/// `<t as {tr}<u>>::{name}`, a method of a comparison trait, as a function of what JS holds
/// of types `held`, references to `t` and `u` or they themselves: a `[u8; N]` held is a
/// typed array, lent as the `rs.ByteSlice` the impl's `&[u8; N]` is. A `ne` is negated.
fn impl_fn<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	tr: &str,
	name: &str,
	t: Ty<'tcx>,
	u: Ty<'tcx>,
	held: [Ty<'tcx>; 2],
	span: Span,
) -> Expr {
	let tcx = b.t.tcx;
	let def_id = trait_fn(tcx, tr, name);
	let args = tcx.mk_args(&[t.into(), u.into()]);
	let f = b.fn_value(def_id, args, span);
	let ne = name == "ne";
	if !ne && !held.iter().any(|&ty| super::byte_array(ty)) {
		return f;
	}
	let names = [b.fresh("a"), b.fresh("b")];
	let values = std::iter::zip(&names, held)
		.map(|(name, ty)| match super::byte_array(ty) {
			true => Expr::rs("byteSliceFrom", vec![Expr::ident(name)]),
			false => Expr::ident(name),
		})
		.collect();
	let call = f.call(values);
	let params = std::iter::zip(names, held)
		.map(|(name, ty)| Param {
			name,
			ty: Some(b.t.ty(ty, span)),
		})
		.collect();
	let output = b.t.sig(def_id, args).output();
	let ret = StmtKind::Return(Some(if ne { call.not() } else { call })).at(None);
	Expr::Arrow(params, b.t.ty(output, span), vec![ret])
}

/// `Hash::hash` of `ty` into a `hasher`, as a function: the runtime's for a std type, or
/// the crate's, of its type or a reference to it.
fn hash_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, hasher: Ty<'tcx>, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let ty = peel_hash(b, ty, hasher);
	match ty.kind() {
		// What std writes of each: a `bool` as a byte, a `char` as its `u32`.
		ty::Bool => Expr::Rs("hash8".into()),
		ty::Char => Expr::Rs("hash32".into()),
		ty::Int(_) | ty::Uint(_) => Expr::Rs(
			hash_int(Int::of(ty).expect("an integer").rs())
				.expect("an integer")
				.into(),
		),
		ty::Str => Expr::Rs("hashStr".into()),
		ty::Adt(..) if library::string(tcx, ty) || library::owned_string(tcx, ty) => Expr::Rs("hashStr".into()),
		// Its derived `Hash` writes its `i8` discriminant.
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Ordering) => Expr::Rs("hashI8".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Duration) => Expr::Rs("durationHash".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::TypeId) => Expr::Rs("typeIdHash".into()),
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Location) => Expr::Rs("locationHash".into()),
		_ if library::nothing(tcx, ty) => Expr::Rs("hashNothing".into()),
		ty::Tuple(tys) => {
			let fns = tys.iter().map(|t| hash_fn(b, t, hasher, span)).collect();
			Expr::rs("hashTuple", vec![Expr::Array(fns)])
		}
		ty::Adt(..) if let Some(inner) = option_of(b, ty) => {
			let hash = hash_fn(b, inner, hasher, span);
			Expr::rs("hashOption", [hash].into_iter().chain(b.t.none_arg(inner)).collect())
		}
		ty::Adt(..) if b.t.std_of(ty) == Some(Std::Poll) => {
			let inner = arg(ty, 0);
			let hash = hash_fn(b, inner, hasher, span);
			Expr::rs("hashPoll", [hash].into_iter().chain(b.t.pending_arg(inner)).collect())
		}
		ty::Adt(..) if let Some(inner) = arc_of(b, ty) => Expr::rs("hashArc", vec![hash_fn(b, inner, hasher, span)]),
		ty::Adt(..) if let Some(inner) = reverse_of(b, ty).or_else(|| overflowing_of(b, ty)) => {
			hash_fn(b, inner, hasher, span)
		}
		ty::Adt(..) if let Some(borrowed) = cow_of(b, ty) => {
			Expr::rs("hashCow", vec![hash_fn(b, borrowed, hasher, span)])
		}
		ty::Adt(..) if let Some(bound) = range_of(b, ty) => {
			Expr::rs("rangeHash", vec![hash_fn(b, bound, hasher, span)])
		}
		_ if let Some(elem) = elements_of(b, ty) => {
			if own_hash_slice(b, elem, hasher) {
				let msg =
					format!("hashing `{ty}` through `{elem}`'s own `Hash::hash_slice`, which is not supported yet");
				return b.err(span, msg);
			}
			// An integer's `hash_slice` writes the bytes of all of them at once.
			if let Some(int) = Int::of(elem) {
				let size = if int.pointer { 4 } else { int.bits / 8 };
				return Expr::rs("hashInts", vec![Expr::lit(size.to_string())]);
			}
			Expr::rs("hashSlice", vec![hash_fn(b, elem, hasher, span)])
		}
		// Its length, then each item's, never by `hash_slice`.
		ty::Adt(..) if let Some(item) = list_of(b, ty) => Expr::rs("hashList", vec![hash_fn(b, item, hasher, span)]),
		// Its length, then its bytes, whichever form it is: a `[u8; N]` hashes as a slice.
		_ if bytes(b, ty) => Expr::Rs("hashBytes".into()),
		_ if by_own(b, ty) => {
			let hash = trait_fn(tcx, "Hash", "hash");
			let args = tcx.mk_args(&[ty.into(), hasher.into()]);
			let f = b.fn_value(hash, args, span);
			// A translated one takes the `&mut` to a hasher the runtime calls the methods of,
			// which its functions wrap; a binding's export takes the `rs.Hasher`.
			match called_hasher(b.t, hasher) && !b.t.binds(hash, args) {
				true => Expr::rs("hashOwn", vec![f]),
				false => f,
			}
		}
		_ if raw_pointee(tcx, ty).is_some() => b.err(
			span,
			format!("hashing `{ty}` by its address, which JS has no counterpart of"),
		),
		_ if b.t.deferred(ty) => Expr::undefined(),
		_ => b.err(span, format!("hashing `{ty}` as a key, which is not supported yet")),
	}
}

/// Whether the crate's own `Hash` of `elem` overrides `hash_slice`, which a slice of it
/// hashes through in place of each element's `hash`. A reference's that std's forwards
/// through is std's default, which hashes each by what it points at.
fn own_hash_slice<'tcx>(b: &Body<'_, '_, 'tcx>, elem: Ty<'tcx>, hasher: Ty<'tcx>) -> bool {
	let tcx = b.t.tcx;
	let f = trait_fn(tcx, "Hash", "hash_slice");
	by_own(b, peel_hash(b, elem, hasher))
		&& Instance::try_resolve(tcx, b.t.typing_env_pub(), f, tcx.mk_args(&[elem.into(), hasher.into()]))
			.ok()
			.flatten()
			.is_some_and(|instance| instance.def_id() != f)
}

/// `Clone::clone` of `ty`, as a function, or `undefined` where it is the value itself.
pub fn clone_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	let (tcx, env) = (b.t.tcx, b.t.typing_env_pub());
	if library::clone_copies(b.t, env, ty)
		&& let Some(copy) = b.t.copy_fn(ty, span)
	{
		return copy;
	}
	if library::clone_copies(b.t, env, ty) || library::clone_shares(b.t, env, ty) {
		return Expr::undefined();
	}
	if let ty::Closure(def_id, _) | ty::CoroutineClosure(def_id, _) = ty.kind() {
		b.t.clone_closure(*def_id);
		return Expr::Rs("closureClone".into());
	}
	// The runtime holds an array as its items, where its `clone` takes a slice of them.
	if let ty::Array(elem, _) = ty.kind() {
		let items = Expr::ident("a");
		let made = match clone_fn(b, *elem, span) {
			Expr::Lit(l) if l == "undefined" => items.member("slice").call(Vec::new()),
			clone => items.member("map").call(vec![clone]),
		};
		let param = crate::ts::Param {
			name: "a".into(),
			ty: Some(b.t.ty(ty, span)),
		};
		return Expr::Lambda(vec![param], Box::new(made));
	}
	let clone = trait_fn(tcx, "Clone", "clone");
	b.fn_value(clone, tcx.mk_args(&[ty.into()]), span)
}

/// An item of one of std's traits, by the trait's diagnostic name: `Ord`'s `cmp`.
pub fn trait_fn(tcx: ty::TyCtxt<'_>, tr: &str, item: &str) -> rustc_hir::def_id::DefId {
	let tr = tcx
		.get_diagnostic_item(rustc_span::Symbol::intern(tr))
		.expect("a trait of std");
	trait_fn_of(tcx, tr, item)
}

/// The function `item` of the trait `tr`.
fn trait_fn_of(tcx: ty::TyCtxt<'_>, tr: DefId, item: &str) -> DefId {
	tcx.associated_items(tr)
		.filter_by_name_unhygienic(rustc_span::Symbol::intern(item))
		.next()
		.expect("an item of a trait of std")
		.def_id
}

/// What `into_iter` makes of a collection of type `ty`, by value or by reference: an
/// `rs.Iter` of its entries, keys, or items, which by value drops those it still holds.
pub fn into_iter<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Option<Expr> {
	let coll = ty.peel_refs();
	// A `&mut` lends each value JS copies as its place.
	let places = |n: usize| ty.ref_mutability() == Some(ty::Mutability::Mut) && b.t.needs_place(arg(coll, n));
	let name = match b.t.std_of(coll)? {
		Std::HashMap if places(1) => "hashIterMut",
		Std::BTreeMap if places(1) => "btreeIterMut",
		Std::VecDeque if places(0) => "dequeIterMut",
		Std::LinkedList if places(0) => "listIterMut",
		Std::BinaryHeap => "heapIter",
		Std::HashMap => "hashIter",
		Std::HashSet => "hashKeys",
		Std::HashTable if places(0) => "hashValuesMut",
		Std::HashTable => "hashValues",
		Std::BTreeMap => "btreeIter",
		Std::BTreeSet => "btreeKeys",
		Std::VecDeque => "dequeIter",
		Std::LinkedList => "listIter",
		_ => return None,
	};
	if !ty.is_ref() {
		// A deque's is the deque itself, which it pops from.
		let name = if name == "dequeIter" { "dequeIntoIter" } else { name };
		let mut droppers = entry_droppers(b, coll, span);
		// A map's own `BuildHasher` drops at once, after what drops each entry.
		if let Some(state) = state_dropper(b, coll, span) {
			droppers.resize(2, Expr::undefined());
			droppers.push(state);
		}
		return Some(Expr::rs(name, super::iter::tail(vec![value], droppers)));
	}
	// What it holds, borrowed.
	let iter = Expr::rs(name, vec![value]);
	Some(match super::iter::iter_of(b.t, ty) {
		Some(into) => super::lend::lend(b, into, into, iter, span),
		None => iter,
	})
}

/// `collect` into a collection of type `ty` from `iter`, an iterator of type `from`, which
/// std's reserves for as it does a `TrustedLen` one if `trusted`.
pub fn collect<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	iter: Expr,
	from: Ty<'tcx>,
	trusted: bool,
	span: Span,
) -> Option<Expr> {
	let std = b.t.std_of(ty)?;
	Some(match std {
		Std::HashMap | Std::HashSet => {
			let state = new_state(b, ty, span);
			let held = state.is_some();
			let target = hash_new(b, ty, state, span)?;
			// hashbrown's map takes its items and reads their size hint before it makes its
			// hasher, where std's and hashbrown's set make it first.
			if std == Std::HashMap
				&& held
				&& matches!(ty.kind(), ty::Adt(adt, _) if super::stock::as_std(b.t.tcx, adt.did()))
			{
				let make = Expr::Lambda(Vec::new(), Box::new(target));
				let tail = hash_extend_tail(b, ty, from, span);
				return Some(Expr::rs("hashFromIter", super::iter::tail(vec![iter, make], tail)));
			}
			hash_extend(b, ty, target, iter, from, span)
		}
		Std::BTreeMap | Std::BTreeSet => btree_collect(b, ty, iter, from, trusted, span),
		// std's collects into a `Vec`, and rebuilds it in place.
		Std::BinaryHeap => {
			let item = arg(ty, 0);
			let d = super::iter::to_vec(b, iter, from, item, trusted, span);
			Expr::rs("heapFrom", [vec![d], heap_tests(b, item, true, span)].concat())
		}
		Std::LinkedList => {
			let target = list_new(b, ty, span);
			extend(b, ty, arg(ty, 0), target, iter, from, span)?
		}
		// std's takes the buffer of a deque's or a `Vec`'s `into_iter` as it is, and
		// collects any other into a `Vec` first, whose capacity it keeps.
		Std::VecDeque => {
			let item = arg(ty, 0);
			let path = from
				.ty_adt_def()
				.map(|adt| library::std_path(b.t.tcx, adt.did()).join("::"));
			let whole = matches!(
				path.as_deref(),
				Some("collections::vec_deque::into_iter::IntoIter" | "vec::into_iter::IntoIter")
			);
			let made = match whole {
				true => Expr::rs("dequeFromIntoIter", [vec![iter], deque_items(b, item, span)].concat()),
				false => {
					let items = super::iter::to_vec(b, iter, from, item, trusted, span);
					Expr::rs("dequeFromVec", [vec![items], deque_items(b, item, span)].concat())
				}
			};
			empty(b, made, ty, span)
		}
		_ => return None,
	})
}
