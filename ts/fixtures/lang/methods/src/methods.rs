//! Every integer width's inherent methods, and those of a `NonZero`, a `Wrapping`, and a
//! `Saturating` of one, which are their integer's. A method that may panic is its own
//! case, so a panic hides nothing; the rest share one.

use std::num::{NonZero, Saturating, Wrapping};

macro_rules! methods {
	($name:ident, $t:ty, $other:ty, $sign:ident) => {
		/// Apply method `kind` to operands taken from `data`: two of the type, and a `u32`.
		pub fn $name(kind: u8, data: &[u8], out: &mut Vec<u8>) {
			const N: usize = core::mem::size_of::<$t>();
			let byte = |i: usize| data.get(i).copied().unwrap_or(0);
			let a = <$t>::from_be_bytes(core::array::from_fn(|i| byte(i)));
			let b = <$t>::from_be_bytes(core::array::from_fn(|i| byte(N + i)));
			let n = u32::from_be_bytes(core::array::from_fn(|i| byte(2 * N + i)));
			// `b` as the type of the other sign, which the `_signed` and `_unsigned` forms take.
			let o = b as $other;
			// An exponent, a shift, or a rotation within a few widths.
			let k = n % (3 * <$t>::BITS);
			let put = |out: &mut Vec<u8>, v: $t| {
				out.extend_from_slice(&v.to_be_bytes());
				// A wide value equals its bytes read back only in its one form.
				out.push((<$t>::from_be_bytes(v.to_be_bytes()) == v) as u8);
			};
			let put_opt = |out: &mut Vec<u8>, v: Option<$t>| match v {
				Some(v) => {
					out.push(1);
					put(out, v);
				}
				None => out.push(0),
			};
			let put_pair = |out: &mut Vec<u8>, (v, over): ($t, bool)| {
				put(out, v);
				out.push(over as u8);
			};
			let put_u32 = |out: &mut Vec<u8>, v: u32| out.extend_from_slice(&v.to_be_bytes());
			let put_opt_u32 = |out: &mut Vec<u8>, v: Option<u32>| match v {
				Some(v) => {
					out.push(1);
					put_u32(out, v);
				}
				None => out.push(0),
			};
			match kind % 42 {
				0 => {
					put_opt(out, a.checked_add(b));
					put_opt(out, a.checked_sub(b));
					put_opt(out, a.checked_mul(b));
					put(out, a.saturating_add(b));
					put(out, a.saturating_sub(b));
					put(out, a.saturating_mul(b));
					put(out, a.wrapping_add(b));
					put(out, a.wrapping_sub(b));
					put(out, a.wrapping_mul(b));
					put_pair(out, a.overflowing_add(b));
					put_pair(out, a.overflowing_sub(b));
					put_pair(out, a.overflowing_mul(b));
					put(out, a.midpoint(b));
					methods!(@$sign mixed, a, o, out, put, put_opt, put_pair);
				}
				1 => {
					put_opt(out, a.checked_div(b));
					put_opt(out, a.checked_rem(b));
					put_opt(out, a.checked_div_euclid(b));
					put_opt(out, a.checked_rem_euclid(b));
					methods!(@$sign divisions, a, b, out, put, put_opt, put_pair);
				}
				2 => {
					put_opt(out, a.checked_neg());
					put(out, a.wrapping_neg());
					put_pair(out, a.overflowing_neg());
					methods!(@$sign unary, $t, a, out, put, put_opt, put_pair, put_u32, put_opt_u32);
				}
				3 => {
					for e in [n, k] {
						put_opt(out, a.checked_pow(e));
						put(out, a.wrapping_pow(e));
						put(out, a.saturating_pow(e));
						put_pair(out, a.overflowing_pow(e));
					}
				}
				4 => {
					for s in [n, k] {
						put_opt(out, a.checked_shl(s));
						put_opt(out, a.checked_shr(s));
						put(out, a.wrapping_shl(s));
						put(out, a.wrapping_shr(s));
						put_pair(out, a.overflowing_shl(s));
						put_pair(out, a.overflowing_shr(s));
						put(out, a.unbounded_shl(s));
						put(out, a.unbounded_shr(s));
						put(out, a.rotate_left(s));
						put(out, a.rotate_right(s));
					}
				}
				5 => {
					for c in [a.count_ones(), a.count_zeros(), a.leading_zeros(), a.trailing_zeros(), a.leading_ones(), a.trailing_ones()] {
						put_u32(out, c);
					}
					for v in [a.swap_bytes(), a.reverse_bits(), a.to_be(), a.to_le(), <$t>::from_be(a), <$t>::from_le(a)] {
						put(out, v);
					}
					put_opt_u32(out, a.highest_one());
					put_opt_u32(out, a.lowest_one());
					put(out, a.isolate_highest_one());
					put(out, a.isolate_lowest_one());
					#[allow(deprecated, reason = "the deprecated methods translate too")]
					{
						put(out, <$t>::max_value());
						put(out, <$t>::min_value());
					}
				}
				6 => {
					put_opt_u32(out, a.checked_ilog(b));
					put_opt_u32(out, a.checked_ilog2());
					put_opt_u32(out, a.checked_ilog10());
					let d = a.abs_diff(b);
					out.extend_from_slice(&d.to_be_bytes());
				}
				7 => put(out, a.pow(k)),
				8 => put(out, a.pow(n)),
				9 => put(out, a.div_euclid(b)),
				10 => put(out, a.rem_euclid(b)),
				11 => put(out, a.isqrt()),
				12 => put_u32(out, a.ilog(b)),
				13 => put_u32(out, a.ilog2()),
				14 => put_u32(out, a.ilog10()),
				15 => put(out, a.strict_add(b)),
				16 => put(out, a.strict_sub(b)),
				17 => put(out, a.strict_mul(b)),
				18 => put(out, a.strict_div(b)),
				19 => put(out, a.strict_rem(b)),
				20 => put(out, a.strict_neg()),
				21 => put(out, a.strict_shl(k)),
				22 => put(out, a.strict_shr(k)),
				23 => put(out, a.strict_pow(k)),
				24 => put(out, a.strict_div_euclid(b)),
				25 => put(out, a.strict_rem_euclid(b)),
				26 => methods!(@$sign strict_add, a, o, out, put),
				27 => methods!(@$sign strict_sub, a, o, out, put),
				28 => put(out, a.wrapping_div(b)),
				29 => put(out, a.wrapping_rem(b)),
				30 => put(out, a.wrapping_div_euclid(b)),
				31 => put(out, a.wrapping_rem_euclid(b)),
				32 => put_pair(out, a.overflowing_div(b)),
				33 => put_pair(out, a.overflowing_rem(b)),
				34 => put_pair(out, a.overflowing_div_euclid(b)),
				35 => put_pair(out, a.overflowing_rem_euclid(b)),
				36 => put(out, a.saturating_div(b)),
				37 => methods!(@$sign panics, a, b, k, out, put),
				38 => methods!(@$sign more, a, b, k, out, put),
				39 => {
					// A `Saturating`'s and a `Wrapping`'s, which are their integer's but for how
					// each overflows.
					let (s, w) = (Saturating(a), Wrapping(a));
					for c in [s.count_ones(), s.count_zeros(), s.leading_zeros(), s.trailing_zeros()] {
						put_u32(out, c);
					}
					for c in [w.count_ones(), w.count_zeros(), w.leading_zeros(), w.trailing_zeros()] {
						put_u32(out, c);
					}
					for v in [s.rotate_left(k), s.rotate_right(n), s.swap_bytes(), s.reverse_bits(), s.to_be(), s.to_le(), Saturating::<$t>::from_be(s), Saturating::<$t>::from_le(s), s.pow(k), s.pow(n)] {
						put(out, v.0);
					}
					for v in [w.rotate_left(k), w.rotate_right(n), w.swap_bytes(), w.reverse_bits(), w.to_be(), w.to_le(), Wrapping::<$t>::from_be(w), Wrapping::<$t>::from_le(w), w.pow(k), w.pow(n)] {
						put(out, v.0);
					}
					methods!(@$sign wrappers, s, w, out, put);
				}
				40 => {
					// A `NonZero`'s, which are its integer's.
					if let (Some(x), Some(y)) = (NonZero::new(a), NonZero::new(b)) {
						for c in [x.leading_zeros(), x.trailing_zeros(), x.count_ones().get(), x.highest_one(), x.lowest_one()] {
							put_u32(out, c);
						}
						put_opt(out, x.checked_mul(y).map(NonZero::get));
						put(out, x.saturating_mul(y).get());
						put_opt(out, x.checked_pow(k).map(NonZero::get));
						put_opt(out, x.checked_pow(n).map(NonZero::get));
						put(out, x.saturating_pow(k).get());
						put(out, x.isolate_highest_one().get());
						put(out, x.isolate_lowest_one().get());
						methods!(@$sign nonzero, x, y, b, out, put, put_opt, put_pair, put_u32);
					}
				}
				_ => methods!(@$sign nonzero_panics, a, out, put),
			}
		}
	};

	// What only a signed type has, or only an unsigned one.
	(@signed mixed, $a:ident, $o:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident) => {{
		$put_opt($out, $a.checked_add_unsigned($o));
		$put_opt($out, $a.checked_sub_unsigned($o));
		$put($out, $a.saturating_add_unsigned($o));
		$put($out, $a.saturating_sub_unsigned($o));
		$put($out, $a.wrapping_add_unsigned($o));
		$put($out, $a.wrapping_sub_unsigned($o));
		$put_pair($out, $a.overflowing_add_unsigned($o));
		$put_pair($out, $a.overflowing_sub_unsigned($o));
	}};
	(@unsigned mixed, $a:ident, $o:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident) => {{
		$put_opt($out, $a.checked_add_signed($o));
		$put_opt($out, $a.checked_sub_signed($o));
		$put($out, $a.saturating_add_signed($o));
		$put($out, $a.saturating_sub_signed($o));
		$put($out, $a.wrapping_add_signed($o));
		$put($out, $a.wrapping_sub_signed($o));
		$put_pair($out, $a.overflowing_add_signed($o));
		$put_pair($out, $a.overflowing_sub_signed($o));
		let b = $o.cast_unsigned();
		match $a.checked_signed_diff(b) {
			Some(d) => $out.extend_from_slice(&d.to_be_bytes()),
			None => $out.push(0),
		}
		for carry in [false, true] {
			$put_pair($out, $a.carrying_add(b, carry));
			$put_pair($out, $a.borrowing_sub(b, carry));
		}
		let (lo, hi) = $a.carrying_mul(b, $a);
		$put($out, lo);
		$put($out, hi);
		let (lo, hi) = $a.carrying_mul_add(b, b, $a);
		$put($out, lo);
		$put($out, hi);
		$put_opt($out, $a.checked_next_multiple_of(b));
	}};
	(@signed divisions, $a:ident, $b:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident) => {
		if $b != 0 {
			$put($out, $a.wrapping_div($b));
			$put($out, $a.wrapping_rem($b));
			$put_pair($out, $a.overflowing_div_euclid($b));
			$put_pair($out, $a.overflowing_rem_euclid($b));
		}
	};
	(@unsigned divisions, $a:ident, $b:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident) => {
		if $b != 0 {
			$put($out, $a.div_ceil($b));
			$out.push($a.is_multiple_of($b) as u8);
		}
	};
	(@signed unary, $t:ty, $a:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident, $put_u32:ident, $put_opt_u32:ident) => {{
		$put_opt($out, $a.checked_abs());
		$put($out, $a.wrapping_abs());
		$put($out, $a.saturating_abs());
		$put_pair($out, $a.overflowing_abs());
		$put($out, $a.saturating_neg());
		$put($out, $a.signum());
		$out.push($a.is_positive() as u8);
		$out.push($a.is_negative() as u8);
		$put_opt($out, $a.checked_isqrt());
		$out.extend_from_slice(&$a.unsigned_abs().to_be_bytes());
		$out.extend_from_slice(&$a.cast_unsigned().to_be_bytes());
	}};
	(@unsigned unary, $t:ty, $a:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident, $put_u32:ident, $put_opt_u32:ident) => {{
		$out.push($a.is_power_of_two() as u8);
		$put_opt($out, $a.checked_next_power_of_two());
		$put_u32($out, $a.bit_width());
		$out.extend_from_slice(&$a.cast_signed().to_be_bytes());
	}};
	(@signed strict_add, $a:ident, $o:ident, $out:ident, $put:ident) => {
		$put($out, $a.strict_add_unsigned($o))
	};
	(@unsigned strict_add, $a:ident, $o:ident, $out:ident, $put:ident) => {
		$put($out, $a.strict_add_signed($o))
	};
	(@signed strict_sub, $a:ident, $o:ident, $out:ident, $put:ident) => {
		$put($out, $a.strict_sub_unsigned($o))
	};
	(@unsigned strict_sub, $a:ident, $o:ident, $out:ident, $put:ident) => {
		$put($out, $a.strict_sub_signed($o))
	};
	(@signed panics, $a:ident, $b:ident, $k:ident, $out:ident, $put:ident) => {
		$put($out, $a.abs())
	};
	(@unsigned panics, $a:ident, $b:ident, $k:ident, $out:ident, $put:ident) => {
		$put($out, $a.next_power_of_two())
	};
	(@signed more, $a:ident, $b:ident, $k:ident, $out:ident, $put:ident) => {
		$put($out, $a.strict_abs())
	};
	(@unsigned more, $a:ident, $b:ident, $k:ident, $out:ident, $put:ident) => {{
		// One panics per case: each of these only when its operand is the one picked.
		match $k % 3 {
			0 => $put($out, $a.next_multiple_of($b)),
			1 => $put($out, $a.funnel_shl($b, $k)),
			_ => $put($out, $a.funnel_shr($b, $k)),
		}
	}};
	(@signed wrappers, $s:ident, $w:ident, $out:ident, $put:ident) => {{
		for v in [$s.abs(), $s.signum()] {
			$put($out, v.0);
		}
		for v in [$w.abs(), $w.signum()] {
			$put($out, v.0);
		}
		$out.extend_from_slice(&[$s.is_positive() as u8, $s.is_negative() as u8, $w.is_positive() as u8, $w.is_negative() as u8]);
	}};
	(@unsigned wrappers, $s:ident, $w:ident, $out:ident, $put:ident) => {{
		$out.extend_from_slice(&[$s.is_power_of_two() as u8, $w.is_power_of_two() as u8]);
		$put($out, $w.next_power_of_two().0);
	}};
	(@signed nonzero, $x:ident, $y:ident, $b:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident, $put_u32:ident) => {{
		$put_opt($out, $x.checked_abs().map(NonZero::get));
		let (v, over) = $x.overflowing_abs();
		$put_pair($out, (v.get(), over));
		$put($out, $x.saturating_abs().get());
		$put($out, $x.wrapping_abs().get());
		$out.extend_from_slice(&$x.unsigned_abs().get().to_be_bytes());
		$out.extend_from_slice(&[$x.is_positive() as u8, $x.is_negative() as u8]);
		$put_opt($out, $x.checked_neg().map(NonZero::get));
		let (v, over) = $x.overflowing_neg();
		$put_pair($out, (v.get(), over));
		$put($out, $x.saturating_neg().get());
		$put($out, $x.wrapping_neg().get());
		$out.extend_from_slice(&$x.cast_unsigned().get().to_be_bytes());
	}};
	(@unsigned nonzero, $x:ident, $y:ident, $b:ident, $out:ident, $put:ident, $put_opt:ident, $put_pair:ident, $put_u32:ident) => {{
		$out.push($x.is_power_of_two() as u8);
		$put_opt($out, $x.checked_add($b).map(NonZero::get));
		$put($out, $x.saturating_add($b).get());
		$put_opt($out, $x.checked_next_power_of_two().map(NonZero::get));
		for c in [$x.ilog2(), $x.ilog10(), $x.bit_width().get()] {
			$put_u32($out, c);
		}
		$put($out, $x.midpoint($y).get());
		$put($out, $x.isqrt().get());
		$put($out, $x.div_ceil($y).get());
		$out.extend_from_slice(&$x.cast_signed().get().to_be_bytes());
	}};
	(@signed nonzero_panics, $a:ident, $out:ident, $put:ident) => {
		if let Some(x) = NonZero::new($a) {
			$put($out, x.abs().get());
		}
	};
	(@unsigned nonzero_panics, $a:ident, $out:ident, $put:ident) => {
		$out.push(0)
	};
}

methods!(u8s, u8, i8, unsigned);
methods!(u16s, u16, i16, unsigned);
methods!(u32s, u32, i32, unsigned);
methods!(u64s, u64, i64, unsigned);
methods!(u128s, u128, i128, unsigned);
methods!(usizes, usize, isize, unsigned);
methods!(i8s, i8, u8, signed);
methods!(i16s, i16, u16, signed);
methods!(i32s, i32, u32, signed);
methods!(i64s, i64, u64, signed);
methods!(i128s, i128, u128, signed);
methods!(isizes, isize, usize, signed);
