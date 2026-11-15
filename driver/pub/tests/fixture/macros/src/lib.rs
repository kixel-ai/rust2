//! A library app uses, whose exported macros facade re-exports: crates outside the audit
//! may expand them, so all it exports stays.

/// Kept: app calls it.
pub fn used() -> u32 {
	hidden::unexported()
}

mod hidden {
	/// Narrowed: what its crate doesn't export, which no macro expanding elsewhere can name.
	pub fn unexported() -> u32 {
		55
	}
}

/// External: what no macro names, which its crate exports.
pub fn unnamed() -> u32 {
	56
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

/// External: a macro facade re-exports, which nothing in the audit expands.
#[macro_export]
macro_rules! call_helper {
	() => {
		$crate::helpers::called()
	};
}

/// Makes an exported macro that calls the helper it is given through `$crate`, where only
/// what it makes names the helper.
macro_rules! make_caller {
	($module:ident, $helper:ident) => {
		#[macro_export]
		macro_rules! call_generated {
			() => {
				$crate::$module::$helper()
			};
		}
	};
}

make_caller!(minted, generated);

// In a private module, so only the exported macro it makes, which facade re-exports, is
// API: makers' macro made it, so all makers exports stays.
mod nested_maker {
	makers::make_maker!();
	make_nested!(nested, generated);
}

/// External: a macro facade re-exports, which imports what it calls through a group.
#[macro_export]
macro_rules! call_grouped {
	() => {{
		use $crate::{grouped as helper, minted as _};
		helper()
	}};
}

/// External: what an exported macro's grouped `$crate` import names, though nothing in the
/// audit expands it.
pub fn grouped() -> u32 {
	52
}

/// External: a macro facade re-exports, whose lone-name call of `call_inner!` is
/// `$crate::call_inner!`.
#[macro_export(local_inner_macros)]
macro_rules! call_outer {
	() => {
		call_inner!()
	};
}

/// External: what only `call_outer!` calls.
#[doc(hidden)]
#[macro_export]
macro_rules! call_inner {
	() => {
		$crate::inner_helpers::inner_called()
	};
}

pub mod inner_helpers {
	/// External: what only `call_inner!` names, which `call_outer!`'s lone-name call reaches.
	pub fn inner_called() -> u32 {
		54
	}
}

pub mod minted {
	/// External: what the `$crate` path of an exported macro another macro made names,
	/// though nothing in the audit expands it.
	pub fn generated() -> u32 {
		50
	}
}

pub mod helpers {

	/// External: what an exported macro's `$crate` path names, though nothing in the audit
	/// expands it.
	pub fn called() -> u32 {
		49
	}
}

/// Makes a function with the visibility it is given, and an exported macro that prints
/// that visibility.
macro_rules! with_vis {
	($v:vis fn $name:ident) => {
		$v fn $name() -> u32 {
			41
		}

		#[macro_export]
		macro_rules! vis_of {
			() => {
				stringify!($v)
			};
		}
	};
}

mod tags {
	/// Restored, unreached: an exported macro facade re-exports holds the text of its
	/// `pub`, which nothing in the audit expands.
	#[derives::export_tag]
	pub fn tagged_export() -> u32 {
		42
	}
}

mod captures {
	// External: an exported macro facade re-exports holds this `pub`, which nothing in the
	// audit expands.
	with_vis!(pub fn captured);
}
