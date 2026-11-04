#!/usr/bin/env python3
"""Rewrite a fresh translation of the codec and task fixtures into one prototype route
of a `Vec`'s capacity, for `just ts bench --route <model>=<dir>` to time against main.
Prototype only, for moq-dev/rust2#555's decision; it goes before anything lands.

usage: vec-routes.py <dir> peak|gated|lazy|wrapper

<dir> holds `translated/` and `task/` as `just ts run` wrote them for the codec and task
fixtures. Each edit is what the translator would emit for that model, on the paths the
bench runs (push, pop, index, iterate, len, collect, sort, and the heap); every edit must
match exactly once.

- peak: a pop records the length it shortens from on the array, as `peak` (37a81708d).
- gated: peak, recorded only by a pop onto a length where the capacity read would drop.
- lazy: a `Vec` as `{ a, cap, peak }`, its items a plain array, which its slices are; a
  pop records `peak` in the object.
- wrapper: a `Vec` as `{ a, cap }`, `cap` std's exact capacity, which a push at capacity grows.
"""
import sys
from pathlib import Path

root, model = Path(sys.argv[1]), sys.argv[2]


def edit(path, pairs):
    p = root / path
    text = p.read_text()
    for old, new in pairs:
        n = text.count(old)
        if n != 1:
            sys.exit(f"{path}: {n} matches for {old!r}")
        text = text.replace(old, new)
    p.write_text(text)


RT_ANCHOR = "/** `Vec::with_capacity` of `size`-byte elements, which panics past what it may hold. */"

if model in ("peak", "gated"):
    # The per-pop tracking measured on 2026-11-02 (37a81708d), on today's runtime: a pop
    # records the length it shortens from, which a capacity read then grows from.
    for rt in ["translated/rt.ts", "task/rt.ts"]:
        edit(rt, [
            ("export interface Capped {\n\tcap?: number;\n}", "export interface Capped {\n\tcap?: number;\n\tpeak?: number;\n}"),
            (
                "export function vecPop<T>(v: T[], none: None): T | None {\n\treturn",
                (
                "export function vecPop<T>(v: T[]): T | undefined;\nexport function vecPop<T>(v: T[], none: None): T | None;\nexport function vecPop<T>(v: T[] & Capped, none?: None): T | None | undefined {\n\tcapKeep(v);\n\treturn"
                if model == "peak" else
                # A pop lowers the capacity its length implies only onto a power of two, unless an operation set `cap`.
                "export function vecPop<T>(v: T[]): T | undefined;\nexport function vecPop<T>(v: T[], none: None): T | None;\nexport function vecPop<T>(v: T[] & Capped, none?: None): T | None | undefined {\n\tif (capDrops(v.length - 1, v.cap)) capKeep(v);\n\treturn"
            ),
            ),
            (
                RT_ANCHOR,
                "/** `v`'s length recorded as its peak, before a step shortens it, so its capacity holds. */\n"
                "export function capKeep(v: { readonly length: number } & Capped): void {\n"
                "\tif (v.peak === undefined || v.peak < v.length) v.peak = v.length;\n}\n\n"
                "/** Whether a pop to `len` lowers the capacity read from it and `cap`: onto `cap` doubled, or a power of two without one. */\n"
                "export function capDrops(len: number, cap: number | undefined): boolean {\n"
                "\tif (!cap) return (len & (len - 1)) === 0;\n"
                "\tconst k = len / cap;\n"
                "\treturn k >= 1 && Number.isInteger(k) && (k & (k - 1)) === 0;\n}\n\n" + RT_ANCHOR,
            ),
        ])
    for c in ["translated/rt/collections.ts", "task/rt/collections.ts"]:
        edit(c, [
            ("\tif (d.length === 0) return none;\n\tlet item = d.pop() as T;", "\tif (d.length === 0) return none;\n\t" + ("capKeep(d);" if model == "peak" else "if (capDrops(d.length - 1, (d as Capped).cap)) capKeep(d);") + "\n\tlet item = d.pop() as T;"),
            ("type Capped, capHold,", "type Capped, capDrops, capHold, capKeep," if model == "gated" else "type Capped, capHold, capKeep,"),
        ])
    edit("translated/bench.ts", [
        ("option_unwrapOr$11c0we2(v.pop(), 0)", "option_unwrapOr$11c0we2(rt.vecPop(v), 0)"),
        ("const _52: rt.u32 | undefined = v.pop();", "const _52: rt.u32 | undefined = rt.vecPop(v);"),
    ])
