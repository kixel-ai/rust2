//! The workloads `just ts bench` times on every route, shaped like moq's own TypeScript
//! benchmarks: `js/net/bench/varint.ts` and `lite-varint.ts`. The first input byte picks
//! the workload and the second its wire format; an encode's output is the matching
//! decode's input. Op 6 runs iterator chains over its bytes instead, as moq-net's model
//! writes them, op 7 decodes frame headers as they arrive a few bytes at a time, op 8
//! keeps a `BTreeMap`, as moq-net's model keeps its state, op 9 converts timestamps
//! between timescales, ops 10 and 11 add and order them as moq-net's `Timestamp` does,
//! op 12 orders integers by std's own `Ord`, op 13 pushes onto vectors, op 14 keeps a
//! `HashMap`, op 15 numbers TS packets as moq-mux does, op 16 formats log lines, op 17
//! writes through the mutable parts of a slice of integers, op 18 splits frames' `Bytes`
//! off a group's, ops 19 and 20 code strings as moq-net codes a path, op 21 joins and
//! strips paths, and op 22 keeps a `HashMap` keyed by `String`.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, VecDeque};
use std::fmt::Write;

use bytes::Bytes;

use crate::coding::{Decode, DecodeError, Decoder, Encode, EncodeError, Encoder, Form, varint};
use crate::harness::{decode_error, encode_error, form, lite_version, put_u64, put_usize, take_u64};
use crate::lite;

/// Run one workload.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	let Some((&op, data)) = input.split_first() else {
		return out;
	};
	let Some((&selector, data)) = data.split_first() else {
		return out;
	};

	match op {
		0 => encoded(encode_varints(data, form(selector), &mut out), &mut out),
		1 => decoded(decode_varints(data, form(selector), &mut out), &mut out),
		2 => encoded(encode_frames(data, lite_version(selector), &mut out), &mut out),
		3 => decoded(decode_frames(data, lite_version(selector), &mut out), &mut out),
		4 => encoded(encode_group(data, lite_version(selector), &mut out), &mut out),
		6 => iterate(selector, data, &mut out),
		7 => decoded(decode_split(data, lite_version(selector), &mut out), &mut out),
		8 => ordered(data, &mut out),
		9 => rescale(selector, data, &mut out),
		10 => stamp(data, &mut out),
		11 => interleave(data, &mut out),
		12 => order(selector, data, &mut out),
		13 => push(selector, data, &mut out),
		14 => hashed(selector, data, &mut out),
		15 => packets(selector, data, &mut out),
		16 => format(data, &mut out),
		17 => parts(selector, data, &mut out),
		18 => frames(data, &mut out),
		19 => encoded(encode_strings(selector, data, &mut out), &mut out),
		20 => decoded(decode_strings(data, &mut out), &mut out),
		21 => join(selector, data, &mut out),
		22 => keyed(selector, data, &mut out),
		_ => decoded(decode_group(data, lite_version(selector), &mut out), &mut out),
	}
	out
}

fn encoded(result: Result<(), EncodeError>, out: &mut Vec<u8>) {
	if let Err(err) = result {
		encode_error(out, &err);
	}
}

fn decoded(result: Result<(), DecodeError>, out: &mut Vec<u8>) {
	if let Err(err) = result {
		decode_error(out, &err);
	}
}

/// The same value, `count` times.
fn encode_varints(data: &[u8], form: Form, out: &mut Vec<u8>) -> Result<(), EncodeError> {
	let (count, data) = take_u64(data);
	let (value, _) = take_u64(data);
	let mut w = Encoder::new(out, form);
	for _ in 0..count as usize {
		w.varint(value)?;
	}
	Ok(())
}

/// Every varint on the wire: how many, and the last.
fn decode_varints(wire: &[u8], form: Form, out: &mut Vec<u8>) -> Result<(), DecodeError> {
	let mut r = Decoder::new(wire, form);
	let mut count = 0;
	let mut last = 0;
	while !r.is_empty() {
		last = r.varint()?;
		count += 1;
	}
	put_usize(out, count);
	put_u64(out, last);
	Ok(())
}

