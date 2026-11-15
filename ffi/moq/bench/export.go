// bench/export.c's cases through the generated Go, so the two differ by what cgo and the
// Go wrapper add. Each round runs twice, the first to warm up. Each case prints its wall
// time and the process's CPU time, the GC's threads included, or under CodSpeed, runs once
// for its CPU simulation to count.
package main

// #include <stdlib.h>
// #include "codspeed.h"
import "C"

import (
	"context"
	"fmt"
	"runtime"
	"sync"
	"syscall"
	"time"
	"unsafe"

	"moq/net"
	"moq/rt"
)

const (
	frames  = 100
	threads = 4
)

// groups is a tenth under CodSpeed, whose simulation counts every call exactly, and n the
// calls a case makes.
var (
	groups = 10000
	n      = groups * frames
)

var sizes = []int{100, 1024, 16384, 65536, 1048576}

// span is wall and CPU nanoseconds: a point, or a span per call.
type span struct {
	wall, cpu float64
}

// epoch is where now's wall clock counts from, monotonic and small enough to keep float64's
// precision.
var epoch = time.Now()

// now is the wall clock and the process's CPU time, user and system.
func now() span {
	var usage syscall.Rusage
	check(syscall.Getrusage(syscall.RUSAGE_SELF, &usage))
	cpu := usage.Utime.Nano() + usage.Stime.Nano()
	return span{float64(time.Since(epoch).Nanoseconds()), float64(cpu)}
}

// per is the nanoseconds per call since start, over calls calls.
func per(start span, calls float64) span {
	end := now()
	return span{(end.wall - start.wall) / calls, (end.cpu - start.cpu) / calls}
}

// begin is a case's start: now, and CodSpeed counting from here.
func begin() span {
	start := now()
	C.codspeed_start()
	return start
}

// end is the case name's nanoseconds per call since start, over calls calls.
func end(start span, name string, calls int) span {
	c := C.CString(name)
	C.codspeed_stop(c)
	C.free(unsafe.Pointer(c))
	return per(start, float64(calls))
}

