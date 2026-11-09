//! Types generic over a const, as arrayvec's `ArrayString<CAP>` and `ArrayVec<T, CAP>`
//! are: each instance's functions read its own value, though the instances share a
//! class. Of a length, a modulus, a `bool`, and a `char`, at several values in one body,
//! held by each other, behind a trait object, and dropping by a `Drop` impl each instance
//! has its own of.

use std::cell::RefCell;
use std::fmt;
use std::ops::{Add, Deref};

/// A string of at most `CAP` bytes, held inline.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Text<const CAP: usize> {
	bytes: [u8; CAP],
	len: usize,
}

/// What didn't fit in a [`Text`], and the capacity it didn't fit in.
#[derive(Debug, PartialEq)]
pub struct Full<const CAP: usize>(String);

impl<const CAP: usize> Text<CAP> {
	const CAPACITY: usize = CAP;

	pub fn new() -> Self {
		Self {
			bytes: [0; CAP],
			len: 0,
		}
	}

	pub fn capacity(&self) -> usize {
		Self::CAPACITY
	}

	pub fn try_push_str(&mut self, s: &str) -> Result<(), Full<CAP>> {
		let end = self.len + s.len();
		if end > CAP {
			return Err(Full(s.to_string()));
		}
		self.bytes[self.len..end].copy_from_slice(s.as_bytes());
		self.len = end;
		Ok(())
	}

	/// At most `CAP` bytes of `s`, cut at a char boundary.
	pub fn truncated(s: &str) -> Self {
		let mut text = Self::new();
		for c in s.chars() {
			if text.try_push_str(c.encode_utf8(&mut [0; 4])).is_err() {
				break;
			}
		}
		text
	}
}

impl<const CAP: usize> Default for Text<CAP> {
	fn default() -> Self {
		Self::new()
	}
}

impl<const CAP: usize> Deref for Text<CAP> {
	type Target = str;

	fn deref(&self) -> &str {
		std::str::from_utf8(&self.bytes[..self.len]).expect("whole chars")
	}
}

impl<const CAP: usize> fmt::Display for Text<CAP> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}/{CAP}", &**self)
	}
}

impl<const CAP: usize> fmt::Debug for Text<CAP> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

/// Strings of each capacity in one body, from each byte's chars: pushed until full,
/// compared, ordered, formatted, and printed through a trait object, which each
/// instance's own `Display` stands behind.
pub fn texts(_: u8, data: &[u8], out: &mut Vec<u8>) {
	let s = crate::harness::text(data);
	let mut small = Text::<3>::new();
	let mut large: Text<16> = Text::default();
	let mut none = Text::<0>::new();
	let mut failed = Vec::new();
	for c in s.chars() {
		let c = c.encode_utf8(&mut [0; 4]).to_string();
		if let Err(full) = small.try_push_str(&c) {
			failed.push(format!("{full:?}"));
		}
		if large.try_push_str(&c).is_err() {
			failed.push(format!("{}", large.len()));
		}
		if none.try_push_str(&c).is_err() && none.try_push_str("").is_ok() {
			failed.push(format!("{none}"));
		}
	}
	let cut = Text::<{ 2 + 3 }>::truncated(&s);
	let again = Text::<5>::truncated(&cut);
	let shown: Vec<Box<dyn fmt::Display>> = vec![Box::new(small), Box::new(large), Box::new(cut), Box::new(none)];
	for d in &shown {
		out.extend_from_slice(d.to_string().as_bytes());
		out.push(b'|');
	}
	let s = format!(
		"{small:?} {large:?} {} {} {} {} {:?} {:?} {failed:?}",
		small.capacity(),
		large.capacity(),
		none.capacity(),
		cut == again,
		cut.cmp(&again),
		Text::<4>::truncated(&small).partial_cmp(&Text::truncated(&large)),
	);
	out.extend_from_slice(s.as_bytes());
}

/// An integer modulo `M`, whose `+` wraps there.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mod<const M: u32>(u32);

impl<const M: u32> Mod<M> {
	pub fn new(n: u32) -> Self {
		Self(n % M)
	}
}

impl<const M: u32> Add for Mod<M> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self((self.0 + other.0) % M)
	}
}

/// A `char` repeated as often as a flag says, by its const parameters alone.
pub struct Mark<const C: char, const TWICE: bool>;

impl<const C: char, const TWICE: bool> fmt::Display for Mark<C, TWICE> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match TWICE {
			true => write!(f, "{C}{C}"),
			false => write!(f, "{C}"),
		}
	}
}

/// What `N` bytes hold, inline or else on the heap, as an enum generic over its length.
#[derive(Clone, Debug, PartialEq)]
pub enum Small<const N: usize> {
	Inline([u8; N], u8),
	Heap(Vec<u8>),
}

impl<const N: usize> Small<N> {
	pub fn from(data: &[u8]) -> Self {
		match data.len() <= N {
			true => {
				let mut bytes = [0; N];
				bytes[..data.len()].copy_from_slice(data);
				Small::Inline(bytes, data.len() as u8)
			}
			false => Small::Heap(data.to_vec()),
		}
	}

