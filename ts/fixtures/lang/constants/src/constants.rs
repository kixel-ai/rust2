//! Comparisons whose sides are both constants once a generic function is instantiated, as
//! `size_of::<T>() == 1` is: each operator on each kind of constant, written in place,
//! cast, and held in a temporary, for a `T` of each outcome.

use std::mem::size_of;

/// A type's constant of each kind.
trait Consts {
	const U8: u8;
	const I32: i32;
	const U64: u64;
	const I128: i128;
	const BOOL: bool;
	const CHAR: char;
	const F32: f32;
	const F64: f64;
}

/// Below [`Mid`] in each.
type Low = u8;

/// What each compares with.
type Mid = u16;

/// Above [`Mid`] in each.
type High = u64;

/// The limits, a NaN, and a negative zero.
type Edge = ();

impl Consts for Low {
	const U8: u8 = 1;
	const I32: i32 = -3;
	const U64: u64 = 1;
	const I128: i128 = -1;
	const BOOL: bool = false;
	const CHAR: char = 'a';
	const F32: f32 = -1.5;
	const F64: f64 = -1.5;
}

impl Consts for Mid {
	const U8: u8 = 2;
	const I32: i32 = -2;
	const U64: u64 = 2;
	const I128: i128 = 0;
	const BOOL: bool = true;
	const CHAR: char = 'b';
	const F32: f32 = 0.0;
	const F64: f64 = 0.0;
}

impl Consts for High {
	const U8: u8 = 3;
	const I32: i32 = -1;
	const U64: u64 = 3;
	const I128: i128 = 1;
	const BOOL: bool = true;
	const CHAR: char = 'c';
	const F32: f32 = 1.5;
	const F64: f64 = 1.5;
}

impl Consts for Edge {
	const U8: u8 = u8::MAX;
	const I32: i32 = i32::MIN;
	const U64: u64 = u64::MAX;
	const I128: i128 = i128::MAX;
	const BOOL: bool = false;
	const CHAR: char = char::MAX;
	const F32: f32 = -0.0;
	const F64: f64 = f64::NAN;
}

/// Each operator on `$a` and `$b`, a byte each.
macro_rules! compare {
	($out:ident, $a:expr, $b:expr) => {
		$out.extend([$a == $b, $a != $b, $a < $b, $a <= $b, $a > $b, $a >= $b].map(u8::from))
	};
}

/// `T`'s constants compared with [`Mid`]'s.
fn against<T: Consts>(out: &mut Vec<u8>) {
	compare!(out, size_of::<T>(), 2);
	compare!(out, size_of::<T>(), size_of::<Mid>());
	compare!(out, T::U8, Mid::U8);
	compare!(out, T::I32, Mid::I32);
	compare!(out, T::U64, Mid::U64);
	compare!(out, T::I128, Mid::I128);
	compare!(out, T::BOOL, Mid::BOOL);
	compare!(out, T::CHAR, Mid::CHAR);
	compare!(out, T::F32, Mid::F32);
	compare!(out, T::F64, Mid::F64);
	compare!(out, T::I32 as i128, Mid::I32 as i128);
	compare!(out, T::U8 as char, Mid::U8 as char);
	// Folded into a literal, the quotient compares from its temporary.
	compare!(out, T::U8 / 2, Mid::U8 / 2);
	compare!(out, T::I32 % 2, Mid::I32 % 2);
	out.push(if size_of::<T>() == 2 { 1 } else { 0 });
	out.push(if T::CHAR < 'b' { 3 } else { 4 });
}

/// The constants of each type compared with [`Mid`]'s.
pub fn compares(out: &mut Vec<u8>) {
	against::<Low>(out);
	against::<Mid>(out);
	against::<High>(out);
	against::<Edge>(out);
}
