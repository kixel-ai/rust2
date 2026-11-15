// The runtime the generated packages share, after the generated code names the export's
// buffer as cBuf, its frees as cBufFree and cBufsFree, its waker as cWaker, woken and
// dropped by cWakerWake and cWakerDrop, and its callbacks into Go as goCall, goRelease,
// and goPoll: the handles objects own, the bytes and wakers Rust returns, and what Rust
// keeps of Go's. Each generated package keeps its own copy of what converts its C types, which
// cgo makes its own, in ffi_cgo.go; it reaches this through Go's types alone.

// The holds a handle counts, one for its owner until closed and one per call, shifted
// past a bit set once closed, one set while an unchecked Close decides, and one set once
// calls contended for them, after which they hold it in slots_: in one word, so a call
// reads them all at once, and the count is below held_ only once no hold is left.
const (
	closed_  = 1
	bias_    = 2
	closing_ = 4
	held_    = 8
)

// Once calls contend for a handle's count, each holds it in a slot of slots_ instead, so
// goroutines sharing a handle write nothing in common. A close that would free the handle
// claims it closed first, then counts each hold it finds there, which lets go through the
// count, so the last to let go frees it: ffi/tests/hold.rs models this under loom.
const (
	// Set in a slot whose hold a close counted.
	revoked_ = 1
	// A cache line of slots to a column, which a goroutine's stack address picks, and a
	// row, which the handle's address picks: two goroutines in one column, or two handles
	// in one row on one goroutine, fall back to the count.
	slotRows_    = 8
	slotColumns_ = 256
)

// slots_ is the table, its columns on cache lines of their own.
var slots_ = func() []atomic.Uintptr {
	s := make([]atomic.Uintptr, (slotColumns_+1)*slotRows_)
	skip := (64 - uintptr(unsafe.Pointer(&s[0]))%64) % 64 / 8
	return s[skip : skip+slotColumns_*slotRows_]
}()

// fibonacci_ spreads an address's bits into a hash's top ones.
const fibonacci_ = 0x9E3779B97F4A7C15

// Slot_ is where a hold went: a slot of slots_, or nil for the count.
type Slot_ struct {
	p *atomic.Uintptr
}

// ErrClosed is what a call returns, wrapped with the object's type, once an object it takes
// is closed: by Close, by a call that moved it into Rust, or, for one Rust lent a callback,
// once that callback returned.
var ErrClosed = errors.New("closed")

// ErrBorrowed is what a call returns, wrapped with the function's name, once Rust's rules
// refuse a borrow it makes: of an object another call or future borrows in a way the
// borrow checker forbids beside it, or of one that is not Sync from a second thread.
var ErrBorrowed = errors.New("borrowed")

// checkError_ is what a call returns once Rust refuses it before running: its status, or 0
// for an item not on the library's target, which matches errors.ErrUnsupported.
type checkError_ struct {
	what    string
	message string
	status  int32
}

func (e checkError_) Error() string {
	return e.what + ": " + e.message
}

func (e checkError_) Is(target error) bool {
	switch e.status {
	case 0:
		return target == errors.ErrUnsupported
	// Moved out, or lent from a handle that since changed: closed either way.
	case 1, 6:
		return target == ErrClosed
	default:
		return target == ErrBorrowed
	}
}

type closedError_ struct {
	name string
}

func (e closedError_) Error() string {
	return e.name + " is closed"
}

func (e closedError_) Is(target error) bool {
	return target == ErrClosed
}

// Handle_ is what an object owns, or is lent for a call: each call holds it, so it is
// freed only once no call does, and a call after Close returns ErrClosed. Close of one
// whose value Rust checks, which drop drops, drops the value as Rust's own drop does,
// failing while a call borrows it in Rust; of any other, it fails while a call holds it.
// For the generated packages, which share it: use none of it.
type Handle_ struct {
	raw   unsafe.Pointer
	state atomic.Int64
	// Nil for a handle Rust lends, which it frees itself.
	free func(unsafe.Pointer)
	// Non-nil for an owned one of a value Rust checks.
	drop func(unsafe.Pointer) error
	name string
	// The nursery an owned handle was returned into, and its slot there.
	nursery *nursery_
	slot    int
	// What frees an owned handle once the GC collects its object, from when it leaves its
	// nursery still open.
	cleanup runtime.Cleanup
}

