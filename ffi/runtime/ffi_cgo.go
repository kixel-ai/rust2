// What each generated package converts its C types with, which cgo makes its own, after
// the generated code names the export's C types as cBuf, cWaker, cForeignFuture,
// cForeignReader, cU128, cDuration, and each address's, its buffer's free and copy as cBufFree and
// cBufCopy, and the runtime's callbacks into Go as goCall, goRelease, goPoll, goRead, and
// goSeek: the arena a call lowers nested values into, values across the C ABI, and what a
// future waits on.

// emptyBuf_ is where a present but empty buffer or array points: C reads a null pointer as
// None. Aligned for any element.
var emptyBuf_ [2]uint64

func emptyPtr_() unsafe.Pointer {
	return unsafe.Pointer(&emptyBuf_)
}

// bufString_ is text C reads during the call, without a copy.
func bufString_(s string) cBuf {
	if len(s) == 0 {
		return cBuf{ptr: (*C.uint8_t)(emptyPtr_())}
	}
	return cBuf{ptr: (*C.uint8_t)(unsafe.Pointer(unsafe.StringData(s))), len: C.size_t(len(s))}
}

// bufBytes_ is bytes C reads during the call, without a copy.
func bufBytes_(b []byte) cBuf {
	if len(b) == 0 {
		return cBuf{ptr: (*C.uint8_t)(emptyPtr_())}
	}
	return cBuf{ptr: (*C.uint8_t)(unsafe.Pointer(unsafe.SliceData(b))), len: C.size_t(len(b))}
}

// bufTake_ is b as a buffer Rust takes: Rust's own shared without a copy, or Go's memory,
// which C reads during the call and Rust copies.
func bufTake_(b rt.Bytes) cBuf {
	ptr, n, owner := rt.BytesTake_(b)
	if n == 0 {
		return cBuf{ptr: (*C.uint8_t)(emptyPtr_())}
	}
	return cBuf{ptr: (*C.uint8_t)(ptr), len: C.size_t(n), owner: owner}
}

func bufTakeOption_(b *rt.Bytes) cBuf {
	if b == nil {
		return cBuf{}
	}
	return bufTake_(*b)
}

func bufStringOption_(s *string) cBuf {
	if s == nil {
		return cBuf{}
	}
	return bufString_(*s)
}

func bufBytesOption_(b *[]byte) cBuf {
	if b == nil {
		return cBuf{}
	}
	return bufBytes_(*b)
}

// view_ is the bytes buf points at, which C keeps.
func view_(buf cBuf) []byte {
	if buf.len == 0 {
		return nil
	}
	return unsafe.Slice((*byte)(unsafe.Pointer(buf.ptr)), int(buf.len))
}

// liftString_ is text Rust returned, copied, then freed. A path, too: a Go string holds
// any bytes.
func liftString_(buf cBuf) string {
	s := string(view_(buf))
	cBufFree(buf)
	return s
}

func liftStringOption_(buf cBuf) *string {
	if buf.ptr == nil {
		return nil
	}
	s := liftString_(buf)
	return &s
}

// liftBytes_ is bytes Rust returned, as the runtime keeps them.
func liftBytes_(buf cBuf) rt.Bytes {
	return rt.LiftBytes_(unsafe.Pointer(&buf))
}

func liftBytesOption_(buf cBuf) *rt.Bytes {
	if buf.ptr == nil {
		return nil
	}
	b := liftBytes_(buf)
	return &b
}

// lentString_ is text C lends a callback for its call, copied.
func lentString_(buf cBuf) string {
	return string(view_(buf))
}

func lentStringOption_(buf cBuf) *string {
	if buf.ptr == nil {
		return nil
	}
	s := lentString_(buf)
	return &s
}

// lentBytes_ is bytes C lends a callback for its call, copied.
func lentBytes_(buf cBuf) []byte {
	return append([]byte{}, view_(buf)...)
}

