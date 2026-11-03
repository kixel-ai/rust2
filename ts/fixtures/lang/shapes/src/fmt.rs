//! Formatting: `format!` and `write!` with every option, derived and written `Debug` and
//! `Display`, and panics that format their messages. A `Box`, `Arc`, or `Rc` of a
//! `dyn Debug` or `dyn Display`, written with options, dropping what it owns as the last
//! pointer drops.

#![allow(dead_code, reason = "fields only `Debug` reads")]

use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{self, Debug, Display};
use std::ops::{Range, RangeToInclusive};
use std::rc::Rc;
use std::sync::Arc;

use crate::generics::Wrap;
use crate::harness::put;
use crate::shapes::{Point, Shape};

/// Take an integer's big-endian bytes from the front of `data`, zero-padded.
macro_rules! take {
	($t:ty, $data:expr) => {{
		let data: &[u8] = $data;
		let mut buf = [0u8; core::mem::size_of::<$t>()];
		let n = data.len().min(buf.len());
		buf[..n].copy_from_slice(&data[..n]);
		<$t>::from_be_bytes(buf)
	}};
}

/// Every option on one integer: width, fill, alignment, signs, zeros, precision, and
/// each radix and exponent.
macro_rules! specs {
	($t:ty, $data:expr, $out:expr) => {{
		let v = take!($t, $data);
		put(
			$out,
			&format!(
				"{v} {v:?} {v:5}|{v:<5}|{v:^7}|{v:>4}|{v:*^9}|{v:é>6}|{v:05}|{v:<05}|{v:x} {v:X} {v:#x} {v:#X} {v:08x} {v:#010x} {v:>8x}|{v:<#6X}|{v:#?} {v:3?}"
			),
		);
		put(
			$out,
			&format!(
				"{v:+} {v:+06} {v:+#x} {v:-} {v:.3} {v:8.2}|{v:b} {v:#b} {v:#012b} {v:o} {v:#o} {v:+o}|{v:x?} {v:#X?} {v:06x?} {v:+?}"
			),
		);
		put(
			$out,
			&format!("{v:e} {v:E} {v:.0e} {v:.1e} {v:.3e} {v:.45E} {v:+.2e} {v:012.1e} {v:<+9e}| {v:#e} {v:>12.4E}|"),
		);
		put($out, &v.to_string());
	}};
}

/// Integers of every width, with each spec.
pub fn ints(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 12 {
		0 => specs!(u8, data, out),
		1 => specs!(u16, data, out),
		2 => specs!(u32, data, out),
		3 => specs!(u64, data, out),
		4 => specs!(u128, data, out),
		5 => specs!(usize, data, out),
		6 => specs!(i8, data, out),
		7 => specs!(i16, data, out),
		8 => specs!(i32, data, out),
		9 => specs!(i64, data, out),
		10 => specs!(i128, data, out),
		_ => specs!(isize, data, out),
	}
}

/// Strings that `Debug` escapes, or that pad by chars rather than bytes.
const STRINGS: [&str; 14] = [
	"",
	"a",
	"héllo",
	"日本語",
	"tab\tnew\nline\r",
	"quote\"back\\slash'",
	"\0\x01\x1f\x7f",
	"\u{80}\u{9f}\u{a0}\u{ad}",
	"e\u{301}",
	"\u{200d}\u{feff}\u{115f}\u{2028}\u{3000}",
	// Unicode 18, newer than an engine may know, assigns U+3D000 but not U+3FC40.
	"\u{e000}\u{378}\u{10fffd}\u{3d000}\u{3fc40}",
	"😀 \u{1f1e6}",
	"\u{ff9e}\u{ff9f}\u{ff9d}",
	"{}",
];

/// Strings padded and escaped, and bytes that may not be UTF-8.
pub fn strings(data: &[u8], out: &mut Vec<u8>) {
	for &b in &data[..data.len().min(3)] {
		let s = STRINGS[b as usize % STRINGS.len()];
		put(
			out,
			&format!("{s}|{s:8}|{s:>8}|{s:^8}|{s:-<3}|{s:?}|{s:12?}|{s:#?}|{{{s}}}"),
		);
		// A precision cuts by chars, then the width pads; `Debug` of a string takes neither.
		put(
			out,
			&format!("{s:.2}|{s:8.3}|{s:>6.1}|{s:_^7.2}|{s:.0}|{s:.20}|{s:.2?}|{s:10.2?}|{s:+}"),
		);
	}
	let c = char::from_u32(u32::from(data.first().copied().unwrap_or(0)) * 0x101).unwrap_or('?');
	put(out, &format!("{c}|{c:.0}|{c:3.1}|{c:.0?}|{c:>4?}"));
	match String::from_utf8(data.to_vec()) {
		Ok(s) => put(out, &format!("{s:?} {s:>4} {s:.1}")),
		Err(e) => put(out, &format!("{e} {e:?} {e:#?} {e:>80} {e:.4} {e:x?} {e:#X?}")),
	}
	let flag = matches!(data.first(), Some(b) if b % 2 == 0);
	put(
		out,
		&format!(
			"{flag} {flag:?} {flag:>6} {flag:.2} {flag:>4.1?} {:?} {:#?} {:.1?}",
			(),
			(),
			()
		),
	);
	put(out, &flag.to_string());
	match u8::try_from(data.len() as i32 - 3) {
		Ok(v) => put(out, &format!("{v}")),
		Err(e) => put(out, &format!("{e} {e:?} {e:>40}| {e:.6}|{e:10.3}|")),
	}
}

#[derive(Debug)]
pub struct Unit;

#[derive(Debug, PartialEq)]
pub struct Braced {}

#[derive(Debug, PartialEq)]
pub struct Bare();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pair(pub u8, pub i64);

/// Past five fields, derived `Debug` takes the fields as slices.
#[derive(Debug)]
pub struct Wide {
	a: u8,
	b: u16,
	c: u32,
	d: u64,
	e: i8,
	f: bool,
}

#[derive(Debug)]
pub struct Six(u8, u8, u8, u8, u8, u8);

#[derive(Debug)]
pub enum Kind {
	Empty,
	Braced {},
	Bare(),
	One(u8),
	Many { count: u32, items: Vec<u8> },
}