elif model in ("wrapper", "lazy"):
    # A `Vec<T>` as an object over its items, `a`, a plain array, which a slice of it is,
    # so deref stays free. `wrapper` holds std's exact capacity as `cap`, which a push at
    # capacity grows; `lazy` holds the `cap` an operation set and the `peak` a pop recorded,
    # as fields, which a capacity read grows from, so a push is untouched.
    eager = model == "wrapper"
    more = "" if eager else ", peak: 0"
    fields = "cap: number;" if eager else "cap: number;\n\tpeak: number;"
    empty = "cap: size === 0 ? USIZE_MAX : 0" + more
    wrapper = (
        "/** `Vec<T>`: its items, `a`, which its slices are, and what gives its capacity. */\n"
        f"export interface Vec<T> {{\n\ta: T[];\n\t{fields}\n}}\n\n"
        "/** `Vec::new` of `size`-byte elements. */\n"
        f"export function vecNew<T>(size: number): Vec<T> {{\n\treturn {{ a: [], {empty} }};\n}}\n\n"
        "/** A `Vec` of `a`'s items, with the capacity the runtime holds for it, as a collect makes one. */\n"
        "export function vecFrom<T>(a: T[] & Capped, size: number): Vec<T> {\n"
        f"\treturn {{ a, cap: capOf(a.length, a.cap, size){more} }};\n}}\n\n"
        "/** `vec![..]` of `a`'s items: room for exactly them. */\n"
        f"export function vecOf<T>(a: T[], size: number): Vec<T> {{\n\treturn {{ a, cap: size === 0 ? USIZE_MAX : a.length{more} }};\n}}\n\n"
        "/** `mem::take` of a `Vec`: its items and capacity, leaving it new. */\n"
        "export function vecTake<T>(v: Vec<T>): Vec<T> {\n"
        f"\tconst out = {{ ...v }};\n\tv.a = [];\n\tv.cap = 0;\n{'' if eager else chr(9) + 'v.peak = 0;' + chr(10)}\treturn out;\n}}\n\n"
    )
    if eager:
        wrapper += (
            "/** `Vec::push` of `size`-byte elements, which grows a full one as std's `RawVec::grow_one`. */\n"
            "export function vecPush<T>(v: Vec<T>, item: T, size: number): void {\n"
            "\tif (v.a.length === v.cap) v.cap = capReserve(v.cap, v.cap, 1, size);\n\tv.a.push(item);\n}\n\n"
        )
    else:
        wrapper += (
            "/** `Vec::pop`, which records the length it shortens from, so the capacity holds. */\n"
            "export function vecPopLast<T>(v: Vec<T>): T | undefined {\n"
            "\tconst a = v.a;\n\tif (a.length > v.peak) v.peak = a.length;\n\treturn a.pop();\n}\n\n"
        )
    for rt in ["translated/rt.ts", "task/rt.ts"]:
        edit(rt, [(RT_ANCHOR, wrapper + RT_ANCHOR)])
    # A heap is a wrapper already, so it holds its `Vec`'s capacity beside its items.
    hf = "cap" if eager else "peak"
    heap = [
        ("export interface BinaryHeap<T> {\n\td: T[];\n}", f"export interface BinaryHeap<T> {{\n\td: T[];\n\t{hf}: number;\n}}"),
        ("export function heapNew<T>(): BinaryHeap<T> {\n\treturn { d: [] };", f"export function heapNew<T>(): BinaryHeap<T> {{\n\treturn {{ d: [], {hf}: 0 }};"),
        # The other constructors, which the bench never runs, only type-check.
        ("return { d: vecWithCapacity(n, size) };", f"return {{ d: vecWithCapacity(n, size), {hf}: {'n' if eager else '0'} }};"),
        ("rebuild(d, le, ge, lt);\n\treturn { d };", f"rebuild(d, le, ge, lt);\n\treturn {{ d, {hf}: {'d.length' if eager else '0'} }};"),
        ("return { d: vecExact(h.d.map((value) => item(value)), size) };", f"return {{ d: vecExact(h.d.map((value) => item(value)), size), {hf}: {'h.d.length' if eager else '0'} }};"),
    ]
    if eager:
        heap.append((
            "export function heapPush<T>(h: BinaryHeap<T>, item: T, le: Test<T>): void {\n\th.d.push(item);",
            "export function heapPush<T>(h: BinaryHeap<T>, item: T, le: Test<T>, size = 4): void {\n\tif (h.d.length === h.cap) h.cap = capReserve(h.cap, h.cap, 1, size);\n\th.d.push(item);",
        ))
    else:
        heap.append(("\tif (d.length === 0) return none;\n\tlet item = d.pop() as T;", "\tif (d.length === 0) return none;\n\tif (d.length > h.peak) h.peak = d.length;\n\tlet item = d.pop() as T;"))
    for c in ["translated/rt/collections.ts", "task/rt/collections.ts"]:
        edit(c, heap)

    def push(v, x, tabs):
        return "\t" * tabs + (f"rt.vecPush({v}, {x}, 4);" if eager else f"{v}.a.push({x});")

    def pop(v):
        return f"{v}.a.pop()" if eager else f"rt.vecPopLast({v})"

    edit("translated/bench.ts", [
        # order
        ("let v: rt.u32[] = rt.iterToArray(", "let v: rt.Vec<rt.u32> = rt.vecFrom(rt.iterToArray("),
        ("}), undefined, 4, true);\n\tif (selector === 0) {", "}), undefined, 4, true), 4);\n\tif (selector === 0) {"),
        ("rt.sliceSort(v, rt.lt, 4, true);", "rt.sliceSort(v.a, rt.lt, 4, true);"),
        ("rt.sliceSort(v, rt.keyLt(", "rt.sliceSort(v.a, rt.keyLt("),
        ("v = [rt.optionUnwrap(rt.iterMax(rt.iterSlice(v), rt.lt)), rt.optionUnwrap(rt.iterMin(rt.iterSlice(v), rt.lt))];",
         "v = rt.vecOf([rt.optionUnwrap(rt.iterMax(rt.iterSlice(v.a), rt.lt)), rt.optionUnwrap(rt.iterMin(rt.iterSlice(v.a), rt.lt))], 4);"),
        ("rt.iterFold(rt.iterSlice(v), [0, 4294967295]", "rt.iterFold(rt.iterSlice(v.a), [0, 4294967295]"),
        ("v = [_35[0], _35[1]];", "v = rt.vecOf([_35[0], _35[1]], 4);"),
        ("const _43: rt.Iter<[rt.u32, rt.u32]> = rt.iterMap(rt.iterSlice(v),", "const _43: rt.Iter<[rt.u32, rt.u32]> = rt.iterMap(rt.iterSlice(v.a),"),
        ("v = [rt.optionUnwrap(rt.iterToOption(rt.btreeKeys(map).next())), rt.btreeLen(map) as rt.u32, rt.optionUnwrap(rt.iterToOption(rt.btreeValues(map).nextBack()))];",
         "v = rt.vecOf([rt.optionUnwrap(rt.iterToOption(rt.btreeKeys(map).next())), rt.btreeLen(map) as rt.u32, rt.optionUnwrap(rt.iterToOption(rt.btreeValues(map).nextBack()))], 4);"),
        ("const _65: rt.Iter<rt.u32> = rt.iterMap(rt.iterSlice(v),", "const _65: rt.Iter<rt.u32> = rt.iterMap(rt.iterSlice(v.a),"),
        ("v = [rt.optionUnwrap(rt.btreeFirstKey(set)), rt.btreeLen(set) as rt.u32, rt.optionUnwrap(rt.btreeFirstKey(set, true))];",
         "v = rt.vecOf([rt.optionUnwrap(rt.btreeFirstKey(set)), rt.btreeLen(set) as rt.u32, rt.optionUnwrap(rt.btreeFirstKey(set, true))], 4);"),
        ("for (const _87 of rt.replaceIn(v, [])) {\n\t\t\trt.heapPush(heap, _87, rt.le);", "for (const _87 of rt.vecTake(v).a) {\n\t\t\trt.heapPush(heap, _87, rt.le" + (", 4" if eager else "") + ");"),
        ("\t\t\t\tv.push(_93);", push("v", "_93", 4)),
        ("const _103: rt.u32[] = v;", "const _103: rt.u32[] = v.a;"),
        ("const _106: rt.u32[] = v;", "const _106: rt.u32[] = v.a;"),
        ("rt.usizeDiv(v.length, 2))];\n\tconst _113: rt.u32[] = v;", "rt.usizeDiv(v.a.length, 2))];\n\tconst _113: rt.u32[] = v.a;"),
        ("rt.usizeSub(v.length, 1))]]);", "rt.usizeSub(v.a.length, 1))]]);"),
        # push/stack
        ("\t\tconst v: rt.u32[] = [];\n\t\tlet sum", "\t\tconst v: rt.Vec<rt.u32> = rt.vecNew(4);\n\t\tlet sum"),
        ("\t\t\tv.push(i2);", push("v", "i2", 3)),
        ("option_unwrapOr$11c0we2(v.pop(), 0)", "option_unwrapOr$11c0we2(" + pop("v") + ", 0)"),
        ("const _52: rt.u32 | undefined = v.pop();", "const _52: rt.u32 | undefined = " + pop("v") + ";"),
        ("_13 = [v.length, sum]", "_13 = [v.a.length, sum]"),
        # push/vec
        ("const v2: rt.u32[] = [];", "const v2: rt.Vec<rt.u32> = rt.vecNew(4);"),
        ("\t\t\tv2.push(_66);", push("v2", "_66", 3)),
        ("_13 = [v2.length, rt.sliceGet(v2, rt.usizeDiv(v2.length, 2))]", "_13 = [v2.a.length, rt.sliceGet(v2.a, rt.usizeDiv(v2.a.length, 2))]"),
    ])
else:
    sys.exit(f"unknown model {model}")
