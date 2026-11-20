// The api fixture's Go test: tests/fixtures/transcript generates the script
// fixtures/api/examples/transcript.rs runs natively as transcript.go, beside this file, which
// calls the functions here that the transcript names. tests/fixtures.rs compares the
// transcripts.
//
// With an argument, it instead checks the futures' semantics, what Rust keeps of closures
// and reads of readers, what closing and the GC free, what a closed object returns, how
// long returned bytes live, what the transcript does not print, or breaks a contract,
// which must fail.
package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"math"
	"math/big"
	"net/netip"
	"os"
	"runtime"
	"runtime/pprof"
	"slices"
	"strings"
	"sync"
	"sync/atomic"
	"time"
	"unicode"
	"unsafe"

	"api/api"
	"api/rt"
	"api/serdejson"
	"api/tls"
	"api/url"
)

// Print is the transcript's: it keeps even levels.
type Print struct{}

// Close runs once Rust drops the implementation.
func (Print) Close() error {
	fmt.Println("freed")
	return nil
}

func (Print) Log(level uint8, line string) bool {
	fmt.Printf("log %d %s\n", level, line)
	return level%2 == 0
}

// list is items as Rust's Debug prints a slice, each by show.
func list[T any](items []T, show func(T) string) string {
	shown := make([]string, len(items))
	for i, item := range items {
		shown[i] = show(item)
	}
	return "[" + strings.Join(shown, ", ") + "]"
}

func number[T any](n T) string {
	return fmt.Sprint(n)
}

// numbers prints each of n's lists, its floats as their bits, as transcript.rs does.
func numbers(what string, n api.Numbers) {
	fmt.Printf("%s u16 %s\n", what, list(n.U16s, number))
	fmt.Printf("%s u32 %s\n", what, list(n.U32s, number))
	fmt.Printf("%s u64 %s\n", what, list(n.U64s, number))
	fmt.Printf("%s i8 %s\n", what, list(n.I8s, number))
	fmt.Printf("%s i16 %s\n", what, list(n.I16s, number))
	fmt.Printf("%s i32 %s\n", what, list(n.I32s, number))
	fmt.Printf("%s i64 %s\n", what, list(n.I64s, number))
	fmt.Printf("%s f32 %s\n", what, list(n.F32s, func(f float32) string { return number(math.Float32bits(f)) }))
	fmt.Printf("%s f64 %s\n", what, list(f64Bits(n.F64s), number))
	if n.Some == nil {
		fmt.Printf("%s some None\n", what)
	} else {
		fmt.Printf("%s some Some(%s)\n", what, list(*n.Some, number))
	}
}

// f64Bits is each float's bits.
func f64Bits(floats []float64) []uint64 {
	bits := make([]uint64, len(floats))
	for i, f := range floats {
		bits[i] = math.Float64bits(f)
	}
	return bits
}

// hex is bytes, a slice or an array of them, in lowercase hex, two digits each.
func hex(bytes any) string {
	if b, ok := bytes.(rt.Bytes); ok {
		bytes = b.Copy()
	}
	return fmt.Sprintf("%x", bytes)
}

// pathHex is a path's bytes in hex.
func pathHex(path string) string {
	return fmt.Sprintf("%x", path)
}

// seconds is a duration as its seconds and nine digits of nanoseconds.
func seconds(d rt.Duration) string {
	return fmt.Sprintf("%d.%09d", d.Secs(), d.Nanos())
}

// millis is a duration in whole milliseconds, as Rust's `as u64` truncates them.
func millis(d rt.Duration) uint64 {
	return d.Secs()*1000 + uint64(d.Nanos())/1_000_000
}

// code is a char's code point.
func code(c rune) uint32 {
	return uint32(c)
}

// reversed is the list reversed.
func reversed(list []*tls.Codec) []*tls.Codec {
	list = slices.Clone(list)
	slices.Reverse(list)
	return list
}

// unspelled is a path no Rust string spells: on Unix, bytes that are not UTF-8.
func unspelled() string {
	return api.Stray()
}

// socketHex is a socket address of any family as its IP's octets in hex, its port, and,
// IPv6's alone, its flow label and scope, apart by colons.
func socketHex(addr any) string {
	switch a := addr.(type) {
	case rt.SocketAddrV4:
		return fmt.Sprintf("%x:%d", a.IP, a.Port)
	case rt.SocketAddrV6:
		return fmt.Sprintf("%x:%d:%d:%d", a.IP, a.Port, a.FlowInfo, a.ScopeID)
	case rt.SocketAddr:
		if v4, ok := a.V4(); ok {
			return socketHex(v4)
		}
		v6, _ := a.V6()
		return socketHex(v6)
	}
	panic(fmt.Sprintf("no socket address is a %T", addr))
}

// ipHex is an IP address of any family's octets in hex, so the length shows its family.
func ipHex(ip any) string {
	switch a := ip.(type) {
	case rt.Ipv4Addr:
		return fmt.Sprintf("%x", a)
	case rt.Ipv6Addr:
		return fmt.Sprintf("%x", a)
	case rt.IpAddr:
		if v4, ok := a.V4(); ok {
			return ipHex(v4)
		}
		v6, _ := a.V6()
		return ipHex(v6)
	}
	panic(fmt.Sprintf("no IP address is a %T", ip))
}

// escaped is a path's bytes as Rust's escape_ascii prints them.
func escaped(path string) string {
	var out strings.Builder
	for _, b := range []byte(path) {
		switch {
		case b == '\t':
			out.WriteString(`\t`)
		case b == '\r':
			out.WriteString(`\r`)
		case b == '\n':
			out.WriteString(`\n`)
		case b == '\\' || b == '\'' || b == '"':
			out.WriteByte('\\')
			out.WriteByte(b)
		case b >= 0x20 && b < 0x7f:
			out.WriteByte(b)
		default:
			fmt.Fprintf(&out, `\x%02x`, b)
		}
	}
	return out.String()
}

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

// ok fails the test on err, of a call that returns nothing else.
func ok(err error) {
	if err != nil {
		panic(err)
	}
}

func ptr[T any](v T) *T {
	return &v
}

// watched is a context that reports when a call first waits on it, which it does only
// once its future pends, and that holds that call between a wake and its next poll until
// the test releases it.
type watched struct {
	context.Context
	waiting chan struct{}
	once    sync.Once
	gate    atomic.Pointer[chan struct{}]
}

func watch(ctx context.Context) *watched {
	return &watched{Context: ctx, waiting: make(chan struct{})}
}

func (w *watched) Done() <-chan struct{} {
	w.once.Do(func() { close(w.waiting) })
	return w.Context.Done()
}

func (w *watched) Err() error {
	if gate := w.gate.Load(); gate != nil {
		<-*gate
	}
	return w.Context.Err()
}

