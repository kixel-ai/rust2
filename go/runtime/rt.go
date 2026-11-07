// The runtime of rust2-go's output: what std does that Go does not, in the package of
// the translation. Its enums are laid out as the translation lays out any enum, a tag
// and a field of each variant's fields, so translated code reaches them as its own.

package rt

import (
	"encoding/binary"
	"fmt"
	"math/bits"
	"slices"
	"sync"
	"unsafe"
)

type tuple1[A any] struct{ F0 A }
type tuple2[A, B any] struct {
	F0 A
	F1 B
}
type tuple3[A, B, C any] struct {
	F0 A
	F1 B
	F2 C
}
type tuple4[A, B, C, D any] struct {
	F0 A
	F1 B
	F2 C
	F3 D
}
type tuple5[A, B, C, D, E any] struct {
	F0 A
	F1 B
	F2 C
	F3 D
	F4 E
}
type tuple6[A, B, C, D, E, F any] struct {
	F0 A
	F1 B
	F2 C
	F3 D
	F4 E
	F5 F
}
type tuple7[A, B, C, D, E, F, G any] struct {
	F0 A
	F1 B
	F2 C
	F3 D
	F4 E
	F5 F
	F6 G
}
type tuple8[A, B, C, D, E, F, G, H any] struct {
	F0 A
	F1 B
	F2 C
	F3 D
	F4 E
	F5 F
	F6 G
	F7 H
}

// Option is Rust's `Option<T>`: tag 0 is `None`, 1 is `Some`.
type Option[T any] struct {
	tag  int
	Some struct{ F0 T }
}

// Result is Rust's `Result<T, E>`: tag 0 is `Ok`, 1 is `Err`.
type Result[T, E any] struct {
	tag int
	Ok  struct{ F0 T }
	Err struct{ F0 E }
}

// Ordering is Rust's `cmp::Ordering`: -1, 0, or 1.
type Ordering struct{ tag int8 }

// fmtError is Rust's `fmt::Error`.
type fmtError struct{}

// RustPanic is what a translated panic panics with: native Rust's message. As with
// `panic = "abort"`, nothing translated runs cleanup on the way out, so a caller that
// recovers one must not use what the translation held.
type RustPanic struct{ Message string }

func (p RustPanic) Error() string { return p.Message }

func rustPanic(msg string) RustPanic { return RustPanic{msg} }

func panicBounds(len, index uint) RustPanic {
	return RustPanic{fmt.Sprintf("index out of bounds: the len is %d but the index is %d", len, index)}
}

type signed interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64
}

type unsigned interface {
	~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr
}

type integer interface{ signed | unsigned }

func addU[T unsigned](a, b T) tuple2[T, bool] {
	r := a + b
	return tuple2[T, bool]{r, r < a}
}

func subU[T unsigned](a, b T) tuple2[T, bool] {
	return tuple2[T, bool]{a - b, b > a}
}

func mulU[T unsigned](a, b T) tuple2[T, bool] {
	r := a * b
	return tuple2[T, bool]{r, a != 0 && r/a != b}
}

func addS[T signed](a, b T) tuple2[T, bool] {
	r := a + b
	return tuple2[T, bool]{r, (a >= 0) == (b >= 0) && (r >= 0) != (a >= 0)}
}

func subS[T signed](a, b T) tuple2[T, bool] {
	r := a - b
	return tuple2[T, bool]{r, (a >= 0) != (b >= 0) && (r >= 0) != (a >= 0)}
}

func mulS[T signed](a, b T) tuple2[T, bool] {
	r := a * b
	if a == 0 || b == 0 {
		return tuple2[T, bool]{r, false}
	}
	// The product overflowed if dividing it back fails, or if it is the minimum times -1,
	// which Go's division gives back unchanged.
	min := ^T(0) << (bitsOf[T]() - 1)
	over := r/b != a || (a == -1 && b == min) || (b == -1 && a == min)
	return tuple2[T, bool]{r, over}
}

func bitsOf[T integer]() uint {
	var x T
	switch any(x).(type) {
	case int8, uint8:
		return 8
	case int16, uint16:
		return 16
	case int32, uint32:
		return 32
	default:
		return 64
	}
}

func satAddU[T unsigned](a, b T) T {
	if r := a + b; r >= a {
		return r
	}
	return ^T(0)
}

func satSubU[T unsigned](a, b T) T {
	if b > a {
		return 0
	}
	return a - b
}

func satAddS[T signed](a, b T) T {
	r := addS(a, b)
	if !r.F1 {
		return r.F0
	}
	max := ^(^T(0) << (bitsOf[T]() - 1))
	if a < 0 {
		return ^max
	}
	return max
}

