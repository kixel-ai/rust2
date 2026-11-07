//! `Vec`s adopting memory by `from_raw_parts`, with its length and capacity: memory from
//! `alloc` laid out as an array, grown by `realloc` or not, and a `Vec`'s own, given up by
//! `ManuallyDrop`. Each drops what it holds and frees the memory as std's does.

use std::alloc::{self, Layout};
use std::mem::{self, ManuallyDrop};
use std::ptr::NonNull;
use std::sync::Arc;

/// Memory from `alloc` for `n` `T`s, laid out by `Layout::array`, or by an equal layout of
/// `T`'s size and alignment, grown from `n / 2 + 1` by `realloc` if `grown`.
fn slots<T>(n: usize, equal: bool, grown: bool) -> *mut T {
	let layout = |n: usize| match equal {
		false => Layout::array::<T>(n).unwrap(),
		true => Layout::from_size_align(mem::size_of::<T>() * n, mem::align_of::<T>()).unwrap(),
	};
	unsafe {
		if grown {
			let first = layout(n / 2 + 1);
			let p = alloc::alloc(first);
			NonNull::new(alloc::realloc(p, first, layout(n).size()))
				.unwrap()
				.cast()
				.as_ptr()
		} else {
			NonNull::new(alloc::alloc(layout(n))).unwrap().cast().as_ptr()
		}
	}
}

/// A step on an adopted `Vec`, by `b`, which `make` gives values for: pushes, a `set_len`
/// over the slots written past its length, a truncate, or a shrink.
fn step<T>(v: &mut Vec<T>, written: usize, b: u8, make: impl Fn(usize) -> T) {
	match b % 4 {
		0 => (0..usize::from(b >> 2) % 4).for_each(|i| v.push(make(i + 100))),
		// SAFETY: the slots up to `written` hold what was written there before the `Vec`
		// adopted them, and no step since touched them.
		1 => unsafe { v.set_len(written) },
		2 => v.truncate(usize::from(b >> 2) % 3),
		_ => v.shrink_to_fit(),
	}
}

/// `Vec`s of `u32`s, of `Arc`s, whose strong counts show each drop, and of `Option` pairs,
/// adopting memory from `alloc` of `cap` slots, `written` of them written, `len` of which the
/// `Vec` holds, and what each holds and its capacity before and after a step.
pub fn allocated(data: &[u8], out: &mut Vec<u8>) {
	let shared = Arc::new(7u8);
	for part in data.chunks(4) {
		let [a, b, c, d] = std::array::from_fn(|i| part.get(i).copied().unwrap_or(0));
		let cap = usize::from(a % 6) + 1;
		let len = usize::from(b) % (cap + 1);
		let written = len + usize::from(b >> 4) % (cap - len + 1);
		let (equal, grown) = (a & 0x40 != 0, a & 0x80 != 0);

		let ints = slots::<u32>(cap, equal, grown);
		(0..written).for_each(|i| unsafe { ints.add(i).write(u32::from(c) * 1000 + i as u32) });
		let mut ints = unsafe { Vec::from_raw_parts(ints, len, cap) };
		out.extend([ints.len() as u8, ints.capacity() as u8]);
		step(&mut ints, written, d, |i| i as u32);
		out.extend([ints.len() as u8, ints.capacity() as u8]);
		out.extend(ints.iter().flat_map(|x| x.to_le_bytes()));

		let arcs = slots::<Arc<u8>>(cap, equal, grown);
		(0..written).for_each(|i| unsafe { arcs.add(i).write(shared.clone()) });
		let mut arcs = unsafe { Vec::from_raw_parts(arcs, len, cap) };
		out.push(Arc::strong_count(&shared) as u8);
		step(&mut arcs, written, d >> 1, |_| shared.clone());
		out.extend([
			arcs.len() as u8,
			arcs.capacity() as u8,
			Arc::strong_count(&shared) as u8,
		]);
		drop(arcs);
		out.push(Arc::strong_count(&shared) as u8);

		let pairs = slots::<Option<(u8, u16)>>(cap, !equal, false);
		(0..written).for_each(|i| unsafe { pairs.add(i).write((c & 1 << i != 0).then_some((c, i as u16))) });
		let mut pairs = unsafe { Vec::from_raw_parts(pairs, len, cap) };
		step(&mut pairs, written, d >> 2, |i| Some((d, i as u16)));
		out.extend([pairs.len() as u8, pairs.capacity() as u8]);
		out.extend(pairs.iter().map(|p| p.map_or(0, |(x, y)| x ^ y as u8)));
	}
}