// channel is the transcript's: a channel's futures, each a call blocking until Rust's
// future is ready, its pending seen as the call waits on its context.
func channel() {
	ctx := context.Background()
	c := api.NewChannel()
	// The receive waits once its future pends, until the send wakes it.
	watching := watch(ctx)
	got := make(chan []byte)
	go func() { got <- must(c.Recv(watching)).Copy() }()
	<-watching.waiting
	fmt.Println("recv pending true")
	c.Send(rt.NewBytes([]byte("hi")))
	payload := <-got
	wakes := 1
	fmt.Printf("woken %d\n", wakes)
	fmt.Printf("recv %s\n", payload)
	c.Send(rt.Bytes{})
	if nonempty, err := c.RecvNonempty(ctx); err != nil {
		fmt.Printf("nonempty err %s\n", err)
	} else {
		fmt.Printf("nonempty %d\n", nonempty.Len())
	}
	fmt.Print("fill")
	for _, v := range append(must(api.Fill4(ctx, api.NewBatch(), 9)), must(api.Fill2(ctx, api.Pair(), 1))...) {
		fmt.Printf(" %d", v)
	}
	fmt.Println()
	// The send happens at the call: the future is cancelled before its first poll.
	cancelled, cancel := context.WithCancel(ctx)
	cancel()
	check(errors.Is(c.SendLater(cancelled, rt.NewBytes([]byte("eager"))), context.Canceled), "a cancelled call returns its context's error")
	fmt.Printf("recv %s\n", must(c.Recv(ctx)).Copy())
	// The send happens at the call, and the future borrows the channel and the tag.
	fmt.Printf("tagged %s\n", must(c.Tagged(ctx, "tag")))
	fmt.Printf("itemized %d\n", must(c.Itemized(ctx, &[]uint32{1, 2, 3})))
	fmt.Printf("byte_len %d\n", must(api.ByteLen(ctx, rt.NewBytes([]byte("abc")))))
	c.Send(rt.NewBytes([]byte("sized")))
	sized, size := must2(c.RecvSized(ctx))
	fmt.Printf("sized %s %d\n", sized.Copy(), size)
	// The waker moves into Rust, which keeps it until the send wakes it.
	c.Notify(func() { wakes++ })
	c.Send(rt.NewBytes([]byte("rung")))
	fmt.Printf("woken %d\n", wakes)
	// A poll with no async twin is a future.
	fmt.Printf("take %s\n", must(c.PollTake(ctx)).Copy())
	c.Send(rt.NewBytes([]byte("measured")))
	fmt.Printf("measure %d\n", must(c.PollMeasure(ctx, 2)))
	// A context is made of the waker, which Rust keeps until the send wakes it.
	c.Listen(func() { wakes++ })
	c.Send(rt.NewBytes([]byte("heard")))
	fmt.Printf("woken %d\n", wakes)
	// A returned waker wakes the waiter's task through Rust until the GC collects it.
	waiter := api.NewWaiter(func() { wakes++ })
	must(waiter.Waker())()
	must(waiter.Context())()
	waiter.Close()
	fmt.Printf("woken %d\n", wakes)
}

// streams is the transcript's: streams ranged over by All, to their end, left early, and
// to an error, which ends the range but not the stream.
func streams() {
	ctx := context.Background()
	var counts []string
	for count, err := range api.NewCountdown(3).All(ctx) {
		ok(err)
		counts = append(counts, fmt.Sprint(count))
	}
	fmt.Printf("countdown %s\n", strings.Join(counts, " "))
	// A range left early takes nothing past the item it stopped at.
	countdown := api.NewCountdown(5)
	for count := range countdown.All(ctx) {
		if count == 4 {
			break
		}
	}
	fmt.Printf("countdown after break %d\n", *must(countdown.Next(ctx)))
	// Each payload sends the next, which `!` fails and an empty one ends.
	c := api.NewChannel()
	feed := must(c.Feed())
	replies := map[string]string{"a": "b", "b": "!"}
	for _, first := range []string{"a", "c"} {
		c.Send(rt.NewBytes([]byte(first)))
		failed := false
		for payload, err := range feed.All(ctx) {
			check(!failed, "a stream yields nothing past its error")
			if err != nil {
				fmt.Printf("feed error %s\n", err)
				failed = true
				continue
			}
			fmt.Printf("feed %s\n", payload.Copy())
			c.Send(rt.NewBytes([]byte(replies[string(payload.Copy())])))
		}
		if !failed {
			fmt.Println("feed end")
		}
	}
}

// must2 is a call's two values, or a panic on its error.
func must2[A, B any](a A, b B, err error) (A, B) {
	if err != nil {
		panic(err)
	}
	return a, b
}

// drive is the transcript's: a kept closure, called again at the instant it asks for, then
// once its waiter wakes.
func drive() {
	start := *api.NewMoment(0, 1000)
	calls := 0
	stopped := must(api.Drive(context.Background(), start, func(now api.Moment, waiter *api.Waiter) (*api.Moment, error) {
		calls++
		switch calls {
		case 1:
			later := now.After(5)
			return &later, nil
		case 2:
			fmt.Printf("drive %d\n", now.Millis()-start.Millis())
			must(waiter.Waker())()
			return nil, nil
		}
		return nil, fmt.Errorf("stopped after %d", calls)
	}))
	fmt.Printf("drive %s\n", stopped)
}

// readers is the transcript's: one over bytes, one whose reads fail, and a seek from the
// start past `math.MaxInt64`, which reaches the reader whole, as `Cursor` takes one.
func readers() {
	tail(strings.NewReader("hello world"))
	tail(brokenReader{})
	if at, err := api.SeekTo(farReader{}, math.MaxUint64); err != nil {
		fmt.Printf("sought error %s\n", err)
	} else {
		fmt.Printf("sought %d\n", at)
	}
}

// localLater is the transcript's: futures of a value that is not `Send`, each awaited on
// the home that runs it.
func localLater(local *api.Local) {
	ctx := context.Background()
	fmt.Printf("local later %d\n", must(must(api.LocalOrNewLater(ctx, local)).Bump()))
	fmt.Printf("local later fresh %d\n", must(must(api.LocalOrNewLater(ctx, nil)).Bump()))
}

