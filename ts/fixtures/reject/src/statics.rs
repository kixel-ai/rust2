//! Statics whose state JS can't share or copy.

static mut COUNT: u8 = 0;

pub fn statics() -> u8 {
	unsafe { COUNT }
}

/// A pointer into memory the translation can't name.
static NAMED: &u32 = &5;

static SLOT: std::sync::atomic::AtomicPtr<u32> = std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

pub fn pointer_statics(p: *mut u32) -> u32 {
	use std::sync::atomic::Ordering::Relaxed;
	// Two `rt.Place`s of one place are two objects, which `===` can't tell are one.
	let _ = SLOT.compare_exchange(p, std::ptr::null_mut(), Relaxed, Relaxed);
	*NAMED
}
