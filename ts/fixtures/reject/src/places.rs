//! Writes through references whose referent JS keeps as the place, or as another object.

pub struct Noisy(pub u8);

impl Drop for Noisy {
	fn drop(&mut self) {}
}

/// A guard is its mutex in JS: replacing it would copy the other mutex's value in.
pub fn replace_guard<'a>(dst: &mut std::sync::MutexGuard<'a, u8>, src: std::sync::MutexGuard<'a, u8>) {
	*dst = src;
}

/// A boxed byte array is a `Uint8Array`, which a replacement can't copy out as an object.
pub fn replace_boxed_bytes(
	dst: &mut std::pin::Pin<Box<[u8; 2]>>,
	src: std::pin::Pin<Box<[u8; 2]>>,
) -> std::pin::Pin<Box<[u8; 2]>> {
	std::mem::replace(dst, src)
}

/// A boxed future is whatever class its value's type is.
pub fn replace_boxed_future(
	dst: &mut std::pin::Pin<Box<dyn std::future::Future<Output = u8>>>,
	src: std::pin::Pin<Box<dyn std::future::Future<Output = u8>>>,
) {
	*dst = src;
}

/// Swapping two guards would swap their mutexes' values instead.
pub fn swap_guards<'a>(a: &mut std::sync::MutexGuard<'a, u8>, b: &mut std::sync::MutexGuard<'a, u8>) {
	std::mem::swap(a, b);
}

/// A type of no bytes.
pub struct Bare;

/// A pinned reference is its referent in JS: replacing it would change what it points at.
pub fn replace_pinned_ref<'a>(dst: &mut std::pin::Pin<&'a mut Noisy>, src: std::pin::Pin<&'a mut Noisy>) {
	*dst = src;
}

/// Each copy of a closure has its own captures, which copies of an arrow function share.
pub fn copied_closure(n: u8) -> u8 {
	let mut a = [n];
	let mut g = move || {
		a[0] = a[0].wrapping_add(1);
		a[0]
	};
	let mut h = g;
	g().wrapping_add(h())
}

fn bumped(x: &mut u8) {
	*x += 1;
}

/// A `Result`'s value passed on: a store through it makes the `Result` again, which no
/// `rt.Place` of a path does.
pub fn ok_passed(r: &mut Result<u8, u8>) {
	if let Ok(v) = r {
		bumped(v);
	}
}

/// `Option`s holding `rt.Place`s, which std's runtime would compare as the Places.
pub fn places_equal(a: &mut u32, b: &mut u32) -> bool {
	Some(a) == Some(b)
}

/// A copy of a closure that rebinds a shared reference it captured: each Rust copy has
/// its own, where JS would share one binding.
pub fn rebound_reference(a: &u32, b: &u32) -> (u32, u32) {
	let mut r = a;
	let c = move || {
		let old = *r;
		r = b;
		old
	};
	let mut x = c;
	let mut y = c;
	(x(), y())
}

/// Two `rt.Place`s of one number, which are two objects.
pub fn places_compared(n: &mut u32) -> bool {
	let a = &raw mut *n;
	let b = &raw mut *n;
	a == b
}

/// An `Option` of pointers to a number, each an `rt.Place` JS makes of its own.
pub fn optional_places_compared(n: &mut u32) -> bool {
	let p = std::ptr::NonNull::from(&mut *n);
	Some(p) == std::ptr::NonNull::new(&raw mut *n)
}

/// The places of two `Rc`s' numbers, which `as_ptr` makes each of its own.
pub fn rc_places_compared(a: &std::rc::Rc<u32>, b: &std::rc::Rc<u32>) -> bool {
	std::ptr::addr_eq(std::rc::Rc::as_ptr(a), std::rc::Rc::as_ptr(b))
}

/// Pointers to two types, which JS keeps apart whatever their addresses.
pub fn addresses_across(a: &(u8, u8), b: &[u8; 2]) -> bool {
	std::ptr::addr_eq(a, b)
}

/// A pointer to the trait object an `Arc` holds.
pub fn arc_dyn_pointer(a: &std::sync::Arc<dyn std::fmt::Debug>) -> *const dyn std::fmt::Debug {
	std::sync::Arc::as_ptr(a)
}

/// A `RefMut` mapped to a closure, which a write through the guard would put in place of
/// the cell's, though JS can't write one closure's captures into another.
pub fn mapped_closure(add: u8) -> u8 {
	let cell = std::cell::RefCell::new((move |x: u8| x + add, 0u8));
	let f = std::cell::RefMut::map(cell.borrow_mut(), |pair| &mut pair.0);
	f(1)
}
