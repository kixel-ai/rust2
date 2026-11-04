//! `==` and `!=` of a `Result` and of std's errors: every pair of a type's values at its
//! edges and of the input, so `Ok` meets `Err` and each error kind meets every other. A
//! `dyn Error` of the crate's errors and std's, borrowed, boxed by `?`, and upcast from the
//! crate's own trait, written by its `Display` and `Debug`, walked by `source`, downcast to
//! the type it was made of, and dropping what it owns as its box drops.

use std::cell::Cell;
use std::collections::{HashSet, TryReserveError, VecDeque};
use std::convert::Infallible;
use std::error::Error;
use std::fmt;
use std::num::{IntErrorKind, NonZero, ParseIntError, TryFromIntError};
use std::str::FromStr;
use std::time::Duration;

use crate::harness::put;

/// Strings that parse into one of the types, or fail each way it can: empty, an invalid
/// digit, past either end, zero, more than one char, a sign alone, and NaN.
const EDGES: &[&str] = &[
	"",
	"0",
	"-0",
	"1",
	"-1",
	"+",
	"a",
	"ab",
	"\u{1f600}",
	"true",
	"false",
	"127",
	"128",
	"-128",
	"-129",
	"256",
	"18446744073709551615",
	"18446744073709551616",
	"-170141183460469231731687303715884105729",
	"1.5",
	"1e400",
	"NaN",
	"inf",
];

/// Bytes that are UTF-8, or fail at each place and in each way it can.
const BYTES: &[&[u8]] = &[
	b"",
	b"a",
	b"\xff",
	b"a\xff",
	b"\xc3",
	b"a\xc3",
	b"\xe0\x80",
	b"\xf0\x9f\x98",
	b"\xf0\x9f\x98\x80",
	b"\xed\xa0\x80",
];

/// Each pair's `==` and `!=`, a bit each.
fn pairs<T: PartialEq>(values: &[T], out: &mut Vec<u8>) {
	for a in values {
		for b in values {
			out.push(u8::from(a == b) | (u8::from(a != b) << 1));
		}
	}
}

fn parsed<T: FromStr>(all: &[&str]) -> Vec<Result<T, T::Err>> {
	all.iter().map(|s| s.parse()).collect()
}

/// The values of the type `kind` picks, from the edges and the input as text of digits,
/// signs, and letters, or as bytes.
pub fn eqs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	const ALPHABET: &[char] = &[
		'0', '1', '2', '5', '9', '+', '-', '.', 'e', 'a', 'b', 'N', 't', 'f', '\u{e9}',
	];
	let mut input = String::new();
	for &b in data {
		input.push(ALPHABET[usize::from(b) % ALPHABET.len()]);
	}
	let mut all = EDGES.to_vec();
	all.push(&input);
	let mut bytes = BYTES.to_vec();
	bytes.push(data);
	match kind % 16 {
		0 => pairs(&parsed::<bool>(&all), out),
		// `EmptyString` and `TooManyChars`, private but for `==`, `Display`, and `Debug`.
		1 => pairs(&parsed::<char>(&all), out),
		2 => pairs(&parsed::<u8>(&all), out),
		3 => pairs(&parsed::<i8>(&all), out),
		4 => pairs(&parsed::<NonZero<u32>>(&all), out),
		5 => pairs(&parsed::<u64>(&all), out),
		6 => pairs(&parsed::<i128>(&all), out),
		// A NaN is not equal to itself, and `-0.0` is to `0.0`.
		7 => pairs(&parsed::<f64>(&all), out),
		// `PosOverflow` and `NegOverflow`.
		8 => {
			let mut ints = vec![0, 127, 128, -128, -129, i64::MAX, i64::MIN];
			ints.extend(data.iter().map(|&b| i64::from(b as i8) * 2));
			let v: Vec<Result<i8, TryFromIntError>> = ints.into_iter().map(i8::try_from).collect();
			pairs(&v, out);
		}
		9 => pairs(&bytes.iter().map(|b| std::str::from_utf8(b)).collect::<Vec<_>>(), out),
		10 => pairs(
			&bytes.iter().map(|b| String::from_utf8(b.to_vec())).collect::<Vec<_>>(),
			out,
		),
		// `Negative` and `OverflowOrNan`.
		11 => {
			let mut secs = vec![0.0, -0.0, 1.5, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1e30];
			secs.extend(data.iter().map(|&b| f64::from(b as i8) / 4.0));
			pairs(
				&secs.into_iter().map(Duration::try_from_secs_f64).collect::<Vec<_>>(),
				out,
			);
		}
		// What a `Result` holds may be an `Option` or another `Result`.
		12 => {
			let v: Vec<Result<Option<u8>, ParseIntError>> =
				all.iter().map(|s| s.parse::<u8>().map(|v| v.checked_sub(1))).collect();
			pairs(&v, out);
			let v: Vec<Result<Result<char, _>, _>> = all
				.iter()
				.map(|s| s.parse::<bool>().map(|_| s.parse::<char>()))
				.collect();
			pairs(&v, out);
			let v: Vec<Result<u8, IntErrorKind>> = all
				.iter()
				.map(|s| s.parse::<u8>().map_err(|e| e.kind().clone()))
				.collect();
			pairs(&v, out);
		}
		// One that can't fail, and one of `()`.
		13 => {
			let v: Vec<Result<u16, Infallible>> = data.iter().map(|&b| Ok(u16::from(b))).collect();
			pairs(&v, out);
			let v: Vec<Result<(), std::fmt::Error>> = data
				.iter()
				.map(|&b| if b % 2 == 0 { Ok(()) } else { Err(std::fmt::Error) })
				.collect();
			pairs(&v, out);
		}
		// `Ok`, or a capacity past what each collection holds, and the error's clone.
		14 => {
			let v: Vec<Result<(), TryReserveError>> = data
				.iter()
				.take(6)
				.flat_map(|&b| {
					let n = if b % 2 == 0 { usize::from(b) } else { usize::MAX - usize::from(b) };
					let mut text = String::from(&"ab"[..usize::from(b % 3)]);
					[
						Vec::<u8>::new().try_reserve(n),
						text.try_reserve_exact(n),
						VecDeque::<u16>::new().try_reserve(n),
						HashSet::<u8>::new().try_reserve(n),
					]
				})
				.collect();
			pairs(&v.iter().filter_map(|r| r.clone().err()).collect::<Vec<_>>(), out);
			pairs(&[v.clone(), v].concat(), out);
		}
		// Held in an `Option`, a `Vec`, and a tuple, and the errors on their own.
		_ => {
			let v: Vec<Option<Result<u8, ParseIntError>>> =
				all.iter().map(|s| (!s.is_empty()).then(|| s.parse())).collect();
			pairs(&v, out);
			let v: Vec<Vec<Result<bool, _>>> = all.chunks(2).map(parsed).collect();
			pairs(&v, out);
			let v: Vec<(Result<char, _>, usize)> = all.iter().map(|s| (s.parse(), s.len() % 2)).collect();
			pairs(&v, out);
			pairs(
				&all.iter().filter_map(|s| s.parse::<char>().err()).collect::<Vec<_>>(),
				out,
			);
			pairs(
				&all.iter().filter_map(|s| s.parse::<bool>().err()).collect::<Vec<_>>(),
				out,
			);
			pairs(
				&all.iter().filter_map(|s| s.parse::<i8>().err()).collect::<Vec<_>>(),
				out,
			);
		}
	}
}

