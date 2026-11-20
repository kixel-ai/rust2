//! What to change in a facade to clear each refusal: the `help:` note of its error.
//!
//! Keyed by the refusal's construct, the name the census ranks it by. `of` is total over
//! every construct that refuses an item, which a test checks against the collector's source.

/// The note for `construct`, or none for one that is no refusal of the facade: a left-out item
/// the census lists, or the exporter's own bug.
pub(crate) fn of(construct: &str) -> Option<&'static str> {
	Some(match construct {
		// Items.
		"re-export of a whole crate" => {
			"re-export the crate's items with `pub use dep::*;`, or wrap the few types a signature needs in types of your own"
		}
		"re-export of the standard library's item" => {
			"name the standard library's type in signatures, where it maps by role, instead of re-exporting it"
		}
		"macro" => "export a function that does what the macro does, or mark the macro `#[doc(hidden)]`",
		"static" => "export a function returning the value, or a `const`",
		"item kind" => "export a function, type, trait, or constant that wraps it, or mark it `#[doc(hidden)]`",
		"excluded type" => "the config leaves this type out, so leave out each item that names it too",
		"doc(hidden) type" => "remove `#[doc(hidden)]` from the type, or mark this item `#[doc(hidden)]` too",
		"discriminant outside `i32`" => "give each variant a discriminant within `i32`, as a C enum holds",
		"non-exhaustive variant" => "remove `#[non_exhaustive]` from the variant, since a binding lists every field",

		// Types.
		"union" => "use a struct or an enum",
		"unsized type" => {
			"pass `&str` or `&[T]` instead, or return an owned `String` or `Vec<T>`; a struct whose last field is unsized needs a sized form"
		}
		"recursive value" => {
			"hold the child behind an `Arc`, or make the type an object by giving it a private field, so it is no value that holds itself"
		}
		"generic type" => "give the trait no type parameters, since only a trait C can implement exports",
		"type argument" => {
			"instantiate it at types that map, such as integers, `String`, or an exported type; `&str`, `Cow`, and other borrows do not"
		}
		"instance holding an instance of a type inside it" => {
			"do not nest the generic type in itself at a deeper instance, as `Cell<Cell<u8>>` does"
		}
		"unmet bound of the instance" | "unmet bound" => {
			"implement the bound for the type, or loosen it; an instance exports only where every bound holds"
		}
		"const argument" => "use a `bool` or an integer below 128 bits for a const argument",
		"const generic" => {
			"pass the const parameter to a generic type in the signature, so each instance an export names supplies its values"
		}
		"constant type" | "constant value" => {
			"export a `bool` or an integer below 128 bits, a value that evaluates at compile time, or a function returning the value"
		}
		"type" => "use a type the README's rules table maps, or wrap it in one of your own",
		"128-bit integer" => "use `u128`, which exports, or an integer of 64 bits or fewer",
		"NonZero" => "use `NonZero` of an integer of 64 bits or fewer",
		"Range" | "Bound" => "use `Range` or `Bound` of an integer of 64 bits or fewer",
		"never type" => "return `()`, or no value, instead of `!`",
		"raw pointer" => "take a reference, `Vec<u8>`, or `&[u8]`, or wrap the pointer in a safe type of your own",
		"function value" => "take a generic `impl Fn(..)` instead of a function pointer or a closure's type",
		"opaque type" => {
			"return a named type, or an `impl Trait` hiding an exported object that captures no borrow, as `+ use<>` ensures"
		}
		"type parameter" => {
			"bound it by a trait the exporter maps, such as `Into<String>`, `AsRef<str>`, `Fn(..)`, or `Read + Seek`, or take a concrete type"
		}
		"generic parameter" => {
			"bound it by a trait the exporter maps, such as `Into<String>`, `AsRef<str>`, `Fn(..)`, or `IntoIterator`, or one of the crate's own traits, or take a concrete type"
		}
		"foreign type" => {
			"use a type that maps, such as `BTreeMap` for a `HashMap` and `Vec` for an iterator, or wrap it in a type of your own whose methods take and return mapped types"
		}
		"refused type" => "fix the refusal of the type it names first, or use a type that exports",
		"type not reachable from the crate root" => {
			"re-export the type with `pub use` from the root, or make it public in its own crate"
		}
		"instance holding a foreign future" => "name a future of `()` C implements, or make the instance own no future",
		"object in a value" => {
			"hold a `Copy` handle, or make the containing type an object by giving it a private field"
		}
		"Arc" => {
			"take an `Arc` of a type that is `Send` and `Sync` and maps, or the type itself, which C holds as an `Arc` already"
		}
		"&mut of another crate's Arc" => {
			"take `&Arc<T>`, or the `Arc` by value, since an opaque handle cannot be replaced in place"
		}
		"another crate's type both by value and in an Arc" => {
			"name the type one way across the facade's signatures, behind an `Arc` or by value"
		}
		"Pin" => "take `Pin<&mut F>` only of a future in a poll, or take the value unpinned",
		"Waker outside a parameter or a return" => {
			"take or return a `Waker` only as a parameter or a return, not in a field or a collection"
		}
		"Context outside a return or a polled parameter" => {
			"take `&mut Context` as the parameter of a function returning `Poll`, or take a `Waker`"
		}
		"nested Option" => "use one `Option`, or wrap the inner one in an enum, since C has one `None` per option",
		"Result outside a return" => "return the `Result`, instead of taking or holding it",
		"unit outside a return" => "leave `()` out of fields, collections, and options, or use a `bool`",
		"unit error" => {
			"return a `bool` for `Result<(), ()>` or an `Option<T>` for `Result<T, ()>`, or give the error a value, such as a `String` or an error type of yours"
		}
		"array of an element other than u8" => "take `&[T]` or `Vec<T>`; an array of other elements only returns",
		"array of unknown length" => "write the length as a literal, not a const generic parameter",
		"empty array" => "remove the zero-length array, which C cannot hold",
		"array too long" => "take a slice, `&[u8]`, instead of an array this long",
		"boxed slice of bytes" => "use `Vec<u8>` or `bytes::Bytes` instead of `Box<[u8]>`",
		"boxed slice of references"
		| "Vec of references"
		| "slice of references"
		| "collection of references"
		| "iterator of references"
		| "tuple of references" => {
			"take owned items, or only return the references: C lends nothing inside a collection, tuple, or iterator"
		}
		"collection not built from its items" => {
			"use a standard collection, or one implementing `FromIterator` of its items"
		}
		"trait object" => {
			"take a generic `impl Trait` or an `Arc<dyn Trait>` of one of the crate's traits C can implement, or a closure `impl Fn(..)`"
		}
		"reference field" => "make the field owned, a `String`, `Vec<T>`, or `Arc<T>`, since a field holds no borrow",
		"reference to a handle inside a value" => {
			"take the object as `&T` itself, not inside an `Option` or a `Box`, which move it out of the caller's handle"
		}
		"borrowed return that is not Clone" => {
			"return an owned value or a `Clone` type, since a borrowed return crosses as a clone"
		}
		"&mut of a slice" => {
			"take `&[u8]` and return the changed `Vec<u8>`, or hold the buffer in an object; C lends no buffer mutably"
		}
		"reader not lent as &mut" => "take the reader as `&mut R`",
		"'static borrow" => {
			"take an owned value, such as a `String` or an `Arc<T>`, where the handle keeps it; only `&str` and `&[u8]` may be kept"
		}
		"reference inside a parameter" => {
			"take the reference as the whole parameter, or as `Some` of an `Option`, and an owned value inside a `Box` or a struct"
		}
		"returned object that borrows" => {
			"return an owned object that borrows nothing, or a guard of a `&mut self` method borrowing the receiver alone"
		}
		"lent object that borrows" => {
			"name no lifetime on the lent type, or fix it to `'static`, since a handle holds it at `'static`"
		}
		"returned borrow of a value written back" => {
			"return an owned value, or make the type an object by giving it a private field, so `&mut self` writes nothing back"
		}

		// Functions.
		"unsafe fn" | "C-variadic function" | "uncallable ABI" => {
			"make it safe, or export a safe function that checks the preconditions and calls it"
		}
		"`&mut self` of a value"
		| "`&mut self` of a value in a future"
		| "`&mut self` of a value beside a JNI value" => {
			"take `&self`, or return the changed value instead of mutating it, or make the type an object by giving it a private field"
		}
		"`&mut self` of a value returning a Result" => {
			"return the changed value with the result, or make the type an object by giving it a private field, since a failure would lose the change"
		}
		"&mut of a value inside an Option" => "take the `&mut` value directly, not inside an `Option`",
		"&mut of a value in a callback" | "&mut of a value in a future" => {
			"take the value by value and return the changed one, or hold it in an object changed through `&self`"
		}
		"&mut of a value beside a JNI value" => {
			"take the value by value and return the changed one, or drop the JNI value"
		}
		"receiver" => "take `self`, `&self`, or `&mut self`",
		"several task parameters" => "take one `Context` or one waiter, not several",
		"Poll with no Context or waiter parameter" => {
			"take a `&mut Context`, or a waiter, which a poll registers its wake through"
		}
		"borrowed Poll" => "return an owned `Poll<T>` that borrows nothing from its arguments",
		"Context of a function returning a future" => {
			"take a `Context` only in a function returning `Poll`; a function returning a future makes its own"
		}
		"borrowing output of a future from a function that is not async" => {
			"make it an `async fn`, or return an output that borrows nothing"
		}
		"poll taking self by value" => "take `&mut self`, or make the type `Copy`, so each poll can call it again",
		"poll's parameter taken by value that is not Copy" => {
			"take it by reference, or make it `Copy`, since each poll calls the function again"
		}
		"iterator of borrows in a future or callback" => {
			"take `Vec<T>` or `impl IntoIterator<Item = T>` of owned items, which the future or callback may keep"
		}
		"iterator in a callback" => "take a `Vec<T>` or a slice, not an iterator",
		"reader lent to a future" => "read the bytes first, and pass them as `Vec<u8>`",
		"JNI value in a callback" | "JNI value lent to a future" => {
			"take JNI's `&mut Env` and `JObject` in a plain function, which Kotlin calls, not in a callback or a future"
		}
		"several JNI envs" => "take one `&mut jni::Env`",
		"JObject with no &mut jni::Env" => "take a `&mut jni::Env` beside the `JObject`",
		"JNI type other than a parameter's &mut Env or JObject" => {
			"take only `&mut jni::Env` or `JObject` of jni's types, and as a parameter"
		}
		"callback bound" => "fix the refusal of the trait it names first, or take a concrete type",
		"ambiguous conversion" => {
			"convert from one of the sources, as `&str` and `String` are one value, not both a `String` and a `Vec<u8>`, which C passes alike"
		}
		"growing conversion" => "break the chain of `From` impls, which names ever more types",
		"closure type taken twice" => "take the closure's type once, with a type parameter for each closure",
		"closure bound" => {
			"bound the closure by `Fn`, `FnMut`, or `FnOnce` alone, besides `Send`, `Sync`, and a lifetime"
		}
		"closure signature" => {
			"take and return values a callback maps: `bool`, integers, `String`, objects by reference, opaque values, an `Option`, or a `Result` with a `String` error"
		}
		"reader bound" => "bound the reader by `Read` and `Seek` alone",
		"reader kept past the call" => "do not return or keep the reader, which C lends only for the call",

		// Traits.
		"unsafe trait" => "make the trait safe, or take an exported type instead of the trait",
		"trait with a blanket impl" => {
			"implement the trait for the types it covers, not for every `T`, so C's implementation cannot clash"
		}
		"supertrait" => "drop the supertrait, which only a marker such as `Send` or `Sync` may be",
		"async trait method" => "return the value directly, since C implements no async method",
		"callback signature" => {
			"take and return values C passes, such as integers, `String`, and objects by reference, and return no `Result` or `Future`"
		}
		"trait method without &self" => "take `&self` or `&mut self`, which C calls its implementation through",
		"associated type" | "associated const" => {
			"move it into the methods' signatures or a function, since C implements methods alone"
		}

		// The Swift binding's.
		"optional borrowed objects past what Swift overloads" => {
			"take at most two optional borrowed objects, or split the function, or group them into a record"
		}
		"error holding an object Swift cannot copy" => {
			"hold values, a `String`, or a `Copy` handle in the error, since `Swift.Error` must be copyable"
		}
		_ => return None,
	})
}