// Init makes h a handle of raw, which free frees, or nil for one Rust lends, and drop
// drops the value of, or nil for one Rust does not check. The generated packages allocate
// it with the object owning it, which each call holding it then keeps reachable: so once
// the GC collects the object, no call holds the handle, and its cleanup frees it at once.
func (h *Handle_) Init(raw unsafe.Pointer, free func(unsafe.Pointer), drop func(unsafe.Pointer) error, name string) {
	h.raw, h.free, h.drop, h.name = raw, free, drop, name
	h.state.Store(held_)
}

// Own puts h, which Rust returned and its object owns, in a nursery of the latest
// returned, where a Close frees it without the GC's registration, which costs more than
// the rest of a return and close together. One that leaves it open, pushed out by later
// returns or with its nursery once the pool drops that, is registered then. For the
// generated packages.
func (h *Handle_) Own() {
	r, _ := nurseries_.Get().(*ring_)
	if r == nil {
		r = &ring_{n: new(nursery_)}
		runtime.AddCleanup(r, (*nursery_).flush, r.n)
	}
	r.n.push(h)
	nurseries_.Put(r)
}

// nurseries_ holds the ring each P returns its objects into; a GC drops an idle one.
var nurseries_ sync.Pool

// ring_ is what the pool holds of a nursery, which its handles point at: so its cleanup
// runs once the pool dropped it, whatever its handles do.
type ring_ struct {
	n *nursery_
}

// nurseryLen_ is how many of the latest objects returned a nursery holds: an object
// dropped unclosed is freed only once this many more were returned on its P, or the pool
// dropped its nursery, so it bounds what a dropped object keeps of Rust's.
const nurseryLen_ = 64

// nursery_ is the latest handles returned, oldest at next once full. A slot's raw handle is
// cleared by whichever comes first, of its Close and its eviction, with a compare-and-swap.
type nursery_ struct {
	// Kept reachable while here, so an eviction can register them. Only the ring's holder
	// touches these two.
	held [nurseryLen_]*Handle_
	next int
	// Each slot's raw handle until closed or evicted, else nil.
	raw [nurseryLen_]unsafe.Pointer
}

func (n *nursery_) push(h *Handle_) {
	i := n.next
	n.next = (i + 1) % nurseryLen_
	if old := n.held[i]; old != nil {
		n.evict(i, old)
	}
	n.held[i] = h
	h.nursery, h.slot = n, i
	atomic.StorePointer(&n.raw[i], h.raw)
}

// evict registers h, in slot i, with the GC, unless a Close took the slot first.
func (n *nursery_) evict(i int, h *Handle_) {
	if atomic.LoadPointer(&n.raw[i]) == nil {
		return
	}
	// Registered before the slot is given up, so a Close that finds it given up finds the
	// cleanup to stop.
	c := runtime.AddCleanup(h, h.free, h.raw)
	h.cleanup = c
	if !atomic.CompareAndSwapPointer(&n.raw[i], h.raw, nil) {
		c.Stop()
	}
}

// flush registers each handle still open in n, once the pool dropped its ring, so none
// waits on the others to be collected.
func (n *nursery_) flush() {
	for i := range n.raw {
		// Loaded first, as push stores it after the handle.
		if atomic.LoadPointer(&n.raw[i]) != nil {
			n.evict(i, n.held[i])
		}
	}
}

// leave takes h out of its nursery as it closes, before anything frees it, or else stops
// the cleanup its eviction registered.
func (h *Handle_) leave() {
	if n := h.nursery; n != nil && atomic.CompareAndSwapPointer(&n.raw[h.slot], h.raw, nil) {
		return
	}
	h.cleanup.Stop()
}

