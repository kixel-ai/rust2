//! Constructs outside the subset: each root fails translation with a span, and no
//! output is written. Each module holds the cases of one kind of construct.

#![allow(dead_code, reason = "each item is a translation root, not called from Rust")]
#![feature(cow_is_borrowed, extend_one, f16, is_ascii_octdigit, linked_list_retain)]

mod addresses;
mod alloc;
mod buffers;
mod clones;
mod config;
mod fmt;
mod futures;
mod iters;
mod kept;
mod keys;
mod objects;
mod panics;
mod parts;
mod places;
mod pointers;
mod rebound;
mod spans;
mod statics;
mod strs;
mod values;
