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
	/// A step on setup's output before each call, neither counted nor timed.
	before: Option<Path>,
	/// A step on setup's output after each call, neither counted nor timed.
	after: Option<Path>,
}

fn options(attr: TokenStream) -> syn::Result<Options> {
	let mut options = Options::default();
	let parser = syn::meta::parser(|meta| {
		let (slot, name) = if meta.path.is_ident("setup") {
			(&mut options.setup, "setup")
		} else if meta.path.is_ident("before") {
			(&mut options.before, "before")
		} else if meta.path.is_ident("after") {
			(&mut options.after, "after")
		} else if meta.path.is_ident("args") {
			if options.args.is_some() {
				return Err(meta.error("args is given twice"));
			}
			options.args = Some(meta.value()?.parse()?);
			return Ok(());
		} else {
			return Err(meta.error("#[bench] takes args, setup, before, and after alone"));
		};
		if slot.is_some() {
			return Err(meta.error(format!("{name} is given twice")));
		}
		*slot = Some(meta.value()?.parse()?);
		Ok(())
	});
	syn::parse::Parser::parse2(parser, attr)?;
	Ok(options)
}

/// How the benchmark takes its input.
enum Pass {
	/// By value, moving it in.
	Value,
	/// By shared reference, the harness keeping it until the count has stopped.
	Shared,
	/// By mutable reference, the same.
	Mut,
}

/// Whether `ty` names the harness's `Iterations`, which a benchmark that times itself takes.
fn iterations(ty: &Type) -> bool {
	matches!(ty, Type::Path(path) if path.qself.is_none() && path.path.segments.last().is_some_and(|last| last.ident == "Iterations" && last.arguments.is_none()))
}