// Hold is the handle, held until Release takes the Slot_, or nil once closed.
func (h *Handle_) Hold() (unsafe.Pointer, Slot_) {
	if raw := h.Count(); raw != nil {
		return raw, Slot_{}
	}
	return h.Contend()
}

// Count is Hold's fast path, inlined into each call: the handle, held in the count until
// Uncount, unless calls contend for it or it is closed, when it is nil and Contend holds
// it instead.
func (h *Handle_) Count() unsafe.Pointer {
	if n := h.state.Load(); n&(closed_|bias_) == 0 && h.state.CompareAndSwap(n, n+held_) {
		return h.raw
	}
	return nil
}

// Contend is Hold once Count failed: in the table, unless another handle or goroutine has
// its slot, then in the count, after which later calls go to the table.
func (h *Handle_) Contend() (unsafe.Pointer, Slot_) {
	if h.state.Load()&bias_ != 0 {
		var here byte
		column := (uint64(uintptr(unsafe.Pointer(&here))) >> 11) * fibonacci_ >> 56
		s := &slots_[uintptr(column)*slotRows_+h.row()]
		key := h.key()
		if s.CompareAndSwap(0, key) {
			if h.state.Load()&closed_ == 0 {
				return h.raw, Slot_{s}
			}
			// A close claimed it meanwhile, and may have counted this hold.
			if !s.CompareAndSwap(key, 0) {
				s.Store(0)
				h.Uncount()
			}
			return nil, Slot_{}
		}
	}
	for {
		n := h.state.Load()
		if n&closed_ != 0 {
			return nil, Slot_{}
		}
		if h.state.CompareAndSwap(n, n+held_) {
			break
		}
	}
	// Count failed on an unbiased handle only as another call changed the count
	// meanwhile: later ones go to the table.
	if h.state.Load()&bias_ == 0 {
		h.state.Or(bias_)
	}
	return h.raw, Slot_{}
}

// key is what a slot holds for h: its address, which no other live handle has, and
// whose low bit is clear for revoked_.
func (h *Handle_) key() uintptr {
	return uintptr(unsafe.Pointer(h))
}

// row is h's row of slots_.
func (h *Handle_) row() uintptr {
	return uintptr((uint64(h.key()) >> 4) * fibonacci_ >> 61)
}

// Closed is the error a call returns once h is closed.
func (h *Handle_) Closed() error {
	return closedError_{h.name}
}

// Release gives up a call's hold, wherever Hold put it.
func (h *Handle_) Release(s Slot_) {
	if s.Counted() {
		h.Uncount()
	} else {
		h.Unlist(s)
	}
}

// Counted is whether a hold is in the count, for Uncount, or a slot, for Unlist.
func (s Slot_) Counted() bool {
	return s.p == nil
}

// Uncount gives up a hold in the count, small enough to inline.
func (h *Handle_) Uncount() {
	if h.state.Add(-held_) < held_ && h.free != nil {
		h.free(h.raw)
	}
}

// Unlist gives up a hold in a slot, or in the count once a close counted it.
func (h *Handle_) Unlist(s Slot_) {
	if s.p.Swap(0) != h.key() {
		h.Uncount()
	}
}

// Close closes h as Handle_ says; again, it does nothing.
func (h *Handle_) Close() error {
	if h.drop != nil {
		raw, s := h.Hold()
		if raw == nil {
			return nil
		}
		err := h.drop(raw)
		h.Release(s)
		if err != nil {
			return err
		}
		h.Moved()
		return nil
	}
	for {
		n := h.state.Load()
		// Another Close is deciding: wait for its outcome, which may leave h open.
		if n&closing_ != 0 {
			runtime.Gosched()
			continue
		}
		if n&closed_ != 0 {
			return nil
		}
		if n&^bias_ != held_ {
			return checkError_{h.name, "a handle is already borrowed", 3}
		}
		if h.state.CompareAndSwap(n, n|closed_|closing_) {
			break
		}
	}
	// Claimed closed, so a call holding it in the table after this finds it closed, and
	// one before is in the table. A call that left the table finding it closed held
	// nothing, so the count is still the owner's alone.
	if h.listed() {
		h.state.And(^(closed_ | closing_))
		return checkError_{h.name, "a handle is already borrowed", 3}
	}
	h.state.Store(closed_)
	h.leave()
	if h.free != nil {
		h.free(h.raw)
	}
	return nil
}

