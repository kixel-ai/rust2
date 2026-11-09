// The runtime the generated packages share, after the generated code names the export's
// buffer as cBuf, its frees as cBufFree and cBufsFree, and its callbacks into Go as goCall
// and goRelease: the handles objects own, the bytes Rust returns, and what Rust keeps of
// Go's. Each generated package keeps its own copy of what converts its C types, which
// cgo makes its own, in ffi_cgo.go; it reaches this through Go's types alone.

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

// Handle_ is what an object owns, or is lent for a call: each call holds it, so Close frees
// it only once no call does, and a call after Close returns ErrClosed. For the generated
// packages, which share it: use none of it.
type Handle_ struct {
	raw   unsafe.Pointer
	state atomic.Int64
	// Nil for a handle Rust lends, which it frees itself.
	free func(unsafe.Pointer)
	name string
	// What closes an owned handle the GC finds unclosed.
	Cleanup runtime.Cleanup
}

// NewHandle_ is a handle of raw, which free frees, or nil for one Rust lends.
func NewHandle_(raw unsafe.Pointer, free func(unsafe.Pointer), name string) *Handle_ {
	h := &Handle_{raw: raw, free: free, name: name}
	h.state.Store(held_)
	return h
}

// Hold is the handle, held until Release, or nil once closed.
func (h *Handle_) Hold() unsafe.Pointer {
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

// Closed is the error a call returns once h is closed.
func (h *Handle_) Closed() error {
	return closedError_{h.name}
}

// Release gives up a call's hold.
func (h *Handle_) Release() {
	if h.state.Add(-held_) < held_ && h.free != nil {
		h.free(h.raw)
	}
}

// Close gives up the owner's hold, freeing the handle once no call holds it; again, it
// does nothing.
func (h *Handle_) Close() {
	for {
		n := h.state.Load()
		if n&closed_ != 0 {
			return
		}
		next := (n | closed_) - held_
		if h.state.CompareAndSwap(n, next) {
			// A cleanup left registered until the GC slows each one registered after it.
			h.Cleanup.Stop()
			if next < held_ && h.free != nil {
				h.free(h.raw)
			}
			return
		}
	}
}

// End ends a lend, closing the handle, which Rust frees next: a call still holding it
// would reach the freed handle, so it panics, as Rust's own check fails once a call
// borrowed.
func (h *Handle_) End() {
	h.Close()
	if h.state.Load() != closed_ {
		panic(h.name + ": a call still holds it when its lend ends")
	}
}

// view_ is the bytes buf points at, which C keeps.
func view_(buf cBuf) []byte {
	if buf.len == 0 {
		return nil
	}
	return unsafe.Slice((*byte)(unsafe.Pointer(buf.ptr)), int(buf.len))
}

// Bytes is bytes Rust returned, or a record holds. From lendFrom_ bytes they are Rust's
// buffer, read in place and freed only once the GC finds no Bytes, sub-view, or reader of
// it or of the buffers lent in its batch, so none reads freed memory; below it, a copy in
// Go's memory. No
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

// bytesOwner_ is what owns a batch of buffers Rust returned, freed once the GC collects it.
type bytesOwner_ struct {
	batch *lentBatch_
}

// lentBatch_ is buffers lent one after another, freed together by one cleanup, which costs
// far less than one each: up to 16, within lentBatchMax_ bytes but for one larger alone.
type lentBatch_ struct {
	bufs [16]cBuf
	held int
	// How many of bufs are lent, stored after each so the cleanup reads them all.
	n atomic.Int32
}

// lentBatchMax_ is the most bytes a batch lends, which bounds what a Bytes kept long keeps
// of the others in its batch.
const lentBatchMax_ = 64 << 10

// lending_ holds the owner of the batch each P's lends join until it fills; a GC drops it.
var lending_ sync.Pool

// lentFreeing_ frees one batch at a time: Go runs cleanups on several goroutines, and all of
// them freeing at once contend for the C allocator's lock, which costs a lent buffer
// several times more.
var lentFreeing_ sync.Mutex

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
// it, a copy with Rust's buffer freed at once costs less than a lent buffer, about 200 ns
// at any size.
const lendFrom_ = 512

// lentMax_ is the most bytes lent from Rust at once, freed or not. The GC sees none of
// them, so a program making little garbage of its own would never collect the Bytes
// holding them; past this, bytes are copied into Go's heap, whose garbage the GC does see.
const lentMax_ = 64 << 20

// lentHeld_ is the bytes lent from Rust and not yet freed.
var lentHeld_ atomic.Int64

// lends_ is whether n bytes Rust returned are lent rather than copied: from lendFrom_, and
// within lentMax_, which they then count against until freed. A call racing another past
// the limit may copy where it could have lent, never the reverse.
func lends_(n int64) bool {
	if n < lendFrom_ {
		return false
	}
	if lentHeld_.Add(n) <= lentMax_ {
		return true
	}
	lentHeld_.Add(-n)
	return false
}

// lend_ is the owner of the batch buf joins, which keeps it until the GC collects them.
func lend_(buf cBuf) *bytesOwner_ {
	o, _ := lending_.Get().(*bytesOwner_)
	if o == nil || o.batch.held+int(buf.len) > lentBatchMax_ {
		o = &bytesOwner_{batch: new(lentBatch_)}
		runtime.AddCleanup(o, freeLent_, o.batch)
	}
	b := o.batch
	n := b.n.Load()
	b.bufs[n] = buf
	b.held += int(buf.len)
	b.n.Store(n + 1)
	if int(n)+1 < len(b.bufs) && b.held < lentBatchMax_ {
		lending_.Put(o)
	}
	return o
}

// freeLent_ frees a batch of buffers lends_ counted, uncounted first so none Rust dropped
// counts.
func freeLent_(b *lentBatch_) {
	n := b.n.Load()
	lentHeld_.Add(-int64(b.held))
	lentFreeing_.Lock()
	cBufsFree(b.bufs[:n])
	lentFreeing_.Unlock()
}

// LiftBytes_ is the bytes of the buffer at buf, which Rust returned: copied, with Rust's
// buffer freed at once, unless it lends_ them in place until the GC frees their batch. For
// the generated packages, whose buffers are each package's C type.
func LiftBytes_(p unsafe.Pointer) Bytes {
	buf := *(*cBuf)(p)
	if !lends_(int64(buf.len)) {
		b := NewBytes(append([]byte(nil), view_(buf)...))
		cBufFree(buf)
		return b
	}
	return Bytes{owner: lend_(buf), ptr: unsafe.Pointer(buf.ptr), len: int(buf.len)}
}

// BytesBuf_ is what b reads, and what keeps it alive, nil for Go's memory, which a call
// pins instead. For the generated packages.
func BytesBuf_(b Bytes) (unsafe.Pointer, int, any) {
	if b.owner == nil {
		return b.ptr, b.len, nil
	}
	return b.ptr, b.len, b.owner
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

// GoCall_ is the C function calling what give_ kept as a func(), and GoRelease_ the one
// releasing it, for the generated packages' wakers and tables.
func GoCall_() unsafe.Pointer {
	return unsafe.Pointer(goCall)
}

func GoRelease_() unsafe.Pointer {
	return unsafe.Pointer(goRelease)
}