/// A group's FRAME headers without their payloads, as a lite publisher writes them: the
/// zigzag timestamp delta, then the size. The first frame has no delta and its own size,
/// and every `key_every`th frame after it is `key_size`, as moq's benchmark samples are.
fn encode_frames(data: &[u8], version: lite::Version, out: &mut Vec<u8>) -> Result<(), EncodeError> {
	let (count, data) = take_u64(data);
	let (delta, data) = take_u64(data);
	let (first, data) = take_u64(data);
	let (size, data) = take_u64(data);
	let (key_every, data) = take_u64(data);
	let (key_size, _) = take_u64(data);

	let (delta, key_every) = (delta as i64, key_every as usize);

	let mut w = Encoder::new(out, version.into());
	for n in 0..count as usize {
		let (delta, size) = match n {
			0 => (0, first),
			n if key_every != 0 && n % key_every == 0 => (delta, key_size),
			_ => (delta, size),
		};
		w.varint(varint::zigzag(delta))?;
		w.varint(size)?;
	}
	Ok(())
}

/// Every FRAME header on the wire: how many, their total size, and the last delta.
fn decode_frames(wire: &[u8], version: lite::Version, out: &mut Vec<u8>) -> Result<(), DecodeError> {
	let mut r = Decoder::new(wire, version.into());
	let mut count = 0;
	let mut total = 0;
	let mut last = 0;
	while !r.is_empty() {
		last = varint::unzigzag(r.varint()?);
		total += r.varint()?;
		count += 1;
	}
	put_usize(out, count);
	put_u64(out, total);
	put_u64(out, last as u64);
	Ok(())
}

/// The FRAME headers of [`decode_frames`] as a subscriber reads them: the wire after the
/// read size arrives that many bytes at a time, and each read decodes every header it
/// completes. A header cut by the end of a read comes up [`DecodeError::Short`] and is
/// decoded again from its start on the next. How many, their total size, the last delta,
/// and how many decodes came up short.
fn decode_split(data: &[u8], version: lite::Version, out: &mut Vec<u8>) -> Result<(), DecodeError> {
	let (read, wire) = take_u64(data);
	let read = (read as usize).max(1);
	let mut start = 0;
	let mut end = 0;
	let mut count = 0;
	let mut total = 0;
	let mut last = 0;
	let mut short = 0;
	while end < wire.len() {
		end = wire.len().min(end + read);
		while start < end {
			let mut r = Decoder::new(&wire[start..end], version.into());
			match frame_header(&mut r) {
				Ok((delta, size)) => {
					start = end - r.remaining();
					last = delta;
					total += size;
					count += 1;
				}
				Err(DecodeError::Short) => {
					short += 1;
					break;
				}
				Err(err) => return Err(err),
			}
		}
	}
	// The wire ended mid-header.
	if start < end {
		return Err(DecodeError::Short);
	}
	put_usize(out, count);
	put_u64(out, total);
	put_u64(out, last as u64);
	put_usize(out, short);
	Ok(())
}

/// One FRAME header: the zigzag timestamp delta, then the size.
fn frame_header(r: &mut Decoder<'_>) -> Result<(i64, u64), DecodeError> {
	let delta = varint::unzigzag(r.varint()?);
	Ok((delta, r.varint()?))
}

/// One size-prefixed GROUP header, starting at the first frame.
fn encode_group(data: &[u8], version: lite::Version, out: &mut Vec<u8>) -> Result<(), EncodeError> {
	let (subscribe, data) = take_u64(data);
	let (sequence, _) = take_u64(data);
	let group = lite::Group {
		subscribe,
		sequence,
		frame_start: 0,
	};
	group.encode(&mut Encoder::new(out, version.into()), version)
}

