//! `f64` and `f32`. An `f64` is a JS number, which is the same IEEE 754 double, so its
//! arithmetic and comparisons are JS's operators, rounded as Rust's are. An `f32` is a
//! number a single holds exactly: each arithmetic result is rounded to one by
//! `Math.fround`, which gives the single nearest the exact value, since a double holds
//! the sum, product, quotient, or square root of singles closely enough. Where JS differs
//! from std, as in rounding half away from zero, `max` and `min` around NaN, casts that
//! saturate, and `mul_add`'s one rounding, the runtime has std's. The functions std leaves to the platform's libm
//! are `Math`'s.

use rustc_middle::mir::{BinOp, UnOp};
use rustc_middle::ty::{self, ScalarInt, Ty};
use rustc_span::Span;

use super::body::Body;
use super::num::{Int, Operand};
use crate::ts::{Expr, Type};

/// A float type JS has a value of.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Float {
	F32,
	F64,
}

impl Float {
	/// `ty`'s, if it is `f32` or `f64`. The other float types have no JS value yet.
	pub fn of(ty: Ty<'_>) -> Option<Self> {
		match ty.kind() {
			ty::Float(ty::FloatTy::F32) => Some(Self::F32),
			ty::Float(ty::FloatTy::F64) => Some(Self::F64),
			_ => None,
		}
	}

	/// The Rust name, which also prefixes the type's runtime functions: `rs.f32Round`.
	pub fn name(self) -> &'static str {
		match self {
			Self::F32 => "f32",
			Self::F64 => "f64",
		}
	}

	/// The TypeScript type of a value.
	pub fn ty(self) -> Type {
		Type::Rs(self.name(), Vec::new())
	}

	/// `e`, a number JS computed exactly or as a double, rounded to this type.
	pub fn round(self, e: Expr) -> Expr {
		match self {
			Self::F32 => Expr::ident("Math").member("fround").call(vec![e]),
			Self::F64 => e,
		}
	}

	/// A constant from its bits.
	pub fn constant(self, int: ScalarInt) -> Expr {
		match self {
			Self::F32 => lit32(f32::from_bits(int.to_u32())),
			Self::F64 => lit(f64::from_bits(int.to_u64())),
		}
	}

	/// `v`, which this type holds exactly, as a literal.
	pub fn lit(self, v: f64) -> Expr {
		match self {
			Self::F32 => lit32(v as f32),
			Self::F64 => lit(v),
		}
	}

	/// The type's runtime function `name`, uncalled.
	fn func(self, name: &str) -> Expr {
		Expr::rs_of(self.name(), name)
	}
}

/// Whether `ty` is a float type JS has a value of.
pub fn is(ty: Ty<'_>) -> bool {
	Float::of(ty).is_some()
}

/// `v` as a literal: the shortest digits that read back as it, as Rust's `Debug` prints them.
/// A NaN other than `f64::NAN` is made from its bits, which keeps its sign and payload.
pub fn lit(v: f64) -> Expr {
	match v {
		_ if v.is_nan() && v.to_bits() != f64::NAN.to_bits() => {
			let u64 = Int {
				bits: 64,
				signed: false,
				pointer: false,
			};
			Expr::rs("f64FromBits", vec![u64.lit(i128::from(v.to_bits()))])
		}
		_ if v.is_nan() => Expr::lit("NaN"),
		f64::INFINITY => Expr::lit("Infinity"),
		f64::NEG_INFINITY => Expr::lit("-Infinity"),
		_ => Expr::lit(format!("{v:?}")),
	}
}

/// An `f32` as a literal: the double it is exactly, or a NaN other than `f32::NAN` from its bits.
fn lit32(v: f32) -> Expr {
	match v {
		_ if v.is_nan() && v.to_bits() != f32::NAN.to_bits() => {
			let u32 = Int {
				bits: 32,
				signed: false,
				pointer: false,
			};
			Expr::rs("f32FromBits", vec![u32.lit(i128::from(v.to_bits()))])
		}
		_ => lit(f64::from(v)),
	}
}

/// `l op r` of two floats, which JS's operators compute as Rust's do, an `f32`'s rounded.
pub fn binary<'tcx>(b: &mut Body<'_, '_, 'tcx>, float: Float, op: BinOp, l: Expr, r: Expr, span: Span) -> Expr {
	let rounds = |js| float.round(l.clone().binary(js, r.clone()));
	match op {
		BinOp::Add => rounds("+"),
		BinOp::Sub => rounds("-"),
		BinOp::Mul => rounds("*"),
		BinOp::Div => rounds("/"),
		// Both are C's `fmod`, which is exact, with the dividend's sign.
		BinOp::Rem => l.binary("%", r),
		BinOp::Eq => l.binary("===", r),
		BinOp::Ne => l.binary("!==", r),
		BinOp::Lt => l.binary("<", r),
		BinOp::Le => l.binary("<=", r),
		BinOp::Gt => l.binary(">", r),
		BinOp::Ge => l.binary(">=", r),
		_ => b.err(span, format!("`{op:?}` on `{}`", float.name())),
	}
}

pub fn unary<'tcx>(b: &mut Body<'_, '_, 'tcx>, float: Float, op: UnOp, value: Expr, span: Span) -> Expr {
	match op {
		UnOp::Neg => Expr::unary("-", value),
		_ => b.err(span, format!("`{op:?}` on `{}`", float.name())),
	}
}