/// `Vec`s given up by `ManuallyDrop` and adopted again by `from_raw_parts`, with and
/// without room past their length, some of it written through the pointer, one from the
/// pointer to its spare capacity while empty, one grown by `realloc` and adopted again,
/// `Vec`s of a `Box`'s value and of bytes from `alloc`, and the empty `Vec` of a dangling
/// pointer.
pub fn readopted(data: &[u8], out: &mut Vec<u8>) {
	let shared = Arc::new(7u8);
	let n = data.len();
	let extra = usize::from(data.first().copied().unwrap_or(0) % 4);

	let mut ints = Vec::with_capacity(n + extra);
	ints.extend(data.iter().map(|&b| u32::from(b) << 4));
	let mut ints = ManuallyDrop::new(ints);
	let p = ints.as_mut_ptr();
	(n..n + extra).for_each(|i| unsafe { p.add(i).write(i as u32) });
	let mut ints = unsafe { Vec::from_raw_parts(p, n, ints.capacity()) };
	out.extend([ints.len() as u8, ints.capacity() as u8]);
	unsafe { ints.set_len(n + extra) };
	ints.push(9);
	out.extend([ints.len() as u8, ints.capacity() as u8]);
	out.extend(ints.iter().flat_map(|x| x.to_le_bytes()));

	let arcs: Vec<_> = data.iter().map(|_| shared.clone()).collect();
	let mut arcs = ManuallyDrop::new(arcs);
	let (p, len, cap) = (arcs.as_mut_ptr(), arcs.len(), arcs.capacity());
	let arcs = unsafe { Vec::from_raw_parts(p, len / 2, cap) };
	out.extend([
		arcs.len() as u8,
		arcs.capacity() as u8,
		Arc::strong_count(&shared) as u8,
	]);
	drop(arcs);
	out.push(Arc::strong_count(&shared) as u8);

	let mut bytes = ManuallyDrop::new(data.to_vec());
	bytes.reserve(extra);
	let (p, len, cap) = (bytes.as_mut_ptr(), bytes.len(), bytes.capacity());
	let mut bytes = unsafe { Vec::from_raw_parts(p, len, cap) };
	bytes.push(1);
	out.extend([bytes.len() as u8, bytes.capacity() as u8]);
	out.extend_from_slice(&bytes);

	let mut spare = ManuallyDrop::new(Vec::<u32>::with_capacity(n + 1));
	let cap = spare.capacity();
	let p = spare.spare_capacity_mut().as_mut_ptr().cast::<u32>();
	(0..cap).for_each(|i| unsafe { p.add(i).write(i as u32 * 3) });
	let spare = unsafe { Vec::from_raw_parts(p, n / 2 + 1, cap) };
	out.extend([spare.len() as u8, spare.capacity() as u8]);
	out.extend(spare.iter().flat_map(|x| x.to_le_bytes()));

	let p = slots::<u32>(2, false, false);
	unsafe { (p.write(10 + n as u32), p.add(1).write(20)) };
	let mut kept = ManuallyDrop::new(unsafe { Vec::from_raw_parts(p, 1, 2) });
	let old = Layout::array::<u32>(kept.capacity()).unwrap();
	let q = unsafe { alloc::realloc(kept.as_mut_ptr() as *mut u8, old, old.size() + 4) };
	let grown = unsafe { Vec::from_raw_parts(q.cast::<u32>(), 2, 3) };
	out.extend([grown.len() as u8, grown.capacity() as u8]);
	out.extend(grown.iter().flat_map(|x| x.to_le_bytes()));

	let mut boxed = unsafe { Vec::from_raw_parts(Box::into_raw(Box::new(n as u32 * 7)), n % 2, 1) };
	unsafe { boxed.set_len(1) };
	boxed.push(3);
	out.extend([boxed.len() as u8, boxed.capacity() as u8]);
	out.extend(boxed.iter().flat_map(|x| x.to_le_bytes()));
	let mut byte = unsafe { Vec::from_raw_parts(Box::into_raw(Box::new(n as u8)), 1, 1) };
	byte.push(4);
	out.extend_from_slice(&byte);
	let raw = unsafe { alloc::alloc(Layout::array::<u8>(extra + 1).unwrap()) };
	let mut fresh = unsafe { Vec::from_raw_parts(raw, 0, extra + 1) };
	fresh.extend_from_slice(data);
	out.extend([fresh.len() as u8, fresh.capacity() as u8]);
	out.extend_from_slice(&fresh);

	let mut none = unsafe { Vec::<u32>::from_raw_parts(NonNull::dangling().as_ptr(), 0, 0) };
	out.extend([none.len() as u8, none.capacity() as u8]);
	none.push(n as u32);
	out.extend([none.len() as u8, none.capacity() as u8, none[0] as u8]);
}
