// `std::panic`'s hook, for rust2-ts output: only a program that sets one reaches this
// module, so a bundler drops all of it from one that doesn't. `rt.ts` re-exports all of it.

import { Abort, type Arguments, type FmtResult, type Formatter, fmtWriteFmt, Panic, panic } from "../rt.ts";
import { fmtLocationDisplay, type Location } from "./location.ts";
import { drop } from "./task.ts";

/** `std::panic::PanicHookInfo`: what a panic hook is given. */
export interface PanicHookInfo {
	/** `payload_as_str`: the payload, if it is a `&str` or a `String`. */
	readonly text: string | undefined;
	/** Whether that text is a `String`, as a message `panic!` formats is, rather than a `&'static str`. */
	readonly owned: boolean;
	/** Any other payload, as the `dyn Any` object `panic_any` made of it. */
	readonly payload: object | undefined;
	/** Where it panicked. */
	readonly location: Location;
}

/** A panic hook, as `set_hook` takes it. */
export type Hook = (info: PanicHookInfo) => undefined;

/** Where std's code panics, for each of its panics the runtime makes, as the native build names it. */
export interface Sites {
	readonly [site: string]: Location;
}

/** The default hook, which writes to the standard error wasm32 has none of. */
function defaultHook(_info: PanicHookInfo): undefined {}

let hook: Hook = defaultHook;
/** Whether a hook is running: a panic then aborts, and `thread::panicking` is true. */
let hooking = false;
/** Where std's own code panics outside a `#[track_caller]` function, which `setHook` is given. */
let sites: Sites | undefined;

/**
 * Run the hook on a panic at `at`, or at the key of std's site in `Sites`, whose payload is
 * `text` or else `payload`: a `String` if `owned`, or if it is the `Arguments` the text was
 * formatted from and they have placeholders, as `Arguments::as_str` says. A panic while
 * the hook runs aborts, as std's does, without running it again.
 */
function run(text: string | undefined, at: Location | string | undefined, owned: boolean | Arguments | undefined, payload?: object): void {
	if (hooking) throw new Abort(`${text ?? "Box<dyn Any>"}\nthread panicked while processing panic. aborting.`);
	if (hook === defaultHook) return;
	const location = typeof at === "string" ? sites?.[at] : at;
	if (location === undefined) throw new Error(`rust2-ts: a panic its hook can't locate: ${text ?? "Box<dyn Any>"}`);
	if (typeof owned === "object") owned = owned.length > 1 || owned.some((part) => typeof part !== "string");
	hooking = true;
	try {
		hook({ text, owned: owned ?? false, payload, location });
	} finally {
		hooking = false;
	}
}

/** `panic::set_hook`, given where std's own code panics, which its hooks read. */
export function setHook(f: Hook, table: Sites): void {
	// Within a hook, which a panic there aborts.
	if (hooking) panic("cannot modify the panic hook from a panicking thread");
	Panic.hooked = run;
	const old = hook;
	hook = f;
	sites = table;
	drop(old);
}

/** `panic::take_hook`: the hook, which the default replaces. */
export function takeHook(): Hook {
	if (hooking) panic("cannot modify the panic hook from a panicking thread");
	const old = hook;
	hook = defaultHook;
	return old;
}

/** `thread::panicking`, which is true while a hook runs, as a panic aborts once it has. */
export function panicking(): boolean {
	return hooking;
}

/** `panic_any` of a payload that is no text: the `dyn Any` object made of it. */
export function panicAny(payload: object, at?: Location): never {
	run(undefined, at, false, payload);
	throw new Panic("Box<dyn Any>");
}

/**
 * `PanicHookInfo::payload`: what `panic_any` was given, or else the text, as a `str` or a
 * `String` object, whose classes translation makes.
 */
export function hookPayload(info: PanicHookInfo, str: new (text: string) => object, string: new (text: string) => object): object {
	return info.payload ?? new (info.owned ? string : str)(info.text ?? "");
}

/** `PanicHookInfo`'s `Display`: where it panicked, then any text, as the default hook's first lines. */
export function fmtPanicHookInfoDisplay(info: PanicHookInfo, f: Formatter): FmtResult {
	const r = fmtWriteFmt(f, ["panicked at ", (f) => fmtLocationDisplay(info.location, f)]);
	if (!r.ok || info.text === undefined) return r;
	return fmtWriteFmt(f, [":\n", info.text]);
}