#[derive(Debug)]
pub struct Nested {
	name: String,
	point: Point,
	shape: Shape,
	tags: Vec<u8>,
	pairs: Vec<Pair>,
	maybe: Option<Pair>,
	none: Option<u8>,
	result: Result<u8, String>,
	list: [i16; 2],
	tuple: (u8, bool, &'static str),
	single: (u8,),
	empty: Vec<u8>,
	unit: (),
	boxed: Box<Kind>,
}

fn nested(data: &[u8]) -> Nested {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	Nested {
		name: STRINGS[at(0) as usize % STRINGS.len()].to_string(),
		point: Point {
			x: at(1) as i32 - 128,
			y: -7,
		},
		shape: match at(2) % 3 {
			0 => Shape::Dot,
			1 => Shape::Line(Point { x: 1, y: 2 }, Point { x: -3, y: 4 }),
			_ => Shape::Rect {
				corner: Point { x: 0, y: 0 },
				width: at(3) as u16,
				height: 9,
			},
		},
		tags: data[..data.len().min(4)].to_vec(),
		pairs: [Pair(at(4), -1), Pair(2, i64::MIN)].to_vec(),
		maybe: if at(5) % 2 == 0 { Some(Pair(at(5), 5)) } else { None },
		none: None,
		result: if at(6) % 2 == 0 {
			Ok(at(6))
		} else {
			Err("no".to_string())
		},
		list: [at(7) as i16 - 100, i16::MAX],
		tuple: (at(8), true, "t"),
		single: (at(9),),
		empty: Vec::new(),
		unit: (),
		boxed: Box::new(match at(10) % 3 {
			0 => Kind::Empty,
			1 => Kind::One(at(11)),
			_ => Kind::Many {
				count: 2,
				items: [at(11), at(12)].to_vec(),
			},
		}),
	}
}

/// Derived `Debug` of every shape, compact and pretty, with options passed to fields.
pub fn derived(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let value = nested(data);
	put(out, &format!("{value:?}"));
	put(out, &format!("{value:#?}"));
	// Each field takes the options: hex for integers, and the precision for strings.
	put(out, &format!("{value:x?}"));
	put(out, &format!("{value:#X?}"));
	put(out, &format!("{value:+.1?}"));
	let wide = Wide {
		a: at(0),
		b: 300,
		c: 70_000,
		d: u64::MAX,
		e: -1,
		f: at(1) > 10,
	};
	let six = Six(at(0), 1, 2, 3, 4, at(1));
	put(out, &format!("{wide:?} {six:?} {:?} {:#?}", Unit, Unit));
	put(out, &format!("{wide:#?} {six:#?}"));
	let point = Point { x: at(2) as i32, y: -1 };
	put(
		out,
		&format!("{point:5?}|{point:<4?}|{point:#4?}|{:>3?}", Pair(at(3), -9)),
	);
	let kinds = [
		Kind::Empty,
		Kind::One(at(4)),
		Kind::Braced {},
		Kind::Bare(),
		Kind::Many {
			count: 0,
			items: Vec::new(),
		},
	];
	put(out, &format!("{kinds:?} {:?}", &&kinds[1]));
	put(out, &format!("{kinds:#?}"));
	let bytes: &[u8] = data;
	let words = [at(5) as u16, 0xffff];
	put(
		out,
		&format!("{bytes:?} {words:?} {:?} {:?}", data.first(), data.get(100)),
	);
	put(
		out,
		&format!("{:?} {:#?} {:?} {:#?}", Braced {}, Braced {}, Bare(), Bare()),
	);
	out.extend_from_slice(&[(Braced {} == Braced {}) as u8, (Bare() == Bare()) as u8]);
}

/// `Display` written with `write!`, which ignores the outer options, unless `#`.
pub struct Hex(pub u32);

/// Through to the field, which takes the options.
impl fmt::Binary for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Binary::fmt(&self.0, f)
	}
}

impl fmt::Octal for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:o}", self.0)
	}
}

impl fmt::LowerExp for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::LowerExp::fmt(&self.0, f)
	}
}

impl fmt::UpperExp for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::UpperExp::fmt(&self.0, f)
	}
}

impl fmt::Display for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match f.alternate() {
			true => write!(f, "{:#x}", self.0),
			false => write!(f, "{:x}", self.0),
		}
	}
}

impl fmt::Debug for Hex {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

/// `Display` through `pad`, which keeps the options.
pub struct Padded(&'static str);

impl fmt::Display for Padded {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match f.width() {
			Some(width) if width > 20 => f.write_str("wide"),
			_ => f.pad(self.0),
		}
	}
}

/// `Display` that fails after writing, for odd values.
pub struct Fails(pub u8);

impl fmt::Display for Fails {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0 % 2 == 0 {
			return f.write_str("fine");
		}
		write!(f, "half {}", self.0)?;
		Err(fmt::Error)
	}
}

/// The builders, written out.
pub struct Manual {
	id: u8,
	list: Vec<u16>,
}

impl fmt::Debug for Manual {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Manual")
			.field("id", &self.id)
			.field("list", &self.list)
			.finish_non_exhaustive()
	}
}

pub struct Seq(Vec<Pair>);

impl fmt::Debug for Seq {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

pub struct Tup(u8, Option<u8>);

impl fmt::Debug for Tup {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.1 {
			Some(_) => f.debug_tuple("Tup").field(&self.0).field(&self.1).finish(),
			None => f.debug_tuple("Tup").field(&self.0).finish_non_exhaustive(),
		}
	}
}

/// `v`, logged, so an argument evaluated more than once shows.
fn note(log: &mut Vec<u8>, v: u8) -> u8 {
	log.push(v);
	v
}

/// `Display` through `write!` of arguments with effects.
pub struct Noted(pub u8);

impl fmt::Display for Noted {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut log = Vec::new();
		write!(
			f,
			"{} {:?} {log:?}",
			note(&mut log, self.0),
			note(&mut log, 1) == self.0
		)
	}
}

/// Arguments that read `log` before and after one changes it, through a reference.
fn logged(log: &mut Vec<u8>, data: &[u8]) -> String {
	format!(
		"{} {} {} {}",
		log.len(),
		note(log, data[0]),
		data[log.len() % data.len()],
		log.len()
	)
}

/// Written `Display` and `Debug`, and a `Display` that fails inside `format!`.
pub fn written(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let hex = Hex(u32::from(at(0)) * 0x1001);
	put(out, &format!("{hex}|{hex:>12}|{hex:#}|{hex:?}|{hex:>6?}|{hex:#?}"));
	// `Debug` through `Display` is decimal under `{:x?}` too.
	put(
		out,
		&format!("{hex:x?}|{hex:#b}|{hex:>8o}|{hex:.2e}|{hex:E}|{hex:+.1?}"),
	);
	put(out, &hex.to_string());
	let padded = Padded("pad");
	put(out, &format!("{padded}|{padded:>6}|{padded:_^9}|{padded:30}|"));
	let manual = Manual {
		id: at(1),
		list: [1, u16::from(at(2))].to_vec(),
	};
	let empty = Manual {
		id: 0,
		list: Vec::new(),
	};
	put(out, &format!("{manual:?} {manual:#?} {empty:?}"));
	let seq = Seq([Pair(at(3), 0), Pair(1, -1)].to_vec());
	let none = Seq(Vec::new());
	put(out, &format!("{seq:?} {seq:#?} {none:?} {none:#?}"));
	let tup = Tup(at(4), if at(5) % 2 == 0 { Some(at(5)) } else { None });
	put(out, &format!("{tup:?} {tup:#?}"));
	// The first argument is evaluated before the second panics.
	let fine = Fails(at(6) & !1);
	put(out, &format!("{fine} {fine:>9}"));
	put(
		out,
		&format!("{1} {0} {0:?} {name} {name:>5}", at(7), at(8), name = "named"),
	);
	// Each argument is evaluated once, in order, however many placeholders read it.
	let mut log = Vec::new();
	let first = format!(
		"{1} {0} {0:?} {2} {}",
		note(&mut log, at(9)),
		note(&mut log, at(10)) == 3,
		log.len()
	);
	put(out, &first);
	put(out, &format!("{}|{:>6}", Noted(at(11)), Noted(2)));
	if !data.is_empty() {
		put(out, &logged(&mut log, data));
	}
	// One argument, read by its one placeholder, is evaluated once, where `format!` is.
	let once = format!("{:?}", note(&mut log, at(13)));
	put(out, &format!("{} {}", once, log.len()));
	out.extend_from_slice(&log);
	if at(12) > 250 {
		panic!("{} {}", note(&mut log, at(12)), log.len());
	}
}

