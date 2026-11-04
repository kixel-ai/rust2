// std's slice sorts, for rust2-ts output: the stable driftsort and the unstable ipnsort,
// as `core::slice::sort` reads at the pinned toolchain on wasm32, so each calls `less` in
// the order and number std's does and leaves the slice as std's would, whatever `less`
// answers. `rs.ts` re-exports all of it.
//
// std picks its small sorts, partitions, and scratch by the element type's size, and by
// whether it is `Freeze` and `Copy`, which translation knows and passes. std's
// branchless selects and unrolled loops make the same calls in the same order as the
// plain loops here. A panic in `less` leaves the slice as it is, which wasm32, where a
// panic aborts, never shows.

import { byValue, panic, type SliceLike } from "../rs.ts";
import { drop as dispose } from "./task.ts";

type Items<T> = { [i: number]: T; length: number };
type Less<T> = (a: T, b: T) => boolean;

/** The sort of one slice: its elements and test, and what std picks by the type. */
type Sort<T> = {
	s: Items<T>;
	less: Less<T>;
	/** `size_of::<T>()` on wasm32. */
	size: number;
	/** Whether a pivot stands for the elements equal to it right of it, as a `Freeze` one's copy does in std's stable sort. */
	freeze: boolean;
	/** The largest slice `smallSort` takes. */
	small: number;
	smallSort: (c: Sort<T>, lo: number, hi: number) => void;
	/** The scratch std's would have, by its length, which decides when a merge waits. */
	buf: T[];
	room: number;
};

/** `sort`, `sort_by`, and `sort_by_key`: std's driftsort of `s` by `less`, of elements `size` bytes long on wasm32. */
export function sliceSort<T>(s: SliceLike<T>, less: Less<T>, size: number, freeze: boolean): void {
	byValue(s, (s) => driftsort(s, less, size, freeze));
}

function driftsort<T>(s: Items<T>, less: Less<T>, size: number, freeze: boolean): void {
	const n = s.length;
	if (size === 0 || n < 2) return;
	if (n <= 20) {
		insertionSort(s, 0, n, 1, less);
		return;
	}
	// The larger of half the slice and as much of it as 8 MB holds, or else the small sort's
	// 48, from the stack's 4 KiB where that holds it.
	const want = Math.max(n - Math.floor(n / 2), Math.min(n, Math.floor(8_000_000 / size)), 48);
	const stack = Math.floor(4096 / size);
	const smallSort = freeze ? smallSortGeneral : smallSortInsertion;
	const c: Sort<T> = { s, less, size, freeze, small: freeze ? 32 : 16, smallSort, buf: scratch(s, n + 16), room: stack >= want ? stack : want };
	drift(c, 0, n, n <= c.small * 2);
}

/** `sort_unstable` and its `_by` and `_by_key` forms: std's ipnsort of `s` by `less`, of elements `size` bytes long on wasm32. */
export function sliceSortUnstable<T>(s: SliceLike<T>, less: Less<T>, size: number, freeze: boolean, copy: boolean): void {
	byValue(s, (s) => ipnsort(s, less, size, freeze, copy));
}

function ipnsort<T>(s: Items<T>, less: Less<T>, size: number, freeze: boolean, copy: boolean): void {
	const n = s.length;
	if (size === 0 || n < 2) return;
	if (n <= 20) {
		insertionSort(s, 0, n, 1, less);
		return;
	}
	const run = existingRun(s, 0, n, less);
	if (Math.abs(run) === n) {
		if (run < 0) reverse(s, 0, n);
		return;
	}
	// The network sort for `Copy` values that fit a register, else the general one for those
	// whose 48 fit the stack's 4 KiB, else insertion sort.
	const smallSort = freeze && copy && size <= 8 ? smallSortNetwork : freeze && size * 48 <= 4096 ? smallSortGeneral : smallSortInsertion;
	const c: Sort<T> = { s, less, size, freeze, small: smallSort === smallSortInsertion ? 16 : 32, smallSort, buf: scratch(s, 48), room: 0 };
	quicksortUnstable(c, 0, n, false, undefined, 2 * log2(n));
}

/**
 * `sort_by_cached_key`: `key` of each element in order, paired with its index, and then
 * `key` dropped if the sort `owns` it, the pairs, `size` bytes long on wasm32, sorted by
 * `sort_unstable` by `lt`, which then permute `s`, and drop by `drop`.
 */
