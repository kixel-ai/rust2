// Times the codec's workloads, rust2-ts's from ts/bench/workloads.ts, translated to Go
// and through rust2-ffi's C ABI by cgo: `just go bench`.
package bench

import (
	"bytes"
	"encoding/binary"
	"fmt"
	"strings"
	"syscall"
	"testing"

	"codec"
)

type workload struct {
	name  string
	items int
	input []uint8
}

func u64(v uint64) []uint8 { return binary.BigEndian.AppendUint64(nil, v) }

// workloads are ts/bench/workloads.ts's that code moq's wire format, and a group's whole
// stream: each encode, and a decode of what it wrote.
func workloads() []workload {
	var out []workload
	input := func(op, selector uint8, params ...uint64) []uint8 {
		input := []uint8{op, selector}
		for _, p := range params {
			input = append(input, u64(p)...)
		}
		return input
	}
	pair := func(name string, items int, input []uint8) []uint8 {
		wire := codec.BenchRun(input)
		name = strings.Replace(name, "%", fmt.Sprintf("%dB", len(wire)/items), 1)
		out = append(out, workload{name + "/encode", items, input})
		out = append(out, workload{name + "/decode", items, append([]uint8{input[0] + 1, input[1]}, wire...)})
		return wire
	}
	const run = 1024
	for _, v := range []uint64{1<<6 - 1, 1<<14 - 1, 1<<30 - 1, 1<<53 - 1} {
		pair("varint/quic/%", run, input(0, 0, run, v))
	}
	for _, v := range []uint64{1<<7 - 1, 1<<14 - 1, 1<<28 - 1, 1<<53 - 1} {
		pair("varint/leading-ones/%", run, input(0, 1, run, v))
	}
	// The payloads each stream's frames take from.
	payload := make([]uint8, 60_000)
	for i := range payload {
		payload[i] = uint8(i)
	}
	for _, v := range []struct {
		name     string
		selector uint8
	}{{"lite-06", 5}, {"lite-07", 6}} {
		video := []uint64{60, 33_333, 60_000, 8_000, 10, 17_000}
		audio := []uint64{50, 20_000, 160, 160, 0, 0}
		wire := pair("frames/video/"+v.name, 60, input(2, v.selector, video...))
		reads := append(input(7, v.selector, 3), wire...)
		out = append(out, workload{"frames/video/" + v.name + "/3B-reads/decode", 60, reads})
		pair("frames/audio/"+v.name, 50, input(2, v.selector, audio...))
		pair("group/"+v.name, 1, input(4, v.selector, 3, 1_234))
		// A group's stream end to end, header, frames, and payloads, as moq's consumer runs it.
		pair("stream/video/"+v.name, 1, append(input(8, v.selector, append([]uint64{3, 1_234}, video...)...), payload...))
		pair("stream/audio/"+v.name, 1, append(input(8, v.selector, append([]uint64{3, 1_234}, audio...)...), payload...))
	}
	return out
}

// TestAgree checks the two routes give the same output on every workload, so the
// benchmark times the same work.
func TestAgree(t *testing.T) {
	if !contract() {
		t.Fatal("the library and codec.h declare different APIs")
	}
	for _, w := range workloads() {
		if got, want := codec.BenchRun(w.input), native(w.input); !bytes.Equal(got, want) {
			t.Errorf("%s: translated %x, cgo %x", w.name, got, want)
		}
	}
}

func BenchmarkCodec(b *testing.B) {
	for _, w := range workloads() {
		for _, route := range []struct {
			name string
			run  func([]uint8) []uint8
		}{{"go", codec.BenchRun}, {"cgo", native}} {
			b.Run(w.name+"/"+route.name, func(b *testing.B) {
				start := usage()
				for b.Loop() {
					route.run(w.input)
				}
				used := usage()
				items := float64(b.N * w.items)
				b.ReportMetric(float64(b.Elapsed().Nanoseconds())/items, "ns/item")
				b.ReportMetric(float64(used.cpu-start.cpu)/items, "cpu-ns/item")
				b.ReportMetric(float64(used.faults-start.faults)/float64(b.N), "faults/op")
			})
		}
	}
}

// rusage is what the process has used: CPU time in nanoseconds, every thread's, the
// collector's workers included, so it counts what a run costs where wall time would count
// how many of the host's cores were free; and minor page faults, every fault the kernel
// served without I/O, copy-on-write included, so an overall diagnostic rather than a count
// of fresh pages.
type rusage struct{ cpu, faults int64 }

func usage() rusage {
	var u syscall.Rusage
	if err := syscall.Getrusage(syscall.RUSAGE_SELF, &u); err != nil {
		panic(err)
	}
	return rusage{u.Utime.Nano() + u.Stime.Nano(), u.Minflt}
}
