// A pointer the translation can't follow, such as one a struct's field holds, fails with
// a rust2-ts error where JS keeps apart what Rust's reaches, which native Rust can't show:
// there each is defined, as `repr(C)` fields are one allocation, and a `Vec`'s capacity
// is past its length. One Rust leaves undefined, such as a read through a dangling
// pointer, aborts as std's debug checks do.
//
// usage: node pointers.ts

import { strict as assert } from "node:assert";

import * as rs from "../runtime/rs.ts";

const unsure = (f: () => void) => assert.throws(f, (e: Error) => !(e instanceof rs.Panic) && e.message.startsWith("rust2-ts: "));
// What Rust leaves undefined aborts, as std's debug checks of unsafe preconditions do.
const undefinedBehavior = (f: () => void) => assert.throws(f, (e: Error) => e instanceof rs.Abort && e.message.startsWith("unsafe precondition(s) violated: "));

// Two `&raw const x` of one variable, each its own `rs.Place`.
let x = 5;
const a = rs.place(() => x, (v) => (x = v));
const b = rs.place(() => x, (v) => (x = v));
unsure(() => rs.ptrOffsetFrom(a, b));
assert.equal(rs.ptrOffsetFrom(rs.ptrAdd(a, 1), a), 1);

// A `repr(C)` struct's fields, each a value in no array: reached from the other, and as a slice of both.
const pair = { low: 1, high: 2 };
const low = rs.place(() => pair.low, (v) => (pair.low = v));
const high = rs.place(() => pair.high, (v) => (pair.high = v));
unsure(() => rs.ptrOffsetFrom(high, low));
unsure(() => rs.ptrAdd(high, -1).v);
unsure(() => rs.ptrSlice(low, 2));
unsure(() => rs.ptrCopy(low, high, 2));

// A byte in no array, as a raw or mutable slice, as a shared one past it, and the
// pointer of a shared one, a copy, against the byte's.
let byte = 3;
const lone = rs.place(() => byte, (v) => (byte = v));
unsure(() => rs.ptrByteSlice(lone, 1, false));
unsure(() => rs.ptrByteSlice(lone, 2, true));
const shared = rs.ptrByteSlice(lone, 1, true);
assert.deepEqual(shared.data, Uint8Array.of(3));
unsure(() => rs.ptrOffsetFrom(rs.byteSliceAsPtr(shared), lone));

// A write past an array that is no `Vec`'s items, which a JS array would grow by.
const vec: number[] = [7];
unsure(() => {
	rs.ptrAdd(rs.arrayAsPtr(vec), 1).v = 8;
});
assert.equal(vec.length, 1);

// A write past a `Vec`'s length lands in its room, but not past its capacity, and the
// room moves with the items to the `Vec` that takes them.
const held = rs.vecFrom([7]);
const end = rs.ptrAdd(rs.vecAsPtr(held, 4), 1);
unsure(() => {
	end.v = 8;
});
rs.vecReserveExact(held, 1, 4);
end.v = 8;
assert.deepEqual(held.items, [7]);
rs.vecSetLen(held, 2, 4, () => 0);
assert.deepEqual(held.items, [7, 8]);
rs.vecReserveExact(held, 4, 4);
rs.ptrAdd(end, 1).v = 9;
const taken = new rs.Vec(held.items, held.cap);
held.items = [];
assert.deepEqual(Array.from(rs.ptrSlice(rs.ptrAdd(end, -1), 3).data.slice(0, 3)), [7, 8, 9]);
rs.vecSetLen(taken, 3, 4, () => 0);
assert.deepEqual(taken.items, [7, 8, 9]);

// Memory from `alloc`, which JS keeps as values once a cast types it: its bytes, typed as
// another type of the same size, a pointer past the start of what it holds cast back to
// bytes, and one to its values passed as if a block.
const block = rs.alloc({ size: 12, align: 4 });
unsure(() => block.v);
const words = rs.allocAs<number>(block, 4, "u32");
words.v = 0xffffffff;
assert.equal(rs.allocAs<number>(block, 4, "u32").v, 0xffffffff);
unsure(() => rs.allocAs(block, 4, "i32"));
unsure(() => rs.allocBytes(words, 4, "f32"));
unsure(() => rs.allocBytes(rs.ptrAdd(words, 1), 4, "u32"));
// Grown, it keeps its values, as many as fit; freed or grown, a pointer to its values is
// no block.
undefinedBehavior(() => rs.dealloc(words));
undefinedBehavior(() => rs.realloc(words, 16));
const grown = rs.realloc(rs.allocBytes(words, 4, "u32"), 18);
assert.deepEqual(Array.from(grown.array ?? []), [0xffffffff, undefined, undefined, undefined]);

// A dangling pointer read through, written through, or copied from, which Rust leaves
// undefined, rather than reading `undefined`.
const dangling: rs.Place<number> = rs.DANGLING;
undefinedBehavior(() => rs.ptrAdd(dangling, 1).v);
undefinedBehavior(() => {
	rs.ptrAdd(dangling, 1).v = 1;
});
undefinedBehavior(() => rs.ptrCopy(dangling, rs.arrayAsPtr(vec), 1));
undefinedBehavior(() => rs.ptrSlice(dangling, 1));
undefinedBehavior(() => rs.ptrAdd(rs.NULL as rs.Place<number>, 1).v);
assert.equal(vec[0], 7);

