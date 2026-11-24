// A.ts
export type A = {
    b: number;
};
export declare function debug(self: A): string;
export declare function eq(a: A, b: A): boolean;

// Area.ts
import * as rt from "./rt.ts";
import type { Point } from "./Point.ts";
import type { Size } from "./Size.ts";
export type Area = rt.Handle<"Area">;
export declare function debug(self: Area): string;
export declare function clone(self: Area): Area;
export declare function new$(size: Size): Area;
export declare function size(self: Area): Size;
export declare function pixels(self: Area): rt.u64;
export declare function scaledPoint(self: Area, by: Point): rt.u64;
export { new$ as new };

// Array_.ts
export type Array_ = {
    tag: "U8x4";
    _0: number;
};

// Atlas.ts
import type { Point } from "./Point.ts";
export type Atlas = {
    points: ([string, Point])[];
    ids: number[];
};
export declare function debug(self: Atlas): string;
export declare function eq(a: Atlas, b: Atlas): boolean;
export declare function with$(self: Atlas, name: string, at: Point): Atlas;
export { with$ as with };

// B.ts
export type B = {
    tag: "One";
} | {
    tag: "Two";
    _0: number;
};
export declare function debug(self: B): string;
export declare function eq(a: B, b: B): boolean;

// Batch.ts
import * as rt from "./rt.ts";
export type Batch = rt.Handle<"Batch">;
export declare function new$(): Batch;
export declare function push(self: Batch, v: number): boolean;
export declare function capacity(self: Batch): rt.u64;
export { new$ as new };

// Batch2.ts
import * as rt from "./rt.ts";
export type Batch2 = rt.Handle<"Batch2">;
export declare function new$(): Batch2;
export declare function push(self: Batch2, v: number): boolean;
export declare function capacity(self: Batch2): rt.u64;
export { new$ as new };

// Berth.ts
import * as rt from "./rt.ts";
export type Berth = rt.Handle<"Berth">;
export declare function debug(self: Berth): string;
export declare function clone(self: Berth): Berth;
export declare function new$(at: number | undefined): Berth;
export declare function at(self: Berth): number | undefined;
export { new$ as new };

// Board.ts
import * as rt from "./rt.ts";
import type { Counter } from "./Counter.ts";
export type Board = rt.Handle<"Board">;
export declare function new$(counters: ([string, Counter])[]): Board;
export declare function intoCounters(self: Board): ([string, Counter])[];
export declare function ranked(self: Board): ([string, Counter])[];
export declare function best(self: Board): Counter;
export { new$ as new };

// BoxedU16.ts
export type BoxedU16 = {
    value: number;
};
export declare function debug(self: BoxedU16): string;
export declare function eq(a: BoxedU16, b: BoxedU16): boolean;

// BoxedU8.ts
export type BoxedU8 = {
    value: number;
};
export declare function debug(self: BoxedU8): string;
export declare function eq(a: BoxedU8, b: BoxedU8): boolean;

// Buffer.ts
import * as rt from "./rt.ts";
export type Buffer = rt.Handle<"Buffer">;
export declare function new$(len: number): Buffer;
export declare function blank(self: Buffer): void;
export declare function zeroed(self: Buffer): boolean;
export { new$ as new };

// Builtin.ts
import * as rt from "./rt.ts";
export type Builtin = {
    tag: "Any";
    _0: Int32Array;
} | {
    tag: "Boolean";
    _0: rt.f64;
} | {
    tag: "Int";
    _0: rt.f64;
} | {
    tag: "Native";
    _0: rt.f64;
};
export declare function eq(a: Builtin, b: Builtin): boolean;

// C.ts
import * as rt from "./rt.ts";
export type C = rt.Handle<"C">;
export declare function debug(self: C): string;
export declare function eq(a: C, b: C): boolean;
export declare function new$(value: number): C;
export { new$ as new };

// CancellationError.ts
export type CancellationError = {
    code: number;
};

// Cancelled.ts
export type Cancelled = {
    step: number;
};
export declare function debug(self: Cancelled): string;
export declare function eq(a: Cancelled, b: Cancelled): boolean;

// Channel.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Feed } from "./Feed.ts";
export type Channel = rt.Handle<"Channel">;
export declare function clone(self: Channel): Channel;
export declare function default$(): Channel;
export declare function new$(): Channel;
export declare function clear(self: Channel): void;
export declare function send(self: Channel, payload: Uint8Array | rt.Bytes): void;
export declare function recv(self: Channel): rt.Call<rt.Bytes>;
export declare function notify(self: Channel, waker: () => void): void;
export declare function sendLater(self: Channel, payload: Uint8Array | rt.Bytes): rt.Call<void>;
export declare function tagged(self: Channel, tag: string): rt.Call<string>;
export declare function itemized(self: Channel, items: Uint32Array | undefined): rt.Call<rt.u64>;
export declare function pollTake(self: Channel): rt.Call<rt.Bytes>;
export declare function pollMeasure(self: Channel, extra: rt.u64): rt.Call<rt.Result<rt.u64, Error_>>;
export declare function listen(self: Channel, cx: () => void): void;
export declare function recvSized(self: Channel): rt.Call<[Uint8Array, rt.u64]>;
export declare function recvNonempty(self: Channel): rt.Call<rt.Result<rt.Bytes, Error_>>;
export declare function feed(self: Channel): Feed;
export declare function sendFromThread(self: Channel, payload: Uint8Array | rt.Bytes): void;
export { default$ as default, new$ as new };

// Choice.ts
import type { Mark } from "./Mark.ts";
export type Choice = {
    tag: "Empty";
} | {
    tag: "Picked";
    _0: Mark;
};

// Clash.ts
import * as rt from "./rt.ts";
import type { Event } from "./Event.ts";
import type { Moment } from "./Moment.ts";
import type { Point } from "./Point.ts";
import type { Reading } from "./Reading.ts";
import type { Shape } from "./Shape.ts";
import type { Stamp } from "./Stamp.ts";
export type Clash = {
    tag: "Point";
    _0: Point;
} | {
    tag: "Shape";
    _0: Shape;
} | {
    tag: "Event";
    _0: Event;
} | {
    tag: "Double";
    _0: rt.f64;
} | {
    tag: "Float";
    _0: rt.f32;
} | {
    tag: "String";
    _0: string;
} | {
    tag: "Int";
    _0: number;
} | {
    tag: "Boolean";
    _0: boolean;
} | {
    tag: "Native";
    _0: number;
} | {
    tag: "Bytes";
    _0: Uint8Array;
} | {
    tag: "Any";
} | {
    tag: "Unit";
} | {
    tag: "Waker";
} | {
    tag: "Mut";
} | {
    tag: "Pair";
    _0: number;
    _1: number;
} | {
    tag: "Stamp";
    _0: rt.i64;
} | {
    tag: "Moment";
    _0: Moment;
};
export declare function debug(self: Clash): string;
export declare function eq(a: Clash, b: Clash): boolean;
export declare function point(self: Clash): Point | undefined;
export declare function of(value: rt.f64): Clash;
export declare function bytes(self: Clash): rt.Bytes;
export declare function reading(self: Clash, kotlin: number): Reading;
export declare function listed(java: Uint32Array, kotlin: rt.Place<number>): Uint32Array[];
export declare function after(first: Promise<unknown>, kotlin: rt.f64): rt.Call<Clash>;
export declare function joined(self: Clash, lent: Uint8Array | rt.Bytes, owned: Uint8Array | rt.Bytes, maybe: Uint8Array | rt.Bytes | undefined): rt.Bytes;
export declare function tagged(self: Clash, tag: string): rt.Call<string>;
export declare function stamp(self: Clash, kotlin: rt.u64): Stamp;
export declare function moment(self: Clash, kotlin: rt.u64): Moment;
export declare function at(api: Point): Clash;
export declare function describe(self: Clash, text: string, n: number, shape: Shape, out: rt.Place<number>): string;

// ClashAny.ts
import * as rt from "./rt.ts";
export type ClashAny = {
    typeUrl: string;
};
export declare function debug(self: ClashAny): string;
export declare function eq(a: ClashAny, b: ClashAny): boolean;
export declare function hash(self: ClashAny): rt.u64;

// ClashBreak.ts
export type ClashBreak = {
    _0: number;
};
export declare function debug(self: ClashBreak): string;
export declare function eq(a: ClashBreak, b: ClashBreak): boolean;
export declare function SP_(): ClashBreak;
export declare function SP(): ClashBreak;

// ClashCandidate.ts
export type ClashCandidate = {
    tag: "None";
} | {
    tag: "Match";
    _0: number;
};
export declare function debug(self: ClashCandidate): string;
export declare function eq(a: ClashCandidate, b: ClashCandidate): boolean;

// ClashChange.ts
export type ClashChange = {
    tag: "Set";
    new2: number;
    new: number;
} | {
    tag: "Clear";
};
export declare function debug(self: ClashChange): string;
export declare function eq(a: ClashChange, b: ClashChange): boolean;

// ClashCount.ts
export interface ClashCount {
    next(): number;
    close?(): void;
}

// ClashDescribed.ts
export type ClashDescribed = {
    description: number;
    debugDescription: number;
};
export declare function toString(self: ClashDescribed): string;
export declare function debug(self: ClashDescribed): string;
export declare function eq(a: ClashDescribed, b: ClashDescribed): boolean;

// ClashEmptyString.ts
import * as rt from "./rt.ts";
export type ClashEmptyString = rt.Handle<"ClashEmptyString">;
export declare function isEmpty(self: ClashEmptyString): boolean;

// ClashEraser.ts
export interface ClashEraser {
    delete_(new_: number, new_2: number): number;
    delete$(): number;
    close?(): void;
}

// ClashErrno.ts
export type ClashErrno = {
    errno: number;
};
export declare function debug(self: ClashErrno): string;
export declare function eq(a: ClashErrno, b: ClashErrno): boolean;
export declare function NULL(): ClashErrno;

// ClashEscaped.ts
export type ClashEscaped = {
    new: number;
};
export declare function debug(self: ClashEscaped): string;
export declare function eq(a: ClashEscaped, b: ClashEscaped): boolean;
export declare function delete_(self: ClashEscaped, new_2: number, new_: number): number;
export declare function delete$(self: ClashEscaped): number;
export { delete$ as delete };

// ClashLevel.ts
export type ClashLevel = "Default" | "Debug";
export declare function debug(self: ClashLevel): string;
export declare function default$(): ClashLevel;
export declare function index(self: ClashLevel): number;
export { default$ as default };

// ClashMalformed.ts
export type ClashMalformed = {
    at: number;
};
export declare function toString(self: ClashMalformed): string;
export declare function debug(self: ClashMalformed): string;
export declare function eq(a: ClashMalformed, b: ClashMalformed): boolean;
export declare function is(self: ClashMalformed, at: number): boolean;

// ClashNotch.ts
import * as rt from "./rt.ts";
export type ClashNotch = rt.Handle<"ClashNotch">;
export declare function toString(self: ClashNotch): string;
export declare function debug(self: ClashNotch): string;
export declare function clone(self: ClashNotch): ClashNotch;
export declare function eq(a: ClashNotch, b: ClashNotch): boolean;
export declare function hash(self: ClashNotch): rt.u64;
export declare function partialCmp(a: ClashNotch, b: ClashNotch): number | undefined;
export declare function cmp(a: ClashNotch, b: ClashNotch): number;
export declare function default$(): ClashNotch;
export declare function new$(count: number): ClashNotch;
export declare function count(self: ClashNotch): number;
export { default$ as default, new$ as new };

// ClashOp.ts
export type ClashOp = "Delete_" | "Delete";
export declare function debug(self: ClashOp): string;

// ClashReader.ts
import * as rt from "./rt.ts";
import type { ClashMalformed } from "./ClashMalformed.ts";
export type ClashReader = rt.Handle<"ClashReader">;
export declare function new$(bytes: Uint8Array | rt.Bytes): ClashReader;
export declare function readByte(self: ClashReader): rt.Result<number, ClashMalformed>;
export declare function unreadByte(self: ClashReader): rt.Result<undefined, ClashMalformed>;
export declare function wait(self: ClashReader, ms: rt.u64): void;
export { new$ as new };

// ClashSeed.ts
import * as rt from "./rt.ts";
export type ClashSeed = rt.Handle<"ClashSeed">;
export declare function default$(): ClashSeed;
export declare function default_(): ClashSeed;
export declare function value(self: ClashSeed): number;
export declare function bool(self: ClashSeed): boolean;
export { default$ as default };

// ClashShift.ts
export type ClashShift = {
    tag: "Up";
    _0: number;
} | {
    tag: "Down";
} | {
    tag: "Tag";
    _0: number;
};
export declare function debug(self: ClashShift): string;
export declare function eq(a: ClashShift, b: ClashShift): boolean;

// ClashSource.ts
export interface ClashSource {
    wait(): void;
    readByte(): number;
    close?(): void;
}

// ClashSpan.ts
export type ClashSpan = {
    width: number;
};
export declare function toString(self: ClashSpan): string;
export declare function debug(self: ClashSpan): string;
export declare function eq(a: ClashSpan, b: ClashSpan): boolean;
export declare function width(self: ClashSpan): number;
export declare function toString_(self: ClashSpan): string;

// ClashStore.ts
import * as rt from "./rt.ts";
import type { Ticket } from "./Ticket.ts";
export type ClashStore = rt.Handle<"ClashStore">;
export declare function new$(count: number): ClashStore;
export declare function free(self: ClashStore): number;
export declare function close(self: ClashStore): number;
export declare function wait(self: ClashStore): void;
export declare function finalize(self: ClashStore): void;
export declare function take(self: ClashStore, ticket: Ticket): number;
export declare function take_(ticket: Ticket): number;
export declare function count_(): number;
export declare function count(self: ClashStore): number;
export declare function open_(): ClashStore;
export declare function open(self: ClashStore): number;
export { new$ as new };

// ClashTag.ts
export type ClashTag = {
    tag: "Application";
    number: number;
} | {
    tag: "Private";
    number: number;
};
export declare function debug(self: ClashTag): string;
export declare function eq(a: ClashTag, b: ClashTag): boolean;
export declare function number(self: ClashTag): number;

// ClashTone.ts
export type ClashTone = {
    hz: number;
};
export declare function debug(self: ClashTone): string;
export declare function eq(a: ClashTone, b: ClashTone): boolean;

// ClashTone_.ts
export type ClashTone_ = {
    db: number;
};
export declare function debug(self: ClashTone_): string;
export declare function eq(a: ClashTone_, b: ClashTone_): boolean;

// ClashUnderscored.ts
export type ClashUnderscored = {
    new2: number;
    new: number;
    debug: number;
    default: number;
};
export declare function debug(self: ClashUnderscored): string;
export declare function eq(a: ClashUnderscored, b: ClashUnderscored): boolean;
export declare function default$(): ClashUnderscored;
export { default$ as default };

// Clocked.ts
import type { Moment } from "./Moment.ts";
export type Clocked = {
    at: Moment;
    count: number;
};
export declare function debug(self: Clocked): string;
export declare function eq(a: Clocked, b: Clocked): boolean;

// Codecs.ts
import * as rt from "./rt.ts";
import type { TlsCodec } from "./TlsCodec.ts";
export type Codecs = rt.Handle<"Codecs">;
export declare function default$(): Codecs;
export declare function listGet(self: Codecs): TlsCodec[];
export declare function listSet(self: Codecs, value: TlsCodec[]): void;
export declare function new$(): Codecs;
export declare function ids(self: Codecs): number;
export { default$ as default, new$ as new };

// Core.ts
export type Core = {
    id: number;
};

// Countdown.ts
import * as rt from "./rt.ts";
import type { Log } from "./Log.ts";
export type Countdown = rt.Handle<"Countdown">;
export declare function new$(from: number): Countdown;
export declare function told(from: number, log: Log): Countdown;
export declare function next(self: Countdown): rt.Call<number | undefined>;
export declare function values(self: Countdown): rt.Stream<number>;
export { new$ as new };

// Counter.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Label } from "./Label.ts";
import type { Point } from "./Point.ts";
import type { PolarPoint } from "./PolarPoint.ts";
import type { Size } from "./Size.ts";
import type { WrapU32 } from "./WrapU32.ts";
export type Counter = rt.Handle<"Counter">;
export declare function debug(self: Counter): string;
export declare function clone(self: Counter): Counter;
export declare function default$(): Counter;
export declare function new$(name: string, start: rt.u64): Counter;
export declare function get(self: Counter): rt.u64;
export declare function offset(self: Counter, handle: rt.u64, h: rt.u64): rt.u64;
export declare function intoSum(self: Counter, close: rt.u64): rt.u64;
export declare function name(self: Counter): string;
export declare function add(self: Counter, n: rt.u64): rt.Result<rt.u64, Error_>;
export declare function sub(self: Counter, n: rt.u64): rt.Result<rt.u64, Error_>;
export declare function edited(self: Counter): boolean;
export declare function plus(self: Counter, other: Counter | undefined): rt.u64;
export declare function clamped(self: Counter, low: Counter | undefined, high: Counter | undefined): rt.u64;
export declare function plusLater(self: Counter, other: Counter | undefined): rt.Call<rt.u64>;
export declare function addTo(self: Counter, other: Counter | undefined): boolean;
export declare function intoLabel(self: Counter, at: Point | undefined): Label;
export declare function rename(self: Counter, name: string): void;
export declare function split(self: Counter, at: rt.u64): [Counter, Counter];
export declare function is(self: Counter, name: string): boolean;
export declare function seededPoint(name: string, from: Point): Counter;
export declare function seededCounter(name: string, from: Counter): Counter;
export declare function seededSize(name: string, from: Size): Counter;
export declare function seededWrapU32(name: string, from: WrapU32): Counter;
export declare function seededPolarPoint(name: string, from: PolarPoint): Counter;
export { default$ as default, new$ as new };

// Cursor.ts
import * as rt from "./rt.ts";
export type Cursor = rt.Handle<"Cursor">;
export declare function at(self: Cursor): rt.u64;

// Deprecated.ts
export type Deprecated = {
    on: boolean;
    lower: number;
    deprecated: boolean;
};
export declare function debug(self: Deprecated): string;
export declare function eq(a: Deprecated, b: Deprecated): boolean;
export declare function default$(): Deprecated;
export { default$ as default };

// Der.ts
import * as rt from "./rt.ts";
export type Der = rt.Handle<"Der">;
export declare function fromSlice(bytes: Uint8Array | rt.Bytes): Der;
export declare function bytes(self: Der): rt.Bytes;