/// Widths and precisions from arguments, by position and by name, and one past
/// `u16::MAX`, which panics.
pub fn counts(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (w, p) = (usize::from(at(0) % 24), usize::from(at(1) % 12));
	let v = take!(i64, &data[data.len().min(2)..]);
	let s = STRINGS[usize::from(at(3)) % STRINGS.len()];
	put(
		out,
		&format!(
			"{v:w$}|{v:<w$e}|{v:+0w$.p$e}|{s:.p$}|{s:^w$.p$}|{:.*}|{2:>0$}|{1:2$.0$}|{s:>p$?}",
			p, s, w
		),
	);
	// Each argument is evaluated once, before any is written, even a count.
	let mut log = Vec::new();
	put(
		out,
		&format!("{:>1$}|{:.*}", note(&mut log, at(4)), usize::from(note(&mut log, 6)), s),
	);
	out.extend_from_slice(&log);
	let big = usize::from(u16::MAX) + usize::from(at(5) % 2);
	put(out, &format!("{s:.big$}|"));
}

/// `{:e}` at each rounding: below, above, and ties to even, which may carry a digit.
pub fn rounding(out: &mut Vec<u8>) {
	for v in [0u32, 5, 15, 25, 125, 135, 994, 995, 999, 1000, 1_250_000, 4_294_967_295] {
		put(out, &format!("{v:.0e} {v:.1e} {v:.2e} {v:e}"));
	}
	put(out, &format!("{:.0e} {:e} {:.3E}", u128::MAX, i128::MIN, i64::MIN));
}

/// Panics, each with the message native Rust formats.
pub fn panics(kind: u8, rest: &[u8], out: &mut Vec<u8>) {
	let n = rest.first().copied().unwrap_or(0);
	match kind % 20 {
		0 => panic!("bad {n}: {:?}", rest),
		1 => unreachable!("never {n:#x} {}", Hex(n.into())),
		2 => assert!(n > 200, "small {} {:?}", n, Pair(n, 0)),
		3 => assert!(n > 200),
		4 => assert_eq!(n, 7),
		5 => assert_ne!(Pair(n, 1), Pair(n, 1), "same {}", n),
		6 => assert_eq!(n, 9, "with {}", "context"),
		7 => {
			let r: Result<u8, Kind> = match n % 2 {
				0 => Err(Kind::One(n)),
				_ => Err(Kind::Many {
					count: 1,
					items: [n].to_vec(),
				}),
			};
			put(out, &r.unwrap().to_string());
		}
		8 => {
			let r: Result<u8, Pair> = if n > 128 { Ok(n) } else { Err(Pair(n, -2)) };
			put(out, &r.expect("expected a byte").to_string());
		}
		9 => {
			let r: Result<Point, u8> = Ok(Point { x: n as i32, y: 0 });
			put(out, &r.unwrap_err().to_string());
		}
		10 => {
			let missing = rest.get(200).copied();
			put(out, &missing.expect(&format!("missing {n}")).to_string());
		}
		11 => put(out, &format!("{}", Fails(n))),
		12 => put(out, &Fails(n).to_string()),
		13 => panic!("{} then {}", Hex(n.into()), Fails(n)),
		14 => {
			let (start, end) = ((n % 8) as usize, (n / 32) as usize);
			put(out, &format!("{:?} {:?}", &rest[start..], &rest[start..end]));
		}
		15 => {
			let mut buf = [0u8; 3];
			buf.copy_from_slice(&rest[..rest.len().min(4)]);
			put(out, &format!("{buf:?}"));
		}
		16 => {
			let r: Result<u8, std::string::FromUtf8Error> = match String::from_utf8(rest.to_vec()) {
				Ok(s) => Ok(s.len() as u8),
				Err(e) => Err(e),
			};
			put(out, &r.unwrap().to_string());
		}
		17 => {
			let mut buf = [1u8, 2, 3, 4];
			let dest = rest.get(1).copied().unwrap_or(0) as usize % 4;
			buf.copy_within((n % 6) as usize..(n / 6 % 6) as usize, dest);
			put(out, &format!("{buf:?}"));
		}
		18 => put(out, &rest[(n % 8) as usize].to_string()),
		_ => {
			let r: Result<u8, u8> = if n % 2 == 0 { Err(n) } else { Ok(n) };
			put(out, &format!("{}", r.unwrap_err()));
		}
	}
}

/// `Debug` that pads its text, so a width and precision passed to it show, and marks `#`.
#[derive(Clone, Copy)]
pub struct Clip(&'static str);

impl fmt::Debug for Clip {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			f.write_str("#")?;
		}
		f.pad(self.0)
	}
}

#[derive(Debug)]
pub struct Shared {
	arc: Arc<Pair>,
	span: Range<u16>,
	upto: RangeToInclusive<u8>,
}

