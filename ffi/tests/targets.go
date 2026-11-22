// What api.go leaves out because it depends on the export's targets. The release builder's
// export holds every host and no wasm32; the tests' holds the host and wasm32. So a name only
// wasm32 has exists only in the tests' export, where on the host's library it is unsupported,
// and a call to an item some target lacks returns an error as well in whichever export
// misses it on a target: the web lacks threads, and Windows a path that is not UTF-8.
//
// With an argument, it instead runs a call the host's library lacks, which must fail.
package main

import (
	"errors"
	"fmt"
	"os"
	"runtime"
	"sync/atomic"
	"time"

	"api/api"
)

func check(ok bool, what string) {
	if !ok {
		fmt.Fprintf(os.Stderr, "failed: %s\n", what)
		os.Exit(1)
	}
}

func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

// released waits for the GC to collect what released marks, once Rust released the closure
// holding it.
func released(released *atomic.Bool, what string) {
	deadline := time.Now().Add(10 * time.Second)
	for !released.Load() {
		check(time.Now().Before(deadline), what)
		runtime.GC()
		runtime.Gosched()
	}
}

func main() {
	if len(os.Args) > 1 {
		broken(os.Args[1])
		return
	}
	// The traits of a type only wasm32 has fail as its functions do.
	nines := api.MarkOf(9)
	_, err := api.PageDefault()
	check(errors.Is(err, errors.ErrUnsupported), "a Default off the target is unsupported")
	_, err = api.Page{Size: 1}.Equal(api.Page{Size: 1})
	check(errors.Is(err, errors.ErrUnsupported), "an Equal off the target is unsupported")
	booth := api.Booth{Mark: nines}
	_, err = booth.Equal(booth)
	check(errors.Is(err, errors.ErrUnsupported), "a checked Equal off the target is unsupported")
	check(
		booth.GoString() == "api_Booth_debug: it is not on this library's target",
		"a GoString off the target reads as its error",
	)
	check(nines.Close() == nil, "a mark closes")

	// A path that is no UTF-8 is a Go string all the same: Unix and WASI have it, Windows
	// does not, so the builder's export serves it with an error.
	check(api.Stray() == "\xff", "a path that is not UTF-8 crosses as its bytes")
	var lent string
	api.VisitStray(func(path string) { lent = path })
	check(lent == "\xff", "a path Rust lends that is not UTF-8 crosses as its bytes")
	// A kept closure, called on another thread after the call that took it returned, which
	// the web lacks: so the tests' export returns an error with it, and the builder's none.
	spawned := must(api.NewSpawned(func() uint32 { return 7 }))
	check(must(spawned.Join()) == 7, "a kept closure runs on another thread")
	// One Rust releases once it drops it, after its thread called it.
	marker := new([4]int)
	var freed atomic.Bool
	runtime.AddCleanup(marker, func(freed *atomic.Bool) { freed.Store(true) }, &freed)
	spawned = must(api.NewSpawned(func() uint32 { return uint32(marker[0]) + 1 }))
	check(must(spawned.Join()) == 1, "a kept closure runs on Rust's thread")
	released(&freed, "Rust releases a kept closure it dropped")
	fmt.Println("targets ok")
}

// broken runs a call that panics with a message naming its item.
func broken(which string) {
	switch which {
	case "target":
		_, err := api.WasmOnly()
		check(errors.Is(err, errors.ErrUnsupported), "an item off the target is unsupported")
		panic(err)
	case "variant":
		api.PointerBits(api.PlatformWasm{Bits: 32})
	case "plain variant":
		api.IsFamily(api.FamilyWasm)
	default:
		check(false, "no case "+which)
	}
}
