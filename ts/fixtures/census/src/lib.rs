//! Constructs translation refuses for good, which a census ranks: each item is a root, a
//! generic one lowered over its type parameter, and the same construct refused at
//! different types groups together.

#![allow(dead_code, reason = "each item is a census root, not called from Rust")]

/// A pointer's address, which JS has no counterpart of.
pub fn addressed(n: u32) -> usize {
	&n as *const u32 as usize
}

/// A pointer's address in a body no instance reaches, lowered over `T`.
pub fn generic<T>(t: T) -> usize {
	&t as *const T as usize
}

/// A pointer read as another type's bytes, at two types.
pub fn reinterpreted(n: u32, m: u64) -> u8 {
	unsafe { *(&n as *const u32 as *const u8) ^ *(&m as *const u64 as *const u8) }
}

/// A pointer's address, formatted.
pub fn printed(n: u8) -> String {
	format!("{:p}", &n)
}

/// A union whose fields overlap in values JS keeps apart.
pub union Bits {
	int: u32,
	float: f64,
}

static mut COUNT: u8 = 0;

/// A mutable static, which any code may write.
pub fn counted() -> u8 {
	unsafe {
		COUNT += 1;
		COUNT
	}
}

/// The address of a dependency's type's value, whose API is its path.
pub fn located(b: bytes::Bytes) -> usize {
	&b as *const bytes::Bytes as usize
}