func satSubS[T signed](a, b T) T {
	r := subS(a, b)
	if !r.F1 {
		return r.F0
	}
	max := ^(^T(0) << (bitsOf[T]() - 1))
	if a < 0 {
		return ^max
	}
	return max
}

func cmp3[T integer](a, b T) Ordering {
	switch {
	case a < b:
		return Ordering{-1}
	case a > b:
		return Ordering{1}
	}
	return Ordering{0}
}

func boolTo[T integer](b bool) T {
	if b {
		return 1
	}
	return 0
}

func replace[T any](p *T, v T) T {
	old := *p
	*p = v
	return old
}

func ptrTo[T any](v T) *T { return &v }

// val is v as a value, not a constant, for an expression Go would otherwise fold.
func val[T any](v T) T { return v }

func clz[T integer](x T) uint {
	w := bitsOf[T]()
	return uint(bits.LeadingZeros64(uint64(x)&(^uint64(0)>>(64-w)))) - (64 - w)
}

func ctz[T integer](x T) uint {
	if x == 0 {
		return bitsOf[T]()
	}
	return uint(bits.TrailingZeros64(uint64(x)))
}

func popcount[T integer](x T) uint {
	return uint(bits.OnesCount64(uint64(x) & (^uint64(0) >> (64 - bitsOf[T]()))))
}

func bswap[T integer](x T) T {
	switch bitsOf[T]() {
	case 8:
		return x
	case 16:
		return T(bits.ReverseBytes16(uint16(x)))
	case 32:
		return T(bits.ReverseBytes32(uint32(x)))
	}
	return T(bits.ReverseBytes64(uint64(x)))
}

func rotl[T integer](x T, n uint32) T {
	w := bitsOf[T]()
	m := uint64(x) & (^uint64(0) >> (64 - w))
	s := uint(n) % w
	return T(m<<s | m>>((w-s)%w))
}

func rotr[T integer](x T, n uint32) T {
	w := bitsOf[T]()
	return rotl(x, uint32((w-uint(n)%w)%w))
}

// sliceIter is Rust's `slice::Iter<T>`: the slice it has left.
type sliceIter[T any] struct{ s []T }

func sliceIterNext[T any](it *sliceIter[T]) Option[*T] {
	if len(it.s) == 0 {
		return Option[*T]{}
	}
	p := &it.s[0]
	it.s = it.s[1:]
	return Option[*T]{tag: 1, Some: struct{ F0 *T }{p}}
}

func vecPop[T any](v *[]T) Option[T] {
	n := len(*v)
	if n == 0 {
		return Option[T]{}
	}
	x := (*v)[n-1]
	zero(&(*v)[n-1])
	*v = (*v)[:n-1]
	return Option[T]{tag: 1, Some: struct{ F0 T }{x}}
}

// vecWithCapacity is `Vec::with_capacity` of elements Rust lays out in `size` bytes, which
// panics where Rust's would allocate more than `isize::MAX` bytes. Its capacity is only a
// hint: Go's growth is not Rust's, so the translation reads no capacity back.
func vecWithCapacity[T any](n uint, size uint64) []T {
	if size == 0 {
		// Rust allocates nothing for any capacity of these.
		return make([]T, 0)
	}
	if uint64(n) > (1<<63-1)/size {
		panic(RustPanic{"capacity overflow"})
	}
	return make([]T, 0, n)
}

// vecPush is `Vec::push`.
func vecPush[T any](v *[]T, x T) {
	if len(*v) == cap(*v) {
		vecGrow(v, 1)
	}
	*v = append(*v, x)
}

// vecExtend is `Vec::extend_from_slice`.
func vecExtend[T any](v *[]T, xs []T) {
	if len(xs) > cap(*v)-len(*v) {
		vecGrow(v, len(xs))
	}
	*v = append(*v, xs...)
}

// vecGrow makes room for n more elements as std's `RawVec` does, to at least twice the
// capacity, where `append` grows a slice past 256 elements by about 1.25x and so copies
// a growing buffer several times as often.
func vecGrow[T any](v *[]T, n int) {
	*v = slices.Grow(*v, max(2*cap(*v), len(*v)+n)-len(*v))
}

// A `Vec<u8>`'s storage is the translation's own, to grow without zeroing and to recycle
// once outgrown, until Go code outside the translation can reach it. Own storage is told
// apart by its shape: a capacity that is a power of two of at least 8, never filled, since
// own growth always leaves a byte spare. A root function hands Go storage in through
// bytesEnter, which takes a Go slice of that capacity off it, and own storage out through
// bytesEscape, which clips it to its length, so storage Go can reach never has the shape
// and is never recycled. Empty storage, such as `Vec::new`'s, grows into own storage.

