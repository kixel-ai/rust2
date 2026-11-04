//! The runtime's own behavior, beyond what translated fixtures exercise.

// The Unicode properties `str::to_lowercase` reads for a final sigma, which `unicode` copies.
#![feature(case_ignorable, titlecase)]

use std::path::PathBuf;
use std::process::Command;

/// `rt.spawn` yields to a macrotask once a batch spends its poll budget, and a signal cancels it.
#[test]
fn spawn() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("spawn.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// A NaN keeps its payload and sign through `f64`'s and `f32`'s bits, which `nan.ts` checks in Node.
#[test]
fn nan_payloads() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("nan.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// The lang fixture's `mul_add` allowance rounds exactly and accepts native a last place
/// off only where the translation is exact, which `fma.ts` checks.
#[test]
fn fma_allowance() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fma.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// A recursive `OnceLock` init, or `LazyLock` or `LazyCell` force, panics with std's
/// message, which `once.ts` checks.
#[test]
fn once_recursive() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("once.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// `try_reserve` of bytes gives std's `AllocError` where JS can't make them, which
/// `reserve.ts` checks, as no allocation fails on demand natively.
#[test]
fn reserve_alloc_error() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("reserve.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// A lazy thread-local's first value comes as wasm32's `LazyStorage` makes it, which
/// `local.ts` checks.
#[test]
fn local_lazy() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("local.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}

/// The runtime's B-tree runs `btree.ts`'s script of random operations as std's does.
#[test]
fn btree_script() {
	use std::collections::BTreeMap;

	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("btree.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let got = String::from_utf8(output.stdout).unwrap();

	// xorshift32, as `btree.ts` runs it.
	let mut state: u32 = 0x2545f491;
	let mut rand = move || {
		state ^= state << 13;
		state ^= state >> 17;
		state ^= state << 5;
		state
	};
	fn walk<'a>(
		rand: &mut impl FnMut() -> u32,
		it: &mut impl DoubleEndedIterator<Item = (&'a u32, &'a u32)>,
	) -> String {
		let mut seen = Vec::new();
		for _ in 0..40 {
			let item = if rand() & 1 == 1 { it.next() } else { it.next_back() };
			let Some((k, _)) = item else { break };
			seen.push(k.to_string());
		}
		seen.join(",")
	}
	let show = |v: Option<u32>| v.map_or("-".to_string(), |v| v.to_string());
	let entry = |e: Option<(&u32, &u32)>| e.map_or("-".to_string(), |(k, v)| format!("{k} {v}"));

	let mut want = Vec::new();
	let mut m: BTreeMap<u32, u32> = BTreeMap::new();
	let steps = 120_000;
	for step in 0..steps {
		let keys = 65_536;
		let phase = 3 * step / steps;
		let op = rand() % 10;
		let k = rand() % keys;
		if op < 5 {
			if phase == 0 || (phase == 1 && k % 2 == 0) {
				want.push(format!("insert {k} {}", show(m.insert(k, step))));
			} else {
				want.push(format!("remove {k} {}", show(m.remove(&k))));
			}
		} else if op == 5 {
			want.push(format!("get {k} {}", show(m.get(&k).copied())));
		} else if op == 6 {
			let j = rand() % keys;
			let (lo, hi) = if k < j { (k, j) } else { (j, k) };
			let inclusive = rand() & 1;
			let seen = match inclusive {
				1 => walk(&mut rand, &mut m.range(lo..=hi)),
				_ => walk(&mut rand, &mut m.range(lo..hi)),
			};
			want.push(format!("range {lo} {hi} {inclusive} {seen}"));
		} else if op == 7 {
			let last = rand() & 1 == 1;
			let popped = if last { m.pop_last() } else { m.pop_first() };
			want.push(format!(
				"pop {last} {}",
				popped.map_or("-".to_string(), |(k, v)| format!("{k} {v}"))
			));
		} else if op == 8 {
			want.push(format!("remove {k} {}", show(m.remove(&k))));
		} else if step % 97 == 0 {
			let mut it = m.iter();
			let seen = walk(&mut rand, &mut it);
			want.push(format!("iter {} {} {seen}", m.len(), it.len()));
		} else if step % 1009 == 0 {
			let copy = m.clone();
			m.retain(|key, _| key % 7 != k % 7);
			want.push(format!(
				"retain {} {} {}",
				m.len(),
				entry(m.first_key_value()),
				entry(m.last_key_value())
			));
			if rand() & 1 == 1 {
				m = copy;
			}
			want.push(format!("kept {}", m.len()));
		} else if step % 211 == 0 {
			let j = rand() % keys;
			let (lo, hi) = if k < j { (k, j) } else { (j, k) };
			let limit = rand() % 400;
			let copy = m.clone();
			let taken: Vec<String> = (m.extract_if(lo..hi, |key, value| (*key ^ *value) % 3 != 0))
				.take(limit as usize)
				.map(|(k, _)| k.to_string())
				.collect();
			want.push(format!("extract {lo} {hi} {} {}", m.len(), taken.join(",")));
			if rand() & 1 == 1 {
				m = copy;
			}
		}
	}
	let keys: Vec<String> = m.keys().map(u32::to_string).collect();
	want.push(format!("end {} {}", m.len(), keys.join(",")));
	let mut drained = Vec::new();
	while !m.is_empty() {
		let popped = if rand() & 1 == 1 { m.pop_last() } else { m.pop_first() };
		drained.push(popped.unwrap().0.to_string());
	}
	want.push(format!("drain {}", drained.join(",")));
	for (got, want) in got.lines().zip(&want) {
		assert_eq!(got, want);
	}
	assert_eq!(got.lines().count(), want.len());
}

/// A value that logs itself when it drops.
struct Logged<'a, T: Copy>(T, &'a std::cell::RefCell<Vec<T>>);

impl<T: Copy> Drop for Logged<'_, T> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// `retain` leaves a B-tree, a deque, and a `Vec` as std's do, and `dedup_by` a `Vec`,
/// each dropping what std's drops, which `retain.ts` prints for every keep mask of up to
/// 6 items.
#[test]
fn retain() {
	use std::collections::{BTreeMap, VecDeque};

	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests")
		.join("retain.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
	let got = String::from_utf8(output.stdout).unwrap();

	let mut want = String::new();
	for n in 0..=6u32 {
		for mask in 0..1u32 << n {
			let keep = |i: u32| (mask >> i) & 1 == 1;
			let mut tree: BTreeMap<u32, u32> = (0..n).map(|i| (i, i * 10)).collect();
			tree.retain(|&k, _| keep(k));
			let mut deque = VecDeque::new();
			for i in (0..n / 2).rev() {
				deque.push_front(i);
			}
			deque.extend(n / 2..n);
			deque.retain(|&i| keep(i));
			let dropped = std::cell::RefCell::new(Vec::new());
			let mut vec: Vec<Logged<u32>> = (0..n).map(|i| Logged(i, &dropped)).collect();
			vec.retain(|i| keep(i.0));
			let vec: Vec<u32> = vec.iter().map(|i| i.0).collect();
			let dropped = std::mem::take(&mut *dropped.borrow_mut());
			// A run ends at each item the mask clears.
			let deduped = std::cell::RefCell::new(Vec::new());
			let mut run = 0;
			let mut runs: Vec<Logged<(u32, u32)>> = (0..n)
				.map(|i| {
					if !keep(i) {
						run += 1;
					}
					Logged((i, run), &deduped)
				})
				.collect();
			runs.dedup_by(|a, b| a.0.1 == b.0.1);
			let ran: Vec<u32> = runs.iter().map(|r| r.0.0).collect();
			let deduped: Vec<u32> = std::mem::take(&mut *deduped.borrow_mut())
				.into_iter()
				.map(|r| r.0)
				.collect();
			let entries: Vec<String> = tree.iter().map(|(k, v)| format!("{k},{v}")).collect();
			let items: Vec<String> = deque.iter().map(u32::to_string).collect();
			let join = |v: &[u32]| v.iter().map(u32::to_string).collect::<Vec<_>>().join(",");
			want += &format!(
				"{n} {mask} btree {} deque {} vec {} dropped {} dedup {} dropped {}\n",
				entries.join(";"),
				items.join(","),
				join(&vec),
				join(&dropped),
				join(&ran),
				join(&deduped),
			);
		}
	}
	for (got, want) in got.lines().zip(want.lines()) {
		assert_eq!(got, want);
	}
	assert_eq!(got.lines().count(), want.lines().count());
}

/// `rt/unicode.ts` holds std's Unicode tables that `char`'s classes, case maps, and `Debug`
/// read, and `str::to_lowercase`'s final sigma, by this toolchain's std, whose Unicode version
/// the JS engine's may trail. `UPDATE_EXPECT=1 just ts test --test runtime unicode` rewrites it.
#[test]
fn unicode() {
	let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("runtime/rt/unicode.ts");
	// The runtime reads `is_cased` from three of the tables, as std defines it.
	for c in (0..=char::MAX as u32).filter_map(char::from_u32) {
		let cased = c.is_lowercase() || c.is_uppercase() || c.is_titlecase();
		assert_eq!(c.is_cased(), cased, "U+{:04X}", u32::from(c));
	}
	let tables = [
		class("UNPRINTABLE", "The chars `Debug` writes as `\\u{..}`", |c| {
			c.escape_debug().to_string().starts_with("\\u")
		}),
		class("ALPHABETIC", "The chars `is_alphabetic` holds of", char::is_alphabetic),
		class("LOWERCASE", "The chars `is_lowercase` holds of", char::is_lowercase),
		class("UPPERCASE", "The chars `is_uppercase` holds of", char::is_uppercase),
		class("NUMERIC", "The chars `is_numeric` holds of", char::is_numeric),
		class("TITLECASE", "The chars `is_titlecase` holds of", char::is_titlecase),
		class(
			"CASE_IGNORABLE",
			"The chars `is_case_ignorable` holds of",
			char::is_case_ignorable,
		),
		case_map("TO_LOWERCASE", "to_lowercase", |c| c.to_lowercase().collect()),
		case_map("TO_UPPERCASE", "to_uppercase", |c| c.to_uppercase().collect()),
	];
	let want = format!(
		"// Generated by `UPDATE_EXPECT=1 just ts test --test runtime unicode` from std. Do not edit.\n{}",
		tables.concat()
	);
	if std::env::var_os("UPDATE_EXPECT").is_some() {
		std::fs::write(&path, want).unwrap();
		return;
	}
	let got = std::fs::read_to_string(&path).unwrap_or_default();
	assert!(
		got == want,
		"{} differs from std's Unicode; rerun with UPDATE_EXPECT=1",
		path.display()
	);
}

/// The table `name` of the chars `held` holds of, which `doc` names.
fn class(name: &str, doc: &str, held: fn(char) -> bool) -> String {
	let doc = format!("{doc}: alternately how far past the last run's end a run starts, and its length.");
	table(name, &doc, &runs(held))
}

/// The runs of chars `class` holds of: alternately how far past the last run's end a run
/// starts, and its length.
fn runs(class: fn(char) -> bool) -> Vec<i64> {
	let mut runs = Vec::new();
	let (mut end, mut start) = (0, None);
	for point in 0..=char::MAX as u32 + 1 {
		let held = char::from_u32(point).is_some_and(class);
		match (held, start) {
			(true, None) => start = Some(point),
			(false, Some(s)) => {
				runs.extend([i64::from(s - end), i64::from(point - s)]);
				(end, start) = (point, None);
			}
			_ => {}
		}
	}
	runs
}

/// The tables `name` and `{name}_MULTI` of what `method`, as `map`, maps each char to.
fn case_map(name: &str, method: &str, map: fn(char) -> Vec<char>) -> String {
	let (runs, multi) = case_runs(map);
	let doc = format!(
		"The chars `{method}` maps to one other: in fours, how far past the last run's end a run \
		 starts, its length, the step between its chars, and how far past each its char is."
	);
	let mut out = table(name, &doc, &runs);
	let doc = format!(
		"The chars `{method}` maps to more than one: how far past the last such char each is, how \
		 many it maps to, and those chars."
	);
	out += &table(&format!("{name}_MULTI"), &doc, &multi);
	out
}

/// `map`'s table of the chars it maps to one other, in runs that step by one or two with
/// the same offset, and of those it maps to more than one, in `rt/unicode.ts`'s formats.
fn case_runs(map: fn(char) -> Vec<char>) -> (Vec<i64>, Vec<i64>) {
	// Each run's first char, length, step, and offset.
	let mut runs: Vec<(u32, u32, u32, i64)> = Vec::new();
	let mut multi = Vec::new();
	let mut last_multi = 0;
	for c in (0..=char::MAX as u32).filter_map(char::from_u32) {
		let point = u32::from(c);
		match map(c).as_slice() {
			[one] if *one == c => {}
			[one] => {
				let delta = i64::from(u32::from(*one)) - i64::from(point);
				match runs.last_mut() {
					Some((start, len, step, d))
						if *d == delta
							&& matches!(point - (*start + (*len - 1) * *step), gap @ 1..=2 if *len == 1 || gap == *step) =>
					{
						*step = point - (*start + (*len - 1) * *step);
						*len += 1;
					}
					_ => runs.push((point, 1, 1, delta)),
				}
			}
			many => {
				multi.extend([i64::from(point - last_multi), many.len() as i64]);
				multi.extend(many.iter().map(|&m| i64::from(u32::from(m))));
				last_multi = point;
			}
		}
	}
	let mut table = Vec::new();
	let mut end = 0;
	for (start, len, step, delta) in runs {
		// A run that steps by two must not hold a char of the next run between its own.
		assert!(start >= end, "case runs overlap at U+{start:04X}");
		table.extend([i64::from(start - end), i64::from(len), i64::from(step), delta]);
		end = start + (len - 1) * step + 1;
	}
	(table, multi)
}

/// `values` as `rt/unicode.ts` exports a table: `name`, documented by `doc`, in lines of
/// at most 90 columns.
fn table(name: &str, doc: &str, values: &[i64]) -> String {
	let wrap = |text: &str, first: &str, rest: &str| {
		let mut lines = vec![first.to_string()];
		for word in text.split(' ') {
			if lines.last().unwrap().len() + word.len() + 1 > 90 {
				lines.push(rest.to_string());
			}
			let line = lines.last_mut().unwrap();
			if !line.ends_with(' ') {
				*line += " ";
			}
			*line += word;
		}
		lines.join("\n")
	};
	let mut out = format!("\n{} */\n", wrap(doc, "/**", " *"));
	out += &format!("export const {name}: readonly number[] = [\n");
	let mut line = String::new();
	for value in values.iter().map(i64::to_string) {
		if line.len() + value.len() + 3 > 90 {
			out += &format!("\t{}\n", line.trim_end());
			line.clear();
		}
		line += &format!("{value}, ");
	}
	if !line.is_empty() {
		out += &format!("\t{}\n", line.trim_end());
	}
	out + "];\n"
}

/// A pointer the translation can't follow fails where JS keeps apart what Rust's reaches,
/// which `pointers.ts` checks.
#[test]
fn pointers_apart() {
	let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests")
		.join("pointers.ts");
	let output = Command::new("node")
		.arg(script)
		.output()
		.expect("running node, which the dev shell provides");
	assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
}
