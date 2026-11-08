// tokio's sync channels, for rust2-ts output: `mpsc`, bounded and unbounded, `watch`, and
// `oneshot`, and what `select!` calls. Each keeps its waiters in queues it wakes as tokio's
// intrusive lists do, in the same order, and drops each value where tokio drops it. Nothing
// blocks, as no JS thread can. `rs.ts` re-exports all of it.

import { Err, type FmtResult, type Formatter, debugStructField, debugStructFinish, fmtBool, fmtDebugStruct, fmtDebugTupleFields, type None, OK, Ok, panic, type Place, place, type Result, type Debug, type Vec } from "../rs.ts";
import { usizeDebug, type usize } from "./int.ts";
import type { Location } from "./location.ts";
import { DONE, type Done, Iter } from "./iter.ts";
import { randBelow } from "./rand.ts";
import { asyncFnResumed, type Context, type Future, Pending, type Poll, RefCell, refMutRelease, refRelease, rwLockDebug, rwLockRead, rwLockWrite, type Waker, willWake } from "./task.ts";

/** What drops a value a channel holds, if it drops at all. */
type Drop<T> = ((value: T) => void) | undefined;

// Waiters

/** tokio's `AtomicWaker`: the waker its last registration cloned, which a wake takes. */
class WakerSlot {
	#waker: Waker | undefined;

	/** Hold a clone of `cx`'s waker, dropping the one it held after, as tokio's does. */
	register(cx: Context): void {
		const waker = cx.clone();
		const old = this.#waker;
		this.#waker = waker;
		old?.[Symbol.dispose]();
	}

	/** Wake the waker it held, if any, which the wake consumes. */
	wake(): void {
		this.take()?.wake();
	}

	take(): Waker | undefined {
		const waker = this.#waker;
		this.#waker = undefined;
		return waker;
	}
}

/** How many wakers tokio takes from a list before it wakes them, unlocked. */
const WAKE_BATCH = 32;

/** Wake each of `wakers` in order, as tokio's `WakeList` does. */
function wakeAll(wakers: Waker[]): void {
	for (const waker of wakers) waker.wake();
}

/** tokio's `Notify`, of which these channels only call `notify_waiters`. */
class Notify {
	/** How many times `notify_waiters` ran, which a `Notified` made before one sees changed. */
	calls = 0;
	/** The waiting `Notified`s, oldest first. */
	waiters: Notified[] = [];

	notified(): Notified {
		return new Notified(this);
	}

	/** Wake every waiter, oldest first, in batches, as tokio's does. */
	notifyWaiters(): void {
		this.calls++;
		const waiters = this.waiters;
		this.waiters = [];
		for (let i = 0; i < waiters.length; ) {
			const wakers: Waker[] = [];
			for (; i < waiters.length && wakers.length < WAKE_BATCH; i++) {
				const waiter = waiters[i];
				waiter.notified = true;
				const waker = waiter.waker;
				waiter.waker = undefined;
				if (waker !== undefined) wakers.push(waker);
			}
			wakeAll(wakers);
		}
	}
}

/** tokio's `Notified`: ready once `notify_waiters` runs after it was made. */
class Notified implements Disposable {
	readonly #notify: Notify;
	readonly #calls: number;
	#waiting = false;
	#done = false;
	notified = false;
	waker: Waker | undefined;

	constructor(notify: Notify) {
		this.#notify = notify;
		this.#calls = notify.calls;
	}

	poll(cx: Context): Poll<undefined> {
		if (this.#done) return undefined;
		if (!this.#waiting) {
			if (this.#notify.calls !== this.#calls) return this.#finish();
			const old = this.waker;
			this.waker = cx.clone();
			this.#notify.waiters.push(this);
			this.#waiting = true;
			old?.[Symbol.dispose]();
			return Pending;
		}
		if (this.notified) {
			this.#take()?.[Symbol.dispose]();
			return this.#finish();
		}
		if (this.waker === undefined || !willWake(this.waker, cx)) {
			const old = this.waker;
			this.waker = cx.clone();
			old?.[Symbol.dispose]();
		}
		return Pending;
	}

	#finish(): undefined {
		this.#waiting = false;
		this.#done = true;
		return undefined;
	}

	#take(): Waker | undefined {
		const waker = this.waker;
		this.waker = undefined;
		return waker;
	}

	[Symbol.dispose](): void {
		if (this.#waiting && !this.notified) {
			const waiters = this.#notify.waiters;
			waiters.splice(waiters.indexOf(this), 1);
		}
		this.#take()?.[Symbol.dispose]();
	}
}

/** The most permits tokio's semaphore holds: `usize::MAX >> 3`, of wasm32's `usize`, 2^29 - 1, a literal a bundler drops unread. */
const MAX_PERMITS = 536870911;

/** The panic of a semaphore asked for more than `MAX_PERMITS`. */
function checkPermits(n: number, at?: Location): void {
	if (n > MAX_PERMITS) panic(`a semaphore may not have more than MAX_PERMITS permits (${MAX_PERMITS})`, at);
}

/** tokio's batch semaphore, which a bounded channel's senders wait on, first come first served. */
class Semaphore {
	permits: number;
	closed = false;
	/** The waiting `Acquire`s, oldest first. */
	waiters: Acquire[] = [];

	constructor(permits: number) {
		this.permits = permits;
	}

	/** `release`: give `n` permits to the oldest waiters, waking each that has all it needs, and keep the rest. */
	release(n: number): void {
		if (n > 0) this.addPermits(n);
	}

	/** `add_permits_locked`. */
	addPermits(rem: number): void {
		while (rem > 0) {
			const wakers: Waker[] = [];
			let empty = false;
			while (wakers.length < WAKE_BATCH) {
				const waiter = this.waiters[0];
				if (waiter === undefined) {
					empty = true;
					break;
				}
				const assign = Math.min(waiter.remaining, rem);
				waiter.remaining -= assign;
				rem -= assign;
				if (waiter.remaining > 0) break;
				this.waiters.shift();
				const waker = waiter.takeWaker();
				if (waker !== undefined) wakers.push(waker);
			}
			if (rem > 0 && empty) {
				if (this.permits + rem > MAX_PERMITS) panic(`number of added permits (${rem}) would overflow MAX_PERMITS (${MAX_PERMITS})`);
				this.permits += rem;
				rem = 0;
			}
			wakeAll(wakers);
		}
	}

	/** `close`: wake every waiter, which then fails. */
	close(): void {
		this.closed = true;
		while (this.waiters.length > 0) {
			const wakers: Waker[] = [];
			while (wakers.length < WAKE_BATCH && this.waiters.length > 0) {
				const waker = this.waiters.shift()!.takeWaker();
				if (waker !== undefined) wakers.push(waker);
			}
			wakeAll(wakers);
		}
	}

	/** `try_acquire`: `n` permits now, or why not. */
	tryAcquire(n: number): "ok" | "closed" | "none" {
		checkPermits(n);
		if (this.closed) return "closed";
		if (this.permits < n) return "none";
		this.permits -= n;
		return "ok";
	}
}