fn expand(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
	let options = options(attr)?;
	let function: ItemFn = syn::parse2(item)?;
	let sig = &function.sig;
	let refuse = |tokens: &dyn quote::ToTokens, what: &str| Err(Error::new_spanned(tokens, what));
	if let Some(token) = &sig.asyncness {
		return refuse(
			token,
			"a benchmark cannot be async: build its runtime in setup and block on the future inside it",
		);
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
	let mut typed = Vec::new();
	for input in &sig.inputs {
		match input {
			FnArg::Receiver(receiver) => return refuse(receiver, "a benchmark is a free function, not a method"),
			FnArg::Typed(param) if matches!(&*param.ty, Type::ImplTrait(_)) => {
				return refuse(&param.ty, "a benchmark cannot be generic: name its parameter's type");
			}
			FnArg::Typed(param) => typed.push(param),
		}
	}
	// A last parameter of `Iterations` makes a benchmark that times itself; the one before it,
	// if any, is its input.
	let timed = typed.last().filter(|param| iterations(&param.ty)).copied();
	let inputs: Vec<_> = typed.iter().take(typed.len() - usize::from(timed.is_some())).collect();
	if let Some(extra) = inputs.get(1) {
		return refuse(
			*extra,
			"a benchmark takes one input at most: its argument, or with setup, setup's output, then Iterations if it times itself",
		);
	}
	let param = inputs.first().copied();
	if let Some(timed) = timed {
		let returns = match &sig.output {
			syn::ReturnType::Type(_, ty) => matches!(&**ty, Type::Path(path) if path.path.segments.last().is_some_and(|last| last.ident == "Duration")),
			syn::ReturnType::Default => false,
		};
		if !returns {
			return refuse(timed, "a benchmark that takes Iterations returns the Duration they took");
		}
	}
	// Without setup the argument moves in as args holds it, whatever its type; with setup,
	// a reference borrows setup's output, which the harness drops after the count stops.
	let pass = param.map(|param| match (&*param.ty, &options.setup) {
		(Type::Reference(reference), Some(_)) if reference.mutability.is_some() => Pass::Mut,
		(Type::Reference(_), Some(_)) => Pass::Shared,
		_ => Pass::Value,
	});
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
	for step in [&options.before, &options.after].into_iter().flatten() {
		if timed.is_some() {
			return refuse(step, "a benchmark that times itself takes its own steps between calls");
		}
		if !matches!(pass, Some(Pass::Shared | Pass::Mut)) {
			return refuse(
				step,
				"a step runs on setup's output between calls, which the benchmark must borrow: one that takes it by value gets a new one each call",
			);
		}
	}

	let ident = &sig.ident;
	let h = quote!(::rust2_bench_harness::__private);
	// Locals hygienic to the expansion, so no item of the same name, such as a benchmark
	// named `input`, is shadowed by them.
	let local = |name: &str| proc_macro2::Ident::new(name, proc_macro2::Span::mixed_site());
	let (input, arg, iterations) = (local("input"), local("arg"), local("iterations"));
	// Called through a pointer the optimizer cannot see through, the benchmark compiles on
	// its own, as written: inlined, an input whose address escaped to `black_box` here would
	// be reloaded around every `black_box` inside it. The pointer's parameter is the
	// benchmark's own type, which an argument's literal would otherwise infer, such as `i32`
	// for `[1000]`.
	let abi = &sig.abi;
	let types = typed.iter().map(|param| &param.ty);
	let pointer = quote!(#h::black_box(#ident as #abi fn(#(#types),*) -> _));
	let mut passed = Vec::new();
	match &pass {
		None => {}
		Some(Pass::Value) => passed.push(quote!(#h::black_box(#input))),
		Some(Pass::Shared) => passed.push(quote!(#h::black_box(&*#input))),
		Some(Pass::Mut) => passed.push(quote!(#h::black_box(&mut *#input))),
	}
	if timed.is_some() {
		passed.push(quote!(#iterations));
	}
	let call = quote!(#pointer(#(#passed),*));
	// What makes the input: setup, from the case's argument `arg` when there are args, or the
	// argument itself. A new input each call is cloned from the argument, which stays the case's.
	let taken = options.args.as_ref().map(|_| quote!(::core::clone::Clone::clone(&#arg)));
	let made = match &options.setup {
		Some(setup) => quote!(#setup(#taken)),
		None => quote!(#taken),
	};
	let kept = options.args.as_ref().map(|_| &arg);
	let step = |step: &Option<Path>| match (step, &pass) {
		(Some(path), Some(Pass::Shared)) => quote!(#h::step(|#input: &mut _| #path(&*#input))),
		(Some(path), _) => quote!(#h::step(|#input: &mut _| #path(#input))),
		(None, _) => quote!(::core::option::Option::None),
	};
	let (before, after) = (step(&options.before), step(&options.after));
	// One case's setup: what the harness runs once its name is known. The args expression is
	// evaluated once, and each argument moves into the case it names, so no case can run
	// another's. Its type is inferred from this body, so each argument gets the type and
	// coercions it runs with, such as `u64` for `[4_294_967_296]`.
	let build = match (&pass, timed) {
		(None, None) => quote!(#h::fresh(|| (), move |()| #call)),
		(None, Some(_)) => quote!(#h::fresh_timed(|| (), move |(), #iterations| #call)),
		(Some(Pass::Value), None) => quote!(#h::fresh(move || #made, move |#input| #call)),
		(Some(Pass::Value), Some(_)) => quote!(#h::fresh_timed(move || #made, move |#input, #iterations| #call)),
		(Some(_), None) => {
			let setup = options.setup.as_ref().expect("a borrowed input comes from setup");
			quote!(#h::kept(#setup(#kept), move |#input: &mut _| #call, #before, #after))
		}
		(Some(_), Some(_)) => {
			let setup = options.setup.as_ref().expect("a borrowed input comes from setup");
			quote!(#h::kept_timed(#setup(#kept), move |#input: &mut _, #iterations| #call))
		}
	};
	let cases = match &options.args {
		Some(args) => quote!(#h::each(#args, |#arg| move || #build)),
		None => quote!(#h::single(move || #build)),
	};
	let span = ident.span();
	let registration = quote::quote_spanned! {span=>
		const _: () = {
			::core::assert!(
				#h::relative(::core::file!()),
				"a benchmark's file must be relative to its workspace, as Cargo builds a member's: \
				 build it from the workspace that holds it, or remap its path with --remap-path-prefix",
			);
			#h::inventory::submit! {
				#h::Bench {
					file: ::core::file!(),
					module: ::core::module_path!(),
					name: ::core::stringify!(#ident),
					cases: || #cases,
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
			("args = [1]", "fn f(a: u8, b: u8) {}", "one input at most"),
			("", "fn f(a: u8) {}", "nothing supplies this parameter"),
			("args = [1]", "fn f() {}", "no parameter for its argument"),
			("setup = s", "fn f() {}", "no parameter for setup's output"),
			("arg = [1]", "fn f(a: u8) {}", "takes args, setup, before, and after alone"),
			("args = [1], args = [2]", "fn f(a: u8) {}", "args is given twice"),
			("setup = a, setup = b", "fn f(a: u8) {}", "setup is given twice"),
			("setup = s, before = a, before = b", "fn f(a: &mut u8) {}", "before is given twice"),
			("setup = s, before = b", "fn f(a: u8) {}", "which the benchmark must borrow"),
			("args = [1], after = b", "fn f(a: &u8) {}", "which the benchmark must borrow"),
			("before = b", "fn f() {}", "which the benchmark must borrow"),
			("", "fn f(n: Iterations) {}", "returns the Duration they took"),
			("", "fn f(n: Iterations) -> u64 {}", "returns the Duration they took"),
			("setup = s, after = b", "fn f(a: &mut u8, n: Iterations) -> Duration {}", "takes its own steps"),
			("args = [1]", "fn f(a: u8, b: u8, n: Iterations) -> Duration {}", "one input at most"),
		] {
			let err = error(attr, item);
			assert!(err.contains(message), "#[bench({attr})] {item}: {err}");
		}
	}

	#[test]
	fn passes_setup_output_as_the_parameter_takes_it() {
		let h = ":: rust2_bench_harness :: __private ::";
		let by = |item: &str| expands("setup = s", item);
		let kept = by("fn f(x: &mut X) {}");
		assert!(
			kept.contains(&format!(
				"{h} kept (s () , move | input : & mut _ | {h} black_box (f as fn (& mut X) -> _) ({h} black_box (& mut * input)) , :: core :: option :: Option :: None"
			)),
			"{kept}"
		);
		assert!(by("fn f(x: &X) {}").contains("black_box (& * input)"));
		let value = by("fn f(x: X) {}");
		assert!(value.contains(&format!("{h} fresh (move || s () , move | input | ")), "{value}");
		assert!(value.contains(&format!("({h} black_box (input))")), "{value}");
		// Without setup, each call takes a clone of the argument, even where the parameter is a reference.
		let arg = expands("args = [\"a\"]", "fn f(x: &str) {}");
		assert!(
			arg.contains(&format!(
				"each ([\"a\"] , | arg | move || {h} fresh (move || :: core :: clone :: Clone :: clone (& arg) , move | input |"
			)),
			"{arg}"
		);
		assert!(arg.contains(&format!("(f as fn (& str) -> _) ({h} black_box (input))")), "{arg}");
		let none = expands("", "extern \"C\" fn f() {}");
		assert!(none.contains("(f as extern \"C\" fn () -> _) ()"), "{none}");
		// A borrowed input is set up once, from the argument itself.
		let both = expands("args = SIZES, setup = s", "fn f(x: &X) {}");
		assert!(both.contains(&format!("each (SIZES , | arg | move || {h} kept (s (arg) ,")), "{both}");
		// And by value, each call's from a clone of it.
		let fresh = expands("args = SIZES, setup = s", "fn f(x: X) {}");
		assert!(fresh.contains("fresh (move || s (:: core :: clone :: Clone :: clone (& arg)) ,"), "{fresh}");
	}

	#[test]
	fn passes_steps_the_input_as_the_benchmark_borrows_it() {
		let shared = expands("setup = s, before = b, after = a", "fn f(x: &X) {}");
		assert!(shared.contains("step (| input : & mut _ | b (& * input))"), "{shared}");
		assert!(shared.contains("step (| input : & mut _ | a (& * input))"), "{shared}");
		let exclusive = expands("setup = s, before = b", "fn f(x: &mut X) {}");
		assert!(exclusive.contains("step (| input : & mut _ | b (input))"), "{exclusive}");
	}

	#[test]
	fn a_benchmark_that_takes_iterations_times_itself() {
		let h = ":: rust2_bench_harness :: __private ::";
		let none = expands("", "fn f(n: Iterations) -> Duration {}");
		assert!(
			none.contains(&format!(
				"{h} fresh_timed (|| () , move | () , iterations | {h} black_box (f as fn (Iterations) -> _) (iterations))"
			)),
			"{none}"
		);
		let kept = expands("setup = s", "fn f(x: &mut X, n: rust2_bench_harness::Iterations) -> std::time::Duration {}");
		assert!(
			kept.contains(&format!(
				"{h} kept_timed (s () , move | input : & mut _ , iterations | {h} black_box (f as fn (& mut X , rust2_bench_harness :: Iterations) -> _) ({h} black_box (& mut * input) , iterations))"
			)),
			"{kept}"
		);
		let arg = expands("args = [1]", "fn f(x: u64, n: Iterations) -> Duration {}");
		assert!(
			arg.contains("fresh_timed (move || :: core :: clone :: Clone :: clone (& arg) , move | input , iterations |"),
			"{arg}"
		);
	}
}
