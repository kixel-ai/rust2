//! `MaybeUninit<u8>`, which JS keeps as the `u8`, so its arrays, slices, and `Vec`s are
//! those of bytes: written through as slices, by element, by `write_copy_of_slice`, and
//! through what it gives back, then read as what they hold, lent or held in an `Option`
//! unwritten, and held in another `MaybeUninit`; beside a slice of wider values copied in, and copies of mismatched
//! lengths, which panic. Boxed slices not yet written, of bytes and of other values,
//! written by `write_clone_of_slice` too, in a generic function, and through a pointer.

use std::mem::MaybeUninit;

/// An array of bytes not yet written, a `Vec` of them, and an array of `u16`s, each
/// written from the input and read back.
pub fn uninits(data: &[u8], out: &mut Vec<u8>) {
	let mut array = [MaybeUninit::<u8>::uninit(); 8];
	let n = data.len().min(array.len());
	array[..n].write_copy_of_slice(&data[..n]).sort();
	if let Some(slot) = array[..n].last_mut() {
		slot.write(0xee);
	}
	out.extend_from_slice(unsafe { array[..n].assume_init_ref() });
	unsafe { array[..n].assume_init_mut() }.reverse();
	out.extend(array[..n].iter().map(|b| unsafe { b.assume_init() }));

	let mut vec: Vec<MaybeUninit<u8>> = data.iter().map(|&b| MaybeUninit::new(b)).collect();
	vec.push(MaybeUninit::new(7));
	for (i, slot) in vec.iter_mut().enumerate() {
		slot.write(unsafe { slot.assume_init() } ^ i as u8);
	}
	vec.truncate(vec.len() / 2 + 1);
	out.extend(vec.iter().map(|b| unsafe { b.assume_init() }));

	let wide: Vec<u16> = data
		.chunks(2)
		.map(|c| c.iter().fold(0, |w, &b| w << 8 | u16::from(b)))
		.collect();
	let mut words = [MaybeUninit::<u16>::uninit(); 4];
	let k = wide.len().min(words.len());
	if let Some(w) = words[..k].write_copy_of_slice(&wide[..k]).first_mut() {
		*w ^= 0x0101;
	}
	for w in unsafe { words[..k].assume_init_ref() } {
		out.extend_from_slice(&w.to_le_bytes());
	}

	// Lent or held in an `Option`, which holds one whether or not it was written.
	let fresh = [MaybeUninit::<u8>::uninit(); 2];
	let lone: Option<MaybeUninit<u8>> = data.first().map(|_| MaybeUninit::uninit());
	out.extend([fresh.first().is_some(), fresh.get(2).is_none(), lone.is_some()].map(u8::from));

	// One held in another, a slice of which lends the inner ones.
	let mut nested = [MaybeUninit::<MaybeUninit<u8>>::uninit(); 3];
	let k = data.len().min(nested.len());
	let src: Vec<MaybeUninit<u8>> = data[..k].iter().map(|&b| MaybeUninit::new(!b)).collect();
	nested[..k].write_copy_of_slice(&src);
	out.extend(
		unsafe { nested[..k].assume_init_ref() }
			.iter()
			.map(|b| unsafe { b.assume_init() }),
	);

	// Copies of mismatched lengths panic as `copy_from_slice` does.
	match data.first() {
		Some(0xfe) => _ = array[..1].write_copy_of_slice(&data[..0]),
		Some(0xff) => _ = words[..1].write_copy_of_slice(&wide[..0]),
		_ => {}
	}
}

/// Counts its clones and drops, so a write that clones in shows it drops nothing.
struct Counted<'a>(u8, &'a std::cell::Cell<(u8, u8)>);

impl Clone for Counted<'_> {
	fn clone(&self) -> Self {
		let (c, d) = self.1.get();
		self.1.set((c.wrapping_add(1), d));
		Counted(self.0.wrapping_add(1), self.1)
	}
}

