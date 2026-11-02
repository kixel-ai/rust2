// The runtime every generated Go file includes, after the generated code names the
// export's C types as cBuf, cWaker, cDuration, and cAddr, its buffer free as cBufFree, and
// its callbacks into Go as goCall and goRelease: the handles objects own, the arena a call
// lowers nested values into, values across the C ABI, and what a future waits on.

// The holds a handle counts, one for its owner until closed and one per call, shifted
// past a bit set once closed.
const (
	closed_ = 1
	held_   = 2
)

// ErrClosed is what a call returns, wrapped with the object's type, once an object it takes
// is closed: by Close, by a call that moved it into Rust, or, for one Rust lent a callback,
// once that callback returned.
var ErrClosed = errors.New("closed")

type closedError_ struct {
	name string
}

func (e closedError_) Error() string {
	return e.name + " is closed"
}

func (e closedError_) Is(target error) bool {
	return target == ErrClosed
}

// handle is what an object owns, or is lent for a call: each call holds it, so Close frees
// it only once no call does, and a call after Close returns ErrClosed.
type handle struct {
	raw   unsafe.Pointer
	state atomic.Int64
	// Nil for a handle Rust lends, which it frees itself.
	free func(unsafe.Pointer)
	name string
	// What closes an owned handle the GC finds unclosed.
	cleanup runtime.Cleanup
}

func newHandle(raw unsafe.Pointer, free func(unsafe.Pointer), name string) *handle {
	h := &handle{raw: raw, free: free, name: name}
	h.state.Store(held_)
	return h
}

// hold is the handle, held until release, or nil once closed.
func (h *handle) hold() unsafe.Pointer {
	for {
		n := h.state.Load()
		if n&closed_ != 0 {
			return nil
		}
		if h.state.CompareAndSwap(n, n+held_) {
			return h.raw
		}
	}
}

// closed is the error a call returns once h is closed.
func (h *handle) closed() error {
	return closedError_{h.name}
}

func (h *handle) release() {
	if h.state.Add(-held_) < held_ && h.free != nil {
		h.free(h.raw)
	}
}

// close gives up the owner's hold, freeing the handle once no call holds it; again, it
// does nothing.
func (h *handle) close() {
	for {
		n := h.state.Load()
		if n&closed_ != 0 {
			return
		}
		next := (n | closed_) - held_
		if h.state.CompareAndSwap(n, next) {
			// A cleanup left registered until the GC slows each one registered after it.
			h.cleanup.Stop()
			if next < held_ && h.free != nil {
				h.free(h.raw)
			}
			return
		}
	}
}

// end ends a lend, closing the handle, which Rust frees next: a call still holding it
// would reach the freed handle, so it panics, as Rust's own check fails once a call
// borrowed.
func (h *handle) end() {
	h.close()
	if h.state.Load() != closed_ {
		panic(h.name + ": a call still holds it when its lend ends")
	}
}

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

// Bytes is bytes Rust returned, or a record holds. From lendFrom_ bytes they are Rust's
// buffer, read in place and freed only once the GC finds no Bytes, sub-view, or reader of
// it, so none reads freed memory; below it, a copy in Go's memory. No
// method lets Rust's memory out as a []byte but for a call of an io.Writer, which must not
// keep it. Read and WriteTo consume what they read, as bytes.Buffer's do; the rest leave
// it. The zero Bytes is empty.
type Bytes struct {
	// What frees Rust's buffer once unreachable, or nil for Go's memory or none.
	owner *bytesOwner_
	// Nil once empty, since Go's memory may not be pointed past its end.
	ptr unsafe.Pointer
	len int
}

// bytesOwner_ is what owns a buffer Rust returned, freed once the GC collects it.
type bytesOwner_ struct {
	buf cBuf
}

// NewBytes is b as Bytes, for a record passed in, holding b itself without a copy.
func NewBytes(b []byte) Bytes {
	if len(b) == 0 {
		return Bytes{}
	}
	return Bytes{ptr: unsafe.Pointer(unsafe.SliceData(b)), len: len(b)}
}

// view is what b reads, valid only while b's owner is reachable.
func (b Bytes) view() []byte {
	if b.len == 0 {
		return nil
	}
	return unsafe.Slice((*byte)(b.ptr), b.len)
}

// Len is how many bytes b holds.
func (b Bytes) Len() int {
	return b.len
}

// At is the byte at i, panicking out of range as a slice does.
func (b Bytes) At(i int) byte {
	v := b.view()[i]
	runtime.KeepAlive(b.owner)
	return v
}

// Slice is the bytes from i up to j, without a copy, keeping b's buffer alive while
// reachable; it panics out of range as a slice does.
func (b Bytes) Slice(i, j int) Bytes {
	_ = b.view()[i:j]
	if i == j {
		return Bytes{}
	}
	return Bytes{owner: b.owner, ptr: unsafe.Add(b.ptr, i), len: j - i}
}