// listed is whether a call holds h in the table.
func (h *Handle_) listed() bool {
	if h.state.Load()&bias_ == 0 {
		return false
	}
	key := h.key()
	for i := h.row(); i < uintptr(len(slots_)); i += slotRows_ {
		if slots_[i].Load() == key {
			return true
		}
	}
	return false
}

// Moved closes an owned handle a call moved the value out of, or Close dropped, freeing it
// once no call holds it.
func (h *Handle_) Moved() {
	h.abandon()
}

// abandon gives up the owner's hold, freeing it once no call holds it.
func (h *Handle_) abandon() {
	for {
		n := h.state.Load()
		// An unchecked Close is deciding: wait for its outcome, which may leave h open.
		if n&closing_ != 0 {
			runtime.Gosched()
			continue
		}
		if n&closed_ != 0 {
			return
		}
		if h.state.CompareAndSwap(n, n|closed_) {
			break
		}
	}
	h.leave()
	// Claimed closed, so each call holding it in the table did so before: count them.
	if h.state.Load()&bias_ != 0 {
		key := h.key()
		for i := h.row(); i < uintptr(len(slots_)); i += slotRows_ {
			if s := &slots_[i]; s.Load() == key {
				h.state.Add(held_)
				if !s.CompareAndSwap(key, key|revoked_) {
					h.state.Add(-held_)
				}
			}
		}
	}
	h.Uncount()
}

