// The inputs `shapes`' cases name, which run with the generated ones, and a `Fault`
// printed with the `Display` and `Debug` that `printable` brings along, which nothing
// translated calls: native Rust's `entry` prints one for a first byte of 0xff.

import * as Fault from "./fmt.Fault.ts";
import { be, edges, op, ops as picks, run as harness } from "./lang.ts";
import * as rs from "./rs.ts";

/** The bytes that pick each case, and a `Fault`'s first byte. */
export const ops = new Map<string, Uint8Array>([...picks, ["fmt::Fault", Uint8Array.of(0xff)]]);

export function run(input: rs.ByteSlice): rs.ByteVec {
	if (rs.byteSliceFirst(input) !== 0xff) return harness(input);
	const fault = Fault.new(rs.byteSliceRange(input, 1));
	const display: rs.Fmt = (f) => Fault.fmt$fmt_Display(fault, f);
	const debug: rs.Fmt = (f) => Fault.fmt$fmt_Debug(fault, f);
	const text = rs.fmtFormat([
		display,
		"|",
		rs.fmtSpec({ align: ">", width: 16 }, display),
		"|",
		debug,
		"|",
		rs.fmtSpec({ alternate: true }, debug),
	]);
	return rs.byteVecFrom(new TextEncoder().encode(text.v));
}

/** `shapes::loops` with each first byte that one of its cases of `!` takes, with one more byte and without. */
const loops = [0xf1, 0xf3, 0xf4, 0xf5, 0xf6, 0xf9, 0xfa].flatMap((b) => [Uint8Array.of(...op("shapes::loops"), 0, b), Uint8Array.of(...op("shapes::loops"), 0, b, 1)]);

/** `fmt::ints` of the 64-bit operands random bytes rarely make, each with every format spec, as the high half of a `u128` too. */
const fmtInts = edges().flatMap((x) =>
	[0n, 1n, 1n << 32n, 1n << 63n, 0xffffffffffffffffn].flatMap((y) => [3, 4, 9, 10].map((from) => Uint8Array.of(...op("fmt::ints"), from, ...be(x), ...be(y)))),
);

export const cases: Uint8Array[] = [...loops, ...fmtInts];
