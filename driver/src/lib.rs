//! The front end rust2's rustc drivers share.
//!
//! A driver binary is two programs. Run directly, it is a CLI that runs cargo on a
//! package with itself as `RUSTC_WRAPPER` ([`cargo`]). Run by cargo as that wrapper, it
//! is rustc ([`rustc`]), which compiles every crate as rustc would and hands the
//! configured library to the driver's callbacks. A translator structures each function's
//! control flow into its own statements with [`cfg`].

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_session;

pub mod cargo;
pub mod cfg;
pub mod rustc;