// checks checks what the transcript does not print, each by an assertion: errors as their
// types, what a refused call leaves, and what only Go's calls can pass.
func checks() {
	ctx := context.Background()
	// An exported error type is returned as itself.
	{
		c := must(api.NewCounter("c", 5))
		var e api.Error
		_, err := c.Add(2000)
		check(errors.As(err, &e) && e.GoString() == "Overflow", "add returns an api.Error")
		ok(c.Close())
	}
	// Refused over a closed mark, the call writes nothing back.
	{
		noted := api.ScoreOf(2)
		by := api.MarkOf(5)
		check(must(noted.Note(by)) == 2 && noted.Count() == 7, "a note adds its mark")
		ok(by.Close())
		_, refused := noted.Note(by)
		check(refused != nil && noted.Count() == 7, "a refused note leaves the score as it was")
	}
	// An eager call whose callback cancels its context while it makes its future is never
	// polled: only the eager half logs.
	{
		inner, cancel := context.WithCancel(ctx)
		log := &cancelLog{cancel: cancel}
		_, err := api.EmitLater(inner, log, rt.NewBytes([]byte{1, 2}))
		check(errors.Is(err, context.Canceled), "an eager call cancelled as it makes its future returns its context's error")
		check(log.lines.Load() == 2, "an eager call cancelled as it makes its future is never polled")
	}
	// A function named as a type of its package, which `area` names in another's.
	check(api.Span_(1, 4) == 3, "a function named as a type of its package")
	// A path that is no UTF-8 is a Go string all the same.
	check(api.Stray() == "\xff", "a path that is not UTF-8 crosses as its bytes")
	var lent string
	api.VisitStray(func(path string) { lent = path })
	check(lent == "\xff", "a path Rust lends that is not UTF-8 crosses as its bytes")
	// An error Rust returns only boxed is its type.
	_, err := api.Refuse("why")
	var refusal *api.Refusal
	check(errors.As(err, &refusal) && err.Error() == "refused: why", "a boxed error is its type")
	// Another crate's error crosses as its message.
	var parse rt.StringError
	_, err = url.UrlParse("no scheme")
	check(errors.As(err, &parse) && parse.Value == "relative URL without a base", "a URL's parse returns its message")
	_, err = serdejson.JsonParse(`{"a":`)
	check(errors.As(err, &parse), "a JSON parse returns its message")
	// A draft borrows its ledger exclusively until it closes: the ledger refuses any other
	// use meanwhile.
	{
		ledger := api.NewLedger()
		draft := must(ledger.Edit())
		_, exclusive := ledger.Published()
		check(
			errors.Is(exclusive, rt.ErrBorrowed) && exclusive.Error() == "api_Ledger_published: a handle is borrowed exclusively",
			fmt.Sprintf("a lent ledger refuses a read: %v", exclusive),
		)
		check(draft.Close() == nil, "a draft closes")
		check(must(ledger.Published()) == 0, "a closed draft frees its ledger")
		check(ledger.Close() == nil, "a ledger closes")
	}
	// A block lends a sub-slice in place, and Rust's `None` and empty `Some` stay apart.
	framed := []uint32{9, 1, 2, 9}
	check(api.SumU32(framed[1:3]) == 3, "a sub-slice lends only its own items")
	edges := api.NumbersEdges()
	edges.Some = nil
	check(edges.Reversed().Some == nil, "a nil block in an option is None")
	edges.Some = &[]int32{}
	check(edges.Reversed().Some != nil && len(*edges.Reversed().Some) == 0, "an empty block in an option is Some")
	edges.Some = &[]int32{1, 2}
	check(list(*edges.Reversed().Some, number) == "[2, 1]", "a block in an option crosses whole")
	// The packages serve wasm32 too: the traits of a type only wasm32 has fail on the host's
	// library as its functions do.
	{
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
	}
	// A kept closure, called on another thread after the call that took it returned.
	spawned := must(api.NewSpawned(func() uint32 { return 7 }))
	check(must(spawned.Join()) == 7, "a kept closure runs on another thread")
	checked()
	fmt.Println("checks ok")
}

// farReader is a reader whose reads fail, which seeks anywhere from its start, as Rust's
// Cursor does.
type farReader struct{ brokenReader }

var _ rt.StartSeeker = farReader{}

func (farReader) SeekStart(offset uint64) (uint64, error) {
	return offset, nil
}

// tail prints the last 5 bytes of file, or its error.
func tail(file io.ReadSeeker) {
	if bytes, at, err := api.Tail(file, 5); err != nil {
		fmt.Printf("tail error %s\n", err)
	} else {
		fmt.Printf("tail %s %d\n", bytes.Copy(), at)
	}
}

// brokenReader is a reader whose reads fail, as the transcript's: each seek lands at 0.
type brokenReader struct{}

func (brokenReader) Read([]byte) (int, error) {
	return 0, errors.New("broken")
}

func (brokenReader) Seek(int64, int) (int64, error) {
	return 0, nil
}

// lastReader is bytes in memory whose last read returns err beside its bytes, as a Go
// reader may.
type lastReader struct {
	*strings.Reader
	err error
}

func (r lastReader) Read(p []byte) (int, error) {
	n, err := r.Reader.Read(p)
	if err == nil && r.Reader.Len() == 0 {
		err = r.err
	}
	return n, err
}

// stuckReader reads nothing and no error, forever.
type stuckReader struct {
	brokenReader
}

func (stuckReader) Read([]byte) (int, error) {
	return 0, nil
}

// unseekable is a reader whose seeks fail.
type unseekable struct {
	brokenReader
}

