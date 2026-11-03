//! What std reads by index drops: a `skip` of a zip of two `TrustedRandomAccess` iterators,
//! which makes and drops each skipped pair whole where what wraps it reads it by index, and
//! a side at a time through the zip's `nth` where it reads by `next`; and a zip collected in
//! place, read by index and then dropped whole.

use crate::log::Log;
use crate::model::Guard;

/// Run scenario `arg % 9`.
pub(super) fn indexed(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	// A guard made by a side's closure, which says so, where std reads by index.
	let made = |n: u32| {
		log.note(format!("make {n}"));
		guard(n)
	};
	let v = [1u32, 2, 3, 4];
	let w = [11u32, 12, 13, 14];
	// A zip of guards std reads by index, each side's made as it reads it.
	let pairs = || v.iter().map(move |&n| made(n)).zip(w.iter().map(move |&n| made(n)));
	let ids = |p: Option<((Guard, Guard), u32)>| p.map(|((a, b), n)| (a.id() - id, b.id() - id, n));
	match arg % 9 {
		0 => {
			// Read by index by a zip that wraps it, from the front: the skipped pair first.
			let n = pairs().skip(1).zip(0..2).count();
			let mut it = pairs().skip(2).zip(0..5u32);
			log.note(format!("count {n} {:?}", ids(it.next())));
			log.note(format!("next {:?}", ids(it.next())));
			log.note(format!("past {:?}", ids(it.next())));
		}
		1 => {
			// From the back, where the last read is of index 0, and by `nth`.
			let mut it = pairs().skip(1).zip(0..2u32);
			log.note(format!("back {:?}", ids(it.next_back())));
			log.note(format!("back {:?}", ids(it.next_back())));
			log.note(format!("back {:?}", ids(it.next_back())));
			let mut it = pairs().skip(1).zip(0..3u32);
			log.note(format!("nth {:?}", ids(it.nth(1))));
			let mut it = pairs().skip(1).zip(0..3u32);
			log.note(format!("front {:?}", ids(it.next())));
			log.note(format!("back {:?}", ids(it.next_back())));
		}
		2 => {
			// Read by `next`, through the zip's `nth`, a side at a time.
			let mut it = pairs().skip(1);
			let first = it.next().map(|(a, b)| (a.id() - id, b.id() - id));
			log.note(format!("skip next {first:?}"));
			drop(it);
			let n = pairs().skip(2).count();
			let last = pairs().skip(1).last().map(|(a, b)| (a.id() - id, b.id() - id));
			log.note(format!("skip count {n} {last:?}"));
		}
		3 => {
			// A `take` folds what it wraps by index.
			let n = pairs()
				.skip(1)
				.take(2)
				.fold(0, |n, (a, b)| n * 100 + a.id() + b.id() - 2 * id);
			log.note(format!("take fold {n}"));
			pairs()
				.skip(2)
				.take(5)
				.for_each(|(a, b)| log.note(format!("each {} {}", a.id() - id, b.id() - id)));
			let found = pairs()
				.skip(1)
				.take(3)
				.find(|(a, _)| a.id() == id + 3)
				.map(|(a, _)| a.id() - id);
			log.note(format!("take find {found:?}"));
			// Bounded by what is left of what it wraps, which reads nothing once a `skip` passes it.
			let n = pairs().skip(4).take(1).fold(0, |n, (a, b)| n + a.id() + b.id());
			let m = pairs()
				.skip(3)
				.take(2)
				.fold(0, |n, (a, b)| n * 100 + a.id() + b.id() - 2 * id);
			log.note(format!("take past {n} {m}"));
			// A zip of it and what is `TrustedRandomAccessNoCoerce` alone folds both by index.
			let n = pairs().skip(1).zip(vec![0u32, 1].into_iter()).count();
			log.note(format!("vec count {n}"));
		}
		4 => {
			// Adaptors that read what they wrap by index as they are read.
			let n = pairs().skip(1).map(|(a, b)| a.id() + b.id() - 2 * id).zip(0..2).count();
			let e = pairs()
				.skip(1)
				.enumerate()
				.zip(0..2)
				.last()
				.map(|((i, (a, _)), n)| (i, a.id() - id, n));
			let f = pairs().skip(1).fuse().zip(0..1).count();
			log.note(format!("adapted {n} {e:?} {f}"));
		}
		5 => {
			// A `skip` past the end, of nothing, and nested.
			let n = pairs().skip(9).zip(0..2).count();
			let m = pairs().skip(0).zip(0..2).count();
			let o = pairs().skip(1).skip(1).zip(0..2).count();
			let p = pairs().skip(1).zip(0..3).skip(1).zip(0..1).count();
			log.note(format!("edges {n} {m} {o} {p}"));
		}
		6 => {
			// Read by index from the back, untrimmed: a `skip` of what is no zip, which makes and
			// drops its prefix as it reads index 0, and a zip of sides of unequal length.
			let mut it = v.iter().map(move |&n| made(n)).skip(1).zip(0..2u32);
			let id1 = |p: Option<(Guard, u32)>| p.map(|(a, n)| (a.id() - id, n));
			log.note(format!("back {:?}", id1(it.next_back())));
			log.note(format!("back {:?}", id1(it.next_back())));
			log.note(format!("back {:?}", id1(it.next_back())));
			let mut it = v
				.iter()
				.map(move |&n| made(n))
				.zip(w[..3].iter().map(move |&n| made(n)))
				.zip(0..5u32);
			log.note(format!("uneven {:?}", ids(it.next_back())));
			log.note(format!("uneven {:?}", ids(it.next())));
			let n = v
				.iter()
				.map(move |&n| made(n))
				.zip(w[..2].iter().map(move |&n| made(n)))
				.zip(0..5u32)
				.rev()
				.count();
			log.note(format!("uneven rev {n}"));
			// From both ends, the sides trimmed by the first read from the back alone.
			let mut it = v.iter().map(move |&n| made(n)).zip(w[..3].iter().map(move |&n| made(n)));
			let id2 = |p: Option<(Guard, Guard)>| p.map(|(a, b)| (a.id() - id, b.id() - id));
			log.note(format!("ends {:?}", id2(it.next())));
			log.note(format!("ends {:?}", id2(it.next_back())));
			log.note(format!("ends {:?}", id2(it.next())));
			log.note(format!("ends {:?}", id2(it.next_back())));
			// Read from the back, then by index from the back by the zips that wrap it, the
			// middle one shorter.
			let mut inner = pairs();
			log.note(format!("inner {:?}", id2(inner.next_back())));
			let mut it = inner.zip(0..2u32).zip(0..2u32);
			let id3 = |p: Option<(((Guard, Guard), u32), u32)>| {
				p.map(|(((a, b), m), n)| (a.id() - id, b.id() - id, m, n))
			};
			log.note(format!("nested {:?}", id3(it.next_back())));
			log.note(format!("nested {:?}", id3(it.next_back())));
			log.note(format!("nested {:?}", id3(it.next_back())));
			let mut it = v
				.iter()
				.map(move |&n| made(n))
				.enumerate()
				.zip(w[..3].iter().map(move |&n| made(n)))
				.zip(0..5u32);
			let last = it.next_back().map(|(((i, a), b), n)| (i, a.id() - id, b.id() - id, n));
			log.note(format!("enumerated {last:?}"));
		}
		7 => {
			// Collected in place by index, then dropped whole, inner closures before outer ones.
			let (a, b) = (guard(20), guard(21));
			let x = (&a).id() - id;
			let src: Vec<u32> = vec![1, 2, 3, 4];
			let out: Vec<u32> = src
				.into_iter()
				.map(move |n| n + (&a).id() - id - x)
				.zip(0..2u32)
				.map(move |(n, k)| n * 10 + k + (&b).id() - id - x - 1)
				.collect();
			log.note(format!("in place {out:?}"));
			let c = guard(22);
			let out: Vec<u32> = vec![5u32, 6, 7]
				.into_iter()
				.zip((0..9u32).map(move |k| k + (&c).id() - id - 22))
				.map(|(n, k)| n + k)
				.collect();
			log.note(format!("in place {out:?}"));
		}
		_ => {
			// Collected in place, skipping by index first, and empty.
			let d = guard(23);
			let out: Vec<u32> = vec![1u32, 2, 3, 4]
				.into_iter()
				.map(move |n| {
					log.note(format!("map {n}"));
					n + (&d).id() - id - 23
				})
				.zip(0..3u32)
				.skip(1)
				.map(|(n, k)| n + k)
				.collect();
			log.note(format!("skipped {out:?}"));
			// A `skip` of a zip std reads by index beside the `Vec` it collects in place of.
			let out: Vec<u32> = vec![1u32, 2, 3]
				.into_iter()
				.zip(pairs().skip(1))
				.map(|(n, (a, b))| n + a.id() + b.id() - 2 * id)
				.collect();
			log.note(format!("beside {out:?}"));
			let e = guard(24);
			let out: Vec<u32> = Vec::<u32>::new()
				.into_iter()
				.map(move |n| n + (&e).id())
				.zip(0..3u32)
				.map(|(n, k)| n + k)
				.collect();
			log.note(format!("empty {out:?}"));
		}
	}
}