// End ends a lend, closing the handle, which Rust frees next: a call still holding it
// would reach the freed handle, so it panics, as Rust's own check fails once a call
// borrowed.
func (h *Handle_) End() {
	h.abandon()
	if h.state.Load()&^bias_ != closed_ {
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

// BytesTake_ is what b reads as a buffer Rust takes: a second owner of Rust's buffer,
// sharing it without a copy, narrowed to b, or a nil owner for Go's memory, which C reads
// during the call and Rust copies. For the generated packages.
func BytesTake_(b Bytes) (unsafe.Pointer, int, unsafe.Pointer) {
	if b.owner == nil {
		return b.ptr, b.len, nil
	}
	// A Bytes of Rust's is one of its batch's buffers, or a range of one.
	batch := b.owner.batch
	p := uintptr(b.ptr)
	for _, buf := range batch.bufs[:batch.n.Load()] {
		start := uintptr(unsafe.Pointer(buf.ptr))
		if p >= start && p+uintptr(b.len) <= start+uintptr(buf.len) {
			owner := cBufClone(buf).owner
			runtime.KeepAlive(b.owner)
			return b.ptr, b.len, owner
		}
	}
	panic("a Bytes of Rust's is in none of its batch's buffers")
}

// BytesBuf_ is what b reads, and what keeps it alive, nil for Go's memory, which a call
// pins instead. For the generated packages.
func BytesBuf_(b Bytes) (unsafe.Pointer, int, any) {
	if b.owner == nil {
		return b.ptr, b.len, nil
	}
	return b.ptr, b.len, b.owner
}

// returned_ holds a waker Rust returned, which it drops once the GC collects this.
type returned_ struct {
	raw cWaker
}

// LiftWaker_ is the waker at p, which Rust returned, as a func waking it through Rust
// until the GC collects the func. For the generated packages, whose wakers are each
// package's C type.
func LiftWaker_(p unsafe.Pointer) func() {
	r := &returned_{raw: *(*cWaker)(p)}
	runtime.AddCleanup(r, cWakerDrop, r.raw)
	return func() {
		cWakerWake(r.raw)
		// Not dropped while it wakes.
		runtime.KeepAlive(r)
	}
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

// GoCall_ is the C function calling what give_ kept as a func(), GoRelease_ the one
// releasing it, and GoPoll_ the one polling what Pollable_ made, for the generated
// packages' wakers, tables, and futures.
func GoCall_() unsafe.Pointer {
	return unsafe.Pointer(goCall)
}

func GoRelease_() unsafe.Pointer {
	return unsafe.Pointer(goRelease)
}

func GoPoll_() unsafe.Pointer {
	return unsafe.Pointer(goPoll)
}

// GoRead_ and GoSeek_ are the C functions reading and seeking what Reader_ made.
func GoRead_() unsafe.Pointer {
	return unsafe.Pointer(goRead)
}

func GoSeek_() unsafe.Pointer {
	return unsafe.Pointer(goSeek)
}

// Reader_ is r as a reader Rust reads and seeks during a call. For the generated packages:
// use none of it.
func Reader_(r io.ReadSeeker) any {
	if r == nil {
		panic("a nil io.ReadSeeker")
	}
	return &reader_{r: r}
}

// StartSeeker is an io.ReadSeeker that also seeks to any offset from its start, as Rust's
// SeekFrom::Start takes a uint64, past the int64 an io.Seeker takes. Rust calls SeekStart
// only for an offset past math.MaxInt64; a reader without it fails that seek.
type StartSeeker interface {
	io.ReadSeeker
	SeekStart(offset uint64) (uint64, error)
}

// reader_ is a Go reader Rust reads and seeks one call at a time, as Rust's Read and Seek.
type reader_ struct {
	r io.ReadSeeker
	// What a read returned beside bytes, which the next read returns: Rust takes a read's
	// bytes or its error, never both.
	err error
}

// read is a read as Rust's: none only at the end, which a Go read marks with io.EOF.
func (r *reader_) read(p []byte) (int, error) {
	if err := r.err; err != nil {
		r.err = nil
		return 0, err
	}
	if len(p) == 0 {
		return 0, nil
	}
	// A Go read of none without an error did nothing, so it reads again, as bufio does,
	// until it gives up.
	for range 100 {
		n, err := r.r.Read(p)
		switch {
		case n > 0:
			if err != io.EOF {
				r.err = err
			}
			return n, nil
		case err == io.EOF:
			return 0, nil
		case err != nil:
			return 0, err
		}
	}
	return 0, io.ErrNoProgress
}

// seek is a seek as Rust's, whose offset from the start is offset's bits, unsigned.
func (r *reader_) seek(offset int64, whence int) (uint64, error) {
	if whence == io.SeekStart && offset < 0 {
		s, ok := r.r.(StartSeeker)
		if !ok {
			return 0, errors.New("a seek past math.MaxInt64 of a reader that is no StartSeeker")
		}
		at, err := s.SeekStart(uint64(offset))
		if err != nil {
			return 0, err
		}
		r.err = nil
		return at, nil
	}
	at, err := r.r.Seek(offset, whence)
	if err != nil {
		return 0, err
	}
	if at < 0 {
		return 0, errors.New("a Go reader sought before its start")
	}
	// What a read returned beside bytes was where it read, which a seek leaves.
	r.err = nil
	return uint64(at), nil
}

// Pollable_ is run as a future Rust polls, under a context derived from parent, which
// closing also cancels. For the generated packages: use none of it.
func Pollable_(parent context.Context, run func(context.Context)) io.Closer {
	ctx, cancel := context.WithCancel(parent)
	return &pollable_{run: run, ctx: ctx, cancel: cancel}
}

// pollable_ is a Go function Rust polls as a future, one poll at a time from any thread:
// its first poll runs it on a goroutine of its own, and once it returns, the waker of the
// latest poll is woken.
type pollable_ struct {
	run    func(context.Context)
	ctx    context.Context
	cancel context.CancelFunc
	mu     sync.Mutex
	// Whether a poll ran it, whether it returned, and whether the latest poll's waker is held.
	started, done, held bool
	waker               cWaker
}

func (p *pollable_) poll(waker cWaker) bool {
	p.mu.Lock()
	if p.done {
		p.mu.Unlock()
		cWakerDrop(waker)
		return true
	}
	old, had := p.waker, p.held
	p.waker, p.held = waker, true
	if !p.started {
		p.started = true
		go p.finish()
	}
	p.mu.Unlock()
	if had {
		cWakerDrop(old)
	}
	return false
}

// finish runs it, unless its context is done first, then wakes the latest poll's waker.
// Once its context is done, by the caller or by Rust dropping it, it ends unfinished:
// Rust never sees it done, so a call the caller cancelled reports the caller's error.
func (p *pollable_) finish() {
	if p.ctx.Err() == nil {
		p.run(p.ctx)
	}
	if p.ctx.Err() != nil {
		return
	}
	p.mu.Lock()
	p.done = true
	waker, had := p.waker, p.held
	p.held = false
	p.mu.Unlock()
	if had {
		cWakerWake(waker)
		cWakerDrop(waker)
	}
}

// Close cancels its context, once Rust drops it, and drops the waker it holds.
func (p *pollable_) Close() error {
	p.cancel()
	p.mu.Lock()
	waker, had := p.waker, p.held
	p.held = false
	p.mu.Unlock()
	if had {
		cWakerDrop(waker)
	}
	return nil
}

// ErrInvalid is what a checked conversion returns, wrapped with what failed, for a value one
// type holds and the other does not: text that is not UTF-8 or a rune that is not a Unicode
// scalar value, which a call taking one returns before it is made, or a value outside the
// type converted to.
var ErrInvalid = errors.New("invalid")

type invalidError_ string

func (e invalidError_) Error() string {
	return string(e)
}

func (e invalidError_) Is(target error) bool {
	return target == ErrInvalid
}

// Invalid_ is the error, matching ErrInvalid, that what says. For the generated packages.
func Invalid_(what string) error {
	return invalidError_(what)
}

// Duration is Rust's Duration: whole seconds, and the nanoseconds past them, below a
// second. The zero Duration is zero, and == compares two.
type Duration struct {
	secs  uint64
	nanos uint32
}

// NewDuration is secs seconds and nanos nanoseconds, nanoseconds past a second carrying into
// the seconds as Rust's Duration::new does, failing past Rust's longest.
func NewDuration(secs uint64, nanos uint32) (Duration, error) {
	carry := uint64(nanos / 1e9)
	if secs > math.MaxUint64-carry {
		return Duration{}, Invalid_("a duration past Rust's longest")
	}
	return Duration{secs: secs + carry, nanos: nanos % 1e9}, nil
}

// DurationOf is d as Rust's Duration, failing on a negative one, which Rust's has none of.
func DurationOf(d time.Duration) (Duration, error) {
	if d < 0 {
		return Duration{}, Invalid_("a negative duration has no Rust Duration")
	}
	return Duration{secs: uint64(d / time.Second), nanos: uint32(d % time.Second)}, nil
}

// Secs is d's whole seconds.
func (d Duration) Secs() uint64 {
	return d.secs
}

// Nanos is d's nanoseconds past its whole seconds, below a second.
func (d Duration) Nanos() uint32 {
	return d.nanos
}

// Std is d as a time.Duration, failing past its longest, about 292 years.
func (d Duration) Std() (time.Duration, error) {
	if d.secs > uint64((math.MaxInt64-time.Duration(d.nanos))/time.Second) {
		return 0, Invalid_("a duration past time.Duration's longest")
	}
	return time.Duration(d.secs)*time.Second + time.Duration(d.nanos), nil
}

// U128 is Rust's u128: its high and its low 64 bits.
type U128 struct {
	Hi, Lo uint64
}

// U128Of is v as a U128, failing outside 0 through 2^128 - 1.
func U128Of(v *big.Int) (U128, error) {
	if v.Sign() < 0 || v.BitLen() > 128 {
		return U128{}, Invalid_("an int outside Rust's u128")
	}
	var b [16]byte
	v.FillBytes(b[:])
	return U128{Hi: binary.BigEndian.Uint64(b[:8]), Lo: binary.BigEndian.Uint64(b[8:])}, nil
}

// Big is v as a big.Int.
func (v U128) Big() *big.Int {
	var b [16]byte
	binary.BigEndian.PutUint64(b[:8], v.Hi)
	binary.BigEndian.PutUint64(b[8:], v.Lo)
	return new(big.Int).SetBytes(b[:])
}

// Ipv4Addr is Rust's Ipv4Addr: its octets, in network order.
type Ipv4Addr [4]byte

// Ipv4AddrOf is ip as an Ipv4Addr, failing unless it is IPv4.
func Ipv4AddrOf(ip netip.Addr) (Ipv4Addr, error) {
	if !ip.Is4() {
		return Ipv4Addr{}, Invalid_("an address that is not IPv4 has no Rust Ipv4Addr")
	}
	return ip.As4(), nil
}

// Netip is ip as a netip.Addr.
func (ip Ipv4Addr) Netip() netip.Addr {
	return netip.AddrFrom4(ip)
}

// Ipv6Addr is Rust's Ipv6Addr: its octets, in network order.
type Ipv6Addr [16]byte

// Ipv6AddrOf is ip as an Ipv6Addr, failing unless it is IPv6 without a zone, which Rust's
// has no room for.
func Ipv6AddrOf(ip netip.Addr) (Ipv6Addr, error) {
	if !ip.Is6() {
		return Ipv6Addr{}, Invalid_("an address that is not IPv6 has no Rust Ipv6Addr")
	}
	if ip.Zone() != "" {
		return Ipv6Addr{}, Invalid_("an IPv6 address with a zone has no Rust Ipv6Addr")
	}
	return ip.As16(), nil
}

// Netip is ip as a netip.Addr, IPv6 even where it maps IPv4.
func (ip Ipv6Addr) Netip() netip.Addr {
	return netip.AddrFrom16(ip)
}

// IpAddr is Rust's IpAddr: an Ipv4Addr or an Ipv6Addr. The zero IpAddr is IPv4's 0.0.0.0,
// and == compares two.
type IpAddr struct {
	v4  Ipv4Addr
	v6  Ipv6Addr
	is6 bool
}

// IpAddrFromV4 is ip as an IpAddr.
func IpAddrFromV4(ip Ipv4Addr) IpAddr {
	return IpAddr{v4: ip}
}

// IpAddrFromV6 is ip as an IpAddr.
func IpAddrFromV6(ip Ipv6Addr) IpAddr {
	return IpAddr{v6: ip, is6: true}
}

// IpAddrOf is ip as an IpAddr, of its family, failing on an invalid one or a zone, which
// Rust's has no room for.
func IpAddrOf(ip netip.Addr) (IpAddr, error) {
	if ip.Is4() {
		return IpAddrFromV4(ip.As4()), nil
	}
	if !ip.IsValid() {
		return IpAddr{}, Invalid_("an invalid address has no Rust IpAddr")
	}
	v6, err := Ipv6AddrOf(ip)
	if err != nil {
		return IpAddr{}, err
	}
	return IpAddrFromV6(v6), nil
}

// V4 is a's IPv4 address, if it is one.
func (a IpAddr) V4() (Ipv4Addr, bool) {
	return a.v4, !a.is6
}

// V6 is a's IPv6 address, if it is one.
func (a IpAddr) V6() (Ipv6Addr, bool) {
	return a.v6, a.is6
}

// Netip is a as a netip.Addr.
func (a IpAddr) Netip() netip.Addr {
	if a.is6 {
		return a.v6.Netip()
	}
	return a.v4.Netip()
}

// SocketAddrV4 is Rust's SocketAddrV4: an IPv4 address and a port.
type SocketAddrV4 struct {
	IP   Ipv4Addr
	Port uint16
}

// SocketAddrV4Of is a as a SocketAddrV4, failing unless it is IPv4.
func SocketAddrV4Of(a netip.AddrPort) (SocketAddrV4, error) {
	ip, err := Ipv4AddrOf(a.Addr())
	if err != nil {
		return SocketAddrV4{}, Invalid_("an address that is not IPv4 has no Rust SocketAddrV4")
	}
	return SocketAddrV4{IP: ip, Port: a.Port()}, nil
}

// Netip is a as a netip.AddrPort.
func (a SocketAddrV4) Netip() netip.AddrPort {
	return netip.AddrPortFrom(a.IP.Netip(), a.Port)
}

// SocketAddrV6 is Rust's SocketAddrV6: an IPv6 address, a port, a flow label, and the scope
// a link-local address is in, a zone's interface number, or 0 for none.
type SocketAddrV6 struct {
	IP       Ipv6Addr
	Port     uint16
	FlowInfo uint32
	ScopeID  uint32
}

// SocketAddrV6Of is a as a SocketAddrV6, failing unless it is IPv6 with no zone or one naming
// an interface, by its number or its name, which it holds as that interface's number.
func SocketAddrV6Of(a netip.AddrPort) (SocketAddrV6, error) {
	ip := a.Addr()
	if !ip.Is6() {
		return SocketAddrV6{}, Invalid_("an address that is not IPv6 has no Rust SocketAddrV6")
	}
	v := SocketAddrV6{IP: ip.As16(), Port: a.Port()}
	if zone := ip.Zone(); zone != "" {
		if id, err := strconv.ParseUint(zone, 10, 32); err == nil {
			v.ScopeID = uint32(id)
		} else if iface, err := net.InterfaceByName(zone); err == nil {
			v.ScopeID = uint32(iface.Index)
		} else {
			return SocketAddrV6{}, Invalid_("an IPv6 zone that names no interface: " + zone)
		}
	}
	return v, nil
}

// Netip is a as a netip.AddrPort, its scope a zone of its number, failing on a flow label,
// which netip has no room for.
func (a SocketAddrV6) Netip() (netip.AddrPort, error) {
	if a.FlowInfo != 0 {
		return netip.AddrPort{}, Invalid_("an IPv6 flow label has no netip form")
	}
	ip := a.IP.Netip()
	if a.ScopeID != 0 {
		ip = ip.WithZone(strconv.FormatUint(uint64(a.ScopeID), 10))
	}
	return netip.AddrPortFrom(ip, a.Port), nil
}

// SocketAddr is Rust's SocketAddr: a SocketAddrV4 or a SocketAddrV6. The zero SocketAddr is
// IPv4's 0.0.0.0, port 0, and == compares two.
type SocketAddr struct {
	v4  SocketAddrV4
	v6  SocketAddrV6
	is6 bool
}

// SocketAddrFromV4 is a as a SocketAddr.
func SocketAddrFromV4(a SocketAddrV4) SocketAddr {
	return SocketAddr{v4: a}
}

// SocketAddrFromV6 is a as a SocketAddr.
func SocketAddrFromV6(a SocketAddrV6) SocketAddr {
	return SocketAddr{v6: a, is6: true}
}

// SocketAddrOf is a as a SocketAddr, of its family, failing on an invalid one or a zone
// SocketAddrV6Of fails on.
func SocketAddrOf(a netip.AddrPort) (SocketAddr, error) {
	if a.Addr().Is4() {
		return SocketAddrFromV4(SocketAddrV4{IP: a.Addr().As4(), Port: a.Port()}), nil
	}
	if !a.Addr().IsValid() {
		return SocketAddr{}, Invalid_("an invalid address has no Rust SocketAddr")
	}
	v6, err := SocketAddrV6Of(a)
	if err != nil {
		return SocketAddr{}, err
	}
	return SocketAddrFromV6(v6), nil
}

// V4 is a's IPv4 socket address, if it is one.
func (a SocketAddr) V4() (SocketAddrV4, bool) {
	return a.v4, !a.is6
}

// V6 is a's IPv6 socket address, if it is one.
func (a SocketAddr) V6() (SocketAddrV6, bool) {
	return a.v6, a.is6
}

// Netip is a as a netip.AddrPort, failing as SocketAddrV6's does.
func (a SocketAddr) Netip() (netip.AddrPort, error) {
	if a.is6 {
		return a.v6.Netip()
	}
	return a.v4.Netip(), nil
}