func (unseekable) Seek(int64, int) (int64, error) {
	return 0, errors.New("no seek")
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

// marked is a value whose collection sets the returned flag, for a closure to hold.
func marked() (*[4]int, *atomic.Bool) {
	var released atomic.Bool
	// Larger than the tiny allocator's blocks, whose objects may never be cleaned up.
	marker := new([4]int)
	runtime.AddCleanup(marker, func(released *atomic.Bool) { released.Store(true) }, &released)
	return marker, &released
}

// closures checks what Rust keeps and releases, and readers' reads, seeks, and errors.
func closures() {
	ctx := context.Background()

	// A kept closure Rust releases once it drops it, after its thread called it.
	{
		marker, freed := marked()
		spawned := must(api.NewSpawned(func() uint32 { return uint32(marker[0]) + 1 }))
		check(must(spawned.Join()) == 1, "a kept closure runs on Rust's thread")
		released(freed, "Rust releases a kept closure it dropped")
	}

	// A kept closure the future holds until it is dropped, unfinished.
	{
		marker, freed := marked()
		called := make(chan struct{}, 1)
		start := *api.NewMoment(0, 1000)
		cancelled, cancel := context.WithCancel(ctx)
		done := make(chan error)
		go func() {
			_, err := api.Drive(cancelled, start, func(api.Moment, *api.Waiter) (*api.Moment, error) {
				_ = marker[0]
				select {
				case called <- struct{}{}:
				default:
				}
				return nil, nil
			})
			done <- err
		}()
		<-called
		for range 5 {
			runtime.GC()
			runtime.Gosched()
		}
		check(!freed.Load(), "a pending future keeps its closure")
		cancel()
		check(errors.Is(<-done, context.Canceled), "a cancelled drive returns its context's error")
		released(freed, "a dropped future releases its closure")
	}

	// A callback a failed check stops before the call never reaches Rust, so nothing keeps it.
	{
		marker, freed := marked()
		gone := must(api.NewCounter("gone", 1))
		check(gone.Close() == nil, "Close passes")
		_, err := api.LogCounts(markedLog{marker}, 0, must(api.NewCounter("c", 1)), []*api.Counter{gone})
		check(errors.Is(err, rt.ErrClosed), "a closed object a list holds fails the call")
		released(freed, "a call a check failed releases its callback")
	}

	// What a closure returns crosses: the instant it asks for, then its error.
	{
		start := *api.NewMoment(0, 1000)
		later := start.After(1000)
		var seen []uint64
		stopped := must(api.Drive(ctx, start, func(now api.Moment, _ *api.Waiter) (*api.Moment, error) {
			seen = append(seen, now.Millis()-start.Millis())
			if len(seen) == 1 {
				return &later, nil
			}
			return nil, errors.New("")
		}))
		check(stopped == "" && len(seen) == 2 && seen[1] == 1000, "a returned instant and an empty error cross")
	}

	// A read's bytes and error both reach Rust, the error on the next read.
	{
		bytes, at, err := api.Tail(lastReader{strings.NewReader("hello world"), errors.New("late")}, 5)
		check(err != nil && err.Error() == "late" && bytes.Len() == 0 && at == 0, "an error beside bytes is the next read's")
		bytes, at, err = api.Tail(lastReader{strings.NewReader("hello world"), io.EOF}, 5)
		check(err == nil && string(bytes.Copy()) == "world" && at == 6, "io.EOF beside bytes ends the reader")
		_, _, err = api.Tail(stuckReader{}, 5)
		check(err != nil && err.Error() == io.ErrNoProgress.Error(), "a reader reading nothing gives up")
		_, _, err = api.Tail(unseekable{}, 5)
		check(err != nil && err.Error() == "no seek", "a seek's error crosses")
		// A file, as moq-mux's repair reads one.
		file := must(os.CreateTemp("", "rust2-ffi-go-tail"))
		defer os.Remove(file.Name())
		must(file.WriteString("on disk"))
		bytes, at, err = api.Tail(file, 4)
		check(err == nil && string(bytes.Copy()) == "disk" && at == 3, "a file reads as a reader")
		// Lent, never closed: the file is still the caller's.
		check(must(file.Seek(0, io.SeekCurrent)) == 3, "Rust leaves a file it read open, where it sought")
		file.Close()
	}
	fmt.Println("closures ok")
}

// futures checks the futures' semantics beyond the transcript, each by an assertion.
func futures() {
	ctx := context.Background()
	channel := api.NewChannel()

	// The send lands from another goroutine's thread while the receive waits.
	go channel.Send(rt.NewBytes([]byte("blocked")))
	check(string(must(channel.Recv(ctx)).Copy()) == "blocked", "recv returns the payload another goroutine sent")

	// A stream's range ends with ctx's error once ctx is done while its Next pends, which
	// drops Rust's future and loses nothing: the stream stays usable, and the payload sent
	// afterwards is the next call's.
	{
		feed := must(channel.Feed())
		cancelled, cancel := context.WithCancel(ctx)
		watching := watch(cancelled)
		ended := make(chan error)
		go func() {
			for _, err := range feed.All(watching) {
				ended <- err
				return
			}
			ended <- nil
		}()
		<-watching.waiting
		cancel()
		check(errors.Is(<-ended, context.Canceled), "a range whose context ends yields its error")
		channel.Send(rt.NewBytes([]byte("kept")))
		check(string(must(feed.Next(ctx)).Copy()) == "kept", "a cancelled range loses nothing")
		// A break takes nothing past its item: the payload sent after it is the next call's.
		channel.Send(rt.NewBytes([]byte("first")))
		for payload, err := range feed.All(ctx) {
			ok(err)
			check(string(payload.Copy()) == "first", "a range yields the payload sent")
			channel.Send(rt.NewBytes([]byte("second")))
			break
		}
		check(string(must(feed.Next(ctx)).Copy()) == "second", "a range reads nothing ahead")
	}

	// A wake that lands while the future polls, before it returns pending, is kept: the
	// call polls again rather than waiting for a wake that already happened.
	check(must(api.Yielded(ctx, 7)) == 7, "a wake during the poll polls again")

	// A value that is not Send lives on a home, a thread Rust owns, which runs each call on
	// it from any goroutine, and drops it there once closed or collected.
	local := api.NewLocal()
	shared := must(local.Share())
	must(local.Bump())
	check(must(shared.Bump()) == 2, "a shared local counts together")
	check(must(local.Later(ctx)) == 3, "a local's future runs on its home")
	check(must(api.LocalLater(ctx, 5)) == 10, "a future that is not Send runs on a home")
	check(must(api.BumpEither(nil, local)) == 4, "a call whose first local is absent runs on its second's home")
	check(must(local.Visit(func() uint32 { return must(local.Bump()) })) == 5, "a callback on a home calls back into its local")
	other := api.NewLocal()
	nested := must(local.Visit(func() uint32 {
		return must(other.Visit(func() uint32 { return must(local.Bump()) }))
	}))
	check(nested == 6, "a home waiting on another runs a call back into its own local")
	ok(other.Close())
	made := must(api.LocalAfter(ctx))
	check(must(made.OnOwnThread()), "a future's output that is not Send is made on its home")
	ok(made.Close())
	fresh := must(api.LocalOrNew(nil))
	var wg sync.WaitGroup
	for range 8 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			check(must(local.OnOwnThread()), "a local's call from any goroutine runs on its home")
			check(must(fresh.OnOwnThread()), "a local made with its optional local absent lives on a home")
		}()
	}
	wg.Wait()
	ok(fresh.Close())
	before := api.LocalsDropped()
	wg.Add(1)
	go func() {
		defer wg.Done()
		ok(local.Close())
	}()
	wg.Wait()
	check(api.LocalsDropped() == before+1, "a local closed on another goroutine drops on its home")
	// Collected only from here, after the count before it.
	runtime.KeepAlive(shared)
	shared = nil
	deadline := time.Now().Add(10 * time.Second)
	for api.LocalsDropped() < before+2 {
		check(time.Now().Before(deadline), "a collected local drops on its home")
		runtime.GC()
		runtime.Gosched()
	}

	// A future's output of None is nil.
	half := must(api.Halved(ctx, 4))
	check(half != nil && *half == 2, "a future's Some output reads back")
	check(must(api.Halved(ctx, 3)) == nil, "a future's None output is nil")

	// Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	// for the next.
	{
		inner, cancel := context.WithCancel(ctx)
		watching := watch(inner)
		done := make(chan error)
		go func() {
			_, err := channel.Recv(watching)
			done <- err
		}()
		<-watching.waiting
		// Hold the call between the wake and its next poll, where it reads its context.
		gate := make(chan struct{})
		watching.gate.Store(&gate)
		channel.Send(rt.NewBytes([]byte("kept")))
		cancel()
		close(gate)
		check(errors.Is(<-done, context.Canceled), "a receive cancelled after its wake returns its context's error")
		check(string(must(channel.Recv(ctx)).Copy()) == "kept", "a cancelled receive loses nothing")
	}

	// A context done before the call takes nothing, even a payload ready at the first poll.
	{
		channel.Send(rt.NewBytes([]byte("ready")))
		cancelled, cancel := context.WithCancel(ctx)
		cancel()
		_, err := channel.Recv(cancelled)
		check(errors.Is(err, context.Canceled), "a call with a done context returns its error")
		check(string(must(channel.Recv(ctx)).Copy()) == "ready", "a call with a done context takes nothing")
	}

	// A deadline ends a receive that waits for no wake.
	{
		short, cancel := context.WithTimeout(ctx, time.Millisecond)
		defer cancel()
		_, err := channel.Recv(short)
		check(errors.Is(err, context.DeadlineExceeded), "a receive past its deadline returns its error")
	}

	// Closing a handle a call waiting on it borrows fails, as Rust refuses a drop under a
	// borrow, and changes nothing: the call ends as it would, and then Close passes.
	{
		other := api.NewChannel()
		watching := watch(ctx)
		got := make(chan string)
		go func() { got <- string(must(other.Recv(watching)).Copy()) }()
		<-watching.waiting
		sender := must(other.Clone())
		err := other.Close()
		check(errors.Is(err, rt.ErrBorrowed) && err.Error() == "api_Channel_drop: a handle is already borrowed", "Close under a waiting call fails")
		sender.Send(rt.NewBytes([]byte("held")))
		check(<-got == "held", "the waiting call ends as it would")
		check(other.Close() == nil, "Close passes once no call borrows it")
		_, err = other.Recv(ctx)
		check(errors.Is(err, rt.ErrClosed) && err.Error() == "Channel is closed", "a call after Close fails")
		sender.Close()
	}

	// Go functions Rust polls as futures, each run on a goroutine of its own.
	{
		ran := make(chan struct{}, 1)
		check(must(api.After(ctx, func(context.Context) { ran <- struct{}{} }, 4)) == 8 && len(ran) == 1, "Rust awaits a Go function")
	}
	{
		gate := make(chan struct{})
		got := make(chan uint32)
		go func() { got <- must(api.After(ctx, func(context.Context) { <-gate }, 5)) }()
		select {
		case <-got:
			check(false, "Rust waits on a Go function that blocks")
		case <-time.After(10 * time.Millisecond):
		}
		close(gate)
		check(<-got == 10, "a Go function's return wakes Rust")
	}
	{
		err := api.Within(ctx, 0, func(context.Context) {})
		check(errors.Is(err, api.ErrorOverflow{}), "Rust gives up on a Go function")
	}
	{
		short, cancel := context.WithTimeout(ctx, 10*time.Millisecond)
		defer cancel()
		cancelled := make(chan struct{})
		err := api.Within(short, 1000, func(c context.Context) {
			<-c.Done()
			close(cancelled)
		})
		check(errors.Is(err, context.DeadlineExceeded), "a call waiting on a Go function ends at its deadline")
		select {
		case <-cancelled:
		case <-time.After(5 * time.Second):
			check(false, "dropping a future Rust polls cancels its context")
		}
	}
	{
		// The body's context carries the caller's values and deadline.
		type key struct{}
		caller, cancel := context.WithTimeout(context.WithValue(ctx, key{}, "caller"), time.Hour)
		want, _ := caller.Deadline()
		var value any
		var deadline time.Time
		ok(api.Within(caller, 1000, func(c context.Context) {
			value = c.Value(key{})
			deadline, _ = c.Deadline()
		}))
		cancel()
		check(value == "caller" && deadline.Equal(want), "a Go function runs under the caller's context")
	}
	{
		// A body that returns once the caller cancelled, even one that ignores its
		// context, ends unfinished: the call reports the cancel, never Rust's success.
		caller, cancel := context.WithCancel(ctx)
		_, err := api.After(caller, func(context.Context) { cancel() }, 1)
		check(errors.Is(err, context.Canceled), "a caller's cancel the body sees reports the cancel")
	}
	{
		// The caller's cancel reaches the body's context.
		caller, cancel := context.WithCancel(ctx)
		started, cancelled := make(chan struct{}), make(chan struct{})
		got := make(chan error)
		go func() {
			_, err := api.After(caller, func(c context.Context) {
				close(started)
				<-c.Done()
				close(cancelled)
			}, 1)
			got <- err
		}()
		// Cancelled before its first poll, the call would never start the body.
		<-started
		cancel()
		check(errors.Is(<-got, context.Canceled), "a cancelled call waiting on a Go function reports the cancel")
		<-cancelled
	}

	// A waiter's poll of a Go function, made of each poll's waker, which the future owns.
	{
		ran := make(chan struct{}, 1)
		ok(api.WaiterPollFuture(ctx, func(context.Context) { ran <- struct{}{} }))
		check(len(ran) == 1, "a waiter polls a Go function to its end")
	}
	{
		gate := make(chan struct{})
		got := make(chan error)
		go func() { got <- api.WaiterPollFuture(ctx, func(context.Context) { <-gate }) }()
		select {
		case <-got:
			check(false, "a waiter's poll waits on a Go function that blocks")
		case <-time.After(10 * time.Millisecond):
		}
		close(gate)
		check(<-got == nil, "a Go function's return wakes the waiter's poll")
	}
	{
		// Dropped before it finishes, it cancels the Go function it owns.
		short, cancel := context.WithTimeout(ctx, 10*time.Millisecond)
		defer cancel()
		cancelled := make(chan struct{})
		err := api.WaiterPollFuture(short, func(c context.Context) {
			<-c.Done()
			close(cancelled)
		})
		check(errors.Is(err, context.DeadlineExceeded), "a waiter's poll ends at its deadline")
		select {
		case <-cancelled:
		case <-time.After(5 * time.Second):
			check(false, "dropping a waiter's poll cancels the Go function it owns")
		}
	}
	fmt.Println("futures ok")
}