/// `value as to` where either side is a float.
pub fn cast<'tcx>(b: &mut Body<'_, '_, 'tcx>, value: Operand, from: Ty<'tcx>, to: Ty<'tcx>, span: Span) -> Expr {
	match (Float::of(from), Float::of(to), Int::of(from), Int::of(to)) {
		(Some(f), Some(t), ..) if f == t => value.expr,
		// Every single is exactly a double.
		(Some(Float::F32), Some(Float::F64), ..) => Expr::As(Box::new(value.expr), Float::F64.ty()),
		(Some(Float::F64), Some(Float::F32), ..) => Float::F32.round(value.expr),
		// `From<bool>`, which `as` has no form of.
		(None, Some(t), ..) if from.is_bool() => {
			Expr::Cond(Box::new(value.expr), Box::new(t.lit(1.0)), Box::new(t.lit(0.0)))
		}
		// An integer to the nearest float, rounded once: an integer of at most 32 bits is
		// exactly a double, which then rounds to a single once.
		(None, Some(t), Some(int), _) => match (value.value, t) {
			(Some(c), Float::F64) if int.signed => lit(c as f64),
			(Some(c), Float::F64) => lit(c as u128 as f64),
			(Some(c), Float::F32) if int.signed => lit32(c as f32),
			(Some(c), Float::F32) => lit32(c as u128 as f32),
			(None, Float::F64) if int.big() => int.method("toF64", vec![value.expr]),
			(None, Float::F32) if int.big() => int.method("toF32", vec![value.expr]),
			(None, Float::F64) => Expr::As(Box::new(value.expr), t.ty()),
			(None, Float::F32) => Float::F32.round(value.expr),
		},
		// Truncated, saturated at the type's bounds, and 0 for NaN, of an `f32` as the double it is.
		(Some(f), None, _, Some(int)) => {
			let value = match f {
				Float::F32 => Expr::As(Box::new(value.expr), Float::F64.ty()),
				Float::F64 => value.expr,
			};
			int.method("fromF64", vec![value])
		}
		_ => b.err(span, format!("a cast from `{from}` to `{to}`")),
	}
}

/// A call of the float type's own `name`, or `None` if it is not mapped.
pub fn method(float: Float, name: &str, args: Vec<Expr>) -> Option<Expr> {
	let math = |f: &str, args: Vec<Expr>| float.round(Expr::ident("Math").member(f).call(args));
	let rs = |f: &str, args: Vec<Expr>| float.func(f).call(args);
	let one = |args: Vec<Expr>| <[Expr; 1]>::try_from(args).ok().map(|[x]| x);
	Some(match name {
		// Exact in JS as in std.
		"floor" | "ceil" | "trunc" | "abs" => Expr::ident("Math").member(name).call(args),
		// Correctly rounded, so a single's is its double's rounded once.
		"sqrt" => math(name, args),
		// libm's in Rust, the engine's in JS: each within a few ULPs, the README says how many.
		"exp" | "log2" | "log10" | "cbrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "sinh" | "cosh"
		| "tanh" | "asinh" | "acosh" | "atan2" | "hypot" => math(name, args),
		"ln" => math("log", args),
		"exp_m1" => math("expm1", args),
		"ln_1p" => math("log1p", args),
		"exp2" => math("pow", vec![Expr::lit("2"), one(args)?]),
		// std divides the two `ln`s, each of the type.
		"log" => {
			let [x, base] = <[Expr; 2]>::try_from(args).ok()?;
			float.round(math("log", vec![x]).binary("/", math("log", vec![base])))
		}
		"is_nan" => Expr::ident("Number").member("isNaN").call(args),
		"is_finite" => Expr::ident("Number").member("isFinite").call(args),
		"recip" => float.round(Expr::lit("1").binary("/", one(args)?)),
		// std multiplies by these constants, of the type, rounded once.
		"to_degrees" => {
			let c = match float {
				Float::F32 => lit32(57.295_78_f32),
				Float::F64 => lit(180.0 / std::f64::consts::PI),
			};
			float.round(one(args)?.binary("*", c))
		}
		"to_radians" => {
			let c = match float {
				Float::F32 => lit32(std::f32::consts::PI / 180.0),
				Float::F64 => lit(std::f64::consts::PI / 180.0),
			};
			float.round(one(args)?.binary("*", c))
		}
		"round" | "round_ties_even" | "fract" | "signum" | "copysign" | "is_infinite" | "is_normal"
		| "is_subnormal" | "is_sign_positive" | "is_sign_negative" | "next_up" | "next_down" | "max" | "min"
		| "midpoint" | "to_bits" | "from_bits" | "to_be_bytes" | "to_le_bytes" | "from_be_bytes" | "from_le_bytes"
		| "total_cmp" | "powi" | "powf" | "sin_cos" | "atanh" | "div_euclid" | "rem_euclid" | "clamp" | "mul_add" => {
			rs(&super::camel(name), args)
		}
		// wasm32 is little-endian.
		"to_ne_bytes" => rs("toLeBytes", args),
		"from_ne_bytes" => rs("fromLeBytes", args),
		_ => return None,
	})
}
