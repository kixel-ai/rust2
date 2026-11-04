//! Integers: literals, arithmetic, and casts with Rust's semantics.
//!
//! Integers up to 32 bits are JS numbers, in which their sums, differences, and
//! products are exact or provably out of range, so each is checked as computed. A wider
//! integer is a number too while it is a safe integer, and the runtime's immutable
//! object of 32-bit limbs past that, so every operator and comparison on one goes
//! through the runtime. Every operation that can overflow is checked, as a debug build
//! checks it: none wraps silently.

use rustc_middle::mir::{BinOp, UnOp};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use super::body::Body;
use super::float;
use crate::ts::{Expr, Key, Type};

/// An integer type, by width and signedness. `usize` and `isize` are 32 bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Int {
	pub bits: u32,
	pub signed: bool,
	/// `usize` or `isize`, which are named so in the runtime.
	pub pointer: bool,
}

impl Int {
	pub fn of(ty: Ty<'_>) -> Option<Self> {
		let (bits, signed, pointer) = match ty.kind() {
			ty::Int(int) => match int {
				ty::IntTy::Isize => (32, true, true),
				int => (int.bit_width().unwrap() as u32, true, false),
			},
			ty::Uint(uint) => match uint {
				ty::UintTy::Usize => (32, false, true),
				uint => (uint.bit_width().unwrap() as u32, false, false),
			},
			_ => return None,
		};
		Some(Self { bits, signed, pointer })
	}

	/// Whether values past 2^53 are runtime objects rather than numbers.
	pub fn big(self) -> bool {
		self.bits > 32
	}

	/// The TypeScript type of a value, named as the runtime's: `rs.u32`.
	pub fn ty(self) -> Type {
		Type::Rs(self.rs(), Vec::new())
	}

	pub fn min(self) -> i128 {
		match (self.signed, self.bits) {
			(true, 128) => i128::MIN,
			(true, bits) => -(1i128 << (bits - 1)),
			(false, _) => 0,
		}
	}

	pub fn max(self) -> u128 {
		match (self.signed, self.bits) {
			(true, bits) => (1u128 << (bits - 1)) - 1,
			(false, 128) => u128::MAX,
			(false, bits) => (1u128 << bits) - 1,
		}
	}

	/// `value as Self`: its low bits, read with this type's sign.
	pub fn fold(self, value: i128) -> i128 {
		let size = rustc_abi::Size::from_bits(self.bits);
		let bits = size.truncate(value as u128);
		match self.signed {
			true => size.sign_extend(bits),
			false => bits as i128,
		}
	}