// owned checks what frees an object: Close at once, or else the GC once it collects the
// object.
func owned() {
	for n := 2047; n <= 2049; n++ {
		payload := make([]byte, n)
		for i := range payload {
			payload[i] = byte(i)
		}
		packet := api.NextPacket(api.Packet{Seq: 1, Payload: rt.NewBytes(payload)})
		check(packet.Seq == 2 && string(packet.Payload.Copy()) == string(payload), fmt.Sprintf("a record's %d bytes cross both ways", n))
	}
	before := api.Dropped()
	token := api.NewToken(2)
	token.Close()
	check(api.Dropped() == before+1, "closing frees at once")
	token.Close()
	check(api.Dropped() == before+1, "closing again frees nothing")

	unclosed()
	collected(before+2, "the GC frees what nobody closed")
	runtime.GC()
	check(api.Dropped() == before+2, "the GC frees each once")

	before = api.Dropped()
	forward()
	collected(before+1, "a forwarded payload drops once nothing reaches it")
	uncollected(before+1, "a forwarded payload drops once")

	nursed()
	fmt.Println("owned ok")
}

// nursed checks the nursery of the latest objects returned, which a Close leaves at once
// and the GC frees only once later returns push one out unclosed: each object is freed
// once, however Close and the nursery race.
func nursed() {
	// One P, so every return below shares one nursery, which each GC finds in use, so its
	// pool never drops it: only the push out of it registers the dropped token. The race
	// detector's pool drops some at random, which frees it as a dropped nursery does.
	procs := runtime.GOMAXPROCS(1)
	before := api.Dropped()
	unclosed()
	deadline := time.Now().Add(10 * time.Second)
	for api.Dropped() < before+1 {
		check(time.Now().Before(deadline), "the GC frees an object pushed out of its nursery unclosed")
		fill()
		runtime.GC()
		runtime.Gosched()
	}

	// Closed after the push out registered it, so its cleanup is stopped.
	kept := api.NewToken(4)
	fill()
	kept.Close()
	check(api.Dropped() == before+2, "closing an object pushed out of its nursery frees it at once")
	runtime.GOMAXPROCS(procs)
	uncollected(before+2, "the GC frees nothing a Close freed after a push out")

	// Returned on many goroutines, each closing some of its own and of another's, as
	// others' returns push them out, and dropping the rest.
	const workers, each = 8, 1000
	handoff := make(chan *api.Token, workers*each)
	var wg sync.WaitGroup
	for w := range workers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := range each {
				t := api.NewToken(uint32(i))
				switch i % 3 {
				case 0:
					t.Close()
				case 1:
					handoff <- t
				}
				if i%3 != 0 {
					select {
					case other := <-handoff:
						other.Close()
					default:
					}
				}
				if w == 0 && i%100 == 0 {
					runtime.GC()
				}
			}
		}()
	}
	wg.Wait()
	close(handoff)
	for t := range handoff {
		t.Close()
	}
	collected(before+2+workers*each, "the GC frees every object a nursery held unclosed")
	uncollected(before+2+workers*each, "each object is freed once, however its Close and its nursery race")
}

// fill returns a nursery's worth of objects, each closed, pushing out every earlier one.
func fill() {
	for range 64 {
		api.NewTally().Close()
	}
}

