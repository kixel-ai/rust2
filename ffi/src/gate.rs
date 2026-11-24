//! Where a binding compiled for each target declares an item only some of its targets
//! have: C's and C++'s headers, Go's packages, and Swift's module test the target they
//! compile for, so a constant only some targets have exists only on those.
//!
//! A gate is the language's own compile-time condition: an `||` of the `&&`s that each
//! target the item is on meets, over the fewest of the target's traits that tell those
//! targets apart from the binding's others, or its negation where that is shorter. A trait
//! the language cannot test, such as a C library's (`gnu` or `musl`), tells none apart,
//! and an item it cannot gate is refused at its span.

use crate::ir::{Api, Constant, On};

/// A binding that tests at compile time the target it compiles for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Lang {
	/// C's preprocessor, which C++'s header shares.
	C,
	Swift,
	Go,
}

impl Lang {
	fn name(self) -> &'static str {
		match self {
			Lang::C => "C",
			Lang::Swift => "Swift",
			Lang::Go => "Go",
		}
	}
}

/// Where a binding declares an item.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(crate) enum Gate {
	/// On every target the binding serves, with no condition.
	Everywhere,
	/// On no target the binding serves, so it declares nothing.
	Nowhere,
	/// Where the condition holds.
	If(String),
}

/// The traits of a target a gate may test, each as the condition a language writes for
/// the target's value of it, in the order a gate prefers them.
const TRAITS: usize = 4;

/// The conditions that hold on the target `cfgs` describes, as `lang` writes each of
/// [`TRAITS`]: its pointer width, architecture, operating system, and whether it is an
/// Apple simulator or Mac Catalyst. None where `lang` cannot test it, so a gate never does.
fn traits(lang: Lang, cfgs: &std::collections::BTreeMap<String, String>) -> [Option<String>; TRAITS] {
	let cfg = |name: &str| cfgs.get(name).map(String::as_str).unwrap_or_default();
	let (width, arch, os) = (cfg("TARGET_POINTER_WIDTH"), cfg("TARGET_ARCH"), cfg("TARGET_OS"));
	// A newer rustc names an Apple simulator or Mac Catalyst by its `target_env`, an older by its
	// `target_abi`.
	let environment = ["sim", "macabi"]
		.into_iter()
		.find(|e| cfg("TARGET_ENV") == *e || cfg("TARGET_ABI") == *e)
		.unwrap_or("");
	let s = |s: &str| Some(s.to_string());
	match lang {
		Lang::C => {
			let apple = |macro_: &str| Some(format!("(defined(__APPLE__) && {macro_})"));
			[
				match width {
					"16" => s("UINTPTR_MAX == 0xFFFFu"),
					"32" => s("UINTPTR_MAX == 0xFFFFFFFFu"),
					"64" => s("UINTPTR_MAX == 0xFFFFFFFFFFFFFFFFu"),
					_ => None,
				},
				match arch {
					// ARM64EC defines MSVC's x64 macro too.
					"x86_64" => s("(defined(__x86_64__) || (defined(_M_X64) && !defined(_M_ARM64EC)))"),
					"x86" => s("(defined(__i386__) || defined(_M_IX86))"),
					"aarch64" => s("(defined(__aarch64__) || defined(_M_ARM64))"),
					"arm" => s("(defined(__arm__) || defined(_M_ARM))"),
					"wasm32" => s("defined(__wasm32__)"),
					"riscv64" => s("(defined(__riscv) && __riscv_xlen == 64)"),
					_ => None,
				},
				match os {
					"linux" => s("(defined(__linux__) && !defined(__ANDROID__))"),
					"android" => s("defined(__ANDROID__)"),
					"windows" => s("defined(_WIN32)"),
					"freebsd" => s("defined(__FreeBSD__)"),
					"wasi" => s("defined(__wasi__)"),
					// From `TargetConditionals.h`, which the header includes on Apple's systems.
					"macos" => apple("TARGET_OS_OSX"),
					"ios" => apple("TARGET_OS_IOS"),
					"tvos" => apple("TARGET_OS_TV"),
					"watchos" => apple("TARGET_OS_WATCH"),
					"visionos" => apple("TARGET_OS_VISION"),
					_ => None,
				},
				match environment {
					"sim" => apple("TARGET_OS_SIMULATOR"),
					"macabi" => apple("TARGET_OS_MACCATALYST"),
					_ => s("!(defined(__APPLE__) && (TARGET_OS_SIMULATOR || TARGET_OS_MACCATALYST))"),
				},
			]
		}
		Lang::Swift => [
			// Swift tests a pointer width only through an underscored condition, and its
			// architectures already tell one apart.
			None,
			match (arch, width) {
				("x86_64", "64") => s("arch(x86_64)"),
				("x86", _) => s("arch(i386)"),
				("aarch64", "64") => s("arch(arm64)"),
				("aarch64", "32") => s("arch(arm64_32)"),
				("arm", _) => s("arch(arm)"),
				("wasm32", _) => s("arch(wasm32)"),
				_ => None,
			},
			match os {
				"macos" => s("os(macOS)"),
				"ios" => s("os(iOS)"),
				"tvos" => s("os(tvOS)"),
				"watchos" => s("os(watchOS)"),
				"visionos" => s("os(visionOS)"),
				"linux" => s("os(Linux)"),
				"android" => s("os(Android)"),
				"windows" => s("os(Windows)"),
				"freebsd" => s("os(FreeBSD)"),
				"wasi" => s("os(WASI)"),
				_ => None,
			},
			match environment {
				"sim" => s("targetEnvironment(simulator)"),
				"macabi" => s("targetEnvironment(macCatalyst)"),
				_ => s("(!targetEnvironment(simulator) && !targetEnvironment(macCatalyst))"),
			},
		],
		// Go links only the targets of `go::PLATFORMS`, none an Apple simulator.
		Lang::Go => [
			None,
			match (arch, width) {
				("x86_64", "64") => s("amd64"),
				("aarch64", "64") => s("arm64"),
				_ => None,
			},
			match os {
				// Go's `linux` holds on Android too, and its `darwin` on iOS.
				"linux" => s("(linux && !android)"),
				"android" => s("android"),
				"macos" => s("(darwin && !ios)"),
				"ios" => s("ios"),
				"windows" => s("windows"),
				_ => None,
			},
			None,
		],
	}
}

