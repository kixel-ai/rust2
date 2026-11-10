//! `#[track_caller]`, as rustc's codegen passes it: each function that requires its
//! caller's `panic::Location` takes it as a last parameter, and each call the crate
//! writes of one passes the location of the call, or, in a function that takes one
//! itself, the one its own caller passed. A function pointer to one, or a trait object's
//! method that only its impl tracks, passes the function's own location, as rustc's shim
//! does. A call through std's code passes a location in std's source, which translation
//! can't name, so it is refused.
//!
//! A program that reaches `panic::set_hook` passes where each panic is, which its hook
//! reads: each statement's own, the caller's location to each std function that takes it,
//! and the call of the macro that defines one, as an integer's methods. A panic in std's
//! own code is where the toolchain's source has it, which `rs.setHook` is given, with one in
//! `bytes`' where its compiled code has it. One in tokio's is where its compiled code has
//! it, passed at the call.

use std::sync::OnceLock;

use rustc_attr_ir::LangItem;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use rustc_middle::mir::TerminatorKind;
use rustc_middle::ty::{Instance, InstanceKind, TyCtxt};
use rustc_span::{RemapPathScopeComponents, SourceFile, Span};

use super::Translator;
use super::body::Body;
use super::library::{self, Std};
use crate::ts::{Expr, Key, Param, Stmt, StmtKind, Type};

/// The `rs.Location` rustc gives a call at `span`: where the outermost macro call it came
/// from is, in the file as the native build names it, by line and column from 1. The column
/// counts a tab as 4, which rustc reads from the file's source, so a dependency's file,
/// imported without it, is loaded first.
pub fn location(tcx: TyCtxt<'_>, span: Span) -> Expr {
	let topmost = span.ctxt().outer_expn().expansion_cause().unwrap_or(span);
	let map = tcx.sess.source_map();
	map.ensure_source_file_source_present(&map.lookup_source_file(topmost.lo()));
	let pos = map.lookup_char_pos(topmost.lo());
	object(file(tcx, &pos.file), pos.line, pos.col_display + 1)
}

/// Where the toolchain's prebuilt std remaps the crates.io crates it is built from, as
/// `hashbrown`: a directory the toolchain's build picked, which nothing else names.
const STD_DEPS: &str = "/cargo/registry/25cdd57fae9f0462";

/// The name of `file` as the session names it, or for std's own source, as the toolchain's
/// prebuilt std does, which the native build links: by the commit it was built from. The
/// native build names std's code it compiles into the crate, as a generic or inlined
/// function, by the sysroot's copy of the source instead, where the toolchain has one: a
/// path of the machine that built it, which this doesn't follow.
fn file(tcx: TyCtxt<'_>, file: &SourceFile) -> String {
	let name = file
		.name
		.display(RemapPathScopeComponents::MACRO)
		.to_string_lossy()
		.into_owned();
	if !super::of_std(tcx, file.cnum) {
		return name;
	}
	match name.split_once("/library/") {
		Some((_, rest)) => std_file(tcx, rest),
		None => name,
	}
}

/// The name the toolchain's prebuilt std gives `rest`, a file under `library/`.
fn std_file(tcx: TyCtxt<'_>, rest: &str) -> String {
	match rest.strip_prefix("vendor/") {
		Some(dep) => format!("{STD_DEPS}/{dep}"),
		None => format!("/rustc/{}/library/{rest}", commit(tcx)),
	}
}