// forward reads a payload Rust returned and writes it back, whole, sliced, and in a
// record: Rust takes a second owner of its own buffer each time, so the bytes stay where
// Rust put them, in a frame of its own so nothing keeps them reachable after.
//
//go:noinline
func forward() {
	ctx := context.Background()
	channel := api.NewChannel()
	defer channel.Close()
	// Long enough that Rust's buffer is lent rather than copied.
	payload := api.TokenBytes(4, 1024)
	at := addr(payload)
	channel.Send(payload)
	got := must(channel.Recv(ctx))
	check(addr(got) == at && got.Len() == 1024, "a payload sent back is Rust's own")
	channel.Send(got.Slice(1, 1023))
	got = must(channel.Recv(ctx))
	check(addr(got) == at+1 && got.Len() == 1022, "a slice of a payload sent back is Rust's own")
	packet := api.NextPacket(api.Packet{Seq: 1, Payload: got})
	check(addr(packet.Payload) == at+1 && packet.Payload.Len() == 1022, "a record carries a payload back as Rust's own")
}

// addr is where b's bytes start, as an io.Writer sees them.
func addr(b rt.Bytes) uintptr {
	var w addrWriter
	_, _ = b.WriteTo(&w)
	return uintptr(w)
}

type addrWriter uintptr

func (w *addrWriter) Write(p []byte) (int, error) {
	*w = addrWriter(unsafe.Pointer(&p[0]))
	return len(p), nil
}

// unclosed makes a token in a frame of its own, so nothing keeps it reachable.
//
//go:noinline
func unclosed() {
	api.NewToken(3)
}

// collected waits for the GC to have dropped Rust's values until dropped counts n.
func collected(n uint32, what string) {
	deadline := time.Now().Add(10 * time.Second)
	for api.Dropped() < n {
		check(time.Now().Before(deadline), what)
		runtime.GC()
		runtime.Gosched()
	}
}

// uncollected checks the GC drops nothing of Rust's while what it frees is reachable.
func uncollected(n uint32, what string) {
	for range 5 {
		runtime.GC()
		runtime.Gosched()
		time.Sleep(time.Millisecond)
	}
	check(api.Dropped() == n, what)
}

// closed checks that every call taking a closed object returns ErrClosed, however it was
// closed, and that a second Close does nothing.
func closed() {
	ctx := context.Background()
	panics := func(f func()) (panicked any) {
		defer func() { panicked = recover() }()
		f()
		return nil
	}
	isClosed := func(err error, want, what string) {
		check(errors.Is(err, rt.ErrClosed) && err.Error() == want, fmt.Sprintf("%s: %v", what, err))
	}
	// A method, a trait's, and a field's accessors.
	c := must(api.NewCounter("c", 1))
	check(c.Close() == nil && c.Close() == nil, "Close, then again, passes")
	_, err := c.Get()
	isClosed(err, "Counter is closed", "a method of a closed object")
	_, err = c.Clone()
	isClosed(err, "Counter is closed", "Clone of a closed object")
	check(c.GoString() == "Counter is closed", "GoString of a closed object is the error")
	tally := api.NewTally()
	tally.Close()
	_, err = api.NewTally().Equal(tally)
	isClosed(err, "Tally is closed", "Equal with a closed object")
	registry := must(api.NewRegistry(must(api.NewCounter("first", 1))))
	registry.Close()
	_, err = registry.FirstGet()
	isClosed(err, "Registry is closed", "a field's getter of a closed object")
	// A call that fails moves nothing in.
	second := must(api.NewCounter("second", 2))
	isClosed(registry.FirstSet(second), "Registry is closed", "a field's setter of a closed object")
	check(must(second.Get()) == 2, "a failed call leaves what it would move")

	// A record's method, whose receiver holds a value only Rust makes: Go's zero value of
	// one panics before Rust reads it.
	stamped := api.Stamped{At: api.StampFromMillis(2500), Count: 3}
	check(stamped.Total() == 5, "a record's method reads the value it holds")
	check(panics(func() { api.Stamped{Count: 3}.Total() }) == "a zero Stamp, which only Rust makes", "a zero value panics")

	// An argument, alone, or nested where the call holds it through its arena.
	_, err = api.BumpBoth(api.NewTally(), tally)
	isClosed(err, "Tally is closed", "a closed argument")
	open := must(api.NewCounter("open", 3))
	_, err = api.Worth(&[]*api.Counter{open, c}, nil)
	isClosed(err, "Counter is closed", "a closed object in a list")
	check(must(open.Get()) == 3, "a failed call leaves the rest of a list")

	// An async call.
	channel := api.NewChannel()
	channel.Close()
	_, err = channel.Recv(ctx)
	isClosed(err, "Channel is closed", "an async call of a closed object")

	// Moved by a call, plain or boxed, or closed once the callback it was lent to returned.
	taken := must(api.NewCounter("m", 0))
	must(taken.IntoLabel(nil))
	_, err = taken.Get()
	isClosed(err, "Counter is closed", "a method of a moved object")
	tail := must(api.NewNode(1, nil))
	must(api.NewNode(2, tail))
	_, err = tail.Sum()
	isClosed(err, "Node is closed", "a method of a boxed moved object")
	var kept *api.Counter
	must(must(api.NewRegistry(must(api.NewCounter("c", 0)))).Edit(func(counter *api.Counter) bool {
		kept = counter
		return false
	}))
	_, err = kept.Get()
	isClosed(err, "Counter is closed", "a method of a lent object kept past its callback")

	// Close racing calls on other goroutines, of a value Go counts and of one Rust checks.
	latch := api.LatchDefault()
	must(latch.Set(5))
	race(latch, func() (bool, error) {
		v, err := latch.Get()
		return err == nil && *v == 5, err
	}, []string{"Latch is closed"}, "Latch: a handle is already borrowed")
	before := api.Dropped()
	token := api.NewToken(5)
	race(token, func() (bool, error) {
		id, err := token.IdGet()
		return id == 5, err
	}, []string{"Token is closed", "api_Token_id_get: a handle's value was moved out"}, "api_Token_drop: a handle is already borrowed")
	check(api.Dropped() == before+1, "Close racing calls drops Rust's value once")
	fmt.Println("closed ok")
}

// race closes c while 8 goroutines call call on it: each call runs on the live value or,
// once c is closed, fails with one of closed, which for a value Rust checks includes its
// moved check, met by a call that held the handle before Close; a Close a running call
// beats fails with borrowed, as Rust refuses a drop under a borrow, and changes nothing, so
// the next tries again.
func race(c interface{ Close() error }, call func() (bool, error), closed []string, borrowed string) {
	var wg sync.WaitGroup
	var ran, failed atomic.Int64
	for range 8 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				live, err := call()
				if err != nil {
					check(errors.Is(err, rt.ErrClosed) && slices.Contains(closed, err.Error()), fmt.Sprintf("a call racing Close: %v", err))
					failed.Add(1)
					return
				}
				check(live, "a call racing Close reads the live value")
				ran.Add(1)
			}
		}()
	}
	for ran.Load() < 100 {
		runtime.Gosched()
	}
	for {
		err := c.Close()
		if err == nil {
			break
		}
		check(errors.Is(err, rt.ErrBorrowed) && err.Error() == borrowed, fmt.Sprintf("Close racing a call: %v", err))
	}
	wg.Wait()
	check(failed.Load() == 8, "every call after Close fails")
	check(c.Close() == nil, "Close again passes")
}

