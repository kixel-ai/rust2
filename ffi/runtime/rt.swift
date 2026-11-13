// The runtime every generated Swift file includes, after `Native` names the export's C
// types and functions: the handles classes own, the arena a call lowers its values into,
// bytes and text across the C ABI, and the signal a future parks on. Each helper that
// returns what its body does takes a noncopyable result, an object's struct.

/// An IP socket address, as Rust's `SocketAddr` holds it.
public struct SocketAddr: Hashable, Sendable {
	/// The IP in network order: 4 bytes for IPv4, or 16 for IPv6.
	public var ip: [UInt8]
	public var port: UInt16
	/// IPv6's flow label, zero for IPv4.
	public var flowinfo: UInt32
	/// IPv6's scope, zero for IPv4.
	public var scopeId: UInt32

	public init(ip: [UInt8], port: UInt16, flowinfo: UInt32 = 0, scopeId: UInt32 = 0) {
		self.ip = ip
		self.port = port
		self.flowinfo = flowinfo
		self.scopeId = scopeId
	}
}

/// Bytes as Rust holds them: Rust's stay in the buffer it returned, shared without a copy
/// by every `Bytes` and slice reading them and passed back to Rust uncopied, and Swift's are
/// a `Data`. Rust's are never lent as a `Data`, so nothing writes into the buffer Rust shares.
public struct Bytes: Swift.Hashable, Swift.Sendable, Swift.RandomAccessCollection {
	// The bytes: Swift's, or a view of Rust's buffer that keeps `owner` alive.
	let storage: Data
	// Rust's buffer holding the bytes, or nil for Swift's.
	let owner: Native.Owner?

	init(storage: Data, owner: Native.Owner?) {
		self.storage = storage
		self.owner = owner
	}

	/// No bytes.
	public init() {
		self.init(storage: Data(), owner: nil)
	}

	/// `data`'s bytes, sharing its storage as `Data` does: a call passing them lends them to
	/// Rust, which copies them where it keeps them.
	public init(_ data: Data) {
		self.init(storage: data, owner: nil)
	}

	/// `bytes`, copied.
	public init(_ bytes: some Swift.Sequence<Swift.UInt8>) {
		self.init(Data(bytes))
	}

	/// The bytes as a `Data`: Swift's as they are, and Rust's copied.
	public var data: Data {
		owner == nil ? storage : Data(storage)
	}

	/// Indices count from the bytes sliced from, as `Data`'s do.
	public var startIndex: Swift.Int { storage.startIndex }
	public var endIndex: Swift.Int { storage.endIndex }

	public subscript(i: Swift.Int) -> Swift.UInt8 {
		storage[i]
	}

	/// The bytes in `bounds`, sharing these without a copy.
	public subscript(bounds: Swift.Range<Swift.Int>) -> Bytes {
		Bytes(storage: storage[bounds], owner: owner)
	}

	/// Run `body` with the bytes in place, which it must not keep.
	public func withUnsafeBytes<R>(_ body: (UnsafeRawBufferPointer) throws -> R) rethrows -> R {
		try storage.withUnsafeBytes(body)
	}

	/// Whether `a` and `b` hold the same bytes, wherever each keeps them.
	public static func == (a: Bytes, b: Bytes) -> Swift.Bool {
		a.storage == b.storage
	}

	public func hash(into hasher: inout Swift.Hasher) {
		storage.hash(into: &hasher)
	}
}

/// A Rust `u128`, as its two halves, until the deployment target allows Swift's `UInt128`.
public struct U128: Hashable, Comparable, Sendable, CustomStringConvertible {
	public var high: UInt64
	public var low: UInt64

	public init(high: UInt64, low: UInt64) {
		self.high = high
		self.low = low
	}

	public init(_ value: UInt64) {
		self.init(high: 0, low: value)
	}

	public static func < (a: U128, b: U128) -> Bool {
		(a.high, a.low) < (b.high, b.low)
	}

