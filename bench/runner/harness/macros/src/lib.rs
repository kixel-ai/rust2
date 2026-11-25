//! The `#[bench]` attribute of rust2-bench-harness: it keeps the function as written and
//! registers it as a benchmark at link time. Its docs are the harness's.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, Expr, FnArg, ItemFn, Path, Safety, Type};

/// Marks a function as a benchmark; see rust2-bench-harness.
#[proc_macro_attribute]
pub fn bench(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	expand(attr.into(), item.into())
		.unwrap_or_else(Error::into_compile_error)
		.into()
}

/// What the attribute declares.
#[derive(Default)]
struct Options {
	/// The arguments, one case each, as an expression a `for` loop takes.
	args: Option<Expr>,
	/// The function that builds the benchmark's input, from its argument when it has args.
	setup: Option<Path>,
}

fn options(attr: TokenStream) -> syn::Result<Options> {
	let mut options = Options::default();
	let parser = syn::meta::parser(|meta| {
		if meta.path.is_ident("args") {
			if options.args.is_some() {
				return Err(meta.error("args is given twice"));
			}
			options.args = Some(meta.value()?.parse()?);
		} else if meta.path.is_ident("setup") {
			if options.setup.is_some() {
				return Err(meta.error("setup is given twice"));
			}
			options.setup = Some(meta.value()?.parse()?);
		} else {
			return Err(meta.error("#[bench] takes args and setup alone"));
		}
		Ok(())
	});
	syn::parse::Parser::parse2(parser, attr)?;
	Ok(options)
}

/// How the benchmark takes its one parameter.
enum Pass {
	/// By value, moving it in.
	Value,
	/// By shared reference, the harness keeping it until the count has stopped.
	Shared,
	/// By mutable reference, the same.
	Mut,
}