export function sliceSortByCachedKey<T, K>(s: SliceLike<T>, key: (item: T) => K, lt: Less<[K, number]>, size: number, freeze: boolean, copy: boolean, owns = false, drop?: (key: K) => void): void {
	const n = s.length;
	if (n < 2) {
		if (owns) dispose(key);
		return;
	}
	const indices: [K, number][] = [];
	for (let i = 0; i < n; i++) indices.push([key(s.data[s.start + i]), i]);
	if (owns) dispose(key);
	ipnsort(indices, lt, size, freeze, copy);
	byValue(s, (s) => {
		for (let i = 0; i < n; i++) {
			let index = indices[i][1];
			while (index < i) index = indices[index][1];
			indices[i][1] = index;
			const t = s[i];
			s[i] = s[index];
			s[index] = t;
		}
	});
	if (drop !== undefined) for (const [k] of indices) drop(k);
}

/** `sort_by_key`'s test: `lt` of the keys `key` makes of each, the first's first. */
export function keyLt<T, K>(key: (item: T) => K, lt: Less<K>): Less<T> {
	return (a, b) => lt(key(a), key(b));
}

/** Scratch for `n` elements of `s`, filled so that engines keep it a dense array. */
function scratch<T>(s: Items<T>, n: number): T[] {
	return new Array<T>(n).fill(s[0]);
}

function log2(n: number): number {
	return 31 - Math.clz32(n | 1);
}

function reverse<T>(s: Items<T>, lo: number, hi: number): void {
	for (hi--; lo < hi; lo++, hi--) {
		const x = s[lo];
		s[lo] = s[hi];
		s[hi] = x;
	}
}

function swap<T>(s: Items<T>, a: number, b: number): void {
	const x = s[a];
	s[a] = s[b];
	s[b] = x;
}

/** The run `s[lo..hi]` starts with: its length, negative where it strictly descends. */
function existingRun<T>(s: Items<T>, lo: number, hi: number, less: Less<T>): number {
	const n = hi - lo;
	if (n < 2) return n;
	let run = 2;
	if (less(s[lo + 1], s[lo])) {
		while (run < n && less(s[lo + run], s[lo + run - 1])) run++;
		return -run;
	}
	while (run < n && !less(s[lo + run], s[lo + run - 1])) run++;
	return run;
}

/** Sorts `v[begin..=tail]` where `v[begin..tail]` is sorted. */
function insertTail<T>(v: Items<T>, begin: number, tail: number, less: Less<T>): void {
	let sift = tail - 1;
	if (!less(v[tail], v[sift])) return;
	const x = v[tail];
	let gap = tail;
	for (;;) {
		v[gap] = v[sift];
		gap = sift;
		if (sift === begin) break;
		sift--;
		if (!less(x, v[sift])) break;
	}
	v[gap] = x;
}

/** The small sort of a type with no other: insertion. */
function smallSortInsertion<T>(c: Sort<T>, lo: number, hi: number): void {
	if (hi - lo >= 2) insertionSort(c.s, lo, hi, 1, c.less);
}

/** Sorts `v[lo..hi]` where `v[lo..lo + sorted]` is. */
function insertionSort<T>(v: Items<T>, lo: number, hi: number, sorted: number, less: Less<T>): void {
	for (let tail = lo + sorted; tail < hi; tail++) insertTail(v, lo, tail, less);
}

// driftsort: runs found or made, merged in powersort's order, the unsorted ones left for
// a stable quicksort until a merge needs them sorted.

/** A run's length, doubled, plus one if sorted. */
type Run = number;

function drift<T>(c: Sort<T>, lo: number, hi: number, eager: boolean): void {
	const n = hi - lo;
	if (n < 2) return;
	const scale = (2n ** 62n + BigInt(n) - 1n) / BigInt(n);
	const minRun = n <= 64 * 64 ? Math.min(n - Math.floor(n / 2), 64) : sqrtApprox(n);
	const runs: Run[] = [];
	const depths: number[] = [];
	let scan = 0;
	let prev: Run = 1;
	for (;;) {
		let next: Run = 1;
		let depth = 0;
		if (scan < n) {
			next = createRun(c, lo + scan, hi, minRun, eager);
			depth = mergeDepth(scan - runLen(prev), scan, scan + runLen(next), scale);
		}
		while (runs.length > 1 && depths[depths.length - 1] >= depth) {
			const left = runs.pop() as Run;
			depths.pop();
			const start = lo + scan - runLen(left) - runLen(prev);
			prev = logicalMerge(c, start, lo + scan, left, prev);
		}
		runs.push(prev);
		depths.push(depth);
		if (scan >= n) break;
		scan += runLen(next);
		prev = next;
	}
	if (prev % 2 === 0) stableQuicksort(c, lo, hi);
}