	/// Its decimal digits.
	public var description: Swift.String {
		if high == 0 {
			return Swift.String(low)
		}
		// Split off the last 19 digits, as 10^19 is the largest power of ten a UInt64 holds.
		let base: UInt64 = 10_000_000_000_000_000_000
		let (q, r) = high.quotientAndRemainder(dividingBy: base)
		let (lowQ, digits) = base.dividingFullWidth((r, low))
		let tail = Swift.String(digits)
		return U128(high: q, low: lowQ).description + Swift.String(repeating: "0", count: 19 - tail.count) + tail
	}
}

/// Where a seek counts its offset from, as swift-system's `FileDescriptor.SeekOrigin` and
/// Rust's `SeekFrom` do.
public enum SeekOrigin: Sendable {
	case start
	case current
	case end
}

/// Bytes Rust reads and seeks during the call it is lent to, as Rust's `Read + Seek`.
/// Rust calls it from any thread, one call at a time, and keeps it no longer than the call.
/// A method that throws fails Rust's read or seek with the error's description as its
/// `io::Error`'s message.
public protocol SeekableReader {
	/// Read at most `buffer.count` bytes into `buffer`, returning how many; 0 is the end.
	mutating func read(into buffer: UnsafeMutableRawBufferPointer) throws -> Swift.Int

	/// Move to `offset` bytes from `origin`, returning the new position from the start.
	mutating func seek(offset: Swift.Int64, from origin: SeekOrigin) throws -> Swift.UInt64

	/// Move to `offset` bytes from the start, as Rust's `SeekFrom::Start` takes a `UInt64`,
	/// returning the new position. Rust calls it only for an offset past `Int64.max`, which
	/// by default fails.
	mutating func seek(start offset: Swift.UInt64) throws -> Swift.UInt64
}

extension SeekableReader {
	public mutating func seek(start offset: Swift.UInt64) throws -> Swift.UInt64 {
		guard let offset = Swift.Int64(exactly: offset) else { throw Native.SeekPast() }
		return try seek(offset: offset, from: .start)
	}
}

extension Native {
	/// A seek past `Int64.max` of a reader with no `seek(start:)` of its own.
	struct SeekPast: Swift.Error, CustomStringConvertible {
		var description: Swift.String { "a seek past Int64.max of a reader with no seek(start:)" }
	}
}

extension Native {
	/// `file` as the reader C lends Rust during `body`: boxed as itself, and written back
	/// once `body` returns, so a value type's reads and seeks stay.
	static func reader<R: SeekableReader, T: ~Copyable>(
		_ file: inout R, _ body: (CForeignReader) throws -> T
	) rethrows -> T {
		var boxed: any SeekableReader = file
		defer { file = boxed as! R }
		return try withUnsafeMutablePointer(to: &boxed) { try body(reader(UnsafeMutableRawPointer($0))) }
	}

	/// The reader C lends Rust of the boxed reader at `data`. Apart from `reader`, which is
	/// generic, since a C function captures no generic context.
	static func reader(_ data: UnsafeMutableRawPointer) -> CForeignReader {
				CForeignReader(
					read: { data, buf, len, n, err in
						let file = data!.assumingMemoryBound(to: (any SeekableReader).self)
						do {
							n!.pointee = try file.pointee.read(into: UnsafeMutableRawBufferPointer(start: buf, count: len))
							return true
						} catch {
							err!.pointee = Native.owned("\(error)")
							return false
						}
					},
					seek: { data, whence, offset, position, err in
						let file = data!.assumingMemoryBound(to: (any SeekableReader).self)
						// From the start, the offset's bits are unsigned, past what an Int64 holds.
						if whence == 0 && offset < 0 {
							do {
								position!.pointee = try file.pointee.seek(start: Swift.UInt64(bitPattern: offset))
								return true
							} catch {
								err!.pointee = Native.owned("\(error)")
								return false
							}
						}
						let origin: SeekOrigin
						switch whence {
						case 0: origin = .start
						case 1: origin = .current
						default: origin = .end
						}
						do {
							position!.pointee = try file.pointee.seek(offset: offset, from: origin)
							return true
						} catch {
							err!.pointee = Native.owned("\(error)")
							return false
						}
					},
					data: data)
	}

