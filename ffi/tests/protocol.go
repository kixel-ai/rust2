// Runs a script tests/protocol/model.rs writes through the api fixture's Go: each step on
// the main goroutine or a worker goroutine, printing what it returned a line, which the
// model's transcript must match line for line. A drop is `Close()`; the collector frees in
// its own time, so the script frees nothing. A future is a blocking call, made and run to
// its end where the script awaits it.
package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"

	"api/api"
	"api/rt"
)

// A failed check's message, after the function's name, and the status it names.
var statuses = []struct{ text, name string }{
	{"a handle a call lent is borrowed exclusively or moved out", "lent"},
	{"a handle's value was moved out", "moved"},
	{"a handle is borrowed exclusively", "exclusive"},
	{"a handle is already borrowed", "borrowed"},
	{"a handle is borrowed on another thread", "thread"},
	{"a handle lent from another outlived it", "outlived"},
	{"a value that is not Send is used on another thread", "not_send"},
}

// The line for what a step returned: its value, Rust's own error, or the check that failed,
// a closed object's own error as a moved one's.
func outcome(value string, err error) string {
	if err == nil {
		return value
	}
	if !errors.Is(err, rt.ErrClosed) && !errors.Is(err, rt.ErrBorrowed) {
		return "error"
	}
	for _, status := range statuses {
		if strings.Contains(err.Error(), status.text) {
			return status.name
		}
	}
	if errors.Is(err, rt.ErrClosed) {
		return "moved"
	}
	panic(err)
}

func number[T uint32 | uint64](v T, err error) string {
	return outcome(strconv.FormatUint(uint64(v), 10), err)
}

func ok(err error) string {
	return outcome("ok", err)
}

// A future's call, which the step awaiting it makes.
type later func() (uint64, error)

var slots []any

func at[T any](token string) T {
	var none T
	if token == "-" {
		return none
	}
	n, _ := strconv.Atoi(token)
	return slots[n].(T)
}

func run(op string, args []string) string {
	a := -1
	if len(args) > 0 && args[0] != "-" {
		a, _ = strconv.Atoi(args[0])
	}
	arg := func(i int) uint64 {
		n, _ := strconv.ParseUint(args[i], 10, 64)
		return n
	}
	// Fill slot `a` with what a step made, if it made it.
	made := func(value any, err error) string {
		if err == nil {
			slots[a] = value
		}
		return ok(err)
	}
	ctx := context.Background()
	switch op {
	case "counter_new":
		return made(api.NewCounter("c", arg(1)))
	case "counter_get":
		return number(at[*api.Counter](args[0]).Get())
	case "counter_add":
		return number(at[*api.Counter](args[0]).Add(arg(1)))
	case "counter_into_sum":
		return number(at[*api.Counter](args[0]).IntoSum(arg(1)))
	case "counter_plus":
		return number(at[*api.Counter](args[0]).Plus(at[*api.Counter](args[1])))
	case "counter_add_to":
		added, err := at[*api.Counter](args[0]).AddTo(at[*api.Counter](args[1]))
		n := 0
		if added {
			n = 1
		}
		return number(uint64(n), err)
	case "counter_clone":
		return made(at[*api.Counter](args[1]).Clone())
	case "plus_later":
		self, other := at[*api.Counter](args[1]), at[*api.Counter](args[2])
		slots[a] = later(func() (uint64, error) { return self.PlusLater(ctx, other) })
	case "ledger_new":
		slots[a] = api.NewLedger()
	case "ledger_edit":
		draft, err := at[*api.Ledger](args[1]).Edit()
		if err == nil {
			slots[a] = draft
			return "1"
		}
		return ok(err)
	case "ledger_published":
		return number(at[*api.Ledger](args[0]).Published())
	case "ledger_seal":
		return ok(at[*api.Ledger](args[0]).Seal())
	case "ledger_receipt":
		return made(at[*api.Ledger](args[1]).Receipt())
	case "draft_write":
		return ok(at[*api.Draft](args[0]).Write("line"))
	case "draft_lines":
		return number(at[*api.Draft](args[0]).Lines())
	case "draft_commit":
		return number(at[*api.Draft](args[0]).Commit())
	case "receipt_count":
		return number(at[*api.Receipt](args[0]).Count())
	case "receipt_clone":
		return made(at[*api.Receipt](args[1]).Clone())
	case "tallied":
		return number(api.Tallied(at[*api.Receipt](args[0])))
	case "stored":
		return number(api.Stored(at[*api.Receipt](args[0])))
	case "tally_new":
		slots[a] = api.NewTally()
	case "tally_bump":
		return number(at[*api.Tally](args[0]).Bump())
	case "bump_later":
		tally := at[*api.Tally](args[1])
		slots[a] = later(func() (uint64, error) {
			n, err := tally.BumpLater(ctx)
			return uint64(n), err
		})
	case "local_new":
		// Made on a home, as every Go value that is not Send is.
		slots[a] = api.NewLocal()
	case "local_bump":
		return number(at[*api.Local](args[0]).Bump())
	case "local_share":
		return made(at[*api.Local](args[1]).Share())
	case "local_later":
		local := at[*api.Local](args[1])
		slots[a] = later(func() (uint64, error) {
			n, err := local.Later(ctx)
			return uint64(n), err
		})
	case "meters_new":
		slots[a] = api.NewMeters()
	case "meters_first":
		return made(at[*api.Meters](args[1]).First())
	case "meters_clear":
		return ok(at[*api.Meters](args[0]).Clear())
	case "meter_new":
		slots[a] = api.MeterDefault()
	case "meter_add":
		return number(at[*api.Meter](args[0]).Add(arg(1)))
	case "meter_reset":
		return ok(at[*api.Meter](args[0]).Reset())
	case "drop":
		return ok(slots[a].(interface{ Close() error }).Close())
	case "await":
		call := slots[a].(later)
		slots[a] = nil
		return number(call())
	case "drain":
		// Nothing stays on a thread of the script's, so no thread has a drop queued.
	default:
		panic("unknown op " + op)
	}
	return "ok"
}

type step struct {
	op   string
	args []string
}

func main() {
	file, err := os.Open(os.Args[1])
	if err != nil {
		panic(err)
	}
	lines := bufio.NewScanner(file)
	lines.Buffer(nil, 1<<20)
	lines.Scan()
	slots = make([]any, len(strings.Fields(lines.Text())))
	steps, done := make(chan step), make(chan string)
	go func() {
		for s := range steps {
			done <- run(s.op, s.args)
		}
	}()
	out := bufio.NewWriter(os.Stdout)
	for lines.Scan() {
		fields := strings.Fields(lines.Text())
		s := step{fields[1], fields[2:]}
		if fields[0] == "0" {
			fmt.Fprintln(out, run(s.op, s.args))
		} else {
			steps <- s
			fmt.Fprintln(out, <-done)
		}
	}
	close(steps)
	out.Flush()
}
