// A pointer the translation can't follow, such as one a struct's field holds, fails with
// a rust2-ts error where JS keeps apart what Rust's reaches, which native Rust can't show:
// there each is defined, as `repr(C)` fields and a shared subslice's parent are one
// allocation, and a `Vec`'s capacity is past its length.
//
// usage: node pointers.ts

import { strict as assert } from "node:assert";

import * as rt from "../runtime/rt.ts";

const unsure = (f: () => void) => assert.throws(f, (e: Error) => !(e instanceof rt.Panic) && e.message.startsWith("rust2-ts: "));

// Two `&raw const x` of one variable, each its own `rt.Place`.
let x = 5;
const a = rt.place(() => x, (v) => (x = v));
const b = rt.place(() => x, (v) => (x = v));
unsure(() => rt.ptrOffsetFrom(a, b));
assert.equal(rt.ptrOffsetFrom(rt.ptrAdd(a, 1), a), 1);

// A `repr(C)` struct's fields, each a value in no array: reached from the other, and as a slice of both.
const pair = { low: 1, high: 2 };
const low = rt.place(() => pair.low, (v) => (pair.low = v));
const high = rt.place(() => pair.high, (v) => (pair.high = v));
unsure(() => rt.ptrOffsetFrom(high, low));
unsure(() => rt.ptrAdd(high, -1).v);
unsure(() => rt.ptrSlice(low, 2));
unsure(() => rt.ptrCopy(low, high, 2));

// A shared subslice, which is a copy: before it, past it, and against its parent.
const lanes = [1, 2, 3, 4];
const part = rt.sliceRange(lanes, 1, 3);
unsure(() => rt.ptrAdd(rt.sliceAsPtr(part), -1).v);
unsure(() => rt.ptrAdd(rt.sliceAsPtr(part), 2).v);
unsure(() => rt.ptrOffsetFrom(rt.sliceAsPtr(part), rt.sliceAsPtr(lanes)));
unsure(() => rt.ptrSlice(rt.sliceAsPtr(part), 3));
unsure(() => rt.ptrCopy(rt.sliceAsPtr(part), rt.sliceAsPtr(lanes), 3));
assert.equal(rt.ptrAdd(rt.sliceAsPtr(part), 1).v, 3);

// A byte in no array, as a raw or mutable slice, as a shared one past it, and the
// pointer of a shared one, a copy, against the byte's.
let byte = 3;
const lone = rt.place(() => byte, (v) => (byte = v));
unsure(() => rt.ptrByteSlice(lone, 1, false));
unsure(() => rt.ptrByteSlice(lone, 2, true));
const shared = rt.ptrByteSlice(lone, 1, true);
assert.deepEqual(shared.data, Uint8Array.of(3));
unsure(() => rt.ptrOffsetFrom(rt.byteSliceAsPtr(shared), lone));

// A write into a `Vec`'s capacity past its length, which a JS array would grow by.
const vec: number[] = [7];
unsure(() => {
	rt.ptrAdd(rt.sliceAsPtr(vec), 1).v = 8;
});
assert.equal(vec.length, 1);

// Memory from `alloc`, which JS keeps as values once a cast types it: its bytes, typed as
// another type of the same size, a pointer past the start of what it holds cast back to
// bytes, and one to its values passed as if a block.
const block = rt.alloc({ size: 12, align: 4 });
unsure(() => block.v);
const words = rt.allocAs<number>(block, 4, "u32");
words.v = 0xffffffff;
assert.equal(rt.allocAs<number>(block, 4, "u32").v, 0xffffffff);
unsure(() => rt.allocAs(block, 4, "i32"));
unsure(() => rt.allocBytes(words, 4, "f32"));
unsure(() => rt.allocBytes(rt.ptrAdd(words, 1), 4, "u32"));
// Grown, it keeps its values, as many as fit; freed or grown, a pointer to its values is
// no block.
unsure(() => rt.dealloc(words));
unsure(() => rt.realloc(words, 16));
const grown = rt.realloc(rt.allocBytes(words, 4, "u32"), 18);
assert.deepEqual(Array.from(grown.array ?? []), [0xffffffff, undefined, undefined, undefined]);

