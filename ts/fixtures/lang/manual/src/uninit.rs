//! `MaybeUninit<u8>`, which JS keeps as the `u8`, so its arrays, slices, and `Vec`s are
//! those of bytes: written through as slices, by element, by `write_copy_of_slice`, and
//! through what it gives back, then read as what they hold, and lent or held in an
//! `Option` unwritten; beside a slice of wider values copied in, and copies of mismatched
//! lengths, which panic.

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

	// Copies of mismatched lengths panic as `copy_from_slice` does.
	match data.first() {
		Some(0xfe) => _ = array[..1].write_copy_of_slice(&data[..0]),
		Some(0xff) => _ = words[..1].write_copy_of_slice(&wide[..0]),
		_ => {}
	}
}