/** tokio's `Acquire`: a wait for `n` permits, which takes those there are and queues for the rest. */
class Acquire implements Disposable {
	readonly #semaphore: Semaphore;
	readonly #n: number;
	/** Whether it is queued, or holds permits it must give back as it drops. */
	#queued = false;
	/** The permits it still waits for, once queued. */
	remaining: number;
	#waker: Waker | undefined;

	constructor(semaphore: Semaphore, n: number) {
		checkPermits(n);
		this.#semaphore = semaphore;
		this.#n = n;
		this.remaining = n;
	}

	takeWaker(): Waker | undefined {
		const waker = this.#waker;
		this.#waker = undefined;
		return waker;
	}

	/** `true` once it holds every permit, `false` if the semaphore closed. */
	poll(cx: Context): Poll<boolean> {
		const sem = this.#semaphore;
		// Which a closed semaphore's failure leaves queued, so its drop gives back what it took.
		if (sem.closed) return false;
		const needed = this.#queued ? this.remaining : this.#n;
		let acquired: number;
		if (sem.permits >= needed) {
			sem.permits -= needed;
			if (!this.#queued) {
				this.remaining = 0;
				return true;
			}
			acquired = needed;
		} else {
			acquired = sem.permits;
			sem.permits = 0;
		}
		const wasQueued = this.#queued;
		this.#queued = true;
		const assign = Math.min(this.remaining, acquired);
		this.remaining -= assign;
		acquired -= assign;
		if (this.remaining === 0) {
			const at = sem.waiters.indexOf(this);
			if (at >= 0) sem.waiters.splice(at, 1);
			sem.addPermits(acquired);
			this.#queued = false;
			return true;
		}
		let old: Waker | undefined;
		if (this.#waker === undefined || !willWake(this.#waker, cx)) {
			old = this.#waker;
			this.#waker = cx.clone();
		}
		if (!wasQueued) sem.waiters.push(this);
		old?.[Symbol.dispose]();
		return Pending;
	}

	[Symbol.dispose](): void {
		if (!this.#queued) return;
		const sem = this.#semaphore;
		const at = sem.waiters.indexOf(this);
		if (at >= 0) sem.waiters.splice(at, 1);
		const acquired = this.#n - this.remaining;
		if (acquired > 0) sem.addPermits(acquired);
		this.takeWaker()?.[Symbol.dispose]();
	}
}

// An `async fn`'s future

/**
 * The future of one of tokio's `async fn`s: `step` until it is ready, which a later poll
 * panics after, as any `async fn` does; dropped before then, `cancel` drops what it holds.
 */
abstract class AsyncFn<T> implements Future<T> {
	#done = false;

