//! Closures that write what they captured and return it, read after by the caller or by
//! the closure's next call.

/// Take an integer's big-endian bytes from the front of `data`, zero-padded.
macro_rules! take {
	($t:ty, $data:expr) => {{
		let data: &[u8] = $data;
		let mut buf = [0u8; core::mem::size_of::<$t>()];
		let n = data.len().min(buf.len());
		buf[..n].copy_from_slice(&data[..n]);
		(<$t>::from_be_bytes(buf), &data[n..])
	}};
}

macro_rules! writes {
	($name:ident, $t:ty) => {
		/// Each way a closure writes a captured integer, from two operands taken from `data`.
		fn $name(data: &[u8], out: &mut Vec<u8>) {
			let (a, data) = take!($t, data);
			let (b, _) = take!($t, data);
			let mut put = |v: $t| out.extend_from_slice(&v.to_be_bytes());

			// By reference, called once and twice.
			let mut n = a;
			let mut bump = || {
				n = n.wrapping_add(b);
				n
			};
			put(bump());
			put(n);
			let mut bump = || {
				n = n.wrapping_mul(b);
				n
			};
			put(bump());
			put(bump());
			put(n);

			// Only on one branch.
			let mut maybe = || {
				if b & 1 == 1 {
					n = n.wrapping_sub(a);
					n
				} else {
					n
				}
			};
			put(maybe());
			put(n);

			// An `FnOnce`, which moves what it owns out.
			let owned = data.to_vec();
			let consume = || {
				n = n.wrapping_add(owned.len() as $t);
				drop(owned);
				n
			};
			put(consume());
			put(n);

			// Through a type parameter.
			put(once(|| {
				n = n.wrapping_add(b);
				n
			}));
			put(n);
			let (x, y) = twice(|| {
				n = n.wrapping_add(a);
				n
			});
			put(x);
			put(y);
			put(n);

			// By value, so the closure's copy changes and the caller's doesn't.
			let mut m = a;
			let mut own = move || {
				m = m.wrapping_add(b);
				m
			};
			put(own());
			put(own());
			put(m);

			// A field, captured alone.
			let mut pair = (a, b);
			let mut first = || {
				pair.0 = pair.0.wrapping_add(pair.1);
				pair.0
			};
			put(first());
			put(pair.0);

			// What a `&mut` parameter points at.
			fn through(n: &mut $t, b: $t) -> $t {
				let mut add = || {
					*n = n.wrapping_add(b);
					*n
				};
				add()
			}
			put(through(&mut m, b));
			put(m);

			// From a closure inside another.
			let mut outer = || {
				let mut inner = || {
					n = n.wrapping_sub(b);
					n
				};
				inner()
			};
			put(outer());
			put(n);
		}
	};
}

writes!(u8s, u8);
writes!(u16s, u16);
writes!(u32s, u32);
writes!(u64s, u64);
writes!(u128s, u128);
writes!(usizes, usize);
writes!(i8s, i8);
writes!(i16s, i16);
writes!(i32s, i32);
writes!(i64s, i64);
writes!(i128s, i128);
writes!(isizes, isize);

/// Run [`writes`] over the integer type `kind` picks.
pub fn captures(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 12 {
		0 => u8s(data, out),
		1 => u16s(data, out),
		2 => u32s(data, out),
		3 => u64s(data, out),
		4 => u128s(data, out),
		5 => usizes(data, out),
		6 => i8s(data, out),
		7 => i16s(data, out),
		8 => i32s(data, out),
		9 => i64s(data, out),
		10 => i128s(data, out),
		_ => isizes(data, out),
	}
}

fn once<T, F: FnOnce() -> T>(f: F) -> T {
	f()
}

fn twice<T, F: FnMut() -> T>(mut f: F) -> (T, T) {
	(f(), f())
}