// bytes checks that Bytes Rust returns read Rust's buffer in place, kept alive by any view
// or reader of it and freed by the GC once none is reachable.
func bytes() {
	budgeted()
	// Under 512 B, copied and Rust's buffer freed at the call; from 512 B, lent.
	before := api.Dropped()
	small := api.TokenBytes(1, 511)
	check(api.Dropped() == before+1 && small.Len() == 511 && small.At(510) == 1, "bytes under 512 B are copied and Rust's freed")
	before++
	b := api.TokenBytes(1, 512)
	check(api.Dropped() == before, "returned bytes from 512 B keep Rust's buffer")
	want := strings.Repeat("\x01", 512)
	check(b.Len() == 512 && b.At(511) == 1 && string(b.Copy()) == want, "returned bytes read back")
	dst := make([]byte, 600)
	check(b.CopyTo(dst) == 512 && string(dst[:512]) == want, "CopyTo copies them all")
	sub := b.Slice(487, 512)
	check(sub.Len() == 25 && string(sub.Slice(5, 10).Copy()) == "\x01\x01\x01\x01\x01", "a sub-view reads its range")
	check(b.Slice(3, 3).Len() == 0, "an empty sub-view is empty")
	var out strings.Builder
	reader := b
	n, err := reader.WriteTo(&out)
	check(err == nil && n == 512 && out.String() == want && reader.Len() == 0, "WriteTo writes and consumes them")
	reader = b
	read, err := io.ReadAll(&reader)
	check(err == nil && string(read) == want && b.Len() == 512, "Read reads and consumes a copy of the view only")
	runtime.KeepAlive(b)
	collected(before+1, "the GC frees lent bytes once nothing reaches them")

	// Kept alive by a sub-view alone, then freed once nothing reaches it. 64 KiB are lent
	// in a batch of their own, so nothing else lent keeps them.
	view := subView(2)
	uncollected(before+1, "a sub-view keeps Rust's buffer")
	check(view.Len() == 10 && view.At(9) == 2, "a sub-view reads Rust's buffer after its parent is gone")
	view = rt.Bytes{}
	collected(before+2, "the GC frees bytes no view reaches")

	// Kept alive by a reader alone.
	r := bufferedReader(3)
	uncollected(before+2, "a reader keeps Rust's buffer")
	all, err := io.ReadAll(r)
	check(err == nil && len(all) == 64<<10 && all[64<<10-1] == 3, "a reader reads Rust's buffer after the Bytes are gone")
	r = nil
	collected(before+3, "the GC frees bytes no reader reaches")

	// Lent in batches, each freed once none of its Bytes is reachable.
	kept := batched()
	for range 5 {
		runtime.GC()
		runtime.Gosched()
	}
	check(api.Dropped() <= before+3+36, "the GC frees no batch whose Bytes are reachable")
	for i, k := range kept {
		check(k.Len() == 512 && k.At(511) == uint8(10*i), "kept bytes read Rust's buffer after their batch's others are gone")
	}
	kept = nil
	collected(before+43, "the GC frees every batch once none of its Bytes is reachable")

	// Bytes an object returns outlive its Close: no Bytes depends on an object.
	body := strings.Repeat("b", 2048)
	stamp := api.StampFromMillis(1)
	message := api.NewMessage(&stamp, rt.NewBytes([]byte(body)))
	payload := must(message.PayloadGet())
	message.Close()
	runtime.GC()
	check(string(payload.Copy()) == body, "bytes an object returned outlive its Close")
	fmt.Println("bytes ok")
}

// invalid checks that err is a checked conversion's or a refused value's error, matching
// rt.ErrInvalid.
func invalid(err error, what string) {
	check(errors.Is(err, rt.ErrInvalid), what)
}

// checked checks that each value Go's type holds and Rust's does not fails the call taking
// it, before it is made, and that each conversion to or from the runtime's types fails on a
// value the other has no room for: each with an error matching rt.ErrInvalid, never a panic.
func checked() {
	// Text that is not UTF-8, alone, optional, and inside a record, a list, and a receiver.
	_, err := api.SameString("\xff")
	invalid(err, "a string that is not UTF-8 fails the call")
	_, err = api.Greet("\xff", api.ShapeSquare)
	invalid(err, "a borrowed string that is not UTF-8 fails the call")
	_, err = api.BoxedCounter(ptr("\xff"))
	invalid(err, "an optional string that is not UTF-8 fails the call")
	_, err = api.HasText(api.Note{Text: ptr("a\xffb")})
	invalid(err, "a record's string that is not UTF-8 fails the call")
	_, err = api.Longest([]string{"a", "\xc3"})
	invalid(err, "a list's string that is not UTF-8 fails the call")
	_, err = api.Peer{Names: []string{"\xff"}}.Aliases()
	invalid(err, "a receiver's string that is not UTF-8 fails the call")
	check(must(api.SameString("é")) == "é" && must(api.SameString("")) == "", "UTF-8 text crosses whole")
	check(must(api.SamePath("\xff")) == "\xff", "a path is any bytes on Unix")
	// A rune that is a surrogate, past Unicode, or negative.
	for _, r := range []rune{0xd800, 0xdfff, unicode.MaxRune + 1, -1} {
		_, err = api.SameChar(r)
		invalid(err, fmt.Sprintf("the rune %d fails the call", r))
	}
	// A zero for a NonZero, optional and inside a record.
	noHop := uint64(0)
	invalid(second(api.NextHop(&noHop)), "a zero NonZero fails the call")
	invalid(second(api.FartherHop(api.PairNonZeroU16{Left: 0, Right: 1})), "a record's zero NonZero fails the call")

	// Durations: built exactly, and converted to and from time.Duration where it holds them.
	invalid(second(rt.NewDuration(math.MaxUint64, 1e9)), "a duration past Rust's longest fails")
	carried := must(rt.NewDuration(1, 2_500_000_000))
	check(carried.Secs() == 3 && carried.Nanos() == 500_000_000, "nanoseconds past a second carry")
	invalid(second(rt.DurationOf(-time.Nanosecond)), "a negative duration fails")
	check(must(must(rt.DurationOf(math.MaxInt64)).Std()) == math.MaxInt64, "time.Duration's longest crosses")
	longest := api.SameDuration(must(rt.NewDuration(math.MaxUint64, 999_999_999)))
	invalid(second(longest.Std()), "a duration past time.Duration's longest fails")
	extended := api.Extended(must(rt.NewDuration(math.MaxInt64, 0)), must(rt.NewDuration(1, 0)))
	check(extended != nil && extended.Secs() == 1<<63, "a duration past time.Duration's longest crosses")
	check(api.SameDuration(rt.Duration{}) == rt.Duration{}, "the zero duration is zero")

	// u128s: converted to and from big.Int within u128's range.
	invalid(second(rt.U128Of(big.NewInt(-1))), "a negative int is no u128")
	past := new(big.Int).Lsh(big.NewInt(1), 128)
	invalid(second(rt.U128Of(past)), "an int past u128 fails")
	most := new(big.Int).Sub(past, big.NewInt(1))
	check(must(rt.U128Of(most)) == rt.U128{Hi: math.MaxUint64, Lo: math.MaxUint64}, "u128's most converts")
	check(api.SameU128(must(rt.U128Of(most))).Big().Cmp(most) == 0, "a u128 converts back")

	// Addresses: each family converted from netip only where it fits, and back.
	invalid(second(rt.SocketAddrV4Of(netip.MustParseAddrPort("[::]:1"))), "an IPv6 address is no SocketAddrV4")
	invalid(second(rt.SocketAddrV6Of(netip.MustParseAddrPort("1.2.3.4:1"))), "an IPv4 address is no SocketAddrV6")
	invalid(second(rt.SocketAddrV6Of(netip.MustParseAddrPort("[fe80::1%no-such-interface]:1"))), "a zone naming no interface fails")
	invalid(second(rt.SocketAddrOf(netip.AddrPort{})), "an invalid address is no SocketAddr")
	invalid(second(rt.Ipv4AddrOf(netip.IPv6Unspecified())), "an IPv6 address is no Ipv4Addr")
	invalid(second(rt.Ipv6AddrOf(netip.MustParseAddr("1.2.3.4"))), "an IPv4 address is no Ipv6Addr")
	invalid(second(rt.Ipv6AddrOf(netip.MustParseAddr("fe80::1%3"))), "an Ipv6Addr has no zone")
	invalid(second(rt.IpAddrOf(netip.MustParseAddr("fe80::1%3"))), "an IpAddr has no zone")
	invalid(second(rt.IpAddrOf(netip.Addr{})), "an invalid address is no IpAddr")
	flowing := api.SameSocketAddrV6(rt.SocketAddrV6{FlowInfo: 1})
	invalid(second(flowing.Netip()), "a flow label has no netip form")
	invalid(second(rt.SocketAddrFromV6(flowing).Netip()), "a socket address's flow label has no netip form")
	scoped := api.SameSocketAddr(must(rt.SocketAddrOf(netip.MustParseAddrPort("[fe80::1%3]:443"))))
	check(must(scoped.Netip()) == netip.MustParseAddrPort("[fe80::1%3]:443"), "a scope crosses as its zone")
	mapped := netip.MustParseAddr("::ffff:1.2.3.4")
	check(api.SameIpAddr(must(rt.IpAddrOf(mapped))).Netip() == mapped, "an IPv4-mapped IPv6 address stays IPv6")
	zero, isV4 := api.SameSocketAddr(rt.SocketAddr{}).V4()
	check(isV4 && zero == rt.SocketAddrV4{}, "the zero socket address is IPv4's unspecified, port 0")
	zeroIp, isV4 := api.SameIpAddr(rt.IpAddr{}).V4()
	check(isV4 && zeroIp == rt.Ipv4Addr{}, "the zero IP address is IPv4's unspecified")
}

