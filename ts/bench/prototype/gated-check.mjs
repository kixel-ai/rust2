// Prototype only, for moq-dev/rust2#555: `node gated-check.mjs` checks vec-routes.py's `gated` gate.
// Random pushes and pops, with and without a `cap` set first: the gated pop's capacity
// equals the per-pop peak's after every step.
function capOf(len, cap, size) {
	let c = cap ?? 0;
	while (c < len) c = Math.max(c * 2, size === 1 ? 8 : size <= 1024 ? 4 : 1);
	return c;
}
const capacity = (v, size) => capOf(v.peak !== undefined && v.peak > v.length ? v.peak : v.length, v.cap, size);
const keep = (v) => { if (v.peak === undefined || v.peak < v.length) v.peak = v.length; };
function capDrops(len, cap) {
	if (!cap) return (len & (len - 1)) === 0;
	const k = len / cap;
	return k >= 1 && Number.isInteger(k) && (k & (k - 1)) === 0;
}
let seed = 7;
const rand = (n) => ((seed = (seed * 1103515245 + 12345) >>> 0) % n);
let checks = 0;
for (const size of [4, 1100, 1]) for (let trial = 0; trial < 2000; trial++) {
	const full = [], gated = [];
	if (trial % 3 === 0) full.cap = gated.cap = rand(40);
	for (let step = 0; step < 400; step++) {
		if (rand(3) > 0 || full.length === 0) { full.push(0); gated.push(0); }
		else {
			keep(full); full.pop();
			if (capDrops(gated.length - 1, gated.cap)) keep(gated);
			gated.pop();
		}
		if (capacity(full, size) !== capacity(gated, size)) throw new Error(`size ${size} trial ${trial} step ${step}: ${capacity(full, size)} vs ${capacity(gated, size)}`);
		checks++;
	}
}
console.log(`ok, ${checks} steps`);
// Pops that keep, by gate, against those that had to: a gate that keeps too often costs writes.
let kept = 0, needed = 0;
seed = 9;
for (let trial = 0; trial < 2000; trial++) {
	const v = [];
	const cap = 1 + rand(40);
	for (let step = 0; step < 400; step++) {
		if (rand(3) > 0 || v.length === 0) v.push(0);
		else {
			if (capDrops(v.length - 1, cap)) kept++;
			if (capOf(v.length - 1, cap, 4) < capOf(v.length, cap, 4)) needed++;
			v.pop();
		}
	}
}
console.log(`with cap: ${kept} pops kept, ${needed} lowered the capacity`);
