//! The crate's generic structs holding, over their type parameters, what only each
//! instance's glue drops, as `hashlink`'s `LruCache` holds a `LinkedHashMap`: a struct
//! with a `Drop` impl of every instance, an enum, a `Result`, an `Option` of an enum, a
//! tuple of one, a struct of a `Vec`, and structs that hold themselves or each other.
//! Each is dropped at the end of its scope, by assignment, from a `Vec`, and from an
//! `Arc`, and a field is moved out while the rest drop in place, every drop logged in the
//! order it happens.

use std::cell::{OnceCell, RefCell};
use std::sync::Arc;

/// Logs its byte as it drops.
struct Logged<'a>(u8, &'a RefCell<Vec<u8>>);

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// Logs its mark by a `Drop` impl of every instance, before what it holds drops.
struct Guard<'a, T> {
	mark: u8,
	value: T,
	log: &'a RefCell<Vec<u8>>,
}

impl<T> Drop for Guard<'_, T> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.mark);
	}
}

enum Either<T> {
	Left(T),
	Right(T, T),
	Neither,
}

/// One class's glue drops every instance of it.
struct Plain<U> {
	u: U,
	n: u8,
}

struct InGuard<'a, T> {
	g: Guard<'a, T>,
	after: T,
}

struct InEnum<T> {
	e: Either<T>,
	after: T,
}

struct InResult<'a, T> {
	r: Result<T, Logged<'a>>,
	after: T,
}

struct InOption<T> {
	o: Option<Either<T>>,
	cell: OnceCell<Either<T>>,
}

struct InPlain<T> {
	p: Plain<Vec<T>>,
	t: (Either<T>, T),
}

struct InBox<'a, T> {
	b: Box<Guard<'a, T>>,
	v: Vec<Guard<'a, T>>,
}

/// Three deep, the innermost with its own `Drop` impl.
struct Deep<'a, T> {
	inner: InGuard<'a, T>,
	after: T,
}

/// Holds itself through a `Vec`, beside an enum.
struct Tree<T> {
	e: Either<T>,
	kids: Vec<Tree<T>>,
}

/// Each holds the other, and only `Pong` an enum, so neither drops alike in every instance.
struct Ping<T> {
	pong: Option<Box<Pong<T>>>,
	v: T,
}

struct Pong<T> {
	ping: Option<Box<Ping<T>>>,
	e: Either<T>,
}

/// Holds itself through a box alone, so one class's glue still drops every instance.
struct Chain<T> {
	v: T,
	next: Option<Box<Chain<T>>>,
}

/// What an `if let` moves out of a temporary, beside variants whose rest the statement
/// drops.
enum Picked<'a, T> {
	A(Logged<'a>),
	B(T),
	C(Option<T>),
}

/// The byte logged between drops, so where each happens shows.
const MARK: u8 = 0xfe;

/// Two of what `make` makes, each from its own base of bytes, dropped by the way `how`
/// picks.
fn ways<T>(make: impl Fn(u8) -> T, how: u8, log: &RefCell<Vec<u8>>) {
	let mark = || log.borrow_mut().push(MARK);
	match how % 4 {
		// The later first.
		0 => {
			let _a = make(0x00);
			let _b = make(0x40);
			mark();
		}
		// The old value as the new one is assigned, after the new one is made.
		1 => {
			let mut a = make(0x00);
			mark();
			a = make(0x40);
			mark();
			drop(a);
		}
		2 => {
			let mut v = vec![make(0x00), make(0x40), make(0x80)];
			v.truncate(usize::from(how >> 2) % 3);
			mark();
			if !v.is_empty() {
				drop(v.swap_remove(0));
				mark();
			}
		}
		// By the last owner.
		_ => {
			let a = Arc::new(make(0x00));
			let b = Arc::clone(&a);
			drop(a);
			mark();
			drop(b);
			mark();
		}
	}
}

