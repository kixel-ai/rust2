//! `bytes`' `BytesMut`, `Buf`, and `BufMut`: each case runs ops its input picks, so the
//! differential test reaches their panics and what `split_to` and `freeze` share.

use std::cell::Cell;
use std::collections::VecDeque;
use std::io::IoSlice;
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use bytes::buf::UninitSlice;
use bytes::{Buf, BufMut, Bytes, BytesMut};

/// A `BytesMut` split, frozen, grown, written, and joined, its handles and `Bytes` dropped,
/// cloned, and taken back, and written past its length through `spare_capacity_mut`, ending
/// with every piece's bytes and capacity: a write to one that reached another would show,
/// and so would a capacity `bytes` reclaimed only once the other handles of its storage
/// dropped, or a write to its spare room that missed its storage.
pub fn bytes_mut(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut buf = BytesMut::with_capacity(ops.byte() as usize % 24);
	let mut pieces: Vec<BytesMut> = Vec::new();
	let mut frozen: Vec<Bytes> = Vec::new();
	while let Some(op) = ops.next() {
		let n = ops.byte() as usize;
		match op % 30 {
			0 => buf.extend_from_slice(ops.take(n % 12)),
			1 => buf.put_u8(n as u8),
			2 => buf.put_u16(n as u16 * 257),
			3 => pieces.push(buf.split_to(n % (buf.len() + 2))),
			4 => {
				if let Some(piece) = pieces.pop() {
					frozen.push(piece.freeze());
				}
			}
			5 => buf.truncate(n % (buf.len() + 2)),
			6 => buf.resize(n % 40, n as u8),
			// Past what a vector holds, by overflowing `usize` or not, counting what it read
			// past: each panics as `Vec` does, or as `bytes` does of shared storage.
			7 if n % 4 == 0 => {
				let mut fresh = BytesMut::from(ops.take(n % 6));
				let read = n % (fresh.len() + 1);
				fresh.advance(read);
				let len = fresh.len();
				fresh.reserve(match n % 3 {
					0 => usize::MAX - n,
					1 => isize::MAX as usize - len + 1,
					_ => isize::MAX as usize + 1 - read - len,
				});
			}
			7 if n % 4 == 2 => {
				let len = buf.len();
				buf.reserve(match n % 8 {
					2 => usize::MAX,
					_ => isize::MAX as usize - len + 1,
				});
			}
			7 => buf.reserve(n),
			8 => {
				if let Some(piece) = pieces.pop() {
					buf.unsplit(piece);
				}
			}
			9 => buf.advance(n % (buf.len() + 2)),
			10 => out.extend_from_slice(&read(&mut buf, n).to_be_bytes()),
			11 => buf.clear(),
			12 => pieces.push(buf.split()),
			13 => frozen.push(buf.copy_to_bytes(n % (buf.len() + 2))),
			14 => buf.put_bytes(n as u8, n % 9),
			15 => {
				// Through `DerefMut`, into what this handle alone holds.
				if !buf.is_empty() {
					let at = n % buf.len();
					buf[at] ^= 0xff;
				}
				if let Some(piece) = pieces.last_mut() {
					piece.put_slice(b"xy");
				}
			}
			16 => buf.put(Bytes::copy_from_slice(ops.take(n % 5))),
			17 => {
				let mut source = BytesMut::from(ops.take(n % 6));
				// Its own `Buf`, read into a `BufMut`, then the rest whole.
				let first = source.get_u8_checked();
				buf.put_u8(first);
				buf.put(source);
			}
			18 => pieces.push(buf.split_off(n % (buf.capacity() + 2))),
			19 => out.push(buf.try_reclaim(n) as u8),
			20 => {
				if let Some(piece) = pieces.pop() {
					match buf.try_unsplit(piece) {
						Ok(()) => out.push(1),
						Err(piece) => {
							out.push(0);
							pieces.push(piece);
						}
					}
				}
			}
			// Any one of them dropped, which may leave another the last of its storage.
			21 if n % 2 == 0 && !pieces.is_empty() => drop(pieces.swap_remove(n / 2 % pieces.len())),
			21 if !frozen.is_empty() => drop(frozen.swap_remove(n / 2 % frozen.len())),
			22 => {
				if let Some(bytes) = frozen.get(n % (frozen.len() + 1)) {
					frozen.push(bytes.clone());
				}
			}
			// A `Bytes` taken back, which takes its storage if it is the last handle of it.
			23 => {
				if let Some(bytes) = frozen.pop() {
					match n % 3 {
						0 => pieces.push(BytesMut::from(bytes)),
						1 => match bytes.try_into_mut() {
							Ok(piece) => pieces.push(piece),
							Err(bytes) => {
								out.push(2);
								frozen.push(bytes);
							}
						},
						_ => {
							let vec: Vec<u8> = bytes.into();
							out.extend_from_slice(&(vec.capacity() as u32).to_le_bytes());
							out.extend_from_slice(&vec);
						}
					}
				}
			}
			// A promotable `Bytes` truncated to nothing gives its storage up.
			24 => {
				if let Some(bytes) = frozen.last_mut() {
					match n % 3 {
						0 => bytes.truncate(n / 3 % (bytes.len() + 1)),
						1 => bytes.clear(),
						_ => out.push(bytes.is_unique() as u8),
					}
				}
			}
			25 => {
				if let Some(piece) = pieces.last_mut() {
					std::mem::swap(&mut buf, piece);
				}
			}
			// Its spare room, as long as its capacity leaves, written byte by byte and taken in.
			26 => {
				let spare = buf.spare_capacity_mut();
				out.extend_from_slice(&(spare.len() as u32).to_le_bytes());
				let k = n % (spare.len() + 1);
				for (i, slot) in spare[..k].iter_mut().enumerate() {
					slot.write(n as u8 ^ i as u8);
				}
				unsafe { buf.set_len(buf.len() + k) };
			}
			// Copied into, then taken in by `advance_mut`, which panics past the room.
			27 => {
				let src = ops.take(n % 6);
				let spare = buf.spare_capacity_mut();
				let (k, room) = (src.len().min(spare.len()), spare.len());
				spare[..k].write_copy_of_slice(&src[..k]);
				unsafe { buf.advance_mut(if n % 16 == 15 { room + 1 } else { k }) };
			}
			// Written, then read again through a second call, before it is taken in.
			28 => {
				if let Some(slot) = buf.spare_capacity_mut().first_mut() {
					slot.write(n as u8);
				}
				if let Some(slot) = buf.spare_capacity_mut().first() {
					out.push(unsafe { slot.assume_init() });
				}
			}
			_ => out.extend_from_slice(&(buf.capacity() as u32).to_le_bytes()),
		}
	}
	let tail = BytesMut::from(&b"tail"[..]);
	out.push((buf == tail) as u8);
	out.push((buf.clone() == buf[..]) as u8);
	out.extend_from_slice(format!("{buf:?} {} {}\n", buf.len(), buf.capacity()).as_bytes());
	for piece in &pieces {
		out.extend_from_slice(format!("{piece:?} {}\n", piece.capacity()).as_bytes());
	}
	for bytes in &frozen {
		out.extend_from_slice(format!("{bytes:?} {}\n", bytes.is_unique()).as_bytes());
	}
	let vec: Vec<u8> = buf.into();
	out.extend_from_slice(&(vec.capacity() as u32).to_le_bytes());
	out.extend_from_slice(&vec);
}

