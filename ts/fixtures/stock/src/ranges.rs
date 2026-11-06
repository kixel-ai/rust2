//! `rangemap`'s maps and sets, which translate from their source over std's B-tree, keyed
//! by a wrapper of a range's start that its B-tree searches by a wrapper of its end.

use std::ops::Range;

use rangemap::{RangeInclusiveMap, RangeInclusiveSet, RangeMap, RangeSet};

/// A range's bounds, as eight bytes.
fn put(out: &mut Vec<u8>, start: u64, end: u64) {
	out.extend_from_slice(&(start as u32).to_le_bytes());
	out.extend_from_slice(&(end as u32).to_le_bytes());
}

/// Each range of `ranges`, then a marker.
fn put_all<'a>(out: &mut Vec<u8>, ranges: impl Iterator<Item = (u64, u64, u64)>) {
	for (start, end, value) in ranges {
		put(out, start, end);
		out.push(value as u8);
	}
	out.push(0xff);
}

/// A `RangeSet` and a `RangeMap` of `u64`, and their inclusive twins, through each triple
/// of bytes: a step, then a range's bounds, which may be empty or backwards. Each step
/// writes what it reads, then each collection's ranges.
pub fn ranges(data: &[u8], out: &mut Vec<u8>) {
	let mut set: RangeSet<u64> = RangeSet::new();
	let mut map: RangeMap<u64, u64> = RangeMap::new();
	let mut inclusive: RangeInclusiveSet<u64> = RangeInclusiveSet::new();
	let mut values: RangeInclusiveMap<u64, u32> = RangeInclusiveMap::new();
	for step in data.chunks(3) {
		let op = step[0];
		let (a, b) = (
			u64::from(step.get(1).copied().unwrap_or(0)),
			u64::from(step.get(2).copied().unwrap_or(0)),
		);
		let range: Range<u64> = a..b;
		let valid = a < b;
		match op % 8 {
			0 if valid => {
				set.insert(range.clone());
				map.insert(range, u64::from(op));
				inclusive.insert(a..=b - 1);
				values.insert(a..=b - 1, u32::from(op));
			}
			1 if valid => {
				set.remove(range.clone());
				map.remove(range);
				inclusive.remove(a..=b - 1);
				values.remove(a..=b - 1);
			}
			2 => {
				out.push(set.contains(&a) as u8);
				out.push(map.get(&a).map_or(0xfe, |&v| v as u8));
				out.push(inclusive.contains(&b) as u8);
				let (r, v) = values
					.get_key_value(&b)
					.map_or((0..0, 0xfe), |(r, v)| (*r.start()..*r.end(), *v));
				put(out, r.start, r.end);
				out.push(v as u8);
				let got = set.get(&b).map_or(0..0, Range::clone);
				put(out, got.start, got.end);
			}
			3 => {
				put_all(out, set.overlapping(&range).map(|r| (r.start, r.end, 0)));
				put_all(out, map.overlapping(range.clone()).map(|(r, v)| (r.start, r.end, *v)));
				put_all(out, inclusive.overlapping(a..=b).map(|r| (*r.start(), *r.end(), 0)));
				put_all(
					out,
					values
						.overlapping(&(a..=b))
						.map(|(r, v)| (*r.start(), *r.end(), u64::from(*v))),
				);
				out.push(set.overlaps(&range) as u8);
				out.push(values.overlaps(&(a..=b)) as u8);
			}
			4 => {
				put_all(out, set.gaps(&range).map(|r| (r.start, r.end, 0)));
				put_all(out, map.gaps(&range).map(|r| (r.start, r.end, 0)));
				let outer = a..=b;
				put_all(out, inclusive.gaps(&outer).map(|r| (*r.start(), *r.end(), 0)));
				put_all(out, values.gaps(&outer).map(|r| (*r.start(), *r.end(), 0)));
			}
			5 => {
				let first = set.first().map_or(0..0, Range::clone);
				let last = map.last_range_value().map_or(0..0, |(r, _)| r.clone());
				put(out, first.start, first.end);
				put(out, last.start, last.end);
				out.push(set.is_empty() as u8);
				out.push(set.len() as u8);
				out.push(map.len() as u8);
			}
			6 => {
				let copy = set.clone();
				put_all(out, copy.into_iter().rev().map(|r| (r.start, r.end, 0)));
				let pairs: Vec<_> = values.clone().into_iter().collect();
				put_all(
					out,
					pairs.into_iter().map(|(r, v)| (*r.start(), *r.end(), u64::from(v))),
				);
				out.push((copy_eq(&set, &map)) as u8);
			}
			7 if valid => {
				let mut more = RangeSet::from_iter([range.clone(), b..b + 3]);
				more.extend(set.iter().cloned().filter(|r| r.start % 2 == 0));
				put_all(out, more.iter().map(|r| (r.start, r.end, 0)));
				put_all(out, set.union(&more).map(|r| (r.start, r.end, 0)));
				put_all(out, set.intersection(&more).map(|r| (r.start, r.end, 0)));
			}
			// An empty range to insert or remove, which rangemap panics on.
			_ => out.push(0xfd),
		}
		put_all(out, set.iter().map(|r| (r.start, r.end, 0)));
		put_all(out, map.iter().map(|(r, v)| (r.start, r.end, *v)));
		put_all(out, inclusive.iter().map(|r| (*r.start(), *r.end(), 0)));
		put_all(out, values.iter().map(|(r, v)| (*r.start(), *r.end(), u64::from(*v))));
	}
}

/// Whether the set holds just the ranges the map does.
fn copy_eq(set: &RangeSet<u64>, map: &RangeMap<u64, u64>) -> bool {
	set.iter().eq(map.iter().map(|(r, _)| r))
}
