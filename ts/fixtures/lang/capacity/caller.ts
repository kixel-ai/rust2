// The inputs `capacity`'s cases name, which run with the generated ones.

import { op } from "./lang.ts";

export { ops, run } from "./lang.ts";

/**
 * `capacity` cases of each element type: each step of a vector, a deque, and a heap, by a
 * few, past what it may hold, and within eight of `usize::MAX`, once it grew past its
 * least capacity, made by `new` and by `with_capacity`; each step after a `reserve` or
 * `reserve_exact` that needed no room; a run of pushes that doubles it from each; a heap
 * that a `reserve` gave room past its length, remade from its `Vec` and cleared, which
 * keeps that room; pushes past pops, a truncate, and a clear of a vector and a heap,
 * which grow only past the most each held; each way `collects` makes one, of each length
 * it tries; each splice of a vector a run of pushes grew; and each step of a string, of a
 * char and a text of each width, by a few, past what it may hold, and within eight of
 * `usize::MAX`, from each way it is made, once pushes grew it.
 */
const capacityCases = [
	...[["capacity::vecs", 41], ["capacity::deques", 28], ["capacity::heaps", 15]].flatMap(([name, steps]) =>
		Array.from({ length: 5 * 2 * (steps as number) * 4 }, (_, i) => {
			const [kind, first, step, arg] = [i % 5, (Math.floor(i / 5) % 2) * 0x11, Math.floor(i / 10) % (steps as number), [3, 0xc1, 0xe3, 0x16][Math.floor(i / (10 * (steps as number)))]];
			return Uint8Array.of(...op(name as string), kind, first, ...[0, 1, 0, 2, 0, 3, 0, 4, 0, 5].map((b, j) => (j % 2 === 0 ? b : b * 41)), step, arg, step, arg ^ 1);
		}),
	),
	...[["capacity::vecs", 41], ["capacity::deques", 28], ["capacity::heaps", 15]].flatMap(([name, steps]) =>
		Array.from({ length: 5 * 2 * (steps as number) }, (_, i) => Uint8Array.of(...op(name as string), i % 5, 0, 0, 1, 3 + (Math.floor(i / 5) % 2), 3, Math.floor(i / 10), 0)),
	),
	...["capacity::vecs", "capacity::deques", "capacity::heaps"].flatMap((name) =>
		Array.from({ length: 5 * 2 }, (_, i) => Uint8Array.of(...op(name), i % 5, Math.floor(i / 5) * 0x11, ...Array.from({ length: 40 }, (_, p) => [0, p]).flat())),
	),
	...Array.from({ length: 5 }, (_, kind) => Uint8Array.of(...op("capacity::heaps"), kind, 0, 8, 15, 3, 0, 12, 0, 7, 0)),
	...["capacity::vecs", "capacity::heaps"].flatMap((name) =>
		Array.from({ length: 5 * 2 }, (_, i) => Uint8Array.of(...op(name), i % 5, Math.floor(i / 5) * 0x11, ...[0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0].flatMap((o) => [o, 1]))),
	),
	...Array.from({ length: 5 * 2 * 256 }, (_, i) => Uint8Array.of(...op("capacity::vecs"), i % 5, (Math.floor(i / 5) % 2) * 0x11, 0, 1, 0, 2, 0, 3, 0, 4, 0, 5, 35, Math.floor(i / 10))),
	...Array.from({ length: 8 * 6 }, (_, i) => Uint8Array.of(...op("capacity::collects"), i % 8, [0, 1, 2, 5, 9, 23][Math.floor(i / 8)], 1, 2, 3, 4, 5, 6, 7, 8, 9, 10)),
	...Array.from({ length: 54 * 4 * 4 }, (_, i) => {
		const [step, first, arg] = [i % 54, [0, 0x29, 0x0e, 0x0f][Math.floor(i / 54) % 4], [1, 6, 0xc3, 0xe2][Math.floor(i / 216)]];
		return Uint8Array.of(...op("capacity::strings"), first, 1, 3, 0, 2, 1, 1, step, arg, step, arg ^ 3, 0, 1);
	}),
];

export const cases: Uint8Array[] = [...capacityCases];
