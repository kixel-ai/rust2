//! std's operator traits' methods called by path on primitives, which the operators lower
//! to MIR's own operations: of every integer width, `f64`, `f32`, and `bool`, by value and
//! by reference, and their assign forms. Each is written as a call, passed as a function
//! value that a generic body calls, and coerced to a function pointer, and must overflow,
//! panic, and round where its operator does. One operation per case, so a panic hides
//! nothing.

use std::ops::{
	Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign,
	Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

/// `f(a)`, as a generic body calls a function value.
fn one<A, R>(f: impl FnOnce(A) -> R, a: A) -> R {
	f(a)
}

/// `f(a, b)`, as a generic body calls a function value.
fn two<A, B, R>(f: impl FnOnce(A, B) -> R, a: A, b: B) -> R {
	f(a, b)
}

/// `f` applied to the arguments as `form` picks: written as a call, as a function value, or
/// through a function pointer.
macro_rules! by {
	($form:expr, $f:expr, $a:expr) => {
		match $form % 3 {
			0 => $f($a),
			1 => one($f, $a),
			_ => {
				let f: fn(_) -> _ = $f;
				f($a)
			}
		}
	};
	($form:expr, $f:expr, $a:expr, $b:expr) => {
		match $form % 3 {
			0 => $f($a, $b),
			1 => two($f, $a, $b),
			_ => {
				let f: fn(_, _) -> _ = $f;
				f($a, $b)
			}
		}
	};
}

/// `x` after the assign form `f` applies to it and `b`, as `form` picks.
macro_rules! assigned {
	($form:expr, $f:expr, $x:expr, $b:expr) => {{
		let mut x = $x;
		by!($form, $f, &mut x, $b);
		x
	}};
}

/// `Neg` of a signed type, by value and by reference, which an unsigned one lacks.
macro_rules! neg {
	(signed, $t:ty, $form:expr, $a:expr) => {
		Some(match $form / 3 % 2 {
			0 => by!($form, <$t as Neg>::neg, $a),
			_ => by!($form, <&$t as Neg>::neg, &$a),
		})
	};
	(unsigned, $t:ty, $form:expr, $a:expr) => {
		None::<$t>
	};
}

macro_rules! ints {
	($name:ident, $t:ty, $wide:ty, $sign:ident) => {
		/// Apply operator `kind` to operands taken from `data`, in the form its first byte
		/// picks: two operands, then a shift amount, which may pass the width many times over.
		pub fn $name(kind: u8, data: &[u8], out: &mut Vec<u8>) {
			const N: usize = core::mem::size_of::<$t>();
			let take = |at: usize| {
				let mut buf = [0u8; N];
				for (i, b) in buf.iter_mut().enumerate() {
					*b = data.get(at + i).copied().unwrap_or(0);
				}
				<$t>::from_be_bytes(buf)
			};
			let form = data.first().copied().unwrap_or(0);
			let (a, b) = (take(1), take(1 + N));
			let byte = |i: usize| data.get(1 + 2 * N + i).copied().unwrap_or(0);
			let n = u32::from_be_bytes([byte(1), 0, 0, byte(0)]);
			let put = |out: &mut Vec<u8>, v: $t| out.extend_from_slice(&(v as $wide).to_be_bytes());
			let v = match kind % 32 {
				0 => by!(form, <$t as Add>::add, a, b),
				1 => by!(form, <$t as Sub>::sub, a, b),
				2 => by!(form, <$t as Mul>::mul, a, b),
				3 => by!(form, <$t as Div>::div, a, b),
				4 => by!(form, <$t as Rem>::rem, a, b),
				5 => by!(form, <$t as BitAnd>::bitand, a, b),
				6 => by!(form, <$t as BitOr>::bitor, a, b),
				7 => by!(form, <$t as BitXor>::bitxor, a, b),
				8 => by!(form, <$t as Shl<u32>>::shl, a, n),
				9 => by!(form, <$t as Shr<u32>>::shr, a, n),
				// Amounts of other types: a negative one, and one past 32 bits.
				10 => by!(form, <$t as Shl<i16>>::shl, a, n as i16),
				11 => by!(form, <$t as Shr<u128>>::shr, a, u128::from(n) << (n & 64)),
				12 => by!(form, <$t as Not>::not, a),
				13 => match neg!($sign, $t, form, a) {
					Some(v) => v,
					None => return,
				},
				14 => assigned!(form, <$t as AddAssign>::add_assign, a, b),
				15 => assigned!(form, <$t as SubAssign>::sub_assign, a, b),
				16 => assigned!(form, <$t as MulAssign>::mul_assign, a, b),
				17 => assigned!(form, <$t as DivAssign>::div_assign, a, b),
				18 => assigned!(form, <$t as RemAssign>::rem_assign, a, b),
				19 => assigned!(form, <$t as BitAndAssign>::bitand_assign, a, b),
				20 => assigned!(form, <$t as BitOrAssign>::bitor_assign, a, b),
				21 => assigned!(form, <$t as BitXorAssign>::bitxor_assign, a, b),
				22 => assigned!(form, <$t as ShlAssign<u32>>::shl_assign, a, n),
				23 => assigned!(form, <$t as ShrAssign<i64>>::shr_assign, a, i64::from(n as i16)),
				// By reference, on either side.
				24 => by!(form, <$t as Add<&$t>>::add, a, &b),
				25 => by!(form, <&$t as Sub<$t>>::sub, &a, b),
				26 => by!(form, <&$t as Mul<&$t>>::mul, &a, &b),
				27 => by!(form, <&$t as Shl<&u8>>::shl, &a, &(n as u8)),
				28 => by!(form, <&$t as Not>::not, &a),
				29 => assigned!(form, <$t as AddAssign<&$t>>::add_assign, a, &b),
				30 => assigned!(form, <$t as DivAssign<&$t>>::div_assign, a, &b),
				_ => assigned!(form, <$t as ShrAssign<&u16>>::shr_assign, a, &(n as u16)),
			};
			put(out, v);
		}
	};
}

ints!(u8s, u8, u128, unsigned);
ints!(u16s, u16, u128, unsigned);
ints!(u32s, u32, u128, unsigned);
ints!(u64s, u64, u128, unsigned);
ints!(u128s, u128, u128, unsigned);
ints!(usizes, usize, u128, unsigned);
ints!(i8s, i8, i128, signed);
ints!(i16s, i16, i128, signed);
ints!(i32s, i32, i128, signed);
ints!(i64s, i64, i128, signed);
ints!(i128s, i128, i128, signed);
ints!(isizes, isize, i128, signed);

macro_rules! floats {
	($name:ident, $t:ty) => {
		/// Apply operator `kind` to two doubles' bits taken from `data`, rounded to the type,
		/// in the form its first byte picks.
		pub fn $name(kind: u8, data: &[u8], out: &mut Vec<u8>) {
			let take = |at: usize| {
				let mut buf = [0u8; 8];
				for (i, b) in buf.iter_mut().enumerate() {
					*b = data.get(at + i).copied().unwrap_or(0);
				}
				f64::from_bits(u64::from_be_bytes(buf)) as $t
			};
			let form = data.first().copied().unwrap_or(0);
			let (a, b) = (take(1), take(9));
			let v = match kind % 16 {
				0 => by!(form, <$t as Add>::add, a, b),
				1 => by!(form, <$t as Sub>::sub, a, b),
				2 => by!(form, <$t as Mul>::mul, a, b),
				3 => by!(form, <$t as Div>::div, a, b),
				4 => by!(form, <$t as Rem>::rem, a, b),
				5 => by!(form, <$t as Neg>::neg, a),
				6 => assigned!(form, <$t as AddAssign>::add_assign, a, b),
				7 => assigned!(form, <$t as SubAssign>::sub_assign, a, b),
				8 => assigned!(form, <$t as MulAssign>::mul_assign, a, b),
				9 => assigned!(form, <$t as DivAssign>::div_assign, a, b),
				10 => assigned!(form, <$t as RemAssign>::rem_assign, a, b),
				11 => by!(form, <&$t as Add<&$t>>::add, &a, &b),
				12 => by!(form, <$t as Rem<&$t>>::rem, a, &b),
				13 => by!(form, <&$t as Neg>::neg, &a),
				14 => assigned!(form, <$t as MulAssign<&$t>>::mul_assign, a, &b),
				_ => assigned!(form, <$t as RemAssign<&$t>>::rem_assign, a, &b),
			};
			// Any NaN as the type's own, whose payload no operator here keeps.
			let v = if v.is_nan() { <$t>::NAN } else { v };
			out.extend_from_slice(&v.to_bits().to_be_bytes());
		}
	};
}

floats!(f64s, f64);
floats!(f32s, f32);

/// Apply operator `kind` to the two bools the low bits of `data`'s second byte pick, in the
/// form its first byte picks.
pub fn bools(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let form = data.first().copied().unwrap_or(0);
	let bits = data.get(1).copied().unwrap_or(0);
	let (a, b) = (bits & 1 == 1, bits & 2 == 2);
	let v = match kind % 10 {
		0 => by!(form, <bool as BitAnd>::bitand, a, b),
		1 => by!(form, <bool as BitOr>::bitor, a, b),
		2 => by!(form, <bool as BitXor>::bitxor, a, b),
		3 => by!(form, <bool as Not>::not, a),
		4 => assigned!(form, <bool as BitAndAssign>::bitand_assign, a, b),
		5 => assigned!(form, <bool as BitOrAssign>::bitor_assign, a, b),
		6 => assigned!(form, <bool as BitXorAssign>::bitxor_assign, a, b),
		7 => by!(form, <&bool as BitAnd<&bool>>::bitand, &a, &b),
		8 => by!(form, <&bool as Not>::not, &a),
		_ => assigned!(form, <bool as BitOrAssign<&bool>>::bitor_assign, a, &b),
	};
	out.push(v as u8);
}
