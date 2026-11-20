// The runtime every generated Swift file includes, after `Native` names the export's C
// types and functions: the handles classes own, the arena a call lowers its values into,
// bytes and text across the C ABI, and the signal a future parks on. Each helper that
// returns what its body does takes a noncopyable result, an object's struct.

/// An IPv4 address, as Rust's `Ipv4Addr` holds it.
public struct Ipv4Addr: Hashable, Sendable {
	/// Its 32 bits, the first octet highest, as Rust's `to_bits` gives them.
	public var bits: UInt32

	public init(bits: UInt32) {
		self.bits = bits
	}

	/// The address `a.b.c.d`.
	public init(_ a: UInt8, _ b: UInt8, _ c: UInt8, _ d: UInt8) {
		bits = UInt32(a) << 24 | UInt32(b) << 16 | UInt32(c) << 8 | UInt32(d)
	}

	/// The address of `octets`, in network order, or nil unless it holds exactly 4.
	public init?(octets: some Swift.Collection<UInt8>) {
		guard octets.count == 4 else { return nil }
		bits = octets.reduce(0) { $0 << 8 | UInt32($1) }
	}

	/// Its 4 octets, in network order.
	public var octets: [UInt8] {
		[24, 16, 8, 0].map { UInt8(truncatingIfNeeded: bits >> $0) }
	}
}

/// An IPv6 address, as Rust's `Ipv6Addr` holds it.
public struct Ipv6Addr: Hashable, Sendable {
	/// Its 128 bits, the first octet highest, as Rust's `to_bits` gives them.
	public var bits: U128

	public init(bits: U128) {
		self.bits = bits
	}

	/// The address of `octets`, in network order, or nil unless it holds exactly 16.
	public init?(octets: some Swift.Collection<UInt8>) {
		guard octets.count == 16 else { return nil }
		let high = octets.prefix(8).reduce(0) { $0 << 8 | UInt64($1) }
		bits = U128(high: high, low: octets.dropFirst(8).reduce(0) { $0 << 8 | UInt64($1) })
	}

	/// Its 16 octets, in network order.
	public var octets: [UInt8] {
		[bits.high, bits.low].flatMap { half in stride(from: 56, through: 0, by: -8).map { UInt8(truncatingIfNeeded: half >> $0) } }
	}
}

/// An IP address of either family, as Rust's `IpAddr`.
public enum IpAddr: Hashable, Sendable {
	case v4(Ipv4Addr)
	case v6(Ipv6Addr)
}

/// An IPv4 socket address, as Rust's `SocketAddrV4`.
public struct SocketAddrV4: Hashable, Sendable {
	public var ip: Ipv4Addr
	public var port: UInt16

	public init(ip: Ipv4Addr, port: UInt16) {
		self.ip = ip
		self.port = port
	}
}

/// An IPv6 socket address, as Rust's `SocketAddrV6`.
public struct SocketAddrV6: Hashable, Sendable {
	public var ip: Ipv6Addr
	public var port: UInt16
	/// Its flow label.
	public var flowinfo: UInt32
	/// Its scope, as an interface index.
	public var scopeId: UInt32

	public init(ip: Ipv6Addr, port: UInt16, flowinfo: UInt32 = 0, scopeId: UInt32 = 0) {
		self.ip = ip
		self.port = port
		self.flowinfo = flowinfo
		self.scopeId = scopeId
	}
}

/// A socket address of either family, as Rust's `SocketAddr`.
public enum SocketAddr: Hashable, Sendable {
	case v4(SocketAddrV4)
	case v6(SocketAddrV6)

	/// `ip` at `port`, with no flow label or scope for IPv6, as Rust's `SocketAddr::new`.
	public init(ip: IpAddr, port: UInt16) {
		switch ip {
		case .v4(let ip): self = .v4(SocketAddrV4(ip: ip, port: port))
		case .v6(let ip): self = .v6(SocketAddrV6(ip: ip, port: port))
		}
	}

	/// Its IP, of its family.
	public var ip: IpAddr {
		switch self {
		case .v4(let addr): return .v4(addr.ip)
		case .v6(let addr): return .v6(addr.ip)
		}
	}

