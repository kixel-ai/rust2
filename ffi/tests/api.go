// The api fixture's Go test: the script fixtures/api/examples/transcript.rs runs natively,
// through the generated Go. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what closing and the GC
// free, what a closed object returns, how long returned bytes live, or breaks a contract,
// which must fail.
package main

import (
	"bufio"
	"context"
	"errors"
	"fmt"
	"io"
	"math"
	"net/netip"
	"os"
	"runtime"
	"strings"
	"sync"
	"sync/atomic"
	"time"

	"api/api"
	"api/dep"
	"api/rt"
	"api/serdejson"
	"api/url"
)

// printLog keeps even levels.
type printLog struct{}

// Close runs once Rust drops the implementation.
func (printLog) Close() error {
	fmt.Println("freed")
	return nil
}

func (printLog) Log(level uint8, line string) bool {
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

func quoted(s string) string {
	return fmt.Sprintf("%q", s)
}

func number[T any](n T) string {
	return fmt.Sprint(n)
}

func bound(b rt.BoundU64) string {
	switch b.Kind {
	case rt.BoundIncluded:
		return fmt.Sprintf("Included(%d)", b.Value)
	case rt.BoundExcluded:
		return fmt.Sprintf("Excluded(%d)", b.Value)
	}
	return "Unbounded"
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

func script() {
	ctx := context.Background()
	fmt.Printf("limit %d\n", api.LIMIT)

	p := api.Point{X: 1, Y: 2}.Offset(2, 3)
	fmt.Printf("offset %d %d\n", p.X, p.Y)
	fmt.Printf("debug %s\n", p.GoString())
	fmt.Printf("eq %t\n", p.Equal(api.Point{X: 3, Y: 5}))
	d := api.PointDefault()
	fmt.Printf("default %d %d\n", d.X, d.Y)

	c := api.NewCounter("c", 5)
	fmt.Printf("get %d\n", must(c.Get()))
	fmt.Printf("name %s\n", must(c.Name()))
	fmt.Printf("add ok %d\n", must(c.Add(10)))
	// An exported error type is returned as itself.
	var e api.Error
	_, err := c.Add(2000)
	check(errors.As(err, &e), "add returns an api.Error")
	fmt.Printf("add err %s %s\n", e.Error(), e.GoString())
	_, err = c.Sub(20)
	check(errors.As(err, &e), "sub returns an api.Error")
	fmt.Printf("sub err %s %s\n", err, e.GoString())
	ok(c.Rename("d"))
	fmt.Printf("name %s\n", must(c.Name()))
	fmt.Printf("is d %t\n", must(c.Is("d")))
	cp := must(c.Clone())
	must(cp.Add(1))
	fmt.Printf("clone %d original %d\n", must(cp.Get()), must(c.Get()))
	cp.Close()
	fmt.Printf("debug %s\n", c.GoString())
	label := must(c.IntoLabel(&p))
	fmt.Printf("label %s\n", label.GoString())
	fmt.Printf("note %t %t\n", api.HasText(api.Note{Text: ptr("")}), api.HasText(api.NoteDefault()))

	below, above, err := api.NewCounter("s", 7).Split(5)
	ok(err)
	fmt.Printf("split %s %d %s %d\n", must(below.Name()), must(below.Get()), must(above.Name()), must(above.Get()))
	one := must(api.Only(api.NewCounter("o", 3)))
	fmt.Printf("only %s %d\n", must(one.Name()), must(one.Get()))
	w8, w16, w32, w64 := api.Widths(98305)
	fmt.Printf("widths %d %d %d %d\n", w8, w16, w32, w64)
	for _, b := range []uint64{2, 0} {
		if q, r, err := api.Divide(7, b); err != nil {
			fmt.Printf("divide err %s\n", err)
		} else {
			fmt.Printf("divide %d %d\n", q, r)
		}
	}
	for _, n := range []uint64{98305, 0} {
		if s := api.Spread(n); s != nil {
			fmt.Printf("spread %d %d %d %d\n", s.V0, s.V1, s.V2, s.V3)
		} else {
			fmt.Println("spread none")
		}
	}
	for _, needle := range []string{"lo", "x"} {
		if found := api.Find([]byte("hello"), []byte(needle)); found != nil {
			fmt.Printf("find %d %d\n", found.V0, found.V1)
		} else {
			fmt.Println("find none")
		}
	}
	rows := api.Rows([]api.Point{{X: 1, Y: 2}, {X: 3, Y: 3}})
	shapes := map[api.Shape]string{api.ShapeCircle: "Circle", api.ShapeSquare: "Square"}
	fmt.Printf("rows %s\n", strings.Trim(list(rows, func(r rt.Tuple3[string, api.Shape, api.Point]) string {
		return fmt.Sprintf("%s %s %d %d", r.V0, shapes[r.V1], r.V2.X, r.V2.Y)
	}), "[]"))
	// Each handle in a tuple moves in, as one taken by value does.
	board := must(api.NewBoard([]rt.Tuple2[string, *api.Counter]{
		{V0: "low", V1: api.NewCounter("l", 1)},
		{V0: "high", V1: api.NewCounter("h", 9)},
	}))
	fmt.Printf("ranked %s\n", strings.Trim(list(must(board.Ranked()), func(r rt.Tuple2[string, *api.Counter]) string {
		return fmt.Sprintf("%s %s %d", r.V0, must(r.V1.Name()), must(r.V1.Get()))
	}), "[]"))
	board.Close()
	square := api.ShapeSquare
	span := api.Span{Bounds: rt.Tuple2[uint32, uint32]{V0: 2, V1: 10}, Named: &rt.Tuple2[string, api.Shape]{V0: "s", V1: square}}
	lower, upper := api.Halves(span)
	fmt.Printf("halves %d %d %d %d %s\n", lower.V0, lower.V1, upper.V0, upper.V1, span.GoString())
	n, text := api.Swapped(rt.Tuple2[string, uint32]{V0: "t", V1: 4})
	fmt.Printf("swapped %d %s\n", n, text)
	// A tuple of one is its element, so `Option<(Option<u32>,)>` nests two pointers.
	for _, value := range []**uint32{nil, ptr[*uint32](nil), ptr(ptr[uint32](3))} {
		switch doubled := api.Doubled(value); {
		case doubled == nil:
			fmt.Println("doubled None")
		case *doubled == nil:
			fmt.Println("doubled Some((None,))")
		default:
			fmt.Printf("doubled Some((Some(%d),))\n", **doubled)
		}
	}
	fmt.Printf("lent back %s boxed %s\n", list(api.LentBack(rt.NewBytes([]byte{1, 2})).Copy(), number), list(api.BoxedTuple([]byte{3}).Copy(), number))
	// A borrowed string and an owned one share a tuple struct.
	check(api.Title(false) == nil, "an informal title is none")
	fmt.Printf("title Some((%s,)) None named Some((%s,))\n", quoted(*api.Title(true)), quoted(*api.Named("x")))

	// Each lent counter is a new handle to a clone.
	registry := must(api.NewRegistry(api.NewCounter("first", 1)))
	fmt.Printf("last %t\n", must(registry.Last()) != nil)
	ok(registry.Push(api.NewCounter("last", 2)))
	first := must(registry.FirstGet())
	must(first.Add(10))
	fmt.Printf("first %d lent %d\n", must(must(registry.FirstGet()).Get()), must(first.Get()))
	last := must(registry.Last())
	must(last.Add(10))
	fmt.Printf("last %d lent %d\n", must(must(registry.Last()).Get()), must(last.Get()))
	// A closure lent for the call edits the first counter through a handle.
	by := uint64(5)
	edited := must(registry.Edit(func(counter *api.Counter) bool {
		must(counter.Add(by))
		return must(counter.Edited()) && must(counter.Get()) == 6
	}))
	fmt.Printf("edit %t %d\n", edited, must(must(registry.FirstGet()).Get()))
	// A setter moves an object in, as assignment does, closing its handle.
	ok(registry.FirstSet(api.NewCounter("second", 3)))
	fmt.Printf("set first %d %s\n", must(must(registry.FirstGet()).Get()), must(must(registry.FirstGet()).Name()))
	fmt.Printf("walk %d\n", api.Walk(3, func(cursor *api.Cursor) uint64 { return must(cursor.At()) * 2 }))
	fmt.Printf("each %d\n", api.Each(2, func(at uint32) { fmt.Printf("visit %d\n", at) }))
	fmt.Printf("offset %d\n", api.Offset(func() uint32 { return 7 }, 3))
	fmt.Printf("shift %d\n", api.Shift(func() uint32 { return 7 }, 4))

	name := api.NameOwned("a")
	joined := must(name.Join(api.NameOwned("b")))
	fmt.Printf("joined %s %s %t\n", must(joined.AsStr()), joined.GoString(), must(joined.Equal(name)))
	ok(joined.Rename("c"))
	fmt.Printf("renamed %s\n", must(joined.AsStr()))
	long := must(api.Longer(joined, api.NameOwned("dd")))
	fmt.Printf("longer %s\n", must(long.AsStr()))
	fmt.Printf("unnamed [%s]\n", must(api.Unnamed().AsStr()))

	fmt.Printf("total %d\n", api.Total(rt.RangeU64{Start: 2, End: 5}))
	wide := api.Widen(rt.RangeU64{Start: 2, End: 5}, 1)
	fmt.Printf("widen %d %d\n", wide.Start, wide.End)
	included := func(n uint64) rt.BoundU64 { return rt.BoundU64{Kind: rt.BoundIncluded, Value: n} }
	excluded := func(n uint64) rt.BoundU64 { return rt.BoundU64{Kind: rt.BoundExcluded, Value: n} }
	unbounded := rt.BoundU64{Kind: rt.BoundUnbounded}
	counts := list([]rt.BoundsU64{
		{Start: included(2), End: excluded(5)},
		{Start: included(2), End: included(5)},
		{Start: included(3), End: unbounded},
		{Start: unbounded, End: unbounded},
		{Start: excluded(2), End: included(4)},
	}, func(b rt.BoundsU64) string { return fmt.Sprint(api.Count(b)) })
	fmt.Printf("count %s\n", strings.ReplaceAll(strings.Trim(counts, "[]"), ",", ""))
	fmt.Printf("end %s %s\n", bound(api.End(3)), bound(api.End(0)))
	start, end := api.Ends(3)
	fmt.Printf("ends %s %s\n", bound(start), bound(end))
	for _, n := range []uint64{2, 5} {
		got, err := api.Below(n, 3)
		var bounds rt.BoundsU64Error
		switch {
		case err == nil:
			fmt.Printf("below %d\n", got)
		case errors.As(err, &bounds):
			fmt.Printf("below err %s %s\n", bound(bounds.Value.Start), bound(bounds.Value.End))
		default:
			check(false, "below returns its bounds")
		}
	}
	window := api.Window{Range: rt.RangeU64{Start: 2, End: 8}, End: api.End(5)}
	fmt.Printf("window %t %t %s\n", window.Contains(4), window.Contains(6), window.GoString())

	fmt.Println(api.Greet("x", api.ShapeSquare))
	fmt.Printf("len %d\n", api.Len([]byte("abc")))
	binary := []byte{0xff, 0, 0x61}
	fmt.Printf("hex %s\n", api.Hex(binary))
	fmt.Printf("timed hex %s\n", api.TimedHex(binary))
	fmt.Printf("sample hex %s\n", api.SampleHex(binary))
	fmt.Printf("queued hex %s\n", api.QueuedHex(binary))
	fmt.Printf("tick %s\n", api.TickName(api.Label{Name: "tick"}))
	fmt.Printf("rooted %s\n", api.RootedName(api.Label{Name: "rooted"}))
	fmt.Printf("stored %s\n", api.StoredText("stored"))
	fmt.Printf("layer %s\n", api.LayerName(api.Label{Name: "layer"}))
	for _, event := range []api.Event{api.EventMoved{V0: p}, api.EventNamed{Name: "n"}, api.EventDone{}} {
		fmt.Println(api.Describe(event))
	}

	// A `Copy` handle passed by value is copied, so it stays usable.
	stamp := api.StampFromMillis(2500)
	fmt.Printf("secs %d\n", must(stamp.Secs()))
	fmt.Printf("later %t\n", must(api.StampFromMillis(3000).Later(stamp)))
	fmt.Printf("secs %d\n", must(stamp.Secs()))
	message := must(api.NewMessage(stamp, []byte("body")))
	fmt.Printf("payload %s\n", must(message.PayloadGet()).Copy())
	fmt.Printf("stamp %s\n", must(message.StampGet()).GoString())
	fmt.Printf("sent %t\n", must(message.Sent()))
	// Setters assign in place: the bytes are copied in, and a `Copy` handle copied.
	reset := api.StampFromMillis(4000)
	ok(message.StampSet(reset))
	ok(message.PayloadSet([]byte("edited")))
	fmt.Printf("payload %s\n", must(message.PayloadGet()).Copy())
	fmt.Printf("stamp %s\n", must(message.StampGet()).GoString())
	fmt.Printf("secs %d\n", must(reset.Secs()))
	ok(message.StampSet(nil))
	fmt.Printf("stamp %t\n", must(message.StampGet()) != nil)

	// Rust frees the implementation once it drops it, releasing Go's.
	fmt.Printf("kept %d\n", api.Emit(printLog{}, []byte{1, 2, 3, 4}))
	logger := api.NewLogger(printLog{})
	fmt.Printf("write %t\n", must(logger.Write("hi")))
	logger.Close()
	fmt.Println("dropped")
	// Rust calls it on another thread, before the call returns.
	threshold := uint8(2)
	filtered := api.Filter([]byte{1, 2, 3, 4}, func(level uint8, line string) bool {
		fmt.Printf("filter %d %s\n", level, line)
		return level > threshold
	})
	fmt.Printf("filtered %d\n", filtered)

	channel := api.NewChannel()
	// The receive waits once its future pends, until the send wakes it.
	watching := watch(ctx)
	got := make(chan []byte)
	go func() { got <- must(channel.Recv(watching)).Copy() }()
	<-watching.waiting
	fmt.Println("recv pending true")
	channel.Send([]byte("hi"))
	payload := <-got
	wakes := 1
	fmt.Printf("woken %d\n", wakes)
	fmt.Printf("recv %s\n", payload)
	channel.Send(nil)
	if _, err := channel.RecvNonempty(ctx); err != nil {
		fmt.Printf("nonempty err %s\n", err)
	}
	// The send happens at the call: the future is cancelled before its first poll.
	cancelled, cancel := context.WithCancel(ctx)
	cancel()
	check(errors.Is(channel.SendLater(cancelled, []byte("eager")), context.Canceled), "a cancelled call returns its context's error")
	fmt.Printf("recv %s\n", must(channel.Recv(ctx)).Copy())
	channel.Send([]byte("sized"))
	sized, size, err := channel.RecvSized(ctx)
	check(err == nil, "recv sized")
	fmt.Printf("sized %s %d\n", sized.Copy(), size)
	// The waker moves into Rust, which keeps it until the send wakes it.
	channel.Notify(func() { wakes++ })
	channel.Send([]byte("rung"))
	fmt.Printf("woken %d\n", wakes)

	list1 := must(api.NewNode(1, nil))
	ok(list1.Push(2))
	fmt.Printf("node %d\n", must(list1.Sum()))

	tally := api.NewTally()
	must(tally.Bump())
	fmt.Printf("tally %d %t\n", must(tally.Bump()), must(tally.Equal(tally)))
	fmt.Printf("both %d\n", must(api.BumpBoth(tally, tally)))
	fmt.Printf("peek %d\n", must(api.Peek(tally, func(lent *api.Tally) uint32 { return must(lent.Bump()) })))

	area := must(api.Grow(dep.NewArea(dep.Size{Width: 2, Height: 3}), 1))
	areaSize := must(area.Size())
	fmt.Printf("area %d %d %d\n", areaSize.Width, areaSize.Height, must(area.Pixels()))

	// Generic types, each at the instances it infers, named by their arguments.
	pair := api.PairU32{Left: 2, Right: 5}
	swapped := pair.Swap()
	fmt.Printf("pair %d %d sum %d %s %t\n", swapped.Left, swapped.Right, pair.Sum(), pair.GoString(), pair.Equal(swapped))
	fmt.Printf("even %t %t\n", pair.Even(), api.PairU32{Left: 7, Right: 7}.Even())
	// An instance renamed alone, beside another of `Pair` that keeps its name.
	tagged := api.Tag(dep.Size{Width: 1, Height: 2}, 3)
	flipped := tagged.Swap()
	fmt.Printf("sized %d %d %s\n", flipped.Left, flipped.Right.Width, flipped.GoString())
	beside := api.PairU32{Left: 258, Right: 9}.BesideSize(tagged.Left)
	fmt.Printf("beside %d %s\n", beside.Left, beside.GoString())
	// At its default, named as the type itself.
	stack := api.NewStack()
	for i, n := range []string{"a", "b"} {
		ok(stack.Push(api.NewCounter(n, uint64(i)+1)))
	}
	top := must(must(stack.Pop()).Get())
	fmt.Printf("stack %d Some(%d) %t %t\n", must(stack.Depth()), top, must(stack.Pop()) != nil, must(stack.Pop()) == nil)
	// Generic functions and methods, at each implementation of the trait bounding them, by
	// reference, which leaves Go its handle.
	fromPoint := api.CounterSeededPoint("p", api.Point{X: -3, Y: 4})
	fromSize := api.CounterSeededSize("s", dep.Size{Width: 5, Height: 6})
	fromCounter := must(api.CounterSeededCounter("c", fromPoint))
	fromPolar := api.CounterSeededPolarPoint("q", api.PolarPoint{Turns: 1, Distance: 2})
	fmt.Printf("seeded %d %d %d %d past %d seeds %d\n", must(fromPoint.Get()), must(fromSize.Get()), must(fromCounter.Get()), must(fromPolar.Get()),
		must(stack.DepthPastPoint(api.Point{X: 1, Y: 2})), api.SeedsPointSize(dep.Size{Width: 2, Height: 3}))

	// A trait implemented for `Point` and `&Point` apart: one instance.
	fmt.Printf("weigh %d\n", api.WeighPoint(api.Point{X: -1, Y: 2}))
	// A function named as the type it returns, and one named as a type it does not. `area`
	// names a type of another crate's package, which leaves its name alone.
	polar := api.NewPolarPoint(2)
	check(polar != nil, "a point off the origin")
	check(api.Span_(1, 4) == 3, "a function named as a type of its package")
	fmt.Printf("polar %d %t area %d\n", polar.Distance, api.NewPolarPoint(0) == nil, api.Area(dep.Size{Width: 2, Height: 3}))
	// Two types whose paths join alike, the second kept apart where its path splits.
	fmt.Printf("wind %d\n", api.Wind(api.PolarTurn{Whole: 2}, api.Polar_Turn{Part: 3}))
	// Instances that name each other, and one made only by an impl of it.
	crossed := api.Cross(api.LeftRightU8{Value: api.RightU8{Value: 1}}, api.RightLeftU8{Value: api.LeftU8{Value: 2}})
	fmt.Printf("cross %d outer %d\n", crossed, must(api.NewOuterInnerU8(4).Get()))
	// A bound that outlives the call takes a value, and an instance only a constant names.
	fmt.Printf("kept %d boxed %d\n", api.KeptPoint(api.Point{X: 1, Y: -2}), api.BOXED().Value)
	fmt.Printf("keep %d\n", api.KeepPoint(api.Point{X: 1, Y: -2}))
	// Inferred instances taking a lent closure and a foreign implementation, and one only
	// its impl of a trait names.
	sides := api.PairU32{Left: 2, Right: 5}
	fmt.Printf("apply %d\n", api.ApplyU32(sides, func(side uint32) uint32 { return side * 3 }))
	fmt.Printf("report %d\n", api.ReportU32(sides, printLog{}))
	fmt.Printf("wrapped %d\n", api.KeptWrapU32(api.WrapU32{Value: 4}))
	fmt.Printf("keeps %d\n", api.Keeps(printLog{}).Value)

	// Its Drop runs when Go closes the handle, not before.
	token := api.NewToken(7)
	fmt.Printf("token %d dropped %d\n", must(token.IdGet()), api.Dropped())
	token.Close()
	fmt.Printf("dropped %d\n", api.Dropped())

	// A field and the method of its name, each its own member.
	retry := api.NewRetry(ptr[uint32](5))
	limit := must(retry.LimitGet())
	isSome := 0
	if limit != nil {
		isSome = 1
	}
	fmt.Printf("retry %d %d %d %d\n", isSome, *limit, must(retry.Limit()), must(retry.Delay()))
	// The renamed field's setter, named as its getter.
	ok(retry.LimitSet(nil))
	fmt.Printf("retry %d\n", must(retry.Limit()))
	fmt.Printf("slot %d\n", must(api.NewSlotU8(200).Value()))

	// Foreign types: a URL parsed from text, socket addresses, OS paths, and slices.
	base := must(url.UrlParse("https://example.com/a/b"))
	fmt.Printf("url %s host Some(%s)\n", base, quoted(*must(api.Host(base))))
	resolved := must(api.Resolve(must(base.Clone()), "c?d"))
	fmt.Printf("resolve %s %t\n", resolved, must(resolved.Equal(base)))
	resolved.Close()
	var parse rt.StringError
	_, err = url.UrlParse("no scheme")
	check(errors.As(err, &parse), "parse returns its message")
	fmt.Printf("parse err %s\n", parse.Value)
	v4 := netip.MustParseAddrPort("127.0.0.1:8080")
	v6 := netip.MustParseAddrPort("[::1%3]:443")
	fmt.Printf("ports %s\n", list(api.Ports([]netip.AddrPort{v4, v6}), number))
	peer := api.Peer{Addr: v6, Names: []string{"a", "b"}}.Next("c")
	fmt.Printf("peer %s %s\n", peer.GoString(), list(peer.Aliases(), quoted))
	file := api.File("dir", "f\\'\"\t")
	fmt.Printf("file %s\n", escaped(file))
	fmt.Printf("parents %s\n", strings.Trim(strings.ReplaceAll(list(api.Parents([]string{file, "/"}), func(p *string) string {
		if p == nil {
			return "none"
		}
		return escaped(*p)
	}), ",", ""), "[]"))
	// A path that is no UTF-8 is a Go string all the same.
	check(api.Stray() == "\xff", "a path that is not UTF-8 crosses as its bytes")
	var lent string
	api.VisitStray(func(path string) { lent = path })
	check(lent == "\xff", "a path Rust lends that is not UTF-8 crosses as its bytes")
	// An error Rust returns only boxed is its type.
	_, err = api.Refuse("why")
	var refusal *api.Refusal
	check(errors.As(err, &refusal) && err.Error() == "refused: why", "a boxed error is its type")
	words := api.Words(" one two  three ")
	fmt.Printf("words %d %s\n", len(words), list(words, quoted))
	fmt.Printf("sentence %s %s\n", api.Sentence(&words), api.Sentence(nil))
	fmt.Printf("sentence %s|\n", api.Sentence(&[]string{}))
	fmt.Printf("chunks %s\n", list(api.Chunks([]byte("abcde"), 2), func(b rt.Bytes) string { return list(b.Copy(), number) }))
	counters := api.Counters([]string{"x", "y"})
	fmt.Printf("counters %s\n", strings.Trim(list(counters, func(c *api.Counter) string {
		return fmt.Sprintf("%s %d", must(c.Name()), must(c.Get()))
	}), "[]"))
	// Each counter moves in, so Go closes it.
	fmt.Printf("worth %d %d\n", must(api.Worth(&[]*api.Counter{counters[0]}, []*api.Counter{counters[1]})), must(api.Worth(nil, nil)))
	held := api.NewCounter("h", 4)
	fmt.Print("held")
	for _, n := range []*uint64{must(api.HeldCount(&held)), must(api.HeldCount(nil))} {
		if n == nil {
			fmt.Print(" None")
		} else {
			fmt.Printf(" Some(%d)", *n)
		}
	}
	fmt.Println()
	b := "b"
	unnamed := "None"
	if s := must(api.BoxedName(nil)); s != nil {
		unnamed = *s
	}
	fmt.Printf("boxed Some(%q) %s %t\n", *must(api.BoxedName(api.BoxedCounter(&b))), unnamed, api.BoxedCounter(nil) == nil)
	fmt.Printf("less %d\n", api.Less(5, 2))
	fmt.Printf("sums %s\n", list(api.Sums([][]uint32{{1, 2}, {}, {3}}), number))
	sum := api.Summed([]api.Point{{X: 1, Y: 2}, {X: -4, Y: 6}})
	none := api.Summed(nil)
	fmt.Printf("summed %d %d %d %d\n", sum.X, sum.Y, none.X, none.Y)
	fmt.Printf("longest %d %d\n", api.Longest([]string{"a", "abc"}), api.Longest(nil))
	doc := must(serdejson.JsonParse(`{"b": [1, 2.5, null], "a": "é"}`))
	fmt.Printf("json %s\n", must(api.Encode(doc)))
	fmt.Printf("timed text %s\n", must(api.TimedText(doc)))
	tagged2 := must(api.Tagged(doc, "t"))
	fmt.Printf("tagged %s\n", must(tagged2.ToText()))
	tagged2.Close()
	_, err = serdejson.JsonParse(`{"a":`)
	check(errors.As(err, &parse), "parse returns its message")
	fmt.Printf("json parse err %s\n", err)
	site := must(api.NewSite(base))
	fmt.Printf("site %s %s\n", must(site.HomeGet()), must(site.HomeGet()))

	// Slices, passed in and returned as clones, and the socket address of each family.
	fmt.Printf("shapes %s greetings %s\n", list(api.ShapeALL(), func(s api.Shape) string { return shapes[s] }), list(api.GREETINGS(), quoted))
	trail := api.NewTrail([]api.Point{{X: 1, Y: 2}, {X: 3, Y: 4}}, []string{"a", "b"})
	point := func(p api.Point) string { return p.GoString() }
	labeled := func(l api.Label) string { return l.GoString() }
	fmt.Printf("trail %s %s\n", list(must(trail.Points()), point), list(must(trail.Labels()), labeled))
	// Returned iterators, each collected at the call: drained labels are gone after.
	fmt.Printf("iterated %s %s %s\n", list(must(trail.Iter()), point), list(must(trail.Names()), quoted), list(must(trail.Bytes()).Copy(), number))
	fmt.Printf("drained %s then %s\n", list(must(trail.Drain()), labeled), list(must(trail.Labels()), labeled))
	trail.Close()
	fmt.Printf("numbered %s\n", strings.Trim(list(api.Numbered([]string{"p", "q"}), func(c *api.Counter) string {
		return fmt.Sprintf("%s %d", must(c.Name()), must(c.Get()))
	}), "[]"))
	next4, next6 := api.Nexts(v4, v6)
	fmt.Printf("nexts %s %d %d %s\n", next4.Addr(), next4.Port(), next6.Port(), next6.Addr().Zone())
}

// futures checks the futures' semantics beyond the transcript, each by an assertion.
func futures() {
	ctx := context.Background()
	channel := api.NewChannel()

	// The send lands from another goroutine's thread while the receive waits.
	go channel.Send([]byte("blocked"))
	check(string(must(channel.Recv(ctx)).Copy()) == "blocked", "recv returns the payload another goroutine sent")

	// A wake that lands while the future polls, before it returns pending, is kept: the
	// call polls again rather than waiting for a wake that already happened.
	check(must(api.Yielded(ctx, 7)) == 7, "a wake during the poll polls again")

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
		channel.Send([]byte("kept"))
		cancel()
		close(gate)
		check(errors.Is(<-done, context.Canceled), "a receive cancelled after its wake returns its context's error")
		check(string(must(channel.Recv(ctx)).Copy()) == "kept", "a cancelled receive loses nothing")
	}

	// A context done before the call takes nothing, even a payload ready at the first poll.
	{
		channel.Send([]byte("ready"))
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

	// A handle closed while a call waits on it stays alive until the call ends.
	{
		other := api.NewChannel()
		watching := watch(ctx)
		got := make(chan string)
		go func() { got <- string(must(other.Recv(watching)).Copy()) }()
		<-watching.waiting
		sender := must(other.Clone())
		other.Close()
		sender.Send([]byte("held"))
		check(<-got == "held", "a closed handle stays alive in the call waiting on it")
		sender.Close()
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
	fmt.Println("owned ok")
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
	isClosed := func(err error, name, what string) {
		check(errors.Is(err, rt.ErrClosed) && err.Error() == name+" is closed", what)
	}

	// A method, a trait's, and a field's accessors.
	c := api.NewCounter("c", 1)
	c.Close()
	c.Close()
	_, err := c.Get()
	isClosed(err, "Counter", "a method of a closed object")
	_, err = c.Clone()
	isClosed(err, "Counter", "Clone of a closed object")
	check(c.GoString() == "Counter is closed", "GoString of a closed object is the error")
	tally := api.NewTally()
	tally.Close()
	_, err = api.NewTally().Equal(tally)
	isClosed(err, "Tally", "Equal with a closed object")
	registry := must(api.NewRegistry(api.NewCounter("first", 1)))
	registry.Close()
	_, err = registry.FirstGet()
	isClosed(err, "Registry", "a field's getter of a closed object")
	// A call that fails moves nothing in.
	second := api.NewCounter("second", 2)
	isClosed(registry.FirstSet(second), "Registry", "a field's setter of a closed object")
	check(must(second.Get()) == 2, "a failed call leaves what it would move")

	// A record's method, whose receiver holds an object.
	stamp := api.StampFromMillis(2500)
	stamped := api.Stamped{At: stamp, Count: 3}
	check(must(stamped.Total()) == 5, "a record's method reads the object it holds")
	stamp.Close()
	_, err = stamped.Total()
	isClosed(err, "Stamp", "a record's method whose object is closed")

	// An argument, alone, or nested where the call holds it through its arena.
	_, err = api.BumpBoth(api.NewTally(), tally)
	isClosed(err, "Tally", "a closed argument")
	open := api.NewCounter("open", 3)
	_, err = api.Worth(&[]*api.Counter{open, c}, nil)
	isClosed(err, "Counter", "a closed object in a list")
	check(must(open.Get()) == 3, "a failed call leaves the rest of a list")

	// An async call.
	channel := api.NewChannel()
	channel.Close()
	_, err = channel.Recv(ctx)
	isClosed(err, "Channel", "an async call of a closed object")

	// Closed by a call that moved it, plain or boxed, or once the callback it was lent to
	// returned.
	moved := api.NewCounter("m", 0)
	must(moved.IntoLabel(nil))
	_, err = moved.Get()
	isClosed(err, "Counter", "a method of a moved object")
	tail := must(api.NewNode(1, nil))
	must(api.NewNode(2, tail))
	_, err = tail.Sum()
	isClosed(err, "Node", "a method of a boxed moved object")
	var kept *api.Counter
	must(must(api.NewRegistry(api.NewCounter("c", 0))).Edit(func(counter *api.Counter) bool {
		kept = counter
		return false
	}))
	_, err = kept.Get()
	isClosed(err, "Counter", "a method of a lent object kept past its callback")

	// Close racing calls on other goroutines: each call either runs, holding the handle
	// so Rust's value outlives it, or returns ErrClosed; Rust's is dropped once.
	before := api.Dropped()
	token := api.NewToken(5)
	var wg sync.WaitGroup
	var ran, failed atomic.Int64
	for range 8 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				id, err := token.IdGet()
				if err != nil {
					isClosed(err, "Token", "a call racing Close")
					failed.Add(1)
					return
				}
				check(id == 5, "a call racing Close reads the live value")
				ran.Add(1)
			}
		}()
	}
	for ran.Load() < 100 {
		runtime.Gosched()
	}
	token.Close()
	wg.Wait()
	check(failed.Load() == 8 && api.Dropped() == before+1, "Close racing calls drops Rust's value once")
	fmt.Println("closed ok")
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
	message := must(api.NewMessage(api.StampFromMillis(1), []byte(body)))
	payload := must(message.PayloadGet())
	message.Close()
	runtime.GC()
	check(string(payload.Copy()) == body, "bytes an object returned outlive its Close")
	fmt.Println("bytes ok")
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
	case "lent moved":
		must(api.NewRegistry(api.NewCounter("c", 0))).Edit(func(counter *api.Counter) bool {
			counter.IntoLabel(nil)
			return false
		})
	// An IPv6 address where Rust takes `SocketAddrV4`.
	case "family":
		any := netip.MustParseAddrPort("[::]:1")
		api.Nexts(any, any)
	// No Rust string holds text that is not UTF-8.
	case "text":
		api.Greet("\xff", api.ShapeSquare)
	case "callback panicked":
		api.Emit(panicking{}, []byte{1})
	// A Rust Duration longer than Go's longest.
	case "duration":
		api.Extended(math.MaxInt64, 1)
	default:
		check(false, "no case "+which)
	}
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
	default:
		broken(os.Args[1])
	}
}