	/// `text` in a buffer Rust owns, as what a closure returns or throws.
	static func owned(_ text: Swift.String) -> Buf {
		view(text) { copy($0) }
	}
	/// Checked before any other call: a Swift file and a library from different builds
	/// must not mix.
	static let checked: Bool = {
		if !contract() {
			fatalError("\(file) and the library are from different builds")
		}
		return true
	}()

	@inline(__always)
	static func check() {
		_ = checked
	}

	/// A non-null pointer for empty bytes, which C reads as present, unlike a null one.
	nonisolated(unsafe) static let empty: UnsafeMutablePointer<UInt8> = {
		let empty = UnsafeMutablePointer<UInt8>.allocate(capacity: 1)
		empty.initialize(to: 0)
		return empty
	}()

	static var none: Buf { Buf(ptr: nil, len: 0, owner: nil) }

	/// `bytes` as a view C reads during `body`, without a copy. `Data`'s closure returns
	/// only a copyable result, so `body`'s leaves through an optional.
	@inline(__always)
	static func view<R: ~Copyable>(_ bytes: Bytes, _ body: (Buf) throws -> R) rethrows -> R {
		var result: R? = nil
		try bytes.storage.withUnsafeBytes { raw in
			result = try body(view(raw))
		}
		return result.take()!
	}

	@inline(__always)
	static func view<R: ~Copyable>(_ bytes: Bytes?, _ body: (Buf) throws -> R) rethrows -> R {
		guard let bytes else { return try body(none) }
		return try view(bytes, body)
	}

	/// `text`'s UTF-8 as a view C reads during `body`, without a copy for a native string.
	@inline(__always)
	static func view<R: ~Copyable>(_ text: Swift.String, _ body: (Buf) throws -> R) rethrows -> R {
		var text = text
		var result: R? = nil
		try text.withUTF8 { utf8 in
			result = try body(view(UnsafeRawBufferPointer(utf8)))
		}
		return result.take()!
	}

	@inline(__always)
	static func view<R: ~Copyable>(_ text: Swift.String?, _ body: (Buf) throws -> R) rethrows -> R {
		guard let text else { return try body(none) }
		return try view(text, body)
	}

	/// A view of `raw`, never null, so empty bytes read as present.
	@inline(__always)
	static func view(_ raw: UnsafeRawBufferPointer) -> Buf {
		Buf(
			ptr: raw.baseAddress?.assumingMemoryBound(to: UInt8.self) ?? UnsafePointer(empty), len: raw.count,
			owner: nil)
	}

	/// The buffer Rust returned that `Bytes` read, freed once no `Bytes` does.
	final class Owner: @unchecked Swift.Sendable {
		let buf: Buf

		init(_ buf: Buf) {
			self.buf = buf
		}

		deinit {
			Native.free(buf)
		}

		/// A second owner of `buf`, narrowed to `raw`, its bytes some `Bytes` reads, which
		/// Rust takes.
		func share(_ raw: UnsafeRawBufferPointer) -> Buf {
			var shared = Native.clone(buf)
			shared.ptr = raw.baseAddress!.assumingMemoryBound(to: UInt8.self)
			shared.len = raw.count
			return shared
		}
	}

	/// Bytes Rust returned, lent to `Bytes` without a copy and freed with the last one.
	static func bytes(_ buf: Buf) -> Bytes {
		guard let ptr = buf.ptr, buf.len > 0 else {
			free(buf)
			return Bytes()
		}
		let owner = Owner(buf)
		let storage = Data(
			bytesNoCopy: UnsafeMutableRawPointer(mutating: ptr), count: buf.len,
			deallocator: .custom { _, _ in withExtendedLifetime(owner) {} })
		// `Data` copies a few bytes inline, freeing Rust's buffer at once: those are Swift's.
		let lent = storage.withUnsafeBytes { $0.baseAddress == UnsafeRawPointer(ptr) }
		return Bytes(storage: storage, owner: lent ? owner : nil)
	}

	static func bytes(option buf: Buf) -> Bytes? {
		buf.ptr == nil ? nil : bytes(buf)
	}

	/// `bytes` as a buffer Rust takes during `body`: a second owner of Rust's own, or a view
	/// of Swift's, which Rust copies.
	@inline(__always)
	static func take<R: ~Copyable>(_ bytes: Bytes, _ body: (Buf) throws -> R) rethrows -> R {
		guard let owner = bytes.owner, !bytes.isEmpty else { return try view(bytes, body) }
		return try body(bytes.storage.withUnsafeBytes { owner.share($0) })
	}