/// std's ranges' `Debug`, which passes its options to each bound, and an `Arc`'s `Debug`
/// and `Display`, which pass them to what it holds.
pub fn wrapped(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (a, b) = (i32::from(at(0)) - 128, i32::from(at(1)) - 100);
	let w = usize::from(at(2) % 10);
	put(
		out,
		&format!("{:?} {:?} {:?} {:?} {:?} {:?}", a..b, a..=b, a.., ..b, ..=b, ..),
	);
	put(
		out,
		&format!(
			"{:5?}|{:<4?}|{:+?}|{:05?}|{:x?}|{:#X?}|{:w$?}|{:^w$?}|{:>3?}|{:#?}",
			a..b,
			a..=b,
			a..,
			..b,
			..=b,
			a..b,
			a..=b,
			..=b,
			..,
			a..b
		),
	);
	let lo = Clip(STRINGS[usize::from(at(3)) % STRINGS.len()]);
	let hi = Clip(STRINGS[usize::from(at(4)) % STRINGS.len()]);
	put(
		out,
		&format!(
			"{:?}|{:8.2?}|{:#?}|{:_>w$.1?}|{:#4?}",
			lo..hi,
			lo..=hi,
			..hi,
			lo..,
			..=lo
		),
	);
	let (x, y) = (u32::from(at(5)) * 0x101, u32::from(at(6)));
	put(
		out,
		&format!("{:?}|{:>9?}|{:#?}", Hex(x)..Hex(y), Hex(x)..=Hex(y), ..Hex(y)),
	);
	let pairs = Pair(at(7), -1)..Pair(1, i64::MIN);
	put(out, &format!("{pairs:?}|{pairs:#?}|{pairs:x?}|{pairs:+?}"));
	// Iteration leaves an inclusive range in the state its `Debug` prints.
	let mut drained = (at(8) % 4)..=(at(9) % 4);
	while drained.next().is_some() {}
	let mut top = at(8).max(250)..=255;
	while top.next().is_some() {}
	let mut back = 0..=(at(9) % 3);
	while back.next_back().is_some() {}
	let mut far = at(8)..=at(9);
	far.nth(usize::from(at(10) % 8));
	put(out, &format!("{drained:?} {top:?} {back:?} {far:?} {:?}", 0..=0));

	let arc = Arc::new(Hex(x));
	put(out, &format!("{arc}|{arc:>12}|{arc:#}|{arc:?}|{arc:>6?}|{arc:#?}"));
	put(out, &arc.to_string());
	let nested = Arc::new(Arc::new(a));
	put(
		out,
		&format!("{nested}|{nested:+05}|{nested:?}|{nested:x?}|{nested:#X?}|{nested:<6?}|{nested:^+w$}"),
	);
	let rc = Rc::new(STRINGS[usize::from(at(3)) % STRINGS.len()].to_string());
	put(out, &format!("{rc}|{rc:>8.2}|{rc:?}"));
	let list = Arc::new([Pair(at(7), 2), Pair(3, -4)].to_vec());
	put(out, &format!("{list:?} {list:#?}"));
	let held = Arc::new(a)..Arc::new(b);
	put(out, &format!("{held:?} {held:+04?} {:?}", Arc::new(lo)..Arc::new(hi)));
	let shared = Shared {
		arc: Arc::new(Pair(at(7), 5)),
		span: u16::from(at(8))..300,
		upto: ..=at(9),
	};
	put(out, &format!("{shared:?} {shared:#?}"));
	let t: Arc<str> = Arc::from(STRINGS[usize::from(at(3)) % STRINGS.len()]);
	put(out, &format!("{t} {t:?} {t:>9.2}"));
}

/// std's collections' `Debug`, which passes its options to each element: a deque from its
/// head however its buffer wraps, sets and maps in braces, nested, and pretty. A hash
/// collection holds at most one entry, since std leaves their order unspecified.
pub fn collections(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	// Each byte pushes or pops at either end, so the head wraps around the buffer.
	let mut q: VecDeque<Option<u8>> = VecDeque::with_capacity(4);
	for &b in data {
		match b % 4 {
			0 => q.push_back((b % 3 != 0).then_some(b)),
			1 => q.push_front((b % 3 != 0).then_some(b)),
			2 => {
				q.pop_back();
			}
			_ => {
				q.pop_front();
			}
		}
	}
	put(out, &format!("{q:?}|{q:#?}|{q:x?}|{q:>4?}"));
	// Pushed at the front, the head wraps to the buffer's end, and popping the back past
	// where it started leaves only what wrapped.
	let mut wrapped = VecDeque::with_capacity(4);
	wrapped.extend([Hex(1), Hex(2)]);
	wrapped.push_front(Hex(u32::from(at(0)) * 0x1001));
	wrapped.push_front(Hex(u32::from(at(1))));
	wrapped.pop_back();
	let mid = format!("{wrapped:?}");
	wrapped.pop_back();
	put(out, &format!("{mid}|{wrapped:?}|{wrapped:#?}|{wrapped:6?}"));
	let empty = VecDeque::<u8>::new();
	let units: VecDeque<()> = (0..at(2) % 3).map(|_| ()).collect();
	put(out, &format!("{empty:?}|{empty:#?}|{units:?}"));
	let nested: VecDeque<VecDeque<Pair>> = [VecDeque::new(), VecDeque::from([Pair(at(3), -1), Pair(2, 3)])].into();
	put(out, &format!("{nested:?}|{nested:#?}"));

	let sorted: BTreeSet<Option<u8>> = data.iter().map(|&b| (b % 3 != 0).then_some(b % 8)).collect();
	put(out, &format!("{sorted:?}|{sorted:#?}|{sorted:02x?}"));
	let mut map: BTreeMap<u8, VecDeque<Pair>> = BTreeMap::new();
	for &b in data.iter().take(6) {
		map.entry(b % 4).or_default().push_front(Pair(b, -i64::from(b)));
	}
	put(
		out,
		&format!(
			"{map:?}|{map:#?}|{:?}|{:#?}",
			BTreeMap::<u8, u8>::new(),
			BTreeSet::<u8>::new()
		),
	);
	let (mut one, mut set) = (HashMap::new(), HashSet::new());
	if let Some(&b) = data.first() {
		one.insert((b % 2 == 0).then_some(b), Hex(b.into()));
	}
	if let Some(&b) = data.get(1) {
		set.insert(b);
	}
	put(out, &format!("{one:?}|{one:#?}|{one:>3?}|{set:?}|{set:#?}|{set:#04x?}"));
}

