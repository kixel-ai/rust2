// What reads the capacity of a `String`'s bytes, which JS keeps none of, throws a rust2-ts
// error, which native Rust can't show: there it is the `String`'s.
//
// usage: node capacity.ts

import { strict as assert } from "node:assert";

import * as rt from "../runtime/rt.ts";

const unsure = (f: () => void) => assert.throws(f, (e: Error) => !(e instanceof rt.Panic) && e.message.startsWith("rust2-ts: "));
const string = () => rt.bytesFromString(rt.stringAsBytes("text"));

// Taken by the one handle of it, as a `BytesMut` or a vector.
unsure(() => rt.bytesMutFromBytes(string()));
unsure(() => rt.bytesToByteVec(string()));
// Promotable only if the `String` had no room to spare, which gives the storage up.
unsure(() => rt.bytesTruncate({ v: string() }, 0 as rt.usize));

// Another handle of it holds it, so either copies.
const shared = string();
const other = rt.bytesClone(shared);
assert.equal(rt.bytesMutFromBytes(shared).cap, 4);
assert.equal(rt.bytesIsUnique(other), true);
// A clone is never promotable.
const truncated = { v: other };
rt.bytesTruncate(truncated, 0 as rt.usize);
assert.equal(rt.bytesIsUnique(truncated.v), true);
