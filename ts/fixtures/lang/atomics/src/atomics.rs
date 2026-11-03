//! Atomics, which one thread never races: each width's loads, stores, and
//! read-modify-writes, which wrap where arithmetic would panic, and std's panics on an
//! ordering an operation can't take, through a `&`, an `Arc`, a field, and a `&mut`. An
//! update whose callback stores to the atomic it updates retries, as std's loop does.

use std::sync::Arc;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Relaxed, Release, SeqCst};
use std::sync::atomic::{
	self, AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16, AtomicU32,
	AtomicU64, AtomicUsize,
};

const ORDERS: [Ordering; 5] = [Relaxed, Release, Acquire, AcqRel, SeqCst];
/// What a load, and a compare-exchange that fails, can take.
const LOADS: [Ordering; 3] = [Relaxed, Acquire, SeqCst];
const STORES: [Ordering; 3] = [Relaxed, Release, SeqCst];
const FENCES: [Ordering; 4] = [Release, Acquire, AcqRel, SeqCst];

/// An ordering from `b`: one of `valid`, or from 0xf0 on any, which may panic.
fn order(b: u8, valid: &[Ordering]) -> Ordering {
	match b >= 0xf0 {
		true => ORDERS[usize::from(b) % ORDERS.len()],
		false => valid[usize::from(b) % valid.len()],
	}
}

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

macro_rules! ints {
	($($name:ident: $atomic:ident, $t:ty;)*) => {$(
		/// Steps of an operation, an ordering, and an operand on one atomic, each putting
		/// what it read.
		fn $name(data: &[u8], out: &mut Vec<u8>) {
			let (first, mut data) = take!($t, data);
			let mut atomic = $atomic::new(first);
			while let [op, b, rest @ ..] = data {
				let (v, rest) = take!($t, rest);
				data = rest;
				let (b, op) = (*b, *op);
				let any = order(b, &ORDERS);
				let failure = order(b ^ 0x0f, &LOADS);
				let old = match op % 16 {
					0 => atomic.load(order(b, &LOADS)),
					1 => {
						atomic.store(v, order(b, &STORES));
						v
					}
					2 => atomic.swap(v, any),
					3 => atomic.fetch_add(v, any),
					4 => atomic.fetch_sub(v, any),
					5 => atomic.fetch_and(v, any),
					6 => atomic.fetch_or(v, any),
					7 => atomic.fetch_xor(v, any),
					8 => atomic.fetch_max(v, any),
					9 => atomic.fetch_min(v, any),
					10 | 11 => {
						// The value it holds, half the time, so that some succeed.
						let current = if b & 1 == 0 { atomic.load(Relaxed) } else { v.wrapping_add(1) };
						let result = match op % 16 == 10 {
							true => atomic.compare_exchange(current, v, any, failure),
							false => atomic.compare_exchange_weak(current, v, any, failure),
						};
						out.push(result.is_ok() as u8);
						match result {
							Ok(x) | Err(x) => x,
						}
					}
					// Each callback stores `v` once, which its first compare-exchange then misses.
					12 => match atomic.try_update(any, failure, |x| {
						out.push(0xc0);
						if x != v && b & 2 == 0 {
							atomic.store(v, Relaxed);
						}
						x.checked_add(v)
					}) {
						Ok(x) => x,
						Err(x) => x.wrapping_add(1),
					},
					13 => atomic.update(any, failure, |x| {
						out.push(0xc1);
						if x != v && b & 2 == 0 {
							atomic.store(v, Relaxed);
						}
						x.wrapping_sub(v) ^ 1
					}),
					14 => {
						#[expect(deprecated, reason = "moq-net calls it")]
						let r = atomic.fetch_update(any, failure, |x| (x != v).then_some(v));
						r.unwrap_or(v)
					}
					_ => {
						// Through `&mut`, which no other reference can see change.
						*atomic.get_mut() ^= v;
						atomic.load(SeqCst)
					}
				};
				out.extend_from_slice(&old.to_be_bytes());
			}
			out.extend_from_slice(&atomic.into_inner().to_be_bytes());
		}
	)*};
}

ints! {
	u8s: AtomicU8, u8;
	u16s: AtomicU16, u16;
	u32s: AtomicU32, u32;
	u64s: AtomicU64, u64;
	usizes: AtomicUsize, usize;
	i8s: AtomicI8, i8;
	i16s: AtomicI16, i16;
	i32s: AtomicI32, i32;
	i64s: AtomicI64, i64;
	isizes: AtomicIsize, isize;
}

fn bools(data: &[u8], out: &mut Vec<u8>) {
	let flag = AtomicBool::new(data.first().is_some_and(|&b| b & 1 == 1));
	let mut data = data;
	while let [op, b, rest @ ..] = data {
		data = rest;
		let (op, b) = (*op, *b);
		let v = b & 1 == 1;
		let any = order(b, &ORDERS);
		let old = match op % 11 {
			0 => flag.load(order(b, &LOADS)),
			1 => {
				flag.store(v, order(b, &STORES));
				v
			}
			2 => flag.swap(v, any),
			3 => flag.fetch_and(v, any),
			4 => flag.fetch_or(v, any),
			5 => flag.fetch_xor(v, any),
			6 => flag.fetch_nand(v, any),
			7 => flag.fetch_not(any),
			8 => match flag.compare_exchange(v, !v, any, order(b ^ 0x0f, &LOADS)) {
				Ok(x) => x,
				Err(x) => !x,
			},
			9 => {
				atomic::fence(order(b, &FENCES));
				atomic::compiler_fence(order(b ^ 0x0f, &FENCES));
				v
			}
			_ => flag.update(any, Relaxed, |x| {
				out.push(0xc1);
				if x != v {
					flag.store(v, Relaxed);
				}
				x != v
			}),
		};
		out.push(old as u8);
	}
	out.push(flag.into_inner() as u8);
}

/// Counters shared by `Arc`, in fields, as moq-net's stats are.
#[derive(Default)]
struct Stats {
	sent: AtomicU64,
	open: AtomicBool,
	peers: AtomicUsize,
}

/// A load at an ordering passed in, as moq-net's frame does.
fn sent(stats: &Stats, ord: Ordering) -> u64 {
	stats.sent.load(ord)
}

fn shared(data: &[u8], out: &mut Vec<u8>) {
	let stats = Arc::new(Stats::default());
	let peer = Arc::clone(&stats);
	for &b in data {
		let stats = if b & 1 == 0 { &stats } else { &peer };
		stats.sent.fetch_add(u64::from(b) << 50, Relaxed);
		stats.peers.fetch_sub(usize::from(b & 2 == 0), Release);
		stats.open.store(b > 0x80, Release);
		out.push(stats.open.load(Acquire) as u8);
	}
	out.extend_from_slice(&sent(&peer, order(data.len() as u8, &LOADS)).to_be_bytes());
	out.extend_from_slice(&peer.peers.load(SeqCst).to_be_bytes());
	out.push(Arc::strong_count(&stats) as u8);
}

pub fn atomics(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 12 {
		0 => u8s(data, out),
		1 => u16s(data, out),
		2 => u32s(data, out),
		3 => u64s(data, out),
		4 => usizes(data, out),
		5 => i8s(data, out),
		6 => i16s(data, out),
		7 => i32s(data, out),
		8 => i64s(data, out),
		9 => isizes(data, out),
		10 => bools(data, out),
		_ => shared(data, out),
	}
}
