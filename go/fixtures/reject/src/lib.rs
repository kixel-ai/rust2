//! Constructs outside the subset: each `pub` item fails translation with a span, and no
//! output is written.

#![allow(dead_code, reason = "each `pub` item is a translation root, not called from Rust")]

pub fn generic<T>(t: T) -> T {
	t
}

/// Read and written only in `unsafe`, which Go has no counterpart for.
pub static mut COUNT: u8 = 0;

/// Go callers would share it unsynchronized.
pub static READY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

unsafe extern "C" {
	/// Another language's, which has no Go value.
	pub safe static ENVIRON: u8;
}

pub struct Wrap<T>(pub T);

impl<T> Wrap<T> {
	/// Its value differs by `T`, which only its uses instantiate.
	pub const SIZE: usize = size_of::<T>();
}

static HALVES: (u8, u8) = (1, 2);

/// A reference into part of a static, which its Go variable has no place for apart.
const SECOND: &u8 = &HALVES.1;

pub fn second() -> u8 {
	*SECOND
}

/// Go has no 128-bit integer.
pub fn wide(a: u64, b: u64) -> u128 {
	a as u128 * b as u128
}

struct Logged(u8);

impl Drop for Logged {
	fn drop(&mut self) {}
}

/// A `Drop` impl runs where Rust drops the value, which Go has no place for yet.
pub fn dropped(v: u8) -> u8 {
	let logged = Logged(v);
	logged.0
}

pub fn dynamic(f: &mut dyn Iterator<Item = u8>) -> Option<u8> {
	f.next()
}

union Bits {
	int: u32,
	float: f32,
}

pub fn unioned(v: u32) -> f32 {
	unsafe { Bits { int: v }.float }
}

/// An array's by-value iterator is generic over its length.
pub fn owned(v: [u8; 4]) -> u32 {
	let mut sum = 0;
	for b in v {
		sum += u32::from(b);
	}
	sum
}

pub fn float(a: f32) -> f32 {
	a * 2.0
}

/// Go's growth is not Rust's, so a capacity read back would differ.
pub fn capacity() -> usize {
	Vec::<u8>::with_capacity(4).capacity()
}

/// A recursive type whose drop runs no `Drop` impl, which translates beside those refused.
pub struct Node {
	next: Option<Box<Node>>,
	value: u8,
}

pub fn linked(value: u8) -> u8 {
	let node = Node { next: None, value };
	node.value
}

/// Go compares a slice's pointer and length not at all.
pub fn compared(a: *const [u8], b: *const [u8]) -> bool {
	a == b
}

/// Go orders no pointers.
pub fn ordered(a: *const u8, b: *const u8) -> bool {
	a < b
}

/// One constant's array and its second byte, which Go's two variables could not alias.
const PAIR: (&[u8; 3], &u8) = {
	let a = b"abc";
	(a, &a[1])
};

pub fn aliased() -> bool {
	let (a, b) = PAIR;
	std::ptr::eq(&a[1], b)
}

/// Held as its parts, which would hold a Go function type holding itself.
pub enum Callback {
	Call(fn(Callback) -> u8),
	Data([u8; 8]),
}

pub fn called(c: Callback) -> u8 {
	match c {
		Callback::Call(f) => f(Callback::Data([0; 8])),
		Callback::Data(d) => d[0],
	}
}

mod value {
	pub struct Value<T>(pub T);

	impl Value<u8> {
		pub fn new(v: u8) -> Self {
			Self(v)
		}
	}
}

/// Go callers would reach `new` only through the alias, which takes knowing which impls
/// apply to `Value<u8>`.
pub type Byte = value::Value<u8>;

/// A raw pointer's wrapper, which no Go type stands for.
pub use std::ptr::NonNull;

/// Go exports a field only by an uppercase first letter, which `count` takes, and the
/// translation exports only ASCII names.
#[allow(non_snake_case, reason = "the names Go cannot export")]
pub struct Fields {
	pub _count: u8,
	pub Count: u8,
	pub é: u8,
	pub café: u8,
}

/// Its Go name would be `1`, which is no identifier.
#[allow(non_camel_case_types, reason = "the name Go cannot export")]
pub struct _1 {
	pub count: u8,
}

/// A function pointer over byte storage, which Go code could fill with a function that
/// keeps what it is handed past the call.
pub fn handed(f: fn(&mut Vec<u8>)) -> usize {
	let mut v = Vec::new();
	f(&mut v);
	v.len()
}

/// A function pointer beside byte storage, which Go code could fill with a closure that
/// reads `v` while it holds storage of the translation's own.
pub fn beside(v: &mut Vec<u8>, f: fn()) {
	v.push(1);
	f();
	v.push(2);
}

/// A reference inside an argument beside byte storage, which the translation could move
/// out of what handing back walks.
pub fn popped(mut refs: Vec<&mut Vec<u8>>) {
	if let Some(v) = refs.pop() {
		v.push(1);
	}
}

/// A raw pointer beside byte storage, which could see that handing in copied it.
pub fn same(v: Vec<u8>, p: *const u8) -> bool {
	std::ptr::eq(&v[0], p)
}
