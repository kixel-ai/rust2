//! A harness over js-sys and web-sys, and imports of its own of each kind wasm-bindgen
//! declares, which translation makes direct JS calls: the JS value is the value itself.
//!
//! `harness::run` is the entry point both sides of the differential test run: natively
//! through wasm-bindgen's CLI, and translated.

pub mod harness;
mod imports;
#[cfg(feature = "refused")]
pub mod refused;
#[cfg(target_arch = "wasm32")]
#[path = "../../wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;
