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

/// Why a certificate is refused, which the api fixture's `verify` returns as its message
/// and `blame` takes back, though no export returns one: C never holds one.
#[derive(Debug)]
pub struct Invalid {
	pub reason: String,
}

impl std::fmt::Display for Invalid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "invalid certificate: {}", self.reason)
	}
}

impl std::error::Error for Invalid {}

/// A count that is not `Send`, which the reject fixture takes by value: no handle may hold it.
pub struct Local(std::rc::Rc<u32>);

impl Local {
	pub fn get(&self) -> u32 {
		*self.0
	}
}
