// `core::panic::Location`, for rust2-ts output: where a `#[track_caller]` function was
// called, which never changes once made, so values share it. `rs.ts` re-exports all of it.

import { type FmtResult, type Formatter, fmtDebugStructFields, fmtPad, fmtStr, fmtWriteFmt } from "../rs.ts";
import { cmp, cmpStr, type Ordering } from "./collections.ts";
import { type Hasher, hash32, hashStr } from "./hash.ts";
import { type u32, u32Debug, u32Display } from "./int.ts";

/** `core::panic::Location`: the file as rustc names it, and the line and column from 1. */
export interface Location {
	readonly file: string;
	readonly line: u32;
	readonly col: u32;
}

/** `PartialEq::eq`: the same column, line, and file. */
export function locationEq(a: Location, b: Location): boolean {
	return a.col === b.col && a.line === b.line && a.file === b.file;
}

/** `Ord::cmp`: by file, then line, then column. */
export function locationCmp(a: Location, b: Location): Ordering {
	return cmpStr(a.file, b.file) || cmp(a.line, b.line) || cmp(a.col, b.col);
}

/** `Hash::hash`: its file, line, and column, as std's impl writes them. */
export function locationHash(l: Location, s: Hasher): void {
	hashStr(l.file, s);
	hash32(l.line, s);
	hash32(l.col, s);
}

/** `Display`: `file:line:column`, a part at a time, which no width pads, as std's `write!` writes it. */
export function fmtLocationDisplay(l: Location, f: Formatter): FmtResult {
	return fmtWriteFmt(f, [(f) => fmtPad(f, l.file), ":", (f) => u32Display(l.line, f), ":", (f) => u32Display(l.col, f)]);
}

/** `Debug`, as std's: a struct of the file, line, and column. */
export function fmtLocationDebug(l: Location, f: Formatter): FmtResult {
	return fmtDebugStructFields(
		f,
		"Location",
		["file", "line", "column"],
		[(f) => fmtStr(l.file, f), (f) => u32Debug(l.line, f), (f) => u32Debug(l.col, f)],
	);
}
