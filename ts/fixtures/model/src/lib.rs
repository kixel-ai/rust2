//! A harness over moq-net's model, which translates with it as one program: the types of
//! `moq/model.toml`'s roots that translate with nothing refused or bound.
//!
//! `harness::run` is the entry point both sides of the differential test run.

pub mod harness;
#[cfg(target_arch = "wasm32")]
#[path = "../../wasm.rs"]
mod wasm;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;
