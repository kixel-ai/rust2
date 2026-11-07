// What each generated package converts its C types with, which cgo makes its own, after
// the generated code names the export's C types as cBuf, cWaker, cDuration, and cAddr, its
// buffer's free as cBufFree, and the runtime's callbacks into Go as goCall and goRelease:
// the arena a call lowers nested values into, values across the C ABI, and what a future
// waits on.

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

// lowerDuration_ is a Duration as Rust holds it, which has no negative one.
func lowerDuration_(d time.Duration) cDuration {
	if d < 0 {
		panic("a negative duration has no Rust Duration")
	}
	return cDuration{secs: C.uint64_t(d / time.Second), nanos: C.uint32_t(d % time.Second)}
}

// liftDuration_ is a Duration Rust returned, which panics past time.Duration's longest,
// about 292 years.
func liftDuration_(c cDuration) time.Duration {
	secs, nanos := uint64(c.secs), time.Duration(c.nanos)
	if secs > uint64((math.MaxInt64-nanos)/time.Second) {
		panic("a Rust Duration past time.Duration's longest")
	}
	return time.Duration(secs)*time.Second + nanos
}

// lowerAddr_ is an address as Rust's SocketAddr holds it: a zone names IPv6's scope by
// its number or its interface's name.
func lowerAddr_(a netip.AddrPort) cAddr {
	ip := a.Addr()
	if !ip.IsValid() {
		panic("an invalid address has no Rust SocketAddr")
	}
	var c cAddr
	c.port = C.uint16_t(a.Port())
	if ip.Is4() {
		b := ip.As4()
		copy(unsafe.Slice((*byte)(unsafe.Pointer(&c.ip)), 4), b[:])
		return c
	}
	b := ip.As16()
	copy(unsafe.Slice((*byte)(unsafe.Pointer(&c.ip)), 16), b[:])
	c.v6 = true
	if zone := ip.Zone(); zone != "" {
		if id, err := strconv.ParseUint(zone, 10, 32); err == nil {
			c.scope_id = C.uint32_t(id)
		} else if iface, err := net.InterfaceByName(zone); err == nil {
			c.scope_id = C.uint32_t(iface.Index)
		} else {
			panic("an IPv6 zone that names no interface: " + zone)
		}
	}
	return c
}

// lowerAddrV4_ is an IPv4 address, for Rust's SocketAddrV4.
func lowerAddrV4_(a netip.AddrPort) cAddr {
	if !a.Addr().Is4() {
		panic("an address that is not IPv4 has no Rust SocketAddrV4")
	}
	return lowerAddr_(a)
}

// lowerAddrV6_ is an IPv6 address, for Rust's SocketAddrV6.
func lowerAddrV6_(a netip.AddrPort) cAddr {
	if !a.Addr().Is6() {
		panic("an address that is not IPv6 has no Rust SocketAddrV6")
	}
	return lowerAddr_(a)
}

// liftAddr_ is an address Rust returned. Go keeps no IPv6 flow label, so one panics.
func liftAddr_(c cAddr) netip.AddrPort {
	b := unsafe.Slice((*byte)(unsafe.Pointer(&c.ip)), 16)
	if !c.v6 {
		return netip.AddrPortFrom(netip.AddrFrom4([4]byte(b[:4])), uint16(c.port))
	}
	if c.flowinfo != 0 {
		panic("an IPv6 flow label has no Go form")
	}
	ip := netip.AddrFrom16([16]byte(b))
	if c.scope_id != 0 {
		ip = ip.WithZone(strconv.FormatUint(uint64(c.scope_id), 10))
	}
	return netip.AddrPortFrom(ip, uint16(c.port))
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

// unhold_ releases the hold a call took on h, the handle of the object o. Deferred, it
// keeps o reachable until the call returns, so the GC cannot close h between the call
// reading and holding it.
func unhold_(h *rt.Handle_, o any) {
	h.Release()
	runtime.KeepAlive(o)
}

// held_ is a handle a call holds, and the object owning it, kept reachable.
type held_ struct {
	h *rt.Handle_
	o any
}

// arena_ is what a call lowers nested values into, kept until it returns: the Go memory C
// reads through it, pinned, and the objects whose handles it holds, kept reachable.
type arena_ struct {
	pins runtime.Pinner
	held []held_
	// The owners of Rust's buffers C reads.
	kept []any
	// The first object the call takes that was closed, which fails the call before it is
	// made.
	err error
}

func (a *arena_) finish() {
	for _, held := range a.held {
		held.h.Release()
	}
	a.pins.Unpin()
}

func (a *arena_) pin(p unsafe.Pointer) {
	if p != emptyPtr_() {
		a.pins.Pin(p)
	}
}

func (a *arena_) bufString(s string) cBuf {
	b := bufString_(s)
	a.pin(unsafe.Pointer(b.ptr))
	return b
}

// bufView is the buffer of b, which C reads during the call: Go's memory pinned, and
// Rust's kept reachable.
func (a *arena_) bufView(b rt.Bytes) cBuf {
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

func (a *arena_) bufViewOption(b *rt.Bytes) cBuf {
	if b == nil {
		return cBuf{}
	}
	return a.bufView(*b)
}

// hold is h, the handle of the object o, held until the call returns, or nil, failing the
// call, once closed.
func (a *arena_) hold(h *rt.Handle_, o any) unsafe.Pointer {
	raw := h.Hold()
	if raw == nil {
		if a.err == nil {
			a.err = h.Closed()
		}
		return nil
	}
	a.held = append(a.held, held_{h, o})
	return raw
}

// list_ is items as a C array, each lowered by lower, pinned until the call returns:
// never null, so an empty one is told apart from a missing one.
func list_[T, CT any](a *arena_, items []T, lower func(T) CT) (*CT, C.size_t) {
	if len(items) == 0 {
		return (*CT)(emptyPtr_()), 0
	}
	out := make([]CT, len(items))
	for i, item := range items {
		out[i] = lower(item)
	}
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

// waker_ is wake as a waker Rust holds until it drops it, calling it from any thread.
func waker_(wake func()) cWaker {
	return cWaker{wake: goCall, drop: goRelease, data: give_(wake)}
}

// await_ polls a future until poll reports it finished, returning its error, or ctx's
// once ctx is done: then the caller frees the future unfinished, which cancels it in Rust
// and loses nothing, and the next call sees what it would have taken. Its first poll makes
// no waker, which only a future that pends needs; a wake that lands between a poll that
// pends and the wait is kept, so the wait returns at once.
func await_(ctx context.Context, poll func(cWaker) (bool, error)) error {
	// A context done already takes nothing, even what is ready.
	if err := ctx.Err(); err != nil {
		return err
	}
	if done, err := poll(cWaker{}); done {
		return err
	}
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