trait Checked {
	/// The next byte, or 0 if none remains.
	fn get_u8_checked(&mut self) -> u8;
}

impl<B: Buf> Checked for B {
	fn get_u8_checked(&mut self) -> u8 {
		match self.has_remaining() {
			true => self.get_u8(),
			false => 0,
		}
	}
}

/// A scalar read off the front of a buffer by `n`'s pick, as a `u128`'s bytes, or a
/// `try_get_*`'s error as what it asked for and what remained.
fn read<B: Buf + ?Sized>(buf: &mut B, n: usize) -> u128 {
	let tried = |got: Result<u128, bytes::TryGetError>| match got {
		Ok(v) => v,
		Err(e) => (e.requested as u128) << 64 | e.available as u128,
	};
	match n % 19 {
		15 => tried(buf.try_get_u16().map(u128::from)),
		16 => tried(buf.try_get_i64_le().map(|v| v as u128)),
		17 => tried(buf.try_get_u128()),
		18 => tried(buf.try_get_u8().map(u128::from)),
		0 => buf.get_u8().into(),
		1 => buf.get_i8() as u128,
		2 => buf.get_u16().into(),
		3 => buf.get_i16_le() as u128,
		4 => buf.get_u32_le().into(),
		5 => buf.get_i32() as u128,
		6 => buf.get_u64().into(),
		7 => buf.get_i64_le() as u128,
		8 => buf.get_u128_le(),
		9 => buf.get_i128() as u128,
		10 => buf.get_uint(n % 10).into(),
		11 => buf.get_int_le(n % 10) as u128,
		12 => buf.get_f64().to_bits().into(),
		13 => buf.get_f32_le().to_bits().into(),
		_ => {
			let mut dst = [0; 3];
			buf.copy_to_slice(&mut dst[..n % 4]);
			u32::from_be_bytes([0, dst[0], dst[1], dst[2]]).into()
		}
	}
}

/// A `BytesMut` read past the furthest a vector reads past on wasm32, 128 MiB, or just
/// that far, which makes its storage shared with no other handle, then frozen full, which
/// is promotable only of a vector, and taken back through a clone, and grown or reclaimed.
/// Only the inputs `caller.ts` names reach these, as each allocates that much.
pub fn far(data: &[u8], out: &mut Vec<u8>) {
	const MAX_VEC_POS: usize = (1 << 27) - 1;
	let &[b'f', b'a', b'r', past, op] = data else {
		return;
	};
	let mut buf = BytesMut::with_capacity(MAX_VEC_POS + 64);
	buf.resize(MAX_VEC_POS + 8, 1);
	buf.advance(MAX_VEC_POS + usize::from(past % 2));
	if op & 4 != 0 {
		buf.resize(buf.capacity(), 2);
		let mut bytes = buf.freeze();
		let clone = bytes.clone();
		bytes.clear();
		buf = BytesMut::from(clone);
	}
	match op % 4 {
		0 => buf.reserve(100),
		1 => out.push(buf.try_reclaim(1 << 20) as u8),
		2 => {
			drop(buf.split_off(buf.len()));
			buf.reserve(4);
		}
		_ => {
			let vec: Vec<u8> = buf.split().into();
			out.extend_from_slice(&(vec.capacity() as u32).to_le_bytes());
		}
	}
	out.extend_from_slice(&(buf.capacity() as u32).to_le_bytes());
	out.extend_from_slice(&buf);
}

/// A `BytesMut` reclaimed and reserved for after each of the other handles of its storage
/// dropped or not, by the bits of `kept`: what it split off its end, a `Bytes` frozen of
/// what it split off its front, and a clone of that. `bytes` reclaims the storage only once
/// no other handle holds it, and the `Bytes` taken back takes it only then too.
pub fn reclaims(data: &[u8], out: &mut Vec<u8>) {
	let &[cap, kept, add, read] = data else {
		return;
	};
	let (cap, add) = (usize::from(cap) * 8, usize::from(add));
	let mut buf = BytesMut::with_capacity(cap);
	buf.put_bytes(7, cap / 2);
	buf.advance(usize::from(read) % (buf.len() + 1));
	let head = buf.split_to(buf.len() / 3);
	let mut tail = Some(buf.split_off(buf.len()));
	let mut frozen = Some(head.freeze());
	let mut clone = frozen.clone();
	if kept & 1 == 0 {
		drop(tail.take());
	}
	if kept & 2 == 0 {
		drop(frozen.take());
	}
	if kept & 4 == 0 {
		drop(clone.take());
	}
	let capacity = |buf: &BytesMut, out: &mut Vec<u8>| out.extend_from_slice(&(buf.capacity() as u32).to_le_bytes());
	out.push(buf.try_reclaim(add) as u8);
	capacity(&buf, out);
	buf.reserve(add * 2);
	capacity(&buf, out);
	if let Some(bytes) = frozen.take() {
		capacity(&BytesMut::from(bytes), out);
	}
	drop(tail);
	if let Some(bytes) = clone {
		capacity(&BytesMut::from(bytes), out);
	}
	out.extend_from_slice(&buf);
}

/// A buffer of the crate's own, in pieces: `Buf`'s provided methods read across them.
struct Pieces {
	/// None empty, so `chunk` is empty only once nothing remains, as `Buf` requires.
	parts: Vec<Vec<u8>>,
	/// How far into the first part reads are.
	at: usize,
}

impl Pieces {
	/// `input` in two parts, cut at `cut`.
	fn new(input: &[u8], cut: usize) -> Self {
		let (a, b) = input.split_at(cut);
		let mut parts = vec![a.to_vec(), b.to_vec()];
		parts.retain(|part| !part.is_empty());
		Self { parts, at: 0 }
	}
}

