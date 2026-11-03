// The harness, with inputs it names for `differential.ts`: `buf::reserves`' edges of
// `rt.BytesMut`'s limit, which only these reach, as each allocates 512 MiB, each of
// `hashlink`'s iterators read through every pick of steps from either end, each
// `buf::chains` chain copied by each pick of a length, split at either end and between,
// and every pair of steps over `hashlink`'s maps of what drops.

import { op, run } from "./harness.ts";

export { run };

const reserves = op("buf::reserves");

/** Each edge but shared storage past the limit, which throws, read past by none and by some. */
const edges = [0, 1, 2].flatMap((kind) => [0, 3].map((read) => Uint8Array.of(reserves, ...new TextEncoder().encode("edge"), kind, read)));

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

export const cases: Uint8Array[] = [...edges, ...linked, ...chains, ...nested];