func lentBytesOption_(buf cBuf) *[]byte {
	if buf.ptr == nil {
		return nil
	}
	b := lentBytes_(buf)
	return &b
}

func lowerU128_(v rt.U128) cU128 {
	return cU128{lo: C.uint64_t(v.Lo), hi: C.uint64_t(v.Hi)}
}

func liftU128_(c cU128) rt.U128 {
	return rt.U128{Hi: uint64(c.hi), Lo: uint64(c.lo)}
}

func lowerDuration_(d rt.Duration) cDuration {
	return cDuration{secs: C.uint64_t(d.Secs()), nanos: C.uint32_t(d.Nanos())}
}

// liftDuration_ is a Duration Rust returned, whose nanoseconds are below a second, so it
// never fails.
func liftDuration_(c cDuration) rt.Duration {
	d, _ := rt.NewDuration(uint64(c.secs), uint32(c.nanos))
	return d
}

// The addresses, whose C structs hold their octets as Go's arrays do.

func lowerIpv4Addr_(ip rt.Ipv4Addr) cIpv4Addr {
	return *(*cIpv4Addr)(unsafe.Pointer(&ip))
}

func liftIpv4Addr_(c cIpv4Addr) rt.Ipv4Addr {
	return *(*rt.Ipv4Addr)(unsafe.Pointer(&c))
}

func lowerIpv6Addr_(ip rt.Ipv6Addr) cIpv6Addr {
	return *(*cIpv6Addr)(unsafe.Pointer(&ip))
}

func liftIpv6Addr_(c cIpv6Addr) rt.Ipv6Addr {
	return *(*rt.Ipv6Addr)(unsafe.Pointer(&c))
}

func lowerIpAddr_(a rt.IpAddr) cIpAddr {
	var c cIpAddr
	if v6, ok := a.V6(); ok {
		c.v6 = true
		*(*cIpv6Addr)(unsafe.Pointer(&c.ip)) = lowerIpv6Addr_(v6)
	} else {
		v4, _ := a.V4()
		*(*cIpv4Addr)(unsafe.Pointer(&c.ip)) = lowerIpv4Addr_(v4)
	}
	return c
}

func liftIpAddr_(c cIpAddr) rt.IpAddr {
	if c.v6 {
		return rt.IpAddrFromV6(liftIpv6Addr_(*(*cIpv6Addr)(unsafe.Pointer(&c.ip))))
	}
	return rt.IpAddrFromV4(liftIpv4Addr_(*(*cIpv4Addr)(unsafe.Pointer(&c.ip))))
}

func lowerSocketAddrV4_(a rt.SocketAddrV4) cSocketAddrV4 {
	return cSocketAddrV4{ip: lowerIpv4Addr_(a.IP), port: C.uint16_t(a.Port)}
}

func liftSocketAddrV4_(c cSocketAddrV4) rt.SocketAddrV4 {
	return rt.SocketAddrV4{IP: liftIpv4Addr_(c.ip), Port: uint16(c.port)}
}

func lowerSocketAddrV6_(a rt.SocketAddrV6) cSocketAddrV6 {
	return cSocketAddrV6{
		ip:       lowerIpv6Addr_(a.IP),
		port:     C.uint16_t(a.Port),
		flowinfo: C.uint32_t(a.FlowInfo),
		scope_id: C.uint32_t(a.ScopeID),
	}
}

func liftSocketAddrV6_(c cSocketAddrV6) rt.SocketAddrV6 {
	return rt.SocketAddrV6{
		IP:       liftIpv6Addr_(c.ip),
		Port:     uint16(c.port),
		FlowInfo: uint32(c.flowinfo),
		ScopeID:  uint32(c.scope_id),
	}
}

