// Reads the items fixture's exported constants and statics as a Go caller does, and
// prints what it reads for fixtures.rs to compare with native Rust's.
package main

import (
	"fmt"

	fixture "fixture"
)

// Go constants, which no Go caller can reassign.
const (
	maxLen   = fixture.MaxLen
	countMax = fixture.CountMax
	floor    = fixture.Floor
	enabled  = fixture.Enabled
	letter   = fixture.Letter
	greeting = fixture.Greeting
	sides    = fixture.ShapesSides
	text     = fixture.Text
)

func main() {
	fmt.Println(maxLen, countMax, floor, enabled, string(letter), sides)
	fmt.Printf("%x %s\n", greeting, text)
	// Each call is a fresh value, which changing leaves the next call's alone.
	origin := fixture.Origin()
	origin.Sum = 99
	start := fixture.CountStart()
	fixture.CountBump(&start)
	fmt.Println(origin.Sum, fixture.Origin().Sum, start.Count, fixture.CountStart().Count)
	// A static is one variable: Go callers read what translated code reads, at one
	// address, and translated code reads what they write to it.
	fmt.Println(fixture.Hits.Count, fixture.Table, string(fixture.Name), string(fixture.Bytes[:]), fixture.ShapesUnit)
	before := fixture.Run([]uint8{2})
	hits := &fixture.Hits
	hits.Count = 9
	after := fixture.Run([]uint8{2})
	fmt.Println(before[4], after[4], fixture.Hits.Count)
	fixture.Letters[1] = 'q'
	fmt.Println(string(fixture.Tail))
}
