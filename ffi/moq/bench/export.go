// bench/export.c's cases through the generated Go, so the two differ by what cgo and the
// Go wrapper add. Each round runs twice, the first to warm up. Each case prints its wall
// time and the process's CPU time, the GC's threads included.
package main

import (
	"context"
	"fmt"
	"runtime"
	"sync"
	"syscall"
	"time"

	"moq/net"
	"moq/rt"
)

const (
	groups  = 10000
	frames  = 100
	n       = groups * frames
	threads = 4
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
	start := now()
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
	ns := per(start, float64(count*frames*size))
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
	ns := per(start, n)
	for _, sum := range sums {
		sink += sum
	}
	return span{ns.wall, ns.cpu / float64(len(calls))}
}

type result struct {
	name  string
	value span
}

func round() []result {
	var out []result
	broadcast := must(net.NewBroadcastInfo().Produce())
	micro := net.TimescaleMICRO()
	track := must(broadcast.CreateTrack("bench", must(net.TrackInfoDefault().WithTimescale(&micro))))
	frame := rt.NewBytes(make([]byte, 100))

	start := now()
	for range n {
		if must(track.IsClosed()) {
			sink++
		}
	}
	out = append(out, result{"call", per(start, n)})

	demanded := must(track.AppendGroup())
	demand := must(demanded.Demand())
	start = now()
	for range n {
		sink += int(must(demand.Sequence()))
	}
	out = append(out, result{"getter", per(start, n)})

	// The handle call and the getter from threads goroutines: on one shared handle, then on
	// a clone each, a handle of its own to the same value.
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
	demand.Close()
	check(demanded.Finish())
	demanded.Close()

	start = now()
	for range n {
		if must(track.Largest()) != nil {
			sink++
		}
	}
	out = append(out, result{"record", per(start, n)})

	start = now()
	for range n {
		must(track.Info()).Close()
	}
	out = append(out, result{"object", per(start, n)})

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must.
	var micros uint64
	start = now()
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
	out = append(out, result{"stamped", per(start, n)})

	at := must(net.TimestampFromMicros(micros))
	start = now()
	for range groups {
		group := must(track.AppendGroup())
		for range frames {
			check(group.WriteFrame(&at, frame))
		}
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"write", per(start, n)})

	ctx := context.Background()
	consumer := must(track.Consume())
	subscriber := must(consumer.Subscribe(ctx, nil))
	consumer.Close()

	// Each frame written, then read through a call whose future is ready at its first poll,
	// so it never waits.
	var last *net.FrameFrame
	start = now()
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
	out = append(out, result{"ready", per(start, n)})

	// Each read started on a goroutine before its frame is written, which the yield lets
	// block, and the write's wake resumes it.
	start = now()
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
	out = append(out, result{"woken", per(start, n)})

	// One writer and the calling reader live for the whole case. Each write
	// waits until the read has pended with its waker registered.
	start = now()
	permits := make(chan struct{})
	pending := &pendingContext{Context: ctx, parked: permits}
	done := make(chan struct{})
	go func() {
		defer close(done)
		for range groups {
			group := must(track.AppendGroup())
			for range frames {
				<-permits
				check(group.WriteFrame(at, frame))
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
	out = append(out, result{"loop", per(start, n)})

	for _, size := range sizes {
		out = append(out, result{fmt.Sprintf("mbps:%d", size), throughput(ctx, track, subscriber, &at, size)})
	}
	subscriber.Close()

	// The last frame's 100 bytes, as a Bytes, and one of them read.
	start = now()
	for range n {
		sink += int(must(last.PayloadGet()).At(0))
	}
	out = append(out, result{"payload", per(start, n)})
	last.Close()
	track.Close()
	broadcast.Close()
	return out
}

func main() {
	round()
	for _, r := range round() {
		fmt.Printf("%s\t%f\t%f\n", r.name, r.value.wall, r.value.cpu)
	}
}
