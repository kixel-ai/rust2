// The harness, with inputs it names for `differential.ts`: `buf::reserves`' edges of
// `rt.BytesMut`'s limit, which only these reach, as each allocates 512 MiB, and each of
// `hashlink`'s iterators read through every pick of steps from either end.

import { op, run } from "./harness.ts";

export { run };

const reserves = op("buf::reserves");

/** Each edge but shared storage past the limit, which throws, read past by none and by some. */
const edges = [0, 1, 2].flatMap((kind) => [0, 3].map((read) => Uint8Array.of(reserves, ...new TextEncoder().encode("edge"), kind, read)));

/** `maps::linked_iters` of maps empty, of one, and fuller, each iterator and adaptor by each pick. */
const linked = [0x00, 0x01, 0x02, 0x15, 0x0b, 0x3b].flatMap((len) =>
	[[], [0x00, 0x01, 0x00, 0x01, 0x00, 0x01], [0x07, 0x02, 0x05, 0x0b, 0x03, 0x00]].flatMap((steps) => Array.from({ length: 256 }, (_, pick) => Uint8Array.of(op("maps::linked_iters"), len, pick, ...steps))),
);

export const cases: Uint8Array[] = [...edges, ...linked];
