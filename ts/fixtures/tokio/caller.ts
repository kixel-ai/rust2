// The harness, with its cases by name and scripts for `differential.ts` that random steps
// rarely reach: a receiver awaited after it completed, which panics, a decoder that reads
// its close as an error where none was sent, which panics, and channels filled, closed,
// and dropped with tasks waiting on each side.

import { cases as named, op, run } from "./harness.ts";

export { run };

/** Each case's name and the byte that picks it. */
export const ops = new Map(named().items.map(([name]): [string, Uint8Array] => [name, Uint8Array.of(op(name))]));

/** Each case's script, as steps of an op and its argument. */
const script = (name: string, ...steps: number[][]): Uint8Array => Uint8Array.of(op(name), ...steps.flat());

export const cases: Uint8Array[] = [
	// A receiver awaited to its value, then awaited again.
	script("oneshot::script", [0, 0], [8, 0], [2, 7], [8, 0], [9, 0], [10, 0x5a]),
	// A receiver awaited to the sender's drop, then awaited again.
	script("oneshot::script", [0, 0], [1, 0], [8, 0], [5, 0], [8, 0], [10, 0x5a]),
	// The sender waiting for the receiver to close, which it does, then sending.
	script("oneshot::script", [1, 1], [8, 0], [4, 0], [9, 0], [8, 0], [2, 3], [3, 0]),
	// A close the decoder reads as an error, though it holds none.
	script("select::decoded", [5, 0], [0, 1], [5, 0], [1, 0x5a], [5, 0]),
	// Frames, then the close, then frames the decoder never reads.
	script("select::decoded", [0, 1], [0, 2], [1, 9], [5, 0], [0, 3], [5, 0]),
	// Frames until the sender drops, with the close's sender gone first.
	script("select::decoded", [3, 0], [5, 0], [0, 1], [5, 0], [2, 0], [5, 0]),
	// Values on both, then both done.
	script("select::unbiased", [0, 1], [1, 2], [0, 3], [2, 0], [3, 0]),
	// A full channel of one slot, a sender waiting on it, and a receiver that drops what it holds.
	script("mpsc::bounded", [0], [3, 0], [3, 0], [3, 0], [17, 0], [6, 0], [17, 0], [15, 0], [17, 0], [19, 0]),
	// Senders waiting for room, the channel closed under them, and the rest received.
	script("mpsc::bounded", [1], [3, 0], [3, 0], [4, 0], [5, 0x13], [17, 0], [12, 0], [17, 0], [0, 0], [17, 0], [0, 0], [17, 0], [19, 0]),
	// Each sender dropped while the receiver waits, then a weak one upgraded.
	script("mpsc::unbounded", [18, 0], [0, 0], [17, 0], [13, 0], [17, 0], [18, 1], [19, 0]),
	// The channel of no room, of more than the most, and of the most.
	script("mpsc::zero", [0]),
	script("mpsc::zero", [1]),
	script("mpsc::zero", [2]),
];