impl Buf for Pieces {
	fn remaining(&self) -> usize {
		self.parts.iter().map(Vec::len).sum::<usize>() - self.at
	}

	fn chunk(&self) -> &[u8] {
		match self.parts.first() {
			Some(part) => &part[self.at..],
			None => &[],
		}
	}

	fn advance(&mut self, mut cnt: usize) {
		while cnt > 0 {
			let left = self.parts[0].len() - self.at;
			if cnt < left {
				self.at += cnt;
				return;
			}
			cnt -= left;
			self.parts.remove(0);
			self.at = 0;
		}
	}

	// Its own, as moq's buffers have: the provided one reads through a `Take<&mut Self>`.
	fn copy_to_bytes(&mut self, len: usize) -> Bytes {
		assert!(len <= self.remaining(), "`len` greater than remaining");
		let mut out = BytesMut::with_capacity(len);
		while out.len() < len {
			let take = self.chunk().len().min(len - out.len());
			out.extend_from_slice(&self.chunk()[..take]);
			self.advance(take);
		}
		out.freeze()
	}
}

/// A buffer in pieces with only `Buf`'s required methods, so `copy_to_bytes` is the
/// provided one, which reads it through a `Take<&mut Self>`.
struct Provided(Pieces);

impl Buf for Provided {
	fn remaining(&self) -> usize {
		self.0.remaining()
	}

	fn chunk(&self) -> &[u8] {
		self.0.chunk()
	}

	fn advance(&mut self, cnt: usize) {
		self.0.advance(cnt);
	}
}

/// The same reads off a `Bytes`, a `BytesMut`, a `&[u8]`, buffers in pieces, a `Take`, and
/// each through the impl forwarding through a `&mut`: each scalar, and what remains.
pub fn reads(data: &[u8], out: &mut Vec<u8>) {
	let [kind, cut, ops @ ..] = data else {
		return;
	};
	let (ops, input) = ops.split_at(ops.len() / 2);
	let cut = *cut as usize % (input.len() + 1);
	let rest = match kind % 9 {
		0 => read_all(&mut Bytes::copy_from_slice(input), ops, out),
		1 => read_all(&mut BytesMut::from(input), ops, out),
		2 => read_all(&mut Pieces::new(input, cut), ops, out),
		3 => read_all(&mut &input[..], ops, out),
		4 => read_all(&mut Provided(Pieces::new(input, cut)), ops, out),
		5 => read_all(&mut &mut Bytes::copy_from_slice(input), ops, out),
		6 => read_all(&mut &mut &input[..], ops, out),
		7 => read_all(&mut Bytes::copy_from_slice(input).take(cut), ops, out),
		_ => read_all(&mut &mut Pieces::new(input, cut), ops, out),
	};
	out.extend_from_slice(&rest);
	// Put whole, chunk by chunk.
	let mut vec = Vec::new();
	vec.put(Pieces::new(input, cut));
	out.extend_from_slice(&vec);
	let slice: &[u8] = input;
	out.push((slice.remaining() == input.len()) as u8);
	out.push((slice.chunk() == input) as u8);
	out.push(slice.has_remaining() as u8);
}

/// `read`s by each of `ops`, then the rest, as `copy_to_bytes` takes it.
fn read_all<B: Buf + ?Sized>(buf: &mut B, ops: &[u8], out: &mut Vec<u8>) -> Vec<u8> {
	for &op in ops {
		if op == 0xff {
			buf.advance(1);
			continue;
		}
		out.extend_from_slice(&read(buf, op as usize).to_be_bytes());
		out.push(buf.has_remaining() as u8);
	}
	let n = buf.remaining();
	buf.copy_to_bytes(n).to_vec()
}

/// The reads of `reads` off a `VecDeque<u8>` whose ring wraps `cut` slots from its end,
/// each after its `chunk`, the front slice until that is read, then the rest through a
/// `&mut`. An `advance` past what remains panics.
pub fn deques(data: &[u8], out: &mut Vec<u8>) {
	let [cut, ops @ ..] = data else {
		return;
	};
	let (ops, input) = ops.split_at(ops.len() / 2);
	let mut deque = VecDeque::with_capacity(input.len());
	for _ in 0..*cut as usize % (input.len() + 1) {
		deque.push_back(0);
		deque.pop_front();
	}
	deque.extend(input);
	for &op in ops {
		let chunk = deque.chunk();
		out.push(chunk.len() as u8);
		out.extend_from_slice(&chunk[..chunk.len().min(1)]);
		match op.checked_sub(0xf0) {
			Some(n) => deque.advance(n.into()),
			None => out.extend_from_slice(&read(&mut deque, op.into()).to_be_bytes()),
		}
	}
	let rest = read_all(&mut &mut deque, &[], out);
	out.extend_from_slice(&rest);
}

/// A buffer of the crate's own over `Buf`, as moq's transports lend a stream's chunks.
trait Leased: Buf {
	fn pieces(&self) -> usize;
	/// Reads a byte of `other`, then skips as many bytes as it has left.
	fn skip(&mut self, other: &mut dyn Buf);
}

impl Leased for Pieces {
	fn pieces(&self) -> usize {
		self.parts.len()
	}

	fn skip(&mut self, other: &mut dyn Buf) {
		if other.has_remaining() {
			other.advance(1);
		}
		self.advance(other.remaining().min(self.remaining()));
	}
}

/// Pieces that count their drops.
struct Counted(Pieces, Rc<Cell<u8>>);

impl Buf for Counted {
	fn remaining(&self) -> usize {
		self.0.remaining()
	}

	fn chunk(&self) -> &[u8] {
		self.0.chunk()
	}

	fn advance(&mut self, cnt: usize) {
		self.0.advance(cnt);
	}
}

impl Drop for Counted {
	fn drop(&mut self) {
		self.1.set(self.1.get() + 1);
	}
}

