//! `panic::set_hook` and `take_hook`, and the `PanicHookInfo` a hook reads: its payload,
//! as text and downcast, its location, and its `Display`, for panics of the crate's own
//! code, of std's functions that take their caller's location, of those an integer's macro
//! defines, of std's own code, at the path the native build names, and of a 2018-edition
//! crate's `panic!`, whose payload is any value; a hook that panics, which aborts; and
//! `thread::panicking` within one and outside.
//!
//! Each case's hook writes what it read to [`NOTE`], which the host reads once the panic
//! traps, and passes the panic on to the hook it replaced, the harness's natively.

use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap, LinkedList, VecDeque};
use std::fmt;
use std::future::Future;
use std::ops::Bound;
use std::panic::{self, PanicHookInfo};
use std::pin::Pin;
use std::sync::atomic::{Ordering, compiler_fence, fence};
use std::sync::{Mutex, OnceLock};
use std::task::{Context, Poll, Waker};
use std::time::Duration;

use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::NOTE;

/// Write what `info` says: its `Display`, its payload's type and text, a `String` payload's
/// capacity, its location's parts, and whether the thread is panicking.
fn record(info: &PanicHookInfo<'_>) {
	let payload = info.payload();
	let kind = if payload.type_id() == std::any::TypeId::of::<&'static str>() {
		"&str".to_string()
	} else if let Some(s) = payload.downcast_ref::<String>() {
		format!("String {}", s.capacity())
	} else if let Some(n) = payload.downcast_ref::<u32>() {
		format!("u32 {n}")
	} else if let Some(n) = payload.downcast_ref::<u8>() {
		format!("u8 {n}")
	} else {
		"other".to_string()
	};
	let at = info.location().unwrap();
	let line = format!(
		"{}\n{kind} {:?} {} {} {} {}\n",
		info.to_string().replace(at.file(), &std_named(at.file())),
		info.payload_as_str(),
		std_named(at.file()),
		at.line(),
		at.column(),
		std::thread::panicking(),
	);
	NOTE.with_borrow_mut(|note| note.push_str(&line));
}

/// `file`, with std's source under one name. The native build names std's code by the
/// sysroot's copy of its source where it compiled that code into this crate, as it does a
/// generic or inlined function, and by `/rustc/<commit>/library/` where std's own build
/// did, as the translation always does.
fn std_named(file: &str) -> String {
	match file.split_once("/library/") {
		Some((root, rest)) if root.starts_with("/rustc/") || root.ends_with("/lib/rustlib/src/rust") => {
			format!("<std>/library/{rest}")
		}
		_ => file.to_string(),
	}
}

/// Replace the hook with one that records each panic, then passes it on.
fn hook() {
	let prev = panic::take_hook();
	panic::set_hook(Box::new(move |info| {
		record(info);
		prev(info);
	}));
}

#[track_caller]
fn tracked(n: u8) -> u8 {
	panic!("tracked {n}")
}

