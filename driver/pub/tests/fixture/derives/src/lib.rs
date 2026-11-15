//! Macros that read their item's visibility, as any may, and one that does not.
#![feature(proc_macro_span)]

use proc_macro::{TokenStream, TokenTree};

/// Kept: a proc-macro crate's macros run with nothing naming them.
#[proc_macro_derive(Label)]
pub fn derive_label(input: TokenStream) -> TokenStream {
	let (visibility, name) = read(input);
	format!("impl {name} {{ fn visibility() -> &'static str {{ {visibility:?} }} pub(crate) fn label() -> &'static str {{ Self::visibility() }} }}")
		.parse()
		.unwrap()
}

/// Kept: hands the visibility it reads to `make_label!`, a macro of the crate it expands
/// in.
#[proc_macro_derive(NestedLabel)]
pub fn derive_nested_label(input: TokenStream) -> TokenStream {
	let (visibility, name) = read(input);
	format!("make_label!({name}, {visibility:?});").parse().unwrap()
}

/// Kept: adds a function returning the visibility of the function it is on.
#[proc_macro_attribute]
pub fn tag(_: TokenStream, item: TokenStream) -> TokenStream {
	let (visibility, name) = read(item.clone());
	let tag: TokenStream = format!("fn {name}_tag() -> &'static str {{ {visibility:?} }}")
		.parse()
		.unwrap();
	item.into_iter().chain(tag).collect()
}

/// Kept: returns its item, and an exported macro whose string is its visibility.
#[proc_macro_attribute]
pub fn export_tag(_: TokenStream, item: TokenStream) -> TokenStream {
	let (visibility, name) = read(item.clone());
	let tag: TokenStream = format!("#[macro_export] macro_rules! {name}_vis {{ () => {{ {visibility:?} }}; }}")
		.parse()
		.unwrap();
	item.into_iter().chain(tag).collect()
}

/// Kept: returns its item, and a method of `crate::Exposed` that is `pub` when its item
/// is, written anew rather than copied.
#[proc_macro_attribute]
pub fn method_like(_: TokenStream, item: TokenStream) -> TokenStream {
	let (visibility, name) = read(item.clone());
	let visibility = if visibility == "pub" { "pub" } else { "" };
	let method: TokenStream = format!("impl crate::Exposed {{ {visibility} fn {name}_method() -> u32 {{ 57 }} }}")
		.parse()
		.unwrap();
	item.into_iter().chain(method).collect()
}

/// Kept: [`method_like`], but writing `pub` or `pub(crate)`, each spanning its item from
/// its own `pub` to its end.
#[proc_macro_attribute]
pub fn method_spanned(_: TokenStream, item: TokenStream) -> TokenStream {
	let (visibility, name) = read(item.clone());
	let span = item
		.clone()
		.into_iter()
		.skip_while(|t| !matches!(t, TokenTree::Ident(i) if i.to_string() == "pub"))
		.map(|t| t.span())
		.reduce(|a, b| a.join(b).expect("one file"))
		.expect("a `pub` item");
	let visibility = if visibility == "pub" { "pub" } else { "pub(crate)" };
	let mut method: TokenStream = format!("impl crate::Exposed {{ {visibility} fn {name}_method() -> u32 {{ 59 }} }}")
		.parse()
		.unwrap();
	method = respan(method, span);
	item.into_iter().chain(method).collect()
}

/// `stream`, every token of it at `span`.
fn respan(stream: TokenStream, span: proc_macro::Span) -> TokenStream {
	stream
		.into_iter()
		.map(|mut token| {
			if let TokenTree::Group(group) = &token {
				let mut inner = proc_macro::Group::new(group.delimiter(), respan(group.stream(), span));
				inner.set_span(span);
				token = TokenTree::Group(inner);
			}
			token.set_span(span);
			token
		})
		.collect()
}

/// Kept: returns its item as it is.
#[proc_macro_attribute]
pub fn keep(_: TokenStream, item: TokenStream) -> TokenStream {
	item
}

/// The visibility of the struct, enum, or function `item`, and its name.
fn read(item: TokenStream) -> (String, String) {
	let mut visibility = String::new();
	let mut tokens = item.into_iter();
	while let Some(token) = tokens.next() {
		match &token {
			TokenTree::Ident(ident) if ["struct", "enum", "fn"].contains(&ident.to_string().as_str()) => {
				let name = tokens.next().expect("a name").to_string();
				return (visibility, name);
			}
			TokenTree::Punct(_) => {
				// An attribute: `#` and its bracketed group.
				tokens.next();
			}
			_ => visibility.push_str(&token.to_string()),
		}
	}
	panic!("a struct, enum, or function")
}
