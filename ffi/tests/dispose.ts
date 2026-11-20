// An engine without `Symbol.dispose`, as stable Safari is: the runtime, bundled and run in a
// context whose `Symbol` lacks it, defines it as the registry's `Symbol.for("Symbol.dispose")`,
// which esbuild's `using` helper falls back to, before any class keys a method on it.
//
// usage: node dispose.ts <runtime-entry>

import { execFileSync } from "node:child_process";
import vm from "node:vm";

const entry = process.argv[2];
if (!entry) throw new Error("usage: dispose.ts <runtime-entry>");
const code = execFileSync("esbuild", [entry, "--bundle", "--format=iife", "--global-name=runtime", "--target=es2022"], {
	encoding: "utf8",
});

// The built-in `Symbol`, but for `dispose` and `asyncDispose`, which cannot be deleted from it.
const FakeSymbol = Object.defineProperties(
	(description?: string) => Symbol(description),
	Object.fromEntries(Reflect.ownKeys(Symbol).filter((key) => key !== "dispose" && key !== "asyncDispose").map((key) => [key, Object.getOwnPropertyDescriptor(Symbol, key)!])),
);

// What a browser gives the runtime that a bare context lacks.
const { TextEncoder, TextDecoder, URL, console, queueMicrotask, setTimeout, clearTimeout, performance } = globalThis;
const context = vm.createContext({ Symbol: FakeSymbol, TextEncoder, TextDecoder, URL, console, queueMicrotask, setTimeout, clearTimeout, performance });
if (vm.runInContext("typeof Symbol.dispose", context) !== "undefined") throw new Error("the context's Symbol still has dispose");
vm.runInContext(`${code}\nglobalThis.runtime = runtime;`, context);

const registry = Symbol.for("Symbol.dispose");
const defined = vm.runInContext("Symbol.dispose", context);
if (defined !== registry) throw new Error(`Symbol.dispose is ${String(defined)}, not the registry's`);

// Each class that disposes does so under that symbol, and none under the string "undefined".
let disposing = 0;
for (const [name, value] of Object.entries(vm.runInContext("runtime", context) as Record<string, unknown>)) {
	if (typeof value !== "function" || !value.prototype) continue;
	if (Object.hasOwn(value.prototype, "undefined")) throw new Error(`${name} keys a method on "undefined"`);
	if (Object.getOwnPropertySymbols(value.prototype).includes(registry)) disposing++;
}
if (disposing === 0) throw new Error("no exported class disposes under the registry's symbol");
