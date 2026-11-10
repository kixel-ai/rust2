// bench/export.c's cases through the generated Go, so the two differ by what cgo and the
// Go wrapper add. Each round runs twice, the first to warm up.
package main

import (
	"context"
	"fmt"
	"runtime"
	"time"

	"moq/net"
)

const (
	groups = 10000
	frames = 100
	n      = groups * frames
)

var sizes = []int{100, 1024, 16384, 65536, 1048576}

// per is the nanoseconds per call since start, over n calls.
func per(start time.Time) float64 {
	return float64(time.Since(start).Nanoseconds()) / n
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
// payload's first byte, in about 64 MiB and at least 300 frames: megabytes a second.
func throughput(ctx context.Context, track *net.TrackProducer, subscriber *net.TrackSubscriber, at *net.Timestamp, size int) float64 {
	count := min(max((64<<20)/(size*frames), 3), 2000)
	frame := make([]byte, size)
	start := time.Now()
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
	return float64(count*frames*size) * 1e3 / float64(time.Since(start).Nanoseconds())
}

type result struct {
	name  string
	value float64
}

func round() []result {
	var out []result
	broadcast := must(net.NewBroadcastInfo().Produce())
	micro := net.TimescaleMICRO()
	track := must(broadcast.CreateTrack("bench", must(net.TrackInfoDefault().WithTimescale(micro))))
	micro.Close()
	frame := make([]byte, 100)

	start := time.Now()
	for range n {
		if must(track.IsClosed()) {
			sink++
		}
	}
	out = append(out, result{"call", per(start)})

	demanded := must(track.AppendGroup())
	demand := must(demanded.Demand())
	start = time.Now()
	for range n {
		sink += int(must(demand.Sequence()))
	}
	out = append(out, result{"getter", per(start)})
	demand.Close()
	check(demanded.Finish())
	demanded.Close()

	start = time.Now()
	for range n {
		if must(track.Largest()) != nil {
			sink++
		}
	}
	out = append(out, result{"record", per(start)})

	start = time.Now()
	for range n {
		must(track.Info()).Close()
	}
	out = append(out, result{"object", per(start)})

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var micros uint64
	start = time.Now()
	for range groups {
		group := must(track.AppendGroup())
		for range frames {
			at := must(net.TimestampFromMicros(micros))
			micros++
			check(group.WriteFrame(at, frame))
			at.Close()
		}
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"stamped", per(start)})

	at := must(net.TimestampFromMicros(micros))
	start = time.Now()
	for range groups {
		group := must(track.AppendGroup())
		for range frames {
			check(group.WriteFrame(at, frame))
		}
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"write", per(start)})

	ctx := context.Background()
	consumer := must(track.Consume())
	subscriber := must(consumer.Subscribe(ctx, nil))
	consumer.Close()

	// Each frame written, then read through a call whose future is ready at its first poll,
	// so it never waits.
	var last *net.FrameFrame
	start = time.Now()
	for range groups {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			check(group.WriteFrame(at, frame))
			if last != nil {
				last.Close()
			}
			last = must(reader.ReadFrame(ctx))
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"ready", per(start)})

	// Each read started on a goroutine before its frame is written, which the yield lets
	// block, and the write's wake resumes it.
	start = time.Now()
	read := make(chan *net.FrameFrame)
	for range groups {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for range frames {
			go func() { read <- must(reader.ReadFrame(ctx)) }()
			runtime.Gosched()
			check(group.WriteFrame(at, frame))
			(<-read).Close()
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	out = append(out, result{"woken", per(start)})

	for _, size := range sizes {
		out = append(out, result{fmt.Sprintf("mbps:%d", size), throughput(ctx, track, subscriber, at, size)})
	}
	subscriber.Close()

	// The last frame's 100 bytes, as a Bytes, and one of them read.
	start = time.Now()
	for range n {
		sink += int(must(last.PayloadGet()).At(0))
	}
	out = append(out, result{"payload", per(start)})
	last.Close()
	at.Close()
	track.Close()
	broadcast.Close()
	return out
}

func main() {
	round()
	for _, r := range round() {
		fmt.Printf("%s\t%f\n", r.name, r.value)
	}
}
