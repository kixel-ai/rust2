//! What a case writes, and the values its channels carry, which log as they drop.

use std::cell::RefCell;
use std::fmt;

thread_local! {
	static LOG: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Write a line.
pub fn note(line: String) {
	LOG.with_borrow_mut(|log| {
		log.push_str(&line);
		log.push('\n');
	});
}

/// Take what was written.
pub fn take() -> String {
	LOG.with_borrow_mut(std::mem::take)
}

/// A value sent through a channel, which logs as it drops.
#[derive(Debug, PartialEq)]
pub struct Item(pub u8);

impl Drop for Item {
	fn drop(&mut self) {
		note(format!("drop {}", self.0));
	}
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "#{}", self.0)
	}
}