	@inline(__always)
	static func take<R: ~Copyable>(_ bytes: Bytes?, _ body: (Buf) throws -> R) rethrows -> R {
		guard let bytes else { return try body(none) }
		return try take(bytes, body)
	}

	/// Text Rust takes during `body`: a view, which Rust copies.
	@inline(__always)
	static func take<R: ~Copyable>(_ text: Swift.String, _ body: (Buf) throws -> R) rethrows -> R {
		try view(text, body)
	}

	@inline(__always)
	static func take<R: ~Copyable>(_ text: Swift.String?, _ body: (Buf) throws -> R) rethrows -> R {
		try view(text, body)
	}

	/// Bytes C lends a callback for its call, copied.
	static func bytes(lent buf: Buf) -> Bytes {
		buf.len == 0 ? Bytes() : Bytes(Data(bytes: buf.ptr!, count: buf.len))
	}

	static func bytes(lentOption buf: Buf) -> Bytes? {
		buf.ptr == nil ? nil : bytes(lent: buf)
	}

	/// Text Rust returned, copied, then freed.
	static func string(_ buf: Buf) -> Swift.String {
		defer { free(buf) }
		return string(lent: buf)
	}

	static func string(option buf: Buf) -> Swift.String? {
		buf.ptr == nil ? nil : string(buf)
	}

	/// Text C lends a callback for its call, which Rust made UTF-8, copied.
	static func string(lent buf: Buf) -> Swift.String {
		Swift.String(decoding: UnsafeBufferPointer(start: buf.ptr, count: buf.len), as: UTF8.self)
	}

	static func string(lentOption buf: Buf) -> Swift.String? {
		buf.ptr == nil ? nil : string(lent: buf)
	}

	/// A path Rust returned, copied, then freed.
	static func path(_ buf: Buf) -> Swift.String {
		defer { free(buf) }
		return path(lent: buf)
	}

	static func path(option buf: Buf) -> Swift.String? {
		buf.ptr == nil ? nil : path(buf)
	}

	/// A path from Rust, copied, which on Unix may hold bytes no `String` holds: those trap.
	static func path(lent buf: Buf) -> Swift.String {
		var bytes = UnsafeBufferPointer(start: buf.ptr, count: buf.len).makeIterator()
		var parser = Unicode.UTF8.ForwardParser()
		while true {
			switch parser.parseScalar(from: &bytes) {
			case .valid: continue
			case .emptyInput: return string(lent: buf)
			case .error: fatalError("a path from Rust is not UTF-8")
			}
		}
	}

	static func path(lentOption buf: Buf) -> Swift.String? {
		buf.ptr == nil ? nil : path(lent: buf)
	}

	static func scalar(_ code: UInt32) -> Unicode.Scalar {
		guard let scalar = Unicode.Scalar(code) else {
			fatalError("a char is not a Unicode scalar")
		}
		return scalar
	}

	/// Rust's longest `Duration`: `u64::MAX` seconds and 999,999,999 nanoseconds.
	static var longest: Swift.Duration {
		.seconds(Int64.max) * 2 + .seconds(1) + .nanoseconds(999_999_999)
	}

	static func duration(_ value: Swift.Duration) -> CDuration {
		if value < .zero {
			fatalError("a negative duration has no Rust Duration")
		}
		if value > longest {
			fatalError("a duration past Rust's Duration::MAX")
		}
		// In halves, since `components` gives Int64's seconds, and Rust's are UInt64's.
		let half = value / 2
		let (secs, attos) = half.components
		let odd = (value - half * 2).components.attoseconds
		var nanos = (attos * 2 + odd) / 1_000_000_000
		var whole = UInt64(secs) * 2
		if nanos >= 1_000_000_000 {
			nanos -= 1_000_000_000
			whole += 1
		}
		return CDuration(secs: whole, nanos: UInt32(nanos))
	}