// CopyTo copies b into dst, as copy does, returning how many bytes it copied.
func (b Bytes) CopyTo(dst []byte) int {
	n := copy(dst, b.view())
	runtime.KeepAlive(b.owner)
	return n
}

// Copy is a copy of b in Go's memory.
func (b Bytes) Copy() []byte {
	v := make([]byte, b.len)
	copy(v, b.view())
	runtime.KeepAlive(b.owner)
	return v
}

// Read reads into p what b holds, consuming it, and io.EOF once b is empty.
func (b *Bytes) Read(p []byte) (int, error) {
	if b.len == 0 {
		if len(p) == 0 {
			return 0, nil
		}
		return 0, io.EOF
	}
	n := copy(p, b.view())
	runtime.KeepAlive(b.owner)
	b.consume(n)
	return n, nil
}

// WriteTo writes w what b holds, without a copy, consuming what w took.
func (b *Bytes) WriteTo(w io.Writer) (int64, error) {
	if b.len == 0 {
		return 0, nil
	}
	n, err := w.Write(b.view())
	runtime.KeepAlive(b.owner)
	if n > b.len {
		panic("an io.Writer wrote more than it was given")
	}
	b.consume(n)
	if err == nil && b.len != 0 {
		err = io.ErrShortWrite
	}
	return int64(n), err
}

func (b *Bytes) consume(n int) {
	if n == b.len {
		*b = Bytes{}
		return
	}
	b.ptr = unsafe.Add(b.ptr, n)
	b.len -= n
}

// lendFrom_ is the size from which bytes Rust returns are lent rather than copied: below
// it, a copy with Rust's buffer freed at once costs less than the cleanup a lent buffer
// registers, about 700 ns at any size.
const lendFrom_ = 2048

// liftBytes_ is bytes Rust returned: below lendFrom_ copied, with Rust's buffer freed at
// once, and otherwise lent in place until the GC frees them.
func liftBytes_(buf cBuf) Bytes {
	if buf.len < lendFrom_ {
		b := NewBytes(append([]byte(nil), view_(buf)...))
		cBufFree(buf)
		return b
	}
	owner := &bytesOwner_{buf: buf}
	runtime.AddCleanup(owner, cBufFree, buf)
	return Bytes{owner: owner, ptr: unsafe.Pointer(buf.ptr), len: int(buf.len)}
}

func liftBytesOption_(buf cBuf) *Bytes {
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

// owner_ is an object, which owns the handle it is lent or Rust returned.
type owner_ interface {
	handle_() *handle
}

// unhold_ releases the hold a call took on o's handle. Deferred, it keeps o reachable until
// the call returns, so the GC cannot close o's handle between the call reading and holding
// it.
func unhold_(o owner_) {
	o.handle_().release()
}

// arena_ is what a call lowers nested values into, kept until it returns: the Go memory C
// reads through it, pinned, and the objects whose handles it holds, kept reachable.
type arena_ struct {
	pins runtime.Pinner
	held []owner_
	// The owners of Rust's buffers C reads.
	kept []*bytesOwner_
	// The first object the call takes that was closed, which fails the call before it is
	// made.
	err error
}

func (a *arena_) finish() {
	for _, o := range a.held {
		o.handle_().release()
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
func (a *arena_) bufView(b Bytes) cBuf {
	if b.len == 0 {
		return cBuf{ptr: (*C.uint8_t)(emptyPtr_())}
	}
	if b.owner == nil {
		a.pins.Pin(b.ptr)
	} else {
		a.kept = append(a.kept, b.owner)
	}
	return cBuf{ptr: (*C.uint8_t)(b.ptr), len: C.size_t(b.len)}
}

func (a *arena_) bufStringOption(s *string) cBuf {
	if s == nil {
		return cBuf{}
	}
	return a.bufString(*s)
}

func (a *arena_) bufViewOption(b *Bytes) cBuf {
	if b == nil {
		return cBuf{}
	}
	return a.bufView(*b)
}

// hold is o's handle, held until the call returns, or nil, failing the call, once closed.
func (a *arena_) hold(o owner_) unsafe.Pointer {
	h := o.handle_()
	raw := h.hold()
	if raw == nil {
		if a.err == nil {
			a.err = h.closed()
		}
		return nil
	}
	a.held = append(a.held, o)
	return raw
}

// holdOption is the handle o holds, or nil for none.
func (a *arena_) holdOption(o owner_) unsafe.Pointer {
	if o.handle_() == nil {
		return nil
	}
	return a.hold(o)
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

// release_ lets go of what give_ kept, closing it if it is an io.Closer.
func release_(p unsafe.Pointer) {
	h := *(*cgo.Handle)(p)
	v := h.Value()
	h.Delete()
	C.free(p)
	if closer, ok := v.(io.Closer); ok {
		closer.Close()
	}
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
