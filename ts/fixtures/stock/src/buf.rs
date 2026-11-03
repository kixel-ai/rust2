//! `bytes`' `BytesMut`, `Buf`, and `BufMut`: each case runs ops its input picks, so the
//! differential test reaches their panics and what `split_to` and `freeze` share.

use std::cell::Cell;
use std::rc::Rc;

use bytes::{Buf, BufMut, Bytes, BytesMut};

/// A `BytesMut` split, frozen, grown, and written, ending with every piece's bytes: a
/// write to one that reached another would show.
pub fn bytes_mut(data: &[u8], out: &mut Vec<u8>) {
	let mut ops = Ops(data);
	let mut buf = BytesMut::with_capacity(ops.byte() as usize % 24);
	let mut pieces: Vec<BytesMut> = Vec::new();
	let mut frozen: Vec<Bytes> = Vec::new();
	while let Some(op) = ops.next() {
		let n = ops.byte() as usize;
		match op % 18 {
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
			// past: each panics as `Vec` does. Storage that was shared throws there instead,
			// so these reserve into a vector of their own.
			7 if n % 2 == 0 => {
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
			_ => {
				let mut source = BytesMut::from(ops.take(n % 6));
				// Its own `Buf`, read into a `BufMut`, then the rest whole.
				let first = source.get_u8_checked();
				buf.put_u8(first);
				buf.put(source);
			}
		}
	}
	let tail = BytesMut::from(&b"tail"[..]);
	out.push((buf == tail) as u8);
	out.push((buf.clone() == buf[..]) as u8);
	out.extend_from_slice(format!("{buf:?} {}\n", buf.len()).as_bytes());
	for piece in &pieces {
		out.extend_from_slice(format!("{piece:?}\n").as_bytes());
	}
	for bytes in &frozen {
		out.extend_from_slice(format!("{bytes:?}\n").as_bytes());
	}
	let vec: Vec<u8> = buf.into();
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

/// `reserve` at the edges of `rt.BytesMut`'s limit for storage that was shared, 512 MiB
/// counting what it read past: a vector of its own grows as `Vec` does on either side of
/// it, and shared storage grows up to it. Only the inputs `caller.ts` names reach these,
/// as each allocates that much.
pub fn reserves(data: &[u8], out: &mut Vec<u8>) {
	const LIMIT: usize = 1 << 29;
	let &[b'e', b'd', b'g', b'e', kind, read] = data else {
		return;
	};
	let mut buf = BytesMut::from(&b"edges"[..]);
	let read = usize::from(read) % 5;
	buf.advance(read);
	let head = (kind % 2 == 1).then(|| buf.split_to(0));
	let past = usize::from(kind / 2 % 2);
	buf.reserve(LIMIT + past - read - buf.len());
	buf.put_slice(b"!");
	out.extend_from_slice(&buf);
	out.push(head.is_some() as u8);
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

/// Pieces whose `copy_to_bytes` gives zeros, so what `put` reads it by shows.
struct Zeroes(Pieces);

impl Buf for Zeroes {
	fn remaining(&self) -> usize {
		self.0.remaining()
	}

	fn chunk(&self) -> &[u8] {
		self.0.chunk()
	}

	fn advance(&mut self, cnt: usize) {
		self.0.advance(cnt);
	}

	fn copy_to_bytes(&mut self, len: usize) -> Bytes {
		self.0.advance(len);
		Bytes::from(vec![0; len])
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

/// A buffer of the crate's own over `Buf`, as moq's transports lend a stream's chunks.
trait Leased: Buf {
	fn pieces(&self) -> usize;
}

impl Leased for Pieces {
	fn pieces(&self) -> usize {
		self.parts.len()
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
/// own trait over `Buf`. A box drops what it owns once.
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
	let rest = read_all(&mut *leased, ops, out);
	out.extend_from_slice(&rest);
	out.push(leased.pieces() as u8);
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
			0 => write_all(&mut vec, data, cap > 0),
			1 => write_all(&mut buf, data, cap > 0),
			2 => write_all(&mut &mut through_vec, data, cap > 0),
			3 => write_all(&mut &mut through_buf, data, cap > 0),
			4 => write_all(&mut boxed_vec, data, cap > 0),
			_ => write_all(&mut boxed_buf, data, cap > 0),
		}
	}
	// A buffer of the crate's own, chunk by chunk, through a `&mut` or a `Box` in chunks as
	// long as the room `chunk_mut` gives.
	let pieces = || Pieces::new(data, data.len() / 2);
	vec.put(pieces());
	buf.put(pieces());
	BufMut::put(&mut &mut through_vec, pieces());
	BufMut::put(&mut &mut through_buf, pieces());
	BufMut::put(&mut boxed_vec, pieces());
	BufMut::put(&mut boxed_buf, pieces());
	// Into no capacity, `bytes` takes the storage of what `copy_to_bytes` makes, through the
	// `&mut` it reads past, and into storage that holds any, which this shares, reads chunk
	// by chunk.
	let mut zeroes = Zeroes(pieces());
	let mut taken = BytesMut::new();
	taken.put(&mut zeroes);
	taken.put(&mut zeroes);
	taken.put(Zeroes(pieces()));
	out.extend_from_slice(&taken);
	for vec in [&vec, &through_vec, &boxed_vec] {
		out.extend_from_slice(&(vec.capacity() as u32).to_le_bytes());
	}
	out.extend_from_slice(&through_vec);
	out.extend_from_slice(&through_buf);
	out.push((*boxed_vec == through_vec) as u8);
	out.push((*boxed_buf == through_buf) as u8);
	out.push((vec == buf) as u8);
	out.push((vec.remaining_mut() == buf.remaining_mut()) as u8);
	out.push(buf.has_remaining_mut() as u8);
	out.extend_from_slice(&vec);
}

/// A write by each pair of `data`'s bytes, and with `overflow`, one past what a vector holds.
fn write_all<B: BufMut>(buf: &mut B, data: &[u8], overflow: bool) {
	let mut data = data;
	while let [op, n, rest @ ..] = data {
		data = rest;
		let (op, n) = (*op, *n);
		let v = u64::from(n) * 0x0101_0101_0101_0101;
		match op % 20 {
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
			// Past what a vector holds, which panics as `Vec` does directly, and as
			// `remaining_mut` says through a `&mut`. Not from zero capacity, where a `put`
			// of a `Bytes` takes its storage, which `rt.BytesMut` refuses to reserve past.
			12 if n == 0xff && overflow => buf.put_bytes(n, usize::MAX),
			12 => buf.put_bytes(n, n as usize % 100),
			13 => buf.put(Bytes::from_static(b"put")),
			14 => buf.put(&[op; 70][..usize::from(n) % 71]),
			15 => buf.put(BytesMut::from(&[n; 3][..])),
			16 => buf.put_f32(f32::from(n) / 3.0),
			17 => buf.put_int(v as i64, n as usize % 10),
			18 => buf.put_uint_le(v, n as usize % 10),
			_ => buf.put_f64(f64::from(n) / 3.0),
		}
		buf.put_u8(buf.has_remaining_mut() as u8 + (buf.remaining_mut() % 7) as u8);
	}
}

/// `Bytes`' own edits, through a `&mut`, and `slice_ref` of a part of it.
pub fn edits(data: &[u8], out: &mut Vec<u8>) {
	let [kind, a, rest @ ..] = data else {
		return;
	};
	let mut bytes = Bytes::copy_from_slice(rest);
	let a = *a as usize % (rest.len() + 2);
	let part = match kind % 6 {
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
			let sub = &bytes[a.min(rest.len())..];
			bytes.slice_ref(sub)
		}
		_ => {
			bytes.advance(a);
			bytes.copy_to_bytes(a / 2)
		}
	};
	out.extend_from_slice(format!("{part:?} {bytes:?}").as_bytes());
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

/// The bytes of a case's input, taken from the front.
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
