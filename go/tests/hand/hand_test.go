// Holds onto byte storage the translated `fixtures/hand` hands Go code, as Go code may:
// what it is handed has no capacity past its length, and nothing it keeps is recycled.
package hand_test

import (
	"bytes"
	"testing"
	"unsafe"

	fixture "fixture"
)

// churn grows and outgrows buffers of every size, which would overwrite any buffer Go
// code keeps that the translation recycled.
func churn() {
	for i := 0; i < 64; i++ {
		var v []uint8
		fixture.Append(&v, uint8(255-i))
		fixture.Run(bytes.Repeat([]uint8{uint8(i)}, 40))
	}
}

func clipped(t *testing.T, what string, s []uint8) {
	t.Helper()
	if len(s) != cap(s) {
		t.Errorf("%s: length %d, capacity %d", what, len(s), cap(s))
	}
}

// clean checks that s shows nothing past its length but zeros, as Go storage the
// translation grew does, which `append` would have grown the same.
func clean(t *testing.T, what string, s []uint8) {
	t.Helper()
	if !bytes.Equal(s[len(s):cap(s)], make([]uint8, cap(s)-len(s))) {
		t.Errorf("%s: bytes past its length", what)
	}
}

// kept checks that keep still holds what it did, all the way to its capacity, after
// the translation outgrew it.
func kept(t *testing.T, what string, keep []uint8) {
	t.Helper()
	want := bytes.Clone(keep[:cap(keep)])
	churn()
	if got := keep[:cap(keep)]; !bytes.Equal(got, want) {
		t.Errorf("%s changed after the translation outgrew it", what)
	}
}

func TestResultsClipped(t *testing.T) {
	for _, n := range []uint8{1, 7, 8, 100, 255} {
		clipped(t, "Fill", fixture.Fill(n))
		clipped(t, "Grow", fixture.Grow(nil, n))
		clipped(t, "Maybe", fixture.Maybe(n).Some.F0)
		for i, v := range fixture.Nested(n) {
			clipped(t, "Nested", v)
			if len(v) != i {
				t.Errorf("Nested: buffer %d holds %d bytes", i, len(v))
			}
		}
		var v []uint8
		clipped(t, "View", fixture.View(&v, n))
		clipped(t, "View's argument", v)
		b := fixture.BufNew()
		fixture.BufFill(&b, n)
		clipped(t, "BufBytes", fixture.BufBytes(&b))
	}
}

// Go's storage of the shape of the translation's own is trimmed, not recycled.
func TestGoStorageKept(t *testing.T) {
	s := make([]uint8, 5, 64)
	for i := range s[:cap(s)] {
		s[:cap(s)][i] = 0xaa
	}
	keep := s
	fixture.Append(&s, 200)
	if len(s) != 205 {
		t.Fatalf("Append: %d bytes", len(s))
	}
	kept(t, "Go's slice passed by pointer", keep)
	g := make([]uint8, 3, 128)
	keepG := g
	r := fixture.Grow(g, 200)
	if len(r) != 203 {
		t.Fatalf("Grow: %d bytes", len(r))
	}
	kept(t, "Go's slice passed by value", keepG)
}

// What the translation hands out, handed back and outgrown, is not recycled.
func TestHandedKept(t *testing.T) {
	r := fixture.Fill(100)
	keep := r
	fixture.Append(&r, 200)
	kept(t, "a result passed back", keep)

	b := fixture.Boxed(&fixture.Open{}, 100)
	clipped(t, "a boxed field", b.Data)
	keep = b.Data
	fixture.Reboxed(&b, 200)
	clean(t, "a boxed field grown again", b.Data)
	kept(t, "a boxed field", keep)

	var e fixture.Either
	fixture.Flip(&e, 50)
	clipped(t, "Right", e.Right.F1)
	keep = e.Right.F1
	fixture.Flip(&e, 200)
	clean(t, "Left", e.Left.F0)
	if e.Right.F1 != nil {
		t.Errorf("Right still holds %d bytes after switching to Left", len(e.Right.F1))
	}
	kept(t, "a variant switched away from", keep)
}

// Go's full storage of the capacity of the translation's own, shortened and grown, is
// not recycled, whether Go made it or was handed it.
func TestFullKept(t *testing.T) {
	s := bytes.Repeat([]uint8{0xaa}, 64)
	keep := s
	fixture.Refill(&s, 200)
	kept(t, "Go's full slice", keep)
	r := fixture.Fill(8)
	keep = r
	fixture.Refill(&r, 200)
	kept(t, "a full result passed back", keep)
}

// What Go passes, the translation works on a copy of wherever it could leave storage of
// its own behind: Go's Box and an array it outgrows keep what Go put there.
func TestContainersCopied(t *testing.T) {
	g := make([]uint8, 3, 16)
	o := &fixture.Open{Data: g}
	b := fixture.Boxed(o, 100)
	if b == o || unsafe.SliceData(o.Data) != unsafe.SliceData(g) || len(o.Data) != 3 {
		t.Errorf("Boxed wrote into the Box Go passed")
	}
	old := b
	fixture.Reboxed(&b, 100)
	if b == old || len(old.Data) != 103 {
		t.Errorf("Reboxed wrote into the Box Go passed behind a pointer")
	}

	v := make([][]uint8, 1)
	alias := v
	if n := fixture.Regrow(&v, 100); n != 300 {
		t.Fatalf("Regrow: %d bytes", n)
	}
	if alias[0] != nil {
		t.Errorf("an array the translation outgrew holds %d bytes Go did not put there", len(alias[0]))
	}
	clipped(t, "Regrow's argument", v[0])
}

// A slot the translation vacates past a length shows nothing it held.
func TestVacatedZeroed(t *testing.T) {
	v := make([][]uint8, 0, 4)
	if n := fixture.Vacate(&v, 100); n != 300 {
		t.Fatalf("Vacate: %d bytes", n)
	}
	if s := v[:cap(v)][0]; s != nil {
		t.Errorf("a vacated slot holds %d bytes", len(s))
	}
}

// A panic unwinding out of the translation still hands back what Go keeps.
func TestPanicHandsBack(t *testing.T) {
	var v []uint8
	func() {
		defer func() { recover() }()
		fixture.AppendOrPanic(&v, 101)
	}()
	if len(v) != 101 {
		t.Fatalf("AppendOrPanic: %d bytes", len(v))
	}
	clipped(t, "AppendOrPanic's argument", v)
}

// Go's storage the translation grows keeps growing as `append` would: each call to
// append one byte copies the buffer only as often as doubling it does.
func TestGoStorageAmortized(t *testing.T) {
	var v []uint8
	moves := 0
	var last *uint8
	for range 10_000 {
		fixture.Append(&v, 1)
		if p := unsafe.SliceData(v); p != last {
			moves++
			last = p
		}
	}
	if moves > 20 {
		t.Errorf("10000 one-byte appends moved the buffer %d times", moves)
	}
}