	/// The runtime's name for the type: `rs.u32`.
	pub fn rs(self) -> &'static str {
		match (self.signed, self.bits, self.pointer) {
			(false, _, true) => "usize",
			(true, _, true) => "isize",
			(false, 8, _) => "u8",
			(false, 16, _) => "u16",
			(false, 32, _) => "u32",
			(false, 64, _) => "u64",
			(false, _, _) => "u128",
			(true, 8, _) => "i8",
			(true, 16, _) => "i16",
			(true, 32, _) => "i32",
			(true, 64, _) => "i64",
			(true, _, _) => "i128",
		}
	}

	/// `value` as a literal of this type. A `u128` past `i128::MAX` is its bits.
	pub fn lit(self, value: i128) -> Expr {
		let text = match self.signed {
			true => value.to_string(),
			false => (value as u128).to_string(),
		};
		self.spelled(text, value)
	}

	/// A literal of this type spelled `text`, JS number digits of `value`. A wide value
	/// past 2^53 is its limbs, written out as the runtime makes them, `{ hi: 0xffffffff,
	/// lo: 0xffffffff } as rs.u64`: an object literal costs what a hoisted constant does,
	/// where converting a bigint cost more than the arithmetic around it.
	fn spelled(self, text: String, value: i128) -> Expr {
		let safe = (1 - (1i128 << 53)..(1 << 53)).contains(&value) && (self.signed || value >= 0);
		if safe {
			return Expr::lit(text);
		}
		let word = |i: u32| (value as u128 >> (32 * i)) as u32;
		let hex = |negative: bool, w: u32| match w {
			0 => Expr::lit("0"),
			w => Expr::lit(format!("{}0x{w:x}", if negative { "-" } else { "" })),
		};
		let top = self.bits / 32 - 1;
		let hi = match self.signed {
			true => hex((word(top) as i32) < 0, (word(top) as i32).unsigned_abs()),
			false => hex(false, word(top)),
		};
		let names: &[&str] = if top == 1 { &["lo"] } else { &["lo", "w1", "w2"] };
		let mut fields = vec![(Key::Name("hi".into()), hi)];
		for (i, name) in names.iter().enumerate().rev() {
			fields.push((Key::Name((*name).into()), hex(false, word(i as u32))));
		}
		Expr::As(Box::new(Expr::Object(fields)), self.ty())
	}

	/// A call of the type's runtime function `name`: `rs.u8Add` for `add`.
	pub fn method(self, name: &str, args: Vec<Expr>) -> Expr {
		self.func(name).call(args)
	}

	/// The type's runtime function `name`, uncalled.
	pub fn func(self, name: &str) -> Expr {
		Expr::rs_of(self.rs(), name)
	}

	/// `value`, an exact integer of at most 32 bits, wrapped into this narrow type's range.
	pub fn wrap(self, value: Expr) -> Expr {
		debug_assert!(!self.big());
		match (self.signed, self.bits) {
			(false, 32) => value.binary(">>>", Expr::lit("0")),
			(true, 32) => value.binary("|", Expr::lit("0")),
			(false, bits) => value.binary("&", Expr::lit(format!("0x{:x}", (1u64 << bits) - 1))),
			(true, bits) => {
				let shift = Expr::lit((32 - bits).to_string());
				value.binary("<<", shift.clone()).binary(">>", shift)
			}
		}
	}

	/// A comparison, `===` or `<` for instance, of two values of this type.
	pub fn compare(self, js: &'static str, l: Expr, r: Expr) -> Expr {
		// A wide value has one form, so it equals a number literal only as that number.
		let literal = |e: &Expr| matches!(e, Expr::Lit(_));
		if !self.big() || (matches!(js, "===" | "!==") && (literal(&l) || literal(&r))) {
			return l.binary(js, r);
		}
		let name = match js {
			"===" => "eq",
			"!==" => "ne",
			"<" => "lt",
			"<=" => "le",
			">" => "gt",
			_ => "ge",
		};
		self.method(name, vec![l, r])
	}
}

/// An operand, and its value when it is a constant.
pub struct Operand {
	pub expr: Expr,
	pub value: Option<i128>,
}

/// Fold a constant operation on `int`s as Rust would, or `None` if it overflows.
pub fn fold(op: BinOp, a: i128, b: i128, int: Int) -> Option<i128> {
	// 128-bit values don't all fit the fold's `i128`.
	if int.bits == 128 {
		return None;
	}
	let value = match op {
		BinOp::Add => a.checked_add(b)?,
		BinOp::Sub => a.checked_sub(b)?,
		BinOp::Mul => a.checked_mul(b)?,
		BinOp::Div if b != 0 => a.checked_div(b)?,
		BinOp::Rem if b != 0 => a.checked_rem(b)?,
		BinOp::BitAnd => a & b,
		BinOp::BitOr => a | b,
		BinOp::BitXor => a ^ b,
		BinOp::Shl if (0..int.bits as i128).contains(&b) => {
			let shifted = a.checked_shl(b as u32)?;
			// Bits shifted out are lost; keep the fold exact by declining then.
			if shifted >> b != a {
				return None;
			}
			shifted
		}
		BinOp::Shr if (0..int.bits as i128).contains(&b) => a >> b,
		_ => return None,
	};
	(int.min()..=int.max() as i128).contains(&value).then_some(value)
}

