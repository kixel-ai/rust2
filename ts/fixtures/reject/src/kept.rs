//! Pointers kept in memory, structs, and slots, read after a whole write replaced what they point at.

use crate::parts::cell;

/// Structs JS keeps as objects, each holding an `rs.Cell` of an object, one for each write
/// of fresh memory made while a pointer into it may still be read, which gives the cell
/// another object.
pub struct Escaped(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Called(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Looped(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Aliased(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Rewritten(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Keeper {
	pub whole: *mut Escaped,
	pub part: *mut (u16, u16),
}

fn keep_part(keeper: &mut Keeper) {
	keeper.part = unsafe { (*keeper.whole).0.get() };
}

/// A fresh allocation written whole again after its pointer escaped into a struct.
pub fn escaped_into_struct(n: u16, keeper: &mut Keeper) -> u16 {
	let p = Box::into_raw(Box::new(std::mem::MaybeUninit::<Escaped>::uninit())).cast::<Escaped>();
	unsafe { p.write(Escaped(std::cell::UnsafeCell::new((n, n)))) };
	keeper.whole = p;
	keep_part(keeper);
	unsafe { p.write(Escaped(std::cell::UnsafeCell::new((0, 0)))) };
	unsafe { (*keeper.part).0 }
}

fn kept_part(p: *mut Called, kept: &mut Vec<*mut (u16, u16)>) {
	kept.push(unsafe { (*p).0.get() });
}

/// A fresh allocation written whole again after its pointer escaped into a call.
pub fn escaped_into_call(n: u16) -> u16 {
	let p = Box::into_raw(Box::new(std::mem::MaybeUninit::<Called>::uninit())).cast::<Called>();
	unsafe { p.write(Called(std::cell::UnsafeCell::new((n, n)))) };
	let mut kept = Vec::new();
	kept_part(p, &mut kept);
	unsafe { p.write(Called(std::cell::UnsafeCell::new((0, 0)))) };
	unsafe { (*kept[0]).0 }
}

/// A fresh allocation written whole in each iteration, read after through the pointer an
/// earlier iteration took into it.
pub fn kept_across_writes(data: &[u16]) -> u16 {
	let p = Box::into_raw(Box::new(std::mem::MaybeUninit::<Looped>::uninit())).cast::<Looped>();
	let mut kept: *mut (u16, u16) = std::ptr::null_mut();
	let mut sum = 0;
	for &n in data {
		unsafe { p.write(Looped(std::cell::UnsafeCell::new((n, n)))) };
		if !kept.is_null() {
			sum += unsafe { (*kept).0 };
		}
		kept = unsafe { (*p).0.get() };
	}
	sum
}

/// A fresh allocation written whole through an alias of its pointer while a pointer into it
/// may still be read.
pub fn written_through_alias(n: u16) -> u16 {
	let p = Box::into_raw(Box::new(std::mem::MaybeUninit::<Aliased>::uninit())).cast::<Aliased>();
	unsafe { p.write(Aliased(std::cell::UnsafeCell::new((n, n)))) };
	let part = unsafe { (*p).0.get() };
	let alias = p;
	unsafe { alias.write(Aliased(std::cell::UnsafeCell::new((0, 0)))) };
	unsafe { (*part).0 }
}

/// A `MaybeUninit` written whole again while a pointer into it may still be read.
pub fn uninit_rewritten(n: u16) -> u16 {
	let mut slot = std::mem::MaybeUninit::<Rewritten>::uninit();
	slot.write(Rewritten(std::cell::UnsafeCell::new((n, n))));
	let part = unsafe { slot.assume_init_ref().0.get() };
	slot.write(Rewritten(std::cell::UnsafeCell::new((0, 0))));
	unsafe { (*part).0 }
}

pub struct Loaded(pub std::cell::UnsafeCell<(u16, u16)>);

/// A fresh allocation written whole again through a pointer to it read out of another
/// allocation, while a pointer into it may still be read.
pub fn written_through_loaded(n: u16) -> u16 {
	let p = Box::into_raw(Box::new(std::mem::MaybeUninit::<Loaded>::uninit())).cast::<Loaded>();
	unsafe { p.write(Loaded(std::cell::UnsafeCell::new((n, n)))) };
	let part = unsafe { (*p).0.get() };
	let container = Box::into_raw(Box::new(p.cast_const()));
	unsafe {
		container
			.read()
			.cast_mut()
			.write(Loaded(std::cell::UnsafeCell::new((0, 0))))
	};
	unsafe { (*part).0 }
}

/// Structs JS keeps as objects, each written whole into slots through std's iterators
/// while a pointer into one may still be read, which gives what holds it another object.
pub struct Mapped(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Repeated(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Onced(pub std::cell::UnsafeCell<(u16, u16)>);

pub struct Pushed {
	pub part: (u16, u16),
}

pub struct Copied {
	pub span: (u16, u16),
}

std::thread_local! {
	static COPIED: std::cell::Cell<*const (u16, u16)> = const { std::cell::Cell::new(std::ptr::null()) };
}

/// A clone that keeps a pointer into what it copies.
impl Clone for Copied {
	fn clone(&self) -> Self {
		COPIED.with(|kept| kept.set(&raw const self.span));
		Copied { span: self.span }
	}
}

/// Slots written through an iterator whose closure keeps a pointer into each before it is
/// written.
pub fn mapped_kept(n: u16) -> u16 {
	let mut slots = [const { std::mem::MaybeUninit::<Mapped>::uninit() }; 2];
	let mut kept: *mut (u16, u16) = std::ptr::null_mut();
	for slot in slots.iter_mut().map(|s| {
		kept = unsafe { (*s.as_mut_ptr()).0.get() };
		s
	}) {
		slot.write(Mapped(std::cell::UnsafeCell::new((n, n))));
	}
	unsafe { (*kept).0 }
}

/// A slot written whole again after `cloned` handed it to the crate's `Clone`, which kept
/// a pointer into it.
pub fn cloned_kept(n: u16) -> u16 {
	let mut slots = [const { std::mem::MaybeUninit::<Copied>::uninit() }; 1];
	for slot in &mut slots {
		slot.write(Copied { span: (n, n) });
	}
	let copies: Vec<Copied> = std::iter::once(unsafe { slots[0].assume_init_ref() })
		.cloned()
		.collect();
	slots[0].write(Copied { span: (0, 0) });
	COPIED.with(|kept| unsafe { (*kept.get()).0 }) + copies.len() as u16
}

/// Slots written whole on each pass, read after through the pointer a write on the pass
/// before took into the same slot.
pub fn kept_across_iterations(data: &[u16]) -> u16 {
	let mut slots = [const { std::mem::MaybeUninit::<Repeated>::uninit() }; 1];
	let mut kept: *mut (u16, u16) = std::ptr::null_mut();
	let mut sum = 0;
	for &n in data {
		for slot in slots.iter_mut() {
			let made = slot.write(Repeated(std::cell::UnsafeCell::new((n, n))));
			if !kept.is_null() {
				sum += unsafe { (*kept).0 };
			}
			kept = made.0.get();
		}
	}
	sum
}

/// A slot written whole through a borrow that std's `once` handed the crate a mutable
/// borrow of, which pointed it at memory a pointer into may still be read.
pub fn lent_kept(n: u16) -> u16 {
	let mut slots = [const { std::mem::MaybeUninit::<Onced>::uninit() }; 1];
	let mut old = std::mem::MaybeUninit::<Onced>::uninit();
	old.write(Onced(std::cell::UnsafeCell::new((n, n))));
	let part = unsafe { old.assume_init_ref().0.get() };
	let mut slot: &mut std::mem::MaybeUninit<Onced> = &mut slots[0];
	if let Some(lent) = std::iter::once(&mut slot).next() {
		*lent = &mut old;
	}
	slot.write(Onced(std::cell::UnsafeCell::new((0, 0))));
	unsafe { (*part).0 }
}

/// An element of memory `Vec::push` allocated, written whole while a pointer into it may
/// still be read.
pub fn pushed_part_kept(n: u16) -> u16 {
	let mut v = Vec::new();
	v.push(Pushed { part: (n, n) });
	let p = &raw mut v[0];
	let part = unsafe { &raw mut (*p).part };
	unsafe { p.write(Pushed { part: (0, 0) }) };
	unsafe { (*part).0 }
}

/// Structs JS keeps as objects, each holding an `rs.Cell`, one for each path by which a
/// pointer kept in memory reaches a whole write of what it points at, which gives it
/// another cell, while a pointer to the cell, taken through a reference to the struct,
/// may still be read.
pub struct ByRead(pub std::cell::UnsafeCell<u16>);

pub struct ByReplace(pub std::cell::UnsafeCell<u16>);

pub struct ByBox(pub std::cell::UnsafeCell<u16>);

pub struct ByClosure(pub std::cell::UnsafeCell<u16>);

pub struct ByObject(pub std::cell::UnsafeCell<u16>);

pub struct ByInstance(pub std::cell::UnsafeCell<u16>);

pub struct BySwapped(pub std::cell::UnsafeCell<u16>);

pub struct ByExchanged(pub std::cell::UnsafeCell<u16>);

/// A pointer kept in memory, which only the crate's own code reads back.
pub struct Kept<T> {
	pub at: *mut T,
}

impl<T> Kept<T> {
	fn put(&self, value: T) {
		unsafe { *self.at = value }
	}
}

pub trait Put {
	fn put(&self);
}

impl Put for Kept<ByObject> {
	fn put(&self) {
		unsafe { *self.at = ByObject(cell(0)) }
	}
}

/// Read back from memory, then written whole through.
pub fn kept_read(n: u16) -> u16 {
	let mut whole = ByRead(cell(n));
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = &raw mut whole;
	let part = unsafe { (*kept.at).0.get() };
	let at = kept.at;
	unsafe { *at = ByRead(cell(0)) };
	unsafe { *part }
}

/// Read back from memory, then replaced whole by `mem::replace` through a `&mut`.
pub fn kept_replaced(n: u16) -> u16 {
	let mut whole = ByReplace(cell(n));
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = &raw mut whole;
	let part = unsafe { (*kept.at).0.get() };
	drop(std::mem::replace(unsafe { &mut *kept.at }, ByReplace(cell(0))));
	unsafe { *part }
}

/// Read back from memory into `Box::from_raw`, then written whole through the box.
pub fn kept_boxed(n: u16) -> u16 {
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = Box::into_raw(Box::new(ByBox(cell(n))));
	let part = unsafe { (*kept.at).0.get() };
	let mut boxed = unsafe { Box::from_raw(kept.at) };
	*boxed = ByBox(cell(0));
	let read = unsafe { *part };
	drop(boxed);
	read
}

/// Read back from memory by a closure, which writes whole through it.
pub fn kept_called(n: u16) -> u16 {
	let mut whole = ByClosure(cell(n));
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = &raw mut whole;
	let part = unsafe { (*kept.at).0.get() };
	let put: Box<dyn Fn()> = Box::new(|| unsafe { *kept.at = ByClosure(cell(0)) });
	put();
	unsafe { *part }
}

/// Read back from memory by a trait object's method, which writes whole through it.
pub fn kept_object(n: u16) -> u16 {
	let mut whole = ByObject(cell(n));
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = &raw mut whole;
	let part = unsafe { (*kept.at).0.get() };
	let put: &dyn Put = &kept;
	put.put();
	unsafe { *part }
}

/// Read back from memory by a generic method, which writes whole through it.
pub fn kept_instance(n: u16) -> u16 {
	let mut whole = ByInstance(cell(n));
	let mut kept = Kept {
		at: std::ptr::null_mut(),
	};
	kept.at = &raw mut whole;
	let part = unsafe { (*kept.at).0.get() };
	kept.put(ByInstance(cell(0)));
	unsafe { *part }
}

/// Given to an atomic, handed back by its `swap`, then written whole through.
pub fn atomic_swapped(n: u16) -> u16 {
	let mut whole = BySwapped(cell(n));
	let atomic = std::sync::atomic::AtomicPtr::new(&raw mut whole);
	let part = unsafe { (*atomic.load(std::sync::atomic::Ordering::SeqCst)).0.get() };
	let at = atomic.swap(std::ptr::null_mut(), std::sync::atomic::Ordering::SeqCst);
	unsafe { *at = BySwapped(cell(0)) };
	unsafe { *part }
}

/// Given to an atomic, handed back by its `compare_exchange`, then written whole through.
pub fn atomic_exchanged(n: u16) -> u16 {
	let mut whole = ByExchanged(cell(n));
	let at = &raw mut whole;
	let atomic = std::sync::atomic::AtomicPtr::new(at);
	let part = unsafe { (*at).0.get() };
	let order = std::sync::atomic::Ordering::SeqCst;
	if let Ok(at) = atomic.compare_exchange(at, std::ptr::null_mut(), order, order) {
		unsafe { *at = ByExchanged(cell(0)) };
	}
	unsafe { *part }
}

/// Structs JS keeps as objects, each holding an `rs.Cell`, one for each std function that
/// puts another object in place of a `RefCell`'s value through a shared reference, while a
/// pointer to the cell, taken through a borrow of that value, may still be read.
pub struct ByRefReplace(pub std::cell::UnsafeCell<u16>);

#[derive(Default)]
pub struct ByRefTake(pub std::cell::UnsafeCell<u16>);

pub struct ByKeySet(pub std::cell::UnsafeCell<u16>);

pub struct ByKeyReplace(pub std::cell::UnsafeCell<u16>);

#[derive(Default)]
pub struct ByKeyTake(pub std::cell::UnsafeCell<u16>);

std::thread_local! {
	static KEY_SET: std::cell::RefCell<(u16, ByKeySet)> = const { std::cell::RefCell::new((0, ByKeySet(std::cell::UnsafeCell::new(0)))) };
	static KEY_REPLACE: std::cell::RefCell<(u16, ByKeyReplace)> = const { std::cell::RefCell::new((0, ByKeyReplace(std::cell::UnsafeCell::new(0)))) };
	static KEY_TAKE: std::cell::RefCell<(u16, ByKeyTake)> = const { std::cell::RefCell::new((0, ByKeyTake(std::cell::UnsafeCell::new(0)))) };
}

/// Replaced by `RefCell::replace`.
pub fn ref_replaced(n: u16) -> u16 {
	let whole = std::cell::RefCell::new((n, ByRefReplace(cell(n))));
	let part = whole.borrow().1.0.get();
	drop(whole.replace((0, ByRefReplace(cell(0)))));
	unsafe { *part }
}

/// Replaced by `RefCell::take`.
pub fn ref_taken(n: u16) -> u16 {
	let whole = std::cell::RefCell::new((n, ByRefTake(cell(n))));
	let part = whole.borrow().1.0.get();
	drop(whole.take());
	unsafe { *part }
}

/// Replaced by a thread-local `RefCell`'s `set`.
pub fn key_set(n: u16) -> u16 {
	KEY_SET.set((n, ByKeySet(cell(n))));
	let part = KEY_SET.with_borrow(|whole| whole.1.0.get());
	KEY_SET.set((0, ByKeySet(cell(0))));
	unsafe { *part }
}

/// Replaced by a thread-local `RefCell`'s `replace`.
pub fn key_replaced(n: u16) -> u16 {
	drop(KEY_REPLACE.replace((n, ByKeyReplace(cell(n)))));
	let part = KEY_REPLACE.with_borrow(|whole| whole.1.0.get());
	drop(KEY_REPLACE.replace((0, ByKeyReplace(cell(0)))));
	unsafe { *part }
}

/// Replaced by a thread-local `RefCell`'s `take`.
pub fn key_taken(n: u16) -> u16 {
	let part = KEY_TAKE.with_borrow(|whole| whole.1.0.get());
	unsafe { *part = n };
	drop(KEY_TAKE.take());
	unsafe { *part }
}

pub struct Reaimed(pub (u16, u16));

/// A fresh allocation written whole through a pointer that a shared borrow of the
/// `UnsafeCell` holding it aimed at another value, into which a pointer may still be read.
pub fn reaimed(n: u16) -> u16 {
	let mut other = Reaimed((n, n));
	let part = &raw mut other.0;
	let at = std::cell::UnsafeCell::new(Box::into_raw(Box::new(Reaimed((0, 0)))));
	let shared = &at;
	unsafe { *shared.get() = &raw mut other };
	unsafe { *at.into_inner() = Reaimed((1, 1)) };
	unsafe { (*part).0 }
}

pub struct Reversed(pub (u16, u16));

/// A value std's `Reverse` holds, swapped whole for another while a pointer into it may
/// still be read.
pub fn reversed(n: u16) -> u16 {
	let mut slot = std::cmp::Reverse(Reversed((n, n)));
	let mut other = std::cmp::Reverse(Reversed((0, 0)));
	let part = &raw mut slot.0.0;
	std::mem::swap(&mut slot, &mut other);
	unsafe { (*part).0 }
}

pub struct Zipped(pub [u8; 2], pub (u8, u8));

/// `value`, borrowed for as long as the caller likes.
fn unbound<'a, T>(value: &T) -> &'a T {
	unsafe { &*(value as *const T) }
}

/// Slots written whole beside an iterator zipped with them over references to their own
/// rows, each read after the write replaced it.
pub fn zipped_parts(fill: [u8; 2]) -> u8 {
	let mut slots = [Zipped(fill, (0, 1)), Zipped(fill, (2, 3))];
	let rows = [unbound(&slots[0].0), unbound(&slots[1].0)];
	let mut sum = 0;
	for (slot, row) in slots.iter_mut().zip(rows) {
		*slot = Zipped([0, 0], (0, 1));
		let tags = &raw mut slot.1;
		unsafe { (*tags).0 ^= row[0] };
		sum ^= unsafe { (*tags).0 };
	}
	sum
}

/// Slots of rows written whole beside the rows std's `flatten` reads out of the slots,
/// zipped again with rows read out of another value the same way, whose iterator is of the
/// same type: the next row is the replaced second row of the slot just written.
pub fn rezipped_parts(fill: [[u8; 3]; 2], other: [[[u8; 3]; 2]; 2]) -> u8 {
	let mut slots = [fill; 2];
	let alias = unsafe { &*(&raw const slots) };
	let mut sum = 0;
	for ((slot, row), o) in slots.iter_mut().zip(alias.iter().flatten()).zip(other.iter().flatten()) {
		sum ^= row[0];
		*slot = [*o, [0, 1, 2]];
	}
	sum ^ unbound(&slots[1][0])[0]
}

pub struct Collected(pub [u8; 2], pub (u8, u8));

/// [`zipped_parts`] with the references collected into a vector first, whose type
/// `collect` relates to its iterator's only through a bound.
pub fn collected_parts(fill: [u8; 2]) -> u8 {
	let mut slots = [Collected(fill, (0, 1)), Collected(fill, (2, 3))];
	let rows = [unbound(&slots[0].0), unbound(&slots[1].0)];
	let rows: Vec<&[u8; 2]> = rows.into_iter().collect();
	let mut sum = 0;
	for (slot, row) in slots.iter_mut().zip(rows) {
		*slot = Collected([0, 0], (0, 1));
		let tags = &raw mut slot.1;
		unsafe { (*tags).0 ^= row[0] };
		sum ^= unsafe { (*tags).0 };
	}
	sum
}