	/// Its port.
	public var port: UInt16 {
		switch self {
		case .v4(let addr): return addr.port
		case .v6(let addr): return addr.port
		}
	}
}

/// A Rust `Duration`: whole seconds and the nanoseconds past them, below a second, so it
/// holds every value Rust's does and no other.
public struct RustDuration: Hashable, Comparable, Sendable {
	public let secs: UInt64
	/// Below 1,000,000,000.
	public let nanos: UInt32

	init(_c: Native.CDuration) {
		secs = _c.secs
		nanos = _c.nanos
	}

	/// `secs` seconds and `nanos` nanoseconds, throwing unless `nanos` is below a second.
	public init(secs: UInt64, nanos: UInt32) throws {
		guard nanos < 1_000_000_000 else {
			throw CheckError(function: "RustDuration(secs:nanos:)", message: "\(nanos) nanoseconds are a second or more")
		}
		self.init(_c: Native.CDuration(secs: secs, nanos: nanos))
	}

	/// `secs` whole seconds.
	public static func seconds(_ secs: UInt64) -> RustDuration {
		RustDuration(_c: Native.CDuration(secs: secs, nanos: 0))
	}

	/// `millis` milliseconds, as Rust's `from_millis`.
	public static func milliseconds(_ millis: UInt64) -> RustDuration {
		RustDuration(_c: Native.CDuration(secs: millis / 1000, nanos: UInt32(millis % 1000) * 1_000_000))
	}

	/// `micros` microseconds, as Rust's `from_micros`.
	public static func microseconds(_ micros: UInt64) -> RustDuration {
		RustDuration(_c: Native.CDuration(secs: micros / 1_000_000, nanos: UInt32(micros % 1_000_000) * 1000))
	}

	/// `nanos` nanoseconds, as Rust's `from_nanos`.
	public static func nanoseconds(_ nanos: UInt64) -> RustDuration {
		RustDuration(_c: Native.CDuration(secs: nanos / 1_000_000_000, nanos: UInt32(nanos % 1_000_000_000)))
	}

	/// `duration`, throwing for one that is negative, has a part below a nanosecond, or is
	/// past Rust's longest, `UInt64.max` seconds and 999,999,999 nanoseconds.
	public init(_ duration: Swift.Duration) throws {
		let what = "RustDuration(_:)"
		guard duration >= .zero else {
			throw CheckError(function: what, message: "a negative duration")
		}
		guard duration <= RustDuration.longest else {
			throw CheckError(function: what, message: "a duration past Rust's longest")
		}
		// In halves, since `components` gives Int64's seconds, and Rust's are UInt64's.
		let half = duration / 2
		let (secs, attos) = half.components
		let odd = (duration - half * 2).components.attoseconds
		let (whole, part) = (attos * 2 + odd).quotientAndRemainder(dividingBy: 1_000_000_000)
		guard part == 0 else {
			throw CheckError(function: what, message: "a duration with a part below a nanosecond")
		}
		// `whole` is below two seconds' nanoseconds.
		let carry = whole >= 1_000_000_000
		self.init(_c: Native.CDuration(secs: UInt64(secs) * 2 + (carry ? 1 : 0), nanos: UInt32(carry ? whole - 1_000_000_000 : whole)))
	}

	/// Rust's longest `Duration`, which `Swift.Duration` holds.
	static var longest: Swift.Duration {
		.seconds(Int64.max) * 2 + .seconds(1) + .nanoseconds(999_999_999)
	}

	/// It as a `Swift.Duration`, which holds every Rust `Duration`.
	public var duration: Swift.Duration {
		// Past Int64's seconds, as Rust's may be, in halves.
		Swift.Duration.seconds(Int64(secs >> 1)) * 2 + .seconds(Int64(secs & 1)) + .nanoseconds(Int64(nanos))
	}

	public static func < (a: RustDuration, b: RustDuration) -> Bool {
		(a.secs, a.nanos) < (b.secs, b.nanos)
	}
}

