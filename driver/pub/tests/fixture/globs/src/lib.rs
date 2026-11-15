//! A library app uses, whose exported macro, which facade re-exports, imports its whole
//! crate root with a glob.

/// Kept: app calls it.
pub fn used() -> u32 {
	55
}

/// External: a macro facade re-exports, which imports what it calls through a `$crate`
/// glob of the crate root.
#[macro_export]
macro_rules! call_globbed {
	() => {{
		use $crate::*;
		through_root_glob()
	}};
}

/// External: what an exported macro's `$crate` glob supplies, though nothing in the audit
/// expands it.
pub fn through_root_glob() -> u32 {
	53
}
