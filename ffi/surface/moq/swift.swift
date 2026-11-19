// moq.swift
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
@frozen public struct TupleSessionDriver: ~Copyable, Sendable {
	public var _0: Session
	public var _1: Driver
	@inlinable public init(_ _0: consuming Session, _ _1: consuming Driver)
}
@frozen public struct TupleAuthCredentialAuthAnswer: ~Copyable, Sendable {
	public var _0: AuthCredential
	public var _1: AuthAnswer
	@inlinable public init(_ _0: consuming AuthCredential, _ _1: AuthAnswer)
}
@frozen public struct TupleMoqTokioQuicDriverSendStreamMoqTokioQuicDriverRecvStream: ~Copyable, Sendable {
	public var _0: MoqTokioQuicDriverSendStream
	public var _1: MoqTokioQuicDriverRecvStream
	@inlinable public init(_ _0: consuming MoqTokioQuicDriverSendStream, _ _1: consuming MoqTokioQuicDriverRecvStream)
}
@frozen public struct TupleOriginProducerOriginDriver: ~Copyable, Sendable {
	public var _0: OriginProducer
	public var _1: OriginDriver
	@inlinable public init(_ _0: consuming OriginProducer, _ _1: consuming OriginDriver)
}
@frozen public struct TupleStatsTierStatsMessageStatsTotals: ~Copyable, Sendable {
	public var _0: StatsTier
	public var _1: StatsMessage
	public var _2: StatsTotals
	@inlinable public init(_ _0: consuming StatsTier, _ _1: StatsMessage, _ _2: StatsTotals)
}
@frozen public struct TupleStatsTierStatsPresence: ~Copyable, Sendable {
	public var _0: StatsTier
	public var _1: StatsPresence
	@inlinable public init(_ _0: consuming StatsTier, _ _1: StatsPresence)
}
@frozen public struct TupleStatsTierStatsRoleStatsTraffic: ~Copyable, Sendable {
	public var _0: StatsTier
	public var _1: StatsRole
	public var _2: StatsTraffic
	@inlinable public init(_ _0: consuming StatsTier, _ _1: StatsRole, _ _2: StatsTraffic)
}
@frozen public struct TupleStringJson: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: Json
	@inlinable public init(_ _0: Swift.String, _ _1: consuming Json)
}
@frozen public struct TupleStringHangCatalogAudioConfig: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogAudioConfig
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogAudioConfig)
}
@frozen public struct TupleStringHangCatalogBinaryConfig: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogBinaryConfig
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogBinaryConfig)
}
@frozen public struct TupleStringHangCatalogContribution: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogContribution
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogContribution)
}
@frozen public struct TupleStringHangCatalogJsonConfig: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogJsonConfig
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogJsonConfig)
}
@frozen public struct TupleStringHangCatalogTextConfig: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogTextConfig
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogTextConfig)
}
@frozen public struct TupleStringHangCatalogVideoConfig: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogVideoConfig
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogVideoConfig)
}
@frozen public struct TupleStringHangEchoRendition: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangEchoRendition
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangEchoRendition)
}
@frozen public struct TupleStringHangStatsRendition: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangStatsRendition
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangStatsRendition)
}
@frozen public struct TupleStringMuxContainerTsTrack: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: MuxContainerTsTrack
	@inlinable public init(_ _0: Swift.String, _ _1: consuming MuxContainerTsTrack)
}
@frozen public struct TupleTransportMemorySessionTransportMemorySession: ~Copyable, Sendable {
	public var _0: TransportMemorySession
	public var _1: TransportMemorySession
	@inlinable public init(_ _0: consuming TransportMemorySession, _ _1: consuming TransportMemorySession)
}
@frozen public struct TupleU16MuxContainerTsStatsStream: ~Copyable, Sendable {
	public var _0: Swift.UInt16
	public var _1: MuxContainerTsStatsStream
	@inlinable public init(_ _0: Swift.UInt16, _ _1: consuming MuxContainerTsStatsStream)
}
@frozen public struct TupleU16VecTupleU8MuxContainerTsSiEntry: ~Copyable, Sendable {
	public var _0: Swift.UInt16
	public var _1: UniqueArray<TupleU8MuxContainerTsSiEntry>
	@inlinable public init(_ _0: Swift.UInt16, _ _1: consuming UniqueArray<TupleU8MuxContainerTsSiEntry>)
}
@frozen public struct TupleU8MuxContainerTsSiEntry: ~Copyable, Sendable {
	public var _0: Swift.UInt8
	public var _1: MuxContainerTsSiEntry
	@inlinable public init(_ _0: Swift.UInt8, _ _1: consuming MuxContainerTsSiEntry)
}
public struct ArrayU8x12: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x12, b: ArrayU8x12) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x16: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x16, b: ArrayU8x16) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x2: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x2, b: ArrayU8x2) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x3: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x3, b: ArrayU8x3) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x32: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x32, b: ArrayU8x32) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x4: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x4, b: ArrayU8x4) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ArrayU8x6: Swift.Hashable, Swift.Sendable, Swift.MutableCollection, Swift.RandomAccessCollection {
	@inlinable public init?(_ bytes: some Swift.Collection<Swift.UInt8>)
	@inlinable public var startIndex: Swift.Int { get }
	@inlinable public var endIndex: Swift.Int { get }
	@inlinable public subscript(i: Swift.Int) -> Swift.UInt8 { get set }
	@inlinable public static func == (a: ArrayU8x6, b: ArrayU8x6) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct TimeOverflow: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TimeOverflow, b: TimeOverflow) -> Swift.Bool
}
public struct Timescale: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Timescale, b: Timescale) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var SECOND: Timescale { get }
	@inlinable public static var MILLI: Timescale { get }
	@inlinable public static var MICRO: Timescale { get }
	@inlinable public static var NANO: Timescale { get }
	@inlinable public static func new(_ unitsPerSecond: Swift.UInt64) throws -> Timescale
	@inlinable public func asU64() -> Swift.UInt64
}
public struct Timestamp: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Timestamp, b: Timestamp) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var ZERO: Timestamp { get }
	@inlinable public static func new(_ value: Swift.UInt64, _ scale: Timescale) throws -> Timestamp
	@inlinable public static func fromScale(_ value: Swift.UInt64, _ unitsPerSecond: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromSecs(_ seconds: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromMillis(_ millis: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromMicros(_ micros: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromNanos(_ nanos: Swift.UInt64) throws -> Timestamp
	@inlinable public func value() -> Swift.UInt64
	@inlinable public func scale() -> Timescale
	@inlinable public func isZero() -> Swift.Bool
	@inlinable public func convert(_ newScale: Timescale) throws -> Timestamp
	@inlinable public func asScale(_ target: Timescale) -> U128
	@inlinable public func asSecs() -> Swift.UInt64
	@inlinable public func asMillis() -> U128
	@inlinable public func asMicros() -> U128
	@inlinable public func asNanos() -> U128
	@inlinable public func checkedAdd(_ rhs: Timestamp) throws -> Timestamp
	@inlinable public func checkedSub(_ rhs: Timestamp) throws -> Timestamp
	@inlinable public static func now() -> Timestamp
}
public struct Datagram: Sendable, CustomDebugStringConvertible, Equatable {
	public var sequence: Swift.UInt64
	public var timestamp: Timestamp?
	public var payload: Bytes
	@inlinable public init(sequence: Swift.UInt64, timestamp: Timestamp?, payload: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Datagram, b: Datagram) -> Swift.Bool
}
public struct Hop: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Hop, b: Hop) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var UNKNOWN: Hop { get }
	@inlinable public static func new(_ id: Swift.UInt64) throws -> Hop
	@inlinable public static func random() -> Hop
	@inlinable public func id() -> Swift.UInt64
}
public struct ClockInstant: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClockInstant, b: ClockInstant) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var ZERO: ClockInstant { get }
	@inlinable public static func now() -> ClockInstant
	@inlinable public func elapsed() -> RustDuration
	@inlinable public func durationSince(_ earlier: ClockInstant) -> RustDuration
	@inlinable public func checkedDurationSince(_ earlier: ClockInstant) -> RustDuration?
	@inlinable public func saturatingDurationSince(_ earlier: ClockInstant) -> RustDuration
	@inlinable public func checkedAdd(_ duration: RustDuration) -> ClockInstant?
	@inlinable public func checkedSub(_ duration: RustDuration) -> ClockInstant?
	@inlinable public func intoStd() -> Instant
	@inlinable public static func fromStd(_ at: Instant) -> ClockInstant
}
public struct ClockElapsed: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClockElapsed, b: ClockElapsed) -> Swift.Bool
}
public struct ClockWall: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClockWall, b: ClockWall) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var UNIXEPOCH: ClockWall { get }
	@inlinable public static func now() -> ClockWall
	@inlinable public func checkedDurationSince(_ earlier: ClockWall) -> RustDuration?
	@inlinable public func saturatingDurationSince(_ earlier: ClockWall) -> RustDuration
	@inlinable public func checkedAdd(_ duration: RustDuration) -> ClockWall?
	@inlinable public func saturatingAdd(_ duration: RustDuration) -> ClockWall
	@inlinable public func checkedSub(_ duration: RustDuration) -> ClockWall?
}
public struct SetupExtensions: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: SetupExtensions, b: SetupExtensions) -> Swift.Bool
	@inlinable public static func `default`() -> SetupExtensions
	@inlinable public var authGet: Swift.Bool { get }
	@inlinable public mutating func authSet(_ value: Swift.Bool)
	@inlinable public var solicitGet: Swift.Bool { get }
	@inlinable public mutating func solicitSet(_ value: Swift.Bool)
	@inlinable public var ietfGet: Swift.Bool { get }
	@inlinable public mutating func ietfSet(_ value: Swift.Bool)
}
public struct SetupToken: Sendable, CustomDebugStringConvertible, Equatable {
	public var kind: Swift.UInt64
	public var value: Bytes
	@inlinable public init(kind: Swift.UInt64, value: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: SetupToken, b: SetupToken) -> Swift.Bool
}
public struct SessionLimits: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: SessionLimits, b: SessionLimits) -> Swift.Bool
	@inlinable public static func `default`() -> SessionLimits
	@inlinable public var announcesGet: Swift.UInt { get }
	@inlinable public mutating func announcesSet(_ value: Swift.UInt)
}
public struct StatsContent: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsContent, b: StatsContent) -> Swift.Bool
	@inlinable public static func `default`() -> StatsContent
	@inlinable public var bytesGet: Swift.UInt64 { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64)
	@inlinable public var framesGet: Swift.UInt64 { get }
	@inlinable public mutating func framesSet(_ value: Swift.UInt64)
	@inlinable public var groupsGet: Swift.UInt64 { get }
	@inlinable public mutating func groupsSet(_ value: Swift.UInt64)
}
public struct StatsTraffic: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsTraffic, b: StatsTraffic) -> Swift.Bool
	@inlinable public static func `default`() -> StatsTraffic
	@inlinable public var announcesStartedGet: Swift.UInt64 { get }
	@inlinable public mutating func announcesStartedSet(_ value: Swift.UInt64)
	@inlinable public var announcesEndedGet: Swift.UInt64 { get }
	@inlinable public mutating func announcesEndedSet(_ value: Swift.UInt64)
	@inlinable public var announcedBytesGet: Swift.UInt64 { get }
	@inlinable public mutating func announcedBytesSet(_ value: Swift.UInt64)
	@inlinable public var broadcastsStartedGet: Swift.UInt64 { get }
	@inlinable public mutating func broadcastsStartedSet(_ value: Swift.UInt64)
	@inlinable public var broadcastsEndedGet: Swift.UInt64 { get }
	@inlinable public mutating func broadcastsEndedSet(_ value: Swift.UInt64)
	@inlinable public var subscriptionsStartedGet: Swift.UInt64 { get }
	@inlinable public mutating func subscriptionsStartedSet(_ value: Swift.UInt64)
	@inlinable public var subscriptionsEndedGet: Swift.UInt64 { get }
	@inlinable public mutating func subscriptionsEndedSet(_ value: Swift.UInt64)
	@inlinable public var fetchesGet: Swift.UInt64 { get }
	@inlinable public mutating func fetchesSet(_ value: Swift.UInt64)
	@inlinable public var bytesGet: Swift.UInt64 { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64)
	@inlinable public var framesGet: Swift.UInt64 { get }
	@inlinable public mutating func framesSet(_ value: Swift.UInt64)
	@inlinable public var groupsGet: Swift.UInt64 { get }
	@inlinable public mutating func groupsSet(_ value: Swift.UInt64)
	@inlinable public var datagramsGet: Swift.UInt64 { get }
	@inlinable public mutating func datagramsSet(_ value: Swift.UInt64)
	@inlinable public var inflatedGet: Swift.UInt64 { get }
	@inlinable public mutating func inflatedSet(_ value: Swift.UInt64)
	@inlinable public var staleGet: StatsContent { get }
	@inlinable public mutating func staleSet(_ value: StatsContent)
	@inlinable public mutating func add(_ other: StatsTraffic)
	@inlinable public func activeBroadcasts() -> Swift.UInt64
	@inlinable public func activeSubscriptions() -> Swift.UInt64
	@inlinable public func isIdle() -> Swift.Bool
}
public struct StatsPresence: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsPresence, b: StatsPresence) -> Swift.Bool
	@inlinable public static func `default`() -> StatsPresence
	@inlinable public var sessionsRefusedGet: StatsRefusals { get }
	@inlinable public mutating func sessionsRefusedSet(_ value: StatsRefusals)
	@inlinable public var sessionsFailedGet: StatsFailures { get }
	@inlinable public mutating func sessionsFailedSet(_ value: StatsFailures)
	@inlinable public var requestsGet: Swift.UInt64 { get }
	@inlinable public mutating func requestsSet(_ value: Swift.UInt64)
	@inlinable public var sessionsStartedGet: Swift.UInt64 { get }
	@inlinable public mutating func sessionsStartedSet(_ value: Swift.UInt64)
	@inlinable public var sessionsEndedGet: Swift.UInt64 { get }
	@inlinable public mutating func sessionsEndedSet(_ value: Swift.UInt64)
	@inlinable public var sentBytesGet: Swift.UInt64 { get }
	@inlinable public mutating func sentBytesSet(_ value: Swift.UInt64)
	@inlinable public var receivedBytesGet: Swift.UInt64 { get }
	@inlinable public mutating func receivedBytesSet(_ value: Swift.UInt64)
	@inlinable public var announcesPeakGet: Swift.UInt64 { get }
	@inlinable public mutating func announcesPeakSet(_ value: Swift.UInt64)
	@inlinable public var subscriptionsPeakGet: Swift.UInt64 { get }
	@inlinable public mutating func subscriptionsPeakSet(_ value: Swift.UInt64)
	@inlinable public var wireTxGet: Swift.UInt64 { get }
	@inlinable public mutating func wireTxSet(_ value: Swift.UInt64)
	@inlinable public var wireRxGet: Swift.UInt64 { get }
	@inlinable public mutating func wireRxSet(_ value: Swift.UInt64)
	@inlinable public mutating func add(_ other: StatsPresence)
	@inlinable public func active() -> Swift.UInt64
}
public struct StatsTotals: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsTotals, b: StatsTotals) -> Swift.Bool
	@inlinable public static func `default`() -> StatsTotals
	@inlinable public var txGet: Swift.UInt64 { get }
	@inlinable public mutating func txSet(_ value: Swift.UInt64)
	@inlinable public var rxGet: Swift.UInt64 { get }
	@inlinable public mutating func rxSet(_ value: Swift.UInt64)
}
public struct StatsFailures: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsFailures, b: StatsFailures) -> Swift.Bool
	@inlinable public static func `default`() -> StatsFailures
	@inlinable public var idleTimeoutGet: Swift.UInt64 { get }
	@inlinable public mutating func idleTimeoutSet(_ value: Swift.UInt64)
	@inlinable public var transportGet: Swift.UInt64 { get }
	@inlinable public mutating func transportSet(_ value: Swift.UInt64)
	@inlinable public var applicationGet: Swift.UInt64 { get }
	@inlinable public mutating func applicationSet(_ value: Swift.UInt64)
	@inlinable public var expiredGet: Swift.UInt64 { get }
	@inlinable public mutating func expiredSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsFailure) -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsFailures)
	@inlinable public func regressed(_ previous: StatsFailures) -> Swift.Bool
}
public struct StatsRefusals: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRefusals, b: StatsRefusals) -> Swift.Bool
	@inlinable public static func `default`() -> StatsRefusals
	@inlinable public var refusedGet: Swift.UInt64 { get }
	@inlinable public mutating func refusedSet(_ value: Swift.UInt64)
	@inlinable public var unavailableGet: Swift.UInt64 { get }
	@inlinable public mutating func unavailableSet(_ value: Swift.UInt64)
	@inlinable public var requestGet: Swift.UInt64 { get }
	@inlinable public mutating func requestSet(_ value: Swift.UInt64)
	@inlinable public var forbiddenGet: Swift.UInt64 { get }
	@inlinable public mutating func forbiddenSet(_ value: Swift.UInt64)
	@inlinable public var lanGet: Swift.UInt64 { get }
	@inlinable public mutating func lanSet(_ value: Swift.UInt64)
	@inlinable public var expiredGet: Swift.UInt64 { get }
	@inlinable public mutating func expiredSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsRefusalKind) -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsRefusals)
	@inlinable public func regressed(_ previous: StatsRefusals) -> Swift.Bool
}
public struct TransportReceiveWindow: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportReceiveWindow, b: TransportReceiveWindow) -> Swift.Bool
	@inlinable public var connectionGet: Swift.UInt64 { get }
	@inlinable public mutating func connectionSet(_ value: Swift.UInt64)
	@inlinable public var streamGet: Swift.UInt64 { get }
	@inlinable public mutating func streamSet(_ value: Swift.UInt64)
	@inlinable public static func new(_ connection: Swift.UInt64, _ stream: Swift.UInt64) -> TransportReceiveWindow
}
public struct BandwidthRate: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: BandwidthRate, b: BandwidthRate) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> BandwidthRate
	@inlinable public static var ZERO: BandwidthRate { get }
	@inlinable public static func fromBps(_ bps: Swift.UInt64) -> BandwidthRate
	@inlinable public static func fromKbps(_ kbps: Swift.UInt64) -> BandwidthRate
	@inlinable public static func fromMbps(_ mbps: Swift.UInt64) -> BandwidthRate
	@inlinable public func asBps() -> Swift.UInt64
	@inlinable public func scaled(_ factor: Swift.Double) -> BandwidthRate
	@inlinable public func absDiff(_ other: BandwidthRate) -> BandwidthRate
}
public struct FrameInfo: Sendable, CustomDebugStringConvertible {
	public var size: Swift.UInt64
	public var timestamp: Timestamp?
	@inlinable public init(size: Swift.UInt64, timestamp: Timestamp?)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct GroupInfo: Sendable, CustomDebugStringConvertible, Hashable {
	public var sequence: Swift.UInt64
	@inlinable public init(sequence: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: GroupInfo, b: GroupInfo) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct TrackDelay: Sendable, CustomDebugStringConvertible, Equatable {
	public var min: RustDuration?
	public var max: RustDuration?
	@inlinable public init(min: RustDuration?, max: RustDuration?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackDelay, b: TrackDelay) -> Swift.Bool
	@inlinable public static func `default`() -> TrackDelay
	@inlinable public func withMin(_ min: RustDuration?) -> TrackDelay
	@inlinable public func withMax(_ max: RustDuration?) -> TrackDelay
}
public struct TrackPosition: Sendable, CustomDebugStringConvertible, Hashable {
	public var group: Swift.UInt64
	public var frame: Swift.UInt64
	@inlinable public init(group: Swift.UInt64, frame: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackPosition, b: TrackPosition) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> TrackPosition
	@inlinable public static func group(_ group: Swift.UInt64) -> TrackPosition
	@inlinable public static func after(_ group: Swift.UInt64, _ frame: Swift.UInt64) -> TrackPosition?
	@inlinable public static func afterGroup(_ group: Swift.UInt64) -> TrackPosition?
	@inlinable public func groupEnd() -> BoundU64
}
public struct TrackRange: Sendable, CustomDebugStringConvertible, Hashable {
	public var start: TrackAnchor
	public var end: TrackAnchor?
	@inlinable public init(start: TrackAnchor, end: TrackAnchor?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackRange, b: TrackRange) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func live() -> TrackRange
	@inlinable public static func oldest() -> TrackRange
	@inlinable public static func groups(_ groups: BoundsU64) -> TrackRange
}
public struct OriginCost: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginCost, b: OriginCost) -> Swift.Bool
	@inlinable public static func `default`() -> OriginCost
	@inlinable public static func new(_ cost: Swift.UInt64) -> OriginCost
	@inlinable public func value() -> Swift.UInt64
	@inlinable public static var MAX: OriginCost { get }
	@inlinable public static var DRAIN: OriginCost { get }
}
public struct OriginHopLimit: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginHopLimit, b: OriginHopLimit) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> OriginHopLimit
	@inlinable public static func new(_ max: Swift.UInt) -> OriginHopLimit?
	@inlinable public func get() -> Swift.UInt
}
public struct OriginPreference: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginPreference, b: OriginPreference) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> OriginPreference
	@inlinable public static func new(_ preference: Swift.Int8) -> OriginPreference
	@inlinable public func value() -> Swift.Int8
}
public struct HangHealthLimitsF64: Sendable, CustomDebugStringConvertible, Equatable {
	public var degraded: Swift.Double
	public var unhealthy: Swift.Double
	@inlinable public init(degraded: Swift.Double, unhealthy: Swift.Double)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthLimitsF64, b: HangHealthLimitsF64) -> Swift.Bool
}
public struct HangHealthLimitsDuration: Sendable, CustomDebugStringConvertible, Equatable {
	public var degraded: RustDuration
	public var unhealthy: RustDuration
	@inlinable public init(degraded: RustDuration, unhealthy: RustDuration)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthLimitsDuration, b: HangHealthLimitsDuration) -> Swift.Bool
}
public struct HangCatalogAac: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	@inlinable public init(profile: Swift.UInt8)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAac, b: HangCatalogAac) -> Swift.Bool
}
public struct HangCatalogClock: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogClock, b: HangCatalogClock) -> Swift.Bool
	@inlinable public var wallGet: Timestamp { get }
	@inlinable public mutating func wallSet(_ value: Timestamp)
	@inlinable public static func new(_ wall: Timestamp) throws -> HangCatalogClock
	@inlinable public func wallClock(_ pts: Timestamp) throws -> ClockWall
}
public struct HangCatalogPriorities: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogPriorities, b: HangCatalogPriorities) -> Swift.Bool
	@inlinable public var catalogGet: Swift.Int8 { get }
	@inlinable public mutating func catalogSet(_ value: Swift.Int8)
	@inlinable public var textGet: Swift.Int8 { get }
	@inlinable public mutating func textSet(_ value: Swift.Int8)
	@inlinable public var audioGet: Swift.Int8 { get }
	@inlinable public mutating func audioSet(_ value: Swift.Int8)
	@inlinable public var videoGet: Swift.Int8 { get }
	@inlinable public mutating func videoSet(_ value: Swift.Int8)
}
public struct HangCatalogAv1: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	public var level: Swift.UInt8
	public var tier: Swift.Unicode.Scalar
	public var bitdepth: Swift.UInt8
	public var monoChrome: Swift.Bool
	public var chromaSubsamplingX: Swift.Bool
	public var chromaSubsamplingY: Swift.Bool
	public var chromaSamplePosition: Swift.UInt8
	public var colorPrimaries: Swift.UInt8
	public var transferCharacteristics: Swift.UInt8
	public var matrixCoefficients: Swift.UInt8
	public var fullRange: Swift.Bool
	@inlinable public init(profile: Swift.UInt8, level: Swift.UInt8, tier: Swift.Unicode.Scalar, bitdepth: Swift.UInt8, monoChrome: Swift.Bool, chromaSubsamplingX: Swift.Bool, chromaSubsamplingY: Swift.Bool, chromaSamplePosition: Swift.UInt8, colorPrimaries: Swift.UInt8, transferCharacteristics: Swift.UInt8, matrixCoefficients: Swift.UInt8, fullRange: Swift.Bool)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAv1, b: HangCatalogAv1) -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogAv1
	@inlinable public static var PREFIX: Swift.String { get }
}
public struct HangCatalogMastering: Sendable, CustomDebugStringConvertible, Equatable {
	public var red: HangCatalogChromaticity
	public var green: HangCatalogChromaticity
	public var blue: HangCatalogChromaticity
	public var white: HangCatalogChromaticity
	public var maxLuminance: Swift.Double
	public var minLuminance: Swift.Double
	@inlinable public init(red: HangCatalogChromaticity, green: HangCatalogChromaticity, blue: HangCatalogChromaticity, white: HangCatalogChromaticity, maxLuminance: Swift.Double, minLuminance: Swift.Double)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogMastering, b: HangCatalogMastering) -> Swift.Bool
}
public struct HangCatalogChromaticity: Sendable, CustomDebugStringConvertible, Equatable {
	public var x: Swift.Double
	public var y: Swift.Double
	@inlinable public init(x: Swift.Double, y: Swift.Double)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogChromaticity, b: HangCatalogChromaticity) -> Swift.Bool
}
public struct HangCatalogContentLight: Sendable, CustomDebugStringConvertible, Equatable {
	public var maxCll: Swift.UInt16
	public var maxFall: Swift.UInt16
	@inlinable public init(maxCll: Swift.UInt16, maxFall: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogContentLight, b: HangCatalogContentLight) -> Swift.Bool
}
public struct HangCatalogH264: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var inline: Swift.Bool
	public var profile: Swift.UInt8
	public var constraints: Swift.UInt8
	public var level: Swift.UInt8
	@inlinable public init(inline: Swift.Bool, profile: Swift.UInt8, constraints: Swift.UInt8, level: Swift.UInt8)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogH264, b: HangCatalogH264) -> Swift.Bool
}
public struct HangCatalogH265: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var inBand: Swift.Bool
	public var profileSpace: Swift.UInt8
	public var profileIdc: Swift.UInt8
	public var profileCompatibilityFlags: ArrayU8x4
	public var tierFlag: Swift.Bool
	public var levelIdc: Swift.UInt8
	public var constraintFlags: ArrayU8x6
	@inlinable public init(inBand: Swift.Bool, profileSpace: Swift.UInt8, profileIdc: Swift.UInt8, profileCompatibilityFlags: ArrayU8x4, tierFlag: Swift.Bool, levelIdc: Swift.UInt8, constraintFlags: ArrayU8x6)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogH265, b: HangCatalogH265) -> Swift.Bool
}
public struct HangCatalogVp9: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	public var level: Swift.UInt8
	public var bitDepth: Swift.UInt8
	public var chromaSubsampling: Swift.UInt8
	public var colorPrimaries: Swift.UInt8
	public var transferCharacteristics: Swift.UInt8
	public var matrixCoefficients: Swift.UInt8
	public var fullRange: Swift.Bool
	@inlinable public init(profile: Swift.UInt8, level: Swift.UInt8, bitDepth: Swift.UInt8, chromaSubsampling: Swift.UInt8, colorPrimaries: Swift.UInt8, transferCharacteristics: Swift.UInt8, matrixCoefficients: Swift.UInt8, fullRange: Swift.Bool)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogVp9, b: HangCatalogVp9) -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogVp9
}
public struct HangCatalogDisplay: Sendable, CustomDebugStringConvertible, Equatable {
	public var width: Swift.UInt32
	public var height: Swift.UInt32
	@inlinable public init(width: Swift.UInt32, height: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogDisplay, b: HangCatalogDisplay) -> Swift.Bool
}
public struct HangContainerFrame: Sendable, CustomDebugStringConvertible {
	public var timestamp: Timestamp
	public var payload: Bytes
	@inlinable public init(timestamp: Timestamp, payload: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func encode(_ buf: inout Bytes) throws
	@inlinable public static func decode(_ buf: Bytes) throws -> HangContainerFrame
	@inlinable public func writeTo(_ group: inout GroupProducer) throws
}
public struct MuxContainerFrame: Sendable, CustomDebugStringConvertible {
	public var timestamp: Timestamp
	public var duration: Timestamp?
	public var payload: Bytes
	public var keyframe: Swift.Bool
	@inlinable public init(timestamp: Timestamp, duration: Timestamp?, payload: Bytes, keyframe: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MuxContainerTimestampRewind: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var timestamp: Timestamp
	public var floor: Timestamp
	@inlinable public init(timestamp: Timestamp, floor: Timestamp)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTimestampRewind, b: MuxContainerTimestampRewind) -> Swift.Bool
}
public struct MuxSelectSelected: Sendable, CustomDebugStringConvertible, Equatable {
	public var video: [Swift.String]
	public var audio: [Swift.String]
	@inlinable public init(video: [Swift.String], audio: [Swift.String])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxSelectSelected, b: MuxSelectSelected) -> Swift.Bool
	@inlinable public static func `default`() -> MuxSelectSelected
	@inlinable public static func ofExtra(_ catalog: borrowing HangCatalogExtra) throws -> MuxSelectSelected
	@inlinable public static func ofExt(_ catalog: borrowing HangCatalogExt) throws -> MuxSelectSelected
	@inlinable public static func ofUnit(_ catalog: borrowing HangCatalog) throws -> MuxSelectSelected
}
public struct MuxTimelineTransition: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxTimelineTransition, b: MuxTimelineTransition) -> Swift.Bool
}
public struct MuxTimelineAudio: Sendable, CustomDebugStringConvertible {
	public var end: RustDuration
	public var buffered: RustDuration
	public var target: RustDuration
	@inlinable public init(end: RustDuration, buffered: RustDuration, target: RustDuration)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MuxCodecAacConfig: Sendable {
	public var profile: Swift.UInt8
	public var sampleRate: Swift.UInt32
	public var channelCount: Swift.UInt32
	@inlinable public init(profile: Swift.UInt8, sampleRate: Swift.UInt32, channelCount: Swift.UInt32)
	@inlinable public static func parse(_ buf: inout Bytes) throws -> MuxCodecAacConfig
	@inlinable public func encode() throws -> Bytes
}
public struct MuxCodecFlacConfig: Sendable, CustomDebugStringConvertible, Equatable {
	public var minBlockSize: Swift.UInt16
	public var maxBlockSize: Swift.UInt16
	public var minFrameSize: Swift.UInt32
	public var maxFrameSize: Swift.UInt32
	public var sampleRate: Swift.UInt32
	public var channelCount: Swift.UInt32
	public var bitsPerSample: Swift.UInt32
	public var totalSamples: Swift.UInt64
	public var md5: ArrayU8x16
	@inlinable public init(minBlockSize: Swift.UInt16, maxBlockSize: Swift.UInt16, minFrameSize: Swift.UInt32, maxFrameSize: Swift.UInt32, sampleRate: Swift.UInt32, channelCount: Swift.UInt32, bitsPerSample: Swift.UInt32, totalSamples: Swift.UInt64, md5: ArrayU8x16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCodecFlacConfig, b: MuxCodecFlacConfig) -> Swift.Bool
	@inlinable public static func parseStreamInfo(_ buf: inout Bytes) throws -> MuxCodecFlacConfig
	@inlinable public static func parse(_ buf: inout Bytes) throws -> MuxCodecFlacConfig
	@inlinable public func encodeStreamInfo() -> Bytes
	@inlinable public func description() -> Bytes
}
public struct MuxCodecH264Sps: Sendable, CustomDebugStringConvertible {
	public var profile: Swift.UInt8
	public var constraints: Swift.UInt8
	public var level: Swift.UInt8
	public var codedWidth: Swift.UInt32
	public var codedHeight: Swift.UInt32
	@inlinable public init(profile: Swift.UInt8, constraints: Swift.UInt8, level: Swift.UInt8, codedWidth: Swift.UInt32, codedHeight: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func parse(_ nal: Bytes) throws -> MuxCodecH264Sps
}
public struct MuxCodecMp3Config: Sendable {
	public var sampleRate: Swift.UInt32
	public var channelCount: Swift.UInt32
	@inlinable public init(sampleRate: Swift.UInt32, channelCount: Swift.UInt32)
	@inlinable public static func parse(_ data: Bytes) throws -> MuxCodecMp3Config
}
public struct MuxCodecOpusMapping: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCodecOpusMapping, b: MuxCodecOpusMapping) -> Swift.Bool
	@inlinable public func family() -> Swift.UInt8
	@inlinable public func streams() -> Swift.UInt8
	@inlinable public func coupled() -> Swift.UInt8
	@inlinable public func table() -> Bytes
}
public struct MuxContainerFmp4Fragment: Sendable, CustomDebugStringConvertible {
	public var data: Bytes
	public var independent: Swift.Bool
	public var duration: RustDuration
	@inlinable public init(data: Bytes, independent: Swift.Bool, duration: RustDuration)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MuxContainerLegacyWire: Sendable {
	public var _0: MuxContainerKind
	@inlinable public init(_ _0: MuxContainerKind)
}
public struct MuxContainerLocWire: Sendable {
	public var _0: MuxContainerKind
	@inlinable public init(_ _0: MuxContainerKind)
}
public struct MuxContainerMp4Finish: Sendable, CustomDebugStringConvertible {
	public var moovOffset: Swift.UInt64
	public var moov: Bytes
	public var mdatOffset: Swift.UInt64
	public var mdat: Bytes
	@inlinable public init(moovOffset: Swift.UInt64, moov: Bytes, mdatOffset: Swift.UInt64, mdat: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MuxContainerTsDescriptor: Sendable, CustomDebugStringConvertible, Equatable {
	public var tag: Swift.UInt8
	public var data: Bytes
	@inlinable public init(tag: Swift.UInt8, data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsDescriptor, b: MuxContainerTsDescriptor) -> Swift.Bool
}
public struct MuxContainerFmp4FragmentConfig: Sendable, CustomDebugStringConvertible {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> MuxContainerFmp4FragmentConfig
	@inlinable public var missingDurationGet: MuxContainerFmp4FragmentMissingDuration { get }
	@inlinable public mutating func missingDurationSet(_ value: MuxContainerFmp4FragmentMissingDuration)
}
public struct Mp4AtomFourCc: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFourCc, b: Mp4AtomFourCc) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func new(_ value: ArrayU8x4) -> Mp4AtomFourCc
}
public struct Mp4AtomMp4a: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var esds: Mp4AtomEsds
	public var btrt: Mp4AtomBtrt?
	public var taic: Mp4AtomTaic?
	@inlinable public init(audio: Mp4AtomAudio, esds: Mp4AtomEsds, btrt: Mp4AtomBtrt?, taic: Mp4AtomTaic?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMp4a, b: Mp4AtomMp4a) -> Swift.Bool
}
public struct Mp4AtomMp3: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	@inlinable public init(audio: Mp4AtomAudio)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMp3, b: Mp4AtomMp3) -> Swift.Bool
}
public struct Mp4AtomOpus: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var dops: Mp4AtomDops
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, dops: Mp4AtomDops, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomOpus, b: Mp4AtomOpus) -> Swift.Bool
}
public struct Mp4AtomFlac: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var dfla: Mp4AtomDfla
	@inlinable public init(audio: Mp4AtomAudio, dfla: Mp4AtomDfla)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFlac, b: Mp4AtomFlac) -> Swift.Bool
}
public struct Mp4AtomAc3: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var dac3: Mp4AtomAc3SpecificBox
	@inlinable public init(audio: Mp4AtomAudio, dac3: Mp4AtomAc3SpecificBox)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAc3, b: Mp4AtomAc3) -> Swift.Bool
}
public struct Mp4AtomEac3: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var dec3: Mp4AtomEc3SpecificBox
	@inlinable public init(audio: Mp4AtomAudio, dec3: Mp4AtomEc3SpecificBox)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEac3, b: Mp4AtomEac3) -> Swift.Bool
}
public struct Mp4AtomIpcm: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIpcm, b: Mp4AtomIpcm) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomFpcm: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFpcm, b: Mp4AtomFpcm) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomSowt: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSowt, b: Mp4AtomSowt) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomTwos: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTwos, b: Mp4AtomTwos) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomLpcm: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomLpcm, b: Mp4AtomLpcm) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomIn24: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIn24, b: Mp4AtomIn24) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomIn32: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIn32, b: Mp4AtomIn32) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomFl32: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFl32, b: Mp4AtomFl32) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomFl64: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFl64, b: Mp4AtomFl64) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomS16l: Sendable, CustomDebugStringConvertible, Equatable {
	public var audio: Mp4AtomAudio
	public var pcmc: Mp4AtomPcmC?
	public var chnl: Mp4AtomChnl?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(audio: Mp4AtomAudio, pcmc: Mp4AtomPcmC?, chnl: Mp4AtomChnl?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomS16l, b: Mp4AtomS16l) -> Swift.Bool
	@inlinable public func format() -> Mp4AtomPcmFormat?
}
public struct Mp4AtomWvtt: Sendable, CustomDebugStringConvertible, Equatable {
	public var plaintext: Mp4AtomPlainText
	public var config: Mp4AtomVttC
	public var label: Mp4AtomVlab?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(plaintext: Mp4AtomPlainText, config: Mp4AtomVttC, label: Mp4AtomVlab?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomWvtt, b: Mp4AtomWvtt) -> Swift.Bool
}
public struct Mp4AtomSamr: Sendable, CustomDebugStringConvertible, Equatable {
	public var amrsampleentry: Mp4AtomAmrSampleEntry
	public var damr: Mp4AtomDamr
	@inlinable public init(amrsampleentry: Mp4AtomAmrSampleEntry, damr: Mp4AtomDamr)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSamr, b: Mp4AtomSamr) -> Swift.Bool
}
public struct Mp4AtomMett: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	public var contentEncoding: Swift.String
	public var mimeFormat: Swift.String
	public var btrt: Mp4AtomBtrt?
	public var txtc: Mp4AtomTxtC?
	@inlinable public init(metadata: Mp4AtomMetaData, contentEncoding: Swift.String, mimeFormat: Swift.String, btrt: Mp4AtomBtrt?, txtc: Mp4AtomTxtC?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMett, b: Mp4AtomMett) -> Swift.Bool
}
public struct Mp4AtomMetx: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	public var contentEncoding: Swift.String
	public var namespace: Swift.String
	public var schemaLocation: Swift.String
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(metadata: Mp4AtomMetaData, contentEncoding: Swift.String, namespace: Swift.String, schemaLocation: Swift.String, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMetx, b: Mp4AtomMetx) -> Swift.Bool
}
public struct Mp4AtomUrim: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	public var theLabel: Mp4AtomUri
	public var init_: Mp4AtomUriI?
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(metadata: Mp4AtomMetaData, theLabel: Mp4AtomUri, init_: Mp4AtomUriI?, btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUrim, b: Mp4AtomUrim) -> Swift.Bool
}
public struct Mp4AtomCamm: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	@inlinable public init(metadata: Mp4AtomMetaData)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCamm, b: Mp4AtomCamm) -> Swift.Bool
}
public struct Mp4AtomEvte: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	public var btrt: Mp4AtomBtrt?
	public var silb: Mp4AtomSilb?
	@inlinable public init(metadata: Mp4AtomMetaData, btrt: Mp4AtomBtrt?, silb: Mp4AtomSilb?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEvte, b: Mp4AtomEvte) -> Swift.Bool
}
public struct Mp4AtomMebx: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadata: Mp4AtomMetaData
	public var keys: [Mp4AtomMebxKey]
	public var btrt: Mp4AtomBtrt?
	@inlinable public init(metadata: Mp4AtomMetaData, keys: [Mp4AtomMebxKey], btrt: Mp4AtomBtrt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMebx, b: Mp4AtomMebx) -> Swift.Bool
}
public struct Mp4AtomTkhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var creationTime: Swift.UInt64
	public var modificationTime: Swift.UInt64
	public var trackId: Swift.UInt32
	public var duration: Swift.UInt64
	public var layer: Swift.UInt16
	public var alternateGroup: Swift.UInt16
	public var enabled: Swift.Bool
	public var inMovie: Swift.Bool
	public var sizeIsAspectRatio: Swift.Bool
	public var volume: Mp4AtomFixedPointU8
	public var matrix: Mp4AtomMatrix
	public var width: Mp4AtomFixedPointU16
	public var height: Mp4AtomFixedPointU16
	@inlinable public init(creationTime: Swift.UInt64, modificationTime: Swift.UInt64, trackId: Swift.UInt32, duration: Swift.UInt64, layer: Swift.UInt16, alternateGroup: Swift.UInt16, enabled: Swift.Bool, inMovie: Swift.Bool, sizeIsAspectRatio: Swift.Bool, volume: Mp4AtomFixedPointU8, matrix: Mp4AtomMatrix, width: Mp4AtomFixedPointU16, height: Mp4AtomFixedPointU16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTkhd, b: Mp4AtomTkhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTkhd
}
public struct Mp4AtomEdts: Sendable, CustomDebugStringConvertible, Equatable {
	public var elst: Mp4AtomElst?
	@inlinable public init(elst: Mp4AtomElst?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEdts, b: Mp4AtomEdts) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEdts
}
public struct Mp4AtomSenc: Sendable, CustomDebugStringConvertible, Equatable {
	public var version: Mp4AtomSencBoxVersion
	public var useSubsamples: Swift.Bool
	public var data: Bytes
	@inlinable public init(version: Mp4AtomSencBoxVersion, useSubsamples: Swift.Bool, data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSenc, b: Mp4AtomSenc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSenc
}
public struct Mp4AtomTref: Sendable, CustomDebugStringConvertible, Equatable {
	public var trackReferenceTypeBoxes: [Mp4AtomTrackReferenceTypeBox]
	@inlinable public init(trackReferenceTypeBoxes: [Mp4AtomTrackReferenceTypeBox])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTref, b: Mp4AtomTref) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTref
}
public struct Mp4AtomAudio: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataReferenceIndex: Swift.UInt16
	public var channelCount: Swift.UInt16
	public var sampleSize: Swift.UInt16
	public var sampleRate: Mp4AtomFixedPointU16
	@inlinable public init(dataReferenceIndex: Swift.UInt16, channelCount: Swift.UInt16, sampleSize: Swift.UInt16, sampleRate: Mp4AtomFixedPointU16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAudio, b: Mp4AtomAudio) -> Swift.Bool
}
public struct Mp4AtomMetaData: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataReferenceIndex: Swift.UInt16
	@inlinable public init(dataReferenceIndex: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMetaData, b: Mp4AtomMetaData) -> Swift.Bool
}
public struct RustlsClientTicketRequest: Sendable, CustomDebugStringConvertible, Equatable {
	public var newSessionCount: Swift.UInt8
	public var resumptionCount: Swift.UInt8
	@inlinable public init(newSessionCount: Swift.UInt8, resumptionCount: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RustlsClientTicketRequest, b: RustlsClientTicketRequest) -> Swift.Bool
}
public struct MoqQuicVarInt: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicVarInt, b: MoqQuicVarInt) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> MoqQuicVarInt
	@inlinable public static var MAX: MoqQuicVarInt { get }
	@inlinable public static func fromU32(_ x: Swift.UInt32) -> MoqQuicVarInt
	@inlinable public static func fromU64(_ x: Swift.UInt64) throws -> MoqQuicVarInt
	@inlinable public func intoInner() -> Swift.UInt64
	@inlinable public func encode(_ output: inout Bytes)
}
public struct Mp4AtomAvcc: Sendable, CustomDebugStringConvertible, Equatable {
	public var configurationVersion: Swift.UInt8
	public var avcProfileIndication: Swift.UInt8
	public var profileCompatibility: Swift.UInt8
	public var avcLevelIndication: Swift.UInt8
	public var lengthSize: Swift.UInt8
	public var sequenceParameterSets: [Bytes]
	public var pictureParameterSets: [Bytes]
	public var ext: Mp4AtomAvccExt?
	@inlinable public init(configurationVersion: Swift.UInt8, avcProfileIndication: Swift.UInt8, profileCompatibility: Swift.UInt8, avcLevelIndication: Swift.UInt8, lengthSize: Swift.UInt8, sequenceParameterSets: [Bytes], pictureParameterSets: [Bytes], ext: Mp4AtomAvccExt?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAvcc, b: Mp4AtomAvcc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAvcc
	@inlinable public static func new(_ sps: Bytes, _ pps: Bytes) throws -> Mp4AtomAvcc
}
public struct Mp4AtomBtrt: Sendable, CustomDebugStringConvertible, Equatable {
	public var bufferSizeDb: Swift.UInt32
	public var maxBitrate: Swift.UInt32
	public var avgBitrate: Swift.UInt32
	@inlinable public init(bufferSizeDb: Swift.UInt32, maxBitrate: Swift.UInt32, avgBitrate: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomBtrt, b: Mp4AtomBtrt) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomBtrt
	@inlinable public static func new(_ bufferSizeDb: Swift.UInt32, _ maxBitrate: Swift.UInt32, _ avgBitrate: Swift.UInt32) throws -> Mp4AtomBtrt
}
public struct Mp4AtomPasp: Sendable, CustomDebugStringConvertible, Equatable {
	public var hSpacing: Swift.UInt32
	public var vSpacing: Swift.UInt32
	@inlinable public init(hSpacing: Swift.UInt32, vSpacing: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPasp, b: Mp4AtomPasp) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPasp
	@inlinable public static func new(_ hSpacing: Swift.UInt32, _ vSpacing: Swift.UInt32) throws -> Mp4AtomPasp
}
public struct Mp4AtomTaic: Sendable, CustomDebugStringConvertible, Equatable {
	public var timeUncertainty: Swift.UInt64
	public var clockResolution: Swift.UInt32
	public var clockDriftRate: Swift.Int32
	public var clockType: Mp4AtomClockType
	@inlinable public init(timeUncertainty: Swift.UInt64, clockResolution: Swift.UInt32, clockDriftRate: Swift.Int32, clockType: Mp4AtomClockType)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTaic, b: Mp4AtomTaic) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTaic
	@inlinable public static func new(_ timeUncertainty: Swift.UInt64, _ clockResolution: Swift.UInt32, _ clockDriftRate: Swift.Int32, _ clockType: Mp4AtomClockType) throws -> Mp4AtomTaic
}
public struct Mp4AtomFiel: Sendable, CustomDebugStringConvertible, Equatable {
	public var fieldCount: Swift.UInt8
	public var fieldOrder: Swift.UInt8
	@inlinable public init(fieldCount: Swift.UInt8, fieldOrder: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFiel, b: Mp4AtomFiel) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFiel
	@inlinable public static func new(_ fieldCount: Swift.UInt8, _ fieldOrder: Swift.UInt8) throws -> Mp4AtomFiel
}
public struct Mp4AtomHvcc: Sendable, CustomDebugStringConvertible, Equatable {
	public var configurationVersion: Swift.UInt8
	public var generalProfileSpace: Swift.UInt8
	public var generalTierFlag: Swift.Bool
	public var generalProfileIdc: Swift.UInt8
	public var generalProfileCompatibilityFlags: ArrayU8x4
	public var generalConstraintIndicatorFlags: ArrayU8x6
	public var generalLevelIdc: Swift.UInt8
	public var minSpatialSegmentationIdc: Swift.UInt16
	public var parallelismType: Swift.UInt8
	public var chromaFormatIdc: Swift.UInt8
	public var bitDepthLumaMinus8: Swift.UInt8
	public var bitDepthChromaMinus8: Swift.UInt8
	public var avgFrameRate: Swift.UInt16
	public var constantFrameRate: Swift.UInt8
	public var numTemporalLayers: Swift.UInt8
	public var temporalIdNested: Swift.Bool
	public var lengthSizeMinusOne: Swift.UInt8
	public var arrays: [Mp4AtomHvcCArray]
	@inlinable public init(configurationVersion: Swift.UInt8, generalProfileSpace: Swift.UInt8, generalTierFlag: Swift.Bool, generalProfileIdc: Swift.UInt8, generalProfileCompatibilityFlags: ArrayU8x4, generalConstraintIndicatorFlags: ArrayU8x6, generalLevelIdc: Swift.UInt8, minSpatialSegmentationIdc: Swift.UInt16, parallelismType: Swift.UInt8, chromaFormatIdc: Swift.UInt8, bitDepthLumaMinus8: Swift.UInt8, bitDepthChromaMinus8: Swift.UInt8, avgFrameRate: Swift.UInt16, constantFrameRate: Swift.UInt8, numTemporalLayers: Swift.UInt8, temporalIdNested: Swift.Bool, lengthSizeMinusOne: Swift.UInt8, arrays: [Mp4AtomHvcCArray])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHvcc, b: Mp4AtomHvcc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHvcc
	@inlinable public static func new() -> Mp4AtomHvcc
}
public struct Mp4AtomLhvc: Sendable, CustomDebugStringConvertible, Equatable {
	public var configurationVersion: Swift.UInt8
	public var minSpatialSegmentationIdc: Swift.UInt16
	public var parallelismType: Swift.UInt8
	public var numTemporalLayers: Swift.UInt8
	public var temporalIdNested: Swift.Bool
	public var lengthSizeMinusOne: Swift.UInt8
	public var arrays: [Mp4AtomHvcCArray]
	@inlinable public init(configurationVersion: Swift.UInt8, minSpatialSegmentationIdc: Swift.UInt16, parallelismType: Swift.UInt8, numTemporalLayers: Swift.UInt8, temporalIdNested: Swift.Bool, lengthSizeMinusOne: Swift.UInt8, arrays: [Mp4AtomHvcCArray])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomLhvc, b: Mp4AtomLhvc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomLhvc
}
public struct Mp4AtomCcst: Sendable, CustomDebugStringConvertible, Equatable {
	public var allRefPicsIntra: Swift.Bool
	public var intraPredUsed: Swift.Bool
	public var maxRefPerPic: Swift.UInt8
	@inlinable public init(allRefPicsIntra: Swift.Bool, intraPredUsed: Swift.Bool, maxRefPerPic: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCcst, b: Mp4AtomCcst) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCcst
}
public struct Mp4AtomVpcC: Sendable, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	public var level: Swift.UInt8
	public var bitDepth: Swift.UInt8
	public var chromaSubsampling: Swift.UInt8
	public var videoFullRangeFlag: Swift.Bool
	public var colorPrimaries: Swift.UInt8
	public var transferCharacteristics: Swift.UInt8
	public var matrixCoefficients: Swift.UInt8
	public var codecInitializationData: Bytes
	@inlinable public init(profile: Swift.UInt8, level: Swift.UInt8, bitDepth: Swift.UInt8, chromaSubsampling: Swift.UInt8, videoFullRangeFlag: Swift.Bool, colorPrimaries: Swift.UInt8, transferCharacteristics: Swift.UInt8, matrixCoefficients: Swift.UInt8, codecInitializationData: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomVpcC, b: Mp4AtomVpcC) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVpcC
}
public struct Mp4AtomAv1c: Sendable, CustomDebugStringConvertible, Equatable {
	public var seqProfile: Swift.UInt8
	public var seqLevelIdx0: Swift.UInt8
	public var seqTier0: Swift.Bool
	public var highBitdepth: Swift.Bool
	public var twelveBit: Swift.Bool
	public var monochrome: Swift.Bool
	public var chromaSubsamplingX: Swift.Bool
	public var chromaSubsamplingY: Swift.Bool
	public var chromaSamplePosition: Swift.UInt8
	public var initialPresentationDelay: Swift.UInt8?
	public var configObus: Bytes
	@inlinable public init(seqProfile: Swift.UInt8, seqLevelIdx0: Swift.UInt8, seqTier0: Swift.Bool, highBitdepth: Swift.Bool, twelveBit: Swift.Bool, monochrome: Swift.Bool, chromaSubsamplingX: Swift.Bool, chromaSubsamplingY: Swift.Bool, chromaSamplePosition: Swift.UInt8, initialPresentationDelay: Swift.UInt8?, configObus: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAv1c, b: Mp4AtomAv1c) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAv1c
}
public struct Mp4AtomEsds: Sendable, CustomDebugStringConvertible, Equatable {
	public var esDesc: Mp4AtomEsdsEsDescriptor
	@inlinable public init(esDesc: Mp4AtomEsdsEsDescriptor)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEsds, b: Mp4AtomEsds) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEsds
}
public struct Mp4AtomRgbaColor: Sendable, CustomDebugStringConvertible, Equatable {
	public var red: Swift.UInt8
	public var green: Swift.UInt8
	public var blue: Swift.UInt8
	public var alpha: Swift.UInt8
	@inlinable public init(red: Swift.UInt8, green: Swift.UInt8, blue: Swift.UInt8, alpha: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomRgbaColor, b: Mp4AtomRgbaColor) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomRgbaColor
}
public struct Mp4AtomFtab: Sendable, CustomDebugStringConvertible, Equatable {
	public var fontEntries: [Mp4AtomFontEntry]
	@inlinable public init(fontEntries: [Mp4AtomFontEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFtab, b: Mp4AtomFtab) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFtab
}
public struct Mp4AtomDisp: Sendable, CustomDebugStringConvertible, Equatable {
	public var disparityShiftIn16thPel: Swift.Int16
	@inlinable public init(disparityShiftIn16thPel: Swift.Int16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDisp, b: Mp4AtomDisp) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomDisp
}
public struct Mp4AtomDops: Sendable, CustomDebugStringConvertible, Equatable {
	public var outputChannelCount: Swift.UInt8
	public var preSkip: Swift.UInt16
	public var inputSampleRate: Swift.UInt32
	public var outputGain: Swift.Int16
	public var channelMapping: Mp4AtomOpusChannelMapping?
	@inlinable public init(outputChannelCount: Swift.UInt8, preSkip: Swift.UInt16, inputSampleRate: Swift.UInt32, outputGain: Swift.Int16, channelMapping: Mp4AtomOpusChannelMapping?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDops, b: Mp4AtomDops) -> Swift.Bool
}
public struct Mp4AtomCmpd: Sendable, CustomDebugStringConvertible, Equatable {
	public var components: [Mp4AtomComponent]
	@inlinable public init(components: [Mp4AtomComponent])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCmpd, b: Mp4AtomCmpd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCmpd
}
public struct Mp4AtomDfla: Sendable, CustomDebugStringConvertible, Equatable {
	public var blocks: [Mp4AtomFlacMetadataBlock]
	@inlinable public init(blocks: [Mp4AtomFlacMetadataBlock])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDfla, b: Mp4AtomDfla) -> Swift.Bool
}
public struct Mp4AtomAc3SpecificBox: Sendable, CustomDebugStringConvertible, Equatable {
	public var fscod: Swift.UInt8
	public var bsid: Swift.UInt8
	public var bsmod: Swift.UInt8
	public var acmod: Swift.UInt8
	public var lfeon: Swift.Bool
	public var bitRateCode: Swift.UInt8
	@inlinable public init(fscod: Swift.UInt8, bsid: Swift.UInt8, bsmod: Swift.UInt8, acmod: Swift.UInt8, lfeon: Swift.Bool, bitRateCode: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAc3SpecificBox, b: Mp4AtomAc3SpecificBox) -> Swift.Bool
}
public struct Mp4AtomEc3SpecificBox: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataRate: Swift.UInt16
	public var substreams: [Mp4AtomEc3IndependentSubstream]
	@inlinable public init(dataRate: Swift.UInt16, substreams: [Mp4AtomEc3IndependentSubstream])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEc3SpecificBox, b: Mp4AtomEc3SpecificBox) -> Swift.Bool
}
public struct Mp4AtomPcmC: Sendable, CustomDebugStringConvertible, Equatable {
	public var bigEndian: Swift.Bool
	public var sampleSize: Swift.UInt8
	@inlinable public init(bigEndian: Swift.Bool, sampleSize: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPcmC, b: Mp4AtomPcmC) -> Swift.Bool
}
public struct Mp4AtomChnl: Sendable, CustomDebugStringConvertible, Equatable {
	public var channelStructure: Mp4AtomChannelStructure?
	public var objectCount: Swift.UInt8?
	public var formatOrdering: Swift.UInt8?
	public var baseChannelCount: Swift.UInt8?
	@inlinable public init(channelStructure: Mp4AtomChannelStructure?, objectCount: Swift.UInt8?, formatOrdering: Swift.UInt8?, baseChannelCount: Swift.UInt8?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomChnl, b: Mp4AtomChnl) -> Swift.Bool
	@inlinable public static func decodeBodyWithChannelCount(_ buf: inout Bytes, _ channelCount: Swift.UInt16) throws -> Mp4AtomChnl
}
public struct Mp4AtomPlainText: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataReferenceIndex: Swift.UInt16
	@inlinable public init(dataReferenceIndex: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPlainText, b: Mp4AtomPlainText) -> Swift.Bool
}
public struct Mp4AtomVttC: Sendable, CustomDebugStringConvertible, Equatable {
	public var config: Swift.String
	@inlinable public init(config: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomVttC, b: Mp4AtomVttC) -> Swift.Bool
}
public struct Mp4AtomVlab: Sendable, CustomDebugStringConvertible, Equatable {
	public var sourceLabel: Swift.String
	@inlinable public init(sourceLabel: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomVlab, b: Mp4AtomVlab) -> Swift.Bool
}
public struct Mp4AtomAmrSampleEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataReferenceIndex: Swift.UInt16
	public var timescale: Swift.UInt16
	@inlinable public init(dataReferenceIndex: Swift.UInt16, timescale: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAmrSampleEntry, b: Mp4AtomAmrSampleEntry) -> Swift.Bool
}
public struct Mp4AtomDamr: Sendable, CustomDebugStringConvertible, Equatable {
	public var vendor: Mp4AtomFourCc
	public var decoderVersion: Swift.UInt8
	public var modeSet: Swift.UInt16
	public var modeChangePeriod: Swift.UInt8
	public var framesPerSample: Swift.UInt8
	@inlinable public init(vendor: Mp4AtomFourCc, decoderVersion: Swift.UInt8, modeSet: Swift.UInt16, modeChangePeriod: Swift.UInt8, framesPerSample: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDamr, b: Mp4AtomDamr) -> Swift.Bool
}
public struct Mp4AtomTxtC: Sendable, CustomDebugStringConvertible, Equatable {
	public var textConfig: Swift.String
	@inlinable public init(textConfig: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTxtC, b: Mp4AtomTxtC) -> Swift.Bool
}
public struct Mp4AtomUri: Sendable, CustomDebugStringConvertible, Equatable {
	public var theUri: Swift.String
	@inlinable public init(theUri: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUri, b: Mp4AtomUri) -> Swift.Bool
}
public struct Mp4AtomUriI: Sendable, CustomDebugStringConvertible, Equatable {
	public var uriInitializationData: Bytes
	@inlinable public init(uriInitializationData: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUriI, b: Mp4AtomUriI) -> Swift.Bool
}
public struct Mp4AtomSilb: Sendable, CustomDebugStringConvertible, Equatable {
	public var schemes: [Mp4AtomSchemeIdEntry]
	public var otherSchemes: Swift.Bool
	@inlinable public init(schemes: [Mp4AtomSchemeIdEntry], otherSchemes: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSilb, b: Mp4AtomSilb) -> Swift.Bool
}
public struct Mp4AtomMebxKey: Sendable, CustomDebugStringConvertible, Equatable {
	public var localKeyId: Mp4AtomFourCc
	public var keyd: Mp4AtomKeyd
	public var dtyp: Mp4AtomDtyp?
	public var loca: Mp4AtomLoca?
	public var setu: Mp4AtomSetu?
	@inlinable public init(localKeyId: Mp4AtomFourCc, keyd: Mp4AtomKeyd, dtyp: Mp4AtomDtyp?, loca: Mp4AtomLoca?, setu: Mp4AtomSetu?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMebxKey, b: Mp4AtomMebxKey) -> Swift.Bool
}
public struct Mp4AtomFixedPointI16: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFixedPointI16, b: Mp4AtomFixedPointI16) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFixedPointI16
	@inlinable public static func new(_ int: Swift.Int16, _ dec: Swift.Int16) -> Mp4AtomFixedPointI16
	@inlinable public func integer() -> Swift.Int16
	@inlinable public func decimal() -> Swift.Int16
}
public struct Mp4AtomFixedPointI8: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFixedPointI8, b: Mp4AtomFixedPointI8) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFixedPointI8
	@inlinable public static func new(_ int: Swift.Int8, _ dec: Swift.Int8) -> Mp4AtomFixedPointI8
	@inlinable public func integer() -> Swift.Int8
	@inlinable public func decimal() -> Swift.Int8
}
public struct Mp4AtomFixedPointU16: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFixedPointU16, b: Mp4AtomFixedPointU16) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFixedPointU16
	@inlinable public static func new(_ int: Swift.UInt16, _ dec: Swift.UInt16) -> Mp4AtomFixedPointU16
	@inlinable public func integer() -> Swift.UInt16
	@inlinable public func decimal() -> Swift.UInt16
}
public struct Mp4AtomFixedPointU8: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFixedPointU8, b: Mp4AtomFixedPointU8) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFixedPointU8
	@inlinable public static func new(_ int: Swift.UInt8, _ dec: Swift.UInt8) -> Mp4AtomFixedPointU8
	@inlinable public func integer() -> Swift.UInt8
	@inlinable public func decimal() -> Swift.UInt8
}
public struct Mp4AtomMatrix: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var a: Swift.Int32
	public var b: Swift.Int32
	public var u: Swift.Int32
	public var c: Swift.Int32
	public var d: Swift.Int32
	public var v: Swift.Int32
	public var x: Swift.Int32
	public var y: Swift.Int32
	public var w: Swift.Int32
	@inlinable public init(a: Swift.Int32, b: Swift.Int32, u: Swift.Int32, c: Swift.Int32, d: Swift.Int32, v: Swift.Int32, x: Swift.Int32, y: Swift.Int32, w: Swift.Int32)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMatrix, b: Mp4AtomMatrix) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMatrix
}
public struct Mp4AtomElst: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomElstEntry]
	@inlinable public init(entries: [Mp4AtomElstEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomElst, b: Mp4AtomElst) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomElst
}
public struct Mp4AtomHdlr: Sendable, CustomDebugStringConvertible, Equatable {
	public var handler: Mp4AtomFourCc
	public var name: Swift.String
	@inlinable public init(handler: Mp4AtomFourCc, name: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHdlr, b: Mp4AtomHdlr) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHdlr
}
public struct Mp4AtomMdhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var creationTime: Swift.UInt64
	public var modificationTime: Swift.UInt64
	public var timescale: Swift.UInt32
	public var duration: Swift.UInt64
	public var language: Swift.String
	@inlinable public init(creationTime: Swift.UInt64, modificationTime: Swift.UInt64, timescale: Swift.UInt32, duration: Swift.UInt64, language: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMdhd, b: Mp4AtomMdhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMdhd
}
public struct Mp4AtomTrackReferenceTypeBox: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceType: Mp4AtomFourCc
	public var trackIds: [Swift.UInt32]
	@inlinable public init(referenceType: Mp4AtomFourCc, trackIds: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTrackReferenceTypeBox, b: Mp4AtomTrackReferenceTypeBox) -> Swift.Bool
}
public struct Mp4AtomChpl: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomChplEntry]
	@inlinable public init(entries: [Mp4AtomChplEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomChpl, b: Mp4AtomChpl) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomChpl
}
public struct Mp4AtomCprt: Sendable, CustomDebugStringConvertible, Equatable {
	public var language: Swift.String
	public var notice: Swift.String
	@inlinable public init(language: Swift.String, notice: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCprt, b: Mp4AtomCprt) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCprt
}
public struct Mp4AtomKind: Sendable, CustomDebugStringConvertible, Equatable {
	public var schemeUri: Swift.String
	public var value: Swift.String
	@inlinable public init(schemeUri: Swift.String, value: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomKind, b: Mp4AtomKind) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomKind
}
public struct Mp4AtomRtng: Sendable, CustomDebugStringConvertible, Equatable {
	public var entity: Mp4AtomFourCc
	public var criteria: Mp4AtomFourCc
	public var language: Swift.String
	public var ratingInfo: Swift.String
	@inlinable public init(entity: Mp4AtomFourCc, criteria: Mp4AtomFourCc, language: Swift.String, ratingInfo: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomRtng, b: Mp4AtomRtng) -> Swift.Bool
}
public struct Mp4AtomPcmFormat: Sendable, CustomDebugStringConvertible, Equatable {
	public var bigEndian: Swift.Bool
	public var sampleSize: Swift.UInt16
	public var float: Swift.Bool
	@inlinable public init(bigEndian: Swift.Bool, sampleSize: Swift.UInt16, float: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPcmFormat, b: Mp4AtomPcmFormat) -> Swift.Bool
}
public struct MoqQuicStreamId: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicStreamId, b: MoqQuicStreamId) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func new(_ initiator: MoqQuicSide, _ dir: MoqQuicStreamDir, _ index: Swift.UInt64) -> MoqQuicStreamId
	@inlinable public func initiator() -> MoqQuicSide
	@inlinable public func dir() -> MoqQuicStreamDir
	@inlinable public func index() -> Swift.UInt64
}
public struct MoqQuicUdpStats: Sendable, CustomDebugStringConvertible {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> MoqQuicUdpStats
	@inlinable public var datagramsGet: Swift.UInt64 { get }
	@inlinable public mutating func datagramsSet(_ value: Swift.UInt64)
	@inlinable public var bytesGet: Swift.UInt64 { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64)
	@inlinable public var iosGet: Swift.UInt64 { get }
	@inlinable public mutating func iosSet(_ value: Swift.UInt64)
}
public struct MoqQuicCreditStats: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicCreditStats, b: MoqQuicCreditStats) -> Swift.Bool
	@inlinable public static func `default`() -> MoqQuicCreditStats
	@inlinable public var leasedGet: Swift.UInt64 { get }
	@inlinable public mutating func leasedSet(_ value: Swift.UInt64)
	@inlinable public var availableGet: Swift.UInt64 { get }
	@inlinable public mutating func availableSet(_ value: Swift.UInt64)
	@inlinable public var evictedGet: Swift.UInt64 { get }
	@inlinable public mutating func evictedSet(_ value: Swift.UInt64)
	@inlinable public var peakSpentGet: Swift.UInt64 { get }
	@inlinable public mutating func peakSpentSet(_ value: Swift.UInt64)
}
public struct Mp4AtomAvccExt: Sendable, CustomDebugStringConvertible, Equatable {
	public var chromaFormat: Swift.UInt8
	public var bitDepthLuma: Swift.UInt8
	public var bitDepthChroma: Swift.UInt8
	public var sequenceParameterSetsExt: [Bytes]
	@inlinable public init(chromaFormat: Swift.UInt8, bitDepthLuma: Swift.UInt8, bitDepthChroma: Swift.UInt8, sequenceParameterSetsExt: [Bytes])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAvccExt, b: Mp4AtomAvccExt) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAvccExt
}
public struct Mp4AtomSt3d: Sendable, CustomDebugStringConvertible, Equatable {
	public var stereoMode: Swift.UInt8
	@inlinable public init(stereoMode: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSt3d, b: Mp4AtomSt3d) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSt3d
}
public struct Mp4AtomHfov: Sendable, CustomDebugStringConvertible, Equatable {
	public var fieldOfView: Swift.UInt32
	@inlinable public init(fieldOfView: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHfov, b: Mp4AtomHfov) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHfov
}
public struct Mp4AtomHvcCArray: Sendable, CustomDebugStringConvertible, Equatable {
	public var completeness: Swift.Bool
	public var nalUnitType: Swift.UInt8
	public var nalus: [Bytes]
	@inlinable public init(completeness: Swift.Bool, nalUnitType: Swift.UInt8, nalus: [Bytes])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHvcCArray, b: Mp4AtomHvcCArray) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHvcCArray
}
public struct Mp4AtomEsdsEsDescriptor: Sendable, CustomDebugStringConvertible, Equatable {
	public var esId: Swift.UInt16
	public var dependsOnEsId: Swift.UInt16?
	public var url: Bytes?
	public var ocrEsId: Swift.UInt16?
	public var streamPriority: Swift.UInt8
	public var decConfig: Mp4AtomEsdsDecoderConfig
	public var slConfig: Mp4AtomEsdsSlConfig
	@inlinable public init(esId: Swift.UInt16, dependsOnEsId: Swift.UInt16?, url: Bytes?, ocrEsId: Swift.UInt16?, streamPriority: Swift.UInt8, decConfig: Mp4AtomEsdsDecoderConfig, slConfig: Mp4AtomEsdsSlConfig)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEsdsEsDescriptor, b: Mp4AtomEsdsEsDescriptor) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEsdsEsDescriptor
}
public struct Mp4AtomFontEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var fontId: Swift.UInt16
	public var font: Swift.String
	@inlinable public init(fontId: Swift.UInt16, font: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFontEntry, b: Mp4AtomFontEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFontEntry
}
public struct Mp4AtomOpusChannelMapping: Sendable, CustomDebugStringConvertible, Equatable {
	public var family: Swift.UInt8
	public var streamCount: Swift.UInt8
	public var coupledCount: Swift.UInt8
	public var channelMapping: Bytes
	@inlinable public init(family: Swift.UInt8, streamCount: Swift.UInt8, coupledCount: Swift.UInt8, channelMapping: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomOpusChannelMapping, b: Mp4AtomOpusChannelMapping) -> Swift.Bool
}
public struct Mp4AtomComponent: Sendable, CustomDebugStringConvertible, Equatable {
	public var componentType: Swift.UInt16
	public var componentTypeUri: Swift.String?
	@inlinable public init(componentType: Swift.UInt16, componentTypeUri: Swift.String?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomComponent, b: Mp4AtomComponent) -> Swift.Bool
}
public struct Mp4AtomUncompressedComponent: Sendable, CustomDebugStringConvertible, Equatable {
	public var componentIndex: Swift.UInt16
	public var componentBitDepthMinusOne: Swift.UInt8
	public var componentFormat: Swift.UInt8
	public var componentAlignSize: Swift.UInt8
	@inlinable public init(componentIndex: Swift.UInt16, componentBitDepthMinusOne: Swift.UInt8, componentFormat: Swift.UInt8, componentAlignSize: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUncompressedComponent, b: Mp4AtomUncompressedComponent) -> Swift.Bool
}
public struct Mp4AtomEc3IndependentSubstream: Sendable, CustomDebugStringConvertible, Equatable {
	public var fscod: Swift.UInt8
	public var bsid: Swift.UInt8
	public var asvc: Swift.Bool
	public var bsmod: Swift.UInt8
	public var acmod: Swift.UInt8
	public var lfeon: Swift.Bool
	public var numDepSub: Swift.UInt8
	public var chanLoc: Swift.UInt16?
	@inlinable public init(fscod: Swift.UInt8, bsid: Swift.UInt8, asvc: Swift.Bool, bsmod: Swift.UInt8, acmod: Swift.UInt8, lfeon: Swift.Bool, numDepSub: Swift.UInt8, chanLoc: Swift.UInt16?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEc3IndependentSubstream, b: Mp4AtomEc3IndependentSubstream) -> Swift.Bool
}
public struct Mp4AtomSchemeIdEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var schemeIdUri: Swift.String
	public var value: Swift.String
	public var atleastOnce: Swift.Bool
	@inlinable public init(schemeIdUri: Swift.String, value: Swift.String, atleastOnce: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSchemeIdEntry, b: Mp4AtomSchemeIdEntry) -> Swift.Bool
}
public struct Mp4AtomKeyd: Sendable, CustomDebugStringConvertible, Equatable {
	public var keyNamespace: Mp4AtomFourCc
	public var keyValue: Bytes
	@inlinable public init(keyNamespace: Mp4AtomFourCc, keyValue: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomKeyd, b: Mp4AtomKeyd) -> Swift.Bool
}
public struct Mp4AtomDtyp: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataType: Mp4AtomDataType
	@inlinable public init(dataType: Mp4AtomDataType)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDtyp, b: Mp4AtomDtyp) -> Swift.Bool
}
public struct Mp4AtomLoca: Sendable, CustomDebugStringConvertible, Equatable {
	public var locale: Swift.String
	@inlinable public init(locale: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomLoca, b: Mp4AtomLoca) -> Swift.Bool
}
public struct Mp4AtomSetu: Sendable, CustomDebugStringConvertible, Equatable {
	public var data: Bytes
	@inlinable public init(data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSetu, b: Mp4AtomSetu) -> Swift.Bool
}
public struct Mp4AtomElstEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var segmentDuration: Swift.UInt64
	public var mediaTime: Swift.UInt64?
	public var mediaRate: Mp4AtomFixedPointI16
	@inlinable public init(segmentDuration: Swift.UInt64, mediaTime: Swift.UInt64?, mediaRate: Mp4AtomFixedPointI16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomElstEntry, b: Mp4AtomElstEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomElstEntry
}
public struct Mp4AtomFtyp: Sendable, CustomDebugStringConvertible, Equatable {
	public var majorBrand: Mp4AtomFourCc
	public var minorVersion: Swift.UInt32
	public var compatibleBrands: [Mp4AtomFourCc]
	@inlinable public init(majorBrand: Mp4AtomFourCc, minorVersion: Swift.UInt32, compatibleBrands: [Mp4AtomFourCc])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFtyp, b: Mp4AtomFtyp) -> Swift.Bool
}
public struct Mp4AtomStyp: Sendable, CustomDebugStringConvertible, Equatable {
	public var majorBrand: Mp4AtomFourCc
	public var minorVersion: Swift.UInt32
	public var compatibleBrands: [Mp4AtomFourCc]
	@inlinable public init(majorBrand: Mp4AtomFourCc, minorVersion: Swift.UInt32, compatibleBrands: [Mp4AtomFourCc])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStyp, b: Mp4AtomStyp) -> Swift.Bool
}
public struct Mp4AtomPitm: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemId: Swift.UInt32
	@inlinable public init(itemId: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPitm, b: Mp4AtomPitm) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPitm
}
public struct Mp4AtomIloc: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemLocations: [Mp4AtomItemLocation]
	@inlinable public init(itemLocations: [Mp4AtomItemLocation])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIloc, b: Mp4AtomIloc) -> Swift.Bool
}
public struct Mp4AtomIinf: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemInfos: [Mp4AtomItemInfoEntry]
	@inlinable public init(itemInfos: [Mp4AtomItemInfoEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIinf, b: Mp4AtomIinf) -> Swift.Bool
}
public struct Mp4AtomAuxc: Sendable, CustomDebugStringConvertible, Equatable {
	public var auxType: Swift.String
	public var auxSubtype: Bytes
	@inlinable public init(auxType: Swift.String, auxSubtype: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAuxc, b: Mp4AtomAuxc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAuxc
}
public struct Mp4AtomClap: Sendable, CustomDebugStringConvertible, Equatable {
	public var cleanApertureWidthN: Swift.UInt32
	public var cleanApertureWidthD: Swift.UInt32
	public var cleanApertureHeightN: Swift.UInt32
	public var cleanApertureHeightD: Swift.UInt32
	public var horizOffN: Swift.Int32
	public var horizOffD: Swift.UInt32
	public var vertOffN: Swift.Int32
	public var vertOffD: Swift.UInt32
	@inlinable public init(cleanApertureWidthN: Swift.UInt32, cleanApertureWidthD: Swift.UInt32, cleanApertureHeightN: Swift.UInt32, cleanApertureHeightD: Swift.UInt32, horizOffN: Swift.Int32, horizOffD: Swift.UInt32, vertOffN: Swift.Int32, vertOffD: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomClap, b: Mp4AtomClap) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomClap
}
public struct Mp4AtomImir: Sendable, CustomDebugStringConvertible, Equatable {
	public var axis: Swift.UInt8
	@inlinable public init(axis: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomImir, b: Mp4AtomImir) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomImir
}
public struct Mp4AtomIrot: Sendable, CustomDebugStringConvertible, Equatable {
	public var angle: Swift.UInt8
	@inlinable public init(angle: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIrot, b: Mp4AtomIrot) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIrot
}
public struct Mp4AtomIscl: Sendable, CustomDebugStringConvertible, Equatable {
	public var targetWidthNumerator: Swift.UInt16
	public var targetWidthDenominator: Swift.UInt16
	public var targetHeightNumerator: Swift.UInt16
	public var targetHeightDenominator: Swift.UInt16
	@inlinable public init(targetWidthNumerator: Swift.UInt16, targetWidthDenominator: Swift.UInt16, targetHeightNumerator: Swift.UInt16, targetHeightDenominator: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIscl, b: Mp4AtomIscl) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIscl
}
public struct Mp4AtomIspe: Sendable, CustomDebugStringConvertible, Equatable {
	public var width: Swift.UInt32
	public var height: Swift.UInt32
	@inlinable public init(width: Swift.UInt32, height: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIspe, b: Mp4AtomIspe) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIspe
}
public struct Mp4AtomPixi: Sendable, CustomDebugStringConvertible, Equatable {
	public var bitsPerChannel: Bytes
	@inlinable public init(bitsPerChannel: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPixi, b: Mp4AtomPixi) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPixi
}
public struct Mp4AtomRref: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceTypes: [Mp4AtomFourCc]
	@inlinable public init(referenceTypes: [Mp4AtomFourCc])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomRref, b: Mp4AtomRref) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomRref
}
public struct Mp4AtomIpma: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemProperties: [Mp4AtomPropertyAssociations]
	@inlinable public init(itemProperties: [Mp4AtomPropertyAssociations])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIpma, b: Mp4AtomIpma) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIpma
}
public struct Mp4AtomIref: Sendable, CustomDebugStringConvertible, Equatable {
	public var references: [Mp4AtomReference]
	@inlinable public init(references: [Mp4AtomReference])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIref, b: Mp4AtomIref) -> Swift.Bool
}
public struct Mp4AtomIdat: Sendable, CustomDebugStringConvertible, Equatable {
	public var data: Bytes
	@inlinable public init(data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIdat, b: Mp4AtomIdat) -> Swift.Bool
}
public struct Mp4AtomIlst: Sendable, CustomDebugStringConvertible, Equatable {
	public var name: Mp4AtomName?
	public var year: Mp4AtomYear?
	public var covr: Mp4AtomCovr?
	public var desc: Mp4AtomDesc?
	public var ctoo: Mp4AtomTool?
	public var cprt: Mp4AtomCopyright?
	public var mdta: [(Swift.UInt32, Mp4AtomIlstData)]
	@inlinable public init(name: Mp4AtomName?, year: Mp4AtomYear?, covr: Mp4AtomCovr?, desc: Mp4AtomDesc?, ctoo: Mp4AtomTool?, cprt: Mp4AtomCopyright?, mdta: [(Swift.UInt32, Mp4AtomIlstData)])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIlst, b: Mp4AtomIlst) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIlst
}
public struct Mp4AtomCovr: Sendable, CustomDebugStringConvertible, Equatable {
	public var _0: Bytes
	@inlinable public init(_ _0: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCovr, b: Mp4AtomCovr) -> Swift.Bool
}
public struct Mp4AtomDesc: Sendable, CustomDebugStringConvertible, Equatable {
	public var countryIndicator: Swift.UInt16
	public var languageIndicator: Swift.UInt16
	public var text: Swift.String
	@inlinable public init(countryIndicator: Swift.UInt16, languageIndicator: Swift.UInt16, text: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDesc, b: Mp4AtomDesc) -> Swift.Bool
}
public struct Mp4AtomTool: Sendable, CustomDebugStringConvertible, Equatable {
	public var countryIndicator: Swift.UInt16
	public var languageIndicator: Swift.UInt16
	public var text: Swift.String
	@inlinable public init(countryIndicator: Swift.UInt16, languageIndicator: Swift.UInt16, text: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTool, b: Mp4AtomTool) -> Swift.Bool
	@inlinable public static func new(_ countryIndicator: Swift.UInt16, _ languageIndicator: Swift.UInt16, _ text: Swift.String) throws -> Mp4AtomTool
}
public struct Mp4AtomName: Sendable, CustomDebugStringConvertible, Equatable {
	public var _0: Swift.String
	@inlinable public init(_ _0: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomName, b: Mp4AtomName) -> Swift.Bool
}
public struct Mp4AtomYear: Sendable, CustomDebugStringConvertible, Equatable {
	public var _0: Swift.String
	@inlinable public init(_ _0: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomYear, b: Mp4AtomYear) -> Swift.Bool
}
public struct Mp4AtomKeys: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomKeyEntry]
	@inlinable public init(entries: [Mp4AtomKeyEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomKeys, b: Mp4AtomKeys) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomKeys
}
public struct Mp4AtomMvhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var creationTime: Swift.UInt64
	public var modificationTime: Swift.UInt64
	public var timescale: Swift.UInt32
	public var duration: Swift.UInt64
	public var rate: Mp4AtomFixedPointU16
	public var volume: Mp4AtomFixedPointU8
	public var matrix: Mp4AtomMatrix
	public var nextTrackId: Swift.UInt32
	@inlinable public init(creationTime: Swift.UInt64, modificationTime: Swift.UInt64, timescale: Swift.UInt32, duration: Swift.UInt64, rate: Mp4AtomFixedPointU16, volume: Mp4AtomFixedPointU8, matrix: Mp4AtomMatrix, nextTrackId: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMvhd, b: Mp4AtomMvhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMvhd
}
public struct Mp4AtomAinf: Sendable, CustomDebugStringConvertible, Equatable {
	public var hidden: Swift.Bool
	public var profileVersion: Swift.UInt32
	public var apid: Swift.String
	@inlinable public init(hidden: Swift.Bool, profileVersion: Swift.UInt32, apid: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAinf, b: Mp4AtomAinf) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAinf
}
public struct Mp4AtomSkip: Sendable, CustomDebugStringConvertible, Equatable {
	public var zeroed: Mp4AtomZeroed
	@inlinable public init(zeroed: Mp4AtomZeroed)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSkip, b: Mp4AtomSkip) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSkip
}
public struct Mp4AtomStts: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomSttsEntry]
	@inlinable public init(entries: [Mp4AtomSttsEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStts, b: Mp4AtomStts) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStts
}
public struct Mp4AtomStsc: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomStscEntry]
	@inlinable public init(entries: [Mp4AtomStscEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStsc, b: Mp4AtomStsc) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStsc
}
public struct Mp4AtomStsz: Sendable, CustomDebugStringConvertible, Equatable {
	public var samples: Mp4AtomStszSamples
	@inlinable public init(samples: Mp4AtomStszSamples)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStsz, b: Mp4AtomStsz) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStsz
}
public struct Mp4AtomStss: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Swift.UInt32]
	@inlinable public init(entries: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStss, b: Mp4AtomStss) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStss
}
public struct Mp4AtomStco: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Swift.UInt32]
	@inlinable public init(entries: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStco, b: Mp4AtomStco) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStco
}
public struct Mp4AtomCo64: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Swift.UInt64]
	@inlinable public init(entries: [Swift.UInt64])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCo64, b: Mp4AtomCo64) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCo64
}
public struct Mp4AtomCslg: Sendable, CustomDebugStringConvertible, Equatable {
	public var compositionToDtsShift: Swift.Int64
	public var leastDecodeToDisplayShift: Swift.Int64
	public var greatestDecodeToDisplayDelta: Swift.Int64
	public var compositionStartTime: Swift.Int64
	public var compositionEndTime: Swift.Int64
	@inlinable public init(compositionToDtsShift: Swift.Int64, leastDecodeToDisplayShift: Swift.Int64, greatestDecodeToDisplayDelta: Swift.Int64, compositionStartTime: Swift.Int64, compositionEndTime: Swift.Int64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCslg, b: Mp4AtomCslg) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCslg
}
public struct Mp4AtomCtts: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomCttsEntry]
	@inlinable public init(entries: [Mp4AtomCttsEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCtts, b: Mp4AtomCtts) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCtts
}
public struct Mp4AtomSbgp: Sendable, CustomDebugStringConvertible, Equatable {
	public var groupingType: Mp4AtomFourCc
	public var groupingTypeParameter: Swift.UInt32?
	public var entries: [Mp4AtomSbgpEntry]
	@inlinable public init(groupingType: Mp4AtomFourCc, groupingTypeParameter: Swift.UInt32?, entries: [Mp4AtomSbgpEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSbgp, b: Mp4AtomSbgp) -> Swift.Bool
}
public struct Mp4AtomSgpd: Sendable, CustomDebugStringConvertible, Equatable {
	public var groupingType: Mp4AtomFourCc
	public var defaultLength: Swift.UInt32?
	public var defaultGroupDescriptionIndex: Swift.UInt32?
	public var staticGroupDescription: Swift.Bool
	public var staticMapping: Swift.Bool
	public var essential: Swift.Bool
	public var entries: [Mp4AtomSgpdEntry]
	@inlinable public init(groupingType: Mp4AtomFourCc, defaultLength: Swift.UInt32?, defaultGroupDescriptionIndex: Swift.UInt32?, staticGroupDescription: Swift.Bool, staticMapping: Swift.Bool, essential: Swift.Bool, entries: [Mp4AtomSgpdEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSgpd, b: Mp4AtomSgpd) -> Swift.Bool
}
public struct Mp4AtomSubs: Sendable, CustomDebugStringConvertible, Equatable {
	public var flags: ArrayU8x3
	public var entries: [Mp4AtomSubsEntry]
	@inlinable public init(flags: ArrayU8x3, entries: [Mp4AtomSubsEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSubs, b: Mp4AtomSubs) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSubs
}
public struct Mp4AtomSaio: Sendable, CustomDebugStringConvertible, Equatable {
	public var auxInfo: Mp4AtomAuxInfo?
	public var offsets: [Swift.UInt64]
	@inlinable public init(auxInfo: Mp4AtomAuxInfo?, offsets: [Swift.UInt64])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSaio, b: Mp4AtomSaio) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSaio
}
public struct Mp4AtomSaiz: Sendable, CustomDebugStringConvertible, Equatable {
	public var auxInfo: Mp4AtomAuxInfo?
	public var defaultSampleInfoSize: Swift.UInt8
	public var sampleCount: Swift.UInt32
	public var sampleInfoSize: Bytes
	@inlinable public init(auxInfo: Mp4AtomAuxInfo?, defaultSampleInfoSize: Swift.UInt8, sampleCount: Swift.UInt32, sampleInfoSize: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSaiz, b: Mp4AtomSaiz) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSaiz
}
public struct Mp4AtomDinf: Sendable, CustomDebugStringConvertible, Equatable {
	public var dref: Mp4AtomDref
	@inlinable public init(dref: Mp4AtomDref)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDinf, b: Mp4AtomDinf) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomDinf
}
public struct Mp4AtomDref: Sendable, CustomDebugStringConvertible, Equatable {
	public var entries: [Mp4AtomDrefEntry]
	@inlinable public init(entries: [Mp4AtomDrefEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDref, b: Mp4AtomDref) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomDref
}
public struct Mp4AtomHmhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var maxPduSize: Swift.UInt16
	public var avgPduSize: Swift.UInt16
	public var maxBitrate: Swift.UInt32
	public var avgBitrate: Swift.UInt32
	@inlinable public init(maxPduSize: Swift.UInt16, avgPduSize: Swift.UInt16, maxBitrate: Swift.UInt32, avgBitrate: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHmhd, b: Mp4AtomHmhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHmhd
}
public struct Mp4AtomSmhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var balance: Mp4AtomFixedPointI8
	@inlinable public init(balance: Mp4AtomFixedPointI8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSmhd, b: Mp4AtomSmhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSmhd
}
public struct Mp4AtomVmhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var graphicsMode: Swift.UInt16
	public var opColor: Mp4AtomRgbColor
	@inlinable public init(graphicsMode: Swift.UInt16, opColor: Mp4AtomRgbColor)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomVmhd, b: Mp4AtomVmhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVmhd
}
public struct Mp4AtomMvex: Sendable, CustomDebugStringConvertible, Equatable {
	public var mehd: Mp4AtomMehd?
	public var trex: [Mp4AtomTrex]
	@inlinable public init(mehd: Mp4AtomMehd?, trex: [Mp4AtomTrex])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMvex, b: Mp4AtomMvex) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMvex
}
public struct Mp4AtomMehd: Sendable, CustomDebugStringConvertible, Equatable {
	public var fragmentDuration: Swift.UInt64
	@inlinable public init(fragmentDuration: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMehd, b: Mp4AtomMehd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMehd
}
public struct Mp4AtomTrex: Sendable, CustomDebugStringConvertible, Equatable {
	public var trackId: Swift.UInt32
	public var defaultSampleDescriptionIndex: Swift.UInt32
	public var defaultSampleDuration: Swift.UInt32
	public var defaultSampleSize: Swift.UInt32
	public var defaultSampleFlags: Swift.UInt32
	@inlinable public init(trackId: Swift.UInt32, defaultSampleDescriptionIndex: Swift.UInt32, defaultSampleDuration: Swift.UInt32, defaultSampleSize: Swift.UInt32, defaultSampleFlags: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTrex, b: Mp4AtomTrex) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTrex
}
public struct Mp4AtomEmsg: Sendable, CustomDebugStringConvertible, Equatable {
	public var timescale: Swift.UInt32
	public var presentationTime: Mp4AtomEmsgTimestamp
	public var eventDuration: Swift.UInt32
	public var id: Swift.UInt32
	public var schemeIdUri: Swift.String
	public var value: Swift.String
	public var messageData: Bytes
	@inlinable public init(timescale: Swift.UInt32, presentationTime: Mp4AtomEmsgTimestamp, eventDuration: Swift.UInt32, id: Swift.UInt32, schemeIdUri: Swift.String, value: Swift.String, messageData: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEmsg, b: Mp4AtomEmsg) -> Swift.Bool
}
public struct Mp4AtomMfhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var sequenceNumber: Swift.UInt32
	@inlinable public init(sequenceNumber: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMfhd, b: Mp4AtomMfhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMfhd
}
public struct Mp4AtomTfhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var trackId: Swift.UInt32
	public var baseDataOffset: Swift.UInt64?
	public var sampleDescriptionIndex: Swift.UInt32?
	public var defaultSampleDuration: Swift.UInt32?
	public var defaultSampleSize: Swift.UInt32?
	public var defaultSampleFlags: Swift.UInt32?
	public var durationIsEmpty: Swift.Bool
	public var defaultBaseIsMoof: Swift.Bool
	@inlinable public init(trackId: Swift.UInt32, baseDataOffset: Swift.UInt64?, sampleDescriptionIndex: Swift.UInt32?, defaultSampleDuration: Swift.UInt32?, defaultSampleSize: Swift.UInt32?, defaultSampleFlags: Swift.UInt32?, durationIsEmpty: Swift.Bool, defaultBaseIsMoof: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTfhd, b: Mp4AtomTfhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTfhd
}
public struct Mp4AtomTfdt: Sendable, CustomDebugStringConvertible, Equatable {
	public var baseMediaDecodeTime: Swift.UInt64
	@inlinable public init(baseMediaDecodeTime: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTfdt, b: Mp4AtomTfdt) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTfdt
}
public struct Mp4AtomTrun: Sendable, CustomDebugStringConvertible, Equatable {
	public var dataOffset: Swift.Int32?
	public var samples: Mp4AtomTrunSamples
	@inlinable public init(dataOffset: Swift.Int32?, samples: Mp4AtomTrunSamples)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTrun, b: Mp4AtomTrun) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTrun
}
public struct Mp4AtomMdat: Sendable, CustomDebugStringConvertible, Equatable {
	public var data: Bytes
	@inlinable public init(data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMdat, b: Mp4AtomMdat) -> Swift.Bool
}
public struct Mp4AtomFree: Sendable, CustomDebugStringConvertible, Equatable {
	public var zeroed: Mp4AtomZeroed
	@inlinable public init(zeroed: Mp4AtomZeroed)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFree, b: Mp4AtomFree) -> Swift.Bool
}
public struct Mp4AtomSidx: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceId: Swift.UInt32
	public var timescale: Swift.UInt32
	public var earliestPresentationTime: Swift.UInt64
	public var firstOffset: Swift.UInt64
	public var references: [Mp4AtomSegmentReference]
	@inlinable public init(referenceId: Swift.UInt32, timescale: Swift.UInt32, earliestPresentationTime: Swift.UInt64, firstOffset: Swift.UInt64, references: [Mp4AtomSegmentReference])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSidx, b: Mp4AtomSidx) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSidx
}
public struct Mp4AtomPrft: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceTrackId: Swift.UInt32
	public var ntpTimestamp: Swift.UInt64
	public var mediaTime: Swift.UInt64
	public var utcTimeSemantics: Mp4AtomReferenceTime
	@inlinable public init(referenceTrackId: Swift.UInt32, ntpTimestamp: Swift.UInt64, mediaTime: Swift.UInt64, utcTimeSemantics: Mp4AtomReferenceTime)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPrft, b: Mp4AtomPrft) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPrft
}
public struct Mp4AtomMfra: Sendable, CustomDebugStringConvertible, Equatable {
	public var tfra: [Mp4AtomTfra]
	public var mfro: Mp4AtomMfro
	@inlinable public init(tfra: [Mp4AtomTfra], mfro: Mp4AtomMfro)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMfra, b: Mp4AtomMfra) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMfra
}
public struct Mp4AtomTfra: Sendable, CustomDebugStringConvertible, Equatable {
	public var trackId: Swift.UInt32
	public var entries: [Mp4AtomFragmentInfo]
	@inlinable public init(trackId: Swift.UInt32, entries: [Mp4AtomFragmentInfo])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTfra, b: Mp4AtomTfra) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTfra
}
public struct Mp4AtomMfro: Sendable, CustomDebugStringConvertible, Equatable {
	public var parentSize: Swift.UInt32
	@inlinable public init(parentSize: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMfro, b: Mp4AtomMfro) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMfro
}
public struct Mp4AtomChplEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var start: Swift.UInt64
	public var title: Swift.String
	@inlinable public init(start: Swift.UInt64, title: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomChplEntry, b: Mp4AtomChplEntry) -> Swift.Bool
}
public struct Mp4AtomSvhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var metadataSource: Swift.String
	@inlinable public init(metadataSource: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSvhd, b: Mp4AtomSvhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSvhd
}
public struct Mp4AtomMust: Sendable, CustomDebugStringConvertible, Equatable {
	public var requiredBoxTypes: [Mp4AtomFourCc]
	@inlinable public init(requiredBoxTypes: [Mp4AtomFourCc])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMust, b: Mp4AtomMust) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMust
}
public struct Mp4AtomEsdsDecoderConfig: Sendable, CustomDebugStringConvertible, Equatable {
	public var objectTypeIndication: Swift.UInt8
	public var streamType: Swift.UInt8
	public var upStream: Swift.UInt8
	public var bufferSizeDb: Mp4AtomU24
	public var maxBitrate: Swift.UInt32
	public var avgBitrate: Swift.UInt32
	public var decSpecific: Mp4AtomEsdsDecoderSpecific?
	@inlinable public init(objectTypeIndication: Swift.UInt8, streamType: Swift.UInt8, upStream: Swift.UInt8, bufferSizeDb: Mp4AtomU24, maxBitrate: Swift.UInt32, avgBitrate: Swift.UInt32, decSpecific: Mp4AtomEsdsDecoderSpecific?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEsdsDecoderConfig, b: Mp4AtomEsdsDecoderConfig) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEsdsDecoderConfig
}
public struct Mp4AtomEsdsSlConfig: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEsdsSlConfig, b: Mp4AtomEsdsSlConfig) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEsdsSlConfig
}
public struct Mp4AtomU24: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomU24, b: Mp4AtomU24) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomU24
}
public struct Mp4AtomItemLocation: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemId: Swift.UInt32
	public var constructionMethod: Swift.UInt8
	public var dataReferenceIndex: Swift.UInt16
	public var baseOffset: Swift.UInt64
	public var extents: [Mp4AtomItemLocationExtent]
	@inlinable public init(itemId: Swift.UInt32, constructionMethod: Swift.UInt8, dataReferenceIndex: Swift.UInt16, baseOffset: Swift.UInt64, extents: [Mp4AtomItemLocationExtent])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomItemLocation, b: Mp4AtomItemLocation) -> Swift.Bool
}
public struct Mp4AtomItemInfoEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemId: Swift.UInt32
	public var itemProtectionIndex: Swift.UInt16
	public var itemType: Mp4AtomFourCc?
	public var itemName: Swift.String
	public var contentType: Swift.String?
	public var contentEncoding: Swift.String?
	public var itemUriType: Swift.String?
	public var itemNotInPresentation: Swift.Bool
	@inlinable public init(itemId: Swift.UInt32, itemProtectionIndex: Swift.UInt16, itemType: Mp4AtomFourCc?, itemName: Swift.String, contentType: Swift.String?, contentEncoding: Swift.String?, itemUriType: Swift.String?, itemNotInPresentation: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomItemInfoEntry, b: Mp4AtomItemInfoEntry) -> Swift.Bool
}
public struct Mp4AtomPropertyAssociations: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemId: Swift.UInt32
	public var associations: [Mp4AtomPropertyAssociation]
	@inlinable public init(itemId: Swift.UInt32, associations: [Mp4AtomPropertyAssociation])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPropertyAssociations, b: Mp4AtomPropertyAssociations) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPropertyAssociations
}
public struct Mp4AtomReference: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceType: Mp4AtomFourCc
	public var fromItemId: Swift.UInt32
	public var toItemIds: [Swift.UInt32]
	@inlinable public init(referenceType: Mp4AtomFourCc, fromItemId: Swift.UInt32, toItemIds: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomReference, b: Mp4AtomReference) -> Swift.Bool
}
public struct Mp4AtomCopyright: Sendable, CustomDebugStringConvertible, Equatable {
	public var countryIndicator: Swift.UInt16
	public var languageIndicator: Swift.UInt16
	public var text: Swift.String
	@inlinable public init(countryIndicator: Swift.UInt16, languageIndicator: Swift.UInt16, text: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCopyright, b: Mp4AtomCopyright) -> Swift.Bool
}
public struct Mp4AtomIlstData: Sendable, CustomDebugStringConvertible, Equatable {
	public var countryIndicator: Swift.UInt16
	public var languageIndicator: Swift.UInt16
	public var value: Mp4AtomIlstDataValue
	@inlinable public init(countryIndicator: Swift.UInt16, languageIndicator: Swift.UInt16, value: Mp4AtomIlstDataValue)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIlstData, b: Mp4AtomIlstData) -> Swift.Bool
}
public struct Mp4AtomKeyEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var keyNamespace: Mp4AtomFourCc
	public var keyValue: Swift.String
	@inlinable public init(keyNamespace: Mp4AtomFourCc, keyValue: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomKeyEntry, b: Mp4AtomKeyEntry) -> Swift.Bool
}
public struct Mp4AtomZeroed: Sendable, CustomDebugStringConvertible, Equatable {
	public var size: Swift.UInt
	@inlinable public init(size: Swift.UInt)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomZeroed, b: Mp4AtomZeroed) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomZeroed
	@inlinable public static func new(_ size: Swift.UInt) -> Mp4AtomZeroed
}
public struct Mp4AtomSttsEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var sampleCount: Swift.UInt32
	public var sampleDelta: Swift.UInt32
	@inlinable public init(sampleCount: Swift.UInt32, sampleDelta: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSttsEntry, b: Mp4AtomSttsEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSttsEntry
}
public struct Mp4AtomStscEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var firstChunk: Swift.UInt32
	public var samplesPerChunk: Swift.UInt32
	public var sampleDescriptionIndex: Swift.UInt32
	@inlinable public init(firstChunk: Swift.UInt32, samplesPerChunk: Swift.UInt32, sampleDescriptionIndex: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStscEntry, b: Mp4AtomStscEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStscEntry
}
public struct Mp4AtomCttsEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var sampleCount: Swift.UInt32
	public var sampleOffset: Swift.Int64
	@inlinable public init(sampleCount: Swift.UInt32, sampleOffset: Swift.Int64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCttsEntry, b: Mp4AtomCttsEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCttsEntry
}
public struct Mp4AtomSbgpEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var sampleCount: Swift.UInt32
	public var groupDescriptionIndex: Swift.UInt32
	@inlinable public init(sampleCount: Swift.UInt32, groupDescriptionIndex: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSbgpEntry, b: Mp4AtomSbgpEntry) -> Swift.Bool
}
public struct Mp4AtomSgpdEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var descriptionLength: Swift.UInt32?
	public var entry: Mp4AtomAnySampleGroupEntry
	@inlinable public init(descriptionLength: Swift.UInt32?, entry: Mp4AtomAnySampleGroupEntry)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSgpdEntry, b: Mp4AtomSgpdEntry) -> Swift.Bool
}
public struct Mp4AtomSubsEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var sampleDelta: Swift.UInt32
	public var subsamples: [Mp4AtomSubsSubsample]
	@inlinable public init(sampleDelta: Swift.UInt32, subsamples: [Mp4AtomSubsSubsample])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSubsEntry, b: Mp4AtomSubsEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSubsEntry
}
public struct Mp4AtomAuxInfo: Sendable, CustomDebugStringConvertible, Equatable {
	public var auxInfoType: Mp4AtomFourCc
	public var auxInfoTypeParameter: Swift.UInt32
	@inlinable public init(auxInfoType: Mp4AtomFourCc, auxInfoTypeParameter: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAuxInfo, b: Mp4AtomAuxInfo) -> Swift.Bool
}
public struct Mp4AtomRgbColor: Sendable, CustomDebugStringConvertible, Equatable {
	public var red: Swift.UInt16
	public var green: Swift.UInt16
	public var blue: Swift.UInt16
	@inlinable public init(red: Swift.UInt16, green: Swift.UInt16, blue: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomRgbColor, b: Mp4AtomRgbColor) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomRgbColor
}
public struct Mp4AtomSegmentReference: Sendable, CustomDebugStringConvertible, Equatable {
	public var referenceType: Swift.Bool
	public var referencedSize: Swift.UInt32
	public var subsegmentDuration: Swift.UInt32
	public var startsWithSap: Swift.Bool
	public var sapType: Swift.UInt8
	public var sapDeltaTime: Swift.UInt32
	@inlinable public init(referenceType: Swift.Bool, referencedSize: Swift.UInt32, subsegmentDuration: Swift.UInt32, startsWithSap: Swift.Bool, sapType: Swift.UInt8, sapDeltaTime: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSegmentReference, b: Mp4AtomSegmentReference) -> Swift.Bool
}
public struct Mp4AtomFragmentInfo: Sendable, CustomDebugStringConvertible, Equatable {
	public var time: Swift.UInt64
	public var moofOffset: Swift.UInt64
	public var trafNumber: Swift.UInt32
	public var trunNumber: Swift.UInt32
	public var sampleDelta: Swift.UInt32
	@inlinable public init(time: Swift.UInt64, moofOffset: Swift.UInt64, trafNumber: Swift.UInt32, trunNumber: Swift.UInt32, sampleDelta: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFragmentInfo, b: Mp4AtomFragmentInfo) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomFragmentInfo
}
public struct Mp4AtomC2pa: Sendable, CustomDebugStringConvertible, Equatable {
	public var boxPurpose: Swift.String
	public var data: Bytes
	@inlinable public init(boxPurpose: Swift.String, data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomC2pa, b: Mp4AtomC2pa) -> Swift.Bool
}
public struct Mp4AtomExtendedType: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomExtendedType, b: Mp4AtomExtendedType) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomExtendedType
	@inlinable public static func new(_ value: ArrayU8x16) -> Mp4AtomExtendedType
}
public struct Mp4AtomPrhd: Sendable, CustomDebugStringConvertible, Equatable {
	public var poseYawDegrees: Swift.Int32
	public var posePitchDegrees: Swift.Int32
	public var poseRollDegrees: Swift.Int32
	@inlinable public init(poseYawDegrees: Swift.Int32, posePitchDegrees: Swift.Int32, poseRollDegrees: Swift.Int32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPrhd, b: Mp4AtomPrhd) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPrhd
}
public struct Mp4AtomPrji: Sendable, CustomDebugStringConvertible, Equatable {
	public var projectionKind: Mp4AtomFourCc
	@inlinable public init(projectionKind: Mp4AtomFourCc)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPrji, b: Mp4AtomPrji) -> Swift.Bool
	@inlinable public static var RECTILINEAR: Mp4AtomFourCc { get }
	@inlinable public static var EQUIRECTANGULAR: Mp4AtomFourCc { get }
	@inlinable public static var HALFEQUIRECTANGULAR: Mp4AtomFourCc { get }
	@inlinable public static var FISHEYE: Mp4AtomFourCc { get }
	@inlinable public static var PARAMETRICIMMERSIVE: Mp4AtomFourCc { get }
}
public struct Mp4AtomStri: Sendable, CustomDebugStringConvertible, Equatable {
	public var eyeViewsReversed: Swift.Bool
	public var hasAdditionalViews: Swift.Bool
	public var hasRightEyeView: Swift.Bool
	public var hasLeftEyeView: Swift.Bool
	@inlinable public init(eyeViewsReversed: Swift.Bool, hasAdditionalViews: Swift.Bool, hasRightEyeView: Swift.Bool, hasLeftEyeView: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStri, b: Mp4AtomStri) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStri
}
public struct Mp4AtomHero: Sendable, CustomDebugStringConvertible, Equatable {
	public var heroEyeIndicator: Swift.UInt8
	@inlinable public init(heroEyeIndicator: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomHero, b: Mp4AtomHero) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHero
}
public struct Mp4AtomPkin: Sendable, CustomDebugStringConvertible, Equatable {
	public var viewPackingKind: Mp4AtomFourCc
	@inlinable public init(viewPackingKind: Mp4AtomFourCc)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPkin, b: Mp4AtomPkin) -> Swift.Bool
	@inlinable public static var SIDEBYSIDE: Mp4AtomFourCc { get }
	@inlinable public static var OVERUNDER: Mp4AtomFourCc { get }
}
public struct Mp4AtomEsdsDecoderSpecific: Sendable, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	public var freqIndex: Swift.UInt8
	public var chanConf: Swift.UInt8
	public var raw: Bytes
	@inlinable public init(profile: Swift.UInt8, freqIndex: Swift.UInt8, chanConf: Swift.UInt8, raw: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEsdsDecoderSpecific, b: Mp4AtomEsdsDecoderSpecific) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEsdsDecoderSpecific
}
public struct Mp4AtomExplicitSpeakerPosition: Sendable, CustomDebugStringConvertible, Equatable {
	public var azimuth: Swift.Int16
	public var elevation: Swift.Int8
	@inlinable public init(azimuth: Swift.Int16, elevation: Swift.Int8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomExplicitSpeakerPosition, b: Mp4AtomExplicitSpeakerPosition) -> Swift.Bool
}
public struct Mp4AtomItemLocationExtent: Sendable, CustomDebugStringConvertible, Equatable {
	public var itemReferenceIndex: Swift.UInt64
	public var offset: Swift.UInt64
	public var length: Swift.UInt64
	@inlinable public init(itemReferenceIndex: Swift.UInt64, offset: Swift.UInt64, length: Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomItemLocationExtent, b: Mp4AtomItemLocationExtent) -> Swift.Bool
}
public struct Mp4AtomPropertyAssociation: Sendable, CustomDebugStringConvertible, Equatable {
	public var essential: Swift.Bool
	public var propertyIndex: Swift.UInt16
	@inlinable public init(essential: Swift.Bool, propertyIndex: Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomPropertyAssociation, b: Mp4AtomPropertyAssociation) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPropertyAssociation
}
public struct Mp4AtomSubsSubsample: Sendable, CustomDebugStringConvertible, Equatable {
	public var size: Mp4AtomSubsSubsampleSize
	public var priority: Swift.UInt8
	public var discardable: Swift.Bool
	public var codecSpecificParameters: Bytes
	@inlinable public init(size: Mp4AtomSubsSubsampleSize, priority: Swift.UInt8, discardable: Swift.Bool, codecSpecificParameters: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSubsSubsample, b: Mp4AtomSubsSubsample) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSubsSubsample
}
public struct Mp4AtomUrl: Sendable, CustomDebugStringConvertible, Equatable {
	public var location: Swift.String
	@inlinable public init(location: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUrl, b: Mp4AtomUrl) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomUrl
}
public struct Mp4AtomUrn: Sendable, CustomDebugStringConvertible, Equatable {
	public var name: Swift.String
	public var location: Swift.String
	@inlinable public init(name: Swift.String, location: Swift.String)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUrn, b: Mp4AtomUrn) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomUrn
}
public struct Mp4AtomTrunEntry: Sendable, CustomDebugStringConvertible, Equatable {
	public var duration: Swift.UInt32?
	public var size: Swift.UInt32?
	public var flags: Swift.UInt32?
	public var cts: Swift.Int64?
	@inlinable public init(duration: Swift.UInt32?, size: Swift.UInt32?, flags: Swift.UInt32?, cts: Swift.Int64?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTrunEntry, b: Mp4AtomTrunEntry) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTrunEntry
}
public struct RustlsCryptoHpkeHpkePublicKey: Sendable, CustomDebugStringConvertible {
	public var _0: Bytes
	@inlinable public init(_ _0: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct Mp4AtomEqui: Sendable, CustomDebugStringConvertible, Equatable {
	public var projectionBoundsTop: Swift.UInt32
	public var projectionBoundsBottom: Swift.UInt32
	public var projectionBoundsLeft: Swift.UInt32
	public var projectionBoundsRight: Swift.UInt32
	@inlinable public init(projectionBoundsTop: Swift.UInt32, projectionBoundsBottom: Swift.UInt32, projectionBoundsLeft: Swift.UInt32, projectionBoundsRight: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEqui, b: Mp4AtomEqui) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEqui
}
public struct Mp4AtomCbmp: Sendable, CustomDebugStringConvertible, Equatable {
	public var layout: Swift.UInt32
	public var padding: Swift.UInt32
	@inlinable public init(layout: Swift.UInt32, padding: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomCbmp, b: Mp4AtomCbmp) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCbmp
}
public struct Mp4AtomMshp: Sendable, CustomDebugStringConvertible, Equatable {
	public var crc: Swift.UInt32
	public var encodingFourCc: Mp4AtomFourCc
	public var data: Bytes
	@inlinable public init(crc: Swift.UInt32, encodingFourCc: Mp4AtomFourCc, data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomMshp, b: Mp4AtomMshp) -> Swift.Bool
}
public struct Mp4AtomBlin: Sendable, CustomDebugStringConvertible, Equatable {
	public var baselineValue: Swift.UInt32
	@inlinable public init(baselineValue: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomBlin, b: Mp4AtomBlin) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomBlin
}
public struct Mp4AtomDadj: Sendable, CustomDebugStringConvertible, Equatable {
	public var disparityAdjustment: Swift.Int32
	@inlinable public init(disparityAdjustment: Swift.Int32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDadj, b: Mp4AtomDadj) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomDadj
}
public enum EncodeError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case boundsExceeded
	case tooLarge
	case short
	case invalidState
	case tooMany
	case version
	case unsupported
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum InvalidPattern: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case emptySegment
	case invalidSegment(Swift.String)
	case multipleGlobstars
	case tooManySegments
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: InvalidPattern, b: InvalidPattern) -> Swift.Bool
}
public enum SessionError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case cancel
	case `internal`
	case unauthorized
	case expired
	case protocolViolation
	case keyValueFormatting
	case tooManyRequests
	case goawayTimeout
	case timeout
	case authTokenCacheOverflow
	case version
	case malformedAuthToken
	case unknownAuthTokenAlias
	case tooManyRequestUpdates
	case app(Swift.UInt16)
	case unknown(Swift.UInt32)
	case unsupported
	case invalidRequestId
	case invalidPath
	case invalidAuthority
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: SessionError, b: SessionError) -> Swift.Bool
	@inlinable public func toCode() -> Swift.UInt32
	@inlinable public static func fromCode(_ code: Swift.UInt32) -> SessionError
}
public enum StreamError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case session(SessionError)
	case `internal`
	case cancel
	case deliveryTimeout
	case controlTimeout
	case goingAway
	case tooFarBehind
	case malformedTrack
	case notFound
	case unroutable
	case old
	case evicted
	case wrongSize
	case frameTooLarge
	case groupTooLarge
	case timestampMismatch
	case unauthorized
	case app(Swift.UInt16)
	case unknown(Swift.UInt32)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StreamError, b: StreamError) -> Swift.Bool
	@inlinable public func toCode() -> Swift.UInt32
	@inlinable public static func fromCode(_ code: Swift.UInt32) -> StreamError
}
public enum InvalidHop: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case range
	case tooMany
	case duplicate
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: InvalidHop, b: InvalidHop) -> Swift.Bool
}
public enum IntersectionError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case tooManyPatterns
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: IntersectionError, b: IntersectionError) -> Swift.Bool
}
public enum Segment: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case literal(Swift.String)
	case wildcard
	case partial(prefix: Swift.String, suffix: Swift.String)
	case globstar
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Segment, b: Segment) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum ClockMissedTickBehavior: Sendable, CustomDebugStringConvertible, Equatable {
	case burst
	case delay
	case skip
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ClockMissedTickBehavior, b: ClockMissedTickBehavior) -> Swift.Bool
	@inlinable public static func `default`() -> ClockMissedTickBehavior
}
public enum TokioConnectError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case unauthorized
	case forbidden
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioConnectError, b: TokioConnectError) -> Swift.Bool
	@inlinable public func isAuth() -> Swift.Bool
}
public enum TokioRestart: Sendable, CustomDebugStringConvertible, Equatable {
	case redial
	case wait
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioRestart, b: TokioRestart) -> Swift.Bool
	@inlinable public static func `default`() -> TokioRestart
}
public enum TokioStatus: Sendable, CustomDebugStringConvertible, Equatable {
	case connected
	case disconnected
	case migrating
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioStatus, b: TokioStatus) -> Swift.Bool
}
public enum TokioTransport: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case quic
	case webSocket
	case tcp
	case unix
	case webTransport
	case webRtc
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioTransport, b: TokioTransport) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func asStr() -> Swift.String
}
public enum AuthAnswer: Sendable, CustomDebugStringConvertible, Equatable {
	case granted
	case requests
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: AuthAnswer, b: AuthAnswer) -> Swift.Bool
}
public enum AuthRequestKind: Sendable, CustomDebugStringConvertible, Hashable {
	case subscribe
	case publishNamespace
	case discover
	case publish
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: AuthRequestKind, b: AuthRequestKind) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum PathError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case tooLong
	case emptySegment
	case invalidEscape
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: PathError, b: PathError) -> Swift.Bool
}
public enum SetupRole: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case publisher
	case subscriber
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: SetupRole, b: SetupRole) -> Swift.Bool
	@inlinable public func asStr() -> Swift.String
}
public enum StatsRole: Sendable, CustomDebugStringConvertible, Hashable {
	case publisher
	case subscriber
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRole, b: StatsRole) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func asStr() -> Swift.String
}
public enum StatsMessage: Sendable, CustomDebugStringConvertible, Equatable {
	case session
	case announce
	case trackInfo
	case subscribe
	case fetch
	case probe
	case auth
	case goaway
	case other
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsMessage, b: StatsMessage) -> Swift.Bool
	@inlinable public func asStr() -> Swift.String
}
public enum StatsFailure: Sendable, CustomDebugStringConvertible, Equatable {
	case idleTimeout
	case transport
	case application
	case expired
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsFailure, b: StatsFailure) -> Swift.Bool
	@inlinable public static var ALL: [StatsFailure] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum StatsRefusalKind: Sendable, CustomDebugStringConvertible, Equatable {
	case refused
	case unavailable
	case request
	case forbidden
	case lan
	case expired
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRefusalKind, b: StatsRefusalKind) -> Swift.Bool
	@inlinable public static var ALL: [StatsRefusalKind] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum TransportAddress: Sendable, CustomDebugStringConvertible, Equatable {
	case uri
	case setup
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportAddress, b: TransportAddress) -> Swift.Bool
}
public enum TransportCloseKind: Sendable, CustomDebugStringConvertible, Equatable {
	case transport
	case idleTimeout
	case closed
	case evicted
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportCloseKind, b: TransportCloseKind) -> Swift.Bool
	@inlinable public static func `default`() -> TransportCloseKind
}
public enum TrackAnchor: Sendable, CustomDebugStringConvertible, Hashable {
	case absolute(Swift.UInt64)
	case relative(Swift.UInt64)
	case time(Timestamp)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackAnchor, b: TrackAnchor) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum TrackOrder: Sendable, CustomDebugStringConvertible, Hashable {
	case ascending
	case descending
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackOrder, b: TrackOrder) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> TrackOrder
}
public enum OriginLink: Sendable, CustomDebugStringConvertible, Hashable {
	case local
	case peer
	case upstream
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginLink, b: OriginLink) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> OriginLink
}
public enum OriginSource: Sendable, CustomDebugStringConvertible, Hashable {
	case local
	case peer(Hop)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginSource, b: OriginSource) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> OriginSource
}
public enum HangHealthVerdict: Sendable, CustomDebugStringConvertible, Hashable {
	case unknown
	case healthy
	case degraded
	case unhealthy
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthVerdict, b: HangHealthVerdict) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> HangHealthVerdict
}
public enum HangHealthRole: Sendable, CustomDebugStringConvertible, Hashable {
	case publisher
	case viewer
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthRole, b: HangHealthRole) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangHealthReset: Sendable, CustomDebugStringConvertible, Hashable {
	case decrease
	case replaced
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthReset, b: HangHealthReset) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangHealthInput: Sendable, CustomDebugStringConvertible, Hashable {
	case rtt
	case loss
	case headroom
	case skipped
	case late
	case errors
	case stalled
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangHealthInput, b: HangHealthInput) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangCatalogAudioCodec: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case aac(HangCatalogAac)
	case opus
	case pcm
	case flac
	case mp3
	case mp2
	case ac3
	case ec3
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAudioCodec, b: HangCatalogAudioCodec) -> Swift.Bool
	@inlinable public func kind() -> HangCatalogAudioCodecKind
}
public enum HangCatalogAudioCodecKind: Sendable, CustomDebugStringConvertible, Hashable {
	case aac
	case opus
	case unknown
	case flac
	case mp3
	case pcm
	case ac3
	case ec3
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAudioCodecKind, b: HangCatalogAudioCodecKind) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangCatalogAudioLayout: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case ambisonic
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAudioLayout, b: HangCatalogAudioLayout) -> Swift.Bool
}
public enum HangCatalogMode: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case snapshot
	case stream
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogMode, b: HangCatalogMode) -> Swift.Bool
}
public enum HangCatalogTextFormat: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case vtt
	case ttml
	case utf8
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogTextFormat, b: HangCatalogTextFormat) -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogTextFormat
}
public enum HangCatalogTextRole: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case subtitle
	case caption
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogTextRole, b: HangCatalogTextRole) -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogTextRole
}
public enum HangCatalogVideoCodec: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case h264(HangCatalogH264)
	case h265(HangCatalogH265)
	case vp9(HangCatalogVp9)
	case av1(HangCatalogAv1)
	case vp8
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogVideoCodec, b: HangCatalogVideoCodec) -> Swift.Bool
	@inlinable public func kind() -> HangCatalogVideoCodecKind
}
public enum HangCatalogVideoCodecKind: Sendable, CustomDebugStringConvertible, Hashable {
	case h264
	case h265
	case vp8
	case vp9
	case av1
	case unknown
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogVideoCodecKind, b: HangCatalogVideoCodecKind) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangCatalogRange: Sendable, CustomDebugStringConvertible, Hashable {
	case limited
	case full
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogRange, b: HangCatalogRange) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum HangCatalogStereo: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case mono
	case leftRight
	case topBottom
	case unknown(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogStereo, b: HangCatalogStereo) -> Swift.Bool
}
public enum MuxCatalogCatalogFormat: Sendable, CustomDebugStringConvertible, Hashable {
	case hang
	case msf
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCatalogCatalogFormat, b: MuxCatalogCatalogFormat) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> MuxCatalogCatalogFormat
	@inlinable public static var DEFAULT: MuxCatalogCatalogFormat { get }
	@inlinable public func `extension`() -> Swift.String
	@inlinable public static func detect(_ name: Swift.String) -> MuxCatalogCatalogFormat?
}
public enum MuxContainerKind: Sendable, CustomDebugStringConvertible, Equatable {
	case audio
	case video
	case data
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerKind, b: MuxContainerKind) -> Swift.Bool
}
public enum MuxContainerRead: Sendable, CustomDebugStringConvertible {
	case frame(group: Swift.UInt64, frame: MuxContainerFrame)
	case gap(timestamp: Timestamp, duration: Timestamp)
	case end(group: Swift.UInt64, end: Timestamp)
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxImportAudioFormat: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case aac
	case opus
	case flac
	case mp3
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxImportAudioFormat, b: MuxImportAudioFormat) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum MuxImportContainerFormat: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case fmp4
	case mkv
	case ts
	case flv
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxImportContainerFormat, b: MuxImportContainerFormat) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum MuxImportFormat: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case audio(MuxImportAudioFormat)
	case video(MuxImportVideoFormat)
	case container(MuxImportContainerFormat)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxImportFormat, b: MuxImportFormat) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func kind() -> MuxImportKind
}
public enum MuxImportKind: Sendable, CustomDebugStringConvertible, Hashable {
	case audio
	case video
	case container
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxImportKind, b: MuxImportKind) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func name() -> Swift.String
}
public enum MuxImportVideoFormat: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case avc1
	case avc3
	case hvc1
	case hev1
	case av01
	case vp8
	case vp9
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxImportVideoFormat, b: MuxImportVideoFormat) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum MuxRateTarget: Sendable, CustomDebugStringConvertible, Equatable {
	case enabled(BandwidthRate)
	case disabled
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxRateTarget, b: MuxRateTarget) -> Swift.Bool
}
public enum MuxTimelineKind: Sendable, CustomDebugStringConvertible, Equatable {
	case video
	case audio
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxTimelineKind, b: MuxTimelineKind) -> Swift.Bool
}
public enum MuxTimelineError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case overflow
	case beforeOrigin
	case delayOverflow
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxTimelineError, b: MuxTimelineError) -> Swift.Bool
}
public enum TokioListenBind: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case addr(SocketAddr)
	case host(Swift.String, Swift.UInt16)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioListenBind, b: TokioListenBind) -> Swift.Bool
}
public enum TransportMemoryError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case session(code: Swift.UInt32, reason: Swift.String)
	case reset(Swift.UInt32)
	case stopped(Swift.UInt32)
	case closed
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCatalogMsfError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case unresolvableNamespace(name: Swift.String, namespace: Swift.String)
	case invalidUtf8
	case parseFrame
	case missingCmafInit(Swift.String)
	case malformedInitData(Swift.String)
	case missingVideoCodec(Swift.String)
	case missingAudioCodec(Swift.String)
	case unsupportedPackaging(name: Swift.String, packaging: Swift.String)
	case missingTextCodec(Swift.String)
	case invalidVideoCodec(name: Swift.String, codec: Swift.String)
	case invalidAudioCodec(name: Swift.String, codec: Swift.String)
	case missingAudioParams(Swift.String)
	case unsupportedDerivationPackaging(name: Swift.String, packaging: Swift.String)
	case malformedAac(Swift.String)
	case malformedOpus(Swift.String)
	case malformedFlac(Swift.String)
	case opusTrailingBytes(Swift.String)
	case unsupportedDerivationCodec(Swift.String)
	case malformedInitSegment(Swift.String)
	case missingInitMoov(Swift.String)
	case missingAudioSampleEntry(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCodecFlacError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case short
	case invalidMarker
	case missingStreamInfo
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCodecMp3Error: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case headerTooShort
	case missingSync
	case reservedVersion
	case notLayer3
	case reservedSampleRate
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCodecOpusError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case headTooShort
	case invalidSignature
	case unsupportedVersion(Swift.UInt8)
	case unsupportedChannelCount(Swift.UInt32)
	case mappingTableTooShort
	case invalidMappingTable
	case unsupportedMappingFamily(Swift.UInt8)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCodecVp8Error: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case frameTooShort
	case keyframeHeaderTooShort
	case startCodeMismatch
	case emptyFrame
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxCodecVp9Error: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case invalidFrameMarker
	case invalidSyncCode
	case truncated
	case emptyFrame
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxContainerFmp4Chunk: Sendable, CustomDebugStringConvertible {
	case init_(Bytes)
	case fragment(MuxContainerFmp4Fragment)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func data() -> Bytes
	@inlinable public func intoData() -> Bytes
	@inlinable public func init_() -> Bytes?
	@inlinable public func fragment() -> MuxContainerFmp4Fragment?
}
public enum MuxContainerTsFraming: Sendable, CustomDebugStringConvertible, Equatable {
	case section
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsFraming, b: MuxContainerTsFraming) -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsFraming
}
public enum MuxContainerTsStep: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case rewind(by: RustDuration, flagged: Swift.Bool)
	case leap(by: RustDuration)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsStep, b: MuxContainerTsStep) -> Swift.Bool
}
public enum MuxCodecAacAdtsError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case truncated
	case syncword
	case sampleRate
	case frameLength
	case packetLength
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCodecAacAdtsError, b: MuxCodecAacAdtsError) -> Swift.Bool
}
public enum MuxContainerFmp4FragmentMissingDuration: Sendable, CustomDebugStringConvertible {
	case reject
	case inferFromPresentationTime
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> MuxContainerFmp4FragmentMissingDuration
}
public enum MuxContainerTsStatsClass: Sendable, CustomDebugStringConvertible, Equatable {
	case audio
	case video
	case data
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsStatsClass, b: MuxContainerTsStatsClass) -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsClass
}
public enum MoqTokioQuicCongestionControl: Sendable, CustomDebugStringConvertible, Equatable {
	case loss
	case delay
	case none
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioQuicCongestionControl, b: MoqTokioQuicCongestionControl) -> Swift.Bool
}
public enum MoqTokioConnectionPhase: Sendable, CustomDebugStringConvertible, Equatable {
	case connecting
	case connected
	case disconnected
	case migrating
	case closed
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioConnectionPhase, b: MoqTokioConnectionPhase) -> Swift.Bool
}
public enum MoqTokioAcceptFailure: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case connection
	case exhausted
	case unknown
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioAcceptFailure, b: MoqTokioAcceptFailure) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var ALL: [MoqTokioAcceptFailure] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum MoqTokioRedirect: Sendable, CustomDebugStringConvertible, Equatable {
	case follow
	case sameHost
	case ignore
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioRedirect, b: MoqTokioRedirect) -> Swift.Bool
	@inlinable public static func `default`() -> MoqTokioRedirect
}
public enum MoqQuicRate: Sendable, CustomDebugStringConvertible, Equatable {
	case unlimited
	case bytesPerSecond(NonZero<Swift.UInt64>)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicRate, b: MoqQuicRate) -> Swift.Bool
}
public enum Mp4AtomColr: Sendable, CustomDebugStringConvertible, Equatable {
	case nclx(colourPrimaries: Swift.UInt16, transferCharacteristics: Swift.UInt16, matrixCoefficients: Swift.UInt16, fullRangeFlag: Swift.Bool)
	case nclc(colourPrimaries: Swift.UInt16, transferCharacteristics: Swift.UInt16, matrixCoefficients: Swift.UInt16)
	case ricc(profile: Bytes)
	case prof(profile: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomColr, b: Mp4AtomColr) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomColr
	@inlinable public static func new(_ colourPrimaries: Swift.UInt16, _ transferCharacteristics: Swift.UInt16, _ matrixCoefficients: Swift.UInt16, _ fullRangeFlag: Swift.Bool) throws -> Mp4AtomColr
}
public enum Mp4AtomUncC: Sendable, CustomDebugStringConvertible, Equatable {
	case v1(profile: Mp4AtomFourCc)
	case v0(profile: Mp4AtomFourCc, components: [Mp4AtomUncompressedComponent], samplingType: Swift.UInt8, interleaveType: Swift.UInt8, blockSize: Swift.UInt8, componentsLittleEndian: Swift.Bool, blockPadLsb: Swift.Bool, blockLittleEndian: Swift.Bool, blockReversed: Swift.Bool, padUnknown: Swift.Bool, pixelSize: Swift.UInt32, rowAlignSize: Swift.UInt32, tileAlignSize: Swift.UInt32, numTileColsMinusOne: Swift.UInt32, numTileRowsMinusOne: Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUncC, b: Mp4AtomUncC) -> Swift.Bool
}
public enum Mp4AtomSencBoxVersion: Sendable, CustomDebugStringConvertible, Equatable {
	case v0
	case v1
	case v2
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSencBoxVersion, b: Mp4AtomSencBoxVersion) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSencBoxVersion
}
public enum MoqTokioConnectError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case emptyAddresses
	case unsupportedScheme(Swift.String)
	case missingHost
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioConnectError, b: MoqTokioConnectError) -> Swift.Bool
}
public enum RustlsProtocolVersion: Sendable, CustomDebugStringConvertible, Equatable {
	case ssLv2
	case ssLv3
	case tlSv10
	case tlSv11
	case tlSv12
	case tlSv13
	case dtlSv10
	case dtlSv12
	case dtlSv13
	case unknown(Swift.UInt16)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RustlsProtocolVersion, b: RustlsProtocolVersion) -> Swift.Bool
	@inlinable public func toArray() -> ArrayU8x2
	@inlinable public func asStr() -> Swift.String?
}
public enum RustlsClientTls12Resumption: Sendable, CustomDebugStringConvertible, Equatable {
	case disabled
	case sessionIdOnly
	case sessionIdOrTickets
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RustlsClientTls12Resumption, b: RustlsClientTls12Resumption) -> Swift.Bool
}
public enum MoqSockDemuxDiscard: Sendable, CustomDebugStringConvertible, Equatable {
	case stun
	case unknown
	case unclaimed
	case overflow
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqSockDemuxDiscard, b: MoqSockDemuxDiscard) -> Swift.Bool
	@inlinable public static var ALL: [MoqSockDemuxDiscard] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum Mp4AtomClockType: Sendable, CustomDebugStringConvertible, Equatable {
	case unknown
	case doesNotSync
	case canSync
	case reserved
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomClockType, b: Mp4AtomClockType) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomClockType
}
public enum Mp4AtomFlacMetadataBlock: Sendable, CustomDebugStringConvertible, Equatable {
	case streamInfo(minimumBlockSize: Swift.UInt16, maximumBlockSize: Swift.UInt16, minimumFrameSize: Mp4AtomU24, maximumFrameSize: Mp4AtomU24, sampleRate: Swift.UInt32, numChannelsMinusOne: Swift.UInt8, bitsPerSampleMinusOne: Swift.UInt8, numberOfInterchannelSamples: Swift.UInt64, md5Checksum: Bytes)
	case padding
	case application
	case seekTable
	case vorbisComment(vendorString: Swift.String, comments: [Swift.String])
	case cueSheet
	case picture
	case reserved
	case forbidden
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomFlacMetadataBlock, b: Mp4AtomFlacMetadataBlock) -> Swift.Bool
}
public enum Mp4AtomChannelStructure: Sendable, CustomDebugStringConvertible, Equatable {
	case explicitPositions(positions: [Mp4AtomSpeakerPosition])
	case definedLayout(layout: Swift.UInt8, omittedChannelsMap: Swift.UInt64?, channelOrderDefinition: Swift.UInt8?)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomChannelStructure, b: Mp4AtomChannelStructure) -> Swift.Bool
}
public enum Mp4AtomUuid: Sendable, CustomDebugStringConvertible, Equatable {
	case c2pa(Mp4AtomC2pa)
	case unknown(Mp4AtomExtendedType, Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomUuid, b: Mp4AtomUuid) -> Swift.Bool
}
public enum MoqSockUdpEcnCodepoint: Sendable, CustomDebugStringConvertible, Equatable {
	case ect0
	case ect1
	case ce
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqSockUdpEcnCodepoint, b: MoqSockUdpEcnCodepoint) -> Swift.Bool
	@inlinable public static func fromBits(_ x: Swift.UInt8) -> MoqSockUdpEcnCodepoint?
}
public enum MoqQuicSide: Sendable, CustomDebugStringConvertible, Hashable {
	case client
	case server
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicSide, b: MoqQuicSide) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func isClient() -> Swift.Bool
	@inlinable public func isServer() -> Swift.Bool
}
public enum MoqQuicStreamDir: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	case bi
	case uni
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicStreamDir, b: MoqQuicStreamDir) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum Mp4AtomSpeakerPosition: Sendable, CustomDebugStringConvertible, Equatable {
	case standard(Mp4AtomAudioChannelPosition)
	case explicit(Mp4AtomExplicitSpeakerPosition)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSpeakerPosition, b: Mp4AtomSpeakerPosition) -> Swift.Bool
}
public enum Mp4AtomDataType: Sendable, CustomDebugStringConvertible, Equatable {
	case wellKnown(Swift.UInt32)
	case custom(Swift.String)
	case unknown(Swift.UInt32, Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDataType, b: Mp4AtomDataType) -> Swift.Bool
}
public enum Mp4AtomStszSamples: Sendable, CustomDebugStringConvertible, Equatable {
	case identical(count: Swift.UInt32, size: Swift.UInt32)
	case different(sizes: [Swift.UInt32])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomStszSamples, b: Mp4AtomStszSamples) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStszSamples
}
public enum Mp4AtomDrefEntry: Sendable, CustomDebugStringConvertible, Equatable {
	case url(Mp4AtomUrl)
	case urn(Mp4AtomUrn)
	case unknown(Mp4AtomFourCc, Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomDrefEntry, b: Mp4AtomDrefEntry) -> Swift.Bool
	@inlinable public func kind() -> Mp4AtomFourCc
}
public enum Mp4AtomEmsgTimestamp: Sendable, CustomDebugStringConvertible, Equatable {
	case relative(Swift.UInt32)
	case absolute(Swift.UInt64)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomEmsgTimestamp, b: Mp4AtomEmsgTimestamp) -> Swift.Bool
}
public enum Mp4AtomTrunSamples: Sendable, CustomDebugStringConvertible, Equatable {
	case inherited(count: Swift.UInt32, firstFlags: Swift.UInt32?)
	case explicit([Mp4AtomTrunEntry])
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomTrunSamples, b: Mp4AtomTrunSamples) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTrunSamples
	@inlinable public func len() -> Swift.UInt
	@inlinable public func isEmpty() -> Swift.Bool
	@inlinable public func get(_ index: Swift.UInt) -> Mp4AtomTrunEntry?
}
public enum Mp4AtomReferenceTime: Sendable, CustomDebugStringConvertible, Equatable {
	case input
	case output
	case finalised
	case written
	case consistent
	case realTime
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomReferenceTime, b: Mp4AtomReferenceTime) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomReferenceTime
}
public enum Mp4AtomProjKind: Sendable, CustomDebugStringConvertible, Equatable {
	case equi(Mp4AtomEqui)
	case cbmp(Mp4AtomCbmp)
	case mshp(Mp4AtomMshp)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomProjKind, b: Mp4AtomProjKind) -> Swift.Bool
}
public enum Mp4AtomVexuProjKind: Sendable, CustomDebugStringConvertible, Equatable {
	case rectilinear
	case equirectangular
	case halfEquirectangular
	case fisheye
	case parametricImmersive
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomVexuProjKind, b: Mp4AtomVexuProjKind) -> Swift.Bool
	@inlinable public func kind() -> Mp4AtomFourCc
}
public enum Mp4AtomAudioChannelPosition: Sendable, CustomDebugStringConvertible, Equatable {
	case frontLeft
	case frontRight
	case frontCenter
	case lfe1
	case leftSurround
	case rightSurround
	case frontLeftOfCenter
	case frontRightOfCenter
	case rearLeft
	case rearRight
	case rearCenter
	case surroundLeft
	case surroundRight
	case sideLeft
	case sideRight
	case frontLeftWide
	case frontRightWide
	case topFrontLeft
	case topFrontRight
	case topFrontCenter
	case topRearLeft
	case topRearRight
	case topRearCenter
	case topSideLeft
	case topSideRight
	case topCenter
	case lfe2
	case bottomFrontLeft
	case bottomFrontRight
	case bottomFrontCenter
	case topSurroundLeft
	case topSurroundRight
	case lfe3
	case leos
	case reos
	case hwbcal
	case hwbcar
	case lbs
	case rbs
	case leftEar
	case rightEar
	case unknown
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAudioChannelPosition, b: Mp4AtomAudioChannelPosition) -> Swift.Bool
	@inlinable public static func fromU8(_ value: Swift.UInt8) -> Mp4AtomAudioChannelPosition?
}
public enum Mp4AtomIlstDataValue: Sendable, CustomDebugStringConvertible, Equatable {
	case reserved(Bytes)
	case utf8(Swift.String)
	case utf16(Swift.String)
	case jpeg(Bytes)
	case png(Bytes)
	case bmp(Bytes)
	case beSignedInt(Swift.Int64)
	case beUnsignedInt(Swift.UInt64)
	case beFloat32(Swift.Float)
	case beFloat64(Swift.Double)
	case unknown(Swift.UInt32, Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomIlstDataValue, b: Mp4AtomIlstDataValue) -> Swift.Bool
}
public enum Mp4AtomAnySampleGroupEntry: Sendable, CustomDebugStringConvertible, Equatable {
	case directReferenceSampleList(Swift.UInt32, [Swift.UInt32])
	case unknownGroupingType(Mp4AtomFourCc, Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomAnySampleGroupEntry, b: Mp4AtomAnySampleGroupEntry) -> Swift.Bool
}
public enum Mp4AtomSubsSubsampleSize: Sendable, CustomDebugStringConvertible, Equatable {
	case u16(Swift.UInt16)
	case u32(Swift.UInt32)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Mp4AtomSubsSubsampleSize, b: Mp4AtomSubsSubsampleSize) -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSubsSubsampleSize
	@inlinable public func value() -> Swift.UInt32
}
public enum Level: Sendable, CustomStringConvertible, Hashable {
	case trace
	case debug
	case info
	case warn
	case error
	@inlinable public var description: Swift.String { get }
	@inlinable public static func == (a: Level, b: Level) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct Park: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Park
	@inlinable public static func `default`() -> Park
	@inlinable public static func new(_ waiter: consuming Waiter) throws -> Park
	@inlinable public mutating func hold(_ cx: @escaping @Sendable () -> Void) throws -> Waiter
}
public struct Waiter: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Waiter
	@inlinable public static func new(_ waker: @escaping @Sendable () -> Void) -> Waiter
	@inlinable public static func noop() -> Waiter
	@inlinable public func register(_ list: inout WaiterList) throws
	@inlinable public func waker() throws -> @Sendable () -> Void
	@inlinable public func context() throws -> @Sendable () -> Void
	@inlinable public func batch(_ poll: @Sendable () -> Swift.String) throws -> Swift.String
}
public struct WaiterList: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> WaiterList
	@inlinable public static func new() -> WaiterList
	@inlinable public mutating func register(_ waiter: borrowing Waiter) throws
	@inlinable public mutating func take() throws -> WaiterList
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public mutating func wake() throws
}
public final class DecodeError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> DecodeError
}
public struct Driver: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public mutating func poll(_ now: ClockInstant, _ waiter: borrowing Waiter) throws -> ClockInstant?
}
public struct Instance: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Instance
	@inlinable public static func == (a: borrowing Instance, b: borrowing Instance) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func random() -> Instance
	@inlinable public func asStr() throws -> Swift.String
	@inlinable public func isMinted() throws -> Swift.Bool
}
public struct Path: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Path
	@inlinable public static func == (a: borrowing Path, b: borrowing Path) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> Path
	@inlinable public static func new(_ s: Swift.String) throws -> Path
	@inlinable public static func fromStatic(_ s: Swift.String) -> Path
	@inlinable public func hasPrefix(_ prefix: borrowing Path) throws -> Swift.Bool
	@inlinable public func stripPrefix(_ prefix: borrowing Path) throws -> Path?
	@inlinable public func head(_ n: Swift.UInt) throws -> Path?
	@inlinable public func parts() throws -> [Swift.String]
	@inlinable public func asStr() throws -> Swift.String
	@inlinable public static func empty() -> Path
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func len() throws -> Swift.UInt
	@inlinable public func join(_ other: borrowing Path) throws -> Path
	@inlinable public func resolve(_ rel: borrowing PathRelative) throws -> Path
	@inlinable public func tryResolve(_ rel: borrowing PathRelative) throws -> Path?
	@inlinable public func relative(_ base: borrowing Path) throws -> PathRelative?
}
public struct Pattern: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Pattern
	@inlinable public static func == (a: borrowing Pattern, b: borrowing Pattern) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> Pattern
	@inlinable public static func new(_ segments: [Segment]) throws -> Pattern
	@inlinable public static func all() -> Pattern
	@inlinable public static func literal(_ path: Swift.String) throws -> Pattern
	@inlinable public static func subtree(_ path: Swift.String) throws -> Pattern
	@inlinable public func asStr() throws -> Swift.String
	@inlinable public func segments() throws -> [Segment]
	@inlinable public func head() throws -> Swift.String
	@inlinable public func isLiteral() throws -> Swift.Bool
	@inlinable public func asPrefix() throws -> Swift.String?
	@inlinable public func hasGlobstar() throws -> Swift.Bool
	@inlinable public func matches(_ path: Swift.String) throws -> Swift.Bool
	@inlinable public func contains(_ other: borrowing Pattern) throws -> Swift.Bool
	@inlinable public func overlaps(_ other: borrowing Pattern) throws -> Swift.Bool
	@inlinable public func specificity() throws -> Specificity
	@inlinable public func rebase(_ root: Swift.String) throws -> Patterns
	@inlinable public func intersect(_ other: borrowing Pattern) throws -> Patterns
	@inlinable public func captures(_ matched: borrowing Pattern) throws -> UniqueArray<Pattern>?
	@inlinable public func rooted(_ root: Swift.String) throws -> Pattern
}
public struct Patterns: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Patterns
	@inlinable public static func == (a: borrowing Patterns, b: borrowing Patterns) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> Patterns
	@inlinable public static func new() -> Patterns
	@inlinable public mutating func insert(_ pattern: consuming Pattern) throws -> Swift.Bool
	@inlinable public func matches(_ path: Swift.String) throws -> Swift.Bool
	@inlinable public func contains(_ pattern: borrowing Pattern) throws -> Swift.Bool
	@inlinable public func covers(_ other: borrowing Patterns) throws -> Swift.Bool
	@inlinable public func overlaps(_ pattern: borrowing Pattern) throws -> Swift.Bool
	@inlinable public func intersect(_ other: borrowing Patterns) throws -> Patterns
	@inlinable public func rebase(_ root: Swift.String) throws -> Patterns
	@inlinable public func rooted(_ root: Swift.String) throws -> Patterns
	@inlinable public func iter() throws -> UniqueArray<Pattern>
	@inlinable public func len() throws -> Swift.UInt
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct Server: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Server
	@inlinable public static func `default`() -> Server
	@inlinable public static func new() -> Server
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> Server
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> Server
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> Server
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> Server
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> Server
	@inlinable public consuming func withVersions(_ versions: consuming Versions) throws -> Server
	@inlinable public consuming func withExtensions(_ extensions: SetupExtensions) throws -> Server
	@inlinable public consuming func withImplementation(_ implementation: Swift.String) throws -> Server
}
public struct Session: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Session
	@inlinable public func version() throws -> Version
	@inlinable public func sendBandwidth() throws -> BandwidthConsumer?
	@inlinable public func allocator() throws -> BandwidthAllocator
	@inlinable public func recvBandwidth() throws -> BandwidthConsumer?
	@inlinable public func viewers() throws -> BandwidthViewers
	@inlinable public func stats() throws -> SessionStats
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func drain() throws -> GoawayProducer
	@inlinable public func draining() throws -> GoawayConsumer
	@inlinable public func auth() throws -> AuthHandle
}
public struct Client: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> Client
	@inlinable public static func `default`() -> Client
	@inlinable public static func new() -> Client
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> Client
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> Client
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> Client
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> Client
	@inlinable public consuming func withOrigin(_ origin: consuming OriginProducer) throws -> Client
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> Client
	@inlinable public consuming func withAllocator(_ allocator: consuming BandwidthAllocator) throws -> Client
	@inlinable public consuming func withViewers(_ viewers: consuming BandwidthViewers) throws -> Client
	@inlinable public consuming func withVersions(_ versions: consuming Versions) throws -> Client
	@inlinable public consuming func withUrl(_ url: consuming Url) throws -> Client
	@inlinable public consuming func withToken(_ token: SetupToken) throws -> Client
	@inlinable public consuming func withAuth(_ auth: consuming AuthHandle) throws -> Client
	@inlinable public consuming func withExtensions(_ extensions: SetupExtensions) throws -> Client
	@inlinable public consuming func withImplementation(_ implementation: Swift.String) throws -> Client
	@inlinable public consuming func withCost(_ cost: OriginCost) throws -> Client
	@inlinable public consuming func withPeerHop(_ hop: Hop) throws -> Client
}
public final class Error: @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Error
	@inlinable public func session() throws -> SessionError?
	@inlinable public func stream() throws -> StreamError?
	@inlinable public static func fromTransportError(_ err: Error) throws -> Error
	@inlinable public static func fromTransportMemoryError(_ err: TransportMemoryError) -> Error
}
public final class Version: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Version
	@inlinable public static func == (a: Version, b: Version) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func hasSetupToken() -> Swift.Bool
	@inlinable public func asStr() -> Swift.String
	@inlinable public static func fromCode(_ code: Swift.UInt64) -> Version?
	@inlinable public func code() -> Swift.UInt64
	@inlinable public func alpn() -> Swift.String
	@inlinable public func isLite() -> Swift.Bool
}
public struct Versions: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Versions
	@inlinable public static func `default`() -> Versions
	@inlinable public static func all() -> Versions
	@inlinable public func alpns() throws -> [Swift.String]
	@inlinable public func filter(_ other: borrowing Versions) throws -> Versions?
	@inlinable public func select(_ version: Version) throws -> Version?
	@inlinable public func contains(_ version: Version) throws -> Swift.Bool
	@inlinable public func iter() throws -> [Version]
}
public struct TimedJson: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TimedJson
	@inlinable public func valueGet() throws -> Json
	@inlinable public mutating func valueSet(_ value: consuming Json) throws
	@inlinable public var atGet: Timestamp? { get }
	@inlinable public mutating func atSet(_ value: Timestamp?) throws
	@inlinable public consuming func at(_ at: Timestamp) throws -> TimedJson
}
public struct Hops: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Hops
	@inlinable public static func == (a: borrowing Hops, b: borrowing Hops) throws -> Swift.Bool
	@inlinable public static func `default`() -> Hops
	@inlinable public static func new() -> Hops
	@inlinable public mutating func push(_ hop: Hop) throws
	@inlinable public func contains(_ hop: Hop) throws -> Swift.Bool
	@inlinable public func len() throws -> Swift.UInt
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func iter() throws -> [Hop]
	@inlinable public func asSlice() throws -> [Hop]
}
public final class Specificity: @unchecked Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Specificity
	@inlinable public static func == (a: Specificity, b: Specificity) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct ClockClock: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> ClockClock
	@inlinable public static func == (a: borrowing ClockClock, b: borrowing ClockClock) -> Swift.Bool
	@inlinable public static func current() -> ClockClock
	@inlinable public func now() -> ClockInstant
	@inlinable public func wall() -> ClockWall
}
#if compiler(>=6.4)
extension ClockClock: Equatable {}
#endif
public struct ClockDeadline: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func new(_ deadlines: borrowing ClockDeadlines) -> ClockDeadline
	@inlinable public static func after(_ deadlines: borrowing ClockDeadlines, _ duration: RustDuration) -> ClockDeadline
	@inlinable public mutating func set(_ at: ClockInstant?) throws
	@inlinable public func deadline() throws -> ClockInstant?
}
public struct ClockDeadlines: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> ClockDeadlines
	@inlinable public static func new(_ now: ClockInstant) -> ClockDeadlines
	@inlinable public func advance(_ now: ClockInstant)
	@inlinable public func park(_ waiter: borrowing Waiter) throws -> ClockInstant?
	@inlinable public func next() -> ClockInstant?
	@inlinable public func now() -> ClockInstant
	@inlinable public func enter() -> ClockEntered
}
public struct ClockEntered: ~Copyable {
	@inlinable public var debugDescription: Swift.String { get }
}
public struct ClockManual: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ClockManual
	@inlinable public static func `default`() -> ClockManual
	@inlinable public static var WALLSTART: ClockWall { get }
	@inlinable public static func new() -> ClockManual
	@inlinable public func now() throws -> ClockInstant
	@inlinable public func wall() throws -> ClockWall
	@inlinable public mutating func advance(_ duration: RustDuration) throws
	@inlinable public mutating func stepWall(_ wall: ClockWall) throws
}
public struct ClockRng: ~Copyable {}
public struct ClockInterval: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func setMissedTickBehavior(_ missed: ClockMissedTickBehavior) throws
	@inlinable public func period() throws -> RustDuration
}
public final class ClockWallMap: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ClockWallMap
	@inlinable public static func `default`() -> ClockWallMap
	@inlinable public static var TOLERANCE: RustDuration { get }
	@inlinable public func sample(_ now: ClockInstant, _ wall: ClockWall) throws
	@inlinable public func map(_ at: ClockWall) throws -> ClockInstant?
}
public struct CoopBudget: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> CoopBudget
	@inlinable public static func new(_ passes: Swift.UInt32) -> CoopBudget
}
public struct HangCatalog: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalog
	@inlinable public static func == (a: borrowing HangCatalog, b: borrowing HangCatalog) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalog
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public static var DEFAULTNAME: Swift.String { get }
	@inlinable public static func defaultTrackInfo() -> TrackInfo
	@inlinable public static func defaultSubscription() -> TrackSubscription
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalog
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct HangCatalogExtra: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogExtra
	@inlinable public static func == (a: borrowing HangCatalogExtra, b: borrowing HangCatalogExtra) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogExtra
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public func extGet() throws -> MuxCatalogHangExtra
	@inlinable public mutating func extSet(_ value: consuming MuxCatalogHangExtra) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalogExtra
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalogExtra
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct HangCatalogExt: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogExt
	@inlinable public static func == (a: borrowing HangCatalogExt, b: borrowing HangCatalogExt) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogExt
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public func extGet() throws -> MuxContainerTsExt
	@inlinable public mutating func extSet(_ value: consuming MuxContainerTsExt) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalogExt
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalogExt
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public final class HangError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> HangError
}
public struct MuxClock: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxClock
	@inlinable public static func == (a: borrowing MuxClock, b: borrowing MuxClock) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxClock
	@inlinable public static var TIMESCALE: Timescale { get }
	@inlinable public static func new() -> MuxClock
	@inlinable public static func at(_ epoch: ClockInstant, _ wall: ClockWall) throws -> MuxClock
	@inlinable public func now() throws -> Timestamp
	@inlinable public func capture(_ at: ClockInstant) throws -> Timestamp
	@inlinable public func timescale() throws -> Timescale
	@inlinable public func wall() throws -> HangCatalogClock?
	@inlinable public func wallClock(_ pts: Timestamp) throws -> ClockWall
}
public struct MuxLingerUnit: ~Copyable, @unchecked Sendable {
	@inlinable public static var DEFAULT: RustDuration { get }
	@inlinable public static func new(_ broadcasts: consuming UniqueArray<BroadcastProducer>, _ duration: RustDuration) throws -> MuxLingerUnit
}
public struct MuxPacer: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxPacer
	@inlinable public consuming func withLead(_ lead: RustDuration) throws -> MuxPacer
	@inlinable public consuming func withDelay(_ delay: RustDuration) throws -> MuxPacer
	@inlinable public mutating func pace(_ ts: Timestamp, _ now: ClockInstant) throws -> ClockInstant
	@inlinable public func due(_ ts: Timestamp) throws -> ClockInstant?
	@inlinable public func slack() throws -> RustDuration
	@inlinable public mutating func hurry(_ ts: Timestamp, _ now: ClockInstant) throws -> ClockInstant
}
public struct MuxBinding: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastConsumer) throws -> MuxBinding
}
public struct MuxSource: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxSource
	@inlinable public static func new(_ origin: consuming OriginConsumer, _ broadcast: consuming BroadcastConsumer) throws -> MuxSource
	@inlinable public func stats() throws -> UniqueArray<TupleStringHangEchoRendition>
	@inlinable public func broadcast() throws -> BroadcastConsumer
	@inlinable public func resolveReference() throws -> Path?
	@inlinable public func resolveReference(rel: borrowing PathRelative) throws -> Path?
	@inlinable public func bind() throws -> MuxBinding
	@inlinable public func bind(rel: borrowing PathRelative) throws -> MuxBinding
	@inlinable public func bind(instance: borrowing Instance) throws -> MuxBinding
	@inlinable public func bind(rel: borrowing PathRelative, instance: borrowing Instance) throws -> MuxBinding
	@inlinable public func follow() throws -> MuxFollowInstances
	@inlinable public func follow(rel: borrowing PathRelative) throws -> MuxFollowInstances
	@inlinable public func follow(instance: borrowing Instance) throws -> MuxFollowInstances
	@inlinable public func follow(rel: borrowing PathRelative, instance: borrowing Instance) throws -> MuxFollowInstances
}
public final class MuxError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxError
}
public final class TokioDuration: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TokioDuration
	@inlinable public static func == (a: TokioDuration, b: TokioDuration) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> TokioDuration
}
public struct TokioBackoff: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioBackoff
	@inlinable public static func `default`() -> TokioBackoff
	@inlinable public var initialGet: TokioDuration? { get }
	@inlinable public mutating func initialSet(_ value: TokioDuration?) throws
	@inlinable public var multiplierGet: Swift.UInt32 { get }
	@inlinable public mutating func multiplierSet(_ value: Swift.UInt32) throws
	@inlinable public var maxGet: TokioDuration? { get }
	@inlinable public mutating func maxSet(_ value: TokioDuration?) throws
	@inlinable public var healthyGet: TokioDuration? { get }
	@inlinable public mutating func healthySet(_ value: TokioDuration?) throws
	@inlinable public var timeoutGet: TokioDuration? { get }
	@inlinable public mutating func timeoutSet(_ value: TokioDuration?) throws
	@inlinable public func initial() throws -> RustDuration
	@inlinable public func max() throws -> RustDuration
	@inlinable public func timeout() throws -> RustDuration
}
public struct TokioClient: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TokioClient
	@inlinable public static func new(_ config: consuming TokioClientConfig) throws -> TokioClient
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> TokioClient
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> TokioClient
	@inlinable public consuming func withOrigin(_ origin: consuming OriginProducer) throws -> TokioClient
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> TokioClient
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> TokioClient
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> TokioClient
	@inlinable public consuming func withCost(_ cost: Swift.UInt64) throws -> TokioClient
	@inlinable public consuming func withPeerHop(_ hop: Hop) throws -> TokioClient
	@inlinable public consuming func withReconnect(_ reconnect: Swift.Bool) throws -> TokioClient
	@inlinable public consuming func withRestart(_ restart: TokioRestart) throws -> TokioClient
	@inlinable public func connect(_ addrs: consuming MoqTokioAddrs) throws -> TokioConnection
	@inlinable public consuming func publish(_ origin: consuming OriginConsumer) throws -> TokioConnection?
	@inlinable public consuming func consume(_ origin: consuming OriginProducer) throws -> TokioConnection?
}
public struct TokioConnection: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TokioConnection
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func connected() throws -> Swift.Bool
	@inlinable public func connects() throws -> Swift.UInt64
	@inlinable public func version() throws -> Version?
	@inlinable public func transport() throws -> TokioTransport?
	@inlinable public func draining() throws -> GoawayConsumer?
	@inlinable public func sendBandwidth() throws -> BandwidthConsumer
	@inlinable public func allocator() throws -> BandwidthAllocator
	@inlinable public func recvBandwidth() throws -> BandwidthConsumer
	@inlinable public func viewers() throws -> BandwidthViewers
	@inlinable public func auth() throws -> AuthHandle
	@inlinable public func tokens() throws -> AuthTokens
	@inlinable public func monitor() throws -> TokioConnectionMonitor
}
public final class TokioError: @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioError
	@inlinable public func connectError() throws -> TokioConnectError?
	@inlinable public func isAuth() throws -> Swift.Bool
	@inlinable public func status() throws -> Swift.UInt16?
}
public struct TokioLog: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioLog
	@inlinable public static func `default`() -> TokioLog
	@inlinable public var levelGet: Level { get }
	@inlinable public mutating func levelSet(_ value: Level) throws
	@inlinable public static func new(_ level: Level) -> TokioLog
	@inlinable public func level() throws -> Level?
	@inlinable public func init_() throws
}
public struct TokioServer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ config: consuming TokioServerConfig) throws -> TokioServer
	@inlinable public func tlsAcceptor() throws -> MoqTokioTcpTlsAcceptor?
	@inlinable public func certificates() throws -> TokioTlsCertificates
	@inlinable public func acceptHealth() throws -> UniqueArray<MoqTokioAcceptHealth>
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func websocketLocalAddr() throws -> SocketAddr?
}
public struct AuthWatch: ~Copyable, @unchecked Sendable {
	@inlinable public func peek() throws -> AuthGrant?
}
public struct AuthRequests: ~Copyable, @unchecked Sendable {}
public struct AuthRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func token() throws -> Bytes
	@inlinable public func kind() throws -> AuthRequestKind?
	@inlinable public func path() throws -> Path?
	@inlinable public consuming func accept(_ grant: consuming AuthGrant) throws -> AuthIssued
	@inlinable public consuming func reject(_ code: SessionError, _ reason: Swift.String) throws
}
public struct AuthIssued: ~Copyable, @unchecked Sendable {
	@inlinable public func update(_ grant: consuming AuthGrant) throws
	@inlinable public consuming func revoke(_ code: SessionError, _ reason: Swift.String) throws
}
public struct AuthGrant: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> AuthGrant
	@inlinable public static func == (a: borrowing AuthGrant, b: borrowing AuthGrant) throws -> Swift.Bool
	@inlinable public static func `default`() -> AuthGrant
	@inlinable public func publishGet() throws -> Patterns
	@inlinable public mutating func publishSet(_ value: consuming Patterns) throws
	@inlinable public func subscribeGet() throws -> Patterns
	@inlinable public mutating func subscribeSet(_ value: consuming Patterns) throws
	@inlinable public var expiresGet: ClockInstant? { get }
	@inlinable public mutating func expiresSet(_ value: ClockInstant?) throws
	@inlinable public static func all() -> AuthGrant
}
public struct AuthHandle: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> AuthHandle
	@inlinable public static func `default`() -> AuthHandle
	@inlinable public static func new() -> AuthHandle
	@inlinable public func grant() throws -> AuthWatch
	@inlinable public func authorize(_ grant: borrowing AuthGrant) throws
	@inlinable public func abort(_ err: Error) throws
}
public struct AuthCredential: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> AuthCredential
	@inlinable public static func == (a: borrowing AuthCredential, b: borrowing AuthCredential) throws -> Swift.Bool
	@inlinable public func grant() throws -> AuthWatch
}
public struct AuthDial: ~Copyable, @unchecked Sendable {
	@inlinable public func urlGet() throws -> Url
	@inlinable public mutating func urlSet(_ value: consuming Url) throws
	@inlinable public var tokenGet: SetupToken? { get }
	@inlinable public mutating func tokenSet(_ value: SetupToken?) throws
}
public struct AuthToken: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> AuthToken
	@inlinable public static func == (a: borrowing AuthToken, b: borrowing AuthToken) throws -> Swift.Bool
	@inlinable public func asStr() throws -> Swift.String
}
public struct AuthTokens: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> AuthTokens
	@inlinable public static func new(_ tokens: consuming UniqueArray<AuthToken>) throws -> AuthTokens
	@inlinable public func handle_() -> AuthHandle
	@inlinable public func set(_ tokens: consuming UniqueArray<AuthToken>) throws
	@inlinable public func dial(_ url: borrowing Url, _ versions: borrowing Versions) throws -> AuthDial
	@inlinable public func connected(_ session: borrowing Session) throws
	@inlinable public func changed(_ dial: borrowing AuthDial) throws -> Swift.Bool
}
public struct GoawayProducer: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func send(_ goaway: consuming GoawayGoaway) throws
}
public struct GoawayGoaway: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> GoawayGoaway
	@inlinable public static func == (a: borrowing GoawayGoaway, b: borrowing GoawayGoaway) throws -> Swift.Bool
	@inlinable public static func `default`() -> GoawayGoaway
	@inlinable public static func new() -> GoawayGoaway
	@inlinable public static func redirect(_ uri: Swift.String) -> GoawayGoaway
	@inlinable public consuming func withTimeout(_ timeout: RustDuration) throws -> GoawayGoaway
	@inlinable public func uri() throws -> Swift.String
	@inlinable public func timeout() throws -> RustDuration?
}
public struct GoawayConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> GoawayConsumer
}
public struct PathRelative: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> PathRelative
	@inlinable public static func == (a: borrowing PathRelative, b: borrowing PathRelative) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> PathRelative
	@inlinable public static func new(_ s: Swift.String) throws -> PathRelative
	@inlinable public func asStr() throws -> Swift.String
	@inlinable public static func empty() -> PathRelative
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func len() throws -> Swift.UInt
}
public struct ServerHandshake: ~Copyable, @unchecked Sendable {
	@inlinable public func path() throws -> Swift.String
	@inlinable public func role() throws -> SetupRole?
	@inlinable public func peerHop() throws -> Hop?
	@inlinable public func peerImplementation() throws -> Swift.String?
	@inlinable public func auth() throws -> AuthHandle
	@inlinable public mutating func requests() throws -> AuthRequests
	@inlinable public func token() throws -> SetupToken?
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> ServerHandshake
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> ServerHandshake
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> ServerHandshake
	@inlinable public consuming func withPeerHop(_ hop: Hop) throws -> ServerHandshake
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> ServerHandshake
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> ServerHandshake
	@inlinable public consuming func withDeadline(_ at: ClockInstant) throws -> ServerHandshake
	@inlinable public consuming func close(_ err: Error) throws
}
public struct ServerActivity: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ServerActivity
	@inlinable public static func `default`() -> ServerActivity
	@inlinable public func enter() throws -> ServerActive?
}
public struct ServerActive: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
}
public final class SessionStats: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> SessionStats
	@inlinable public static func == (a: SessionStats, b: SessionStats) throws -> Swift.Bool
	@inlinable public static func `default`() -> SessionStats
	@inlinable public var rttGet: RustDuration? { get }
	@inlinable public func rttSet(_ value: RustDuration?) throws
	@inlinable public var estimatedSendRateGet: BandwidthRate? { get }
	@inlinable public func estimatedSendRateSet(_ value: BandwidthRate?) throws
	@inlinable public var bytesSentGet: Swift.UInt64? { get }
	@inlinable public func bytesSentSet(_ value: Swift.UInt64?) throws
	@inlinable public var bytesReceivedGet: Swift.UInt64? { get }
	@inlinable public func bytesReceivedSet(_ value: Swift.UInt64?) throws
	@inlinable public var bytesLostGet: Swift.UInt64? { get }
	@inlinable public func bytesLostSet(_ value: Swift.UInt64?) throws
	@inlinable public var packetsSentGet: Swift.UInt64? { get }
	@inlinable public func packetsSentSet(_ value: Swift.UInt64?) throws
	@inlinable public var packetsReceivedGet: Swift.UInt64? { get }
	@inlinable public func packetsReceivedSet(_ value: Swift.UInt64?) throws
	@inlinable public var packetsLostGet: Swift.UInt64? { get }
	@inlinable public func packetsLostSet(_ value: Swift.UInt64?) throws
	@inlinable public var streamsEvictedGet: Swift.UInt64? { get }
	@inlinable public func streamsEvictedSet(_ value: Swift.UInt64?) throws
}
public struct StatsTier: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsTier
	@inlinable public static func == (a: borrowing StatsTier, b: borrowing StatsTier) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> StatsTier
	@inlinable public static func new(_ label: consuming Path) throws -> StatsTier
	@inlinable public func label() throws -> Path
	@inlinable public func asStr() throws -> Swift.String
}
public struct StatsSnapshot: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> StatsSnapshot
	@inlinable public static func == (a: borrowing StatsSnapshot, b: borrowing StatsSnapshot) -> Swift.Bool
	@inlinable public static func `default`() -> StatsSnapshot
	@inlinable public func control() -> UniqueArray<TupleStatsTierStatsMessageStatsTotals>
	@inlinable public func traffic() -> UniqueArray<TupleStatsTierStatsRoleStatsTraffic>
	@inlinable public func sessions() -> UniqueArray<TupleStatsTierStatsPresence>
}
#if compiler(>=6.4)
extension StatsSnapshot: Equatable {}
#endif
public struct StatsReport: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsReport
	@inlinable public static func `default`() -> StatsReport
	@inlinable public func trafficGet() throws -> UniqueArray<StatsTrafficEntry>
	@inlinable public mutating func trafficSet(_ value: consuming UniqueArray<StatsTrafficEntry>) throws
	@inlinable public func sessionsGet() throws -> UniqueArray<StatsSessionEntry>
	@inlinable public mutating func sessionsSet(_ value: consuming UniqueArray<StatsSessionEntry>) throws
}
public struct StatsTrafficEntry: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsTrafficEntry
	@inlinable public func pathGet() throws -> Path
	@inlinable public mutating func pathSet(_ value: consuming Path) throws
	@inlinable public func tierGet() throws -> StatsTier
	@inlinable public mutating func tierSet(_ value: consuming StatsTier) throws
	@inlinable public var publisherGet: StatsTraffic { get }
	@inlinable public mutating func publisherSet(_ value: StatsTraffic) throws
	@inlinable public var subscriberGet: StatsTraffic { get }
	@inlinable public mutating func subscriberSet(_ value: StatsTraffic) throws
	@inlinable public var closedGet: Swift.Bool { get }
	@inlinable public mutating func closedSet(_ value: Swift.Bool) throws
}
public struct StatsSessionEntry: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsSessionEntry
	@inlinable public func tierGet() throws -> StatsTier
	@inlinable public mutating func tierSet(_ value: consuming StatsTier) throws
	@inlinable public func labelGet() throws -> Path
	@inlinable public mutating func labelSet(_ value: consuming Path) throws
	@inlinable public var presenceGet: StatsPresence { get }
	@inlinable public mutating func presenceSet(_ value: StatsPresence) throws
	@inlinable public var closedGet: Swift.Bool { get }
	@inlinable public mutating func closedSet(_ value: Swift.Bool) throws
}
public struct StatsConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsConfig
	@inlinable public static func `default`() -> StatsConfig
	@inlinable public func excludeGet() throws -> Patterns
	@inlinable public mutating func excludeSet(_ value: consuming Patterns) throws
	@inlinable public static func new() -> StatsConfig
	@inlinable public consuming func withExclude(_ pattern: consuming Pattern) throws -> StatsConfig
}
public struct StatsRegistry: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> StatsRegistry
	@inlinable public static func `default`() -> StatsRegistry
	@inlinable public static func new(_ config: consuming StatsConfig) throws -> StatsRegistry
	@inlinable public static func disabled() -> StatsRegistry
	@inlinable public func isEnabled() -> Swift.Bool
	@inlinable public func tier(_ tier: consuming StatsTier) throws -> StatsHandle
	@inlinable public func snapshot() -> StatsSnapshot
	@inlinable public func report(_ report: inout StatsReport) throws
}
public struct StatsHandle: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> StatsHandle
	@inlinable public static func `default`() -> StatsHandle
	@inlinable public func refuse(_ label: consuming Path, _ reason: StatsRefusalKind) throws
	@inlinable public func session(_ label: consuming Path) throws -> StatsSession
	@inlinable public func request(_ label: consuming Path) throws -> StatsSession
}
public struct StatsWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> StatsWire
}
public struct StatsSession: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> StatsSession
	@inlinable public static func `default`() -> StatsSession
	@inlinable public func wire(_ counters: consuming TransportWire) throws -> StatsWire
	@inlinable public func retag(_ tier: consuming StatsTier, _ label: consuming Path) throws
}
public struct TransportWire: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TransportWire
	@inlinable public static func `default`() -> TransportWire
	@inlinable public static func new() -> TransportWire
	@inlinable public func sentBytes() throws -> Swift.UInt64
	@inlinable public func receivedBytes() throws -> Swift.UInt64
	@inlinable public func setSentBytes(_ total: Swift.UInt64) throws
	@inlinable public func setReceivedBytes(_ total: Swift.UInt64) throws
}
public struct TransportProbe: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TransportProbe
	@inlinable public static func `default`() -> TransportProbe
	@inlinable public static func new() -> TransportProbe
	@inlinable public func target() -> BandwidthRate
	@inlinable public func setTarget(_ rate: BandwidthRate)
}
public struct BandwidthDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BandwidthDemand
	@inlinable public func wanted() throws -> Swift.Bool
}
public struct BandwidthReservation: ~Copyable, @unchecked Sendable {
	@inlinable public func rate() -> BandwidthRate?
	@inlinable public func consume() -> BandwidthConsumer
	@inlinable public func update(_ max: BandwidthRate)
}
public struct BandwidthViewer: ~Copyable, @unchecked Sendable {
	@inlinable public func rate() -> BandwidthRate?
	@inlinable public func consume() -> BandwidthConsumer
	@inlinable public func update(_ max: BandwidthRate) throws
}
public struct BandwidthProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> BandwidthProducer
	@inlinable public static func `default`() -> BandwidthProducer
	@inlinable public static func new() -> BandwidthProducer
	@inlinable public func set(_ bitrate: BandwidthRate?) throws
	@inlinable public func consume() -> BandwidthConsumer
	@inlinable public func weak() -> BandwidthWeak
	@inlinable public func demand() -> BandwidthDemand
}
public struct BandwidthWeak: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BandwidthWeak
	@inlinable public func consume() throws -> BandwidthConsumer
}
public struct BandwidthAllocator: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BandwidthAllocator
	@inlinable public static func `default`() -> BandwidthAllocator
	@inlinable public static func new(_ estimate: consuming BandwidthConsumer) throws -> BandwidthAllocator
	@inlinable public func setTarget(_ target: BandwidthRate?) throws
	@inlinable public static func unlimited() -> BandwidthAllocator
	@inlinable public func reserve(_ track: borrowing TrackDemand, _ max: BandwidthRate) throws -> BandwidthReservation
}
public struct BandwidthViewers: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BandwidthViewers
	@inlinable public static func `default`() -> BandwidthViewers
	@inlinable public static var MAXTARGET: BandwidthRate { get }
	@inlinable public static func new(_ estimate: consuming BandwidthWeak) throws -> BandwidthViewers
	@inlinable public func watch(_ max: BandwidthRate) throws -> BandwidthViewer
	@inlinable public func setTarget(_ target: BandwidthRate?) throws
}
public struct BandwidthConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BandwidthConsumer
	@inlinable public func peek() throws -> BandwidthRate?
	@inlinable public func probed() throws -> Swift.Bool
}
public struct BroadcastDatagramRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastDatagramRequests
}
public struct BroadcastConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastConsumer
	@inlinable public func tracks() throws -> BroadcastTracks
	@inlinable public func info() throws -> BroadcastInfo
	@inlinable public func track(_ name: Swift.String) throws -> TrackConsumer
	@inlinable public func datagrams(_ name: Swift.String) throws -> DatagramConsumer
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func isClone(_ other: borrowing BroadcastConsumer) throws -> Swift.Bool
}
public struct BroadcastDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastDemand
	@inlinable public func wanted() throws -> Swift.Bool
}
public struct BroadcastInfo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BroadcastInfo
	@inlinable public static func `default`() -> BroadcastInfo
	@inlinable public func poolGet() throws -> CachePool
	@inlinable public mutating func poolSet(_ value: consuming CachePool) throws
	@inlinable public var cacheDurationGet: RustDuration { get }
	@inlinable public mutating func cacheDurationSet(_ value: RustDuration) throws
	@inlinable public func pathGet() throws -> Path
	@inlinable public mutating func pathSet(_ value: consuming Path) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public static func new() -> BroadcastInfo
	@inlinable public consuming func produce() throws -> BroadcastProducer
}
public struct BroadcastProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastProducer
	@inlinable public static func new(_ info: consuming BroadcastInfo) throws -> BroadcastProducer
	@inlinable public func info() throws -> BroadcastInfo
	@inlinable public func demand() throws -> BroadcastDemand
	@inlinable public func createTrack(_ name: Swift.String, _ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public func createDatagrams(_ name: Swift.String, _ info: DatagramInfo) throws -> DatagramProducer
	@inlinable public func reserveTrack(_ name: Swift.String) throws -> TrackRequest
	@inlinable public func uniqueTrack(_ suffix: Swift.String, _ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public func uniqueName(_ suffix: Swift.String) throws -> Swift.String
	@inlinable public func requests() throws -> BroadcastRequests
	@inlinable public func consume() throws -> BroadcastConsumer
	@inlinable public func close() throws
	@inlinable public func isClone(_ other: borrowing BroadcastProducer) throws -> Swift.Bool
}
public struct BroadcastRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastRequests
	@inlinable public func info() throws -> BroadcastInfo
	@inlinable public func consume() throws -> BroadcastConsumer
	@inlinable public func datagrams() throws -> BroadcastDatagramRequests
	@inlinable public func lister() throws -> BroadcastLister
	@inlinable public func isClone(_ other: borrowing BroadcastRequests) throws -> Swift.Bool
}
public struct BroadcastLister: ~Copyable, @unchecked Sendable {
	@inlinable public func add(_ name: Swift.String, _ info: consuming TrackInfo) throws
	@inlinable public func remove(_ name: Swift.String) throws
	@inlinable public func finish(_ name: Swift.String) throws
	@inlinable public func synced()
	@inlinable public func clear()
	@inlinable public func end()
	@inlinable public func abort(_ err: Error) throws
}
public struct BroadcastTracks: ~Copyable, @unchecked Sendable {
	@inlinable public func broadcast() throws -> BroadcastConsumer
}
public struct BroadcastTracksEvent: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> BroadcastTracksEvent
}
public struct CacheConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> CacheConfig
	@inlinable public static func `default`() -> CacheConfig
	@inlinable public consuming func withCapacity(_ capacity: Swift.UInt64?) throws -> CacheConfig
	@inlinable public consuming func withExpiry(_ expiry: RustDuration?) throws -> CacheConfig
}
public struct CachePool: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> CachePool
	@inlinable public static func `default`() -> CachePool
	@inlinable public static func new(_ config: consuming CacheConfig) throws -> CachePool
	@inlinable public static func unbounded() -> CachePool
	@inlinable public func capacity() throws -> Swift.UInt64?
	@inlinable public func used() throws -> Swift.UInt64
	@inlinable public func resize(_ capacity: Swift.UInt64?) throws
	@inlinable public func expiry() throws -> RustDuration?
	@inlinable public func poll(_ now: ClockInstant, _ waiter: borrowing Waiter) throws -> ClockInstant?
	@inlinable public func downgrade() throws -> CachePoolWeak
}
public struct CachePoolWeak: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> CachePoolWeak
	@inlinable public func upgrade() -> CachePool?
}
public struct DatagramProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> DatagramProducer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func info() throws -> DatagramInfo
	@inlinable public mutating func append(_ timestamp: Timestamp?, _ payload: Bytes) throws -> Swift.UInt64
	@inlinable public mutating func insert(_ datagram: Datagram) throws
	@inlinable public func consume() throws -> DatagramConsumer
	@inlinable public consuming func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func priority() throws -> Swift.Int8?
	@inlinable public func isClone(_ other: borrowing DatagramProducer) throws -> Swift.Bool
}
public struct DatagramRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func priority() throws -> Swift.Int8?
	@inlinable public consuming func accept(_ info: DatagramInfo) throws -> DatagramProducer
	@inlinable public consuming func reject(_ err: Error) throws
}
public struct DatagramConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> DatagramConsumer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func priority() throws -> Swift.Int8
	@inlinable public mutating func setPriority(_ priority: Swift.Int8) throws
	@inlinable public func isClone(_ other: borrowing DatagramConsumer) throws -> Swift.Bool
}
public final class DatagramInfo: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> DatagramInfo
	@inlinable public static func == (a: DatagramInfo, b: DatagramInfo) throws -> Swift.Bool
	@inlinable public static func `default`() -> DatagramInfo
	@inlinable public var timescaleGet: Timescale? { get }
	@inlinable public func timescaleSet(_ value: Timescale?) throws
	@inlinable public var priorityGet: Swift.Int8 { get }
	@inlinable public func prioritySet(_ value: Swift.Int8) throws
	@inlinable public func withTimescale(_ timescale: Timescale?) throws -> DatagramInfo
	@inlinable public func withPriority(_ priority: Swift.Int8) throws -> DatagramInfo
	@inlinable public func rescale(_ timestamp: Timestamp?) throws -> Timestamp?
	@inlinable public func produce(_ name: Swift.String) throws -> DatagramProducer
}
public struct FrameProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func group() throws -> GroupProducer
	@inlinable public func remaining() throws -> Swift.UInt
	@inlinable public mutating func write(_ chunk: Bytes) throws
	@inlinable public consuming func finish() throws -> GroupProducer
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public var sizeGet: Swift.UInt64 { get }
	@inlinable public var timestampGet: Timestamp? { get }
}
public final class FrameFrame: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> FrameFrame
	@inlinable public var timestampGet: Timestamp? { get }
	@inlinable public func timestampSet(_ value: Timestamp?) throws
	@inlinable public var payloadGet: Bytes { get }
	@inlinable public func payloadSet(_ value: Bytes) throws
	@inlinable public static func new(_ timestamp: Timestamp?, _ payload: Bytes) -> FrameFrame
	@inlinable public func arrived() throws -> ClockInstant
}
public struct FrameBuffer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> FrameBuffer
	@inlinable public static func new() -> FrameBuffer
	@inlinable public func capacity() throws -> Swift.UInt
	@inlinable public func len() throws -> Swift.UInt
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func filled() throws -> [FrameFrame]
	@inlinable public mutating func push(_ frame: FrameFrame) throws
	@inlinable public mutating func clear() throws
}
public struct FrameConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> FrameConsumer
	@inlinable public var sizeGet: Swift.UInt64 { get }
	@inlinable public var timestampGet: Timestamp? { get }
}
public struct GroupProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func timescale() throws -> Timescale?
	@inlinable public mutating func startAt(_ index: Swift.UInt64) throws
	@inlinable public mutating func writeFrame(_ timestamp: Timestamp?, _ data: Bytes) throws
	@inlinable public mutating func writeFrames8(_ frames: inout FrameBuffer) throws
	@inlinable public consuming func createFrame(_ frame: FrameInfo) throws -> FrameProducer
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func frameCount() throws -> Swift.UInt
	@inlinable public func consume() throws -> GroupConsumer
	@inlinable public func demand() throws -> GroupDemand
	@inlinable public var sequenceGet: Swift.UInt64 { get }
}
public struct GroupDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> GroupDemand
	@inlinable public func sequence() throws -> Swift.UInt64
	@inlinable public func wanted() throws -> Swift.Bool
}
public struct GroupConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> GroupConsumer
	@inlinable public func keepAlive() throws
	@inlinable public func timescale() throws -> Timescale?
	@inlinable public func arrived() throws -> ClockInstant
	@inlinable public func index() throws -> Swift.UInt64
	@inlinable public consuming func withRange(_ frames: BoundsU64) throws -> GroupConsumer
	@inlinable public mutating func skipTo(_ index: Swift.UInt64) throws
	@inlinable public func frameCount() throws -> Swift.UInt
	@inlinable public var sequenceGet: Swift.UInt64 { get }
}
public struct GroupFetch: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> GroupFetch
	@inlinable public static func `default`() -> GroupFetch
	@inlinable public var priorityGet: Swift.Int8 { get }
	@inlinable public mutating func prioritySet(_ value: Swift.Int8) throws
	@inlinable public var maxDelayGet: RustDuration? { get }
	@inlinable public mutating func maxDelaySet(_ value: RustDuration?) throws
	@inlinable public consuming func withPriority(_ priority: Swift.Int8) throws -> GroupFetch
	@inlinable public consuming func withMaxDelay(_ maxDelay: RustDuration) throws -> GroupFetch
}
public struct TrackRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TrackRequests
	@inlinable public func name() -> Swift.String
	@inlinable public func demand() -> TrackDemand
}
public struct TrackDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
	@inlinable public func wanted() throws -> Swift.UInt64?
}
public struct TrackSubscriber: ~Copyable, @unchecked Sendable {
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func control() throws -> TrackControl
	@inlinable public consuming func ordered() throws -> TrackOrdered
	@inlinable public func isClone(_ other: borrowing TrackSubscriber) throws -> Swift.Bool
	@inlinable public func floor() throws -> Swift.UInt64?
	@inlinable public func nextInRange(_ from: Swift.UInt64) throws -> Swift.UInt64?
	@inlinable public consuming func withRange(_ groups: BoundsU64) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription
	@inlinable public mutating func update(_ subscription: consuming TrackSubscription) throws
}
public struct TrackOrdered: ~Copyable, @unchecked Sendable {
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public consuming func withRange(_ groups: BoundsU64) throws -> TrackOrdered
	@inlinable public func control() throws -> TrackControl
	@inlinable public func subscription() throws -> TrackSubscription
	@inlinable public mutating func update(_ subscription: consuming TrackSubscription) throws
	@inlinable public func isClone(_ other: borrowing TrackOrdered) throws -> Swift.Bool
}
public struct TrackRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackConsumer
	@inlinable public func requests() throws -> TrackRequests
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public consuming func accept(_ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public consuming func reject(_ err: Error) throws
	@inlinable public func subscription() throws -> TrackSubscription?
}
public struct TrackInfo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TrackInfo
	@inlinable public static func == (a: borrowing TrackInfo, b: borrowing TrackInfo) throws -> Swift.Bool
	@inlinable public static func `default`() -> TrackInfo
	@inlinable public var timescaleGet: Timescale? { get }
	@inlinable public mutating func timescaleSet(_ value: Timescale?) throws
	@inlinable public var maxAgeGet: RustDuration? { get }
	@inlinable public mutating func maxAgeSet(_ value: RustDuration?) throws
	@inlinable public var priorityGet: Swift.Int8 { get }
	@inlinable public mutating func prioritySet(_ value: Swift.Int8) throws
	@inlinable public var compressGet: Swift.Bool { get }
	@inlinable public mutating func compressSet(_ value: Swift.Bool) throws
	@inlinable public var refreshGet: Swift.UInt32? { get }
	@inlinable public mutating func refreshSet(_ value: Swift.UInt32?) throws
	@inlinable public consuming func withTimescale(_ timescale: Timescale?) throws -> TrackInfo
	@inlinable public consuming func withMaxAge(_ maxAge: RustDuration?) throws -> TrackInfo
	@inlinable public consuming func withPriority(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public consuming func withCompress(_ compress: Swift.Bool) throws -> TrackInfo
	@inlinable public consuming func withRefresh(_ refresh: Swift.UInt32?) throws -> TrackInfo
	@inlinable public func rescale(_ timestamp: Timestamp?) throws -> Timestamp?
}
public final class TrackProducer: @unchecked Sendable {
	@inlinable public func clone() throws -> TrackProducer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func writeFrame(_ timestamp: Timestamp?, _ frame: Bytes) throws
	@inlinable public func finish() throws
	@inlinable public func finishAt(_ end: TrackPosition, _ error: Error?) throws
	@inlinable public func startAt(_ sequence: Swift.UInt64?) throws
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func consume() throws -> TrackConsumer
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct TrackConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TrackConsumer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
}
public struct TrackControl: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TrackControl
	@inlinable public func subscription() -> TrackSubscription
	@inlinable public func update(_ subscription: consuming TrackSubscription) throws
}
public struct TrackSubscription: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TrackSubscription
	@inlinable public static func == (a: borrowing TrackSubscription, b: borrowing TrackSubscription) throws -> Swift.Bool
	@inlinable public static func `default`() -> TrackSubscription
	@inlinable public var priorityGet: Swift.Int8 { get }
	@inlinable public mutating func prioritySet(_ value: Swift.Int8) throws
	@inlinable public var delayGet: TrackDelay { get }
	@inlinable public mutating func delaySet(_ value: TrackDelay) throws
	@inlinable public var rangesGet: [TrackRange] { get }
	@inlinable public mutating func rangesSet(_ value: [TrackRange]) throws
	@inlinable public var orderGet: TrackOrder { get }
	@inlinable public mutating func orderSet(_ value: TrackOrder) throws
	@inlinable public consuming func withPriority(_ priority: Swift.Int8) throws -> TrackSubscription
	@inlinable public consuming func withDelay(_ delay: TrackDelay) throws -> TrackSubscription
	@inlinable public consuming func withRanges(_ ranges: [TrackRange]) throws -> TrackSubscription
	@inlinable public consuming func withRange(_ groups: BoundsU64) throws -> TrackSubscription
	@inlinable public consuming func withOrder(_ order: TrackOrder) throws -> TrackSubscription
}
public struct TrackFill: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TrackFill
	@inlinable public func start() throws -> TrackPosition
	@inlinable public func frameStart(_ sequence: Swift.UInt64) throws -> Swift.UInt64
	@inlinable public func end() throws -> Swift.UInt64
	@inlinable public func time() throws -> Timestamp?
	@inlinable public func priority() throws -> Swift.Int8
	@inlinable public func maxDelay() throws -> RustDuration?
	@inlinable public func order() throws -> TrackOrder
	@inlinable public func accept(_ sequence: Swift.UInt64, _ info: consuming TrackInfo?) throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64) throws
	@inlinable public func refuseGroups(_ groups: RangeU64, _ err: Error) throws
	@inlinable public func isWanted(_ sequence: Swift.UInt64) throws -> Swift.Bool
	@inlinable public func wanted() throws -> Swift.UInt64?
}
public struct OriginFollowing: ~Copyable, @unchecked Sendable {}
public struct OriginConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginConfig
	@inlinable public static func `default`() -> OriginConfig
	@inlinable public var hopGet: Hop { get }
	@inlinable public mutating func hopSet(_ value: Hop) throws
	@inlinable public var maxHopsGet: OriginHopLimit { get }
	@inlinable public mutating func maxHopsSet(_ value: OriginHopLimit) throws
	@inlinable public func poolGet() throws -> CachePool
	@inlinable public mutating func poolSet(_ value: consuming CachePool) throws
	@inlinable public var cacheDurationGet: RustDuration { get }
	@inlinable public mutating func cacheDurationSet(_ value: RustDuration) throws
	@inlinable public var updateHoldGet: RustDuration { get }
	@inlinable public mutating func updateHoldSet(_ value: RustDuration) throws
	@inlinable public static func new(_ hop: Hop) -> OriginConfig
}
public struct OriginConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> OriginConsumer
	@inlinable public func hop() throws -> Hop
	@inlinable public consuming func local() throws -> OriginConsumer
	@inlinable public consuming func withHidden(_ hidden: Swift.Bool) throws -> OriginConsumer
	@inlinable public consuming func covering() throws -> OriginConsumer
	@inlinable public func exclude(_ prefix: Swift.String) throws -> OriginConsumer
	@inlinable public consuming func withStats(_ session: consuming StatsSession) throws -> OriginConsumer
	@inlinable public func announced() throws -> AnnounceConsumer
	@inlinable public func consume() throws -> OriginConsumer
	@inlinable public func follow(_ path: Swift.String, _ instance: consuming Instance?) throws -> OriginFollowing
	@inlinable public func scope(_ root: Swift.String, _ patterns: borrowing Patterns) throws -> OriginConsumer
	@inlinable public func root() throws -> Path
	@inlinable public func allowed() throws -> Patterns
	@inlinable public func absolute(_ path: Swift.String) throws -> Path
}
public struct OriginDriver: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func poll(_ now: ClockInstant, _ waiter: borrowing Waiter) throws -> ClockInstant?
}
public struct OriginPrepared: ~Copyable, @unchecked Sendable {
	@inlinable public func broadcast() throws -> BroadcastProducer
	@inlinable public consuming func publish() throws -> OriginPublication
}
public struct OriginProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> OriginProducer
	@inlinable public static func new(_ config: consuming OriginConfig) throws -> TupleOriginProducerOriginDriver
	@inlinable public consuming func withStats(_ session: consuming StatsSession) throws -> OriginProducer
	@inlinable public consuming func withLink(_ link: OriginLink) throws -> OriginProducer
	@inlinable public consuming func withPreference(_ preference: OriginPreference) throws -> OriginProducer
	@inlinable public func config() throws -> OriginConfig
	@inlinable public func hop() throws -> Hop
	@inlinable public func createBroadcast(_ path: Swift.String) throws -> OriginPublication
	@inlinable public func publish(_ path: Swift.String, _ route: consuming OriginRoute) throws -> OriginPublication
	@inlinable public func requests(_ prefix: Swift.String) throws -> OriginRequests
	@inlinable public func scope(_ root: Swift.String, _ patterns: borrowing Patterns) throws -> OriginProducer
	@inlinable public func mount(_ at: Swift.String, _ target: Swift.String) throws -> OriginProducer
	@inlinable public func consume() throws -> OriginConsumer
	@inlinable public func root() throws -> Path
	@inlinable public func allowed() throws -> Patterns
	@inlinable public func absolute(_ path: Swift.String) throws -> Path
}
public struct OriginPublication: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> OriginPublication
	@inlinable public func broadcast() -> BroadcastProducer
	@inlinable public func announce(_ route: consuming OriginRoute) throws
	@inlinable public func unannounce()
	@inlinable public func route() -> OriginRoute?
	@inlinable public func info() -> BroadcastInfo
	@inlinable public func demand() -> BroadcastDemand
	@inlinable public func createTrack(_ name: Swift.String, _ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public func createDatagrams(_ name: Swift.String, _ info: DatagramInfo) throws -> DatagramProducer
	@inlinable public func reserveTrack(_ name: Swift.String) throws -> TrackRequest
	@inlinable public func uniqueTrack(_ suffix: Swift.String, _ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public func uniqueName(_ suffix: Swift.String) -> Swift.String
	@inlinable public func requests() -> BroadcastRequests
	@inlinable public func consume() -> BroadcastConsumer
	@inlinable public func close()
	@inlinable public func isClone(_ other: borrowing BroadcastProducer) throws -> Swift.Bool
}
public struct OriginRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func path() throws -> Path
	@inlinable public consuming func prepare() throws -> OriginPrepared
	@inlinable public consuming func serveProducer(_ broadcast: borrowing BroadcastProducer) throws
	@inlinable public consuming func serveConsumer(_ broadcast: borrowing BroadcastConsumer) throws
	@inlinable public consuming func reject(_ err: Error) throws
}
public struct OriginRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func route() -> OriginRoute
}
public struct OriginRoute: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginRoute
	@inlinable public static func == (a: borrowing OriginRoute, b: borrowing OriginRoute) throws -> Swift.Bool
	@inlinable public static func `default`() -> OriginRoute
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public func hopsGet() throws -> Hops
	@inlinable public mutating func hopsSet(_ value: consuming Hops) throws
	@inlinable public var costGet: OriginCost { get }
	@inlinable public mutating func costSet(_ value: OriginCost) throws
	@inlinable public func authGet() throws -> AuthCredential?
	@inlinable public mutating func authSet(_ value: consuming AuthCredential?) throws
	@inlinable public consuming func withInstance(_ instance: consuming Instance, _ epoch: Swift.UInt64) throws -> OriginRoute
	@inlinable public consuming func withHops(_ hops: consuming Hops) throws -> OriginRoute
	@inlinable public consuming func withCost(_ cost: OriginCost) throws -> OriginRoute
	@inlinable public consuming func withAuth(_ auth: consuming AuthCredential) throws -> OriginRoute
	@inlinable public func isAnonymous() throws -> Swift.Bool
	@inlinable public func source() throws -> OriginSource
}
public struct AnnounceConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func tryNext() throws -> AnnounceEvent?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func root() throws -> Path
	@inlinable public func absolute(_ prefix: Swift.String) throws -> Path
}
public struct AnnounceEvent: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> AnnounceEvent
}
public struct HangStatsSnapshot: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangStatsSnapshot
	@inlinable public static func == (a: borrowing HangStatsSnapshot, b: borrowing HangStatsSnapshot) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangStatsSnapshot
	@inlinable public func transportGet() throws -> HangStatsTransport
	@inlinable public mutating func transportSet(_ value: consuming HangStatsTransport) throws
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangStatsRendition>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangStatsRendition>) throws
}
public struct HangStatsSnapshotString: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangStatsSnapshotString
	@inlinable public static func == (a: borrowing HangStatsSnapshotString, b: borrowing HangStatsSnapshotString) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangStatsSnapshotString
	@inlinable public func transportGet() throws -> HangStatsTransport
	@inlinable public mutating func transportSet(_ value: consuming HangStatsTransport) throws
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangStatsRendition>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangStatsRendition>) throws
	@inlinable public var extGet: Swift.String { get }
	@inlinable public mutating func extSet(_ value: Swift.String) throws
}
public struct HangStatsRendition: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangStatsRendition
	@inlinable public static func == (a: borrowing HangStatsRendition, b: borrowing HangStatsRendition) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangStatsRendition
	@inlinable public var framesGet: Swift.UInt64 { get }
	@inlinable public mutating func framesSet(_ value: Swift.UInt64) throws
	@inlinable public var bytesGet: Swift.UInt64 { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64) throws
	@inlinable public var keyframesGet: Swift.UInt64 { get }
	@inlinable public mutating func keyframesSet(_ value: Swift.UInt64) throws
	@inlinable public var skippedGet: Swift.UInt64 { get }
	@inlinable public mutating func skippedSet(_ value: Swift.UInt64) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
}
public struct HangStatsTransport: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangStatsTransport
	@inlinable public static func == (a: borrowing HangStatsTransport, b: borrowing HangStatsTransport) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangStatsTransport
	@inlinable public var rttGet: RustDuration? { get }
	@inlinable public mutating func rttSet(_ value: RustDuration?) throws
	@inlinable public var rateGet: Swift.UInt64? { get }
	@inlinable public mutating func rateSet(_ value: Swift.UInt64?) throws
	@inlinable public var lostBytesGet: Swift.UInt64? { get }
	@inlinable public mutating func lostBytesSet(_ value: Swift.UInt64?) throws
	@inlinable public var lostPacketsGet: Swift.UInt64? { get }
	@inlinable public mutating func lostPacketsSet(_ value: Swift.UInt64?) throws
	@inlinable public var ageGet: RustDuration? { get }
	@inlinable public mutating func ageSet(_ value: RustDuration?) throws
}
public struct HangEchoSnapshot: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangEchoSnapshot
	@inlinable public static func == (a: borrowing HangEchoSnapshot, b: borrowing HangEchoSnapshot) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangEchoSnapshot
	@inlinable public func transportGet() throws -> HangStatsTransport
	@inlinable public mutating func transportSet(_ value: consuming HangStatsTransport) throws
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangEchoRendition>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangEchoRendition>) throws
}
public struct HangEchoSnapshotString: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangEchoSnapshotString
	@inlinable public static func == (a: borrowing HangEchoSnapshotString, b: borrowing HangEchoSnapshotString) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangEchoSnapshotString
	@inlinable public func transportGet() throws -> HangStatsTransport
	@inlinable public mutating func transportSet(_ value: consuming HangStatsTransport) throws
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangEchoRendition>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangEchoRendition>) throws
	@inlinable public var extGet: Swift.String { get }
	@inlinable public mutating func extSet(_ value: Swift.String) throws
}
public struct HangEchoRendition: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangEchoRendition
	@inlinable public static func == (a: borrowing HangEchoRendition, b: borrowing HangEchoRendition) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangEchoRendition
	@inlinable public var framesGet: Swift.UInt64 { get }
	@inlinable public mutating func framesSet(_ value: Swift.UInt64) throws
	@inlinable public var bytesGet: Swift.UInt64 { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64) throws
	@inlinable public var decodedGet: Swift.UInt64 { get }
	@inlinable public mutating func decodedSet(_ value: Swift.UInt64) throws
	@inlinable public var lateGet: Swift.UInt64 { get }
	@inlinable public mutating func lateSet(_ value: Swift.UInt64) throws
	@inlinable public var errorsGet: Swift.UInt64 { get }
	@inlinable public mutating func errorsSet(_ value: Swift.UInt64) throws
	@inlinable public var stallsGet: Swift.UInt64 { get }
	@inlinable public mutating func stallsSet(_ value: Swift.UInt64) throws
	@inlinable public var underrunsGet: Swift.UInt64 { get }
	@inlinable public mutating func underrunsSet(_ value: Swift.UInt64) throws
	@inlinable public var stalledGet: RustDuration { get }
	@inlinable public mutating func stalledSet(_ value: RustDuration) throws
	@inlinable public func arrivalGet() throws -> HangEchoArrival?
	@inlinable public mutating func arrivalSet(_ value: consuming HangEchoArrival?) throws
	@inlinable public var latencyGet: RustDuration? { get }
	@inlinable public mutating func latencySet(_ value: RustDuration?) throws
}
public struct HangEchoArrival: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangEchoArrival
	@inlinable public static func == (a: borrowing HangEchoArrival, b: borrowing HangEchoArrival) throws -> Swift.Bool
	@inlinable public var timestampGet: Timestamp { get }
	@inlinable public mutating func timestampSet(_ value: Timestamp) throws
	@inlinable public var wallGet: ClockWall { get }
	@inlinable public mutating func wallSet(_ value: ClockWall) throws
	@inlinable public static func new(_ timestamp: Timestamp, _ wall: ClockWall) -> HangEchoArrival
}
public struct HangHealthObserver: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthObserver
	@inlinable public static func == (a: borrowing HangHealthObserver, b: borrowing HangHealthObserver) throws -> Swift.Bool
	@inlinable public func pathGet() throws -> Path
	@inlinable public mutating func pathSet(_ value: consuming Path) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public static func new(_ path: consuming Path, _ instance: consuming Instance?) throws -> HangHealthObserver
}
public struct HangHealthReportEchoSnapshotString: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthReportEchoSnapshotString
	@inlinable public static func == (a: borrowing HangHealthReportEchoSnapshotString, b: borrowing HangHealthReportEchoSnapshotString) throws -> Swift.Bool
	@inlinable public func observerGet() throws -> HangHealthObserver
	@inlinable public mutating func observerSet(_ value: consuming HangHealthObserver) throws
	@inlinable public func snapshotGet() throws -> HangEchoSnapshotString
	@inlinable public mutating func snapshotSet(_ value: consuming HangEchoSnapshotString) throws
	@inlinable public static func new(_ observer: consuming HangHealthObserver, _ snapshot: consuming HangEchoSnapshotString) throws -> HangHealthReportEchoSnapshotString
}
public struct HangHealthReportStatsSnapshotString: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthReportStatsSnapshotString
	@inlinable public static func == (a: borrowing HangHealthReportStatsSnapshotString, b: borrowing HangHealthReportStatsSnapshotString) throws -> Swift.Bool
	@inlinable public func observerGet() throws -> HangHealthObserver
	@inlinable public mutating func observerSet(_ value: consuming HangHealthObserver) throws
	@inlinable public func snapshotGet() throws -> HangStatsSnapshotString
	@inlinable public mutating func snapshotSet(_ value: consuming HangStatsSnapshotString) throws
	@inlinable public static func new(_ observer: consuming HangHealthObserver, _ snapshot: consuming HangStatsSnapshotString) throws -> HangHealthReportStatsSnapshotString
}
public struct HangHealthThresholds: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthThresholds
	@inlinable public static func == (a: borrowing HangHealthThresholds, b: borrowing HangHealthThresholds) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangHealthThresholds
	@inlinable public var staleGet: RustDuration { get }
	@inlinable public mutating func staleSet(_ value: RustDuration) throws
	@inlinable public func transportGet() throws -> HangHealthTransportThresholds
	@inlinable public mutating func transportSet(_ value: consuming HangHealthTransportThresholds) throws
	@inlinable public func publisherGet() throws -> HangHealthPublisherThresholds
	@inlinable public mutating func publisherSet(_ value: consuming HangHealthPublisherThresholds) throws
	@inlinable public func viewerGet() throws -> HangHealthViewerThresholds
	@inlinable public mutating func viewerSet(_ value: consuming HangHealthViewerThresholds) throws
}
public struct HangHealthTransportThresholds: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthTransportThresholds
	@inlinable public static func == (a: borrowing HangHealthTransportThresholds, b: borrowing HangHealthTransportThresholds) throws -> Swift.Bool
	@inlinable public var rttGet: HangHealthLimitsDuration { get }
	@inlinable public mutating func rttSet(_ value: HangHealthLimitsDuration) throws
	@inlinable public var lossGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func lossSet(_ value: HangHealthLimitsF64) throws
	@inlinable public var headroomGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func headroomSet(_ value: HangHealthLimitsF64) throws
}
public struct HangHealthPublisherThresholds: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthPublisherThresholds
	@inlinable public static func == (a: borrowing HangHealthPublisherThresholds, b: borrowing HangHealthPublisherThresholds) throws -> Swift.Bool
	@inlinable public var skippedGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func skippedSet(_ value: HangHealthLimitsF64) throws
}
public struct HangHealthViewerThresholds: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthViewerThresholds
	@inlinable public static func == (a: borrowing HangHealthViewerThresholds, b: borrowing HangHealthViewerThresholds) throws -> Swift.Bool
	@inlinable public var lateGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func lateSet(_ value: HangHealthLimitsF64) throws
	@inlinable public var errorsGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func errorsSet(_ value: HangHealthLimitsF64) throws
	@inlinable public var stalledGet: HangHealthLimitsF64 { get }
	@inlinable public mutating func stalledSet(_ value: HangHealthLimitsF64) throws
}
public struct HangHealthEvidence: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthEvidence
	@inlinable public static func == (a: borrowing HangHealthEvidence, b: borrowing HangHealthEvidence) throws -> Swift.Bool
	@inlinable public var inputGet: HangHealthInput { get }
	@inlinable public mutating func inputSet(_ value: HangHealthInput) throws
	@inlinable public var valueGet: Swift.Double? { get }
	@inlinable public mutating func valueSet(_ value: Swift.Double?) throws
	@inlinable public var verdictGet: HangHealthVerdict { get }
	@inlinable public mutating func verdictSet(_ value: HangHealthVerdict) throws
}
public struct HangHealthConnection: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthConnection
	@inlinable public static func == (a: borrowing HangHealthConnection, b: borrowing HangHealthConnection) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangHealthConnection
	@inlinable public var verdictGet: HangHealthVerdict { get }
	@inlinable public mutating func verdictSet(_ value: HangHealthVerdict) throws
	@inlinable public func evidenceGet() throws -> UniqueArray<HangHealthEvidence>
	@inlinable public mutating func evidenceSet(_ value: consuming UniqueArray<HangHealthEvidence>) throws
	@inlinable public var staleGet: Swift.Bool { get }
	@inlinable public mutating func staleSet(_ value: Swift.Bool) throws
	@inlinable public var lostBytesGet: Swift.UInt64? { get }
	@inlinable public mutating func lostBytesSet(_ value: Swift.UInt64?) throws
	@inlinable public var lostPacketsGet: Swift.UInt64? { get }
	@inlinable public mutating func lostPacketsSet(_ value: Swift.UInt64?) throws
}
public struct HangHealthRendition: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangHealthRendition
	@inlinable public static func == (a: borrowing HangHealthRendition, b: borrowing HangHealthRendition) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangHealthRendition
	@inlinable public var verdictGet: HangHealthVerdict { get }
	@inlinable public mutating func verdictSet(_ value: HangHealthVerdict) throws
	@inlinable public func evidenceGet() throws -> UniqueArray<HangHealthEvidence>
	@inlinable public mutating func evidenceSet(_ value: consuming UniqueArray<HangHealthEvidence>) throws
	@inlinable public var framesGet: Swift.UInt64? { get }
	@inlinable public mutating func framesSet(_ value: Swift.UInt64?) throws
	@inlinable public var bytesGet: Swift.UInt64? { get }
	@inlinable public mutating func bytesSet(_ value: Swift.UInt64?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
}
public struct HangCatalogConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> HangCatalogConsumer
}
public struct HangCatalogConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> HangCatalogConsumerExtra
}
public struct HangCatalogConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> HangCatalogConsumerExt
}
public struct HangCatalogAudio: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogAudio
	@inlinable public static func == (a: borrowing HangCatalogAudio, b: borrowing HangCatalogAudio) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogAudio
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangCatalogAudioConfig>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogAudioConfig>) throws
	@inlinable public mutating func insert(_ name: Swift.String, _ config: consuming HangCatalogAudioConfig) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> HangCatalogAudioConfig?
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func enabled() throws -> UniqueArray<TupleStringHangCatalogAudioConfig>
	@inlinable public func ranked() throws -> UniqueArray<TupleStringHangCatalogAudioConfig>
}
public struct HangCatalogAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogAudioConfig
	@inlinable public static func == (a: borrowing HangCatalogAudioConfig, b: borrowing HangCatalogAudioConfig) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative?
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative?) throws
	@inlinable public var trackGet: Swift.String? { get }
	@inlinable public mutating func trackSet(_ value: Swift.String?) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public var labelGet: Swift.String? { get }
	@inlinable public mutating func labelSet(_ value: Swift.String?) throws
	@inlinable public var codecGet: HangCatalogAudioCodec { get }
	@inlinable public mutating func codecSet(_ value: HangCatalogAudioCodec) throws
	@inlinable public var sampleRateGet: Swift.UInt32 { get }
	@inlinable public mutating func sampleRateSet(_ value: Swift.UInt32) throws
	@inlinable public var channelCountGet: Swift.UInt32 { get }
	@inlinable public mutating func channelCountSet(_ value: Swift.UInt32) throws
	@inlinable public var layoutGet: HangCatalogAudioLayout? { get }
	@inlinable public mutating func layoutSet(_ value: HangCatalogAudioLayout?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var enabledGet: Swift.Bool { get }
	@inlinable public mutating func enabledSet(_ value: Swift.Bool) throws
	@inlinable public var descriptionGet: Bytes? { get }
	@inlinable public mutating func descriptionSet(_ value: Bytes?) throws
	@inlinable public func containerGet() throws -> HangCatalogContainer
	@inlinable public mutating func containerSet(_ value: consuming HangCatalogContainer) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var warmupGet: RustDuration? { get }
	@inlinable public mutating func warmupSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public static func new(_ codec: HangCatalogAudioCodec, _ sampleRate: Swift.UInt32, _ channelCount: Swift.UInt32) -> HangCatalogAudioConfig
	@inlinable public func sameIdentity(_ other: borrowing HangCatalogAudioConfig) throws -> Swift.Bool
}
public struct HangCatalogBinary: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogBinary
	@inlinable public static func == (a: borrowing HangCatalogBinary, b: borrowing HangCatalogBinary) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogBinary
	@inlinable public func tracksGet() throws -> UniqueArray<TupleStringHangCatalogBinaryConfig>
	@inlinable public mutating func tracksSet(_ value: consuming UniqueArray<TupleStringHangCatalogBinaryConfig>) throws
	@inlinable public mutating func insert(_ name: Swift.String, _ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> HangCatalogBinaryConfig?
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct HangCatalogBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogBinaryConfig
	@inlinable public static func == (a: borrowing HangCatalogBinaryConfig, b: borrowing HangCatalogBinaryConfig) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative?
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative?) throws
	@inlinable public var trackGet: Swift.String? { get }
	@inlinable public mutating func trackSet(_ value: Swift.String?) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public var modeGet: HangCatalogMode { get }
	@inlinable public mutating func modeSet(_ value: HangCatalogMode) throws
	@inlinable public var mimeGet: Swift.String? { get }
	@inlinable public mutating func mimeSet(_ value: Swift.String?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public func extraGet() throws -> UniqueArray<TupleStringJson>
	@inlinable public mutating func extraSet(_ value: consuming UniqueArray<TupleStringJson>) throws
	@inlinable public static func new(_ mode: HangCatalogMode) -> HangCatalogBinaryConfig
}
public struct HangCatalogContainer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogContainer
	@inlinable public static func == (a: borrowing HangCatalogContainer, b: borrowing HangCatalogContainer) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogContainer
}
public struct HangCatalogContribution: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogContribution
	@inlinable public static func == (a: borrowing HangCatalogContribution, b: borrowing HangCatalogContribution) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public static func new(_ broadcast: consuming PathRelative) throws -> HangCatalogContribution
}
public struct HangCatalogEcho: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogEcho
	@inlinable public static func == (a: borrowing HangCatalogEcho, b: borrowing HangCatalogEcho) throws -> Swift.Bool
	@inlinable public func pathGet() throws -> PathRelative
	@inlinable public mutating func pathSet(_ value: consuming PathRelative) throws
	@inlinable public static func new(_ path: consuming PathRelative) throws -> HangCatalogEcho
}
public struct HangCatalogJson: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogJson
	@inlinable public static func == (a: borrowing HangCatalogJson, b: borrowing HangCatalogJson) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogJson
	@inlinable public func tracksGet() throws -> UniqueArray<TupleStringHangCatalogJsonConfig>
	@inlinable public mutating func tracksSet(_ value: consuming UniqueArray<TupleStringHangCatalogJsonConfig>) throws
	@inlinable public mutating func insert(_ name: Swift.String, _ config: consuming HangCatalogJsonConfig) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> HangCatalogJsonConfig?
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct HangCatalogJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogJsonConfig
	@inlinable public static func == (a: borrowing HangCatalogJsonConfig, b: borrowing HangCatalogJsonConfig) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative?
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative?) throws
	@inlinable public var trackGet: Swift.String? { get }
	@inlinable public mutating func trackSet(_ value: Swift.String?) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public var modeGet: HangCatalogMode { get }
	@inlinable public mutating func modeSet(_ value: HangCatalogMode) throws
	@inlinable public var schemaGet: Swift.String? { get }
	@inlinable public mutating func schemaSet(_ value: Swift.String?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public func extraGet() throws -> UniqueArray<TupleStringJson>
	@inlinable public mutating func extraSet(_ value: consuming UniqueArray<TupleStringJson>) throws
	@inlinable public static func new(_ mode: HangCatalogMode) -> HangCatalogJsonConfig
}
public struct HangCatalogM2ts: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogM2ts
	@inlinable public static func == (a: borrowing HangCatalogM2ts, b: borrowing HangCatalogM2ts) throws -> Swift.Bool
	@inlinable public var trackGet: Swift.String { get }
	@inlinable public mutating func trackSet(_ value: Swift.String) throws
	@inlinable public var randomAccessGet: Swift.Bool { get }
	@inlinable public mutating func randomAccessSet(_ value: Swift.Bool) throws
	@inlinable public var muxRateGet: Swift.UInt64? { get }
	@inlinable public mutating func muxRateSet(_ value: Swift.UInt64?) throws
	@inlinable public static func new(_ track: Swift.String) -> HangCatalogM2ts
}
public final class HangCatalogMedia: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogMedia
	@inlinable public static func == (a: HangCatalogMedia, b: HangCatalogMedia) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogMedia
	@inlinable public var clockGet: HangCatalogClock? { get }
	@inlinable public func clockSet(_ value: HangCatalogClock?) throws
	@inlinable public var joinGet: HangCatalogJoin? { get }
	@inlinable public func joinSet(_ value: HangCatalogJoin?) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func interval() throws -> RustDuration?
}
public final class HangCatalogJoin: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogJoin
	@inlinable public static func == (a: HangCatalogJoin, b: HangCatalogJoin) throws -> Swift.Bool
	@inlinable public var intervalGet: RustDuration { get }
	@inlinable public func intervalSet(_ value: RustDuration) throws
	@inlinable public static func new(_ interval: RustDuration) -> HangCatalogJoin?
}
public struct HangCatalogStats: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogStats
	@inlinable public static func == (a: borrowing HangCatalogStats, b: borrowing HangCatalogStats) throws -> Swift.Bool
	@inlinable public var trackGet: Swift.String { get }
	@inlinable public mutating func trackSet(_ value: Swift.String) throws
	@inlinable public static func new(_ track: Swift.String) -> HangCatalogStats
}
public struct HangCatalogText: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogText
	@inlinable public static func == (a: borrowing HangCatalogText, b: borrowing HangCatalogText) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogText
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangCatalogTextConfig>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogTextConfig>) throws
	@inlinable public mutating func insert(_ name: Swift.String, _ config: consuming HangCatalogTextConfig) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> HangCatalogTextConfig?
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct HangCatalogTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogTextConfig
	@inlinable public static func == (a: borrowing HangCatalogTextConfig, b: borrowing HangCatalogTextConfig) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative?
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative?) throws
	@inlinable public var trackGet: Swift.String? { get }
	@inlinable public mutating func trackSet(_ value: Swift.String?) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public var formatGet: HangCatalogTextFormat { get }
	@inlinable public mutating func formatSet(_ value: HangCatalogTextFormat) throws
	@inlinable public var roleGet: HangCatalogTextRole { get }
	@inlinable public mutating func roleSet(_ value: HangCatalogTextRole) throws
	@inlinable public var langGet: Swift.String? { get }
	@inlinable public mutating func langSet(_ value: Swift.String?) throws
	@inlinable public var labelGet: Swift.String? { get }
	@inlinable public mutating func labelSet(_ value: Swift.String?) throws
	@inlinable public func containerGet() throws -> HangCatalogContainer
	@inlinable public mutating func containerSet(_ value: consuming HangCatalogContainer) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public static func new(_ format: HangCatalogTextFormat) -> HangCatalogTextConfig
}
public struct HangCatalogColor: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogColor
	@inlinable public static func == (a: borrowing HangCatalogColor, b: borrowing HangCatalogColor) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogColor
	@inlinable public var primariesGet: Swift.UInt8? { get }
	@inlinable public mutating func primariesSet(_ value: Swift.UInt8?) throws
	@inlinable public var transferGet: Swift.UInt8? { get }
	@inlinable public mutating func transferSet(_ value: Swift.UInt8?) throws
	@inlinable public var matrixGet: Swift.UInt8? { get }
	@inlinable public mutating func matrixSet(_ value: Swift.UInt8?) throws
	@inlinable public var rangeGet: HangCatalogRange? { get }
	@inlinable public mutating func rangeSet(_ value: HangCatalogRange?) throws
	@inlinable public var masteringGet: HangCatalogMastering? { get }
	@inlinable public mutating func masteringSet(_ value: HangCatalogMastering?) throws
	@inlinable public var contentLightGet: HangCatalogContentLight? { get }
	@inlinable public mutating func contentLightSet(_ value: HangCatalogContentLight?) throws
}
public struct HangCatalogProjection: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogProjection
	@inlinable public static func == (a: borrowing HangCatalogProjection, b: borrowing HangCatalogProjection) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogProjection
}
public struct HangCatalogVideo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogVideo
	@inlinable public static func == (a: borrowing HangCatalogVideo, b: borrowing HangCatalogVideo) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogVideo
	@inlinable public func renditionsGet() throws -> UniqueArray<TupleStringHangCatalogVideoConfig>
	@inlinable public mutating func renditionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogVideoConfig>) throws
	@inlinable public var displayGet: HangCatalogDisplay? { get }
	@inlinable public mutating func displaySet(_ value: HangCatalogDisplay?) throws
	@inlinable public var rotationGet: Swift.Double? { get }
	@inlinable public mutating func rotationSet(_ value: Swift.Double?) throws
	@inlinable public var flipGet: Swift.Bool? { get }
	@inlinable public mutating func flipSet(_ value: Swift.Bool?) throws
	@inlinable public mutating func insert(_ name: Swift.String, _ config: consuming HangCatalogVideoConfig) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> HangCatalogVideoConfig?
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func enabled() throws -> UniqueArray<TupleStringHangCatalogVideoConfig>
	@inlinable public func ranked() throws -> UniqueArray<TupleStringHangCatalogVideoConfig>
	@inlinable public mutating func setProperties(_ properties: consuming HangCatalogVideoProperties) throws
}
public struct HangCatalogVideoProperties: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogVideoProperties
	@inlinable public static func == (a: borrowing HangCatalogVideoProperties, b: borrowing HangCatalogVideoProperties) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogVideoProperties
	@inlinable public var displayGet: HangCatalogDisplay? { get }
	@inlinable public mutating func displaySet(_ value: HangCatalogDisplay?) throws
	@inlinable public var rotationGet: Swift.Double? { get }
	@inlinable public mutating func rotationSet(_ value: Swift.Double?) throws
	@inlinable public var flipGet: Swift.Bool? { get }
	@inlinable public mutating func flipSet(_ value: Swift.Bool?) throws
}
public struct HangCatalogVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogVideoConfig
	@inlinable public static func == (a: borrowing HangCatalogVideoConfig, b: borrowing HangCatalogVideoConfig) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative?
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative?) throws
	@inlinable public var trackGet: Swift.String? { get }
	@inlinable public mutating func trackSet(_ value: Swift.String?) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public var labelGet: Swift.String? { get }
	@inlinable public mutating func labelSet(_ value: Swift.String?) throws
	@inlinable public var codecGet: HangCatalogVideoCodec { get }
	@inlinable public mutating func codecSet(_ value: HangCatalogVideoCodec) throws
	@inlinable public var descriptionGet: Bytes? { get }
	@inlinable public mutating func descriptionSet(_ value: Bytes?) throws
	@inlinable public var codedWidthGet: Swift.UInt32? { get }
	@inlinable public mutating func codedWidthSet(_ value: Swift.UInt32?) throws
	@inlinable public var codedHeightGet: Swift.UInt32? { get }
	@inlinable public mutating func codedHeightSet(_ value: Swift.UInt32?) throws
	@inlinable public var displayAspectWidthGet: Swift.UInt32? { get }
	@inlinable public mutating func displayAspectWidthSet(_ value: Swift.UInt32?) throws
	@inlinable public var displayAspectHeightGet: Swift.UInt32? { get }
	@inlinable public mutating func displayAspectHeightSet(_ value: Swift.UInt32?) throws
	@inlinable public func colorGet() throws -> HangCatalogColor?
	@inlinable public mutating func colorSet(_ value: consuming HangCatalogColor?) throws
	@inlinable public func projectionGet() throws -> HangCatalogProjection?
	@inlinable public mutating func projectionSet(_ value: consuming HangCatalogProjection?) throws
	@inlinable public var stereoGet: HangCatalogStereo? { get }
	@inlinable public mutating func stereoSet(_ value: HangCatalogStereo?) throws
	@inlinable public var interlacedGet: Swift.Bool { get }
	@inlinable public mutating func interlacedSet(_ value: Swift.Bool) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var enabledGet: Swift.Bool { get }
	@inlinable public mutating func enabledSet(_ value: Swift.Bool) throws
	@inlinable public var preferenceGet: Swift.Int32 { get }
	@inlinable public mutating func preferenceSet(_ value: Swift.Int32) throws
	@inlinable public var framerateGet: Swift.Double? { get }
	@inlinable public mutating func framerateSet(_ value: Swift.Double?) throws
	@inlinable public var reorderGet: Swift.UInt8? { get }
	@inlinable public mutating func reorderSet(_ value: Swift.UInt8?) throws
	@inlinable public var optimizeForLatencyGet: Swift.Bool? { get }
	@inlinable public mutating func optimizeForLatencySet(_ value: Swift.Bool?) throws
	@inlinable public func containerGet() throws -> HangCatalogContainer
	@inlinable public mutating func containerSet(_ value: consuming HangCatalogContainer) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var warmupGet: RustDuration? { get }
	@inlinable public mutating func warmupSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public static func new(_ codec: HangCatalogVideoCodec) -> HangCatalogVideoConfig
	@inlinable public func sameIdentity(_ other: borrowing HangCatalogVideoConfig) throws -> Swift.Bool
}
public struct MuxCatalogConsumer: ~Copyable, @unchecked Sendable {}
public struct MuxCatalogConsumerExtra: ~Copyable, @unchecked Sendable {}
public struct MuxCatalogConsumerExt: ~Copyable, @unchecked Sendable {}
public struct MuxCatalogEstimate: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCatalogEstimate
	@inlinable public static func == (a: borrowing MuxCatalogEstimate, b: borrowing MuxCatalogEstimate) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxCatalogEstimate
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public var framerateGet: Swift.Double? { get }
	@inlinable public mutating func framerateSet(_ value: Swift.Double?) throws
	@inlinable public consuming func withJitter(_ jitter: RustDuration?) throws -> MuxCatalogEstimate
	@inlinable public consuming func withBitrate(_ bitrate: Swift.UInt64?) throws -> MuxCatalogEstimate
	@inlinable public consuming func withDelay(_ delay: RustDuration?) throws -> MuxCatalogEstimate
	@inlinable public consuming func withFramerate(_ framerate: Swift.Double?) throws -> MuxCatalogEstimate
}
public struct MuxCatalogEstimator: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCatalogEstimator
	@inlinable public static func new() -> MuxCatalogEstimator
	@inlinable public mutating func write(_ timestamp: Timestamp, _ bytes: Swift.UInt) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public func estimate() throws -> MuxCatalogEstimate
}
public struct MuxCatalogConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogConfig
	@inlinable public static func `default`() -> MuxCatalogConfig
	@inlinable public consuming func withCatalogExtra(_ catalog: consuming HangCatalogExtra) throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withCatalogExt(_ catalog: consuming HangCatalogExt) throws -> MuxCatalogConfigExt
	@inlinable public consuming func withCatalogUnit(_ catalog: consuming HangCatalog) throws -> MuxCatalogConfig
	@inlinable public consuming func withClock(_ clock: consuming MuxClock) throws -> MuxCatalogConfig
	@inlinable public consuming func withMaxAge(_ maxAge: RustDuration?) throws -> MuxCatalogConfig
	@inlinable public consuming func withBandwidth(_ bandwidth: consuming BandwidthAllocator) throws -> MuxCatalogConfig
}
public struct MuxCatalogConfigExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withCatalogExtra(_ catalog: consuming HangCatalogExtra) throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withCatalogExt(_ catalog: consuming HangCatalogExt) throws -> MuxCatalogConfigExt
	@inlinable public consuming func withCatalogUnit(_ catalog: consuming HangCatalog) throws -> MuxCatalogConfig
	@inlinable public consuming func withClock(_ clock: consuming MuxClock) throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withMaxAge(_ maxAge: RustDuration?) throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withBandwidth(_ bandwidth: consuming BandwidthAllocator) throws -> MuxCatalogConfigExtra
}
public struct MuxCatalogConfigExt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogConfigExt
	@inlinable public consuming func withCatalogExtra(_ catalog: consuming HangCatalogExtra) throws -> MuxCatalogConfigExtra
	@inlinable public consuming func withCatalogExt(_ catalog: consuming HangCatalogExt) throws -> MuxCatalogConfigExt
	@inlinable public consuming func withCatalogUnit(_ catalog: consuming HangCatalog) throws -> MuxCatalogConfig
	@inlinable public consuming func withClock(_ clock: consuming MuxClock) throws -> MuxCatalogConfigExt
	@inlinable public consuming func withMaxAge(_ maxAge: RustDuration?) throws -> MuxCatalogConfigExt
	@inlinable public consuming func withBandwidth(_ bandwidth: consuming BandwidthAllocator) throws -> MuxCatalogConfigExt
}
public struct MuxCatalogGuard: ~Copyable {
	@inlinable public consuming func commit() throws
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct MuxCatalogGuardExtra: ~Copyable {
	@inlinable public consuming func commit() throws
	@inlinable public mutating func setSection(_ name: Swift.String, _ value: consuming Json) throws
	@inlinable public mutating func removeSection(_ name: Swift.String) throws -> Json?
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public func extGet() throws -> MuxCatalogHangExtra
	@inlinable public mutating func extSet(_ value: consuming MuxCatalogHangExtra) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct MuxCatalogGuardExt: ~Copyable {
	@inlinable public consuming func commit() throws
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var mediaGet: HangCatalogMedia { get }
	@inlinable public mutating func mediaSet(_ value: HangCatalogMedia) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func m2tsGet() throws -> HangCatalogM2ts?
	@inlinable public mutating func m2tsSet(_ value: consuming HangCatalogM2ts?) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func contributionsGet() throws -> UniqueArray<TupleStringHangCatalogContribution>
	@inlinable public mutating func contributionsSet(_ value: consuming UniqueArray<TupleStringHangCatalogContribution>) throws
	@inlinable public func extGet() throws -> MuxContainerTsExt
	@inlinable public mutating func extSet(_ value: consuming MuxContainerTsExt) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func media() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toJsonPretty() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct MuxCatalogProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogProducer
	@inlinable public static func new(_ broadcast: inout BroadcastProducer, _ config: consuming MuxCatalogConfig) throws -> MuxCatalogProducer
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public mutating func mutate(_ f: @Sendable (inout HangCatalog) -> Void) throws
	@inlinable public mutating func modify() throws -> MuxCatalogGuard
	@inlinable public func snapshot() throws -> HangCatalog
	@inlinable public func clock() throws -> MuxClock
	@inlinable public func timebase() throws -> MuxCatalogTimebase
	@inlinable public func reserve() throws -> MuxCatalogReserved
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func isClaimedJsonConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedBinaryConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedVideoConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedAudioConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedTextConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func stats() throws -> UniqueArray<TupleStringHangStatsRendition>
	@inlinable public func jsonSnapshotConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonSnapshotJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonStreamConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func jsonStreamJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func consume() throws -> MuxCatalogHangConsumer
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func finish() throws
}
public struct MuxCatalogProducerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogProducerExtra
	@inlinable public static func new(_ broadcast: inout BroadcastProducer, _ config: consuming MuxCatalogConfigExtra) throws -> MuxCatalogProducerExtra
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public mutating func mutate(_ f: @Sendable (inout HangCatalogExtra) -> Void) throws
	@inlinable public mutating func modify() throws -> MuxCatalogGuardExtra
	@inlinable public func snapshot() throws -> HangCatalogExtra
	@inlinable public func clock() throws -> MuxClock
	@inlinable public func timebase() throws -> MuxCatalogTimebaseExtra
	@inlinable public func reserve() throws -> MuxCatalogReservedExtra
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func isClaimedJsonConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedBinaryConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedVideoConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedAudioConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedTextConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func stats() throws -> UniqueArray<TupleStringHangStatsRendition>
	@inlinable public func jsonSnapshotConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonSnapshotJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonStreamConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func jsonStreamJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func consume() throws -> MuxCatalogHangConsumerExtra
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func finish() throws
}
public struct MuxCatalogProducerExt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogProducerExt
	@inlinable public static func new(_ broadcast: inout BroadcastProducer, _ config: consuming MuxCatalogConfigExt) throws -> MuxCatalogProducerExt
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public mutating func mutate(_ f: @Sendable (inout HangCatalogExt) -> Void) throws
	@inlinable public mutating func modify() throws -> MuxCatalogGuardExt
	@inlinable public func snapshot() throws -> HangCatalogExt
	@inlinable public func clock() throws -> MuxClock
	@inlinable public func timebase() throws -> MuxCatalogTimebaseExt
	@inlinable public func reserve() throws -> MuxCatalogReservedExt
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func isClaimedJsonConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedBinaryConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedVideoConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedAudioConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func isClaimedTextConfig(_ name: Swift.String) throws -> Swift.Bool
	@inlinable public func stats() throws -> UniqueArray<TupleStringHangStatsRendition>
	@inlinable public func jsonSnapshotConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonSnapshotJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig, _ options: consuming MuxJsonOptions) throws -> MuxJsonSnapshotJson
	@inlinable public func jsonStreamConfig(_ track: TrackProducer, _ config: consuming MuxJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func jsonStreamJsonConfig(_ track: TrackProducer, _ config: consuming HangCatalogJsonConfig) throws -> MuxJsonStreamJson
	@inlinable public func consume() throws -> MuxCatalogHangConsumerExt
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func finish() throws
}
public struct MuxCatalogTimebase: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> MuxCatalogTimebase
	@inlinable public func reserve() -> MuxCatalogReserved
	@inlinable public func place(_ pts: Timestamp, _ wall: ClockWall) throws
}
public struct MuxCatalogTimebaseExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> MuxCatalogTimebaseExtra
	@inlinable public func reserve() -> MuxCatalogReservedExtra
	@inlinable public func place(_ pts: Timestamp, _ wall: ClockWall) throws
}
public struct MuxCatalogTimebaseExt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> MuxCatalogTimebaseExt
	@inlinable public func reserve() -> MuxCatalogReservedExt
	@inlinable public func place(_ pts: Timestamp, _ wall: ClockWall) throws
}
public struct MuxCatalogReserved: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogReserved
	@inlinable public consuming func withInterval(_ interval: RustDuration?) throws -> MuxCatalogReserved
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
}
public struct MuxCatalogReservedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogReservedExtra
	@inlinable public consuming func withInterval(_ interval: RustDuration?) throws -> MuxCatalogReservedExtra
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
}
public struct MuxCatalogReservedExt: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxCatalogReservedExt
	@inlinable public consuming func withInterval(_ interval: RustDuration?) throws -> MuxCatalogReservedExt
	@inlinable public func trackInfo(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public func videoContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func videoFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func videoLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func videoLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func audioContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func audioFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func audioLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func audioLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func textContainer(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func textFmp4Wire(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func textLegacyWire(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func textLocWire(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func trackContainerJsonConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func trackContainerBinaryConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func trackContainerVideoConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func trackContainerAudioConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func trackContainerTextConfig(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func trackFmp4WireJsonConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func trackFmp4WireBinaryConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func trackFmp4WireVideoConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func trackFmp4WireAudioConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func trackFmp4WireTextConfig(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func trackLegacyWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func trackLegacyWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func trackLegacyWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func trackLegacyWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func trackLegacyWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLegacyWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func trackLocWireJsonConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogJsonConfig?) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func trackLocWireBinaryConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogBinaryConfig?) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func trackLocWireVideoConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogVideoConfig?) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func trackLocWireAudioConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogAudioConfig?) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func trackLocWireTextConfig(_ track: TrackProducer, _ container: MuxContainerLocWire, _ config: consuming HangCatalogTextConfig?) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
}
public struct MuxCatalogVideoHint: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCatalogVideoHint
	@inlinable public static func == (a: borrowing MuxCatalogVideoHint, b: borrowing MuxCatalogVideoHint) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxCatalogVideoHint
	@inlinable public var codecGet: HangCatalogVideoCodec? { get }
	@inlinable public mutating func codecSet(_ value: HangCatalogVideoCodec?) throws
	@inlinable public var codedWidthGet: Swift.UInt32? { get }
	@inlinable public mutating func codedWidthSet(_ value: Swift.UInt32?) throws
	@inlinable public var codedHeightGet: Swift.UInt32? { get }
	@inlinable public mutating func codedHeightSet(_ value: Swift.UInt32?) throws
	@inlinable public var displayAspectWidthGet: Swift.UInt32? { get }
	@inlinable public mutating func displayAspectWidthSet(_ value: Swift.UInt32?) throws
	@inlinable public var displayAspectHeightGet: Swift.UInt32? { get }
	@inlinable public mutating func displayAspectHeightSet(_ value: Swift.UInt32?) throws
	@inlinable public func colorGet() throws -> HangCatalogColor?
	@inlinable public mutating func colorSet(_ value: consuming HangCatalogColor?) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public var framerateGet: Swift.Double? { get }
	@inlinable public mutating func framerateSet(_ value: Swift.Double?) throws
	@inlinable public var reorderGet: Swift.UInt8? { get }
	@inlinable public mutating func reorderSet(_ value: Swift.UInt8?) throws
	@inlinable public var optimizeForLatencyGet: Swift.Bool? { get }
	@inlinable public mutating func optimizeForLatencySet(_ value: Swift.Bool?) throws
	@inlinable public var jitterGet: RustDuration? { get }
	@inlinable public mutating func jitterSet(_ value: RustDuration?) throws
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public func containerGet() throws -> HangCatalogContainer
	@inlinable public mutating func containerSet(_ value: consuming HangCatalogContainer) throws
	@inlinable public var preferenceGet: Swift.Int32 { get }
	@inlinable public mutating func preferenceSet(_ value: Swift.Int32) throws
	@inlinable public func apply(_ config: inout HangCatalogVideoConfig) throws
	@inlinable public func toConfig() throws -> HangCatalogVideoConfig?
}
public struct MuxContainerConsumerContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: consuming MuxCatalogHangContainer) throws -> MuxContainerConsumerContainer
	@inlinable public consuming func withWarmup(_ warmup: RustDuration) throws -> MuxContainerConsumerContainer
	@inlinable public consuming func withDelay(_ delay: TrackDelay) throws -> MuxContainerConsumerContainer
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setMaxBuffer(_ bytes: Swift.UInt) throws
}
public struct MuxContainerConsumerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: consuming MuxContainerFmp4Wire) throws -> MuxContainerConsumerFmp4Wire
	@inlinable public consuming func withWarmup(_ warmup: RustDuration) throws -> MuxContainerConsumerFmp4Wire
	@inlinable public consuming func withDelay(_ delay: TrackDelay) throws -> MuxContainerConsumerFmp4Wire
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setMaxBuffer(_ bytes: Swift.UInt) throws
}
public struct MuxContainerConsumerLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: MuxContainerLegacyWire) throws -> MuxContainerConsumerLegacyWire
	@inlinable public consuming func withWarmup(_ warmup: RustDuration) throws -> MuxContainerConsumerLegacyWire
	@inlinable public consuming func withDelay(_ delay: TrackDelay) throws -> MuxContainerConsumerLegacyWire
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setMaxBuffer(_ bytes: Swift.UInt) throws
}
public struct MuxContainerConsumerLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: MuxContainerLocWire) throws -> MuxContainerConsumerLocWire
	@inlinable public consuming func withWarmup(_ warmup: RustDuration) throws -> MuxContainerConsumerLocWire
	@inlinable public consuming func withDelay(_ delay: TrackDelay) throws -> MuxContainerConsumerLocWire
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setMaxBuffer(_ bytes: Swift.UInt) throws
}
public struct MuxContainerGroupConsumerContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: consuming MuxCatalogHangContainer) throws -> MuxContainerGroupConsumerContainer
	@inlinable public func sequence() throws -> Swift.UInt64
}
public struct MuxContainerGroupConsumerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: consuming MuxContainerFmp4Wire) throws -> MuxContainerGroupConsumerFmp4Wire
	@inlinable public func sequence() throws -> Swift.UInt64
}
public struct MuxContainerGroupConsumerLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: MuxContainerLegacyWire) throws -> MuxContainerGroupConsumerLegacyWire
	@inlinable public func sequence() throws -> Swift.UInt64
}
public struct MuxContainerGroupConsumerLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: MuxContainerLocWire) throws -> MuxContainerGroupConsumerLocWire
	@inlinable public func sequence() throws -> Swift.UInt64
}
public struct MuxContainerProducerContainerAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerContainerBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerContainerJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerContainerTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerContainerVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer) throws -> MuxContainerProducerContainer
	@inlinable public mutating func set() throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainer
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainer
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4WireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4WireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4WireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4WireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4WireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire) throws -> MuxContainerProducerFmp4Wire
	@inlinable public mutating func set() throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4Wire
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4Wire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: TrackProducer, _ container: MuxContainerLegacyWire) throws -> MuxContainerProducerLegacyWire
	@inlinable public mutating func set() throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWire
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxContainerProducerLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: TrackProducer, _ container: MuxContainerLocWire) throws -> MuxContainerProducerLocWire
	@inlinable public mutating func set() throws
	@inlinable public mutating func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public mutating func reorder(_ delay: Timestamp) throws
	@inlinable public mutating func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public consuming func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWire
	@inlinable public consuming func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public mutating func write(_ frame: MuxContainerFrame) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func fail(_ err: Error) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup() throws -> GroupProducer
	@inlinable public func dropGroups(_ groups: RangeU64, _ err: StreamError) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxFollowInstances: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ origin: borrowing OriginConsumer, _ path: Swift.String) throws -> MuxFollowInstances
	@inlinable public static func new(_ origin: borrowing OriginConsumer, _ path: Swift.String, instance: borrowing Instance) throws -> MuxFollowInstances
}
public struct MuxFollowEvent: ~Copyable, @unchecked Sendable {}
public struct MuxImportAudioInit: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxImportAudioInit
	@inlinable public static func == (a: borrowing MuxImportAudioInit, b: borrowing MuxImportAudioInit) throws -> Swift.Bool
	@inlinable public var formatGet: MuxImportAudioFormat { get }
	@inlinable public mutating func formatSet(_ value: MuxImportAudioFormat) throws
	@inlinable public var dataGet: Bytes { get }
	@inlinable public mutating func dataSet(_ value: Bytes) throws
	@inlinable public var labelGet: Swift.String? { get }
	@inlinable public mutating func labelSet(_ value: Swift.String?) throws
	@inlinable public func containerGet() throws -> HangCatalogContainer
	@inlinable public mutating func containerSet(_ value: consuming HangCatalogContainer) throws
	@inlinable public var bitrateGet: Swift.UInt64? { get }
	@inlinable public mutating func bitrateSet(_ value: Swift.UInt64?) throws
	@inlinable public static func new(_ format: MuxImportAudioFormat, _ data: Bytes) -> MuxImportAudioInit
}
public struct MuxImportContainerInit: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxImportContainerInit
	@inlinable public static func == (a: borrowing MuxImportContainerInit, b: borrowing MuxImportContainerInit) throws -> Swift.Bool
	@inlinable public var formatGet: MuxImportContainerFormat { get }
	@inlinable public mutating func formatSet(_ value: MuxImportContainerFormat) throws
	@inlinable public var dataGet: Bytes { get }
	@inlinable public mutating func dataSet(_ value: Bytes) throws
	@inlinable public static func new(_ format: MuxImportContainerFormat, _ data: Bytes) -> MuxImportContainerInit
}
public struct MuxImportVideoInit: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxImportVideoInit
	@inlinable public static func == (a: borrowing MuxImportVideoInit, b: borrowing MuxImportVideoInit) throws -> Swift.Bool
	@inlinable public var formatGet: MuxImportVideoFormat { get }
	@inlinable public mutating func formatSet(_ value: MuxImportVideoFormat) throws
	@inlinable public var dataGet: Bytes { get }
	@inlinable public mutating func dataSet(_ value: Bytes) throws
	@inlinable public var labelGet: Swift.String? { get }
	@inlinable public mutating func labelSet(_ value: Swift.String?) throws
	@inlinable public func hintGet() throws -> MuxCatalogVideoHint
	@inlinable public mutating func hintSet(_ value: consuming MuxCatalogVideoHint) throws
	@inlinable public var priorityGet: Swift.Int8 { get }
	@inlinable public mutating func prioritySet(_ value: Swift.Int8) throws
	@inlinable public static func new(_ format: MuxImportVideoFormat, _ data: Bytes) -> MuxImportVideoInit
}
public struct MuxImportContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainer
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
}
public struct MuxImportContainerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainerExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
}
public struct MuxImportContainerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainerExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
}
public struct MuxImportTrack: ~Copyable, @unchecked Sendable {
	@inlinable public static func audioExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func audioExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func audioUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func videoExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public static func videoExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public static func videoUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
}
public struct MuxImportTrackStream: ~Copyable, @unchecked Sendable {
	@inlinable public static func videoExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public static func videoExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public static func videoUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public mutating func initialize(_ data: Bytes) throws
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
}
public struct MuxJsonSnapshotJson: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func update(_ value: borrowing Json, _ timestamp: Timestamp) throws
	@inlinable public consuming func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxJsonStreamJson: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func append(_ value: borrowing Json, _ timestamp: Timestamp) throws
	@inlinable public consuming func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxJsonConsumerJson: ~Copyable, @unchecked Sendable {
	@inlinable public static func fromTrack(_ track: consuming TrackSubscriber, _ config: borrowing HangCatalogJsonConfig) throws -> MuxJsonConsumerJson
	@inlinable public func mode() throws -> HangCatalogMode
}
public struct MuxJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxJsonConfig
	@inlinable public static func `default`() -> MuxJsonConfig
	@inlinable public var schemaGet: Swift.String? { get }
	@inlinable public mutating func schemaSet(_ value: Swift.String?) throws
	@inlinable public consuming func withSchema(_ schema: Swift.String) throws -> MuxJsonConfig
}
public struct MuxJsonOptions: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxJsonOptions
	@inlinable public static func `default`() -> MuxJsonOptions
	@inlinable public var deltasGet: Swift.Bool { get }
	@inlinable public mutating func deltasSet(_ value: Swift.Bool) throws
	@inlinable public consuming func withDeltas(_ deltas: Swift.Bool) throws -> MuxJsonOptions
}
public struct MuxRateFollower: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ grant: consuming BandwidthConsumer, _ policy: consuming MuxRatePolicy) throws -> MuxRateFollower
	@inlinable public func policy() throws -> MuxRatePolicy
	@inlinable public mutating func setPolicy(_ policy: consuming MuxRatePolicy) throws
	@inlinable public mutating func failed() throws
}
public struct MuxRatePolicy: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxRatePolicy
	@inlinable public static func == (a: borrowing MuxRatePolicy, b: borrowing MuxRatePolicy) throws -> Swift.Bool
	@inlinable public var maxGet: BandwidthRate { get }
	@inlinable public mutating func maxSet(_ value: BandwidthRate) throws
	@inlinable public var minGet: BandwidthRate { get }
	@inlinable public mutating func minSet(_ value: BandwidthRate) throws
	@inlinable public var lowerGet: BandwidthRate? { get }
	@inlinable public mutating func lowerSet(_ value: BandwidthRate?) throws
	@inlinable public static func new(_ max: BandwidthRate) -> MuxRatePolicy
}
public struct MuxRateControl: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxRateControl
	@inlinable public static func new(_ policy: consuming MuxRatePolicy) throws -> MuxRateControl
	@inlinable public func target() throws -> MuxRateTarget
	@inlinable public mutating func setProbed(_ probed: Swift.Bool) throws
	@inlinable public mutating func update(_ estimate: BandwidthRate?, _ now: ClockInstant) throws -> MuxRateTarget?
}
public struct MuxSelectBroadcast: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxSelectBroadcast
	@inlinable public static func `default`() -> MuxSelectBroadcast
	@inlinable public consuming func video(_ video: consuming MuxSelectVideo) throws -> MuxSelectBroadcast
	@inlinable public consuming func audio(_ audio: consuming MuxSelectAudio) throws -> MuxSelectBroadcast
	@inlinable public func hasVideo() throws -> Swift.Bool
	@inlinable public func hasAudio() throws -> Swift.Bool
	@inlinable public func retainExtra(_ catalog: inout HangCatalogExtra, _ selected: MuxSelectSelected) throws
	@inlinable public func retainExt(_ catalog: inout HangCatalogExt, _ selected: MuxSelectSelected) throws
	@inlinable public func retainUnit(_ catalog: inout HangCatalog, _ selected: MuxSelectSelected) throws
}
public struct MuxSelectVideo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxSelectVideo
	@inlinable public static func `default`() -> MuxSelectVideo
	@inlinable public consuming func name(_ name: Swift.String) throws -> MuxSelectVideo
	@inlinable public consuming func codec(_ codec: HangCatalogVideoCodecKind) throws -> MuxSelectVideo
}
public struct MuxSelectAudio: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxSelectAudio
	@inlinable public static func `default`() -> MuxSelectAudio
	@inlinable public consuming func name(_ name: Swift.String) throws -> MuxSelectAudio
	@inlinable public consuming func codec(_ codec: HangCatalogAudioCodecKind) throws -> MuxSelectAudio
}
public struct MuxTimelineTrack: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func retire(_ drained: ClockInstant?) throws
	@inlinable public func map(_ source: Timestamp) throws -> Timestamp
	@inlinable public func declare() throws -> MuxTimelineTransition
	@inlinable public func end(_ source: Timestamp) throws
	@inlinable public func plan(_ transition: MuxTimelineTransition, _ source: Timestamp, _ delay: RustDuration?) throws
	@inlinable public func gap() throws
	@inlinable public func abandon(_ transition: MuxTimelineTransition) throws
}
public struct MuxTimelineProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxTimelineProducer
	@inlinable public static func new(_ delay: RustDuration) -> MuxTimelineProducer
	@inlinable public consuming func reset(_ delay: RustDuration) throws -> MuxTimelineProducer
	@inlinable public func consume() throws -> MuxTimelineConsumer
	@inlinable public func track(_ kind: MuxTimelineKind) throws -> MuxTimelineTrack
	@inlinable public func video(_ timestamp: Timestamp, _ now: ClockInstant) throws -> Swift.Bool
	@inlinable public func videoRestarted() throws
	@inlinable public func audio(_ audio: MuxTimelineAudio, _ now: ClockInstant) throws -> Swift.Bool
	@inlinable public func audioRestarted() throws
	@inlinable public func audioStopped() throws
	@inlinable public func due(_ timestamp: Timestamp) throws -> ClockInstant?
}
public struct MuxTimelineConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> MuxTimelineConsumer
	@inlinable public func playhead(_ now: ClockInstant) -> RustDuration?
	@inlinable public func target() -> RustDuration?
	@inlinable public func due(_ timestamp: Timestamp) -> ClockInstant?
	@inlinable public func present(_ timestamps: [Timestamp], _ now: ClockInstant) -> Swift.UInt?
}
public final class TokioTlsError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public struct TokioTlsIdentity: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioTlsIdentity
	@inlinable public static func generate(_ hostnames: [Swift.String]) throws -> TokioTlsIdentity
	@inlinable public func fingerprint() throws -> Swift.String
}
public struct TokioTlsPeers: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioTlsPeers
	@inlinable public static func `default`() -> TokioTlsPeers
	@inlinable public static func new() -> TokioTlsPeers
	@inlinable public func insert(_ fingerprint: Swift.String) throws
	@inlinable public func remove(_ fingerprint: Swift.String) throws
	@inlinable public func contains(_ fingerprint: Swift.String) throws -> Swift.Bool
}
public struct TokioTlsConnect: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioTlsConnect
	@inlinable public static func `default`() -> TokioTlsConnect
	@inlinable public var rootGet: [PathBuf] { get }
	@inlinable public mutating func rootSet(_ value: [PathBuf]) throws
	@inlinable public var systemRootsGet: Swift.Bool? { get }
	@inlinable public mutating func systemRootsSet(_ value: Swift.Bool?) throws
	@inlinable public var fingerprintGet: [Swift.String] { get }
	@inlinable public mutating func fingerprintSet(_ value: [Swift.String]) throws
	@inlinable public func identityGet() throws -> TokioTlsIdentity?
	@inlinable public mutating func identitySet(_ value: consuming TokioTlsIdentity?) throws
	@inlinable public var certGet: PathBuf? { get }
	@inlinable public mutating func certSet(_ value: PathBuf?) throws
	@inlinable public var keyGet: PathBuf? { get }
	@inlinable public mutating func keySet(_ value: PathBuf?) throws
	@inlinable public var insecureGet: Swift.Bool? { get }
	@inlinable public mutating func insecureSet(_ value: Swift.Bool?) throws
	@inlinable public var hostNameGet: Swift.String? { get }
	@inlinable public mutating func hostNameSet(_ value: Swift.String?) throws
	@inlinable public func deprecated() throws -> MoqTokioCliDeprecated
	@inlinable public func build() throws -> RustlsClientConfig
}
public struct TokioTlsListen: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioTlsListen
	@inlinable public static func `default`() -> TokioTlsListen
	@inlinable public var certGet: [PathBuf] { get }
	@inlinable public mutating func certSet(_ value: [PathBuf]) throws
	@inlinable public var keyGet: [PathBuf] { get }
	@inlinable public mutating func keySet(_ value: [PathBuf]) throws
	@inlinable public var generateGet: [Swift.String] { get }
	@inlinable public mutating func generateSet(_ value: [Swift.String]) throws
	@inlinable public func identityGet() throws -> TokioTlsIdentity?
	@inlinable public mutating func identitySet(_ value: consuming TokioTlsIdentity?) throws
	@inlinable public func peersGet() throws -> TokioTlsPeers?
	@inlinable public mutating func peersSet(_ value: consuming TokioTlsPeers?) throws
	@inlinable public var rootGet: [PathBuf] { get }
	@inlinable public mutating func rootSet(_ value: [PathBuf]) throws
	@inlinable public func load() throws -> TokioTlsServe
	@inlinable public func deprecated() throws -> MoqTokioCliDeprecated
	@inlinable public func loadRoots() throws -> RustlsRootCertStore
	@inlinable public func serverConfig(_ alpn: [Bytes]) throws -> RustlsServerConfig
}
public struct TokioTlsServe: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioTlsServe
	@inlinable public func certificates() throws -> TokioTlsCertificates
	@inlinable public func config(_ alpn: [Bytes]) throws -> RustlsServerConfig
}
public struct TokioTlsPeerIdentity: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TokioTlsPeerIdentity
	@inlinable public static func fromChain(_ chain: consuming UniqueArray<RustlsPkiTypesCertificateDer>) throws -> TokioTlsPeerIdentity
	@inlinable public func chain() -> UniqueArray<RustlsPkiTypesCertificateDer>
	@inlinable public func fingerprint() -> Swift.String?
	@inlinable public func name() -> Swift.String?
	@inlinable public func issuer() -> Swift.String?
	@inlinable public func expiry() -> ClockWall?
}
public struct TokioTlsCertificates: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TokioTlsCertificates
	@inlinable public static func fromPem(_ chain: Bytes) throws -> TokioTlsCertificates
	@inlinable public func fingerprints() -> [Swift.String]
}
public struct TokioTlsKeyLog: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `open`(_ path: PathBuf) throws -> TokioTlsKeyLog
}
public struct TokioClientConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioClientConfig
	@inlinable public static func `default`() -> TokioClientConfig
	@inlinable public func connectGet() throws -> TokioConnectConfig
	@inlinable public mutating func connectSet(_ value: consuming TokioConnectConfig) throws
	@inlinable public func quicGet() throws -> TokioQuicConfig
	@inlinable public mutating func quicSet(_ value: consuming TokioQuicConfig) throws
	@inlinable public consuming func init_() throws -> TokioClient
}
public struct TokioConnectConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioConnectConfig
	@inlinable public static func `default`() -> TokioConnectConfig
	@inlinable public func urlGet() throws -> Url?
	@inlinable public mutating func urlSet(_ value: consuming Url?) throws
	@inlinable public func tokensGet() throws -> UniqueArray<AuthToken>
	@inlinable public mutating func tokensSet(_ value: consuming UniqueArray<AuthToken>) throws
	@inlinable public var bindGet: SocketAddr? { get }
	@inlinable public mutating func bindSet(_ value: SocketAddr?) throws
	@inlinable public var raceGet: TokioDuration? { get }
	@inlinable public mutating func raceSet(_ value: TokioDuration?) throws
	@inlinable public var resolutionDelayGet: TokioDuration? { get }
	@inlinable public mutating func resolutionDelaySet(_ value: TokioDuration?) throws
	@inlinable public var timeoutGet: TokioDuration? { get }
	@inlinable public mutating func timeoutSet(_ value: TokioDuration?) throws
	@inlinable public var versionGet: [Version] { get }
	@inlinable public mutating func versionSet(_ value: [Version]) throws
	@inlinable public var extensionsGet: SetupExtensions { get }
	@inlinable public mutating func extensionsSet(_ value: SetupExtensions) throws
	@inlinable public var implementationGet: Swift.String? { get }
	@inlinable public mutating func implementationSet(_ value: Swift.String?) throws
	@inlinable public func tlsGet() throws -> TokioTlsConnect
	@inlinable public mutating func tlsSet(_ value: consuming TokioTlsConnect) throws
	@inlinable public func backoffGet() throws -> TokioBackoff
	@inlinable public mutating func backoffSet(_ value: consuming TokioBackoff) throws
	@inlinable public func goawayGet() throws -> MoqTokioConnectionGoaway
	@inlinable public mutating func goawaySet(_ value: consuming MoqTokioConnectionGoaway) throws
	@inlinable public func websocketGet() throws -> MoqTokioWebsocketConfig
	@inlinable public mutating func websocketSet(_ value: consuming MoqTokioWebsocketConfig) throws
	@inlinable public func deprecated() throws -> MoqTokioCliDeprecated
	@inlinable public consuming func init_(_ quic: consuming TokioQuicConfig) throws -> TokioClient
	@inlinable public func versions() throws -> Versions
	@inlinable public func resolve() throws -> MoqTokioConnectResolved
}
public struct TokioConnectionMonitor: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TokioConnectionMonitor
	@inlinable public func state() throws -> MoqTokioConnectionObservation
	@inlinable public func presence() throws -> StatsPresence
	@inlinable public func stats() throws -> SessionStats?
	@inlinable public func failure() throws -> TokioError?
	@inlinable public func snapshot() throws -> MoqTokioConnectionSnapshot?
}
public struct TokioListenConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioListenConfig
	@inlinable public static func `default`() -> TokioListenConfig
	@inlinable public var bindGet: TokioListenBind? { get }
	@inlinable public mutating func bindSet(_ value: TokioListenBind?) throws
	@inlinable public func tcpGet() throws -> MoqTokioTcpConfig
	@inlinable public mutating func tcpSet(_ value: consuming MoqTokioTcpConfig) throws
	@inlinable public func unixGet() throws -> MoqTokioUnixConfig
	@inlinable public mutating func unixSet(_ value: consuming MoqTokioUnixConfig) throws
	@inlinable public var versionGet: [Version] { get }
	@inlinable public mutating func versionSet(_ value: [Version]) throws
	@inlinable public var extensionsGet: SetupExtensions { get }
	@inlinable public mutating func extensionsSet(_ value: SetupExtensions) throws
	@inlinable public var implementationGet: Swift.String? { get }
	@inlinable public mutating func implementationSet(_ value: Swift.String?) throws
	@inlinable public var timeoutGet: TokioDuration? { get }
	@inlinable public mutating func timeoutSet(_ value: TokioDuration?) throws
	@inlinable public func tlsGet() throws -> TokioTlsListen
	@inlinable public mutating func tlsSet(_ value: consuming TokioTlsListen) throws
	@inlinable public var preferredV4Get: SocketAddrV4? { get }
	@inlinable public mutating func preferredV4Set(_ value: SocketAddrV4?) throws
	@inlinable public var preferredV6Get: SocketAddrV6? { get }
	@inlinable public mutating func preferredV6Set(_ value: SocketAddrV6?) throws
	@inlinable public func lbGet() throws -> MoqTokioListenLoadBalancer
	@inlinable public mutating func lbSet(_ value: consuming MoqTokioListenLoadBalancer) throws
	@inlinable public func resetGet() throws -> MoqTokioListenReset
	@inlinable public mutating func resetSet(_ value: consuming MoqTokioListenReset) throws
	@inlinable public func deprecated() throws -> MoqTokioCliDeprecated
	@inlinable public func resolvedTimeout() throws -> RustDuration?
	@inlinable public func resolve() throws -> MoqTokioListenResolved
	@inlinable public consuming func init_(_ quic: consuming TokioQuicConfig) throws -> TokioServer
	@inlinable public consuming func initStreams() throws -> TokioServer
	@inlinable public func versions() throws -> Versions
	@inlinable public func hasExplicitBind() throws -> Swift.Bool
}
public struct TokioQuicConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TokioQuicConfig
	@inlinable public static func `default`() -> TokioQuicConfig
	@inlinable public var maxStreamsGet: Swift.UInt64? { get }
	@inlinable public mutating func maxStreamsSet(_ value: Swift.UInt64?) throws
	@inlinable public var gsoGet: Swift.Bool? { get }
	@inlinable public mutating func gsoSet(_ value: Swift.Bool?) throws
	@inlinable public var idleTimeoutGet: TokioDuration? { get }
	@inlinable public mutating func idleTimeoutSet(_ value: TokioDuration?) throws
	@inlinable public var keepAliveGet: TokioDuration? { get }
	@inlinable public mutating func keepAliveSet(_ value: TokioDuration?) throws
	@inlinable public var mtuDiscoveryGet: Swift.Bool? { get }
	@inlinable public mutating func mtuDiscoverySet(_ value: Swift.Bool?) throws
	@inlinable public var congestionControlGet: MoqTokioQuicCongestionControl? { get }
	@inlinable public mutating func congestionControlSet(_ value: MoqTokioQuicCongestionControl?) throws
	@inlinable public func controllerGet() throws -> MoqTokioQuicController?
	@inlinable public mutating func controllerSet(_ value: consuming MoqTokioQuicController?) throws
	@inlinable public var receiveWindowGet: Swift.UInt64? { get }
	@inlinable public mutating func receiveWindowSet(_ value: Swift.UInt64?) throws
	@inlinable public var streamReceiveWindowGet: Swift.UInt64? { get }
	@inlinable public mutating func streamReceiveWindowSet(_ value: Swift.UInt64?) throws
	@inlinable public var sendWindowGet: Swift.UInt64? { get }
	@inlinable public mutating func sendWindowSet(_ value: Swift.UInt64?) throws
	@inlinable public var qlogGet: PathBuf? { get }
	@inlinable public mutating func qlogSet(_ value: PathBuf?) throws
	@inlinable public var keylogGet: PathBuf? { get }
	@inlinable public mutating func keylogSet(_ value: PathBuf?) throws
	@inlinable public func deprecated() throws -> MoqTokioCliDeprecated
	@inlinable public func validate() throws
	@inlinable public func resolve() throws -> MoqTokioQuicResolved
}
public struct TokioServerConfig: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> TokioServerConfig
	@inlinable public func tlsGet() throws -> TokioTlsServe?
	@inlinable public mutating func tlsSet(_ value: consuming TokioTlsServe?) throws
	@inlinable public func activityGet() throws -> ServerActivity
	@inlinable public mutating func activitySet(_ value: consuming ServerActivity) throws
	@inlinable public func listenGet() throws -> TokioListenConfig
	@inlinable public mutating func listenSet(_ value: consuming TokioListenConfig) throws
	@inlinable public func quicGet() throws -> TokioQuicConfig
	@inlinable public mutating func quicSet(_ value: consuming TokioQuicConfig) throws
	@inlinable public func websocketGet() throws -> MoqTokioWebsocketListener?
	@inlinable public mutating func websocketSet(_ value: consuming MoqTokioWebsocketListener?) throws
	@inlinable public func publisherGet() throws -> OriginConsumer?
	@inlinable public mutating func publisherSet(_ value: consuming OriginConsumer?) throws
	@inlinable public func subscriberGet() throws -> OriginProducer?
	@inlinable public mutating func subscriberSet(_ value: consuming OriginProducer?) throws
	@inlinable public func statsGet() throws -> StatsSession
	@inlinable public mutating func statsSet(_ value: consuming StatsSession) throws
	@inlinable public func demuxGet() throws -> MoqTokioQuicDemuxDemux
	@inlinable public mutating func demuxSet(_ value: consuming MoqTokioQuicDemuxDemux) throws
	@inlinable public consuming func init_() throws -> TokioServer
	@inlinable public consuming func initStreams() throws -> TokioServer
}
public struct TokioServerListener: ~Copyable, @unchecked Sendable {
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func websocketLocalAddr() throws -> SocketAddr?
	@inlinable public func tcpLocalAddr() throws -> SocketAddr?
	@inlinable public func certificates() throws -> TokioTlsCertificates
	@inlinable public func acceptHealth() throws -> UniqueArray<MoqTokioAcceptHealth>
}
public struct TokioServerRequest: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> TokioServerRequest
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> TokioServerRequest
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> TokioServerRequest
	@inlinable public consuming func withPeerHop(_ hop: Hop) throws -> TokioServerRequest
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> TokioServerRequest
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> TokioServerRequest
	@inlinable public func quic() throws -> MoqTokioQuicDriverConnectionConnection?
	@inlinable public consuming func withQuicLimits(_ limits: MoqQuicLimits) throws -> TokioServerRequest
	@inlinable public func transport() throws -> TokioTransport
	@inlinable public func url() throws -> Url?
	@inlinable public func authority() throws -> Swift.String?
	@inlinable public func path() throws -> Swift.String
	@inlinable public func query() throws -> Swift.String?
	@inlinable public func remoteAddr() throws -> SocketAddr?
	@inlinable public func localAddr() throws -> SocketAddr?
	@inlinable public func serverName() throws -> Swift.String?
	@inlinable public func alpn() throws -> Swift.String?
	@inlinable public func role() throws -> SetupRole?
	@inlinable public func auth() throws -> AuthHandle
	@inlinable public mutating func requests() throws -> AuthRequests
	@inlinable public func peerHop() throws -> Hop?
	@inlinable public func peerImplementation() throws -> Swift.String?
	@inlinable public func token() throws -> SetupToken?
	@inlinable public func peerIdentity() throws -> TokioTlsPeerIdentity?
}
public struct TransportMemoryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TransportMemoryConfig
	@inlinable public static func `default`() -> TransportMemoryConfig
	@inlinable public var streamBufferGet: Swift.UInt { get }
	@inlinable public mutating func streamBufferSet(_ value: Swift.UInt) throws
	@inlinable public var protocolGet: Swift.String? { get }
	@inlinable public mutating func protocolSet(_ value: Swift.String?) throws
	@inlinable public var maxConcurrentUniStreamsGet: Swift.UInt? { get }
	@inlinable public mutating func maxConcurrentUniStreamsSet(_ value: Swift.UInt?) throws
	@inlinable public var maxConcurrentBiStreamsGet: Swift.UInt? { get }
	@inlinable public mutating func maxConcurrentBiStreamsSet(_ value: Swift.UInt?) throws
}
public struct TransportMemorySession: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TransportMemorySession
}
public struct MuxCatalogHangConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogHangConsumer
}
public struct MuxCatalogHangConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogHangConsumerExtra
}
public struct MuxCatalogHangConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogHangConsumerExt
}
public struct MuxCatalogHangContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ container: borrowing HangCatalogContainer, _ kind: MuxContainerKind) throws -> MuxCatalogHangContainer
}
public struct MuxCatalogHangExtra: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCatalogHangExtra
	@inlinable public static func == (a: borrowing MuxCatalogHangExtra, b: borrowing MuxCatalogHangExtra) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxCatalogHangExtra
	@inlinable public func get(_ name: Swift.String) throws -> Json?
	@inlinable public func iter() throws -> UniqueArray<TupleStringJson>
	@inlinable public func len() throws -> Swift.UInt
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public mutating func set(_ name: Swift.String, _ value: consuming Json) throws
	@inlinable public mutating func remove(_ name: Swift.String) throws -> Json?
}
public struct MuxCatalogMsfConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogMsfConsumer
}
public struct MuxCatalogMsfConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogMsfConsumerExtra
}
public struct MuxCatalogMsfConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogMsfConsumerExt
}
public final class MuxCodecAacError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecAacError
}
public struct MuxCodecAacImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public final class MuxCodecAnnexbError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecAnnexbError
}
public struct MuxCodecAv1Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public mutating func initialize(_ buf: Bytes) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func decode(_ frames: [MuxContainerFrame]) throws
}
public struct MuxCodecAv1Split: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecAv1Split
	@inlinable public static func new() -> MuxCodecAv1Split
	@inlinable public mutating func decode(_ data: Bytes, _ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func flush(_ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func reset() throws
}
public struct MuxCodecFlacImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public struct MuxCodecH264Avc1: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecH264Avc1
	@inlinable public static func new() -> MuxCodecH264Avc1
	@inlinable public func avcc() throws -> Bytes?
	@inlinable public mutating func transform(_ payload: Bytes) throws -> Bytes?
}
public final class MuxCodecH264Error: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecH264Error
}
public struct MuxCodecH264Avcc: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCodecH264Avcc
	@inlinable public var profileGet: Swift.UInt8 { get }
	@inlinable public mutating func profileSet(_ value: Swift.UInt8) throws
	@inlinable public var constraintsGet: Swift.UInt8 { get }
	@inlinable public mutating func constraintsSet(_ value: Swift.UInt8) throws
	@inlinable public var levelGet: Swift.UInt8 { get }
	@inlinable public mutating func levelSet(_ value: Swift.UInt8) throws
	@inlinable public var lengthSizeGet: Swift.UInt { get }
	@inlinable public mutating func lengthSizeSet(_ value: Swift.UInt) throws
	@inlinable public var spsGet: [Bytes] { get }
	@inlinable public mutating func spsSet(_ value: [Bytes]) throws
	@inlinable public var ppsGet: [Bytes] { get }
	@inlinable public mutating func ppsSet(_ value: [Bytes]) throws
	@inlinable public var codedWidthGet: Swift.UInt32? { get }
	@inlinable public mutating func codedWidthSet(_ value: Swift.UInt32?) throws
	@inlinable public var codedHeightGet: Swift.UInt32? { get }
	@inlinable public mutating func codedHeightSet(_ value: Swift.UInt32?) throws
	@inlinable public static func parse(_ avcc: Bytes) throws -> MuxCodecH264Avcc
}
public struct MuxCodecH264Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public mutating func initialize(_ buf: Bytes) throws
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func track() throws -> TrackProducer
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func observeReorder(_ reorder: Timestamp) throws
	@inlinable public mutating func decode(_ frames: [MuxContainerFrame]) throws
}
public struct MuxCodecH264Split: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecH264Split
	@inlinable public static func new() -> MuxCodecH264Split
	@inlinable public mutating func decode(_ data: Bytes, _ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func flush(_ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func reset() throws
}
public struct MuxCodecH265Hvc1: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecH265Hvc1
	@inlinable public static func new() -> MuxCodecH265Hvc1
	@inlinable public func hvcc() throws -> Bytes?
	@inlinable public mutating func transform(_ payload: Bytes) throws -> Bytes?
}
public final class MuxCodecH265Error: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecH265Error
}
public struct MuxCodecH265Hvcc: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCodecH265Hvcc
	@inlinable public var lengthSizeGet: Swift.UInt { get }
	@inlinable public mutating func lengthSizeSet(_ value: Swift.UInt) throws
	@inlinable public var vpsGet: [Bytes] { get }
	@inlinable public mutating func vpsSet(_ value: [Bytes]) throws
	@inlinable public var spsGet: [Bytes] { get }
	@inlinable public mutating func spsSet(_ value: [Bytes]) throws
	@inlinable public var ppsGet: [Bytes] { get }
	@inlinable public mutating func ppsSet(_ value: [Bytes]) throws
	@inlinable public static func parse(_ hvcc: Bytes) throws -> MuxCodecH265Hvcc
}
public struct MuxCodecH265Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public mutating func initialize(_ buf: Bytes) throws
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func track() throws -> TrackProducer
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func observeReorder(_ reorder: Timestamp) throws
	@inlinable public mutating func decode(_ frames: [MuxContainerFrame]) throws
}
public struct MuxCodecH265Split: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecH265Split
	@inlinable public static func new() -> MuxCodecH265Split
	@inlinable public mutating func decode(_ data: Bytes, _ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func flush(_ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func reset() throws
}
public struct MuxCodecMp3Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public final class MuxCodecOpusConfig: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCodecOpusConfig
	@inlinable public static func == (a: MuxCodecOpusConfig, b: MuxCodecOpusConfig) throws -> Swift.Bool
	@inlinable public var sampleRateGet: Swift.UInt32 { get }
	@inlinable public func sampleRateSet(_ value: Swift.UInt32) throws
	@inlinable public var channelCountGet: Swift.UInt32 { get }
	@inlinable public func channelCountSet(_ value: Swift.UInt32) throws
	@inlinable public var preSkipGet: Swift.UInt16 { get }
	@inlinable public func preSkipSet(_ value: Swift.UInt16) throws
	@inlinable public var outputGainGet: Swift.Int16 { get }
	@inlinable public func outputGainSet(_ value: Swift.Int16) throws
	@inlinable public var mappingGet: MuxCodecOpusMapping? { get }
	@inlinable public func mappingSet(_ value: MuxCodecOpusMapping?) throws
	@inlinable public static func new(_ sampleRate: Swift.UInt32, _ channelCount: Swift.UInt32) -> MuxCodecOpusConfig
	@inlinable public func withPreSkip(_ preSkip: Swift.UInt16) throws -> MuxCodecOpusConfig
	@inlinable public static func parse(_ buf: inout Bytes) throws -> MuxCodecOpusConfig
	@inlinable public func encode() throws -> Bytes
}
public struct MuxCodecOpusImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public struct MuxCodecVp8Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public mutating func initialize(_ buf: Bytes) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
}
public struct MuxCodecVp9Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public mutating func initialize(_ buf: Bytes) throws
	@inlinable public mutating func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut(_ end: Timestamp?) throws
	@inlinable public mutating func discontinuity() throws
	@inlinable public mutating func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
}
public struct MuxContainerFlvImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerFlvImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerFlvImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerFlvImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ trak: consuming Mp4AtomTrak) throws -> MuxContainerFmp4Wire
	@inlinable public static func fromInit(_ initData: Bytes) throws -> MuxContainerFmp4Wire
	@inlinable public func trak() throws -> Mp4AtomTrak
}
public final class MuxContainerFmp4Error: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxContainerFmp4Error
}
public struct MuxContainerFmp4Fragmenter: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func push(_ frame: MuxContainerFrame) throws -> [MuxContainerFmp4Fragment]
	@inlinable public mutating func pushGroup(_ frame: MuxContainerFrame) throws -> [MuxContainerFmp4Fragment]
	@inlinable public consuming func flush() throws -> MuxContainerFmp4Fragment?
}
public struct MuxContainerFmp4Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerFmp4Import
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4Import
	@inlinable public consuming func withPassthrough(_ passthrough: Swift.Bool) throws -> MuxContainerFmp4Import
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4ImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerFmp4ImportExtra
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4ImportExtra
	@inlinable public consuming func withPassthrough(_ passthrough: Swift.Bool) throws -> MuxContainerFmp4ImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4ImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerFmp4ImportExt
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4ImportExt
	@inlinable public consuming func withPassthrough(_ passthrough: Swift.Bool) throws -> MuxContainerFmp4ImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4Muxer: ~Copyable, @unchecked Sendable {
	@inlinable public static func video(_ config: borrowing HangCatalogVideoConfig) throws -> MuxContainerFmp4Muxer
	@inlinable public static func audio(_ config: borrowing HangCatalogAudioConfig) throws -> MuxContainerFmp4Muxer
	@inlinable public func timescale() throws -> Timescale
	@inlinable public consuming func withTimescale(_ timescale: Timescale) throws -> MuxContainerFmp4Muxer
	@inlinable public consuming func withFramerate(_ framerate: Swift.Double?) throws -> MuxContainerFmp4Muxer
	@inlinable public func init_() throws -> Bytes?
	@inlinable public func fragment(_ sequence: Swift.UInt32, _ frames: [MuxContainerFrame], _ end: Timestamp?) throws -> Bytes
	@inlinable public func fragmenter(_ config: MuxContainerFmp4FragmentConfig) throws -> MuxContainerFmp4Fragmenter
}
public struct MuxContainerMkvImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerMkvImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerMkvImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerMkvImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsExt: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsExt
	@inlinable public static func == (a: borrowing MuxContainerTsExt, b: borrowing MuxContainerTsExt) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsExt
	@inlinable public func mpegtsGet() throws -> MuxContainerTsMpegts
	@inlinable public mutating func mpegtsSet(_ value: consuming MuxContainerTsMpegts) throws
}
public struct MuxContainerTsMpegts: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsMpegts
	@inlinable public static func == (a: borrowing MuxContainerTsMpegts, b: borrowing MuxContainerTsMpegts) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsMpegts
	@inlinable public func tracksGet() throws -> UniqueArray<TupleStringMuxContainerTsTrack>
	@inlinable public mutating func tracksSet(_ value: consuming UniqueArray<TupleStringMuxContainerTsTrack>) throws
	@inlinable public var programDescriptorsGet: [MuxContainerTsDescriptor] { get }
	@inlinable public mutating func programDescriptorsSet(_ value: [MuxContainerTsDescriptor]) throws
	@inlinable public func programGet() throws -> MuxContainerTsProgram?
	@inlinable public mutating func programSet(_ value: consuming MuxContainerTsProgram?) throws
	@inlinable public func siGet() throws -> UniqueArray<TupleU16VecTupleU8MuxContainerTsSiEntry>
	@inlinable public mutating func siSet(_ value: consuming UniqueArray<TupleU16VecTupleU8MuxContainerTsSiEntry>) throws
	@inlinable public var muxRateGet: Swift.UInt64? { get }
	@inlinable public mutating func muxRateSet(_ value: Swift.UInt64?) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct MuxContainerTsPesPacket: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsPesPacket
	@inlinable public static func == (a: borrowing MuxContainerTsPesPacket, b: borrowing MuxContainerTsPesPacket) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsPesPacket
	@inlinable public var streamTypeGet: Swift.UInt8 { get }
	@inlinable public mutating func streamTypeSet(_ value: Swift.UInt8) throws
	@inlinable public static func new(_ streamType: Swift.UInt8) -> MuxContainerTsPesPacket
}
public struct MuxContainerTsProgram: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsProgram
	@inlinable public static func == (a: borrowing MuxContainerTsProgram, b: borrowing MuxContainerTsProgram) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsProgram
	@inlinable public var transportStreamIdGet: Swift.UInt16 { get }
	@inlinable public mutating func transportStreamIdSet(_ value: Swift.UInt16) throws
	@inlinable public var programNumberGet: Swift.UInt16 { get }
	@inlinable public mutating func programNumberSet(_ value: Swift.UInt16) throws
	@inlinable public var pmtPidGet: Swift.UInt16 { get }
	@inlinable public mutating func pmtPidSet(_ value: Swift.UInt16) throws
}
public struct MuxContainerTsSiEntry: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsSiEntry
	@inlinable public static func == (a: borrowing MuxContainerTsSiEntry, b: borrowing MuxContainerTsSiEntry) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsSiEntry
	@inlinable public var trackGet: Swift.String { get }
	@inlinable public mutating func trackSet(_ value: Swift.String) throws
	@inlinable public var intervalGet: RustDuration? { get }
	@inlinable public mutating func intervalSet(_ value: RustDuration?) throws
}
public struct MuxContainerTsTrack: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsTrack
	@inlinable public static func == (a: borrowing MuxContainerTsTrack, b: borrowing MuxContainerTsTrack) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsTrack
	@inlinable public var pidGet: Swift.UInt16 { get }
	@inlinable public mutating func pidSet(_ value: Swift.UInt16) throws
	@inlinable public var descriptorsGet: [MuxContainerTsDescriptor] { get }
	@inlinable public mutating func descriptorsSet(_ value: [MuxContainerTsDescriptor]) throws
	@inlinable public func verbatimGet() throws -> MuxContainerTsVerbatim?
	@inlinable public mutating func verbatimSet(_ value: consuming MuxContainerTsVerbatim?) throws
	@inlinable public func pesPacketGet() throws -> MuxContainerTsPesPacket?
	@inlinable public mutating func pesPacketSet(_ value: consuming MuxContainerTsPesPacket?) throws
	@inlinable public static func new(_ pid: Swift.UInt16) -> MuxContainerTsTrack
}
public struct MuxContainerTsVerbatim: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsVerbatim
	@inlinable public static func == (a: borrowing MuxContainerTsVerbatim, b: borrowing MuxContainerTsVerbatim) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsVerbatim
	@inlinable public var streamTypeGet: Swift.UInt8 { get }
	@inlinable public mutating func streamTypeSet(_ value: Swift.UInt8) throws
	@inlinable public var framingGet: MuxContainerTsFraming { get }
	@inlinable public mutating func framingSet(_ value: MuxContainerTsFraming) throws
	@inlinable public static func new(_ streamType: Swift.UInt8, _ framing: MuxContainerTsFraming) -> MuxContainerTsVerbatim
}
public struct MuxContainerTsExportConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumerExtra) throws -> MuxContainerTsExportConsumerExtra
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumerExtra
	@inlinable public consuming func withDelay(_ delay: RustDuration) throws -> MuxContainerTsExportConsumerExtra
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumerExtra) throws -> MuxContainerTsExportConsumerExtra
}
public struct MuxContainerTsExportConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumerExt) throws -> MuxContainerTsExportConsumerExt
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumerExt
	@inlinable public consuming func withDelay(_ delay: RustDuration) throws -> MuxContainerTsExportConsumerExt
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumerExt) throws -> MuxContainerTsExportConsumerExt
}
public struct MuxContainerTsExportConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumer) throws -> MuxContainerTsExportConsumer
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumer
	@inlinable public consuming func withDelay(_ delay: RustDuration) throws -> MuxContainerTsExportConsumer
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumer) throws -> MuxContainerTsExportConsumer
}
public struct MuxContainerTsFollower: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumer) throws -> MuxContainerTsFollower
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollower
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollower
	@inlinable public func export() throws -> MuxContainerTsExportConsumer
}
public struct MuxContainerTsFollowerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumerExtra) throws -> MuxContainerTsFollowerExtra
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollowerExtra
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollowerExtra
	@inlinable public func export() throws -> MuxContainerTsExportConsumerExtra
}
public struct MuxContainerTsFollowerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumerExt) throws -> MuxContainerTsFollowerExt
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollowerExt
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollowerExt
	@inlinable public func export() throws -> MuxContainerTsExportConsumerExt
}
public struct MuxContainerTsPassthrough: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerTsPassthrough
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthrough
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerTsPassthroughExtra
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthroughExtra
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerTsPassthroughExt
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthroughExt
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPrograms: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ origin: consuming OriginProducer, _ name: consuming Path, _ config: consuming MuxCatalogConfig) throws -> MuxContainerTsPrograms
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public mutating func finish() throws -> UniqueArray<BroadcastProducer>
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsRelease: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func withDelay(_ delay: RustDuration) throws -> MuxContainerTsRelease
	@inlinable public func dropped() throws -> Swift.UInt64
	@inlinable public func drift() throws -> Swift.Double?
	@inlinable public func outOfTolerance() throws -> Swift.UInt64
	@inlinable public func stats() throws -> MuxContainerTsStatsRelease
}
public struct MuxContainerTsImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerTsImport
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerTsImportExtra
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerTsImportExt
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public mutating func finish() throws
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public final class MuxCodecAacAdtsHeader: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxCodecAacAdtsHeader
	@inlinable public var objectTypeGet: Swift.UInt8 { get }
	@inlinable public func objectTypeSet(_ value: Swift.UInt8) throws
	@inlinable public var sampleRateGet: Swift.UInt32 { get }
	@inlinable public func sampleRateSet(_ value: Swift.UInt32) throws
	@inlinable public var channelConfigGet: Swift.UInt8 { get }
	@inlinable public func channelConfigSet(_ value: Swift.UInt8) throws
	@inlinable public var blocksGet: Swift.UInt8 { get }
	@inlinable public func blocksSet(_ value: Swift.UInt8) throws
	@inlinable public static func parse(_ data: Bytes) throws -> MuxCodecAacAdtsHeader
	@inlinable public func payload(_ packet: Bytes) throws -> Bytes
}
public struct MuxContainerTsStatsReport: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsStatsReport
	@inlinable public static func == (a: borrowing MuxContainerTsStatsReport, b: borrowing MuxContainerTsStatsReport) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsReport
	@inlinable public func mpegtsGet() throws -> MuxContainerTsStatsSnapshot?
	@inlinable public mutating func mpegtsSet(_ value: consuming MuxContainerTsStatsSnapshot?) throws
	@inlinable public func releaseGet() throws -> MuxContainerTsStatsRelease?
	@inlinable public mutating func releaseSet(_ value: consuming MuxContainerTsStatsRelease?) throws
}
public struct MuxContainerTsStatsSnapshot: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsStatsSnapshot
	@inlinable public static func == (a: borrowing MuxContainerTsStatsSnapshot, b: borrowing MuxContainerTsStatsSnapshot) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsSnapshot
	@inlinable public func streamsGet() throws -> UniqueArray<TupleU16MuxContainerTsStatsStream>
	@inlinable public mutating func streamsSet(_ value: consuming UniqueArray<TupleU16MuxContainerTsStatsStream>) throws
	@inlinable public var crcErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func crcErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var tsSyncLossGet: Swift.UInt64 { get }
	@inlinable public mutating func tsSyncLossSet(_ value: Swift.UInt64) throws
	@inlinable public var syncByteErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func syncByteErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var patErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func patErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var continuityCountErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func continuityCountErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var pmtErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func pmtErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var transportErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func transportErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var pcrRepetitionErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func pcrRepetitionErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var pcrDiscontinuityIndicatorErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func pcrDiscontinuityIndicatorErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var ptsErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func ptsErrorSet(_ value: Swift.UInt64) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct MuxContainerTsStatsStream: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsStatsStream
	@inlinable public static func == (a: borrowing MuxContainerTsStatsStream, b: borrowing MuxContainerTsStatsStream) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsStream
	@inlinable public var trackGet: Swift.String { get }
	@inlinable public mutating func trackSet(_ value: Swift.String) throws
	@inlinable public var classGet: MuxContainerTsStatsClass { get }
	@inlinable public mutating func classSet(_ value: MuxContainerTsStatsClass) throws
	@inlinable public var unitsGet: Swift.UInt64 { get }
	@inlinable public mutating func unitsSet(_ value: Swift.UInt64) throws
	@inlinable public var quietGet: RustDuration? { get }
	@inlinable public mutating func quietSet(_ value: RustDuration?) throws
	@inlinable public var resyncsGet: Swift.UInt64 { get }
	@inlinable public mutating func resyncsSet(_ value: Swift.UInt64) throws
	@inlinable public var discardedGet: Swift.UInt64 { get }
	@inlinable public mutating func discardedSet(_ value: Swift.UInt64) throws
	@inlinable public var unconfirmedGet: Swift.UInt64 { get }
	@inlinable public mutating func unconfirmedSet(_ value: Swift.UInt64) throws
	@inlinable public var damagedGet: Swift.UInt64 { get }
	@inlinable public mutating func damagedSet(_ value: Swift.UInt64) throws
	@inlinable public var unplacedGet: Swift.UInt64 { get }
	@inlinable public mutating func unplacedSet(_ value: Swift.UInt64) throws
	@inlinable public var refusedGet: Swift.UInt64 { get }
	@inlinable public mutating func refusedSet(_ value: Swift.UInt64) throws
	@inlinable public var continuityCountErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func continuityCountErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var transportErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func transportErrorSet(_ value: Swift.UInt64) throws
	@inlinable public var ptsErrorGet: Swift.UInt64 { get }
	@inlinable public mutating func ptsErrorSet(_ value: Swift.UInt64) throws
}
public struct MuxContainerTsStatsExport: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsStatsExport
	@inlinable public static func == (a: borrowing MuxContainerTsStatsExport, b: borrowing MuxContainerTsStatsExport) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsExport
	@inlinable public func streamsGet() throws -> UniqueArray<TupleU16MuxContainerTsStatsStream>
	@inlinable public mutating func streamsSet(_ value: consuming UniqueArray<TupleU16MuxContainerTsStatsStream>) throws
	@inlinable public func releaseGet() throws -> MuxContainerTsStatsRelease
	@inlinable public mutating func releaseSet(_ value: consuming MuxContainerTsStatsRelease) throws
}
public struct MuxContainerTsStatsRelease: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsStatsRelease
	@inlinable public static func == (a: borrowing MuxContainerTsStatsRelease, b: borrowing MuxContainerTsStatsRelease) throws -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsRelease
	@inlinable public var droppedGet: Swift.UInt64 { get }
	@inlinable public mutating func droppedSet(_ value: Swift.UInt64) throws
	@inlinable public var trimmedGet: Swift.UInt64 { get }
	@inlinable public mutating func trimmedSet(_ value: Swift.UInt64) throws
	@inlinable public var gapsGet: Swift.UInt64 { get }
	@inlinable public mutating func gapsSet(_ value: Swift.UInt64) throws
	@inlinable public var marginGet: Swift.Int64? { get }
	@inlinable public mutating func marginSet(_ value: Swift.Int64?) throws
	@inlinable public var offsetGet: Swift.Int64? { get }
	@inlinable public mutating func offsetSet(_ value: Swift.Int64?) throws
	@inlinable public var driftGet: Swift.Double? { get }
	@inlinable public mutating func driftSet(_ value: Swift.Double?) throws
	@inlinable public var outOfToleranceGet: Swift.UInt64 { get }
	@inlinable public mutating func outOfToleranceSet(_ value: Swift.UInt64) throws
}
public struct MuxContainerTsStatsLog: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxContainerTsStatsLog
	@inlinable public static var INTERVAL: RustDuration { get }
	@inlinable public mutating func sample(_ latest: consuming MuxContainerTsStatsSnapshot) throws
	@inlinable public func finish(_ latest: borrowing MuxContainerTsStatsSnapshot) throws
}
public struct MoqTokioAddrs: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioAddrs
	@inlinable public static func == (a: borrowing MoqTokioAddrs, b: borrowing MoqTokioAddrs) throws -> Swift.Bool
	@inlinable public static func new(_ addr: consuming MoqTokioConnectAddr) throws -> MoqTokioAddrs
	@inlinable public consuming func or(_ addr: consuming MoqTokioConnectAddr) throws -> MoqTokioAddrs
	@inlinable public static func collect(_ urls: consuming UniqueArray<MoqTokioConnectAddr>) throws -> MoqTokioAddrs?
	@inlinable public func asSlice() throws -> UniqueArray<MoqTokioConnectAddr>
}
public struct MoqTokioCliDeprecated: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioCliDeprecated
	@inlinable public static func `default`() -> MoqTokioCliDeprecated
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public mutating func extend(_ other: consuming MoqTokioCliDeprecated) throws
	@inlinable public mutating func flag(_ old: Swift.String, _ env: Swift.String?, _ new: Swift.String) throws
	@inlinable public mutating func changed(_ old: Swift.String, _ env: Swift.String?, _ new: Swift.String, _ note: Swift.String) throws
	@inlinable public mutating func removed(_ old: Swift.String, _ env: Swift.String?, _ note: Swift.String) throws
	@inlinable public mutating func toml(_ old: Swift.String, _ new: Swift.String, _ note: Swift.String?) throws
}
public struct Mp4AtomCodec: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomCodec
	@inlinable public static func == (a: borrowing Mp4AtomCodec, b: borrowing Mp4AtomCodec) throws -> Swift.Bool
	@inlinable public func visual() throws -> Mp4AtomVisual?
	@inlinable public func audio() throws -> Mp4AtomAudio?
	@inlinable public func metadata() throws -> Mp4AtomMetaData?
}
public struct MoqTokioTcpTlsAcceptor: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> MoqTokioTcpTlsAcceptor
	@inlinable public func config() -> RustlsServerConfig
	@inlinable public func offered(_ alpns: [Bytes]) -> Swift.Bool
}
public struct MoqTokioAcceptHealth: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioAcceptHealth
	@inlinable public static func new(_ listener: Swift.String) -> MoqTokioAcceptHealth
	@inlinable public func listener() -> Swift.String
	@inlinable public func accepted()
	@inlinable public func failures(_ failure: MoqTokioAcceptFailure) -> Swift.UInt64
	@inlinable public func stalled() -> RustDuration?
}
public struct RustlsClientConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsClientConfig
	@inlinable public var alpnProtocolsGet: [Bytes] { get }
	@inlinable public mutating func alpnProtocolsSet(_ value: [Bytes]) throws
	@inlinable public var checkSelectedAlpnGet: Swift.Bool { get }
	@inlinable public mutating func checkSelectedAlpnSet(_ value: Swift.Bool) throws
	@inlinable public func resumptionGet() throws -> RustlsClientResumption
	@inlinable public mutating func resumptionSet(_ value: consuming RustlsClientResumption) throws
	@inlinable public var maxFragmentSizeGet: Swift.UInt? { get }
	@inlinable public mutating func maxFragmentSizeSet(_ value: Swift.UInt?) throws
	@inlinable public func clientAuthCertResolverGet() throws -> RustlsResolvesClientCert
	@inlinable public mutating func clientAuthCertResolverSet(_ value: consuming RustlsResolvesClientCert) throws
	@inlinable public var enableSniGet: Swift.Bool { get }
	@inlinable public mutating func enableSniSet(_ value: Swift.Bool) throws
	@inlinable public func keyLogGet() throws -> RustlsKeyLog
	@inlinable public mutating func keyLogSet(_ value: consuming RustlsKeyLog) throws
	@inlinable public var enableSecretExtractionGet: Swift.Bool { get }
	@inlinable public mutating func enableSecretExtractionSet(_ value: Swift.Bool) throws
	@inlinable public var enableEarlyDataGet: Swift.Bool { get }
	@inlinable public mutating func enableEarlyDataSet(_ value: Swift.Bool) throws
	@inlinable public var requireEmsGet: Swift.Bool { get }
	@inlinable public mutating func requireEmsSet(_ value: Swift.Bool) throws
	@inlinable public func timeProviderGet() throws -> RustlsTimeProvider
	@inlinable public mutating func timeProviderSet(_ value: consuming RustlsTimeProvider) throws
	@inlinable public func certDecompressorsGet() throws -> UniqueArray<RustlsCertDecompressor>
	@inlinable public mutating func certDecompressorsSet(_ value: consuming UniqueArray<RustlsCertDecompressor>) throws
	@inlinable public func certCompressorsGet() throws -> UniqueArray<RustlsCertCompressor>
	@inlinable public mutating func certCompressorsSet(_ value: consuming UniqueArray<RustlsCertCompressor>) throws
	@inlinable public func certCompressionCacheGet() throws -> RustlsCompressionCache
	@inlinable public mutating func certCompressionCacheSet(_ value: consuming RustlsCompressionCache) throws
	@inlinable public var sendTicketRequestGet: RustlsClientTicketRequest? { get }
	@inlinable public mutating func sendTicketRequestSet(_ value: RustlsClientTicketRequest?) throws
	@inlinable public func fips() throws -> Swift.Bool
	@inlinable public func cryptoProvider() throws -> RustlsCryptoProvider
	@inlinable public mutating func dangerous() throws -> RustlsClientDangerDangerousClientConfig
}
public struct RustlsRootCertStore: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsRootCertStore
	@inlinable public func rootsGet() throws -> UniqueArray<RustlsPkiTypesTrustAnchor>
	@inlinable public mutating func rootsSet(_ value: consuming UniqueArray<RustlsPkiTypesTrustAnchor>) throws
	@inlinable public static func empty() -> RustlsRootCertStore
	@inlinable public mutating func addParsableCertificates(_ derCerts: consuming UniqueArray<RustlsPkiTypesCertificateDer>) throws -> (Swift.UInt, Swift.UInt)
	@inlinable public mutating func add(_ der: consuming RustlsPkiTypesCertificateDer) throws
	@inlinable public func subjects() throws -> UniqueArray<RustlsDistinguishedName>
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func len() throws -> Swift.UInt
}
public struct RustlsPkiTypesCertificateDer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsPkiTypesCertificateDer
	@inlinable public static func == (a: borrowing RustlsPkiTypesCertificateDer, b: borrowing RustlsPkiTypesCertificateDer) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func fromSlice(_ bytes: Bytes) -> RustlsPkiTypesCertificateDer
	@inlinable public consuming func intoOwned() throws -> RustlsPkiTypesCertificateDer
}
public struct MoqTokioConnectResolved: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectResolved
	@inlinable public var bindGet: SocketAddr { get }
	@inlinable public mutating func bindSet(_ value: SocketAddr) throws
	@inlinable public var raceGet: RustDuration { get }
	@inlinable public mutating func raceSet(_ value: RustDuration) throws
	@inlinable public var resolutionDelayGet: RustDuration { get }
	@inlinable public mutating func resolutionDelaySet(_ value: RustDuration) throws
	@inlinable public var timeoutGet: RustDuration { get }
	@inlinable public mutating func timeoutSet(_ value: RustDuration) throws
}
public struct MoqTokioConnectionGoaway: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectionGoaway
	@inlinable public static func `default`() -> MoqTokioConnectionGoaway
	@inlinable public var redirectGet: MoqTokioRedirect { get }
	@inlinable public mutating func redirectSet(_ value: MoqTokioRedirect) throws
	@inlinable public var handoverGet: TokioDuration? { get }
	@inlinable public mutating func handoverSet(_ value: TokioDuration?) throws
}
public struct MoqTokioWebsocketConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioWebsocketConfig
	@inlinable public static func `default`() -> MoqTokioWebsocketConfig
	@inlinable public var enabledGet: Swift.Bool? { get }
	@inlinable public mutating func enabledSet(_ value: Swift.Bool?) throws
	@inlinable public var delayGet: TokioDuration? { get }
	@inlinable public mutating func delaySet(_ value: TokioDuration?) throws
	@inlinable public func resolve() throws -> MoqTokioWebsocketResolved
}
public struct MoqTokioConnectionObservation: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectionObservation
	@inlinable public var revisionGet: Swift.UInt64 { get }
	@inlinable public mutating func revisionSet(_ value: Swift.UInt64) throws
	@inlinable public var phaseGet: MoqTokioConnectionPhase { get }
	@inlinable public mutating func phaseSet(_ value: MoqTokioConnectionPhase) throws
	@inlinable public var connectsGet: Swift.UInt64 { get }
	@inlinable public mutating func connectsSet(_ value: Swift.UInt64) throws
	@inlinable public var lastConnectedGet: ClockInstant? { get }
	@inlinable public mutating func lastConnectedSet(_ value: ClockInstant?) throws
	@inlinable public var transportGet: TokioTransport? { get }
	@inlinable public mutating func transportSet(_ value: TokioTransport?) throws
	@inlinable public var failureGet: TokioError? { get }
	@inlinable public mutating func failureSet(_ value: TokioError?) throws
	@inlinable public var versionGet: Version? { get }
	@inlinable public mutating func versionSet(_ value: Version?) throws
}
public struct MoqTokioConnectionSnapshot: ~Copyable, @unchecked Sendable {
	@inlinable public var statsGet: SessionStats { get }
	@inlinable public mutating func statsSet(_ value: SessionStats) throws
	@inlinable public var versionGet: Version { get }
	@inlinable public mutating func versionSet(_ value: Version) throws
}
public struct MoqTokioListenResolved: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioListenResolved
	@inlinable public var bindGet: TokioListenBind? { get }
	@inlinable public mutating func bindSet(_ value: TokioListenBind?) throws
	@inlinable public func versionsGet() throws -> Versions
	@inlinable public mutating func versionsSet(_ value: consuming Versions) throws
	@inlinable public var extensionsGet: SetupExtensions { get }
	@inlinable public mutating func extensionsSet(_ value: SetupExtensions) throws
	@inlinable public var implementationGet: Swift.String? { get }
	@inlinable public mutating func implementationSet(_ value: Swift.String?) throws
	@inlinable public var timeoutGet: RustDuration? { get }
	@inlinable public mutating func timeoutSet(_ value: RustDuration?) throws
	@inlinable public func tlsGet() throws -> TokioTlsListen
	@inlinable public mutating func tlsSet(_ value: consuming TokioTlsListen) throws
	@inlinable public var preferredV4Get: SocketAddrV4? { get }
	@inlinable public mutating func preferredV4Set(_ value: SocketAddrV4?) throws
	@inlinable public var preferredV6Get: SocketAddrV6? { get }
	@inlinable public mutating func preferredV6Set(_ value: SocketAddrV6?) throws
	@inlinable public func lbGet() throws -> MoqTokioListenLoadBalancer
	@inlinable public mutating func lbSet(_ value: consuming MoqTokioListenLoadBalancer) throws
	@inlinable public var resetSecretGet: PathBuf? { get }
	@inlinable public mutating func resetSecretSet(_ value: PathBuf?) throws
}
public struct MoqTokioTcpConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioTcpConfig
	@inlinable public static func `default`() -> MoqTokioTcpConfig
	@inlinable public var bindGet: SocketAddr? { get }
	@inlinable public mutating func bindSet(_ value: SocketAddr?) throws
	@inlinable public var tlsGet: Swift.Bool? { get }
	@inlinable public mutating func tlsSet(_ value: Swift.Bool?) throws
}
public struct MoqTokioUnixConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioUnixConfig
	@inlinable public static func `default`() -> MoqTokioUnixConfig
	@inlinable public var bindGet: PathBuf? { get }
	@inlinable public mutating func bindSet(_ value: PathBuf?) throws
	@inlinable public func allowGet() throws -> MoqTokioUnixAllow
	@inlinable public mutating func allowSet(_ value: consuming MoqTokioUnixAllow) throws
}
public struct MoqTokioListenLoadBalancer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioListenLoadBalancer
	@inlinable public static func `default`() -> MoqTokioListenLoadBalancer
	@inlinable public func idGet() throws -> MoqTokioQuicServerId?
	@inlinable public mutating func idSet(_ value: consuming MoqTokioQuicServerId?) throws
	@inlinable public var nonceGet: Swift.UInt? { get }
	@inlinable public mutating func nonceSet(_ value: Swift.UInt?) throws
	@inlinable public func forwardGet() throws -> MoqTokioListenForward
	@inlinable public mutating func forwardSet(_ value: consuming MoqTokioListenForward) throws
}
public struct MoqTokioListenReset: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioListenReset
	@inlinable public static func `default`() -> MoqTokioListenReset
	@inlinable public var secretGet: PathBuf? { get }
	@inlinable public mutating func secretSet(_ value: PathBuf?) throws
}
public struct MoqTokioQuicResolved: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicResolved
	@inlinable public static func `default`() -> MoqTokioQuicResolved
	@inlinable public var maxStreamsGet: Swift.UInt64 { get }
	@inlinable public mutating func maxStreamsSet(_ value: Swift.UInt64) throws
	@inlinable public var gsoGet: Swift.Bool? { get }
	@inlinable public mutating func gsoSet(_ value: Swift.Bool?) throws
	@inlinable public var idleTimeoutGet: RustDuration { get }
	@inlinable public mutating func idleTimeoutSet(_ value: RustDuration) throws
	@inlinable public var keepAliveGet: RustDuration? { get }
	@inlinable public mutating func keepAliveSet(_ value: RustDuration?) throws
	@inlinable public var mtuDiscoveryGet: Swift.Bool { get }
	@inlinable public mutating func mtuDiscoverySet(_ value: Swift.Bool) throws
	@inlinable public var congestionControlGet: MoqTokioQuicCongestionControl? { get }
	@inlinable public mutating func congestionControlSet(_ value: MoqTokioQuicCongestionControl?) throws
	@inlinable public func controllerGet() throws -> MoqTokioQuicController?
	@inlinable public mutating func controllerSet(_ value: consuming MoqTokioQuicController?) throws
	@inlinable public var receiveWindowGet: Swift.UInt64 { get }
	@inlinable public mutating func receiveWindowSet(_ value: Swift.UInt64) throws
	@inlinable public var streamReceiveWindowGet: Swift.UInt64 { get }
	@inlinable public mutating func streamReceiveWindowSet(_ value: Swift.UInt64) throws
	@inlinable public var sendWindowGet: Swift.UInt64 { get }
	@inlinable public mutating func sendWindowSet(_ value: Swift.UInt64) throws
	@inlinable public var qlogGet: PathBuf? { get }
	@inlinable public mutating func qlogSet(_ value: PathBuf?) throws
	@inlinable public var keylogGet: PathBuf? { get }
	@inlinable public mutating func keylogSet(_ value: PathBuf?) throws
	@inlinable public func hasController() throws -> Swift.Bool
	@inlinable public func keepAliveInterval() throws -> RustDuration
}
public struct MoqTokioQuicController: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicController
	@inlinable public func _0_get() throws -> MoqCongestionFactory
	@inlinable public mutating func _0_set(_ value: consuming MoqCongestionFactory) throws
}
public struct MoqTokioWebsocketListener: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func withProtocols(_ protocols: [Swift.String]) throws -> MoqTokioWebsocketListener
	@inlinable public consuming func withQuic(_ quic: borrowing MoqTokioQuicResolved) throws -> MoqTokioWebsocketListener
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func acceptHealth() throws -> MoqTokioAcceptHealth
}
public struct MoqTokioQuicDemuxDemux: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicDemuxDemux
	@inlinable public static func `default`() -> MoqTokioQuicDemuxDemux
	@inlinable public func counters() throws -> MoqSockDemuxCounters
	@inlinable public func webrtc() throws -> MoqTokioQuicDemuxWebRtc
}
public struct MoqTokioQuicDriverConnectionConnection: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioQuicDriverConnectionConnection
	@inlinable public func sendGroup(_ priority: Swift.Int64) -> MoqTokioQuicDriverSendGroup
	@inlinable public func closeReason() -> MoqQuicError?
	@inlinable public func close(_ errorCode: MoqQuicVarInt, _ reason: Swift.String)
	@inlinable public func sendDatagram(_ data: Bytes, _ options: MoqTokioQuicDriverSendOptions) throws
	@inlinable public func maxDatagramSize() -> Swift.UInt?
	@inlinable public func datagramSendBufferSpace() -> Swift.UInt
	@inlinable public func remoteAddress() -> SocketAddr
	@inlinable public func rtt() -> RustDuration
	@inlinable public func stats() -> MoqQuicConnectionStats
	@inlinable public func handshakeData() -> MoqQuicCryptoHandshakeData?
	@inlinable public func peerIdentity() -> UniqueArray<RustlsPkiTypesCertificateDer>?
	@inlinable public func stableId() -> Swift.UInt
	@inlinable public func setLimits(_ limits: MoqQuicLimits) throws
}
public final class MoqQuicLimits: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqQuicLimits
	@inlinable public static func == (a: MoqQuicLimits, b: MoqQuicLimits) throws -> Swift.Bool
	@inlinable public static func `default`() -> MoqQuicLimits
	@inlinable public var maxStreamsGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func maxStreamsSet(_ value: NonZero<Swift.UInt64>?) throws
	@inlinable public var receiveWindowGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func receiveWindowSet(_ value: NonZero<Swift.UInt64>?) throws
	@inlinable public var streamReceiveWindowGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func streamReceiveWindowSet(_ value: NonZero<Swift.UInt64>?) throws
	@inlinable public var sendWindowGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func sendWindowSet(_ value: NonZero<Swift.UInt64>?) throws
	@inlinable public var receiveRateGet: MoqQuicRate? { get }
	@inlinable public func receiveRateSet(_ value: MoqQuicRate?) throws
	@inlinable public var sendRateGet: MoqQuicRate? { get }
	@inlinable public func sendRateSet(_ value: MoqQuicRate?) throws
	@inlinable public func validate() throws
}
public struct Mp4AtomTrak: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomTrak
	@inlinable public static func == (a: borrowing Mp4AtomTrak, b: borrowing Mp4AtomTrak) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTrak
	@inlinable public var tkhdGet: Mp4AtomTkhd { get }
	@inlinable public mutating func tkhdSet(_ value: Mp4AtomTkhd) throws
	@inlinable public var edtsGet: Mp4AtomEdts? { get }
	@inlinable public mutating func edtsSet(_ value: Mp4AtomEdts?) throws
	@inlinable public func metaGet() throws -> Mp4AtomMeta?
	@inlinable public mutating func metaSet(_ value: consuming Mp4AtomMeta?) throws
	@inlinable public func mdiaGet() throws -> Mp4AtomMdia
	@inlinable public mutating func mdiaSet(_ value: consuming Mp4AtomMdia) throws
	@inlinable public var sencGet: Mp4AtomSenc? { get }
	@inlinable public mutating func sencSet(_ value: Mp4AtomSenc?) throws
	@inlinable public var trefGet: Mp4AtomTref? { get }
	@inlinable public mutating func trefSet(_ value: Mp4AtomTref?) throws
	@inlinable public func udtaGet() throws -> Mp4AtomUdta?
	@inlinable public mutating func udtaSet(_ value: consuming Mp4AtomUdta?) throws
}
public struct MoqTokioConnectAddr: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectAddr
	@inlinable public static func == (a: borrowing MoqTokioConnectAddr, b: borrowing MoqTokioConnectAddr) throws -> Swift.Bool
	@inlinable public static func new(_ url: consuming Url) throws -> MoqTokioConnectAddr
	@inlinable public static func pinned(_ url: consuming Url, _ addrs: [SocketAddr]) throws -> MoqTokioConnectAddr
	@inlinable public func url() throws -> Url
	@inlinable public func addresses() throws -> [SocketAddr]?
}
public struct Mp4AtomAvc1: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomAvc1
	@inlinable public static func == (a: borrowing Mp4AtomAvc1, b: borrowing Mp4AtomAvc1) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAvc1
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var avccGet: Mp4AtomAvcc { get }
	@inlinable public mutating func avccSet(_ value: Mp4AtomAvcc) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var taicGet: Mp4AtomTaic? { get }
	@inlinable public mutating func taicSet(_ value: Mp4AtomTaic?) throws
	@inlinable public var fielGet: Mp4AtomFiel? { get }
	@inlinable public mutating func fielSet(_ value: Mp4AtomFiel?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomAvc3: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomAvc3
	@inlinable public static func == (a: borrowing Mp4AtomAvc3, b: borrowing Mp4AtomAvc3) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAvc3
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var avccGet: Mp4AtomAvcc { get }
	@inlinable public mutating func avccSet(_ value: Mp4AtomAvcc) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var taicGet: Mp4AtomTaic? { get }
	@inlinable public mutating func taicSet(_ value: Mp4AtomTaic?) throws
	@inlinable public var fielGet: Mp4AtomFiel? { get }
	@inlinable public mutating func fielSet(_ value: Mp4AtomFiel?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomJpeg: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomJpeg
	@inlinable public static func == (a: borrowing Mp4AtomJpeg, b: borrowing Mp4AtomJpeg) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomJpeg
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var fielGet: Mp4AtomFiel? { get }
	@inlinable public mutating func fielSet(_ value: Mp4AtomFiel?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomHev1: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomHev1
	@inlinable public static func == (a: borrowing Mp4AtomHev1, b: borrowing Mp4AtomHev1) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHev1
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var hvccGet: Mp4AtomHvcc { get }
	@inlinable public mutating func hvccSet(_ value: Mp4AtomHvcc) throws
	@inlinable public var lhvcGet: Mp4AtomLhvc? { get }
	@inlinable public mutating func lhvcSet(_ value: Mp4AtomLhvc?) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var taicGet: Mp4AtomTaic? { get }
	@inlinable public mutating func taicSet(_ value: Mp4AtomTaic?) throws
	@inlinable public var fielGet: Mp4AtomFiel? { get }
	@inlinable public mutating func fielSet(_ value: Mp4AtomFiel?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomHvc1: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomHvc1
	@inlinable public static func == (a: borrowing Mp4AtomHvc1, b: borrowing Mp4AtomHvc1) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomHvc1
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var hvccGet: Mp4AtomHvcc { get }
	@inlinable public mutating func hvccSet(_ value: Mp4AtomHvcc) throws
	@inlinable public var lhvcGet: Mp4AtomLhvc? { get }
	@inlinable public mutating func lhvcSet(_ value: Mp4AtomLhvc?) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var taicGet: Mp4AtomTaic? { get }
	@inlinable public mutating func taicSet(_ value: Mp4AtomTaic?) throws
	@inlinable public var fielGet: Mp4AtomFiel? { get }
	@inlinable public mutating func fielSet(_ value: Mp4AtomFiel?) throws
	@inlinable public var ccstGet: Mp4AtomCcst? { get }
	@inlinable public mutating func ccstSet(_ value: Mp4AtomCcst?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomVp08: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomVp08
	@inlinable public static func == (a: borrowing Mp4AtomVp08, b: borrowing Mp4AtomVp08) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVp08
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var vpccGet: Mp4AtomVpcC { get }
	@inlinable public mutating func vpccSet(_ value: Mp4AtomVpcC) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomVp09: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomVp09
	@inlinable public static func == (a: borrowing Mp4AtomVp09, b: borrowing Mp4AtomVp09) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVp09
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var vpccGet: Mp4AtomVpcC { get }
	@inlinable public mutating func vpccSet(_ value: Mp4AtomVpcC) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomAv01: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomAv01
	@inlinable public static func == (a: borrowing Mp4AtomAv01, b: borrowing Mp4AtomAv01) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomAv01
	@inlinable public func visualGet() throws -> Mp4AtomVisual
	@inlinable public mutating func visualSet(_ value: consuming Mp4AtomVisual) throws
	@inlinable public var av1cGet: Mp4AtomAv1c { get }
	@inlinable public mutating func av1cSet(_ value: Mp4AtomAv1c) throws
	@inlinable public var btrtGet: Mp4AtomBtrt? { get }
	@inlinable public mutating func btrtSet(_ value: Mp4AtomBtrt?) throws
	@inlinable public var ccstGet: Mp4AtomCcst? { get }
	@inlinable public mutating func ccstSet(_ value: Mp4AtomCcst?) throws
	@inlinable public var colrGet: Mp4AtomColr? { get }
	@inlinable public mutating func colrSet(_ value: Mp4AtomColr?) throws
	@inlinable public var paspGet: Mp4AtomPasp? { get }
	@inlinable public mutating func paspSet(_ value: Mp4AtomPasp?) throws
	@inlinable public var taicGet: Mp4AtomTaic? { get }
	@inlinable public mutating func taicSet(_ value: Mp4AtomTaic?) throws
	@inlinable public func spatialGet() throws -> Mp4AtomSpatial?
	@inlinable public mutating func spatialSet(_ value: consuming Mp4AtomSpatial?) throws
}
public struct Mp4AtomTx3g: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomTx3g
	@inlinable public static func == (a: borrowing Mp4AtomTx3g, b: borrowing Mp4AtomTx3g) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTx3g
	@inlinable public var dataReferenceIndexGet: Swift.UInt16 { get }
	@inlinable public mutating func dataReferenceIndexSet(_ value: Swift.UInt16) throws
	@inlinable public var displayFlagsGet: Swift.UInt32 { get }
	@inlinable public mutating func displayFlagsSet(_ value: Swift.UInt32) throws
	@inlinable public var horizontalJustificationGet: Swift.Int8 { get }
	@inlinable public mutating func horizontalJustificationSet(_ value: Swift.Int8) throws
	@inlinable public var verticalJustificationGet: Swift.Int8 { get }
	@inlinable public mutating func verticalJustificationSet(_ value: Swift.Int8) throws
	@inlinable public var bgColorRgbaGet: Mp4AtomRgbaColor { get }
	@inlinable public mutating func bgColorRgbaSet(_ value: Mp4AtomRgbaColor) throws
	@inlinable public var styleRecordGet: ArrayU8x12 { get }
	@inlinable public mutating func styleRecordSet(_ value: ArrayU8x12) throws
	@inlinable public var ftabGet: Mp4AtomFtab? { get }
	@inlinable public mutating func ftabSet(_ value: Mp4AtomFtab?) throws
	@inlinable public var dispGet: Mp4AtomDisp? { get }
	@inlinable public mutating func dispSet(_ value: Mp4AtomDisp?) throws
}
public struct RustlsPkiTypesTrustAnchor: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsPkiTypesTrustAnchor
	@inlinable public static func == (a: borrowing RustlsPkiTypesTrustAnchor, b: borrowing RustlsPkiTypesTrustAnchor) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public func subjectGet() throws -> RustlsPkiTypesDer
	@inlinable public mutating func subjectSet(_ value: consuming RustlsPkiTypesDer) throws
	@inlinable public func subjectPublicKeyInfoGet() throws -> RustlsPkiTypesDer
	@inlinable public mutating func subjectPublicKeyInfoSet(_ value: consuming RustlsPkiTypesDer) throws
	@inlinable public func nameConstraintsGet() throws -> RustlsPkiTypesDer?
	@inlinable public mutating func nameConstraintsSet(_ value: consuming RustlsPkiTypesDer?) throws
	@inlinable public func toOwned() throws -> RustlsPkiTypesTrustAnchor
}
public struct MoqTokioQuicServerId: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicServerId
	@inlinable public static func == (a: borrowing MoqTokioQuicServerId, b: borrowing MoqTokioQuicServerId) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
}
public struct MoqTokioListenForward: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioListenForward
	@inlinable public static func `default`() -> MoqTokioListenForward
	@inlinable public var bindGet: SocketAddr? { get }
	@inlinable public mutating func bindSet(_ value: SocketAddr?) throws
	@inlinable public var keyGet: PathBuf? { get }
	@inlinable public mutating func keySet(_ value: PathBuf?) throws
	@inlinable public var peersGet: PathBuf? { get }
	@inlinable public mutating func peersSet(_ value: PathBuf?) throws
	@inlinable public func configured() throws -> Swift.Bool
}
public struct Mp4AtomMeta: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomMeta
	@inlinable public static func == (a: borrowing Mp4AtomMeta, b: borrowing Mp4AtomMeta) throws -> Swift.Bool
	@inlinable public var hdlrGet: Mp4AtomHdlr { get }
	@inlinable public mutating func hdlrSet(_ value: Mp4AtomHdlr) throws
	@inlinable public func itemsGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func itemsSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
	@inlinable public func getPitm() throws -> Mp4AtomPitm?
	@inlinable public func getDinf() throws -> Mp4AtomDinf?
	@inlinable public func getIloc() throws -> Mp4AtomIloc?
	@inlinable public func getIinf() throws -> Mp4AtomIinf?
	@inlinable public func getIprp() throws -> Mp4AtomIprp?
	@inlinable public func getIref() throws -> Mp4AtomIref?
	@inlinable public func getIdat() throws -> Mp4AtomIdat?
	@inlinable public func getIlst() throws -> Mp4AtomIlst?
	@inlinable public func getKeys() throws -> Mp4AtomKeys?
	@inlinable public mutating func getMutPitm() throws -> Mp4AtomPitm?
	@inlinable public mutating func getMutDinf() throws -> Mp4AtomDinf?
	@inlinable public mutating func getMutIloc() throws -> Mp4AtomIloc?
	@inlinable public mutating func getMutIinf() throws -> Mp4AtomIinf?
	@inlinable public mutating func getMutIprp() throws -> Mp4AtomIprp?
	@inlinable public mutating func getMutIref() throws -> Mp4AtomIref?
	@inlinable public mutating func getMutIdat() throws -> Mp4AtomIdat?
	@inlinable public mutating func getMutIlst() throws -> Mp4AtomIlst?
	@inlinable public mutating func getMutKeys() throws -> Mp4AtomKeys?
	@inlinable public mutating func pushPitm(_ atom: Mp4AtomPitm) throws
	@inlinable public mutating func pushDinf(_ atom: Mp4AtomDinf) throws
	@inlinable public mutating func pushIloc(_ atom: Mp4AtomIloc) throws
	@inlinable public mutating func pushIinf(_ atom: Mp4AtomIinf) throws
	@inlinable public mutating func pushIprp(_ atom: consuming Mp4AtomIprp) throws
	@inlinable public mutating func pushIref(_ atom: Mp4AtomIref) throws
	@inlinable public mutating func pushIdat(_ atom: Mp4AtomIdat) throws
	@inlinable public mutating func pushIlst(_ atom: Mp4AtomIlst) throws
	@inlinable public mutating func pushKeys(_ atom: Mp4AtomKeys) throws
	@inlinable public mutating func removePitm() throws -> Mp4AtomPitm?
	@inlinable public mutating func removeDinf() throws -> Mp4AtomDinf?
	@inlinable public mutating func removeIloc() throws -> Mp4AtomIloc?
	@inlinable public mutating func removeIinf() throws -> Mp4AtomIinf?
	@inlinable public mutating func removeIprp() throws -> Mp4AtomIprp?
	@inlinable public mutating func removeIref() throws -> Mp4AtomIref?
	@inlinable public mutating func removeIdat() throws -> Mp4AtomIdat?
	@inlinable public mutating func removeIlst() throws -> Mp4AtomIlst?
	@inlinable public mutating func removeKeys() throws -> Mp4AtomKeys?
}
public struct Mp4AtomMdia: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomMdia
	@inlinable public static func == (a: borrowing Mp4AtomMdia, b: borrowing Mp4AtomMdia) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMdia
	@inlinable public var mdhdGet: Mp4AtomMdhd { get }
	@inlinable public mutating func mdhdSet(_ value: Mp4AtomMdhd) throws
	@inlinable public var hdlrGet: Mp4AtomHdlr { get }
	@inlinable public mutating func hdlrSet(_ value: Mp4AtomHdlr) throws
	@inlinable public func minfGet() throws -> Mp4AtomMinf
	@inlinable public mutating func minfSet(_ value: consuming Mp4AtomMinf) throws
}
public struct Mp4AtomUdta: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomUdta
	@inlinable public static func == (a: borrowing Mp4AtomUdta, b: borrowing Mp4AtomUdta) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomUdta
	@inlinable public var chplGet: Mp4AtomChpl? { get }
	@inlinable public mutating func chplSet(_ value: Mp4AtomChpl?) throws
	@inlinable public var cprtGet: [Mp4AtomCprt] { get }
	@inlinable public mutating func cprtSet(_ value: [Mp4AtomCprt]) throws
	@inlinable public var kindGet: [Mp4AtomKind] { get }
	@inlinable public mutating func kindSet(_ value: [Mp4AtomKind]) throws
	@inlinable public func metaGet() throws -> Mp4AtomMeta?
	@inlinable public mutating func metaSet(_ value: consuming Mp4AtomMeta?) throws
	@inlinable public var rtngGet: [Mp4AtomRtng] { get }
	@inlinable public mutating func rtngSet(_ value: [Mp4AtomRtng]) throws
}
public struct Mp4AtomVisual: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomVisual
	@inlinable public static func == (a: borrowing Mp4AtomVisual, b: borrowing Mp4AtomVisual) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVisual
	@inlinable public var dataReferenceIndexGet: Swift.UInt16 { get }
	@inlinable public mutating func dataReferenceIndexSet(_ value: Swift.UInt16) throws
	@inlinable public var widthGet: Swift.UInt16 { get }
	@inlinable public mutating func widthSet(_ value: Swift.UInt16) throws
	@inlinable public var heightGet: Swift.UInt16 { get }
	@inlinable public mutating func heightSet(_ value: Swift.UInt16) throws
	@inlinable public var horizresolutionGet: Mp4AtomFixedPointU16 { get }
	@inlinable public mutating func horizresolutionSet(_ value: Mp4AtomFixedPointU16) throws
	@inlinable public var vertresolutionGet: Mp4AtomFixedPointU16 { get }
	@inlinable public mutating func vertresolutionSet(_ value: Mp4AtomFixedPointU16) throws
	@inlinable public var frameCountGet: Swift.UInt16 { get }
	@inlinable public mutating func frameCountSet(_ value: Swift.UInt16) throws
	@inlinable public func compressorGet() throws -> Mp4AtomCompressor
	@inlinable public mutating func compressorSet(_ value: consuming Mp4AtomCompressor) throws
	@inlinable public var depthGet: Swift.UInt16 { get }
	@inlinable public mutating func depthSet(_ value: Swift.UInt16) throws
}
public struct RustlsClientDangerDangerousClientConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
}
public struct RustlsClientResumption: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsClientResumption
	@inlinable public static func `default`() -> RustlsClientResumption
	@inlinable public static func inMemorySessions(_ num: Swift.UInt) -> RustlsClientResumption
	@inlinable public static func disabled() -> RustlsClientResumption
	@inlinable public consuming func tls12Resumption(_ tls12: RustlsClientTls12Resumption) throws -> RustlsClientResumption
}
public struct RustlsDistinguishedName: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> RustlsDistinguishedName
	@inlinable public static func inSequence(_ bytes: Bytes) -> RustlsDistinguishedName
}
public struct MoqTokioWebsocketResolved: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioWebsocketResolved
	@inlinable public var enabledGet: Swift.Bool { get }
	@inlinable public mutating func enabledSet(_ value: Swift.Bool) throws
	@inlinable public var delayGet: RustDuration { get }
	@inlinable public mutating func delaySet(_ value: RustDuration) throws
}
public struct MoqTokioUnixAllow: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioUnixAllow
	@inlinable public static func `default`() -> MoqTokioUnixAllow
	@inlinable public var uidGet: [Swift.UInt32] { get }
	@inlinable public mutating func uidSet(_ value: [Swift.UInt32]) throws
	@inlinable public var gidGet: [Swift.UInt32] { get }
	@inlinable public mutating func gidSet(_ value: [Swift.UInt32]) throws
	@inlinable public var pidGet: [Swift.Int32] { get }
	@inlinable public mutating func pidSet(_ value: [Swift.Int32]) throws
}
public final class MoqTokioWebsocketError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MoqSockDemuxCounters: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqSockDemuxCounters
	@inlinable public static func `default`() -> MoqSockDemuxCounters
	@inlinable public func discard(_ reason: MoqSockDemuxDiscard)
	@inlinable public func get(_ reason: MoqSockDemuxDiscard) -> Swift.UInt64
}
public struct MoqTokioQuicDemuxWebRtc: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func pin(_ remote: SocketAddr) -> MoqSockDemuxPin
}
public final class MoqTokioQuicDriverSendOptions: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicDriverSendOptions
	@inlinable public static func `default`() -> MoqTokioQuicDriverSendOptions
	@inlinable public var priorityGet: Swift.Int64 { get }
	@inlinable public func prioritySet(_ value: Swift.Int64) throws
}
public struct MoqTokioQuicDriverSendStream: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public consuming func reset(_ errorCode: MoqQuicVarInt) throws
	@inlinable public consuming func resetAt(_ offset: MoqTokioQuicDriverSendOffset, _ errorCode: MoqQuicVarInt) throws
	@inlinable public func offset() throws -> MoqTokioQuicDriverSendOffset
	@inlinable public mutating func setPriority(_ priority: Swift.Int64) throws
	@inlinable public func priority() throws -> Swift.Int64
	@inlinable public mutating func setGroup() throws
	@inlinable public mutating func setGroup(group: borrowing MoqTokioQuicDriverSendGroup) throws
	@inlinable public func id() throws -> MoqQuicStreamId
}
public struct MoqTokioQuicDriverSendGroup: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioQuicDriverSendGroup
	@inlinable public func setPriority(_ priority: Swift.Int64)
	@inlinable public func priority() -> Swift.Int64
}
public struct MoqTokioQuicDriverRecvStream: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public mutating func setPriority(_ priority: Swift.Int64) throws
	@inlinable public consuming func stop(_ errorCode: MoqQuicVarInt) throws
	@inlinable public func id() throws -> MoqQuicStreamId
}
public final class MoqTokioQuicDriverSendError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioQuicDriverSendError
	@inlinable public static func == (a: MoqTokioQuicDriverSendError, b: MoqTokioQuicDriverSendError) -> Swift.Bool
}
public final class MoqQuicConnectionStats: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqQuicConnectionStats
	@inlinable public static func `default`() -> MoqQuicConnectionStats
	@inlinable public var udpTxGet: MoqQuicUdpStats { get }
	@inlinable public func udpTxSet(_ value: MoqQuicUdpStats) throws
	@inlinable public var udpRxGet: MoqQuicUdpStats { get }
	@inlinable public func udpRxSet(_ value: MoqQuicUdpStats) throws
	@inlinable public var frameTxGet: MoqQuicFrameStats { get }
	@inlinable public func frameTxSet(_ value: MoqQuicFrameStats) throws
	@inlinable public var frameRxGet: MoqQuicFrameStats { get }
	@inlinable public func frameRxSet(_ value: MoqQuicFrameStats) throws
	@inlinable public var pathGet: MoqQuicPathStats { get }
	@inlinable public func pathSet(_ value: MoqQuicPathStats) throws
	@inlinable public var creditGet: MoqQuicCreditStats { get }
	@inlinable public func creditSet(_ value: MoqQuicCreditStats) throws
}
public struct MoqQuicCryptoHandshakeData: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqQuicCryptoHandshakeData
	@inlinable public static func == (a: borrowing MoqQuicCryptoHandshakeData, b: borrowing MoqQuicCryptoHandshakeData) throws -> Swift.Bool
	@inlinable public var alpnGet: Bytes? { get }
	@inlinable public mutating func alpnSet(_ value: Bytes?) throws
	@inlinable public var serverNameGet: Swift.String? { get }
	@inlinable public mutating func serverNameSet(_ value: Swift.String?) throws
}
public struct Mp4AtomSpatial: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomSpatial
	@inlinable public static func == (a: borrowing Mp4AtomSpatial, b: borrowing Mp4AtomSpatial) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSpatial
	@inlinable public var st3dGet: Mp4AtomSt3d? { get }
	@inlinable public mutating func st3dSet(_ value: Mp4AtomSt3d?) throws
	@inlinable public func sv3dGet() throws -> Mp4AtomSv3d?
	@inlinable public mutating func sv3dSet(_ value: consuming Mp4AtomSv3d?) throws
	@inlinable public func vexuGet() throws -> Mp4AtomVexu?
	@inlinable public mutating func vexuSet(_ value: consuming Mp4AtomVexu?) throws
	@inlinable public var hfovGet: Mp4AtomHfov? { get }
	@inlinable public mutating func hfovSet(_ value: Mp4AtomHfov?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomAny: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomAny
	@inlinable public static func == (a: borrowing Mp4AtomAny, b: borrowing Mp4AtomAny) throws -> Swift.Bool
	@inlinable public func kind() throws -> Mp4AtomFourCc
}
public struct Mp4AtomMinf: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomMinf
	@inlinable public static func == (a: borrowing Mp4AtomMinf, b: borrowing Mp4AtomMinf) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMinf
	@inlinable public var vmhdGet: Mp4AtomVmhd? { get }
	@inlinable public mutating func vmhdSet(_ value: Mp4AtomVmhd?) throws
	@inlinable public var smhdGet: Mp4AtomSmhd? { get }
	@inlinable public mutating func smhdSet(_ value: Mp4AtomSmhd?) throws
	@inlinable public func nmhdGet() throws -> Mp4AtomNmhd?
	@inlinable public mutating func nmhdSet(_ value: consuming Mp4AtomNmhd?) throws
	@inlinable public func sthdGet() throws -> Mp4AtomSthd?
	@inlinable public mutating func sthdSet(_ value: consuming Mp4AtomSthd?) throws
	@inlinable public var hmhdGet: Mp4AtomHmhd? { get }
	@inlinable public mutating func hmhdSet(_ value: Mp4AtomHmhd?) throws
	@inlinable public var dinfGet: Mp4AtomDinf { get }
	@inlinable public mutating func dinfSet(_ value: Mp4AtomDinf) throws
	@inlinable public func stblGet() throws -> Mp4AtomStbl
	@inlinable public mutating func stblSet(_ value: consuming Mp4AtomStbl) throws
}
public struct Mp4AtomCompressor: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomCompressor
	@inlinable public static func == (a: borrowing Mp4AtomCompressor, b: borrowing Mp4AtomCompressor) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCompressor
}
public struct RustlsPkiTypesDer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> RustlsPkiTypesDer
	@inlinable public static func == (a: borrowing RustlsPkiTypesDer, b: borrowing RustlsPkiTypesDer) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func fromSlice(_ der: Bytes) -> RustlsPkiTypesDer
}
public struct MoqTokioQuicDemuxDatagram: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicDemuxDatagram
	@inlinable public var remoteGet: SocketAddr { get }
	@inlinable public mutating func remoteSet(_ value: SocketAddr) throws
	@inlinable public var localGet: IpAddr? { get }
	@inlinable public mutating func localSet(_ value: IpAddr?) throws
	@inlinable public var dataGet: Bytes { get }
	@inlinable public mutating func dataSet(_ value: Bytes) throws
}
public struct MoqSockDemuxPin: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
}
public final class MoqTokioQuicDriverSendOffset: @unchecked Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioQuicDriverSendOffset
	@inlinable public static func == (a: MoqTokioQuicDriverSendOffset, b: MoqTokioQuicDriverSendOffset) -> Swift.Bool
}
public final class MoqTokioQuicDriverRecvError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioQuicDriverRecvError
	@inlinable public static func == (a: MoqTokioQuicDriverRecvError, b: MoqTokioQuicDriverRecvError) -> Swift.Bool
}
public final class MoqQuicFrameStats: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqQuicFrameStats
	@inlinable public static func `default`() -> MoqQuicFrameStats
	@inlinable public var acksGet: Swift.UInt64 { get }
	@inlinable public func acksSet(_ value: Swift.UInt64) throws
	@inlinable public var ackFrequencyGet: Swift.UInt64 { get }
	@inlinable public func ackFrequencySet(_ value: Swift.UInt64) throws
	@inlinable public var cryptoGet: Swift.UInt64 { get }
	@inlinable public func cryptoSet(_ value: Swift.UInt64) throws
	@inlinable public var connectionCloseGet: Swift.UInt64 { get }
	@inlinable public func connectionCloseSet(_ value: Swift.UInt64) throws
	@inlinable public var dataBlockedGet: Swift.UInt64 { get }
	@inlinable public func dataBlockedSet(_ value: Swift.UInt64) throws
	@inlinable public var datagramGet: Swift.UInt64 { get }
	@inlinable public func datagramSet(_ value: Swift.UInt64) throws
	@inlinable public var handshakeDoneGet: Swift.UInt8 { get }
	@inlinable public func handshakeDoneSet(_ value: Swift.UInt8) throws
	@inlinable public var immediateAckGet: Swift.UInt64 { get }
	@inlinable public func immediateAckSet(_ value: Swift.UInt64) throws
	@inlinable public var maxDataGet: Swift.UInt64 { get }
	@inlinable public func maxDataSet(_ value: Swift.UInt64) throws
	@inlinable public var maxStreamDataGet: Swift.UInt64 { get }
	@inlinable public func maxStreamDataSet(_ value: Swift.UInt64) throws
	@inlinable public var maxStreamsBidiGet: Swift.UInt64 { get }
	@inlinable public func maxStreamsBidiSet(_ value: Swift.UInt64) throws
	@inlinable public var maxStreamsUniGet: Swift.UInt64 { get }
	@inlinable public func maxStreamsUniSet(_ value: Swift.UInt64) throws
	@inlinable public var newConnectionIdGet: Swift.UInt64 { get }
	@inlinable public func newConnectionIdSet(_ value: Swift.UInt64) throws
	@inlinable public var newTokenGet: Swift.UInt64 { get }
	@inlinable public func newTokenSet(_ value: Swift.UInt64) throws
	@inlinable public var pathChallengeGet: Swift.UInt64 { get }
	@inlinable public func pathChallengeSet(_ value: Swift.UInt64) throws
	@inlinable public var pathResponseGet: Swift.UInt64 { get }
	@inlinable public func pathResponseSet(_ value: Swift.UInt64) throws
	@inlinable public var pingGet: Swift.UInt64 { get }
	@inlinable public func pingSet(_ value: Swift.UInt64) throws
	@inlinable public var resetStreamGet: Swift.UInt64 { get }
	@inlinable public func resetStreamSet(_ value: Swift.UInt64) throws
	@inlinable public var retireConnectionIdGet: Swift.UInt64 { get }
	@inlinable public func retireConnectionIdSet(_ value: Swift.UInt64) throws
	@inlinable public var streamDataBlockedGet: Swift.UInt64 { get }
	@inlinable public func streamDataBlockedSet(_ value: Swift.UInt64) throws
	@inlinable public var streamsBlockedBidiGet: Swift.UInt64 { get }
	@inlinable public func streamsBlockedBidiSet(_ value: Swift.UInt64) throws
	@inlinable public var streamsBlockedUniGet: Swift.UInt64 { get }
	@inlinable public func streamsBlockedUniSet(_ value: Swift.UInt64) throws
	@inlinable public var stopSendingGet: Swift.UInt64 { get }
	@inlinable public func stopSendingSet(_ value: Swift.UInt64) throws
	@inlinable public var streamGet: Swift.UInt64 { get }
	@inlinable public func streamSet(_ value: Swift.UInt64) throws
}
public final class MoqQuicPathStats: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqQuicPathStats
	@inlinable public static func `default`() -> MoqQuicPathStats
	@inlinable public var rttGet: RustDuration { get }
	@inlinable public func rttSet(_ value: RustDuration) throws
	@inlinable public var minRttGet: RustDuration { get }
	@inlinable public func minRttSet(_ value: RustDuration) throws
	@inlinable public var cwndGet: Swift.UInt64 { get }
	@inlinable public func cwndSet(_ value: Swift.UInt64) throws
	@inlinable public var bandwidthEstimateGet: Swift.UInt64? { get }
	@inlinable public func bandwidthEstimateSet(_ value: Swift.UInt64?) throws
	@inlinable public var congestionEventsGet: Swift.UInt64 { get }
	@inlinable public func congestionEventsSet(_ value: Swift.UInt64) throws
	@inlinable public var spuriousCongestionEventsGet: Swift.UInt64 { get }
	@inlinable public func spuriousCongestionEventsSet(_ value: Swift.UInt64) throws
	@inlinable public var lostPacketsGet: Swift.UInt64 { get }
	@inlinable public func lostPacketsSet(_ value: Swift.UInt64) throws
	@inlinable public var lostBytesGet: Swift.UInt64 { get }
	@inlinable public func lostBytesSet(_ value: Swift.UInt64) throws
	@inlinable public var sentPacketsGet: Swift.UInt64 { get }
	@inlinable public func sentPacketsSet(_ value: Swift.UInt64) throws
	@inlinable public var sentPlpmtudProbesGet: Swift.UInt64 { get }
	@inlinable public func sentPlpmtudProbesSet(_ value: Swift.UInt64) throws
	@inlinable public var lostPlpmtudProbesGet: Swift.UInt64 { get }
	@inlinable public func lostPlpmtudProbesSet(_ value: Swift.UInt64) throws
	@inlinable public var sentBandwidthProbesGet: Swift.UInt64 { get }
	@inlinable public func sentBandwidthProbesSet(_ value: Swift.UInt64) throws
	@inlinable public var sentBandwidthProbeBytesGet: Swift.UInt64 { get }
	@inlinable public func sentBandwidthProbeBytesSet(_ value: Swift.UInt64) throws
	@inlinable public var blackHolesDetectedGet: Swift.UInt64 { get }
	@inlinable public func blackHolesDetectedSet(_ value: Swift.UInt64) throws
	@inlinable public var currentMtuGet: Swift.UInt16 { get }
	@inlinable public func currentMtuSet(_ value: Swift.UInt16) throws
}
public struct Mp4AtomSv3d: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomSv3d
	@inlinable public static func == (a: borrowing Mp4AtomSv3d, b: borrowing Mp4AtomSv3d) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSv3d
	@inlinable public var svhdGet: Mp4AtomSvhd? { get }
	@inlinable public mutating func svhdSet(_ value: Mp4AtomSvhd?) throws
	@inlinable public func projGet() throws -> Mp4AtomProj?
	@inlinable public mutating func projSet(_ value: consuming Mp4AtomProj?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomVexu: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomVexu
	@inlinable public static func == (a: borrowing Mp4AtomVexu, b: borrowing Mp4AtomVexu) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVexu
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public func projGet() throws -> Mp4AtomVexuProj?
	@inlinable public mutating func projSet(_ value: consuming Mp4AtomVexuProj?) throws
	@inlinable public func eyesGet() throws -> Mp4AtomEyes?
	@inlinable public mutating func eyesSet(_ value: consuming Mp4AtomEyes?) throws
	@inlinable public func packGet() throws -> Mp4AtomPack?
	@inlinable public mutating func packSet(_ value: consuming Mp4AtomPack?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomIprp: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomIprp
	@inlinable public static func == (a: borrowing Mp4AtomIprp, b: borrowing Mp4AtomIprp) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIprp
	@inlinable public func ipcoGet() throws -> Mp4AtomIpco
	@inlinable public mutating func ipcoSet(_ value: consuming Mp4AtomIpco) throws
	@inlinable public var ipmaGet: [Mp4AtomIpma] { get }
	@inlinable public mutating func ipmaSet(_ value: [Mp4AtomIpma]) throws
}
public struct Mp4AtomIpco: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomIpco
	@inlinable public static func == (a: borrowing Mp4AtomIpco, b: borrowing Mp4AtomIpco) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomIpco
	@inlinable public func propertiesGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func propertiesSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomMoov: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomMoov
	@inlinable public static func == (a: borrowing Mp4AtomMoov, b: borrowing Mp4AtomMoov) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMoov
	@inlinable public var mvhdGet: Mp4AtomMvhd { get }
	@inlinable public mutating func mvhdSet(_ value: Mp4AtomMvhd) throws
	@inlinable public func metaGet() throws -> Mp4AtomMeta?
	@inlinable public mutating func metaSet(_ value: consuming Mp4AtomMeta?) throws
	@inlinable public var mvexGet: Mp4AtomMvex? { get }
	@inlinable public mutating func mvexSet(_ value: Mp4AtomMvex?) throws
	@inlinable public func trakGet() throws -> UniqueArray<Mp4AtomTrak>
	@inlinable public mutating func trakSet(_ value: consuming UniqueArray<Mp4AtomTrak>) throws
	@inlinable public func udtaGet() throws -> Mp4AtomUdta?
	@inlinable public mutating func udtaSet(_ value: consuming Mp4AtomUdta?) throws
	@inlinable public var ainfGet: Mp4AtomAinf? { get }
	@inlinable public mutating func ainfSet(_ value: Mp4AtomAinf?) throws
}
public struct Mp4AtomStbl: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomStbl
	@inlinable public static func == (a: borrowing Mp4AtomStbl, b: borrowing Mp4AtomStbl) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStbl
	@inlinable public func stsdGet() throws -> Mp4AtomStsd
	@inlinable public mutating func stsdSet(_ value: consuming Mp4AtomStsd) throws
	@inlinable public var sttsGet: Mp4AtomStts { get }
	@inlinable public mutating func sttsSet(_ value: Mp4AtomStts) throws
	@inlinable public var cttsGet: Mp4AtomCtts? { get }
	@inlinable public mutating func cttsSet(_ value: Mp4AtomCtts?) throws
	@inlinable public var stssGet: Mp4AtomStss? { get }
	@inlinable public mutating func stssSet(_ value: Mp4AtomStss?) throws
	@inlinable public var stscGet: Mp4AtomStsc { get }
	@inlinable public mutating func stscSet(_ value: Mp4AtomStsc) throws
	@inlinable public var stszGet: Mp4AtomStsz { get }
	@inlinable public mutating func stszSet(_ value: Mp4AtomStsz) throws
	@inlinable public var stcoGet: Mp4AtomStco? { get }
	@inlinable public mutating func stcoSet(_ value: Mp4AtomStco?) throws
	@inlinable public var co64Get: Mp4AtomCo64? { get }
	@inlinable public mutating func co64Set(_ value: Mp4AtomCo64?) throws
	@inlinable public var sbgpGet: [Mp4AtomSbgp] { get }
	@inlinable public mutating func sbgpSet(_ value: [Mp4AtomSbgp]) throws
	@inlinable public var sgpdGet: [Mp4AtomSgpd] { get }
	@inlinable public mutating func sgpdSet(_ value: [Mp4AtomSgpd]) throws
	@inlinable public var subsGet: [Mp4AtomSubs] { get }
	@inlinable public mutating func subsSet(_ value: [Mp4AtomSubs]) throws
	@inlinable public var saizGet: [Mp4AtomSaiz] { get }
	@inlinable public mutating func saizSet(_ value: [Mp4AtomSaiz]) throws
	@inlinable public var saioGet: [Mp4AtomSaio] { get }
	@inlinable public mutating func saioSet(_ value: [Mp4AtomSaio]) throws
	@inlinable public var cslgGet: Mp4AtomCslg? { get }
	@inlinable public mutating func cslgSet(_ value: Mp4AtomCslg?) throws
}
public struct Mp4AtomStsd: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomStsd
	@inlinable public static func == (a: borrowing Mp4AtomStsd, b: borrowing Mp4AtomStsd) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomStsd
	@inlinable public func codecsGet() throws -> UniqueArray<Mp4AtomCodec>
	@inlinable public mutating func codecsSet(_ value: consuming UniqueArray<Mp4AtomCodec>) throws
}
public struct Mp4AtomNmhd: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomNmhd
	@inlinable public static func == (a: borrowing Mp4AtomNmhd, b: borrowing Mp4AtomNmhd) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomNmhd
}
public struct Mp4AtomSthd: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomSthd
	@inlinable public static func == (a: borrowing Mp4AtomSthd, b: borrowing Mp4AtomSthd) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomSthd
}
public struct Mp4AtomMoof: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomMoof
	@inlinable public static func == (a: borrowing Mp4AtomMoof, b: borrowing Mp4AtomMoof) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomMoof
	@inlinable public var mfhdGet: Mp4AtomMfhd { get }
	@inlinable public mutating func mfhdSet(_ value: Mp4AtomMfhd) throws
	@inlinable public func trafGet() throws -> UniqueArray<Mp4AtomTraf>
	@inlinable public mutating func trafSet(_ value: consuming UniqueArray<Mp4AtomTraf>) throws
}
public struct Mp4AtomTraf: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomTraf
	@inlinable public static func == (a: borrowing Mp4AtomTraf, b: borrowing Mp4AtomTraf) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomTraf
	@inlinable public var tfhdGet: Mp4AtomTfhd { get }
	@inlinable public mutating func tfhdSet(_ value: Mp4AtomTfhd) throws
	@inlinable public var tfdtGet: Mp4AtomTfdt? { get }
	@inlinable public mutating func tfdtSet(_ value: Mp4AtomTfdt?) throws
	@inlinable public var trunGet: [Mp4AtomTrun] { get }
	@inlinable public mutating func trunSet(_ value: [Mp4AtomTrun]) throws
	@inlinable public var sbgpGet: [Mp4AtomSbgp] { get }
	@inlinable public mutating func sbgpSet(_ value: [Mp4AtomSbgp]) throws
	@inlinable public var sgpdGet: [Mp4AtomSgpd] { get }
	@inlinable public mutating func sgpdSet(_ value: [Mp4AtomSgpd]) throws
	@inlinable public var subsGet: [Mp4AtomSubs] { get }
	@inlinable public mutating func subsSet(_ value: [Mp4AtomSubs]) throws
	@inlinable public var saizGet: [Mp4AtomSaiz] { get }
	@inlinable public mutating func saizSet(_ value: [Mp4AtomSaiz]) throws
	@inlinable public var saioGet: [Mp4AtomSaio] { get }
	@inlinable public mutating func saioSet(_ value: [Mp4AtomSaio]) throws
	@inlinable public func metaGet() throws -> Mp4AtomMeta?
	@inlinable public mutating func metaSet(_ value: consuming Mp4AtomMeta?) throws
	@inlinable public var sencGet: Mp4AtomSenc? { get }
	@inlinable public mutating func sencSet(_ value: Mp4AtomSenc?) throws
	@inlinable public func udtaGet() throws -> Mp4AtomUdta?
	@inlinable public mutating func udtaSet(_ value: consuming Mp4AtomUdta?) throws
}
public struct Mp4AtomProj: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomProj
	@inlinable public static func == (a: borrowing Mp4AtomProj, b: borrowing Mp4AtomProj) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomProj
	@inlinable public var prhdGet: Mp4AtomPrhd? { get }
	@inlinable public mutating func prhdSet(_ value: Mp4AtomPrhd?) throws
	@inlinable public var kindGet: Mp4AtomProjKind? { get }
	@inlinable public mutating func kindSet(_ value: Mp4AtomProjKind?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomVexuProj: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomVexuProj
	@inlinable public static func == (a: borrowing Mp4AtomVexuProj, b: borrowing Mp4AtomVexuProj) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomVexuProj
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public var prjiGet: Mp4AtomPrji? { get }
	@inlinable public mutating func prjiSet(_ value: Mp4AtomPrji?) throws
	@inlinable public var kindGet: Mp4AtomVexuProjKind? { get }
	@inlinable public mutating func kindSet(_ value: Mp4AtomVexuProjKind?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomEyes: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomEyes
	@inlinable public static func == (a: borrowing Mp4AtomEyes, b: borrowing Mp4AtomEyes) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomEyes
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public var striGet: Mp4AtomStri? { get }
	@inlinable public mutating func striSet(_ value: Mp4AtomStri?) throws
	@inlinable public var heroGet: Mp4AtomHero? { get }
	@inlinable public mutating func heroSet(_ value: Mp4AtomHero?) throws
	@inlinable public func camsGet() throws -> Mp4AtomCams?
	@inlinable public mutating func camsSet(_ value: consuming Mp4AtomCams?) throws
	@inlinable public func cmfyGet() throws -> Mp4AtomCmfy?
	@inlinable public mutating func cmfySet(_ value: consuming Mp4AtomCmfy?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomPack: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomPack
	@inlinable public static func == (a: borrowing Mp4AtomPack, b: borrowing Mp4AtomPack) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomPack
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public var pkinGet: Mp4AtomPkin? { get }
	@inlinable public mutating func pkinSet(_ value: Mp4AtomPkin?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomCams: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomCams
	@inlinable public static func == (a: borrowing Mp4AtomCams, b: borrowing Mp4AtomCams) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCams
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public var blinGet: Mp4AtomBlin? { get }
	@inlinable public mutating func blinSet(_ value: Mp4AtomBlin?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Mp4AtomCmfy: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Mp4AtomCmfy
	@inlinable public static func == (a: borrowing Mp4AtomCmfy, b: borrowing Mp4AtomCmfy) throws -> Swift.Bool
	@inlinable public static func `default`() -> Mp4AtomCmfy
	@inlinable public var mustGet: Mp4AtomMust? { get }
	@inlinable public mutating func mustSet(_ value: Mp4AtomMust?) throws
	@inlinable public var dadjGet: Mp4AtomDadj? { get }
	@inlinable public mutating func dadjSet(_ value: Mp4AtomDadj?) throws
	@inlinable public func unknownGet() throws -> UniqueArray<Mp4AtomAny>
	@inlinable public mutating func unknownSet(_ value: consuming UniqueArray<Mp4AtomAny>) throws
}
public struct Url: ~Copyable, @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Url
	@inlinable public static func == (a: borrowing Url, b: borrowing Url) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func parse(_ input: Swift.String) throws -> Url
}
public struct Json: ~Copyable, @unchecked Sendable {
	@inlinable public static func parse(_ input: Swift.String) throws -> Json
	@inlinable public func toText() throws -> Swift.String
}
public final class Instant: @unchecked Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Instant
	@inlinable public static func == (a: Instant, b: Instant) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public struct RustlsServerConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> RustlsServerConfig
}
public struct RustlsResolvesClientCert: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> RustlsResolvesClientCert
}
public struct RustlsKeyLog: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> RustlsKeyLog
}
public struct RustlsTimeProvider: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> RustlsTimeProvider
}
public struct RustlsCertDecompressor: ~Copyable, @unchecked Sendable {}
public struct RustlsCertCompressor: ~Copyable, @unchecked Sendable {}
public struct RustlsCompressionCache: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> RustlsCompressionCache
}
public struct RustlsCryptoProvider: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> RustlsCryptoProvider
}
public struct MoqCongestionFactory: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MoqCongestionFactory
}
public struct MoqQuicError: ~Copyable, @unchecked Sendable {}
public protocol TransportMetrics: Sendable {}
public struct StringError: Swift.Error, CustomStringConvertible {
	public let error: Swift.String
	@inlinable public init(error: Swift.String)
	@inlinable public var description: Swift.String { get }
}
extension ClockElapsed: Swift.Error {}
extension DecodeError: Swift.Error {}
extension EncodeError: Swift.Error {}
extension Error: Swift.Error {}
extension FrameFrame: Swift.Error {}
extension HangError: Swift.Error {}
extension IntersectionError: Swift.Error {}
extension InvalidHop: Swift.Error {}
extension InvalidPattern: Swift.Error {}
extension MoqTokioConnectError: Swift.Error {}
extension MoqTokioQuicDriverRecvError: Swift.Error {}
extension MoqTokioQuicDriverSendError: Swift.Error {}
extension MoqTokioWebsocketError: Swift.Error {}
extension MuxCodecAacAdtsError: Swift.Error {}
extension MuxCodecAacError: Swift.Error {}
extension MuxCodecAnnexbError: Swift.Error {}
extension MuxCodecFlacError: Swift.Error {}
extension MuxCodecH264Error: Swift.Error {}
extension MuxCodecH265Error: Swift.Error {}
extension MuxCodecMp3Error: Swift.Error {}
extension MuxCodecOpusError: Swift.Error {}
extension MuxContainerFmp4Error: Swift.Error {}
extension MuxError: Swift.Error {}
extension MuxTimelineError: Swift.Error {}
extension PathError: Swift.Error {}
extension TimeOverflow: Swift.Error {}
extension TokioError: Swift.Error {}
extension TokioTlsError: Swift.Error {}
extension TrackProducer: Swift.Error {}
public let PATH_MAX_PARTS: Swift.UInt
public let PATH_MAX_BYTES: Swift.UInt
public let PATTERN_MAX_SEGMENTS: Swift.UInt
public let PATTERN_MAX_INTERSECTIONS: Swift.UInt
public let SETUP_TOKEN_OUT_OF_BAND: Swift.UInt64
public let TRANSPORT_MIN_RECEIVE_WINDOW: Swift.UInt64
public let TRANSPORT_SETUP_RECEIVE_WINDOW: Swift.UInt64
public let GROUP_MAX_CACHE_BYTES: Swift.UInt64
public let HANG_CATALOG_MOQ_EPOCH_UNIX_MILLIS: Swift.UInt64
public let HANG_CATALOG_MAX_SAFE_INTEGER: Swift.UInt64
public let MUX_CONTAINER_DEFAULT_MAX_BUFFER: Swift.UInt
public let MUX_CONTAINER_FMP4_MAX_HELD_BYTES: Swift.UInt
public let MUX_CONTAINER_FMP4_MAX_METADATA_BYTES: Swift.UInt
public let MUX_CONTAINER_FMP4_MAX_FRAGMENT_SAMPLES: Swift.UInt64
public let MUX_CONTAINER_FMP4_MAX_TRACKS: Swift.UInt
public let MOQ_QUIC_VARINT_MAX_SIZE: Swift.UInt
public let MP4_ATOM_ST3D_MONO: Swift.UInt8
public let MP4_ATOM_ST3D_TOP_BOTTOM: Swift.UInt8
public let MP4_ATOM_ST3D_LEFT_RIGHT: Swift.UInt8
public let MP4_ATOM_ST3D_STEREO_CUSTOM: Swift.UInt8
public let MP4_ATOM_ST3D_RIGHT_LEFT: Swift.UInt8
public let MP4_ATOM_HFOV_DENOMINATOR: Swift.UInt32
public let MP4_ATOM_ESDS_ESDESCRIPTOR_TAG: Swift.UInt8
public let MP4_ATOM_ESDS_DECODERCONFIG_TAG: Swift.UInt8
public let MP4_ATOM_ESDS_SLCONFIG_TAG: Swift.UInt8
public let MP4_ATOM_U24_MAX: Swift.UInt32
public let MP4_ATOM_HERO_NONE: Swift.UInt8
public let MP4_ATOM_HERO_LEFT: Swift.UInt8
public let MP4_ATOM_HERO_RIGHT: Swift.UInt8
public let MP4_ATOM_ESDS_DECODERSPECIFIC_TAG: Swift.UInt8
public let MP4_ATOM_DADJ_DENOMINATOR: Swift.Int32
@inlinable public var IMPLEMENTATION: Swift.String { get }
@inlinable public var ALPNS: [Swift.String] { get }
@inlinable public func clockNow() -> ClockInstant
@inlinable public func clockWall() -> ClockWall
@inlinable public func clockOrigin() -> (ClockInstant, ClockWall)
@inlinable public func clockRng() -> ClockRng
@inlinable public func clockSeed() -> Swift.UInt64
@inlinable public func clockInterval(_ period: RustDuration) -> ClockInterval
@inlinable public func clockIntervalAt(_ start: ClockInstant, _ period: RustDuration) -> ClockInterval
@inlinable public func varintDecodeQuic(_ r: inout Bytes) throws -> Swift.UInt64
@inlinable public func varintEncodeQuic(_ value: Swift.UInt64, _ w: inout Bytes) throws
@inlinable public var CACHEDEFAULTEXPIRY: RustDuration { get }
@inlinable public var HANGECHOSUFFIX: Swift.String { get }
@inlinable public var HANGECHOTRACK: Swift.String { get }
@inlinable public func hangEchoIsEcho(_ path: borrowing Path) throws -> Swift.Bool
@inlinable public var HANGCATALOGPRIORITY: HangCatalogPriorities { get }
@inlinable public var HANGCONTAINERMAXAGE: RustDuration { get }
@inlinable public var HANGCONTAINERTIMESCALE: Timescale { get }
@inlinable public func hangContainerTrackInfo(_ priority: Swift.Int8) -> TrackInfo
@inlinable public func tokioTlsParseFingerprint(_ value: Swift.String) throws -> ArrayU8x32
@inlinable public func tokioOriginSpawn() -> OriginProducer
@inlinable public func transportMemoryPair(_ config: consuming TransportMemoryConfig) throws -> TupleTransportMemorySessionTransportMemorySession
@inlinable public func muxCodecAacConfig(_ `init`: Bytes) throws -> HangCatalogAudioConfig
@inlinable public var MUXCODECANNEXBSTARTCODE: Bytes { get }
@inlinable public func muxCodecAnnexbFromLengthPrefixed(_ payload: Bytes, _ lengthSize: Swift.UInt, _ prefix: Bytes?) throws -> Bytes
@inlinable public func muxCodecAnnexbBuildPrefix(_ nals: [Bytes]) -> Bytes
@inlinable public func muxCodecAnnexbLengthPrefixedToAnnexb(_ data: Bytes, _ lengthSize: Swift.UInt, _ out: inout Bytes) throws
@inlinable public func muxCodecAnnexbAfterStartCode(_ b: Bytes) throws -> Swift.UInt?
@inlinable public func muxCodecAnnexbFindStartCode(_ b: Bytes) -> (Swift.UInt, Swift.UInt)?
@inlinable public func muxCodecFlacConfig(_ `init`: Bytes) throws -> HangCatalogAudioConfig
@inlinable public func muxCodecH264Config(_ buf: Bytes) throws -> HangCatalogVideoConfig
@inlinable public func muxCodecH265Config(_ buf: Bytes) throws -> HangCatalogVideoConfig
@inlinable public func muxCodecMp3Config(_ `init`: Bytes) throws -> HangCatalogAudioConfig
@inlinable public var MUXCODECOPUSWARMUP: RustDuration { get }
@inlinable public func muxCodecOpusPacketSamples(_ packet: Bytes) -> Swift.UInt32?
@inlinable public func muxCodecOpusConfig(_ `init`: Bytes) throws -> HangCatalogAudioConfig
@inlinable public func muxContainerMp4Repair(_ file: inout some SeekableReader) throws -> MuxContainerMp4Finish?
