// api.swift
public struct Ipv4Addr: Hashable, Sendable {
	public var bits: UInt32
	@inlinable public init(bits: UInt32)
	@inlinable public init(_ a: UInt8, _ b: UInt8, _ c: UInt8, _ d: UInt8)
	@inlinable public init?(octets: some Swift.Collection<UInt8>)
	@inlinable public var octets: [UInt8] { get }
}
public struct Ipv6Addr: Hashable, Sendable {
	public var bits: U128
	@inlinable public init(bits: U128)
	@inlinable public init?(octets: some Swift.Collection<UInt8>)
	@inlinable public var octets: [UInt8] { get }
}
public enum IpAddr: Hashable, Sendable {
	case v4(Ipv4Addr)
	case v6(Ipv6Addr)
}
public struct SocketAddrV4: Hashable, Sendable {
	public var ip: Ipv4Addr
	public var port: UInt16
	@inlinable public init(ip: Ipv4Addr, port: UInt16)
}
public struct SocketAddrV6: Hashable, Sendable {
	public var ip: Ipv6Addr
	public var port: UInt16
	public var flowinfo: UInt32
	public var scopeId: UInt32
	@inlinable public init(ip: Ipv6Addr, port: UInt16, flowinfo: UInt32 = 0, scopeId: UInt32 = 0)
}
public enum SocketAddr: Hashable, Sendable {
	case v4(SocketAddrV4)
	case v6(SocketAddrV6)
	@inlinable public init(ip: IpAddr, port: UInt16)
	@inlinable public var ip: IpAddr { get }
	@inlinable public var port: UInt16 { get }
}
public struct RustDuration: Hashable, Comparable, Sendable {
	public let secs: UInt64
	public let nanos: UInt32
	@inlinable public init(secs: UInt64, nanos: UInt32) throws
	@inlinable public static func seconds(_ secs: UInt64) -> RustDuration
	@inlinable public static func milliseconds(_ millis: UInt64) -> RustDuration
	@inlinable public static func microseconds(_ micros: UInt64) -> RustDuration
	@inlinable public static func nanoseconds(_ nanos: UInt64) -> RustDuration
	@inlinable public init(_ duration: Swift.Duration) throws
	@inlinable public var duration: Swift.Duration { get }
	@inlinable public static func < (a: RustDuration, b: RustDuration) -> Bool
}
public struct PathBuf: Hashable, Sendable, Swift.ExpressibleByStringLiteral {
	public let bytes: Bytes
	@inlinable public init(bytes: Bytes)
	@inlinable public init(_ string: Swift.String)
	@inlinable public init(stringLiteral string: Swift.String)
	@inlinable public var string: Swift.String? { get }
}
public struct Bytes: Hashable, Sendable, RandomAccessCollection {
	@inlinable public init()
	@inlinable public init(_ data: Data)
	@inlinable public init(_ bytes: some Swift.Sequence<Swift.UInt8>)
	@inlinable public var data: Data { get }
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get }
	@inlinable public subscript(bounds: Swift.Range<Swift.Int>) -> Bytes { get }
	@inlinable public func withUnsafeBytes<R>(_ body: (UnsafeRawBufferPointer) throws -> R) rethrows -> R
	@inlinable public static func == (a: Bytes, b: Bytes) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct U128: Hashable, Comparable, Sendable, CustomStringConvertible {
	public var high: UInt64
	public var low: UInt64
	@inlinable public init(high: UInt64, low: UInt64)
	@inlinable public init(_ value: UInt64)
	@inlinable public static func < (a: U128, b: U128) -> Bool
	@inlinable public var description: Swift.String { get }
}
public struct NonZero<T: Swift.FixedWidthInteger & Sendable>: Hashable, Comparable, Sendable, CustomStringConvertible {
	public let value: T
	@inlinable public init(_ value: T) throws
	@inlinable public static func < (a: NonZero, b: NonZero) -> Swift.Bool
	@inlinable public var description: Swift.String { get }
}
public struct CheckError: Swift.Error, CustomStringConvertible, Sendable {
	public let function: Swift.String
	public let message: Swift.String
	@inlinable public var description: Swift.String { get }
}
public enum SeekOrigin: Sendable {
	case start
	case current
	case end
}
public protocol SeekableReader {
	mutating func read(into buffer: UnsafeMutableRawBufferPointer) throws -> Swift.Int
	mutating func seek(offset: Swift.Int64, from origin: SeekOrigin) throws -> Swift.UInt64
	mutating func seek(start offset: Swift.UInt64) throws -> Swift.UInt64
}
extension SeekableReader {
	@inlinable public mutating func seek(start offset: Swift.UInt64) throws -> Swift.UInt64
}
public enum BoundU64: Hashable, Sendable {
	case unbounded
	case included(Swift.UInt64)
	case excluded(Swift.UInt64)
}
public struct BoundsU64: Hashable, Sendable {
	public var start: BoundU64
	public var end: BoundU64
	@inlinable public init(start: BoundU64, end: BoundU64)
}
public struct RangeU64: Hashable, Sendable {
	public var start: Swift.UInt64
	public var end: Swift.UInt64
	@inlinable public init(start: Swift.UInt64, end: Swift.UInt64)
}
@frozen public struct TupleCounterCounter: ~Copyable, Sendable {
	public var _0: Counter
	public var _1: Counter
	@inlinable public init(_ _0: consuming Counter, _ _1: consuming Counter)
}
@frozen public struct TupleCounterMarkSeatChoice: ~Copyable, Sendable {
	public var _0: Counter
	public var _1: Mark
	public var _2: Seat
	public var _3: Choice
	@inlinable public init(_ _0: consuming Counter, _ _1: Mark, _ _2: Seat, _ _3: Choice)
}
@frozen public struct TupleCounterRefusal: ~Copyable, Sendable {
	public var _0: Counter
	public var _1: Refusal
	@inlinable public init(_ _0: consuming Counter, _ _1: Refusal)
}
@frozen public struct TupleCounterBytes: ~Copyable, Sendable {
	public var _0: Counter
	public var _1: Bytes
	@inlinable public init(_ _0: consuming Counter, _ _1: Bytes)
}
@frozen public struct TupleStringCounter: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: Counter
	@inlinable public init(_ _0: Swift.String, _ _1: consuming Counter)
}
@frozen public struct TupleStringJson: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: Json
	@inlinable public init(_ _0: Swift.String, _ _1: consuming Json)
}
public struct ArrayU8x4: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x4, b: ArrayU8x4) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct Parcel: Sendable {
	public var mark: Mark
	public var payload: Bytes
	@inlinable public init(mark: Mark, payload: Bytes)
}
public struct Weighted: Sendable {
	public var score: Score
	public var weight: Swift.UInt32
	@inlinable public init(score: Score, weight: Swift.UInt32)
	@inlinable public func total() -> Swift.UInt64
}
public struct Stamped: Sendable {
	public var at: Stamp
	public var count: Swift.UInt32
	@inlinable public init(at: Stamp, count: Swift.UInt32)
	@inlinable public func total() -> Swift.UInt64
}
public struct CancellationError_: Sendable {
	public var code: Swift.UInt32
	@inlinable public init(code: Swift.UInt32)
}
public struct Packet: Sendable {
	public var seq: Swift.UInt32
	public var payload: Bytes
	@inlinable public init(seq: Swift.UInt32, payload: Bytes)
}
public struct Hasher: Sendable {
	public var seed: Swift.UInt8
	@inlinable public init(seed: Swift.UInt8)
}
public struct Size: Sendable, CustomDebugStringConvertible, Equatable {
	public var width: Swift.UInt32
	public var height: Swift.UInt32
	@inlinable public init(width: Swift.UInt32, height: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Size, b: Size) -> Swift.Bool
}
public struct Point: Sendable, CustomDebugStringConvertible, Equatable {
	public var x: Swift.Int32
	public var y: Swift.Int32
	@inlinable public init(x: Swift.Int32, y: Swift.Int32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Point, b: Point) -> Swift.Bool
	@inlinable public static func `default`() -> Point
	@inlinable public func offset(_ dx: Swift.Int32, _ dy: Swift.Int32) -> Point
	@inlinable public static func getX() -> Swift.Int32
}
public struct Label: Sendable, CustomDebugStringConvertible, Equatable {
	public var name: Swift.String
	public var at: Point?
	public var ttl: RustDuration?
	@inlinable public init(name: Swift.String, at: Point?, ttl: RustDuration?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Label, b: Label) -> Swift.Bool
}
public struct Note: Sendable, CustomDebugStringConvertible, Equatable {
	public var text: Swift.String?
	@inlinable public init(text: Swift.String?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Note, b: Note) -> Swift.Bool
	@inlinable public static func `default`() -> Note
}
public struct Reading: Sendable, CustomDebugStringConvertible, Equatable {
	public var ok: Swift.Bool
	public var level: Swift.Float
	public var ratio: Swift.Double
	public var mark: Swift.Unicode.Scalar
	public var shape: Shape
	public var at: Point
	public var note: Swift.String
	public var count: Swift.UInt64
	public var offset: Swift.Int8
	public var len: Swift.UInt
	public var tag: Swift.UInt16
	@inlinable public init(ok: Swift.Bool, level: Swift.Float, ratio: Swift.Double, mark: Swift.Unicode.Scalar, shape: Shape, at: Point, note: Swift.String, count: Swift.UInt64, offset: Swift.Int8, len: Swift.UInt, tag: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Reading, b: Reading) -> Swift.Bool
	@inlinable public func next() -> Reading?
}
public struct Packaged: Sendable, CustomDebugStringConvertible, Equatable {
	public var api: Swift.UInt32
	@inlinable public init(api: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Packaged, b: Packaged) -> Swift.Bool
	@inlinable public func point() -> Point
	@inlinable public func shape() -> Shape
}
public struct Atlas: Sendable, CustomDebugStringConvertible, Equatable {
	public var points: [(Swift.String, Point)]
	public var ids: [Swift.UInt32]
	@inlinable public init(points: [(Swift.String, Point)], ids: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Atlas, b: Atlas) -> Swift.Bool
	@inlinable public func with(_ name: Swift.String, _ at: Point) -> Atlas
}
public struct Span: Sendable, CustomDebugStringConvertible, Equatable {
	public var bounds: (Swift.UInt32, Swift.UInt32)
	public var named: (Swift.String, Shape)?
	@inlinable public init(bounds: (Swift.UInt32, Swift.UInt32), named: (Swift.String, Shape)?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Span, b: Span) -> Swift.Bool
}
public struct Window: Sendable, CustomDebugStringConvertible, Equatable {
	public var range: RangeU64
	public var end: BoundU64
	@inlinable public init(range: RangeU64, end: BoundU64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Window, b: Window) -> Swift.Bool
	@inlinable public func contains(_ n: Swift.UInt64) -> Swift.Bool
}
public struct Ticket: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Ticket, b: Ticket) -> Swift.Bool
	@inlinable public static func new(_ number: Swift.UInt32) -> Ticket
	@inlinable public func number() -> Swift.UInt32
	@inlinable public static func clones() -> Swift.UInt32
}
public struct Seat: Sendable {
	public var mark: Mark
	public var row: Swift.UInt32
	@inlinable public init(mark: Mark, row: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Seat, b: Seat) throws -> Swift.Bool
	@inlinable public func count() throws -> Swift.UInt64
}
public struct Stamp: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Stamp, b: Stamp) -> Swift.Bool
	@inlinable public static func fromMillis(_ millis: Swift.UInt64) -> Stamp
	@inlinable public func secs() -> Swift.UInt64
	@inlinable public func whole() -> Stamp
	@inlinable public func later(_ other: Stamp) -> Swift.Bool
}
public struct Moment: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Moment, b: Moment) -> Swift.Bool
	@inlinable public static func new(_ ticks: Swift.UInt64, _ rate: Swift.UInt64) -> Moment?
	@inlinable public func millis() -> Swift.UInt64
	@inlinable public func after(_ ticks: Swift.UInt64) -> Moment
}
public struct LongArray: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LongArray, b: LongArray) -> Swift.Bool
	@inlinable public static func new(_ low: Swift.UInt64, _ high: Swift.UInt64) -> LongArray
	@inlinable public func c() -> Swift.UInt64
	@inlinable public func also(_ f: @Sendable (Swift.UInt64) -> Void)
}
public struct JvmField: Sendable, CustomDebugStringConvertible, Equatable {
	public var n: Swift.UInt32
	@inlinable public init(n: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: JvmField, b: JvmField) -> Swift.Bool
}
public struct JvmStatic: Sendable, CustomDebugStringConvertible, Equatable {
	public var n: Swift.UInt32
	@inlinable public init(n: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: JvmStatic, b: JvmStatic) -> Swift.Bool
}
public struct OptIn: Sendable, CustomDebugStringConvertible, Equatable {
	public var n: Swift.UInt32
	@inlinable public init(n: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OptIn, b: OptIn) -> Swift.Bool
}
public struct Wide: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Wide, b: Wide) -> Swift.Bool
	@inlinable public static func new(_ seed: Swift.UInt64) -> Wide
	@inlinable public func sum() -> Swift.UInt64
}
public struct Clocked: Sendable, CustomDebugStringConvertible, Equatable {
	public var at: Moment
	public var count: Swift.UInt32
	@inlinable public init(at: Moment, count: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Clocked, b: Clocked) -> Swift.Bool
}
public struct Rated: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Rated, b: Rated) -> Swift.Bool
	@inlinable public static func new(_ count: Swift.UInt64, _ scale: Swift.UInt64) -> Rated
	@inlinable public func scale() -> Swift.UInt64
}
public struct Score: Sendable, CustomDebugStringConvertible {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func of(_ count: Swift.UInt64) -> Score
	@inlinable public func count() -> Swift.UInt64
	@inlinable public mutating func add(_ other: Score)
	@inlinable public func doubled() -> Score
	@inlinable public mutating func take() -> Swift.UInt64
	@inlinable public mutating func note(_ mark: Mark) throws -> Swift.UInt64
	@inlinable public mutating func absorb(_ other: inout Score) -> (Swift.UInt64, Swift.UInt64)
}
public struct Hits: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Hits, b: Hits) -> Swift.Bool
	@inlinable public static func `default`() -> Hits
	@inlinable public var hitsGet: Swift.UInt64 { get }
	@inlinable public mutating func hitsSet(_ value: Swift.UInt64)
	@inlinable public var missesGet: Swift.UInt64 { get }
	@inlinable public mutating func missesSet(_ value: Swift.UInt64)
	@inlinable public static func of(_ hits: Swift.UInt64, _ misses: Swift.UInt64) -> Hits
	@inlinable public mutating func add(_ other: Hits)
	@inlinable public mutating func eq(_ other: Hits) -> Swift.Bool
}
public struct Extent: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Extent, b: Extent) -> Swift.Bool
	@inlinable public var startGet: Swift.UInt64 { get }
	@inlinable public mutating func startSet(_ value: Swift.UInt64)
	@inlinable public static func new(_ start: Swift.UInt64, _ len: Swift.UInt64) -> Extent
	@inlinable public func end() -> Swift.UInt64
	@inlinable public mutating func slide(_ by: Swift.UInt64) -> Swift.UInt64
}
public struct PairSizeU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var left: Size
	public var right: Swift.UInt8
	@inlinable public init(left: Size, right: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PairSizeU8, b: PairSizeU8) -> Swift.Bool
	@inlinable public func swap() -> PairU8Size
}
public struct PairNonZeroU16: Sendable, CustomDebugStringConvertible, Equatable {
	public var left: NonZero<Swift.UInt16>
	public var right: NonZero<Swift.UInt16>
	@inlinable public init(left: NonZero<Swift.UInt16>, right: NonZero<Swift.UInt16>)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PairNonZeroU16, b: PairNonZeroU16) -> Swift.Bool
	@inlinable public func swap() -> PairNonZeroU16
	@inlinable public func even() -> Swift.Bool
}
public struct PairU32: Sendable, CustomDebugStringConvertible, Equatable {
	public var left: Swift.UInt32
	public var right: Swift.UInt32
	@inlinable public init(left: Swift.UInt32, right: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PairU32, b: PairU32) -> Swift.Bool
	@inlinable public func swap() -> PairU32
	@inlinable public func even() -> Swift.Bool
	@inlinable public func sum() -> Swift.UInt32
	@inlinable public func besideSize(_ right: Size) -> PairU8Size
}
public struct PairU8Size: Sendable, CustomDebugStringConvertible, Equatable {
	public var left: Swift.UInt8
	public var right: Size
	@inlinable public init(left: Swift.UInt8, right: Size)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PairU8Size, b: PairU8Size) -> Swift.Bool
}
public struct LeftRightU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: RightU8
	@inlinable public init(value: RightU8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LeftRightU8, b: LeftRightU8) -> Swift.Bool
}
public struct LeftU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt8
	@inlinable public init(value: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LeftU8, b: LeftU8) -> Swift.Bool
}
public struct RightLeftU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: LeftU8
	@inlinable public init(value: LeftU8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RightLeftU8, b: RightLeftU8) -> Swift.Bool
}
public struct RightU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt8
	@inlinable public init(value: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RightU8, b: RightU8) -> Swift.Bool
}
public struct InnerU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt8
	@inlinable public init(value: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: InnerU8, b: InnerU8) -> Swift.Bool
}
public struct BoxedU16: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt16
	@inlinable public init(value: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: BoxedU16, b: BoxedU16) -> Swift.Bool
}
public struct BoxedU8: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt8
	@inlinable public init(value: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: BoxedU8, b: BoxedU8) -> Swift.Bool
}
public struct LogCount: Sendable, CustomDebugStringConvertible, Equatable {
	public var kept: Swift.UInt32
	@inlinable public init(kept: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LogCount, b: LogCount) -> Swift.Bool
}
public struct WrapU32: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.UInt32
	@inlinable public init(value: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: WrapU32, b: WrapU32) -> Swift.Bool
}
public struct PolarTurn: Sendable, CustomDebugStringConvertible, Equatable {
	public var whole: Swift.UInt8
	@inlinable public init(whole: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PolarTurn, b: PolarTurn) -> Swift.Bool
}
public struct Peer: Sendable, CustomDebugStringConvertible, Equatable {
	public var addr: SocketAddr
	public var names: [Swift.String]
	@inlinable public init(addr: SocketAddr, names: [Swift.String])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Peer, b: Peer) -> Swift.Bool
	@inlinable public func aliases() -> [Swift.String]
	@inlinable public func next(_ name: Swift.String) -> Peer
}
public struct Shadowed: Sendable, CustomDebugStringConvertible, Equatable {
	public var other: [Swift.Int32]
	public var a: Swift.UInt8
	public var b: Swift.UInt8
	@inlinable public init(other: [Swift.Int32], a: Swift.UInt8, b: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Shadowed, b: Shadowed) -> Swift.Bool
}
public struct Numbers: Sendable, CustomDebugStringConvertible, Equatable {
	public var u16s: [Swift.UInt16]
	public var u32s: [Swift.UInt32]
	public var u64s: [Swift.UInt64]
	public var i8s: [Swift.Int8]
	public var i16s: [Swift.Int16]
	public var i32s: [Swift.Int32]
	public var i64s: [Swift.Int64]
	public var f32s: [Swift.Float]
	public var f64s: [Swift.Double]
	public var some: [Swift.Int32]?
	@inlinable public init(u16s: [Swift.UInt16], u32s: [Swift.UInt32], u64s: [Swift.UInt64], i8s: [Swift.Int8], i16s: [Swift.Int16], i32s: [Swift.Int32], i64s: [Swift.Int64], f32s: [Swift.Float], f64s: [Swift.Double], some: [Swift.Int32]?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Numbers, b: Numbers) -> Swift.Bool
	@inlinable public static func edges() -> Numbers
	@inlinable public static func zero(_ negative: Swift.Bool) -> Numbers
	@inlinable public func reversed() -> Numbers
}
public struct Measure: Sendable, CustomDebugStringConvertible, Equatable {
	public var value: Swift.Double
	public var numbers: Numbers
	@inlinable public init(value: Swift.Double, numbers: Numbers)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Measure, b: Measure) -> Swift.Bool
	@inlinable public static func new(_ value: Swift.Double, _ negative: Swift.Bool) -> Measure
}
public struct LimitsBytes: Sendable, CustomDebugStringConvertible, Equatable {
	public var low: Bytes
	public var high: Bytes
	@inlinable public init(low: Bytes, high: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LimitsBytes, b: LimitsBytes) -> Swift.Bool
}
public struct LimitsDuration: Sendable, CustomDebugStringConvertible, Equatable {
	public var low: RustDuration
	public var high: RustDuration
	@inlinable public init(low: RustDuration, high: RustDuration)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: LimitsDuration, b: LimitsDuration) -> Swift.Bool
}
public struct Page: Sendable, CustomDebugStringConvertible, Equatable {
	public var size: Swift.UInt32
	@inlinable public init(size: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Page, b: Page) -> Swift.Bool
	@inlinable public static func `default`() throws -> Page
}
public struct Booth: Sendable {
	public var mark: Mark
	@inlinable public init(mark: Mark)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Booth, b: Booth) throws -> Swift.Bool
}
public struct Cancelled: Sendable, CustomDebugStringConvertible, Equatable {
	public var step: Swift.UInt8
	@inlinable public init(step: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Cancelled, b: Cancelled) -> Swift.Bool
}
public struct StartSeekable: Sendable, CustomDebugStringConvertible, Equatable {
	public var at: Swift.UInt64
	@inlinable public init(at: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StartSeekable, b: StartSeekable) -> Swift.Bool
}
public struct DoubleArray: Sendable, CustomDebugStringConvertible, Equatable {
	public var len: Swift.UInt32
	@inlinable public init(len: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: DoubleArray, b: DoubleArray) -> Swift.Bool
}
public struct NonZeros: Sendable, CustomDebugStringConvertible {
	public var u8: NonZero<Swift.UInt8>
	public var u16: NonZero<Swift.UInt16>
	public var u32: NonZero<Swift.UInt32>
	public var u64: NonZero<Swift.UInt64>
	public var usize: NonZero<Swift.UInt>
	public var i8: NonZero<Swift.Int8>
	public var i16: NonZero<Swift.Int16>
	public var i32: NonZero<Swift.Int32>
	public var i64: NonZero<Swift.Int64>
	public var isize: NonZero<Swift.Int>
	@inlinable public init(u8: NonZero<Swift.UInt8>, u16: NonZero<Swift.UInt16>, u32: NonZero<Swift.UInt32>, u64: NonZero<Swift.UInt64>, usize: NonZero<Swift.UInt>, i8: NonZero<Swift.Int8>, i16: NonZero<Swift.Int16>, i32: NonZero<Swift.Int32>, i64: NonZero<Swift.Int64>, isize: NonZero<Swift.Int>)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct A: Sendable, CustomDebugStringConvertible, Equatable {
	public var b: Swift.UInt8
	@inlinable public init(b: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: A, b: A) -> Swift.Bool
}
public struct Qualifiers: Sendable, Equatable {
	public var api: Swift.Double
	public var kotlin: Swift.Double
	@inlinable public init(api: Swift.Double, kotlin: Swift.Double)
	@inlinable public static func == (a: Qualifiers, b: Qualifiers) -> Swift.Bool
}
public struct Version: Sendable, CustomDebugStringConvertible, Equatable, Comparable {
	public var major: Swift.UInt16
	public var minor: Swift.UInt16
	@inlinable public init(major: Swift.UInt16, minor: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Version, b: Version) -> Swift.Bool
	@inlinable public static func < (a: Version, b: Version) -> Swift.Bool
	@inlinable public static func <= (a: Version, b: Version) -> Swift.Bool
	@inlinable public static func > (a: Version, b: Version) -> Swift.Bool
	@inlinable public static func >= (a: Version, b: Version) -> Swift.Bool
}
public struct Deprecated: Sendable, CustomDebugStringConvertible, Equatable {
	public var on: Swift.Bool
	@available(*, deprecated) @inlinable public var lower: Swift.UInt32 { get set }
	public var deprecated: Swift.Bool
	@inlinable public init(on: Swift.Bool, lower: Swift.UInt32, deprecated: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Deprecated, b: Deprecated) -> Swift.Bool
	@inlinable public static func `default`() -> Deprecated
}
public struct Dial: Sendable, CustomDebugStringConvertible, Equatable {
	public var level: Swift.UInt64
	@available(*, deprecated, message: "keep the `Dial` from before the turn") @inlinable public var before: Swift.UInt64 { get set }
	@inlinable public init(level: Swift.UInt64, before: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Dial, b: Dial) -> Swift.Bool
	@inlinable public static func `default`() -> Dial
	@inlinable public func turn(_ by: Swift.UInt64) throws -> Dial
	@available(*, deprecated, message: "turn it by `LIMIT` with `Dial::turn`") @inlinable public func max() -> Dial
}
@available(*, deprecated, message: "use `Dial`, whose 'level' is \"the same\" at $0\nand past it") public struct Knob: Sendable, CustomDebugStringConvertible, Equatable {
	public var level: Swift.UInt64
	@inlinable public init(level: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Knob, b: Knob) -> Swift.Bool
	@inlinable public static func `default`() -> Knob
	@inlinable public func doubled() -> Swift.UInt64
}
public struct PolarPoint: Sendable, CustomDebugStringConvertible, Equatable {
	public var turns: Swift.UInt8
	public var distance: Swift.UInt32
	@inlinable public init(turns: Swift.UInt8, distance: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PolarPoint, b: PolarPoint) -> Swift.Bool
}
public struct Polar_Turn: Sendable, CustomDebugStringConvertible, Equatable {
	public var part: Swift.UInt8
	@inlinable public init(part: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Polar_Turn, b: Polar_Turn) -> Swift.Bool
}
public struct ClashTone: Sendable, CustomDebugStringConvertible, Equatable {
	public var hz: Swift.UInt16
	@inlinable public init(hz: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashTone, b: ClashTone) -> Swift.Bool
}
public struct ClashTone_: Sendable, CustomDebugStringConvertible, Equatable {
	public var db: Swift.UInt8
	@inlinable public init(db: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashTone_, b: ClashTone_) -> Swift.Bool
}
public struct ClashSpan: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var width: Swift.UInt32
	@inlinable public init(width: Swift.UInt32)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashSpan, b: ClashSpan) -> Swift.Bool
	@inlinable public func width_() -> Swift.UInt32
	@inlinable public func toString() -> Swift.String
}
public struct ClashAny: Sendable, CustomDebugStringConvertible, Hashable {
	public var typeUrl: Swift.String
	@inlinable public init(typeUrl: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashAny, b: ClashAny) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ClashBreak: Sendable, CustomDebugStringConvertible, Equatable {
	public var _0: Swift.UInt8
	@inlinable public init(_ _0: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashBreak, b: ClashBreak) -> Swift.Bool
	@inlinable public static var SP: ClashBreak { get }
	@inlinable public static var SP_: ClashBreak { get }
}
public struct TlsPolicyLifetime: Sendable, CustomDebugStringConvertible, Equatable {
	public var seconds: Swift.UInt32
	@inlinable public init(seconds: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TlsPolicyLifetime, b: TlsPolicyLifetime) -> Swift.Bool
}
public struct Core: Sendable {
	public var id: Swift.UInt32
	@inlinable public init(id: Swift.UInt32)
}
public enum Choice: Sendable {
	case empty
	case picked(Mark)
}
public enum Array_: Sendable {
	case u8x4(Swift.UInt8)
}
public enum Platform: Sendable {
	case wasm(bits: Swift.UInt32)
	case native(Swift.UInt32)
}
public enum Placement: Sendable {
	case anywhere
	case pinned(Core)
}
public enum Shape: Sendable, CustomDebugStringConvertible, Equatable {
	case circle
	case square
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Shape, b: Shape) -> Swift.Bool
	@inlinable public static func valueOf(_ name: Swift.String) -> Shape?
	@inlinable public static var ALL: [Shape] { get }
	@inlinable public static var BOTH: [Shape] { get }
}
public enum Event: Sendable, CustomDebugStringConvertible, Equatable {
	case moved(Point)
	case named(name: Swift.String)
	case done
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Event, b: Event) -> Swift.Bool
}
public enum Clash: Sendable, CustomDebugStringConvertible, Equatable {
	case point(Point)
	case shape(Shape)
	case event(Event)
	case double(Swift.Double)
	case float(Swift.Float)
	case string(Swift.String)
	case int(Swift.Int32)
	case boolean(Swift.Bool)
	case native(Swift.UInt32)
	case bytes(Bytes)
	case any
	case unit
	case waker
	case mut
	case pair(Swift.UInt8, Swift.UInt8)
	case stamp(Swift.Int64)
	case moment(Moment)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Clash, b: Clash) -> Swift.Bool
	@inlinable public func point() -> Point?
	@inlinable public static func of(_ value: Swift.Double) -> Clash
	@inlinable public func bytes() -> Bytes
	@inlinable public func reading(_ kotlin: Swift.UInt32) -> Reading
	@inlinable public static func listed(_ java: [Swift.UInt32], _ kotlin: inout Swift.UInt32) -> [[Swift.UInt32]]
	@inlinable nonisolated(nonsending) public static func after(_ first: @escaping @Sendable () async -> Void, _ kotlin: Swift.Double) async throws -> Clash
	@inlinable public func joined(_ lent: Bytes, _ owned: Bytes, _ maybe: Bytes?) -> Bytes
	@inlinable nonisolated(nonsending) public func tagged(_ tag: Swift.String) async throws -> Swift.String
	@inlinable public func stamp(_ kotlin: Swift.UInt64) -> Stamp
	@inlinable public func moment(_ kotlin: Swift.UInt64) -> Moment
	@inlinable public static func at(_ api: Point) -> Clash
	@inlinable public func describe(_ text: Swift.String, _ n: Swift.UInt32, _ shape: Shape, _ out: inout Swift.UInt32) -> Swift.String
}
public enum Stage: Sendable, CustomDebugStringConvertible, Equatable {
	case init_(Swift.UInt32)
	case done
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Stage, b: Stage) -> Swift.Bool
}
public enum Error: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case underflow(by: Swift.UInt64)
	case overflow
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Error, b: Error) -> Swift.Bool
}
public enum Status: Sendable, CustomDebugStringConvertible, Equatable {
	case moved
	case kept
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Status, b: Status) -> Swift.Bool
}
public enum Labelled: Sendable, CustomDebugStringConvertible, Equatable {
	case tagged(tag: Swift.UInt32)
	case bare
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Labelled, b: Labelled) -> Swift.Bool
}
public enum Family: Sendable, CustomDebugStringConvertible, Equatable {
	case native
	case wasm
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Family, b: Family) -> Swift.Bool
}
public enum VecClashTone: Sendable, CustomDebugStringConvertible, Equatable {
	case error
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: VecClashTone, b: VecClashTone) -> Swift.Bool
}
public enum B: Sendable, CustomDebugStringConvertible, Equatable {
	case one
	case two(Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: B, b: B) -> Swift.Bool
}
public enum Builtin: Sendable, Equatable {
	case any([Swift.Int32])
	case boolean(Swift.Double)
	case int(Swift.Double)
	case native(Swift.Double)
	@inlinable public static func == (a: Builtin, b: Builtin) -> Swift.Bool
}
public enum Tier: Sendable, CustomDebugStringConvertible, Equatable, Comparable {
	case low
	case high
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Tier, b: Tier) -> Swift.Bool
	@inlinable public static func < (a: Tier, b: Tier) -> Swift.Bool
	@inlinable public static func <= (a: Tier, b: Tier) -> Swift.Bool
	@inlinable public static func > (a: Tier, b: Tier) -> Swift.Bool
	@inlinable public static func >= (a: Tier, b: Tier) -> Swift.Bool
}
public enum ClashCandidate: Sendable, CustomDebugStringConvertible, Equatable {
	case none
	case match(Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClashCandidate, b: ClashCandidate) -> Swift.Bool
}
public enum Level: Sendable, CustomStringConvertible, Hashable, Comparable {
	case trace
	case debug
	case info
	case warn
	case error
	@inlinable public var description: Swift.String { get }
	@inlinable public static func == (a: Level, b: Level) -> Swift.Bool
	@inlinable public static func < (a: Level, b: Level) -> Swift.Bool
	@inlinable public static func <= (a: Level, b: Level) -> Swift.Bool
	@inlinable public static func > (a: Level, b: Level) -> Swift.Bool
	@inlinable public static func >= (a: Level, b: Level) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct Board: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ counters: consuming UniqueArray<TupleStringCounter>) throws -> Board
	@inlinable public consuming func intoCounters() throws -> UniqueArray<TupleStringCounter>
	@inlinable public func ranked() throws -> UniqueArray<TupleStringCounter>
	@inlinable public func best() throws -> Counter
}
public struct Receipt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Receipt
	@inlinable public func count() throws -> Swift.UInt32
}
public struct Draft: ~Copyable {
	@inlinable public func edited() throws -> Swift.Bool
	@inlinable public func describe() throws -> Swift.String
	@inlinable public consuming func commit() throws -> Swift.UInt32
	@inlinable public var titleGet: Swift.String { get }
	@inlinable public mutating func titleSet(_ value: Swift.String) throws
	@inlinable public func lines() throws -> Swift.UInt32
	@inlinable public mutating func write(_ line: Swift.String) throws
}
public struct FiledReceipt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> FiledReceipt
	@inlinable public func count() throws -> Swift.UInt32
}
public struct PadU16: ~Copyable, @unchecked Sendable {
	@inlinable public static func wide() -> PadU16
	@inlinable public func tag() -> Swift.UInt16
	@inlinable public func title() -> Swift.UInt16
	@inlinable public func describe() -> Swift.String
}
public struct PadU8: ~Copyable, @unchecked Sendable {
	@inlinable public static func small() -> PadU8
	@inlinable public func describe() -> Swift.String
	@inlinable public var titleGet: Swift.String { get }
	@inlinable public func lines() -> Swift.UInt32
}
public struct Layered: ~Copyable, @unchecked Sendable {
	@inlinable public static func new() -> Layered
	@inlinable public var titleGet: Swift.String { get }
	@inlinable public func lines() -> Swift.UInt32
}
public struct Marker: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ label: Swift.String, _ x: Swift.Int32, _ y: Swift.Int32) -> Marker
	@inlinable public func label() throws -> Swift.String
	@inlinable public var xGet: Swift.Int32 { get }
	@inlinable public mutating func xSet(_ value: Swift.Int32) throws
	@inlinable public var yGet: Swift.Int32 { get }
	@inlinable public mutating func ySet(_ value: Swift.Int32) throws
	@inlinable public func offset(_ dx: Swift.Int32, _ dy: Swift.Int32) throws -> Point
}
public struct Batch: ~Copyable, @unchecked Sendable {
	@inlinable public static func new() -> Batch
	@inlinable public mutating func push(_ v: Swift.UInt32) throws -> Swift.Bool
	@inlinable public func capacity() throws -> Swift.UInt
}
public struct Batch2: ~Copyable, @unchecked Sendable {
	@inlinable public static func new() -> Batch2
	@inlinable public mutating func push(_ v: Swift.UInt32) throws -> Swift.Bool
	@inlinable public func capacity() throws -> Swift.UInt
}
public struct Waiter: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ waker: @escaping @Sendable () -> Void) -> Waiter
	@inlinable public func waker() throws -> @Sendable () -> Void
	@inlinable public func context() throws -> @Sendable () -> Void
	@inlinable nonisolated(nonsending) public static func pollFuture(_ future: @escaping @Sendable () async -> Void) async throws
}
public struct Logger: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ sink: any Log) -> Logger
	@inlinable public func write(_ line: Swift.String) -> Swift.Bool
	@inlinable public func writeFromThread(_ line: Swift.String) throws -> Swift.Bool
}
public struct Journal: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ sink: any Log) -> Journal
	@inlinable public mutating func redirect(_ sink: any Log) throws
	@inlinable public func write(_ line: Swift.String) throws -> Swift.Bool
}
public struct Local: ~Copyable {
	@inlinable public static func == (a: borrowing Local, b: borrowing Local) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> Local
	@inlinable public static func new() -> Local
	@inlinable public func bump() throws -> Swift.UInt32
	@inlinable public func share() throws -> Local
	@inlinable nonisolated(nonsending) public func later() async throws -> Swift.UInt32
	@inlinable public func visit(_ f: @Sendable () -> Swift.UInt32) throws -> Swift.UInt32
	@inlinable nonisolated(nonsending) public func pollCount() async throws -> Swift.UInt32
	@inlinable public func threadNumber() throws -> Swift.UInt64
	@inlinable public func onOwnThread() throws -> Swift.Bool
}
public struct LocalWaiter: ~Copyable {
	@inlinable public static func new(_ waker: @escaping @Sendable () -> Void) -> LocalWaiter
	@inlinable nonisolated(nonsending) public static func pollReady(_ n: Swift.UInt32) async throws -> Swift.UInt32
}
public struct Retry: ~Copyable, @unchecked Sendable {
	@inlinable public var limitGet: Swift.UInt32? { get }
	@inlinable public mutating func limitSet(_ value: Swift.UInt32?) throws
	@inlinable public static func new(_ limit: Swift.UInt32?) -> Retry
	@inlinable public func limit() throws -> Swift.UInt32
	@inlinable public func delay() throws -> Swift.UInt32
	@inlinable public mutating func setLimit(_ limit: Swift.UInt32) throws
}
public struct Digest: ~Copyable, @unchecked Sendable {
	@inlinable public var sumGet: ArrayU8x4 { get }
	@inlinable public mutating func sumSet(_ value: ArrayU8x4) throws
	@inlinable public static func new() -> Digest
	@inlinable public func flipped(_ mask: ArrayU8x4) throws -> ArrayU8x4
}
public struct Cursor: ~Copyable, @unchecked Sendable {
	@inlinable public func at() throws -> Swift.UInt64
}
public struct OuterInnerU8: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ value: Swift.UInt8) -> OuterInnerU8
	@inlinable public func get() -> Swift.UInt8
}
public struct SlotU32: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ value: Swift.UInt32) -> SlotU32
}
public struct SlotU8: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ value: Swift.UInt8) -> SlotU8
	@inlinable public func value() -> Swift.UInt16
}
public struct Resolving: ~Copyable, @unchecked Sendable {
	@inlinable public func _0_get() throws -> TlsResolver
	@inlinable public mutating func _0_set(_ value: consuming TlsResolver) throws
	@inlinable public static func new(_ name: Swift.String) -> Resolving
}
public struct Codecs: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Codecs
	@inlinable public func listGet() throws -> UniqueArray<TlsCodec>
	@inlinable public mutating func listSet(_ value: consuming UniqueArray<TlsCodec>) throws
	@inlinable public static func new() -> Codecs
	@inlinable public mutating func ids() throws -> Swift.UInt32
}
public struct Der: ~Copyable, @unchecked Sendable {
	@inlinable public static func fromSlice(_ bytes: Bytes) -> Der
	@inlinable public func bytes() -> Bytes
}
public struct Verbosity: ~Copyable, @unchecked Sendable {
	@inlinable public var levelGet: Level { get }
	@inlinable public mutating func levelSet(_ value: Level) throws
	@inlinable public static func new(_ level: Level) -> Verbosity
	@inlinable public func filter() throws -> Level?
}
public struct Site: ~Copyable, @unchecked Sendable {
	@inlinable public func homeGet() throws -> Url
	@inlinable public mutating func homeSet(_ value: consuming Url) throws
	@inlinable public static func new(_ home: consuming Url) throws -> Site
}
public struct Trail: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ points: [Point], _ names: [Swift.String]) -> Trail
	@inlinable public func points() throws -> [Point]
	@inlinable public func labels() throws -> [Label]
	@inlinable public func iter() throws -> [Point]
	@inlinable public func names() throws -> [Swift.String]
	@inlinable public func beyond(_ x: Swift.Int32) throws -> [Point]
	@inlinable public func bytes() throws -> Bytes
	@inlinable public mutating func drain() throws -> [Label]
}
public struct Worker: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ id: Swift.UInt32) throws -> Worker
	@inlinable public func id() throws -> Swift.UInt32
}
public struct Handle: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ count: Swift.UInt32) -> Handle
	@inlinable public func count() -> Swift.UInt32
	@inlinable nonisolated(nonsending) public static func later(_ count: Swift.UInt32) async throws -> Handle
}
public struct Holder: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ level: Swift.UInt64) -> Holder
	@available(*, deprecated, message: "use `Dial`, whose 'level' is \"the same\" at $0\nand past it") @inlinable public var levelGet: Swift.UInt64 { get }
	@available(*, deprecated, message: "use `Dial`, whose 'level' is \"the same\" at $0\nand past it") @inlinable public func doubled() -> Swift.UInt64
}
public struct Area: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Area
	@inlinable public static func new(_ size: Size) -> Area
	@inlinable public func size() -> Size
	@inlinable public func pixels() -> Swift.UInt64
	@inlinable public func scaledPoint(_ by: Point) -> Swift.UInt64
}
public struct Counter: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Counter
	@inlinable public static func `default`() -> Counter
	@inlinable public static func new(_ name: Swift.String, _ start: Swift.UInt64) -> Counter
	@inlinable public func get() throws -> Swift.UInt64
	@inlinable public func offset(_ handle: Swift.UInt64, _ h: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public consuming func intoSum(_ close: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public func name() throws -> Swift.String
	@inlinable public mutating func add(_ n: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public mutating func sub(_ n: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public func edited() throws -> Swift.Bool
	@inlinable public func plus() throws -> Swift.UInt64
	@inlinable public func plus(other: borrowing Counter) throws -> Swift.UInt64
	@inlinable public func clamped() throws -> Swift.UInt64
	@inlinable public func clamped(low: borrowing Counter) throws -> Swift.UInt64
	@inlinable public func clamped(high: borrowing Counter) throws -> Swift.UInt64
	@inlinable public func clamped(low: borrowing Counter, high: borrowing Counter) throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public func plusLater() async throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public func plusLater(other: borrowing Counter) async throws -> Swift.UInt64
	@inlinable public func addTo(_ other: inout Counter?) throws -> Swift.Bool
	@inlinable public consuming func intoLabel(_ at: Point?) throws -> Label
	@inlinable public mutating func rename(_ name: Swift.String) throws
	@inlinable public consuming func split(_ at: Swift.UInt64) throws -> TupleCounterCounter
	@inlinable public func `is`(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public static func seededPoint(_ name: Swift.String, _ from: Point) -> Counter
	@inlinable public static func seededCounter(_ name: Swift.String, _ from: borrowing Counter) throws -> Counter
	@inlinable public static func seededSize(_ name: Swift.String, _ from: Size) -> Counter
	@inlinable public static func seededWrapU32(_ name: Swift.String, _ from: WrapU32) -> Counter
	@inlinable public static func seededPolarPoint(_ name: Swift.String, _ from: PolarPoint) -> Counter
}
public struct Meter: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Meter
	@inlinable public func add(_ n: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public func count() throws -> Swift.UInt64
	@inlinable public mutating func reset() throws
}
public struct Meters: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Meters
	@inlinable public func lastGet() throws -> Meter
	@inlinable public mutating func lastSet(_ value: consuming Meter) throws
	@inlinable public func spareGet() throws -> Meter?
	@inlinable public mutating func spareSet(_ value: consuming Meter?) throws
	@inlinable public static func new() -> Meters
	@inlinable public func first() throws -> Meter
	@inlinable public mutating func clear() throws
}
public struct Registry: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Registry
	@inlinable public func firstGet() throws -> Counter
	@inlinable public mutating func firstSet(_ value: consuming Counter) throws
	@inlinable public static func new(_ first: consuming Counter) throws -> Registry
	@inlinable public func last() throws -> Counter?
	@inlinable public mutating func push(_ counter: consuming Counter) throws
	@inlinable public mutating func edit(_ edit: @Sendable (inout Counter) -> Swift.Bool) throws -> Swift.Bool
}
public struct Ledger: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Ledger
	@inlinable public static func new() -> Ledger
	@inlinable public mutating func edit() throws -> Draft
	@inlinable public func published() throws -> Swift.UInt32
	@inlinable public func title() throws -> Swift.String
	@inlinable public mutating func seal() throws
	@inlinable public mutating func receipt() throws -> Receipt
	@inlinable public mutating func noted() throws -> Receipt
	@inlinable public mutating func fileReceipt(_ item: consuming Receipt) throws -> FiledReceipt
	@inlinable public mutating func slip() throws -> Slip
}
public struct Slip: ~Copyable {
	@inlinable public func clone() throws -> Slip
	@inlinable public func receiptGet() throws -> Receipt
	@inlinable public mutating func receiptSet(_ value: consuming Receipt) throws
	@inlinable public func globalGet() throws -> Receipt
	@inlinable public mutating func globalSet(_ value: consuming Receipt) throws
	@inlinable public func count() throws -> Swift.UInt32
}
public struct Sheet: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Sheet
	@inlinable public static func `default`() -> Sheet
	@inlinable public var titleGet: Swift.String { get }
	@inlinable public mutating func titleSet(_ value: Swift.String) throws
	@inlinable public static func new() -> Sheet
	@inlinable public func lines() throws -> Swift.UInt32
	@inlinable public mutating func write(_ line: Swift.String) throws
	@inlinable public func describe() throws -> Swift.String
}
public struct Name: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Name
	@inlinable public static func == (a: borrowing Name, b: borrowing Name) throws -> Swift.Bool
	@inlinable public static func owned(_ text: Swift.String) -> Name
	@inlinable public func asStr() throws -> Swift.String
	@inlinable public func join(_ other: borrowing Name) throws -> Name
	@inlinable public static func borrowed(_ text: Swift.String) -> Name
	@inlinable public func shares(_ other: borrowing Name) throws -> Swift.Bool
	@inlinable public mutating func rename(_ text: Swift.String) throws
}
public struct Channel: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Channel
	@inlinable public static func `default`() -> Channel
	@inlinable public static func new() -> Channel
	@inlinable public mutating func clear() throws
	@inlinable public func send(_ payload: Bytes) throws
	@inlinable nonisolated(nonsending) public func recv() async throws -> Bytes
	@inlinable public func notify(_ waker: @escaping @Sendable () -> Void) throws
	@inlinable nonisolated(nonsending) public func sendLater(_ payload: Bytes) async throws
	@inlinable nonisolated(nonsending) public func tagged(_ tag: Swift.String) async throws -> Swift.String
	@inlinable nonisolated(nonsending) public func itemized(_ items: [Swift.UInt32]?) async throws -> Swift.UInt
	@inlinable nonisolated(nonsending) public func pollTake() async throws -> Bytes
	@inlinable nonisolated(nonsending) public mutating func pollMeasure(_ extra: Swift.UInt) async throws -> Swift.UInt
	@inlinable public func listen(_ cx: @escaping @Sendable () -> Void) throws
	@inlinable nonisolated(nonsending) public func recvSized() async throws -> (Bytes, Swift.UInt)
	@inlinable nonisolated(nonsending) public func recvNonempty() async throws -> Bytes
	@inlinable public func sendFromThread(_ payload: Bytes) throws
}
public final class Gap: @unchecked Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Gap
	@inlinable public static func == (a: Gap, b: Gap) -> Swift.Bool
	@inlinable public static func new(_ wide: Swift.UInt64, _ narrow: Swift.UInt8) -> Gap
	@inlinable public func sum() -> Swift.UInt64
}
public final class Berth: @unchecked Sendable, CustomDebugStringConvertible {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Berth
	@inlinable public static func new(_ at: Swift.UInt32?) -> Berth
	@inlinable public func at() -> Swift.UInt32?
}
public final class Mark: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mark
	@inlinable public static func == (a: Mark, b: Mark) throws -> Swift.Bool
	@inlinable public static func of(_ count: Swift.UInt64) -> Mark
	@inlinable public func count() throws -> Swift.UInt64
	@inlinable public func add(_ by: Swift.UInt64) throws
}
public struct Message: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Message
	@inlinable public var stampGet: Stamp? { get }
	@inlinable public mutating func stampSet(_ value: Stamp?) throws
	@inlinable public var payloadGet: Bytes { get }
	@inlinable public mutating func payloadSet(_ value: Bytes) throws
	@inlinable public static func new(_ stamp: Stamp?, _ payload: Bytes) -> Message
	@inlinable public func sent() throws -> Swift.Bool
}
public struct Node: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public var valueGet: Swift.UInt64 { get }
	@inlinable public mutating func valueSet(_ value: Swift.UInt64) throws
	@inlinable public static func new(_ value: Swift.UInt64, _ next: consuming Node?) throws -> Node
	@inlinable public mutating func push(_ value: Swift.UInt64) throws
	@inlinable public func sum() throws -> Swift.UInt64
}
public struct Tally: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Tally, b: borrowing Tally) throws -> Swift.Bool
	@inlinable public static func `default`() -> Tally
	@inlinable public static func new() -> Tally
	@inlinable public func bump() throws -> Swift.UInt32
	@inlinable nonisolated(nonsending) public func bumpLater() async throws -> Swift.UInt32
}
public struct Tallies: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> Tallies
	@inlinable public static func new() -> Tallies
	@inlinable public func first() throws -> Tally
}
public struct Plain: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Plain, b: borrowing Plain) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public var nGet: Swift.UInt64 { get }
	@inlinable public mutating func nSet(_ value: Swift.UInt64) throws
	@inlinable public var onGet: Swift.Bool { get }
	@inlinable public mutating func onSet(_ value: Swift.Bool) throws
	@inlinable public static func new(_ n: Swift.UInt64, _ on: Swift.Bool) -> Plain
}
public struct Labeled: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Labeled, b: borrowing Labeled) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func new(_ label: Swift.String) -> Labeled
}
#if compiler(>=6.4)
extension Labeled: Hashable {}
#endif
public struct Gauge: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Gauge
	@inlinable public static func new(_ level: Swift.UInt64) -> Gauge
	@inlinable public func set(_ level: Swift.UInt64) throws
	@inlinable public func level() throws -> Swift.UInt64
	@inlinable public consuming func intoLevel() throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public func later() async throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public func reaches(_ this: Swift.UInt64) async throws -> Swift.Bool
}
public struct Latch: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> Latch
	@inlinable public static func `default`() -> Latch
	@inlinable public func set(_ value: Swift.UInt32) -> Swift.Bool
	@inlinable public func get() -> Swift.UInt32?
}
public struct Token: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Token
	@inlinable public var idGet: Swift.UInt32 { get }
	@inlinable public mutating func idSet(_ value: Swift.UInt32) throws
	@inlinable public static func new(_ id: Swift.UInt32) -> Token
	@inlinable public func twice() throws -> UniqueArray<Token>
}
public struct Stack: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> Stack
	@inlinable public static func new() -> Stack
	@inlinable public mutating func push(_ item: consuming Counter) throws
	@inlinable public mutating func pop() throws -> Counter?
	@inlinable public func depth() throws -> Swift.UInt
	@inlinable public func depthPastPoint(_ from: Point) throws -> Swift.UInt64
	@inlinable public func depthPastCounter(_ from: borrowing Counter) throws -> Swift.UInt64
	@inlinable public func depthPastSize(_ from: Size) throws -> Swift.UInt64
	@inlinable public func depthPastWrapU32(_ from: WrapU32) throws -> Swift.UInt64
}
public final class Refusal: @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public final class Fault: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public struct C: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing C, b: borrowing C) -> Swift.Bool
	@inlinable public static func new(_ value: Swift.UInt8) -> C
}
#if compiler(>=6.4)
extension C: Equatable {}
#endif
public struct Quantity: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Quantity, b: borrowing Quantity) -> Swift.Bool
	@inlinable public static func < (a: borrowing Quantity, b: borrowing Quantity) -> Swift.Bool
	@inlinable public static func <= (a: borrowing Quantity, b: borrowing Quantity) -> Swift.Bool
	@inlinable public static func > (a: borrowing Quantity, b: borrowing Quantity) -> Swift.Bool
	@inlinable public static func >= (a: borrowing Quantity, b: borrowing Quantity) -> Swift.Bool
	@inlinable public static func new(_ value: Swift.Int64, _ unit: Swift.Unicode.Scalar) -> Quantity
}
#if compiler(>=6.4)
extension Quantity: Equatable {}
#endif
public struct Key: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Key
	@inlinable public static func == (a: borrowing Key, b: borrowing Key) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func new(_ name: Swift.String) -> Key
	@inlinable public func hashOf() -> Swift.UInt64
}
#if compiler(>=6.4)
extension Key: Hashable {}
#endif
public struct ClashSeed: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> ClashSeed
	@inlinable public static func default_() -> ClashSeed
	@inlinable public func value() -> Swift.UInt32
	@inlinable public func bool() -> Swift.Bool
}
public struct ClashEmptyString: ~Copyable, @unchecked Sendable {
	@inlinable public func isEmpty() -> Swift.Bool
}
public struct ClashStore: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ count: Swift.UInt32) -> ClashStore
	@inlinable public func free() -> Swift.UInt32
	@inlinable public func close() -> Swift.UInt32
}
public struct TlsSession: ~Copyable, @unchecked Sendable {
	@inlinable public func alpn() -> Swift.String
	@inlinable public func lifetime() -> TlsPolicyLifetime
}
public struct Url: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Url
	@inlinable public static func == (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public static func < (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public static func <= (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public static func > (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public static func >= (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func parse(_ input: Swift.String) throws -> Url
}
public struct Json: ~Copyable, @unchecked Sendable {
	@inlinable public static func parse(_ input: Swift.String) throws -> Json
	@inlinable public func toText() throws -> Swift.String
}
public final class Instant: @unchecked Sendable, CustomDebugStringConvertible, Hashable, Comparable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Instant
	@inlinable public static func == (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public static func < (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public static func <= (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public static func > (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public static func >= (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct StdIoError: ~Copyable, @unchecked Sendable {}
public struct TlsConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TlsConfig
}
public struct TlsCert: ~Copyable, @unchecked Sendable {}
public struct TlsResolver: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TlsResolver
}
public struct TlsCodec: ~Copyable, @unchecked Sendable {}
public struct Spawned: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ task: @escaping @Sendable () -> Swift.UInt32) throws -> Spawned
	@inlinable public consuming func join() throws -> Swift.UInt32
}
public struct LogThread: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ log: any Log, _ line: Swift.String) throws -> LogThread
	@inlinable public consuming func join() throws -> Swift.Bool
}
public struct Pinned: ~Copyable {
	@inlinable public func valueGet() throws -> Swift.UInt64
	@inlinable public mutating func valueSet(_ value: Swift.UInt64) throws
	@inlinable public static func new(_ value: Swift.UInt64) throws -> Pinned
	@inlinable public func part() throws -> Part
}
public struct Hooked: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Hooked, b: borrowing Hooked) throws -> Swift.Bool
	@inlinable public func nGet() throws -> Swift.UInt64
	@inlinable public mutating func nSet(_ value: Swift.UInt64) throws
	@inlinable public static func new(_ n: Swift.UInt64) throws -> Hooked
}
public struct Forged: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Forged, b: borrowing Forged) throws -> Swift.Bool
	@inlinable public func nGet() throws -> Swift.UInt64
	@inlinable public mutating func nSet(_ value: Swift.UInt64) throws
	@inlinable public static func new(_ n: Swift.UInt64) throws -> Forged
}
public struct Part: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: borrowing Part, b: borrowing Part) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public func valueGet() throws -> Swift.UInt64
	@inlinable public mutating func valueSet(_ value: Swift.UInt64) throws
}
public protocol Log: Sendable {
	func log(_ level: Swift.UInt8, _ line: Swift.String) -> Swift.Bool
}
public protocol Beat: Sendable {
	func beat() -> Swift.UInt32
}
public protocol Turner: Sendable {
	func by() -> Swift.UInt64
	func once() -> Swift.UInt64
}
public struct BoundsU64Error: Swift.Error, CustomStringConvertible {
	public let error: BoundsU64
	@inlinable public init(error: BoundsU64)
	@inlinable public var description: Swift.String { get }
}
public struct StringError: Swift.Error, CustomStringConvertible {
	public let error: Swift.String
	@inlinable public init(error: Swift.String)
	@inlinable public var description: Swift.String { get }
}
public struct VecClashToneError: Swift.Error, CustomStringConvertible {
	public let error: [ClashTone]
	@inlinable public init(error: [ClashTone])
	@inlinable public var description: Swift.String { get }
}
extension Error: Swift.Error {}
extension Fault: Swift.Error {}
extension Refusal: Swift.Error {}
public let LIMIT: Swift.UInt64
public let FAR: Swift.Int64
public let URL: Swift.UInt64
@available(*, deprecated, message: "pass a step to `Dial::turn`, whose `}` or `{` alone stays in the note") public let STEP: Swift.UInt64
@inlinable public func hasText(_ note: Note) -> Swift.Bool
@inlinable public func marked(_ apiMarked: Mark) throws -> Swift.UInt64
@inlinable public func reading(_ n: Swift.UInt32) -> Reading?
@inlinable public func readingDebug(_ n: Swift.UInt32) -> Swift.String
@inlinable public func readingEcho(_ reading: Reading?) -> Reading?
@inlinable public func readingBump(_ reading: inout Reading?)
@inlinable public func advance(_ stage: Stage) -> Stage
@inlinable public func only(_ counter: consuming Counter) throws -> Counter
@inlinable public func widths(_ n: Swift.UInt64) -> (Swift.UInt8, Swift.Int16, Swift.UInt32, Swift.UInt64)
@inlinable public func divide(_ a: Swift.UInt64, _ b: Swift.UInt64) throws -> (Swift.UInt64, Swift.UInt64)
@inlinable public func corner(_ unit: Swift.Bool) throws -> (Swift.UInt32, Swift.UInt32)
@inlinable nonisolated(nonsending) public func cornerLater() async throws -> (Swift.UInt32, Swift.UInt32)
@inlinable public func spread(_ n: Swift.UInt64) -> (Swift.UInt8, Swift.Int16, Swift.UInt32, Swift.UInt64)?
@inlinable public func find(_ haystack: Bytes, _ needle: Bytes) -> (Swift.UInt, Swift.UInt)?
@inlinable public func rows(_ points: [Point]) -> [(Swift.String, Shape, Point)]
@inlinable public func port(_ text: Swift.String) throws -> Swift.UInt16
@inlinable public func span(_ start: Swift.UInt32, _ end: Swift.UInt32) -> Swift.UInt32
@inlinable public func halves(_ span: Span) -> ((Swift.UInt32, Swift.UInt32), (Swift.UInt32, Swift.UInt32))
@inlinable public func doubled(_ value: Swift.UInt32??) -> Swift.UInt32??
@inlinable public func title(_ formal: Swift.Bool) -> Swift.String?
@inlinable public func named(_ name: Swift.String) -> Swift.String?
@inlinable public func lentBack(_ value: Bytes) -> Bytes
@inlinable public func boxedTuple(_ bytes: Bytes) -> Bytes
@inlinable public func swapped(_ pair: (Swift.String, Swift.UInt32)) -> (Swift.UInt32, Swift.String)
@inlinable public func lasting(_ draft: borrowing Draft) throws -> Swift.Bool
@inlinable public func stored(_ receipt: consuming Receipt) throws -> Swift.UInt32
@inlinable public func tallied(_ receipt: consuming Receipt) throws -> Swift.UInt32
@inlinable public func summedUpReceipt(_ totaled: consuming Receipt) throws -> Swift.UInt32
@inlinable public func erased(_ receipt: consuming Receipt) throws -> Swift.UInt32
@inlinable public func longer(_ a: consuming Name, _ b: borrowing Name) throws -> Name
@inlinable public func unnamed() -> Name
@inlinable public func total(_ range: RangeU64) -> Swift.UInt64
@inlinable public func reach(_ start: Swift.UInt32, _ end: Swift.UInt32) -> Swift.UInt32
@inlinable public func scaled(_ v: U128, _ by: Swift.UInt64) -> U128?
@inlinable public func pair() -> Batch2
@inlinable public func drain4(_ batch: inout Batch) throws -> Swift.UInt64
@inlinable public func drain2(_ batch: inout Batch2) throws -> Swift.UInt64
@inlinable public func capacities44(_ a: borrowing Batch, _ b: borrowing Batch) throws -> Swift.UInt
@inlinable public func capacities42(_ a: borrowing Batch, _ b: borrowing Batch2) throws -> Swift.UInt
@inlinable public func capacities24(_ a: borrowing Batch2, _ b: borrowing Batch) throws -> Swift.UInt
@inlinable public func capacities22(_ a: borrowing Batch2, _ b: borrowing Batch2) throws -> Swift.UInt
@inlinable nonisolated(nonsending) public func fill4(_ batch: inout Batch, _ v: Swift.UInt32) async throws -> [Swift.UInt32]
@inlinable nonisolated(nonsending) public func fill2(_ batch: inout Batch2, _ v: Swift.UInt32) async throws -> [Swift.UInt32]
@inlinable public func widen(_ range: RangeU64, _ by: Swift.UInt64) -> RangeU64
@inlinable public func count(_ range: BoundsU64) -> Swift.UInt
@inlinable public func end(_ limit: Swift.UInt64) -> BoundU64
@inlinable public func ends(_ limit: Swift.UInt64) -> (BoundU64, BoundU64)
@inlinable public func below(_ n: Swift.UInt64, _ limit: Swift.UInt64) throws -> Swift.UInt64
@inlinable nonisolated(nonsending) public func drive(_ start: Moment, _ poll: @escaping @Sendable (Moment, borrowing Waiter) throws -> Moment?) async throws -> Swift.String
@inlinable public func made(_ n: Swift.UInt32, _ make: @Sendable (Swift.UInt32) -> Swift.String) -> Swift.String
@inlinable public func tickets(_ entries: [(Swift.String, Ticket)]) -> [(Swift.String, Ticket)]
@inlinable public func reissue(_ ticket: Ticket, _ swap: @Sendable (Ticket) -> Ticket) -> Ticket
@inlinable public func reissueLent(_ ticket: Ticket, _ swap: @Sendable (Ticket) -> Ticket) -> Ticket
@inlinable public func seat(_ mark: Mark, _ row: Swift.UInt32) throws -> Seat
@inlinable public func admit(_ entry: consuming TupleCounterMarkSeatChoice) throws -> Swift.UInt64
@inlinable public func choose(_ choice: Choice) throws -> Swift.UInt64
@inlinable public func heft(_ marked: (Mark, Bytes), _ lent: Parcel, _ moved: consuming TupleCounterBytes) throws -> Swift.UInt64
@inlinable public func spend(_ e: consuming Counter, _ it: consuming Counter) throws -> Swift.UInt64
@inlinable public func isKept(_ status: Status) -> Swift.Bool
@inlinable public func attempt(_ check: @Sendable () throws -> Void) throws
@inlinable public func tail(_ file: inout some SeekableReader, _ len: Swift.UInt64) throws -> (Bytes, Swift.UInt64)
@inlinable public func seekTo(_ file: inout some SeekableReader, _ at: Swift.UInt64) throws -> Swift.UInt64
@inlinable nonisolated(nonsending) public func halved(_ n: Swift.UInt32) async throws -> Swift.UInt32?
@inlinable nonisolated(nonsending) public func yielded(_ value: Swift.UInt32) async throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func after(_ first: @escaping @Sendable () async -> Void, _ value: Swift.UInt32) async throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func within(_ polls: Swift.UInt32, _ future: @escaping @Sendable () async -> Void) async throws
@inlinable public func momentAt(_ ticks: Swift.UInt64, _ rate: Swift.UInt64) throws -> Moment
@inlinable public func stampAt(_ millis: Swift.UInt64) throws -> Stamp
@inlinable public func shifted(_ moment: Moment, _ shift: @Sendable (Moment) -> Moment) -> Moment
@inlinable public func jvmSum(_ field: JvmField, _ method: JvmStatic, _ opt: OptIn) -> Swift.UInt32
@inlinable public func clockedAfter(_ clocked: Clocked, _ ticks: Swift.UInt64) -> Clocked
@inlinable public func emit(_ log: any Log, _ lines: Bytes) -> Swift.UInt32
@inlinable nonisolated(nonsending) public func counted(_ n: Swift.UInt32, _ count: @Sendable (Swift.UInt32) -> Void) async throws -> Swift.UInt32
@inlinable public func filter(_ lines: Bytes, _ keep: @Sendable (Swift.UInt8, Swift.String) -> Swift.Bool) -> Swift.UInt32
@inlinable nonisolated(nonsending) public func logLater(_ log: any Log) async throws -> Swift.Bool
@inlinable public func logCounts(_ log: any Log, _ level: Swift.UInt8, _ counter: borrowing Counter, _ more: consuming UniqueArray<Counter>) throws -> Swift.UInt32
@inlinable public func onFirstThread() -> Swift.Bool
@inlinable public func localsDropped() -> Swift.UInt32
@inlinable public func localOrNew() throws -> Local
@inlinable public func localOrNew(local: borrowing Local) throws -> Local
@inlinable nonisolated(nonsending) public func localOrNewLater() async throws -> Local
@inlinable nonisolated(nonsending) public func localOrNewLater(local: borrowing Local) async throws -> Local
@inlinable public func bumpEither(_ second: borrowing Local) throws -> Swift.UInt32
@inlinable public func bumpEither(first: borrowing Local, _ second: borrowing Local) throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func localAfter() async throws -> Local
@inlinable nonisolated(nonsending) public func localLater(_ n: Swift.UInt32) async throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func pollTripled(_ n: Swift.UInt32) async throws -> Swift.UInt32
@inlinable public func watch(_ gauge: borrowing Gauge, _ watch: @Sendable (borrowing Gauge) -> Swift.UInt64) throws -> Swift.UInt64
@inlinable public func tokenBytes(_ id: Swift.UInt32, _ len: Swift.UInt32) -> Bytes
@inlinable public func tokenSlice(_ id: Swift.UInt32, _ len: Swift.UInt32, _ of: Swift.UInt32) -> Bytes
@inlinable public func nextPacket(_ packet: Packet) -> Packet
@inlinable public func fingerprint(_ hex: Swift.String) throws -> ArrayU8x4
@inlinable public func dropped() -> Swift.UInt32
@inlinable public func bumpBoth(_ a: borrowing Tally, _ b: borrowing Tally) throws -> Swift.UInt32
@inlinable public func offset(_ by: @Sendable () -> Swift.UInt32, _ byData: Swift.UInt32) -> Swift.UInt32
@inlinable public func shift(_ by: @Sendable () -> Swift.UInt32, _ cBy: Swift.UInt32) -> Swift.UInt32
@inlinable public func walk(_ steps: Swift.UInt64, _ visit: @Sendable (borrowing Cursor) -> Swift.UInt64) -> Swift.UInt64
@inlinable public func each(_ steps: Swift.UInt32, _ visit: @Sendable (Swift.UInt32) -> Void) -> Swift.UInt32
@inlinable public func peek(_ tally: borrowing Tally, _ peek: @Sendable (borrowing Tally) -> Swift.UInt32) throws -> Swift.UInt32
@inlinable public func len(_ payload: Bytes) -> Swift.UInt
@inlinable public func hex(_ payload: Bytes) -> Swift.String
@inlinable public func timedHex(_ payload: Bytes) -> Swift.String
@inlinable public func timedText(_ value: borrowing Json) throws -> Swift.String
@inlinable public func sampleHex(_ sample: Bytes) -> Swift.String
@inlinable public func queuedHex(_ queued: Bytes) -> Swift.String
@inlinable public func tickName(_ tick: Label) -> Swift.String
@inlinable public func rootedName(_ rooted: Label) -> Swift.String
@inlinable public func storedText(_ stored: Swift.String) -> Swift.String
@inlinable public func layerName(_ layer: Label) -> Swift.String
@inlinable public func greet(_ name: Swift.String, _ shape: Shape) -> Swift.String
@inlinable public func describe(_ event: Event) -> Swift.String
@inlinable public func tag(_ size: Size, _ tag: Swift.UInt8) -> PairSizeU8
@inlinable public func cross(_ left: LeftRightU8, _ right: RightLeftU8) -> Swift.UInt8
@inlinable public func keptPoint(_ from: Point) -> Swift.UInt64
@inlinable public func keptCounter(_ from: consuming Counter) throws -> Swift.UInt64
@inlinable public func keptSize(_ from: Size) -> Swift.UInt64
@inlinable public func keptWrapU32(_ from: WrapU32) -> Swift.UInt64
@inlinable public func keptPolarPoint(_ from: PolarPoint) -> Swift.UInt64
@inlinable public func keepPoint(_ kept: Point) -> Swift.Int32
@inlinable public func keptOrPoint(_ kept: Point?) -> Swift.Int32
@inlinable public func keeperPoint(_ keeper: Point) -> Swift.Int32
@inlinable public var BOXED: BoxedU16 { get }
@inlinable public func keeps(_ log: any Log) -> BoxedU8
@inlinable public func weighPoint(_ from: Point) -> Swift.UInt32
@inlinable public func applyNonZeroU16(_ pair: PairNonZeroU16, _ f: @Sendable (NonZero<Swift.UInt16>) -> Swift.UInt32) -> Swift.UInt32
@inlinable public func applyU32(_ pair: PairU32, _ f: @Sendable (Swift.UInt32) -> Swift.UInt32) -> Swift.UInt32
@inlinable public func reportNonZeroU16(_ pair: PairNonZeroU16, _ log: any Log) -> Swift.UInt32
@inlinable public func reportU32(_ pair: PairU32, _ log: any Log) -> Swift.UInt32
@inlinable public func tallyLogCount(_ log: any Log) -> Swift.UInt32
@inlinable public func wind(_ whole: PolarTurn, _ part: Polar_Turn) -> Swift.UInt32
@inlinable public func area(_ size: Size) -> Swift.UInt64
@inlinable public func seedsPointPoint(_ from: Point) -> Swift.UInt64
@inlinable public func seedsPointCounter(_ from: borrowing Counter) throws -> Swift.UInt64
@inlinable public func seedsPointSize(_ from: Size) -> Swift.UInt64
@inlinable public func seedsPointWrapU32(_ from: WrapU32) -> Swift.UInt64
@inlinable public func seedsPointPolarPoint(_ from: PolarPoint) -> Swift.UInt64
@inlinable public func seedsCounterPoint(_ from: Point) -> Swift.UInt64
@inlinable public func seedsCounterCounter(_ from: borrowing Counter) throws -> Swift.UInt64
@inlinable public func seedsCounterSize(_ from: Size) -> Swift.UInt64
@inlinable public func seedsCounterWrapU32(_ from: WrapU32) -> Swift.UInt64
@inlinable public func seedsCounterPolarPoint(_ from: PolarPoint) -> Swift.UInt64
@inlinable public func grow(_ area: borrowing Area, _ by: Swift.UInt32) -> Area
@inlinable public func wide(_ value: Swift.UInt32) -> SlotU32
@inlinable public func host(_ url: borrowing Url) throws -> Swift.String?
@inlinable public func resolve(_ base: consuming Url, _ path: Swift.String) throws -> Url
@inlinable public func ports(_ addrs: [SocketAddr]) -> [Swift.UInt16]
@inlinable public func doubledTtl(_ ttl: RustDuration) -> RustDuration
@inlinable public func file(_ dir: PathBuf, _ name: Swift.String) -> PathBuf
@inlinable public func stray() -> PathBuf
@inlinable public func visitStray(_ visit: @Sendable (PathBuf) -> Void)
@inlinable public func nextChar(_ c: Swift.Unicode.Scalar) -> Swift.Unicode.Scalar?
@inlinable public func extended(_ duration: RustDuration, _ by: RustDuration) -> RustDuration?
@inlinable public func parents(_ paths: [PathBuf]) -> [PathBuf?]
@inlinable public func words(_ text: Swift.String) -> [Swift.String]
@inlinable public func sentence(_ words: [Swift.String]?) -> Swift.String
@inlinable public func putNumber(_ value: Swift.UInt32, _ buf: inout Bytes)
@inlinable public func secondLater(_ w235: RustDuration) -> RustDuration
@inlinable public func digit(_ r: inout Bytes) throws -> Swift.UInt8
@inlinable public func pushCounted(_ entry: inout (Bytes, Swift.UInt32), _ byte: Swift.UInt8) -> Swift.UInt32
@inlinable public func nudge(_ point: inout Point, _ by: Swift.Int32)
@inlinable public func nudgeBy(_ point: inout Point, _ refusal: Refusal) throws
@inlinable public func secure(_ alpn: Swift.String) -> TlsConfig
@inlinable public func alpn(_ config: consuming TlsConfig) throws -> Swift.String
@inlinable public func alpnLent(_ config: borrowing TlsConfig) throws -> Swift.String
@inlinable public func certify(_ der: Bytes) -> TlsCert
@inlinable nonisolated(nonsending) public func byteLen(_ bytes: Bytes) async throws -> Swift.UInt
@inlinable public func certLen(_ cert: borrowing TlsCert) throws -> Swift.UInt
@inlinable public func certDer(_ cert: consuming TlsCert) throws -> Bytes
@inlinable public func chainLen(_ chain: consuming UniqueArray<TlsCert>) throws -> Swift.UInt
@inlinable public func verify(_ cert: borrowing TlsCert) throws -> Swift.UInt32
@inlinable public func parseCert(_ text: Swift.String) throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func parseLater(_ text: Swift.String) async throws -> Swift.UInt32
@inlinable public func resolver(_ name: Swift.String) -> TlsResolver
@inlinable public func resolved(_ resolver: consuming TlsResolver) throws -> Swift.String
@inlinable public func failure(_ message: Swift.String) -> StdIoError
@inlinable public func failureMessage(_ error: borrowing StdIoError) throws -> Swift.String
@inlinable public func failureKind(_ error: consuming StdIoError) throws -> Swift.String
@inlinable public func sumAll(_ items: [Swift.UInt32]) -> Swift.UInt32
@inlinable public func weighAllCounter(_ items: consuming UniqueArray<Counter>) throws -> Swift.UInt64
@inlinable public func offeredLen(_ alpns: [Bytes]) -> Swift.UInt
@inlinable public func joinWords(_ words: [Swift.String]) -> Swift.String
@inlinable public func resume(_ alpn: Swift.String, _ seconds: Swift.UInt32) throws -> TlsSession
@inlinable public func either(_ text: Swift.String?) -> Swift.UInt
@inlinable nonisolated(nonsending) public func eitherLater(_ text: Swift.String?) async throws -> Swift.UInt
@inlinable public func prefixed(_ payload: Bytes, _ prefix: Bytes?) -> Bytes
@inlinable public func sumLists(_ a: [Swift.Int32], _ b: [Swift.Int32]?, _ c: [Swift.Int32]?) -> Swift.Int32
@inlinable public func coordinates(_ point: Point?) -> Swift.Int32
@inlinable public func chunks(_ bytes: Bytes, _ size: Swift.UInt) -> [Bytes]
@inlinable public func counters(_ names: [Swift.String]) -> UniqueArray<Counter>
@inlinable public func worth(_ some: consuming UniqueArray<Counter>?, _ boxed: consuming UniqueArray<Counter>) throws -> Swift.UInt64
@inlinable public func heldCount(_ held: consuming Counter?) throws -> Swift.UInt64?
@inlinable public func boxedCounter(_ name: Swift.String?) -> Counter?
@inlinable public func boxedName(_ boxed: consuming Counter?) throws -> Swift.String?
@inlinable public func less(_ value: Swift.UInt32, _ value2: Swift.UInt32) -> Swift.UInt32
@inlinable public func sums(_ rows: [[Swift.UInt32]]) -> [Swift.UInt32]
@inlinable public func least(_ sets: [[Swift.UInt32]]) -> [[Swift.UInt32]]
@inlinable public func splitLast(_ items: [Swift.Int32]) -> ([Swift.Int32], Swift.UInt32)
@inlinable public func sumU32(_ items: [Swift.UInt32]) -> Swift.UInt64
@inlinable public func doubledI64(_ items: [Swift.Int64]) -> [Swift.Int64]
@inlinable public func boxedFloats(_ items: [Swift.Double]) -> [Swift.Double]
@inlinable public func summed(_ points: [Point]) -> Point
@inlinable public func breadth(_ limits: LimitsDuration) -> RustDuration
@inlinable public func byteLimits(_ low: Bytes, _ high: Bytes) -> LimitsBytes
@inlinable public func joined(_ parts: [Bytes]) -> Bytes
@inlinable public func firstOf(_ items: [Swift.String]) -> Swift.String?
@inlinable public func readLen(_ reader: Bytes) -> Swift.UInt64
@inlinable public func longest(_ words: [Swift.String]) -> Swift.UInt
@inlinable public func encode(_ value: borrowing Json) throws -> Swift.String
@inlinable public func tagged(_ doc: consuming Json, _ tag: Swift.String) throws -> Json
@inlinable public func entries(_ entries: consuming UniqueArray<TupleStringJson>, _ tag: Swift.String) throws -> UniqueArray<TupleStringJson>
@inlinable public func sections(_ entries: consuming UniqueArray<TupleStringJson>) throws -> UniqueArray<TupleStringJson>
@inlinable public func now() -> Instant
@inlinable public func later(_ at: Instant, _ by: RustDuration) -> Instant
@inlinable public func since(_ later: Instant, _ earlier: Instant) -> RustDuration
@inlinable public func hasInstant(_ at: Instant?) -> Swift.Bool
@inlinable public func louder(_ filter: Level?) -> Level?
@inlinable public func spell(_ filter: Level?) -> Swift.String
@inlinable public var GREETINGS: [Swift.String] { get }
@inlinable public var PRIMES: [Swift.UInt32] { get }
@inlinable public func numbered(_ names: [Swift.String]) -> UniqueArray<Counter>
@inlinable public func nexts(_ v4: SocketAddrV4, _ v6: SocketAddrV6) -> (SocketAddrV4, SocketAddrV6)
@inlinable public func nextIps(_ v4: Ipv4Addr, _ v6: Ipv6Addr) -> (Ipv4Addr, Ipv6Addr)
@inlinable public func flip(_ ip: IpAddr) -> IpAddr
@inlinable public func reasons(_ pairs: consuming UniqueArray<TupleCounterRefusal>, _ last: consuming TupleCounterRefusal?) throws -> Swift.String
@inlinable public func excuse(_ note: Bytes, _ entry: (Bytes, Refusal)) throws -> Swift.UInt
@inlinable public func relay(_ log: any Log, _ refusals: [Refusal]) throws -> Swift.UInt32
@inlinable public func refuse(_ reason: Swift.String) throws -> Swift.UInt32
@inlinable public func jam(_ n: Swift.UInt32) throws -> Swift.UInt32
@inlinable public func platform() -> Platform
@inlinable public func pointerBits(_ platform: Platform) -> Swift.UInt32
@inlinable public func sameLabelled(_ labelled: Labelled) -> Labelled
@inlinable public func placement() -> Placement
@inlinable public func isFamily(_ family: Family) -> Swift.Bool
@inlinable public func wasmOnly() throws -> Swift.UInt32
@inlinable public func wasmTake(_ counter: consuming Counter) throws -> Swift.UInt64
@inlinable public func wasmLog(_ log: any Log) throws
@inlinable public func boothCount(_ booth: Booth) throws -> Swift.UInt64
@inlinable public func page() throws -> Page
@inlinable public func beats(_ beat: any Beat) throws -> Swift.UInt32
@inlinable public func same(_ a: Swift.Double, _ b: Swift.Double) -> Swift.Int32
@inlinable public func returnValues(_ point: inout Point, _ fail: Swift.Bool) throws -> (Swift.String, (Swift.UInt8, Swift.Int16, Swift.UInt32, Swift.UInt64)?, [Bytes])
@inlinable public func returnMoved(_ counter: consuming Counter, _ fail: Swift.Bool) throws -> Counter
@inlinable public func nonZeros() -> NonZeros
@inlinable public func describeNonZeros(_ nonZeros: NonZeros) -> Swift.String
@inlinable public func nextHop(_ hop: NonZero<Swift.UInt64>?) -> NonZero<Swift.UInt64>?
@inlinable public func fartherHop(_ hops: PairNonZeroU16) -> NonZero<Swift.UInt16>
@inlinable public func sameDuration(_ v: RustDuration) -> RustDuration
@inlinable public func sameU64(_ v: Swift.UInt64) -> Swift.UInt64
@inlinable public func sameUsize(_ v: Swift.UInt) -> Swift.UInt
@inlinable public func sameIsize(_ v: Swift.Int) -> Swift.Int
@inlinable public func sameU128(_ v: U128) -> U128
@inlinable public func sameChar(_ v: Swift.Unicode.Scalar) -> Swift.Unicode.Scalar
@inlinable public func sameString(_ v: Swift.String) -> Swift.String
@inlinable public func samePath(_ v: PathBuf) -> PathBuf
@inlinable public func sameSocketAddr(_ v: SocketAddr) -> SocketAddr
@inlinable public func sameSocketAddrV4(_ v: SocketAddrV4) -> SocketAddrV4
@inlinable public func sameSocketAddrV6(_ v: SocketAddrV6) -> SocketAddrV6
@inlinable public func sameIpAddr(_ v: IpAddr) -> IpAddr
@inlinable public func deprecated(_ note: Swift.UInt8) -> Swift.UInt8
@available(*, deprecated) @inlinable public func knob(_ dial: Dial) -> Knob
@available(*, deprecated, message: "read its `level`") @inlinable nonisolated(nonsending) public func levelLater(_ dial: Dial) async throws -> Swift.UInt64
@inlinable public func turned(_ dial: Dial, _ turner: any Turner) throws -> Dial
@inlinable public func crowded(_ reading: Reading, _ a: Stamp, _ b: Stamp, _ n0: Swift.UInt64, _ n1: Swift.UInt64, _ n2: Swift.UInt64, _ n3: Swift.UInt64, _ n4: Swift.UInt64, _ n5: Swift.UInt64, _ n6: Swift.UInt64, _ n7: Swift.UInt64, _ n8: Swift.UInt64, _ n9: Swift.UInt64, _ n10: Swift.UInt64, _ n11: Swift.UInt64, _ n12: Swift.UInt64, _ n13: Swift.UInt64, _ n14: Swift.UInt64, _ n15: Swift.UInt64, _ n16: Swift.UInt64, _ n17: Swift.UInt64, _ n18: Swift.UInt64, _ n19: Swift.UInt64, _ n20: Swift.UInt64, _ n21: Swift.UInt64, _ n22: Swift.UInt64, _ n23: Swift.UInt64, _ n24: Swift.UInt64, _ n25: Swift.UInt64, _ n26: Swift.UInt64, _ n27: Swift.UInt64, _ n28: Swift.UInt64, _ n29: Swift.UInt64, _ n30: Swift.UInt64, _ n31: Swift.UInt64, _ n32: Swift.UInt64, _ n33: Swift.UInt64, _ n34: Swift.UInt64, _ n35: Swift.UInt64, _ n36: Swift.UInt64, _ n37: Swift.UInt64, _ n38: Swift.UInt64, _ n39: Swift.UInt64, _ n40: Swift.UInt64, _ n41: Swift.UInt64, _ n42: Swift.UInt64, _ n43: Swift.UInt64, _ n44: Swift.UInt64, _ n45: Swift.UInt64, _ n46: Swift.UInt64, _ n47: Swift.UInt64, _ n48: Swift.UInt64, _ n49: Swift.UInt64, _ n50: Swift.UInt64, _ n51: Swift.UInt64, _ n52: Swift.UInt64, _ n53: Swift.UInt64, _ n54: Swift.UInt64, _ n55: Swift.UInt64, _ n56: Swift.UInt64, _ n57: Swift.UInt64, _ n58: Swift.UInt64, _ n59: Swift.UInt64, _ n60: Swift.UInt64, _ n61: Swift.UInt64, _ n62: Swift.UInt64, _ n63: Swift.UInt64, _ n64: Swift.UInt64, _ n65: Swift.UInt64, _ n66: Swift.UInt64, _ n67: Swift.UInt64, _ n68: Swift.UInt64, _ n69: Swift.UInt64, _ n70: Swift.UInt64, _ n71: Swift.UInt64, _ n72: Swift.UInt64, _ n73: Swift.UInt64, _ n74: Swift.UInt64, _ n75: Swift.UInt64, _ n76: Swift.UInt64, _ n77: Swift.UInt64, _ n78: Swift.UInt64, _ n79: Swift.UInt64, _ n80: Swift.UInt64, _ n81: Swift.UInt64, _ n82: Swift.UInt64, _ n83: Swift.UInt64, _ n84: Swift.UInt64, _ n85: Swift.UInt64, _ n86: Swift.UInt64, _ n87: Swift.UInt64, _ n88: Swift.UInt64, _ n89: Swift.UInt64, _ n90: Swift.UInt64, _ n91: Swift.UInt64, _ n92: Swift.UInt64, _ n93: Swift.UInt64, _ n94: Swift.UInt64, _ n95: Swift.UInt64, _ n96: Swift.UInt64, _ n97: Swift.UInt64, _ n98: Swift.UInt64, _ n99: Swift.UInt64, _ n100: Swift.UInt64, _ n101: Swift.UInt64, _ n102: Swift.UInt64, _ n103: Swift.UInt64, _ n104: Swift.UInt64, _ n105: Swift.UInt64, _ n106: Swift.UInt64, _ n107: Swift.UInt64, _ n108: Swift.UInt64, _ n109: Swift.UInt64, _ n110: Swift.UInt64, _ n111: Swift.UInt64, _ n112: Swift.UInt64, _ n113: Swift.UInt64, _ n114: Swift.UInt64, _ n115: Swift.UInt64, _ n116: Swift.UInt64, _ n117: Swift.UInt64, _ n118: Swift.UInt64) -> Swift.UInt64
@inlinable public func crammed(_ a: Moment, _ b: Moment, _ n0: Swift.UInt64, _ n1: Swift.UInt64, _ n2: Swift.UInt64, _ n3: Swift.UInt64, _ n4: Swift.UInt64, _ n5: Swift.UInt64, _ n6: Swift.UInt64, _ n7: Swift.UInt64, _ n8: Swift.UInt64, _ n9: Swift.UInt64, _ n10: Swift.UInt64, _ n11: Swift.UInt64, _ n12: Swift.UInt64, _ n13: Swift.UInt64, _ n14: Swift.UInt64, _ n15: Swift.UInt64, _ n16: Swift.UInt64, _ n17: Swift.UInt64, _ n18: Swift.UInt64, _ n19: Swift.UInt64, _ n20: Swift.UInt64, _ n21: Swift.UInt64, _ n22: Swift.UInt64, _ n23: Swift.UInt64, _ n24: Swift.UInt64, _ n25: Swift.UInt64, _ n26: Swift.UInt64, _ n27: Swift.UInt64, _ n28: Swift.UInt64, _ n29: Swift.UInt64, _ n30: Swift.UInt64, _ n31: Swift.UInt64, _ n32: Swift.UInt64, _ n33: Swift.UInt64, _ n34: Swift.UInt64, _ n35: Swift.UInt64, _ n36: Swift.UInt64, _ n37: Swift.UInt64, _ n38: Swift.UInt64, _ n39: Swift.UInt64, _ n40: Swift.UInt64, _ n41: Swift.UInt64, _ n42: Swift.UInt64, _ n43: Swift.UInt64, _ n44: Swift.UInt64, _ n45: Swift.UInt64, _ n46: Swift.UInt64, _ n47: Swift.UInt64, _ n48: Swift.UInt64, _ n49: Swift.UInt64, _ n50: Swift.UInt64, _ n51: Swift.UInt64, _ n52: Swift.UInt64, _ n53: Swift.UInt64, _ n54: Swift.UInt64, _ n55: Swift.UInt64, _ n56: Swift.UInt64, _ n57: Swift.UInt64, _ n58: Swift.UInt64, _ n59: Swift.UInt64, _ n60: Swift.UInt64, _ n61: Swift.UInt64, _ n62: Swift.UInt64, _ n63: Swift.UInt64, _ n64: Swift.UInt64, _ n65: Swift.UInt64, _ n66: Swift.UInt64, _ n67: Swift.UInt64, _ n68: Swift.UInt64, _ n69: Swift.UInt64, _ n70: Swift.UInt64, _ n71: Swift.UInt64, _ n72: Swift.UInt64, _ n73: Swift.UInt64, _ n74: Swift.UInt64, _ n75: Swift.UInt64, _ n76: Swift.UInt64, _ n77: Swift.UInt64, _ n78: Swift.UInt64, _ n79: Swift.UInt64, _ n80: Swift.UInt64, _ n81: Swift.UInt64, _ n82: Swift.UInt64, _ n83: Swift.UInt64, _ n84: Swift.UInt64, _ n85: Swift.UInt64, _ n86: Swift.UInt64, _ n87: Swift.UInt64, _ n88: Swift.UInt64, _ n89: Swift.UInt64, _ n90: Swift.UInt64, _ n91: Swift.UInt64, _ n92: Swift.UInt64, _ n93: Swift.UInt64, _ n94: Swift.UInt64, _ n95: Swift.UInt64, _ n96: Swift.UInt64, _ n97: Swift.UInt64, _ n98: Swift.UInt64, _ n99: Swift.UInt64, _ n100: Swift.UInt64, _ n101: Swift.UInt64, _ n102: Swift.UInt64, _ n103: Swift.UInt64, _ n104: Swift.UInt64, _ n105: Swift.UInt64, _ n106: Swift.UInt64, _ n107: Swift.UInt64, _ n108: Swift.UInt64, _ n109: Swift.UInt64, _ n110: Swift.UInt64, _ n111: Swift.UInt64, _ n112: Swift.UInt64, _ n113: Swift.UInt64, _ n114: Swift.UInt64, _ n115: Swift.UInt64, _ n116: Swift.UInt64, _ n117: Swift.UInt64, _ n118: Swift.UInt64, _ n119: Swift.UInt64, _ n120: Swift.UInt64, _ n121: Swift.UInt64, _ n122: Swift.UInt64, _ n123: Swift.UInt64) -> Swift.UInt64
@inlinable public func jniOne() -> Swift.UInt32
@inlinable public func polarPoint(_ distance: Swift.UInt32) -> PolarPoint?
@inlinable public func clashAny(_ typeUrl: Swift.String) -> ClashAny
@inlinable public func clashCandidate(_ at: Swift.UInt8) -> ClashCandidate
@inlinable public func clashIgnore(_ arg0: Swift.UInt8, _ arg1: Swift.UInt8, _ arg2: Swift.UInt8) -> Swift.UInt8
@inlinable public func clashEmpty() -> ClashEmptyString
@inlinable public func clashAudible(_ tone: ClashTone) throws -> ClashTone
@inlinable public func clashNested(_ a: ((Swift.UInt8, Swift.UInt16), Swift.UInt32)?, _ b: (Swift.UInt8, Swift.UInt16, Swift.UInt32)?) -> Swift.UInt32
@inlinable public func clashAudioEncoderFinish() -> Swift.UInt8
@inlinable public func clashAudioEncoderFinish_() -> Swift.UInt8
@inlinable public func emitFromThread(_ log: any Log, _ lines: Bytes) throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func emitLater(_ log: any Log, _ lines: Bytes) async throws -> Swift.UInt32
@inlinable nonisolated(nonsending) public func logOnDrop(_ log: any Log) async throws
@inlinable public func logWaker(_ log: any Log) throws -> @Sendable () -> Void
@inlinable public func hook(_ f: @escaping @Sendable () -> Void) throws
@inlinable public func unhook() throws