/// The targets of `api` that `lang`'s binding serves, by index: Go's those it links.
fn served(api: &Api, lang: Lang) -> Vec<usize> {
	(api.targets.iter().enumerate())
		.filter(|(_, target)| lang != Lang::Go || crate::go::PLATFORMS.iter().any(|p| p.0 == target.as_str()))
		.map(|(i, _)| i)
		.collect()
}

/// Where `lang`'s binding of `api` declares an item on `on`, or why it cannot tell those
/// targets apart from its others at compile time.
pub(crate) fn gate(api: &Api, lang: Lang, on: On) -> Result<Gate, String> {
	let served = served(api, lang);
	let (inside, outside): (Vec<usize>, Vec<usize>) = served.iter().partition(|&&i| on.0 >> i & 1 == 1);
	if outside.is_empty() {
		return Ok(Gate::Everywhere);
	}
	if inside.is_empty() {
		return Ok(Gate::Nowhere);
	}
	let mut of = Vec::new();
	for &i in &served {
		let target = &api.targets[i];
		let Some(cfgs) = api.profiles.get(target).and_then(|p| p.cfgs.first()) else {
			return Err(format!(
				"the staged API of {target} names no cfgs, which {}'s binding gates an item only some targets have by: stage it again with this rust2-ffi",
				lang.name()
			));
		};
		of.push((i, traits(lang, cfgs)));
	}
	let traits = |i: usize| &of.iter().find(|(t, _)| *t == i).expect("a served target").1;
	// A trait some target has no condition for tells none apart: on that target, every
	// other value's condition would have to be false, which no condition says.
	let usable: Vec<usize> = (0..TRAITS)
		.filter(|&k| served.iter().all(|&i| traits(i)[k].is_some()))
		.collect();
	// Each set of usable traits, the fewest first, then by the order of [`TRAITS`].
	let mut sets: Vec<Vec<usize>> = (1..1u32 << usable.len())
		.map(|bits| {
			(0..usable.len())
				.filter(|b| bits >> b & 1 == 1)
				.map(|b| usable[b])
				.collect()
		})
		.collect();
	sets.sort_by_key(|set: &Vec<usize>| set.len());
	let key = |i: usize, set: &[usize]| -> Vec<String> {
		set.iter()
			.map(|&k| traits(i)[k].clone().expect("a usable trait"))
			.collect()
	};
	for set in sets {
		let keys = |targets: &[usize]| {
			let mut keys: Vec<Vec<String>> = Vec::new();
			for &i in targets {
				let key = key(i, &set);
				if !keys.contains(&key) {
					keys.push(key);
				}
			}
			keys
		};
		let (yes, no) = (keys(&inside), keys(&outside));
		if yes.iter().any(|key| no.contains(key)) {
			continue;
		}
		// The negation only where it names fewer: it also holds on a target the binding
		// does not serve.
		return Ok(Gate::If(match no.len() < yes.len() {
			true => format!("!{}", group(&any(&no))),
			false => any(&yes),
		}));
	}
	let named = |targets: &[usize]| {
		let names: Vec<&str> = targets.iter().map(|&i| api.targets[i].as_str()).collect();
		names.join(", ")
	};
	Err(format!(
		"{}'s binding, which declares it only where it compiles for one of {}, cannot tell those targets apart from {} at compile time",
		lang.name(),
		named(&inside),
		named(&outside),
	))
}