	static func duration(_ value: CDuration) -> Swift.Duration {
		// Past Int64's seconds, as Rust's may be, in halves.
		let half = Swift.Duration.seconds(Int64(value.secs >> 1))
		return half * 2 + .seconds(Int64(value.secs & 1)) + .nanoseconds(Int64(value.nanos))
	}

	static func u128(_ value: U128) -> CU128 {
		CU128(lo: value.low, hi: value.high)
	}

	static func u128(_ value: CU128) -> U128 {
		U128(high: value.hi, low: value.lo)
	}

	static func socketAddr(_ addr: SocketAddr) -> CAddr {
		let v6: Bool
		switch addr.ip.count {
		case 4: v6 = false
		case 16: v6 = true
		default: fatalError("an IP address is 4 bytes or 16, not \(addr.ip.count)")
		}
		var c = CAddr()
		withUnsafeMutableBytes(of: &c.ip) { ip in ip.copyBytes(from: addr.ip) }
		c.port = addr.port
		c.v6 = v6
		c.flowinfo = addr.flowinfo
		c.scope_id = addr.scopeId
		return c
	}

	static func socketAddr(_ c: CAddr) -> SocketAddr {
		var c = c
		let ip = withUnsafeBytes(of: &c.ip) { ip in Array(ip.prefix(c.v6 ? 16 : 4)) }
		return SocketAddr(ip: ip, port: c.port, flowinfo: c.flowinfo, scopeId: c.scope_id)
	}

	/// The C array Rust returned, as its items each lifted by `lift`, which takes what
	/// each holds; `free` then frees the array alone.
	static func list<C, T>(_ ptr: UnsafePointer<C>?, _ len: Int, _ lift: (C) throws -> T) rethrows -> [T] {
		guard let ptr else { return [] }
		return try UnsafeBufferPointer(start: ptr, count: len).map(lift)
	}

	/// As `list`, of items Swift cannot copy.
	static func unique<C, T: ~Copyable>(
		_ ptr: UnsafePointer<C>?, _ len: Int, _ lift: (C) throws -> T
	) rethrows -> UniqueArray<T> {
		var items = UniqueArray<T>(minimumCapacity: len)
		for i in 0..<len {
			items.append(try lift(ptr![i]))
		}
		return items
	}

	/// `some` of what `value` holds, borrowed, or else `none`: `Optional.map` takes only a
	/// copyable value.
	@inline(__always)
	static func option<T: ~Copyable, C>(_ value: borrowing T?, _ some: (borrowing T) throws -> C, _ none: C) rethrows -> C {
		switch value {
		case .some(let value): return try some(value)
		case .none: return none
		}
	}

	/// A handle a class owns, or is lent for a call: each call holds it, so `close`, once
	/// a call moves its value out or a lend ends, frees it only once no call does, and a
	/// call after `close` traps. A struct needs none of this: Swift's compiler orders every
	/// call before its move or the end of its lend.
	struct Handle {
		let raw: OpaquePointer?
		/// The holds, one for the owner until closed and one per call, shifted past a bit
		/// set once closed.
		let state: UnsafeMutablePointer<Int>
		/// Null for a handle Rust lends, which it frees itself.
		let free: (@convention(c) (OpaquePointer?) -> Void)?
		let type: StaticString

		static let closed = 1
		static let held = 2

		init(_ raw: OpaquePointer?, _ free: (@convention(c) (OpaquePointer?) -> Void)?, _ type: StaticString) {
			self.raw = raw
			self.free = free
			self.type = type
			state = UnsafeMutablePointer<Int>.allocate(capacity: 1)
			state.initialize(to: Handle.held)
		}

		func hold() -> OpaquePointer? {
			var n = load(state)
			while true {
				if n & Handle.closed != 0 {
					fatalError("\(type) is closed")
				}
				let seen = swap(state, n, n + Handle.held)
				if seen == n {
					return raw
				}
				n = seen
			}
		}

		func release() {
			if add(state, -Handle.held) < Handle.held {
				free?(raw)
			}
		}

		/// Run `body` with the raw handle, held until it returns.
		@inline(__always)
		func with<R: ~Copyable>(_ body: (OpaquePointer?) throws -> R) rethrows -> R {
			let raw = hold()
			defer { release() }
			return try body(raw)
		}

