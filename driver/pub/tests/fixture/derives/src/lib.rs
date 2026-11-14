//! A derive that reads its item's visibility, as any derive may.

use proc_macro::{TokenStream, TokenTree};

/// Kept: a proc-macro crate's macros run with nothing naming them.
#[proc_macro_derive(Label)]
pub fn derive_label(input: TokenStream) -> TokenStream {
	let mut visibility = String::new();
	let mut name = None;
	let mut tokens = input.into_iter();
	while let Some(token) = tokens.next() {
		match &token {
			TokenTree::Ident(ident) if ident.to_string() == "struct" || ident.to_string() == "enum" => {
				name = tokens.next().map(|t| t.to_string());
				break;
			}
			TokenTree::Punct(_) => {
				// An attribute: `#` and its bracketed group.
				tokens.next();
			}
			_ => visibility.push_str(&token.to_string()),
		}
	}
	let name = name.expect("a struct or enum");
	format!("impl {name} {{ fn visibility() -> &'static str {{ {visibility:?} }} pub(crate) fn label() -> &'static str {{ Self::visibility() }} }}")
		.parse()
		.unwrap()
}