// Dial.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type Dial = {
    level: rt.u64;
    before: rt.u64;
};
export declare function debug(self: Dial): string;
export declare function eq(a: Dial, b: Dial): boolean;
export declare function default$(): Dial;
export declare function turn(self: Dial, by: rt.u64): rt.Result<Dial, Error_>;
export declare function max(self: Dial): Dial;
export { default$ as default };

// Digest.ts
import * as rt from "./rt.ts";
export type Digest = rt.Handle<"Digest">;
export declare function sumGet(self: Digest): Uint8Array;
export declare function sumSet(self: Digest, value: Uint8Array): void;
export declare function new$(): Digest;
export declare function flipped(self: Digest, mask: Uint8Array): Uint8Array;
export { new$ as new };

// DoubleArray.ts
export type DoubleArray = {
    len: number;
};
export declare function debug(self: DoubleArray): string;
export declare function eq(a: DoubleArray, b: DoubleArray): boolean;

// Draft.ts
import * as rt from "./rt.ts";
export type Draft = rt.Handle<"Draft">;
export declare function edited(self: Draft): boolean;
export declare function describe(self: Draft): string;
export declare function commit(self: Draft): number;
export declare function titleGet(self: Draft): string;
export declare function titleSet(self: Draft, value: string): void;
export declare function lines(self: Draft): number;
export declare function write(self: Draft, line: string): void;

// Epilogue.ts
import * as rt from "./rt.ts";
import type { Log } from "./Log.ts";
export type Epilogue = rt.Handle<"Epilogue">;
export declare function new$(sink: Log, id: number): Epilogue;
export declare function redirect(self: Epilogue, sink: Log): void;
export { new$ as new };

// Error_.ts
import * as rt from "./rt.ts";
export type Error_ = {
    tag: "Underflow";
    by: rt.u64;
} | {
    tag: "Overflow";
};
export declare function toString(self: Error_): string;
export declare function debug(self: Error_): string;
export declare function eq(a: Error_, b: Error_): boolean;

// Event.ts
import type { Point } from "./Point.ts";
export type Event = {
    tag: "Moved";
    _0: Point;
} | {
    tag: "Named";
    name: string;
} | {
    tag: "Done";
};
export declare function debug(self: Event): string;
export declare function eq(a: Event, b: Event): boolean;

// Extent.ts
import * as rt from "./rt.ts";
export type Extent = rt.Opaque<"Extent">;
export declare function debug(self: Extent): string;
export declare function eq(a: Extent, b: Extent): boolean;
export declare function startGet(self: Extent): rt.u64;
export declare function startSet(self: Extent, value: rt.u64): Extent;
export declare function new$(start: rt.u64, len: rt.u64): Extent;
export declare function end(self: Extent): rt.u64;
export declare function slide(self: Extent, by: rt.u64): [Extent, rt.u64];
export { new$ as new };

// Family.ts
export type Family = "Native";
export declare function debug(self: Family): string;

// Fault.ts
import * as rt from "./rt.ts";
export type Fault = rt.Handle<"Fault">;
export declare function toString(self: Fault): string;
export declare function debug(self: Fault): string;

// Feed.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type Feed = rt.Handle<"Feed">;
export declare function next(self: Feed): rt.Call<rt.Result<rt.Bytes | undefined, Error_>>;
export declare function values(self: Feed): rt.Stream<rt.Result<rt.Bytes, Error_>>;

// FiledReceipt.ts
import * as rt from "./rt.ts";
export type FiledReceipt = rt.Handle<"FiledReceipt">;
export declare function clone(self: FiledReceipt): FiledReceipt;
export declare function count(self: FiledReceipt): number;

// Folio.ts
import * as rt from "./rt.ts";
import type { Hasp } from "./Hasp.ts";
export type Folio = rt.Handle<"Folio">;
export declare function pageGet(self: Folio): number;
export declare function pageSet(self: Folio, value: number): void;
export declare function titleGet(self: Folio): string;
export declare function titleSet(self: Folio, value: string): void;
export declare function noteGet(self: Folio): string | undefined;
export declare function noteSet(self: Folio, value: string | undefined): void;
export declare function haspGet(self: Folio): Hasp;
export declare function haspSet(self: Folio, value: Hasp): void;
export declare function dockGet(self: Folio): Hasp;
export declare function new$(page: number, title: string): Folio;
export declare function seed(self: Folio): number;
export { new$ as new };

// Forged.ts
import * as rt from "./rt.ts";
export type Forged = rt.Handle<"Forged">;
export declare function debug(self: Forged): string;
export declare function eq(a: Forged, b: Forged): boolean;
export declare function nGet(self: Forged): rt.u64;
export declare function nSet(self: Forged, value: rt.u64): void;
export declare function new$(n: rt.u64): Forged;
export { new$ as new };

// Gap.ts
import * as rt from "./rt.ts";
export type Gap = rt.Handle<"Gap">;
export declare function debug(self: Gap): string;
export declare function clone(self: Gap): Gap;
export declare function eq(a: Gap, b: Gap): boolean;
export declare function new$(wide: rt.u64, narrow: number): Gap;
export declare function sum(self: Gap): rt.u64;
export { new$ as new };

// Gauge.ts
import * as rt from "./rt.ts";
export type Gauge = rt.Handle<"Gauge">;
export declare function clone(self: Gauge): Gauge;
export declare function new$(level: rt.u64): Gauge;
export declare function set(self: Gauge, level: rt.u64): void;
export declare function level(self: Gauge): rt.u64;
export declare function intoLevel(self: Gauge): rt.u64;
export declare function later(self: Gauge): rt.Call<rt.u64>;
export declare function reaches(self: Gauge, this_: rt.u64): rt.Call<boolean>;
export { new$ as new };

// Guarded.ts
import * as rt from "./rt.ts";
export type Guarded = rt.Handle<"Guarded">;
export declare function new$(n: rt.u64): Guarded;
export declare function nGet(self: Guarded): rt.u64;
export declare function onGet(self: Guarded): boolean;
export declare function count(self: Guarded): rt.u64;
export declare function steps(self: Guarded): rt.u64;
export declare function locals(self: Guarded): number;
export { new$ as new };

// Handle.ts
import * as rt from "./rt.ts";
export type Handle = rt.Handle<"Handle">;
export declare function new$(count: number): Handle;
export declare function count(self: Handle): number;
export declare function later(count: number): rt.Call<Handle>;
export { new$ as new };

// Hasher.ts
export type Hasher = {
    seed: number;
};

// Hasp.ts
import * as rt from "./rt.ts";
export type Hasp = rt.Handle<"Hasp">;
export declare function default$(): Hasp;
export declare function set(self: Hasp): boolean;
export { default$ as default };

// Hits.ts
import * as rt from "./rt.ts";
export type Hits = rt.Opaque<"Hits">;
export declare function debug(self: Hits): string;
export declare function eq(a: Hits, b: Hits): boolean;
export declare function default$(): Hits;
export declare function hitsGet(self: Hits): rt.u64;
export declare function hitsSet(self: Hits, value: rt.u64): Hits;
export declare function missesGet(self: Hits): rt.u64;
export declare function missesSet(self: Hits, value: rt.u64): Hits;
export declare function of(hits: rt.u64, misses: rt.u64): Hits;
export declare function add(self: Hits, other: Hits): Hits;
export declare function eq_(self: Hits, other: Hits): [Hits, boolean];
export { default$ as default };

// Holder.ts
import * as rt from "./rt.ts";
export type Holder = rt.Handle<"Holder">;
export declare function new$(level: rt.u64): Holder;
export declare function levelGet(self: Holder): rt.u64;
export declare function doubled(self: Holder): rt.u64;
export { new$ as new };

// Hooked.ts
import * as rt from "./rt.ts";
export type Hooked = rt.Handle<"Hooked">;
export declare function debug(self: Hooked): string;
export declare function eq(a: Hooked, b: Hooked): boolean;
export declare function nGet(self: Hooked): rt.u64;
export declare function nSet(self: Hooked, value: rt.u64): void;
export declare function new$(n: rt.u64): Hooked;
export declare function count(self: Hooked): rt.u64;
export { new$ as new };

// InnerU8.ts
export type InnerU8 = {
    value: number;
};
export declare function debug(self: InnerU8): string;
export declare function eq(a: InnerU8, b: InnerU8): boolean;

// Instant.ts
import * as rt from "./rt.ts";
export type Instant = rt.Handle<"Instant">;
export declare function debug(self: Instant): string;
export declare function clone(self: Instant): Instant;
export declare function eq(a: Instant, b: Instant): boolean;
export declare function hash(self: Instant): rt.u64;
export declare function partialCmp(a: Instant, b: Instant): number | undefined;
export declare function cmp(a: Instant, b: Instant): number;

// Journal.ts
import * as rt from "./rt.ts";
import type { Log } from "./Log.ts";
export type Journal = rt.Handle<"Journal">;
export declare function new$(sink: Log): Journal;
export declare function redirect(self: Journal, sink: Log): void;
export declare function write(self: Journal, line: string): boolean;
export { new$ as new };

// Json.ts
import * as rt from "./rt.ts";
export type Json = rt.Handle<"Json">;
export declare function parse(input: string): rt.Result<Json, string>;
export declare function toText(self: Json): string;

// JvmField.ts
export type JvmField = {
    n: number;
};
export declare function debug(self: JvmField): string;
export declare function eq(a: JvmField, b: JvmField): boolean;

// JvmStatic.ts
export type JvmStatic = {
    n: number;
};
export declare function debug(self: JvmStatic): string;
export declare function eq(a: JvmStatic, b: JvmStatic): boolean;

// Key.ts
import * as rt from "./rt.ts";
export type Key = rt.Handle<"Key">;
export declare function debug(self: Key): string;
export declare function clone(self: Key): Key;
export declare function eq(a: Key, b: Key): boolean;
export declare function hash(self: Key): rt.u64;
export declare function new$(name: string): Key;
export declare function hashOf(self: Key): rt.u64;
export { new$ as new };

// Knob.ts
import * as rt from "./rt.ts";
export type Knob = {
    level: rt.u64;
};
export declare function debug(self: Knob): string;
export declare function eq(a: Knob, b: Knob): boolean;
export declare function default$(): Knob;
export declare function doubled(self: Knob): rt.u64;
export { default$ as default };

// Label.ts
import * as rt from "./rt.ts";
import type { Point } from "./Point.ts";
export type Label = {
    name: string;
    at: Point | undefined;
    ttl: rt.Duration | undefined;
};
export declare function debug(self: Label): string;
export declare function eq(a: Label, b: Label): boolean;

// Labeled.ts
import * as rt from "./rt.ts";
export type Labeled = rt.Handle<"Labeled">;
export declare function debug(self: Labeled): string;
export declare function eq(a: Labeled, b: Labeled): boolean;
export declare function hash(self: Labeled): rt.u64;
export declare function new$(label: string): Labeled;
export { new$ as new };

// Labelled.ts
export type Labelled = {
    tag: "Tagged";
    tag$: number;
} | {
    tag: "Bare";
};
export declare function debug(self: Labelled): string;
export declare function eq(a: Labelled, b: Labelled): boolean;

// Latch.ts
import * as rt from "./rt.ts";
export type Latch = rt.Handle<"Latch">;
export declare function clone(self: Latch): Latch;
export declare function default$(): Latch;
export declare function set(self: Latch, value: number): boolean;
export declare function get(self: Latch): number | undefined;
export { default$ as default };

// Layered.ts
import * as rt from "./rt.ts";
export type Layered = rt.Handle<"Layered">;
export declare function new$(): Layered;
export declare function titleGet(self: Layered): string;
export declare function lines(self: Layered): number;
export { new$ as new };

// Ledger.ts
import * as rt from "./rt.ts";
import type { Draft } from "./Draft.ts";
import type { Error_ } from "./Error_.ts";
import type { FiledReceipt } from "./FiledReceipt.ts";
import type { Receipt } from "./Receipt.ts";
import type { Slip } from "./Slip.ts";
import type { Tab } from "./Tab.ts";
export type Ledger = rt.Handle<"Ledger">;
export declare function default$(): Ledger;
export declare function new$(): Ledger;
export declare function edit(self: Ledger): rt.Result<Draft, Error_>;
export declare function published(self: Ledger): number;
export declare function title(self: Ledger): string;
export declare function seal(self: Ledger): void;
export declare function receipt(self: Ledger): Receipt;
export declare function noted(self: Ledger): Receipt;
export declare function fileReceipt(self: Ledger, item: Receipt): FiledReceipt;
export declare function tab(self: Ledger): Tab;
export declare function slip(self: Ledger): Slip;
export { default$ as default, new$ as new };

// LeftRightU8.ts
import type { RightU8 } from "./RightU8.ts";
export type LeftRightU8 = {
    value: RightU8;
};
export declare function debug(self: LeftRightU8): string;
export declare function eq(a: LeftRightU8, b: LeftRightU8): boolean;

// LeftU8.ts
export type LeftU8 = {
    value: number;
};
export declare function debug(self: LeftU8): string;
export declare function eq(a: LeftU8, b: LeftU8): boolean;

// Level.ts
export type Level = "Error" | "Warn" | "Info" | "Debug" | "Trace";
export declare function toString(self: Level): string;
export declare function partialCmp(a: Level, b: Level): number | undefined;
export declare function cmp(a: Level, b: Level): number;

// LimitsBytes.ts
export type LimitsBytes = {
    low: Uint8Array;
    high: Uint8Array;
};
export declare function debug(self: LimitsBytes): string;
export declare function eq(a: LimitsBytes, b: LimitsBytes): boolean;

// LimitsDuration.ts
import * as rt from "./rt.ts";
export type LimitsDuration = {
    low: rt.Duration;
    high: rt.Duration;
};
export declare function debug(self: LimitsDuration): string;
export declare function eq(a: LimitsDuration, b: LimitsDuration): boolean;

// Local.ts
import * as rt from "./rt.ts";
export type Local = rt.Handle<"Local">;
export declare function eq(a: Local, b: Local): boolean;
export declare function hash(self: Local): rt.u64;
export declare function default$(): Local;
export declare function new$(): Local;
export declare function bump(self: Local): number;
export declare function share(self: Local): Local;
export declare function later(self: Local): rt.Call<number>;
export declare function visit(self: Local, f: () => number): number;
export declare function pollCount(self: Local): rt.Call<number>;
export declare function threadNumber(self: Local): rt.u64;
export declare function onOwnThread(self: Local): boolean;
export { default$ as default, new$ as new };

// LocalWaiter.ts
import * as rt from "./rt.ts";
export type LocalWaiter = rt.Handle<"LocalWaiter">;
export declare function new$(waker: () => void): LocalWaiter;
export declare function pollReady(n: number): rt.Call<number>;
export { new$ as new };

// Log.ts
export interface Log {
    log(level: number, line: string): boolean;
    close?(): void;
}

// LogCount.ts
export type LogCount = {
    kept: number;
};
export declare function debug(self: LogCount): string;
export declare function eq(a: LogCount, b: LogCount): boolean;

// LogThread.ts
import * as rt from "./rt.ts";
import type { Log } from "./Log.ts";
export type LogThread = rt.Handle<"LogThread">;
export declare function new$(log: Log, line: string): LogThread;
export declare function join(self: LogThread): boolean;
export { new$ as new };

// Logger.ts
import * as rt from "./rt.ts";
import type { Log } from "./Log.ts";
export type Logger = rt.Handle<"Logger">;
export declare function new$(sink: Log): Logger;
export declare function write(self: Logger, line: string): boolean;
export declare function writeFromThread(self: Logger, line: string): boolean;
export { new$ as new };

// LongArray.ts
import * as rt from "./rt.ts";
export type LongArray = rt.Opaque<"LongArray">;
export declare function debug(self: LongArray): string;
export declare function eq(a: LongArray, b: LongArray): boolean;
export declare function new$(low: rt.u64, high: rt.u64): LongArray;
export declare function c(self: LongArray): rt.u64;
export declare function also(self: LongArray, f: (a0: rt.u64) => void): void;
export { new$ as new };

// MacrosStreams.ts
export type MacrosStreams = {
    stdin: number;
    stdout: number;
    stderr: number;
    errno: number;
};
export declare function debug(self: MacrosStreams): string;
export declare function eq(a: MacrosStreams, b: MacrosStreams): boolean;

// Mark.ts
import * as rt from "./rt.ts";
export type Mark = rt.Handle<"Mark">;
export declare function debug(self: Mark): string;
export declare function clone(self: Mark): Mark;
export declare function eq(a: Mark, b: Mark): boolean;
export declare function of(count: rt.u64): Mark;
export declare function count(self: Mark): rt.u64;
export declare function add(self: Mark, by: rt.u64): void;

// Marker.ts
import * as rt from "./rt.ts";
import type { Point } from "./Point.ts";
export type Marker = rt.Handle<"Marker">;
export declare function new$(label: string, x: number, y: number): Marker;
export declare function label(self: Marker): string;
export declare function x(self: Marker): number;
export declare function y(): number;
export declare function xGet(self: Marker): number;
export declare function xSet(self: Marker, value: number): void;
export declare function yGet(self: Marker): number;
export declare function ySet(self: Marker, value: number): void;
export declare function offset(self: Marker, dx: number, dy: number): Point;
export { new$ as new };

// Measure.ts
import * as rt from "./rt.ts";
import type { Numbers } from "./Numbers.ts";
export type Measure = {
    value: rt.f64;
    numbers: Numbers;
};
export declare function debug(self: Measure): string;
export declare function eq(a: Measure, b: Measure): boolean;
export declare function new$(value: rt.f64, negative: boolean): Measure;
export { new$ as new };

// Message.ts
import * as rt from "./rt.ts";
import type { Stamp } from "./Stamp.ts";
export type Message = rt.Handle<"Message">;
export declare function debug(self: Message): string;
export declare function clone(self: Message): Message;
export declare function stampGet(self: Message): Stamp | undefined;
export declare function stampSet(self: Message, value: Stamp | undefined): void;
export declare function payloadGet(self: Message): rt.Bytes;
export declare function payloadSet(self: Message, value: Uint8Array | rt.Bytes): void;
export declare function new$(stamp: Stamp | undefined, payload: Uint8Array | rt.Bytes): Message;
export declare function sent(self: Message): boolean;
export { new$ as new };

// Meter.ts
import * as rt from "./rt.ts";
export type Meter = rt.Handle<"Meter">;
export declare function default$(): Meter;
export declare function add(self: Meter, n: rt.u64): rt.u64;
export declare function count(self: Meter): rt.u64;
export declare function reset(self: Meter): void;
export { default$ as default };