/// `Debug` of std's enums beside its ranges and comparisons, each derived: a variant with
/// a field passes the options to it, and one without writes its name unpadded. A
/// `ControlFlow` that `?` would break with holds the `Err` or `None` it was applied to.
pub fn enums(data: &[u8], out: &mut Vec<u8>) {
	use std::cmp::{Ordering, Reverse};
	use std::convert::Infallible;
	use std::ops::{Bound, ControlFlow, RangeBounds};
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let w = usize::from(at(0) % 10);
	fn pick<T>(by: u8, x: T) -> Bound<T> {
		match by % 3 {
			0 => Bound::Included(x),
			1 => Bound::Excluded(x),
			_ => Bound::Unbounded,
		}
	}
	let n = pick(at(1), at(2));
	put(out, &format!("{n:?}|{n:#?}|{n:x?}|{n:#X?}|{n:>w$?}|{n:_^7?}|{n:+04?}"));
	let s = pick(at(3), STRINGS[usize::from(at(4)) % STRINGS.len()]);
	put(out, &format!("{s:?}|{s:#?}|{s:>9?}|{s:*<w$.2?}"));
	let c = pick(at(5), char::from(at(6)));
	put(out, &format!("{c:?}|{c:#?}|{c:^6?}"));
	let o = pick(at(7), (at(8) % 2 == 0).then_some(at(9)));
	put(out, &format!("{o:?}|{o:#?}|{o:x?}|{o:>4?}"));
	let bytes = pick(at(10), &data[..data.len().min(3)]);
	put(out, &format!("{bytes:?}|{bytes:#?}|{bytes:x?}|{bytes:3?}"));
	let clip = pick(at(11), Clip(STRINGS[usize::from(at(12)) % STRINGS.len()]));
	put(out, &format!("{:8.2?}|{:#?}|{:?}", clip.as_ref(), Some(clip), (n, s)));
	// A range's bounds borrow its own.
	let r = at(1)..=at(2);
	put(
		out,
		&format!(
			"{:?} {:?} {:?} {:x?} {:#?}",
			r.start_bound(),
			r.end_bound(),
			(at(1)..).end_bound(),
			(..at(2)).end_bound(),
			RangeBounds::<u8>::start_bound(&..)
		),
	);

	let ord = at(1).cmp(&at(2));
	let all = [Ordering::Less, Ordering::Equal, Ordering::Greater];
	put(out, &format!("{ord:?}|{ord:>8?}|{ord:#?}|{all:?}|{all:#?}"));
	let rev = Reverse(at(1));
	let nested = Reverse(Reverse(STRINGS[usize::from(at(4)) % STRINGS.len()]));
	put(
		out,
		&format!("{rev:?}|{rev:#?}|{rev:x?}|{rev:>w$?}|{nested:?}|{nested:#?}"),
	);

	let text = STRINGS[usize::from(at(4)) % STRINGS.len()];
	let flow = match at(1) % 2 {
		0 => ControlFlow::Break(text),
		_ => ControlFlow::Continue(u16::from(at(2)) * 300),
	};
	put(out, &format!("{flow:?}|{flow:#?}|{flow:x?}|{flow:>w$?}"));
	let unit: ControlFlow<u8> = match at(1) % 2 {
		0 => ControlFlow::Break(at(2)),
		_ => ControlFlow::Continue(()),
	};
	put(out, &format!("{unit:?}|{unit:#?}|{unit:03?}"));
	let erred: ControlFlow<Result<Infallible, u8>, u16> = match at(1) % 2 {
		0 => ControlFlow::Break(Err(at(2))),
		_ => ControlFlow::Continue(u16::from(at(2))),
	};
	put(out, &format!("{erred:?}|{erred:#?}|{erred:>5?}"));
	let none: ControlFlow<Option<Infallible>, u8> = match at(1) % 2 {
		0 => ControlFlow::Break(None),
		_ => ControlFlow::Continue(at(2)),
	};
	// `()` is `undefined`, so its `None` is `rt.None`.
	let unit_none: ControlFlow<Option<Infallible>> = match at(2) % 2 {
		0 => ControlFlow::Break(None),
		_ => ControlFlow::Continue(()),
	};
	put(
		out,
		&format!("{none:?}|{none:#?}|{none:<4?}|{unit_none:?}|{unit_none:#?}"),
	);
	let ok: Result<u8, Infallible> = Ok(at(2));
	put(out, &format!("{ok:?}|{ok:#?}"));
}

