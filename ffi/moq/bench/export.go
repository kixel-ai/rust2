// bench/export.c's frame writes and reads through the generated Go, so the two differ by
// what cgo and the Go wrapper add. Each round runs twice, the first to warm up.
package main

import (
	"context"
	"fmt"
	"time"

	"moq/net"
)

const (
	groups = 10000
	frames = 100
)

// perFrame is the nanoseconds per frame since start.
func perFrame(start time.Time) float64 {
	return float64(time.Since(start).Nanoseconds()) / (groups * frames)
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

func round() string {
	broadcast := must(net.NewBroadcastInfo().Produce())
	micro := net.TimescaleMICRO()
	track := must(broadcast.CreateTrack("bench", must(net.TrackInfoDefault().WithTimescale(micro))))
	micro.Close()

	// A frame of 100 zero bytes, as bench/export.c writes.
	frame := make([]byte, 100)
	var micros uint64

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	start := time.Now()
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
	stamped := perFrame(start)

	// The same, reusing one timestamp handle: what the handle costs.
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
	reused := perFrame(start)

	// The floor: a hold of a handle and a bool back.
	closed := 0
	start = time.Now()
	for range groups * frames {
		if must(track.IsClosed()) {
			closed++
		}
	}
	call := perFrame(start)

	// Reads: each frame written, then read through a call whose future is ready at its
	// first poll, so it never waits.
	ctx := context.Background()
	consumer := must(track.Consume())
	subscriber := must(consumer.Subscribe(ctx, nil))
	consumer.Close()
	var last *net.FrameFrame
	start = time.Now()
	for g := range groups {
		group := must(track.AppendGroup())
		reader := must(subscriber.RecvGroup(ctx))
		for f := range frames {
			check(group.WriteFrame(at, frame))
			read := must(reader.ReadFrame(ctx))
			if g == groups-1 && f == frames-1 {
				last = read
			} else {
				read.Close()
			}
		}
		reader.Close()
		check(group.Finish())
		group.Close()
	}
	read := perFrame(start)

	// And one 512 B frame, the least lent, and one 4 KiB.
	group := must(track.AppendGroup())
	reader := must(subscriber.RecvGroup(ctx))
	check(group.WriteFrame(at, make([]byte, 512)))
	edge := must(reader.ReadFrame(ctx))
	check(group.WriteFrame(at, make([]byte, 4096)))
	big := must(reader.ReadFrame(ctx))
	reader.Close()
	group.Close()
	subscriber.Close()

	// Payload reads: each frame's bytes, as a Bytes, then copied out of it. 100 B are copied
	// at the call; from 512 B, they are lent without a copy until the GC frees them.
	lent := func(frame *net.FrameFrame) float64 {
		start := time.Now()
		for range groups * frames {
			closed += int(must(frame.PayloadGet()).At(0))
		}
		return perFrame(start)
	}
	copied := func(frame *net.FrameFrame) float64 {
		start := time.Now()
		for range groups * frames {
			closed += int(must(frame.PayloadGet()).Copy()[0])
		}
		return perFrame(start)
	}
	small, smallCopied := lent(last), copied(last)
	least, leastCopied := lent(edge), copied(edge)
	large, largeCopied := lent(big), copied(big)
	last.Close()
	edge.Close()
	big.Close()
	at.Close()
	track.Close()
	broadcast.Close()

	bang := ""
	if closed != 0 {
		bang = "!"
	}
	return fmt.Sprintf("export (Go): %.0f ns per stamped 100 B frame, %.0f reusing the timestamp, %.1f ns per handle call, %.0f ns per frame written and read, %.0f ns per 100 B payload read (%.0f copied), %.0f per 512 B (%.0f copied), %.0f per 4 KiB (%.0f copied)%s",
		stamped, reused, call, read, small, smallCopied, least, leastCopied, large, largeCopied, bang)
}

func main() {
	round()
	fmt.Println(round())
}