// bytePools[k] holds own buffers of capacity 1<<k that a grow left behind.
var bytePools [63]sync.Pool

// u8Type is the runtime's type descriptor of uint8, the type word of an interface's.
var u8Type = func() unsafe.Pointer {
	var x any = uint8(0)
	return (*[2]unsafe.Pointer)(unsafe.Pointer(&x))[0]
}()

// bytesOwn reports whether s has the shape of own storage.
func bytesOwn(s []uint8) bool {
	c := cap(s)
	return c >= 8 && c&(c-1) == 0 && len(s) < c
}

// bytesTake is own storage of capacity 1<<k: a recycled buffer, or a new one the
// runtime's growslice leaves unzeroed, since the translation writes what it reads.
func bytesTake(k int) []uint8 {
	if p, _ := bytePools[k].Get().(unsafe.Pointer); p != nil {
		return unsafe.Slice((*uint8)(p), 1<<k)[:0]
	}
	r := growslice(nil, 1<<k, 0, 1<<k, u8Type)
	return unsafe.Slice((*uint8)(r.ptr), 1<<k)[:0]
}

// bytesWithCapacity is `Vec::<u8>::with_capacity`, as vecWithCapacity is for others.
func bytesWithCapacity(n uint) []uint8 {
	if n > 1<<63-1 {
		panic(RustPanic{"capacity overflow"})
	}
	if n == 0 {
		return nil
	}
	if n >= 1<<62 {
		// More than any allocation, which fails where Rust's would abort.
		return make([]uint8, 0, n)
	}
	return bytesTake(max(bits.Len(n), 3))
}

// bytesPush is `Vec::<u8>::push`.
func bytesPush(v *[]uint8, x uint8) {
	if len(*v)+1 >= cap(*v) {
		bytesGrow(v, 1)
	}
	*v = append(*v, x)
}

// bytesExtend is `Vec::<u8>::extend_from_slice`.
func bytesExtend(v *[]uint8, xs []uint8) {
	if len(*v)+len(xs) >= cap(*v) {
		bytesGrow(v, len(xs))
	}
	*v = append(*v, xs...)
}

// bytesClone is a copy of xs in own storage: `Vec::<u8>::clone` and `<[u8]>::to_vec`.
func bytesClone(xs []uint8) []uint8 {
	var v []uint8
	bytesExtend(&v, xs)
	return v
}

// bytesGrow makes room for n more bytes and a spare one, as vecGrow does. Own storage
// grows into own storage and is recycled; Go's grows as `append` would, zeroed, and is
// trimmed off the shape of own storage.
func bytesGrow(v *[]uint8, n int) {
	s := *v
	want := max(2*cap(s), len(s)+n+1)
	if cap(s) == 0 || bytesOwn(s) {
		next := bytesTake(max(bits.Len(uint(want-1)), 3))[:len(s)]
		copy(next, s)
		if cap(s) != 0 {
			bytePools[bits.Len(uint(cap(s)))-1].Put(unsafe.Pointer(unsafe.SliceData(s)))
		}
		*v = next
		return
	}
	s = slices.Grow(s, want-len(s))
	if c := cap(s); c&(c-1) == 0 {
		s = s[: len(s) : c-1]
	}
	*v = s
}

// bytesEnter takes Go's storage into a root function off the shape of own storage, so it
// is never recycled: trimmed by a byte, or copied when full, since shortening it would
// give it the shape.
func bytesEnter(v *[]uint8) {
	s := *v
	c := cap(s)
	if c < 8 || c&(c-1) != 0 {
		return
	}
	if len(s) < c {
		*v = s[: len(s) : c-1]
		return
	}
	next := make([]uint8, len(s), len(s)+1)
	copy(next, s)
	*v = next
}

// bytesEscape hands own storage out of a root function to Go, clipped to its length,
// past which lie another buffer's bytes, and so never recycled.
func bytesEscape(v *[]uint8) {
	if bytesOwn(*v) {
		clip(v)
	}
}

// clip hides a slice's capacity past its length, which may hold another buffer's bytes.
func clip[T any](v *[]T) {
	*v = (*v)[:len(*v):len(*v)]
}

type rawSlice struct {
	ptr      unsafe.Pointer
	len, cap int
}

// growslice is the runtime's, which keeps its signature for the packages that link it:
// go.dev/issue/67401.
//
//go:linkname growslice runtime.growslice
func growslice(oldPtr unsafe.Pointer, newLen, oldCap, num int, et unsafe.Pointer) rawSlice

// vecTruncate is `Vec::truncate`, which zeroes the slots it vacates, as `slices.Delete`
// does, so Go code reading past the length sees nothing moved out of them.
func vecTruncate[T any](v *[]T, n uint) {
	if n < uint(len(*v)) {
		clear((*v)[n:])
		*v = (*v)[:n]
	}
}

