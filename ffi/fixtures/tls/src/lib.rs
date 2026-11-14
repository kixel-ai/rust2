//! A crate the api fixture names in signatures but re-exports nothing from, as moq-tokio's
//! listener names rustls's server config and moq-net's `dial_tcp` names kio's `Pending`.

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

mod session {
	use crate::policy::Lifetime;

	/// A session: an object of a private module, which the api fixture returns bare and no
	/// re-export names, so it exports by its path in this crate.
	pub struct Session {
		alpn: String,
		lifetime: Lifetime,
	}

	impl Session {
		pub fn alpn(&self) -> String {
			self.alpn.clone()
		}

		/// A record only this method names, which exports once the session does.
		pub fn lifetime(&self) -> Lifetime {
			self.lifetime
		}
	}

	/// A session of `alpn` lasting `seconds`, or an error when `alpn` is empty.
	pub fn resume(alpn: &str, seconds: u32) -> Result<Session, crate::Error> {
		if alpn.is_empty() {
			return Err(crate::Error("no protocol to resume".to_string()));
		}
		Ok(Session {
			alpn: alpn.to_string(),
			lifetime: Lifetime { seconds },
		})
	}
}

pub use session::{Session, resume};

pub mod policy {
	/// How long a session lasts.
	#[derive(Clone, Copy, Debug, PartialEq)]
	pub struct Lifetime {
		pub seconds: u32,
	}
}

/// An error, which the api fixture returns only as a `Result`'s, so it crosses as its
/// message.
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}