pub fn binary<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	op: BinOp,
	l: Operand,
	r: Operand,
	lty: Ty<'tcx>,
	rty: Ty<'tcx>,
	span: Span,
) -> Expr {
	let (lty, rty) = (lty.peel_refs(), rty.peel_refs());
	if let Some(float) = float::Float::of(lty) {
		return float::binary(b, float, op, l.expr, r.expr, span);
	}
	let compare = match op {
		BinOp::Eq => Some("==="),
		BinOp::Ne => Some("!=="),
		BinOp::Lt => Some("<"),
		BinOp::Le => Some("<="),
		BinOp::Gt => Some(">"),
		BinOp::Ge => Some(">="),
		_ => None,
	};
	if let Some(js) = compare {
		return match lty.kind() {
			ty::Int(_) | ty::Uint(_) => Int::of(lty).unwrap().compare(js, l.expr, r.expr),
			ty::Bool | ty::Char => l.expr.binary(js, r.expr),
			// A function is its JS function, which Rust leaves free to compare as another's
			// or apart from itself, and which has no address to order by.
			ty::FnPtr(..) if matches!(op, BinOp::Eq | BinOp::Ne) => l.expr.binary(js, r.expr),
			_ => b.err(span, format!("comparing `{lty}`")),
		};
	}

	if let ty::Bool = lty.kind() {
		return match op {
			BinOp::BitXor => l.expr.binary("!==", r.expr),
			// Both sides are already evaluated, so short-circuiting changes nothing.
			BinOp::BitAnd if r.expr.is_path() => l.expr.binary("&&", r.expr),
			BinOp::BitOr if r.expr.is_path() => l.expr.binary("||", r.expr),
			BinOp::BitAnd => Expr::rs("and", vec![l.expr, r.expr]),
			BinOp::BitOr => Expr::rs("or", vec![l.expr, r.expr]),
			_ => b.err(span, format!("`{op:?}` on `bool`")),
		};
	}

	let Some(int) = Int::of(lty) else {
		return b.err(span, format!("arithmetic on `{lty}`"));
	};

	// Constants fold, written as the source has them where JS computes the same.
	if let (Some(a), Some(c)) = (l.value, r.value)
		&& let Some(value) = fold(op, a, c, int)
	{
		let exact = !int.big()
			&& match op {
				BinOp::Add | BinOp::Sub | BinOp::Mul => value.abs() < (1 << 53),
				// JS bitwise operators read their operands as 32-bit signed integers.
				_ => [a, c, value].iter().all(|v| (0..=i32::MAX as i128).contains(v)),
			};
		if !exact {
			return int.lit(value);
		}
		let js = match op {
			BinOp::Add => "+",
			BinOp::Sub => "-",
			BinOp::Mul => "*",
			BinOp::Div | BinOp::Rem => return int.lit(value),
			BinOp::BitAnd => "&",
			BinOp::BitOr => "|",
			BinOp::BitXor => "^",
			BinOp::Shl => "<<",
			BinOp::Shr => ">>",
			_ => return int.lit(value),
		};
		return l.expr.binary(js, r.expr);
	}

	if int.big() {
		let name = match op {
			BinOp::Add => "add",
			BinOp::Sub => "sub",
			BinOp::Mul => "mul",
			BinOp::Div => "div",
			BinOp::Rem => "rem",
			BinOp::BitAnd => "and",
			BinOp::BitOr => "or",
			BinOp::BitXor => "xor",
			BinOp::Shl => "shl",
			BinOp::Shr => "shr",
			_ => return b.err(span, format!("unsupported operator `{op:?}`")),
		};
		// A constant shift amount is a number, whatever its type.
		let rhs = match (op, r.value) {
			(BinOp::Shl | BinOp::Shr, Some(c)) => Expr::lit(c.to_string()),
			_ => r.expr,
		};
		return int.method(name, vec![l.expr, rhs]);
	}

	match op {
		BinOp::Add => int.method("add", vec![l.expr, r.expr]),
		BinOp::Sub => int.method("sub", vec![l.expr, r.expr]),
		BinOp::Mul => int.method("mul", vec![l.expr, r.expr]),
		BinOp::Div => int.method("div", vec![l.expr, r.expr]),
		BinOp::Rem => int.method("rem", vec![l.expr, r.expr]),
		BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor => {
			let js = match op {
				BinOp::BitAnd => "&",
				BinOp::BitOr => "|",
				_ => "^",
			};
			let value = l.expr.binary(js, r.expr);
			// Only an unsigned 32-bit result can land outside a JS int32.
			match (int.signed, int.bits) {
				(false, 32) => value.binary(">>>", Expr::lit("0")),
				_ => value,
			}
		}
		BinOp::Shl | BinOp::Shr => {
			let amount = match (r.value, Int::of(rty)) {
				(Some(c), _) if (0..int.bits as i128).contains(&c) => Some(c),
				_ => None,
			};
			let Some(c) = amount else {
				let name = if op == BinOp::Shl { "shl" } else { "shr" };
				return int.method(name, vec![l.expr, r.expr]);
			};
			match op {
				BinOp::Shl => int.wrap(l.expr.binary("<<", Expr::lit(c.to_string()))),
				_ if !int.signed && int.bits == 32 => l.expr.binary(">>>", Expr::lit(c.to_string())),
				_ => l.expr.binary(">>", Expr::lit(c.to_string())),
			}
		}
		_ => b.err(span, format!("unsupported operator `{op:?}`")),
	}
}