thread_local! {
	static DROPPED: Cell<u32> = const { Cell::new(0) };
}

/// The crate's error without a source of its own, which counts its drops.
#[derive(Debug)]
struct Leaf {
	code: u8,
}

impl fmt::Display for Leaf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "leaf {}", self.code)
	}
}

impl Error for Leaf {}

impl Drop for Leaf {
	fn drop(&mut self) {
		DROPPED.set(DROPPED.get().wrapping_mul(10).wrapping_add(u32::from(self.code % 10)));
	}
}

/// An error whose source is the crate's, std's, or none.
#[derive(Debug)]
enum Failure {
	Short(u8),
	Wrapped(Leaf),
	Parse(ParseIntError),
}

impl fmt::Display for Failure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Failure::Short(n) => write!(f, "short by {n}"),
			Failure::Wrapped(_) => f.write_str("wrapped"),
			Failure::Parse(_) => f.write_str("unparsed"),
		}
	}
}

impl Error for Failure {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Failure::Short(_) => None,
			Failure::Wrapped(leaf) => Some(leaf),
			Failure::Parse(e) => Some(e),
		}
	}
}

/// The crate's own trait over `Error`, as moq's transports' errors are.
trait Coded: Error {
	fn code(&self) -> u8;
}

impl Coded for Failure {
	fn code(&self) -> u8 {
		match self {
			Failure::Short(n) => *n,
			Failure::Wrapped(leaf) => leaf.code,
			Failure::Parse(_) => 0,
		}
	}
}

/// A boxed `Coded`, forwarding `Error` to it, as moq's boxed transport error does.
#[derive(Debug)]
struct Transported(Box<dyn Coded>);

impl fmt::Display for Transported {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl Error for Transported {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		self.0.source()
	}
}

/// Each error in the chain, by `Display` and `Debug`.
fn chain(e: &dyn Error) -> String {
	let mut s = format!("{e} {e:?}");
	let mut cur = e.source();
	while let Some(c) = cur {
		s += &format!(" <- {c} {c:?}");
		cur = c.source();
	}
	s
}

fn failure(kind: u8, n: u8) -> Failure {
	match kind % 3 {
		0 => Failure::Short(n),
		1 => Failure::Wrapped(Leaf { code: n }),
		_ => Failure::Parse(u8::from_str(["", "x", "999"][usize::from(n % 3)]).unwrap_err()),
	}
}