function runLen(r: Run): number {
	return Math.floor(r / 2);
}

/** The depth powersort wants for the merge of `[left, mid)` and `[mid, right)`, as std's scales it to 2^62. */
function mergeDepth(left: number, mid: number, right: number, scale: bigint): number {
	const x = (scale * BigInt(left + mid)) ^ (scale * BigInt(mid + right));
	const high = Number(x >> 32n);
	return high !== 0 ? Math.clz32(high) : 32 + Math.clz32(Number(x & 0xffffffffn));
}

function sqrtApprox(n: number): number {
	const shift = Math.ceil(log2(n) / 2);
	return Math.floor((2 ** shift + (n >>> shift)) / 2);
}

function logicalMerge<T>(c: Sort<T>, lo: number, hi: number, left: Run, right: Run): Run {
	const n = hi - lo;
	const mid = lo + runLen(left);
	if (n > c.room || left % 2 === 1 || right % 2 === 1) {
		if (left % 2 === 0) stableQuicksort(c, lo, mid);
		if (right % 2 === 0) stableQuicksort(c, mid, hi);
		merge(c, lo, hi, mid);
		return 2 * n + 1;
	}
	return 2 * n;
}

/** A sorted run at `lo` at least `minRun` long, or else one small-sorted if `eager`, or else an unsorted one. */
function createRun<T>(c: Sort<T>, lo: number, hi: number, minRun: number, eager: boolean): Run {
	const n = hi - lo;
	if (n >= minRun) {
		const run = existingRun(c.s, lo, hi, c.less);
		if (Math.abs(run) >= minRun) {
			if (run < 0) reverse(c.s, lo, lo - run);
			return 2 * Math.abs(run) + 1;
		}
	}
	if (eager) {
		const len = Math.min(c.small, n);
		stableQuicksortFrom(c, lo, lo + len, 0, false, undefined);
		return 2 * len + 1;
	}
	return 2 * Math.min(minRun, n);
}

/** Merges the sorted `s[lo..mid]` and `s[mid..hi]`, the shorter through scratch. */
function merge<T>(c: Sort<T>, lo: number, hi: number, mid: number): void {
	const { s, less, buf } = c;
	if (mid === lo || mid >= hi || c.room < Math.min(mid - lo, hi - mid)) return;
	if (mid - lo <= hi - mid) {
		const end = mid - lo;
		for (let i = 0; i < end; i++) buf[i] = s[lo + i];
		let l = 0;
		let r = mid;
		let out = lo;
		while (l !== end && r !== hi) s[out++] = less(s[r], buf[l]) ? s[r++] : buf[l++];
		while (l !== end) s[out++] = buf[l++];
		return;
	}
	let end = hi - mid;
	for (let i = 0; i < end; i++) buf[i] = s[mid + i];
	let dst = mid;
	let out = hi;
	for (;;) {
		out--;
		if (less(buf[end - 1], s[dst - 1])) s[out] = s[--dst];
		else s[out] = buf[--end];
		if (dst === lo || end === 0) break;
	}
	for (let i = 0; i < end; i++) s[dst + i] = buf[i];
}

function stableQuicksort<T>(c: Sort<T>, lo: number, hi: number): void {
	stableQuicksortFrom(c, lo, hi, 2 * log2(hi - lo), false, undefined);
}

/**
 * Sorts `s[lo..hi]`, after `limit` more bad pivots by an eager driftsort. A `Freeze`
 * type's last pivot to the left, `ancestor`, lets a run of elements equal to it partition
 * once.
 */
function stableQuicksortFrom<T>(c: Sort<T>, lo: number, hi: number, limit: number, hasAncestor: boolean, ancestor: T | undefined): void {
	for (;;) {
		if (hi - lo <= c.small) {
			c.smallSort(c, lo, hi);
			return;
		}
		if (limit === 0) {
			drift(c, lo, hi, true);
			return;
		}
		limit--;
		const p = choosePivot(c.s, lo, hi, c.less);
		const pivot = c.s[p];
		let equal = hasAncestor && !c.less(ancestor as T, pivot);
		let left = 0;
		if (!equal) {
			left = stablePartition(c, lo, hi, p, false, false);
			equal = left === 0;
		}
		if (equal) {
			lo += stablePartition(c, lo, hi, p, true, true);
			hasAncestor = false;
			continue;
		}
		stableQuicksortFrom(c, lo + left, hi, limit, c.freeze, pivot);
		hi = lo + left;
	}
}