// second is a pair's second value, the error a conversion returns.
func second[T any](_ T, err error) error {
	return err
}

// budgeted checks that at most 64 MiB of Rust's bytes are lent at once, freed or not, since
// the GC sees none of them: past that, bytes are copied, Rust's freed at the call, until the
// GC frees what was lent. First, so no bytes an earlier step lent count against it.
func budgeted() {
	before := api.Dropped()
	lentToBudget(before)
	collected(before+65, "the GC frees bytes lent up to the budget")
	check(api.TokenBytes(3, 2048).At(2047) == 3 && api.Dropped() == before+65, "bytes are lent again once the GC frees them")
	collected(before+66, "the GC frees bytes lent once the budget is free")
}

// lentToBudget lends 64 MiB in a frame of its own, so nothing keeps them reachable after.
//
//go:noinline
func lentToBudget(before uint32) {
	const mib = 1 << 20
	held := make([]rt.Bytes, 64)
	for i := range held {
		held[i] = api.TokenBytes(1, mib)
	}
	check(api.Dropped() == before, "64 MiB of Rust's bytes are lent")
	copied := api.TokenBytes(2, mib)
	check(api.Dropped() == before+1, "bytes past 64 MiB lent are copied, Rust's freed at the call")
	check(copied.Len() == mib && copied.At(0) == 2 && copied.At(mib-1) == 2 && held[63].At(mib-1) == 1, "bytes past the budget read as lent ones do")
}

// subView is a sub-view of returned bytes, whose Bytes nothing else keeps.
//
//go:noinline
func subView(id uint32) rt.Bytes {
	return api.TokenBytes(id, 64<<10).Slice(64<<10-10, 64<<10)
}

// bufferedReader is a reader of returned bytes, whose Bytes nothing else keeps.
//
//go:noinline
func bufferedReader(id uint32) io.Reader {
	b := api.TokenBytes(id, 64<<10)
	return bufio.NewReader(&b)
}

// batched lends 40 Bytes, more than a batch holds, keeping only every tenth.
//
//go:noinline
func batched() []rt.Bytes {
	var kept []rt.Bytes
	for i := range 40 {
		b := api.TokenBytes(uint32(i), 512)
		if i%10 == 0 {
			kept = append(kept, b)
		}
	}
	return kept
}

// broken breaks a contract, which must fail with a message naming it.
func broken(which string) {
	switch which {
	// A failed check returns an error, which the case raises once it checked its kind.
	case "outlived":
		meters := api.NewMeters()
		first := must(meters.First())
		ok(meters.Clear())
		_, err := first.Count()
		check(errors.Is(err, rt.ErrClosed), "a handle that outlived its lender is closed")
		panic(err)
	case "lent moved":
		must(api.NewRegistry(must(api.NewCounter("c", 0)))).Edit(func(counter *api.Counter) bool {
			_, err := counter.IntoLabel(nil)
			check(errors.Is(err, rt.ErrBorrowed), "a lent handle moved out is borrowed")
			panic(err)
		})
	case "callback panicked":
		api.Emit(panicking{}, rt.NewBytes([]byte{1}))
	// The packages serve wasm32 too, whose items the host's library lacks.
	case "target":
		_, err := api.WasmOnly()
		check(errors.Is(err, errors.ErrUnsupported), "an item off the target is unsupported")
		panic(err)
	case "variant":
		api.PointerBits(api.PlatformWasm{Bits: 32})
	case "plain variant":
		api.IsFamily(api.FamilyWasm)
	case "nil reader":
		api.Tail(nil, 1)
	// A moment Go made, which Rust cannot read.
	case "returned zero":
		api.Drive(context.Background(), *api.NewMoment(0, 1000), func(api.Moment, *api.Waiter) (*api.Moment, error) {
			return &api.Moment{}, nil
		})
	default:
		check(false, "no case "+which)
	}
}

// cancelLog cancels its context at each line, and counts them.
type cancelLog struct {
	cancel context.CancelFunc
	lines  atomic.Int32
}

func (l *cancelLog) Log(uint8, string) bool {
	l.cancel()
	l.lines.Add(1)
	return true
}

// markedLog holds a marker, so its collection shows nothing keeps it.
type markedLog struct{ marker *[4]int }

func (l markedLog) Log(uint8, string) bool {
	return l.marker[0] == 0
}

type panicking struct{}

func (panicking) Log(uint8, string) bool {
	panic("a log failed")
}

func main() {
	switch {
	case len(os.Args) < 2:
		script()
	case os.Args[1] == "futures":
		futures()
	case os.Args[1] == "owned":
		owned()
	case os.Args[1] == "closed":
		closed()
	case os.Args[1] == "bytes":
		bytes()
	case os.Args[1] == "closures":
		closures()
	case os.Args[1] == "checks":
		checks()
	default:
		broken(os.Args[1])
	}
	unleaked()
}

// unleaked fails if a goroutine is blocked forever on what nothing else can reach, such
// as a future's wake channel a cancelled call dropped.
func unleaked() {
	leaks := pprof.Lookup("goroutineleak")
	var stacks strings.Builder
	// Writing it runs the collection that finds them.
	ok(leaks.WriteTo(&stacks, 1))
	if leaks.Count() != 0 {
		fmt.Fprint(os.Stderr, stacks.String())
		os.Exit(1)
	}
}