/// A `BTreeMap` of `count` keys, at most 2^16: each inserted in a scattered order, looked
/// up, summed over a quarter of the key space, and removed again. The sum of the values
/// found.
fn ordered(data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.min(1 << 16) as u32;
	// An odd multiplier scatters the keys below 2^16 without repeating one.
	let key = |i: u32| (i * 40503) % (1 << 16);
	let mut map = BTreeMap::new();
	for i in 0..count {
		map.insert(key(i), i);
	}
	let mut sum = 0u64;
	for i in 0..count {
		sum += map.get(&key(count - 1 - i)).copied().unwrap_or(0) as u64;
	}
	for (_, &v) in map.range(1 << 14..2 << 14) {
		sum += v as u64;
	}
	for i in 0..count {
		map.remove(&key(i));
	}
	put_u64(out, sum);
}

/// `count` keys, scattered as `ordered`'s, inserted into a `HashMap`, and unless `selector`
/// is 1, each looked up and removed: the sum of what the lookups found, and the length left.
fn hashed(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.min(1 << 16) as u32;
	let key = |i: u32| (i * 40503) % (1 << 16);
	let mut map = HashMap::new();
	for i in 0..count {
		map.insert(key(i), i);
	}
	let mut sum = 0u64;
	if selector != 1 {
		for i in 0..count {
			sum += map.get(&key(count - 1 - i)).copied().unwrap_or(0) as u64;
		}
		for i in 0..count {
			map.remove(&key(i));
		}
	}
	put_u64(out, sum);
	put_usize(out, map.len());
}

/// `count` integers sorted, sorted by a key, searched for the greatest and least, pushed
/// onto a `BinaryHeap` and popped off, or collected into a `BTreeMap` or `BTreeSet`, as
/// `selector` picks, each by `u32`'s own `Ord`: the first, middle, and last of the order,
/// the greatest and least, or the first key, the length, and the last key or its value.
fn order(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.clamp(1, 1 << 16) as u32;
	let mut v: Vec<u32> = (0..count).map(|i| (i * 40503) % (1 << 16)).collect();
	match selector {
		0 => v.sort(),
		1 => v.sort_by_key(|&x| x % 1024),
		2 => v = vec![*v.iter().max().unwrap(), *v.iter().min().unwrap()],
		3 => {
			let (hi, lo) = v.iter().fold((0, u32::MAX), |(hi, lo), &x| (hi.max(x), lo.min(x)));
			v = vec![hi, lo];
		}
		5 => {
			let map: BTreeMap<u32, u32> = v.iter().map(|&x| (x % 512, x)).collect();
			v = vec![
				*map.keys().next().unwrap(),
				map.len() as u32,
				*map.values().next_back().unwrap(),
			];
		}
		6 => {
			let set: BTreeSet<u32> = v.iter().map(|&x| x % 512).collect();
			v = vec![*set.first().unwrap(), set.len() as u32, *set.last().unwrap()];
		}
		_ => {
			let mut heap = BinaryHeap::new();
			for x in std::mem::take(&mut v) {
				heap.push(x);
			}
			while let Some(x) = heap.pop() {
				v.push(x);
			}
		}
	}
	for x in [v[0], v[v.len() / 2], v[v.len() - 1]] {
		put_u64(out, x as u64);
	}
}

/// `count` integers pushed onto a `Vec`, or a `VecDeque` if `selector` is 1: its length
/// and middle item. With `selector` 2, a `Vec` as a stack, popped after every fourth push
/// and then emptied: its length and the sum of what it popped.
fn push(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.min(1 << 16) as u32;
	let (len, middle) = match selector {
		1 => {
			let mut q = VecDeque::new();
			for i in 0..count {
				q.push_back(i);
			}
			(q.len(), q.get(q.len() / 2).copied())
		}
		2 => {
			let mut v = Vec::new();
			let mut sum = 0u32;
			for i in 0..count {
				v.push(i);
				if i % 4 == 3 {
					sum = sum.wrapping_add(v.pop().unwrap_or(0));
				}
			}
			while let Some(x) = v.pop() {
				sum = sum.wrapping_add(x);
			}
			(v.len(), Some(sum))
		}
		_ => {
			let mut v = Vec::new();
			for i in 0..count {
				v.push(i);
			}
			(v.len(), v.get(v.len() / 2).copied())
		}
	};
	put_usize(out, len);
	put_u64(out, middle.map_or(0, u64::from));
}

