// What every route that compiles a `.wasm` compiles, in the bench's workers and in the
// rust2-wasm route's bundle alike.

/**
 * `module` with a custom section of its own, so no engine serves code it compiled from the
 * same bytes in another worker, as V8 shares a module's code across a process.
 */
export function fresh(module: Uint8Array): Uint8Array<ArrayBuffer> {
	const nonce = crypto.getRandomValues(new Uint8Array(8));
	// A custom section: id 0, its size, then its name's length and name, then its bytes.
	const name = new TextEncoder().encode("fresh");
	const section = Uint8Array.of(0, 1 + name.length + nonce.length, name.length, ...name, ...nonce);
	const out = new Uint8Array(module.length + section.length);
	out.set(module);
	out.set(section, module.length);
	return out;
}
