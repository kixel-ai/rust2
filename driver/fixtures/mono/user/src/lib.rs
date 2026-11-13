//! Calls each of `exec`'s generics 20 ways, behind a feature apiece.

macro_rules! calls {
	($($n:literal)*) => {
		#[cfg(feature = "fat")]
		pub fn fat() -> u64 {
			0 $(+ exec::block(exec::fat(|x| x + $n)))*
		}

		#[cfg(feature = "budgeted")]
		pub fn budgeted() -> u64 {
			0 $(+ exec::block(exec::budgeted(|x| x + $n)))*
		}

		#[cfg(feature = "thin")]
		pub fn thin() -> u64 {
			0 $(+ exec::thin(|x| x + $n))*
		}

		#[cfg(feature = "uneven")]
		pub fn uneven() -> u64 {
			exec::uneven::<exec::Yes, 0>() $(+ exec::uneven::<exec::No, $n>())*
		}
	};
}

calls!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19);

/// A lighter caller, which the error names after `fat`.
#[cfg(feature = "fat")]
pub fn once() -> u64 {
	exec::block(exec::fat(|x| x * 2))
}

/// Sorts 20 ways, so std's sort, which no one here can edit, is copied 20 times.
#[cfg(feature = "sorted")]
pub fn sorted(v: &mut [u64]) {
	macro_rules! sorts {
		($($n:literal)*) => {
			$(v.sort_by_key(|x| x.wrapping_mul($n));)*
		};
	}
	sorts!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19);
}