/// The reads of `reads` through a `dyn Buf` of each buffer: called on the object, through
/// the impls forwarding through a `&mut` to one and a `Box` of one, and through the crate's
/// own trait over `Buf`, whose method takes another `dyn Buf`. A box drops what it owns
/// once.
pub fn dyns(data: &[u8], out: &mut Vec<u8>) {
	let [kind, cut, ops @ ..] = data else {
		return;
	};
	let (ops, input) = ops.split_at(ops.len() / 2);
	let cut = *cut as usize % (input.len() + 1);
	let mut bytes = Bytes::copy_from_slice(input);
	let mut owned = BytesMut::from(input);
	let mut slice = input;
	let mut pieces = Pieces::new(input, cut);
	let mut provided = Provided(Pieces::new(input, cut));
	let mut take = Bytes::copy_from_slice(input).take(cut);
	let buf: &mut dyn Buf = match kind % 6 {
		0 => &mut bytes,
		1 => &mut owned,
		2 => &mut slice,
		3 => &mut pieces,
		4 => &mut provided,
		_ => &mut take,
	};
	let rest = match kind / 6 % 3 {
		0 => read_all(buf, ops, out),
		_ => read_all(&mut &mut *buf, ops, out),
	};
	out.extend_from_slice(&rest);
	out.push(bytes.len() as u8);
	out.push(slice.len() as u8);

	let drops = Rc::new(Cell::new(0));
	let mut boxed: Box<dyn Buf> = match kind % 3 {
		0 => Box::new(Counted(Pieces::new(input, cut), drops.clone())),
		1 => Box::new(Bytes::copy_from_slice(input)),
		_ => Box::new(&input[..]),
	};
	let rest = read_all(&mut boxed, ops, out);
	out.extend_from_slice(&rest);
	drop(boxed);
	out.push(drops.get());

	let mut leased: Box<dyn Leased> = Box::new(Pieces::new(input, cut));
	out.push(leased.pieces() as u8);
	let mut skipped = ops;
	leased.skip(&mut skipped);
	out.push(skipped.len() as u8);
	let rest = read_all(&mut *leased, ops, out);
	out.extend_from_slice(&rest);
	out.push(leased.pieces() as u8);
}

/// A method first called through a `dyn Buf`, then through one of a buffer coerced only in
/// a function this one calls, which translation reaches after, so the object holds the
/// method before that buffer's class is made.
pub fn late(data: &[u8], out: &mut Vec<u8>) {
	let mut slice = data;
	let buf: &mut dyn Buf = &mut slice;
	if buf.remaining() >= 2 {
		out.extend_from_slice(&buf.get_i16_ne().to_le_bytes());
	}
	late_take(slice, out);
}

/// `get_i16_ne` through a box of a `Take` of `data`, a type no other `dyn Buf` is made of.
fn late_take(data: &[u8], out: &mut Vec<u8>) {
	let mut boxed: Box<dyn Buf> = Box::new(data.take(data.len().saturating_sub(1)));
	while boxed.remaining() >= 2 {
		out.extend_from_slice(&boxed.get_i16_ne().to_le_bytes());
	}
	out.push(boxed.remaining() as u8);
}

/// A `Vec<u8>` written past its length through `spare_capacity_mut`, by element, by
/// `write_copy_of_slice` and through the slice it gives back, through a pointer, and
/// split, then lengthened over what was written by `set_len` or by `advance_mut`, which
/// panics past the room; written and read again through a second call; grown and cut. Each
/// step puts its length, capacity, and room, then its bytes.
pub fn vec_spares(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut vec: Vec<u8> = Vec::with_capacity(ops.byte() as usize % 20);
	while let Some(op) = ops.next() {
		let n = ops.byte();
		let spare = vec.spare_capacity_mut();
		let (room, k) = (spare.len(), n as usize % (spare.len() + 1));
		match op % 7 {
			0 => {
				for (i, slot) in spare[..k].iter_mut().enumerate() {
					slot.write(n ^ i as u8);
				}
				unsafe { vec.set_len(vec.len() + k) };
			}
			1 => {
				let src: Vec<u8> = (0..k as u8).map(|i| n.wrapping_add(i)).collect();
				spare[..k].write_copy_of_slice(&src).reverse();
				unsafe { vec.advance_mut(if n % 16 == 15 { room + 1 } else { k }) };
			}
			2 => unsafe {
				let src = [n; 64];
				let k = k.min(src.len());
				std::ptr::copy_nonoverlapping(src.as_ptr(), spare.as_mut_ptr().cast::<u8>(), k);
				vec.set_len(vec.len() + k);
			},
			3 => {
				spare[..k].fill(MaybeUninit::new(n));
				if let Some(slot) = spare[..k].split_at_mut(k / 2).0.last_mut() {
					*slot = MaybeUninit::new(!n);
				}
				unsafe { vec.set_len(vec.len() + k) };
			}
			4 => {
				if let Some(slot) = spare.first_mut() {
					slot.write(n);
				}
				if let Some(slot) = vec.spare_capacity_mut().first() {
					out.push(unsafe { slot.assume_init() });
				}
			}
			5 => vec.reserve(n as usize % 40),
			_ => vec.truncate(n as usize % (vec.len() + 1)),
		}
		for v in [vec.len(), vec.capacity(), vec.spare_capacity_mut().len()] {
			out.extend_from_slice(&(v as u32).to_le_bytes());
		}
	}
	out.extend_from_slice(&vec);
}

/// A `Vec<u8>`, a `BytesMut`, each through a `&mut`, and writers of the crate's own,
/// written through the chunk `chunk_mut` lends, which grows a full buffer by 64 bytes, and
/// taken in by `advance_mut`. Each step puts the chunk's length and the room left, and at
/// the end each buffer's capacity and bytes: a write that missed the storage would show.
pub fn chunks(data: &[u8], out: &mut Vec<u8>) {
	let [first, cap, data @ ..] = data else {
		return;
	};
	let cap = usize::from(*cap % 24);
	let (mut vec, mut buf) = (Vec::with_capacity(cap), BytesMut::with_capacity(cap));
	let mut fixed = Fixed {
		bytes: [0; 48],
		len: cap,
	};
	let mut spare = Spare(Vec::with_capacity(cap), cap % 2 == 0);
	// Each first in turn, so each one's panic is reached.
	for i in 0..6 {
		match first.wrapping_add(i) % 6 {
			0 => chunk_all(&mut vec, data, out),
			1 => chunk_all(&mut buf, data, out),
			2 => chunk_all(&mut &mut vec, data, out),
			3 => chunk_all(&mut Box::new(&mut buf), data, out),
			4 => chunk_all(&mut fixed, data, out),
			_ => chunk_all(&mut spare, data, out),
		}
	}
	for cap in [vec.capacity(), buf.capacity(), spare.0.capacity()] {
		out.extend_from_slice(&(cap as u32).to_le_bytes());
	}
	out.extend_from_slice(&vec);
	out.extend_from_slice(&buf);
	out.extend_from_slice(&fixed.bytes[..fixed.len]);
	out.extend_from_slice(&spare.0);
}

