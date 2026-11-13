//! A crate the api fixture names behind an `Arc` or by value but does not re-export, as
//! moq-tokio's listener names rustls's server config and certificates.

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

/// A certificate: an opaque handle wherever the api fixture takes or returns one by value,
/// as moq-tokio takes rustls's certificates.
#[derive(Clone, Debug)]
pub struct Cert {
	der: Vec<u8>,
}

impl Cert {
	pub fn new(der: &[u8]) -> Self {
		Self { der: der.to_vec() }
	}

	pub fn der(&self) -> &[u8] {
		&self.der
	}
}

/// A session established elsewhere, which no export of the api fixture returns, as
/// moq-tokio takes a qmux session: C can never hold one.
#[derive(Debug)]
pub struct Session {
	pub id: u32,
}