fn expand(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
	let options = options(attr)?;
	let function: ItemFn = syn::parse2(item)?;
	let sig = &function.sig;
	let refuse = |tokens: &dyn quote::ToTokens, what: &str| Err(Error::new_spanned(tokens, what));
	if let Some(token) = &sig.asyncness {
		return refuse(token, "a benchmark cannot be async: poll its future inside it");
	}
	if let Safety::Unsafe(token) = &sig.safety {
		return refuse(token, "a benchmark cannot be unsafe: wrap the unsafe calls inside it");
	}
	if !sig.generics.params.is_empty() || sig.generics.where_clause.is_some() {
		return refuse(&sig.generics, "a benchmark cannot be generic: give each type its own");
	}
	if let Some(variadic) = &sig.variadic {
		return refuse(variadic, "a benchmark cannot be variadic");
	}
	let mut inputs = sig.inputs.iter();
	let param = inputs.next();
	if let Some(extra) = inputs.next() {
		return refuse(
			extra,
			"a benchmark takes one parameter at most: its argument, or with setup, setup's output",
		);
	}
	let pass = match param {
		None => None,
		Some(FnArg::Receiver(receiver)) => return refuse(receiver, "a benchmark is a free function, not a method"),
		// Without setup the argument moves in as args holds it, whatever its type; with setup,
		// a reference borrows setup's output, which the harness drops after the count stops.
		Some(FnArg::Typed(typed)) if matches!(&*typed.ty, Type::ImplTrait(_)) => {
			return refuse(&typed.ty, "a benchmark cannot be generic: name its parameter's type");
		}
		Some(FnArg::Typed(typed)) => Some(match (&*typed.ty, &options.setup) {
			(Type::Reference(reference), Some(_)) if reference.mutability.is_some() => Pass::Mut,
			(Type::Reference(_), Some(_)) => Pass::Shared,
			_ => Pass::Value,
		}),
	};
	match (&param, &options.args, &options.setup) {
		(Some(param), None, None) => {
			return refuse(
				*param,
				"nothing supplies this parameter: declare args or setup in #[bench]",
			);
		}
		(None, Some(args), None) => return refuse(args, "the benchmark takes no parameter for its argument"),
		(None, _, Some(setup)) => return refuse(setup, "the benchmark takes no parameter for setup's output"),
		_ => {}
	}

	let ident = &sig.ident;
	let h = quote!(::rust2_bench_harness::__private);
	// Locals hygienic to the expansion, so no item of the same name, such as a benchmark
	// named `input`, is shadowed by them.
	let local = |name: &str| proc_macro2::Ident::new(name, proc_macro2::Span::mixed_site());
	let (input, output, case, meter) = (local("input"), local("_output"), local("case"), local("meter"));
	// What runs before the count starts, from the argument `arg` when there are args, and the
	// parameter the benchmark is called with.
	let prepare = |arg: &TokenStream| match (&options.setup, &pass) {
		(_, None) => quote!(),
		(None, Some(_)) => quote!(let #input = #arg;),
		(Some(setup), Some(Pass::Mut)) => quote!(let mut #input = #setup(#arg);),
		(Some(setup), Some(_)) => quote!(let #input = #setup(#arg);),
	};
	let pass = match (&options.setup, &pass) {
		(_, None) => quote!(),
		(Some(_), Some(Pass::Shared)) => quote!(#h::black_box(&#input)),
		(Some(_), Some(Pass::Mut)) => quote!(#h::black_box(&mut #input)),
		_ => quote!(#h::black_box(#input)),
	};
	// Called through a pointer the optimizer cannot see through, the benchmark compiles on
	// its own, as written: inlined, an input whose address escaped to `black_box` here would
	// be reloaded around every `black_box` inside it. The pointer's parameter is the
	// benchmark's own type, which an argument's literal would otherwise infer, such as `i32`
	// for `[1000]`.
	let abi = &sig.abi;
	let types = sig.inputs.iter().map(|input| match input {
		FnArg::Typed(typed) => &typed.ty,
		FnArg::Receiver(_) => unreachable!("a receiver was refused"),
	});
	let call = quote!(#h::black_box(#ident as #abi fn(#(#types),*) -> _)(#pass));
	let arg = options.args.as_ref().map(|args| quote!(#h::arg(#args, #case)));
	let run = prepare(&arg.unwrap_or_default());
	// Named through a closure, never called, that sets up and calls the case as running it
	// does, so each argument gets the type and coercions it runs with, such as `u64` for
	// `[4_294_967_296]`.
	let names = match &options.args {
		Some(args) => {
			let taken = local("arg");
			let witness = prepare(&quote!(#taken));
			quote!(::core::option::Option::Some(|| #h::names(#args, |#taken| {
				#witness
				let _ = #call;
			})))
		}
		None => quote!(::core::option::Option::None),
	};
	let span = ident.span();
	let registration = quote::quote_spanned! {span=>
		const _: () = {
			::core::assert!(
				#h::relative(::core::file!()),
				"a benchmark's file must be relative to its workspace, as Cargo builds a member's: \
				 build it from the workspace that holds it, or remap its path with --remap-path-prefix",
			);
			fn __rust2_bench_run(#case: usize, #meter: &#h::Meter) {
				let _ = #case;
				#run
				#meter.start();
				let #output = #h::black_box(#call);
				#meter.stop();
				// The output, then any input setup built, drop here, after the count stopped.
			}
			#h::inventory::submit! {
				#h::Bench {
					file: ::core::file!(),
					module: ::core::module_path!(),
					name: ::core::stringify!(#ident),
					args: #names,
					run: __rust2_bench_run,
				}
			}
		};
	};
	Ok(quote! {
		#function
		#registration
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	fn error(attr: &str, item: &str) -> String {
		match expand(attr.parse().unwrap(), item.parse().unwrap()) {
			Ok(_) => panic!("#[bench({attr})] {item} expanded"),
			Err(err) => err.to_string(),
		}
	}

	fn expands(attr: &str, item: &str) -> String {
		let attr: TokenStream = attr.parse().unwrap();
		let item: TokenStream = item.parse().unwrap();
		expand(attr, item).unwrap().to_string()
	}

	#[test]
	fn refuses_what_it_cannot_call() {
		for (attr, item, message) in [
			("", "async fn f() {}", "cannot be async"),
			("", "unsafe fn f() {}", "cannot be unsafe"),
			("", "fn f<T>() {}", "cannot be generic"),
			("", "fn f() where u8: Copy {}", "cannot be generic"),
			("", "fn f(&self) {}", "not a method"),
			("args = [1]", "fn f(a: impl Copy) {}", "cannot be generic"),
			("args = [1]", "fn f(a: u8, b: u8) {}", "one parameter at most"),
			("", "fn f(a: u8) {}", "nothing supplies this parameter"),
			("args = [1]", "fn f() {}", "no parameter for its argument"),
			("setup = s", "fn f() {}", "no parameter for setup's output"),
			("arg = [1]", "fn f(a: u8) {}", "takes args and setup alone"),
			("args = [1], args = [2]", "fn f(a: u8) {}", "args is given twice"),
			("setup = a, setup = b", "fn f(a: u8) {}", "setup is given twice"),
		] {
			let err = error(attr, item);
			assert!(err.contains(message), "#[bench({attr})] {item}: {err}");
		}
	}

	#[test]
	fn passes_setup_output_as_the_parameter_takes_it() {
		let by = |item: &str| expands("setup = s", item);
		assert!(by("fn f(x: &mut X) {}").contains("let mut input = s () ;"));
		assert!(
			by("fn f(x: &mut X) {}")
				.contains("(f as fn (& mut X) -> _) (:: rust2_bench_harness :: __private :: black_box (& mut input))")
		);
		assert!(by("fn f(x: &X) {}").contains("black_box (& input)"));
		assert!(by("fn f(x: X) {}").contains("black_box (input)"));
		// Without setup, the argument moves in even where the parameter is a reference.
		let arg = expands("args = [\"a\"]", "fn f(x: &str) {}");
		assert!(
			arg.contains("let input = :: rust2_bench_harness :: __private :: arg ([\"a\"] , case) ;"),
			"{arg}"
		);
		assert!(
			arg.contains("(f as fn (& str) -> _) (:: rust2_bench_harness :: __private :: black_box (input))"),
			"{arg}"
		);
		let none = expands("", "extern \"C\" fn f() {}");
		assert!(none.contains("(f as extern \"C\" fn () -> _) ()"), "{none}");
		assert!(
			arg.contains("names ([\"a\"] , | arg | { let input = arg ; let _ ="),
			"{arg}"
		);
		let both = expands("args = SIZES, setup = s", "fn f(x: &X) {}");
		assert!(
			both.contains("names (SIZES , | arg | { let input = s (arg) ;"),
			"{both}"
		);
		assert!(
			both.contains("s (:: rust2_bench_harness :: __private :: arg (SIZES , case))"),
			"{both}"
		);
	}
}
