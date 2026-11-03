// The inputs `differential.ts` runs a harness on. A harness that names its cases draws
// each case's from a stream seeded by its name, so adding, removing, or reordering a case
// changes no other case's inputs.

/** What a harness, or a hand-written caller wrapping one, exports to pick its inputs. */
export interface Harness {
	/** Each case's name and the bytes that pick it, if it has cases. */
	ops?: Map<string, Uint8Array>;
	/** Whether its cases read full-width values, which each u64 boundary then runs through. */
	wide?: boolean;
	/** Case `name`'s input from the bytes generated for it, if not its op then those bytes. */
	input?(name: string, tail: Uint8Array): Uint8Array;
	/** Inputs it names, which run with the generated ones. */
	cases?: Uint8Array[];
}

const MASK = 0xffffffffffffffffn;
const SEED = 0x9e3779b97f4a7c15n;

/** xorshift64*: the same bytes on every run from the same seed. */
class Stream {
	#state: bigint;

	constructor(seed: bigint) {
		this.#state = seed;
	}

	next(): bigint {
		this.#state ^= this.#state >> 12n;
		this.#state ^= (this.#state << 25n) & MASK;
		this.#state ^= this.#state >> 27n;
		return (this.#state * 0x2545f4914f6cdd1dn) & MASK;
	}

	byte(): number {
		return Number(this.next() & 0xffn);
	}

	/** A byte biased toward the values codecs care about: zero, all ones, and boundaries. */
	interesting(): number {
		switch (Number(this.next() % 8n)) {
			case 0:
				return 0;
			case 1:
				return 0xff;
			case 2:
				return [0x3f, 0x40, 0x7f, 0x80, 0xbf, 0xc0, 0xdf, 0xe0, 0xef, 0xf0, 0xf7, 0xf8, 0xfb, 0xfc, 0xfd, 0xfe][Number(this.next() % 16n)];
			default:
				return this.byte();
		}
	}

	/** Up to 47 interesting bytes. */
	tail(): Uint8Array {
		return Uint8Array.from({ length: Number(this.next() % 48n) }, () => this.interesting());
	}
}

/** A stream of `name`'s own: FNV-1a of its UTF-8, odd so that xorshift never sticks at zero. */
function seeded(name: string): Stream {
	let hash = 0xcbf29ce484222325n;
	for (const b of new TextEncoder().encode(name)) hash = ((hash ^ BigInt(b)) * 0x100000001b3n) & MASK;
	return new Stream(hash | 1n);
}

/** A selector byte, then three of each u64 boundary 2^k - 1, 2^k, 2^k + 1, big-endian. */
function boundaries(): Uint8Array[] {
	const tails: Uint8Array[] = [];
	for (let k = 0n; k <= 64n; k++) {
		for (const delta of [-1n, 0n, 1n]) {
			const bytes = new Uint8Array(8);
			new DataView(bytes.buffer).setBigUint64(0, ((1n << k) + delta) & MASK);
			for (const selector of [0, 1, 2, 3, 4, 5, 6]) tails.push(Uint8Array.of(selector, ...bytes, ...bytes, ...bytes));
		}
	}
	return tails;
}

/**
 * Each case's generated inputs, by name: its op alone and with every short tail, so each
 * path's edges are covered exhaustively, each boundary if its cases are `wide`, and
 * `count` random tails from its own stream.
 */
export function byCase(harness: Harness, count: number): Map<string, Uint8Array[]> {
	const ops = harness.ops;
	if (!ops) throw new Error("the harness names no cases");
	if (new Set([...ops.values()].map((op) => Buffer.from(op).toString("hex"))).size !== ops.size) {
		throw new Error("two ops are the same bytes, so one of them never runs");
	}
	const wide = harness.wide ? boundaries() : [];
	const inputs = new Map<string, Uint8Array[]>();
	for (const [name, op] of ops) {
		const tails: Uint8Array[] = [new Uint8Array(0)];
		for (let b = 0; b < 256; b += 17) tails.push(Uint8Array.of(b));
		tails.push(...wide);
		const stream = seeded(name);
		for (let i = 0; i < count; i++) tails.push(stream.tail());
		inputs.set(name, tails.map((tail) => harness.input?.(name, tail) ?? Uint8Array.of(...op, ...tail)));
	}
	return inputs;
}

/**
 * Every input of a harness: of each case, by `byCase`, or of a harness without cases,
 * every first byte with every short tail, each boundary through the first eight, and
 * `count` random inputs; then the inputs it names.
 */
export function inputs(harness: Harness, count: number): Uint8Array[] {
	let inputs: Uint8Array[] = [new Uint8Array(0)];
	if (harness.ops) {
		inputs = inputs.concat(...byCase(harness, count).values());
	} else {
		for (let op = 0; op < 256; op++) {
			inputs.push(Uint8Array.of(op));
			for (let b = 0; b < 256; b += 17) inputs.push(Uint8Array.of(op, b));
		}
		const wide = boundaries();
		for (let op = 0; op < 8; op++) for (const tail of wide) inputs.push(Uint8Array.of(op, ...tail));
		const stream = new Stream(SEED);
		for (let i = 0; i < count; i++) {
			const len = Number(stream.next() % 48n);
			inputs.push(Uint8Array.of(stream.byte(), ...Array.from({ length: len }, () => stream.interesting())));
		}
	}
	return inputs.concat(harness.cases ?? []);
}