func lowerSocketAddr_(a rt.SocketAddr) cSocketAddr {
	var c cSocketAddr
	if v6, ok := a.V6(); ok {
		c.v6 = true
		*(*cSocketAddrV6)(unsafe.Pointer(&c.addr)) = lowerSocketAddrV6_(v6)
	} else {
		v4, _ := a.V4()
		*(*cSocketAddrV4)(unsafe.Pointer(&c.addr)) = lowerSocketAddrV4_(v4)
	}
	return c
}

func liftSocketAddr_(c cSocketAddr) rt.SocketAddr {
	if c.v6 {
		return rt.SocketAddrFromV6(liftSocketAddrV6_(*(*cSocketAddrV6)(unsafe.Pointer(&c.addr))))
	}
	return rt.SocketAddrFromV4(liftSocketAddrV4_(*(*cSocketAddrV4)(unsafe.Pointer(&c.addr))))
}

// What a call returns before it is made once it is given text that is not UTF-8, a rune
// that is not a Unicode scalar value, or a zero for a NonZero, which Rust's types lack.
var (
	errText_ = rt.Invalid_("a string is not UTF-8")
	errRune_ = rt.Invalid_("a rune is not a Unicode scalar value")
	errPath_ = rt.Invalid_("a path is not WTF-8, as a Windows path is")
	errNonZero_ = rt.Invalid_("a NonZero is zero")
)

// checkPath_ is the error a call given the path s returns, or nil for any on a platform
// whose paths are bytes, or on Windows, WTF-8: UTF-8 but for a surrogate alone, never a
// lead and a trail apart, which WTF-8 writes as the scalar they make.
func checkPath_(s string) error {
	if runtime.GOOS != "windows" || utf8.ValidString(s) {
		return nil
	}
	lead := false
	for i := 0; i < len(s); {
		if r, n := utf8.DecodeRuneInString(s[i:]); r != utf8.RuneError || n != 1 {
			i, lead = i+n, false
			continue
		}
		if i+3 > len(s) || s[i] != 0xED || s[i+1] < 0xA0 || s[i+1] > 0xBF || s[i+2] < 0x80 || s[i+2] > 0xBF {
			return errPath_
		}
		trail := s[i+1] >= 0xB0
		if trail && lead {
			return errPath_
		}
		i, lead = i+3, !trail
	}
	return nil
}

// checkString_ is the error a call given s returns, or nil for UTF-8.
func checkString_(s string) error {
	if !utf8.ValidString(s) {
		return errText_
	}
	return nil
}

func checkPathOption_(s *string) error {
	if s == nil {
		return nil
	}
	return checkPath_(*s)
}

func checkStringOption_(s *string) error {
	if s == nil {
		return nil
	}
	return checkString_(*s)
}

// checkRune_ is the error a call given r returns, or nil for a Unicode scalar value.
func checkRune_(r rune) error {
	if !utf8.ValidRune(r) {
		return errRune_
	}
	return nil
}

// liftList_ is the array Rust returned, each item lifted by lift, which takes what it
// holds.
func liftList_[CT, T any](ptr *CT, n C.size_t, lift func(CT) T) []T {
	out := make([]T, int(n))
	if n == 0 {
		return out
	}
	for i, c := range unsafe.Slice(ptr, int(n)) {
		out[i] = lift(c)
	}
	return out
}

// unhold_ releases the hold a call took on h, in s, the handle of the object o. Deferred, it
// keeps o reachable until the call returns, so the GC cannot close h between the call
// reading and holding it.
func unhold_(h *rt.Handle_, s rt.Slot_, o any) {
	// Release, with the count's half inlined.
	if s.Counted() {
		h.Uncount()
	} else {
		h.Unlist(s)
	}
	runtime.KeepAlive(o)
}

// held_ is a handle a call holds, and the object owning it, kept reachable.
type held_ struct {
	h *rt.Handle_
	s rt.Slot_
	o any
}

