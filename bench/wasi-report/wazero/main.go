// What a call costs a wazero host: wb's shapes through the exporter's C ABI in a core module.
package main

import (
	"context"
	"encoding/binary"
	"fmt"
	"os"
	"time"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

const n = 1_000_000

var sink uint64

func measure(name string, iters int, f func()) {
	var ns float64
	for r := 0; r < 2; r++ {
		start := time.Now()
		for i := 0; i < iters; i++ {
			f()
		}
		ns = float64(time.Since(start).Nanoseconds()) / float64(iters)
	}
	fmt.Printf("%s\t%.1f\n", name, ns)
}

func main() {
	ctx := context.Background()
	wasm, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic(err)
	}
	r := wazero.NewRuntime(ctx)
	wasi_snapshot_preview1.MustInstantiate(ctx, r)
	var start api.Function
	_, err = r.NewHostModuleBuilder("rust2").
		NewFunctionBuilder().WithGoModuleFunction(api.GoModuleFunc(func(ctx context.Context, _ api.Module, stack []uint64) {
		if stack[0] == 0 {
			stack[0] = 1
			return
		}
		if err := start.CallWithStack(ctx, stack); err != nil {
			panic(err)
		}
	}), []api.ValueType{api.ValueTypeI32}, []api.ValueType{api.ValueTypeI64}).Export("wb_web_i_fn_0").
		NewFunctionBuilder().WithFunc(func(uint32) {}).Export("rust2_wake").
		NewFunctionBuilder().WithFunc(func(uint32) {}).Export("rust2_waker_drop").
		NewFunctionBuilder().WithFunc(func(uint32) {}).Export("rust2_free").
		NewFunctionBuilder().WithFunc(func(uint32, uint32) uint32 { return 0 }).Export("rust2_future_poll").
		NewFunctionBuilder().WithFunc(func(uint32, uint32, uint32, uint32, uint32) uint32 { return 0 }).Export("rust2_reader_read").
		NewFunctionBuilder().WithFunc(func(uint32, uint32, uint64, uint32, uint32) uint32 { return 0 }).Export("rust2_reader_seek").
		Instantiate(ctx)
	if err != nil {
		panic(err)
	}
	m, err := r.InstantiateWithConfig(ctx, wasm, wazero.NewModuleConfig().WithStartFunctions("_initialize").WithStdout(os.Stdout).WithStderr(os.Stderr))
	if err != nil {
		panic(err)
	}
	mem := m.Memory()
	f := m.ExportedFunction
	call := func(fn api.Function, args ...uint64) []uint64 {
		v, err := fn.Call(ctx, args...)
		if err != nil {
			panic(err)
		}
		return v
	}
	start = f("wb_Track_start")
	stack := uint32(call(f("wb_web_alloc"), 4096, 8)[0])
	track := call(f("wb_Track_new"))[0]
	noop, isClosed, largest := f("wb_noop"), f("wb_Track_is_closed"), f("wb_Track_largest")
	info, infoFree, write := f("wb_Track_info"), f("wb_Info_free"), f("wb_Track_write")
	payload, bufFree, readStart := f("wb_Track_payload"), f("wb_buf_free"), f("wb_Track_read_start")
	big, visit, fn0, boom := f("wb_Track_big"), f("wb_visit"), f("wb_web_fn_0"), f("wb_boom")
	frame := make([]byte, 100)
	// Reused argument and result slices, as wazero's CallWithStack lets a hot path keep.
	st := make([]uint64, 3)
	callStack := func(fn api.Function, args ...uint64) {
		copy(st, args)
		if err := fn.CallWithStack(ctx, st); err != nil {
			panic(err)
		}
	}
	fmt.Println("# core module, wasm32-wasip1, the exporter's C ABI, wazero", os.Args[2])
	measure("noop", n, func() { callStack(noop) })
	measure("handle call", n, func() { callStack(isClosed, track); sink += st[0] })
	measure("u64 getter", n, func() { callStack(start, track); sink += st[0] })
	measure("record return", n, func() {
		callStack(largest, uint64(stack), track)
		b, _ := mem.Read(stack, 24)
		sink += binary.LittleEndian.Uint64(b[8:])
	})
	measure("object return", n, func() { callStack(info, track); callStack(infoFree, st[0]) })
	measure("write 100 B", n, func() {
		mem.Write(stack+16, frame)
		var view [12]byte
		binary.LittleEndian.PutUint32(view[0:], stack+16)
		binary.LittleEndian.PutUint32(view[4:], 100)
		mem.Write(stack, view[:])
		callStack(write, track, uint64(stack))
	})
	read := func(lent bool) {
		b, _ := mem.Read(stack, 8)
		ptr, ln := binary.LittleEndian.Uint32(b), binary.LittleEndian.Uint32(b[4:])
		data, _ := mem.Read(ptr, ln)
		if lent {
			sink += uint64(data[0])
		} else {
			sink += uint64(len(append([]byte(nil), data...)))
		}
		callStack(bufFree, uint64(stack))
	}
	measure("payload 100 B", n, func() { callStack(payload, uint64(stack), track); read(false) })
	measure("payload 100 B lent", n, func() { callStack(payload, uint64(stack), track); read(true) })
	measure("ready read", n, func() {
		callStack(readStart, track, uint64(stack), uint64(stack+8))
		b, _ := mem.Read(stack, 8)
		sink += binary.LittleEndian.Uint64(b)
	})
	shim := call(fn0)[0]
	measure("lent closure", n, func() { callStack(visit, shim, 0) })
	measure("lent closure calling back", n, func() { callStack(visit, shim, track) })
	for _, ln := range []uint64{1 << 10, 1 << 20} {
		iters := n
		if ln > 1<<10 {
			iters = n / 1000
		}
		measure(fmt.Sprintf("payload %d B", ln), iters, func() { callStack(big, uint64(stack), track, ln); read(false) })
		measure(fmt.Sprintf("payload %d B lent", ln), iters, func() { callStack(big, uint64(stack), track, ln); read(true) })
	}
	_, err = boom.Call(ctx)
	fmt.Println("panic:", err != nil)
	_, err = noop.Call(ctx)
	fmt.Println("call after panic:", err)
}