	poll(cx: Context): Poll<T> {
		if (this.#done) asyncFnResumed();
		const output = this.step(cx);
		if (output !== Pending) this.#done = true;
		return output;
	}

	protected abstract step(cx: Context): Poll<T>;

	protected cancel(): void {}

	[Symbol.dispose](): void {
		if (!this.#done) this.cancel();
	}
}

// mpsc

/** The values sent and not yet received, oldest first. */
class Queue<T> {
	#items: T[] = [];
	#head = 0;

	get length(): number {
		return this.#items.length - this.#head;
	}

	push(value: T): void {
		this.#items.push(value);
	}

	/** The oldest value, which must be there. */
	shift(): T {
		const value = this.#items[this.#head];
		this.#items[this.#head++] = undefined as T;
		if (this.#head * 2 >= this.#items.length) {
			this.#items = this.#items.slice(this.#head);
			this.#head = 0;
		}
		return value;
	}
}

/** What a pop of an empty channel finds: no value yet, or the mark the last sender left. */
const EMPTY: unique symbol = /* @__PURE__ */ Symbol("EMPTY");
const CLOSED: unique symbol = /* @__PURE__ */ Symbol("CLOSED");

/** tokio's `Chan`: what an mpsc channel's handles share, which drops as the last of them does. */
class Chan<T> {
	readonly #queue = new Queue<T>();
	/** Whether the last sender dropped, which a receive reads past the values. */
	#txClosed = false;
	rxClosed = false;
	readonly rxWaker = new WakerSlot();
	readonly notifyRxClosed = new Notify();
	/** A bounded channel's semaphore and bound; an unbounded one counts its messages instead. */
	readonly semaphore: Semaphore | undefined;
	readonly bound: number;
	#messages = 0;
	#unboundedClosed = false;
	txCount = 1;
	txWeakCount = 0;
	/** The handles and permits sharing it, as its `Arc`'s count. */
	#refs = 2;
	readonly #drop: Drop<T>;

	constructor(bound: number | undefined, dropValue: Drop<T>) {
		this.semaphore = bound === undefined ? undefined : new Semaphore(bound);
		this.bound = bound ?? 0;
		this.#drop = dropValue;
	}

	retain(): void {
		this.#refs++;
	}

	/** One handle fewer; the last drops what no receiver took. */
	release(): void {
		if (--this.#refs > 0) return;
		while (this.#queue.length > 0) this.#drop?.(this.#queue.shift());
		this.rxWaker.take()?.[Symbol.dispose]();
	}

	/** A value, or what an empty channel is. */
	pop(): T | typeof EMPTY | typeof CLOSED {
		if (this.#queue.length > 0) return this.#queue.shift();
		return this.#txClosed ? CLOSED : EMPTY;
	}

	get length(): number {
		return this.#queue.length;
	}

	send(value: T): void {
		this.#queue.push(value);
		this.rxWaker.wake();
	}

	/** The unbounded channel's count of a message, or `false` once its receiver closed. */
	countMessage(): boolean {
		if (this.#unboundedClosed) return false;
		this.#messages++;
		return true;
	}

	addPermits(n: number): void {
		if (this.semaphore !== undefined) this.semaphore.release(n);
		else this.#messages -= n;
	}

	/** Whether no permit or message is out, so a closed receiver has seen every value. */
	isIdle(): boolean {
		return this.semaphore !== undefined ? this.semaphore.permits === this.bound : this.#messages === 0;
	}

	isClosed(): boolean {
		return this.semaphore !== undefined ? this.semaphore.closed : this.#unboundedClosed;
	}

	/** A permit or several given back: wake the receiver if that was the last one out of a closed channel. */
	givePermits(n: number): void {
		this.addPermits(n);
		if (this.isClosed() && this.isIdle()) this.rxWaker.wake();
	}

	/** The last sender's drop. */
	closeTx(): void {
		this.#txClosed = true;
		this.rxWaker.wake();
	}

	/** `Receiver::close`. */
	close(): void {
		this.rxClosed = true;
		if (this.semaphore !== undefined) this.semaphore.close();
		else this.#unboundedClosed = true;
		this.notifyRxClosed.notifyWaiters();
	}

	/** `Rx::recv`. */
	recv<N>(cx: Context, none: N): Poll<T | N> {
		const take = (): T | N | typeof EMPTY => {
			const value = this.pop();
			if (value === CLOSED) return none;
			if (value !== EMPTY) this.addPermits(1);
			return value;
		};
		let value = take();
		if (value !== EMPTY) return value;
		this.rxWaker.register(cx);
		value = take();
		if (value !== EMPTY) return value;
		return this.rxClosed && this.isIdle() ? none : Pending;
	}

	/** `Rx::recv_many`: how many values it pushed onto `buffer`, by `push`, 0 once closed. */
	recvMany<V>(cx: Context, buffer: V, limit: number, push: (buffer: V, value: T) => void): Poll<number> {
		if (limit === 0) return 0;
		const take = (): number | undefined => {
			let added = 0;
			while (added < limit) {
				const value = this.pop();
				if (value === EMPTY) break;
				if (value === CLOSED) {
					if (added > 0) this.addPermits(added);
					return added;
				}
				push(buffer, value);
				added++;
			}
			if (added === 0) return undefined;
			this.addPermits(added);
			return added;
		};
		let added = take();
		if (added !== undefined) return added;
		this.rxWaker.register(cx);
		added = take();
		if (added !== undefined) return added;
		return this.rxClosed && this.isIdle() ? 0 : Pending;
	}

	/** `Rx::drop`: close, then drop every value left, oldest first. */
	dropRx(): void {
		this.close();
		this.rxWaker.take()?.[Symbol.dispose]();
		for (;;) {
			const value = this.pop();
			if (value === EMPTY || value === CLOSED) break;
			this.addPermits(1);
			this.#drop?.(value);
		}
		this.release();
	}

	/** `Tx::drop`. */
	dropTx(): void {
		if (--this.txCount === 0) this.closeTx();
		this.release();
	}
}

/** `mpsc::Sender<T>` and `UnboundedSender<T>`: a sender of the channel, which drops as one. */
export class MpscSender<T> implements Disposable {
	readonly chan: Chan<T>;

	constructor(chan: Chan<T>) {
		this.chan = chan;
	}

	[Symbol.dispose](): void {
		this.chan.dropTx();
	}
}

/** `mpsc::Receiver<T>` and `UnboundedReceiver<T>`. */
export class MpscReceiver<T> implements Disposable {
	readonly chan: Chan<T>;

	constructor(chan: Chan<T>) {
		this.chan = chan;
	}

	[Symbol.dispose](): void {
		this.chan.dropRx();
	}
}

/** `mpsc::WeakSender<T>` and `WeakUnboundedSender<T>`, which keep the channel but not its senders' count. */
export class MpscWeakSender<T> implements Disposable {
	readonly chan: Chan<T>;

	constructor(chan: Chan<T>) {
		this.chan = chan;
	}

	[Symbol.dispose](): void {
		this.chan.txWeakCount--;
		this.chan.release();
	}
}

/** `mpsc::Permit<T>`: one slot of the channel, given back as it drops unless it sent. */
export class MpscPermit<T> implements Disposable {
	readonly chan: Chan<T>;
	used = false;

	constructor(chan: Chan<T>) {
		this.chan = chan;
	}

	[Symbol.dispose](): void {
		if (!this.used) this.chan.givePermits(1);
	}
}

/** `Permit::send(value)`, which consumes it. */
export function mpscPermitSend<T>(permit: MpscPermit<T>, value: T): void {
	permit.used = true;
	permit.chan.send(value);
}

/** `mpsc::PermitIterator<T>`: its permits, which it gives back as it drops. */
export class MpscPermits<T> extends Iter<MpscPermit<T>> implements Disposable {
	readonly chan: Chan<T>;
	n: number;

	constructor(chan: Chan<T>, n: number) {
		super();
		this.chan = chan;
		this.n = n;
	}

	next(): MpscPermit<T> | Done {
		if (this.n === 0) return DONE;
		this.n--;
		return new MpscPermit(this.chan);
	}

	override sizeHint(): [number, number | undefined] {
		return [this.n, this.n];
	}

	/** `ExactSizeIterator::len`. */
	len(): number {
		return this.n;
	}

	[Symbol.dispose](): void {
		if (this.n > 0) this.chan.givePermits(this.n);
	}
}

/** `mpsc::OwnedPermit<T>`: a slot held with a sender of its own. */
export class MpscOwnedPermit<T> implements Disposable {
	chan: Chan<T> | undefined;

	constructor(chan: Chan<T>) {
		this.chan = chan;
	}

	[Symbol.dispose](): void {
		const chan = this.chan;
		if (chan === undefined) return;
		this.chan = undefined;
		chan.givePermits(1);
		chan.dropTx();
	}
}

/** `mpsc::channel(buffer)`, of values `dropValue` drops. */
export function mpscChannel<T>(buffer: usize, dropValue?: Drop<T>, at?: Location): [MpscSender<T>, MpscReceiver<T>] {
	if (buffer === 0) panic("mpsc bounded channel requires buffer > 0", at);
	checkPermits(buffer, at);
	const chan = new Chan<T>(buffer, dropValue);
	return [new MpscSender(chan), new MpscReceiver(chan)];
}

/** `mpsc::unbounded_channel()`, of values `dropValue` drops. */
export function mpscUnboundedChannel<T>(dropValue?: Drop<T>): [MpscSender<T>, MpscReceiver<T>] {
	const chan = new Chan<T>(undefined, dropValue);
	return [new MpscSender(chan), new MpscReceiver(chan)];
}

/** A sender's `clone`. */
export function mpscSenderClone<T>(tx: MpscSender<T>): MpscSender<T> {
	tx.chan.txCount++;
	tx.chan.retain();
	return new MpscSender(tx.chan);
}

/** `Sender::send`: the future of `value`'s send, ready with `OK` or `error` of it once the channel closed. */
export function mpscSend<T, E>(tx: MpscSender<T>, value: T, error: (value: T) => E, dropValue?: Drop<T>): Future<Result<undefined, E>> {
	return new Reserve(tx.chan, 1, (permits) => {
		if (permits === undefined) return Err(error(value));
		permits.chan.send(value);
		return OK;
	}, () => dropValue?.(value));
}

/** `UnboundedSender::send`: `OK`, or `error` of `value` once the receiver closed. */
export function mpscUnboundedSend<T, E>(tx: MpscSender<T>, value: T, error: (value: T) => E): Result<undefined, E> {
	if (!tx.chan.countMessage()) return Err(error(value));
	tx.chan.send(value);
	return OK;
}

/** `Sender::try_send`: `OK`, or `full` or `closed` of `value`. */
export function mpscTrySend<T, E>(tx: MpscSender<T>, value: T, full: (value: T) => E, closed: (value: T) => E): Result<undefined, E> {
	switch (tx.chan.semaphore!.tryAcquire(1)) {
		case "closed":
			return Err(closed(value));
		case "none":
			return Err(full(value));
	}
	tx.chan.send(value);
	return OK;
}

/** `reserve_inner(n)` and what the reservation becomes, `undefined` once the channel closed. */
class Reserve<T, R> extends AsyncFn<R> {
	readonly #chan: Chan<T>;
	readonly #n: number;
	readonly #then: (permits: { chan: Chan<T> } | undefined) => R;
	readonly #cancel: (() => void) | undefined;
	#acquire: Acquire | undefined;

	constructor(chan: Chan<T>, n: number, then: (permits: { chan: Chan<T> } | undefined) => R, cancel?: () => void) {
		super();
		this.#chan = chan;
		this.#n = n;
		this.#then = then;
		this.#cancel = cancel;
	}

	protected step(cx: Context): Poll<R> {
		if (this.#acquire === undefined) {
			if (this.#n > this.#chan.bound) return this.#then(undefined);
			this.#acquire = new Acquire(this.#chan.semaphore!, this.#n);
		}
		const acquired = this.#acquire.poll(cx);
		if (acquired === Pending) return Pending;
		if (acquired) {
			this.#acquire = undefined;
			return this.#then({ chan: this.#chan });
		}
		this.#dropAcquire();
		return this.#then(undefined);
	}

	/** The `Acquire`'s drop, then the guard's, which wakes the receiver once the last permit of a closed channel is back. */
	#dropAcquire(): void {
		const acquire = this.#acquire;
		if (acquire === undefined) return;
		this.#acquire = undefined;
		acquire[Symbol.dispose]();
		if (this.#chan.isClosed() && this.#chan.isIdle()) this.#chan.rxWaker.wake();
	}

	protected override cancel(): void {
		this.#dropAcquire();
		this.#cancel?.();
	}
}

/** `Sender::reserve`: the future of a `Permit`, or `error` once the channel closed. */
export function mpscReserve<T, E>(tx: MpscSender<T>, error: E): Future<Result<MpscPermit<T>, E>> {
	return new Reserve(tx.chan, 1, (permits) => (permits === undefined ? Err(error) : Ok(new MpscPermit(permits.chan))));
}

/** `Sender::reserve_many(n)`: the future of `n` permits, or `error` once the channel closed or can't hold `n`. */
export function mpscReserveMany<T, E>(tx: MpscSender<T>, n: usize, error: E): Future<Result<MpscPermits<T>, E>> {
	return new Reserve(tx.chan, n, (permits) => (permits === undefined ? Err(error) : Ok(new MpscPermits(permits.chan, n))));
}

/** `Sender::reserve_owned`, which consumes `tx`: the future of an `OwnedPermit`, or `error` once the channel closed, which drops `tx`. */
export function mpscReserveOwned<T, E>(tx: MpscSender<T>, error: E): Future<Result<MpscOwnedPermit<T>, E>> {
	return new Reserve(
		tx.chan,
		1,
		(permits) => {
			if (permits !== undefined) return Ok(new MpscOwnedPermit(permits.chan));
			tx[Symbol.dispose]();
			return Err(error);
		},
		() => tx[Symbol.dispose](),
	);
}

/** `Sender::try_reserve`: a `Permit`, or `full` or `closed`. */
export function mpscTryReserve<T, E>(tx: MpscSender<T>, full: E, closed: E): Result<MpscPermit<T>, E> {
	switch (tx.chan.semaphore!.tryAcquire(1)) {
		case "closed":
			return Err(closed);
		case "none":
			return Err(full);
	}
	return Ok(new MpscPermit(tx.chan));
}

/** `Sender::try_reserve_many(n)`: `n` permits, or `full` or `closed`. */
export function mpscTryReserveMany<T, E>(tx: MpscSender<T>, n: usize, full: E, closed: E): Result<MpscPermits<T>, E> {
	if (n > tx.chan.bound) return Err(full);
	switch (tx.chan.semaphore!.tryAcquire(n)) {
		case "closed":
			return Err(closed);
		case "none":
			return Err(full);
	}
	return Ok(new MpscPermits(tx.chan, n));
}

/** `Sender::try_reserve_owned`, which consumes `tx`: an `OwnedPermit`, or `full` or `closed` of `tx`. */
export function mpscTryReserveOwned<T, E>(tx: MpscSender<T>, full: (tx: MpscSender<T>) => E, closed: (tx: MpscSender<T>) => E): Result<MpscOwnedPermit<T>, E> {
	switch (tx.chan.semaphore!.tryAcquire(1)) {
		case "closed":
			return Err(closed(tx));
		case "none":
			return Err(full(tx));
	}
	return Ok(new MpscOwnedPermit(tx.chan));
}

/** `closed`: ready once the receiver closed or dropped. */
class Closed<T> extends AsyncFn<undefined> {
	readonly #chan: Chan<T>;
	#notified: Notified | undefined;

	constructor(chan: Chan<T>) {
		super();
		this.#chan = chan;
	}

	protected step(cx: Context): Poll<undefined> {
		if (this.#notified === undefined) {
			this.#notified = this.#chan.notifyRxClosed.notified();
			if (this.#chan.isClosed()) return this.#done();
		}
		return this.#notified.poll(cx) === Pending ? Pending : this.#done();
	}

	#done(): undefined {
		this.#notified?.[Symbol.dispose]();
		this.#notified = undefined;
		return undefined;
	}

	protected override cancel(): void {
		this.#done();
	}
}

/** A sender's `closed()`. */
export function mpscClosed<T>(tx: MpscSender<T>): Future<undefined> {
	return new Closed(tx.chan);
}

/** A sender's `is_closed()`. */
export function mpscIsClosed<T>(tx: MpscSender<T>): boolean {
	return tx.chan.isClosed();
}

/** `same_channel` of two senders, receivers, or permits' channels. */
export function mpscSameChannel<T>(a: { chan: Chan<T> }, b: { chan: Chan<T> }): boolean {
	return a.chan === b.chan;
}

/** `capacity()`: the permits not taken. */
export function mpscCapacity<T>(handle: { chan: Chan<T> }): usize {
	return handle.chan.semaphore!.permits as usize;
}

/** `max_capacity()`: the bound. */
export function mpscMaxCapacity<T>(handle: { chan: Chan<T> }): usize {
	return handle.chan.bound as usize;
}

/** `downgrade()`. */
export function mpscDowngrade<T>(tx: MpscSender<T>): MpscWeakSender<T> {
	tx.chan.txWeakCount++;
	tx.chan.retain();
	return new MpscWeakSender(tx.chan);
}

/** `strong_count()`, of any handle. */
export function mpscStrongCount<T>(handle: { chan: Chan<T> }): usize {
	return handle.chan.txCount as usize;
}

/** `weak_count()`, of any handle. */
export function mpscWeakCount<T>(handle: { chan: Chan<T> }): usize {
	return handle.chan.txWeakCount as usize;
}

/** A weak sender's `clone`. */
export function mpscWeakClone<T>(weak: MpscWeakSender<T>): MpscWeakSender<T> {
	weak.chan.txWeakCount++;
	weak.chan.retain();
	return new MpscWeakSender(weak.chan);
}

/** `WeakSender::upgrade`: a sender while one is left, or `undefined`. */
export function mpscUpgrade<T>(weak: MpscWeakSender<T>): MpscSender<T> | undefined {
	if (weak.chan.txCount === 0) return undefined;
	weak.chan.txCount++;
	weak.chan.retain();
	return new MpscSender(weak.chan);
}

/** `recv()`: the future of the next value, or `none` once every sender dropped or the receiver closed and drained. */
class Recv<T, N> extends AsyncFn<T | N> {
	readonly #chan: Chan<T>;
	readonly #none: N;

	constructor(chan: Chan<T>, none: N) {
		super();
		this.#chan = chan;
		this.#none = none;
	}

	protected step(cx: Context): Poll<T | N> {
		return this.#chan.recv(cx, this.#none);
	}
}

/** A receiver's `recv()`, whose `None` is `none` where it is `rs.None`. */
export function mpscRecv<T>(rx: MpscReceiver<T>): Future<T | undefined>;
export function mpscRecv<T>(rx: MpscReceiver<T>, none: None): Future<T | None>;
export function mpscRecv<T>(rx: MpscReceiver<T>, none?: None): Future<T | undefined | None> {
	return new Recv(rx.chan, none);
}

/** A receiver's `poll_recv(cx)`. */
export function mpscPollRecv<T>(rx: MpscReceiver<T>, cx: Context): Poll<T | undefined>;
export function mpscPollRecv<T>(rx: MpscReceiver<T>, cx: Context, none: None): Poll<T | None>;
export function mpscPollRecv<T>(rx: MpscReceiver<T>, cx: Context, none?: None): Poll<T | undefined | None> {
	return rx.chan.recv(cx, none);
}

/** `recv_many(buffer, limit)`, which `push`es each value onto `buffer`. */
class RecvMany<T, V> extends AsyncFn<usize> {
	readonly #chan: Chan<T>;
	readonly #buffer: V;
	readonly #limit: number;
	readonly #push: (buffer: V, value: T) => void;

	constructor(chan: Chan<T>, buffer: V, limit: number, push: (buffer: V, value: T) => void) {
		super();
		this.#chan = chan;
		this.#buffer = buffer;
		this.#limit = limit;
		this.#push = push;
	}

	protected step(cx: Context): Poll<usize> {
		return this.#chan.recvMany(cx, this.#buffer, this.#limit, this.#push) as Poll<usize>;
	}
}

/** A receiver's `recv_many(buffer, limit)`: the future of how many values it pushed by `push`, 0 once closed. */
export function mpscRecvMany<T, V>(rx: MpscReceiver<T>, buffer: V, limit: usize, push: (buffer: V, value: T) => void): Future<usize> {
	return new RecvMany(rx.chan, buffer, limit, push);
}

/** A receiver's `poll_recv_many(cx, buffer, limit)`. */
export function mpscPollRecvMany<T, V>(rx: MpscReceiver<T>, cx: Context, buffer: V, limit: usize, push: (buffer: V, value: T) => void): Poll<usize> {
	return rx.chan.recvMany(cx, buffer, limit, push) as Poll<usize>;
}

/** `Vec::push`, which `recv_many` pushes by. */
export function mpscVecPush<T>(v: Vec<T>, value: T): void {
	v.items.push(value);
}

/** `try_recv()`: the next value, or `empty`, or `disconnected` once every sender dropped or the receiver closed and drained. */
export function mpscTryRecv<T, E>(rx: MpscReceiver<T>, empty: E, disconnected: E): Result<T, E> {
	const chan = rx.chan;
	const value = chan.pop();
	if (value === CLOSED) return Err(disconnected);
	if (value === EMPTY) return Err(chan.rxClosed && chan.isIdle() ? disconnected : empty);
	chan.addPermits(1);
	return Ok(value);
}

/** A receiver's `close()`. */
export function mpscClose<T>(rx: MpscReceiver<T>): void {
	rx.chan.close();
}

/** A receiver's `is_closed()`: closed, or every sender dropped. */
export function mpscRxIsClosed<T>(rx: MpscReceiver<T>): boolean {
	return rx.chan.isClosed() || rx.chan.txCount === 0;
}

/** A receiver's `is_empty()`. */
export function mpscIsEmpty<T>(rx: MpscReceiver<T>): boolean {
	return rx.chan.length === 0;
}

/** A receiver's `len()`. */
export function mpscLen<T>(rx: MpscReceiver<T>): usize {
	return rx.chan.length as usize;
}

/** `OwnedPermit::send(value)`: its sender, once `value` is sent. */
export function mpscOwnedPermitSend<T>(permit: MpscOwnedPermit<T>, value: T): MpscSender<T> {
	const chan = permit.chan!;
	permit.chan = undefined;
	chan.send(value);
	return new MpscSender(chan);
}

/** `OwnedPermit::release()`: its sender, once its slot is given back. */
export function mpscOwnedPermitRelease<T>(permit: MpscOwnedPermit<T>): MpscSender<T> {
	const chan = permit.chan!;
	permit.chan = undefined;
	chan.givePermits(1);
	return new MpscSender(chan);
}

/** `OwnedPermit::same_channel`, of permits that still hold a sender. */
export function mpscOwnedPermitSameChannel<T>(a: MpscOwnedPermit<T>, b: MpscOwnedPermit<T>): boolean {
	return a.chan !== undefined && a.chan === b.chan;
}

/** `OwnedPermit::same_channel_as_sender`. */
export function mpscOwnedPermitSameChannelAsSender<T>(permit: MpscOwnedPermit<T>, tx: MpscSender<T>): boolean {
	return permit.chan === tx.chan;
}

// watch

/** What a watch channel's handles share. */
class Shared<T> {
	/** The value, behind the `RwLock` that a `Ref` holds read. */
	readonly value: RefCell<T>;
	/** Its version, which steps by 2 at each send, as tokio's does below its closed bit. */
	version = 0;
	closed = false;
	rxCount = 1;
	txCount = 1;
	/** tokio's `BigNotify`: eight `Notify`s, a receiver's wait on one picked at random. */
	readonly notifyRx: Notify[] = Array.from({ length: 8 }, () => new Notify());
	readonly notifyTx = new Notify();
	#refs = 2;
	readonly #drop: Drop<T>;

	constructor(init: T, dropValue: Drop<T>) {
		this.value = new RefCell(init);
		this.#drop = dropValue;
	}

	retain(): void {
		this.#refs++;
	}

	release(): void {
		if (--this.#refs === 0) this.#drop?.(this.value.value);
	}

	notifyReceivers(): void {
		for (const notify of this.notifyRx) notify.notifyWaiters();
	}
}

/** `watch::Sender<T>`. */
export class WatchSender<T> implements Disposable {
	readonly shared: Shared<T>;

	constructor(shared: Shared<T>) {
		this.shared = shared;
	}

	[Symbol.dispose](): void {
		const shared = this.shared;
		if (--shared.txCount === 0) {
			shared.closed = true;
			shared.notifyReceivers();
		}
		shared.release();
	}
}

/** `watch::Receiver<T>`: the version it saw last. */
export class WatchReceiver<T> implements Disposable {
	readonly shared: Shared<T>;
	version: number;

	constructor(shared: Shared<T>, version: number) {
		this.shared = shared;
		this.version = version;
	}

	[Symbol.dispose](): void {
		const shared = this.shared;
		if (--shared.rxCount === 0) shared.notifyTx.notifyWaiters();
		shared.release();
	}
}

/** `watch::Ref<T>`: the value, held read until it drops. */
export class WatchRef<T> implements Disposable {
	readonly #lock: RefCell<T>;
	readonly hasChanged: boolean;

	constructor(lock: RefCell<T>, hasChanged: boolean) {
		this.#lock = lock;
		this.hasChanged = hasChanged;
	}

	get value(): T {
		return this.#lock.value;
	}

	[Symbol.dispose](): void {
		refRelease(this.#lock);
	}
}

/** `watch::channel(init)`, of a value `dropValue` drops. */
export function watchChannel<T>(init: T, dropValue?: Drop<T>): [WatchSender<T>, WatchReceiver<T>] {
	const shared = new Shared(init, dropValue);
	return [new WatchSender(shared), new WatchReceiver(shared, 0)];
}

/** `watch::Sender::new(init)`: a sender whose receiver dropped. */
export function watchSenderNew<T>(init: T, dropValue?: Drop<T>): WatchSender<T> {
	const [tx, rx] = watchChannel(init, dropValue);
	rx[Symbol.dispose]();
	return tx;
}

/** A sender's `clone`. */
export function watchSenderClone<T>(tx: WatchSender<T>): WatchSender<T> {
	tx.shared.txCount++;
	tx.shared.retain();
	return new WatchSender(tx.shared);
}

/** A receiver's `clone`, at the version it saw. */
export function watchReceiverClone<T>(rx: WatchReceiver<T>): WatchReceiver<T> {
	return watchSubscribe(rx.shared, rx.version);
}

function watchSubscribe<T>(shared: Shared<T>, version: number): WatchReceiver<T> {
	shared.rxCount++;
	shared.retain();
	return new WatchReceiver(shared, version);
}

/** A sender's `subscribe()`, at the current version. */
export function watchSenderSubscribe<T>(tx: WatchSender<T>): WatchReceiver<T> {
	return watchSubscribe(tx.shared, tx.shared.version);
}

/** A receiver's `borrow()`, or with `update` its `borrow_and_update()`. */
export function watchBorrow<T>(rx: WatchReceiver<T>, update = false): WatchRef<T> {
	const lock = rwLockRead(rx.shared.value);
	const version = rx.shared.version;
	const changed = rx.version !== version;
	if (update) rx.version = version;
	return new WatchRef(lock, changed);
}

/** A sender's `borrow()`, which never has changed. */
export function watchSenderBorrow<T>(tx: WatchSender<T>): WatchRef<T> {
	return new WatchRef(rwLockRead(tx.shared.value), false);
}

/** A receiver's `has_changed()`: whether a send came since it last saw, or `error` once every sender dropped. */
export function watchHasChanged<T, E>(rx: WatchReceiver<T>, error: E): Result<boolean, E> {
	if (rx.shared.closed) return Err(error);
	return Ok(rx.version !== rx.shared.version);
}

/** A receiver's `is_closed()`. */
export function watchIsClosed<T>(rx: WatchReceiver<T>): boolean {
	return rx.shared.closed;
}

/** `mark_changed()`: one version back, wrapping as wasm32's `usize` does. */
export function watchMarkChanged<T>(rx: WatchReceiver<T>): void {
	rx.version = (rx.version - 2) >>> 0;
}

/** `mark_unchanged()`. */
export function watchMarkUnchanged<T>(rx: WatchReceiver<T>): void {
	rx.version = rx.shared.version;
}

/** `same_channel`, of two senders or two receivers. */
export function watchSameChannel<T>(a: { shared: Shared<T> }, b: { shared: Shared<T> }): boolean {
	return a.shared === b.shared;
}

/**
 * tokio's `changed_impl`: ready with `true` once the version is not the receiver's, which
 * it takes, or `false` once every sender dropped, waiting on a `Notify` picked at random.
 */
class Changed<T> implements Disposable {
	readonly #rx: WatchReceiver<T>;
	#notified: Notified | undefined;

	constructor(rx: WatchReceiver<T>) {
		this.#rx = rx;
	}

	poll(cx: Context): Poll<boolean> {
		const rx = this.#rx;
		for (;;) {
			if (this.#notified === undefined) {
				this.#notified = rx.shared.notifyRx[randBelow(8)].notified();
				const version = rx.shared.version;
				if (rx.version !== version) {
					rx.version = version;
					return this.#done(true);
				}
				if (rx.shared.closed) return this.#done(false);
			}
			if (this.#notified.poll(cx) === Pending) return Pending;
			this.#done(true);
		}
	}

	#done(changed: boolean): boolean {
		this.#notified?.[Symbol.dispose]();
		this.#notified = undefined;
		return changed;
	}

	[Symbol.dispose](): void {
		this.#done(true);
	}
}

/** `changed()`, a future of `OK`, or of `error` once every sender dropped. */
class WatchChanged<T, E> extends AsyncFn<Result<undefined, E>> {
	readonly #changed: Changed<T>;
	readonly #error: E;

	constructor(rx: WatchReceiver<T>, error: E) {
		super();
		this.#changed = new Changed(rx);
		this.#error = error;
	}

	protected step(cx: Context): Poll<Result<undefined, E>> {
		const changed = this.#changed.poll(cx);
		if (changed === Pending) return Pending;
		return changed ? OK : Err(this.#error);
	}

	protected override cancel(): void {
		this.#changed[Symbol.dispose]();
	}
}

/** A receiver's `changed()`. */
export function watchChanged<T, E>(rx: WatchReceiver<T>, error: E): Future<Result<undefined, E>> {
	return new WatchChanged(rx, error);
}

/** `wait_for(f)`: the future of a `Ref` to the first value `f` accepts, or `error` once every sender dropped. */
class WaitFor<T, E> extends AsyncFn<Result<WatchRef<T>, E>> {
	readonly #rx: WatchReceiver<T>;
	readonly #f: (value: T) => boolean;
	readonly #dropF: Drop<(value: T) => boolean>;
	readonly #error: E;
	#closed = false;
	#changed: Changed<T> | undefined;

	constructor(rx: WatchReceiver<T>, f: (value: T) => boolean, error: E, dropF?: Drop<(value: T) => boolean>) {
		super();
		this.#rx = rx;
		this.#f = f;
		this.#error = error;
		this.#dropF = dropF;
	}

	protected step(cx: Context): Poll<Result<WatchRef<T>, E>> {
		const rx = this.#rx;
		for (;;) {
			if (this.#changed === undefined) {
				const lock = rwLockRead(rx.shared.value);
				const version = rx.shared.version;
				const changed = rx.version !== version;
				rx.version = version;
				if (!this.#closed || changed) {
					let accepted: boolean;
					try {
						accepted = this.#f(lock.value);
					} catch (err) {
						// tokio releases the lock before the panic goes on, so it poisons nothing.
						refRelease(lock);
						throw err;
					}
					if (accepted) {
						this.#dropF?.(this.#f);
						return Ok(new WatchRef(lock, changed));
					}
				}
				refRelease(lock);
				if (this.#closed) {
					this.#dropF?.(this.#f);
					return Err(this.#error);
				}
				this.#changed = new Changed(rx);
			}
			const changed = this.#changed.poll(cx);
			if (changed === Pending) return Pending;
			this.#changed = undefined;
			this.#closed = !changed;
		}
	}

	protected override cancel(): void {
		this.#changed?.[Symbol.dispose]();
		this.#dropF?.(this.#f);
	}
}

/** A receiver's `wait_for(f)`, whose closure `dropF` drops, if it drops. */
export function watchWaitFor<T, E>(rx: WatchReceiver<T>, f: (value: T) => boolean, error: E, dropF?: Drop<(value: T) => boolean>): Future<Result<WatchRef<T>, E>> {
	return new WaitFor(rx, f, error, dropF);
}

/**
 * `send_if_modified(modify)`: `modify` of the value, or of its place where JS copies it,
 * held written, and a new version and a wake of every receiver if it says it modified it.
 */
export function watchSendIfModified<T, F extends (value: never) => boolean>(tx: WatchSender<T>, modify: F, copied = false, dropModify?: Drop<F>): boolean {
	const shared = tx.shared;
	const lock = rwLockWrite(shared.value);
	const at = copied
		? place(
				() => lock.value,
				(v) => {
					lock.value = v;
				},
			)
		: lock.value;
	const modified = modify(at as never);
	dropModify?.(modify);
	refMutRelease(lock);
	if (!modified) return false;
	shared.version = (shared.version + 2) >>> 0;
	shared.notifyReceivers();
	return true;
}

/** `send_modify(modify)`: `send_if_modified` of `modify`, which always modifies. */
export function watchSendModify<T, F extends (value: never) => void>(tx: WatchSender<T>, modify: F, copied = false, dropModify?: Drop<F>): void {
	watchSendIfModified(
		tx,
		(at: never) => {
			modify(at);
			dropModify?.(modify);
			return true;
		},
		copied,
	);
}

/** `send_replace(value)`: the value it replaces. */
export function watchSendReplace<T>(tx: WatchSender<T>, value: T): T {
	let old!: T;
	watchSendIfModified(
		tx,
		(at: Place<T>) => {
			const lock = at;
			old = lock.v;
			lock.v = value;
			return true;
		},
		true,
	);
	return old;
}

/** `send(value)`: `OK`, dropping the value it replaces by `dropValue`, or `error` of `value` while no receiver is left. */
export function watchSend<T, E>(tx: WatchSender<T>, value: T, error: (value: T) => E, dropValue?: Drop<T>): Result<undefined, E> {
	if (tx.shared.rxCount === 0) return Err(error(value));
	dropValue?.(watchSendReplace(tx, value));
	return OK;
}

/** A sender's `is_closed()`: whether no receiver is left. */
export function watchSenderIsClosed<T>(tx: WatchSender<T>): boolean {
	return tx.shared.rxCount === 0;
}

/** `receiver_count()`. */
export function watchReceiverCount<T>(tx: WatchSender<T>): usize {
	return tx.shared.rxCount as usize;
}

/** `sender_count()`. */
export function watchSenderCount<T>(tx: WatchSender<T>): usize {
	return tx.shared.txCount as usize;
}

/** A sender's `closed()`: ready once no receiver is left. */
class WatchClosed<T> extends AsyncFn<undefined> {
	readonly #shared: Shared<T>;
	#notified: Notified | undefined;

	constructor(shared: Shared<T>) {
		super();
		this.#shared = shared;
	}

	protected step(cx: Context): Poll<undefined> {
		for (;;) {
			if (this.#notified === undefined) {
				if (this.#shared.rxCount === 0) return undefined;
				this.#notified = this.#shared.notifyTx.notified();
				if (this.#shared.rxCount === 0) return this.#done();
			}
			if (this.#notified.poll(cx) === Pending) return Pending;
			this.#done();
		}
	}

	#done(): undefined {
		this.#notified?.[Symbol.dispose]();
		this.#notified = undefined;
		return undefined;
	}

	protected override cancel(): void {
		this.#done();
	}
}

/** A sender's `closed()`. */
export function watchSenderClosed<T>(tx: WatchSender<T>): Future<undefined> {
	return new WatchClosed(tx.shared);
}

/** tokio's `Version` Debug. */
function versionDebug(version: number, f: Formatter): FmtResult {
	return fmtDebugTupleFields(f, "Version", [(f) => usizeDebug(version as usize, f)]);
}

/** `Debug` of what a watch channel shares, as tokio's writes it. */
function sharedDebug<T>(shared: Shared<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const b = fmtDebugStruct(f, "Shared");
	debugStructField(b, "value", (f) => fmtDebugTupleFields(f, "RwLock", [(f) => rwLockDebug(shared.value, f, debug)]));
	debugStructField(b, "version", (f) => versionDebug(shared.version, f));
	debugStructField(b, "is_closed", (f) => fmtBool(shared.closed, f));
	debugStructField(b, "ref_count_rx", (f) => usizeDebug(shared.rxCount as usize, f));
	return debugStructFinish(b);
}

/** `Debug` of a watch sender. */
export function watchSenderDebug<T>(tx: WatchSender<T>, f: Formatter, debug: Debug<T>): FmtResult {
	return debugStructFinish(debugStructField(fmtDebugStruct(f, "Sender"), "shared", (f) => sharedDebug(tx.shared, f, debug)));
}

/** `Debug` of a watch receiver. */
export function watchReceiverDebug<T>(rx: WatchReceiver<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const b = fmtDebugStruct(f, "Receiver");
	debugStructField(b, "shared", (f) => sharedDebug(rx.shared, f, debug));
	debugStructField(b, "version", (f) => versionDebug(rx.version, f));
	return debugStructFinish(b);
}

/** `Debug` of a `watch::Ref`: the value it holds read. */
export function watchRefDebug<T>(r: WatchRef<T>, f: Formatter, debug: Debug<T>): FmtResult {
	const b = fmtDebugStruct(f, "Ref");
	debugStructField(b, "inner", (f) => debug(r.value, f));
	debugStructField(b, "has_changed", (f) => fmtBool(r.hasChanged, f));
	return debugStructFinish(b);
}

// oneshot

/** No value: none sent, or the receiver took it. */
const NO_VALUE: unique symbol = /* @__PURE__ */ Symbol("NO_VALUE");

/** What a oneshot channel's two handles share. */
class Inner<T> {
	complete = false;
	closed = false;
	value: T | typeof NO_VALUE = NO_VALUE;
	rxTask: Waker | undefined;
	txTask: Waker | undefined;
	#refs = 2;
	readonly drop: Drop<T>;

	constructor(dropValue: Drop<T>) {
		this.drop = dropValue;
	}

	release(): void {
		if (--this.#refs > 0) return;
		this.rxTask?.[Symbol.dispose]();
		this.txTask?.[Symbol.dispose]();
	}

	/** The sender's `complete`: `false` if the receiver closed first, and else a wake of its task. */
	completeTx(): boolean {
		if (this.closed) return false;
		this.complete = true;
		this.rxTask?.wakeByRef();
		return true;
	}

	/** The receiver's `close`, which wakes the sender's task and drops its own. */
	close(): void {
		const complete = this.complete;
		this.closed = true;
		if (complete) return;
		this.txTask?.wakeByRef();
		const task = this.rxTask;
		this.rxTask = undefined;
		task?.[Symbol.dispose]();
	}

	take(): T | typeof NO_VALUE {
		const value = this.value;
		this.value = NO_VALUE;
		return value;
	}

	/** `poll_recv`: the value, `NO_VALUE` once the sender dropped without one, or pending. */
	pollRecv(cx: Context): Poll<T | typeof NO_VALUE> {
		if (this.complete) return this.take();
		if (this.closed) return NO_VALUE;
		if (this.rxTask !== undefined && !willWake(this.rxTask, cx)) {
			const task = this.rxTask;
			this.rxTask = undefined;
			task[Symbol.dispose]();
		}
		if (this.rxTask === undefined) {
			this.rxTask = cx.clone();
			if (this.complete) return this.take();
		}
		return Pending;
	}

	/** `poll_closed`. */
	pollClosed(cx: Context): Poll<undefined> {
		if (this.closed) return undefined;
		if (this.txTask !== undefined && !willWake(this.txTask, cx)) {
			const task = this.txTask;
			this.txTask = undefined;
			task[Symbol.dispose]();
		}
		if (this.txTask === undefined) this.txTask = cx.clone();
		return Pending;
	}
}

/** `oneshot::Sender<T>`. */
export class OneshotSender<T> implements Disposable {
	inner: Inner<T> | undefined;

	constructor(inner: Inner<T>) {
		this.inner = inner;
	}

	[Symbol.dispose](): void {
		const inner = this.inner;
		if (inner === undefined) return;
		inner.completeTx();
		inner.release();
	}
}

/** `oneshot::Receiver<T>`, a future of the value, `ok` and `error` as the lowering makes them. */
export class OneshotReceiver<T> implements Disposable {
	inner: Inner<T> | undefined;

	constructor(inner: Inner<T>) {
		this.inner = inner;
	}

	[Symbol.dispose](): void {
		const inner = this.inner;
		if (inner === undefined) return;
		const complete = inner.complete;
		inner.close();
		if (complete) {
			const value = inner.take();
			if (value !== NO_VALUE) inner.drop?.(value);
		}
		inner.release();
	}
}

/** `oneshot::channel()`, of a value `dropValue` drops. */
export function oneshotChannel<T>(dropValue?: Drop<T>): [OneshotSender<T>, OneshotReceiver<T>] {
	const inner = new Inner<T>(dropValue);
	return [new OneshotSender(inner), new OneshotReceiver(inner)];
}

/** `Sender::send(value)`, which consumes it: `OK`, or `value` back once the receiver closed. */
export function oneshotSend<T>(tx: OneshotSender<T>, value: T): Result<undefined, T> {
	const inner = tx.inner!;
	tx.inner = undefined;
	inner.value = value;
	let result: Result<undefined, T> = OK;
	if (!inner.completeTx()) result = Err(inner.take() as T);
	inner.release();
	return result;
}

/** A sender's `poll_closed(cx)`. */
export function oneshotPollClosed<T>(tx: OneshotSender<T>, cx: Context): Poll<undefined> {
	return tx.inner!.pollClosed(cx);
}

/** A sender's `closed()`. */
class OneshotClosed<T> extends AsyncFn<undefined> {
	readonly #tx: OneshotSender<T>;

	constructor(tx: OneshotSender<T>) {
		super();
		this.#tx = tx;
	}

	protected step(cx: Context): Poll<undefined> {
		return this.#tx.inner!.pollClosed(cx);
	}
}

/** A sender's `closed()`. */
export function oneshotClosed<T>(tx: OneshotSender<T>): Future<undefined> {
	return new OneshotClosed(tx);
}

/** A sender's `is_closed()`. */
export function oneshotIsClosed<T>(tx: OneshotSender<T>): boolean {
	return tx.inner!.closed;
}

/** A receiver's `close()`. */
export function oneshotClose<T>(rx: OneshotReceiver<T>): void {
	rx.inner?.close();
}

/** A receiver's `is_terminated()`. */
export function oneshotIsTerminated<T>(rx: OneshotReceiver<T>): boolean {
	return rx.inner === undefined;
}

/** A receiver's `is_empty()`: whether there is no value to take now. */
export function oneshotIsEmpty<T>(rx: OneshotReceiver<T>): boolean {
	const inner = rx.inner;
	return inner === undefined || !inner.complete || inner.value === NO_VALUE;
}

/** The receiver's handle on the channel let go, as it is once it has its outcome. */
function oneshotFinish<T>(rx: OneshotReceiver<T>): void {
	rx.inner!.release();
	rx.inner = undefined;
}

/** A receiver's `try_recv()`: the value, or `empty`, or `closed` once the sender dropped or a receive ended. */
export function oneshotTryRecv<T, E>(rx: OneshotReceiver<T>, empty: E, closed: E): Result<T, E> {
	const inner = rx.inner;
	if (inner === undefined) return Err(closed);
	let result: Result<T, E>;
	if (inner.complete) {
		const value = inner.take();
		result = value === NO_VALUE ? Err(closed) : Ok(value);
	} else if (inner.closed) result = Err(closed);
	else return Err(empty);
	oneshotFinish(rx);
	return result;
}

/** A receiver's `Future::poll`: `Ok` of the value, or `error` once the sender dropped without one, and a panic at `at` once it gave either. */
export function oneshotPoll<T, E>(rx: OneshotReceiver<T>, cx: Context, error: E, at?: Location): Poll<Result<T, E>> {
	const inner = rx.inner;
	if (inner === undefined) panic("called after complete", at);
	const value = inner.pollRecv(cx);
	if (value === Pending) return Pending;
	oneshotFinish(rx);
	return value === NO_VALUE ? Err(error) : Ok(value);
}

/** tokio's oneshot `State` Debug. */
function oneshotStateDebug<T>(inner: Inner<T>, f: Formatter): FmtResult {
	const b = fmtDebugStruct(f, "State");
	debugStructField(b, "is_complete", (f) => fmtBool(inner.complete, f));
	debugStructField(b, "is_closed", (f) => fmtBool(inner.closed, f));
	debugStructField(b, "is_rx_task_set", (f) => fmtBool(inner.rxTask !== undefined, f));
	debugStructField(b, "is_tx_task_set", (f) => fmtBool(inner.txTask !== undefined, f));
	return debugStructFinish(b);
}

/** `Debug` of a oneshot sender or receiver, named `name`, as tokio's writes it. */
export function oneshotDebug<T>(handle: { inner: Inner<T> | undefined }, f: Formatter, name: string): FmtResult {
	const inner = handle.inner;
	const held = (f: Formatter): FmtResult =>
		inner === undefined
			? f.out.writeStr("None")
			: fmtDebugTupleFields(f, "Some", [(f) => debugStructFinish(debugStructField(fmtDebugStruct(f, "Inner"), "state", (f) => oneshotStateDebug(inner, f)))]);
	return debugStructFinish(debugStructField(fmtDebugStruct(f, name), "inner", held));
}

// select!

/** `select!`'s `thread_rng_n(n)`: the branch it polls first, uniform as tokio's is. */
export function selectStart(n: number): number {
	return randBelow(n);
}

