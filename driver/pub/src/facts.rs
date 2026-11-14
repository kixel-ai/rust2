//! What one rustc invocation tells the CLI about a local crate, and what the CLI tells
//! the next invocations to narrow: files both sides read and write as JSON.

use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// The environment the CLI hands the wrapper its job in.
pub mod env {
	/// The directory each compiled local crate's [`Unit`] goes into.
	pub const FACTS: &str = "RUST2_PUB_FACTS";
	/// A JSON list of the local packages' directories, whose crates are collected.
	pub const LOCAL: &str = "RUST2_PUB_LOCAL";
	/// An [`super::Edits`] file: the `pub` tokens rustc reads as `pub(crate)`.
	pub const EDITS: &str = "RUST2_PUB_EDITS";
}

/// A definition's identity in one compilation: rustc's `DefPathHash`, which a crate
/// that depends on it computes the same.
pub type Hash = [u64; 2];

/// One compiled local crate: its definitions and what each refers to.
#[derive(Serialize, Deserialize)]
pub struct Unit {
	pub crate_name: String,
	/// The compilation's `StableCrateId`, the crate half of each of its own hashes.
	pub crate_id: u64,
	/// The package's directory, from `CARGO_MANIFEST_DIR`.
	pub package: PathBuf,
	/// Whether it is a test harness, whose roots are tests.
	pub test: bool,
	/// Whether another crate can depend on it.
	pub lib: bool,
	/// Every file a span here names, absolute; a span's file is its index.
	pub files: Vec<PathBuf>,
	pub defs: Vec<Def>,
	/// Each definition another refers to, once per pair, with the first site.
	pub refs: Vec<Ref>,
	/// Every resolved name in the source, in order, which a narrowed build must resolve
	/// alike.
	pub sites: Vec<Site>,
	/// What runs without anything calling it: `main`, tests, exported symbols.
	pub roots: Vec<Hash>,
	/// Trait impls, each live once all it requires is.
	pub impls: Vec<Impl>,
}

/// A source range: the file's index in [`Unit::files`], and its bytes.
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Span {
	pub file: u32,
	pub lo: u32,
	pub hi: u32,
}

/// A definition.
#[derive(Serialize, Deserialize)]
pub struct Def {
	pub hash: Hash,
	/// What it is, as rustc's `DefKind` prints.
	pub kind: String,
	/// Its path, as rustc prints it.
	pub path: String,
	/// Its header, the span rustc's diagnostics point at.
	pub span: Option<Span>,
	/// The `pub` that makes it public, if written in source.
	pub token: Option<Span>,
	/// Whether another crate can name it: public all the way from its crate's root.
	pub exported: bool,
	/// Whether it is a symbol code outside Rust links to, which rustc keeps whatever its
	/// visibility.
	pub symbol: bool,
	/// Its enclosing definition, live whenever it is.
	pub parent: Option<Hash>,
	/// The type an inherent impl's item belongs to, whose API it is part of.
	pub member_of: Option<Hash>,
}

/// A definition `from` refers to `to`, first at `site`.
#[derive(Serialize, Deserialize)]
pub struct Ref {
	pub from: Hash,
	pub to: Hash,
	pub site: Option<Span>,
	/// Whether `from`'s interface names it, rather than only its body: its signature,
	/// its type, or what it imports.
	pub interface: bool,
}

/// A name the source resolves: where, and to what.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Site {
	pub span: Span,
	pub to: Hash,
}

/// A trait impl, live once its trait and self type, where those are local, are.
#[derive(Serialize, Deserialize)]
pub struct Impl {
	pub hash: Hash,
	pub requires: Vec<Hash>,
}

/// The `pub` tokens to read as `pub(crate)`: each file, absolute, and the byte offsets of
/// its tokens.
pub type Edits = BTreeMap<PathBuf, Vec<u32>>;