// arena_ is what a call lowers nested values into, kept until it returns: the Go memory C
// reads through it, pinned, and the objects whose handles it holds, kept reachable.
type arena_ struct {
	pins runtime.Pinner
	held []held_
	// The owners of Rust's buffers C reads.
	kept []any
	// Whether a buffer lowered now is passed by value, which Rust takes: set once the
	// receiver, which Rust only reads, is lowered, and unset inside an array.
	take bool
	// Second owners of Rust's buffers lowered for Rust to take, freed here unless the
	// call was made.
	taken []cBuf
	made  bool
	// What first failed the call before it is made: an object it takes that was closed, or
	// a value it takes that Rust's type does not hold.
	err error
}

func (a *arena_) finish() {
	if !a.made {
		for _, buf := range a.taken {
			cBufFree(buf)
		}
	}
	for _, held := range a.held {
		held.h.Release(held.s)
	}
	a.pins.Unpin()
}

func (a *arena_) pin(p unsafe.Pointer) {
	if p != emptyPtr_() {
		a.pins.Pin(p)
	}
}

// bufString is s, which C reads during the call, pinned, failing the call unless UTF-8.
func (a *arena_) bufString(s string) cBuf {
	if err := checkString_(s); err != nil {
		a.fail(err)
	}
	return a.bufPath(s)
}

// bufPath is a path's bytes, which C reads during the call, pinned: any a string holds,
// but on Windows only WTF-8, failing the call otherwise.
func (a *arena_) bufPath(s string) cBuf {
	if err := checkPath_(s); err != nil {
		a.fail(err)
	}
	b := bufString_(s)
	a.pin(unsafe.Pointer(b.ptr))
	return b
}

// char is r, failing the call unless a Unicode scalar value.
func (a *arena_) char(r rune) C.uint32_t {
	if err := checkRune_(r); err != nil {
		a.fail(err)
	}
	return C.uint32_t(r)
}

// nonZero_ is v, a NonZero's integer, failing the call if it is zero.
func nonZero_[T comparable](a *arena_, v T) T {
	var zero T
	if v == zero {
		a.fail(errNonZero_)
	}
	return v
}

// fail fails the call with err before it is made, unless it already failed.
func (a *arena_) fail(err error) {
	if a.err == nil {
		a.err = err
	}
}

// bufView is the buffer of b, which C reads during the call: Go's memory pinned, and
// Rust's kept reachable, or where a.take, a second owner of it, which Rust takes.
func (a *arena_) bufView(b rt.Bytes) cBuf {
	if a.take && b.Len() != 0 {
		if ptr, n, owner := rt.BytesTake_(b); owner != nil {
			buf := cBuf{ptr: (*C.uint8_t)(ptr), len: C.size_t(n), owner: owner}
			a.taken = append(a.taken, buf)
			return buf
		}
	}
	ptr, n, owner := rt.BytesBuf_(b)
	if n == 0 {
		return cBuf{ptr: (*C.uint8_t)(emptyPtr_())}
	}
	if owner == nil {
		a.pins.Pin(ptr)
	} else {
		a.kept = append(a.kept, owner)
	}
	return cBuf{ptr: (*C.uint8_t)(ptr), len: C.size_t(n)}
}

func (a *arena_) bufStringOption(s *string) cBuf {
	if s == nil {
		return cBuf{}
	}
	return a.bufString(*s)
}

func (a *arena_) bufPathOption(s *string) cBuf {
	if s == nil {
		return cBuf{}
	}
	return a.bufPath(*s)
}

func (a *arena_) bufViewOption(b *rt.Bytes) cBuf {
	if b == nil {
		return cBuf{}
	}
	return a.bufView(*b)
}

// hold is h, the handle of the object o, held until the call returns, or nil, failing the
// call, once closed.
func (a *arena_) hold(h *rt.Handle_, o any) unsafe.Pointer {
	raw, s := h.Hold()
	if raw == nil {
		a.fail(h.Closed())
		return nil
	}
	a.held = append(a.held, held_{h, s, o})
	return raw
}

