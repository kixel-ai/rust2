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
@frozen public struct TupleOriginProducerOriginDriver: ~Copyable, Sendable {
	public var _0: OriginProducer
	public var _1: OriginDriver
	@inlinable public init(_ _0: consuming OriginProducer, _ _1: consuming OriginDriver)
}
@frozen public struct TupleStatsTierStatsActivity: ~Copyable, Sendable {
	public var _0: StatsTier
	public var _1: StatsActivity
	@inlinable public init(_ _0: consuming StatsTier, _ _1: StatsActivity)
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
@frozen public struct TupleStringHangCatalogInclude: ~Copyable, Sendable {
	public var _0: Swift.String
	public var _1: HangCatalogInclude
	@inlinable public init(_ _0: Swift.String, _ _1: consuming HangCatalogInclude)
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
	@inlinable public func saturatingAdd(_ duration: RustDuration) -> ClockInstant
	@inlinable public func saturatingSub(_ duration: RustDuration) -> ClockInstant
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
	@inlinable public static var MOQEPOCH: ClockWall { get }
	@inlinable public static func now() -> ClockWall
	@inlinable public func checkedDurationSince(_ earlier: ClockWall) -> RustDuration?
	@inlinable public func saturatingDurationSince(_ earlier: ClockWall) -> RustDuration
	@inlinable public func checkedAdd(_ duration: RustDuration) -> ClockWall?
	@inlinable public func saturatingAdd(_ duration: RustDuration) -> ClockWall
	@inlinable public func checkedSub(_ duration: RustDuration) -> ClockWall?
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
	@inlinable public var routesRefusedGet: Swift.UInt64 { get }
	@inlinable public mutating func routesRefusedSet(_ value: Swift.UInt64)
	@inlinable public var routesGet: StatsRoutes { get }
	@inlinable public mutating func routesSet(_ value: StatsRoutes)
	@inlinable public mutating func add(_ other: StatsPresence)
	@inlinable public func active() -> Swift.UInt64
}
public struct StatsActivity: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsActivity, b: StatsActivity) -> Swift.Bool
	@inlinable public static func `default`() -> StatsActivity
	@inlinable public var idleGet: Swift.UInt64 { get }
	@inlinable public mutating func idleSet(_ value: Swift.UInt64)
	@inlinable public var activeGet: Swift.UInt64 { get }
	@inlinable public mutating func activeSet(_ value: Swift.UInt64)
}
public struct StatsTotals: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsTotals, b: StatsTotals) -> Swift.Bool
	@inlinable public static func `default`() -> StatsTotals
	@inlinable public var txGet: Swift.UInt64 { get }
	@inlinable public mutating func txSet(_ value: Swift.UInt64)
	@inlinable public var rxGet: Swift.UInt64 { get }
	@inlinable public mutating func rxSet(_ value: Swift.UInt64)
	@inlinable public var txMessagesGet: Swift.UInt64 { get }
	@inlinable public mutating func txMessagesSet(_ value: Swift.UInt64)
	@inlinable public var rxMessagesGet: Swift.UInt64 { get }
	@inlinable public mutating func rxMessagesSet(_ value: Swift.UInt64)
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
	@inlinable public var membershipGet: Swift.UInt64 { get }
	@inlinable public mutating func membershipSet(_ value: Swift.UInt64)
	@inlinable public var expiredGet: Swift.UInt64 { get }
	@inlinable public mutating func expiredSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsRefusalKind) -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsRefusals)
	@inlinable public func regressed(_ previous: StatsRefusals) -> Swift.Bool
}
public struct StatsRouteCloses: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRouteCloses, b: StatsRouteCloses) -> Swift.Bool
	@inlinable public static func `default`() -> StatsRouteCloses
	@inlinable public var stalledGet: Swift.UInt64 { get }
	@inlinable public mutating func stalledSet(_ value: Swift.UInt64)
	@inlinable public var overrunGet: Swift.UInt64 { get }
	@inlinable public mutating func overrunSet(_ value: Swift.UInt64)
	@inlinable public var malformedGet: Swift.UInt64 { get }
	@inlinable public mutating func malformedSet(_ value: Swift.UInt64)
	@inlinable public var roleGet: Swift.UInt64 { get }
	@inlinable public mutating func roleSet(_ value: Swift.UInt64)
	@inlinable public var unaskedGet: Swift.UInt64 { get }
	@inlinable public mutating func unaskedSet(_ value: Swift.UInt64)
	@inlinable public var regressedGet: Swift.UInt64 { get }
	@inlinable public mutating func regressedSet(_ value: Swift.UInt64)
	@inlinable public var otherGet: Swift.UInt64 { get }
	@inlinable public mutating func otherSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsRouteClose) -> Swift.UInt64
	@inlinable public func total() -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsRouteCloses)
	@inlinable public func regressed(_ previous: StatsRouteCloses) -> Swift.Bool
}
public struct StatsRouteRefusals: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRouteRefusals, b: StatsRouteRefusals) -> Swift.Bool
	@inlinable public static func `default`() -> StatsRouteRefusals
	@inlinable public var pathGet: Swift.UInt64 { get }
	@inlinable public mutating func pathSet(_ value: Swift.UInt64)
	@inlinable public var scopeGet: Swift.UInt64 { get }
	@inlinable public mutating func scopeSet(_ value: Swift.UInt64)
	@inlinable public var loopedGet: Swift.UInt64 { get }
	@inlinable public mutating func loopedSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsRouteRefusal) -> Swift.UInt64
	@inlinable public func total() -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsRouteRefusals)
	@inlinable public func regressed(_ previous: StatsRouteRefusals) -> Swift.Bool
}
public struct StatsRoutes: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRoutes, b: StatsRoutes) -> Swift.Bool
	@inlinable public static func `default`() -> StatsRoutes
	@inlinable public var changedGet: Swift.UInt64 { get }
	@inlinable public mutating func changedSet(_ value: Swift.UInt64)
	@inlinable public var rewrittenGet: Swift.UInt64 { get }
	@inlinable public mutating func rewrittenSet(_ value: Swift.UInt64)
	@inlinable public var sentGet: StatsWithdrawals { get }
	@inlinable public mutating func sentSet(_ value: StatsWithdrawals)
	@inlinable public var receivedGet: StatsWithdrawals { get }
	@inlinable public mutating func receivedSet(_ value: StatsWithdrawals)
	@inlinable public var refusedGet: StatsRouteRefusals { get }
	@inlinable public mutating func refusedSet(_ value: StatsRouteRefusals)
	@inlinable public var closedGet: StatsRouteCloses { get }
	@inlinable public mutating func closedSet(_ value: StatsRouteCloses)
	@inlinable public mutating func add(_ other: StatsRoutes)
	@inlinable public func regressed(_ previous: StatsRoutes) -> Swift.Bool
}
public struct StatsWithdrawals: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsWithdrawals, b: StatsWithdrawals) -> Swift.Bool
	@inlinable public static func `default`() -> StatsWithdrawals
	@inlinable public var deadGet: Swift.UInt64 { get }
	@inlinable public mutating func deadSet(_ value: Swift.UInt64)
	@inlinable public var drainGet: Swift.UInt64 { get }
	@inlinable public mutating func drainSet(_ value: Swift.UInt64)
	@inlinable public func count(_ kind: StatsWithdrawal) -> Swift.UInt64
	@inlinable public func total() -> Swift.UInt64
	@inlinable public mutating func add(_ other: StatsWithdrawals)
	@inlinable public func regressed(_ previous: StatsWithdrawals) -> Swift.Bool
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
public struct DatagramDatagram: Sendable {
	public var sequence: Swift.UInt64
	public var timestamp: Timestamp
	public var payload: Bytes
	@inlinable public init(sequence: Swift.UInt64, timestamp: Timestamp, payload: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: DatagramDatagram, b: DatagramDatagram) throws -> Swift.Bool
}
public struct FrameInfo: Sendable {
	public var size: Swift.UInt64
	public var timestamp: Timestamp
	@inlinable public init(size: Swift.UInt64, timestamp: Timestamp)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct GroupInfo: Sendable, CustomDebugStringConvertible, Hashable {
	public var start: Swift.UInt64
	@inlinable public init(start: Swift.UInt64)
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
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackPosition, b: TrackPosition) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> TrackPosition
	@inlinable public var groupGet: Swift.UInt64 { get }
	@inlinable public mutating func groupSet(_ value: Swift.UInt64)
	@inlinable public var frameGet: Swift.UInt64 { get }
	@inlinable public mutating func frameSet(_ value: Swift.UInt64)
	@inlinable public static func new(_ group: Swift.UInt64, _ frame: Swift.UInt64) -> TrackPosition
	@inlinable public static func group(_ group: Swift.UInt64) -> TrackPosition
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
}
public struct OriginCost: Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginCost, b: OriginCost) -> Swift.Bool
	@inlinable public static func `default`() -> OriginCost
	@inlinable public static func new(_ cost: Swift.UInt64) -> OriginCost
	@inlinable public func value() -> Swift.UInt64
}
public struct OriginPriority: Sendable, CustomDebugStringConvertible, Hashable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginPriority, b: OriginPriority) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> OriginPriority
	@inlinable public static func new(_ priority: Swift.Int8) -> OriginPriority
	@inlinable public func value() -> Swift.Int8
}
public struct HangCatalogAac: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	public var profile: Swift.UInt8
	@inlinable public init(profile: Swift.UInt8)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogAac, b: HangCatalogAac) -> Swift.Bool
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
public struct HangContainerFrame: Sendable {
	public var timestamp: Timestamp
	public var payload: Bytes
	@inlinable public init(timestamp: Timestamp, payload: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func encode(_ buf: inout Bytes) throws
	@inlinable public static func decode(_ buf: Bytes) throws -> HangContainerFrame
	@inlinable public func writeTo(_ group: inout GroupProducer) throws
}
public struct MuxContainerFrame: Sendable {
	public var timestamp: Timestamp
	public var duration: Timestamp?
	public var payload: Bytes
	public var keyframe: Swift.Bool
	@inlinable public init(timestamp: Timestamp, duration: Timestamp?, payload: Bytes, keyframe: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
}
public struct MuxContainerTimestampRewind: Sendable {
	public var timestamp: Timestamp
	public var floor: Timestamp
	@inlinable public init(timestamp: Timestamp, floor: Timestamp)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTimestampRewind, b: MuxContainerTimestampRewind) throws -> Swift.Bool
}
public struct MuxContainerPlayout: Sendable, CustomDebugStringConvertible, Equatable {
	public var delay: TrackDelay
	public var warmup: RustDuration
	@inlinable public init(delay: TrackDelay, warmup: RustDuration)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerPlayout, b: MuxContainerPlayout) -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerPlayout
	@inlinable public static func new(_ delay: TrackDelay, _ warmup: RustDuration) -> MuxContainerPlayout
	@inlinable public func subscription(_ subscription: consuming TrackSubscription) throws -> TrackSubscription
	@inlinable public func consumerContainer(_ track: consuming TrackSubscriber, _ format: consuming MuxCatalogHangContainer) throws -> MuxContainerConsumerContainer
	@inlinable public func consumerFmp4Wire(_ track: consuming TrackSubscriber, _ format: consuming MuxContainerFmp4Wire) throws -> MuxContainerConsumerFmp4Wire
	@inlinable public func consumerLegacyWire(_ track: consuming TrackSubscriber, _ format: MuxContainerLegacyWire) throws -> MuxContainerConsumerLegacyWire
	@inlinable public func consumerLocWire(_ track: consuming TrackSubscriber, _ format: MuxContainerLocWire) throws -> MuxContainerConsumerLocWire
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
public struct MuxCodecH265VideoSignal: Sendable, CustomDebugStringConvertible, Equatable {
	public var primaries: Swift.UInt8
	public var transfer: Swift.UInt8
	public var matrix: Swift.UInt8
	public var fullRange: Swift.Bool
	@inlinable public init(primaries: Swift.UInt8, transfer: Swift.UInt8, matrix: Swift.UInt8, fullRange: Swift.Bool)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCodecH265VideoSignal, b: MuxCodecH265VideoSignal) -> Swift.Bool
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
public struct MuxContainerTsDescriptor: Sendable, CustomDebugStringConvertible, Equatable {
	public var tag: Swift.UInt8
	public var data: Bytes
	@inlinable public init(tag: Swift.UInt8, data: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsDescriptor, b: MuxContainerTsDescriptor) -> Swift.Bool
}
public struct RustlsClientTicketRequest: Sendable, CustomDebugStringConvertible, Equatable {
	public var newSessionCount: Swift.UInt8
	public var resumptionCount: Swift.UInt8
	@inlinable public init(newSessionCount: Swift.UInt8, resumptionCount: Swift.UInt8)
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: RustlsClientTicketRequest, b: RustlsClientTicketRequest) -> Swift.Bool
}
public struct RustlsCryptoHpkeHpkePublicKey: Sendable, CustomDebugStringConvertible {
	public var _0: Bytes
	@inlinable public init(_ _0: Bytes)
	@inlinable public var debugDescription: Swift.String { get }
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
	case tooManyRequests
	case goawayTimeout
	case timeout
	case version
	case malformedAuthToken
	case app(Swift.UInt16)
	case unknown(Swift.UInt32)
	case unsupported
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
public enum Retry: Sendable, CustomDebugStringConvertible, Equatable {
	case same
	case reroute
	case never
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: Retry, b: Retry) -> Swift.Bool
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
	case handoff
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
	case tls
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioTransport, b: TokioTransport) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public func asStr() -> Swift.String
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
public enum ReconnectVerdict: Sendable, CustomDebugStringConvertible, Equatable {
	case retry
	case unusable
	case settled
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ReconnectVerdict, b: ReconnectVerdict) -> Swift.Bool
}
public enum ReconnectConfigError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case empty
	case healthy
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ReconnectConfigError, b: ReconnectConfigError) -> Swift.Bool
}
public enum ReconnectRedirect: Sendable, CustomDebugStringConvertible, Equatable {
	case follow
	case sameHost
	case ignore
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: ReconnectRedirect, b: ReconnectRedirect) -> Swift.Bool
	@inlinable public static func `default`() -> ReconnectRedirect
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
	case membership
	case expired
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRefusalKind, b: StatsRefusalKind) -> Swift.Bool
	@inlinable public static var ALL: [StatsRefusalKind] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum StatsRouteClose: Sendable, CustomDebugStringConvertible, Equatable {
	case stalled
	case overrun
	case malformed
	case role
	case unasked
	case regressed
	case other
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRouteClose, b: StatsRouteClose) -> Swift.Bool
	@inlinable public static var ALL: [StatsRouteClose] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum StatsRouteRefusal: Sendable, CustomDebugStringConvertible, Equatable {
	case path
	case scope
	case looped
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsRouteRefusal, b: StatsRouteRefusal) -> Swift.Bool
	@inlinable public static var ALL: [StatsRouteRefusal] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum StatsWithdrawal: Sendable, CustomDebugStringConvertible, Equatable {
	case dead
	case drain
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: StatsWithdrawal, b: StatsWithdrawal) -> Swift.Bool
	@inlinable public static var ALL: [StatsWithdrawal] { get }
	@inlinable public func asStr() -> Swift.String
}
public enum TransportAddress: Sendable, CustomDebugStringConvertible, Equatable {
	case uri
	case setup
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportAddress, b: TransportAddress) -> Swift.Bool
}
public enum TransportLimitsError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case unsupported
	case refused(Swift.String)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportLimitsError, b: TransportLimitsError) -> Swift.Bool
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
public enum BroadcastKind: Sendable, CustomDebugStringConvertible, Hashable {
	case track
	case datagrams
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: BroadcastKind, b: BroadcastKind) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
}
public enum TrackCompression: Sendable, CustomDebugStringConvertible, Hashable {
	case none
	case deflate
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TrackCompression, b: TrackCompression) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> TrackCompression
}
public enum TrackAnchor: Sendable, CustomDebugStringConvertible, Hashable {
	case relative(Swift.UInt64)
	case oldest
	case time(Swift.UInt64)
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
public enum OriginRouteKind: Sendable, CustomDebugStringConvertible, Equatable {
	case start
	case update
	case end
	case offline
	case restart
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: OriginRouteKind, b: OriginRouteKind) -> Swift.Bool
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
public enum HangCatalogColorRange: Sendable, CustomDebugStringConvertible, Hashable {
	case limited
	case full
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: HangCatalogColorRange, b: HangCatalogColorRange) -> Swift.Bool
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
public enum MuxContainerRead: Sendable {
	case frame(group: Swift.UInt64, frame: MuxContainerFrame)
	case gap(timestamp: Timestamp, duration: Timestamp)
	case end(group: Swift.UInt64, end: Timestamp)
	@inlinable public var debugDescription: Swift.String { get }
}
public enum MuxContainerReorderError: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case depth(reorder: Swift.UInt8, max: Swift.UInt32, codec: Swift.String)
	case deepened(declared: Swift.UInt8, reorder: Swift.UInt8)
	case reordered(Swift.UInt)
	case presentation
	case tie
	case undrained
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerReorderError, b: MuxContainerReorderError) -> Swift.Bool
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
public enum TokioListenBind: Sendable, CustomStringConvertible, CustomDebugStringConvertible, Equatable {
	case addr(SocketAddr)
	case host(Swift.String, Swift.UInt16)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TokioListenBind, b: TokioListenBind) -> Swift.Bool
	@inlinable public func resolve() throws -> SocketAddr
}
public enum TransportMemoryError: Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	case session(code: Swift.UInt32, reason: Swift.String)
	case reset(Swift.UInt32)
	case stopped(Swift.UInt32)
	case closed
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
}
public enum TransportPollAcked: Sendable, CustomDebugStringConvertible, Equatable {
	case delivered
	case unsupported
	case sent
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: TransportPollAcked, b: TransportPollAcked) -> Swift.Bool
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
	case identity(transportStreamId: Swift.UInt16, programNumber: Swift.UInt16)
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
	case objectType(Swift.UInt8)
	case rate(Swift.UInt32)
	case channelConfig(Swift.UInt8)
	case tooLarge(Swift.UInt)
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxCodecAacAdtsError, b: MuxCodecAacAdtsError) -> Swift.Bool
}
public enum MuxContainerTsStatsClass: Sendable, CustomDebugStringConvertible, Equatable {
	case audio
	case video
	case data
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MuxContainerTsStatsClass, b: MuxContainerTsStatsClass) -> Swift.Bool
	@inlinable public static func `default`() -> MuxContainerTsStatsClass
}
public enum MoqTokioQuicCongestionControl: Sendable, CustomDebugStringConvertible, Hashable {
	case bbr
	case cubic
	case unlimited
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqTokioQuicCongestionControl, b: MoqTokioQuicCongestionControl) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static func `default`() -> MoqTokioQuicCongestionControl
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
public enum MoqQuicConnectionMuxVersion: Sendable, CustomDebugStringConvertible, Hashable {
	case qMux00
	case qMux01
	case qMux02
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqQuicConnectionMuxVersion, b: MoqQuicConnectionMuxVersion) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var ALL: [MoqQuicConnectionMuxVersion] { get }
	@inlinable public func usesRecords() -> Swift.Bool
	@inlinable public func alpn() -> Swift.String
	@inlinable public func prefix() -> Swift.String
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
public enum MoqSockUdpEcnCodepoint: Sendable, CustomDebugStringConvertible, Equatable {
	case ect0
	case ect1
	case ce
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func == (a: MoqSockUdpEcnCodepoint, b: MoqSockUdpEcnCodepoint) -> Swift.Bool
	@inlinable public static func fromBits(_ x: Swift.UInt8) -> MoqSockUdpEcnCodepoint?
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
	@inlinable nonisolated(nonsending) public static func pollFuture(_ future: @escaping @Sendable () async -> Void) async throws
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
	@inlinable public consuming func withAllocator(_ allocator: consuming BandwidthAllocator) throws -> Server
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> Server
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> Server
	@inlinable public consuming func withVersions(_ versions: consuming Versions) throws -> Server
	@inlinable public consuming func withImplementation(_ implementation: Swift.String) throws -> Server
	@inlinable public consuming func withTimeout(_ timeout: RustDuration?) throws -> Server
	@inlinable public func deadline(_ arrived: ClockInstant) throws -> ClockInstant?
	@inlinable public consuming func withReceiveWindow(_ window: NonZero<Swift.UInt64>) throws -> Server
	@inlinable nonisolated(nonsending) public func acceptMemorySession(_ now: ClockInstant, _ session: consuming TransportMemorySession) async throws -> TupleSessionDriver
	@inlinable nonisolated(nonsending) public func acceptQmuxSession(_ now: ClockInstant, _ session: consuming QmuxSession) async throws -> TupleSessionDriver
	@inlinable nonisolated(nonsending) public func handshakeMemorySession(_ now: ClockInstant, _ session: consuming TransportMemorySession) async throws -> ServerHandshake
	@inlinable nonisolated(nonsending) public func handshakeQmuxSession(_ now: ClockInstant, _ session: consuming QmuxSession) async throws -> ServerHandshake
}
public struct Session: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Session
	@inlinable public static func == (a: borrowing Session, b: borrowing Session) throws -> Swift.Bool
	@inlinable public func version() throws -> Version
	@inlinable nonisolated(nonsending) public func peerImplementation() async throws -> Swift.String?
	@inlinable public func sendBandwidth() throws -> BandwidthConsumer?
	@inlinable public func allocator() throws -> BandwidthAllocator
	@inlinable public func recvBandwidth() throws -> BandwidthConsumer?
	@inlinable public func viewers() throws -> BandwidthViewers
	@inlinable public func stats() throws -> SessionStats
	@inlinable public func abort(_ err: Error) throws
	@inlinable nonisolated(nonsending) public consuming func close() async throws
	@inlinable nonisolated(nonsending) public func closed() async throws -> Error
	@inlinable public func goaway(_ goaway: consuming GoawayGoaway) throws
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
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> Client
	@inlinable public consuming func withAllocator(_ allocator: consuming BandwidthAllocator) throws -> Client
	@inlinable public consuming func withViewers(_ viewers: consuming BandwidthViewers) throws -> Client
	@inlinable public consuming func withVersions(_ versions: consuming Versions) throws -> Client
	@inlinable public consuming func withUrl(_ url: consuming Url) throws -> Client
	@inlinable public func url() throws -> Url?
	@inlinable public consuming func withToken(_ token: SetupToken) throws -> Client
	@inlinable public consuming func withAuth(_ auth: consuming AuthHandle) throws -> Client
	@inlinable public consuming func withImplementation(_ implementation: Swift.String) throws -> Client
	@inlinable public consuming func withCost(_ cost: OriginCost) throws -> Client
	@inlinable public consuming func withPeerNode(_ node: NonZero<Swift.UInt64>) throws -> Client
	@inlinable nonisolated(nonsending) public func connectMemorySession(_ now: ClockInstant, _ session: consuming TransportMemorySession) async throws -> TupleSessionDriver
	@inlinable nonisolated(nonsending) public func connectQmuxSession(_ now: ClockInstant, _ session: consuming QmuxSession) async throws -> TupleSessionDriver
}
public final class Error: @unchecked Sendable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Error
	@inlinable public func session() throws -> SessionError?
	@inlinable public func stream() throws -> StreamError?
	@inlinable public static func fromTransportError(_ err: Error) throws -> Error
	@inlinable public static func fromTransportMemoryError(_ err: TransportMemoryError) -> Error
	@inlinable public func retry() throws -> Retry
}
public final class Timestamp: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> Timestamp
	@inlinable public static func == (a: Timestamp, b: Timestamp) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static var ZERO: Timestamp { get }
	@inlinable public static func new(_ value: Swift.UInt64, _ scale: Timescale) throws -> Timestamp
	@inlinable public static func unscaled(_ key: Swift.UInt64) -> Timestamp
	@inlinable public static func fromScale(_ value: Swift.UInt64, _ unitsPerSecond: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromSecs(_ seconds: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromMillis(_ millis: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromMicros(_ micros: Swift.UInt64) throws -> Timestamp
	@inlinable public static func fromNanos(_ nanos: Swift.UInt64) throws -> Timestamp
	@inlinable public func value() throws -> Swift.UInt64
	@inlinable public func scale() throws -> Timescale?
	@inlinable public func isZero() throws -> Swift.Bool
	@inlinable public func convert(_ newScale: Timescale) throws -> Timestamp
	@inlinable public func asScale(_ target: Timescale) throws -> U128
	@inlinable public func asSecs() throws -> Swift.UInt64
	@inlinable public func asMillis() throws -> U128
	@inlinable public func asMicros() throws -> U128
	@inlinable public func asNanos() throws -> U128
	@inlinable public func checkedAdd(_ rhs: Timestamp) throws -> Timestamp
	@inlinable public func checkedSub(_ rhs: Timestamp) throws -> Timestamp
}
public final class Version: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> Version
	@inlinable public static func == (a: Version, b: Version) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var ALL: [Version] { get }
	@inlinable public func hasRequestPath() -> Swift.Bool
	@inlinable public func asStr() -> Swift.String
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
	@inlinable nonisolated(nonsending) public mutating func poll() async throws
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
	@inlinable nonisolated(nonsending) public mutating func tick() async throws -> ClockInstant
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
	@inlinable nonisolated(nonsending) public mutating func pollYield() async throws
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
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public static var DEFAULTNAME: Swift.String { get }
	@inlinable public static func defaultTrackInfo() -> TrackInfo
	@inlinable public static func defaultSubscription() -> TrackSubscription
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalog
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
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
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func extGet() throws -> MuxCatalogHangExtra
	@inlinable public mutating func extSet(_ value: consuming MuxCatalogHangExtra) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalogExtra
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalogExtra
	@inlinable public func toJson() throws -> Swift.String
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
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func extGet() throws -> MuxContainerTsExt
	@inlinable public mutating func extSet(_ value: consuming MuxContainerTsExt) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public static func fromSlice(_ v: Bytes) throws -> HangCatalogExt
	@inlinable public static func fromReader(_ reader: Bytes) throws -> HangCatalogExt
	@inlinable public func toJson() throws -> Swift.String
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
public struct MuxIngest: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxIngest
	@inlinable public static func `default`() -> MuxIngest
	@inlinable public var maxAgeGet: RustDuration? { get }
	@inlinable public mutating func maxAgeSet(_ value: RustDuration?) throws
	@inlinable public func bandwidthGet() throws -> BandwidthAllocator
	@inlinable public mutating func bandwidthSet(_ value: consuming BandwidthAllocator) throws
	@inlinable public var lingerGet: RustDuration { get }
	@inlinable public mutating func lingerSet(_ value: RustDuration) throws
	@inlinable public func rateGet() throws -> BandwidthLimit?
	@inlinable public mutating func rateSet(_ value: consuming BandwidthLimit?) throws
	@inlinable public func catalog() throws -> MuxCatalogConfig
}
public struct MuxLingerUnit: ~Copyable, @unchecked Sendable {
	@inlinable public static var DEFAULT: RustDuration { get }
	@inlinable public static func new(_ broadcasts: consuming UniqueArray<BroadcastProducer>, _ duration: RustDuration) throws -> MuxLingerUnit
	@inlinable nonisolated(nonsending) public consuming func wait() async throws
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
	@inlinable nonisolated(nonsending) public func broadcast() async throws -> BroadcastConsumer
}
public struct MuxSource: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxSource
	@inlinable public static func new(_ origin: consuming OriginConsumer, _ broadcast: consuming BroadcastConsumer) throws -> MuxSource
	@inlinable public func stats() throws -> UniqueArray<TupleStringHangEchoRendition>
	@inlinable public func broadcast() throws -> BroadcastConsumer
	@inlinable nonisolated(nonsending) public func catalogExtra(_ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumerExtra
	@inlinable nonisolated(nonsending) public func catalogExt(_ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumerExt
	@inlinable nonisolated(nonsending) public func catalogUnit(_ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumer
	@inlinable public func resolveReference() throws -> Path?
	@inlinable public func resolveReference(rel: borrowing PathRelative) throws -> Path?
	@inlinable nonisolated(nonsending) public func resolve() async throws -> BroadcastConsumer
	@inlinable nonisolated(nonsending) public func resolve(rel: borrowing PathRelative) async throws -> BroadcastConsumer
	@inlinable nonisolated(nonsending) public func resolve(instance: borrowing Instance) async throws -> BroadcastConsumer
	@inlinable nonisolated(nonsending) public func resolve(rel: borrowing PathRelative, instance: borrowing Instance) async throws -> BroadcastConsumer
	@inlinable public func bind() throws -> MuxBinding
	@inlinable public func bind(rel: borrowing PathRelative) throws -> MuxBinding
	@inlinable public func bind(instance: borrowing Instance) throws -> MuxBinding
	@inlinable public func bind(rel: borrowing PathRelative, instance: borrowing Instance) throws -> MuxBinding
	@inlinable nonisolated(nonsending) public func subscribeTrack(_ name: Swift.String) async throws -> TrackSubscriber
	@inlinable nonisolated(nonsending) public func subscribeTrack(rel: borrowing PathRelative, _ name: Swift.String) async throws -> TrackSubscriber
	@inlinable nonisolated(nonsending) public func subscribeTrack(instance: borrowing Instance, _ name: Swift.String) async throws -> TrackSubscriber
	@inlinable nonisolated(nonsending) public func subscribeTrack(rel: borrowing PathRelative, instance: borrowing Instance, _ name: Swift.String) async throws -> TrackSubscriber
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
	@inlinable public var maxGet: MoqTokioCliBound? { get }
	@inlinable public mutating func maxSet(_ value: MoqTokioCliBound?) throws
	@inlinable public var healthyGet: TokioDuration? { get }
	@inlinable public mutating func healthySet(_ value: TokioDuration?) throws
	@inlinable public var timeoutGet: MoqTokioCliBound? { get }
	@inlinable public mutating func timeoutSet(_ value: MoqTokioCliBound?) throws
	@inlinable public func initial() throws -> RustDuration
	@inlinable public func max() throws -> RustDuration?
	@inlinable public func timeout() throws -> RustDuration?
}
public struct TokioClient: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TokioClient
	@inlinable public static func new(_ config: consuming TokioClientConfig) throws -> TokioClient
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> TokioClient
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> TokioClient
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> TokioClient
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> TokioClient
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> TokioClient
	@inlinable public consuming func withCost(_ cost: Swift.UInt64) throws -> TokioClient
	@inlinable public consuming func withPeerNode(_ node: NonZero<Swift.UInt64>) throws -> TokioClient
	@inlinable public consuming func withToken(_ token: SetupToken) throws -> TokioClient
	@inlinable public consuming func withReconnect(_ reconnect: Swift.Bool) throws -> TokioClient
	@inlinable public consuming func withRestart(_ restart: TokioRestart) throws -> TokioClient
	@inlinable public func connect(_ addrs: consuming MoqTokioAddrs) throws -> TokioConnection
	@inlinable nonisolated(nonsending) public consuming func close() async throws
	@inlinable public consuming func publish(_ origin: consuming OriginConsumer) throws -> TokioConnection?
	@inlinable public consuming func consume(_ origin: consuming OriginProducer) throws -> TokioConnection?
}
public struct TokioConnection: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TokioConnection
	@inlinable public func abort(_ err: Error) throws
	@inlinable nonisolated(nonsending) public consuming func close() async throws
	@inlinable nonisolated(nonsending) public consuming func established() async throws -> TokioConnection
	@inlinable nonisolated(nonsending) public mutating func status() async throws -> TokioStatus
	@inlinable public func connected() throws -> Swift.Bool
	@inlinable public func connects() throws -> Swift.UInt64
	@inlinable public func version() throws -> Version?
	@inlinable public func transport() throws -> TokioTransport?
	@inlinable public func draining() throws -> GoawayConsumer?
	@inlinable public func sendBandwidth() throws -> BandwidthConsumer
	@inlinable public func allocator() throws -> BandwidthAllocator
	@inlinable public func recvBandwidth() throws -> BandwidthConsumer
	@inlinable public func viewers() throws -> BandwidthViewers
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func auth() throws -> AuthHandle
	@inlinable public func tokens() throws -> AuthTokens
	@inlinable public func monitor() throws -> TokioConnectionMonitor
}
public final class TokioError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TokioError
	@inlinable public func connectError() -> TokioConnectError?
	@inlinable public func isAuth() -> Swift.Bool
	@inlinable public func status() -> Swift.UInt16?
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
	@inlinable nonisolated(nonsending) public consuming func servePublish(_ origin: consuming OriginConsumer) async throws
	@inlinable public func tlsAcceptor() throws -> MoqTokioTcpTlsAcceptor?
	@inlinable public func certificates() throws -> TokioTlsCertificates
	@inlinable public func acceptHealth() throws -> UniqueArray<MoqTokioAcceptHealth>
	@inlinable nonisolated(nonsending) public consuming func bind() async throws -> TokioServerListener
	@inlinable nonisolated(nonsending) public consuming func listen() async throws -> TokioServerListener
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func websocketLocalAddr() throws -> SocketAddr?
}
public struct AuthWatch: ~Copyable, @unchecked Sendable {
	@inlinable public func peek() throws -> AuthGrant?
	@inlinable nonisolated(nonsending) public mutating func changed() async throws -> AuthGrant?
}
public struct AuthRequests: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> AuthRequest?
}
public struct AuthRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func token() throws -> Bytes
	@inlinable nonisolated(nonsending) public func closed() async throws -> Error
	@inlinable public consuming func accept(_ grant: consuming AuthGrant) throws -> AuthIssued
	@inlinable public consuming func reject(_ code: SessionError, _ reason: Swift.String) throws
}
public struct AuthIssued: ~Copyable, @unchecked Sendable {
	@inlinable public func update(_ grant: consuming AuthGrant) throws
	@inlinable public consuming func revoke(_ code: SessionError, _ reason: Swift.String) throws
	@inlinable nonisolated(nonsending) public func closed() async throws -> Error
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
	@inlinable public func add(_ token: Bytes) throws -> AuthCredential
	@inlinable public func authorize(_ grant: borrowing AuthGrant) throws
	@inlinable public func abort(_ err: Error) throws
}
public struct AuthCredential: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> AuthCredential
	@inlinable public static func == (a: borrowing AuthCredential, b: borrowing AuthCredential) throws -> Swift.Bool
	@inlinable public func grant() throws -> AuthWatch
	@inlinable nonisolated(nonsending) public func closed() async throws -> Error
	@inlinable nonisolated(nonsending) public func answered() async throws
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
	@inlinable nonisolated(nonsending) public func answered() async throws
}
public struct GoawayGoaway: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> GoawayGoaway
	@inlinable public static func == (a: borrowing GoawayGoaway, b: borrowing GoawayGoaway) throws -> Swift.Bool
	@inlinable public static func `default`() -> GoawayGoaway
	@inlinable public static func new() -> GoawayGoaway
	@inlinable public static func redirect(_ uri: Swift.String) -> GoawayGoaway
	@inlinable public consuming func withTimeout(_ timeout: RustDuration?) throws -> GoawayGoaway
	@inlinable public func uri() throws -> Swift.String
	@inlinable public func timeout() throws -> RustDuration?
}
public struct GoawayConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> GoawayConsumer
	@inlinable nonisolated(nonsending) public func poll() async throws -> GoawayGoaway
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
public final class ReconnectBackoff: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ReconnectBackoff
	@inlinable public static func == (a: ReconnectBackoff, b: ReconnectBackoff) throws -> Swift.Bool
	@inlinable public static func `default`() -> ReconnectBackoff
	@inlinable public var initialGet: RustDuration { get }
	@inlinable public func initialSet(_ value: RustDuration) throws
	@inlinable public var multiplierGet: Swift.UInt32 { get }
	@inlinable public func multiplierSet(_ value: Swift.UInt32) throws
	@inlinable public var maxGet: RustDuration? { get }
	@inlinable public func maxSet(_ value: RustDuration?) throws
	@inlinable public var healthyGet: RustDuration { get }
	@inlinable public func healthySet(_ value: RustDuration) throws
	@inlinable public var timeoutGet: RustDuration? { get }
	@inlinable public func timeoutSet(_ value: RustDuration?) throws
}
public final class ReconnectHandover: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ReconnectHandover
	@inlinable public static func == (a: ReconnectHandover, b: ReconnectHandover) throws -> Swift.Bool
	@inlinable public static func `default`() -> ReconnectHandover
	@inlinable public var redirectGet: ReconnectRedirect { get }
	@inlinable public func redirectSet(_ value: ReconnectRedirect) throws
	@inlinable public var handoverGet: RustDuration { get }
	@inlinable public func handoverSet(_ value: RustDuration) throws
}
public final class ReconnectConfig: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ReconnectConfig
	@inlinable public static func == (a: ReconnectConfig, b: ReconnectConfig) throws -> Swift.Bool
	@inlinable public static func `default`() -> ReconnectConfig
	@inlinable public var backoffGet: ReconnectBackoff { get }
	@inlinable public func backoffSet(_ value: ReconnectBackoff) throws
	@inlinable public var goawayGet: ReconnectHandover { get }
	@inlinable public func goawaySet(_ value: ReconnectHandover) throws
	@inlinable public var restartGet: TokioRestart { get }
	@inlinable public func restartSet(_ value: TokioRestart) throws
	@inlinable public var timeoutGet: RustDuration? { get }
	@inlinable public func timeoutSet(_ value: RustDuration?) throws
	@inlinable public var onceGet: Swift.Bool { get }
	@inlinable public func onceSet(_ value: Swift.Bool) throws
	@inlinable public var pinnedGet: Swift.Bool { get }
	@inlinable public func pinnedSet(_ value: Swift.Bool) throws
}
public struct ServerHandshake: ~Copyable, @unchecked Sendable {
	@inlinable public func path() throws -> Swift.String
	@inlinable public func role() throws -> SetupRole?
	@inlinable public func peerNode() throws -> NonZero<Swift.UInt64>?
	@inlinable public func peerImplementation() throws -> Swift.String?
	@inlinable public func auth() throws -> AuthHandle
	@inlinable public mutating func requests() throws -> AuthRequests
	@inlinable public func token() throws -> SetupToken?
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> ServerHandshake
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> ServerHandshake
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> ServerHandshake
	@inlinable public consuming func withPeerNode(_ node: NonZero<Swift.UInt64>) throws -> ServerHandshake
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> ServerHandshake
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> ServerHandshake
	@inlinable public consuming func withReceiveWindow(_ window: NonZero<Swift.UInt64>) throws -> ServerHandshake
	@inlinable nonisolated(nonsending) public mutating func expired() async throws
	@inlinable nonisolated(nonsending) public consuming func ok() async throws -> TupleSessionDriver
	@inlinable public consuming func close(_ err: Error) throws
}
public struct ServerAdmission: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> ServerAdmission
	@inlinable public static func `default`() -> ServerAdmission
	@inlinable public func enter() throws -> ServerAdmitted?
	@inlinable public func drain() throws
	@inlinable nonisolated(nonsending) public func close() async throws -> ClockInstant
}
public struct ServerAdmitted: ~Copyable, @unchecked Sendable {
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
	@inlinable public var datagramsUnroutedGet: Swift.UInt64? { get }
	@inlinable public func datagramsUnroutedSet(_ value: Swift.UInt64?) throws
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
	@inlinable public func activity() -> UniqueArray<TupleStatsTierStatsActivity>
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
	@inlinable public func report(_ report: inout StatsReport, _ now: ClockInstant) throws
}
public struct StatsHandle: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> StatsHandle
	@inlinable public static func `default`() -> StatsHandle
	@inlinable public func refuse(_ refusal: consuming StatsRefusal) throws
	@inlinable public func session(_ session: consuming StatsSessionInfo) throws -> StatsSession
	@inlinable public func request(_ label: consuming Path) throws -> StatsSession
}
public struct StatsRefusal: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsRefusal
	@inlinable public func labelGet() throws -> Path
	@inlinable public mutating func labelSet(_ value: consuming Path) throws
	@inlinable public var reasonGet: StatsRefusalKind { get }
	@inlinable public mutating func reasonSet(_ value: StatsRefusalKind) throws
	@inlinable public static func new(_ label: consuming Path, _ reason: StatsRefusalKind) throws -> StatsRefusal
}
public struct StatsSessionInfo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> StatsSessionInfo
	@inlinable public func labelGet() throws -> Path
	@inlinable public mutating func labelSet(_ value: consuming Path) throws
	@inlinable public static func new(_ label: consuming Path) throws -> StatsSessionInfo
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
public struct StatsBill: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> StatsBill
	@inlinable public static func `default`() -> StatsBill
	@inlinable public func deliver(_ bytes: Swift.UInt64)
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
public final class TransportReceiveWindow: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TransportReceiveWindow
	@inlinable public static func == (a: TransportReceiveWindow, b: TransportReceiveWindow) throws -> Swift.Bool
	@inlinable public var connectionGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func connectionSet(_ value: NonZero<Swift.UInt64>?) throws
	@inlinable public var streamGet: Swift.UInt64 { get }
	@inlinable public func streamSet(_ value: Swift.UInt64) throws
	@inlinable public static func new(_ connection: NonZero<Swift.UInt64>?, _ stream: Swift.UInt64) -> TransportReceiveWindow
}
public final class TransportLimits: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TransportLimits
	@inlinable public static func == (a: TransportLimits, b: TransportLimits) throws -> Swift.Bool
	@inlinable public static func `default`() -> TransportLimits
	@inlinable public var receiveWindowGet: NonZero<Swift.UInt64>? { get }
	@inlinable public func receiveWindowSet(_ value: NonZero<Swift.UInt64>?) throws
}
public struct BandwidthLimit: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BandwidthLimit
	@inlinable public static func `default`() -> BandwidthLimit
	@inlinable public static func new(_ cap: BandwidthCap?) throws -> BandwidthLimit
	@inlinable public func set(_ cap: BandwidthCap?) throws
	@inlinable public mutating func get() throws -> BandwidthCap?
	@inlinable nonisolated(nonsending) public mutating func changed() async throws -> BandwidthCap?
}
public struct BandwidthDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BandwidthDemand
	@inlinable public func wanted() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func wantedChanged() async throws -> Swift.Bool
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
public final class BandwidthCap: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BandwidthCap
	@inlinable public static func == (a: BandwidthCap, b: BandwidthCap) throws -> Swift.Bool
	@inlinable public var rateGet: BandwidthRate { get }
	@inlinable public func rateSet(_ value: BandwidthRate) throws
	@inlinable public var burstGet: RustDuration { get }
	@inlinable public func burstSet(_ value: RustDuration) throws
	@inlinable public static var DEFAULTBURST: RustDuration { get }
	@inlinable public static func new(_ rate: BandwidthRate) -> BandwidthCap
	@inlinable public func withBurst(_ burst: RustDuration) throws -> BandwidthCap
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
	@inlinable nonisolated(nonsending) public mutating func changed() async throws -> BandwidthRate?
}
public struct BroadcastDatagramRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastDatagramRequests
	@inlinable nonisolated(nonsending) public mutating func requested() async throws -> DatagramRequest
}
public struct BroadcastConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastConsumer
	@inlinable public func tracks() throws -> BroadcastTracks
	@inlinable public func info() throws -> BroadcastInfo
	@inlinable public func track(_ name: Swift.String) throws -> TrackConsumer
	@inlinable public func datagrams(_ name: Swift.String) throws -> DatagramConsumer
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClone(_ other: borrowing BroadcastConsumer) throws -> Swift.Bool
}
public struct BroadcastDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> BroadcastDemand
	@inlinable public func wanted() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func wantedChanged() async throws -> Swift.Bool
}
public struct BroadcastInfo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> BroadcastInfo
	@inlinable public static func `default`() -> BroadcastInfo
	@inlinable public func poolGet() throws -> CachePool
	@inlinable public mutating func poolSet(_ value: consuming CachePool) throws
	@inlinable public var cacheDurationGet: RustDuration? { get }
	@inlinable public mutating func cacheDurationSet(_ value: RustDuration?) throws
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
	@inlinable public func reserveUnlistedTrack(_ name: Swift.String) throws -> TrackRequest
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
	@inlinable nonisolated(nonsending) public mutating func requestedTrack() async throws -> TrackRequest
	@inlinable public func consume() throws -> BroadcastConsumer
	@inlinable public func datagrams() throws -> BroadcastDatagramRequests
	@inlinable public func lister() throws -> BroadcastLister
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClone(_ other: borrowing BroadcastRequests) throws -> Swift.Bool
}
public struct BroadcastLister: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public func pollWanted() async throws
	@inlinable nonisolated(nonsending) public func pollUnwanted() async throws
	@inlinable public func add(_ name: Swift.String, _ info: consuming TrackInfo) throws
	@inlinable public func remove(_ kind: BroadcastKind, _ name: Swift.String) throws
	@inlinable public func finish(_ kind: BroadcastKind, _ name: Swift.String) throws
	@inlinable public func synced()
	@inlinable public func clear()
	@inlinable public func end()
	@inlinable public func abort(_ err: Error) throws
}
public struct BroadcastTracks: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> BroadcastTracksEvent?
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
	@inlinable public mutating func append(_ timestamp: Timestamp, _ payload: Bytes) throws -> Swift.UInt64
	@inlinable public mutating func insert(_ datagram: DatagramDatagram) throws
	@inlinable public func consume() throws -> DatagramConsumer
	@inlinable public consuming func finish() throws
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func priority() throws -> Swift.Int8?
	@inlinable nonisolated(nonsending) public mutating func priorityChanged() async throws -> Swift.Int8?
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
	@inlinable nonisolated(nonsending) public func info() async throws -> DatagramInfo
	@inlinable nonisolated(nonsending) public mutating func recv() async throws -> DatagramDatagram?
	@inlinable public func priority() throws -> Swift.Int8
	@inlinable public mutating func setPriority(_ priority: Swift.Int8) throws
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
	@inlinable public func rescale(_ timestamp: Timestamp) throws -> Timestamp
}
public struct FrameProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func group() throws -> GroupProducer
	@inlinable public func remaining() throws -> Swift.UInt
	@inlinable public mutating func write(_ chunk: Bytes) throws
	@inlinable public consuming func finish() throws -> GroupProducer
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public var sizeGet: Swift.UInt64 { get }
	@inlinable public var timestampGet: Timestamp { get }
}
public struct FrameFrame: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> FrameFrame
	@inlinable public var timestampGet: Timestamp { get }
	@inlinable public mutating func timestampSet(_ value: Timestamp) throws
	@inlinable public var payloadGet: Bytes { get }
	@inlinable public mutating func payloadSet(_ value: Bytes) throws
	@inlinable public static func new(_ timestamp: Timestamp, _ payload: Bytes) throws -> FrameFrame
	@inlinable public func arrived() throws -> ClockInstant
}
public struct FrameConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> FrameConsumer
	@inlinable nonisolated(nonsending) public mutating func readChunk() async throws -> Bytes?
	@inlinable nonisolated(nonsending) public mutating func readAll() async throws -> Bytes
	@inlinable public var sizeGet: Swift.UInt64 { get }
	@inlinable public var timestampGet: Timestamp { get }
}
public struct GroupProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func timescale() throws -> Timescale?
	@inlinable public mutating func startAt(_ index: Swift.UInt64) throws
	@inlinable public mutating func writeFrame(_ timestamp: Timestamp, _ data: Bytes) throws
	@inlinable public consuming func createFrame(_ frame: FrameInfo) throws -> FrameProducer
	@inlinable public consuming func finishAt(_ end: Swift.UInt64) throws -> GroupFinished
	@inlinable public consuming func finish() throws -> GroupFinished
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable nonisolated(nonsending) public func ready(_ frame: FrameInfo) async throws
	@inlinable public func frameCount() throws -> Swift.UInt
	@inlinable public func consume() throws -> GroupConsumer
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func demand() throws -> GroupDemand
	@inlinable public var startGet: Swift.UInt64 { get }
}
public struct GroupFinished: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public func frameCount() throws -> Swift.UInt
	@inlinable public func consume() throws -> GroupConsumer
	@inlinable public var startGet: Swift.UInt64 { get }
}
public struct GroupDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> GroupDemand
	@inlinable public func start() throws -> Swift.UInt64
	@inlinable public func wanted() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func wantedChanged() async throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public func closed() async throws
}
public struct GroupConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> GroupConsumer
	@inlinable public func timescale() throws -> Timescale?
	@inlinable public func arrived() throws -> ClockInstant
	@inlinable public func index() throws -> Swift.UInt64
	@inlinable public consuming func withRange(_ frames: BoundsU64) throws -> GroupConsumer
	@inlinable public mutating func skipTo(_ index: Swift.UInt64) throws
	@inlinable public func frameCount() throws -> Swift.UInt
	@inlinable public func end() throws -> Swift.UInt64?
	@inlinable nonisolated(nonsending) public mutating func nextFrame() async throws -> FrameConsumer?
	@inlinable nonisolated(nonsending) public mutating func readFrame() async throws -> FrameFrame?
	@inlinable nonisolated(nonsending) public mutating func finished() async throws -> Swift.UInt64
	@inlinable public var startGet: Swift.UInt64 { get }
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
public struct TrackFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func requests() throws -> TrackRequests
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func consume() throws -> TrackConsumer
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
}
public struct TrackRequests: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TrackRequests
	@inlinable public func name() -> Swift.String
	@inlinable nonisolated(nonsending) public func requestedFill() async throws -> TrackFill
	@inlinable public func demand() -> TrackDemand
}
public struct TrackDemand: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
	@inlinable public func wanted() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func wantedChanged() async throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public func refresh() async throws
	@inlinable nonisolated(nonsending) public func closed() async throws
}
public struct TrackSubscriber: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func bill() throws -> StatsBill
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func control() throws -> TrackControl
	@inlinable nonisolated(nonsending) public mutating func recvGroup() async throws -> GroupConsumer?
	@inlinable public consuming func ordered() throws -> TrackOrdered
	@inlinable public func isClone(_ other: borrowing TrackSubscriber) throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func start() async throws -> Swift.UInt64?
	@inlinable public func floor() throws -> Swift.UInt64?
	@inlinable public func nextInRange(_ from: Swift.UInt64) throws -> Swift.UInt64?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public mutating func finished() async throws -> TrackPosition
	@inlinable public consuming func withRange(_ groups: BoundsU64) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription
	@inlinable public mutating func update(_ subscription: consuming TrackSubscription) throws
	@inlinable nonisolated(nonsending) public mutating func largest() async throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func refresh() async throws
}
public struct TrackOrdered: ~Copyable, @unchecked Sendable {
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable nonisolated(nonsending) public mutating func nextGroup() async throws -> TrackItem?
	@inlinable public func control() throws -> TrackControl
	@inlinable nonisolated(nonsending) public func refresh() async throws
	@inlinable public func subscription() throws -> TrackSubscription
	@inlinable public mutating func update(_ subscription: consuming TrackSubscription) throws
	@inlinable nonisolated(nonsending) public mutating func finished() async throws -> TrackPosition
	@inlinable nonisolated(nonsending) public mutating func largest() async throws -> TrackPosition?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable public func isClone(_ other: borrowing TrackOrdered) throws -> Swift.Bool
}
public struct TrackItem: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func group() throws -> GroupConsumer?
}
public struct TrackRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackConsumer
	@inlinable public func requests() throws -> TrackRequests
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public consuming func accept(_ info: consuming TrackInfo?) throws -> TrackProducer
	@inlinable public consuming func reject(_ err: Error) throws
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable nonisolated(nonsending) public mutating func subscriptionChanged() async throws -> TrackSubscription?
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
	@inlinable public var compressionGet: TrackCompression { get }
	@inlinable public mutating func compressionSet(_ value: TrackCompression) throws
	@inlinable public var refreshGet: TrackRefresh { get }
	@inlinable public mutating func refreshSet(_ value: TrackRefresh) throws
	@inlinable public consuming func withTimescale(_ timescale: Timescale?) throws -> TrackInfo
	@inlinable public consuming func withMaxAge(_ maxAge: RustDuration?) throws -> TrackInfo
	@inlinable public consuming func withPriority(_ priority: Swift.Int8) throws -> TrackInfo
	@inlinable public consuming func withCompression(_ compression: TrackCompression) throws -> TrackInfo
	@inlinable public consuming func withRefresh(_ refresh: TrackRefresh) throws -> TrackInfo
	@inlinable public func rescale(_ timestamp: Timestamp) throws -> Timestamp
	@inlinable public func rescaleIn(_ timestamp: Timestamp, _ key: Swift.UInt64) throws -> Timestamp
}
public final class TrackRefresh: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> TrackRefresh
	@inlinable public static func == (a: TrackRefresh, b: TrackRefresh) throws -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher) throws
	@inlinable public static func `default`() -> TrackRefresh
	@inlinable public var ratioGet: Swift.UInt32? { get }
	@inlinable public func ratioSet(_ value: Swift.UInt32?) throws
	@inlinable public var intervalGet: RustDuration? { get }
	@inlinable public func intervalSet(_ value: RustDuration?) throws
	@inlinable public func withRatio(_ ratio: Swift.UInt32?) throws -> TrackRefresh
	@inlinable public func withInterval(_ interval: RustDuration?) throws -> TrackRefresh
}
public final class TrackProducer: @unchecked Sendable {
	@inlinable public func clone() throws -> TrackProducer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func writeFrame(_ timestamp: Timestamp, _ frame: Bytes) throws
	@inlinable public func finish() throws -> TrackFinished
	@inlinable public func fail(_ err: Error) throws -> TrackFinished
	@inlinable public func finishAt(_ end: TrackPosition) throws
	@inlinable public func startAt(_ key: Swift.UInt64?) throws
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func consume() throws -> TrackConsumer
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable nonisolated(nonsending) public func subscriptionChanged() async throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct TrackAbortHandle: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TrackAbortHandle
	@inlinable public func isClosed() -> Swift.Bool
	@inlinable public func abort(_ err: Error) throws
}
public struct TrackConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TrackConsumer
	@inlinable public consuming func unmetered() throws -> TrackConsumer
	@inlinable public func name() throws -> Swift.String
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable nonisolated(nonsending) public func subscribe(_ subscription: consuming TrackSubscription?) async throws -> TrackSubscriber
	@inlinable nonisolated(nonsending) public func fetchGroup(_ key: Swift.UInt64, _ options: consuming GroupFetch?) async throws -> GroupConsumer
	@inlinable nonisolated(nonsending) public func info() async throws -> TrackInfo
	@inlinable nonisolated(nonsending) public mutating func largest() async throws -> TrackPosition?
}
public struct TrackControl: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TrackControl
	@inlinable public func subscription() -> TrackSubscription
	@inlinable public func update(_ subscription: consuming TrackSubscription) throws
	@inlinable nonisolated(nonsending) public func refresh() async throws
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
	@inlinable public consuming func withOrder(_ order: TrackOrder) throws -> TrackSubscription
}
public struct TrackFill: ~Copyable, @unchecked Sendable {}
public struct OriginFollowing: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func available() async throws -> BroadcastConsumer?
}
public struct OriginClaim: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> OriginClaim
	@inlinable public func instance() -> Instance
	@inlinable public func announce() throws
	@inlinable public func withdraw()
	@inlinable public func isAnnounced() -> Swift.Bool
	@inlinable public func setCost(_ cost: OriginCost) throws
	@inlinable public func setAuth(_ auth: consuming AuthCredential?) throws
	@inlinable nonisolated(nonsending) public func requested() async throws -> OriginRequest?
}
public struct OriginConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginConfig
	@inlinable public static func `default`() -> OriginConfig
	@inlinable public var nodeGet: NonZero<Swift.UInt64> { get }
	@inlinable public mutating func nodeSet(_ value: NonZero<Swift.UInt64>) throws
	@inlinable public var maxHopsGet: NonZero<Swift.UInt> { get }
	@inlinable public mutating func maxHopsSet(_ value: NonZero<Swift.UInt>) throws
	@inlinable public func poolGet() throws -> CachePool
	@inlinable public mutating func poolSet(_ value: consuming CachePool) throws
	@inlinable public var cacheDurationGet: RustDuration? { get }
	@inlinable public mutating func cacheDurationSet(_ value: RustDuration?) throws
	@inlinable public static func new(_ node: NonZero<Swift.UInt64>) -> OriginConfig
}
public struct OriginConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> OriginConsumer
	@inlinable public func node() throws -> NonZero<Swift.UInt64>
	@inlinable public consuming func withHidden(_ hidden: Swift.Bool) throws -> OriginConsumer
	@inlinable public consuming func covering() throws -> OriginConsumer
	@inlinable public func exclude(_ prefix: Swift.String) throws -> OriginConsumer
	@inlinable public consuming func withStats(_ session: consuming StatsSession) throws -> OriginConsumer
	@inlinable public func announced() throws -> AnnounceConsumer
	@inlinable public func routes() throws -> OriginRoutes
	@inlinable public func consume() throws -> OriginConsumer
	@inlinable public func follow(_ path: Swift.String, _ instance: consuming Instance?) throws -> OriginFollowing
	@inlinable public func scope(_ root: Swift.String, _ patterns: borrowing Patterns) throws -> OriginConsumer
	@inlinable nonisolated(nonsending) public func request(_ path: Swift.String, _ instance: consuming Instance?) async throws -> BroadcastConsumer
	@inlinable public func root() throws -> Path
	@inlinable public func allowed() throws -> Patterns
	@inlinable public func absolute(_ path: Swift.String) throws -> Path
}
public struct OriginDriver: ~Copyable, @unchecked Sendable {
	@inlinable public mutating func poll(_ now: ClockInstant, _ waiter: borrowing Waiter) throws -> ClockInstant?
}
public struct OriginProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> OriginProducer
	@inlinable public static func new(_ config: consuming OriginConfig) throws -> TupleOriginProducerOriginDriver
	@inlinable public consuming func withStats(_ session: consuming StatsSession) throws -> OriginProducer
	@inlinable public consuming func withLink(_ link: OriginLink) throws -> OriginProducer
	@inlinable public consuming func withPriority(_ priority: OriginPriority) throws -> OriginProducer
	@inlinable public consuming func withProxy(_ proxy: Swift.Bool) throws -> OriginProducer
	@inlinable public func config() throws -> OriginConfig
	@inlinable public func node() throws -> NonZero<Swift.UInt64>
	@inlinable public func isOwn(_ id: NonZero<Swift.UInt64>) throws -> Swift.Bool
	@inlinable public func route(_ path: Swift.String, _ options: consuming OriginRouteOptions) throws -> OriginRoute
	@inlinable public func claim(_ prefix: Swift.String, _ options: consuming OriginRouteOptions) throws -> OriginClaim
	@inlinable public func scope(_ root: Swift.String, _ patterns: borrowing Patterns) throws -> OriginProducer
	@inlinable public func exclude(_ prefix: Swift.String) throws -> OriginProducer
	@inlinable public func mount(_ at: Swift.String, _ target: Swift.String) throws -> OriginProducer
	@inlinable public func consume() throws -> OriginConsumer
	@inlinable public func root() throws -> Path
	@inlinable public func allowed() throws -> Patterns
	@inlinable public func absolute(_ path: Swift.String) throws -> Path
}
public struct OriginRequest: ~Copyable, @unchecked Sendable {
	@inlinable public func path() throws -> Path
	@inlinable public consuming func route(_ options: consuming OriginRouteOptions) throws -> OriginRoute
	@inlinable public consuming func serveProducer(_ broadcast: borrowing BroadcastProducer) throws
	@inlinable public consuming func serveConsumer(_ broadcast: borrowing BroadcastConsumer) throws
	@inlinable public consuming func reject(_ err: Error) throws
}
public struct OriginRoute: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> OriginRoute
	@inlinable public func broadcast() -> BroadcastProducer
	@inlinable public func instance() -> Instance
	@inlinable public func announce() throws
	@inlinable public func withdraw()
	@inlinable public func isAnnounced() -> Swift.Bool
	@inlinable public func setCost(_ cost: OriginCost) throws
	@inlinable public func setAuth(_ auth: consuming AuthCredential?) throws
}
public struct OriginRouteInfo: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginRouteInfo
	@inlinable public static func == (a: borrowing OriginRouteInfo, b: borrowing OriginRouteInfo) throws -> Swift.Bool
	@inlinable public func instance() throws -> Instance
	@inlinable public func cost() throws -> OriginCost
	@inlinable public func available() throws -> Swift.Bool
}
public struct OriginRouteOptions: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginRouteOptions
	@inlinable public static func `default`() -> OriginRouteOptions
	@inlinable public static func hidden() -> OriginRouteOptions
	@inlinable public consuming func withCost(_ cost: OriginCost) throws -> OriginRouteOptions
	@inlinable public consuming func withAuth(_ auth: consuming AuthCredential) throws -> OriginRouteOptions
	@inlinable public consuming func withInstance(_ instance: consuming Instance, _ epoch: Swift.UInt64) throws -> OriginRouteOptions
}
public struct OriginRouteEvent: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> OriginRouteEvent
	@inlinable public var kindGet: OriginRouteKind { get }
	@inlinable public mutating func kindSet(_ value: OriginRouteKind) throws
	@inlinable public func announceGet() throws -> AnnounceAnnounce
	@inlinable public mutating func announceSet(_ value: consuming AnnounceAnnounce) throws
}
public struct OriginRoutes: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> OriginRouteEvent?
	@inlinable public mutating func tryNext() throws -> OriginRouteEvent?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public func pollSynced() async throws
	@inlinable public func root() throws -> Path
	@inlinable public func absolute(_ prefix: Swift.String) throws -> Path
}
public struct AnnounceConsumer: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> AnnounceEvent?
	@inlinable public mutating func tryNext() throws -> AnnounceEvent?
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable nonisolated(nonsending) public func pollSynced() async throws
	@inlinable public func root() throws -> Path
	@inlinable public func absolute(_ prefix: Swift.String) throws -> Path
}
public struct AnnounceEvent: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> AnnounceEvent
}
public struct AnnounceAnnounce: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> AnnounceAnnounce
	@inlinable public func prefixGet() throws -> Path
	@inlinable public mutating func prefixSet(_ value: consuming Path) throws
	@inlinable public func capturesGet() throws -> UniqueArray<Pattern>?
	@inlinable public mutating func capturesSet(_ value: consuming UniqueArray<Pattern>?) throws
	@inlinable public func routeGet() throws -> OriginRouteInfo
	@inlinable public mutating func routeSet(_ value: consuming OriginRouteInfo) throws
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
}
public struct HangEchoArrival: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangEchoArrival
	@inlinable public static func == (a: borrowing HangEchoArrival, b: borrowing HangEchoArrival) throws -> Swift.Bool
	@inlinable public var timestampGet: Timestamp { get }
	@inlinable public mutating func timestampSet(_ value: Timestamp) throws
	@inlinable public var wallGet: ClockWall { get }
	@inlinable public mutating func wallSet(_ value: ClockWall) throws
	@inlinable public static func new(_ timestamp: Timestamp, _ wall: ClockWall) throws -> HangEchoArrival
}
public struct HangCatalogSkipped: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogSkipped
	@inlinable public static func == (a: borrowing HangCatalogSkipped, b: borrowing HangCatalogSkipped) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogSkipped
	@inlinable public var videoGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func videoSet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public var audioGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func audioSet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public var textGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func textSet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public var jsonGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func jsonSet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public var binaryGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func binarySet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public var includeGet: [(Swift.String, Swift.UInt64)] { get }
	@inlinable public mutating func includeSet(_ value: [(Swift.String, Swift.UInt64)]) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
	@inlinable public func version() throws -> Swift.UInt64?
}
public struct HangCatalogConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackConsumer, _ delay: RustDuration) throws -> HangCatalogConsumer
	@inlinable nonisolated(nonsending) public mutating func pollNext() async throws -> MoqJsonTimedCatalog?
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
public final class HangCatalogClock: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogClock
	@inlinable public static func == (a: HangCatalogClock, b: HangCatalogClock) throws -> Swift.Bool
	@inlinable public var wallGet: Timestamp { get }
	@inlinable public func wallSet(_ value: Timestamp) throws
	@inlinable public static func new(_ wall: Timestamp) throws -> HangCatalogClock
	@inlinable public func wallClock(_ pts: Timestamp) throws -> ClockWall
}
public struct HangCatalogContainer: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogContainer
	@inlinable public static func == (a: borrowing HangCatalogContainer, b: borrowing HangCatalogContainer) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogContainer
}
public struct HangCatalogEcho: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogEcho
	@inlinable public static func == (a: borrowing HangCatalogEcho, b: borrowing HangCatalogEcho) throws -> Swift.Bool
	@inlinable public func pathGet() throws -> PathRelative
	@inlinable public mutating func pathSet(_ value: consuming PathRelative) throws
	@inlinable public static func new(_ path: consuming PathRelative) throws -> HangCatalogEcho
}
public struct HangCatalogInclude: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogInclude
	@inlinable public static func == (a: borrowing HangCatalogInclude, b: borrowing HangCatalogInclude) throws -> Swift.Bool
	@inlinable public func broadcastGet() throws -> PathRelative
	@inlinable public mutating func broadcastSet(_ value: consuming PathRelative) throws
	@inlinable public func instanceGet() throws -> Instance?
	@inlinable public mutating func instanceSet(_ value: consuming Instance?) throws
	@inlinable public static func new(_ broadcast: consuming PathRelative) throws -> HangCatalogInclude
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
public final class HangCatalogTimeline: @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> HangCatalogTimeline
	@inlinable public static func == (a: HangCatalogTimeline, b: HangCatalogTimeline) throws -> Swift.Bool
	@inlinable public static func `default`() -> HangCatalogTimeline
	@inlinable public var clockGet: HangCatalogClock? { get }
	@inlinable public func clockSet(_ value: HangCatalogClock?) throws
	@inlinable public var intervalGet: RustDuration? { get }
	@inlinable public func intervalSet(_ value: RustDuration?) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
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
	@inlinable public var rangeGet: HangCatalogColorRange? { get }
	@inlinable public mutating func rangeSet(_ value: HangCatalogColorRange?) throws
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
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public static func new(_ codec: HangCatalogVideoCodec) -> HangCatalogVideoConfig
	@inlinable public func sameIdentity(_ other: borrowing HangCatalogVideoConfig) throws -> Swift.Bool
}
public struct MuxCatalogConsumer: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func new(_ broadcast: borrowing BroadcastConsumer, _ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumer
}
public struct MuxCatalogConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func new(_ broadcast: borrowing BroadcastConsumer, _ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumerExtra
}
public struct MuxCatalogConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func new(_ broadcast: borrowing BroadcastConsumer, _ format: MuxCatalogCatalogFormat) async throws -> MuxCatalogConsumerExt
}
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
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
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
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func extGet() throws -> MuxCatalogHangExtra
	@inlinable public mutating func extSet(_ value: consuming MuxCatalogHangExtra) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
	@inlinable public func toVec() throws -> Bytes
	@inlinable public func toWriter(_ writer: Bytes) throws
}
public struct MuxCatalogGuardExt: ~Copyable {
	@inlinable public consuming func commit() throws
	@inlinable public func videoGet() throws -> HangCatalogVideo
	@inlinable public mutating func videoSet(_ value: consuming HangCatalogVideo) throws
	@inlinable public func audioGet() throws -> HangCatalogAudio
	@inlinable public mutating func audioSet(_ value: consuming HangCatalogAudio) throws
	@inlinable public var syncGet: HangCatalogTimeline { get }
	@inlinable public mutating func syncSet(_ value: HangCatalogTimeline) throws
	@inlinable public func textGet() throws -> HangCatalogText
	@inlinable public mutating func textSet(_ value: consuming HangCatalogText) throws
	@inlinable public func jsonGet() throws -> HangCatalogJson
	@inlinable public mutating func jsonSet(_ value: consuming HangCatalogJson) throws
	@inlinable public func binaryGet() throws -> HangCatalogBinary
	@inlinable public mutating func binarySet(_ value: consuming HangCatalogBinary) throws
	@inlinable public func statsGet() throws -> HangCatalogStats?
	@inlinable public mutating func statsSet(_ value: consuming HangCatalogStats?) throws
	@inlinable public func echoGet() throws -> HangCatalogEcho?
	@inlinable public mutating func echoSet(_ value: consuming HangCatalogEcho?) throws
	@inlinable public func includeGet() throws -> UniqueArray<TupleStringHangCatalogInclude>
	@inlinable public mutating func includeSet(_ value: consuming UniqueArray<TupleStringHangCatalogInclude>) throws
	@inlinable public func extGet() throws -> MuxContainerTsExt
	@inlinable public mutating func extSet(_ value: consuming MuxContainerTsExt) throws
	@inlinable public func skippedGet() throws -> HangCatalogSkipped
	@inlinable public mutating func skippedSet(_ value: consuming HangCatalogSkipped) throws
	@inlinable public func checkRenditions() throws
	@inlinable public func withoutExt() throws -> HangCatalog
	@inlinable public func toJson() throws -> Swift.String
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
}
public struct MuxContainerConsumerContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: consuming MuxCatalogHangContainer) throws -> MuxContainerConsumerContainer
	@inlinable public func arrival() throws -> ClockInstant?
	@inlinable public func backlog() throws -> ClockInstant?
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerRead?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setPlayhead(_ playhead: consuming MuxSyncConsumer) throws
}
public struct MuxContainerConsumerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: consuming MuxContainerFmp4Wire) throws -> MuxContainerConsumerFmp4Wire
	@inlinable public func arrival() throws -> ClockInstant?
	@inlinable public func backlog() throws -> ClockInstant?
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerRead?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setPlayhead(_ playhead: consuming MuxSyncConsumer) throws
}
public struct MuxContainerConsumerLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: MuxContainerLegacyWire) throws -> MuxContainerConsumerLegacyWire
	@inlinable public func arrival() throws -> ClockInstant?
	@inlinable public func backlog() throws -> ClockInstant?
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerRead?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setPlayhead(_ playhead: consuming MuxSyncConsumer) throws
}
public struct MuxContainerConsumerLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber, _ format: MuxContainerLocWire) throws -> MuxContainerConsumerLocWire
	@inlinable public func arrival() throws -> ClockInstant?
	@inlinable public func backlog() throws -> ClockInstant?
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerRead?
	@inlinable public func isSkipped(_ groups: RangeU64) throws -> Swift.Bool
	@inlinable public mutating func setDelay(_ delay: TrackDelay) throws
	@inlinable public mutating func setPlayhead(_ playhead: consuming MuxSyncConsumer) throws
}
public struct MuxContainerGroupConsumerContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: consuming MuxCatalogHangContainer) throws -> MuxContainerGroupConsumerContainer
	@inlinable public func start() throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerFrame?
}
public struct MuxContainerGroupConsumerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: consuming MuxContainerFmp4Wire) throws -> MuxContainerGroupConsumerFmp4Wire
	@inlinable public func start() throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerFrame?
}
public struct MuxContainerGroupConsumerLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: MuxContainerLegacyWire) throws -> MuxContainerGroupConsumerLegacyWire
	@inlinable public func start() throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerFrame?
}
public struct MuxContainerGroupConsumerLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ group: consuming GroupConsumer, _ format: MuxContainerLocWire) throws -> MuxContainerGroupConsumerLocWire
	@inlinable public func start() throws -> Swift.UInt64
	@inlinable nonisolated(nonsending) public mutating func read() async throws -> MuxContainerFrame?
}
public struct MuxContainerFinishedContainerAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedContainerBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedContainerJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedContainerTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedContainerVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedContainer: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4WireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4WireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4WireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4WireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4WireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLegacyWire: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWireAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWireBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWireJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWireTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWireVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFinishedLocWire: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerGuardUnit: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
}
public struct MuxContainerGuardAudioConfig: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
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
	@inlinable public func sameIdentity(_ other: borrowing HangCatalogAudioConfig) throws -> Swift.Bool
}
public struct MuxContainerGuardBinaryConfig: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
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
}
public struct MuxContainerGuardJsonConfig: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
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
}
public struct MuxContainerGuardTextConfig: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
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
}
public struct MuxContainerGuardVideoConfig: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func commit() throws
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
	@inlinable public var delayGet: RustDuration? { get }
	@inlinable public mutating func delaySet(_ value: RustDuration?) throws
	@inlinable public func sameIdentity(_ other: borrowing HangCatalogVideoConfig) throws -> Swift.Bool
}
public final class MuxContainerProducerContainerAudioConfig {
	@inlinable public func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardAudioConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerContainerBinaryConfig {
	@inlinable public func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardBinaryConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainerBinaryConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainerBinaryConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerContainerJsonConfig {
	@inlinable public func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardJsonConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainerJsonConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainerJsonConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerContainerTextConfig {
	@inlinable public func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainerTextConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainerTextConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerContainerVideoConfig {
	@inlinable public func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardVideoConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainerVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerContainer {
	@inlinable public static func new(_ track: TrackProducer, _ container: consuming MuxCatalogHangContainer) throws -> MuxContainerProducerContainer
	@inlinable public func set() throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardUnit
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerContainer
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerContainer
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedContainer
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedContainer
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4WireAudioConfig {
	@inlinable public func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardAudioConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4WireAudioConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4WireAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4WireBinaryConfig {
	@inlinable public func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardBinaryConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4WireBinaryConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4WireBinaryConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4WireJsonConfig {
	@inlinable public func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardJsonConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4WireJsonConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4WireJsonConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4WireTextConfig {
	@inlinable public func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4WireTextConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4WireTextConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4WireVideoConfig {
	@inlinable public func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardVideoConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4WireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4WireVideoConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4WireVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerFmp4Wire {
	@inlinable public static func new(_ track: TrackProducer, _ container: consuming MuxContainerFmp4Wire) throws -> MuxContainerProducerFmp4Wire
	@inlinable public func set() throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardUnit
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerFmp4Wire
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerFmp4Wire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedFmp4Wire
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedFmp4Wire
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWireAudioConfig {
	@inlinable public func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardAudioConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWireAudioConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWireAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWireBinaryConfig {
	@inlinable public func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardBinaryConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWireBinaryConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWireBinaryConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWireJsonConfig {
	@inlinable public func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardJsonConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWireJsonConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWireJsonConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWireTextConfig {
	@inlinable public func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWireTextConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWireTextConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWireVideoConfig {
	@inlinable public func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardVideoConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWireVideoConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWireVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLegacyWire {
	@inlinable public static func new(_ track: TrackProducer, _ container: MuxContainerLegacyWire) throws -> MuxContainerProducerLegacyWire
	@inlinable public func set() throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardUnit
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLegacyWire
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLegacyWire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLegacyWire
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLegacyWire
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWireAudioConfig {
	@inlinable public func set(_ config: consuming HangCatalogAudioConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardAudioConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireAudioConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWireAudioConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWireAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWireBinaryConfig {
	@inlinable public func set(_ config: consuming HangCatalogBinaryConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardBinaryConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireBinaryConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWireBinaryConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWireBinaryConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWireJsonConfig {
	@inlinable public func set(_ config: consuming HangCatalogJsonConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardJsonConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireJsonConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWireJsonConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWireJsonConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWireTextConfig {
	@inlinable public func set(_ config: consuming HangCatalogTextConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardTextConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireTextConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWireTextConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWireTextConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWireVideoConfig {
	@inlinable public func set(_ config: consuming HangCatalogVideoConfig) throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardVideoConfig
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWireVideoConfig
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWireVideoConfig
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWireVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public final class MuxContainerProducerLocWire {
	@inlinable public static func new(_ track: TrackProducer, _ container: MuxContainerLocWire) throws -> MuxContainerProducerLocWire
	@inlinable public func set() throws
	@inlinable public func setBitrate(_ bitrate: Swift.UInt64) throws
	@inlinable public func modify() throws -> MuxContainerGuardUnit
	@inlinable public func timestamp(_ hint: Timestamp?) throws -> Timestamp
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func estimate() throws -> MuxCatalogEstimate
	@inlinable public func reorder(_ delay: Timestamp) throws
	@inlinable public func history(_ until: Timestamp) throws
	@inlinable public func needsKeyframe() throws -> Swift.Bool
	@inlinable public func withBuffer(_ duration: RustDuration) throws -> MuxContainerProducerLocWire
	@inlinable public func withBandwidth(_ allocator: consuming BandwidthAllocator) throws -> MuxContainerProducerLocWire
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func liveEdge() throws -> Timestamp?
	@inlinable public func write(_ frame: MuxContainerFrame) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func discontinuity() throws
	@inlinable public func finish() throws -> MuxContainerFinishedLocWire
	@inlinable public func fail(_ err: Error) throws -> MuxContainerFinishedLocWire
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func broadcast() throws -> BroadcastInfo
	@inlinable public func info() throws -> TrackInfo
	@inlinable public func createGroup(_ group: GroupInfo) throws -> GroupProducer
	@inlinable public func appendGroup(_ start: Timestamp) throws -> GroupProducer
	@inlinable public func skipGroups(_ groups: RangeU64) throws
	@inlinable public func finished() throws -> TrackPosition?
	@inlinable nonisolated(nonsending) public func closed() async throws
	@inlinable public func isClosed() throws -> Swift.Bool
	@inlinable public func largest() throws -> TrackPosition?
	@inlinable public func isClone(_ other: TrackProducer) throws -> Swift.Bool
	@inlinable public func abortHandle() throws -> TrackAbortHandle
	@inlinable public func subscribe(_ subscription: consuming TrackSubscription?) throws -> TrackSubscriber
	@inlinable public func subscription() throws -> TrackSubscription?
	@inlinable public func requests() throws -> TrackRequests
}
public struct MuxFollowInstances: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> BroadcastConsumer?
}
public struct MuxImportContainer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainer
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxImportContainerFinished
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
}
public struct MuxImportContainerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainerExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxImportContainerFinishedExtra
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
}
public struct MuxImportContainerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt, _ `init`: borrowing MuxImportContainerInit) throws -> MuxImportContainerExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxImportContainerFinishedExt
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func cut() throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
}
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
public final class MuxImportTrack {
	@inlinable public static func audioExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func audioExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func audioUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportAudioInit) throws -> MuxImportTrack
	@inlinable public static func videoExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public static func videoExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public static func videoUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrack
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public func finish() throws -> MuxImportTrackFinished
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
}
public final class MuxImportTrackStream {
	@inlinable public static func videoExtra(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExtra, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public static func videoExt(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReservedExt, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public static func videoUnit(_ request: consuming TrackRequest, _ reserved: consuming MuxCatalogReserved, _ `init`: consuming MuxImportVideoInit) throws -> MuxImportTrackStream
	@inlinable public func initialize(_ data: Bytes) throws
	@inlinable public func decode(_ data: Bytes) throws
	@inlinable public func finish() throws -> MuxImportTrackFinished
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func name() throws -> Swift.String
}
public final class MuxJsonSnapshotJson: @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func demand() throws -> TrackDemand
	@inlinable nonisolated(nonsending) public func refresh() async throws
	@inlinable public func update(_ value: borrowing Json, _ timestamp: Timestamp) throws
	@inlinable public func finish() throws
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
}
public final class MuxJsonStreamJson: @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
	@inlinable public func consume() throws -> TrackSubscriber
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func append(_ value: borrowing Json, _ timestamp: Timestamp) throws
	@inlinable public func finish() throws
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
}
public struct MuxJsonConsumerJson: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func fromTrack(_ track: consuming TrackConsumer, _ config: borrowing HangCatalogJsonConfig) async throws -> MuxJsonConsumerJson
	@inlinable public func mode() throws -> HangCatalogMode
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MoqJsonTimedJson?
	@inlinable nonisolated(nonsending) public mutating func latest() async throws -> MoqJsonTimedJson?
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
public struct MuxRateGrant: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ grant: consuming BandwidthConsumer, _ policy: consuming MuxRatePolicy) throws -> MuxRateGrant
	@inlinable public mutating func assume(_ target: MuxRateTarget) throws
	@inlinable public func policy() throws -> MuxRatePolicy
	@inlinable public mutating func setPolicy(_ policy: consuming MuxRatePolicy) throws
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxRateTarget
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
	@inlinable public func retainExtra(_ catalog: inout HangCatalogExtra) throws
	@inlinable public func retainExt(_ catalog: inout HangCatalogExt) throws
	@inlinable public func retainUnit(_ catalog: inout HangCatalog) throws
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
public struct MuxSyncTrack: ~Copyable, @unchecked Sendable {
	@inlinable public func received(_ timestamp: Timestamp, _ now: ClockInstant) throws -> Swift.Bool
	@inlinable public func reset()
	@inlinable public func setTarget(_ target: RustDuration) -> Swift.Bool
	@inlinable nonisolated(nonsending) public func delayChanged(_ seen: RustDuration) async throws -> RustDuration
}
public final class MuxSyncSchedule: @unchecked Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxSyncSchedule
	@inlinable public static func == (a: MuxSyncSchedule, b: MuxSyncSchedule) -> Swift.Bool
	@inlinable public func delay() -> RustDuration
	@inlinable public func due(_ timestamp: Timestamp) throws -> ClockInstant?
}
public struct MuxSyncProducer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxSyncProducer
	@inlinable public static func new(_ delay: RustDuration) -> MuxSyncProducer
	@inlinable public consuming func reset(_ delay: RustDuration) throws -> MuxSyncProducer
	@inlinable public func consume() throws -> MuxSyncConsumer
	@inlinable public func track(_ kind: MuxContainerKind) throws -> MuxSyncTrack
	@inlinable public func due(_ timestamp: Timestamp) throws -> ClockInstant?
}
public struct MuxSyncConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> MuxSyncConsumer
	@inlinable public func delay() throws -> RustDuration
	@inlinable public func due(_ timestamp: Timestamp) throws -> ClockInstant?
	@inlinable public func position(_ at: ClockInstant) throws -> RustDuration?
	@inlinable public func schedule() throws -> MuxSyncSchedule
	@inlinable nonisolated(nonsending) public func changed(_ seen: MuxSyncSchedule) async throws -> MuxSyncSchedule
	@inlinable public func present(_ timestamps: [Timestamp], _ now: ClockInstant) throws -> Swift.UInt?
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
	@inlinable public func peer(_ chain: consuming UniqueArray<RustlsPkiTypesCertificateDer>) throws -> TokioTlsPeerIdentity
}
public struct TokioTlsPeerIdentity: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> TokioTlsPeerIdentity
	@inlinable public func chain() -> UniqueArray<RustlsPkiTypesCertificateDer>
	@inlinable public func fingerprint() -> Swift.String?
	@inlinable public func name() -> Swift.String?
	@inlinable public func lineage() throws -> [Swift.String]
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
	@inlinable public var implementationGet: Swift.String? { get }
	@inlinable public mutating func implementationSet(_ value: Swift.String?) throws
	@inlinable public func tlsGet() throws -> TokioTlsConnect
	@inlinable public mutating func tlsSet(_ value: consuming TokioTlsConnect) throws
	@inlinable public func backoffGet() throws -> TokioBackoff
	@inlinable public mutating func backoffSet(_ value: consuming TokioBackoff) throws
	@inlinable public func goawayGet() throws -> MoqTokioConnectionHandover
	@inlinable public mutating func goawaySet(_ value: consuming MoqTokioConnectionHandover) throws
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
	@inlinable nonisolated(nonsending) public func stateChanged(_ after: Swift.UInt64) async throws -> MoqTokioConnectionObservation
	@inlinable public func presence() throws -> StatsPresence
	@inlinable nonisolated(nonsending) public mutating func pollPresence() async throws -> StatsPresence
	@inlinable nonisolated(nonsending) public mutating func presenceChanged() async throws -> StatsPresence
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
	@inlinable public func server() throws -> Server
	@inlinable public func hasExplicitBind() throws -> Swift.Bool
	@inlinable public func servesQuic() throws -> Swift.Bool
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
	@inlinable public func admissionGet() throws -> ServerAdmission
	@inlinable public mutating func admissionSet(_ value: consuming ServerAdmission) throws
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
	@inlinable public func streamsGet() throws -> MoqTokioServerStreams
	@inlinable public mutating func streamsSet(_ value: consuming MoqTokioServerStreams) throws
	@inlinable public consuming func init_() throws -> TokioServer
	@inlinable public consuming func initStreams() throws -> TokioServer
}
public struct TokioServerListener: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public mutating func accept() async throws -> TokioServerRequest
	@inlinable nonisolated(nonsending) public consuming func close() async throws
	@inlinable nonisolated(nonsending) public mutating func closeStreams() async throws
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func websocketLocalAddr() throws -> SocketAddr?
	@inlinable public func tcpLocalAddr() throws -> SocketAddr?
	@inlinable public func certificates() throws -> TokioTlsCertificates
	@inlinable public func acceptHealth() throws -> UniqueArray<MoqTokioAcceptHealth>
}
public struct TokioServerRequest: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public consuming func reject(_ code: SessionError) async throws
	@inlinable public consuming func withPublisherProducer(_ publish: borrowing OriginProducer) throws -> TokioServerRequest
	@inlinable public consuming func withPublisherConsumer(_ publish: borrowing OriginConsumer) throws -> TokioServerRequest
	@inlinable public consuming func withSubscriber(_ subscribe: consuming OriginProducer) throws -> TokioServerRequest
	@inlinable public consuming func withPeerNode(_ node: NonZero<Swift.UInt64>) throws -> TokioServerRequest
	@inlinable public consuming func withStats(_ stats: consuming StatsSession) throws -> TokioServerRequest
	@inlinable public consuming func withLimits(_ limits: SessionLimits) throws -> TokioServerRequest
	@inlinable public func quicLimits() throws -> MoqTokioServerQuicLimits?
	@inlinable public consuming func withQuicLimits(_ limits: MoqQuicLimits) throws -> TokioServerRequest
	@inlinable nonisolated(nonsending) public mutating func expired() async throws
	@inlinable nonisolated(nonsending) public consuming func ok() async throws -> Session
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
	@inlinable public func peerNode() throws -> NonZero<Swift.UInt64>?
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
	@inlinable public var latencyGet: RustDuration { get }
	@inlinable public mutating func latencySet(_ value: RustDuration) throws
}
public struct TransportMemorySession: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() throws -> TransportMemorySession
	@inlinable public func closer() throws -> TransportMemoryCloser
}
public struct TransportMemoryCloser: ~Copyable, @unchecked Sendable {
	@inlinable public func clone() -> TransportMemoryCloser
	@inlinable public func close(_ code: Swift.UInt32, _ reason: Swift.String)
	@inlinable public func isClosed() -> Swift.Bool
}
public struct MuxCatalogHangConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackConsumer) throws -> MuxCatalogHangConsumer
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalog?
}
public struct MuxCatalogHangConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackConsumer) throws -> MuxCatalogHangConsumerExtra
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalogExtra?
}
public struct MuxCatalogHangConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackConsumer) throws -> MuxCatalogHangConsumerExt
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalogExt?
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
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalog?
}
public struct MuxCatalogMsfConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogMsfConsumerExtra
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalogExtra?
}
public struct MuxCatalogMsfConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ track: consuming TrackSubscriber) throws -> MuxCatalogMsfConsumerExt
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> HangCatalogExt?
}
public struct MuxCodecAacConfig: ~Copyable, @unchecked Sendable {
	@inlinable public var profileGet: Swift.UInt8 { get }
	@inlinable public mutating func profileSet(_ value: Swift.UInt8) throws
	@inlinable public var sampleRateGet: Swift.UInt32 { get }
	@inlinable public mutating func sampleRateSet(_ value: Swift.UInt32) throws
	@inlinable public var channelCountGet: Swift.UInt32 { get }
	@inlinable public mutating func channelCountSet(_ value: Swift.UInt32) throws
	@inlinable public static func new(_ profile: Swift.UInt8, _ sampleRate: Swift.UInt32, _ channelCount: Swift.UInt32) -> MuxCodecAacConfig
	@inlinable public static func parse(_ buf: inout Bytes) throws -> MuxCodecAacConfig
	@inlinable public func encode() throws -> Bytes
}
public final class MuxCodecAacError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecAacError
}
public final class MuxCodecAacImport {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecAacImport
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public final class MuxCodecAnnexbError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecAnnexbError
}
public final class MuxCodecAv1Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecAv1Import
	@inlinable public func initialize(_ buf: Bytes) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func decode(_ frames: [MuxContainerFrame]) throws
}
public struct MuxCodecAv1Split: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecAv1Split
	@inlinable public static func new() -> MuxCodecAv1Split
	@inlinable public mutating func decode(_ data: Bytes, _ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func flush(_ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func reset() throws
}
public final class MuxCodecFlacImport {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecFlacImport
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
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
public final class MuxCodecH264Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH264Import
	@inlinable public func initialize(_ buf: Bytes) throws
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func observeReorder(_ reorder: Timestamp) throws
	@inlinable public func decode(_ frames: [MuxContainerFrame]) throws
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
public struct MuxCodecH265Sps: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxCodecH265Sps
	@inlinable public static func parse(_ nal: Bytes) throws -> MuxCodecH265Sps
	@inlinable public func size() -> (Swift.UInt32, Swift.UInt32)
	@inlinable public func videoSignal() -> MuxCodecH265VideoSignal?
}
public final class MuxCodecH265Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecH265Import
	@inlinable public func initialize(_ buf: Bytes) throws
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func track() throws -> TrackProducer
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func reportSkipped(_ frames: Swift.UInt64) throws
	@inlinable public func reportBitrate(_ bitrate: BandwidthRate) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func observeReorder(_ reorder: Timestamp) throws
	@inlinable public func decode(_ frames: [MuxContainerFrame]) throws
}
public struct MuxCodecH265Split: ~Copyable, @unchecked Sendable {
	@inlinable public static func `default`() -> MuxCodecH265Split
	@inlinable public static func new() -> MuxCodecH265Split
	@inlinable public mutating func decode(_ data: Bytes, _ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func flush(_ pts: Timestamp?) throws -> [MuxContainerFrame]
	@inlinable public mutating func reset() throws
}
public final class MuxCodecMp3Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecMp3Import
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
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
public final class MuxCodecOpusImport {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ config: consuming HangCatalogAudioConfig) throws -> MuxCodecOpusImport
	@inlinable public func name() throws -> Swift.String
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerAudioConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
}
public final class MuxCodecVp8Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp8Import
	@inlinable public func initialize(_ buf: Bytes) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
}
public final class MuxCodecVp9Import {
	@inlinable public static func newExtra(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExtra, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public static func newExt(_ track: TrackProducer, _ reserved: consuming MuxCatalogReservedExt, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public static func newUnit(_ track: TrackProducer, _ reserved: consuming MuxCatalogReserved, _ hint: consuming MuxCatalogVideoHint) throws -> MuxCodecVp9Import
	@inlinable public func initialize(_ buf: Bytes) throws
	@inlinable public func decode(_ frame: Bytes, _ pts: Timestamp?) throws
	@inlinable public func demand() throws -> TrackDemand
	@inlinable public func finish() throws -> MuxContainerFinishedContainerVideoConfig
	@inlinable public func abort(_ err: Error) throws
	@inlinable public func abortUnwanted(_ err: Error) throws
	@inlinable public func cut(_ end: Timestamp?) throws
	@inlinable public func discontinuity() throws
	@inlinable public func setEnabled(_ enabled: Swift.Bool) throws
	@inlinable public func seek(_ start: Swift.UInt64) throws
}
public struct MuxContainerFlvImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerFlvImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerFlvFinished
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerFlvImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerFlvFinishedExtra
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerFlvImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerFlvImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerFlvFinishedExt
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvFinished: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvFinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFlvFinishedExt: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFmp4Wire: ~Copyable, @unchecked Sendable {
	@inlinable public static func fromInit(_ initData: Bytes) throws -> MuxContainerFmp4Wire
}
public final class MuxContainerFmp4Error: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxContainerFmp4Error
}
public struct MuxContainerFmp4Import: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerFmp4Import
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4Import
	@inlinable public consuming func withCmaf(_ cmaf: Swift.Bool) throws -> MuxContainerFmp4Import
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerFmp4Finished
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public mutating func rejoin() throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4ImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerFmp4ImportExtra
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4ImportExtra
	@inlinable public consuming func withCmaf(_ cmaf: Swift.Bool) throws -> MuxContainerFmp4ImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerFmp4FinishedExtra
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public mutating func rejoin() throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4ImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerFmp4ImportExt
	@inlinable public mutating func cut() throws
	@inlinable public consuming func withSelect(_ select: consuming MuxSelectBroadcast) throws -> MuxContainerFmp4ImportExt
	@inlinable public consuming func withCmaf(_ cmaf: Swift.Bool) throws -> MuxContainerFmp4ImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerFmp4FinishedExt
	@inlinable public consuming func abort(_ err: Error) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public mutating func rejoin() throws
	@inlinable public mutating func discontinuity() throws
}
public struct MuxContainerFmp4Finished: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFmp4FinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFmp4FinishedExt: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerFmp4Muxer: ~Copyable, @unchecked Sendable {
	@inlinable public static func video(_ config: borrowing HangCatalogVideoConfig) throws -> MuxContainerFmp4Muxer
	@inlinable public static func audio(_ config: borrowing HangCatalogAudioConfig) throws -> MuxContainerFmp4Muxer
	@inlinable public func timescale() throws -> Timescale
	@inlinable public consuming func withTimescale(_ timescale: Timescale) throws -> MuxContainerFmp4Muxer
	@inlinable nonisolated(nonsending) public mutating func read(_ group: inout GroupConsumer) async throws -> [MuxContainerFrame]
	@inlinable public mutating func pictureSize(_ keyframe: MuxContainerFrame) throws -> (Swift.UInt32, Swift.UInt32)?
	@inlinable public func init_() throws -> Bytes?
	@inlinable public func fragment(_ sequence: Swift.UInt32, _ frames: [MuxContainerFrame], _ end: Timestamp?) throws -> Bytes
}
public final class MuxContainerLegacyAudio: @unchecked Sendable, CustomDebugStringConvertible, Equatable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxContainerLegacyAudio
	@inlinable public static func == (a: MuxContainerLegacyAudio, b: MuxContainerLegacyAudio) -> Swift.Bool
	@inlinable public static func new(_ config: borrowing HangCatalogAudioConfig) throws -> MuxContainerLegacyAudio?
	@inlinable public func duration(_ payload: Bytes) -> Timestamp?
}
public struct MuxContainerMkvImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerMkvImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerMkvFinished
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerMkvImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerMkvFinishedExtra
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerMkvImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerMkvImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ start: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerMkvFinishedExt
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvFinished: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvFinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMkvFinishedExt: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerMp4Finish: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxContainerMp4Finish
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
	@inlinable public func passthroughGet() throws -> MuxContainerTsPassthroughTrack?
	@inlinable public mutating func passthroughSet(_ value: consuming MuxContainerTsPassthroughTrack?) throws
	@inlinable public func isEmpty() throws -> Swift.Bool
}
public struct MuxContainerTsPassthroughTrack: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsPassthroughTrack
	@inlinable public static func == (a: borrowing MuxContainerTsPassthroughTrack, b: borrowing MuxContainerTsPassthroughTrack) throws -> Swift.Bool
	@inlinable public var trackGet: Swift.String { get }
	@inlinable public mutating func trackSet(_ value: Swift.String) throws
	@inlinable public var randomAccessGet: Swift.Bool { get }
	@inlinable public mutating func randomAccessSet(_ value: Swift.Bool) throws
	@inlinable public static func new(_ track: Swift.String) -> MuxContainerTsPassthroughTrack
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
public final class MuxContainerTsError: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MuxContainerTsError
}
public final class MuxContainerTsExportConsumerExtraStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsExportConsumerExtraStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsExportConsumerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumerExtra, _ delay: RustDuration) throws -> MuxContainerTsExportConsumerExtra
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumerExtra
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumerExtra) throws -> MuxContainerTsExportConsumerExtra
	@inlinable public consuming func stream() -> MuxContainerTsExportConsumerExtraStream
}
public final class MuxContainerTsExportConsumerExtStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsExportConsumerExtStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsExportConsumerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumerExt, _ delay: RustDuration) throws -> MuxContainerTsExportConsumerExt
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumerExt
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumerExt) throws -> MuxContainerTsExportConsumerExt
	@inlinable public consuming func stream() -> MuxContainerTsExportConsumerExtStream
}
public final class MuxContainerTsExportConsumerStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsExportConsumerStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsExportConsumer: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ source: consuming MuxSource, _ catalog: consuming MuxCatalogConsumer, _ delay: RustDuration) throws -> MuxContainerTsExportConsumer
	@inlinable public consuming func withMuxRate(_ muxRate: Swift.UInt64) throws -> MuxContainerTsExportConsumer
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public func stats() throws -> MuxContainerTsStatsExport
	@inlinable public func discontinuity() throws -> Swift.UInt64
	@inlinable public consuming func follow(_ broadcast: consuming BroadcastConsumer, _ catalog: consuming MuxCatalogConsumer) throws -> MuxContainerTsExportConsumer
	@inlinable public consuming func stream() -> MuxContainerTsExportConsumerStream
}
public final class MuxContainerTsFollowerStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsFollowerStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsFollower: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumer) throws -> MuxContainerTsFollower
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollower
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollower
	@inlinable public func export() throws -> MuxContainerTsExportConsumer
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public consuming func stream() -> MuxContainerTsFollowerStream
}
public final class MuxContainerTsFollowerExtraStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsFollowerExtraStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsFollowerExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumerExtra) throws -> MuxContainerTsFollowerExtra
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollowerExtra
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollowerExtra
	@inlinable public func export() throws -> MuxContainerTsExportConsumerExtra
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public consuming func stream() -> MuxContainerTsFollowerExtraStream
}
public final class MuxContainerTsFollowerExtStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsFollowerExtStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsFollowerExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ export: consuming MuxContainerTsExportConsumerExt) throws -> MuxContainerTsFollowerExt
	@inlinable public consuming func withFollow(_ follow: RustDuration) throws -> MuxContainerTsFollowerExt
	@inlinable public consuming func withStitch(_ stitch: Swift.Bool) throws -> MuxContainerTsFollowerExt
	@inlinable public func export() throws -> MuxContainerTsExportConsumerExt
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public consuming func stream() -> MuxContainerTsFollowerExtStream
}
public struct MuxContainerTsPassthrough: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerTsPassthrough
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthrough
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsPassthroughFinished
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughUnit: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerTsPassthroughUnit
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthroughUnit
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsPassthroughFinishedUnit
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerTsPassthroughExtra
	@inlinable public consuming func withPcrPid(_ pid: Swift.UInt16) throws -> MuxContainerTsPassthroughExtra
	@inlinable public func track() throws -> Swift.String
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsPassthroughFinishedExtra
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPrograms: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ origin: consuming OriginProducer, _ name: consuming Path, _ config: consuming MuxCatalogConfig) throws -> MuxContainerTsPrograms
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func finish() throws -> MuxContainerTsProgramsFinished
	@inlinable public consuming func abort(_ err: Error) throws
}
public final class MuxContainerTsReleaseStream: AsyncSequence, AsyncIteratorProtocol {
	public typealias Element = MuxContainerFrame
	public typealias Failure = any Swift.Error
	@inlinable public func makeAsyncIterator() -> MuxContainerTsReleaseStream
	@inlinable nonisolated(nonsending) public func next() async throws -> MuxContainerFrame?
	@inlinable public func next(isolation actor: isolated (any Actor)?) async throws -> MuxContainerFrame?
}
public struct MuxContainerTsRelease: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func new(_ source: consuming MuxSource, _ catalogFormat: MuxCatalogCatalogFormat, _ delay: RustDuration) async throws -> MuxContainerTsRelease
	@inlinable public func dropped() throws -> Swift.UInt64
	@inlinable public func drift() throws -> Swift.Double?
	@inlinable public func outOfTolerance() throws -> Swift.UInt64
	@inlinable public func stats() throws -> MuxContainerTsStatsRelease
	@inlinable nonisolated(nonsending) public mutating func next() async throws -> MuxContainerFrame?
	@inlinable public consuming func stream() -> MuxContainerTsReleaseStream
}
public struct MuxContainerTsRepublish: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ origin: consuming OriginProducer, _ name: consuming Path, _ instance: consuming Instance, _ config: consuming MuxCatalogConfig) throws -> MuxContainerTsRepublish
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsRepublish
	@inlinable public func broadcast() throws -> BroadcastProducer
	@inlinable public func catalog() throws -> MuxCatalogProducerExt
	@inlinable public func instance() throws -> Instance
	@inlinable public mutating func decode(_ data: Bytes) throws -> MuxContainerTsRepublished?
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func finish() throws -> MuxContainerTsRepublishFinished
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsRepublished: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MuxContainerTsRepublished
	@inlinable public var stepGet: MuxContainerTsStep { get }
	@inlinable public mutating func stepSet(_ value: MuxContainerTsStep) throws
	@inlinable public func statsGet() throws -> MuxContainerTsStatsSnapshot
	@inlinable public mutating func statsSet(_ value: consuming MuxContainerTsStatsSnapshot) throws
}
public struct MuxContainerTsImport: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReserved) throws -> MuxContainerTsImport
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImport
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImport
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsFinished
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsImportExtra: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExtra) throws -> MuxContainerTsImportExtra
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImportExtra
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImportExtra
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsFinishedExtra
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsImportExt: ~Copyable, @unchecked Sendable {
	@inlinable public static func new(_ broadcast: consuming BroadcastProducer, _ reserved: consuming MuxCatalogReservedExt) throws -> MuxContainerTsImportExt
	@inlinable public consuming func withProgram(_ program: Swift.UInt16) throws -> MuxContainerTsImportExt
	@inlinable public consuming func withContainer(_ container: consuming HangCatalogContainer) throws -> MuxContainerTsImportExt
	@inlinable public mutating func decode(_ data: Bytes) throws
	@inlinable public mutating func seek(_ sequence: Swift.UInt64) throws
	@inlinable public consuming func finish() throws -> MuxContainerTsFinishedExt
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsFinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsFinishedExt: ~Copyable, @unchecked Sendable {
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxImportContainerFinished: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxImportContainerFinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxImportContainerFinishedExt: ~Copyable, @unchecked Sendable {
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxImportTrackFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func name() throws -> Swift.String
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
public struct MuxContainerTsPassthroughFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func track() throws -> Swift.String
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughFinishedUnit: ~Copyable, @unchecked Sendable {
	@inlinable public func track() throws -> Swift.String
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsPassthroughFinishedExtra: ~Copyable, @unchecked Sendable {
	@inlinable public func track() throws -> Swift.String
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsProgramsFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func broadcasts() throws -> UniqueArray<BroadcastProducer>
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
}
public struct MuxContainerTsRepublishFinished: ~Copyable, @unchecked Sendable {
	@inlinable public func broadcast() throws -> BroadcastProducer
	@inlinable public func stats() throws -> MuxContainerTsStatsSnapshot
	@inlinable public consuming func abort(_ err: Error) throws
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
public final class MoqTokioCliBound: @unchecked Sendable, CustomStringConvertible, CustomDebugStringConvertible, Hashable {
	@inlinable public var description: Swift.String { get }
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioCliBound
	@inlinable public static func == (a: MoqTokioCliBound, b: MoqTokioCliBound) -> Swift.Bool
	@inlinable public func hash(into hasher: inout Swift.Hasher)
	@inlinable public static var NONE: MoqTokioCliBound { get }
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
public struct MoqJsonTimedCatalog: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqJsonTimedCatalog
	@inlinable public static func == (a: borrowing MoqJsonTimedCatalog, b: borrowing MoqJsonTimedCatalog) throws -> Swift.Bool
	@inlinable public func valueGet() throws -> HangCatalog
	@inlinable public mutating func valueSet(_ value: consuming HangCatalog) throws
	@inlinable public var atGet: Timestamp { get }
	@inlinable public mutating func atSet(_ value: Timestamp) throws
	@inlinable public static func new(_ value: consuming HangCatalog, _ at: Timestamp) throws -> MoqJsonTimedCatalog
}
public struct MoqJsonTimedJson: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqJsonTimedJson
	@inlinable public static func == (a: borrowing MoqJsonTimedJson, b: borrowing MoqJsonTimedJson) throws -> Swift.Bool
	@inlinable public func valueGet() throws -> Json
	@inlinable public mutating func valueSet(_ value: consuming Json) throws
	@inlinable public var atGet: Timestamp { get }
	@inlinable public mutating func atSet(_ value: Timestamp) throws
	@inlinable public static func new(_ value: consuming Json, _ at: Timestamp) throws -> MoqJsonTimedJson
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
	@inlinable public var timeoutGet: RustDuration? { get }
	@inlinable public mutating func timeoutSet(_ value: RustDuration?) throws
}
public struct MoqTokioConnectionHandover: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectionHandover
	@inlinable public static func `default`() -> MoqTokioConnectionHandover
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
	@inlinable public var connectsGet: Swift.UInt64 { get }
	@inlinable public mutating func connectsSet(_ value: Swift.UInt64) throws
	@inlinable public var lastConnectedGet: ClockInstant? { get }
	@inlinable public mutating func lastConnectedSet(_ value: ClockInstant?) throws
	@inlinable public func phaseGet() throws -> MoqTokioConnectionPhase
	@inlinable public mutating func phaseSet(_ value: consuming MoqTokioConnectionPhase) throws
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
	@inlinable public var idleTimeoutGet: RustDuration? { get }
	@inlinable public mutating func idleTimeoutSet(_ value: RustDuration?) throws
	@inlinable public var keepAliveGet: RustDuration? { get }
	@inlinable public mutating func keepAliveSet(_ value: RustDuration?) throws
	@inlinable public var mtuDiscoveryGet: Swift.Bool { get }
	@inlinable public mutating func mtuDiscoverySet(_ value: Swift.Bool) throws
	@inlinable public var congestionControlGet: MoqTokioQuicCongestionControl { get }
	@inlinable public mutating func congestionControlSet(_ value: MoqTokioQuicCongestionControl) throws
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
	@inlinable public func keepAliveInterval() throws -> RustDuration
	@inlinable public func limits() throws -> MoqQuicLimits
}
public struct MoqTokioWebsocketListener: ~Copyable, @unchecked Sendable {
	@inlinable nonisolated(nonsending) public static func bind(_ addr: SocketAddr) async throws -> MoqTokioWebsocketListener
	@inlinable public consuming func withProtocols(_ protocols: [Swift.String]) throws -> MoqTokioWebsocketListener
	@inlinable public consuming func withQuic(_ quic: borrowing MoqTokioQuicResolved) throws -> MoqTokioWebsocketListener
	@inlinable public func localAddr() throws -> SocketAddr
	@inlinable public func acceptHealth() throws -> MoqTokioAcceptHealth
	@inlinable nonisolated(nonsending) public func accept() async throws -> QmuxSession
}
public struct MoqTokioQuicDemuxDemux: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioQuicDemuxDemux
	@inlinable public static func `default`() -> MoqTokioQuicDemuxDemux
	@inlinable public func counters() throws -> MoqSockDemuxCounters
	@inlinable public func webrtc() throws -> MoqTokioQuicDemuxWebRtc
}
public struct MoqTokioServerStreams: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public static func `default`() -> MoqTokioServerStreams
}
public struct MoqTokioServerQuicLimits: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() -> MoqTokioServerQuicLimits
	@inlinable public func set(_ limits: MoqQuicLimits) throws
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
public struct MoqTokioConnectAddr: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectAddr
	@inlinable public static func == (a: borrowing MoqTokioConnectAddr, b: borrowing MoqTokioConnectAddr) throws -> Swift.Bool
	@inlinable public static func new(_ url: consuming Url) throws -> MoqTokioConnectAddr
	@inlinable public static func pinned(_ url: consuming Url, _ addrs: [SocketAddr]) throws -> MoqTokioConnectAddr
	@inlinable public func url() throws -> Url
	@inlinable public func addresses() throws -> [SocketAddr]?
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
public struct MoqTokioConnectionPhase: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> MoqTokioConnectionPhase
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
public struct QmuxSession: ~Copyable, @unchecked Sendable {
	@inlinable public var debugDescription: Swift.String { get }
	@inlinable public func clone() throws -> QmuxSession
	@inlinable public func `protocol`() throws -> Swift.String?
	@inlinable public func version() throws -> MoqQuicConnectionMuxVersion
	@inlinable public func setLimits(_ limits: MoqQuicLimits) throws
	@inlinable public func peerCertificates() throws -> UniqueArray<RustlsPkiTypesCertificateDer>?
	@inlinable public func serverName() throws -> Swift.String?
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
	@inlinable nonisolated(nonsending) public func recv() async throws -> MoqTokioQuicDemuxDatagram?
	@inlinable public func pin(_ remote: SocketAddr) -> MoqSockDemuxPin
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
extension HangError: Swift.Error {}
extension IntersectionError: Swift.Error {}
extension InvalidPattern: Swift.Error {}
extension MoqTokioConnectError: Swift.Error {}
extension MoqTokioWebsocketError: Swift.Error {}
extension MuxCodecAacAdtsError: Swift.Error {}
extension MuxCodecAacError: Swift.Error {}
extension MuxCodecAacImport: Swift.Error, @unchecked Sendable {}
extension MuxCodecAnnexbError: Swift.Error {}
extension MuxCodecAv1Import: Swift.Error, @unchecked Sendable {}
extension MuxCodecFlacError: Swift.Error {}
extension MuxCodecFlacImport: Swift.Error, @unchecked Sendable {}
extension MuxCodecH264Error: Swift.Error {}
extension MuxCodecH264Import: Swift.Error, @unchecked Sendable {}
extension MuxCodecH265Error: Swift.Error {}
extension MuxCodecH265Import: Swift.Error, @unchecked Sendable {}
extension MuxCodecMp3Error: Swift.Error {}
extension MuxCodecMp3Import: Swift.Error, @unchecked Sendable {}
extension MuxCodecOpusError: Swift.Error {}
extension MuxCodecOpusImport: Swift.Error, @unchecked Sendable {}
extension MuxCodecVp8Import: Swift.Error, @unchecked Sendable {}
extension MuxCodecVp9Import: Swift.Error, @unchecked Sendable {}
extension MuxContainerFmp4Error: Swift.Error {}
extension MuxContainerProducerContainer: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerContainerAudioConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerContainerBinaryConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerContainerJsonConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerContainerTextConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerContainerVideoConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4Wire: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4WireAudioConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4WireBinaryConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4WireJsonConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4WireTextConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerFmp4WireVideoConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWire: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWireAudioConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWireBinaryConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWireJsonConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWireTextConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLegacyWireVideoConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWire: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWireAudioConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWireBinaryConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWireJsonConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWireTextConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerProducerLocWireVideoConfig: Swift.Error, @unchecked Sendable {}
extension MuxContainerTsError: Swift.Error {}
extension MuxError: Swift.Error {}
extension MuxImportTrack: Swift.Error, @unchecked Sendable {}
extension MuxImportTrackStream: Swift.Error, @unchecked Sendable {}
extension MuxJsonSnapshotJson: Swift.Error {}
extension MuxJsonStreamJson: Swift.Error {}
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
public let HANG_CATALOG_MAX_SAFE_INTEGER: Swift.UInt64
@inlinable public var IMPLEMENTATION: Swift.String { get }
@inlinable public var ALPNS: [Swift.String] { get }
@inlinable public func clockNow() -> ClockInstant
@inlinable public func clockWall() -> ClockWall
@inlinable public func clockOrigin() -> (ClockInstant, ClockWall)
@inlinable public func clockRng() -> ClockRng
@inlinable public func clockSeed() -> Swift.UInt64
@inlinable public func clockHostTime()
@inlinable nonisolated(nonsending) public func clockDrive(_ poll: @escaping @Sendable (ClockInstant, borrowing Waiter) throws -> ClockInstant?) async throws -> Swift.String
@inlinable public func clockInterval(_ period: RustDuration) -> ClockInterval
@inlinable public func clockIntervalAt(_ start: ClockInstant, _ period: RustDuration) -> ClockInterval
@inlinable nonisolated(nonsending) public func clockSleep(_ duration: RustDuration) async throws
@inlinable nonisolated(nonsending) public func clockSleepUntil(_ at: ClockInstant) async throws
@inlinable nonisolated(nonsending) public func clockTimeout(_ duration: RustDuration, _ future: @escaping @Sendable () async -> Void) async throws
@inlinable nonisolated(nonsending) public func clockTimeoutAt(_ deadline: ClockInstant, _ future: @escaping @Sendable () async -> Void) async throws
@inlinable public var SERVERDEFAULTTIMEOUT: RustDuration { get }
@inlinable nonisolated(nonsending) public func timeRunDriver(_ driver: consuming Driver) async throws -> Error
@inlinable nonisolated(nonsending) public func timeRunOriginDriver(_ driver: consuming OriginDriver) async throws -> Error
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
@inlinable public var MUXCONTAINERDELAYMAX: RustDuration { get }
@inlinable public var MUXRATERETRY: RustDuration { get }
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
