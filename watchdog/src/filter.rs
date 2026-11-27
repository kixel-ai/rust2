//! The part of nextest's filterset language a bound override's `filter` may use:
//! `package(...)` and `test(...)`, joined with `&` and `|` and grouped in parentheses.

use anyhow::{Result, anyhow};
use regex::Regex;

/// A parsed `filter`, matched against a test's package and name.
#[derive(Debug)]
pub enum Filter {
	Package(Matcher),
	Test(Matcher),
	And(Box<Filter>, Box<Filter>),
	Or(Box<Filter>, Box<Filter>),
}

/// How a predicate compares its argument, as nextest's `=`, `~`, and `/.../` prefixes say.
#[derive(Debug)]
pub enum Matcher {
	Equal(String),
	Contains(String),
	Regex(Regex),
}

impl Matcher {
	fn matches(&self, text: &str) -> bool {
		match self {
			Matcher::Equal(want) => text == want,
			Matcher::Contains(want) => text.contains(want.as_str()),
			Matcher::Regex(regex) => regex.is_match(text),
		}
	}
}

impl Filter {
	/// Parse a filter, failing on any predicate or operator but those above.
	pub fn parse(text: &str) -> Result<Filter> {
		let mut parser = Parser { text, at: 0 };
		let filter = parser.union()?;
		parser.skip_space();
		match parser.rest().chars().next() {
			None => Ok(filter),
			Some(_) => Err(parser.error("expected `&`, `|`, or the end")),
		}
	}

	/// Whether the test `test` of package `package` matches.
	pub fn matches(&self, package: &str, test: &str) -> bool {
		match self {
			Filter::Package(matcher) => matcher.matches(package),
			Filter::Test(matcher) => matcher.matches(test),
			Filter::And(a, b) => a.matches(package, test) && b.matches(package, test),
			Filter::Or(a, b) => a.matches(package, test) || b.matches(package, test),
		}
	}
}

struct Parser<'a> {
	text: &'a str,
	at: usize,
}