// Meters.ts
import * as rt from "./rt.ts";
import type { Meter } from "./Meter.ts";
export type Meters = rt.Handle<"Meters">;
export declare function default$(): Meters;
export declare function lastGet(self: Meters): Meter;
export declare function lastSet(self: Meters, value: Meter): void;
export declare function spareGet(self: Meters): Meter | undefined;
export declare function spareSet(self: Meters, value: Meter | undefined): void;
export declare function new$(): Meters;
export declare function first(self: Meters): Meter;
export declare function clear(self: Meters): void;
export { default$ as default, new$ as new };

// Moment.ts
import * as rt from "./rt.ts";
export type Moment = rt.Opaque<"Moment">;
export declare function debug(self: Moment): string;
export declare function eq(a: Moment, b: Moment): boolean;
export declare function new$(ticks: rt.u64, rate: rt.u64): Moment | undefined;
export declare function millis(self: Moment): rt.u64;
export declare function after(self: Moment, ticks: rt.u64): Moment;
export { new$ as new };

// Name.ts
import * as rt from "./rt.ts";
export type Name = rt.Handle<"Name">;
export declare function debug(self: Name): string;
export declare function clone(self: Name): Name;
export declare function eq(a: Name, b: Name): boolean;
export declare function owned(text: string): Name;
export declare function asStr(self: Name): string;
export declare function join(self: Name, other: Name): Name;
export declare function borrowed(text: string): Name;
export declare function shares(self: Name, other: Name): boolean;
export declare function rename(self: Name, text: string): void;

// Node.ts
import * as rt from "./rt.ts";
export type Node = rt.Handle<"Node">;
export declare function debug(self: Node): string;
export declare function valueGet(self: Node): rt.u64;
export declare function valueSet(self: Node, value: rt.u64): void;
export declare function nextGet(self: Node): Node | undefined;
export declare function nextSet(self: Node, value: Node | undefined): void;
export declare function new$(value: rt.u64, next: Node | undefined): Node;
export declare function push(self: Node, value: rt.u64): void;
export declare function sum(self: Node): rt.u64;
export { new$ as new };

// NonZeros.ts
import * as rt from "./rt.ts";
export type NonZeros = {
    u8: number;
    u16: number;
    u32: number;
    u64: rt.u64;
    usize: rt.u64;
    i8: number;
    i16: number;
    i32: number;
    i64: rt.i64;
    isize: rt.i64;
};
export declare function debug(self: NonZeros): string;

// Note.ts
export type Note = {
    text: string | undefined;
};
export declare function debug(self: Note): string;
export declare function eq(a: Note, b: Note): boolean;
export declare function default$(): Note;
export { default$ as default };

// Numbers.ts
export type Numbers = {
    u16s: Uint16Array;
    u32s: Uint32Array;
    u64s: BigUint64Array;
    i8s: Int8Array;
    i16s: Int16Array;
    i32s: Int32Array;
    i64s: BigInt64Array;
    f32s: Float32Array;
    f64s: Float64Array;
    some: Int32Array | undefined;
};
export declare function debug(self: Numbers): string;
export declare function eq(a: Numbers, b: Numbers): boolean;
export declare function edges(): Numbers;
export declare function zero(negative: boolean): Numbers;
export declare function reversed(self: Numbers): Numbers;

// OptIn.ts
export type OptIn = {
    n: number;
};
export declare function debug(self: OptIn): string;
export declare function eq(a: OptIn, b: OptIn): boolean;

// OuterInnerU8.ts
import * as rt from "./rt.ts";
export type OuterInnerU8 = rt.Handle<"OuterInnerU8">;
export declare function new$(value: number): OuterInnerU8;
export declare function get(self: OuterInnerU8): number;
export { new$ as new };

// Packaged.ts
import type { Point } from "./Point.ts";
import type { Shape } from "./Shape.ts";
export type Packaged = {
    api: number;
};
export declare function debug(self: Packaged): string;
export declare function eq(a: Packaged, b: Packaged): boolean;
export declare function point(self: Packaged): Point;
export declare function shape(self: Packaged): Shape;

// Packet.ts
export type Packet = {
    seq: number;
    payload: Uint8Array;
};

// PadU16.ts
import * as rt from "./rt.ts";
export type PadU16 = rt.Handle<"PadU16">;
export declare function wide(): PadU16;
export declare function tag(self: PadU16): number;
export declare function title(self: PadU16): number;
export declare function titleGet(self: PadU16): string;
export declare function lines(self: PadU16): number;
export declare function describe(self: PadU16): string;

// PadU8.ts
import * as rt from "./rt.ts";
export type PadU8 = rt.Handle<"PadU8">;
export declare function small(): PadU8;
export declare function describe(self: PadU8): string;
export declare function titleGet(self: PadU8): string;
export declare function lines(self: PadU8): number;

// PairNonZeroU16.ts
export type PairNonZeroU16 = {
    left: number;
    right: number;
};
export declare function debug(self: PairNonZeroU16): string;
export declare function eq(a: PairNonZeroU16, b: PairNonZeroU16): boolean;
export declare function swap(self: PairNonZeroU16): PairNonZeroU16;
export declare function even(self: PairNonZeroU16): boolean;

// PairSizeU8.ts
import type { PairU8Size } from "./PairU8Size.ts";
import type { Size } from "./Size.ts";
export type PairSizeU8 = {
    left: Size;
    right: number;
};
export declare function debug(self: PairSizeU8): string;
export declare function eq(a: PairSizeU8, b: PairSizeU8): boolean;
export declare function swap(self: PairSizeU8): PairU8Size;

// PairU32.ts
import type { PairU8Size } from "./PairU8Size.ts";
import type { Size } from "./Size.ts";
export type PairU32 = {
    left: number;
    right: number;
};
export declare function debug(self: PairU32): string;
export declare function eq(a: PairU32, b: PairU32): boolean;
export declare function swap(self: PairU32): PairU32;
export declare function even(self: PairU32): boolean;
export declare function sum(self: PairU32): number;
export declare function besideSize(self: PairU32, right: Size): PairU8Size;

// PairU8Size.ts
import type { PairSizeU8 } from "./PairSizeU8.ts";
import type { Size } from "./Size.ts";
export type PairU8Size = {
    left: number;
    right: Size;
};
export declare function debug(self: PairU8Size): string;
export declare function eq(a: PairU8Size, b: PairU8Size): boolean;
export declare function swap(self: PairU8Size): PairSizeU8;

// Parcel.ts
import type { Mark } from "./Mark.ts";
export type Parcel = {
    mark: Mark;
    payload: Uint8Array;
};

// Part.ts
import * as rt from "./rt.ts";
export type Part = rt.Handle<"Part">;
export declare function debug(self: Part): string;
export declare function eq(a: Part, b: Part): boolean;
export declare function hash(self: Part): rt.u64;
export declare function valueGet(self: Part): rt.u64;
export declare function valueSet(self: Part, value: rt.u64): void;

// Pass.ts
import * as rt from "./rt.ts";
export type Pass = rt.Handle<"Pass">;
export declare function debug(self: Pass): string;
export declare function eq(a: Pass, b: Pass): boolean;
export declare function partialCmp(a: Pass, b: Pass): number | undefined;
export declare function cmp(a: Pass, b: Pass): number;
export declare function new$(number: number): Pass;
export declare function redeem(self: Pass): number;
export { new$ as new };

// Peer.ts
import * as rt from "./rt.ts";
export type Peer = {
    addr: rt.SocketAddr;
    names: string[];
};
export declare function debug(self: Peer): string;
export declare function eq(a: Peer, b: Peer): boolean;
export declare function aliases(self: Peer): string[];
export declare function next(self: Peer, name: string): Peer;

// Pinned.ts
import * as rt from "./rt.ts";
import type { Part } from "./Part.ts";
export type Pinned = rt.Handle<"Pinned">;
export declare function valueGet(self: Pinned): rt.u64;
export declare function valueSet(self: Pinned, value: rt.u64): void;
export declare function new$(value: rt.u64): Pinned;
export declare function part(self: Pinned): Part;
export { new$ as new };

// Placement.ts
import type { Core } from "./Core.ts";
export type Placement = {
    tag: "Anywhere";
} | {
    tag: "Pinned";
    _0: Core;
};

// Plain.ts
import * as rt from "./rt.ts";
export type Plain = rt.Handle<"Plain">;
export declare function debug(self: Plain): string;
export declare function eq(a: Plain, b: Plain): boolean;
export declare function hash(self: Plain): rt.u64;
export declare function nGet(self: Plain): rt.u64;
export declare function nSet(self: Plain, value: rt.u64): void;
export declare function onGet(self: Plain): boolean;
export declare function onSet(self: Plain, value: boolean): void;
export declare function new$(n: rt.u64, on: boolean): Plain;
export declare function count(self: Plain): rt.u64;
export declare function recount(self: Plain, n: rt.u64): void;
export declare function steps(self: Plain): rt.u64;
export declare function locals(self: Plain): number;
export { new$ as new };

// Platform.ts
export type Platform = {
    tag: "Native";
    _0: number;
};

// Point.ts
export type Point = {
    x: number;
    y: number;
};
export declare function debug(self: Point): string;
export declare function eq(a: Point, b: Point): boolean;
export declare function default$(): Point;
export declare function offset(self: Point, dx: number, dy: number): Point;
export declare function getX(): number;
export { default$ as default };

// PolarPoint.ts
export type PolarPoint = {
    turns: number;
    distance: number;
};
export declare function debug(self: PolarPoint): string;
export declare function eq(a: PolarPoint, b: PolarPoint): boolean;

// PolarTurn.ts
export type PolarTurn = {
    whole: number;
};
export declare function debug(self: PolarTurn): string;
export declare function eq(a: PolarTurn, b: PolarTurn): boolean;

// Polar_Turn.ts
export type Polar_Turn = {
    part: number;
};
export declare function debug(self: Polar_Turn): string;
export declare function eq(a: Polar_Turn, b: Polar_Turn): boolean;

// Qualifiers.ts
import * as rt from "./rt.ts";
export type Qualifiers = {
    api: rt.f64;
    kotlin: rt.f64;
};
export declare function eq(a: Qualifiers, b: Qualifiers): boolean;

// Quantity.ts
import * as rt from "./rt.ts";
export type Quantity = rt.Handle<"Quantity">;
export declare function debug(self: Quantity): string;
export declare function eq(a: Quantity, b: Quantity): boolean;
export declare function partialCmp(a: Quantity, b: Quantity): number | undefined;
export declare function new$(value: rt.i64, unit: number): Quantity;
export { new$ as new };

// Rank.ts
export type Rank = {
    tag: "Unranked";
} | {
    tag: "Ranked";
    compare: number;
    partialCompare: number;
    compareTo: number;
    partialCompareTo: number;
};
export declare function debug(self: Rank): string;
export declare function eq(a: Rank, b: Rank): boolean;
export declare function partialCmp(a: Rank, b: Rank): number | undefined;
export declare function cmp(a: Rank, b: Rank): number;

// Rated.ts
import * as rt from "./rt.ts";
export type Rated = rt.Opaque<"Rated">;
export declare function debug(self: Rated): string;
export declare function eq(a: Rated, b: Rated): boolean;
export declare function new$(count: rt.u64, scale: rt.u64): Rated;
export declare function scale(self: Rated): rt.u64;
export { new$ as new };

// Reading.ts
import * as rt from "./rt.ts";
import type { Point } from "./Point.ts";
import type { Shape } from "./Shape.ts";
export type Reading = {
    ok: boolean;
    level: rt.f32;
    ratio: rt.f64;
    mark: number;
    shape: Shape;
    at: Point;
    note: string;
    count: rt.u64;
    offset: number;
    len: rt.u64;
    tag: number;
};
export declare function debug(self: Reading): string;
export declare function eq(a: Reading, b: Reading): boolean;
export declare function next(self: Reading): Reading | undefined;

// Receipt.ts
import * as rt from "./rt.ts";
export type Receipt = rt.Handle<"Receipt">;
export declare function clone(self: Receipt): Receipt;
export declare function count(self: Receipt): number;

// Refusal.ts
import * as rt from "./rt.ts";
export type Refusal = rt.Handle<"Refusal">;
export declare function toString(self: Refusal): string;
export declare function debug(self: Refusal): string;

// Registry.ts
import * as rt from "./rt.ts";
import type { Counter } from "./Counter.ts";
export type Registry = rt.Handle<"Registry">;
export declare function default$(): Registry;
export declare function firstGet(self: Registry): Counter;
export declare function firstSet(self: Registry, value: Counter): void;
export declare function new$(first: Counter): Registry;
export declare function last(self: Registry): Counter | undefined;
export declare function push(self: Registry, counter: Counter): void;
export declare function edit(self: Registry, edit: (a0: Counter) => boolean): boolean;
export { default$ as default, new$ as new };

// Regs.ts
import * as rt from "./rt.ts";
import type { Point } from "./Point.ts";
export type Regs = rt.Opaque<"Regs">;
export declare function debug(self: Regs): string;
export declare function eq(a: Regs, b: Regs): boolean;
export declare function fcwGet(self: Regs): number;
export declare function fcwSet(self: Regs, value: number): Regs;
export declare function wideGet(self: Regs): rt.u64;
export declare function wideSet(self: Regs, value: rt.u64): Regs;
export declare function atGet(self: Regs): Point;
export declare function atSet(self: Regs, value: Point): Regs;
export declare function tagGet(self: Regs): number;
export declare function tagSet(self: Regs, value: number): Regs;
export declare function new$(fcw: number, wide: rt.u64, at: Point): Regs;
export declare function seed(self: Regs): number;
export { new$ as new };

// Resolving.ts
import * as rt from "./rt.ts";
import type { TlsResolver } from "./TlsResolver.ts";
export type Resolving = rt.Handle<"Resolving">;
export declare function _0_get(self: Resolving): TlsResolver;
export declare function _0_set(self: Resolving, value: TlsResolver): void;
export declare function new$(name: string): Resolving;
export { new$ as new };

// Retired.ts
import * as rt from "./rt.ts";
export type Retired = rt.Handle<"Retired">;
export declare function new$(from: number): Retired;
export declare function next(self: Retired): rt.Call<number | undefined>;
export declare function values(self: Retired): rt.Stream<number>;
export { new$ as new };

// Retry.ts
import * as rt from "./rt.ts";
export type Retry = rt.Handle<"Retry">;
export declare function limitGet(self: Retry): number | undefined;
export declare function limitSet(self: Retry, value: number | undefined): void;
export declare function delayGet(self: Retry): number | undefined;
export declare function delaySet(self: Retry, value: number | undefined): void;
export declare function new$(limit: number | undefined): Retry;
export declare function limit(self: Retry): number;
export declare function delay(self: Retry): number;
export declare function setLimit(self: Retry, limit: number): void;
export { new$ as new };

// RightLeftU8.ts
import type { LeftU8 } from "./LeftU8.ts";
export type RightLeftU8 = {
    value: LeftU8;
};
export declare function debug(self: RightLeftU8): string;
export declare function eq(a: RightLeftU8, b: RightLeftU8): boolean;

// RightU8.ts
export type RightU8 = {
    value: number;
};
export declare function debug(self: RightU8): string;
export declare function eq(a: RightU8, b: RightU8): boolean;

// Scan.ts
import * as rt from "./rt.ts";
export interface Scan {
    scan(payload: Uint8Array, rest: Uint8Array | undefined): rt.u64;
    close?(): void;
}

// Score.ts
import * as rt from "./rt.ts";
import type { Mark } from "./Mark.ts";
export type Score = rt.Opaque<"Score">;
export declare function debug(self: Score): string;
export declare function of(count: rt.u64): Score;
export declare function count(self: Score): rt.u64;
export declare function add(self: Score, other: Score): Score;
export declare function doubled(self: Score): Score;
export declare function take(self: Score): [Score, rt.u64];
export declare function note(self: Score, mark: Mark): [Score, rt.u64];
export declare function absorb(self: Score, other: rt.Place<Score>): [Score, rt.u64, rt.u64];

// Seat.ts
import * as rt from "./rt.ts";
import type { Mark } from "./Mark.ts";
export type Seat = {
    mark: Mark;
    row: number;
};
export declare function debug(self: Seat): string;
export declare function eq(a: Seat, b: Seat): boolean;
export declare function count(self: Seat): rt.u64;

// Sensor.ts
export interface Sensor {
    read(): number;
    interface$(): number;
    close$(): void;
    close?(): void;
}

// Shadowed.ts
export type Shadowed = {
    other: Int32Array;
    a: number;
    b: number;
};
export declare function debug(self: Shadowed): string;
export declare function eq(a: Shadowed, b: Shadowed): boolean;

// Shape.ts
export type Shape = "Circle" | "Square";
export declare function debug(self: Shape): string;
export declare function valueOf(name: string): Shape | undefined;
export declare function ALL(): Shape[];
export declare function BOTH(): Shape[];

// Sheet.ts
import * as rt from "./rt.ts";
export type Sheet = rt.Handle<"Sheet">;
export declare function debug(self: Sheet): string;
export declare function clone(self: Sheet): Sheet;
export declare function default$(): Sheet;
export declare function titleGet(self: Sheet): string;
export declare function titleSet(self: Sheet, value: string): void;
export declare function new$(): Sheet;
export declare function lines(self: Sheet): number;
export declare function write(self: Sheet, line: string): void;
export declare function describe(self: Sheet): string;
export { default$ as default, new$ as new };

// Site.ts
import * as rt from "./rt.ts";
import type { Url } from "./Url.ts";
export type Site = rt.Handle<"Site">;
export declare function homeGet(self: Site): Url;
export declare function homeSet(self: Site, value: Url): void;
export declare function new$(home: Url): Site;
export { new$ as new };

// Size.ts
export type Size = {
    width: number;
    height: number;
};
export declare function debug(self: Size): string;
export declare function eq(a: Size, b: Size): boolean;

// Slip.ts
import * as rt from "./rt.ts";
import type { Receipt } from "./Receipt.ts";
export type Slip = rt.Handle<"Slip">;
export declare function clone(self: Slip): Slip;
export declare function receiptGet(self: Slip): Receipt;
export declare function receiptSet(self: Slip, value: Receipt): void;
export declare function globalGet(self: Slip): Receipt;
export declare function globalSet(self: Slip, value: Receipt): void;
export declare function count(self: Slip): number;

// SlotU32.ts
import * as rt from "./rt.ts";
export type SlotU32 = rt.Handle<"SlotU32">;
export declare function valueGet(self: SlotU32): number;
export declare function valueSet(self: SlotU32, value: number): void;
export declare function new$(value: number): SlotU32;
export { new$ as new };

