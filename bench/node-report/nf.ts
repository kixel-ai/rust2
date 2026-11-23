// The node:ffi route: libmoq's C ABI called straight from JavaScript, and nf.c's twin of
// each function taking or returning a struct of more than one scalar. Arguments and out
// slots go through nf.c's scratch, viewed as typed arrays, as the web binding's stack in
// linear memory; a handle is its address, a bigint, as node:ffi gives every pointer.

import { Lent, OK, Owned, pend, wake, type Handle, type Result, type Route } from "./route.ts";

type Ffi = {
	DynamicLibrary: new (path: string) => {
		getFunctions(defs: Record<string, { arguments?: string[]; return?: string }>): Record<string, (...args: any[]) => any>;
		registerCallback(sig: { arguments?: string[]; return?: string }, fn: (...args: any[]) => any): bigint;
	};
	toArrayBuffer(ptr: bigint, len: number, copy?: boolean): ArrayBuffer;
	getRawPointer(source: ArrayBufferView): bigint;
};
declare const process: { getBuiltinModule(id: "node:ffi"): Ffi };

const P = "pointer";

export function load(dir: string): Route {
	const ffi = process.getBuiltinModule("node:ffi");
	const moq = new ffi.DynamicLibrary(`${dir}/libmoq.so`);
	const nf = new ffi.DynamicLibrary(`${dir}/libnf.so`);
	const c = moq.getFunctions({
		moq_status_message: { arguments: ["int32"], return: P },
		moq_track_Producer_is_closed: { arguments: [P, P], return: "bool" },
		moq_group_Demand_start: { arguments: [P, P], return: "uint64" },
		moq_track_Producer_info: { arguments: [P, P], return: P },
		moq_track_Info_priority_get: { arguments: [P, P], return: "int8" },
		moq_group_Producer_demand: { arguments: [P, P], return: P },
		moq_Timestamp_from_micros: { arguments: ["uint64", P, P], return: "bool" },
		moq_group_Producer_finish: { arguments: [P, P, P, P], return: "bool" },
		moq_broadcast_Info_new: { return: P },
		moq_broadcast_Info_produce: { arguments: [P, P], return: P },
		moq_track_Info_default: { return: P },
		moq_Timescale_MICRO: { return: "uint64" },
		moq_track_Producer_consume: { arguments: [P, P], return: P },
		moq_track_Consumer_subscribe: { arguments: [P, P, P], return: P },
		moq_track_Subscriber_recv_group: { arguments: [P, P], return: P },
		moq_group_Consumer_read_frame_start: { arguments: [P, P, P, P, P], return: "int32" },
		moq_group_Consumer_read_frame_future_free: { arguments: [P] },
		moq_track_Subscriber_recv_group_future_free: { arguments: [P] },
		moq_track_Consumer_subscribe_future_free: { arguments: [P] },
		moq_broadcast_Info_free: { arguments: [P] },
		moq_broadcast_Producer_free: { arguments: [P] },
		moq_track_Producer_free: { arguments: [P] },
		moq_track_Consumer_free: { arguments: [P] },
		moq_track_Subscriber_free: { arguments: [P] },
		moq_track_Info_free: { arguments: [P] },
		moq_group_Demand_free: { arguments: [P] },
		moq_group_Producer_free: { arguments: [P] },
		moq_group_Finished_free: { arguments: [P] },
		moq_group_Consumer_free: { arguments: [P] },
		moq_frame_Frame_free: { arguments: [P] },
	});
	const t = nf.getFunctions({
		nf_scratch_ptr: { return: P },
		nf_track_Producer_largest: { arguments: [P, P, P], return: "bool" },
		nf_track_Producer_append_group: { arguments: [P, P, P, P, P], return: "bool" },
		nf_group_Producer_write_frame: { arguments: [P, P, P, P, P], return: "bool" },
		nf_frame_Frame_payload_get: { arguments: [P, P, P] },
		nf_buf_free: { arguments: [P] },
		nf_group_Consumer_read_frame_poll: { arguments: [P, P, P, P], return: "int32" },
		nf_track_Subscriber_recv_group_poll: { arguments: [P, P, P, P], return: "int32" },
		nf_track_Consumer_subscribe_poll: { arguments: [P, P, P, P], return: "int32" },
		nf_broadcast_Producer_create_track: { arguments: [P, P, P, P, P, P], return: "bool" },
		nf_track_Info_with_timescale: { arguments: [P, P, P], return: P },
	});

	// The scratch's slots, by byte offset, and their addresses.
	const base = t.nf_scratch_ptr() as bigint;
	const buf = ffi.toArrayBuffer(base, 1 << 16, false);
	const view = new DataView(buf);
	const u8 = new Uint8Array(buf);
	const STATUS = 0, OUT = 8, ERR = 16, FUTURE = 24, AT = 32, ARG = 48, BUF = 80, WAKER = 112, OPTION = 144, OVERFLOW = 176, NAME = 1024;
	const status = base + BigInt(STATUS), out = base + BigInt(OUT), err = base + BigInt(ERR), future = base + BigInt(FUTURE);
	const at_ = base + BigInt(AT), arg = base + BigInt(ARG), bufp = base + BigInt(BUF), waker = base + BigInt(WAKER);
	const option = base + BigInt(OPTION), overflow = base + BigInt(OVERFLOW), name_ = base + BigInt(NAME);

	view.setBigUint64(WAKER, moq.registerCallback({ arguments: [P] }, (data: bigint) => wake(Number(data))), true);
	view.setBigUint64(WAKER + 8, moq.registerCallback({ arguments: [P] }, () => {}), true);

	/** Throws the failed check's message if the status slot holds one. */
	const check = (fn: string) => {
		const s = view.getInt32(STATUS, true);
		if (s !== 0) {
			view.setInt32(STATUS, 0, true);
			const msg = c.moq_status_message(s) as bigint;
			throw new Error(`${fn}: ${new TextDecoder().decode(new Uint8Array(ffi.toArrayBuffer(msg, 64, true))).split("\0")[0]}`);
		}
	};
	const fail = (fn: string): never => {
		throw new Error(`${fn} failed`);
	};
	const ptrAt = (offset: number) => view.getBigUint64(offset, true);
	const own = (p: bigint, free: (p: bigint) => void, name: string) => new Owned(p, free, name);
	const p = (h: Handle) => (h as Owned<bigint>).p;
	const ok = <T>(value: T): Result<T> => ({ ok: true, value });
	const READY = 1, PENDING = 0;

	/** A future's result, polled with `poll` and a waker for its id until it is ready. */
	const future_ = (f: bigint, poll: (f: bigint, w: bigint, o: bigint, e: bigint) => number, done: (f: bigint) => void, wrap: (p: bigint) => Handle) =>
		pend((id) => {
			view.setBigUint64(WAKER + 16, BigInt(id), true);
			const r = poll(f, waker, out, err);
			if (r === PENDING) return undefined;
			done(f);
			if (r !== READY || ptrAt(OUT) === 0n) fail("poll");
			return ok(wrap(ptrAt(OUT)));
		});
	const frame = (f: bigint) => own(f, c.moq_frame_Frame_free!, "FrameFrame");
	const reader = (r: bigint) => own(r, c.moq_group_Consumer_free!, "GroupConsumer");
	const subscriber = (s: bigint) => own(s, c.moq_track_Subscriber_free!, "TrackSubscriber");
	const encoder = new TextEncoder();

	return {
		broadcast: () => {
			const info = c.moq_broadcast_Info_new!();
			const b = c.moq_broadcast_Info_produce!(info, status);
			check("moq_broadcast_Info_produce");
			c.moq_broadcast_Info_free!(info);
			return own(b, c.moq_broadcast_Producer_free!, "BroadcastProducer");
		},
		createTrack: (b, name) => {
			// The micro timescale, as an `Option<Timescale>`: its tag, then the scalar.
			view.setUint8(OPTION, 1);
			view.setBigUint64(OPTION + 8, c.moq_Timescale_MICRO!(), true);
			const defaults = c.moq_track_Info_default!();
			const info = t.nf_track_Info_with_timescale!(defaults, option, status);
			check("moq_track_Info_with_timescale");
			c.moq_track_Info_free!(defaults);
			const { written } = encoder.encodeInto(name, u8.subarray(NAME, NAME + 256));
			view.setBigUint64(ARG, name_, true);
			view.setBigUint64(ARG + 8, BigInt(written), true);
			view.setBigUint64(ARG + 16, 0n, true);
			const r = t.nf_broadcast_Producer_create_track!(p(b), arg, info, out, err, status);
			check("moq_broadcast_Producer_create_track");
			c.moq_track_Info_free!(info);
			if (!r) fail("create_track");
			return ok(own(ptrAt(OUT), c.moq_track_Producer_free!, "TrackProducer"));
		},
		isClosed: (h) => {
			const r = c.moq_track_Producer_is_closed!(p(h), status);
			check("moq_track_Producer_is_closed");
			return r !== 0;
		},
		largest: (h) => {
			const r = t.nf_track_Producer_largest!(p(h), option, status);
			check("moq_track_Producer_largest");
			return r ? u8.slice(OPTION + 8, OPTION + 24) : undefined;
		},
		info: (h) => {
			const r = c.moq_track_Producer_info!(p(h), status);
			check("moq_track_Producer_info");
			return own(r, c.moq_track_Info_free!, "TrackInfo");
		},
		priority: (h) => {
			const r = c.moq_track_Info_priority_get!(p(h), status);
			check("moq_track_Info_priority_get");
			return r;
		},
		demand: (h) => {
			const r = c.moq_group_Producer_demand!(p(h), status);
			check("moq_group_Producer_demand");
			return own(r, c.moq_group_Demand_free!, "GroupDemand");
		},
		demandStart: (h) => {
			const r = c.moq_group_Demand_start!(p(h), status) as bigint;
			check("moq_group_Demand_start");
			return r <= 9007199254740991n ? Number(r) : r;
		},
		fromMicros: (u) => {
			if (!c.moq_Timestamp_from_micros!(u, at_, overflow)) fail("from_micros");
			return ok(u8.slice(AT, AT + 16));
		},
		appendGroup: (h, at) => {
			u8.set(at, AT);
			const r = t.nf_track_Producer_append_group!(p(h), at_, out, err, status);
			check("moq_track_Producer_append_group");
			if (!r) fail("append_group");
			return ok(own(ptrAt(OUT), c.moq_group_Producer_free!, "GroupProducer"));
		},
		writeFrame: (h, at, data) => {
			u8.set(at, AT);
			view.setBigUint64(ARG, ffi.getRawPointer(data), true);
			view.setBigUint64(ARG + 8, BigInt(data.length), true);
			view.setBigUint64(ARG + 16, 0n, true);
			const r = t.nf_group_Producer_write_frame!(p(h), at_, arg, err, status);
			check("moq_group_Producer_write_frame");
			if (!r) fail("write_frame");
			return OK;
		},
		finish: (h) => {
			const r = c.moq_group_Producer_finish!(p(h), out, err, status);
			check("moq_group_Producer_finish");
			if (!r) fail("finish");
			return ok(own(ptrAt(OUT), c.moq_group_Finished_free!, "GroupFinished"));
		},
		consume: (h) => {
			const r = c.moq_track_Producer_consume!(p(h), status);
			check("moq_track_Producer_consume");
			return own(r, c.moq_track_Consumer_free!, "TrackConsumer");
		},
		subscribe: (h) => {
			const f = c.moq_track_Consumer_subscribe!(p(h), null, status);
			check("moq_track_Consumer_subscribe");
			return future_(f, t.nf_track_Consumer_subscribe_poll!, c.moq_track_Consumer_subscribe_future_free!, subscriber);
		},
		recvGroup: (h) => {
			const f = c.moq_track_Subscriber_recv_group!(p(h), status);
			check("moq_track_Subscriber_recv_group");
			return future_(f, t.nf_track_Subscriber_recv_group_poll!, c.moq_track_Subscriber_recv_group_future_free!, reader);
		},
		readFrame: (h) => {
			const r = c.moq_group_Consumer_read_frame_start!(p(h), out, err, future, status);
			check("moq_group_Consumer_read_frame_start");
			if (r === READY) {
				if (ptrAt(OUT) === 0n) fail("read_frame");
				return Promise.resolve(ok(frame(ptrAt(OUT))));
			}
			if (r !== PENDING) fail("read_frame");
			return future_(ptrAt(FUTURE), t.nf_group_Consumer_read_frame_poll!, c.moq_group_Consumer_read_frame_future_free!, frame);
		},
		payload: (h) => {
			t.nf_frame_Frame_payload_get!(p(h), bufp, status);
			check("moq_frame_Frame_payload_get");
			const ptr = ptrAt(BUF), len = Number(ptrAt(BUF + 8));
			if (len <= 64) {
				const copy = new Uint8Array(ffi.toArrayBuffer(ptr, len, true));
				t.nf_buf_free!(bufp);
				return new Lent(copy, undefined);
			}
			const owner = ptrAt(BUF + 16);
			return new Lent(new Uint8Array(ffi.toArrayBuffer(ptr, len, false)), () => {
				view.setBigUint64(ARG, ptr, true);
				view.setBigUint64(ARG + 8, BigInt(len), true);
				view.setBigUint64(ARG + 16, owner, true);
				t.nf_buf_free!(arg);
			});
		},
	};
}