/**
 * Moves the elements of `s[lo..hi]` less than `s[p]` before the rest, each side in order,
 * and returns how many: by `less(x, pivot)`, or else by `!less(pivot, x)` if `equal`.
 */
function stablePartition<T>(c: Sort<T>, lo: number, hi: number, p: number, pivotLeft: boolean, equal: boolean): number {
	const { s, less, buf } = c;
	const n = hi - lo;
	const pivot = s[p];
	// Those less fill scratch from the front, the rest from the back, and come back in order.
	let left = 0;
	for (let i = lo; i < hi; i++) {
		const x = s[i];
		// The pivot is never compared with itself.
		const goes = i === p ? pivotLeft : equal ? !less(pivot, x) : less(x, pivot);
		if (goes) buf[left++] = x;
		else buf[n - 1 - (i - lo - left)] = x;
	}
	for (let i = 0; i < left; i++) s[lo + i] = buf[i];
	for (let i = left; i < n; i++) s[lo + i] = buf[n - 1 - i + left];
	return left;
}

/** The index of a pseudomedian of `s[lo..hi]`, which holds at least 8: of 3, or of 3 of 3 recursively past 64. */
function choosePivot<T>(s: Items<T>, lo: number, hi: number, less: Less<T>): number {
	const n = hi - lo;
	const eighth = Math.floor(n / 8);
	const b = lo + eighth * 4;
	const c = lo + eighth * 7;
	return n < 64 ? median3(s, lo, b, c, less) : median3Rec(s, lo, b, c, eighth, less);
}

function median3Rec<T>(s: Items<T>, a: number, b: number, c: number, n: number, less: Less<T>): number {
	if (n * 8 >= 64) {
		const eighth = Math.floor(n / 8);
		a = median3Rec(s, a, a + eighth * 4, a + eighth * 7, eighth, less);
		b = median3Rec(s, b, b + eighth * 4, b + eighth * 7, eighth, less);
		c = median3Rec(s, c, c + eighth * 4, c + eighth * 7, eighth, less);
	}
	return median3(s, a, b, c, less);
}

function median3<T>(s: Items<T>, a: number, b: number, c: number, less: Less<T>): number {
	const x = less(s[a], s[b]);
	const y = less(s[a], s[c]);
	if (x !== y) return a;
	return less(s[b], s[c]) !== x ? c : b;
}

/**
 * Sorts `s[lo..hi]`, of at most 32, by sorting each half into scratch, by 8 or 4 at once
 * and then inserting the rest, and merging them back.
 */
function smallSortGeneral<T>(c: Sort<T>, lo: number, hi: number): void {
	const { s, less, buf } = c;
	const n = hi - lo;
	if (n < 2) return;
	const half = Math.floor(n / 2);
	let sorted = 1;
	if (c.size <= 16 && n >= 16) {
		sort8(s, lo, buf, 0, buf, n, less);
		sort8(s, lo + half, buf, half, buf, n + 8, less);
		sorted = 8;
	} else if (n >= 8) {
		sort4(s, lo, buf, 0, less);
		sort4(s, lo + half, buf, half, less);
		sorted = 4;
	} else {
		buf[0] = s[lo];
		buf[half] = s[lo + half];
	}
	for (let i = sorted; i < half; i++) {
		buf[i] = s[lo + i];
		insertTail(buf, 0, i, less);
	}
	for (let i = sorted; i < n - half; i++) {
		buf[half + i] = s[lo + half + i];
		insertTail(buf, half, half + i, less);
	}
	bidirectionalMerge(buf, 0, n, s, lo, less);
}

