//! moq-net's sync codec, vendored to test rust2-ts against native Rust.
//!
//! `coding/`, `lite/` (version, message, group, probe), and `ietf/version.rs` are
//! copied from moq-dev/moq at 2f146e8324ea867d1e5988d8f4ea62da2390251d
//! (`rs/moq-net/src`) with their test modules removed; `coding/mod.rs` and
//! `version.rs` are trimmed to the sync codec, and this file stubs the rest of the
//! crate the codec names. `harness` is the entry point both sides of the differential
//! test run, and `bench` holds the workloads `just ts bench` times.

#![allow(dead_code, reason = "a vendored subset of moq-net, which uses the rest")]

pub mod bench;
mod coding;
pub mod harness;
mod ietf {
	mod version;
	pub use version::Version;
}
mod lite {
	mod group;
	pub mod message;
	mod probe;
	mod version;
	pub use group::*;
	pub use message::Message;
	pub use probe::*;
	pub use version::Version;
}
mod version;
// wasm32-unknown-unknown alone: on wasm32-wasip1, rust2-ffi exports the crate, and a module
// built from that export would carry these exports too.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[path = "../../wasm.rs"]
mod wasm;
/// What the wasm32-unknown-unknown build exports: the benchmark's workloads with `bench`, else the harness.
#[cfg(all(target_arch = "wasm32", target_os = "unknown", feature = "bench"))]
use bench::run as entry;
#[cfg(all(target_arch = "wasm32", target_os = "unknown", not(feature = "bench")))]
use harness::run as entry;

pub use version::*;

pub mod stats {
	/// The control message classes moq-net's metrics count, trimmed to what the codec names.
	#[derive(Clone, Copy, Debug, PartialEq, Eq)]
	#[non_exhaustive]
	pub enum Control {
		/// Bandwidth probes.
		Probe,
		/// Other protocol control messages.
		Other,
	}
}

/// moq-net's session error, trimmed to a few variants: the codec only carries it.
#[derive(thiserror::Error, Debug, Clone)]
#[non_exhaustive]
pub enum SessionError {
	#[error("protocol violation")]
	ProtocolViolation,
	#[error("app code={0}")]
	App(u16),
}

/// moq-net's track timescale, stubbed for `lite::Version::wire_timescale`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timescale(u64);

impl Timescale {
	/// One unit per millisecond.
	pub const MILLI: Self = Self(1_000);
}
