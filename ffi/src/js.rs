//! The JavaScript and declarations of each TypeScript module the web binding writes, so a
//! registry's consumer runs it with no TypeScript loader: Node refuses to strip types under
//! `node_modules`. The modules hold only syntax type stripping erases, and every export
//! states its type, so each file's declarations come from it alone, as JSR requires too.

use std::path::Path;

use oxc::allocator::Allocator;
use oxc::codegen::Codegen;
use oxc::diagnostics::OxcDiagnostic;
use oxc::isolated_declarations::{IsolatedDeclarations, IsolatedDeclarationsOptions};
use oxc::parser::Parser;
use oxc::semantic::SemanticBuilder;
use oxc::span::SourceType;
use oxc::transformer::{TransformOptions, Transformer, TypeScriptOptions};

/// The JavaScript and the declarations of the module `name`, a `.ts` file, whose imports
/// of its neighbors name their `.js` files instead.
pub(crate) fn emit(name: &str, ts: &str) -> Result<(String, String), String> {
	let fail = |what: &str, errors: &[OxcDiagnostic]| {
		let errors: Vec<String> = errors.iter().map(|e| format!("{name}: {e}")).collect();
		format!("the web binding's {what} failed, a bug in rust2-ffi:\n{}", errors.join("\n"))
	};
	let allocator = Allocator::default();
	let path = Path::new(name);
	let parsed = Parser::new(&allocator, ts, SourceType::ts()).parse();
	if !parsed.diagnostics.is_empty() {
		return Err(fail("parse", &parsed.diagnostics));
	}
	// Each import of a neighbor, `"./<name>.ts"`, names its `.js` file instead: the same
	// length, so every span the parse found still holds.
	let mut source = ts.to_string();
	for (specifier, requests) in &parsed.module_record.requested_modules {
		if !(specifier.starts_with("./") && specifier.ends_with(".ts")) {
			continue;
		}
		for request in requests {
			// The span holds the specifier's quotes.
			let end = request.span.end as usize - 1;
			if source.get(end - 3..end) != Some(".ts") {
				return Err(format!("{name}: the import of {specifier} is not spelled as written"));
			}
			source.replace_range(end - 3..end, ".js");
		}
	}
	let parsed = Parser::new(&allocator, allocator.alloc_str(&source), SourceType::ts()).parse();
	let mut program = parsed.program;
	// The header comment, which neither printing keeps whole.
	let header: String = (ts.lines())
		.take_while(|line| line.starts_with("//"))
		.map(|line| format!("{line}\n"))
		.collect();

	let declared = IsolatedDeclarations::new(&allocator, IsolatedDeclarationsOptions::default()).build(&program);
	if !declared.diagnostics.is_empty() {
		return Err(fail("declarations", &declared.diagnostics));
	}
	let dts = format!("{header}{}", Codegen::new().build(&declared.program).code);

	let scoping = SemanticBuilder::new().build(&program).semantic.into_scoping();
	let options = TransformOptions {
		typescript: TypeScriptOptions {
			// `verbatimModuleSyntax`'s rule: an import is erased only if it says it is a type's.
			only_remove_type_imports: true,
			..TypeScriptOptions::default()
		},
		// No lowering: the modules are ES2024 as they are written.
		..TransformOptions::default()
	};
	let stripped = Transformer::new(&allocator, path, &options).build_with_scoping(scoping, &mut program);
	if !stripped.diagnostics.is_empty() {
		return Err(fail("type stripping", &stripped.diagnostics));
	}
	let js = Codegen::new().build(&program).code;
	// Printing keeps the header but its blank line.
	let js = match js.strip_prefix(&header) {
		Some(rest) if !header.is_empty() => format!("{header}\n{rest}"),
		_ => js,
	};
	Ok((js, dts))
}

#[cfg(test)]
mod tests {
	use super::emit;

	#[test]
	fn strips_and_rewrites() {
		let ts = "// Generated.\n\nimport * as rt from \"./rt.ts\";\nimport type { A } from \"./A.ts\";\n\n/** Doc. */\nexport function f(a: A, n: number): rt.u64 {\n\treturn rt.u64(n as number);\n}\n";
		let (js, dts) = emit("f.ts", ts).unwrap();
		assert_eq!(
			js,
			"// Generated.\n\nimport * as rt from \"./rt.js\";\n/** Doc. */\nexport function f(a, n) {\n\treturn rt.u64(n);\n}\n"
		);
		assert!(dts.starts_with("// Generated.\n"), "{dts}");
		assert!(dts.contains("import * as rt from \"./rt.js\";"), "{dts}");
		assert!(dts.contains("import type { A } from \"./A.js\";"), "{dts}");
		assert!(
			dts.contains("export declare function f(a: A, n: number): rt.u64;"),
			"{dts}"
		);
	}

	#[test]
	fn refuses_an_inferred_export() {
		let err = emit("g.ts", "declare function f(): number;\nexport const g = f();\n").unwrap_err();
		assert!(err.contains("g.ts"), "{err}");
	}
}
