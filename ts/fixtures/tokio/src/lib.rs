//! Passes values through tokio's `sync::mpsc`, `sync::watch`, and `sync::oneshot`, and
//! waits on them with `select!`, all of which translation maps itself, so this config
//! binds and leaves out nothing. A small executor, translated with the rest, polls the
//! tasks that wait, so each log shows when each future became ready.
//!
//! `harness::run` is the entry point both sides of the differential test run.

mod exec;
pub mod harness;
mod log;
mod mpsc;
mod oneshot;
mod select;
#[cfg(target_arch = "wasm32")]
#[path = "../../wasm.rs"]
mod wasm;
mod watch;
#[cfg(target_arch = "wasm32")]
use harness::run as entry;