/// The condition that holds where any of `keys` does, each holding where all of its own do.
fn any(keys: &[Vec<String>]) -> String {
	let terms: Vec<String> = keys
		.iter()
		.map(|key| match key.len() {
			1 => key[0].clone(),
			_ => group(&key.join(" && ")),
		})
		.collect();
	terms.join(" || ")
}

/// `condition` as one operand: in parentheses unless it is one already.
fn group(condition: &str) -> String {
	let whole = condition.starts_with('(') && {
		// Whether its first parenthesis closes at its end.
		let mut depth = 0;
		let mut closes = condition.len();
		for (at, c) in condition.char_indices() {
			match c {
				'(' => depth += 1,
				')' => {
					depth -= 1;
					if depth == 0 {
						closes = at;
						break;
					}
				}
				_ => {}
			}
		}
		closes == condition.len() - 1
	};
	let bare = !condition.contains(' ');
	match whole || bare {
		true => condition.to_string(),
		false => format!("({condition})"),
	}
}

/// Each constant some compile-time binding of `api` cannot gate, with why.
pub(crate) fn refusals(api: &Api) -> Vec<(&Constant, String)> {
	let mut refused = Vec::new();
	for constant in api.constants.iter().filter(|c| !api.everywhere(c.on)) {
		for lang in [Lang::C, Lang::Swift, Lang::Go] {
			if let Err(why) = gate(api, lang, constant.on) {
				refused.push((constant, why));
			}
		}
	}
	refused
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ir::Profile;

	/// An API serving each of `targets`, as `(triple, pointer width, arch, os, env, abi)`.
	fn api(targets: &[(&str, &str, &str, &str, &str, &str)]) -> Api {
		let mut api = Api::of(vec![]);
		api.targets.clear();
		for &(triple, width, arch, os, env, abi) in targets {
			api.targets.push(triple.to_string());
			let cfgs = [
				("TARGET_POINTER_WIDTH", width),
				("TARGET_ARCH", arch),
				("TARGET_OS", os),
				("TARGET_ENV", env),
				("TARGET_ABI", abi),
			];
			let cfgs = cfgs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
			let profile = Profile {
				root: "debug".into(),
				dir: Some("debug".into()),
				opt_level: "0".into(),
				debug_assertions: true,
				panic: "unwind".into(),
				debug: true,
				cfgs: vec![cfgs],
				vars: Default::default(),
			};
			api.profiles.insert(triple.to_string(), profile);
		}
		api
	}

	const LINUX: (&str, &str, &str, &str, &str, &str) =
		("x86_64-unknown-linux-gnu", "64", "x86_64", "linux", "gnu", "");
	const MUSL: (&str, &str, &str, &str, &str, &str) =
		("x86_64-unknown-linux-musl", "64", "x86_64", "linux", "musl", "");
	const WASM: (&str, &str, &str, &str, &str, &str) = ("wasm32-wasip1", "32", "wasm32", "wasi", "p1", "");
	const X64_ANDROID: (&str, &str, &str, &str, &str, &str) =
		("x86_64-linux-android", "64", "x86_64", "android", "", "");
	const X86_ANDROID: (&str, &str, &str, &str, &str, &str) = ("i686-linux-android", "32", "x86", "android", "", "");
	const ANDROID: (&str, &str, &str, &str, &str, &str) = ("aarch64-linux-android", "64", "aarch64", "android", "", "");
	const IOS: (&str, &str, &str, &str, &str, &str) = ("aarch64-apple-ios", "64", "aarch64", "ios", "", "");
	const SIM: (&str, &str, &str, &str, &str, &str) = ("aarch64-apple-ios-sim", "64", "aarch64", "ios", "", "sim");
	const MAC: (&str, &str, &str, &str, &str, &str) = ("aarch64-apple-darwin", "64", "aarch64", "macos", "", "");
	const WINDOWS: (&str, &str, &str, &str, &str, &str) =
		("x86_64-pc-windows-gnu", "64", "x86_64", "windows", "gnu", "");

	fn gated(lang: Lang, targets: &[(&str, &str, &str, &str, &str, &str)], on: u64) -> Result<Gate, String> {
		gate(&api(targets), lang, On(on))
	}

	fn when(condition: &str) -> Result<Gate, String> {
		Ok(Gate::If(condition.to_string()))
	}

	/// A 64-bit constant, as the api fixture's `WORD`, is gated by the pointer width alone in
	/// C, whose architectures would say it too, and by the architecture in Swift.
	#[test]
	fn word() {
		let targets = [LINUX, WASM];
		assert_eq!(
			gated(Lang::C, &targets, 0b01),
			when("UINTPTR_MAX == 0xFFFFFFFFFFFFFFFFu")
		);
		assert_eq!(gated(Lang::Swift, &targets, 0b01), when("arch(x86_64)"));
		// Go links no wasm32, so its binding has it everywhere it serves.
		assert_eq!(gated(Lang::Go, &targets, 0b01), Ok(Gate::Everywhere));
		assert_eq!(gated(Lang::Go, &targets, 0b10), Ok(Gate::Nowhere));
		assert_eq!(gated(Lang::C, &targets, 0b11), Ok(Gate::Everywhere));
	}

	/// Where no one trait tells the targets apart, two do, as an `||` of `&&`s, or the
	/// negation of the rest where that is shorter.
	#[test]
	fn combined() {
		// Only the 64-bit x86 Android: neither its width, its architecture, nor its system
		// alone says so; Go links no 32-bit Android, so its system does there.
		let targets = [LINUX, X64_ANDROID, X86_ANDROID];
		assert_eq!(
			gated(Lang::C, &targets, 0b010),
			when("(UINTPTR_MAX == 0xFFFFFFFFFFFFFFFFu && defined(__ANDROID__))")
		);
		assert_eq!(gated(Lang::Go, &targets, 0b010), when("android"));
		// All but Windows.
		let targets = [LINUX, MAC, IOS, ANDROID, WINDOWS];
		assert_eq!(gated(Lang::Swift, &targets, 0b01111), when("!os(Windows)"));
		assert_eq!(gated(Lang::Go, &targets, 0b01111), when("!windows"));
		assert_eq!(
			gated(Lang::Go, &targets, 0b00011),
			when("(linux && !android) || (darwin && !ios)")
		);
	}

	/// An Apple simulator differs from the device only by its environment.
	#[test]
	fn simulator() {
		let targets = [IOS, SIM, MAC];
		assert_eq!(
			gated(Lang::Swift, &targets, 0b010),
			when("targetEnvironment(simulator)")
		);
		assert_eq!(
			gated(Lang::C, &targets, 0b010),
			when("(defined(__APPLE__) && TARGET_OS_SIMULATOR)")
		);
		assert_eq!(
			gated(Lang::Swift, &targets, 0b001),
			when("(os(iOS) && (!targetEnvironment(simulator) && !targetEnvironment(macCatalyst)))")
		);
	}

	/// A trait no language tests, such as Linux's C library, tells no targets apart.
	#[test]
	fn refused() {
		let targets = [LINUX, MUSL];
		let why = gated(Lang::C, &targets, 0b01).unwrap_err();
		assert!(
			why.contains(
				"one of x86_64-unknown-linux-gnu, cannot tell those targets apart from x86_64-unknown-linux-musl"
			),
			"{why}"
		);
		let mut api = api(&[LINUX, WASM]);
		api.profiles.clear();
		let why = gate(&api, Lang::C, On(0b01)).unwrap_err();
		assert!(why.contains("names no cfgs"), "{why}");
	}

	#[test]
	fn groups() {
		assert_eq!(group("a"), "a");
		assert_eq!(group("(a || b)"), "(a || b)");
		assert_eq!(group("(a) || (b)"), "((a) || (b))");
		assert_eq!(group("a && b"), "(a && b)");
	}
}