/// `count` lines written into one `String` with padded, `Debug`, and fixed-precision
/// placeholders: its length and the sum of its bytes.
fn format(data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.min(1 << 16) as i64;
	let mut s = String::new();
	for i in 0..count {
		let point = Point { x: i, y: -i };
		let _ = writeln!(s, "{i:>6}|{point:?}|{:.2}|{}", i as f64 / 3.0, -i);
	}
	put_usize(out, s.len());
	put_u64(out, s.bytes().map(u64::from).sum());
}

/// Broadcast paths as moq names them: one ASCII, and one with chars of two, three, and four
/// bytes.
const NAMES: [&str; 2] = ["demo/room-42/alice/camera", "démo/salle-42/アリス/📷"];

/// The tracks `join` appends to a broadcast's path.
const TRACKS: [&str; 4] = ["video", "audio", "catalog.json", "字幕"];

/// The `selector`th name of [`NAMES`], `count` times, each size-prefixed, as moq-net's
/// `Encoder::string` writes a path.
fn encode_strings(selector: u8, data: &[u8], out: &mut Vec<u8>) -> Result<(), EncodeError> {
	let (count, _) = take_u64(data);
	let name = NAMES[selector as usize % NAMES.len()];
	let mut w = Encoder::new(out, Form::Quic);
	for _ in 0..count as usize {
		w.string(name)?;
	}
	Ok(())
}

/// Every string on the wire, each an owned `String`, as moq-net decodes a path: how many,
/// their total length, and the last one's.
fn decode_strings(wire: &[u8], out: &mut Vec<u8>) -> Result<(), DecodeError> {
	let mut r = Decoder::new(wire, Form::Quic);
	let mut count = 0;
	let mut total = 0;
	let mut last = String::new();
	while !r.is_empty() {
		last = r.string()?;
		total += last.len();
		count += 1;
	}
	put_usize(out, count);
	put_usize(out, total);
	put_usize(out, last.len());
	Ok(())
}

/// `count` paths, each the `selector`th name of [`NAMES`] and a track joined into a
/// `String` sized up front, as moq-net's `Path::concat` builds one, then stripped of the
/// name at a segment boundary, as `Path::strip_prefix` is: their total length, and the
/// stripped tracks'.
fn join(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let name = NAMES[selector as usize % NAMES.len()];
	let mut total = 0;
	let mut stripped = 0;
	for i in 0..count.min(1 << 16) as usize {
		let track = TRACKS[i % TRACKS.len()];
		let mut path = String::with_capacity(name.len() + 1 + track.len());
		path.push_str(name);
		path.push('/');
		path.push_str(track);
		total += path.len();
		if let Some(rest) = path.strip_prefix(name)
			&& let Some(rest) = rest.strip_prefix('/')
		{
			stripped += rest.len();
		}
	}
	put_usize(out, total);
	put_usize(out, stripped);
}

/// `count` paths, the `selector`th name of [`NAMES`] and a number, each a key of a
/// `HashMap` and looked up by its `&str` four times, as moq-net finds a broadcast by its
/// path: the sum of what the lookups found.
fn keyed(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let name = NAMES[selector as usize % NAMES.len()];
	let keys: Vec<String> = (0..count.min(1 << 16) as u32).map(|i| format!("{name}/{i}")).collect();
	let mut map = HashMap::new();
	for (i, key) in keys.iter().enumerate() {
		map.insert(key.clone(), i as u64);
	}
	let mut sum = 0u64;
	for _ in 0..4 {
		for key in &keys {
			sum += map.get(key.as_str()).copied().unwrap_or(0);
		}
	}
	put_u64(out, sum);
	put_usize(out, map.len());
}

