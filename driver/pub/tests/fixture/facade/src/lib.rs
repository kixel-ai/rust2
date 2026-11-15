//! A library nothing in the workspace depends on, whose API crates outside it may use,
//! as a translator's fixture's is.

/// External: this library's API, re-exported from api.
pub use api::Exposed;

/// External: this library's API, though nothing calls it.
pub fn unused() -> api::Returned {
	api::Returned::default()
}

/// External: this library's API, re-exported from api, with what its items return.
pub use api::Shape;

/// External: this library's API, which exposes all of extra's.
pub extern crate extra;

/// External: this library's API, re-exported from api, with what its `Iterator` yields.
pub use api::Products;

/// External: this library's API, re-exported from api, with their impls.
pub use api::{Make, Slot};

/// External: this library's API, an exported macro of api, made by another.
pub use api::{tagged_export_vis, vis_of};

/// External: this library's API, re-exported from api through a glob there.
pub use api::Part;

/// External: this library's API, one variant of api's enum.
pub use api::Picked::One;

/// External: this library's API, exported macros of api that call helpers of api, one another macro made.
pub use api::{call_generated, call_helper};