/// One panic, by `kind`, with a hook set. `data`'s first byte varies its operands.
pub fn panicked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	hook();
	let v = vec![n, n];
	match kind % 37 {
		0 => panic!("literal"),
		1 => panic!("formatted {n}"),
		2 => out.push(v[usize::from(n) + 2]),
		3 => out.push(n.max(200) + n.max(100)),
		4 => out.push(Some(n).filter(|_| false).unwrap()),
		5 => out.push(Err::<u8, u8>(n).unwrap()),
		6 => out.push(Some(n).filter(|_| false).expect("expected")),
		7 => out.extend_from_slice(&v[usize::from(n % 2) + 2..1]),
		8 => {
			let cell = RefCell::new(n);
			let _held = cell.borrow_mut();
			out.push(*cell.borrow_mut());
		}
		9 => out.push(n.max(2).pow(9)),
		10 => out.push(100 / n.wrapping_mul(0)),
		11 => out.push(v.clone().remove(usize::from(n) + 2)),
		12 => out.push(v.clone().swap_remove(usize::from(n) + 2)),
		13 => v.clone().rotate_left(usize::from(n) + 3),
		14 => out.push(HashMap::<u8, u8>::new()[&n]),
		15 => panic::panic_any(u32::from(n)),
		16 => panic::panic_any(format!("any {n}")),
		17 => assert_eq!(n, n.wrapping_add(1), "differ"),
		18 => out.push(tracked(n)),
		19 => out.push(VecDeque::from(v.clone())[usize::from(n) + 2]),
		20 => out.push(u8::try_from((Duration::MAX + Duration::from_secs(u64::from(n.max(1)))).as_secs()).unwrap_or(0)),
		21 => out.extend_from_slice("éé"[..usize::from(n % 2) * 2 + 1].as_bytes()),
		22 => out.extend((0..n).step_by(usize::from(n) * 0)),
		23 => out.extend(v.clone().drain(usize::from(n) + 3..)),
		24 => panic::panic_any("any"),
		25 => out.extend(v.iter().step_by(usize::from(n) * 0)),
		26 => assert!(n == n.wrapping_add(1)),
		27 => panic!(),
		28 => unreachable!(),
		29 => todo!(),
		30 => assert_ne!(n, n),
		31 => unimplemented!("not {n}"),
		// SAFETY: none, past `u8::MAX`: std's check of its precondition aborts.
		32 => out.push(unsafe { n.max(200).unchecked_add(n.max(100)) }),
		// SAFETY: none, of zero: std's check of its precondition aborts.
		33 => out.push(unsafe { std::num::NonZero::new_unchecked(n.wrapping_mul(0)) }.get()),
		// SAFETY: none, past `u8::MAX`: std's check of its integer's precondition aborts in
		// std's code, at the call of the macro that defines `NonZero<u8>`'s.
		34 => out.push(unsafe { std::num::NonZero::new(n.max(200)).unwrap().unchecked_add(n.max(100)) }.get()),
		// SAFETY: none, past `i16::MAX`, at `NonZero<i16>`'s macro call.
		35 => {
			let x = std::num::NonZero::new(i16::from(n.max(200))).unwrap();
			out.extend_from_slice(&unsafe { x.unchecked_mul(x) }.get().to_be_bytes());
		}
		_ => {
			let len = HashMap::<u8, u8>::with_capacity(usize::MAX).len();
			out.push(len as u8);
		}
	}
}

/// A message whose `Display` writes to [`NOTE`] whether the thread is panicking, as it is
/// once std formats the message for the hook, and then panics if `again`.
struct Shown {
	again: bool,
}

impl fmt::Display for Shown {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let line = format!("shown {}\n", std::thread::panicking());
		NOTE.with_borrow_mut(|note| note.push_str(&line));
		if self.again {
			panic!("shown");
		}
		f.write_str("shown")
	}
}

/// A hook that panics, or that sets the hook, or a message that panics as it formats, which
/// aborts as a panic in a hook does, without running the hook again.
pub fn aborted(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	let prev = panic::take_hook();
	panic::set_hook(Box::new(move |info| {
		record(info);
		match kind % 5 {
			0 => panic!("again"),
			1 => drop(panic::take_hook()),
			_ => prev(info),
		}
	}));
	out.push(n);
	match kind % 5 {
		3 | 4 => panic!("first {n} {}", Shown { again: kind % 5 == 4 }),
		_ => panic!("first {n}"),
	}
}

thread_local! {
	/// Borrowed for good, as the translation, which goes on after a case panics, holds what
	/// that case held, where the native build starts again.
	static SHARED: RefCell<u8> = const { RefCell::new(0) };
	/// Mutably borrowed for good, as `SHARED` is borrowed.
	static EXCLUSIVE: RefCell<u8> = const { RefCell::new(0) };
}