// SlotU8.ts
import * as rt from "./rt.ts";
export type SlotU8 = rt.Handle<"SlotU8">;
export declare function valueGet(self: SlotU8): number;
export declare function valueSet(self: SlotU8, value: number): void;
export declare function new$(value: number): SlotU8;
export declare function value(self: SlotU8): number;
export { new$ as new };

// Span.ts
import type { Shape } from "./Shape.ts";
export type Span = {
    bounds: [number, number];
    named: [string, Shape] | undefined;
};
export declare function debug(self: Span): string;
export declare function eq(a: Span, b: Span): boolean;

// Spawned.ts
import * as rt from "./rt.ts";
export type Spawned = rt.Handle<"Spawned">;
export declare function new$(task: () => number): Spawned;
export declare function join(self: Spawned): number;
export { new$ as new };

// Spot.ts
import * as rt from "./rt.ts";
import type { Json } from "./Json.ts";
export type Spot = {
    x: number;
    y: number;
};
export declare function debug(self: Spot): string;
export declare function eq(a: Spot, b: Spot): boolean;
export declare function deserialize(deserializer: Json): rt.Result<Spot, string>;

// Stack.ts
import * as rt from "./rt.ts";
import type { Counter } from "./Counter.ts";
import type { Point } from "./Point.ts";
import type { PolarPoint } from "./PolarPoint.ts";
import type { Size } from "./Size.ts";
import type { WrapU32 } from "./WrapU32.ts";
export type Stack = rt.Handle<"Stack">;
export declare function debug(self: Stack): string;
export declare function default$(): Stack;
export declare function new$(): Stack;
export declare function push(self: Stack, item: Counter): void;
export declare function pop(self: Stack): Counter | undefined;
export declare function depth(self: Stack): rt.u64;
export declare function depthPastPoint(self: Stack, from: Point): rt.u64;
export declare function depthPastCounter(self: Stack, from: Counter): rt.u64;
export declare function depthPastSize(self: Stack, from: Size): rt.u64;
export declare function depthPastWrapU32(self: Stack, from: WrapU32): rt.u64;
export declare function depthPastPolarPoint(self: Stack, from: PolarPoint): rt.u64;
export { default$ as default, new$ as new };

// Stage.ts
export type Stage = {
    tag: "Init";
    _0: number;
} | {
    tag: "Done";
};
export declare function debug(self: Stage): string;
export declare function eq(a: Stage, b: Stage): boolean;

// Stamp.ts
import * as rt from "./rt.ts";
export type Stamp = rt.Opaque<"Stamp">;
export declare function debug(self: Stamp): string;
export declare function eq(a: Stamp, b: Stamp): boolean;
export declare function partialCmp(a: Stamp, b: Stamp): number | undefined;
export declare function cmp(a: Stamp, b: Stamp): number;
export declare function default$(): Stamp;
export declare function fromMillis(millis: rt.u64): Stamp;
export declare function secs(self: Stamp): rt.u64;
export declare function whole(self: Stamp): Stamp;
export declare function later(self: Stamp, other: Stamp): boolean;
export { default$ as default };

// Stamped.ts
import * as rt from "./rt.ts";
import type { Stamp } from "./Stamp.ts";
export type Stamped = {
    at: Stamp;
    count: number;
};
export declare function total(self: Stamped): rt.u64;

// StartSeekable.ts
import * as rt from "./rt.ts";
export type StartSeekable = {
    at: rt.u64;
};
export declare function debug(self: StartSeekable): string;
export declare function eq(a: StartSeekable, b: StartSeekable): boolean;

// Status.ts
export type Status = "Moved" | "Kept";
export declare function debug(self: Status): string;

// StdIoError.ts
import * as rt from "./rt.ts";
export type StdIoError = rt.Handle<"StdIoError">;

// Stretch.ts
import * as rt from "./rt.ts";
export interface Stretch {
    stretch(): rt.u64;
    close?(): void;
}

// Tab.ts
import * as rt from "./rt.ts";
import type { Ledger } from "./Ledger.ts";
export type Tab = rt.Handle<"Tab">;
export declare function ledgerGet(self: Tab): Ledger;
export declare function labelGet(self: Tab): string;

// Tag.ts
import * as rt from "./rt.ts";
export type Tag = rt.Handle<"Tag">;
export declare function default$(): Tag;
export declare function new$(): Tag;
export declare function tagged(self: Tag, n: rt.Place<number>, f: () => number): string;
export { default$ as default, new$ as new };

// Tallies.ts
import * as rt from "./rt.ts";
import type { Tally } from "./Tally.ts";
export type Tallies = rt.Handle<"Tallies">;
export declare function default$(): Tallies;
export declare function new$(): Tallies;
export declare function first(self: Tallies): Tally;
export { default$ as default, new$ as new };

// Tally.ts
import * as rt from "./rt.ts";
export type Tally = rt.Handle<"Tally">;
export declare function debug(self: Tally): string;
export declare function eq(a: Tally, b: Tally): boolean;
export declare function default$(): Tally;
export declare function new$(): Tally;
export declare function bump(self: Tally): number;
export declare function bumpLater(self: Tally): rt.Call<number>;
export { default$ as default, new$ as new };

// Ticket.ts
import * as rt from "./rt.ts";
export type Ticket = rt.Opaque<"Ticket">;
export declare function debug(self: Ticket): string;
export declare function eq(a: Ticket, b: Ticket): boolean;
export declare function new$(number: number): Ticket;
export declare function number(self: Ticket): number;
export declare function wait(self: Ticket): void;
export declare function clones(): number;
export { new$ as new };

// Tier.ts
export type Tier = "Low" | "High" | "CompareTo" | "PartialCompareTo";
export declare function debug(self: Tier): string;
export declare function partialCmp(a: Tier, b: Tier): number | undefined;
export declare function cmp(a: Tier, b: Tier): number;

// TlsCert.ts
import * as rt from "./rt.ts";
export type TlsCert = rt.Handle<"TlsCert">;

// TlsCodec.ts
import * as rt from "./rt.ts";
export type TlsCodec = rt.Handle<"TlsCodec">;

// TlsConfig.ts
import * as rt from "./rt.ts";
export type TlsConfig = rt.Handle<"TlsConfig">;
export declare function clone(self: TlsConfig): TlsConfig;

// TlsPolicyLifetime.ts
export type TlsPolicyLifetime = {
    seconds: number;
};
export declare function debug(self: TlsPolicyLifetime): string;
export declare function eq(a: TlsPolicyLifetime, b: TlsPolicyLifetime): boolean;

// TlsResolver.ts
import * as rt from "./rt.ts";
export type TlsResolver = rt.Handle<"TlsResolver">;
export declare function clone(self: TlsResolver): TlsResolver;

// TlsSession.ts
import * as rt from "./rt.ts";
import type { TlsPolicyLifetime } from "./TlsPolicyLifetime.ts";
export type TlsSession = rt.Handle<"TlsSession">;
export declare function alpn(self: TlsSession): string;
export declare function lifetime(self: TlsSession): TlsPolicyLifetime;

// Token.ts
import * as rt from "./rt.ts";
export type Token = rt.Handle<"Token">;
export declare function clone(self: Token): Token;
export declare function idGet(self: Token): number;
export declare function idSet(self: Token, value: number): void;
export declare function new$(id: number): Token;
export declare function twice(self: Token): Token[];
export { new$ as new };

// Trail.ts
import * as rt from "./rt.ts";
import type { Label } from "./Label.ts";
import type { Point } from "./Point.ts";
export type Trail = rt.Handle<"Trail">;
export declare function new$(points: Point[], names: string[]): Trail;
export declare function points(self: Trail): Point[];
export declare function labels(self: Trail): Label[];
export declare function iter(self: Trail): Point[];
export declare function names(self: Trail): string[];
export declare function beyond(self: Trail, x: number): Point[];
export declare function bytes(self: Trail): rt.Bytes;
export declare function drain(self: Trail): Label[];
export { new$ as new };

// Turner.ts
import * as rt from "./rt.ts";
export interface Turner {
    by(): rt.u64;
    once(): rt.u64;
    close?(): void;
}

// Undo.ts
export interface Undo {
    delete$(n: number): number;
    free(): number;
    type(): number;
    close?(): void;
}

// Url.ts
import * as rt from "./rt.ts";
export type Url = rt.Handle<"Url">;
export declare function toString(self: Url): string;
export declare function debug(self: Url): string;
export declare function clone(self: Url): Url;
export declare function eq(a: Url, b: Url): boolean;
export declare function hash(self: Url): rt.u64;
export declare function partialCmp(a: Url, b: Url): number | undefined;
export declare function cmp(a: Url, b: Url): number;
export declare function parse(input: string): rt.Result<Url, string>;

// VecClashTone.ts
export type VecClashTone = "Error";
export declare function debug(self: VecClashTone): string;

// Verbosity.ts
import * as rt from "./rt.ts";
import type { Level } from "./Level.ts";
export type Verbosity = rt.Handle<"Verbosity">;
export declare function levelGet(self: Verbosity): Level;
export declare function levelSet(self: Verbosity, value: Level): void;
export declare function new$(level: Level): Verbosity;
export declare function filter(self: Verbosity): Level | undefined;
export { new$ as new };

// Version.ts
export type Version = {
    major: number;
    minor: number;
};
export declare function debug(self: Version): string;
export declare function eq(a: Version, b: Version): boolean;
export declare function partialCmp(a: Version, b: Version): number | undefined;
export declare function cmp(a: Version, b: Version): number;

// Waiter.ts
import * as rt from "./rt.ts";
export type Waiter = rt.Handle<"Waiter">;
export declare function new$(waker: () => void): Waiter;
export declare function waker(self: Waiter): rt.Waker;
export declare function context(self: Waiter): rt.Waker;
export declare function pollFuture(future: Promise<unknown>): rt.Call<void>;
export { new$ as new };

// Weighted.ts
import * as rt from "./rt.ts";
import type { Score } from "./Score.ts";
export type Weighted = {
    score: Score;
    weight: number;
};
export declare function total(self: Weighted): rt.u64;

// Wide.ts
import * as rt from "./rt.ts";
export type Wide = rt.Opaque<"Wide">;
export declare function debug(self: Wide): string;
export declare function eq(a: Wide, b: Wide): boolean;
export declare function new$(seed: rt.u64): Wide;
export declare function sum(self: Wide): rt.u64;
export { new$ as new };