		func close() {
			var n = load(state)
			while n & Handle.closed == 0 {
				let seen = swap(state, n, (n | Handle.closed) - Handle.held)
				if seen == n {
					if (n | Handle.closed) - Handle.held < Handle.held {
						free?(raw)
					}
					return
				}
				n = seen
			}
		}

		/// End a lend, closing the handle, which Rust frees next: a call still holding it
		/// would reach the freed handle, so it traps, as Rust's own check does once a call
		/// borrowed.
		func end() {
			close()
			if load(state) != Handle.closed {
				fatalError("\(type): a call still holds it when its lend ends")
			}
		}

		/// Close it, for its owner's `deinit`, when no call can hold it.
		func drop() {
			close()
			state.deallocate()
		}
	}

	/// Run `body` with the raw handle of `handle`, or null for none, held until it returns.
	@inline(__always)
	static func with<R: ~Copyable>(_ handle: Handle?, _ body: (OpaquePointer?) throws -> R) rethrows -> R {
		guard let handle else { return try body(nil) }
		return try handle.with(body)
	}

	/// What a call lowers nested values into, kept until it returns: copied bytes, arrays,
	/// and the handles it holds.
	final class Arena {
		private var blocks: [UnsafeMutableRawPointer] = []
		private var held: [Handle] = []
		/// Whether bytes lowered now are passed by value, which Rust takes: set once the
		/// receiver, which Rust only reads, is lowered, and unset inside an array.
		var take = false

		func copy(_ bytes: UnsafeRawBufferPointer) -> Buf {
			guard bytes.count > 0 else {
				return Buf(ptr: UnsafePointer(empty), len: 0, owner: nil)
			}
			let block = UnsafeMutableRawPointer.allocate(byteCount: bytes.count, alignment: 1)
			block.copyMemory(from: bytes.baseAddress!, byteCount: bytes.count)
			blocks.append(block)
			return Buf(ptr: block.assumingMemoryBound(to: UInt8.self), len: bytes.count, owner: nil)
		}

		/// `bytes` for Rust to read, copied here, or where `take`, Rust's own: a second owner
		/// of a buffer it returned, else a copy it owns, which it takes without another.
		func buf(_ bytes: Bytes) -> Buf {
			bytes.storage.withUnsafeBytes { raw in
				guard take else { return copy(raw) }
				if let owner = bytes.owner, raw.count > 0 { return owner.share(raw) }
				return Native.copy(view(raw))
			}
		}

		func buf(_ text: Swift.String) -> Buf {
			var text = text
			return text.withUTF8 { utf8 in
				guard take else { return copy(UnsafeRawBufferPointer(utf8)) }
				return Native.copy(view(UnsafeRawBufferPointer(utf8)))
			}
		}

		func buf(_ bytes: Bytes?) -> Buf {
			bytes.map { buf($0) } ?? none
		}

		func buf(_ text: Swift.String?) -> Buf {
			text.map { buf($0) } ?? none
		}

		func hold(_ handle: Handle) -> OpaquePointer? {
			let raw = handle.hold()
			held.append(handle)
			return raw
		}

		func hold(_ handle: Handle?) -> OpaquePointer? {
			handle.map { hold($0) } ?? nil
		}

		/// `items` as a C array, each lowered by `lower`: never null, so an empty one is
		/// told apart from a missing one.
		func list<T, C>(_ items: [T], _ lower: (T) throws -> C) rethrows -> (UnsafePointer<C>, Int) {
			let array = UnsafeMutablePointer<C>.allocate(capacity: Swift.max(items.count, 1))
			blocks.append(UnsafeMutableRawPointer(array))
			// Rust only reads an array's items.
			let take = self.take
			self.take = false
			defer { self.take = take }
			for (i, item) in items.enumerated() {
				(array + i).initialize(to: try lower(item))
			}
			return (UnsafePointer(array), items.count)
		}

