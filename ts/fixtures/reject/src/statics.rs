//! Statics and lazy cells whose state JS can't share or copy.

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

const LAZY_CELL: std::cell::LazyCell<u32> = std::cell::LazyCell::new(|| 2);

const LAZY_LOCK: std::sync::LazyLock<u32> = std::sync::LazyLock::new(|| 2);

/// A lazy cell's state, which JS keeps private, so copying its fields can't move it.
pub fn lazy_celled(c: &mut std::cell::LazyCell<u32>) -> u32 {
	*std::cell::Cell::from_mut(c).replace(LAZY_CELL) + **c
}

/// A lazy lock written whole through a reference, which can't copy its private state.
pub fn lazy_assigned(c: &mut std::sync::LazyLock<u32>) -> u32 {
	*c = LAZY_LOCK;
	**c
}

/// A thread local's key, whose state JS keeps private, set whole through a cell.
pub fn key_celled(key: &'static mut std::thread::LocalKey<u32>, next: std::thread::LocalKey<u32>) -> u32 {
	std::cell::Cell::from_mut(key).set(next);
	key.with(|v| *v)
}