/// An OS path, as Rust's `PathBuf` holds it: its bytes, which on Unix need not be UTF-8,
/// shared with Rust's buffer as `Bytes` are.
public struct PathBuf: Hashable, Sendable, Swift.ExpressibleByStringLiteral {
	/// Its bytes as the OS holds them on Unix, or WTF-8 on Windows: UTF-8 for any path
	/// that has a UTF-8 form.
	public let bytes: Bytes

	/// The path of the OS's `bytes`, which on Unix may be any.
	public init(bytes: Bytes) {
		self.bytes = bytes
	}

	/// The path `string` names.
	public init(_ string: Swift.String) {
		self.init(bytes: Bytes(Data(string.utf8)))
	}

	public init(stringLiteral string: Swift.String) {
		self.init(string)
	}

	/// Its text, or nil if its bytes are not UTF-8.
	public var string: Swift.String? {
		var iterator = bytes.makeIterator()
		var parser = Unicode.UTF8.ForwardParser()
		while true {
			switch parser.parseScalar(from: &iterator) {
			case .valid: continue
			case .emptyInput: return Swift.String(decoding: bytes, as: UTF8.self)
			case .error: return nil
			}
		}
	}
}

/// Bytes as Rust holds them: Rust's stay in the buffer it returned, shared without a copy
/// by every `Bytes` and slice reading them and passed back to Rust uncopied, and Swift's are
/// a `Data`. Rust's are never lent as a `Data`, so nothing writes into the buffer Rust shares.
public struct Bytes: Hashable, Sendable, RandomAccessCollection {
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
		let slice = storage[bounds]
		// `Data` may copy a short slice inline: those bytes are Swift's.
		return Bytes(storage: slice, owner: owner.flatMap { $0.holds(slice) ? $0 : nil })
	}

	/// Run `body` with the bytes in place, as a span the compiler keeps it from keeping.
	public func withSpan<R: ~Copyable>(_ body: (Swift.Span<Swift.UInt8>) throws -> R) rethrows -> R {
		var result: R? = nil
		try storage.withUnsafeBytes { raw in
			result = try body(Swift.Span(_unsafeElements: raw.assumingMemoryBound(to: Swift.UInt8.self)))
		}
		return result.take()!
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

/// Rust's `NonZero<T>`: an integer that is never zero.
public struct NonZero<T: Swift.FixedWidthInteger & Sendable>: Hashable, Comparable, Sendable, CustomStringConvertible {
	/// The integer, never zero, as Rust's `get`.
	public let value: T

	init(_unchecked value: T) {
		self.value = value
	}

	/// `value`, throwing for a zero.
	public init(_ value: T) throws {
		guard value != 0 else {
			throw CheckError(function: "NonZero(_:)", message: "a NonZero is zero")
		}
		self.value = value
	}

	public static func < (a: NonZero, b: NonZero) -> Swift.Bool { a.value < b.value }

	public var description: Swift.String { value.description }
}

/// A call Rust's rules refuse, thrown before any Rust runs: a handle borrowed in a way
/// the borrow checker forbids beside another call's, used once its value moved out or its
/// lend ended, an item called on a library whose target lacks it, or a value Rust's type
/// cannot hold given a runtime type's initializer, such as a zero given `NonZero`'s.
public struct CheckError: Swift.Error, CustomStringConvertible, Sendable {
	/// The function called, by its C name, or the runtime initializer.
	public let function: Swift.String
	/// What broke.
	public let message: Swift.String

	init(function: Swift.String, message: Swift.String) {
		self.function = function
		self.message = message
	}

	public var description: Swift.String { "\(function): \(message)" }
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
		init() {}

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

	/// `span` as a view C reads during `body`, without a copy.
	@inline(__always)
	static func view<R: ~Copyable>(_ span: borrowing Swift.Span<Swift.UInt8>, _ body: (Buf) throws -> R) rethrows -> R {
		try span.withUnsafeBytes { raw in try body(view(raw)) }
	}

	@inline(__always)
	static func view<R: ~Copyable>(_ span: borrowing Swift.Span<Swift.UInt8>?, _ body: (Buf) throws -> R) rethrows -> R {
		switch span {
		case .some(let span): return try view(span, body)
		case .none: return try body(none)
		}
	}

	/// `span` as the array `make` makes of its items, which C reads during `body` without a
	/// copy: never null, so no items read as present.
	@inline(__always)
	static func view<T, L, R: ~Copyable>(
		_ span: borrowing Swift.Span<T>, _ make: (UnsafePointer<T>?, Int, UnsafeMutableRawPointer?) -> L,
		_ body: (L) throws -> R
	) rethrows -> R {
		try span.withUnsafeBufferPointer { items in
			let ptr = items.baseAddress ?? UnsafePointer(bitPattern: MemoryLayout<T>.alignment)!
			return try body(make(ptr, items.count, nil))
		}
	}

	@inline(__always)
	static func view<T, L, R: ~Copyable>(
		_ span: borrowing Swift.Span<T>?, _ make: (UnsafePointer<T>?, Int, UnsafeMutableRawPointer?) -> L,
		_ body: (L) throws -> R
	) rethrows -> R {
		switch span {
		case .some(let span): return try view(span, make, body)
		case .none: return try body(make(nil, 0, nil))
		}
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

		/// Whether `data`'s bytes lie in this buffer rather than in a copy.
		func holds(_ data: Data) -> Swift.Bool {
			guard let ptr = buf.ptr else { return false }
			return data.withUnsafeBytes { raw in
				guard let base = raw.baseAddress else { return false }
				let start = UnsafeRawPointer(ptr)
				return base >= start && base + raw.count <= start + buf.len
			}
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
			deallocator: .custom { _, _ in Swift.withExtendedLifetime(owner) {} })
		// `Data` copies a few bytes inline, freeing Rust's buffer at once: those are Swift's.
		return Bytes(storage: storage, owner: owner.holds(storage) ? owner : nil)
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

	/// A path Rust takes during `body`, as its bytes are.
	@inline(__always)
	static func take<R: ~Copyable>(_ path: PathBuf, _ body: (Buf) throws -> R) rethrows -> R {
		try take(path.bytes, body)
	}

	@inline(__always)
	static func take<R: ~Copyable>(_ path: PathBuf?, _ body: (Buf) throws -> R) rethrows -> R {
		try take(path?.bytes, body)
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

	/// A path Rust returned, its buffer shared as `Bytes` share it.
	static func path(_ buf: Buf) -> PathBuf {
		PathBuf(bytes: bytes(buf))
	}

	static func path(option buf: Buf) -> PathBuf? {
		buf.ptr == nil ? nil : path(buf)
	}

	/// A path C lends a callback for its call, copied.
	static func path(lent buf: Buf) -> PathBuf {
		PathBuf(bytes: bytes(lent: buf))
	}

	static func path(lentOption buf: Buf) -> PathBuf? {
		buf.ptr == nil ? nil : path(lent: buf)
	}

	/// `path`'s bytes in a buffer Rust owns, as what a closure returns.
	static func owned(_ path: PathBuf) -> Buf {
		view(path.bytes) { copy($0) }
	}

	static func scalar(_ code: UInt32) -> Unicode.Scalar {
		guard let scalar = Unicode.Scalar(code) else {
			fatalError("a char is not a Unicode scalar")
		}
		return scalar
	}

	static func u128(_ value: U128) -> CU128 {
		CU128(lo: value.low, hi: value.high)
	}

	static func u128(_ value: CU128) -> U128 {
		U128(high: value.hi, low: value.lo)
	}

	static func duration(_ value: RustDuration) -> CDuration {
		CDuration(secs: value.secs, nanos: value.nanos)
	}

	static func duration(_ value: CDuration) -> RustDuration {
		RustDuration(_c: value)
	}

	// Each address as its C struct, in network order, and back: every value of either
	// is one of the other.

	static func addr(_ ip: Ipv4Addr) -> CIpv4Addr {
		var c = CIpv4Addr()
		withUnsafeMutableBytes(of: &c.octets) { $0.storeBytes(of: ip.bits.bigEndian, as: UInt32.self) }
		return c
	}

	static func addr(_ c: CIpv4Addr) -> Ipv4Addr {
		Ipv4Addr(bits: UInt32(bigEndian: withUnsafeBytes(of: c.octets) { $0.loadUnaligned(as: UInt32.self) }))
	}

	static func addr(_ ip: Ipv6Addr) -> CIpv6Addr {
		var c = CIpv6Addr()
		withUnsafeMutableBytes(of: &c.octets) { octets in
			octets.storeBytes(of: ip.bits.high.bigEndian, as: UInt64.self)
			octets.storeBytes(of: ip.bits.low.bigEndian, toByteOffset: 8, as: UInt64.self)
		}
		return c
	}

	static func addr(_ c: CIpv6Addr) -> Ipv6Addr {
		withUnsafeBytes(of: c.octets) { octets in
			Ipv6Addr(
				bits: U128(
					high: UInt64(bigEndian: octets.loadUnaligned(as: UInt64.self)),
					low: UInt64(bigEndian: octets.loadUnaligned(fromByteOffset: 8, as: UInt64.self))))
		}
	}

	static func addr(_ ip: IpAddr) -> CIpAddr {
		var c = CIpAddr()
		switch ip {
		case .v4(let ip): c.ip.v4 = addr(ip)
		case .v6(let ip):
			c.v6 = true
			c.ip.v6 = addr(ip)
		}
		return c
	}

	static func addr(_ c: CIpAddr) -> IpAddr {
		c.v6 ? .v6(addr(c.ip.v6)) : .v4(addr(c.ip.v4))
	}

	static func addr(_ addr: SocketAddrV4) -> CSocketAddrV4 {
		CSocketAddrV4(ip: Native.addr(addr.ip), port: addr.port)
	}

	static func addr(_ c: CSocketAddrV4) -> SocketAddrV4 {
		SocketAddrV4(ip: addr(c.ip), port: c.port)
	}

	static func addr(_ addr: SocketAddrV6) -> CSocketAddrV6 {
		CSocketAddrV6(ip: Native.addr(addr.ip), port: addr.port, flowinfo: addr.flowinfo, scope_id: addr.scopeId)
	}

	static func addr(_ c: CSocketAddrV6) -> SocketAddrV6 {
		SocketAddrV6(ip: addr(c.ip), port: c.port, flowinfo: c.flowinfo, scopeId: c.scope_id)
	}

	static func addr(_ addr: SocketAddr) -> CSocketAddr {
		var c = CSocketAddr()
		switch addr {
		case .v4(let addr): c.addr.v4 = Native.addr(addr)
		case .v6(let addr):
			c.v6 = true
			c.addr.v6 = Native.addr(addr)
		}
		return c
	}

	static func addr(_ c: CSocketAddr) -> SocketAddr {
		c.v6 ? .v6(addr(c.addr.v6)) : .v4(addr(c.addr.v4))
	}

	/// The C array Rust returned, as its items each lifted by `lift`, which takes what
	/// each holds; `free` then frees the array alone.
	static func list<C, T>(_ ptr: UnsafePointer<C>?, _ len: Int, _ lift: (C) throws -> T) rethrows -> [T] {
		guard let ptr else { return [] }
		return try UnsafeBufferPointer(start: ptr, count: len).map(lift)
	}

	/// The C array of numbers Rust returned, copied whole.
	static func block<T>(_ ptr: UnsafePointer<T>?, _ len: Int) -> [T] {
		guard let ptr else { return [] }
		return Array(UnsafeBufferPointer(start: ptr, count: len))
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
	///
	/// Once calls contend for a handle's count, each holds it in a slot of `slots` instead,
	/// so threads sharing a handle write nothing in common. A close claims it closed first,
	/// then counts each hold it finds there, which lets go through the count, so the last
	/// to let go frees it: ffi/tests/hold.rs models this under loom.
	struct Handle {
		let raw: OpaquePointer?
		/// The holds, one for the owner until closed and one per call, shifted past a bit
		/// set once closed and one set once calls contended for them, after which they hold
		/// it in `slots`: one word, so a call reads all three at once, and below `held` only
		/// once no hold is left.
		let state: UnsafeMutablePointer<Int>
		/// Null for a handle Rust lends, which it frees itself.
		let free: (@convention(c) (OpaquePointer?) -> Void)?
		let type: StaticString

		static var closed: Int { 1 }
		static var bias: Int { 2 }
		static var held: Int { 4 }
		/// Set in a slot whose hold a close counted.
		static var revoked: Int { 1 }
		/// A cache line of rows to a column, which the thread's stack picks, and a row per
		/// handle, with a line between columns: two threads in one column, or two handles in
		/// one row on one thread, use the count.
		static var rowBits: Int { 3 }
		static var columnBits: Int { 8 }
		static var stride: Int { 2 << rowBits }
		/// Only ever touched atomically.
		nonisolated(unsafe) static let slots: UnsafeMutablePointer<Int> = {
			let count = stride << columnBits
			return UnsafeMutableRawPointer.allocate(byteCount: count * Swift.MemoryLayout<Int>.stride, alignment: 64)
				.initializeMemory(as: Int.self, repeating: 0, count: count)
		}()

		init(_ raw: OpaquePointer?, _ free: (@convention(c) (OpaquePointer?) -> Void)?, _ type: StaticString) {
			self.raw = raw
			self.free = free
			self.type = type
			state = UnsafeMutablePointer<Int>.allocate(capacity: 1)
			state.initialize(to: Handle.held)
		}

		/// What a slot holds for this handle: its state's address, which no other live
		/// handle has, and whose low bit is clear for `revoked`.
		private var key: Int { Int(bitPattern: state) }

		/// This handle's row of `slots`.
		private var row: Int {
			let hash = (UInt64(UInt(bitPattern: key)) >> 4) &* 0x9E37_79B9_7F4A_7C15
			return Int(truncatingIfNeeded: hash >> UInt64(64 - Handle.rowBits))
		}

		/// Hold it for a call, until `release` takes what this returns. Traps once closed.
		func hold() -> Int {
			guard let slot = held() else {
				closed()
			}
			return slot
		}

		/// Trap for a call of a closed handle: out of line, so a caller's inlined call stays
		/// small enough to inline.
		@inline(never)
		func closed() -> Never {
			fatalError("\(type) is closed")
		}

		/// The error a call of `what` throws once it is closed, out of line as `closed` is.
		@inline(never)
		func closed(_ what: StaticString) -> CheckError {
			CheckError(function: "\(what)", message: "\(type) is closed")
		}

		/// Hold it for a call, until `release` takes what this returns, unless it is closed.
		@inline(__always)
		func held() -> Int? {
			let n = load(state)
			if n & (Handle.closed | Handle.bias) == 0 && swap(state, n, n + Handle.held) == n {
				return -1
			}
			return contend()
		}

		/// What `listed` returns when the table's slot was taken, so the call holds the count.
		private static let unlisted = -2

		/// Hold it in this thread's slot of `slots`: the slot, `unlisted` if another handle
		/// or thread has it, or nil once closed.
		@inline(never)
		private func listed() -> Int? {
			var here = 0
			let stack = withUnsafeMutablePointer(to: &here) { UInt64(UInt(bitPattern: $0)) }
			let hash = (stack >> 16) &* 0x9E37_79B9_7F4A_7C15
			let slot = Int(truncatingIfNeeded: hash >> UInt64(64 - Handle.columnBits)) * Handle.stride + row
			if swapSC(Handle.slots + slot, 0, key) != 0 {
				return Handle.unlisted
			}
			if loadSC(state) & Handle.closed == 0 {
				return slot
			}
			// A close claimed it meanwhile, and may have counted this hold.
			if swapSC(Handle.slots + slot, key, 0) != key {
				storeSC(Handle.slots + slot, 0)
				release(-1)
			}
			return nil
		}

		/// `held` once calls contend for it: in the table, unless another handle or thread
		/// has its slot, then in the count, after which later calls go to the table.
		@inline(never)
		private func contend() -> Int? {
			if load(state) & Handle.bias != 0 {
				let slot = listed()
				if slot != Handle.unlisted {
					return slot
				}
			}
			var n = load(state)
			while true {
				if n & Handle.closed != 0 {
					return nil
				}
				let seen = swap(state, n, n + Handle.held)
				if seen == n {
					n += Handle.held
					break
				}
				n = seen
			}
			while n & Handle.bias == 0 {
				let seen = swap(state, n, n | Handle.bias)
				if seen == n {
					break
				}
				n = seen
			}
			return -1
		}

		@inline(__always)
		func release(_ slot: Int) {
			if slot >= 0 {
				release(listed: slot)
			} else if add(state, -Handle.held) < Handle.held {
				freed()
			}
		}

		/// `release` of a hold in `slots`, out of line, as `closed` is.
		@inline(never)
		func release(listed slot: Int) {
			if exchange(Handle.slots + slot, 0) != key && add(state, -Handle.held) < Handle.held {
				freed()
			}
		}

		/// Free it once its last hold lets go: out of line, as `closed` is.
		@inline(never)
		func freed() {
			free?(raw)
		}

		/// Run `body` with the raw handle, held until it returns.
		@inline(__always)
		func with<R: ~Copyable>(_ body: (OpaquePointer?) throws -> R) rethrows -> R {
			let slot = hold()
			defer { release(slot) }
			return try body(raw)
		}

		/// Run `body` with the raw handle, held until it returns, or throw for a call of
		/// `what` if it is closed.
		@inline(__always)
		func with<R: ~Copyable>(_ what: StaticString, _ body: (OpaquePointer?) throws -> R) throws -> R {
			guard let slot = held() else {
				throw closed(what)
			}
			defer { release(slot) }
			return try body(raw)
		}

		func close() {
			var n = load(state)
			while true {
				if n & Handle.closed != 0 {
					return
				}
				let seen = swapSC(state, n, n | Handle.closed)
				if seen == n {
					break
				}
				n = seen
			}
			// Claimed closed, so each call holding it in the table did so before: count them.
			if n & Handle.bias != 0 {
				var slot = row
				while slot < Handle.stride << Handle.columnBits {
					if loadSC(Handle.slots + slot) == key {
						_ = add(state, Handle.held)
						if swapSC(Handle.slots + slot, key, key | Handle.revoked) != key {
							_ = add(state, -Handle.held)
						}
					}
					slot += Handle.stride
				}
			}
			release(-1)
		}

		/// End a lend, closing the handle, which Rust frees next: a call still holding it
		/// would reach the freed handle, so it traps, as Rust's own check does once a call
		/// borrowed.
		func end() {
			close()
			if load(state) & ~Handle.bias != Handle.closed {
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

	/// As `with`, throwing for a call of `what` if `handle` is closed.
	@inline(__always)
	static func with<R: ~Copyable>(
		_ what: StaticString, _ handle: Handle?, _ body: (OpaquePointer?) throws -> R
	) throws -> R {
		guard let handle else { return try body(nil) }
		return try handle.with(what, body)
	}

	/// What a call of `what` lowers nested values into, kept until it returns: copied
	/// bytes, arrays, and the handles it holds.
	final class Arena {
		private var blocks: [UnsafeMutableRawPointer] = []
		private var held: [(Handle, Int)] = []
		/// Each buffer given Rust to take, which the call frees unless it reached C.
		private var given: [Buf] = []
		private var reached = false
		private let what: StaticString

		init(_ what: StaticString) {
			self.what = what
		}

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
				if let owner = bytes.owner, raw.count > 0 { return give(owner.share(raw)) }
				return give(Native.copy(view(raw)))
			}
		}

		func buf(_ text: Swift.String) -> Buf {
			var text = text
			return text.withUTF8 { utf8 in
				guard take else { return copy(UnsafeRawBufferPointer(utf8)) }
				return give(Native.copy(view(UnsafeRawBufferPointer(utf8))))
			}
		}

		func buf(_ path: PathBuf) -> Buf {
			buf(path.bytes)
		}

		func buf(_ path: PathBuf?) -> Buf {
			buf(path?.bytes)
		}

		private func give(_ buf: Buf) -> Buf {
			given.append(buf)
			return buf
		}

		/// `result`, what the call's C function returned: it ran, so Rust owns every buffer
		/// given, taken or freed as the call was refused.
		@inline(__always)
		func ran<T>(_ result: T) -> T {
			reached = true
			return result
		}

		func buf(_ bytes: Bytes?) -> Buf {
			bytes.map { buf($0) } ?? none
		}

		func buf(_ text: Swift.String?) -> Buf {
			text.map { buf($0) } ?? none
		}

		/// The handle of a `Copy` object, which nothing closes, held until the call returns.
		func hold(_ handle: Handle) -> OpaquePointer? {
			held.append((handle, handle.hold()))
			return handle.raw
		}

		func hold(_ handle: Handle?) -> OpaquePointer? {
			handle.map { hold($0) } ?? nil
		}

		/// The handle of an object a move may have closed, held until the call returns, or
		/// a throw for the call if it is closed.
		func held(_ handle: Handle) throws -> OpaquePointer? {
			guard let slot = handle.held() else {
				throw handle.closed(what)
			}
			held.append((handle, slot))
			return handle.raw
		}

		func held(_ handle: Handle?) throws -> OpaquePointer? {
			try handle.map { try held($0) } ?? nil
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

		/// `items`, numbers, as a C array copied whole: never null, as `list`'s.
		func block<T>(_ items: [T]) -> (UnsafePointer<T>, Int) {
			let array = UnsafeMutablePointer<T>.allocate(capacity: Swift.max(items.count, 1))
			blocks.append(UnsafeMutableRawPointer(array))
			_ = UnsafeMutableBufferPointer(start: array, count: items.count).initialize(from: items)
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

		/// Release what the call held, once it returns, and, if a lowering threw before the
		/// call reached C, each buffer given Rust.
		func finish() {
			for (handle, slot) in held {
				handle.release(slot)
			}
			if !reached {
				for buf in given {
					Native.free(buf)
				}
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

	/// What a future parks on: it latches a wake that lands between a poll that pends and
	/// the park, so the next poll sees it.
	final class Signal: @unchecked Sendable {
		static var idle: Int { 0 }
		static var woken: Int { 1 }
		static var parked: Int { 2 }

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
		nonisolated(nonsending) func park() async {
			await withUnsafeContinuation { (continuation: UnsafeContinuation<Void, Never>) in
				parked = continuation
				if swap(state, Signal.idle, Signal.parked) != Signal.idle {
					continuation.resume()
				}
			}
		}
	}

	/// The call that starts a future and polls it once, whose output, if ready, has the
	/// type `T` names.
	static func start<T: ~Copyable>(_: T.Type, _ call: () throws -> T?) rethrows -> T? {
		try call()
	}

	/// Await the future `raw` an eager call made, unpolled, as `run` does, but take nothing,
	/// even what is ready, if the task is cancelled already, and poll first with no waker.
	nonisolated(nonsending) static func made<T: ~Copyable>(
		_ raw: OpaquePointer?, _ free: @convention(c) (OpaquePointer?) -> Void,
		_ poll: (OpaquePointer?, CWaker) throws -> T?
	) async throws -> T {
		do {
			try Task.checkCancellation()
			if let output = try poll(raw, CWaker(wake: nil, drop: nil, data: nil)) {
				free(raw)
				return output
			}
		} catch {
			free(raw)
			throw error
		}
		return try await run(raw, free, poll)
	}

	/// Await the future `raw`, whose first poll pended: poll it until `poll` returns its
	/// output, then free it. A cancelled task frees it unfinished, which cancels it in Rust
	/// and loses nothing, and throws `CancellationError`.
	nonisolated(nonsending) static func run<T: ~Copyable>(
		_ raw: OpaquePointer?, _ free: @convention(c) (OpaquePointer?) -> Void,
		_ poll: (OpaquePointer?, CWaker) throws -> T?
	) async throws -> T {
		defer { free(raw) }
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