/// How a `Wrapping`'s or a `Saturating`'s operators overflow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Overflow {
	Wrap,
	Saturate,
}

impl Overflow {
	/// The prefix of the integer methods that overflow so: `wrapping` for `wrapping_pow`.
	pub fn prefix(self) -> &'static str {
		match self {
			Self::Wrap => "wrapping",
			Self::Saturate => "saturating",
		}
	}

	/// The runtime's name for the integer method that overflows so: `wrappingAdd` for `Add`.
	pub fn method(self, op: &str) -> String {
		match self {
			Self::Wrap => format!("wrapping{op}"),
			Self::Saturate => format!("saturating{op}"),
		}
	}
}

/// `l op r` of `Wrapping<ty>`s or `Saturating<ty>`s, as the integer's `wrapping_*` or
/// `saturating_*` computes it. Arithmetic wraps or saturates, and a `Wrapping`'s shift masks
/// its amount to the width. A zero divisor panics, as does a `Saturating`'s `MIN % -1`, whose
/// `Rem` is the integer's.
pub fn overflowing<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	overflow: Overflow,
	op: BinOp,
	l: Operand,
	r: Operand,
	ty: Ty<'tcx>,
	span: Span,
) -> Expr {
	let Some(int) = Int::of(ty) else {
		return b.err(span, format!("wrapping or saturating arithmetic on `{ty}`"));
	};
	let name = match (overflow, op) {
		(_, BinOp::Add) => "Add",
		(_, BinOp::Sub) => "Sub",
		(_, BinOp::Mul) => "Mul",
		(_, BinOp::Div) if int.signed => "Div",
		(Overflow::Wrap, BinOp::Rem) if int.signed => "Rem",
		(Overflow::Wrap, BinOp::Shl | BinOp::Shr) => {
			let mask = i128::from(int.bits - 1);
			let r = match r.value {
				Some(c) => Operand {
					expr: Expr::lit((c & mask).to_string()),
					value: Some(c & mask),
				},
				None => Operand {
					expr: r.expr.binary("&", Expr::lit(mask.to_string())),
					value: None,
				},
			};
			return binary(b, op, l, r, ty, b.t.tcx.types.usize, span);
		}
		// std implements no shift for a `Saturating`.
		(Overflow::Saturate, BinOp::Shl | BinOp::Shr) => {
			return b.err(span, format!("`{op:?}` of `Saturating<{ty}>`"));
		}
		// The integer's own: a bitwise operator never overflows, and an unsigned division or a
		// `Saturating`'s remainder panics where the integer's does.
		_ => return binary(b, op, l, r, ty, ty, span),
	};
	int.method(&overflow.method(name), vec![l.expr, r.expr])
}

