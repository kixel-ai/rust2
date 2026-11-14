//! A library nothing in the workspace depends on, whose API crates outside it may use,
//! as a translator's fixture's is.

/// External: this library's API, re-exported from api.
pub use api::Exposed;

/// External: this library's API, though nothing calls it.
pub fn unused() -> api::Returned {
	api::Returned::default()
}
