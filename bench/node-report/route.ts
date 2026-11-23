// What node.ts calls: the calls of bench/ffi's cases in the web binding's shape, a module
// of functions taking a handle first, which napi.ts and nf.ts each give over the C ABI.

export type Result<T> = { ok: true; value: T } | { ok: false; error: unknown };

export interface Handle {
	[Symbol.dispose](): void;
}

/** Bytes Rust returned, read in place past 64 bytes, as the web binding's `rt.Bytes`. */
export interface Bytes extends Handle {
	readonly length: number;
	at(i: number): number | undefined;
}

export interface Route {
	broadcast(): Handle;
	createTrack(broadcast: Handle, name: string): Result<Handle>;
	isClosed(track: Handle): boolean;
	largest(track: Handle): Uint8Array | undefined;
	info(track: Handle): Handle;
	priority(info: Handle): number;
	demand(group: Handle): Handle;
	demandStart(demand: Handle): number | bigint;
	fromMicros(micros: number): Result<Uint8Array>;
	appendGroup(track: Handle, at: Uint8Array): Result<Handle>;
	writeFrame(group: Handle, at: Uint8Array, data: Uint8Array): Result<undefined>;
	finish(group: Handle): Result<Handle>;
	consume(track: Handle): Handle;
	subscribe(consumer: Handle): Promise<Result<Handle>>;
	recvGroup(subscriber: Handle): Promise<Result<Handle | undefined>>;
	readFrame(reader: Handle): Promise<Result<Handle | undefined>>;
	payload(frame: Handle): Bytes;
}

export const OK: Result<undefined> = { ok: true, value: undefined };

/** A handle whose disposal frees it once, and whose use after throws, as `rt.Handle`. */
export class Owned<P> implements Handle {
	#p: P | undefined;
	readonly #free: (p: P) => void;
	readonly #name: string;

	constructor(p: P, free: (p: P) => void, name: string) {
		this.#p = p;
		this.#free = free;
		this.#name = name;
	}

	get p(): P {
		const p = this.#p;
		if (p === undefined) throw new Error(`${this.#name} is closed`);
		return p;
	}

	[Symbol.dispose](): void {
		const p = this.#p;
		if (p === undefined) return;
		this.#p = undefined;
		this.#free(p);
	}
}

/** Bytes over a view that only this object reaches, so none outlives the buffer it views. */
export class Lent implements Bytes {
	#view: Uint8Array | undefined;
	readonly #free: (() => void) | undefined;

	constructor(view: Uint8Array, free: (() => void) | undefined) {
		this.#view = view;
		this.#free = free;
	}

	get length(): number {
		return this.#open().length;
	}

	at(i: number): number | undefined {
		return this.#open()[i];
	}

	#open(): Uint8Array {
		const view = this.#view;
		if (view === undefined) throw new Error("Bytes is closed");
		return view;
	}

	[Symbol.dispose](): void {
		if (this.#view === undefined) return;
		this.#view = undefined;
		this.#free?.();
	}
}

/** The polls a wake resumes, by the id its waker carries. */
const pending = new Map<number, () => void>();
let nextId = 1;

/** A waker's wake, on the loop's thread: polls again on a microtask, never inside the wake. */
export function wake(id: number): void {
	const poll = pending.get(id);
	if (poll !== undefined) queueMicrotask(poll);
}

/**
 * A future pending at its first poll, as a promise: `poll(id)` polls it with a waker for
 * `id`, returning undefined while it pends.
 */
export function pend<T>(poll: (id: number) => T | undefined): Promise<T> {
	const id = nextId++;
	return new Promise<T>((resolve, reject) => {
		const again = () => {
			try {
				const out = poll(id);
				if (out === undefined) return;
				pending.delete(id);
				resolve(out);
			} catch (e) {
				pending.delete(id);
				reject(e);
			}
		};
		pending.set(id, again);
		again();
	});
}