/// A write through `buf`'s chunk by each pair of `data`'s bytes, of each range of it, and
/// past it, which panics as `bytes` does, then `advance_mut` over what it wrote, or past
/// the chunk, which panics too; or a provided write, which a writer of the crate's own
/// makes through its chunk.
fn chunk_all<B: BufMut>(buf: &mut B, data: &[u8], out: &mut Vec<u8>) {
	let mut data = data;
	while let [op, n, rest @ ..] = data {
		data = rest;
		let (op, n) = (*op, *n);
		match op % 13 {
			10 => buf.put_slice(&[n; 70][..usize::from(n) % 71]),
			11 => buf.put_u32_le(u32::from(n) * 0x0101_0101),
			_ => {
				let chunk = buf.chunk_mut();
				let room = chunk.len();
				let written = chunk_write(chunk, op, n, out);
				let cnt = if n % 16 == 15 { room + 1 } else { written };
				// SAFETY: none past the chunk, where `bytes` panics first; else what was written.
				unsafe { buf.advance_mut(cnt) };
				out.extend_from_slice(&(room as u32).to_le_bytes());
			}
		}
		out.extend_from_slice(&(buf.remaining_mut() as u32).to_le_bytes());
	}
}

/// A write into `chunk` that `op` picks, of `n`'s bytes: how many from its front it wrote.
fn chunk_write(chunk: &mut UninitSlice, op: u8, n: u8, out: &mut Vec<u8>) -> usize {
	let room = chunk.len();
	let k = usize::from(n) % (room + 1);
	let src: Vec<u8> = (0..k).map(|i| n.wrapping_add(i as u8)).collect();
	let half = k / 2;
	match op % 13 {
		0 => {
			for (i, &b) in src.iter().enumerate() {
				chunk.write_byte(i, b);
			}
		}
		1 => chunk[..k].copy_from_slice(&src),
		// SAFETY: `k` is at most the chunk's length.
		2 => unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), chunk.as_mut_ptr(), k) },
		// SAFETY: only initialized bytes are written.
		3 => {
			unsafe { chunk.as_uninit_slice_mut()[..k].write_copy_of_slice(&src) };
		}
		4 => {
			chunk[half..][..k - half].copy_from_slice(&src[half..]);
			chunk[0..half].copy_from_slice(&src[..half]);
		}
		5 if k > 0 => {
			chunk[..=k - 1].copy_from_slice(&src);
			chunk[half..=half].write_byte(0, !n);
		}
		6 => chunk[..][..k].copy_from_slice(&src),
		// Past the chunk where `k` is all of it.
		7 => {
			chunk[k..].write_byte(0, n);
			chunk[..k].copy_from_slice(&src);
			return k + 1;
		}
		// Of a slice of another length unless `k` is 3.
		8 => chunk[..k].copy_from_slice(&[n; 3]),
		// Past either end, or of a start past its end, shared or not.
		9 => match n % 3 {
			0 => out.push(chunk[..room + 1].len() as u8),
			1 => chunk[k + 1..k].write_byte(0, n),
			_ => chunk[room + 1..].write_byte(0, n),
		},
		_ => {
			out.extend_from_slice(format!("{chunk:?}").as_bytes());
			return 0;
		}
	}
	k
}

/// A writer of the crate's own into an array, whose chunk is the room left in it.
struct Fixed {
	bytes: [u8; 48],
	len: usize,
}

// SAFETY: the chunk is the array past what was written, which `advance_mut` takes in.
unsafe impl BufMut for Fixed {
	fn remaining_mut(&self) -> usize {
		self.bytes.len() - self.len
	}

	unsafe fn advance_mut(&mut self, cnt: usize) {
		assert!(cnt <= self.remaining_mut(), "advanced {cnt} past the array");
		self.len += cnt;
	}

	fn chunk_mut(&mut self) -> &mut UninitSlice {
		UninitSlice::new(&mut self.bytes[self.len..])
	}
}

/// A writer of the crate's own into a vector's spare room, lent as `bytes` lends a
/// vector's, by its pointer, or by `spare_capacity_mut` if `.1`: none once it is full.
struct Spare(Vec<u8>, bool);

// SAFETY: the chunk is the vector's spare room, which `advance_mut` takes in.
unsafe impl BufMut for Spare {
	fn remaining_mut(&self) -> usize {
		self.0.capacity() - self.0.len()
	}

	unsafe fn advance_mut(&mut self, cnt: usize) {
		assert!(cnt <= self.remaining_mut(), "advanced {cnt} past the capacity");
		// SAFETY: within the capacity, over what the caller wrote.
		unsafe { self.0.set_len(self.0.len() + cnt) };
	}

	fn chunk_mut(&mut self) -> &mut UninitSlice {
		if self.1 {
			return self.0.spare_capacity_mut().into();
		}
		let (len, cap) = (self.0.len(), self.0.capacity());
		// SAFETY: the room past the length is within the allocation.
		unsafe { UninitSlice::from_raw_parts_mut(self.0.as_mut_ptr().add(len), cap - len) }
	}
}

/// Writes into an array through a `&mut [u8]` and a `&mut [MaybeUninit<u8>]` of the room
/// its input picks, which write at the front and advance past what they wrote, or panic
/// past the room, directly, through a `&mut`, and through a `Box`: the room left, then
/// each array whole, so a write that missed it or landed off the front would show.
pub fn slices(data: &[u8], out: &mut Vec<u8>) {
	let [kind, room, data @ ..] = data else {
		return;
	};
	let room = usize::from(*room);
	let mut bytes = [0xa5; 255];
	let mut uninit = [MaybeUninit::new(0x5a); 255];
	let left = match kind % 8 {
		0 => {
			let mut s = &mut bytes[..room];
			write_all(&mut s, data);
			s.len()
		}
		1 => {
			let mut s = &mut bytes[..room];
			chunk_all(&mut s, data, out);
			s.len()
		}
		2 => {
			let mut s = &mut uninit[..room];
			write_all(&mut s, data);
			s.len()
		}
		3 => {
			let mut s = &mut uninit[..room];
			chunk_all(&mut s, data, out);
			s.len()
		}
		4 => {
			let mut s = &mut bytes[..room];
			write_all(&mut &mut s, data);
			s.len()
		}
		5 => {
			let mut s = &mut uninit[..room];
			chunk_all(&mut Box::new(&mut s), data, out);
			s.len()
		}
		6 => {
			let mut s = &mut bytes[..room];
			chunk_all(&mut Box::new(&mut s), data, out);
			s.len()
		}
		_ => {
			let mut s = &mut uninit[..room];
			write_all(&mut &mut s, data);
			s.len()
		}
	};
	out.push(left as u8);
	out.extend_from_slice(&bytes);
	// SAFETY: every slot starts initialized, and a write leaves it so.
	out.extend(uninit.iter().map(|b| unsafe { b.assume_init() }));
}