// Window.ts
import * as rt from "./rt.ts";
export type Window = {
    range: {
        start: rt.u64;
        end: rt.u64;
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
};
export declare function debug(self: Window): string;
export declare function eq(a: Window, b: Window): boolean;
export declare function contains(self: Window, n: rt.u64): boolean;

// WrapU32.ts
export type WrapU32 = {
    value: number;
};
export declare function debug(self: WrapU32): string;
export declare function eq(a: WrapU32, b: WrapU32): boolean;

// Wrapped.ts
import * as rt from "./rt.ts";
export type Wrapped = rt.Handle<"Wrapped">;
export declare function new$(n: rt.u64): Wrapped;
export declare function nGet(self: Wrapped): rt.u64;
export declare function nSet(self: Wrapped, value: rt.u64): void;
export declare function onGet(self: Wrapped): boolean;
export declare function onSet(self: Wrapped, value: boolean): void;
export declare function count(self: Wrapped): rt.u64;
export declare function recount(self: Wrapped, n: rt.u64): void;
export declare function steps(self: Wrapped): rt.u64;
export declare function locals(self: Wrapped): number;
export { new$ as new };

// glue.ts
import * as rt from "./rt.ts";
import type { A } from "./A.ts";
import type { Atlas } from "./Atlas.ts";
import type { B } from "./B.ts";
import type { BoxedU16 } from "./BoxedU16.ts";
import type { BoxedU8 } from "./BoxedU8.ts";
import type { Builtin } from "./Builtin.ts";
import type { Cancelled } from "./Cancelled.ts";
import type { Choice } from "./Choice.ts";
import type { Clash } from "./Clash.ts";
import type { ClashAny } from "./ClashAny.ts";
import type { ClashBreak } from "./ClashBreak.ts";
import type { ClashCandidate } from "./ClashCandidate.ts";
import type { ClashChange } from "./ClashChange.ts";
import type { ClashDescribed } from "./ClashDescribed.ts";
import type { ClashErrno } from "./ClashErrno.ts";
import type { ClashEscaped } from "./ClashEscaped.ts";
import type { ClashLevel } from "./ClashLevel.ts";
import type { ClashMalformed } from "./ClashMalformed.ts";
import type { ClashNotch } from "./ClashNotch.ts";
import type { ClashOp } from "./ClashOp.ts";
import type { ClashShift } from "./ClashShift.ts";
import type { ClashSpan } from "./ClashSpan.ts";
import type { ClashTag } from "./ClashTag.ts";
import type { ClashTone } from "./ClashTone.ts";
import type { ClashTone_ } from "./ClashTone_.ts";
import type { ClashUnderscored } from "./ClashUnderscored.ts";
import type { Clocked } from "./Clocked.ts";
import type { Core } from "./Core.ts";
import type { Counter } from "./Counter.ts";
import type { Cursor } from "./Cursor.ts";
import type { Deprecated } from "./Deprecated.ts";
import type { Dial } from "./Dial.ts";
import type { DoubleArray } from "./DoubleArray.ts";
import type { Error_ } from "./Error_.ts";
import type { Event } from "./Event.ts";
import type { Extent } from "./Extent.ts";
import type { Family } from "./Family.ts";
import type { Gauge } from "./Gauge.ts";
import type { Handle } from "./Handle.ts";
import type { Hits } from "./Hits.ts";
import type { InnerU8 } from "./InnerU8.ts";
import type { Json } from "./Json.ts";
import type { JvmField } from "./JvmField.ts";
import type { JvmStatic } from "./JvmStatic.ts";
import type { Knob } from "./Knob.ts";
import type { Label } from "./Label.ts";
import type { Labelled } from "./Labelled.ts";
import type { LeftRightU8 } from "./LeftRightU8.ts";
import type { LeftU8 } from "./LeftU8.ts";
import type { Level } from "./Level.ts";
import type { LimitsBytes } from "./LimitsBytes.ts";
import type { LimitsDuration } from "./LimitsDuration.ts";
import type { Local } from "./Local.ts";
import type { LogCount } from "./LogCount.ts";
import type { LongArray } from "./LongArray.ts";
import type { MacrosStreams } from "./MacrosStreams.ts";
import type { Mark } from "./Mark.ts";
import type { Measure } from "./Measure.ts";
import type { Moment } from "./Moment.ts";
import type { NonZeros } from "./NonZeros.ts";
import type { Note } from "./Note.ts";
import type { Numbers } from "./Numbers.ts";
import type { OptIn } from "./OptIn.ts";
import type { Packaged } from "./Packaged.ts";
import type { Packet } from "./Packet.ts";
import type { PairNonZeroU16 } from "./PairNonZeroU16.ts";
import type { PairSizeU8 } from "./PairSizeU8.ts";
import type { PairU32 } from "./PairU32.ts";
import type { PairU8Size } from "./PairU8Size.ts";
import type { Parcel } from "./Parcel.ts";
import type { Peer } from "./Peer.ts";
import type { Placement } from "./Placement.ts";
import type { Platform } from "./Platform.ts";
import type { Point } from "./Point.ts";
import type { PolarPoint } from "./PolarPoint.ts";
import type { PolarTurn } from "./PolarTurn.ts";
import type { Polar_Turn } from "./Polar_Turn.ts";
import type { Qualifiers } from "./Qualifiers.ts";
import type { Rank } from "./Rank.ts";
import type { Rated } from "./Rated.ts";
import type { Reading } from "./Reading.ts";
import type { Refusal } from "./Refusal.ts";
import type { Regs } from "./Regs.ts";
import type { RightLeftU8 } from "./RightLeftU8.ts";
import type { RightU8 } from "./RightU8.ts";
import type { Score } from "./Score.ts";
import type { Seat } from "./Seat.ts";
import type { Shadowed } from "./Shadowed.ts";
import type { Shape } from "./Shape.ts";
import type { Size } from "./Size.ts";
import type { Span } from "./Span.ts";
import type { Spot } from "./Spot.ts";
import type { Stage } from "./Stage.ts";
import type { Stamp } from "./Stamp.ts";
import type { Stamped } from "./Stamped.ts";
import type { StartSeekable } from "./StartSeekable.ts";
import type { Status } from "./Status.ts";
import type { Tally } from "./Tally.ts";
import type { Ticket } from "./Ticket.ts";
import type { Tier } from "./Tier.ts";
import type { TlsCert } from "./TlsCert.ts";
import type { TlsCodec } from "./TlsCodec.ts";
import type { TlsPolicyLifetime } from "./TlsPolicyLifetime.ts";
import type { Token } from "./Token.ts";
import type { VecClashTone } from "./VecClashTone.ts";
import type { Version } from "./Version.ts";
import type { Waiter } from "./Waiter.ts";
import type { Weighted } from "./Weighted.ts";
import type { Wide } from "./Wide.ts";
import type { Window } from "./Window.ts";
import type { WrapU32 } from "./WrapU32.ts";
export declare function e_Family(v: Family): number;
export declare function e_Level(v: Level): number;
export declare function e_Shape(v: Shape): number;
export declare function e_Status_2(v: Status): number;
export declare function e_Tier(v: Tier): number;
export declare function e_VecClashTone(v: VecClashTone): number;
export declare function e_clash_Level(v: ClashLevel): number;
export declare function e_clash_Op(v: ClashOp): number;
export declare function f0(): bigint;
export declare function f1(): bigint;
export declare function f10(): bigint;
export declare function f11(): bigint;
export declare function f12(): bigint;
export declare function f13(): bigint;
export declare function f14(): bigint;
export declare function f15(): bigint;
export declare function f16(): bigint;
export declare function f17(): bigint;
export declare function f18(): bigint;
export declare function f19(): bigint;
export declare function f2(): bigint;
export declare function f20(): bigint;
export declare function f21(): bigint;
export declare function f3(): bigint;
export declare function f4(): bigint;
export declare function f5(): bigint;
export declare function f6(): bigint;
export declare function f7(): bigint;
export declare function f8(): bigint;
export declare function f9(): bigint;
export declare function f_Level(c: number): Level;
export declare function f_Shape(c: number): Shape;
export declare function f_clash_Level(c: number): ClashLevel;
export declare const k_Area: rt.Kind;
export declare const k_Batch: rt.Kind;
export declare const k_Batch_2: rt.Kind;
export declare const k_Berth: rt.Kind;
export declare const k_Board: rt.Kind;
export declare const k_Buffer: rt.Kind;
export declare const k_Channel: rt.Kind;
export declare const k_Codecs: rt.Kind;
export declare const k_Countdown: rt.Kind;
export declare const k_Counter: rt.Kind;
export declare const k_Cursor: rt.Kind;
export declare const k_Der: rt.Kind;
export declare const k_Digest: rt.Kind;
export declare const k_Draft: rt.Kind;
export declare const k_Epilogue: rt.Kind;
export declare const k_Fault: rt.Kind;
export declare const k_Feed: rt.Kind;
export declare const k_Filed_Receipt: rt.Kind;
export declare const k_Folio: rt.Kind;
export declare const k_Forged: rt.Kind;
export declare const k_Gap: rt.Kind;
export declare const k_Gauge: rt.Kind;
export declare const k_Guarded: rt.Kind;
export declare const k_Handle: rt.Kind;
export declare const k_Hasp: rt.Kind;
export declare const k_Holder: rt.Kind;
export declare const k_Hooked: rt.Kind;
export declare const k_Instant: rt.Kind;
export declare const k_Journal: rt.Kind;
export declare const k_Json: rt.Kind;
export declare const k_Key: rt.Kind;
export declare const k_Labeled: rt.Kind;
export declare const k_Latch: rt.Kind;
export declare const k_Layered: rt.Kind;
export declare const k_Ledger: rt.Kind;
export declare const k_Local: rt.Kind;
export declare const k_LocalWaiter: rt.Kind;
export declare const k_LogThread: rt.Kind;
export declare const k_Logger: rt.Kind;
export declare const k_Mark: rt.Kind;
export declare const k_Marker: rt.Kind;
export declare const k_Message: rt.Kind;
export declare const k_Meter: rt.Kind;
export declare const k_Meters: rt.Kind;
export declare const k_Name: rt.Kind;
export declare const k_Node: rt.Kind;
export declare const k_Outer_Inner_u8: rt.Kind;
export declare const k_Pad_u16: rt.Kind;
export declare const k_Pad_u8: rt.Kind;
export declare const k_Part: rt.Kind;
export declare const k_Pass: rt.Kind;
export declare const k_Pinned: rt.Kind;
export declare const k_Plain: rt.Kind;
export declare const k_Quantity: rt.Kind;
export declare const k_Receipt: rt.Kind;
export declare const k_Refusal: rt.Kind;
export declare const k_Registry: rt.Kind;
export declare const k_Resolving: rt.Kind;
export declare const k_Retired: rt.Kind;
export declare const k_Retry: rt.Kind;
export declare const k_Sheet: rt.Kind;
export declare const k_Site: rt.Kind;
export declare const k_Slip: rt.Kind;
export declare const k_Slot_u32: rt.Kind;
export declare const k_Slot_u8: rt.Kind;
export declare const k_Spawned: rt.Kind;
export declare const k_Stack: rt.Kind;
export declare const k_Tab: rt.Kind;
export declare const k_Tag: rt.Kind;
export declare const k_Tallies: rt.Kind;
export declare const k_Tally: rt.Kind;
export declare const k_Token: rt.Kind;
export declare const k_Trail: rt.Kind;
export declare const k_Url: rt.Kind;
export declare const k_Verbosity: rt.Kind;
export declare const k_Waiter: rt.Kind;
export declare const k_Wrapped: rt.Kind;
export declare const k_c: rt.Kind;
export declare const k_clash_Empty_string: rt.Kind;
export declare const k_clash_Notch_: rt.Kind;
export declare const k_clash_Reader: rt.Kind;
export declare const k_clash_Seed: rt.Kind;
export declare const k_clash_Store: rt.Kind;
export declare const k_std_io_Error: rt.Kind;
export declare const k_tls_Cert: rt.Kind;
export declare const k_tls_Codec: rt.Kind;
export declare const k_tls_Config: rt.Kind;
export declare const k_tls_Resolver: rt.Kind;
export declare const k_tls_Session: rt.Kind;
export declare function p_Channel_itemized(f: number, w$: number): [boolean, rt.u64];
export declare function p_Channel_poll_measure(f: number, w$: number): [boolean, rt.Result<rt.u64, Error_>];
export declare function p_Channel_poll_take(f: number, w$: number): [boolean, rt.Bytes];
export declare function p_Channel_recv(f: number, w$: number): [boolean, rt.Bytes];
export declare function p_Channel_recv_nonempty(f: number, w$: number): [boolean, rt.Result<rt.Bytes, Error_>];
export declare function p_Channel_recv_sized(f: number, w$: number): [boolean, [Uint8Array, rt.u64]];
export declare function p_Channel_send_later(f: number, w$: number): [boolean, void];
export declare function p_Channel_tagged(f: number, w$: number): [boolean, string];
export declare function p_Clash_after(f: number, w$: number): [boolean, Clash];
export declare function p_Clash_tagged(f: number, w$: number): [boolean, string];
export declare function p_Countdown_next(f: number, w$: number): [boolean, number | undefined];
export declare function p_Counter_plus_later(f: number, w$: number): [boolean, rt.u64];
export declare function p_Feed_next(f: number, w$: number): [boolean, rt.Result<rt.Bytes | undefined, Error_>];
export declare function p_Gauge_later(f: number, w$: number): [boolean, rt.u64];
export declare function p_Gauge_reaches(f: number, w$: number): [boolean, boolean];
export declare function p_Handle_later(f: number, w$: number): [boolean, Handle];
export declare function p_LocalWaiter_poll_ready_homed(f: number, w$: number): [boolean, number];
export declare function p_Local_later(f: number, w$: number): [boolean, number];
export declare function p_Local_poll_count(f: number, w$: number): [boolean, number];
export declare function p_Retired_next(f: number, w$: number): [boolean, number | undefined];
export declare function p_Tally_bump_later_homed(f: number, w$: number): [boolean, number];
export declare function p_Waiter_poll_future(f: number, w$: number): [boolean, void];
export declare function p_after(f: number, w$: number): [boolean, number];
export declare function p_byte_len(f: number, w$: number): [boolean, rt.u64];
export declare function p_corner_later(f: number, w$: number): [boolean, [number, number]];
export declare function p_counted(f: number, w$: number): [boolean, rt.Result<number, Error_>];
export declare function p_drive(f: number, w$: number): [boolean, string];
export declare function p_either_later(f: number, w$: number): [boolean, rt.u64];
export declare function p_emit_later(f: number, w$: number): [boolean, number];
export declare function p_fill_2(f: number, w$: number): [boolean, Uint32Array];
export declare function p_fill_4(f: number, w$: number): [boolean, Uint32Array];
export declare function p_halved(f: number, w$: number): [boolean, number | undefined];
export declare function p_level_later(f: number, w$: number): [boolean, rt.u64];
export declare function p_local_after_homed(f: number, w$: number): [boolean, Local];
export declare function p_local_later_homed(f: number, w$: number): [boolean, number];
export declare function p_local_or_new_later_homed(f: number, w$: number): [boolean, Local];
export declare function p_log_later(f: number, w$: number): [boolean, boolean];
export declare function p_log_on_drop(f: number, w$: number): [boolean, void];
export declare function p_parse_later(f: number, w$: number): [boolean, rt.Result<number, string>];
export declare function p_poll_tripled_homed(f: number, w$: number): [boolean, number];
export declare function p_unescaped_later(f: number, w$: number): [boolean, string];
export declare function p_unescaped_maybe(f: number, w$: number): [boolean, string | undefined];
export declare function p_within(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_yielded(f: number, w$: number): [boolean, number];
export declare function r_Atlas(p: number, o: boolean): Atlas;
export declare function r_Boxed_u16(p: number, o: boolean): BoxedU16;
export declare function r_Boxed_u8(p: number, o: boolean): BoxedU8;
export declare function r_Clash(p: number, o: boolean): Clash;
export declare function r_Clocked(p: number, o: boolean): Clocked;
export declare function r_Core(p: number, o: boolean): Core;
export declare function r_Deprecated(p: number, o: boolean): Deprecated;
export declare function r_Dial(p: number, o: boolean): Dial;
export declare function r_Error(p: number, o: boolean): Error_;
export declare function r_Event(p: number, o: boolean): Event;
export declare function r_Extent(p: number, o: boolean): Extent;
export declare function r_Hits(p: number, o: boolean): Hits;
export declare function r_Knob(p: number, o: boolean): Knob;
export declare function r_Label(p: number, o: boolean): Label;
export declare function r_Labelled(p: number, o: boolean): Labelled;
export declare function r_Limits_bytes(p: number, o: boolean): LimitsBytes;
export declare function r_LongArray(p: number, o: boolean): LongArray;
export declare function r_Measure(p: number, o: boolean): Measure;
export declare function r_Moment(p: number, o: boolean): Moment;
export declare function r_NonZeros(p: number, o: boolean): NonZeros;
export declare function r_Note(p: number, o: boolean): Note;
export declare function r_Numbers(p: number, o: boolean): Numbers;
export declare function r_Packet(p: number, o: boolean): Packet;
export declare function r_Pair_Size_u8(p: number, o: boolean): PairSizeU8;
export declare function r_Pair_non_zero_u16(p: number, o: boolean): PairNonZeroU16;
export declare function r_Pair_u32(p: number, o: boolean): PairU32;
export declare function r_Pair_u8_Size(p: number, o: boolean): PairU8Size;
export declare function r_Peer(p: number, o: boolean): Peer;
export declare function r_Placement(p: number, o: boolean): Placement;
export declare function r_Platform(p: number, o: boolean): Platform;
export declare function r_Point(p: number, o: boolean): Point;
export declare function r_Rated(p: number, o: boolean): Rated;
export declare function r_Reading(p: number, o: boolean): Reading;
export declare function r_Regs(p: number, o: boolean): Regs;
export declare function r_Score(p: number, o: boolean): Score;
export declare function r_Seat(p: number, o: boolean): Seat;
export declare function r_Size(p: number, o: boolean): Size;
export declare function r_Spot(p: number, o: boolean): Spot;
export declare function r_Stage(p: number, o: boolean): Stage;
export declare function r_Stamp(p: number, o: boolean): Stamp;
export declare function r_Ticket(p: number, o: boolean): Ticket;
export declare function r_Wide(p: number, o: boolean): Wide;
export declare function r_bound_u64(p: number, o: boolean): {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
};
export declare function r_bounds_u64(p: number, o: boolean): {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
};
export declare function r_clash_Any(p: number, o: boolean): ClashAny;
export declare function r_clash_Break(p: number, o: boolean): ClashBreak;
export declare function r_clash_Candidate(p: number, o: boolean): ClashCandidate;
export declare function r_clash_Change(p: number, o: boolean): ClashChange;
export declare function r_clash_Errno(p: number, o: boolean): ClashErrno;
export declare function r_clash_Malformed(p: number, o: boolean): ClashMalformed;
export declare function r_clash_Tone(p: number, o: boolean): ClashTone;
export declare function r_clash_Underscored(p: number, o: boolean): ClashUnderscored;
export declare function r_option_Level(p: number, o: boolean): Level | undefined;
export declare function r_option_Moment(p: number, o: boolean): Moment | undefined;
export declare function r_option_Point(p: number, o: boolean): Point | undefined;
export declare function r_option_Reading(p: number, o: boolean): Reading | undefined;
export declare function r_option_Shape(p: number, o: boolean): Shape | undefined;
export declare function r_option_Stamp(p: number, o: boolean): Stamp | undefined;
export declare function r_option_char(p: number, o: boolean): number | undefined;
export declare function r_option_duration(p: number, o: boolean): rt.Duration | undefined;
export declare function r_option_non_zero_u64(p: number, o: boolean): rt.u64 | undefined;
export declare function r_option_polar_Point(p: number, o: boolean): PolarPoint | undefined;
export declare function r_option_tuple_option_u32(p: number, o: boolean): [number | undefined] | undefined;
export declare function r_option_tuple_string(p: number, o: boolean): [string] | undefined;
export declare function r_option_tuple_string_2(p: number, o: boolean): [string] | undefined;
export declare function r_option_tuple_u8_i16_u32_u64(p: number, o: boolean): [number, number, number, rt.u64] | undefined;
export declare function r_option_tuple_usize_usize(p: number, o: boolean): [rt.u64, rt.u64] | undefined;
export declare function r_option_u128(p: number, o: boolean): rt.u128 | undefined;
export declare function r_option_u32(p: number, o: boolean): number | undefined;
export declare function r_option_u64(p: number, o: boolean): rt.u64 | undefined;
export declare function r_polar_Point(p: number, o: boolean): PolarPoint;
export declare function r_range_u64(p: number, o: boolean): {
    start: rt.u64;
    end: rt.u64;
};
export declare function r_tls_policy_Lifetime(p: number, o: boolean): TlsPolicyLifetime;
export declare function r_tuple_bytes(p: number, o: boolean): [Uint8Array];
export declare function r_tuple_bytes_u32(p: number, o: boolean): [Uint8Array, number];
export declare function r_tuple_option_u32(p: number, o: boolean): [number | undefined];
export declare function r_tuple_string(p: number, o: boolean): [string];
export declare function r_tuple_string_2(p: number, o: boolean): [string];
export declare function r_tuple_string_Counter(p: number, o: boolean): [string, Counter];
export declare function r_tuple_string_Counter_2(p: number, o: boolean): [string, Counter];
export declare function r_tuple_string_Json(p: number, o: boolean): [string, Json];
export declare function r_tuple_string_Point(p: number, o: boolean): [string, Point];
export declare function r_tuple_string_Shape_Point(p: number, o: boolean): [string, Shape, Point];
export declare function r_tuple_string_Ticket(p: number, o: boolean): [string, Ticket];
export declare function r_tuple_u32_u32(p: number, o: boolean): [number, number];
export declare function r_tuple_u8_i16_u32_u64(p: number, o: boolean): [number, number, number, rt.u64];
export declare function r_tuple_usize_usize(p: number, o: boolean): [rt.u64, rt.u64];
export declare function r_vec_Counter(p: number, o: boolean): Counter[];
export declare function r_vec_Label(p: number, o: boolean): Label[];
export declare function r_vec_Label_2(p: number, o: boolean): Label[];
export declare function r_vec_Point(p: number, o: boolean): Point[];
export declare function r_vec_Point_2(p: number, o: boolean): Point[];
export declare function r_vec_Shape(p: number, o: boolean): Shape[];
export declare function r_vec_Shape_2(p: number, o: boolean): Shape[];
export declare function r_vec_Token(p: number, o: boolean): Token[];
export declare function r_vec_bytes(p: number, o: boolean): Uint8Array[];
export declare function r_vec_clash_Notch_(p: number, o: boolean): ClashNotch[];
export declare function r_vec_clash_Tone(p: number, o: boolean): ClashTone[];
export declare function r_vec_f32(p: number, o: boolean): Float32Array;
export declare function r_vec_f64(p: number, o: boolean): Float64Array;
export declare function r_vec_f64_2(p: number, o: boolean): Float64Array;
export declare function r_vec_i16(p: number, o: boolean): Int16Array;
export declare function r_vec_i32(p: number, o: boolean): Int32Array;
export declare function r_vec_i64(p: number, o: boolean): BigInt64Array;
export declare function r_vec_i8(p: number, o: boolean): Int8Array;
export declare function r_vec_option_path(p: number, o: boolean): (rt.PathBuf | undefined)[];
export declare function r_vec_string(p: number, o: boolean): string[];
export declare function r_vec_string_2(p: number, o: boolean): string[];
export declare function r_vec_string_3(p: number, o: boolean): string[];
export declare function r_vec_tls_Codec(p: number, o: boolean): TlsCodec[];
export declare function r_vec_tuple_string_Counter(p: number, o: boolean): ([string, Counter])[];
export declare function r_vec_tuple_string_Counter_2(p: number, o: boolean): ([string, Counter])[];
export declare function r_vec_tuple_string_Json(p: number, o: boolean): ([string, Json])[];
export declare function r_vec_tuple_string_Json_2(p: number, o: boolean): ([string, Json])[];
export declare function r_vec_tuple_string_Point(p: number, o: boolean): ([string, Point])[];
export declare function r_vec_tuple_string_Shape_Point(p: number, o: boolean): ([string, Shape, Point])[];
export declare function r_vec_tuple_string_Ticket(p: number, o: boolean): ([string, Ticket])[];
export declare function r_vec_u16(p: number, o: boolean): Uint16Array;
export declare function r_vec_u32(p: number, o: boolean): number[];
export declare function r_vec_u32_2(p: number, o: boolean): Uint32Array;
export declare function r_vec_u32_3(p: number, o: boolean): Uint32Array;
export declare function r_vec_u32_4(p: number, o: boolean): Uint32Array;
export declare function r_vec_u32_5(p: number, o: boolean): Uint32Array;
export declare function r_vec_u64(p: number, o: boolean): BigUint64Array;
export declare function r_vec_vec_u32(p: number, o: boolean): Uint32Array[];
export declare function r_vec_vec_u32_2(p: number, o: boolean): number[][];
export declare function t0(f: (a0: rt.u64) => void, p0: rt.u64): void;
export declare function t1(f: () => number): number;
export declare function t10(f: (a0: Moment) => Moment, r$: number, p0: number): void;
export declare function t11(f: (a0: number) => void, p0: number): void;
export declare function t12(f: (a0: number, a1: string) => boolean, p0: number, p1: number): number;
export declare function t13(f: (a0: Gauge) => rt.u64, p0: number): rt.u64;
export declare function t14(f: (a0: Cursor) => rt.u64, p0: number): rt.u64;
export declare function t15(f: (a0: Tally) => number, p0: number): number;
export declare function t16(f: (a0: number) => number, p0: number): number;
export declare function t17(f: (a0: number) => number, p0: number): number;
export declare function t18(f: (a0: rt.PathBuf) => void, p0: number): void;
export declare function t19(f: (a0: Uint8Array | undefined) => rt.u64, p0: number): rt.u64;
export declare function t2(f: () => number): number;
export declare function t20(f: () => number): number;
export declare function t21(f: () => rt.u64): rt.u64;
export declare function t22(f: () => rt.i64): rt.i64;
export declare function t23(f: () => void): void;
export declare function t3(f: (a0: Counter) => boolean, p0: number): number;
export declare function t4(f: () => number): number;
export declare function t5(f: (a0: Moment, a1: Waiter) => rt.Result<Moment | undefined, string>, p0: number, p1: number, p2: number, p3: number): number;
export declare function t6(f: (a0: number) => string, r$: number, p0: number): void;
export declare function t7(f: (a0: Ticket) => Ticket, p0: number): number;
export declare function t8(f: (a0: Ticket) => Ticket, p0: number): number;
export declare function t9(f: () => rt.Result<undefined, string>, p0: number): number;
export declare function t_Moment(p: number, v: Moment): void;
export declare function t_Ticket(p: number, v: Ticket): void;
export declare function t_option_Moment(p: number, v: Moment | undefined): void;
export declare function vt_Log(): bigint;
export declare function vt_Scan(): bigint;
export declare function vt_Sensor(): bigint;
export declare function vt_Stretch(): bigint;
export declare function vt_Turner(): bigint;
export declare function vt_Undo(): bigint;
export declare function vt_clash_Count_(): bigint;
export declare function vt_clash_Eraser(): bigint;
export declare function vt_clash_Source(): bigint;
export declare function w_Atlas(p: number, v: Atlas): void;
export declare function w_Boxed_u16(p: number, v: BoxedU16): void;
export declare function w_Boxed_u8(p: number, v: BoxedU8): void;
export declare function w_Builtin(p: number, v: Builtin): void;
export declare function w_Cancelled(p: number, v: Cancelled): void;
export declare function w_Choice(p: number, v: Choice): void;
export declare function w_Clash(p: number, v: Clash): void;
export declare function w_Clocked(p: number, v: Clocked): void;
export declare function w_Deprecated(p: number, v: Deprecated): void;
export declare function w_Dial(p: number, v: Dial): void;
export declare function w_DoubleArray(p: number, v: DoubleArray): void;
export declare function w_Error(p: number, v: Error_): void;
export declare function w_Event(p: number, v: Event): void;
export declare function w_Extent(p: number, v: Extent): void;
export declare function w_Hits(p: number, v: Hits): void;
export declare function w_Inner_u8(p: number, v: InnerU8): void;
export declare function w_JvmField(p: number, v: JvmField): void;
export declare function w_JvmStatic(p: number, v: JvmStatic): void;
export declare function w_Knob(p: number, v: Knob): void;
export declare function w_Label(p: number, v: Label): void;
export declare function w_Labelled(p: number, v: Labelled): void;
export declare function w_Left_Right_u8(p: number, v: LeftRightU8): void;
export declare function w_Left_u8(p: number, v: LeftU8): void;
export declare function w_Limits_bytes(p: number, v: LimitsBytes): void;
export declare function w_Limits_duration(p: number, v: LimitsDuration): void;
export declare function w_LogCount(p: number, v: LogCount): void;
export declare function w_LongArray(p: number, v: LongArray): void;
export declare function w_Measure(p: number, v: Measure): void;
export declare function w_Moment(p: number, v: Moment): void;
export declare function w_NonZeros(p: number, v: NonZeros): void;
export declare function w_Note(p: number, v: Note): void;
export declare function w_Numbers(p: number, v: Numbers): void;
export declare function w_OptIn(p: number, v: OptIn): void;
export declare function w_Packaged(p: number, v: Packaged): void;
export declare function w_Packet(p: number, v: Packet): void;
export declare function w_Pair_Size_u8(p: number, v: PairSizeU8): void;
export declare function w_Pair_non_zero_u16(p: number, v: PairNonZeroU16): void;
export declare function w_Pair_u32(p: number, v: PairU32): void;
export declare function w_Pair_u8_Size(p: number, v: PairU8Size): void;
export declare function w_Parcel(p: number, v: Parcel): void;
export declare function w_Peer(p: number, v: Peer): void;
export declare function w_Platform(p: number, v: Platform): void;
export declare function w_Point(p: number, v: Point): void;
export declare function w_PolarTurn(p: number, v: PolarTurn): void;
export declare function w_Qualifiers(p: number, v: Qualifiers): void;
export declare function w_Rank(p: number, v: Rank): void;
export declare function w_Rated(p: number, v: Rated): void;
export declare function w_Reading(p: number, v: Reading): void;
export declare function w_Regs(p: number, v: Regs): void;
export declare function w_Right_Left_u8(p: number, v: RightLeftU8): void;
export declare function w_Right_u8(p: number, v: RightU8): void;
export declare function w_Score(p: number, v: Score): void;
export declare function w_Seat(p: number, v: Seat): void;
export declare function w_Shadowed(p: number, v: Shadowed): void;
export declare function w_Size(p: number, v: Size): void;
export declare function w_Span(p: number, v: Span): void;
export declare function w_Spot(p: number, v: Spot): void;
export declare function w_Stage(p: number, v: Stage): void;
export declare function w_Stamp(p: number, v: Stamp): void;
export declare function w_Stamped(p: number, v: Stamped): void;
export declare function w_StartSeekable(p: number, v: StartSeekable): void;
export declare function w_Ticket(p: number, v: Ticket): void;
export declare function w_Version(p: number, v: Version): void;
export declare function w_Weighted(p: number, v: Weighted): void;
export declare function w_Wide(p: number, v: Wide): void;
export declare function w_Window(p: number, v: Window): void;
export declare function w_Wrap_u32(p: number, v: WrapU32): void;
export declare function w_a(p: number, v: A): void;
export declare function w_b(p: number, v: B): void;
export declare function w_bound_u64(p: number, v: {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
}): void;
export declare function w_bounds_u64(p: number, v: {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
}): void;
export declare function w_clash_Any(p: number, v: ClashAny): void;
export declare function w_clash_Break(p: number, v: ClashBreak): void;
export declare function w_clash_Candidate(p: number, v: ClashCandidate): void;
export declare function w_clash_Change(p: number, v: ClashChange): void;
export declare function w_clash_Described(p: number, v: ClashDescribed): void;
export declare function w_clash_Errno(p: number, v: ClashErrno): void;
export declare function w_clash_Escaped(p: number, v: ClashEscaped): void;
export declare function w_clash_Malformed(p: number, v: ClashMalformed): void;
export declare function w_clash_Shift_(p: number, v: ClashShift): void;
export declare function w_clash_Span(p: number, v: ClashSpan): void;
export declare function w_clash_TONE(p: number, v: ClashTone_): void;
export declare function w_clash_Tag(p: number, v: ClashTag): void;
export declare function w_clash_Tone(p: number, v: ClashTone): void;
export declare function w_clash_Underscored(p: number, v: ClashUnderscored): void;
export declare function w_macros_Streams(p: number, v: MacrosStreams): void;
export declare function w_option_Level(p: number, v: Level | undefined): void;
export declare function w_option_Point(p: number, v: Point | undefined): void;
export declare function w_option_Point_2(p: number, v: Point | undefined): void;
export declare function w_option_Reading(p: number, v: Reading | undefined): void;
export declare function w_option_Stamp(p: number, v: Stamp | undefined): void;
export declare function w_option_duration(p: number, v: rt.Duration | undefined): void;
export declare function w_option_non_zero_u64(p: number, v: rt.u64 | undefined): void;
export declare function w_option_tuple_Counter(p: number, v: [Counter] | undefined): void;
export declare function w_option_tuple_Counter_Refusal(p: number, v: [Counter, Refusal] | undefined): void;
export declare function w_option_tuple_option_u32(p: number, v: [number | undefined] | undefined): void;
export declare function w_option_tuple_string_Shape(p: number, v: [string, Shape] | undefined): void;
export declare function w_option_tuple_tuple_u8_u16_u32(p: number, v: [[number, number], number] | undefined): void;
export declare function w_option_tuple_tuple_u8_u16_u32_(p: number, v: [[number, number, number]] | undefined): void;
export declare function w_option_u32(p: number, v: number | undefined): void;
export declare function w_polar_Point(p: number, v: PolarPoint): void;
export declare function w_polar_Turn(p: number, v: Polar_Turn): void;
export declare function w_range_u64(p: number, v: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function w_tls_policy_Lifetime(p: number, v: TlsPolicyLifetime): void;
export declare function w_tuple_Counter(p: number, v: [Counter]): void;
export declare function w_tuple_Counter_Mark_Seat_Choice(p: number, v: [Counter, Mark, Seat, Choice]): void;
export declare function w_tuple_Counter_Refusal(p: number, v: [Counter, Refusal]): void;
export declare function w_tuple_Counter_bytes(p: number, v: [Counter, Uint8Array]): void;
export declare function w_tuple_Mark_bytes(p: number, v: [Mark, Uint8Array]): void;
export declare function w_tuple_bytes(p: number, v: [Uint8Array]): void;
export declare function w_tuple_bytes_Refusal(p: number, v: [Uint8Array, Refusal]): void;
export declare function w_tuple_bytes_u32(p: number, v: [Uint8Array, number]): void;
export declare function w_tuple_option_u32(p: number, v: [number | undefined]): void;
export declare function w_tuple_string_Counter(p: number, v: [string, Counter]): void;
export declare function w_tuple_string_Json(p: number, v: [string, Json]): void;
export declare function w_tuple_string_Point(p: number, v: [string, Point]): void;
export declare function w_tuple_string_Shape(p: number, v: [string, Shape]): void;
export declare function w_tuple_string_Ticket(p: number, v: [string, Ticket]): void;
export declare function w_tuple_string_u32(p: number, v: [string, number]): void;
export declare function w_tuple_tuple_u8_u16_u32(p: number, v: [[number, number], number]): void;
export declare function w_tuple_tuple_u8_u16_u32_(p: number, v: [[number, number, number]]): void;
export declare function w_tuple_u32_u32(p: number, v: [number, number]): void;
export declare function w_tuple_u8_u16(p: number, v: [number, number]): void;
export declare function w_tuple_u8_u16_u32(p: number, v: [number, number, number]): void;
export declare function w_vec_Counter(p: number, v: Counter[] | undefined): void;
export declare function w_vec_Counter_2(p: number, v: Counter[] | undefined): void;
export declare function w_vec_Point(p: number, v: Point[] | undefined): void;
export declare function w_vec_Point_2(p: number, v: Point[] | undefined): void;
export declare function w_vec_Refusal(p: number, v: Refusal[] | undefined): void;
export declare function w_vec_bytes(p: number, v: Uint8Array[] | undefined): void;
export declare function w_vec_bytes_2(p: number, v: Uint8Array[] | undefined): void;
export declare function w_vec_f32(p: number, v: Float32Array | undefined): void;
export declare function w_vec_f64(p: number, v: Float64Array | undefined): void;
export declare function w_vec_f64_2(p: number, v: Float64Array | undefined): void;
export declare function w_vec_i16(p: number, v: Int16Array | undefined): void;
export declare function w_vec_i32(p: number, v: Int32Array | undefined): void;
export declare function w_vec_i32_2(p: number, v: Int32Array | undefined): void;
export declare function w_vec_i64(p: number, v: BigInt64Array | undefined): void;
export declare function w_vec_i64_2(p: number, v: BigInt64Array | undefined): void;
export declare function w_vec_i8(p: number, v: Int8Array | undefined): void;
export declare function w_vec_path(p: number, v: rt.PathBuf[] | undefined): void;
export declare function w_vec_socket_addr(p: number, v: rt.SocketAddr[] | undefined): void;
export declare function w_vec_string(p: number, v: string[] | undefined): void;
export declare function w_vec_string_2(p: number, v: string[] | undefined): void;
export declare function w_vec_tls_Cert(p: number, v: TlsCert[] | undefined): void;
export declare function w_vec_tls_Codec(p: number, v: TlsCodec[] | undefined): void;
export declare function w_vec_tuple_Counter_Refusal(p: number, v: ([Counter, Refusal])[] | undefined): void;
export declare function w_vec_tuple_string_Counter(p: number, v: ([string, Counter])[] | undefined): void;
export declare function w_vec_tuple_string_Json(p: number, v: ([string, Json])[] | undefined): void;
export declare function w_vec_tuple_string_Point(p: number, v: ([string, Point])[] | undefined): void;
export declare function w_vec_tuple_string_Ticket(p: number, v: ([string, Ticket])[] | undefined): void;
export declare function w_vec_u16(p: number, v: Uint16Array | undefined): void;
export declare function w_vec_u32(p: number, v: number[] | undefined): void;
export declare function w_vec_u32_2(p: number, v: Uint32Array | undefined): void;
export declare function w_vec_u32_3(p: number, v: Uint32Array | undefined): void;
export declare function w_vec_u32_4(p: number, v: Uint32Array | undefined): void;
export declare function w_vec_u32_5(p: number, v: number[] | undefined): void;
export declare function w_vec_u64(p: number, v: BigUint64Array | undefined): void;
export declare function w_vec_vec_u32(p: number, v: Uint32Array[] | undefined): void;
export declare function w_vec_vec_u32_2(p: number, v: number[][] | undefined): void;
export declare const imports: {
    api_node_cb_Log_log: (data: number, p0: number, p1: number) => number;
    api_node_cb_Scan_scan: (data: number, p0: number, p1: number) => rt.u64;
    api_node_cb_Sensor_close: (data: number) => void;
    api_node_cb_Sensor_interface: (data: number) => number;
    api_node_cb_Sensor_read: (data: number) => number;
    api_node_cb_Stretch_stretch: (data: number) => rt.u64;
    api_node_cb_Turner_by: (data: number) => rt.u64;
    api_node_cb_Turner_once: (data: number) => rt.u64;
    api_node_cb_Undo_delete: (data: number, p0: number) => number;
    api_node_cb_Undo_free: (data: number) => number;
    api_node_cb_Undo_type: (data: number) => number;
    api_node_cb_clash_Count__next: (data: number) => number;
    api_node_cb_clash_Eraser_delete: (data: number) => number;
    api_node_cb_clash_Eraser_delete_: (data: number, p0: number, p1: number) => number;
    api_node_cb_clash_Source_read_byte: (data: number) => number;
    api_node_cb_clash_Source_wait: (data: number) => void;
    api_node_i_fn_0: (data: number, p0: rt.u64) => void;
    api_node_i_fn_1: (data: number) => number;
    api_node_i_fn_10: (data: number, p0: number, p1: number) => number;
    api_node_i_fn_11: (data: number, p0: number) => rt.u64;
    api_node_i_fn_12: (data: number, p0: number) => rt.u64;
    api_node_i_fn_13: (data: number, p0: number) => number;
    api_node_i_fn_14: (data: number, p0: number) => number;
    api_node_i_fn_15: (data: number, p0: number) => number;
    api_node_i_fn_16: (data: number, p0: number) => void;
    api_node_i_fn_17: (data: number, p0: number) => rt.u64;
    api_node_i_fn_18: (data: number) => number;
    api_node_i_fn_19: (data: number) => rt.u64;
    api_node_i_fn_2: (data: number) => number;
    api_node_i_fn_20: (data: number) => rt.i64;
    api_node_i_fn_21: (data: number) => void;
    api_node_i_fn_3: (data: number, p0: number) => number;
    api_node_i_fn_4: (data: number, p0: number, p1: number, p2: number, p3: number) => number;
    api_node_i_fn_5: (r$: number, data: number, p0: number) => void;
    api_node_i_fn_6: (data: number, p0: number) => number;
    api_node_i_fn_7: (data: number, p0: number) => number;
    api_node_i_fn_8: (r$: number, data: number, p0: number) => void;
    api_node_i_fn_9: (data: number, p0: number) => void;
};

// index.ts
import * as rt from "./rt.ts";
import type { Area } from "./Area.ts";
import type { Batch } from "./Batch.ts";
import type { Batch2 } from "./Batch2.ts";
import type { BoxedU16 } from "./BoxedU16.ts";
import type { BoxedU8 } from "./BoxedU8.ts";
import type { Choice } from "./Choice.ts";
import type { ClashAny } from "./ClashAny.ts";
import type { ClashCandidate } from "./ClashCandidate.ts";
import type { ClashChange } from "./ClashChange.ts";
import type { ClashCount } from "./ClashCount.ts";
import type { ClashDescribed } from "./ClashDescribed.ts";
import type { ClashEmptyString } from "./ClashEmptyString.ts";
import type { ClashEraser } from "./ClashEraser.ts";
import type { ClashNotch } from "./ClashNotch.ts";
import type { ClashOp } from "./ClashOp.ts";
import type { ClashShift } from "./ClashShift.ts";
import type { ClashSource } from "./ClashSource.ts";
import type { ClashStore } from "./ClashStore.ts";
import type { ClashTone } from "./ClashTone.ts";
import type { ClashUnderscored } from "./ClashUnderscored.ts";
import type { Clocked } from "./Clocked.ts";
import type { Counter } from "./Counter.ts";
import type { Cursor } from "./Cursor.ts";
import type { Dial } from "./Dial.ts";
import type { Draft } from "./Draft.ts";
import type { Error_ } from "./Error_.ts";
import type { Event } from "./Event.ts";
import type { Family } from "./Family.ts";
import type { Fault } from "./Fault.ts";
import type { Gauge } from "./Gauge.ts";
import type { Instant } from "./Instant.ts";
import type { Json } from "./Json.ts";
import type { JvmField } from "./JvmField.ts";
import type { JvmStatic } from "./JvmStatic.ts";
import type { Knob } from "./Knob.ts";
import type { Label } from "./Label.ts";
import type { Labelled } from "./Labelled.ts";
import type { LeftRightU8 } from "./LeftRightU8.ts";
import type { Level } from "./Level.ts";
import type { LimitsBytes } from "./LimitsBytes.ts";
import type { LimitsDuration } from "./LimitsDuration.ts";
import type { Local } from "./Local.ts";
import type { Log } from "./Log.ts";
import type { MacrosStreams } from "./MacrosStreams.ts";
import type { Mark } from "./Mark.ts";
import type { Moment } from "./Moment.ts";
import type { Name } from "./Name.ts";
import type { NonZeros } from "./NonZeros.ts";
import type { Note } from "./Note.ts";
import type { OptIn } from "./OptIn.ts";
import type { Packet } from "./Packet.ts";
import type { PairNonZeroU16 } from "./PairNonZeroU16.ts";
import type { PairSizeU8 } from "./PairSizeU8.ts";
import type { PairU32 } from "./PairU32.ts";
import type { Parcel } from "./Parcel.ts";
import type { Placement } from "./Placement.ts";
import type { Platform } from "./Platform.ts";
import type { Point } from "./Point.ts";
import type { PolarPoint } from "./PolarPoint.ts";
import type { PolarTurn } from "./PolarTurn.ts";
import type { Polar_Turn } from "./Polar_Turn.ts";
import type { Reading } from "./Reading.ts";
import type { Receipt } from "./Receipt.ts";
import type { Refusal } from "./Refusal.ts";
import type { RightLeftU8 } from "./RightLeftU8.ts";
import type { Scan } from "./Scan.ts";
import type { Seat } from "./Seat.ts";
import type { Sensor } from "./Sensor.ts";
import type { Shape } from "./Shape.ts";
import type { Size } from "./Size.ts";
import type { SlotU32 } from "./SlotU32.ts";
import type { Span } from "./Span.ts";
import type { Stage } from "./Stage.ts";
import type { Stamp } from "./Stamp.ts";
import type { Status } from "./Status.ts";
import type { StdIoError } from "./StdIoError.ts";
import type { Stretch } from "./Stretch.ts";
import type { Tally } from "./Tally.ts";
import type { Ticket } from "./Ticket.ts";
import type { TlsCert } from "./TlsCert.ts";
import type { TlsConfig } from "./TlsConfig.ts";
import type { TlsResolver } from "./TlsResolver.ts";
import type { TlsSession } from "./TlsSession.ts";
import type { Turner } from "./Turner.ts";
import type { Undo } from "./Undo.ts";
import type { Url } from "./Url.ts";
import type { Waiter } from "./Waiter.ts";
import type { WrapU32 } from "./WrapU32.ts";
export declare function load(path?: string | URL): Promise<void>;
export declare const LIMIT: rt.u64;
export declare const FAR: rt.i64;
export declare const EOF: number;
export declare const URL$: rt.u64;
export declare const STEP: rt.u64;
export declare const WORD: rt.u64;
export declare function hasText(note: Note): boolean;
export declare function marked(apiMarked: Mark): rt.u64;
export declare function reading(n: number): Reading | undefined;
export declare function readingDebug(n: number): string;
export declare function readingEcho(reading: Reading | undefined): Reading | undefined;
export declare function readingBump(reading: rt.Place<Reading | undefined>): void;
export declare function advance(stage: Stage): Stage;
export declare function only(counter: Counter): [Counter];
export declare function widths(n: rt.u64): [number, number, number, rt.u64];
export declare function divide(a: rt.u64, b: rt.u64): rt.Result<[rt.u64, rt.u64], Error_>;
export declare function corner(unit: boolean): rt.Result<[number, number], Error_>;
export declare function cornerLater(): rt.Call<[number, number]>;
export declare function spread(n: rt.u64): [number, number, number, rt.u64] | undefined;
export declare function find(haystack: Uint8Array | rt.Bytes, needle: Uint8Array | rt.Bytes): [rt.u64, rt.u64] | undefined;
export declare function rows(points: Point[]): ([string, Shape, Point])[];
export declare function port(text: string): rt.Result<number, string>;
export declare function span(start: number, end: number): number;
export declare function halves(span: Span): [[number, number], [number, number]];
export declare function doubled(value: [number | undefined] | undefined): [number | undefined] | undefined;
export declare function title(formal: boolean): [string] | undefined;
export declare function named(name: string): [string] | undefined;
export declare function lentBack(value: [Uint8Array]): [Uint8Array];
export declare function boxedTuple(bytes: Uint8Array | rt.Bytes): [Uint8Array];
export declare function swapped(pair: [string, number]): [number, string];
export declare function lasting(draft: Draft): boolean;
export declare function stored(receipt: Receipt): number;
export declare function tallied(receipt: Receipt): number;
export declare function summedUpReceipt(totaled: Receipt): number;
export declare function erased(receipt: Receipt): number;
export declare function longer(a: Name, b: Name): Name;
export declare function unnamed(): Name;
export declare function total(range: {
    start: rt.u64;
    end: rt.u64;
}): rt.u64;
export declare function reach(start: number, end: number): number;
export declare function scaled(v: rt.u128, by: rt.u64): rt.u128 | undefined;
export declare function pair(): Batch2;
export declare function drain4(batch: Batch): rt.u64;
export declare function drain2(batch: Batch2): rt.u64;
export declare function capacities44(a: Batch, b: Batch): rt.u64;
export declare function capacities42(a: Batch, b: Batch2): rt.u64;
export declare function capacities24(a: Batch2, b: Batch): rt.u64;
export declare function capacities22(a: Batch2, b: Batch2): rt.u64;
export declare function fill4(batch: Batch, v: number): rt.Call<Uint32Array>;
export declare function fill2(batch: Batch2, v: number): rt.Call<Uint32Array>;
export declare function widen(range: {
    start: rt.u64;
    end: rt.u64;
}, by: rt.u64): {
    start: rt.u64;
    end: rt.u64;
};
export declare function count(range: {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
}): rt.u64;
export declare function end(limit: rt.u64): {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
};
export declare function ends(limit: rt.u64): [{
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
}, {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
}];
export declare function below(n: rt.u64, limit: rt.u64): rt.Result<rt.u64, {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
}>;
export declare function drive(start: Moment, poll: (a0: Moment, a1: Waiter) => rt.Result<Moment | undefined, string>): rt.Call<string>;
export declare function made(n: number, make: (a0: number) => string): string;
export declare function tickets(entries: ([string, Ticket])[]): ([string, Ticket])[];
export declare function reissue(ticket: Ticket, swap: (a0: Ticket) => Ticket): Ticket;
export declare function reissueLent(ticket: Ticket, swap: (a0: Ticket) => Ticket): Ticket;
export declare function seat(mark: Mark, row: number): Seat;
export declare function admit(entry: [Counter, Mark, Seat, Choice]): rt.u64;
export declare function choose(choice: Choice): rt.u64;
export declare function heft(marked: [Mark, Uint8Array], lent: Parcel, moved: [Counter, Uint8Array]): rt.u64;
export declare function spend(e: Counter, it: Counter): rt.Result<rt.u64, string>;
export declare function isKept(status: Status): boolean;
export declare function attempt(check: () => rt.Result<undefined, string>): rt.Result<undefined, string>;
export declare function tail(file: rt.Reader, len: rt.u64): rt.Result<[Uint8Array, rt.u64], string>;
export declare function seekTo(file: rt.Reader, at: rt.u64): rt.Result<rt.u64, string>;
export declare function halved(n: number): rt.Call<number | undefined>;
export declare function yielded(value: number): rt.Call<number>;
export declare function after(first: Promise<unknown>, value: number): rt.Call<number>;
export declare function within(polls: number, future: Promise<unknown>): rt.Call<rt.Result<undefined, Error_>>;
export declare function past(at: Stamp, deadline: Stamp): rt.Result<Stamp, Stamp>;
export declare function odd(n: number): rt.Result<number, number>;
export declare function momentAt(ticks: rt.u64, rate: rt.u64): rt.Result<Moment, string>;
export declare function stampAt(millis: rt.u64): rt.Result<Stamp, string>;
export declare function shifted(moment: Moment, shift: (a0: Moment) => Moment): Moment;
export declare function jvmSum(field: JvmField, method: JvmStatic, opt: OptIn): number;
export declare function clockedAfter(clocked: Clocked, ticks: rt.u64): Clocked;
export declare function emit(log: Log, lines: Uint8Array | rt.Bytes): number;
export declare function sensed(sensor: Sensor): number;
export declare function counted(n: number, count: (a0: number) => void): rt.Call<rt.Result<number, Error_>>;
export declare function filter(lines: Uint8Array | rt.Bytes, keep: (a0: number, a1: string) => boolean): number;
export declare function logLater(log: Log): rt.Call<boolean>;
export declare function logCounts(log: Log, level: number, counter: Counter, more: Counter[]): number;
export declare function onFirstThread(): boolean;
export declare function localsDropped(): number;
export declare function localOrNew(local: Local | undefined): Local;
export declare function localOrNewLater(local: Local | undefined): rt.Call<Local>;
export declare function bumpEither(first: Local | undefined, second: Local): number;
export declare function localAfter(): rt.Call<Local>;
export declare function localLater(n: number): rt.Call<number>;
export declare function pollTripled(n: number): rt.Call<number>;
export declare function watch(gauge: Gauge, watch: (a0: Gauge) => rt.u64): rt.u64;
export declare function tokenBytes(id: number, len: number): rt.Bytes;
export declare function tokenSlice(id: number, len: number, of: number): rt.Bytes;
export declare function nextPacket(packet: Packet): Packet;
export declare function fingerprint(hex: string): rt.Result<Uint8Array, string>;
export declare function dropped(): number;
export declare function bumpBoth(a: Tally, b: Tally): number;
export declare function offset(by: () => number, byData: number): number;
export declare function shift(by: () => number, cBy: number): number;
export declare function walk(steps: rt.u64, visit: (a0: Cursor) => rt.u64): rt.u64;
export declare function each(steps: number, visit: (a0: number) => void): number;
export declare function peek(tally: Tally, peek: (a0: Tally) => number): number;
export declare function len(payload: Uint8Array | rt.Bytes): rt.u64;
export declare function hex(payload: Uint8Array | rt.Bytes): string;
export declare function timedHex(payload: Uint8Array | rt.Bytes): string;
export declare function sampleHex(sample: Uint8Array | rt.Bytes): string;
export declare function queuedHex(queued: Uint8Array | rt.Bytes): string;
export declare function tickName(tick: Label): string;
export declare function rootedName(rooted: Label): string;
export declare function storedText(stored: string): string;
export declare function layerName(layer: Label): string;
export declare function greet(name: string, shape: Shape): string;
export declare function describe(event: Event): string;
export declare function tag(size: Size, tag: number): PairSizeU8;
export declare function cross(left: LeftRightU8, right: RightLeftU8): number;
export declare function keptPoint(from: Point): rt.u64;
export declare function keptCounter(from: Counter): rt.u64;
export declare function keptSize(from: Size): rt.u64;
export declare function keptWrapU32(from: WrapU32): rt.u64;
export declare function keptPolarPoint(from: PolarPoint): rt.u64;
export declare function keepPoint(kept: Point): number;
export declare function keptOrPoint(kept: Point | undefined): number;
export declare function keeperPoint(keeper: Point): number;
export declare function BOXED(): BoxedU16;
export declare function keeps(log: Log): BoxedU8;
export declare function weighPoint(from: Point): number;
export declare function applyNonZeroU16(pair: PairNonZeroU16, f: (a0: number) => number): number;
export declare function applyU32(pair: PairU32, f: (a0: number) => number): number;
export declare function reportNonZeroU16(pair: PairNonZeroU16, log: Log): number;
export declare function reportU32(pair: PairU32, log: Log): number;
export declare function tallyLogCount(log: Log): number;
export declare function wind(whole: PolarTurn, part: Polar_Turn): number;
export declare function area(size: Size): rt.u64;
export declare function seedsPointPoint(from: Point): rt.u64;
export declare function seedsPointCounter(from: Counter): rt.u64;
export declare function seedsPointSize(from: Size): rt.u64;
export declare function seedsPointWrapU32(from: WrapU32): rt.u64;
export declare function seedsPointPolarPoint(from: PolarPoint): rt.u64;
export declare function seedsCounterPoint(from: Point): rt.u64;
export declare function seedsCounterCounter(from: Counter): rt.u64;
export declare function seedsCounterSize(from: Size): rt.u64;
export declare function seedsCounterWrapU32(from: WrapU32): rt.u64;
export declare function seedsCounterPolarPoint(from: PolarPoint): rt.u64;
export declare function grow(area: Area, by: number): Area;
export declare function wide(value: number): SlotU32;
export declare function host(url: Url): string | undefined;
export declare function resolve(base: Url, path: string): rt.Result<Url, string>;
export declare function ports(addrs: rt.SocketAddr[]): Uint16Array;
export declare function doubledTtl(ttl: rt.Duration): rt.Duration;
export declare function file(dir: rt.PathBuf, name: string): rt.PathBuf;
export declare function stray(): rt.PathBuf;
export declare function visitStray(visit: (a0: rt.PathBuf) => void): void;
export declare function nextChar(c: number): number | undefined;
export declare function extended(duration: rt.Duration, by: rt.Duration): rt.Duration | undefined;
export declare function parents(paths: rt.PathBuf[]): (rt.PathBuf | undefined)[];
export declare function words(text: string): string[];
export declare function sentence(words: string[] | undefined): string;
export declare function putNumber(value: number, buf: rt.Place<Uint8Array>): void;
export declare function secondLater(w235: rt.Duration): rt.Duration;
export declare function digit(r: rt.Place<Uint8Array>): rt.Result<number, Error_>;
export declare function pushCounted(entry: rt.Place<[Uint8Array, number]>, byte: number): number;
export declare function nudge(point: rt.Place<Point>, by: number): void;
export declare function nudgeBy(point: rt.Place<Point>, refusal: Refusal): void;
export declare function secure(alpn: string): TlsConfig;
export declare function alpn(config: TlsConfig): string;
export declare function alpnLent(config: TlsConfig): string;
export declare function certify(der: Uint8Array | rt.Bytes): TlsCert;
export declare function byteLen(bytes: Uint8Array | rt.Bytes): rt.Call<rt.u64>;
export declare function certLen(cert: TlsCert): rt.u64;
export declare function certDer(cert: TlsCert): rt.Bytes;
export declare function chainLen(chain: TlsCert[]): rt.u64;
export declare function verify(cert: TlsCert): rt.Result<number, string>;
export declare function parseCert(text: string): rt.Result<number, string>;
export declare function parseLater(text: string): rt.Call<rt.Result<number, string>>;
export declare function resolver(name: string): TlsResolver;
export declare function resolved(resolver: TlsResolver): string;
export declare function failure(message: string): StdIoError;
export declare function failureMessage(error: StdIoError): string;
export declare function failureKind(error: StdIoError): string;
export declare function sumAll(items: number[]): number;
export declare function weighAllCounter(items: Counter[]): rt.u64;
export declare function offeredLen(alpns: Uint8Array[]): rt.u64;
export declare function joinWords(words: string[]): string;
export declare function resume(alpn: string, seconds: number): rt.Result<TlsSession, string>;
export declare function either(text: string | undefined): rt.u64;
export declare function eitherLater(text: string | undefined): rt.Call<rt.u64>;
export declare function prefixed(payload: Uint8Array | rt.Bytes, prefix: Uint8Array | rt.Bytes | undefined): rt.Bytes;
export declare function sumLists(a: Int32Array, b: Int32Array | undefined, c: Int32Array | undefined): number;
export declare function coordinates(point: Point | undefined): number;
export declare function chunks(bytes: Uint8Array | rt.Bytes, size: rt.u64): Uint8Array[];
export declare function counters(names: string[]): Counter[];
export declare function worth(some: Counter[] | undefined, boxed: Counter[]): rt.u64;
export declare function heldCount(held: [Counter] | undefined): rt.u64 | undefined;
export declare function boxedCounter(name: string | undefined): Counter | undefined;
export declare function boxedName(boxed: Counter | undefined): string | undefined;
export declare function less(value: number, value2: number): number;
export declare function sums(rows: Uint32Array[]): Uint32Array;
export declare function least(sets: number[][]): number[][];
export declare function splitLast(items: Int32Array): [Int32Array, number];
export declare function sumU32(items: Uint32Array): rt.u64;
export declare function doubledI64(items: BigInt64Array): BigInt64Array;
export declare function visitChunks(payload: Uint8Array | rt.Bytes, size: rt.u64, visit: (a0: Uint8Array | undefined) => rt.u64): BigUint64Array;
export declare function scanned(scan: Scan, payload: Uint8Array | rt.Bytes): rt.u64;
export declare function boxedFloats(items: Float64Array): Float64Array;
export declare function summed(points: Point[]): Point;
export declare function breadth(limits: LimitsDuration): rt.Duration;
export declare function byteLimits(low: Uint8Array | rt.Bytes, high: Uint8Array | rt.Bytes): LimitsBytes;
export declare function joined(parts: Uint8Array[]): rt.Bytes;
export declare function firstOf(items: string[]): string | undefined;
export declare function readLen(reader: Uint8Array | rt.Bytes): rt.u64;
export declare function longest(words: string[]): rt.u64;
export declare function encode(value: Json): string;
export declare function tagged(doc: Json, tag: string): Json;
export declare function entries(entries: ([string, Json])[], tag: string): ([string, Json])[];
export declare function sections(entries: ([string, Json])[]): ([string, Json])[];
export declare function now(): Instant;
export declare function later(at: Instant, by: rt.Duration): Instant;
export declare function since(later: Instant, earlier: Instant): rt.Duration;
export declare function hasInstant(at: Instant | undefined): boolean;
export declare function louder(filter: Level | undefined): Level | undefined;
export declare function spell(filter: Level | undefined): string;
export declare function GREETINGS(): string[];
export declare function PRIMES(): Uint32Array;
export declare function numbered(names: string[]): Counter[];
export declare function nexts(v4: rt.SocketAddrV4, v6: rt.SocketAddrV6): [rt.SocketAddrV4, rt.SocketAddrV6];
export declare function nextIps(v4: rt.Ipv4Addr, v6: rt.Ipv6Addr): [rt.Ipv4Addr, rt.Ipv6Addr];
export declare function flip(ip: rt.IpAddr): rt.IpAddr;
export declare function reasons(pairs: ([Counter, Refusal])[], last: [Counter, Refusal] | undefined): string;
export declare function excuse(note: Uint8Array | rt.Bytes, entry: [Uint8Array, Refusal]): rt.u64;
export declare function relay(log: Log, refusals: Refusal[]): number;
export declare function refuse(reason: string): rt.Result<number, Refusal>;
export declare function jam(n: number): rt.Result<number, Fault>;
export declare function platform(): Platform;
export declare function pointerBits(platform: Platform): number;
export declare function sameLabelled(labelled: Labelled): Labelled;
export declare function placement(): Placement;
export declare function isFamily(family: Family): boolean;
export declare function paired(f: () => number, fUnwind: () => number): number;
export declare function same(a: rt.f64, b: rt.f64): number;
export declare function errno(): number;
export declare function assert(holds: boolean): boolean;
export declare function offsetof(field: number): number;
export declare function returnValues(point: rt.Place<Point>, fail: boolean): rt.Result<[string, [number, number, number, rt.u64] | undefined, Uint8Array[]], Error_>;
export declare function returnMoved(counter: Counter, fail: boolean): rt.Result<Counter, Error_>;
export declare function nonZeros(): NonZeros;
export declare function describeNonZeros(nonZeros: NonZeros): string;
export declare function nextHop(hop: rt.u64 | undefined): rt.u64 | undefined;
export declare function fartherHop(hops: PairNonZeroU16): number;
export declare function wexitstatus(status: number): number;
export declare function sameDuration(v: rt.Duration): rt.Duration;
export declare function sameU64(v: rt.u64): rt.u64;
export declare function sameUsize(v: rt.u64): rt.u64;
export declare function sameIsize(v: rt.i64): rt.i64;
export declare function sameU128(v: rt.u128): rt.u128;
export declare function sameChar(v: number): number;
export declare function sameString(v: string): string;
export declare function samePath(v: rt.PathBuf): rt.PathBuf;
export declare function sameSocketAddr(v: rt.SocketAddr): rt.SocketAddr;
export declare function sameSocketAddrV4(v: rt.SocketAddrV4): rt.SocketAddrV4;
export declare function sameSocketAddrV6(v: rt.SocketAddrV6): rt.SocketAddrV6;
export declare function sameIpAddr(v: rt.IpAddr): rt.IpAddr;
export declare function unescaped(text: string): string;
export declare function unescapedLater(text: string): rt.Call<string>;
export declare function unescapedMaybe(text: string | undefined): rt.Call<string | undefined>;
export declare function unzeroed(bytes: Uint8Array | rt.Bytes): rt.Bytes;
export declare function deprecated(note: number): number;
export declare function deprecated_(note: number): number;
export declare function all(note: number): number;
export declare function all_(note: number): number;
export declare function knob(dial: Dial): Knob;
export declare function levelLater(dial: Dial): rt.Call<rt.u64>;
export declare function turned(dial: Dial, turner: Turner): rt.Result<Dial, Error_>;
export declare function undone(undo: Undo, n: number): number;
export declare function unwound(): number;
export declare function measured(f: () => rt.u64): rt.u64;
export declare function negated(f: () => rt.i64): rt.i64;
export declare function lettered(f: () => number): number;
export declare function bumped(n: rt.Place<number>, f: () => number): number;
export declare function stretched(stretch: Stretch): rt.u64;
export declare function crowded(reading: Reading, a: Stamp, b: Stamp, n0: rt.u64, n1: rt.u64, n2: rt.u64, n3: rt.u64, n4: rt.u64, n5: rt.u64, n6: rt.u64, n7: rt.u64, n8: rt.u64, n9: rt.u64, n10: rt.u64, n11: rt.u64, n12: rt.u64, n13: rt.u64, n14: rt.u64, n15: rt.u64, n16: rt.u64, n17: rt.u64, n18: rt.u64, n19: rt.u64, n20: rt.u64, n21: rt.u64, n22: rt.u64, n23: rt.u64, n24: rt.u64, n25: rt.u64, n26: rt.u64, n27: rt.u64, n28: rt.u64, n29: rt.u64, n30: rt.u64, n31: rt.u64, n32: rt.u64, n33: rt.u64, n34: rt.u64, n35: rt.u64, n36: rt.u64, n37: rt.u64, n38: rt.u64, n39: rt.u64, n40: rt.u64, n41: rt.u64, n42: rt.u64, n43: rt.u64, n44: rt.u64, n45: rt.u64, n46: rt.u64, n47: rt.u64, n48: rt.u64, n49: rt.u64, n50: rt.u64, n51: rt.u64, n52: rt.u64, n53: rt.u64, n54: rt.u64, n55: rt.u64, n56: rt.u64, n57: rt.u64, n58: rt.u64, n59: rt.u64, n60: rt.u64, n61: rt.u64, n62: rt.u64, n63: rt.u64, n64: rt.u64, n65: rt.u64, n66: rt.u64, n67: rt.u64, n68: rt.u64, n69: rt.u64, n70: rt.u64, n71: rt.u64, n72: rt.u64, n73: rt.u64, n74: rt.u64, n75: rt.u64, n76: rt.u64, n77: rt.u64, n78: rt.u64, n79: rt.u64, n80: rt.u64, n81: rt.u64, n82: rt.u64, n83: rt.u64, n84: rt.u64, n85: rt.u64, n86: rt.u64, n87: rt.u64, n88: rt.u64, n89: rt.u64, n90: rt.u64, n91: rt.u64, n92: rt.u64, n93: rt.u64, n94: rt.u64, n95: rt.u64, n96: rt.u64, n97: rt.u64, n98: rt.u64, n99: rt.u64, n100: rt.u64, n101: rt.u64, n102: rt.u64, n103: rt.u64, n104: rt.u64, n105: rt.u64, n106: rt.u64, n107: rt.u64, n108: rt.u64, n109: rt.u64, n110: rt.u64, n111: rt.u64, n112: rt.u64, n113: rt.u64, n114: rt.u64, n115: rt.u64, n116: rt.u64, n117: rt.u64, n118: rt.u64): rt.u64;
export declare function crammed(a: Moment, b: Moment, n0: rt.u64, n1: rt.u64, n2: rt.u64, n3: rt.u64, n4: rt.u64, n5: rt.u64, n6: rt.u64, n7: rt.u64, n8: rt.u64, n9: rt.u64, n10: rt.u64, n11: rt.u64, n12: rt.u64, n13: rt.u64, n14: rt.u64, n15: rt.u64, n16: rt.u64, n17: rt.u64, n18: rt.u64, n19: rt.u64, n20: rt.u64, n21: rt.u64, n22: rt.u64, n23: rt.u64, n24: rt.u64, n25: rt.u64, n26: rt.u64, n27: rt.u64, n28: rt.u64, n29: rt.u64, n30: rt.u64, n31: rt.u64, n32: rt.u64, n33: rt.u64, n34: rt.u64, n35: rt.u64, n36: rt.u64, n37: rt.u64, n38: rt.u64, n39: rt.u64, n40: rt.u64, n41: rt.u64, n42: rt.u64, n43: rt.u64, n44: rt.u64, n45: rt.u64, n46: rt.u64, n47: rt.u64, n48: rt.u64, n49: rt.u64, n50: rt.u64, n51: rt.u64, n52: rt.u64, n53: rt.u64, n54: rt.u64, n55: rt.u64, n56: rt.u64, n57: rt.u64, n58: rt.u64, n59: rt.u64, n60: rt.u64, n61: rt.u64, n62: rt.u64, n63: rt.u64, n64: rt.u64, n65: rt.u64, n66: rt.u64, n67: rt.u64, n68: rt.u64, n69: rt.u64, n70: rt.u64, n71: rt.u64, n72: rt.u64, n73: rt.u64, n74: rt.u64, n75: rt.u64, n76: rt.u64, n77: rt.u64, n78: rt.u64, n79: rt.u64, n80: rt.u64, n81: rt.u64, n82: rt.u64, n83: rt.u64, n84: rt.u64, n85: rt.u64, n86: rt.u64, n87: rt.u64, n88: rt.u64, n89: rt.u64, n90: rt.u64, n91: rt.u64, n92: rt.u64, n93: rt.u64, n94: rt.u64, n95: rt.u64, n96: rt.u64, n97: rt.u64, n98: rt.u64, n99: rt.u64, n100: rt.u64, n101: rt.u64, n102: rt.u64, n103: rt.u64, n104: rt.u64, n105: rt.u64, n106: rt.u64, n107: rt.u64, n108: rt.u64, n109: rt.u64, n110: rt.u64, n111: rt.u64, n112: rt.u64, n113: rt.u64, n114: rt.u64, n115: rt.u64, n116: rt.u64, n117: rt.u64, n118: rt.u64, n119: rt.u64, n120: rt.u64, n121: rt.u64, n122: rt.u64, n123: rt.u64): rt.u64;
export declare function jniOne(): number;
export declare function polarPoint(distance: number): PolarPoint | undefined;
export declare function macrosStreamed(streams: MacrosStreams, errno: number): number;
export declare function clashAny(typeUrl: string): ClashAny;
export declare function clashCandidate(at: number): ClashCandidate;
export declare function clashIgnore(arg0: number, arg1: number, arg2: number): number;
export declare function clashEmpty(): ClashEmptyString;
export declare function clashDescribed(described: ClashDescribed): number;
export declare function clashWaited(source: ClashSource): number;
export declare function clashUnderscored(underscored: ClashUnderscored, op: ClashOp, change: ClashChange): number;
export declare function clashUnderscoredNext(underscored: ClashUnderscored, change: ClashChange): [ClashUnderscored, ClashChange];
export declare function clashErased(eraser: ClashEraser): number;
export declare function clashNotches(n: number): ClashNotch[];
export declare function clashShifted(at: number, shift: ClashShift): number;
export declare function clashCounted(count: ClashCount): number;
export declare function clashErase_(): number;
export declare function clashErase(): number;
export declare function clashLend_(store: ClashStore | undefined): number;
export declare function clashLend(): number;
export declare function clashRun(work2: () => number, work: number): number;
export declare function clashAudible(tone: ClashTone): rt.Result<ClashTone, ClashTone[]>;
export declare function clashNested(a: [[number, number], number] | undefined, b: [[number, number, number]] | undefined): number;
export declare function clashAudioEncoderFinish(): number;
export declare function clashAudioEncoderFinish_(): number;
export declare function emitFromThread(log: Log, lines: Uint8Array | rt.Bytes): number;
export declare function emitLater(log: Log, lines: Uint8Array | rt.Bytes): rt.Call<number>;
export declare function logOnDrop(log: Log): rt.Call<void>;
export declare function logWaker(log: Log): rt.Waker;
export declare function hook(f: () => void): void;
export declare function unhook(): void;
export { URL$ as URL };

// rt.ts
declare const brand: unique symbol;
export type U64Limbs = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "u64";
};
export type I64Limbs = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "i64";
};
export type U128Limbs = {
    readonly hi: number;
    readonly w2: number;
    readonly w1: number;
    readonly lo: number;
    readonly [brand]: "u128";
};
type Num<T extends string> = number & {
    readonly [brand]?: T;
};
export type u64 = Num<"u64"> | U64Limbs;
export type i64 = Num<"i64"> | I64Limbs;
export type u128 = Num<"u128"> | U128Limbs;
export type F32Nan = {
    readonly bits: number;
    readonly [brand]: "f32";
};
export type F64Nan = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "f64";
};
export type f32 = Num<"f32"> | F32Nan;
export type f64 = Num<"f64"> | F64Nan;
export type Result<T, E> = {
    readonly ok: true;
    readonly value: T;
} | {
    readonly ok: false;
    readonly error: E;
};
export declare function Ok<T>(value: T): Result<T, never>;
export declare function Err<E>(error: E): Result<never, E>;
export declare const None: unique symbol;
export type None = typeof None;
export type Place<T> = {
    v: T;
};
export interface Opaque<T extends string = string> {
    readonly [brand]: T;
}
export declare class Opaque<T extends string = string> {
    #private;
    private constructor();
}
export declare function opaqueMade(bytes: Uint8Array): Opaque;
export type Duration = {
    readonly secs: u64;
    readonly nanos: number;
};
export type PathBuf = {
    readonly bytes: Uint8Array;
};
export declare function pathFrom(text: string): PathBuf;
export declare function pathText(path: PathBuf): string;
export type Ipv4Addr = {
    readonly octets: Uint8Array;
};
export type Ipv6Addr = {
    readonly octets: Uint8Array;
};
export type IpAddr = {
    readonly tag: "V4";
    readonly _0: Ipv4Addr;
} | {
    readonly tag: "V6";
    readonly _0: Ipv6Addr;
};
export type SocketAddrV4 = {
    readonly ip: Ipv4Addr;
    readonly port: number;
};
export type SocketAddrV6 = {
    readonly ip: Ipv6Addr;
    readonly port: number;
    readonly flowinfo: number;
    readonly scopeId: number;
};
export type SocketAddr = {
    readonly tag: "V4";
    readonly _0: SocketAddrV4;
} | {
    readonly tag: "V6";
    readonly _0: SocketAddrV6;
};
export type SeekFrom = "Start" | "Current" | "End";
export interface Reader {
    read(buffer: Uint8Array): number;
    seek(offset: u64 | i64, from: SeekFrom): u64;
}
export type Call<T> = Promise<T> & {
    cancel(): void;
};
export declare class Cancelled extends Error {
    constructor();
}
export type Waker = {
    wake(): void;
    [Symbol.dispose](): void;
};
export declare function opaquePut(v: Opaque, n: number, p: number): void;
type Exports = Record<string, any>;
export declare let w: Exports;
export declare function dv(): DataView;
export declare function mem(): Uint8Array<ArrayBuffer>;
export type Addr = bigint;
export declare const s: {
    d: number;
};
export declare function abort(message: string): never;
export declare function enter(): number;
export declare function leave(mark: number): void;
export declare function alloc(size: number, align: number): number;
export declare function status(p: number, what: string): void;
export interface Handle<T extends string = string> {
    readonly [brand]: T;
}
export type Kind = {
    readonly name: string;
    readonly free: number;
    readonly drop?: string;
};
export declare class Handle<T extends string = string> {
    p: number;
    readonly i: number;
    readonly k: Kind;
    readonly owned: boolean;
    n: number;
    constructor(p: number, k: Kind, owned: boolean);
    evict(): void;
    release(): void;
    [Symbol.dispose](): void;
}
export declare function hold(handle: Handle): number;
export declare function holdOr(handle: Handle | undefined): number;
export declare function lentHandle<T extends string>(p: number, k: Kind): Handle<T>;
export declare function put8(p: number, v: number): void;
export declare function put16(p: number, v: number): void;
export declare function put32(p: number, v: number): void;
export declare function int(v: number, min: number, max: number, what: string): number;
export declare function nonZero<T extends number | u64 | i64>(v: T): T;
export declare function f32Number(v: f32): number;
export declare function f64Number(v: f64): number;
export declare function char(v: number): number;
export declare function u64Of(hi: number, lo: number): u64;
export declare function i64Of(hi: number, lo: number): i64;
export declare function u64Halves(v: u64): [number, number];
export declare function i64Halves(v: i64): [number, number];
export declare function u64Big(v: u64): bigint;
export declare function i64Big(v: i64): bigint;
export declare function u64OfBig(v: bigint): u64;
export declare function i64OfBig(v: bigint): i64;
export declare function u64ToBig(v: u64): bigint;
export declare function i64ToBig(v: i64): bigint;
export declare function u128ToBig(v: u128): bigint;
export declare function u64FromBig(v: bigint): u64;
export declare function i64FromBig(v: bigint): i64;
export declare function u128FromBig(v: bigint): u128;
export declare function f32Get(p: number): f32;
export declare function f32Put(p: number, v: f32): void;
export declare function f64Get(p: number): f64;
export declare function f64Put(p: number, v: f64): void;
export declare function u64Get(p: number): u64;
export declare function i64Get(p: number): i64;
export declare function u64Put(p: number, v: u64): void;
export declare function i64Put(p: number, v: i64): void;
export declare function u128Get(p: number): u128;
export declare function u128Put(p: number, v: u128): void;
export declare function typedBytes(v: unknown, kind: string, size: number): Uint8Array;
export declare function encoded(text: string): Uint8Array;
export declare function putStr(dst: number, text: string): void;
export declare function putBytes(dst: number, data: Uint8Array | Bytes): void;
export declare function shared(dst: number, data: Uint8Array | Bytes | undefined): void;
export declare function putNone(dst: number): void;
export declare function isNone(p: number): boolean;
export declare function bytesLent(p: number): Uint8Array;
export declare function strLent(p: number): string;
export declare function bytes_(p: number): Uint8Array;
export declare class Bytes {
    #private;
    private constructor();
    get length(): number;
    at(index: number): number | undefined;
    slice(start?: number, end?: number): Uint8Array<ArrayBuffer>;
    [Symbol.iterator](): IterableIterator<number>;
    [Symbol.dispose](): void;
}
export declare function returnedBytes(p: number): Bytes;
export declare function str(p: number): string;
export declare function copied(dst: number, text: string, message?: boolean): void;
export declare function copiedBytes(dst: number, data: Uint8Array): void;
export declare function arrayPut(p: number, data: Uint8Array, n: number): void;
export declare function pathBytes(path: PathBuf): Uint8Array;
export declare function durationGet(p: number): Duration;
export declare function durationPut(p: number, v: Duration): void;
export declare function ipv4Get(p: number): Ipv4Addr;
export declare function ipv4Put(p: number, v: Ipv4Addr): void;
export declare function ipv6Get(p: number): Ipv6Addr;
export declare function ipv6Put(p: number, v: Ipv6Addr): void;
export declare function ipGet(p: number): IpAddr;
export declare function ipPut(p: number, v: IpAddr): void;
export declare function socketV4Get(p: number): SocketAddrV4;
export declare function socketV4Put(p: number, v: SocketAddrV4): void;
export declare function socketV6Get(p: number): SocketAddrV6;
export declare function socketV6Put(p: number, v: SocketAddrV6): void;
export declare function socketGet(p: number): SocketAddr;
export declare function socketPut(p: number, v: SocketAddr): void;
export declare function give(value: unknown, drop?: () => void): number;
export declare function callbackPut(p: number, vtable: Addr, value: {
    close?(): void;
}): void;
export declare function lend(value: unknown): number;
export declare function find<T>(t: number): T;
export declare const fn: {
    wake: Addr;
    wakerDrop: Addr;
    free: Addr;
    poll: Addr;
    read: Addr;
    seek: Addr;
};
export declare function out<T>(what: string, f: () => T): T;
export declare function wakerOf(dst: number, wake: () => void): void;
export declare function wakerGet(p: number): Waker;
export declare function futureOf(dst: number, promise: Promise<unknown>): void;
export declare function readerOf(dst: number, reader: Reader): void;
export declare function pending<T>(f: number, poll: (f: number, waker: number) => [boolean, T], free: string): Call<T>;
export declare function ready<T>(value: T): Call<T>;
export declare function failed<T>(error: unknown): Call<T>;
export type Stream<T> = AsyncIterableIterator<T>;
export declare function values<T>(next: () => Call<unknown>, none: unknown, fallible: boolean): Stream<T>;
export declare function on(mask: bigint, name: string): void;
export declare function lenAt(p: number): number;
export declare function mirror(p: number, n: number, align: number): number;
type Block = Uint8Array | Int8Array | Uint16Array | Int16Array | Uint32Array | Int32Array | BigUint64Array | BigInt64Array | Float32Array | Float64Array;
export declare function readInto(p: number, out: Block): void;
export declare function putList(p: number, at: number, n: number): void;
export declare function handleAt(p: number): number;
export declare function u64Plain(v: u64): u64;
export declare function i64Plain(v: i64): i64;
export declare function putHandle(p: number, h: Handle | undefined): void;
export type Glue = {
    readonly prefix: string;
    readonly space: string;
    readonly file: string;
    readonly contract: bigint;
    readonly targets: readonly number[];
    readonly imports: Record<string, (...args: never[]) => unknown>;
};
export declare function load(path: string | URL, glue: Glue): Promise<void>;
export {};
