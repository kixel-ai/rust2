//! Where std drops a closure that owns a guard given to a string's search or edit: as a
//! search that returns a value returns, with the iterator a search returns, and as
//! `String::retain` returns, after the edit. One `retain` borrows drops with its owner,
//! as does a closure each search borrows, by `&` or by `&mut`, which keeps what it changed
//! across the searches. A function pointer searches as a closure does, and a boxed
//! closure, `Box<dyn FnMut>` or `Box<F>`, drops where the closure it owns would.

use crate::log::Log;
use crate::model::Guard;

/// Run scenario `arg % 9`.
pub(super) fn searched(log: &Log, id: u32, arg: u8) {
	let guard = |n: u32| Guard::new(id + n, log.clone());
	let text = "a,b;c,,d";
	let sep = |c: char| c == ',' || c == ';';
	match arg % 9 {
		0 => {
			// Each drops its closure before the call returns, matched or not.
			let a = guard(1);
			let found = text.contains(move |c: char| c == ';' && a.id() > 0);
			let b = guard(2);
			let missing = text.contains(move |c: char| c == 'z' && b.id() > 0);
			log.note(format!("contains {found} {missing}"));
			let (c, d) = (guard(3), guard(4));
			let finds = (
				text.find(move |c| sep(c) && d.id() > 0),
				text.rfind(move |ch| sep(ch) && c.id() > 0),
			);
			log.note(format!("find {finds:?}"));
			let (e, f) = (guard(5), guard(6));
			let ends = (
				text.starts_with(move |c: char| c == 'a' && e.id() > 0),
				text.ends_with(move |c: char| sep(c) && f.id() > 0),
			);
			log.note(format!("ends {ends:?}"));
			let (g, h) = (guard(7), guard(8));
			let stripped = (
				text.strip_prefix(move |c: char| c == 'a' && g.id() > 0),
				text.strip_suffix(move |c: char| c == 'a' && h.id() > 0),
			);
			log.note(format!("strip {stripped:?}"));
			let (i, j) = (guard(9), guard(10));
			let once = (
				text.split_once(move |c| sep(c) && i.id() > 0),
				text.rsplit_once(move |c| sep(c) && j.id() > 0),
			);
			log.note(format!("once {once:?}"));
			let (k, l, m) = (guard(11), guard(12), guard(13));
			let trimmed = (
				",a,".trim_matches(move |c| c == ',' && k.id() > 0),
				",a,".trim_start_matches(move |c| c == ',' && l.id() > 0),
				",a,".trim_end_matches(move |c| c == ',' && m.id() > 0),
			);
			log.note(format!("trim {trimmed:?}"));
			let (n, o) = (guard(14), guard(15));
			let replaced = (
				text.replace(move |c| sep(c) && n.id() > 0, "+"),
				text.replacen(move |c| sep(c) && o.id() > 0, "+", 2),
			);
			log.note(format!("replace {replaced:?}"));
		}
		1 => {
			// The iterator holds its closure: dropped with it, partly read or ended.
			let a = guard(1);
			let mut split = text.split(move |c| sep(c) && a.id() > 0);
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let b = guard(2);
			let rsplit: Vec<&str> = text.rsplit(move |c| sep(c) && b.id() > 0).collect();
			log.note(format!("rsplit {rsplit:?}"));
			let (c, d) = (guard(3), guard(4));
			let counts = (
				text.splitn(2, move |c| sep(c) && d.id() > 0).count(),
				text.rsplitn(3, move |ch| sep(ch) && c.id() > 0).last(),
			);
			log.note(format!("splitn {counts:?}"));
			let e = guard(5);
			for part in text.split_terminator(move |c| sep(c) && e.id() > 0) {
				log.note(format!("part {part}"));
				if part == "c" {
					break;
				}
			}
			let f = guard(6);
			let mut terminated = text.rsplit_terminator(move |c| sep(c) && f.id() > 0);
			log.note(format!("rsplit_terminator {:?}", terminated.nth(1)));
			drop(terminated);
			let g = guard(7);
			let inclusive: Vec<&str> = text
				.split_inclusive(move |c| sep(c) && g.id() > 0)
				.rev()
				.map(|s| s.trim_end_matches(','))
				.collect();
			log.note(format!("split_inclusive {inclusive:?}"));
			let (h, i) = (guard(8), guard(9));
			let mut matched = text.matches(move |c| sep(c) && h.id() > 0);
			let last = text.rmatches(move |c| sep(c) && i.id() > 0).next();
			log.note(format!("matches {:?} {last:?}", matched.next()));
			drop(matched);
			let (j, k) = (guard(10), guard(11));
			let indices = (
				text.match_indices(move |c| sep(c) && j.id() > 0).nth(2),
				text.rmatch_indices(move |c| sep(c) && k.id() > 0).next(),
			);
			log.note(format!("indices {indices:?}"));
		}
		2 => {
			// It drops after the edit, as `retain` returns.
			let a = guard(1);
			let mut s = String::from(text);
			s.retain(move |c| {
				a.note(&format!("retain {c}"));
				!sep(c)
			});
			log.note(format!("retained {s}"));
			// Lent, it drops with its owner, whatever it changed.
			let b = guard(2);
			let mut kept = 0;
			let mut every_other = move |_: char| {
				kept += 1;
				kept % 2 == 0 || b.id() == 0
			};
			s.retain(&mut every_other);
			log.note(format!("every other {s}"));
			s.retain(&mut every_other);
			log.note(format!("every other {s}"));
			drop(every_other);
			// An empty string never calls it, and still drops it.
			let c = guard(3);
			let mut empty = String::new();
			empty.retain(move |_| c.id() > 0);
			log.note(format!("empty {empty:?}"));
		}
		3 => {
			// Borrowed by `&`, it drops with its owner, after every search and iterator.
			let a = guard(1);
			let f = move |c: char| sep(c) && a.id() > 0;
			let found = (text.contains(&f), text.find(&f), text.rfind(&f));
			log.note(format!("contains {found:?}"));
			let ends = (text.starts_with(&f), text.ends_with(&f));
			let stripped = (text.strip_prefix(&f), ",a".strip_suffix(&f));
			log.note(format!("ends {ends:?} {stripped:?}"));
			let once = (text.split_once(&f), text.rsplit_once(&f));
			let trimmed = (
				",a,".trim_matches(&f),
				",a,".trim_start_matches(&f),
				",a,".trim_end_matches(&f),
			);
			let replaced = (text.replace(&f, "+"), text.replacen(&f, "+", 2));
			log.note(format!("once {once:?} {trimmed:?} {replaced:?}"));
			let mut split = text.split(&f);
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let rsplit: Vec<&str> = text.rsplit(&f).collect();
			let counts = (text.splitn(2, &f).count(), text.rsplitn(3, &f).last());
			log.note(format!("rsplit {rsplit:?} {counts:?}"));
			let terminated = (text.split_terminator(&f).nth(3), text.rsplit_terminator(&f).nth(1));
			let inclusive: Vec<&str> = text.split_inclusive(&f).rev().collect();
			log.note(format!("terminated {terminated:?} {inclusive:?}"));
			let mut matched = text.matches(&f);
			let last = text.rmatches(&f).next();
			log.note(format!("matches {:?} {last:?}", matched.next()));
			drop(matched);
			let indices = (text.match_indices(&f).nth(2), text.rmatch_indices(&f).next());
			log.note(format!("indices {indices:?}"));
			// A clone of an iterator shares what it borrows.
			let mut parts = text.split(&f);
			parts.next();
			let rest: Vec<&str> = parts.clone().collect();
			log.note(format!("cloned {rest:?} {:?}", parts.next()));
			drop(parts);
			// A function's item, borrowed.
			let spaced: Vec<&str> = "a b\tc".split(&char::is_whitespace).collect();
			log.note(format!("spaced {spaced:?} {}", f(';')));
			drop(f);
			log.note("dropped".into());
		}
		4 => {
			// Borrowed by `&mut`, it counts every call across the searches and drops with its
			// owner.
			let a = guard(1);
			let mut calls = 0;
			let mut f = move |c: char| {
				calls += 1;
				sep(c) && calls % 3 != 0 && a.id() > 0
			};
			let found = (text.contains(&mut f), text.find(&mut f), text.rfind(&mut f));
			log.note(format!("contains {found:?} {}", f('x')));
			let ends = (text.starts_with(&mut f), text.ends_with(&mut f));
			let stripped = (",a".strip_prefix(&mut f), ",a".strip_suffix(&mut f));
			log.note(format!("ends {ends:?} {stripped:?} {}", f('x')));
			let once = (text.split_once(&mut f), text.rsplit_once(&mut f));
			let trimmed = (",,a,,".trim_matches(&mut f), ",,a".trim_start_matches(&mut f));
			let replaced = (text.replace(&mut f, "+"), text.replacen(&mut f, "+", 2));
			let end = ",a,,".trim_end_matches(&mut f);
			log.note(format!("once {once:?} {trimmed:?} {replaced:?} {end:?} {}", f('x')));
			let mut split = text.split(&mut f);
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let rsplit: Vec<&str> = text.rsplit(&mut f).collect();
			let n = text.splitn(2, &mut f).count();
			let last = text.rsplitn(3, &mut f).last();
			log.note(format!("rsplit {rsplit:?} {n} {last:?} {}", f('x')));
			let forward = text.split_terminator(&mut f).nth(3);
			let back = text.rsplit_terminator(&mut f).nth(1);
			let inclusive: Vec<&str> = text.split_inclusive(&mut f).rev().collect();
			log.note(format!("terminated {forward:?} {back:?} {inclusive:?} {}", f('x')));
			let mut matched = text.matches(&mut f);
			log.note(format!("matches {:?}", matched.next()));
			drop(matched);
			let last = text.rmatches(&mut f).next();
			let indices = (text.match_indices(&mut f).nth(2), text.rmatch_indices(&mut f).next());
			log.note(format!("indices {last:?} {indices:?} {}", f('x')));
			// Through a `&mut` of a `&mut`, the same closure.
			let mut lent = &mut f;
			let parts: Vec<&str> = text.split(&mut lent).collect();
			log.note(format!("lent {parts:?} {}", f('x')));
			drop(f);
			log.note("dropped".into());
		}
		5 => {
			// A function pointer, by value or borrowed, searches as the function does.
			let p: fn(char) -> bool = if id < u32::MAX { is_sep } else { char::is_whitespace };
			let found = (text.contains(p), text.find(p), text.rfind(&p));
			let ends = (text.starts_with(p), text.ends_with(&p));
			let stripped = (",a".strip_prefix(p), ",a".strip_suffix(&p));
			log.note(format!("contains {found:?} {ends:?} {stripped:?}"));
			let once = (text.split_once(p), text.rsplit_once(&p));
			let trimmed = (
				",a,".trim_matches(p),
				",a,".trim_start_matches(&p),
				",a,".trim_end_matches(p),
			);
			let replaced = (text.replace(p, "+"), text.replacen(&p, "+", 2));
			log.note(format!("once {once:?} {trimmed:?} {replaced:?}"));
			let mut split = text.split(p);
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			let rsplit: Vec<&str> = text.rsplit(&p).collect();
			let counts = (text.splitn(2, p).count(), text.rsplitn(3, &p).last());
			log.note(format!("rsplit {rsplit:?} {counts:?}"));
			let terminated = (text.split_terminator(p).nth(3), text.rsplit_terminator(&p).nth(1));
			let inclusive: Vec<&str> = text.split_inclusive(p).rev().collect();
			log.note(format!("terminated {terminated:?} {inclusive:?}"));
			let matched = (text.matches(&p).next(), text.rmatches(p).next());
			let indices = (text.match_indices(p).nth(2), text.rmatch_indices(&p).next());
			log.note(format!("matches {matched:?} {indices:?}"));
		}
		6 => {
			// A `Box<dyn FnMut>` drops the closure it owns where the closure would drop: as a
			// search returns, or with the iterator it returns.
			let boxed = |n: u32| -> Box<dyn FnMut(char) -> bool> {
				let g = guard(n);
				Box::new(move |c| sep(c) && g.id() > 0)
			};
			let found = (text.contains(boxed(1)), text.find(boxed(2)), text.rfind(boxed(3)));
			log.note(format!("contains {found:?}"));
			let ends = (text.starts_with(boxed(4)), text.ends_with(boxed(5)));
			let stripped = (",a".strip_prefix(boxed(6)), ",a".strip_suffix(boxed(7)));
			log.note(format!("ends {ends:?} {stripped:?}"));
			let once = (text.split_once(boxed(8)), text.rsplit_once(boxed(9)));
			let trimmed = (
				",a,".trim_matches(boxed(10)),
				",a,".trim_start_matches(boxed(11)),
				",a,".trim_end_matches(boxed(12)),
			);
			let replaced = (text.replace(boxed(13), "+"), text.replacen(boxed(14), "+", 2));
			log.note(format!("once {once:?} {trimmed:?} {replaced:?}"));
			let mut split = text.split(boxed(15));
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let rsplit: Vec<&str> = text.rsplit(boxed(16)).collect();
			let counts = (text.splitn(2, boxed(17)).count(), text.rsplitn(3, boxed(18)).last());
			log.note(format!("rsplit {rsplit:?} {counts:?}"));
			let terminated = (
				text.split_terminator(boxed(19)).nth(3),
				text.rsplit_terminator(boxed(20)).nth(1),
			);
			let inclusive: Vec<&str> = text.split_inclusive(boxed(21)).rev().collect();
			log.note(format!("terminated {terminated:?} {inclusive:?}"));
			let mut matched = text.matches(boxed(22));
			let last = text.rmatches(boxed(23)).next();
			log.note(format!("matches {:?} {last:?}", matched.next()));
			drop(matched);
			let indices = (
				text.match_indices(boxed(24)).nth(2),
				text.rmatch_indices(boxed(25)).next(),
			);
			log.note(format!("indices {indices:?}"));
			// One that owns nothing drops nothing, by value or with its iterator.
			let plain: Box<dyn FnMut(char) -> bool> = Box::new(is_sep);
			let mut split = text.split(plain);
			let found = text.contains(Box::new(sep) as Box<dyn FnMut(char) -> bool>);
			log.note(format!("plain {:?} {found}", split.next()));
			drop(split);
		}
		7 => {
			// Borrowed by `&mut`, a `Box<dyn FnMut>` counts every call across the searches and
			// drops with its owner.
			let a = guard(1);
			let mut calls = 0;
			let mut b: Box<dyn FnMut(char) -> bool> = Box::new(move |c: char| {
				calls += 1;
				sep(c) && calls % 3 != 0 && a.id() > 0
			});
			let found = (text.contains(&mut b), text.find(&mut b), text.rfind(&mut b));
			log.note(format!("contains {found:?} {}", b('x')));
			let ends = (text.starts_with(&mut b), text.ends_with(&mut b));
			let stripped = (",a".strip_prefix(&mut b), ",a".strip_suffix(&mut b));
			log.note(format!("ends {ends:?} {stripped:?} {}", b('x')));
			let once = (text.split_once(&mut b), text.rsplit_once(&mut b));
			let trimmed = (",,a,,".trim_matches(&mut b), ",,a".trim_start_matches(&mut b));
			let replaced = (text.replace(&mut b, "+"), text.replacen(&mut b, "+", 2));
			let end = ",a,,".trim_end_matches(&mut b);
			log.note(format!("once {once:?} {trimmed:?} {replaced:?} {end:?} {}", b('x')));
			let mut split = text.split(&mut b);
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let rsplit: Vec<&str> = text.rsplit(&mut b).collect();
			let n = text.splitn(2, &mut b).count();
			let last = text.rsplitn(3, &mut b).last();
			log.note(format!("rsplit {rsplit:?} {n} {last:?} {}", b('x')));
			let forward = text.split_terminator(&mut b).nth(3);
			let back = text.rsplit_terminator(&mut b).nth(1);
			let inclusive: Vec<&str> = text.split_inclusive(&mut b).rev().collect();
			log.note(format!("terminated {forward:?} {back:?} {inclusive:?} {}", b('x')));
			let mut matched = text.matches(&mut b);
			log.note(format!("matches {:?}", matched.next()));
			drop(matched);
			let last = text.rmatches(&mut b).next();
			let indices = (text.match_indices(&mut b).nth(2), text.rmatch_indices(&mut b).next());
			log.note(format!("indices {last:?} {indices:?} {}", b('x')));
			drop(b);
			log.note("dropped".into());
		}
		_ => {
			// A `Box` of the closure itself drops it where the closure would drop.
			let (a, b, c) = (guard(1), guard(2), guard(3));
			let found = text.contains(Box::new(move |c: char| c == ';' && a.id() > 0));
			let at = text.rfind(Box::new(move |c| sep(c) && b.id() > 0));
			let replaced = text.replacen(Box::new(move |ch| sep(ch) && c.id() > 0), "+", 2);
			log.note(format!("contains {found} {at:?} {replaced}"));
			let d = guard(4);
			let mut split = text.split(Box::new(move |c| sep(c) && d.id() > 0));
			log.note(format!("split {:?} {:?}", split.next(), split.next_back()));
			drop(split);
			let e = guard(5);
			let indices: Vec<_> = text.rmatch_indices(Box::new(move |c| sep(c) && e.id() > 0)).collect();
			log.note(format!("indices {indices:?}"));
			// One that counts its calls, borrowed by `&mut` across searches, drops with its owner.
			let f = guard(6);
			let mut calls = 0;
			let mut counted = Box::new(move |c: char| {
				calls += 1;
				sep(c) && calls % 2 == 0 && f.id() > 0
			});
			let found = (text.find(&mut counted), text.contains(&mut counted));
			let parts: Vec<&str> = text.split(&mut counted).collect();
			log.note(format!("counted {found:?} {parts:?} {}", (*counted)('x')));
			drop(counted);
			// A clone of an iterator clones the box it owns, with what its closure counted.
			let mut calls = 0;
			let mut split = text.split(Box::new(move |c: char| {
				calls += 1;
				sep(c) && calls != 2
			}));
			let cloned = split.clone();
			log.note(format!("first {:?}", split.next()));
			let rest: Vec<&str> = cloned.collect();
			log.note(format!("cloned {rest:?} {:?}", split.next()));
			drop(split);
			// A `Box<dyn Fn>` borrowed by `&`, and a boxed function pointer, by value.
			let g = guard(7);
			let shared: Box<dyn Fn(char) -> bool> = Box::new(move |c| sep(c) && g.id() > 0);
			let parts: Vec<&str> = text.rsplit(&shared).collect();
			let p: Box<fn(char) -> bool> = Box::new(is_sep);
			log.note(format!("shared {:?} {parts:?} {:?}", text.rfind(&shared), text.find(p)));
			drop(shared);
			log.note("dropped".into());
		}
	}
}

fn is_sep(c: char) -> bool {
	c == ',' || c == ';'
}
