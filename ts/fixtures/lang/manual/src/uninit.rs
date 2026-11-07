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

/// Boxes of one value not yet written: written through what `MaybeUninit::write` lends,
/// through its pointer, field by field, by `Box::write`, and through a raw pointer from
/// `Box::into_raw`, then read as what they hold; a box never written, or written and
/// never taken by `assume_init`, drops nothing.
pub fn boxed_one(data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let mut word = Box::<u16>::new_uninit();
	*word.write(u16::from(first)) += 0x101;
	let word = unsafe { word.assume_init() };
	out.extend_from_slice(&word.to_le_bytes());

	let mut byte = Box::<u8>::new_uninit();
	unsafe { byte.as_mut_ptr().write(first ^ 0x55) };
	out.push(*unsafe { byte.assume_init() });

	let text = Box::write(Box::new_uninit(), format!("{first:x}"));
	out.extend_from_slice(text.as_bytes());

	let mut pair = Box::<Pair>::new_uninit();
	let p = pair.as_mut_ptr();
	unsafe {
		(&raw mut (*p).tag).write(first);
		(&raw mut (*p).name).write(String::from("pair"));
	}
	let pair = unsafe { pair.assume_init() };
	out.push(pair.tag);
	out.extend_from_slice(pair.name.as_bytes());

	// `Box::write` writes into the box, where a pointer taken before it still points.
	let mut slot = Box::<Pair>::new_uninit();
	let p = slot.as_mut_ptr();
	let kept = Box::write(
		slot,
		Pair {
			tag: first,
			name: String::from("kept"),
		},
	);
	out.push(u8::from(std::ptr::eq(p, &*kept)));
	out.extend_from_slice(kept.name.as_bytes());
	let mut slots = Box::<[u16; 3]>::new_uninit();
	let p = slots.as_mut_ptr();
	let kept = Box::write(slots, [1, 2, u16::from(first)]);
	out.push(u8::from(std::ptr::eq(p, &*kept)));
	out.extend_from_slice(&kept[2].to_le_bytes());

	let mut words = Box::<[u16; 3]>::new_uninit();
	words.write([u16::from(first), 2, 3])[1] ^= 0xff;
	for w in *unsafe { words.assume_init() } {
		out.extend_from_slice(&w.to_le_bytes());
	}

	let raw = Box::into_raw(Box::<u32>::new_uninit());
	unsafe { raw.cast::<u32>().write(u32::from(first) << 20) };
	let back = unsafe { Box::from_raw(raw).assume_init() };
	out.extend_from_slice(&back.to_le_bytes());

	let counts = std::cell::Cell::new((0, 0));
	let mut counted = Box::<Counted>::new_uninit();
	counted.write(Counted(first, &counts));
	let counted = unsafe { counted.assume_init() };
	out.push(counted.0);
	drop(counted);
	out.push(counts.get().1);
	drop(Box::<Counted>::new_uninit());
	let mut leaked = Box::<Counted>::new_uninit();
	leaked.write(Counted(first, &counts));
	drop(leaked);
	out.push(counts.get().1);

	// Made by a generic function, of bytes and of values that count their drops.
	out.push(*boxed(first.wrapping_add(1)));
	out.push(boxed(Counted(first, &counts)).0);
	out.push(counts.get().1);
}

/// A struct written field by field through a pointer to it.
struct Pair {
	tag: u8,
	name: String,
}

/// `value` boxed by way of a box not yet written.
fn boxed<T>(value: T) -> Box<T> {
	let mut slot = Box::new_uninit();
	slot.write(value);
	unsafe { slot.assume_init() }
}

/// One of `Tag`'s variants is zero bytes, though not its first.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tag {
	One = 1,
	Zero = 0,
}

/// An enum whose variant of zero bytes may hold a reference, which zero bytes are no
/// value of.
enum Held<'a> {
	Lent(&'a u8, u16),
	Count(u32),
}

/// Every part zero bytes are a value of, a value that counts its drops through one it may
/// not hold, and an array whose length is past what an array literal would spell.
struct Zeroed<'a> {
	n: u32,
	on: bool,
	c: char,
	f: f64,
	wide: u64,
	signed: i128,
	words: [u16; 3],
	many: [u32; 300],
	pair: (u8, i64),
	tag: Tag,
	maybe: Option<Tag>,
	p: *const u8,
	seen: Option<&'a std::cell::Cell<(u8, u8)>>,
}