impl Parser<'_> {
	fn rest(&self) -> &str {
		&self.text[self.at..]
	}

	fn skip_space(&mut self) {
		let rest = self.rest();
		self.at += rest.len() - rest.trim_start().len();
	}

	fn eat(&mut self, token: char) -> bool {
		self.skip_space();
		if self.rest().starts_with(token) {
			self.at += token.len_utf8();
			true
		} else {
			false
		}
	}

	fn error(&self, what: &str) -> anyhow::Error {
		anyhow!("{what} at byte {} of filter `{}`", self.at, self.text)
	}

	// `&` binds tighter than `|`, as in nextest.
	fn union(&mut self) -> Result<Filter> {
		let mut filter = self.intersection()?;
		while self.eat('|') {
			filter = Filter::Or(Box::new(filter), Box::new(self.intersection()?));
		}
		Ok(filter)
	}

	fn intersection(&mut self) -> Result<Filter> {
		let mut filter = self.atom()?;
		while self.eat('&') {
			filter = Filter::And(Box::new(filter), Box::new(self.atom()?));
		}
		Ok(filter)
	}

	fn atom(&mut self) -> Result<Filter> {
		if self.eat('(') {
			let filter = self.union()?;
			if !self.eat(')') {
				return Err(self.error("expected `)`"));
			}
			return Ok(filter);
		}
		let rest = self.rest();
		let name_len = rest
			.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
			.unwrap_or(rest.len());
		let name = &rest[..name_len];
		if name.is_empty() {
			return Err(self.error("expected `package(...)`, `test(...)`, or `(`"));
		}
		let is_package = match name {
			"package" => true,
			"test" => false,
			_ => {
				return Err(self.error(&format!(
					"`{name}` is not supported; a bound's filter may use only package(), test(), `&`, `|`, and parentheses"
				)));
			}
		};
		self.at += name_len;
		if !self.rest().starts_with('(') {
			return Err(self.error("expected `(`"));
		}
		self.at += 1;
		let matcher = self.matcher(is_package)?;
		Ok(if is_package {
			Filter::Package(matcher)
		} else {
			Filter::Test(matcher)
		})
	}

	// The argument and its closing parenthesis.
	fn matcher(&mut self, is_package: bool) -> Result<Matcher> {
		let rest = self.rest();
		if let Some(body) = rest.strip_prefix('/') {
			// A regex runs to the first unescaped `/`, so it may hold parentheses; `\/` is a slash.
			let mut pattern = String::new();
			let mut chars = body.char_indices();
			while let Some((i, c)) = chars.next() {
				match c {
					'\\' if body[i + 1..].starts_with('/') => {
						pattern.push('/');
						chars.next();
					}
					'/' => {
						if !body[i + 1..].starts_with(')') {
							self.at += 1 + i + 1;
							return Err(self.error("expected `)` after the regex"));
						}
						let regex = Regex::new(&pattern).map_err(|e| self.error(&e.to_string()))?;
						self.at += 1 + i + 2;
						return Ok(Matcher::Regex(regex));
					}
					_ => pattern.push(c),
				}
			}
			return Err(self.error("unterminated regex"));
		}
		let Some(end) = rest.find(')') else {
			return Err(self.error("expected `)`"));
		};
		let argument = rest[..end].trim();
		let matcher = if let Some(text) = argument.strip_prefix('=') {
			Matcher::Equal(text.to_owned())
		} else if let Some(text) = argument.strip_prefix('~') {
			Matcher::Contains(text.to_owned())
		} else if argument.starts_with('#') || (is_package && argument.contains(['*', '?', '['])) {
			return Err(self.error("glob matchers are not supported"));
		} else if is_package {
			// package()'s default is a glob, which without wildcards is equality.
			Matcher::Equal(argument.to_owned())
		} else {
			Matcher::Contains(argument.to_owned())
		};
		self.at += end + 1;
		Ok(matcher)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn check(filter: &str, package: &str, test: &str) -> bool {
		Filter::parse(filter).unwrap().matches(package, test)
	}

	#[test]
	fn moq_profile_filters() {
		let uring = "package(moq-uring) | (package(moq-relay) & test(/(^|[:_])uring([:_]|$)/))";
		assert!(check(uring, "moq-uring", "anything"));
		assert!(check(uring, "moq-relay", "net::uring::accept"));
		assert!(!check(uring, "moq-relay", "during_shutdown"));
		assert!(!check(uring, "moq-video", "net::uring::accept"));

		let exact = "package(moq-tokio) & test(=receive_credit::benchmark_join_lag)";
		assert!(check(exact, "moq-tokio", "receive_credit::benchmark_join_lag"));
		assert!(!check(exact, "moq-tokio", "receive_credit::benchmark_join_lag_two"));
		assert!(!check(exact, "moq-ffi", "receive_credit::benchmark_join_lag"));
	}

	#[test]
	fn rind_profile_filter() {
		let filter = "test(/rsa8192_/) | test(=rsa::key_generation_8192) | test(=every_cpu_path_answers_every_set)";
		assert!(check(filter, "rind", "rsa::tests::rsa8192_sign"));
		assert!(check(filter, "rind-acvp", "every_cpu_path_answers_every_set"));
		assert!(!check(filter, "rind", "rsa::key_generation_4096"));
	}

	#[test]
	fn and_binds_tighter_than_or() {
		// a | (b & c), not (a | b) & c.
		let filter = "test(=a) | package(p) & test(=c)";
		assert!(check(filter, "q", "a"));
		assert!(!check(filter, "p", "b"));
		assert!(check(filter, "p", "c"));
	}

	#[test]
	fn default_matchers() {
		assert!(check("test(key)", "p", "rsa::key_generation"));
		assert!(!check("package(rind)", "rind-ring", "t"));
		assert!(check("package(~ring)", "rind-ring", "t"));
		assert!(check(r"test(/a\/b/)", "p", "a/b"));
	}

	#[test]
	fn unsupported_filters_fail_naming_them() {
		for (filter, says) in [
			("kind(bench)", "`kind` is not supported"),
			("not test(=a)", "`not` is not supported"),
			("test(=a) - test(=b)", "expected `&`, `|`, or the end"),
			("package(rind-*)", "glob"),
			("test(#a*)", "glob"),
			("test(/a/", "expected `)` after the regex"),
			("test(/a", "unterminated regex"),
			("(test(=a)", "expected `)`"),
			("test(/(/)", "regex parse error"),
		] {
			let error = Filter::parse(filter).unwrap_err().to_string();
			assert!(error.contains(says), "{filter}: {error}");
		}
	}
}