// block_ is items, lent in place to C as its array of CT, which T is laid out as: pinned
// until the call returns, and never null, so an empty one is told apart from none.
func block_[T, CT any](a *arena_, items []T) (*CT, C.size_t) {
	if len(items) == 0 {
		return (*CT)(emptyPtr_()), 0
	}
	a.pins.Pin(&items[0])
	return (*CT)(unsafe.Pointer(&items[0])), C.size_t(len(items))
}

// liftBlock_ is a copy of the n numbers at ptr, which T is laid out as.
func liftBlock_[T, CT any](ptr *CT, n C.size_t) []T {
	out := make([]T, int(n))
	if n != 0 {
		copy(out, unsafe.Slice((*T)(unsafe.Pointer(ptr)), int(n)))
	}
	return out
}

// list_ is items as a C array, each lowered by lower, pinned until the call returns:
// never null, so an empty one is told apart from a missing one.
func list_[T, CT any](a *arena_, items []T, lower func(T) CT) (*CT, C.size_t) {
	if len(items) == 0 {
		return (*CT)(emptyPtr_()), 0
	}
	out := make([]CT, len(items))
	// Rust only reads an array's items.
	take := a.take
	a.take = false
	for i, item := range items {
		out[i] = lower(item)
	}
	a.take = take
	a.pins.Pin(&out[0])
	return &out[0], C.size_t(len(items))
}

// give_ is v, kept for Rust in C memory until goRelease releases it.
func give_(v any) unsafe.Pointer {
	p := C.malloc(C.size_t(unsafe.Sizeof(cgo.Handle(0))))
	*(*cgo.Handle)(p) = cgo.NewHandle(v)
	return p
}

// given_ is what give_ kept.
func given_(p unsafe.Pointer) any {
	return (*(*cgo.Handle)(p)).Value()
}

// lent_ is the value of a function a call lends C, through a pointer to its handle that
// C passes back.
func lent_(p unsafe.Pointer) any {
	return (*(*cgo.Handle)(p)).Value()
}

// liftWaker_ is a waker Rust returned, as a func waking it through Rust until the GC
// collects the func.
func liftWaker_(w cWaker) func() {
	return rt.LiftWaker_(unsafe.Pointer(&w))
}

// waker_ is wake as a waker Rust holds until it drops it, calling it from any thread.
func waker_(wake func()) cWaker {
	return cWaker{wake: goCall, drop: goRelease, data: give_(wake)}
}

// future_ is run as the future Rust polls, from any thread: its first poll runs it on a
// goroutine of its own, under a context derived from ctx that Rust also cancels once it
// drops the future, finished or not.
func future_(ctx context.Context, run func(context.Context)) cForeignFuture {
	return cForeignFuture{poll: goPoll, free: goRelease, data: give_(rt.Pollable_(ctx, run))}
}

// reader_ is the reader Rust reads and seeks during the call, through the handle k of what
// rt.Reader_ made, which the call deletes once it returns.
func reader_(k *cgo.Handle) cForeignReader {
	return cForeignReader{read: goRead, seek: goSeek, data: unsafe.Pointer(k)}
}

// await_ polls a future whose call pended until poll reports it finished, returning its
// error, or ctx's once ctx is done: then the caller frees the future unfinished, which
// cancels it in Rust and loses nothing, and the next call sees what it would have taken.
// The call polled it first with no waker, which only a future that pends needs, and
// checked ctx before its first poll; a wake that lands between a poll that pends and the
// wait is kept, so the wait returns at once.
func await_(ctx context.Context, poll func(cWaker) (bool, error)) error {
	woken := make(chan struct{}, 1)
	waker := waker_(func() {
		select {
		case woken <- struct{}{}:
		default:
		}
	})
	for {
		if done, err := poll(waker); done {
			return err
		}
		select {
		case <-woken:
		case <-ctx.Done():
		}
		// Done between the wake and the poll: the poll would take what the next call sees.
		if err := ctx.Err(); err != nil {
			return err
		}
	}
}