/// Case `kind % 12`, its variants picked by the first byte of `data` and its drops by the
/// second.
pub fn drops(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = RefCell::new(Vec::new());
	let pick = data.first().copied().unwrap_or(0);
	let how = data.get(1).copied().unwrap_or(0);
	let l = |id: u8| Logged(id, &log);
	let either = |id: u8| match pick % 3 {
		0 => Either::Left(l(id)),
		1 => Either::Right(l(id), l(id + 1)),
		_ => Either::Neither,
	};
	let guard = |id: u8| Guard {
		mark: id | 0x20,
		value: l(id),
		log: &log,
	};
	let in_guard = |b: u8| InGuard {
		g: guard(b + 1),
		after: l(b + 2),
	};
	let in_enum = |b: u8| InEnum {
		e: either(b + 1),
		after: l(b + 3),
	};
	match kind % 12 {
		0 => ways(in_guard, how, &log),
		1 => ways(in_enum, how, &log),
		2 => ways(
			|b| InResult {
				r: if pick & 1 == 0 { Ok(l(b + 1)) } else { Err(l(b + 1)) },
				after: l(b + 2),
			},
			how,
			&log,
		),
		3 => ways(
			|b| {
				let cell = OnceCell::new();
				if pick & 4 != 0 {
					let _ = cell.set(either(b + 4));
				}
				InOption {
					o: (pick & 8 != 0).then(|| either(b + 1)),
					cell,
				}
			},
			how,
			&log,
		),
		4 => ways(
			|b| InPlain {
				p: Plain {
					u: vec![l(b + 1), l(b + 2)],
					n: b,
				},
				t: (either(b + 3), l(b + 5)),
			},
			how,
			&log,
		),
		5 => ways(
			|b| InBox {
				b: Box::new(guard(b + 1)),
				v: (0..pick % 3).map(|i| guard(b + 2 + i)).collect(),
			},
			how,
			&log,
		),
		6 => ways(
			|b| Deep {
				inner: in_guard(b),
				after: l(b + 3),
			},
			how,
			&log,
		),
		7 => ways(
			|b| Tree {
				e: either(b + 1),
				kids: (0..pick % 3)
					.map(|i| Tree {
						e: either(b + 3 + 2 * i),
						kids: Vec::new(),
					})
					.collect(),
			},
			how,
			&log,
		),
		8 => ways(
			|b| Ping {
				pong: Some(Box::new(Pong {
					ping: (pick & 8 != 0).then(|| {
						Box::new(Ping {
							pong: None,
							v: l(b + 3),
						})
					}),
					e: either(b + 1),
				})),
				v: l(b + 4),
			},
			how,
			&log,
		),
		9 => ways(
			|b| Chain {
				v: l(b + 1),
				next: (pick & 8 != 0).then(|| {
					Box::new(Chain {
						v: l(b + 2),
						next: None,
					})
				}),
			},
			how,
			&log,
		),
		// A field moved out, and the rest dropped where it is, at the end of its scope.
		10 => {
			let s = in_guard(0);
			let after = s.after;
			log.borrow_mut().push(MARK);
			if pick & 1 == 0 {
				drop(after);
			}
			log.borrow_mut().push(MARK);
		}
		// What a pattern leaves of a temporary drops at the end of its statement.
		_ => {
			let InEnum { after, .. } = in_enum(0);
			log.borrow_mut().push(MARK);
			drop(after);
		}
	}
	out.extend(log.borrow().iter());
}

/// An `if let` that moves `A`'s value out of a temporary `make` makes, picked by `pick`, of
/// a `Drop` struct, an `Option` of one, and a plain struct holding one, whose rest drops
/// at the end of the statement.
pub fn if_lets(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = RefCell::new(Vec::new());
	let pick = data.first().copied().unwrap_or(0);
	let l = |id: u8| Logged(id, &log);
	fn bind<'a, T>(make: impl Fn() -> Option<Picked<'a, T>>, log: &RefCell<Vec<u8>>) {
		if let Some(Picked::A(x)) = make() {
			log.borrow_mut().push(MARK);
			drop(x);
		}
		log.borrow_mut().push(MARK);
	}
	fn picked<'a, T>(pick: u8, a: Logged<'a>, make: impl Fn() -> T) -> Option<Picked<'a, T>> {
		match pick % 5 {
			0 => None,
			1 => Some(Picked::A(a)),
			2 => Some(Picked::B(make())),
			3 => Some(Picked::C(Some(make()))),
			_ => Some(Picked::C(None)),
		}
	}
	match kind % 3 {
		0 => bind(|| picked(pick, l(0), || l(1)), &log),
		1 => bind(|| picked(pick, l(0), || Some(l(2))), &log),
		_ => bind(|| picked(pick, l(0), || Plain { u: l(3), n: pick }), &log),
	}
	out.extend(log.borrow().iter());
}
