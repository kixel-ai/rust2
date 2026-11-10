//! A crate the api fixture names behind an `Arc` but does not re-export, as moq-tokio's
//! listener names rustls's server config.

/// A config: an opaque handle wherever the api fixture returns or takes an `Arc` of it.
#[derive(Debug)]
pub struct Config {
	alpn: String,
}

impl Config {
	pub fn new(alpn: &str) -> Self {
		Self { alpn: alpn.to_string() }
	}

	pub fn alpn(&self) -> &str {
		&self.alpn
	}
}