/// The commit the wrapped rustc was built from, as `rustc -vV` says.
fn commit(tcx: TyCtxt<'_>) -> &'static str {
	static COMMIT: OnceLock<Option<String>> = OnceLock::new();
	let commit = COMMIT.get_or_init(|| {
		let text = rustc(&["-vV"])?;
		text.lines()
			.find_map(|line| line.strip_prefix("commit-hash: "))
			.map(str::to_string)
	});
	match commit {
		Some(commit) => commit,
		None => tcx
			.dcx()
			.fatal("rust2-ts: the wrapped rustc's verbose version names no commit, which std's files are named by"),
	}
}

/// What the wrapped rustc prints given `args`.
fn rustc(args: &[&str]) -> Option<String> {
	let rustc = crate::driver::RUSTC.get()?;
	let out = std::process::Command::new(rustc).args(args).output().ok()?;
	String::from_utf8(out.stdout).ok()
}

/// Where std's code panics outside a `#[track_caller]` function, for each of its panics the
/// runtime makes, by the key it passes in place of a location: a file under `library/`, `*`
/// standing for a vendored crate's version, the text before it, and the text the call
/// that panics starts with, as a method call's location starts at its name.
const SITES: &[(&str, &str, &str, &str)] = &[
	(
		"capacity",
		"alloc/src/raw_vec/mod.rs",
		"fn capacity_overflow",
		"panic!(",
	),
	(
		"trusted_capacity",
		"alloc/src/vec/spec_from_iter_nested.rs",
		"",
		"panic!(\"capacity overflow\")",
	),
	("swap_remove", "alloc/src/vec/mod.rs", "pub fn swap_remove", "panic!("),
	(
		"vec_set_len",
		"alloc/src/vec/mod.rs",
		"pub const unsafe fn set_len(",
		"ub_checks::assert_unsafe_precondition!(",
	),
	(
		"into_flattened",
		"alloc/src/vec/mod.rs",
		"pub fn into_flattened",
		"expect(",
	),
	(
		"rotate_left",
		"core/src/slice/mod.rs",
		"pub const fn rotate_left",
		"assert!(",
	),
	(
		"rotate_right",
		"core/src/slice/mod.rs",
		"pub const fn rotate_right",
		"assert!(",
	),
	(
		"ord_violation",
		"core/src/slice/sort/shared/smallsort.rs",
		"fn panic_on_ord_violation",
		"panic!(",
	),
	(
		"ord_clamp",
		"core/src/cmp.rs",
		"fn clamp(self, min: Self, max: Self)",
		"assert!(",
	),
	(
		"f32_clamp",
		"core/src/num/f32.rs",
		"pub const fn clamp(",
		"const_assert!(",
	),
	(
		"f64_clamp",
		"core/src/num/f64.rs",
		"pub const fn clamp(",
		"const_assert!(",
	),
	(
		"encode_utf8",
		"core/src/char/methods.rs",
		"pub const fn encode_utf8_raw",
		"const_panic!(",
	),
	(
		"encode_utf16",
		"core/src/char/methods.rs",
		"pub const fn encode_utf16_raw",
		"const_panic!(",
	),
	("format", "alloc/src/fmt.rs", "fn format_inner", "expect("),
	(
		"to_string",
		"alloc/src/string.rs",
		"",
		"expect(\"a Display implementation",
	),
	(
		"debug_struct_fields",
		"core/src/fmt/mod.rs",
		"pub fn debug_struct_fields_finish",
		"assert_eq!(",
	),
	(
		"exact_len",
		"core/src/iter/traits/exact_size.rs",
		"let (lower, upper) = self.size_hint();",
		"assert_eq!(",
	),
	("join_len", "alloc/src/str.rs", "fn join_generic_copy", "expect("),
	(
		"int_sum",
		"core/src/iter/traits/accum.rs",
		"macro_rules! float_sum_product",
		"integer_sum_product! {",
	),
	(
		"int_forward",
		"core/src/iter/range.rs",
		"#[cfg(target_pointer_width = \"32\")]",
		"step_integer_impls! {",
	),
	(
		"char_forward",
		"core/src/iter/range.rs",
		"fn forward(start: Self, count: usize) -> Self {",
		"expect(",
	),
	(
		"hash_capacity",
		"vendor/hashbrown-*/src/raw.rs",
		"",
		"panic!(\"Hash table capacity overflow\")",
	),
	(
		"btree_range_equal_BTreeSet",
		"alloc/src/collections/btree/search.rs",
		"",
		"panic!(\"range start and end are equal and excluded in BTreeSet\")",
	),
	(
		"btree_range_equal_BTreeMap",
		"alloc/src/collections/btree/search.rs",
		"",
		"panic!(\"range start and end are equal and excluded in BTreeMap\")",
	),
	(
		"btree_range_order_BTreeSet",
		"alloc/src/collections/btree/search.rs",
		"",
		"panic!(\"range start is greater than range end in BTreeSet\")",
	),
	(
		"btree_range_order_BTreeMap",
		"alloc/src/collections/btree/search.rs",
		"",
		"panic!(\"range start is greater than range end in BTreeMap\")",
	),
	(
		"deque_insert",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn insert_mut(",
		"assert!(",
	),
	(
		"deque_swap_i",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn swap(&mut self, i: usize, j: usize)",
		"assert!(i",
	),
	(
		"deque_swap_j",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn swap(&mut self, i: usize, j: usize)",
		"assert!(j",
	),
	(
		"deque_rotate_left",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn rotate_left(",
		"assert!(",
	),
	(
		"deque_rotate_right",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn rotate_right(",
		"assert!(",
	),
	(
		"deque_split_off",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn split_off(",
		"assert!(",
	),
	(
		"list_split_off",
		"alloc/src/collections/linked_list.rs",
		"pub fn split_off(",
		"assert!(",
	),
	(
		"mutex_lock",
		"std/src/sys/sync/mutex/no_threads.rs",
		"pub fn lock(",
		"assert_eq!(",
	),
	(
		"once_force",
		"std/src/sync/once.rs",
		"pub fn call_once_force<",
		"call(true",
	),
	(
		"reentrant_init",
		"core/src/cell/once.rs",
		"",
		"panic!(\"reentrant init\")",
	),
	("lazy_poisoned", "core/src/cell/lazy.rs", "fn panic_poisoned", "panic!("),
	("ready_poll", "core/src/future/ready.rs", "fn poll(", "expect("),
	(
		"ready_into_inner",
		"core/src/future/ready.rs",
		"pub fn into_inner(",
		"expect(",
	),
	("duration_new", "core/src/time.rs", "pub const fn new(", "expect("),
	(
		"duration_from_mins",
		"core/src/time.rs",
		"pub const fn from_mins(",
		"panic!(",
	),
	(
		"duration_from_hours",
		"core/src/time.rs",
		"pub const fn from_hours(",
		"panic!(",
	),
	("duration_add", "core/src/time.rs", "", "expect(\"adding durations"),
	("duration_sub", "core/src/time.rs", "", "expect(\"subtracting durations"),
	(
		"duration_mul",
		"core/src/time.rs",
		"",
		"expect(\"multiplying duration by scalar",
	),
	// A macro's panic is at the macro's call, here the one that gives the general zip its
	// `next_back`, which an `ExactSizeIterator` whose length lies reaches.
	(
		"zip_next_back",
		"core/src/iter/adapters/zip.rs",
		"// General Zip impl",
		"zip_impl_general_defaults!",
	),
];

/// Where each std function that neither takes its caller's location nor is defined by a
/// macro panics, for those whose mapping makes a panic that takes its location, by the
/// path [`library::std_path`] gives it: as [`SITES`] says.
const CALLEES: &[(&str, &str, &str, &str)] = &[
	("vec::drain", "alloc/src/vec/mod.rs", "pub fn drain<R>", "slice::range("),
	(
		"vec::splice",
		"alloc/src/vec/mod.rs",
		"pub fn drain<R>",
		"slice::range(",
	),
	(
		"vec::extend_from_within",
		"alloc/src/vec/mod.rs",
		"pub fn extend_from_within<R>",
		"slice::range(",
	),
	(
		"collections::vec_deque::drain",
		"alloc/src/collections/vec_deque/mod.rs",
		"pub fn drain<R>",
		"slice::range(",
	),
	(
		"collections::vec_deque::range",
		"alloc/src/collections/vec_deque/mod.rs",
		"fn slice_ranges<R>",
		"slice::range(",
	),
	(
		"collections::vec_deque::range_mut",
		"alloc/src/collections/vec_deque/mod.rs",
		"fn slice_ranges<R>",
		"slice::range(",
	),
	(
		"vec::extract_if",
		"alloc/src/vec/extract_if.rs",
		"pub(super) fn new<R",
		"slice::range(",
	),
	(
		"iter::traits::iterator::Iterator::step_by",
		"core/src/iter/adapters/step_by.rs",
		"fn new(",
		"assert!(",
	),
	(
		"char::methods::from_digit",
		"core/src/char/convert.rs",
		"const fn from_digit(",
		"panic!(",
	),
	(
		"sync::atomic::fence",
		"core/src/sync/atomic.rs",
		"pub const fn fence(",
		"panic!(",
	),
	(
		"sync::atomic::compiler_fence",
		"core/src/sync/atomic.rs",
		"pub const fn compiler_fence(",
		"panic!(",
	),
	(
		"time::from_secs_f64",
		"core/src/time.rs",
		"pub fn from_secs_f64(secs: f64)",
		"panic!(",
	),
	(
		"time::mul_f64",
		"core/src/time.rs",
		"pub fn from_secs_f64(secs: f64)",
		"panic!(",
	),
	(
		"time::mul_f32",
		"core/src/time.rs",
		"pub fn from_secs_f64(secs: f64)",
		"panic!(",
	),
	(
		"time::div_f64",
		"core/src/time.rs",
		"pub fn from_secs_f64(secs: f64)",
		"panic!(",
	),
	(
		"time::div_f32",
		"core/src/time.rs",
		"pub fn from_secs_f64(secs: f64)",
		"panic!(",
	),
	(
		"time::from_secs_f32",
		"core/src/time.rs",
		"pub fn from_secs_f32(secs: f32)",
		"panic!(",
	),
	(
		"io::io_slice::advance",
		"core/src/io/io_slice/repr_generic.rs",
		"pub(crate) fn advance(&mut self, n: usize)",
		"[n..]",
	),
	(
		"io::io_slice::advance_slices",
		"core/src/io/io_slice.rs",
		"pub fn advance_slices(bufs: &mut &mut [IoSlice<'a>], n: usize)",
		"assert!(",
	),
	(
		"mem::maybe_uninit::write_copy_of_slice",
		"core/src/mem/maybe_uninit.rs",
		"pub const fn write_copy_of_slice(",
		"copy_from_slice(",
	),
	(
		"mem::maybe_uninit::write_clone_of_slice",
		"core/src/mem/maybe_uninit.rs",
		"pub fn write_clone_of_slice(",
		"assert_eq!(",
	),
	("cell::take", "core/src/cell.rs", "pub fn take(&self) -> T", "replace("),
	(
		"thread::local::with_borrow_mut",
		"std/src/thread/local.rs",
		"pub fn with_borrow_mut<F, R>",
		"borrow_mut(",
	),
	(
		"thread::local::with_borrow",
		"std/src/thread/local.rs",
		"pub fn with_borrow<F, R>",
		"borrow(",
	),
];

/// Where `bytes`' code panics, for each of its panics the runtime makes, by the key it
/// passes in place of a location: the function, by its path, and which of its calls that
/// panic it is, in the order of its source, from 0. None of its public functions takes its
/// caller's location, so each panic is where its compiled code has it.
const BYTES: &[(&str, &str, usize)] = &[
	("bytes_advance", "bytes::panic_advance", 0),
	("bytes_does_not_fit", "bytes::panic_does_not_fit", 0),
	// `range` takes its caller's location, so its panics are at its call.
	("bytes_slice", "bytes::Bytes::slice", 0),
	("bytes_slice_ref_start", "bytes::Bytes::slice_ref", 0),
	("bytes_slice_ref_end", "bytes::Bytes::slice_ref", 1),
	("bytes_split_off", "bytes::Bytes::split_off", 0),
	("bytes_split_to", "bytes::Bytes::split_to", 0),
	("bytes_buf_advance", "<bytes::Bytes as bytes::Buf>::advance", 0),
	("bytes_mut_split_off", "bytes::BytesMut::split_off", 0),
	("bytes_mut_split_to", "bytes::BytesMut::split_to", 0),
	("bytes_mut_set_len", "bytes::BytesMut::set_len", 0),
	("bytes_mut_overflow", "bytes::BytesMut::reserve_inner", 0),
	("bytes_mut_overflow_offset", "bytes::BytesMut::reserve_inner", 1),
	("bytes_mut_buf_advance", "<bytes::BytesMut as bytes::Buf>::advance", 0),
];

/// The [`BYTES`] sites of a `debug_assert!`, which a build without debug assertions leaves
/// out, as Rust leaves a call past it undefined.
const BYTES_DEBUG: &[&str] = &["bytes_mut_set_len"];

/// The `rs.Sites` of the toolchain's std, which `rs.setHook` takes: where each panic
/// [`SITES`] names is, as the native build names it, and each [`BYTES`] names, in the
/// program's `bytes`, if it has one.
pub fn sites(t: &mut Translator<'_, '_>, span: Span) -> Expr {
	let tcx = t.tcx;
	let mut props = Vec::new();
	for &(key, file, after, needle) in SITES {
		match find(tcx, file, &[after], needle) {
			Some(at) => props.push((Key::Name(key.into()), at)),
			None => {
				t.err(
					span,
					format!("std's source has no `{needle}` in `{file}` after `{after}`, where it panics"),
				);
			}
		}
	}
	props.extend(bytes_sites(t, span));
	Expr::Object(props)
}

/// The sites [`BYTES`] names, in the program's `bytes`: none without one, and an error for
/// versions whose sites differ, as the runtime's panics are one crate's.
fn bytes_sites(t: &mut Translator<'_, '_>, span: Span) -> Vec<(Key, Expr)> {
	let tcx = t.tcx;
	let mut tables: Vec<Vec<(Key, Expr)>> = Vec::new();
	for &krate in tcx.crates(()) {
		if tcx.crate_name(krate).as_str() != "bytes" {
			continue;
		}
		let mut fns = rustc_data_structures::fx::FxHashMap::default();
		let count = rustc_metadata::creader::CStore::from_tcx(tcx).num_def_ids_untracked(krate);
		for index in 0..count {
			let def_id = DefId {
				krate,
				index: rustc_hir::def_id::DefIndex::from_usize(index),
			};
			if matches!(tcx.def_kind(def_id), DefKind::Fn | DefKind::AssocFn) {
				fns.insert(
					rustc_middle::ty::print::with_no_trimmed_paths!(tcx.def_path_str(def_id)),
					def_id,
				);
			}
		}
		let mut props = Vec::new();
		for &(key, path, nth) in BYTES {
			let found = fns.get(path).map(|&f| panics(tcx, f, true).into_iter().nth(nth));
			match found {
				Some(Some(at)) => props.push((Key::Name(key.into()), location(tcx, at))),
				Some(None) if BYTES_DEBUG.contains(&key) => {}
				_ => {
					t.err(
						span,
						format!("the bytes crate's compiled code has no panic {nth} in `{path}`, counting from 0"),
					);
				}
			}
		}
		if !tables.contains(&props) {
			tables.push(props);
		}
	}
	if tables.len() > 1 {
		t.err(
			span,
			"a panic hook in a program of two versions of the bytes crate, whose panics are at different places",
		);
	}
	tables.pop().unwrap_or_default()
}

/// The calls in the MIR `def_id`'s crate compiled for it that panic, outside what that
/// inlined, in the order of its source, by the macro call each came from: of std's `panic!`,
/// which `assert!` makes too, and if `tracked`, of a function of its own crate that takes its
/// caller's location, whose panics are at the call. None without that MIR.
fn panics(tcx: TyCtxt<'_>, def_id: DefId, tracked: bool) -> Vec<Span> {
	if !tcx.is_mir_available(def_id) {
		return Vec::new();
	}
	let mir = tcx.optimized_mir(def_id);
	let mut spans: Vec<Span> = (mir.basic_blocks.iter())
		.filter_map(|block| {
			let terminator = block.terminator();
			let TerminatorKind::Call { func, .. } = &terminator.kind else {
				return None;
			};
			let (callee, _) = func.const_fn_def()?;
			let panics = [LangItem::PanicFmt, LangItem::Panic, LangItem::PanicDisplay]
				.into_iter()
				.any(|item| tcx.is_lang_item(callee, item))
				|| (tracked
					&& callee.krate == def_id.krate
					&& tcx
						.codegen_fn_attrs(callee)
						.flags
						.contains(CodegenFnAttrFlags::TRACK_CALLER));
			let own = (terminator.source_info.scope)
				.inlined_instance(&mir.source_scopes)
				.is_none();
			let span = terminator.source_info.span;
			(panics && own).then(|| span.ctxt().outer_expn().expansion_cause().unwrap_or(span))
		})
		.collect();
	spans.sort_by_key(|span| span.lo());
	spans.dedup();
	spans
}

/// Where `def_id`, a dependency's function that takes no caller's location, makes its one
/// `panic!`: the call in the MIR its crate compiled, outside what that inlined, whose file is
/// named as the native build names it under any `--remap-path-prefix`. `None` without that
/// MIR, or with other than one such call.
pub fn own_panic(tcx: TyCtxt<'_>, def_id: DefId) -> Option<Expr> {
	match panics(tcx, def_id, false)[..] {
		[span] => Some(location(tcx, span)),
		_ => None,
	}
}

/// Where std's `def_id` panics, if [`CALLEES`] names it.
pub fn callee_site(tcx: TyCtxt<'_>, def_id: DefId) -> Option<Expr> {
	let path = library::std_path(tcx, def_id).join("::");
	let &(_, file, after, needle) = CALLEES.iter().find(|(callee, ..)| *callee == path)?;
	Some(found(tcx, file, &[after], needle))
}

/// Where std's `Sum` of `Duration`s panics, or of `&Duration`s if `by_ref`, or else an
/// integer's `Sum` or `Product`: the call of the macro its impl is, as a macro's panic is
/// where it is called. `Iterator::sum` reaches each, so no one place in [`CALLEES`] holds.
pub fn sum_site(tcx: TyCtxt<'_>, duration: Option<bool>) -> Expr {
	match duration {
		Some(false) => found(tcx, "core/src/time.rs", &["impl Sum for Duration"], "sum_durations!("),
		Some(true) => found(
			tcx,
			"core/src/time.rs",
			&["impl<'a> Sum<&'a Duration>"],
			"sum_durations!(",
		),
		None => site(tcx, "int_sum"),
	}
}

/// Where the panic [`SITES`] names `key` is.
fn site(tcx: TyCtxt<'_>, key: &str) -> Expr {
	let &(_, file, after, needle) = SITES.iter().find(|(site, ..)| *site == key).expect("a key of SITES");
	found(tcx, file, &[after], needle)
}

/// Where a `RefCell`'s `method`, of `PartialEq`, `PartialOrd`, or `Ord`, borrows each: its
/// own, then the other's, either of which panics while it is mutably borrowed.
pub fn cell_sites(tcx: TyCtxt<'_>, method: &str) -> [Expr; 2] {
	let header = format!("fn {method}(&self, other: &RefCell<T>)");
	[
		found(tcx, "core/src/cell.rs", &[&header], "borrow("),
		found(tcx, "core/src/cell.rs", &[&header, "other."], "borrow("),
	]
}

/// [`find`], which std's source always has.
fn found(tcx: TyCtxt<'_>, file: &str, after: &[&str], needle: &str) -> Expr {
	match find(tcx, file, after, needle) {
		Some(at) => at,
		None => tcx.dcx().fatal(format!(
			"rust2-ts: std's source has no `{needle}` in `{file}` after `{}`, where it panics",
			after.join(" then ")
		)),
	}
}

/// The `rs.Location` of the first `needle` after each of `after` in turn in `file`, a file
/// under std's `library/` whose `*` stands for a vendored crate's version, in the
/// toolchain's source.
fn find(tcx: TyCtxt<'_>, file: &str, after: &[&str], needle: &str) -> Option<Expr> {
	type Sources = rustc_data_structures::fx::FxHashMap<String, Option<(String, String)>>;
	static SOURCES: std::sync::Mutex<Option<Sources>> = std::sync::Mutex::new(None);
	static LIBRARY: OnceLock<Option<String>> = OnceLock::new();
	let library = LIBRARY.get_or_init(|| {
		let root = rustc(&["--print", "sysroot"])?;
		Some(format!("{}/lib/rustlib/src/rust/library", root.trim()))
	});
	let mut sources = SOURCES.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
	let source = sources
		.get_or_insert_default()
		.entry(file.to_string())
		.or_insert_with(|| {
			let dir = library.as_deref()?;
			let path = match file.split_once('*') {
				Some((prefix, suffix)) => {
					let (parent, name) = prefix.rsplit_once('/')?;
					let found = (std::fs::read_dir(format!("{dir}/{parent}")).ok()?)
						.filter_map(Result::ok)
						.find(|d| d.file_name().to_string_lossy().starts_with(name))?;
					format!("{parent}/{}{suffix}", found.file_name().to_string_lossy())
				}
				None => file.to_string(),
			};
			Some((std::fs::read_to_string(format!("{dir}/{path}")).ok()?, path))
		});
	let (text, path) = source.as_ref()?;
	let (line, col) = position(text, after, needle)?;
	Some(object(std_file(tcx, path), line, col))
}

/// The line and column, from 1, of the first `needle` after each of `after` in turn in `text`.
fn position(text: &str, after: &[&str], needle: &str) -> Option<(usize, usize)> {
	let mut from = 0;
	for after in after {
		from += text[from..].find(after)? + after.len();
	}
	let at = from + text[from..].find(needle)?;
	let line_start = text[..at].rfind('\n').map_or(0, |i| i + 1);
	let line = text[..at].matches('\n').count() + 1;
	let col = text[line_start..at].chars().count() + 1;
	Some((line, col))
}

/// The `rs.Location` of `file`, `line`, and `col`.
pub fn object(file: String, line: impl ToString, col: impl ToString) -> Expr {
	Expr::Object(vec![
		(Key::Name("file".into()), Expr::Str(file)),
		(Key::Name("line".into()), Expr::lit(line.to_string())),
		(Key::Name("col".into()), Expr::lit(col.to_string())),
	])
}

/// Whether a call of `instance` passes its caller's location: the crate's own, as std's
/// read it only to say where they panic, which the runtime's functions take instead.
pub fn tracks<'tcx>(t: &Translator<'_, 'tcx>, instance: Instance<'tcx>) -> bool {
	t.crates.translates(instance.def_id()) && instance.def.requires_caller_location(t.tcx)
}

/// Whether `def_id` is `Location::caller`, which gives the location its call passes.
pub fn is_caller(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
	library::inherent_std(tcx, def_id) == Some(Std::Location) && tcx.item_name(def_id).as_str() == "caller"
}

/// Whether a call of the crate's trait method `def_id` through a trait object passes its
/// caller's location, as its declaration's `#[track_caller]` says.
pub fn tracks_virtual(t: &Translator<'_, '_>, def_id: DefId) -> bool {
	t.crates.translates(def_id) && InstanceKind::Virtual(def_id, 0).requires_caller_location(t.tcx)
}

impl<'tcx> Body<'_, '_, 'tcx> {
	/// The location a call of `instance`, which takes its caller's, passes: the one the
	/// crate's call of it gave.
	pub fn caller_arg(&mut self, instance: Instance<'tcx>, span: Span) -> Expr {
		match self.site.take() {
			Some((at, location)) if at == instance => location,
			site => {
				self.site = site;
				let path = self.t.tcx.def_path_str(instance.def_id());
				self.err(
					span,
					format!(
						"a call through std's code of `{path}`, a `#[track_caller]` function, whose caller is in std's source"
					),
				)
			}
		}
	}

	/// The function `instance` as a pointer, which passes its own location, as rustc's
	/// shim does, or `None` if it takes none.
	pub fn reified(&mut self, instance: Instance<'tcx>, span: Span) -> Option<Expr> {
		let tcx = self.t.tcx;
		if !tracks(self.t, instance) {
			return None;
		}
		let sig = self.t.sig(instance.def_id(), instance.args);
		let params: Vec<Param> = (sig.inputs().iter())
			.map(|input| Param {
				name: self.fresh("a"),
				ty: Some(self.t.ty(*input, span)),
			})
			.collect();
		let site = (instance, location(tcx, tcx.def_span(instance.def_id())));
		let outer = self.site.replace(site);
		let mut body = Vec::new();
		let values = params.iter().map(|p| Expr::ident(&p.name)).collect();
		let call = self.call_def(instance.def_id(), instance.args, values, span, &mut body);
		self.site = outer;
		body.push(StmtKind::Return(Some(call.typed())).at(None));
		// Declared, so its `()` is `void`, which a value's type says is `undefined`.
		let ret = self.t.ret_ty(sig.output(), span);
		let arrow = Expr::Arrow(params, ret, body);
		Some(match sig.output().is_unit() {
			true => Expr::As(Box::new(arrow), self.t.fn_ty(sig.inputs(), sig.output(), span)),
			false => arrow,
		})
	}
}

/// Refuse `instance`, which std's code calls, if it takes its caller's location, which is
/// then in std's source.
pub fn refuse_std_call<'tcx>(t: &mut Translator<'_, 'tcx>, instance: Instance<'tcx>) {
	if tracks(t, instance) {
		let path = t.tcx.def_path_str(instance.def_id());
		let msg = format!("`{path}`, a `#[track_caller]` function std's code calls, whose caller is in std's source");
		t.err(t.tcx.def_span(instance.def_id()), msg);
	}
}

/// The type of the parameter a `#[track_caller]` function takes its caller's location by.
pub fn param_ty() -> Type {
	Type::Rs("Location", Vec::new())
}

/// Each runtime function that takes where a panic it makes is, as an `at?: Location`
/// parameter, by how many parameters come before it.
fn located() -> &'static rustc_data_structures::fx::FxHashMap<&'static str, usize> {
	static LOCATED: OnceLock<rustc_data_structures::fx::FxHashMap<&'static str, usize>> = OnceLock::new();
	LOCATED.get_or_init(|| {
		(crate::program::RUNTIME.iter())
			.flat_map(|&(_, text)| signatures(text))
			.collect()
	})
}

/// The functions `text` exports that take an `at?: Location` parameter, or one that also
/// takes a key of `rs.Sites`, and its index.
fn signatures(text: &'static str) -> impl Iterator<Item = (&'static str, usize)> {
	["export function ", "export const "]
		.into_iter()
		.flat_map(move |keyword| {
			text.match_indices(keyword).filter_map(move |(at, _)| {
				let rest = &text[at + keyword.len()..];
				let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
				let (name, rest) = rest.split_at(end);
				// A const's type or value, then any type parameters, opens the list.
				let rest = rest.trim_start_matches([':', '=', ' ']);
				let rest = match rest.strip_prefix('<') {
					Some(generic) => &generic[closing(generic)? + 1..],
					None => rest,
				};
				let params = split(rest.strip_prefix('(')?);
				let at = params
					.iter()
					.position(|p| matches!(p.trim(), "at?: Location" | "at?: Location | string"))?;
				Some((name, at))
			})
		})
}

/// Where the `>` that closes the type parameters `text` starts with is.
fn closing(text: &str) -> Option<usize> {
	let (mut depth, mut prev) = (0usize, ' ');
	for (i, c) in text.char_indices() {
		match c {
			'<' => depth += 1,
			// The `>` of an arrow type's `=>` closes nothing.
			'>' if prev == '=' => {}
			'>' if depth == 0 => return Some(i),
			'>' => depth -= 1,
			_ => {}
		}
		prev = c;
	}
	None
}

/// The parameters of the list `text` starts, up to its closing parenthesis.
fn split(text: &str) -> Vec<&str> {
	let mut params = Vec::new();
	let (mut depth, mut start, mut prev) = (0usize, 0, ' ');
	for (i, c) in text.char_indices() {
		match c {
			'(' | '[' | '{' | '<' => depth += 1,
			// The `>` of an arrow type's `=>` closes nothing.
			'>' if prev == '=' => {}
			')' | ']' | '}' | '>' if depth > 0 => depth -= 1,
			')' => {
				if !text[start..i].trim().is_empty() {
					params.push(&text[start..i]);
				}
				return params;
			}
			',' if depth == 0 => {
				params.push(&text[start..i]);
				start = i + 1;
			}
			_ => {}
		}
		prev = c;
	}
	params
}

/// Pass `at` to each call in `stmts` of a runtime function that takes where it panics,
/// which was given nowhere yet.
pub fn locate(stmts: &mut [Stmt], at: &Expr) {
	for stmt in stmts {
		stmt_exprs(stmt, &mut |e| locate_expr(e, at));
	}
}

/// Pass `at` as [`locate`] does, within `e`.
pub fn locate_expr(e: &mut Expr, at: &Expr) {
	walk(e, &mut |e| {
		if let Some((args, index)) = unlocated(e) {
			args.resize_with(index, Expr::undefined);
			args.push(at.clone());
		}
	});
}

/// Whether `stmts` call a runtime function that takes where it panics without saying where.
pub fn unlocated_in(stmts: &mut [Stmt]) -> bool {
	let mut found = false;
	for stmt in stmts {
		stmt_exprs(stmt, &mut |e| walk(e, &mut |e| found |= unlocated(e).is_some()));
	}
	found
}

/// The arguments of `e` if it calls a runtime function that takes where it panics and is
/// given none, and where that goes among them.
fn unlocated(e: &mut Expr) -> Option<(&mut Vec<Expr>, usize)> {
	let Expr::Call(callee, args) = e else {
		return None;
	};
	let name = match &**callee {
		Expr::Rs(name) => name,
		Expr::Instantiate(f, _) if let Expr::Rs(name) = &**f => name,
		_ => return None,
	};
	let index = *located().get(name.as_str())?;
	(args.len() <= index).then_some((args, index))
}

/// Call `f` on each expression `stmt` holds directly, and in its blocks.
fn stmt_exprs(stmt: &mut Stmt, f: &mut dyn FnMut(&mut Expr)) {
	match &mut stmt.kind {
		StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => f(e),
		StmtKind::If { cond, then, els } => {
			f(cond);
			then.iter_mut().chain(els).for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::While { cond: e, body, .. } | StmtKind::ForOf { iter: e, body, .. } => {
			f(e);
			body.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Block { body, .. } => body.iter_mut().for_each(|s| stmt_exprs(s, f)),
		StmtKind::Switch {
			disc, cases, default, ..
		} => {
			f(disc);
			for (value, body) in cases {
				f(value);
				body.iter_mut().for_each(|s| stmt_exprs(s, f));
			}
			default.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Return(None)
		| StmtKind::Decl { init: None, .. }
		| StmtKind::Declare { .. }
		| StmtKind::Break(_)
		| StmtKind::Continue(_) => {}
	}
}

/// Call `f` on `e` and every expression within it, arrow bodies included, innermost first.
fn walk(e: &mut Expr, f: &mut dyn FnMut(&mut Expr)) {
	match e {
		Expr::Ident(_) | Expr::This | Expr::Sym(_) | Expr::Rs(_) | Expr::Lit(_) | Expr::Str(_) => {}
		Expr::Array(items) => items.iter_mut().for_each(|e| walk(e, f)),
		Expr::Object(props) => props.iter_mut().for_each(|(_, e)| walk(e, f)),
		Expr::Member(e, _)
		| Expr::Unary(_, e)
		| Expr::Located(_, e)
		| Expr::As(e, _)
		| Expr::Instantiate(e, _)
		| Expr::Lambda(_, e)
		| Expr::Postfix(e, _) => walk(e, f),
		Expr::Index(a, b) | Expr::Binary(a, _, b) | Expr::Assign(a, b) => {
			walk(a, f);
			walk(b, f);
		}
		Expr::Call(callee, args) | Expr::New(callee, args) => {
			walk(callee, f);
			args.iter_mut().for_each(|e| walk(e, f));
		}
		Expr::Cond(c, a, b) => {
			walk(c, f);
			walk(a, f);
			walk(b, f);
		}
		Expr::Arrow(_, _, body) => body.iter_mut().for_each(|s| stmt_exprs(s, &mut |e| walk(e, f))),
	}
	f(e);
}