/// One panic of std's own code, by `kind`, with a hook set: each where the native build
/// names it, in the toolchain's source.
pub fn sited(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	let at = usize::from(n);
	hook();
	let v = vec![n, n];
	let deque = VecDeque::from(v.clone());
	let map = BTreeMap::from([(u16::from(n), n)]);
	let set = BTreeSet::from([u16::from(n)]);
	match kind % 87 {
		0 => out.push(Vec::<u8>::with_capacity(usize::MAX - at).len() as u8),
		1 => v.clone().rotate_right(at + 3),
		2 => out.push(n.clamp(n.max(1), n.max(1) - 1)),
		3 => out.push(f64::from(n).clamp(1.0, 0.0) as u8),
		4 => out.push(f32::from(n).clamp(1.0, 0.0) as u8),
		5 => out.extend_from_slice(char::from(n.max(128)).encode_utf8(&mut [0; 1]).as_bytes()),
		6 => out.push(
			char::from_u32(0x1F600 + u32::from(n))
				.unwrap()
				.encode_utf16(&mut [0; 1])
				.len() as u8,
		),
		7 => out.extend(
			map.range((Bound::Excluded(u16::from(n)), Bound::Excluded(u16::from(n))))
				.map(|(_, &v)| v),
		),
		8 => out.extend(map.range(u16::from(n) + 1..u16::from(n)).map(|(_, &v)| v)),
		9 => out.extend(
			set.range((Bound::Excluded(u16::from(n)), Bound::Excluded(u16::from(n))))
				.map(|&v| v as u8),
		),
		10 => out.extend(set.range(u16::from(n) + 1..u16::from(n)).map(|&v| v as u8)),
		11 => deque.clone().insert(at + 3, n),
		12 => deque.clone().swap(at + 2, 0),
		13 => deque.clone().swap(0, at + 2),
		14 => deque.clone().rotate_left(at + 3),
		15 => deque.clone().rotate_right(at + 3),
		16 => out.extend(deque.clone().split_off(at + 3)),
		17 => out.extend(LinkedList::from([n]).split_off(at + 2)),
		18 => {
			let mutex = Mutex::new(n);
			let _held = mutex.lock();
			out.push(*mutex.lock().unwrap());
		}
		19 => {
			let cell = OnceCell::new();
			out.push(*cell.get_or_init(|| *cell.get_or_init(|| n)));
		}
		20 => {
			let lock = OnceLock::new();
			out.push(*lock.get_or_init(|| *lock.get_or_init(|| n)));
		}
		21 => {
			let mut ready = std::future::ready(n);
			let mut cx = Context::from_waker(Waker::noop());
			for _ in 0..2 {
				if let Poll::Ready(n) = Pin::new(&mut ready).poll(&mut cx) {
					out.push(n);
				}
			}
		}
		22 => {
			let mut ready = std::future::ready(n);
			let mut cx = Context::from_waker(Waker::noop());
			if let Poll::Ready(n) = Pin::new(&mut ready).poll(&mut cx) {
				out.push(n);
			}
			out.push(ready.into_inner());
		}
		23 => out.push(Duration::new(u64::MAX, 1_000_000_000 + u32::from(n)).subsec_nanos() as u8),
		24 => out.push(Duration::from_mins(u64::MAX - u64::from(n)).subsec_nanos() as u8),
		25 => out.push(Duration::from_hours(u64::MAX - u64::from(n)).subsec_nanos() as u8),
		26 => out.push((Duration::ZERO - Duration::from_secs(u64::from(n) + 1)).subsec_nanos() as u8),
		27 => out.push((Duration::MAX * (u32::from(n) + 2)).subsec_nanos() as u8),
		28 => v.clone().extend_from_within(at + 3..),
		29 => out.extend(deque.clone().drain(at + 3..)),
		30 => out.extend(deque.range(at + 3..)),
		31 => out.extend(deque.clone().range_mut(at + 3..).map(|v| *v)),
		32 => {
			let cell = RefCell::new(vec![n]);
			let _held = cell.borrow();
			out.extend(cell.take());
		}
		33 => {
			SHARED.with(|cell| std::mem::forget(cell.borrow()));
			SHARED.with_borrow_mut(|v| *v = n);
		}
		34 => {
			EXCLUSIVE.with(|cell| cell.try_borrow_mut().map(std::mem::forget).unwrap_or_default());
			EXCLUSIVE.with_borrow(|&v| out.push(v));
		}
		35 => out.extend(v.clone().splice(at + 3.., [n])),
		36 => out.extend(
			char::from_digit(u32::from(n), 37 + u32::from(n))
				.map(u32::from)
				.map(|c| c as u8),
		),
		37 => fence([Ordering::Relaxed, Ordering::SeqCst][at / 256]),
		38 => compiler_fence([Ordering::Relaxed, Ordering::SeqCst][at / 256]),
		39 => out.push(Duration::from_secs_f64(-1.0 - f64::from(n)).subsec_nanos() as u8),
		40 => out.push(Duration::from_secs_f32(-1.0 - f32::from(n)).subsec_nanos() as u8),
		41 => out.push(Duration::from_secs(1).mul_f64(-1.0 - f64::from(n)).subsec_nanos() as u8),
		42 => out.push(Duration::from_secs(1).div_f32(-1.0 - f32::from(n)).subsec_nanos() as u8),
		43 => out.push([n.max(1), u8::MAX].iter().sum::<u8>()),
		44 => out.push([u16::from(n.max(2)), u16::MAX].into_iter().product::<u16>() as u8),
		45 => out.push(
			[Duration::MAX, Duration::from_secs(u64::from(n.max(1)))]
				.into_iter()
				.sum::<Duration>()
				.as_secs() as u8,
		),
		46 => out.push(
			[Duration::MAX, Duration::new(u64::from(n) + 1, 0)]
				.iter()
				.sum::<Duration>()
				.as_secs() as u8,
		),
		47 => out.extend(v.clone().extract_if(at + 3.., |_| true)),
		48 => out.push(vec![u16::from(n)].extract_if(..at + 2, |_| true).count() as u8),
		49 => out.extend((u8::MAX - n % 2..).take(3)),
		50 => out.extend((u8::MAX - n % 4..).nth(usize::from(n % 3) + 4)),
		51 => out.extend((u64::MAX - u64::from(n % 2)..).take(3).map(|x| x as u8)),
		52 => out.extend(
			(char::from_u32(0x10_FFFF - u32::from(n % 2)).unwrap()..)
				.take(3)
				.map(|c| c as u8),
		),
		k @ 53..=56 => joined(k - 53, n, out),
		k @ 57..=67 => out.push(compared(k - 57, n)),
		68 if n % 2 == 0 => {
			let mut v = Vec::<u8>::with_capacity(at % 4);
			// SAFETY: none, past the capacity: std's check of its precondition aborts.
			unsafe { v.set_len(v.capacity() + 1) };
			out.push(v.len() as u8);
		}
		68 => {
			let mut v = Vec::<u32>::with_capacity(at % 4);
			// SAFETY: none, past the capacity: std's check of its precondition aborts.
			unsafe { v.set_len(v.capacity() + 1) };
			out.push(v.len() as u8);
		}
		69 => _ = [std::mem::MaybeUninit::uninit(); 2].write_copy_of_slice(&v[..at % 2]),
		70 => _ = [const { std::mem::MaybeUninit::uninit() }; 2].write_clone_of_slice(&[String::new()][..at % 2]),
		71 => out.push(Box::<[u16]>::new_uninit_slice(usize::MAX - at).len() as u8),
		72 => {
			let mut lent = std::io::IoSlice::new(&v);
			lent.advance(at % 2 + 3);
			out.extend_from_slice(&lent);
		}
		73 => {
			let mut lent = [std::io::IoSlice::new(&v)];
			let mut left = &mut lent[..];
			std::io::IoSlice::advance_slices(&mut left, at % 2 + 3);
			out.push(left.len() as u8);
		}
		74 => v.clone().insert(at + 3, n),
		75 => out.extend(v.clone().split_off(at + 3)),
		76 => out.push(vec![u16::from(n)].remove(at + 1) as u8),
		77 => out.push(vec![u16::from(n)].swap_remove(at + 1) as u8),
		78 => vec![u16::from(n)].insert(at + 2, 0),
		79 => out.extend(vec![u16::from(n)].split_off(at + 2).iter().map(|&w| w as u8)),
		80 => v.clone().copy_within(at + 1..at + 3, 0),
		81 => out.push(u64::from(n).clamp(u64::from(n) + 1, u64::from(n)) as u8),
		82 => out.push((-i32::from(n)).clamp(0, -1) as u8),
		83 => out.extend_from_slice(format!("{Failing}").as_bytes()),
		84 => out.extend_from_slice(Failing.to_string().as_bytes()),
		85 => out.push(Lying(n).len() as u8),
		_ => out.extend(Short(n % 4).zip(0..n % 4 + 1).rev().map(|(a, b)| a ^ b)),
	}
}

