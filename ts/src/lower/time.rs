//! `std::time::Duration`, as `rs/time.ts` gives it: std's seconds and nanoseconds in an
//! object that never changes, so its arithmetic, overflow, and ordering are std's.

use rustc_middle::ty::{self, Ty, ValTree};

use super::body::Body;
use super::library::{Call, Std};
use super::num::Int;
use crate::ts::{Expr, Key};

const U64: Int = Int {
	bits: 64,
	signed: false,
	pointer: false,
};

/// A call of a `Duration` function, or its arguments back if it is none of theirs.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let is = |n: usize| {
		call.args
			.types()
			.nth(n)
			.is_some_and(|t| b.t.std_of(t.peel_refs()) == Some(Std::Duration))
	};
	let f = match (owner, name) {
		("Duration", _) => match name {
			"new" => "durationNew",
			"from_secs" => "durationFromSecs",
			"from_millis" => "durationFromMillis",
			"from_micros" => "durationFromMicros",
			"from_nanos" => "durationFromNanos",
			"from_nanos_u128" => "durationFromNanosU128",
			"from_mins" => "durationFromMins",
			"from_hours" => "durationFromHours",
			"is_zero" => "durationIsZero",
			"as_secs" => "durationAsSecs",
			"subsec_millis" => "durationSubsecMillis",
			"subsec_micros" => "durationSubsecMicros",
			"subsec_nanos" => "durationSubsecNanos",
			"as_millis" => "durationAsMillis",
			"as_micros" => "durationAsMicros",
			"as_nanos" => "durationAsNanos",
			"checked_add" => "durationCheckedAdd",
			"checked_sub" => "durationCheckedSub",
			"checked_mul" => "durationCheckedMul",
			"checked_div" => "durationCheckedDiv",
			"saturating_add" => "durationSaturatingAdd",
			"saturating_sub" => "durationSaturatingSub",
			"saturating_mul" => "durationSaturatingMul",
			"abs_diff" => "durationAbsDiff",
			"as_secs_f64" => "durationAsSecsF64",
			"from_secs_f64" => "durationFromSecsF64",
			"try_from_secs_f64" => "durationTryFromSecsF64",
			"mul_f64" => "durationMulF64",
			"div_f64" => "durationDivF64",
			"div_duration_f64" => "durationDivDurationF64",
			"as_secs_f32" => "durationAsSecsF32",
			"from_secs_f32" => "durationFromSecsF32",
			"try_from_secs_f32" => "durationTryFromSecsF32",
			"mul_f32" => "durationMulF32",
			"div_f32" => "durationDivF32",
			"div_duration_f32" => "durationDivDurationF32",
			_ => return Ok(None),
		},
		("Add", "add") if is(0) => "durationAdd",
		("Sub", "sub") if is(0) => "durationSub",
		("Mul", "mul") if is(0) => "durationMul",
		// `n * d`, which std computes as `d * n`.
		("Mul", "mul") if is(1) => "durationTimes",
		("Div", "div") if is(0) => "durationDiv",
		_ => return Err(args),
	};
	Ok(Some(Expr::rs(f, args)))
}

/// `Duration::ZERO`, which `Default` makes.
pub fn zero() -> Expr {
	object(U64.lit(0), Expr::lit("0"))
}

/// A `Duration` constant from its value tree: its seconds, and its `Nanoseconds`, which
/// holds its `u32` in a pattern type.
pub fn constant<'tcx>(fields: &[(Ty<'tcx>, ValTree<'tcx>)]) -> Option<Expr> {
	let [(_, secs), (_, nanos)] = fields else {
		return None;
	};
	let secs = secs.try_to_leaf()?.to_u64();
	let ty::ValTreeKind::Branch(inner) = &**nanos else {
		return None;
	};
	let [inner] = &inner[..] else {
		return None;
	};
	let ty::ConstKind::Value(inner) = inner.kind() else {
		return None;
	};
	let nanos = inner.valtree.try_to_leaf()?.to_u32();
	Some(object(U64.lit(i128::from(secs)), Expr::lit(nanos.to_string())))
}

/// A `Duration` of `secs` whole seconds and `nanos` nanoseconds.
pub fn object(secs: Expr, nanos: Expr) -> Expr {
	Expr::Object(vec![
		(Key::Name("secs".into()), secs),
		(Key::Name("nanos".into()), nanos),
	])
}