/// The same writes into a `Vec<u8>` and a `BytesMut`, which must match, and through the
/// impls forwarding through a `&mut` and a `Box`, whose provided `put` and `put_bytes`
/// grow by `chunk_mut` instead: the bytes, and each vector's capacity.
pub fn writes(data: &[u8], out: &mut Vec<u8>) {
	let [first, cap, data @ ..] = data else {
		return;
	};
	let cap = usize::from(*cap % 24);
	let (mut vec, mut buf) = (Vec::with_capacity(cap), BytesMut::with_capacity(cap));
	let (mut through_vec, mut through_buf) = (Vec::with_capacity(cap), BytesMut::with_capacity(cap));
	let (mut boxed_vec, mut boxed_buf) = (
		Box::new(Vec::with_capacity(cap)),
		Box::new(BytesMut::with_capacity(cap)),
	);
	// Each first in turn, so each one's panic is reached.
	for i in 0..6 {
		match first.wrapping_add(i) % 6 {
			0 => write_all(&mut vec, data),
			1 => write_all(&mut buf, data),
			2 => write_all(&mut &mut through_vec, data),
			3 => write_all(&mut &mut through_buf, data),
			4 => write_all(&mut boxed_vec, data),
			_ => write_all(&mut boxed_buf, data),
		}
	}
	// A buffer of the crate's own, in chunks as long as the room `chunk_mut` gives, and
	// through a `&mut` to it, which it reads past.
	let pieces = || Pieces::new(data, data.len() / 2);
	let mut lent = pieces();
	vec.put(&mut lent);
	vec.put(&mut lent);
	BufMut::put(&mut &mut through_vec, pieces());
	BufMut::put(&mut boxed_vec, &mut pieces());
	for vec in [&vec, &through_vec, &boxed_vec] {
		out.extend_from_slice(&(vec.capacity() as u32).to_le_bytes());
	}
	for buf in [&buf, &through_buf, &boxed_buf] {
		out.extend_from_slice(&(buf.capacity() as u32).to_le_bytes());
	}
	out.extend_from_slice(&through_vec);
	out.extend_from_slice(&through_buf);
	out.push((*boxed_vec == through_vec) as u8);
	out.push((*boxed_buf == through_buf) as u8);
	out.push((vec == buf) as u8);
	out.push((vec.remaining_mut() == buf.remaining_mut()) as u8);
	out.push(buf.has_remaining_mut() as u8);
	out.extend_from_slice(&vec);
	// Into a `BytesMut` of no capacity, each kind of buffer, which it copies out whole first.
	let half = &data[..data.len() / 2];
	for kind in 0..6 {
		let mut fresh = BytesMut::new();
		match kind {
			0 => fresh.put(half),
			1 => fresh.put(Bytes::copy_from_slice(half)),
			2 => fresh.put(BytesMut::from(half)),
			3 => fresh.put(Pieces::new(half, half.len() / 2)),
			4 => fresh.put(Provided(Pieces::new(half, half.len() / 2))),
			_ => fresh.put(Bytes::copy_from_slice(half).chain(half)),
		}
		out.extend_from_slice(&(fresh.capacity() as u32).to_le_bytes());
		fresh.put_u8(kind);
		out.extend_from_slice(&(fresh.capacity() as u32).to_le_bytes());
		out.extend_from_slice(&fresh);
	}
}

/// A write by each pair of `data`'s bytes, one past what a vector holds included.
fn write_all<B: BufMut>(buf: &mut B, data: &[u8]) {
	let mut data = data;
	while let [op, n, rest @ ..] = data {
		data = rest;
		let (op, n) = (*op, *n);
		let v = u64::from(n) * 0x0101_0101_0101_0101;
		match op % 23 {
			0 => buf.put_u8(n),
			1 => buf.put_i8(n as i8),
			2 => buf.put_u16(v as u16),
			3 => buf.put_i16_le(v as i16),
			4 => buf.put_u32(v as u32),
			5 => buf.put_i32_le(v as i32),
			6 => buf.put_u64_le(v),
			7 => buf.put_i64(v as i64),
			8 => buf.put_u128_le(u128::from(v) << 40),
			9 => buf.put_uint(v, n as usize % 10),
			10 => buf.put_int_le(v as i64, n as usize % 10),
			11 => buf.put_slice(&[op, n]),
			// Past what a vector holds, which panics as `Vec` does directly, as `bytes` does
			// of storage a `put` at zero capacity took, and as `remaining_mut` says through a
			// `&mut`.
			12 if n == 0xff => buf.put_bytes(n, usize::MAX),
			12 => buf.put_bytes(n, n as usize % 100),
			13 => buf.put(Bytes::from_static(b"put")),
			14 => buf.put(&[op; 70][..usize::from(n) % 71]),
			15 => buf.put(BytesMut::from(&[n; 3][..])),
			16 => buf.put_f32(f32::from(n) / 3.0),
			17 => buf.put_int(v as i64, n as usize % 10),
			18 => buf.put_uint_le(v, n as usize % 10),
			19 => buf.put_f64(f64::from(n) / 3.0),
			// The crate's own buffers, which a `BytesMut` of no capacity copies out whole.
			20 => buf.put(Pieces::new(&[op, n, op], usize::from(n) % 4)),
			21 => buf.put(Provided(Pieces::new(&[n; 5], usize::from(n) % 6))),
			_ => buf.put(Bytes::copy_from_slice(&[n; 2]).chain(&[op][..usize::from(n) % 2])),
		}
		buf.put_u8(buf.has_remaining_mut() as u8 + (buf.remaining_mut() % 7) as u8);
	}
}