// A raw slice pointer read through once its `Vec` has shrunk past it, which Rust keeps
// the old values for, and one past the array, which Rust allows until a read.
const rows = [1, 2, 3, 4];
const all = new rs.RawSlice(rs.arrayAsPtr(rows), 4);
const tail = new rs.RawSlice(rs.arrayAsPtr(rows, 1), 3);
assert.equal(rs.sliceItems(all.deref())[3] + rs.sliceItems(tail.deref())[2], 8);
rows.length = 3;
unsure(() => all.deref());
unsure(() => tail.deref());
const long = new rs.RawSlice(rs.arrayAsPtr(rows), 10);
assert.equal(long.length, 10);
unsure(() => long.deref());

// A `*const ()` cast back to a pointer to another type than any cast to it was from, which
// Rust reads as bytes JS doesn't keep, and one no cast erased, such as a waker's `Arc`.
const note = { id: 1 };
const erased = rs.ptrErase(note, "raw::Note", 2);
assert.equal(erased, note);
assert.equal(rs.ptrUnerase(erased, "raw::Note", 2), note);
unsure(() => rs.ptrUnerase(erased, "u16", 2));
unsure(() => rs.ptrUnerase({ v: undefined }, "raw::Note", 2));
// Erased as two types, as a transparent wrapper and what it holds are, it is either.
rs.ptrErase(note, "raw::Wrapper", 2);
assert.equal(rs.ptrUnerase(erased, "raw::Wrapper", 2), note);
assert.equal(rs.ptrUnerase(erased, "raw::Note", 2), note);
assert.throws(() => rs.ptrUnerase(erased, "u16", 2), /`\*const \(\)` cast back to a pointer to `u16`, which points at `raw::Note` or `raw::Wrapper`/);
// Null at any type, and dangling at the address of its type's alignment, which a cast
// through a type of another alignment keeps.
assert.equal(rs.ptrUnerase(rs.ptrErase(rs.NULL, "raw::Note", 2), "u16", 2), rs.NULL);
assert.equal(rs.ptrUnerase(rs.ptrErase(rs.DANGLING, "raw::Note", 2), "u16", 2), rs.DANGLING);
assert.equal(rs.ptrErase(rs.DANGLING, "raw::Byte", 1), rs.DANGLING_UNIT);
const two = rs.ptrErase(rs.DANGLING, "raw::Note", 2);
assert.notEqual(two, rs.DANGLING_UNIT);
const aligned = rs.ptrUnerase(two, "raw::Byte", 1) as object;
assert.notEqual(aligned, rs.DANGLING);
assert.equal(rs.ptrErase(aligned, "raw::Byte", 1), two);
undefinedBehavior(() => (aligned as rs.Place<number>).v);

// `Arc::from_raw` of a pointer no `Arc::into_raw` or `as_ptr` gave, such as a box's, of an
// `Rc`'s, or of one whose value dropped, which Rust leaves undefined.
const task = new rs.Arc({ id: 1 });
const raw = rs.arcRaw(task, task.value, "Arc");
assert.equal(rs.arcFromRaw(raw, "Arc"), task);
undefinedBehavior(() => rs.arcFromRaw({ id: 1 }, "Arc"));
undefinedBehavior(() => rs.arcFromRaw(raw, "Rc"));
task[Symbol.dispose]();
assert.throws(() => rs.arcFromRaw(raw, "Arc"), /Arc::from_raw requires a pointer to a value not yet dropped/);
// Moved off its place by arithmetic and back, which is the same address.
const counted = new rs.Arc(7);
const at = rs.arcRaw(counted, rs.arcPlace(counted), "Rc");
assert.equal(rs.arcFromRaw(rs.ptrAdd(rs.ptrAdd(at, 1), -1), "Rc"), counted);
// A pointer to a value JS copies cast to `*const ()`, which only an `Arc`'s raw pointer is
// the one place of, moved off and back too, or one at no memory.
const flag = new rs.Arc(false);
const flagAt = rs.arcRaw(flag, rs.arcPlace(flag), "Arc");
assert.equal(rs.arcErase(rs.ptrAdd(rs.ptrAdd(flagAt, 1), -1), "bool", 1), flagAt);
assert.equal(rs.arcErase(rs.NULL, "bool", 1), rs.NULL);
assert.throws(() => rs.arcErase({ v: false }, "bool", 1), /only an `Arc`'s or `Rc`'s raw pointer keeps the place of/);

// A `str`'s UTF-8 through `as_ptr`, which JS encodes anew from its text: read through,
// moved by arithmetic, and viewed, but a write or a copy into it, which Rust leaves
// undefined, aborts, and a mutable slice of it, undefined under Stacked Borrows alone, fails.
const text = rs.strAsPtr("aé");
assert.equal(rs.ptrAdd(text, 2).v, 0xa9);
assert.deepEqual(Array.from(rs.byteSliceView(rs.ptrByteSlice(text, 3, true))), [0x61, 0xc3, 0xa9]);
assert.equal(rs.ptrOffsetFrom(rs.ptrWrappingAdd(text, 2, 1, 1), text), 2);
undefinedBehavior(() => {
	rs.ptrAdd(text, 1).v = 0;
});
undefinedBehavior(() => {
	rs.ptrWrappingAdd(text, 1, 1, 1).v = 0;
});
undefinedBehavior(() => rs.ptrCopy(rs.byteSliceAsPtr(rs.byteSliceFrom(Uint8Array.of(1))), rs.ptrAdd(text, 1), 1));
unsure(() => rs.ptrByteSlice(text, 1, false));
unsure(() => new rs.RawSlice(text, 3).derefBytes(false));
// None of it is nothing written, and two calls' pointers are two encodings.
rs.ptrCopy(rs.byteSliceAsPtr(rs.byteSliceFrom(Uint8Array.of(1))), text, 0);
assert.equal(rs.ptrByteSlice(text, 0, false).length, 0);
unsure(() => rs.ptrOffsetFrom(rs.strAsPtr("aé"), text));