impl Drop for Zeroed<'_> {
	fn drop(&mut self) {
		if let Some(seen) = self.seen {
			let (c, d) = seen.get();
			seen.set((c, d.wrapping_add(1)));
		}
	}
}

/// Boxes of zero bytes: every part zero bytes are a value of reads as that value, an
/// enum as the variant zero bytes are, and a part they are no value of, such as a
/// reference or a `String`, is written before it is read; a box never taken by
/// `assume_init` drops nothing.
pub fn boxed_zeroed(data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	out.extend_from_slice(&unsafe { Box::<u32>::new_zeroed().assume_init() }.to_le_bytes());
	out.extend_from_slice(&unsafe { Box::<i64>::new_zeroed().assume_init() }.to_le_bytes());
	out.extend_from_slice(&unsafe { Box::<u128>::new_zeroed().assume_init() }.to_le_bytes());
	out.extend_from_slice(&unsafe { Box::<f32>::new_zeroed().assume_init() }.to_le_bytes());
	out.push(u8::from(*unsafe { Box::<bool>::new_zeroed().assume_init() }));
	out.push(*unsafe { Box::<char>::new_zeroed().assume_init() } as u8);
	out.push(u8::from(
		unsafe { Box::<*const u8>::new_zeroed().assume_init() }.is_null(),
	));

	// Enums, as the variant zero bytes are: a tag of zero, or a niche that holds one.
	out.extend([
		u8::from(unsafe { Box::<Option<u8>>::new_zeroed().assume_init() }.is_none()),
		u8::from(unsafe { *Box::<Option<bool>>::new_zeroed().assume_init() } == Some(false)),
		u8::from(unsafe { Box::<Option<std::num::NonZeroU16>>::new_zeroed().assume_init() }.is_none()),
		u8::from(unsafe { Box::<Option<&u8>>::new_zeroed().assume_init() }.is_none()),
		u8::from(unsafe { Box::<Option<Box<u8>>>::new_zeroed().assume_init() }.is_none()),
		u8::from(*unsafe { Box::<std::cmp::Ordering>::new_zeroed().assume_init() } == std::cmp::Ordering::Equal),
		u8::from(*unsafe { Box::<Tag>::new_zeroed().assume_init() } != Tag::One),
		u8::from(*unsafe { Box::<Option<Tag>>::new_zeroed().assume_init() } == Some(Tag::Zero)),
	]);

	// Std's types that zero bytes are a value of.
	let cell = unsafe { Box::<std::cell::Cell<u16>>::new_zeroed().assume_init() };
	cell.set(cell.get() + u16::from(first));
	out.extend_from_slice(&cell.get().to_le_bytes());
	let atomic = unsafe { Box::<std::sync::atomic::AtomicU32>::new_zeroed().assume_init() };
	atomic.fetch_add(u32::from(first) << 8, std::sync::atomic::Ordering::Relaxed);
	out.extend_from_slice(&atomic.load(std::sync::atomic::Ordering::Relaxed).to_le_bytes());
	let wrapped = unsafe { *Box::<std::num::Wrapping<u8>>::new_zeroed().assume_init() } - std::num::Wrapping(first);
	out.push(wrapped.0);
	let elapsed = unsafe { *Box::<std::time::Duration>::new_zeroed().assume_init() };
	out.extend_from_slice(
		&(elapsed + std::time::Duration::from_millis(u64::from(first)))
			.as_micros()
			.to_le_bytes(),
	);

	let shared = unsafe { Box::<std::cell::RefCell<u16>>::new_zeroed().assume_init() };
	*shared.borrow_mut() += u16::from(first);
	out.extend_from_slice(&shared.borrow().to_le_bytes());
	let locked = unsafe { Box::<std::sync::Mutex<u8>>::new_zeroed().assume_init() };
	*locked.lock().unwrap() ^= first;
	out.push(*locked.lock().unwrap());

	// Std's wrappers of a reference, which zero bytes are no value of, written in part.
	let mut wrapped = Box::<std::num::Wrapping<(u32, &u8)>>::new_zeroed();
	unsafe { (&raw mut (*wrapped.as_mut_ptr()).0.1).write(&first) };
	let std::num::Wrapping((n, lent)) = *unsafe { wrapped.assume_init() };
	out.extend_from_slice(&n.to_le_bytes());
	out.push(*lent);
	let mut cell = Box::<std::cell::UnsafeCell<(u32, &u8)>>::new_zeroed();
	unsafe { (&raw mut (*(*cell.as_mut_ptr()).get()).1).write(&first) };
	let (n, lent) = unsafe { cell.assume_init() }.into_inner();
	out.extend_from_slice(&n.to_le_bytes());
	out.push(*lent);

	// One whose variant of zero bytes holds a reference, which a whole write replaces.
	for value in [Held::Lent(&first, 7), Held::Count(u32::from(first) + 1)] {
		let mut held = Box::<Held>::new_zeroed();
		held.write(value);
		match *unsafe { held.assume_init() } {
			Held::Lent(b, n) => out.extend([*b, n as u8]),
			Held::Count(n) => out.push(n as u8),
		}
	}

	// Arrays and tuples, written in part before they are read.
	let mut bytes = Box::<[u8; 16]>::new_zeroed();
	unsafe { bytes.as_mut_ptr().cast::<u8>().add(3).write(first) };
	out.extend_from_slice(&*unsafe { bytes.assume_init() });
	let mut grid = Box::<[[u16; 2]; 3]>::new_zeroed();
	unsafe { (*grid.as_mut_ptr())[1][0] = u16::from(first) + 1 };
	for row in *unsafe { grid.assume_init() } {
		out.extend(row.iter().flat_map(|w| w.to_le_bytes()));
	}
	let (a, b, c) = *unsafe { Box::<(u8, i32, bool)>::new_zeroed().assume_init() };
	out.extend([a, b as u8, u8::from(c)]);

	// A reference, which zero bytes are no value of, written before it is read.
	let mut lent = Box::<(u32, &u8)>::new_zeroed();
	unsafe { (&raw mut (*lent.as_mut_ptr()).1).write(&first) };
	let lent = unsafe { lent.assume_init() };
	out.extend_from_slice(&lent.0.to_le_bytes());
	out.push(*lent.1);
	let mut named = Box::<(String, u16)>::new_zeroed();
	unsafe { (&raw mut (*named.as_mut_ptr()).0).write(format!("{first}")) };
	let named = unsafe { named.assume_init() };
	out.extend_from_slice(named.0.as_bytes());
	out.extend_from_slice(&named.1.to_le_bytes());

	// The crate's struct, whole, then with the value its drop counts through written in.
	let counts = std::cell::Cell::new((0, 0));
	let zeroed = unsafe { Box::<Zeroed>::new_zeroed().assume_init() };
	out.extend_from_slice(&zeroed.n.to_le_bytes());
	out.extend([
		u8::from(zeroed.on),
		zeroed.c as u8,
		zeroed.f as u8,
		zeroed.wide as u8,
		zeroed.signed as u8,
	]);
	out.extend(zeroed.words.iter().flat_map(|w| w.to_le_bytes()));
	out.push(zeroed.many.iter().fold(0, |n, &w| n | w) as u8);
	out.extend([zeroed.pair.0, zeroed.pair.1 as u8, zeroed.tag as u8]);
	out.extend([u8::from(zeroed.maybe == Some(Tag::Zero)), u8::from(zeroed.p.is_null())]);
	drop(zeroed);
	let mut seen = Box::<Zeroed>::new_zeroed();
	unsafe {
		let p = seen.as_mut_ptr();
		(&raw mut (*p).seen).write(Some(&counts));
		(&raw mut (*p).many[299]).write(u32::from(first));
	}
	let seen = unsafe { seen.assume_init() };
	out.push(seen.many[299] as u8);
	drop(seen);
	out.push(counts.get().1);
	let mut kept = Box::<Zeroed>::new_zeroed();
	unsafe { (&raw mut (*kept.as_mut_ptr()).seen).write(Some(&counts)) };
	drop(kept);
	out.push(counts.get().1);

	// Made by a generic function.
	out.extend_from_slice(&zeroed_box::<u64>().to_le_bytes());
	let (x, y) = *zeroed_box::<(u8, u16)>();
	out.extend([x, y as u8]);
}

/// A box of zero bytes, taken as a `T`.
fn zeroed_box<T: Copy>() -> Box<T> {
	unsafe { Box::new_zeroed().assume_init() }
}
