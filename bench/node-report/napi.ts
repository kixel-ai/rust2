// The Node-API route: addon.c's functions, each handle its address in a number.

import { Lent, OK, Owned, pend, wake, type Handle, type Result, type Route } from "./route.ts";

declare const process: { getBuiltinModule(id: "node:module"): { createRequire(from: string): (id: string) => any } };

export function load(path: string): Route {
	const a = process.getBuiltinModule("node:module").createRequire(import.meta.url)(path);
	a.set_wake(wake);
	const own = (p: number, free: (p: number) => void, name: string) => new Owned(p, free, name);
	const p = (h: Handle) => (h as Owned<number>).p;
	const ok = <T>(value: T): Result<T> => ({ ok: true, value });
	/** A future's result once it is ready, polling it with `poll` until then. */
	const future = (start: number, poll: (f: number, id: number) => number, wrap: (p: number) => Handle) => {
		if (start > 0) return Promise.resolve(ok(wrap(start)));
		const f = -start;
		return pend((id) => {
			const out = poll(f, id);
			return out === 0 ? undefined : ok(wrap(out));
		});
	};
	const frame = (f: number) => own(f, a.frame_free, "FrameFrame");
	const reader = (r: number) => own(r, a.reader_free, "GroupConsumer");
	const subscriber = (s: number) => own(s, a.subscriber_free, "TrackSubscriber");
	return {
		broadcast: () => own(a.broadcast(), a.broadcast_free, "BroadcastProducer"),
		createTrack: (b, name) => ok(own(a.create_track(p(b), name), a.track_free, "TrackProducer")),
		isClosed: (t) => a.is_closed(p(t)),
		largest: (t) => a.largest(p(t)),
		info: (t) => own(a.info(p(t)), a.info_free, "TrackInfo"),
		priority: (i) => a.priority(p(i)),
		demand: (g) => own(a.demand(p(g)), a.demand_free, "GroupDemand"),
		demandStart: (d) => a.demand_start(p(d)),
		fromMicros: (u) => ok(a.from_micros(u)),
		appendGroup: (t, at) => ok(own(a.append_group(p(t), at), a.group_free, "GroupProducer")),
		writeFrame: (g, at, data) => (a.write_frame(p(g), at, data), OK),
		finish: (g) => ok(own(a.finish(p(g)), a.finished_free, "GroupFinished")),
		consume: (t) => own(a.consume(p(t)), a.consumer_free, "TrackConsumer"),
		subscribe: (c) => future(-a.subscribe(p(c)), a.subscribe_poll, subscriber),
		recvGroup: (s) => future(-a.recv_group(p(s)), a.recv_group_poll, reader),
		readFrame: (r) => future(a.read_frame(p(r)), a.read_frame_poll, frame),
		payload: (f) => {
			const view: Uint8Array = a.payload(p(f));
			return new Lent(view, view.length > 64 ? () => a.bytes_free(view) : undefined);
		},
	};
}