/** Sorts `v[i..i + 4]` stably into `dst[j..j + 4]` in 5 tests. */
function sort4<T>(v: Items<T>, i: number, dst: Items<T>, j: number, less: Less<T>): void {
	const c1 = less(v[i + 1], v[i]);
	const c2 = less(v[i + 3], v[i + 2]);
	const a = c1 ? i + 1 : i;
	const b = c1 ? i : i + 1;
	const c = c2 ? i + 3 : i + 2;
	const d = c2 ? i + 2 : i + 3;
	const c3 = less(v[c], v[a]);
	const c4 = less(v[d], v[b]);
	const min = c3 ? c : a;
	const max = c4 ? b : d;
	const left = c3 ? a : c4 ? c : b;
	const right = c4 ? d : c3 ? b : c;
	const c5 = less(v[right], v[left]);
	dst[j] = v[min];
	dst[j + 1] = v[c5 ? right : left];
	dst[j + 2] = v[c5 ? left : right];
	dst[j + 3] = v[max];
}

/** Sorts `v[i..i + 8]` stably into `dst[j..j + 8]`, through `tmp[k..k + 8]`. */
function sort8<T>(v: Items<T>, i: number, dst: Items<T>, j: number, tmp: Items<T>, k: number, less: Less<T>): void {
	sort4(v, i, tmp, k, less);
	sort4(v, i + 4, tmp, k + 4, less);
	bidirectionalMerge(tmp, k, 8, dst, j, less);
}

/**
 * Merges the sorted halves of `src[i..i + n]` into `dst[j..j + n]` from both ends at
 * once, and panics as std's does if they don't meet, as only a `less` that is no strict
 * weak order makes them.
 */
function bidirectionalMerge<T>(src: Items<T>, i: number, n: number, dst: Items<T>, j: number, less: Less<T>): void {
	const half = Math.floor(n / 2);
	let left = i;
	let right = i + half;
	let out = j;
	let leftRev = i + half - 1;
	let rightRev = i + n - 1;
	let outRev = j + n - 1;
	for (let k = 0; k < half; k++) {
		if (less(src[right], src[left])) dst[out++] = src[right++];
		else dst[out++] = src[left++];
		if (less(src[rightRev], src[leftRev])) dst[outRev--] = src[leftRev--];
		else dst[outRev--] = src[rightRev--];
	}
	const leftEnd = leftRev + 1;
	const rightEnd = rightRev + 1;
	if (n % 2 === 1) {
		if (left < leftEnd) dst[out] = src[left++];
		else dst[out] = src[right++];
	}
	if (left !== leftEnd || right !== rightEnd) panic("user-provided comparison function does not correctly implement a total order", "ord_violation");
}

// ipnsort: a quicksort of in-place partitions, after too many bad pivots a heapsort.

/**
 * Sorts `s[lo..hi]`, after `limit` more bad pivots by heapsort. The last pivot to the left,
 * `ancestor`, lets a run of elements equal to it partition once.
 */
function quicksortUnstable<T>(c: Sort<T>, lo: number, hi: number, hasAncestor: boolean, ancestor: T | undefined, limit: number): void {
	const { s, less } = c;
	for (;;) {
		if (hi - lo <= c.small) {
			c.smallSort(c, lo, hi);
			return;
		}
		if (limit === 0) {
			heapsort(s, lo, hi, less);
			return;
		}
		limit--;
		const p = choosePivot(s, lo, hi, less);
		if (hasAncestor && !less(ancestor as T, s[p])) {
			lo += partition(c, lo, hi, p, true) + 1;
			hasAncestor = false;
			continue;
		}
		const lt = partition(c, lo, hi, p, false);
		quicksortUnstable(c, lo, lo + lt, hasAncestor, ancestor, limit);
		ancestor = s[lo + lt];
		hasAncestor = true;
		lo += lt + 1;
	}
}

/**
 * Moves `s[p]` to where it sorts in `s[lo..hi]`, the elements less than it before it and
 * the rest after, and returns how many are less: by `less(x, pivot)`, or else by
 * `!less(pivot, x)` if `equal`. std's partitions types up to 96 bytes by Lomuto's scheme
 * and larger ones by Hoare's, each through a cyclic permutation.
 */
function partition<T>(c: Sort<T>, lo: number, hi: number, p: number, equal: boolean): number {
	const { s, less } = c;
	swap(s, lo, p);
	const pivot = s[lo];
	const test = (x: T) => (equal ? !less(pivot, x) : less(x, pivot));
	const lt = c.size <= 96 ? lomuto(s, lo + 1, hi, test) : hoare(s, lo + 1, hi, test);
	swap(s, lo, lo + lt);
	return lt;
}