/// `Bytes`' own edits, through a `&mut`, and `slice_ref` of a part of it, and whether each
/// part is the last handle of the storage: a promotable `Bytes` truncated to nothing gives
/// it up, none of its clones is promotable, and a `Bytes` of what it lends holds none.
pub fn edits(data: &[u8], out: &mut Vec<u8>) {
	let [kind, a, rest @ ..] = data else {
		return;
	};
	let mut bytes = Bytes::copy_from_slice(rest);
	let a = *a as usize % (rest.len() + 2);
	let part = match kind % 9 {
		0 => bytes.split_to(a),
		1 => bytes.split_off(a),
		2 => {
			bytes.truncate(a);
			bytes.clone()
		}
		3 => {
			bytes.clear();
			Bytes::new()
		}
		4 => {
			let all: &[u8] = bytes.as_ref();
			let sub = &all[a.min(rest.len())..];
			bytes.slice_ref(sub)
		}
		5 => {
			bytes.advance(a);
			bytes.copy_to_bytes(a / 2)
		}
		6 => {
			let mut clone = bytes.clone();
			clone.truncate(a);
			clone
		}
		7 => {
			let clone = bytes.clone();
			bytes.truncate(a);
			clone
		}
		// Bytes it lends, which count none of its storage.
		_ => {
			let owner: &'static Bytes = Box::leak(Box::new(bytes.clone()));
			drop(Bytes::from_static(owner));
			drop(Bytes::from(&owner[..]));
			Bytes::from_owner(owner)
		}
	};
	out.extend_from_slice(format!("{part:?} {bytes:?} {} {}", part.is_unique(), bytes.is_unique()).as_bytes());
}

/// An owner of bytes in an `Arc`, as moq's frames hold theirs, which logs its tag when it
/// drops and zeroes the bytes, so a copy made after it dropped would show.
struct Tagged {
	data: Arc<Vec<u8>>,
	tag: u8,
	log: Arc<Mutex<Vec<u8>>>,
}

impl AsRef<[u8]> for Tagged {
	fn as_ref(&self) -> &[u8] {
		&self.data
	}
}

impl Drop for Tagged {
	fn drop(&mut self) {
		self.log.lock().unwrap().push(self.tag);
		if let Some(data) = Arc::get_mut(&mut self.data) {
			data.fill(0);
		}
	}
}

/// Handles of three owners' `Bytes` cloned, sliced, split, truncated, taken back, put, and
/// dropped, each op logged after it: an owner drops with the last handle of its bytes, in
/// the order the handles drop, and a handle of one is never unique, so taking it back copies.
/// Two are `Tagged`, and one is an `Arc<[u8]>` itself, whose strong count shows when it drops.
pub fn owners(data: &[u8], out: &mut Vec<u8>) {
	let [cut, ops @ ..] = data else {
		return;
	};
	let (ops, input) = ops.split_at(ops.len() / 2);
	let log = Arc::new(Mutex::new(Vec::new()));
	let owner = |tag, data: &[u8]| {
		Bytes::from_owner(Tagged {
			data: Arc::new(data.to_vec()),
			tag,
			log: log.clone(),
		})
	};
	let (a, b) = input.split_at(*cut as usize % (input.len() + 1));
	let shared: Arc<[u8]> = Arc::from(input);
	let mut handles = vec![owner(0xf0, a), owner(0xf1, b), Bytes::from_owner(shared.clone())];
	for &op in ops {
		let i = usize::from(op >> 4) % handles.len().max(1);
		if handles.is_empty() {
			handles.push(owner(0xf2, input));
		}
		let at = usize::from(op >> 4) % (handles[i].len() + 1);
		let noted = match op % 16 {
			0 => {
				let clone = handles[i].clone();
				handles.push(clone);
				vec![]
			}
			1 => {
				let part = handles[i].slice(at..);
				handles.push(part);
				vec![]
			}
			2 => {
				let part = handles[i].split_to(at);
				handles.push(part);
				vec![]
			}
			3 => {
				let part = handles[i].split_off(at);
				handles.push(part);
				vec![]
			}
			4 => {
				handles[i].truncate(at);
				vec![]
			}
			5 => {
				handles[i].clear();
				vec![]
			}
			6 => vec![handles[i].is_unique() as u8],
			7 => match handles.remove(i).try_into_mut() {
				Ok(taken) => {
					handles.push(taken.freeze());
					vec![1]
				}
				Err(back) => {
					handles.insert(i, back);
					vec![0]
				}
			},
			8 => Vec::from(handles.remove(i)),
			9 => BytesMut::from(handles.remove(i)).to_vec(),
			10 => {
				drop(handles.remove(i));
				vec![]
			}
			11 => {
				handles[i].advance(at);
				vec![]
			}
			12 => {
				let part = handles[i].copy_to_bytes(at);
				handles.push(part);
				vec![]
			}
			13 => {
				let handle = &handles[i];
				let part = handle.clone().slice_ref(&handle[at..]);
				handles.push(part);
				vec![]
			}
			14 => {
				let mut buf = BytesMut::new();
				buf.put(handles.remove(i));
				buf.to_vec()
			}
			_ => {
				handles[i] = Bytes::new();
				vec![]
			}
		};
		let mut log = log.lock().unwrap();
		log.push(op);
		log.push(Arc::strong_count(&shared) as u8);
		log.extend_from_slice(&noted);
	}
	for handle in &handles {
		out.extend_from_slice(format!("{handle:?}").as_bytes());
	}
	drop(handles);
	out.extend_from_slice(&log.lock().unwrap());
	// No owner outlives its handles: only these two counts are left.
	out.push(Arc::strong_count(&shared) as u8);
	out.push(Arc::strong_count(&log) as u8);
}

/// `Chain`s of buffers of each kind, nested, and as and of `dyn Buf`s: `copy_to_bytes` of a
/// length picked about where the first half ends, which takes from either half or copies
/// across both, then the reads of `reads` across them.
pub fn chains(data: &[u8], out: &mut Vec<u8>) {
	let [kind, cut, pick, ops @ ..] = data else {
		return;
	};
	let (ops, input) = ops.split_at(ops.len() / 2);
	let cut = *cut as usize % (input.len() + 1);
	let (a, b) = input.split_at(cut);
	let (b1, b2) = b.split_at(b.len() / 2);
	let bytes = Bytes::copy_from_slice;
	// Where each chain's halves end, the inner one's of a nested chain too.
	let at = [a.len(), a.len() + b1.len()];
	let rest = match kind % 9 {
		0 => chained(at, &mut bytes(a).chain(b), pick, ops, out),
		1 => chained(at, &mut a.chain(BytesMut::from(b)), pick, ops, out),
		2 => chained(
			at,
			&mut Pieces::new(a, a.len() / 2).chain(Provided(Pieces::new(b, 1))),
			pick,
			ops,
			out,
		),
		3 => chained(at, &mut Provided(Pieces::new(a, 1)).chain(bytes(b)), pick, ops, out),
		4 => chained(at, &mut bytes(input).take(cut).chain(Pieces::new(b, 0)), pick, ops, out),
		5 => chained(at, &mut a.chain(bytes(b1)).chain(Pieces::new(b2, 1)), pick, ops, out),
		6 => chained(
			at,
			&mut bytes(a).chain(Provided(Pieces::new(b1, 0)).chain(b2)),
			pick,
			ops,
			out,
		),
		7 => {
			let first: Box<dyn Buf> = Box::new(Pieces::new(a, 1));
			let mut chain: Box<dyn Buf> = Box::new(first.chain(bytes(b1)).chain(b2));
			chained(at, &mut *chain, pick, ops, out)
		}
		// A half lent through a `&mut`, which reads past what it lends.
		_ => {
			let mut last = b;
			let mut chain = Provided(Pieces::new(a, 0)).chain(&mut last);
			let buf: &mut dyn Buf = &mut chain;
			let rest = chained(at, buf, pick, ops, out);
			out.push(last.len() as u8);
			rest
		}
	};
	out.extend_from_slice(&rest);
}

