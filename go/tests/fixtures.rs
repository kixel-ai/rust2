//! Each fixture under `fixtures/` is translated, vetted with `go vet`, and run on the
//! same inputs as its native build, failing on any difference.

use std::cell::{Cell, RefCell};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn manifest_dir() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn scratch() -> PathBuf {
	PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
}

fn fixture(name: &str) -> PathBuf {
	manifest_dir().join("fixtures").join(name)
}

/// Translate a fixture into a Go module named after it, returning the module's directory.
fn translated(name: &str) -> PathBuf {
	let module = scratch().join(name);
	let out = module.join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-go"))
		.arg("--manifest-path")
		.arg(fixture(name).join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", scratch().join("build"))
		.output()
		.expect("running rust2-go");
	assert!(
		output.status.success(),
		"translating {name} failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	std::fs::write(out.join("go.mod"), "module fixture\n\ngo 1.24\n").unwrap();
	out
}

/// Run `go` in `dir`, failing with what it printed.
fn go(dir: &Path, args: &[&str]) {
	let output = Command::new("go")
		.args(args)
		.current_dir(dir)
		.output()
		.expect("running go, which the dev shell provides");
	assert!(
		output.status.success(),
		"go {} failed:\n{}{}",
		args.join(" "),
		String::from_utf8_lossy(&output.stdout),
		String::from_utf8_lossy(&output.stderr)
	);
}

#[derive(PartialEq, Debug)]
enum Outcome {
	Output(Vec<u8>),
	Panic(String),
}

thread_local! {
	/// Whether `native` is catching this thread's panics, which the hook then records
	/// instead of printing.
	static CATCHING: Cell<bool> = const { Cell::new(false) };
	/// The message of the last panic `native` caught on this thread.
	static PANIC: RefCell<Option<String>> = const { RefCell::new(None) };
}

fn native(run: fn(&[u8]) -> Vec<u8>, input: &[u8]) -> Outcome {
	static HOOK: std::sync::Once = std::sync::Once::new();
	HOOK.call_once(|| {
		// Any other panic, such as a failing test's assertion, goes to the default hook.
		let rest = std::panic::take_hook();
		std::panic::set_hook(Box::new(move |info| {
			if !CATCHING.get() {
				return rest(info);
			}
			let msg = info
				.payload_as_str()
				.unwrap_or("a panic of a payload other than a message");
			PANIC.set(Some(msg.to_string()));
		}))
	});
	CATCHING.set(true);
	let result = std::panic::catch_unwind(|| run(input));
	CATCHING.set(false);
	match result {
		Ok(out) => Outcome::Output(out),
		Err(_) => Outcome::Panic(PANIC.take().expect("a panic message")),
	}
}

/// Run the translated harness, the root exported as `entry`, on every input in one process.
fn translation(out: &Path, entry: &str, cases: &[Vec<u8>]) -> Vec<Outcome> {
	let main = out.join("differential");
	std::fs::create_dir_all(&main).unwrap();
	let text = std::fs::read_to_string(manifest_dir().join("tests/differential/main.go")).unwrap();
	std::fs::write(
		main.join("main.go"),
		text.replace("fixture.Run(", &format!("fixture.{entry}(")),
	)
	.unwrap();
	go(
		out,
		&["build", "-buildvcs=false", "-o", "differential/run", "./differential"],
	);
	let mut child = Command::new(main.join("run"))
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("running the translated harness");
	let mut stdin = child.stdin.take().unwrap();
	let mut input = Vec::new();
	for case in cases {
		input.extend_from_slice(&(case.len() as u32).to_le_bytes());
		input.extend_from_slice(case);
	}
	let writer = std::thread::spawn(move || {
		// The harness may exit early, which its status reports below.
		let _ = stdin.write_all(&input);
	});
	let output = child.wait_with_output().unwrap();
	writer.join().unwrap();
	assert!(
		output.status.success(),
		"the translated harness failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	let mut rest = &output.stdout[..];
	let mut outcomes = Vec::new();
	while let Some((&kind, tail)) = rest.split_first() {
		let len = u32::from_le_bytes(tail[..4].try_into().unwrap()) as usize;
		let body = tail[4..4 + len].to_vec();
		rest = &tail[4 + len..];
		outcomes.push(match kind {
			0 => Outcome::Output(body),
			_ => Outcome::Panic(String::from_utf8(body).unwrap()),
		});
	}
	outcomes
}

/// xorshift64*: the same cases on every run.
struct Rng(u64);

impl Rng {
	fn next(&mut self) -> u64 {
		self.0 ^= self.0 >> 12;
		self.0 ^= self.0 << 25;
		self.0 ^= self.0 >> 27;
		self.0.wrapping_mul(0x2545f4914f6cdd1d)
	}

	fn byte(&mut self) -> u8 {
		self.next() as u8
	}

	/// A byte biased toward the values codecs care about: zero, all ones, and boundaries.
	fn interesting(&mut self) -> u8 {
		const EDGES: [u8; 16] = [
			0x3f, 0x40, 0x7f, 0x80, 0xbf, 0xc0, 0xdf, 0xe0, 0xef, 0xf0, 0xf7, 0xf8, 0xfb, 0xfc, 0xfd, 0xfe,
		];
		match self.next() % 8 {
			0 => 0,
			1 => 0xff,
			2 => EDGES[(self.next() % 16) as usize],
			_ => self.byte(),
		}
	}
}

/// Every op with every short tail, each u64 boundary through the first `ops` ops with each
/// of `selectors` second bytes, and `count` random ones from one stream.
fn cases(ops: u8, selectors: u8, count: usize) -> Vec<Vec<u8>> {
	let mut cases = vec![Vec::new()];
	for op in 0..=255u8 {
		cases.push(vec![op]);
		for b in (0..=255u8).step_by(17) {
			cases.push(vec![op, b]);
		}
	}
	for op in 0..ops {
		for k in 0..=64u32 {
			for delta in [-1i64, 0, 1] {
				let value = 1u64
					.checked_shl(k)
					.unwrap_or(0)
					.wrapping_add_signed(delta)
					.to_be_bytes();
				for selector in 0..selectors {
					let mut case = vec![op, selector];
					for _ in 0..3 {
						case.extend_from_slice(&value);
					}
					cases.push(case);
				}
			}
		}
	}
	let mut rng = Rng(0x9e3779b97f4a7c15);
	for _ in 0..count {
		let len = (rng.next() % 48) as usize;
		let mut case = vec![rng.byte()];
		case.extend((0..len).map(|_| rng.interesting()));
		cases.push(case);
	}
	cases
}

fn show(outcome: &Outcome) -> String {
	match outcome {
		Outcome::Output(out) => out.iter().map(|b| format!("{b:02x}")).collect(),
		Outcome::Panic(msg) => format!("panic {msg:?}"),
	}
}

/// Run a fixture's harness natively, as `run`, and translated, as `entry`, on the same
/// inputs, failing on any difference.
fn differential(name: &str, run: fn(&[u8]) -> Vec<u8>, entry: &str, cases: Vec<Vec<u8>>) {
	let out = translated(name);
	go(&out, &["vet", "."]);
	let want: Vec<Outcome> = cases.iter().map(|case| native(run, case)).collect();
	let got = translation(&out, entry, &cases);
	assert_eq!(got.len(), cases.len(), "the translated harness stopped early");
	let mut failures = Vec::new();
	for ((input, want), got) in cases.iter().zip(&want).zip(&got) {
		if want != got {
			failures.push(format!(
				"input  {}\nnative {}\ngo     {}\n",
				show(&Outcome::Output(input.clone())),
				show(want),
				show(got)
			));
		}
	}
	let panics = want.iter().filter(|o| matches!(o, Outcome::Panic(_))).count();
	println!(
		"{name}: {} cases, {panics} panics, {} differences",
		cases.len(),
		failures.len()
	);
	assert!(
		failures.is_empty(),
		"{name}: {} cases, {panics} panics, {} differences:\n{}",
		cases.len(),
		failures.len(),
		failures[..failures.len().min(20)].join("\n")
	);
}

/// moq's codec, through rust2-ts's harness of it.
#[test]
fn codec() {
	differential("codec", codec::harness::run, "HarnessRun", cases(8, 7, 20_000));
}

/// Every integer width's arithmetic, which overflows, divides by zero, and shifts past
/// its width where native Rust panics.
#[test]
fn ints() {
	differential("ints", ints::run, "Run", cases(16, 32, 20_000));
}

/// Formatting's failures, and counts at each edge of what one holds.
#[test]
fn fmt() {
	let mut cases = Vec::new();
	for pick in 0..6u8 {
		for count in [0u64, 1, 2, 5, 6, 65_534, 65_535, 65_536, 1 << 32, u64::MAX] {
			let mut case = vec![pick];
			case.extend_from_slice(&count.to_be_bytes());
			cases.push(case);
		}
	}
	differential("fmt", fmt::run, "Run", cases);
}

/// Capacity requests either side of `isize::MAX` bytes, of zero-sized and sized elements,
/// and constants' bytes. A sized request that fits would only abort natively, out of memory.
#[test]
fn vecs() {
	let mut cases = Vec::new();
	for capacity in [0u64, 1, 4, 1 << 59, (1 << 60) - 1, 1 << 60, 1 << 63, u64::MAX] {
		for pick in 0..2u8 {
			if pick == 1 && (4..(1 << 60)).contains(&capacity) {
				continue;
			}
			let mut case = vec![pick];
			case.extend_from_slice(&capacity.to_be_bytes());
			cases.push(case);
		}
	}
	cases.push(vec![2]);
	differential("vecs", vecs::run, "Run", cases);
}

/// Byte storage the runtime owns, made, grown, copied, and shrunk each way, and handed
/// to Go code, which `tests/hand/hand_test.go` holds onto as Go code may.
#[test]
fn hand() {
	differential("hand", hand::run, "Run", cases(0, 0, 500));
	let out = scratch().join("hand/out");
	// Go code reaches the translation only through its roots, named in camel case, which
	// hand storage over.
	let text = std::fs::read_to_string(out.join("hand.go")).unwrap();
	for name in text.lines().filter_map(|l| l.strip_prefix("func ")?.split('(').next()) {
		assert!(
			!name.starts_with(|c: char| c.is_ascii_uppercase()) || !name.contains('_'),
			"hand: `{name}` is exported past the roots"
		);
	}
	std::fs::copy(manifest_dir().join("tests/hand/hand_test.go"), out.join("hand_test.go")).unwrap();
	go(&out, &["test", "-count=1", "."]);
}

/// Fields and variants whose escaped names could meet, constant memory, which each use
/// shares, constants and statics, and the `pub` API, exported by the paths the crate
/// exports it at.
#[test]
fn items() {
	differential("items", items::run, "Run", cases(4, 4, 1_000));
	exports();
	let text = std::fs::read_to_string(scratch().join("items/out/items.go")).unwrap();
	for exported in [
		"type Count struct",
		"func CountNew(",
		"func CountBump(",
		"type TalliesTotal struct",
		"func TalliesSum(",
		"type Meter struct",
		"func MeterNew(",
		"type Byte = Value_2[uint8]",
		"type Value = uint16",
		"type Level = uint8",
		"type Pair[T any] = tuple2[T, T]",
		"func Unwrap(",
		"\tSum uint8",
		"\tTally   uint8",
		"\t_Tally  uint8",
		"\t__Tally uint8",
	] {
		assert!(text.contains(exported), "items.go lacks `{exported}`");
	}
	let dep = std::fs::read_to_string(scratch().join("items/out/items_dep.go")).unwrap();
	assert!(
		dep.contains("func ShapesArea("),
		"items_dep.go lacks `func ShapesArea(`"
	);
	for unexported in ["TalliesCounter", "CountHidden", "type Gauge"] {
		assert!(
			!text.contains(unexported) && !dep.contains(unexported),
			"items has `{unexported}`"
		);
	}
}

/// The items fixture's constants and statics as `tests/exports` reads them from Go: each
/// constant a Go `const` or a fresh value at each call, and each static one variable,
/// which translated code reads what Go writes to.
fn exports() {
	let out = scratch().join("items/out");
	let main = out.join("exports");
	std::fs::create_dir_all(&main).unwrap();
	std::fs::copy(manifest_dir().join("tests/exports/main.go"), main.join("main.go")).unwrap();
	let output = Command::new("go")
		.args(["run", "-buildvcs=false", "./exports"])
		.current_dir(&out)
		.output()
		.expect("running go");
	assert!(
		output.status.success(),
		"tests/exports failed:\n{}",
		String::from_utf8_lossy(&output.stderr)
	);
	use items::{Count, shapes};
	let hex = |s: &str| s.bytes().map(|b| format!("{b:02x}")).collect::<String>();
	let table: Vec<String> = items::TABLE.iter().map(u16::to_string).collect();
	let mut start = Count::START;
	start.bump();
	let want = format!(
		"{} {} {} {} {} {}\n{} {}\n99 {} {} {}\n{} [{}] {} {} {}\n{} 9 9\nq{}\n",
		items::MAX_LEN,
		Count::MAX,
		items::FLOOR,
		items::ENABLED,
		items::LETTER,
		shapes::SIDES,
		hex(items::GREETING),
		items::TEXT,
		items::ORIGIN.sum,
		start.count,
		Count::START.count,
		items::HITS.count,
		table.join(" "),
		items::NAME,
		std::str::from_utf8(&items::BYTES).unwrap(),
		shapes::UNIT,
		items::HITS.count,
		std::str::from_utf8(&items::TAIL[1..]).unwrap(),
	);
	assert_eq!(String::from_utf8_lossy(&output.stdout), want);
}

/// Enums Go holds as their parts, behind pointers where a part is over four words.
#[test]
fn enums() {
	differential("enums", enums::run, "Run", cases(10, 6, 2_000));
}

/// Temporaries folded into what reads them, where moving a call, a read of memory, or
/// an overflow check past another would change the output or the panic.
#[test]
fn order() {
	differential("order", order::run, "Run", cases(11, 4, 2_000));
}

/// Constructs outside the subset each fail translation with a span, as
/// `fixtures/reject/expected.txt` lists them, and no output is written.
#[test]
fn reject() {
	let name = "reject";
	let out = scratch().join(name).join("out");
	let output = Command::new(env!("CARGO_BIN_EXE_rust2-go"))
		.arg("--manifest-path")
		.arg(fixture(name).join("Cargo.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", scratch().join("build"))
		.output()
		.expect("running rust2-go");
	assert!(!output.status.success(), "translating {name} succeeded");
	assert!(!out.exists(), "a failed translation wrote output");
	// Each error and where it points, with std's source from wherever the toolchain keeps it.
	let stderr = String::from_utf8_lossy(&output.stderr);
	let mut got = String::new();
	let mut lines = stderr.lines().peekable();
	while let Some(line) = lines.next() {
		let Some(error) = line.strip_prefix("error: rust2-go: ") else {
			continue;
		};
		got.push_str(error);
		if let Some(at) = lines.peek().and_then(|l| l.trim_start().strip_prefix("--> ")) {
			let at = at.split_once("/rustlib/src/rust/").map_or(at, |(_, path)| path);
			got.push_str(&format!(" at {at}"));
		}
		got.push('\n');
	}
	let expected = fixture(name).join("expected.txt");
	if std::env::var_os("UPDATE_EXPECT").is_some() {
		std::fs::write(&expected, &got).unwrap();
		return;
	}
	let want = std::fs::read_to_string(&expected).unwrap();
	assert!(
		got == want,
		"{name}: the errors differ from expected.txt; rerun with UPDATE_EXPECT=1\n{}",
		diff(&want, &got)
	);
}

/// The first lines where two texts differ.
fn diff(want: &str, got: &str) -> String {
	let (want, got): (Vec<&str>, Vec<&str>) = (want.lines().collect(), got.lines().collect());
	let start = want.iter().zip(&got).take_while(|(a, b)| a == b).count();
	let mut out = String::new();
	for line in want.iter().skip(start).take(8) {
		out.push_str(&format!("- {line}\n"));
	}
	for line in got.iter().skip(start).take(8) {
		out.push_str(&format!("+ {line}\n"));
	}
	out
}
