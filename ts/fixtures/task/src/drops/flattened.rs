//! What a `flatten` or `flat_map` drops of inner iterators that still hold items once they
//! end, as a `take` of a `Vec`'s does: `next` drops each as it ends, and std's `try_fold`
//! keeps it until the next replaces it, or until the fold returns once its source ends.
//! Each adaptor and consumer that reads it by `try_fold` or `try_rfold` sees that.

use crate::log::Log;
use crate::model::Guard;

/// Three `Vec`s of guards, of 3, 2, and 3, numbered from `n`.
fn nested(log: &Log, id: u32, n: u32) -> Vec<Vec<Guard>> {
	let guards = |from: u32, to: u32| (from..to).map(|k| Guard::new(id + k, log.clone())).collect();
	let mut all = Vec::new();
	all.push(guards(n, n + 3));
	all.push(guards(n + 3, n + 5));
	all.push(guards(n + 5, n + 8));
	all
}

/// A flatten of the first of each of `nested`'s from `n`, which logs each it makes.
fn firsts(log: &Log, id: u32, n: u32) -> impl DoubleEndedIterator<Item = Guard> + '_ {
	nested(log, id, n)
		.into_iter()
		.map(move |v| {
			log.note(format!("inner {}", v[0].id() - id));
			v.into_iter().take(1)
		})
		.flatten()
}

/// Run scenario `arg % 10`.
pub(super) fn flattened(log: &Log, id: u32, arg: u8) {
	let id_of = |g: Option<Guard>| g.map(|g| g.id() - id);
	match arg % 10 {
		0 => {
			// `next`, partly and then to the end.
			let mut it = firsts(log, id, 1);
			log.note(format!("next {:?}", id_of(it.next())));
			log.note(format!("next {:?}", id_of(it.next())));
			drop(it);
			for g in firsts(log, id, 20) {
				log.note(format!("for {}", g.id() - id));
			}
		}
		1 => {
			// Folds, which drop each inner iterator as its fold ends.
			firsts(log, id, 1).for_each(|g| log.note(format!("each {}", g.id() - id)));
			log.note(format!("count {}", firsts(log, id, 20).count()));
			log.note(format!("last {:?}", id_of(firsts(log, id, 40).last())));
		}
		2 => {
			// `try_fold`, which keeps the inner iterator it ended in until the next replaces it.
			let mut it = firsts(log, id, 1);
			log.note(format!("find {:?}", id_of(it.find(|g| g.id() == id + 4))));
			log.note(format!("then {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 20);
			log.note(format!("any {}", it.any(|g| g.id() == id + 99)));
			drop(it);
			log.note(format!("all {}", firsts(log, id, 40).all(|g| g.id() != id + 43)));
			log.note(format!("position {:?}", firsts(log, id, 60).position(|g| g.id() == id + 63)));
		}
		3 => {
			// Adaptors whose `next` is their source's `find`.
			let mut it = firsts(log, id, 1).filter(|g| g.id() != id + 1);
			log.note(format!("filter {:?}", id_of(it.next())));
			log.note(format!("filter {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 20).filter_map(|g| (g.id() != id + 20).then_some(g));
			log.note(format!("filter_map {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 40).skip_while(|g| g.id() < id + 44);
			log.note(format!("skip_while {:?}", id_of(it.next())));
			drop(it);
		}
		4 => {
			// Skipping, by `advance_by`, which is `try_fold`'s.
			let mut it = firsts(log, id, 1);
			log.note(format!("nth {:?}", id_of(it.nth(1))));
			drop(it);
			let mut it = firsts(log, id, 20).skip(1);
			log.note(format!("skip {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 40).step_by(2);
			log.note(format!("step {:?}", id_of(it.next())));
			log.note(format!("step {:?}", id_of(it.next())));
			drop(it);
		}
		5 => {
			// From the back, by `next_back` and `try_rfold`.
			let mut it = firsts(log, id, 1);
			log.note(format!("back {:?}", id_of(it.next_back())));
			log.note(format!("front {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 20);
			log.note(format!("rfind {:?}", id_of(it.rfind(|g| g.id() == id + 20))));
			drop(it);
			let mut it = firsts(log, id, 40).rev();
			log.note(format!("rev find {:?}", id_of(it.find(|g| g.id() == id + 43))));
			log.note(format!("rev next {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 60).rev().filter(|g| g.id() != id + 65);
			log.note(format!("rev filter {:?}", id_of(it.next())));
			drop(it);
			// A `take`'s own `try_rfold`, which reads past what it holds beyond the limit by
			// one `nth_back` short of `next_back`'s, and drops what that returns.
			let mut it = nested(log, id, 80).swap_remove(0).into_iter().take(1);
			log.note(format!("take rfind {:?}", id_of(it.rfind(|_| true))));
		}
		6 => {
			// Adaptors whose `try_fold` is their source's.
			let mut it = firsts(log, id, 1).map(|g| g.id() - id);
			log.note(format!("map find {:?}", it.find(|&n| n == 4)));
			drop(it);
			let mut it = firsts(log, id, 20).enumerate();
			log.note(format!("enumerate {:?}", it.find(|(i, _)| *i == 1).map(|(i, g)| (i, g.id() - id))));
			drop(it);
			log.note(format!("take {}", firsts(log, id, 40).take(2).fold(0, |n, g| n + g.id() - id)));
			let mut it = firsts(log, id, 60).chain(firsts(log, id, 80));
			log.note(format!("chain {:?}", id_of(it.find(|g| g.id() == id + 81))));
			drop(it);
		}
		7 => {
			// A `flat_map` of the same, and a `peekable`.
			let mut it = nested(log, id, 1).into_iter().flat_map(|v| v.into_iter().take(1));
			log.note(format!("flat_map {:?}", id_of(it.find(|g| g.id() == id + 4))));
			log.note(format!("flat_map {:?}", id_of(it.next())));
			drop(it);
			let mut it = firsts(log, id, 20).peekable();
			log.note(format!("peek {:?}", it.peek().map(|g| g.id() - id)));
			log.note(format!("peek find {:?}", id_of(it.find(|g| g.id() == id + 23))));
			drop(it);
		}
		8 => {
			// Collected, and reduced.
			let all: Vec<Guard> = firsts(log, id, 1).collect();
			log.note(format!("collect {}", all.len()));
			drop(all);
			log.note(format!("max {:?}", id_of(firsts(log, id, 20).max_by_key(|g| g.id() % 3))));
			log.note(format!("sum {}", firsts(log, id, 40).map(|g| g.id() - id).sum::<u32>()));
		}
		_ => {
			// A fold of its own after a `try_fold` left an inner iterator held.
			let mut it = firsts(log, id, 1);
			log.note(format!("any {}", it.any(|g| g.id() == id + 1)));
			log.note(format!("rest {}", it.fold(0, |n, g| n * 100 + g.id() - id)));
			let mut it = firsts(log, id, 20);
			log.note(format!("any {}", it.any(|g| g.id() == id + 23)));
			log.note(format!("count {}", it.count()));
		}
	}
}