/// A pair `format` writes with its derived `Debug`.
#[derive(Debug)]
struct Point {
	x: i64,
	y: i64,
}

/// `count` TS packets of 188 bytes, each written by element through `as_chunks_mut`, then
/// numbered by continuity counter as moq-mux's `container/ts/export.rs` numbers them; with
/// `selector` 1, each packet is read by value first, a copy of its bytes. The sum of the
/// counters and of each copy's last byte.
fn packets(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, _) = take_u64(data);
	let count = count.min(1 << 12) as usize;
	let mut packets = vec![0u8; count * 188];
	for (i, packet) in packets.as_chunks_mut::<188>().0.iter_mut().enumerate() {
		packet[0] = 0x47;
		packet[2] = (i % 3) as u8;
		packet[3] = if i % 5 == 0 { 0x20 } else { 0x10 };
		packet[187] = i as u8;
	}
	let mut counters = [0u8; 3];
	let mut sum = 0u64;
	for packet in packets.as_chunks_mut::<188>().0 {
		if selector == 1 {
			let copy = *packet;
			sum += u64::from(copy[187]);
		}
		let pid = usize::from(packet[1] & 0x1f) << 8 | usize::from(packet[2]);
		let next = &mut counters[pid % 3];
		let cc = match packet[3] & 0x10 != 0 {
			true => std::mem::replace(next, (*next + 1) & 0x0f),
			false => next.wrapping_sub(1) & 0x0f,
		};
		packet[3] = (packet[3] & 0xf0) | cc;
		sum += u64::from(cc);
	}
	put_u64(out, sum);
}

/// `count` steps through mutable parts of a `Vec` of `size` integers, as `selector` picks:
/// a min-heap of deadlines, as a timer wheel keeps them, whose root is pushed later and
/// sifted down through the halves `split_at_mut` lends, or `parts[..k]` written, `k`
/// cycling from 1 to `size`. Its first item and the sum of what each step read.
fn parts(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, data) = take_u64(data);
	let (size, _) = take_u64(data);
	let size = size.clamp(1, 1 << 16) as usize;
	let mut v: Vec<u64> = (0..size as u64).map(|i| i * 40503 % 65536).collect();
	let mut sum = 0u64;
	match selector {
		0 => {
			v.sort();
			for step in 0..count {
				v[0] += 1 + step % 64 * 1024;
				let mut i = 0;
				loop {
					let l = 2 * i + 1;
					if l >= size {
						break;
					}
					let c = if l + 1 < size && v[l + 1] < v[l] { l + 1 } else { l };
					let (parent, rest) = v.split_at_mut(c);
					if parent[i] <= rest[0] {
						break;
					}
					std::mem::swap(&mut parent[i], &mut rest[0]);
					i = c;
				}
				sum = sum.wrapping_add(v[0]);
			}
		}
		_ => {
			for step in 0..count {
				let part = &mut v[..step as usize % size + 1];
				for x in part.iter_mut() {
					*x = x.wrapping_add(1);
				}
				sum = sum.wrapping_add(part[part.len() - 1]);
			}
		}
	}
	put_u64(out, v[0]);
	put_u64(out, sum);
}