/// A `Display` that fails though the writer did not, which `format!` and `to_string`
/// panic on.
struct Failing;

impl fmt::Display for Failing {
	fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
		Err(fmt::Error)
	}
}

/// A `concat` or `join` whose length overflows, by `kind`: its sum of the parts' lengths,
/// or the capacity the sum reserves.
fn joined(kind: u8, n: u8, out: &mut Vec<u8>) {
	let big = vec![n; 1 << 16];
	let refs = |count: usize| vec![&big[..]; count];
	let joined = match kind {
		0 => refs((1 << 16) + 1).concat(),
		1 => refs((1 << 16) + 1).join(&n),
		2 => refs((1 << 15) + 1).concat(),
		_ => {
			let wide = vec![u16::from(n); 1 << 16];
			let joined = vec![&wide[..]; (1 << 14) + 1].join(&[1u16][..]);
			joined.iter().map(|&w| w as u8).collect()
		}
	};
	out.push(joined.len() as u8);
}

/// A comparison of `RefCell`s, by `kind`, of which `n` picks one to hold mutably borrowed:
/// each of their own comparisons, and those that reach them through a tuple's last element,
/// a `Reverse`, and a vector.
fn compared(kind: u8, n: u8) -> u8 {
	let (a, b) = (RefCell::new(n), RefCell::new(n.wrapping_add(1)));
	let _held = if n % 2 == 0 { a.borrow_mut() } else { b.borrow_mut() };
	let order = |o: Option<std::cmp::Ordering>| o.map_or(9, |o| o as i8 as u8);
	match kind {
		0 => u8::from(a == b),
		1 => u8::from(a != b),
		2 => order(a.partial_cmp(&b)),
		3 => u8::from(a < b),
		4 => u8::from(a <= b),
		5 => u8::from(a > b),
		6 => u8::from(a >= b),
		7 => order(Some(a.cmp(&b))),
		8 => u8::from((n, &a) < (n, &b)),
		9 => u8::from(std::cmp::Reverse(&a) < std::cmp::Reverse(&b)),
		_ => u8::from([&a] == [&b]),
	}
}

