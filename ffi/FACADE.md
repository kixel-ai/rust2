# Writing a facade for rust2-ffi

[rust2-ffi](README.md) exports the `pub` API of one crate, the facade, and refuses
what it cannot map.
A stock crate's whole API rarely maps, so a facade is a small crate of your own that
re-exports what maps and wraps what does not.
This guide goes from a crate you want in another language to a facade with no refusals.
[The README's rules](README.md#rules) are the full table of what maps.

## The loop

1. Make a crate whose library is the facade, depending on the crate at one version:

   ```toml
   [package]
   name = "shortlink"
   version = "0.0.0"
   edition = "2024"

   [dependencies]
   url = "=2.5.8"
   ```

   Start with `pub use url::*;` in `src/lib.rs`: it is the whole API, and what refuses is
   your list of work.
2. Count it with a census, which exports nothing:

   ```bash
   echo 'exclude = []' > ffi.toml
   rust2-ffi --config ffi.toml --census census --manifest-path Cargo.toml
   ```

3. Read `census/<package>.md`, as below, and fix the top construct.
4. Repeat until the census refuses nothing, then export with `--out dir`.

Each refusal is also a compile error at its span when you export, with a `help:` note naming
the change to make. The census only gathers them.

## What exports

A facade exports what its `pub` items and their signatures name, by fixed rules:

- Integers but `i128`, floats, `bool`, `char`, `String`, `&str`, `&[u8]`, `Vec<T>`,
  `Option<T>`, `BTreeMap`, `Duration`, socket addresses, and `Result<T, E>` map by value.
  A `Result`'s error becomes each language's own error.
- A struct of `pub` fields that map is a record, copied across; an enum is a C enum or a
  tagged union.
- Any other type is an object: an opaque handle with its `pub` methods, which is the usual
  shape of a library's type.
- A trait is a callback interface C can implement; a closure parameter is a C function.
- `async fn` and `Future` returns are futures every language awaits in its own way.
- A generic type exports at the instances the facade's signatures and impls name.

What does not map is, in short: a borrow that outlives the call, a type of another crate no
rule names, an iterator, a `&mut` of a slice, a `Result` with a `()` error, an `unsafe fn`,
and a generic with no bound the exporter maps.

## What refuses, and what to change

A refusal reads as a rustc error at the item's span:

```text
error: rust2-ffi: cannot export method `Link::set_port`: unit error `std::result::Result<(), ()>`; ...
  --> src/lib.rs:26:2
   |
26 |     pub fn set_port(&mut self, port: Option<u16>) -> Result<(), ()> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: return a `bool` for `Result<(), ()>` or an `Option<T>` for `Result<T, ()>`, or give the error a value, ...
```

The name after `cannot export` is the item, then the **construct**, which the census ranks
and the `help:` note is keyed by. A construct whose detail is a type names it in backticks.
The common ones, from refusals in the crates.io top 300:

| Construct | Seen in | Change |
| --- | --- | --- |
| foreign type | `url`'s `path_segments` returns `Split` | Collect it into a `Vec`, or use `BTreeMap` for `HashMap`. |
| unit error | `url`'s `set_port` returns `Result<(), ()>` | Return a `bool`, an `Option<T>`, or an error with a value. |
| borrowed return that is not Clone | `walkdir`'s `file_name` returns `&OsStr` | Return an owned `OsString`, or a `String`. |
| returned borrow of a value written back | `uuid`'s `encode_lower` returns `&mut str` of a buffer it was given | Return an owned `String`. |
| &mut of a slice | `base64`'s `encode_engine_slice` writes into `&mut [u8]` | Return the bytes as a `Vec<u8>`. |
| unsized type | `base64`'s `encode_engine` takes `T: AsRef<[u8]>` | Take `&[u8]` or `Vec<u8>`. |
| 'static borrow | `url`'s `ParseOptions::base_url` holds `Option<&Url>` | Hold an owned value, such as an `Arc` or a clone. |
| generic parameter | `shlex`'s `join` takes any `IntoIterator` | Take `Vec<String>`, or bound it by `Into<String>`, `AsRef<str>`, or `Fn`. |
| closure signature | `walkdir`'s `sort_by` returns an `Ordering` | Return a value C holds, such as an integer or `bool`. |
| unsafe fn | `shlex`'s `from_bytes` | Export a safe function that checks what it needs. |
| macro | `indexmap`'s `indexmap!` | Export a function that does the same. |
| re-export of a whole crate | `url`'s `pub use form_urlencoded;` | Re-export its items, `pub use dep::*;`, or wrap the few it needs. |

Where the crate's own type is the problem, as in `Url`'s methods above, the facade defines a
type of its own around it and exposes the methods that map, each calling the original:

```rust
pub struct Link(url::Url);

impl Link {
	pub fn parse(text: &str) -> Result<Link, url::ParseError> {
		url::Url::parse(text).map(Link)
	}

	pub fn segments(&self) -> Vec<String> {
		match self.0.path_segments() {
			Some(segments) => segments.map(str::to_string).collect(),
			None => Vec::new(),
		}
	}

	pub fn set_port(&mut self, port: Option<u16>) -> bool {
		self.0.set_port(port).is_ok()
	}
}
```

This exports with no refusals: `Link` is an object, `parse` fails with `ParseError`'s message,
and every other method is a call a language can make.

## Reading the census

`census/<package>.md` holds, in order:

- **A table of counts**: types, functions, methods and fields, traits, constants, and what
  share of each exports.
- **Refusals by construct**: each construct, how many items it refuses (an item refused for
  several counts under each), and the first span. Fix the top row first: it is the most items
  for one change.
- **Refusals by type or path**: the same by the type the construct names, as 18 items
  refused for `[u8]` points at one signature shape.
- **Refused items**: each item, its span, and every reason, which is what the compile error
  prints.
- **The config that exports what maps**: the `exclude` that leaves the refused items out,
  which the export accepts today.
- **What exports another way, or is left out and listed**: a value exported as an object
  since a field does not map, a `#[doc(hidden)]` item, what C can never hold, such as an
  object no export returns, and a type alias. None of these is an error; they need no change
  unless you want them exported as they are.

A census that refuses nothing is the facade done. Its counts show the share of the API
that exports, so one that is low is a facade whose wrapper types are still to write.