/// `count` frames of `size` bytes split off the front of a group's `Bytes`, as moq-net's
/// model hands a frame's bytes to its consumers: each cloned for a second consumer, which
/// reads past its first byte, then dropped, leaving the group the one handle of its storage
/// again. The sum of what each frame read, then how many times the group was unique.
fn frames(data: &[u8], out: &mut Vec<u8>) {
	let (count, data) = take_u64(data);
	let (size, _) = take_u64(data);
	let count = count.min(1 << 16) as usize;
	let size = size.clamp(1, 1 << 10) as usize;
	let mut group = Bytes::from(vec![0x5a; count * size]);
	let mut sum = 0u64;
	let mut unique = 0u64;
	for _ in 0..count {
		{
			let frame = group.split_to(size);
			let copy = frame.clone();
			let rest = copy.slice(1..);
			sum += u64::from(frame[0]) + rest.len() as u64;
		}
		unique += u64::from(group.is_unique());
	}
	put_u64(out, sum);
	put_u64(out, unique);
}

/// One iterator chain over every byte, picked by `chain`, with closures that cost little
/// next to the iterator protocol they run through.
fn iterate(chain: u8, data: &[u8], out: &mut Vec<u8>) {
	match chain {
		0 => put_usize(out, data.iter().map(|&b| b >> 1).filter(|&b| b & 1 == 0).count()),
		1 => out.push(data.iter().any(|&b| b == 0xff) as u8),
		2 => {
			let odd: Vec<u8> = data.iter().copied().filter(|&b| b & 1 == 1).collect();
			put_usize(out, odd.len());
		}
		3 => put_u64(out, data.iter().map(|&b| b as u32).sum::<u32>() as u64),
		4 => put_u64(
			out,
			data.iter().skip(16).take(512).fold(0u32, |acc, &b| acc ^ b as u32) as u64,
		),
		// `TrustedLen` but not by index, so `count` folds it by its size hint.
		6 => put_usize(out, data.iter().zip(data.iter().rev()).filter(|(a, b)| a < b).count()),
		// By index from the back, which skips the longer range's excess without reading it.
		7 => put_usize(
			out,
			(0..data.len() * 64)
				.zip(data)
				.rev()
				.filter(|&(i, &b)| (i as u8) < b)
				.count(),
		),
		// By index from the front, each side mapped, which may have side effects.
		8 => put_usize(
			out,
			data.iter()
				.map(|&b| b >> 1)
				.zip(data.iter().map(|&b| b ^ 3))
				.filter(|(a, b)| a < b)
				.count(),
		),
		_ => {
			let mut last = 0;
			for (i, &b) in data.iter().enumerate() {
				if b == 0x80 {
					last = i;
				}
			}
			put_usize(out, last);
		}
	}
}

/// Timestamps from one timescale to another, products past 2^53: moq-net's
/// `Timestamp::convert` in `u128`, or the same in `u64` (selector 1), which overflows
/// sooner. How many converted, how many overflowed, and the last converted.
fn rescale(selector: u8, data: &[u8], out: &mut Vec<u8>) {
	let (count, data) = take_u64(data);
	let (start, data) = take_u64(data);
	let (step, data) = take_u64(data);
	let (from, data) = take_u64(data);
	let (to, _) = take_u64(data);
	let (mut converted, mut overflowed, mut last) = (0, 0, 0);
	for i in 0..count as usize {
		let value = start + i as u64 * step;
		let scaled = match selector {
			0 => match (value as u128).checked_mul(to as u128) {
				Some(scaled) => bounded(scaled / from as u128),
				None => None,
			},
			_ => match value.checked_mul(to).map(|scaled| scaled / from) {
				Some(v) if v <= MAX_TIMESTAMP => Some(v),
				_ => None,
			},
		};
		match scaled {
			Some(v) => {
				last = v;
				converted += 1;
			}
			None => overflowed += 1,
		}
	}
	put_u64(out, converted);
	put_u64(out, overflowed);
	put_u64(out, last);
}

/// moq-net's largest timestamp.
const MAX_TIMESTAMP: u64 = (1 << 61) - 1;

/// `value`, or `None` past [`MAX_TIMESTAMP`], as moq-net bounds a converted timestamp.
const fn bounded(value: u128) -> Option<u64> {
	if value <= MAX_TIMESTAMP as u128 {
		Some(value as u64)
	} else {
		None
	}
}

