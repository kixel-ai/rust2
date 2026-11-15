//! A library macros builds on, whose API only an exported macro its macro makes in macros
//! may need.

/// Makes, where it is called, a macro that makes an exported macro calling the helper it
/// is given through this crate's `$crate`.
#[macro_export]
macro_rules! make_maker {
	() => {
		macro_rules! make_nested {
			($module:ident, $helper:ident) => {
				#[macro_export]
				macro_rules! call_nested {
					() => {
						$crate::$module::$helper()
					};
				}
			};
		}
	};
}

pub mod nested {
	/// External: what an exported macro of macros may name, which a macro that this crate's
	/// macro made there made.
	pub fn generated() -> u32 {
		51
	}
}