		/// As `list`, of items Swift cannot copy, each borrowed.
		func unique<T: ~Copyable, C>(
			_ items: borrowing UniqueArray<T>, _ lower: (borrowing T) throws -> C
		) rethrows -> (UnsafePointer<C>, Int) {
			let array = UnsafeMutablePointer<C>.allocate(capacity: Swift.max(items.count, 1))
			blocks.append(UnsafeMutableRawPointer(array))
			// Rust only reads an array's items.
			let take = self.take
			self.take = false
			defer { self.take = take }
			for i in 0..<items.count {
				(array + i).initialize(to: try lower(items[i]))
			}
			return (UnsafePointer(array), items.count)
		}

		/// Release what the call held, once it returns.
		func finish() {
			for handle in held {
				handle.release()
			}
			for block in blocks {
				block.deallocate()
			}
		}
	}

	/// An array C passes, lowered into `arena`.
	@inline(__always)
	static func vec<C, V>(
		_ list: (UnsafePointer<C>, Int), _ make: (UnsafePointer<C>?, Int, UnsafeMutableRawPointer?) -> V
	) -> V {
		make(list.0, list.1, nil)
	}

	/// What Rust holds of a Swift value: a callback's implementation, or a waker's closure.
	final class Box<T> {
		let value: T

		init(_ value: T) {
			self.value = value
		}

		static func unbox(_ data: UnsafeMutableRawPointer?) -> T {
			Unmanaged<Box<T>>.fromOpaque(data!).takeUnretainedValue().value
		}

		static func release(_ data: UnsafeMutableRawPointer?) {
			Unmanaged<Box<T>>.fromOpaque(data!).release()
		}

		/// `value`, retained until Rust releases it.
		static func give(_ value: T) -> UnsafeMutableRawPointer {
			Unmanaged.passRetained(Box(value)).toOpaque()
		}
	}

	/// `wake` as a waker Rust holds until it drops it, calling it from any thread.
	static func waker(_ wake: @escaping @Sendable () -> Void) -> CWaker {
		CWaker(
			wake: { data in Box<@Sendable () -> Void>.unbox(data)() },
			drop: { data in Box<@Sendable () -> Void>.release(data) },
			data: Box.give(wake))
	}

	/// `body` as the future Rust polls: its first poll starts it in a task of its own,
	/// which Rust's free cancels, and the waker of the latest poll is woken once it returns.
	static func future(_ body: @escaping @Sendable () async -> Void) -> CForeignFuture {
		CForeignFuture(
			poll: { data, waker in Box<Pollable>.unbox(data).poll(waker) },
			free: { data in
				Box<Pollable>.unbox(data).cancel()
				Box<Pollable>.release(data)
			},
			data: Box.give(Pollable(body)))
	}

	/// A Swift async function Rust polls, from any thread, one poll at a time.
	final class Pollable: @unchecked Sendable {
		private let body: @Sendable () async -> Void
		/// Started by the first poll; polls and the free never overlap, so only they touch it.
		private var task: Task<Void, Never>?
		/// 1 once `body` returned, changed only atomically.
		private let done = UnsafeMutablePointer<Swift.Int>.allocate(capacity: 1)
		/// The latest poll's waker, boxed, or 0: changed only atomically, so whoever takes it
		/// out of the slot drops it.
		private let waker = UnsafeMutablePointer<Swift.Int>.allocate(capacity: 1)

		init(_ body: @escaping @Sendable () async -> Void) {
			self.body = body
			done.initialize(to: 0)
			waker.initialize(to: 0)
		}

		deinit {
			Pollable.drop(exchange(waker, 0))
			done.deallocate()
			waker.deallocate()
		}

		func poll(_ given: CWaker) -> Swift.Bool {
			if load(done) == 1 {
				given.drop?(given.data)
				return true
			}
			Pollable.drop(exchange(waker, Swift.Int(bitPattern: Box.give(given))))
			if task == nil {
				task = Task { [self] in
					// Dropped before it started, as a Rust future dropped unpolled, it never runs.
					guard !Task.isCancelled else { return }
					await body()
					store(done, 1)
					// A poll storing its waker after this sees `done` and takes it back.
					let taken = exchange(waker, 0)
					if taken != 0 {
						let w = Box<CWaker>.unbox(UnsafeMutableRawPointer(bitPattern: taken))
						w.wake?(w.data)
						Pollable.drop(taken)
					}
				}
			}
			if load(done) == 1 {
				Pollable.drop(exchange(waker, 0))
				return true
			}
			return false
		}