/// `copy_to_bytes` of a length by `pick`: where a half ends by `at`, one more or less, all
/// of it, one past it, which panics, or any, then `read_all` of the rest.
fn chained<B: Buf + ?Sized>(at: [usize; 2], buf: &mut B, pick: &u8, ops: &[u8], out: &mut Vec<u8>) -> Vec<u8> {
	let all = buf.remaining();
	let len = match pick % 8 {
		0 => at[0],
		1 => at[0] + 1,
		2 => at[0].saturating_sub(1),
		3 => at[1],
		4 => at[1] + 1,
		5 => all,
		6 => all + 1,
		_ => usize::from(pick / 8) % (all + 2),
	};
	out.extend_from_slice(&buf.copy_to_bytes(len));
	read_all(buf, ops, out)
}

/// `chunks_vectored` of each buffer into as many slots as its input picks: a `Bytes`, a
/// `BytesMut`, a `&[u8]`, buffers in pieces, a `VecDeque` that wraps, a `Take` and a
/// `Chain` of them, and through a `&mut` and a `dyn Buf`. Then the slots it filled, read,
/// printed, and advanced, one and all together, which panics past what they hold.
pub fn vectored(data: &[u8], out: &mut Vec<u8>) {
	let [kind, cut, slots, skip, input @ ..] = data else {
		return;
	};
	let cut = *cut as usize % (input.len() + 1);
	let bytes = Bytes::copy_from_slice(input);
	let owned = BytesMut::from(input);
	let pieces = Pieces::new(input, cut);
	let provided = Provided(Pieces::new(input, cut));
	let mut deque: VecDeque<u8> = input.iter().copied().collect();
	deque.rotate_left(cut);
	let take = Pieces::new(input, cut).take(usize::from(*skip) % (input.len() + 2));
	let (a, b) = input.split_at(cut);
	let chain = Pieces::new(a, a.len() / 2)
		.chain(Bytes::copy_from_slice(b))
		.chain(&b[..]);
	let mut slice = input;
	let lent = &mut slice;
	let boxed: Box<dyn Buf> = Box::new(Pieces::new(input, cut));
	let mut dst = [IoSlice::new(&[]); 5];
	let dst = &mut dst[..usize::from(*slots) % 6];
	let n = match kind % 11 {
		0 => bytes.chunks_vectored(dst),
		1 => owned.chunks_vectored(dst),
		2 => input.chunks_vectored(dst),
		3 => pieces.chunks_vectored(dst),
		4 => provided.chunks_vectored(dst),
		5 => (&chain as &dyn Buf).chunks_vectored(dst),
		6 => take.chunks_vectored(dst),
		7 => chain.chunks_vectored(dst),
		8 => lent.chunks_vectored(dst),
		9 => deque.chunks_vectored(dst),
		_ => boxed.chunks_vectored(dst),
	};
	out.push(n as u8);
	for chunk in &dst[..n] {
		out.push(chunk.len() as u8);
		out.extend_from_slice(chunk);
	}
	out.extend_from_slice(format!("{:?}", &dst[..n]).as_bytes());
	let total: usize = dst[..n].iter().map(|chunk| chunk.len()).sum();
	let skip = usize::from(*skip) % (total + 2);
	if let Some(first) = dst.first_mut() {
		let mut one = *first;
		one.advance(skip.min(one.len() + usize::from(kind / 11 % 2)));
		out.extend_from_slice(&one);
	}
	let mut left = &mut dst[..n];
	IoSlice::advance_slices(&mut left, skip);
	out.push(left.len() as u8);
	for chunk in left.iter() {
		out.extend_from_slice(chunk);
	}
}

/// The bytes of a case's input, taken from the front.
/// A `Bytes` of a `String`, which takes its storage, of the room its first bytes pick past
/// its text: whether it is the one handle of it, and the room a `BytesMut` or a `Vec` taken
/// back of it holds, with a clone of it held or not.
pub fn strings(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut s = String::with_capacity(ops.byte() as usize % 24);
	let n = ops.byte() as usize % 12;
	s.push_str(&String::from_utf8_lossy(ops.take(n)));
	s.reserve(ops.byte() as usize % 7);
	let bytes = Bytes::from(s);
	out.push(bytes.is_unique() as u8);
	let held = (ops.byte() % 2 == 1).then(|| bytes.clone());
	let (len, cap) = match ops.byte() % 3 {
		0 => {
			let m = BytesMut::from(bytes);
			(m.len(), m.capacity())
		}
		1 => match bytes.try_into_mut() {
			Ok(m) => (m.len(), m.capacity()),
			Err(_) => (usize::MAX, 0),
		},
		_ => {
			let v: Vec<u8> = bytes.into();
			(v.len(), v.capacity())
		}
	};
	out.extend_from_slice(&(len as u32).to_le_bytes());
	out.extend_from_slice(&(cap as u32).to_le_bytes());
	drop(held);
}

struct Ops<'a>(&'a [u8]);

impl<'a> Ops<'a> {
	fn next(&mut self) -> Option<u8> {
		let (&first, rest) = self.0.split_first()?;
		self.0 = rest;
		Some(first)
	}

	fn byte(&mut self) -> u8 {
		self.next().unwrap_or(0)
	}

	fn take(&mut self, n: usize) -> &'a [u8] {
		let (taken, rest) = self.0.split_at(n.min(self.0.len()));
		self.0 = rest;
		taken
	}
}