pub fn unary<'tcx>(b: &mut Body<'_, '_, 'tcx>, op: UnOp, value: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	let ty = ty.peel_refs();
	if let Some(float) = float::Float::of(ty) {
		return float::unary(b, float, op, value, span);
	}
	match (op, ty.kind()) {
		(UnOp::Not, ty::Bool) => value.not(),
		(UnOp::Not, ty::Int(_) | ty::Uint(_)) => {
			let int = Int::of(ty).unwrap();
			if int.big() {
				return int.method("not", vec![value]);
			}
			let inverted = Expr::unary("~", value);
			match int.signed {
				true => inverted,
				false => int.wrap(inverted),
			}
		}
		(UnOp::Neg, ty::Int(_)) => {
			let int = Int::of(ty).unwrap();
			int.method("neg", vec![value])
		}
		_ => b.err(span, format!("unsupported `{op:?}` on `{ty}`")),
	}
}

/// `value as to`.
pub fn cast<'tcx>(b: &mut Body<'_, '_, 'tcx>, value: Operand, from: Ty<'tcx>, to: Ty<'tcx>, span: Span) -> Expr {
	if from.is_floating_point() || to.is_floating_point() {
		return float::cast(b, value, from, to, span);
	}
	// Only a `u8` casts to a `char`, whose scalar value it is.
	if let (ty::Char, ty::Uint(ty::UintTy::U8)) = (to.kind(), from.kind()) {
		return Expr::As(Box::new(value.expr), Type::Rs("char", Vec::new()));
	}
	let Some(target) = Int::of(to) else {
		return b.err(span, format!("a cast to `{to}`"));
	};
	// A `char` is its scalar value, at most 0x10FFFF: it fits a type of more than 21 bits.
	if let ty::Char = from.kind() {
		return match target.bits > 21 {
			true => Expr::As(Box::new(value.expr), target.ty()),
			false => target.wrap(value.expr),
		};
	}
	if let ty::Bool = from.kind() {
		return Expr::Cond(Box::new(value.expr), Box::new(target.lit(1)), Box::new(target.lit(0)));
	}
	if let ty::Adt(adt, _) = from.kind()
		&& adt.is_enum()
		&& adt.variants().iter().all(|v| v.fields.is_empty())
	{
		// A fieldless enum's tag to its discriminant.
		let tcx = b.t.tcx;
		let table = adt
			.discriminants(tcx)
			.map(|(index, discr)| {
				let value = match Int::of(discr.ty) {
					Some(repr) if repr.signed => rustc_abi::Size::from_bits(repr.bits).sign_extend(discr.val),
					_ => discr.val as i128,
				};
				(
					crate::ts::Key::Name(adt.variant(index).name.to_string()),
					target.lit(target.fold(value)),
				)
			})
			.collect();
		return Expr::Index(Box::new(Expr::Object(table)), Box::new(value.expr));
	}
	let Some(source) = Int::of(from) else {
		return b.err(span, format!("a cast from `{from}`"));
	};
	if let Some(c) = value.value {
		return target.lit(target.fold(c));
	}
	let fits = target.min() <= source.min() && source.max() <= target.max();
	// A cast that keeps the value still changes the type `tsc` checks it as.
	let retyped = |value: Expr| match source.rs() == target.rs() {
		true => value,
		false => Expr::As(Box::new(value), target.ty()),
	};
	match (source.big(), target.big()) {
		(false, false) if fits => retyped(value.expr),
		(false, false) => target.wrap(value.expr),
		(true, false) => {
			let low = source.method("low", vec![value.expr]);
			match (target.signed, target.bits) {
				(false, 32) => low,
				_ => target.wrap(low),
			}
		}
		// A narrow integer is already a wide one's number.
		(false, true) if fits => retyped(value.expr),
		(true, true) if source.rs() == target.rs() => value.expr,
		_ => target.method("cast", vec![value.expr]),
	}
}