function lomuto<T>(s: Items<T>, lo: number, hi: number, test: (x: T) => boolean): number {
	if (lo === hi) return 0;
	const first = s[lo];
	let gap = lo;
	let lt = 0;
	for (let right = lo + 1; right <= hi; right++) {
		// After the last element, the first, which the gap's cycle started from.
		const x = right === hi ? first : s[right];
		const goes = test(x);
		const left = lo + lt;
		s[gap] = s[left];
		s[left] = x;
		gap = right;
		if (goes) lt++;
	}
	return lt;
}

function hoare<T>(s: Items<T>, lo: number, hi: number, test: (x: T) => boolean): number {
	let left = lo;
	let right = hi;
	let gap = -1;
	let held: T | undefined;
	for (;;) {
		while (left < right && test(s[left])) left++;
		do right--;
		while (left < right && !test(s[right]));
		if (left >= right) break;
		if (gap < 0) held = s[left];
		else s[gap] = s[left];
		gap = right;
		s[left++] = s[right];
	}
	if (gap >= 0) s[gap] = held as T;
	return left - lo;
}

function heapsort<T>(s: Items<T>, lo: number, hi: number, less: Less<T>): void {
	const n = hi - lo;
	for (let i = n + Math.floor(n / 2) - 1; i >= 0; i--) {
		let node = 0;
		if (i >= n) node = i - n;
		else swap(s, lo, lo + i);
		const end = Math.min(i, n);
		for (;;) {
			let child = 2 * node + 1;
			if (child >= end) break;
			if (child + 1 < end && less(s[lo + child], s[lo + child + 1])) child++;
			if (!less(s[lo + node], s[lo + child])) break;
			swap(s, lo + node, lo + child);
			node = child;
		}
	}
}

/**
 * Sorts `s[lo..hi]`, of at most 32, by optimal networks of 13 or 9 and insertion, each
 * half separately past 17 and then merged through scratch.
 */
function smallSortNetwork<T>(c: Sort<T>, lo: number, hi: number): void {
	const { s, less, buf } = c;
	const n = hi - lo;
	if (n < 2) return;
	if (n < 18) {
		networkSort(s, lo, hi, less);
		return;
	}
	const mid = lo + Math.floor(n / 2);
	networkSort(s, lo, mid, less);
	networkSort(s, mid, hi, less);
	bidirectionalMerge(s, lo, n, buf, 0, less);
	for (let i = 0; i < n; i++) s[lo + i] = buf[i];
}

/** Sorts `s[lo..hi]` by a network of its first 13 or 9, and then insertion. */
function networkSort<T>(s: Items<T>, lo: number, hi: number, less: Less<T>): void {
	let sorted = 1;
	if (hi - lo >= 13) {
		network(s, lo, SORT13, less);
		sorted = 13;
	} else if (hi - lo >= 9) {
		network(s, lo, SORT9, less);
		sorted = 9;
	}
	insertionSort(s, lo, hi, sorted, less);
}

function network<T>(s: Items<T>, lo: number, pairs: readonly number[], less: Less<T>): void {
	for (let i = 0; i < pairs.length; i += 2) {
		const a = lo + pairs[i];
		const b = lo + pairs[i + 1];
		if (less(s[b], s[a])) swap(s, a, b);
	}
}

/** The optimal networks std's takes from <https://bertdobbelaere.github.io/sorting_networks.html>, as pairs. */
const SORT9 = [0, 3, 1, 7, 2, 5, 4, 8, 0, 7, 2, 4, 3, 8, 5, 6, 0, 2, 1, 3, 4, 5, 7, 8, 1, 4, 3, 6, 5, 7, 0, 1, 2, 4, 3, 5, 6, 8, 2, 3, 4, 5, 6, 7, 1, 2, 3, 4, 5, 6] as const;
const SORT13 = [
	0, 12, 1, 10, 2, 9, 3, 7, 5, 11, 6, 8, 1, 6, 2, 3, 4, 11, 7, 9, 8, 10, 0, 4, 1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 4, 6, 5, 9, 8, 11, 10, 12, 0, 5, 3, 8, 4, 7, 6, 11, 9, 10, 0, 1, 2, 5, 6, 9, 7, 8, 10, 11, 1, 3, 2, 4, 5, 6, 9, 10, 1, 2, 3, 4, 5, 7, 6, 8, 2, 3, 4, 5, 6, 7, 8, 9, 3, 4, 5, 6,
] as const;