// A dangling pointer read through, written through, or copied from, which Rust leaves
// undefined, rather than reading `undefined`.
const dangling: rt.Place<number> = rt.DANGLING;
unsure(() => rt.ptrAdd(dangling, 1).v);
unsure(() => {
	rt.ptrAdd(dangling, 1).v = 1;
});
unsure(() => rt.ptrCopy(dangling, rt.sliceAsPtr(vec), 1));
unsure(() => rt.ptrSlice(dangling, 1));
assert.equal(vec[0], 7);

// A raw slice pointer read through once its `Vec` has shrunk past it, which Rust keeps
// the old values for, and one past the array, which Rust allows until a read.
const rows = [1, 2, 3, 4];
const all = new rt.RawSlice(rt.sliceAsPtr(rows), 4);
const tail = new rt.RawSlice(rt.sliceAsPtr(rows, 1), 3);
assert.equal(all.deref()[3] + tail.deref()[2], 8);
rows.length = 3;
unsure(() => all.deref());
unsure(() => tail.deref());
const long = new rt.RawSlice(rt.sliceAsPtr(rows), 10);
assert.equal(long.length, 10);
unsure(() => long.deref());

// A `*const ()` cast back to a pointer to another type than any cast to it was from, which
// Rust reads as bytes JS doesn't keep, and one no cast erased, such as a waker's `Arc`.
const note = { id: 1 };
const erased = rt.ptrErase(note, "raw::Note", 2);
assert.equal(erased, note);
assert.equal(rt.ptrUnerase(erased, "raw::Note", 2), note);
unsure(() => rt.ptrUnerase(erased, "u16", 2));
unsure(() => rt.ptrUnerase({ v: undefined }, "raw::Note", 2));
// Erased as two types, as a transparent wrapper and what it holds are, it is either.
rt.ptrErase(note, "raw::Wrapper", 2);
assert.equal(rt.ptrUnerase(erased, "raw::Wrapper", 2), note);
assert.equal(rt.ptrUnerase(erased, "raw::Note", 2), note);
assert.throws(() => rt.ptrUnerase(erased, "u16", 2), /`\*const \(\)` cast back to a pointer to `u16`, which points at `raw::Note` or `raw::Wrapper`/);
// Null at any type, and dangling at the address of its type's alignment, which a cast
// through a type of another alignment keeps.
assert.equal(rt.ptrUnerase(rt.ptrErase(rt.NULL, "raw::Note", 2), "u16", 2), rt.NULL);
assert.equal(rt.ptrUnerase(rt.ptrErase(rt.DANGLING, "raw::Note", 2), "u16", 2), rt.DANGLING);
assert.equal(rt.ptrErase(rt.DANGLING, "raw::Byte", 1), rt.DANGLING_UNIT);
const two = rt.ptrErase(rt.DANGLING, "raw::Note", 2);
assert.notEqual(two, rt.DANGLING_UNIT);
const aligned = rt.ptrUnerase(two, "raw::Byte", 1) as object;
assert.notEqual(aligned, rt.DANGLING);
assert.equal(rt.ptrErase(aligned, "raw::Byte", 1), two);
unsure(() => (aligned as rt.Place<number>).v);

// `Arc::from_raw` of a pointer no `Arc::into_raw` or `as_ptr` gave, such as a box's, of an
// `Rc`'s, or of one whose value dropped, which Rust leaves undefined.
const task = new rt.Arc({ id: 1 });
const raw = rt.arcRaw(task, task.value, "Arc");
assert.equal(rt.arcFromRaw(raw, "Arc"), task);
unsure(() => rt.arcFromRaw({ id: 1 }, "Arc"));
unsure(() => rt.arcFromRaw(raw, "Rc"));
task[Symbol.dispose]();
assert.throws(() => rt.arcFromRaw(raw, "Arc"), /`Arc::from_raw` of a pointer to a value already dropped/);