	pub fn bytes(&self) -> &[u8] {
		match self {
			Small::Inline(bytes, len) => &bytes[..usize::from(*len)],
			Small::Heap(v) => v,
		}
	}
}

/// Bytes of a length, which copy.
#[derive(Clone, Copy, Debug)]
pub struct Block<const N: usize>([u8; N]);

/// `a` cloned out of an `Option` a pair holds and written back whole, in a body generic
/// over its length.
fn reclone<const N: usize>(a: Option<Block<N>>) -> Option<Block<N>> {
	let mut pair = (a, 0u8);
	#[allow(clippy::clone_on_copy, reason = "the clone `Option`'s `Clone` makes")]
	let copy = pair.0.clone();
	pair.0 = copy;
	pair.0
}

/// Sums of each byte modulo two moduli, marks, small byte strings of two lengths, and
/// blocks of bytes copied out of an `Option`, which a write to the original leaves.
pub fn values(_: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut seven = Mod::<7>::new(0);
	let mut big = Mod::<{ u32::MAX - 4 }>::new(u32::MAX);
	for &b in data {
		seven = seven + Mod::new(u32::from(b));
		big = big + Mod::new(u32::from(b) << 24);
	}
	let marks = format!(
		"{} {} {}",
		Mark::<'x', false>, Mark::<'é', true>, Mark::<'😀', { 1 > 0 }>
	);
	let two = Small::<2>::from(data);
	let four = Small::<4>::from(data);
	let halves: Vec<Small<4>> = data.chunks(3).map(Small::from).collect();
	let s = format!(
		"{seven:?} {big:?} {marks} {two:?} {four:?} {:?} {:?} {}",
		two.bytes(),
		four.bytes(),
		halves.iter().filter(|h| matches!(h, Small::Inline(..))).count(),
	);
	out.extend_from_slice(s.as_bytes());
	out.extend(halves.iter().flat_map(|h| h.bytes()));
	let mut block = Block::<3>(std::array::from_fn(|i| data.get(i).copied().unwrap_or(0)));
	let copied = reclone(Some(block));
	block.0[0] ^= 0xff;
	let s = format!("{block:?} {copied:?} {:?}", reclone::<0>(None));
	out.extend_from_slice(s.as_bytes());
}

thread_local! {
	/// Each drop of a [`Stack`]: its capacity and length.
	static DROPS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// At most `N` values, held inline, which logs its capacity and length as it drops.
pub struct Stack<T, const N: usize> {
	items: [Option<T>; N],
	len: usize,
}

impl<T, const N: usize> Stack<T, N> {
	pub fn new() -> Self {
		Self {
			items: std::array::from_fn(|_| None),
			len: 0,
		}
	}

	/// `t`, or it back once full.
	pub fn push(&mut self, t: T) -> Result<(), T> {
		match self.items.get_mut(self.len) {
			Some(slot) => {
				*slot = Some(t);
				self.len += 1;
				Ok(())
			}
			None => Err(t),
		}
	}

	pub fn pop(&mut self) -> Option<T> {
		self.len = self.len.checked_sub(1)?;
		self.items[self.len].take()
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.items[..self.len].iter().flatten()
	}
}

impl<T, const N: usize> Drop for Stack<T, N> {
	fn drop(&mut self) {
		DROPS.with_borrow_mut(|log| log.extend([N as u8, self.len as u8]));
	}
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for Stack<T, N> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

/// Stacks of each capacity, of bytes, of strings, of texts, and of stacks, pushed and
/// popped by each byte, each dropping, with what it holds, by its own `Drop`.
pub fn stacks(_: u8, data: &[u8], out: &mut Vec<u8>) {
	DROPS.with_borrow_mut(Vec::clear);
	{
		let mut bytes = Stack::<u8, 3>::new();
		let mut words: Stack<String, 2> = Stack::new();
		let mut texts = Stack::<Text<2>, 4>::new();
		let mut nested = Stack::<Stack<u8, 1>, 2>::new();
		let mut none = Stack::<String, 0>::new();
		for &b in data {
			match b % 4 {
				0 => out.push(bytes.pop().unwrap_or(0xff)),
				1 => {
					if let Err(rejected) = bytes.push(b) {
						out.push(rejected);
					}
					if let Err(rejected) = none.push(format!("n{b}")) {
						out.extend_from_slice(rejected.as_bytes());
					}
				}
				2 => {
					if let Err(w) = words.push(format!("w{b}")) {
						out.extend_from_slice(w.as_bytes());
					}
					let _ = texts.push(Text::truncated(&format!("{b}")));
				}
				_ => {
					let mut inner = Stack::new();
					let _ = inner.push(b);
					let _ = inner.push(!b);
					match nested.push(inner) {
						Ok(()) => {}
						Err(inner) => drop(inner),
					}
					drop(words.pop());
				}
			}
		}
		let s = format!("{bytes:?} {words:?} {texts:?} {nested:?} {none:?}");
		out.extend_from_slice(s.as_bytes());
	}
	DROPS.with_borrow_mut(|log| out.append(log));
}