// must is v, failing the bench on err.
func must[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

// check fails the bench on err.
func check(err error) {
	if err != nil {
		panic(err)
	}
}

// sink holds what the loops sum, so the compiler cannot drop the calls.
var sink int

// throughput writes groups of 100 frames of size bytes, reading each back and touching its
// payload's first byte, in about 64 MiB and at least 300 frames: megabytes a wall and a
// CPU second.
func throughput(ctx context.Context, track *net.TrackProducer, subscriber *net.TrackSubscriber, at *net.Timestamp, size int) span {
	count := min(max((64<<20)/(size*frames), 3), 2000)
	frame := rt.NewBytes(make([]byte, size))
	name := fmt.Sprintf("throughput[%d]", size)
	start := begin()
	for range count {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			check(group.WriteFrame(at, frame))
			read := must(reader.ReadFrame(ctx))
			sink += int(must(read.PayloadGet()).At(0))
			read.Close()
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	ns := end(start, name, count*frames*size)
	return span{1e3 / ns.wall, 1e3 / ns.cpu}
}

// pendingContext reports only after the binding polls pending with a real waker,
// like the watched context in the future correctness tests. The writer cannot
// race the initial poll. It is never cancelled, so Done still returns nil.
type pendingContext struct {
	context.Context
	parked chan struct{}
	waits  int
}

func (c *pendingContext) Done() <-chan struct{} {
	c.waits++
	c.parked <- struct{}{}
	return nil
}

// concurrent makes each of calls n times on a goroutine of its own, all at once: wall
// nanoseconds per call on one goroutine, and CPU nanoseconds per call.
func concurrent(calls []func() int) span {
	var ready, done sync.WaitGroup
	gate := make(chan struct{})
	sums := make([]int, len(calls))
	for i, call := range calls {
		ready.Add(1)
		done.Add(1)
		go func() {
			defer done.Done()
			sum := 0
			ready.Done()
			<-gate
			for range n {
				sum += call()
			}
			sums[i] = sum
		}()
	}
	ready.Wait()
	start := now()
	close(gate)
	done.Wait()
	ns := per(start, float64(n))
	for _, sum := range sums {
		sink += sum
	}
	return span{ns.wall, ns.cpu / float64(len(calls))}
}

type result struct {
	name  string
	value span
}

func round(measured bool) []result {
	var out []result
	broadcast := must(net.NewBroadcastInfo().Produce())
	micro := net.TimescaleMICRO()
	track := must(broadcast.CreateTrack("bench", must(net.TrackInfoDefault().WithTimescale(&micro))))
	frame := rt.NewBytes(make([]byte, 100))

	start := begin()
	for range n {
		if must(track.IsClosed()) {
			sink++
		}
	}
	out = append(out, result{"call", end(start, "call", n)})

	demanded := must(track.AppendGroup())
	demand := must(demanded.Demand())
	start = begin()
	for range n {
		sink += int(must(demand.Sequence()))
	}
	out = append(out, result{"getter", end(start, "getter", n)})

	// The handle call and the getter from threads goroutines: on one shared handle, then on
	// a clone each, a handle of its own to the same value. CodSpeed's simulation runs one
	// thread at a time, which contends for nothing, so it skips them.
	if !measured {
		isClosed := func(t *net.TrackProducer) func() int {
			return func() int {
				if must(t.IsClosed()) {
					return 1
				}
				return 0
			}
		}
		sequence := func(d *net.GroupDemand) func() int {
			return func() int { return int(must(d.Sequence())) }
		}
		var sharedCalls, ownCalls, sharedGetters, ownGetters []func() int
		var clones []interface{ Close() }
		for range threads {
			t := must(track.Clone())
			d := must(demand.Clone())
			clones = append(clones, t, d)
			sharedCalls = append(sharedCalls, isClosed(track))
			ownCalls = append(ownCalls, isClosed(t))
			sharedGetters = append(sharedGetters, sequence(demand))
			ownGetters = append(ownGetters, sequence(d))
		}
		out = append(out, result{"shared:call", concurrent(sharedCalls)})
		out = append(out, result{"own:call", concurrent(ownCalls)})
		out = append(out, result{"shared:getter", concurrent(sharedGetters)})
		out = append(out, result{"own:getter", concurrent(ownGetters)})
		for _, clone := range clones {
			clone.Close()
		}
	}
	demand.Close()
	check(demanded.Finish())
	demanded.Close()

	start = begin()
	for range n {
		if must(track.Largest()) != nil {
			sink++
		}
	}
	out = append(out, result{"record", end(start, "record", n)})

	start = begin()
	for range n {
		must(track.Info()).Close()
	}
	out = append(out, result{"object", end(start, "object", n)})

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must.
	var micros uint64
	start = begin()
	for range groups {
		group := must(track.AppendGroup())
		for range frames {
			at := must(net.TimestampFromMicros(micros))
			micros++
			check(group.WriteFrame(&at, frame))
		}
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"stamped", end(start, "stamped", n)})

	at := must(net.TimestampFromMicros(micros))
	start = begin()
	for range groups {
		group := must(track.AppendGroup())
		for range frames {
			check(group.WriteFrame(&at, frame))
		}
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"write", end(start, "write", n)})

	ctx := context.Background()
	consumer := must(track.Consume())
	subscriber := must(consumer.Subscribe(ctx, nil))
	consumer.Close()

	// Each frame written, then read through a call whose future is ready at its first poll,
	// so it never waits.
	var last *net.FrameFrame
	start = begin()
	for range groups {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			check(group.WriteFrame(&at, frame))
			if last != nil {
				last.Close()
			}
			last = must(reader.ReadFrame(ctx))
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"ready", end(start, "ready", n)})

	// Each read started on a goroutine before its frame is written, which the yield lets
	// block, and the write's wake resumes it.
	start = begin()
	read := make(chan *net.FrameFrame)
	for range groups {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			go func() { read <- must(reader.ReadFrame(ctx)) }()
			runtime.Gosched()
			check(group.WriteFrame(&at, frame))
			(<-read).Close()
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"woken", end(start, "woken", n)})

	// One writer and the calling reader live for the whole case. Each write
	// waits until the read has pended with its waker registered.
	start = begin()
	permits := make(chan struct{})
	pending := &pendingContext{Context: ctx, parked: permits}
	done := make(chan struct{})
	go func() {
		defer close(done)
		for range groups {
			group := must(track.AppendGroup())
			for range frames {
				<-permits
				check(group.WriteFrame(&at, frame))
			}
			<-permits // Keep this group until its reader has consumed every frame.
			check(group.Finish())
			group.Close()
		}
	}()
	for range groups {
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			must(reader.ReadFrame(pending)).Close()
		}
		reader.Close()
		permits <- struct{}{}
	}
	<-done
	if pending.waits != n {
		panic("each loop read must pend exactly once")
	}
	out = append(out, result{"loop", end(start, "loop", n)})

	for _, size := range sizes {
		out = append(out, result{fmt.Sprintf("mbps:%d", size), throughput(ctx, track, subscriber, &at, size)})
	}
	subscriber.Close()

	// The last frame's 100 bytes, as a Bytes, and one of them read.
	start = begin()
	for range n {
		sink += int(must(last.PayloadGet()).At(0))
	}
	out = append(out, result{"payload", end(start, "payload", n)})
	last.Close()
	track.Close()
	broadcast.Close()
	return out
}

func main() {
	bench := C.CString("ffi/moq/bench/export.go::go")
	measured := bool(C.codspeed_init(bench))
	C.free(unsafe.Pointer(bench))
	// Under CodSpeed, one round, which prints nothing: its simulation counts each case.
	if measured {
		groups /= 10
		n = groups * frames
		round(true)
		return
	}
	round(false)
	for _, r := range round(false) {
		fmt.Printf("%s\t%f\t%f\n", r.name, r.value.wall, r.value.cpu)
	}
}