/// An iterator whose size hint has no upper bound, though it says its length is exact.
struct Lying(u8);

impl Iterator for Lying {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		None
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(usize::from(self.0), None)
	}
}

impl ExactSizeIterator for Lying {}

/// An iterator of one item fewer than the length it says, which a zip from the back finds
/// short.
struct Short(u8);

impl Iterator for Short {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.next_back()
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = usize::from(self.0) + 1;
		(len, Some(len))
	}
}

impl DoubleEndedIterator for Short {
	fn next_back(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

impl ExactSizeIterator for Short {}

/// One panic of `bytes`' own code, by `kind`, with a hook set: each where the native build
/// names it, in the source of the `bytes` it builds.
pub fn stock(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	let at = usize::from(n);
	let whole = Bytes::from(vec![n; 4]);
	// `slice_ref`'s message names addresses, which JS has none of, so its hook reads where
	// alone, then panics, which aborts and leaves the message unread.
	if let k @ 0..=1 = kind % 33 {
		panic::set_hook(Box::new(|info| {
			let at = info.location().unwrap();
			let line = format!("{} {} {}\n", at.file(), at.line(), at.column());
			NOTE.with_borrow_mut(|note| note.push_str(&line));
			panic!("unread");
		}));
		let subset = whole.slice(..1);
		let held = match k {
			0 => whole.slice(1 + at % 2..).slice_ref(&subset),
			_ => whole.slice(..1).slice_ref(&whole[..2 + at % 2]),
		};
		out.extend_from_slice(&held);
		return;
	}
	hook();
	let mut shared = BytesMut::from(&[n; 8][..]);
	match kind % 33 {
		2 => out.extend_from_slice(&whole.slice(at + 3..2)),
		3 => out.extend_from_slice(&whole.slice(..at + 5)),
		4 => out.extend_from_slice(&whole.slice(usize::MAX - at % 2..=usize::MAX)),
		5 => out.extend_from_slice(&whole.clone().split_to(at + 5)),
		6 => out.extend_from_slice(&whole.clone().split_off(at + 5)),
		7 => whole.clone().advance(at + 5),
		8 => out.push(whole.slice(..1).get_u16() as u8),
		9 => out.push(whole.clone().get_uint(9 + at % 2) as u8),
		10 => (&data[..]).advance(data.len() + 1),
		11 => whole.clone().copy_to_slice(&mut [0; 5]),
		12 => out.extend_from_slice(&whole.clone().copy_to_bytes(at + 5)),
		13 => out.extend_from_slice(&(&data[..]).copy_to_bytes(data.len() + 1)),
		14 => out.extend_from_slice(&BytesMut::from(data).split_to(data.len() + 1)),
		15 => out.extend_from_slice(&BytesMut::with_capacity(4).split_off(at + 5)),
		16 => {
			let mut b = BytesMut::with_capacity(at % 4);
			// SAFETY: none, past the capacity: `bytes`' debug assertion panics first.
			unsafe { b.set_len(b.capacity() + 1) };
			out.push(b.len() as u8);
		}
		17 => {
			let mut b = BytesMut::with_capacity(at % 4);
			// SAFETY: none, past the capacity: `bytes` panics first.
			unsafe { b.advance_mut(b.capacity() + 1) };
			out.push(b.len() as u8);
		}
		18 => {
			let mut v = Vec::<u8>::with_capacity(at % 4);
			// SAFETY: none, past the capacity: `bytes` panics first.
			unsafe { v.advance_mut(v.capacity() + 1) };
			out.push(v.len() as u8);
		}
		19 => BytesMut::from(data).advance(data.len() + 1),
		20 => out.push(BytesMut::from(&data[..data.len().min(3)]).get_u32() as u8),
		21 => out.push(BytesMut::with_capacity(usize::MAX - at).capacity() as u8),
		22 => {
			// Shared with the half split off, so `reserve` sums the length and what it asks.
			let _tail = shared.split_off(4);
			shared.reserve(usize::MAX - at % 2);
		}
		23 => {
			// The one handle of storage it is 4 bytes into, so `reserve` adds that too.
			drop(shared.split_to(4));
			shared.reserve(usize::MAX - 4 - at % 2);
		}
		24 => shared.put_uint(u64::from(n), 9 + at % 2),
		25 => out.put_int(i64::from(n), 9 + at % 2),
		26 => (&mut *out).put_bytes(n, usize::MAX - at % 2),
		27 => out.extend_from_slice(&whole.clone().take(2).copy_to_bytes(at + 3)),
		28 => out.push(whole.clone().take(1).get_u16() as u8),
		// Past the chunk `chunk_mut` lends, which is of 64 bytes once it reserved them.
		29 => {
			let chunk = shared.chunk_mut();
			chunk.write_byte(chunk.len() + at % 2, n);
		}
		30 => Vec::<u8>::with_capacity(at % 4).chunk_mut()[65 + at % 2..].write_byte(0, n),
		31 => BytesMut::new().chunk_mut()[..at % 2].copy_from_slice(&[n; 3]),
		_ => out.push(BytesMut::zeroed(usize::MAX - at).len() as u8),
	}
	out.extend_from_slice(&shared);
}

/// The 2018-edition crate's panics, with a hook set on the paths that panic, so the hook
/// does not outlive a case that returns.
pub fn edition2018_panicked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	if kind % 7 != 6 || data.first().copied().unwrap_or(0) > 200 {
		hook();
	}
	edition2018::panicked(kind, data, out);
}

/// A panic of tokio's own code in what translation maps, with a hook set: a oneshot
/// receiver polled again once it gave its value.
pub fn tokio_stock(_kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().copied().unwrap_or(0);
	hook();
	let (tx, mut rx) = tokio::sync::oneshot::channel();
	tx.send(n).unwrap();
	let mut cx = Context::from_waker(Waker::noop());
	for _ in 0..2 {
		if let Poll::Ready(Ok(n)) = Pin::new(&mut rx).poll(&mut cx) {
			out.push(n);
		}
	}
}