/// `?` boxes a `Failure` as a `dyn Error`, as does `into`.
fn checked(kind: u8, n: u8) -> Result<u8, Box<dyn Error + Send + Sync>> {
	if n % 4 == 0 {
		return Err(failure(kind, n).into());
	}
	let parsed: u8 = u8::from_str(&format!("{n}{n}")).map_err(Failure::Parse)?;
	Ok(parsed)
}

/// `?` boxes the error of a `Vec` reserving past the most `u64`s it holds.
fn reserve(n: u8) -> Result<(), Box<dyn Error>> {
	Vec::<u64>::new().try_reserve(isize::MAX as usize / 8 + 1 + usize::from(n))?;
	Ok(())
}

pub fn objects(data: &[u8], out: &mut Vec<u8>) {
	let [a, b, rest @ ..] = data else {
		return;
	};
	let (a, b) = (*a, *b);

	// Borrowed, of the crate's and std's errors.
	put(
		out,
		&format!("{} | {} | ", chain(&failure(a, b)), chain(&Leaf { code: a })),
	);
	let parse = u8::from_str("x").unwrap_err();
	put(out, &format!("{} | ", chain(&parse)));
	let reserved = reserve(a).unwrap_err();
	put(
		out,
		&format!("{} {reserved:#?} {} | ", chain(&*reserved), reserved.is::<TryReserveError>()),
	);
	// A box and a reference are errors too, whose `source` is what they point at's.
	let held = Box::new(failure(a, b));
	let lent = &*held;
	put(out, &format!("{} | {} | ", chain(&held), chain(&lent)));

	// Through the crate's trait, boxed, and upcast to `dyn Error`.
	let coded: Box<dyn Coded> = Box::new(failure(b, a));
	let up: &dyn Error = &*coded;
	put(out, &format!("{} {} | ", coded.code(), chain(up)));
	let boxed = Transported(coded);
	put(out, &format!("{} {:#?} | ", chain(&boxed), boxed));

	// Boxed by `?` and `into`, then dropped: the box drops what it owns.
	DROPPED.set(0);
	let results: Vec<_> = rest.iter().map(|&n| checked(a.wrapping_add(n), n)).collect();
	for r in &results {
		match r {
			Ok(n) => put(out, &format!("{n} ")),
			Err(e) => put(out, &format!("{} ", chain(&**e))),
		}
	}
	drop(results);
	put(out, &format!("| {} ", DROPPED.get()));

	// A box of a reference drops nothing; the leaf drops with its owner.
	DROPPED.set(0);
	let leaf = Leaf { code: b };
	let borrowed: Box<dyn Error + '_> = (&leaf).into();
	put(out, &format!("{} ", chain(&*borrowed)));
	drop(borrowed);
	put(out, &format!("{} ", DROPPED.get()));
	drop(leaf);
	put(out, &format!("{} | ", DROPPED.get()));

	// Downcast, of the crate's errors and std's, and of one upcast from the crate's trait,
	// whose value is still a `Failure`.
	let coded: Box<dyn Coded> = Box::new(failure(a, b));
	put(out, &format!("{} ", downcast(&*coded)));
	let errors: Vec<Box<dyn Error>> = vec![
		coded,
		Box::new(Leaf { code: a }),
		Box::new(u8::from_str("x").unwrap_err()),
	];
	for e in &errors {
		put(out, &format!("{} ", downcast(&**e)));
	}
	// Changed where it is.
	let mut leaf = Leaf { code: a };
	let e: &mut dyn Error = &mut leaf;
	if let Some(l) = e.downcast_mut::<Leaf>() {
		l.code = l.code.wrapping_add(b);
	}
	put(out, &format!("{} ", leaf.code));
	// A box's downcast gives its value or the box back, which each drop once.
	DROPPED.set(0);
	for e in errors {
		match e.downcast::<Failure>() {
			Ok(f) => put(out, &format!("failure {} {} ", f.code(), DROPPED.get())),
			Err(e) => match e.downcast::<Leaf>() {
				Ok(l) => put(out, &format!("leaf {} {} ", l.code, DROPPED.get())),
				Err(e) => put(out, &format!("{e} ")),
			},
		}
	}
	put(out, &format!("{} ", DROPPED.get()));
	let sent = checked(b, a.wrapping_mul(4)).map_err(|e| e.downcast::<Failure>().map(|f| f.code()));
	put(out, &format!("{:?}", sent.map_err(|e| e.map_err(|e| e.to_string()))));
}

/// Which of the crate's errors and std's `e` and its source are, and the `Failure` it holds.
fn downcast(e: &(dyn Error + 'static)) -> String {
	let is = (e.is::<Failure>(), e.is::<Leaf>(), e.is::<ParseIntError>());
	let source = e.source().map(|s| {
		(
			s.is::<Leaf>(),
			s.downcast_ref::<ParseIntError>().map(|p| p.kind().clone()),
		)
	});
	format!("{is:?} {source:?} {:?}", e.downcast_ref::<Failure>().map(|f| f.code()))
}