impl Drop for Counted<'_> {
	fn drop(&mut self) {
		let (c, d) = self.1.get();
		self.1.set((c, d.wrapping_add(1)));
	}
}

/// Boxed slices not yet written, of bytes, `u16`s, `String`s, and values that count
/// their clones and drops: written by element, by `write_copy_of_slice`, and by
/// `write_clone_of_slice`, which clones each in without dropping what was there, then
/// read as what they hold.
pub fn boxed_uninits(data: &[u8], out: &mut Vec<u8>) {
	let mut bytes = Box::<[u8]>::new_uninit_slice(data.len());
	out.push(bytes.len() as u8);
	bytes.write_copy_of_slice(data).reverse();
	let bytes = unsafe { bytes.assume_init() };
	out.extend_from_slice(&bytes);

	let mut cloned = Box::<[u8]>::new_uninit_slice(data.len() / 2);
	cloned.write_clone_of_slice(&data[..data.len() / 2]).sort();
	out.extend_from_slice(unsafe { cloned.assume_init_ref() });

	let mut words = Box::<[u16]>::new_uninit_slice(data.len() / 2);
	for (slot, c) in words.iter_mut().zip(data.chunks_exact(2)) {
		slot.write(u16::from_le_bytes([c[0], c[1]]));
	}
	for w in unsafe { words.assume_init() } {
		out.extend_from_slice(&w.to_le_bytes());
	}

	let src: Vec<String> = data.iter().map(|b| format!("{b:x}")).collect();
	let mut strings = Box::<[String]>::new_uninit_slice(src.len());
	strings.write_clone_of_slice(&src).sort();
	strings.write_clone_of_slice(&src)[0..].reverse();
	let strings = unsafe { strings.assume_init() };
	out.extend(strings.iter().flat_map(|s| s.bytes()));

	let counts = std::cell::Cell::new((0, 0));
	let src: Vec<Counted> = data.iter().map(|&b| Counted(b, &counts)).collect();
	let mut counted = Box::<[Counted]>::new_uninit_slice(src.len());
	counted.write_clone_of_slice(&src);
	counted.write_clone_of_slice(&src);
	out.push(counts.get().0);
	out.push(counts.get().1);
	let counted = unsafe { counted.assume_init() };
	out.extend(counted.iter().map(|c| c.0));
	out.push(counts.get().1);

	// Made by a generic function, of bytes and of values that count their clones.
	out.extend_from_slice(&boxed_clone(data));
	out.extend(boxed_clone(&counted).iter().map(|c| c.0));
	drop(counted);
	out.extend([counts.get().0, counts.get().1]);

	// Held through a pointer to its bytes, written through it, and boxed again, as a
	// buffer's storage is.
	let len = data.len();
	let p = Box::into_raw(Box::<[u8]>::new_uninit_slice(len)).cast::<u8>();
	unsafe { std::ptr::copy_nonoverlapping(data.as_ptr(), p, len) };
	if len > 1 {
		unsafe { p.add(1).write(0xaa) };
	}
	let raw = std::ptr::slice_from_raw_parts_mut(p.cast::<MaybeUninit<u8>>(), len);
	let boxed = unsafe { Box::from_raw(raw).assume_init() };
	out.extend_from_slice(&boxed);

	// Writes of mismatched lengths panic as `assert_eq!` does.
	match data.first() {
		Some(0xfc) => _ = Box::<[u8]>::new_uninit_slice(1).write_clone_of_slice(&data[..0]),
		Some(0xfd) => _ = Box::<[Counted]>::new_uninit_slice(0).write_clone_of_slice(&src[..1]),
		_ => {}
	}
}

/// A boxed clone of `src`, built in slots not yet written.
fn boxed_clone<T: Clone>(src: &[T]) -> Box<[T]> {
	let mut slots = Box::new_uninit_slice(src.len());
	slots.write_clone_of_slice(src);
	unsafe { slots.assume_init() }
}