/// std's iterators' `Debug`, each the items it has left in its own wrapper, through each
/// element's `Debug` with the formatter's options: consumed from either end, empty, and
/// pretty. A hash iterator holds at most one entry, since std leaves their order unspecified.
pub fn iters(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (front, back) = (usize::from(at(0) % 3), usize::from(at(1) % 3));
	// Taken from the front by `nth`, and from the back by `next_back`.
	fn skip<I: Iterator>(mut it: I, n: usize) -> I {
		if n > 0 {
			it.nth(n - 1);
		}
		it
	}
	fn ends<I: DoubleEndedIterator>(it: I, front: usize, back: usize) -> I {
		let mut it = skip(it, front);
		for _ in 0..back {
			it.next_back();
		}
		it
	}

	let it = ends(data.iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}|{it:>3?}"));
	let mut v: Vec<u8> = data.iter().take(5).copied().collect();
	let it = ends(v.iter_mut(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let pairs: Vec<Pair> = data.iter().take(4).map(|&b| Pair(b, -i64::from(b))).collect();
	let it = ends(pairs.into_iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	put(out, &format!("{:x?}", ends(data.to_vec().into_iter(), front, back)));
	let mut hexes: Vec<Hex> = data.iter().take(5).map(|&b| Hex(u32::from(b) * 0x101)).collect();
	let it = ends(hexes.drain(1..), front, back);
	put(out, &format!("{it:?}|{it:#x?}"));
	drop(it);
	let mut bytes = data.to_vec();
	let it = ends(bytes.drain(..data.len() / 2), front, back);
	put(out, &format!("{it:?}|{it:02X?}"));
	drop(it);
	// Arrays of bytes, which a borrow lends as slices of them.
	let arrays: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	let it = ends(arrays.iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut arrays = arrays;
	put(out, &format!("{:?}", ends(arrays.iter_mut(), back, front)));
	let empty: &[Option<u8>] = &[];
	put(out, &format!("{:?}|{:#?}", empty.iter(), Vec::<u8>::new().into_iter()));

	let heap: BinaryHeap<u8> = data.iter().copied().collect();
	let it = ends(heap.iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	put(out, &format!("{:?}", ends(heap.clone().into_iter(), front, back)));
	let mut drained = heap.clone();
	let it = ends(drained.drain(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	drop(it);
	let q: VecDeque<Option<u8>> = data.iter().map(|&b| (b % 3 != 0).then_some(b)).collect();
	let it = ends(q.into_iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));

	// Text of one-, two-, three-, and four-byte chars, and chars `Debug` escapes.
	let text: String = data
		.iter()
		.map(|&b| ['a', 'é', '中', '🦀', '\n', '\''][usize::from(b % 6)])
		.collect();
	let it = ends(text.chars(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:>3?}"));
	let it = ends(text.bytes(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	put(out, &format!("{:?}|{:#?}|{:?}", "".chars(), "".chars(), "".bytes()));

	let mut map: BTreeMap<u8, Option<u8>> = data.iter().map(|&b| (b % 16, (b % 3 != 0).then_some(b))).collect();
	let it = ends(map.iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	put(
		out,
		&format!(
			"{:?}|{:?}",
			ends(map.keys(), front, back),
			ends(map.values(), front, back)
		),
	);
	let lo = at(2) % 16;
	put(
		out,
		&format!(
			"{:?}|{:?}",
			ends(map.range(lo..), front, back),
			ends(map.range(..=lo), front, back)
		),
	);
	put(out, &format!("{:?}", ends(map.iter_mut(), front, back)));
	put(out, &format!("{:?}", ends(map.values_mut(), front, back)));
	put(out, &format!("{:?}", ends(map.range_mut(lo..), front, back)));
	put(out, &format!("{:?}", ends(map.clone().into_iter(), front, back)));
	put(out, &format!("{:?}", ends(map.clone().into_keys(), front, back)));
	put(out, &format!("{:?}", ends(map.into_values(), front, back)));
	let keyed: BTreeMap<[u8; 2], Hex> = data.chunks_exact(2).map(|c| ([c[0], c[1]], Hex(c[0].into()))).collect();
	let it = ends(keyed.iter(), front, back);
	put(out, &format!("{it:?}|{it:#x?}|{:?}", ends(keyed.keys(), back, front)));
	let set: BTreeSet<u8> = data.iter().map(|&b| b % 16).collect();
	let it = ends(set.iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = ends(set.range(lo..), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	let it = ends(set.into_iter(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let empty = BTreeMap::<u8, u8>::new();
	put(out, &format!("{:?}|{:#?}", empty.iter(), BTreeSet::<u8>::new().iter()));
	put(out, &format!("{:?}", empty.into_keys()));

	let (mut one, mut keys) = (HashMap::new(), HashSet::new());
	if let Some(&b) = data.first() {
		one.insert((b % 2 == 0).then_some(b), u16::from(b) * 0x101);
		keys.insert([b, at(1)]);
	}
	// Past its one entry, or not.
	let n = usize::from(at(1) % 2);
	put(out, &format!("{:?}|{:#?}", one.iter(), skip(one.iter(), n)));
	put(out, &format!("{:?}|{:?}", skip(one.keys(), n), skip(one.values(), n)));
	put(out, &format!("{:?}", skip(one.iter_mut(), n)));
	put(out, &format!("{:x?}", skip(one.values_mut(), 1 - n)));
	put(out, &format!("{:?}", skip(one.clone().into_iter(), n)));
	put(
		out,
		&format!("{:?}|{:?}", one.clone().into_keys(), skip(one.clone().into_values(), n)),
	);
	let it = skip(one.drain(), n);
	put(out, &format!("{it:?}|{it:#?}"));
	drop(it);
	put(
		out,
		&format!("{:?}|{:#?}", skip(keys.iter(), n), keys.clone().into_iter()),
	);
	let it = skip(keys.drain(), 1 - n);
	put(out, &format!("{it:?}"));
}

/// An iterator of the crate's own, counting down from what it holds, which its derived
/// `Debug` writes.
#[derive(Debug)]
struct Countdown(u8);

/// An iterator of the crate's own that JS holds as a value, which its `next` reads through
/// its place.
#[derive(Debug, Clone, Copy)]
enum Toggle {
	Ready,
	Done,
}

impl Iterator for Toggle {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		match self {
			Self::Ready => {
				*self = Self::Done;
				Some(1)
			}
			Self::Done => None,
		}
	}
}

impl Iterator for Countdown {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

/// std's iterator adaptors' `Debug`, each its fields in std's order through what it wraps,
/// with no closure: consumed from either end, so each writes the state it keeps, as a
/// `Peekable` its peeked item and a `Skip` what it has left to skip, plain and pretty.
pub fn adaptors(data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (front, back) = (usize::from(at(0) % 4), usize::from(at(1) % 3));
	fn ends<I: DoubleEndedIterator>(mut it: I, front: usize, back: usize) -> I {
		for _ in 0..front {
			it.next();
		}
		for _ in 0..back {
			it.next_back();
		}
		it
	}
	fn fronts<I: Iterator>(mut it: I, front: usize) -> I {
		for _ in 0..front {
			it.next();
		}
		it
	}
	let bytes = &data[..data.len().min(6)];

	// Those that write only what they wrap.
	let it = ends(bytes.iter().map(|b| b + 1), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	let it = ends(bytes.iter().filter(|&&b| b % 2 == 0), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = ends(bytes.iter().filter_map(|&b| b.checked_sub(9)), front, back);
	put(out, &format!("{it:?}"));
	let it = ends(bytes.iter().inspect(|_| {}), front, back);
	put(out, &format!("{it:?}"));
	let it = fronts(bytes.iter().map_while(|&b| b.checked_sub(3)), front);
	put(out, &format!("{it:?}"));
	let it = ends(bytes.iter().rev(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = ends(bytes.iter().copied(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:02x?}"));
	let it = ends(bytes.iter().cloned(), front, back);
	put(out, &format!("{it:?}"));
	let words: Vec<String> = bytes.iter().map(|b| b.to_string()).collect();
	let it = ends(words.iter().cloned(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let pairs: Vec<[u8; 2]> = data.chunks_exact(2).take(3).map(|c| [c[0], c[1]]).collect();
	let it = ends(pairs.iter().copied(), front, back);
	put(out, &format!("{it:?}"));

	// Those that write a count, a flag, or a state beside it.
	let it = ends(bytes.iter().enumerate(), front, back);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	let mut it = bytes.iter().skip(at(2).into());
	put(out, &format!("{it:?}|{it:#?}"));
	it.next_back();
	put(out, &format!("{it:?}"));
	it.next();
	put(out, &format!("{it:?}"));
	let it = ends(bytes.iter().take(usize::from(at(2) % 8)), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut it = bytes.iter().skip_while(|&&b| b % 3 != 0);
	put(out, &format!("{it:?}"));
	it.next();
	put(out, &format!("{it:?}|{it:#?}"));
	let mut it = bytes.iter().take_while(|&&b| b % 5 != 0);
	put(out, &format!("{it:?}"));
	for _ in 0..front {
		it.next();
	}
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(
		bytes.iter().scan(0u32, |sum, &b| {
			*sum += u32::from(b);
			Some(*sum)
		}),
		front,
	);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	let it = fronts(
		bytes.iter().scan(String::new(), |s, &b| {
			s.push(char::from(b'a' + b % 26));
			(b % 7 != 0).then(|| s.len())
		}),
		front,
	);
	put(out, &format!("{it:?}"));

	// A peeked item, `None` peeked, and none peeked.
	let mut it = ends(bytes.iter().peekable(), front, back);
	put(out, &format!("{it:?}"));
	it.peek();
	put(out, &format!("{it:?}|{it:#?}"));
	it.next();
	put(out, &format!("{it:?}"));
	let mut it = bytes[..bytes.len().min(1)].iter().peekable();
	it.next();
	it.peek();
	put(out, &format!("{it:?}|{it:#?}"));

	// Both halves, until each runs out from its end.
	let it = ends(
		bytes[..2.min(bytes.len())].iter().chain(&bytes[2.min(bytes.len())..]),
		front,
		back,
	);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut it = bytes[..1.min(bytes.len())].iter().chain([7u8, 8].iter());
	it.next_back();
	it.next_back();
	it.next_back();
	put(out, &format!("{it:?}"));
	// A zip of two that read by index writes no field; another writes both.
	let it = ends(bytes.iter().zip(data.iter().map(|b| b ^ 1)), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = ends(bytes.iter().zip(bytes.iter().rev()), front, back);
	put(out, &format!("{it:?}"));
	let text: String = bytes
		.iter()
		.map(|&b| ['a', 'é', '中', '🦀'][usize::from(b % 4)])
		.collect();
	let it = fronts(text.chars().zip(bytes.iter()), front);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(bytes.iter().cycle(), front + usize::from(at(2) % 8));
	put(out, &format!("{it:?}|{it:#?}"));

	// What a `fuse` keeps of a `FusedIterator` past its end, and drops of another.
	let it = fronts(bytes.iter().fuse(), front + 6);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(bytes.iter().step_by(2).fuse(), front + 2);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(bytes.iter().map_while(|&b| (b % 4 != 0).then_some(b)).fuse(), front * 2);
	put(out, &format!("{it:?}"));

	// A step through a slice, and through ranges, std counting down the end of an unsigned
	// one no wider than a `usize`.
	let step = usize::from(at(3) % 4) + 1;
	let it = ends(bytes.iter().step_by(step), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let (lo, hi) = (at(4) % 16, at(5));
	let it = ends((lo..hi).step_by(step), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts((u32::from(lo)..u32::from(hi)).step_by(step), front);
	put(out, &format!("{it:?}"));
	let it = ends((usize::from(lo)..usize::from(hi)).step_by(step), front, back);
	put(out, &format!("{it:?}|{it:x?}"));
	let it = ends((u16::from(lo)..u16::from(hi)).step_by(step), front, back);
	put(out, &format!("{it:?}"));
	let it = ends((i32::from(lo)..i32::from(hi)).step_by(step), front, back);
	put(out, &format!("{it:?}"));
	let it = fronts((u64::from(lo)..u64::from(hi)).step_by(step), front);
	put(out, &format!("{it:?}"));
	// A step past the type's greatest, and a start that wraps past it.
	let it = fronts((250u8..255).step_by(300), front);
	put(out, &format!("{it:?}"));
	let it = fronts((250u8..255).step_by(usize::from(at(3)) + 1), front);
	put(out, &format!("{it:?}"));
	let it = fronts((hi..255).step_by(step + 200), front);
	put(out, &format!("{it:?}"));

	// Each source item's iterator, from the front and the back.
	let lists: Vec<Vec<u8>> = bytes.chunks(2).map(<[u8]>::to_vec).collect();
	let it = ends(lists.iter().flatten(), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = ends(lists.iter().flat_map(|v| v.iter()), front, back);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(lists.iter().flatten().fuse(), front + 6);
	put(out, &format!("{it:?}"));

	// What a unit after a leading surrogate leaves buffered.
	let units: Vec<u16> = bytes
		.iter()
		.map(|&b| [0x61, 0xd800, 0xdc00, 0xe9, 0xdbff][usize::from(b % 5)])
		.collect();
	let it = fronts(char::decode_utf16(units.iter().copied()), front);
	put(out, &format!("{it:?}|{it:#?}|{it:x?}"));
	let it = fronts(char::decode_utf16(vec![0xd800, 0x61, 0x62]), usize::from(at(2) % 3));
	put(out, &format!("{it:?}"));

	// A `str`'s escapes, each the `flat_map` of its chars, partly read.
	let quoted: String = bytes
		.iter()
		.map(|&b| ['a', '"', '\n', '\u{301}', 'é', '\''][usize::from(b % 6)])
		.collect();
	let it = fronts(quoted.escape_debug(), front * 2);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(quoted.escape_default(), front * 3);
	put(out, &format!("{it:?}"));
	let it = fronts(quoted.escape_unicode(), front * 5);
	put(out, &format!("{it:?}|{it:#?}"));

	// An adaptor of a borrowed iterator, or of the crate's own, writes it as it is.
	let mut base = bytes.iter();
	base.next();
	let it = base.by_ref().map(|b| b + 1).skip(1);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut base = units.iter().copied();
	let it = fronts(char::decode_utf16(base.by_ref()), front);
	put(out, &format!("{it:?}"));
	let mut base = bytes.iter();
	let it = fronts(base.by_ref().copied(), front);
	put(out, &format!("{it:?}|{it:#?}"));
	let it = fronts(Box::new(bytes.iter()).map(|b| b + 1), front);
	put(out, &format!("{it:?}"));
	let mut boxed = Box::new(bytes.iter().rev());
	let it = fronts((&mut boxed).zip(Box::new(Countdown(4))), front);
	put(out, &format!("{it:?}"));
	let it = fronts(Countdown(at(2) % 5).map(|n| n * 2).enumerate(), front);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut toggle = if at(3) % 2 == 0 { Toggle::Ready } else { Toggle::Done };
	let it = (&mut toggle).map(|x| x);
	put(out, &format!("{it:?}|{it:#?}"));
	let mut own = Countdown(at(3) % 5);
	let it = fronts(own.by_ref().peekable(), front);
	put(out, &format!("{it:?}"));
	let it = fronts(
		lists.iter().flat_map(|v| Countdown(v[0] % 3)).chain(Countdown(2)),
		front,
	);
	put(out, &format!("{it:?}"));

	// Adaptors of adaptors, with every option passed down.
	let it = ends(
		bytes.iter().map(|&b| Pair(b, -1)).enumerate().skip(1).rev().take(3),
		front,
		back,
	);
	put(out, &format!("{it:?}|{it:#?}|{it:>4?}"));
	let mut it = bytes.iter().copied().peekable();
	it.peek();
	let it = it.zip(1u8..).filter(|p| p.0 != 0).chain(vec![(9, 9)]);
	put(out, &format!("{it:?}|{it:#?}"));
}

/// An error only a TypeScript caller prints, as moq's callers print its errors: nothing
/// translated formats it, so its `Display` and `Debug` come along as `printable`.
#[derive(Debug)]
pub enum Fault {
	Short,
	Code(u16),
	TooLarge { size: usize, max: usize },
	At(Pair),
}

impl fmt::Display for Fault {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Short => f.pad("short buffer"),
			Self::Code(code) => write!(f, "code={code:#06x}"),
			Self::TooLarge { size, max } => write!(f, "{size} bytes exceeds the {max} byte limit"),
			Self::At(pair) => write!(f, "at {pair:?}"),
		}
	}
}

impl Fault {
	/// The fault `data` picks.
	pub fn new(data: &[u8]) -> Self {
		let at = |i: usize| data.get(i).copied().unwrap_or(0);
		match at(0) % 4 {
			0 => Self::Short,
			1 => Self::Code(u16::from_be_bytes([at(1), at(2)])),
			2 => Self::TooLarge {
				size: usize::from(at(1)) << 24 | usize::from(at(2)),
				max: usize::from(at(3)),
			},
			_ => Self::At(Pair(at(1), i64::from(at(2) as i8) << 40)),
		}
	}
}

/// A [`Fault`] as native Rust prints it, which the translated side prints from TypeScript.
pub fn printed(data: &[u8]) -> Vec<u8> {
	let fault = Fault::new(data);
	format!("{fault}|{fault:>16}|{fault:?}|{fault:#?}").into_bytes()
}

/// A writer of the crate's own that fails a `write_str` past `cap` bytes, and counts each.
pub struct Capped {
	text: String,
	calls: u8,
	cap: usize,
}

impl fmt::Write for Capped {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.calls += 1;
		if self.text.len() + s.len() > self.cap {
			return Err(fmt::Error);
		}
		for c in s.chars() {
			self.text.push(c);
		}
		Ok(())
	}
}

/// A writer of the crate's own that writes each `char` twice through its own `write_char`.
pub struct Twice(String);

impl fmt::Write for Twice {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for c in s.chars() {
			self.0.push(c);
		}
		Ok(())
	}

	fn write_char(&mut self, c: char) -> fmt::Result {
		self.0.push(c);
		self.0.push(c);
		Ok(())
	}
}

/// Literals, a `char` of each width, and empty pieces, each its own call, stopping at the
/// first error.
fn literals<W: fmt::Write>(mut w: W, c: char) -> fmt::Result {
	w.write_str("ab")?;
	write!(w, "")?;
	write!(w, "{}", "")?;
	w.write_char(c)?;
	write!(w, "c{}d", "x")?;
	w.write_char('😀')?;
	writeln!(w)?;
	write!(w, "{{}}")?;
	w.write_str("")?;
	writeln!(w, "end")
}

/// Placeholders with options, through a writer std's impl forwards to.
fn placed<W: fmt::Write>(mut w: W, n: u8) -> fmt::Result {
	write!(w, "n={n:03}|{n:>4x}|{:?}|{:<3}|{:+}", "q\"é", 'ß', i16::from(n) - 100)?;
	w.write_char('é')?;
	writeln!(w, "{n:#b}")
}

/// `Display` through a `Formatter` as a writer, which takes no options from the placeholder.
pub struct Placed(u8);

impl fmt::Display for Placed {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		placed(&mut *f, self.0)?;
		literals(f, 'é')
	}
}

/// Each result as a byte: 1 for `Ok`.
fn result(out: &mut Vec<u8>, r: fmt::Result) {
	out.push(r.is_ok() as u8);
}

/// `fmt::Write` for a `String`, through `&mut W`, and for the crate's own writers.
pub fn writes(data: &[u8], out: &mut Vec<u8>) {
	use std::fmt::Write as _;
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let c = char::from_u32(u32::from(at(0)) * 0x101).unwrap_or('?');
	let mut s = String::new();
	result(out, write!(s, "a{}b{:>5}|{:?}|{:x}", at(1), c, "q\"é", at(2)));
	result(out, writeln!(s));
	result(out, s.write_char(c));
	result(out, s.write_str("tail"));
	// An error from a placeholder stops after what it wrote, before the next piece.
	result(out, write!(s, "x{}y{}", Fails(at(3)), at(4)));
	result(out, literals(&mut s, c));
	result(out, placed(&mut s, at(5)));
	let mut inner = &mut s;
	result(out, literals(&mut inner, 'ß'));
	result(out, placed(&mut &mut inner, at(6)));
	put(out, &s);
	put(out, &format!("{}|{:>30}", Placed(at(7)), Placed(at(8))));
	let mut capped = Capped {
		text: String::new(),
		calls: 0,
		cap: usize::from(at(9) % 40),
	};
	result(out, literals(&mut capped, c));
	result(out, capped.write_char(c));
	result(out, write!(capped, "z"));
	result(out, literals(&mut &mut capped, 'é'));
	out.push(capped.calls);
	put(out, &capped.text);
	let mut twice = Twice(String::new());
	result(out, literals(&mut twice, c));
	result(out, twice.write_char('ß'));
	put(out, &twice.0);
}

/// Writes what its cell holds as it is written.
struct Gauge<'a>(&'a Cell<u8>);

impl Debug for Gauge<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_tuple("Gauge").field(&self.0.get()).finish()
	}
}

/// Logs its tag as it drops, and writes it padded by `Display`.
struct Dropper {
	tag: u8,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Debug for Dropper {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("Dropper")
			.field("tag", &self.tag)
			.finish_non_exhaustive()
	}
}

impl Display for Dropper {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.pad(&format!("n{}", self.tag))
	}
}

impl Drop for Dropper {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.tag);
	}
}

pub fn objects(data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let dropper = |tag: u8| Dropper { tag, log: log.clone() };
	let dropped = |out: &mut Vec<u8>| {
		let tags = std::mem::take(&mut *log.borrow_mut());
		put(out, &format!("{tags:?} "));
	};

	// Boxed, of each kind, written with options, then dropped in order.
	let mut shown: Vec<Box<dyn Debug>> = Vec::new();
	let mut written: Vec<Box<dyn Display>> = Vec::new();
	for (i, &x) in data.iter().enumerate() {
		match x % 4 {
			0 => shown.push(Box::new(dropper(x))),
			1 => shown.push(Box::new((x, i))),
			2 => written.push(Box::new(dropper(x))),
			_ => written.push(Box::new(format!("s{x}"))),
		}
	}
	for s in &shown {
		put(out, &format!("{s:?} {s:#?} "));
	}
	for w in &written {
		put(out, &format!("{w} [{w:>6}] [{w:^7}] "));
	}
	let sent: Box<dyn Debug + Send> = Box::new(data.len());
	let up: Box<dyn Debug> = sent;
	put(out, &format!("{up:?} "));
	drop(written);
	dropped(out);
	shown.truncate(1);
	dropped(out);

	// Shared: the value drops with the last pointer, whichever it is.
	let [a, b, ..] = data else {
		return;
	};
	let shared: Arc<dyn Debug + Send + Sync> = Arc::new((*a, *b));
	let kept = shared.clone();
	put(out, &format!("{shared:?} {} ", Arc::strong_count(&kept)));
	let counted: Rc<Dropper> = Rc::new(dropper(*a));
	let as_display: Rc<dyn Display> = counted.clone();
	let as_debug: Rc<dyn Debug> = counted;
	put(out, &format!("{as_display:<5}| {} ", Rc::strong_count(&as_debug)));
	drop(as_display);
	dropped(out);
	match b % 2 {
		0 => drop(as_debug),
		_ => put(out, &format!("{as_debug:?} ")),
	}
	dropped(out);
	drop(shared);
	put(out, &format!("{kept:?} {} ", Arc::strong_count(&kept)));

	// Mutably borrowed, written as a shared borrow would be, from where the value is: a
	// number and a string JS copies, the crate's type, a box of one, and a `Cell` set
	// while borrowed.
	let mut n = a.wrapping_mul(3);
	let mut text = format!("t{b}");
	let mut owned = dropper(*b);
	let mut boxed: Box<dyn Debug> = Box::new(dropper(b.wrapping_add(1)));
	let cell = Cell::new(*a);
	let mut pair = (Gauge(&cell), *b);
	{
		let mut objects: Vec<&mut dyn Debug> = vec![&mut n, &mut text, &mut owned, &mut boxed, &mut pair];
		for o in objects.iter_mut() {
			put(out, &format!("{o:?} {o:#?} "));
		}
		cell.set(cell.get().wrapping_add(1));
		let again: &mut dyn Debug = &mut *objects[4];
		put(out, &format!("{again:?} "));
		let shared: &dyn Debug = again;
		put(out, &format!("{shared:?} "));
	}
	n = n.wrapping_add(1);
	let shown: &mut dyn Display = &mut n;
	put(out, &format!("[{shown:>5}] "));
	let written: &mut dyn Display = &mut owned;
	put(out, &format!("[{written:^6}] "));
	text.push('!');
	let last: &mut dyn Display = &mut text;
	put(out, &format!("{last} "));
	drop(boxed);
	dropped(out);
}
