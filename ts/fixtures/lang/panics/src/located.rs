//! `panic::Location`: `Location::caller` in a function and in a `#[track_caller]` one,
//! through chains of them, closures, trait methods called directly, generically, through
//! `Index`, and through trait objects whose trait or only whose impl tracks its caller,
//! function pointers, constants, and macros; its accessors, `Display`, `Debug`, comparisons, and
//! hash; and a panic whose message carries one.

use std::collections::{BTreeSet, HashSet};
use std::fmt::Write;
use std::ops::Index;
use std::panic::Location;

type Here = &'static Location<'static>;

#[track_caller]
const fn here() -> Here {
	Location::caller()
}

/// A chain, which passes on its own caller's location.
#[track_caller]
fn outer() -> Here {
	here()
}

#[track_caller]
fn twice() -> (Here, Here) {
	(outer(), Location::caller())
}

/// Not tracked: its own call of `here`.
fn plain() -> Here {
	here()
}

#[track_caller]
fn picked(first: u8) -> Here {
	match first % 3 {
		0 => here(),
		1 => Location::caller(),
		_ => plain(),
	}
}

/// A closure is no `#[track_caller]` function, so it gives its own calls'.
#[track_caller]
fn in_closure() -> (Here, Here) {
	let own = || Location::caller();
	let called = || here();
	(own(), called())
}

trait Tracked {
	#[track_caller]
	fn at(&self) -> Here;

	/// A default, untracked, of the tracked method.
	fn via(&self) -> Here {
		self.at()
	}
}

impl Tracked for u8 {
	fn at(&self) -> Here {
		Location::caller()
	}
}

/// Inherits the trait's `#[track_caller]`.
struct Inherits;

impl Tracked for Inherits {
	fn at(&self) -> Here {
		here()
	}
}

trait Untracked {
	fn at(&self) -> Here;
}

/// Only the impl tracks: a call through a trait object passes the impl's own location.
struct Own;

impl Untracked for Own {
	#[track_caller]
	fn at(&self) -> Here {
		Location::caller()
	}
}

fn generic<T: Tracked>(t: &T) -> Here {
	t.at()
}

/// Indexing that gives where it was indexed.
struct Places;

impl Index<u8> for Places {
	type Output = Location<'static>;

	#[track_caller]
	fn index(&self, _: u8) -> &Location<'static> {
		Location::caller()
	}
}

macro_rules! called {
	() => {
		here()
	};
}

macro_rules! nested {
	() => {
		called!()
	};
}

/// Each location's `Display`, a line each.
fn lines(out: &mut Vec<u8>, at: &[Here]) {
	for at in at {
		out.extend_from_slice(format!("{at}\n").as_bytes());
	}
}

/// Calls of `#[track_caller]` functions, and `Location::caller` itself.
pub fn called(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	match kind % 6 {
		0 => {
			let (a, b) = twice();
			lines(
				out,
				&[Location::caller(), here(), outer(), a, b, plain(), picked(first)],
			);
		}
		1 => {
			let (own, called) = in_closure();
			let f = |n: u8| if n % 2 == 0 { here() } else { outer() };
			lines(out, &[own, called, f(first), (|| Location::caller())()]);
		}
		2 => {
			let tracked: &dyn Tracked = &first;
			let inherits: &dyn Tracked = &Inherits;
			let own: &dyn Untracked = &Own;
			lines(
				out,
				&[
					first.at(),
					first.via(),
					Inherits.at(),
					generic(&first),
					generic(&Inherits),
					tracked.at(),
					tracked.via(),
					inherits.at(),
					own.at(),
					Own.at(),
					&Places[first],
				],
			);
		}
		3 => {
			// A function pointer passes the function's own location.
			let f: fn() -> Here = here;
			let g = outer as fn() -> Here;
			let h: fn(u8) -> Here = picked;
			let fs: [fn() -> Here; 2] = [here, plain];
			let own: fn(&Own) -> Here = <Own as Untracked>::at;
			lines(out, &[f(), g(), h(first), fs[usize::from(first % 2)](), own(&Own)]);
		}
		4 => {
			// A constant's and a static's are where its value calls, and a function item held
			// in a variable is called where the call is.
			const HERE: Here = Location::caller();
			const VIA: Here = here();
			static THERE: Here = Location::caller();
			let f = Location::caller;
			let g = here;
			lines(out, &[HERE, VIA, THERE, f(), g()]);
		}
		_ => {
			// A macro's call is where the outermost macro was called; a format argument is
			// where it is written, even past chars of other widths.
			let wide = "é😀\u{3000}";
			let at = [called!(), nested!(), here()];
			lines(out, &at);
			let s = format!("{wide}{}{wide}{}", here(), Location::caller());
			out.extend_from_slice(s.as_bytes());
		}
	}
}

/// What a `Location` says of itself, and how locations compare.
pub fn read(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let a = here();
	let b = picked(first);
	match kind % 4 {
		0 => {
			let mut s = String::new();
			write!(s, "{}|{}|{}|", a.file(), a.line(), a.column()).unwrap();
			write!(s, "{a:>60}|{a:?}|{b:#?}|{b:x?}").unwrap();
			out.extend_from_slice(s.as_bytes());
		}
		1 => {
			let copy: Location<'static> = *a;
			let same = here();
			out.extend([
				u8::from(a == b),
				u8::from(*a == copy),
				u8::from(a != same),
				u8::from(a < b),
				u8::from(a.ge(&same)),
				a.cmp(&b) as u8,
				a.partial_cmp(&same).map_or(9, |o| o as u8),
			]);
			lines(out, &[std::cmp::max(a, b), std::cmp::min(a, b)]);
		}
		2 => {
			let mut all = vec![b, here(), a, outer(), picked(first.wrapping_add(1))];
			all.sort();
			all.dedup();
			lines(out, &all);
			let set: BTreeSet<Here> = all.iter().copied().collect();
			let hashed: HashSet<Location<'static>> = all.iter().map(|l| **l).collect();
			out.extend([set.len() as u8, hashed.len() as u8, u8::from(hashed.contains(a))]);
		}
		_ => {
			// Held in a struct and an `Option`, as a value of any type is.
			#[derive(Debug, Clone, Copy, PartialEq)]
			struct Held {
				at: Here,
				maybe: Option<Here>,
			}
			let held = Held {
				at: a,
				maybe: (first % 2 == 0).then(|| here()),
			};
			let copy = held;
			let s = format!("{held:?}|{}", u8::from(copy == held));
			out.extend_from_slice(s.as_bytes());
		}
	}
}

/// Panics whose message carries where a `#[track_caller]` function was called.
pub fn panicked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	#[track_caller]
	fn check(ok: bool) {
		if !ok {
			panic!("failed at {}", Location::caller());
		}
	}
	let first = data.first().copied().unwrap_or(0);
	match kind % 2 {
		0 => check(first % 3 != 0),
		_ => {
			let f: fn(bool) = check;
			f(first % 3 != 1);
		}
	}
	out.push(first);
}