#[cfg(test)]
mod tests {
	use proc_macro2::{Delimiter, TokenStream, TokenTree};

	use crate::collect::{BOTH, NOT_INFERRED, UNHELD};

	/// Each string literal's value, which `tokens` holds at `i`, if it is one.
	fn string(tokens: &[TokenTree], i: usize) -> Option<String> {
		let TokenTree::Literal(lit) = tokens.get(i)? else {
			return None;
		};
		let text = lit.to_string();
		Some(text.strip_prefix('"')?.strip_suffix('"')?.to_string())
	}

	/// Every construct `source` names: the first argument of `why(..)`, the value of a
	/// `construct:` field, the arms of a `let construct = match`, and what `left_out.insert`
	/// takes.
	fn constructs(stream: TokenStream, found: &mut Vec<String>) {
		let tokens: Vec<TokenTree> = stream.into_iter().collect();
		for (i, token) in tokens.iter().enumerate() {
			match token {
				TokenTree::Ident(ident) if ident == "why" => {
					if let Some(TokenTree::Group(group)) = tokens.get(i + 1)
						&& group.delimiter() == Delimiter::Parenthesis
					{
						let inner: Vec<TokenTree> = group.stream().into_iter().collect();
						found.extend(string(&inner, 0));
					}
				}
				TokenTree::Ident(ident) if ident == "construct" => {
					// `construct: "a" | "b"` in a struct or a pattern.
					if matches!(tokens.get(i + 1), Some(TokenTree::Punct(p)) if p.as_char() == ':') {
						let mut at = i + 2;
						while let Some(value) = string(&tokens, at) {
							found.push(value);
							match tokens.get(at + 1) {
								Some(TokenTree::Punct(p)) if p.as_char() == '|' => at += 2,
								_ => break,
							}
						}
					}
					// `let construct = match .. { .. => "a", .. }`.
					if matches!(tokens.get(i + 1), Some(TokenTree::Punct(p)) if p.as_char() == '=')
						&& matches!(tokens.get(i + 2), Some(TokenTree::Ident(m)) if m == "match")
						&& let Some(TokenTree::Group(arms)) = tokens[i + 3..]
							.iter()
							.find(|t| matches!(t, TokenTree::Group(g) if g.delimiter() == Delimiter::Brace))
					{
						let arms: Vec<TokenTree> = arms.stream().into_iter().collect();
						for (j, arm) in arms.iter().enumerate() {
							if matches!(arm, TokenTree::Punct(p) if p.as_char() == '>')
								&& matches!(arms.get(j.wrapping_sub(1)), Some(TokenTree::Punct(p)) if p.as_char() == '=')
							{
								found.extend(string(&arms, j + 1));
							}
						}
					}
				}
				TokenTree::Ident(ident) if ident == "left_out" => {
					if let Some(TokenTree::Group(group)) = tokens.get(i + 3)
						&& group.delimiter() == Delimiter::Parenthesis
					{
						let inner: Vec<TokenTree> = group.stream().into_iter().collect();
						found.extend(string(&inner, inner.len().wrapping_sub(1)));
					}
				}
				TokenTree::Group(group) => constructs(group.stream(), found),
				_ => {}
			}
		}
	}

	/// Every construct the collector or Swift refuses an item for has a note saying how to
	/// change the facade, so a new construct lands with its note.
	#[test]
	fn every_refusal_has_help() {
		let mut found = vec![BOTH.to_string()];
		for source in [include_str!("collect.rs"), include_str!("swift.rs")] {
			let stream: TokenStream = source.parse().expect("the source tokenizes");
			constructs(stream, &mut found);
		}
		// A sanity floor: the scan must still find the constructs it reads.
		assert!(found.len() > 100, "the scan found {} constructs", found.len());
		let missing: Vec<&String> = found
			.iter()
			.filter(|c| !UNHELD.contains(&c.as_str()) && c.as_str() != NOT_INFERRED && super::of(c).is_none())
			.collect();
		assert!(missing.is_empty(), "no `help` for the constructs {missing:?}");
	}
}
