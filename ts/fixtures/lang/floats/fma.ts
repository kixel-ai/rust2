// The one difference from native Rust that the `floats` fixture accepts in `floats::mul_adds`:
// wasm32's `fma`, compiler-builtins' generic libm, rounds some near-ties a last place off
// (rust-lang/compiler-builtins#1353, fixed by #1358). `/quest/m3/ts-fma-allowance.md`
// removes this once the toolchain ships the fix.

/** A finite double as `m * 2^e`, exactly. */
function parts(x: number): { m: bigint; e: number } {
	const view = new DataView(new ArrayBuffer(8));
	view.setFloat64(0, x);
	const bits = view.getBigUint64(0);
	const biased = Number((bits >> 52n) & 0x7ffn);
	const frac = bits & ((1n << 52n) - 1n);
	const m = biased === 0 ? frac : frac | (1n << 52n);
	return { m: bits >> 63n === 1n ? -m : m, e: Math.max(biased, 1) - 1075 };
}

/** `a * b + c` rounded once to nearest, ties to even, from exact integers; `undefined` if an operand is not finite. */
export function exactMulAdd(a: number, b: number, c: number): number | undefined {
	if (![a, b, c].every(Number.isFinite)) return undefined;
	const [pa, pb, pc] = [parts(a), parts(b), parts(c)];
	const [mp, ep] = [pa.m * pb.m, pa.e + pb.e];
	const e = Math.min(ep, pc.e);
	const sum = (mp << BigInt(ep - e)) + (pc.m << BigInt(pc.e - e));
	// An exact zero is -0 only if the product and `c` both are, so of zeros JS's sum is exact.
	if (sum === 0n) return a === 0 || b === 0 ? a * b + c : 0;
	const sign = sum < 0n ? -1 : 1;
	const mag = sum < 0n ? -sum : sum;
	// The last place of the result: 53 bits below its leading one, or a subnormal's.
	const lead = mag.toString(2).length - 1 + e;
	const q = Math.max(lead - 52, -1074);
	let kept = mag;
	if (q > e) {
		const shift = BigInt(q - e);
		kept = mag >> shift;
		const rest = mag - (kept << shift);
		const half = 1n << (shift - 1n);
		if (rest > half || (rest === half && (kept & 1n) === 1n)) kept += 1n;
	} else {
		kept = mag << BigInt(e - q);
	}
	// `kept` is at most 2^53 and `2^q` a double, so this product rounds only past the range, to infinity.
	return sign * Number(kept) * 2 ** q;
}

/** A finite double's bits as a line through zero, so the distance between two is how many doubles apart they are. */
function line(x: number): bigint {
	const view = new DataView(new ArrayBuffer(8));
	view.setFloat64(0, x);
	const bits = view.getBigInt64(0);
	return bits < 0n ? -(bits & ((1n << 63n) - 1n)) : bits;
}

/**
 * Whether `mul_adds`' outputs, each result after its operands as four big-endian doubles,
 * differ only where the translation's result is the exactly rounded one and native's is
 * a last place from it.
 */
export function mulAddsWithin(native: Uint8Array, translated: Uint8Array): boolean {
	if (native.length !== translated.length || native.length % 32 !== 0) return false;
	const [n, t] = [native, translated].map((v) => new DataView(v.buffer, v.byteOffset, v.length));
	for (let i = 0; i < native.length; i += 32) {
		const differs = (from: number, to: number) => native.subarray(from, to).some((x, j) => x !== translated[from + j]);
		if (differs(i, i + 24)) return false;
		if (!differs(i + 24, i + 32)) continue;
		const [a, b, c, want, got] = [t.getFloat64(i), t.getFloat64(i + 8), t.getFloat64(i + 16), n.getFloat64(i + 24), t.getFloat64(i + 24)];
		const exact = exactMulAdd(a, b, c);
		if (exact === undefined || !Object.is(got, exact) || !Number.isFinite(want) || !Number.isFinite(got)) return false;
		const d = line(want) - line(got);
		if (d !== 1n && d !== -1n) return false;
	}
	return true;
}
