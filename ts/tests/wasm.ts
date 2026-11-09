// A fixture's wasm32 build (`fixtures/wasm.rs`): the host writes an input into the
// instance's memory, runs the crate's entry on it, and copies the output back out.
// `differential.ts` runs it beside the translation, and `bench/time.ts` times each step.

interface Exports {
	memory: WebAssembly.Memory;
	input(len: number): number;
	run(): number;
	output(): number;
	message(): number;
	message_len(): number;
	note?(): number;
	note_len?(): number;
	/** What a module wasm-bindgen's CLI processed runs before any call. */
	__wbindgen_start?(): void;
}

/**
 * The JS glue wasm-bindgen's CLI makes for a module, `--target bundler`'s `_bg.js`: the
 * imports it gives each instance, and where it keeps the instance it calls into. It
 * caches views of that instance's memory, so each instance needs the module loaded anew.
 */
export type Glue = Record<string, unknown> & { __wbg_set_wasm(exports: unknown): void };

/**
 * How a call ended: its output, a panic with the message the module saved before it
 * trapped, an abort, which trapped without running the panic hook, or what JS threw
 * through it, as an import without `catch` throws.
 */
export type Outcome = Uint8Array | { panic: string } | { thrown: string } | "abort";

export class Wasm {
	#module: WebAssembly.Module;
	#exports!: Exports;
	#glued: boolean;
	/** What a fixture's panic hooks wrote before the last call trapped, if it exports `note`. */
	note = "";
	/** Whether the last call left the instance mid-call, which only `renew` replaces for a module with glue. */
	stale = false;

	constructor(module: WebAssembly.Module, glue?: Glue) {
		this.#module = module;
		this.#glued = glue !== undefined;
		this.#instantiate(glue);
	}

	#instantiate(glue?: Glue) {
		const imports: WebAssembly.Imports = {};
		for (const { module } of WebAssembly.Module.imports(this.#module)) {
			if (glue) imports[module] = glue as WebAssembly.ModuleImports;
		}
		this.#exports = new WebAssembly.Instance(this.#module, imports).exports as unknown as Exports;
		if (glue) {
			glue.__wbg_set_wasm(this.#exports);
			this.#exports.__wbindgen_start?.();
		}
	}

	/** A fresh instance, with a fresh load of the glue's module, after a call left this one stale. */
	renew(glue: Glue) {
		this.#instantiate(glue);
		this.stale = false;
	}

	/** Leave the instance after a call that left it mid-call: a fresh one now, or with glue, once `renew` loads its module anew. */
	#leave() {
		if (this.#glued) this.stale = true;
		else this.#instantiate();
	}

	/** Copy `input` into the instance, where the next `run` reads it. */
	input(input: Uint8Array) {
		const ptr = this.#exports.input(input.length);
		new Uint8Array(this.#exports.memory.buffer, ptr, input.length).set(input);
	}

	/** Run the entry on the last input, returning the output's length. A panic traps. */
	run(): number {
		return this.#exports.run();
	}

	/** Copy the last run's `len` bytes of output out of the instance. */
	output(len: number): Uint8Array {
		return new Uint8Array(this.#exports.memory.buffer, this.#exports.output(), len).slice();
	}

	/** All three, or how the entry trapped. */
	call(input: Uint8Array): Outcome {
		this.note = "";
		try {
			this.input(input);
			return this.output(this.run());
		} catch (err) {
			if (!(err instanceof WebAssembly.RuntimeError)) {
				if (!this.#glued) throw err;
				this.#leave();
				return { thrown: `${err}` };
			}
			const ptr = this.#exports.message();
			const text = (ptr: number, len: number) => new TextDecoder().decode(new Uint8Array(this.#exports.memory.buffer, ptr, len));
			const outcome: Outcome = ptr === 0 ? "abort" : { panic: text(ptr, this.#exports.message_len()) };
			if (this.#exports.note && this.#exports.note_len) this.note = text(this.#exports.note(), this.#exports.note_len());
			// A trap leaves the instance mid-call: start a fresh one.
			this.#leave();
			return outcome;
		}
	}
}