		func cancel() {
			task?.cancel()
			Pollable.drop(exchange(waker, 0))
		}

		/// Drop the boxed waker `slot` held, if any.
		private static func drop(_ slot: Swift.Int) {
			guard let data = UnsafeMutableRawPointer(bitPattern: slot) else { return }
			let w = Box<CWaker>.unbox(data)
			Box<CWaker>.release(data)
			w.drop?(w.data)
		}
	}

	/// A waker Rust returned, which Rust drops once Swift lets go of it.
	final class Returned: @unchecked Sendable {
		let raw: CWaker

		init(_ raw: CWaker) {
			self.raw = raw
		}

		deinit {
			raw.drop?(raw.data)
		}
	}

	/// `raw` as a closure waking it through Rust, which holds it until released.
	static func returned(_ raw: CWaker) -> @Sendable () -> Void {
		let owner = Returned(raw)
		return { owner.raw.wake?(owner.raw.data) }
	}

	/// The waker of a future's first poll, which wakes nothing: a poll that pends polls
	/// again with its signal's.
	static var idle: CWaker { CWaker(wake: nil, drop: nil, data: nil) }

	/// What a future parks on: it latches a wake that lands between a poll that pends and
	/// the park, so the next poll sees it.
	final class Signal: @unchecked Sendable {
		static let idle = 0
		static let woken = 1
		static let parked = 2

		/// Which of those it is, changed only atomically.
		private let state = UnsafeMutablePointer<Swift.Int>.allocate(capacity: 1)
		/// The parked task, written before `state` is `parked` and read after.
		private var parked: UnsafeContinuation<Void, Never>?

		init() {
			state.initialize(to: Signal.idle)
		}

		deinit {
			state.deallocate()
		}

		/// This as a waker: given, holding a reference Rust drops, the first time; lent,
		/// which Rust reuses as the same waker, after.
		func waker(given: Bool) -> CWaker {
			let data = given ? Unmanaged.passRetained(self).toOpaque() : Unmanaged.passUnretained(self).toOpaque()
			return CWaker(
				wake: { data in Unmanaged<Signal>.fromOpaque(data!).takeUnretainedValue().wake() },
				drop: { data in Unmanaged<Signal>.fromOpaque(data!).release() },
				data: data)
		}

		/// Resume the parked task, or else latch the wake for the next park. The task
		/// resumes on its own executor, never inside the wake.
		func wake() {
			if exchange(state, Signal.woken) == Signal.parked {
				parked!.resume()
			}
		}

		/// Forget any wake before the next poll, which sees whatever it was for.
		func reset() {
			store(state, Signal.idle)
		}

		/// Suspend until a wake since `reset`, at once if one already landed.
		func park() async {
			await withUnsafeContinuation { (continuation: UnsafeContinuation<Void, Never>) in
				parked = continuation
				if swap(state, Signal.idle, Signal.parked) != Signal.idle {
					continuation.resume()
				}
			}
		}
	}

	/// Await the future `raw`: poll it until `poll` returns its output, then free it. A
	/// cancelled task frees it unfinished, which cancels it in Rust and loses nothing, and
	/// throws `CancellationError`. Its first poll makes no signal, which only a future
	/// that pends needs.
	static func run<T: ~Copyable>(
		_ raw: OpaquePointer?, _ free: @convention(c) (OpaquePointer?) -> Void,
		_ poll: (OpaquePointer?, CWaker) throws -> T?
	) async throws -> T {
		defer { free(raw) }
		// A task cancelled already takes nothing, even what is ready.
		try Task.checkCancellation()
		if let output = try poll(raw, idle) {
			return output
		}
		let signal = Signal()
		var given = false
		// The handler's operation returns only a copyable result, so it leaves through this.
		var output: T? = nil
		try await withTaskCancellationHandler {
			while true {
				signal.reset()
				try Task.checkCancellation()
				if let ready = try poll(raw, signal.waker(given: !given)) {
					output = consume ready
					return
				}
				given = true
				await signal.park()
			}
		} onCancel: {
			signal.wake()
		}
		return output.take()!
	}
}