// bytesTruncate is `Vec::<u8>::truncate`, which has nothing to zero.
func bytesTruncate(v *[]uint8, n uint) {
	if n < uint(len(*v)) {
		*v = (*v)[:n]
	}
}

func sliceIndexFail(start, end, len uint) RustPanic {
	switch {
	case start > len:
		return RustPanic{fmt.Sprintf("range start index %d out of range for slice of length %d", start, len)}
	case end > len:
		return RustPanic{fmt.Sprintf("range end index %d out of range for slice of length %d", end, len)}
	case start > end:
		return RustPanic{fmt.Sprintf("slice index starts at %d but ends at %d", start, end)}
	}
	return RustPanic{fmt.Sprintf("range end index %d out of range for slice of length %d", end, len)}
}

func sliceCopyFrom[T any](dst, src []T) {
	if len(dst) != len(src) {
		panic(RustPanic{fmt.Sprintf("copy_from_slice: source slice length (%d) does not match destination slice length (%d)", len(src), len(dst))})
	}
	copy(dst, src)
}

func sliceCopyWithin[T any](s []T, start, end, dest uint) {
	n := uint(len(s))
	if end > n {
		panic(sliceIndexFail(0, end, n))
	}
	if start > end {
		panic(sliceIndexFail(start, end, n))
	}
	if dest > n-(end-start) {
		panic(RustPanic{"dest is out of bounds"})
	}
	copy(s[dest:], s[start:end])
}

// utf8Check validates UTF-8 as `str::from_utf8` does: on failure, how many bytes from
// the start are valid, and the length of the invalid sequence, or none if the input
// ended inside one.
func utf8Check(b []uint8) (uint, Option[uint8], bool) {
	i := 0
	for i < len(b) {
		n, ok := utf8Seq(b[i:])
		if !ok {
			if n == 0 {
				return uint(i), Option[uint8]{}, false
			}
			return uint(i), Option[uint8]{tag: 1, Some: struct{ F0 uint8 }{uint8(n)}}, false
		}
		i += n
	}
	return 0, Option[uint8]{}, true
}

// utf8Seq is the length of the sequence at the front of b if valid, or else the length
// of the invalid one, 0 when b ends inside it, as core's validation counts them.
func utf8Seq(b []uint8) (int, bool) {
	first := b[0]
	if first < 0x80 {
		return 1, true
	}
	width := 0
	switch {
	case first >= 0xc2 && first <= 0xdf:
		width = 2
	case first >= 0xe0 && first <= 0xef:
		width = 3
	case first >= 0xf0 && first <= 0xf4:
		width = 4
	default:
		return 1, false
	}
	lo, hi := uint8(0x80), uint8(0xbf)
	switch first {
	case 0xe0:
		lo = 0xa0
	case 0xed:
		hi = 0x9f
	case 0xf0:
		lo = 0x90
	case 0xf4:
		hi = 0x8f
	}
	for k := 1; k < width; k++ {
		if k >= len(b) {
			return 0, false
		}
		if b[k] < lo || b[k] > hi {
			return k, false
		}
		lo, hi = 0x80, 0xbf
	}
	return width, true
}

func be16(x uint16) (b [2]uint8) { binary.BigEndian.PutUint16(b[:], x); return }
func be32(x uint32) (b [4]uint8) { binary.BigEndian.PutUint32(b[:], x); return }
func be64(x uint64) (b [8]uint8) { binary.BigEndian.PutUint64(b[:], x); return }
func le16(x uint16) (b [2]uint8) { binary.LittleEndian.PutUint16(b[:], x); return }
func le32(x uint32) (b [4]uint8) { binary.LittleEndian.PutUint32(b[:], x); return }
func le64(x uint64) (b [8]uint8) { binary.LittleEndian.PutUint64(b[:], x); return }

func fromBe16(b [2]uint8) uint16 { return binary.BigEndian.Uint16(b[:]) }
func fromBe32(b [4]uint8) uint32 { return binary.BigEndian.Uint32(b[:]) }
func fromBe64(b [8]uint8) uint64 { return binary.BigEndian.Uint64(b[:]) }
func fromLe16(b [2]uint8) uint16 { return binary.LittleEndian.Uint16(b[:]) }
func fromLe32(b [4]uint8) uint32 { return binary.LittleEndian.Uint32(b[:]) }
func fromLe64(b [8]uint8) uint64 { return binary.LittleEndian.Uint64(b[:]) }

// zero sets *p to its zero value.
func zero[T any](p *T) {
	var z T
	*p = z
}
