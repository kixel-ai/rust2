// `std::panic`'s hook, for rust2-ts output: only a program that sets one reaches this
// module, so a bundler drops all of it from one that doesn't. `rs.ts` re-exports all of it.

import { type String as RsString, stringNew, stringPushStr } from "./str.ts";
import { Abort, type Arguments, type FmtResult, type Formatter, fmtWriteFmt, fmtWriteString, Panic, panic } from "../rs.ts";
import { fmtLocationDisplay, type Location } from "./location.ts";
import { drop } from "./task.ts";

/** `std::panic::PanicHookInfo`: what a panic hook is given. */
export interface PanicHookInfo {
	/** `payload_as_str`: the payload, if it is a `&str` or a `String`. */
	readonly text: string | undefined;
	/** The payload, if it is a `String`, as a message `panic!` formats is, rather than a `&'static str`. */
	readonly string: RsString | undefined;
	/** Any other payload, as the `dyn Any` object `panic_any` made of it. */
	readonly payload: object | undefined;
	/** Where it panicked. */
	readonly location: Location;
}

/** A panic hook, as `set_hook` takes it. */
export type Hook = (info: PanicHookInfo) => undefined;

/** Where std's and `bytes`' code panics, for each of their panics the runtime makes, as the native build names it. */
export interface Sites {
	readonly [site: string]: Location;
}

/** The default hook, which writes to the standard error wasm32 has none of. */
function defaultHook(_info: PanicHookInfo): undefined {}

let hook: Hook = defaultHook;
/** Whether a panic is formatting its message or running the hook: a panic then aborts, and `thread::panicking` is true. */
let hooking = false;
/** Where std's and `bytes`' own code panics outside a `#[track_caller]` function, which `setHook` is given. */
let sites: Sites | undefined;

/**
 * Run the hook on a panic at `at`, or at the key of std's or `bytes`' site in `Sites`, whose payload is
 * `message` or else `payload`: the `String` `owned`, or one written from empty as std's
 * `panic!` writes its message: `message` in one write if `owned` is true, or `Arguments`
 * with placeholders, as `Arguments::as_str` says, by `write_fmt` once the panic has begun.
 * It gives the text. A panic while the hook runs aborts, as std's does,
 * without running it again.
 */
function run(message: string | Arguments | undefined, at: Location | string | undefined, owned?: boolean | RsString, payload?: object): string | undefined {
	const literal = typeof message === "object" ? asStr(message) : message;
	if (hooking) throw new Abort(`${literal ?? ""}\nthread panicked while processing panic. aborting.`);
	if (hook === defaultHook) return undefined;
	const location = typeof at === "string" ? sites?.[at] : at;
	if (location === undefined) throw new Error(`rust2-ts: a panic its hook can't locate: ${literal ?? "a formatted message"}`);
	hooking = true;
	try {
		let string = typeof owned === "object" ? owned : undefined;
		if (typeof message === "object" && literal === undefined) {
			string = stringNew();
			fmtWriteString(string, message);
		} else if (owned === true) {
			string = stringNew();
			stringPushStr(string, literal ?? "");
		}
		const text = string?.v ?? literal;
		hook({ text, string, payload, location });
		return text;
	} finally {
		hooking = false;
	}
}

/** `Arguments::as_str`: the text of a message without placeholders. */
function asStr(args: Arguments): string | undefined {
	const [part = "", ...rest] = args;
	return rest.length === 0 && typeof part === "string" ? part : undefined;
}

/** `panic::set_hook`, given where std's and `bytes`' own code panics, which its hooks read. */
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

/** `thread::panicking`, which is true while a panic formats its message and runs the hook, as a panic aborts once it has. */
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
export function hookPayload<S extends object, O extends object>(info: PanicHookInfo, str: new (text: string) => S, string: new (text: RsString) => O): S | O {
	// `panic_any` made its payload an object of the same `dyn Any` the classes are.
	if (info.payload !== undefined) return info.payload as S | O;
	return info.string !== undefined ? new string(info.string) : new str(info.text ?? "");
}

/** `PanicHookInfo`'s `Display`: where it panicked, then any text, as the default hook's first lines. */
export function fmtPanicHookInfoDisplay(info: PanicHookInfo, f: Formatter): FmtResult {
	const r = fmtWriteFmt(f, ["panicked at ", (f) => fmtLocationDisplay(info.location, f)]);
	if (!r.ok || info.text === undefined) return r;
	return fmtWriteFmt(f, [":\n", info.text]);
}
