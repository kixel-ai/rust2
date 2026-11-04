// A hand-written caller that mixes integer types, which `tsc` must refuse: each line after
// `@ts-expect-error` fails the type check if it is accepted. A plain number, such as a
// literal, passes for any integer type, and a cast picks one.

import * as rs from "./rs.ts";
import * as Meters from "./shapes.Meters.ts";
import * as Point from "./shapes.Point.ts";
import { returnedNever, tailBreak } from "./shapes.ts";

export function mixed(data: rs.ByteSlice): void {
	returnedNever(7, 1);
	const meters: Meters.Meters = { _0: tailBreak(data) as rs.u64 };
	// @ts-expect-error A `u16` is not a `u8`.
	returnedNever(tailBreak(data), 1);
	// @ts-expect-error A `u32` is not a `usize`.
	returnedNever(1, 0xffffffff as rs.u32);
	// @ts-expect-error A `u64` is not a `u8`, even as a number.
	returnedNever(Meters.area(meters), 1);
	// @ts-expect-error An `i64` is not a `u64`.
	Meters.area({ _0: rs.i64Of(9223372036854775807n) });
	// @ts-expect-error A `u32` is not an `i32`.
	Point.new(0xffffffff as rs.u32, 0);
}