/// moq-net's `Timestamp`, with the scale it keeps as a `NonZero` unwrapped.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Timestamp {
	value: u64,
	scale: u64,
}

impl Timestamp {
	/// The sum, or `None` across scales or past [`MAX_TIMESTAMP`], as moq-net's
	/// `Timestamp::checked_add` returns `TimeOverflow`.
	const fn checked_add(self, rhs: Self) -> Option<Self> {
		if self.scale != rhs.scale {
			return None;
		}
		match self.value.checked_add(rhs.value) {
			Some(value) => match bounded(value as u128) {
				Some(value) => Some(Self {
					value,
					scale: self.scale,
				}),
				None => None,
			},
			None => None,
		}
	}
}

impl PartialOrd for Timestamp {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Timestamp {
	/// moq-net's temporal order: across scales, the cross products in `u128`, with a tie
	/// broken by `(scale, value)`.
	fn cmp(&self, other: &Self) -> Ordering {
		if self.scale == other.scale {
			return self.value.cmp(&other.value);
		}
		let lhs = self.value as u128 * other.scale as u128;
		let rhs = other.value as u128 * self.scale as u128;
		lhs.cmp(&rhs)
			.then_with(|| self.scale.cmp(&other.scale))
			.then_with(|| self.value.cmp(&other.value))
	}
}

/// A track's frames stamped one duration apart by [`Timestamp::checked_add`], as a
/// publisher stamps them. How many were stamped, how many overflowed, and the last.
fn stamp(data: &[u8], out: &mut Vec<u8>) {
	let (count, data) = take_u64(data);
	let (start, data) = take_u64(data);
	let (step, data) = take_u64(data);
	let (scale, _) = take_u64(data);
	let step = Timestamp { value: step, scale };
	let mut time = Timestamp { value: start, scale };
	let (mut stamped, mut overflowed) = (0, 0);
	for _ in 0..count {
		match time.checked_add(step) {
			Some(next) => {
				time = next;
				stamped += 1;
			}
			None => overflowed += 1,
		}
	}
	put_u64(out, stamped);
	put_u64(out, overflowed);
	put_u64(out, time.value);
}

/// Two tracks' frames, each one duration after the last at its own scale, interleaved in
/// presentation order by [`Timestamp`]'s `Ord`, as a muxer interleaves them: one
/// comparison per frame taken. How many came from each track, and how many times the
/// interleave switched tracks.
fn interleave(data: &[u8], out: &mut Vec<u8>) {
	let (count, data) = take_u64(data);
	let (mut a, a_step, data) = track(data);
	let (mut b, b_step, _) = track(data);
	let (mut from_a, mut from_b, mut switches) = (0, 0, 0);
	let mut last_a = true;
	for _ in 0..count {
		let take_a = a < b;
		if take_a {
			a.value += a_step;
			from_a += 1;
		} else {
			b.value += b_step;
			from_b += 1;
		}
		if take_a != last_a {
			switches += 1;
		}
		last_a = take_a;
	}
	put_u64(out, from_a);
	put_u64(out, from_b);
	put_u64(out, switches);
}

/// A track's first timestamp and its frame duration: start, step, and scale.
fn track(data: &[u8]) -> (Timestamp, u64, &[u8]) {
	let (value, data) = take_u64(data);
	let (step, data) = take_u64(data);
	let (scale, data) = take_u64(data);
	(Timestamp { value, scale }, step, data)
}

fn decode_group(wire: &[u8], version: lite::Version, out: &mut Vec<u8>) -> Result<(), DecodeError> {
	let (group, used) = lite::Group::decode_slice(wire, version)?;
	put_u64(out, group.subscribe);
	put_u64(out, group.sequence);
	put_u64(out, group.frame_start);
	put_usize(out, used);
	Ok(())
}
