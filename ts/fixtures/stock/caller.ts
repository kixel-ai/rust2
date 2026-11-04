// The harness, with its cases by name and the inputs it names for `differential.ts`:
// `buf::far`'s reads past where a vector shares its storage, which only these reach, as
// each allocates 128 MiB, `buf::reclaims` after each pick of the handles to drop, each of `hashlink`'s iterators read through every pick of steps from
// either end, each `buf::chains` chain copied by each pick of a length, split at either
// end and between, every pair of steps over `hashlink`'s maps of what drops, and each
// `Bytes` of a `String` by each room and way taken back.

import { cases as named, op, run } from "./harness.ts";

export { run };

/** Each case's name and the byte that picks it. */
export const ops = new Map(named().items.map(([name]): [string, Uint8Array] => [name, Uint8Array.of(op(name))]));

/** Its cases read full-width values. */
export const wide = true;

/** Read past just that far and one more, then each op, frozen and taken back first or not. */
const far = [0, 1].flatMap((past) => Array.from({ length: 8 }, (_, step) => Uint8Array.of(op("buf::far"), ...new TextEncoder().encode("far"), past, step)));

/** `maps::linked_iters` of maps empty, of one, and fuller, each iterator and adaptor by each pick. */
const linked = [0x00, 0x01, 0x02, 0x15, 0x0b, 0x3b].flatMap((len) =>
	[[], [0x00, 0x01, 0x00, 0x01, 0x00, 0x01], [0x07, 0x02, 0x05, 0x0b, 0x03, 0x00]].flatMap((steps) => Array.from({ length: 256 }, (_, pick) => Uint8Array.of(op("maps::linked_iters"), len, pick, ...steps))),
);

/** Each chain by each pick, of nothing, and of three bytes cut at each place, read past one and then read by two ops. */
const chains = Array.from({ length: 9 * 8 }, (_, i) => [i % 9, Math.floor(i / 9)]).flatMap(([kind, pick]) => [
	Uint8Array.of(op("buf::chains"), kind, 0, pick),
	...[0, 1, 2, 3].map((cut) => Uint8Array.of(op("buf::chains"), kind, cut, pick, 0xff, 0x00, 0x02, 10, 11, 12)),
]);

/** `maps::nested` at each capacity, after a step of each map, by every pair of steps of keys held and not. */
const nested = [0, 1, 2, 3, 4].flatMap((cap) =>
	Array.from({ length: 81 * 2 }, (_, i) => Uint8Array.of(op("maps::nested"), cap, 0x00, 0x01, 0x02, 0x01, 0x03, 0x02, 0x01, 0x02, i % 9, i >= 81 ? 0x05 : 0x01, Math.floor(i / 9) % 9, 0x02)),
);

/** Of no capacity, of some, and past `bytes`' smallest original capacity, each pick dropped, reserving nothing, a little, and much, read past or not. */
const reclaims = [0, 3, 200].flatMap((cap) =>
	Array.from({ length: 8 * 4 * 2 }, (_, i) => Uint8Array.of(op("buf::reclaims"), cap, i % 8, [0, 3, 40, 255][Math.floor(i / 8) % 4], i < 32 ? 0 : 5)),
);

/** `buf::strings` of no room, of exactly its text, and of more, each way taken back, with a clone held or not. */
const strings = [0, 4, 23].flatMap((cap) =>
	Array.from({ length: 2 * 2 * 3 }, (_, i) => Uint8Array.of(op("buf::strings"), cap, 4, 0x61, 0xc3, 0xa9, 0x62, i % 2 === 0 ? 0 : 6, Math.floor(i / 2) % 2, Math.floor(i / 4))),
);

export const cases: Uint8Array[] = [...far, ...reclaims, ...linked, ...chains, ...nested, ...strings];
