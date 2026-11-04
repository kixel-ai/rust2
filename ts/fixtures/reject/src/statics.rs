//! Statics whose state JS can't share or copy.

static mut COUNT: u8 = 0;

pub fn statics() -> u8 {
	unsafe { COUNT }
}
