// moq.kt
fun interface Waker {
	fun wake()
}
class Bytes private constructor(private val array: ByteArray?, private val lent: _Native.__Lent?, private val offset: Int, val size: Int) {
	constructor(bytes: ByteArray)
	operator fun get(index: Int): Byte
	fun slice(startIndex: Int, endIndex: Int = size): Bytes
	fun copyInto(destination: ByteArray, destinationOffset: Int = 0, startIndex: Int = 0, endIndex: Int = size): ByteArray
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
}
class Duration(val secs: ULong, val nanos: UInt = 0u) : kotlin.Comparable<Duration> {
	constructor(duration: kotlin.time.Duration)
	fun toKotlinDuration(): kotlin.time.Duration
	override fun compareTo(other: Duration): Int
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
	companion object {
		val ZERO = Duration(0u)
		val MAX = Duration(ULong.MAX_VALUE, NANOS - 1u)
	}
}
sealed interface IpAddr {
	fun toInetAddress(): java.net.InetAddress
	companion object {
		fun of(address: java.net.InetAddress): IpAddr
	}
}
@kotlin.jvm.JvmInline value class Ipv4Addr(val bits: UInt) : IpAddr {
	constructor(octets: ByteArray)
	constructor(a: UByte, b: UByte, c: UByte, d: UByte)
	fun octets(): ByteArray
	override fun toInetAddress(): java.net.InetAddress
	override fun toString(): String
}
class Ipv6Addr internal constructor(internal val hi: Long, internal val lo: Long) : IpAddr {
	constructor(octets: ByteArray)
	constructor(address: java.net.Inet6Address)
	fun octets(): ByteArray
	override fun toInetAddress(): java.net.Inet6Address
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
sealed interface SocketAddr {
	val port: UShort
	fun toInetSocketAddress(): java.net.InetSocketAddress
	companion object {
		fun of(address: java.net.InetSocketAddress): SocketAddr
	}
}
@kotlin.jvm.JvmInline value class SocketAddrV4 private constructor(private val bits: Long) : SocketAddr {
	constructor(ip: Ipv4Addr, port: UShort)
	constructor(address: java.net.InetSocketAddress)
	val ip: Ipv4Addr
	override val port: UShort
	override fun toInetSocketAddress(): java.net.InetSocketAddress
	override fun toString(): String
}
class SocketAddrV6(val ip: Ipv6Addr, override val port: UShort, val flowinfo: UInt = 0u, val scopeId: UInt = 0u) : SocketAddr {
	constructor(address: java.net.InetSocketAddress)
	override fun toInetSocketAddress(): java.net.InetSocketAddress
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class PathBuf private constructor(bytes: ByteArray, given: Boolean) {
	constructor(bytes: ByteArray)
	constructor(path: String)
	fun toByteArray(): ByteArray
	fun toText(): String
	fun toPath(): java.nio.file.Path
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class Mut<T>(var value: T)
enum class Bound {
	UNBOUNDED,
	INCLUDED,
	EXCLUDED,
}
data class BoundU64(val kind: Bound, val value: ULong)
data class BoundsU64(val start: BoundU64, val end: BoundU64)
data class RangeU64(val start: ULong, val end: ULong)
class ArrayU8x12(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x16(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x2(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x3(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x32(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x4(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
class ArrayU8x6(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
@kotlin.jvm.JvmInline value class TimeOverflow internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class Timescale internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asU64(): kotlin.ULong
	companion object {
		val SECOND: moq.Timescale
		val MILLI: moq.Timescale
		val MICRO: moq.Timescale
		val NANO: moq.Timescale
		fun new(unitsPerSecond: kotlin.ULong): moq.Timescale
	}
}
class Timestamp internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun value(): kotlin.ULong
	fun scale(): moq.Timescale
	fun isZero(): kotlin.Boolean
	fun convert(newScale: moq.Timescale): moq.Timestamp
	fun asScale(target: moq.Timescale): java.math.BigInteger
	fun asSecs(): kotlin.ULong
	fun asMillis(): java.math.BigInteger
	fun asMicros(): java.math.BigInteger
	fun asNanos(): java.math.BigInteger
	fun checkedAdd(rhs: moq.Timestamp): moq.Timestamp
	fun checkedSub(rhs: moq.Timestamp): moq.Timestamp
	companion object {
		val ZERO: moq.Timestamp
		fun new(value: kotlin.ULong, scale: moq.Timescale): moq.Timestamp
		fun fromScale(value: kotlin.ULong, unitsPerSecond: kotlin.ULong): moq.Timestamp
		fun fromSecs(seconds: kotlin.ULong): moq.Timestamp
		fun fromMillis(millis: kotlin.ULong): moq.Timestamp
		fun fromMicros(micros: kotlin.ULong): moq.Timestamp
		fun fromNanos(nanos: kotlin.ULong): moq.Timestamp
		fun now(): moq.Timestamp
	}
}
data class Datagram(val sequence: kotlin.ULong, val timestamp: moq.Timestamp?, val payload: moq.Bytes) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class Hop internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun id(): kotlin.ULong
	companion object {
		val UNKNOWN: moq.Hop
		fun new(id: kotlin.ULong): moq.Hop
		fun random(): moq.Hop
	}
}
@kotlin.jvm.JvmInline value class ClockInstant internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun elapsed(): moq.Duration
	fun durationSince(earlier: moq.ClockInstant): moq.Duration
	fun checkedDurationSince(earlier: moq.ClockInstant): moq.Duration?
	fun saturatingDurationSince(earlier: moq.ClockInstant): moq.Duration
	fun checkedAdd(duration: moq.Duration): moq.ClockInstant?
	fun checkedSub(duration: moq.Duration): moq.ClockInstant?
	fun intoStd(): moq.Instant
	companion object {
		val ZERO: moq.ClockInstant
		fun now(): moq.ClockInstant
		fun fromStd(at: moq.Instant): moq.ClockInstant
	}
}
@kotlin.jvm.JvmInline value class ClockElapsed internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class ClockWall internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun checkedDurationSince(earlier: moq.ClockWall): moq.Duration?
	fun saturatingDurationSince(earlier: moq.ClockWall): moq.Duration
	fun checkedAdd(duration: moq.Duration): moq.ClockWall?
	fun saturatingAdd(duration: moq.Duration): moq.ClockWall
	fun checkedSub(duration: moq.Duration): moq.ClockWall?
	companion object {
		val UNIXEPOCH: moq.ClockWall
		fun now(): moq.ClockWall
	}
}
@kotlin.jvm.JvmInline value class SetupExtensions internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	val authGet: kotlin.Boolean
	fun authSet(value: kotlin.Boolean): moq.SetupExtensions
	val solicitGet: kotlin.Boolean
	fun solicitSet(value: kotlin.Boolean): moq.SetupExtensions
	val ietfGet: kotlin.Boolean
	fun ietfSet(value: kotlin.Boolean): moq.SetupExtensions
	companion object {
		fun default(): moq.SetupExtensions
	}
}
data class SetupToken(val kind: kotlin.ULong, val value: moq.Bytes) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class SessionLimits internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	val announcesGet: kotlin.ULong
	fun announcesSet(value: kotlin.ULong): moq.SessionLimits
	companion object {
		fun default(): moq.SessionLimits
	}
}
class StatsContent internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val bytesGet: kotlin.ULong
	fun bytesSet(value: kotlin.ULong): moq.StatsContent
	val framesGet: kotlin.ULong
	fun framesSet(value: kotlin.ULong): moq.StatsContent
	val groupsGet: kotlin.ULong
	fun groupsSet(value: kotlin.ULong): moq.StatsContent
	companion object {
		fun default(): moq.StatsContent
	}
}
class StatsTraffic internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val announcesStartedGet: kotlin.ULong
	fun announcesStartedSet(value: kotlin.ULong): moq.StatsTraffic
	val announcesEndedGet: kotlin.ULong
	fun announcesEndedSet(value: kotlin.ULong): moq.StatsTraffic
	val announcedBytesGet: kotlin.ULong
	fun announcedBytesSet(value: kotlin.ULong): moq.StatsTraffic
	val broadcastsStartedGet: kotlin.ULong
	fun broadcastsStartedSet(value: kotlin.ULong): moq.StatsTraffic
	val broadcastsEndedGet: kotlin.ULong
	fun broadcastsEndedSet(value: kotlin.ULong): moq.StatsTraffic
	val subscriptionsStartedGet: kotlin.ULong
	fun subscriptionsStartedSet(value: kotlin.ULong): moq.StatsTraffic
	val subscriptionsEndedGet: kotlin.ULong
	fun subscriptionsEndedSet(value: kotlin.ULong): moq.StatsTraffic
	val fetchesGet: kotlin.ULong
	fun fetchesSet(value: kotlin.ULong): moq.StatsTraffic
	val bytesGet: kotlin.ULong
	fun bytesSet(value: kotlin.ULong): moq.StatsTraffic
	val framesGet: kotlin.ULong
	fun framesSet(value: kotlin.ULong): moq.StatsTraffic
	val groupsGet: kotlin.ULong
	fun groupsSet(value: kotlin.ULong): moq.StatsTraffic
	val datagramsGet: kotlin.ULong
	fun datagramsSet(value: kotlin.ULong): moq.StatsTraffic
	val inflatedGet: kotlin.ULong
	fun inflatedSet(value: kotlin.ULong): moq.StatsTraffic
	val staleGet: moq.StatsContent
	fun staleSet(value: moq.StatsContent): moq.StatsTraffic
	fun add(other: moq.StatsTraffic): moq.StatsTraffic
	fun activeBroadcasts(): kotlin.ULong
	fun activeSubscriptions(): kotlin.ULong
	fun isIdle(): kotlin.Boolean
	companion object {
		fun default(): moq.StatsTraffic
	}
}
class StatsPresence internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val sessionsRefusedGet: moq.StatsRefusals
	fun sessionsRefusedSet(value: moq.StatsRefusals): moq.StatsPresence
	val sessionsFailedGet: moq.StatsFailures
	fun sessionsFailedSet(value: moq.StatsFailures): moq.StatsPresence
	val requestsGet: kotlin.ULong
	fun requestsSet(value: kotlin.ULong): moq.StatsPresence
	val sessionsStartedGet: kotlin.ULong
	fun sessionsStartedSet(value: kotlin.ULong): moq.StatsPresence
	val sessionsEndedGet: kotlin.ULong
	fun sessionsEndedSet(value: kotlin.ULong): moq.StatsPresence
	val sentBytesGet: kotlin.ULong
	fun sentBytesSet(value: kotlin.ULong): moq.StatsPresence
	val receivedBytesGet: kotlin.ULong
	fun receivedBytesSet(value: kotlin.ULong): moq.StatsPresence
	val announcesPeakGet: kotlin.ULong
	fun announcesPeakSet(value: kotlin.ULong): moq.StatsPresence
	val subscriptionsPeakGet: kotlin.ULong
	fun subscriptionsPeakSet(value: kotlin.ULong): moq.StatsPresence
	val wireTxGet: kotlin.ULong
	fun wireTxSet(value: kotlin.ULong): moq.StatsPresence
	val wireRxGet: kotlin.ULong
	fun wireRxSet(value: kotlin.ULong): moq.StatsPresence
	fun add(other: moq.StatsPresence): moq.StatsPresence
	fun active(): kotlin.ULong
	companion object {
		fun default(): moq.StatsPresence
	}
}
class StatsTotals internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val txGet: kotlin.ULong
	fun txSet(value: kotlin.ULong): moq.StatsTotals
	val rxGet: kotlin.ULong
	fun rxSet(value: kotlin.ULong): moq.StatsTotals
	companion object {
		fun default(): moq.StatsTotals
	}
}
class StatsFailures internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val idleTimeoutGet: kotlin.ULong
	fun idleTimeoutSet(value: kotlin.ULong): moq.StatsFailures
	val transportGet: kotlin.ULong
	fun transportSet(value: kotlin.ULong): moq.StatsFailures
	val applicationGet: kotlin.ULong
	fun applicationSet(value: kotlin.ULong): moq.StatsFailures
	val expiredGet: kotlin.ULong
	fun expiredSet(value: kotlin.ULong): moq.StatsFailures
	fun count(kind: moq.StatsFailure): kotlin.ULong
	fun add(other: moq.StatsFailures): moq.StatsFailures
	fun regressed(previous: moq.StatsFailures): kotlin.Boolean
	companion object {
		fun default(): moq.StatsFailures
	}
}
class StatsRefusals internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val refusedGet: kotlin.ULong
	fun refusedSet(value: kotlin.ULong): moq.StatsRefusals
	val unavailableGet: kotlin.ULong
	fun unavailableSet(value: kotlin.ULong): moq.StatsRefusals
	val requestGet: kotlin.ULong
	fun requestSet(value: kotlin.ULong): moq.StatsRefusals
	val forbiddenGet: kotlin.ULong
	fun forbiddenSet(value: kotlin.ULong): moq.StatsRefusals
	val lanGet: kotlin.ULong
	fun lanSet(value: kotlin.ULong): moq.StatsRefusals
	val expiredGet: kotlin.ULong
	fun expiredSet(value: kotlin.ULong): moq.StatsRefusals
	fun count(kind: moq.StatsRefusalKind): kotlin.ULong
	fun add(other: moq.StatsRefusals): moq.StatsRefusals
	fun regressed(previous: moq.StatsRefusals): kotlin.Boolean
	companion object {
		fun default(): moq.StatsRefusals
	}
}
class TransportReceiveWindow internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val connectionGet: kotlin.ULong
	fun connectionSet(value: kotlin.ULong): moq.TransportReceiveWindow
	val streamGet: kotlin.ULong
	fun streamSet(value: kotlin.ULong): moq.TransportReceiveWindow
	companion object {
		fun new(connection: kotlin.ULong, stream: kotlin.ULong): moq.TransportReceiveWindow
	}
}
@kotlin.jvm.JvmInline value class BandwidthRate internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun asBps(): kotlin.ULong
	fun scaled(factor: kotlin.Double): moq.BandwidthRate
	fun absDiff(other: moq.BandwidthRate): moq.BandwidthRate
	companion object {
		fun default(): moq.BandwidthRate
		val ZERO: moq.BandwidthRate
		fun fromBps(bps: kotlin.ULong): moq.BandwidthRate
		fun fromKbps(kbps: kotlin.ULong): moq.BandwidthRate
		fun fromMbps(mbps: kotlin.ULong): moq.BandwidthRate
	}
}
data class FrameInfo(val size: kotlin.ULong, val timestamp: moq.Timestamp?) {
	fun debug(): kotlin.String
}
data class GroupInfo(val sequence: kotlin.ULong) {
	fun debug(): kotlin.String
}
data class TrackDelay(val min: moq.Duration?, val max: moq.Duration?) {
	fun debug(): kotlin.String
	fun withMin(min: moq.Duration?): moq.TrackDelay
	fun withMax(max: moq.Duration?): moq.TrackDelay
	companion object {
		fun default(): moq.TrackDelay
	}
}
data class TrackPosition(val group: kotlin.ULong, val frame: kotlin.ULong) {
	fun debug(): kotlin.String
	fun groupEnd(): moq.BoundU64
	companion object {
		fun default(): moq.TrackPosition
		fun group(group: kotlin.ULong): moq.TrackPosition
		fun after(group: kotlin.ULong, frame: kotlin.ULong): moq.TrackPosition?
		fun afterGroup(group: kotlin.ULong): moq.TrackPosition?
	}
}
data class TrackRange(val start: moq.TrackAnchor, val end: moq.TrackAnchor?) {
	fun debug(): kotlin.String
	companion object {
		fun live(): moq.TrackRange
		fun oldest(): moq.TrackRange
		fun groups(groups: moq.BoundsU64): moq.TrackRange
	}
}
@kotlin.jvm.JvmInline value class OriginCost internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun value(): kotlin.ULong
	companion object {
		fun default(): moq.OriginCost
		fun new(cost: kotlin.ULong): moq.OriginCost
		val MAX: moq.OriginCost
		val DRAIN: moq.OriginCost
	}
}
@kotlin.jvm.JvmInline value class OriginHopLimit internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun get(): kotlin.ULong
	companion object {
		fun default(): moq.OriginHopLimit
		fun new(max: kotlin.ULong): moq.OriginHopLimit?
	}
}
@kotlin.jvm.JvmInline value class OriginPreference internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun value(): kotlin.Byte
	companion object {
		fun default(): moq.OriginPreference
		fun new(preference: kotlin.Byte): moq.OriginPreference
	}
}
data class HangHealthLimitsF64(val degraded: kotlin.Double, val unhealthy: kotlin.Double) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class HangHealthLimitsDuration(val degraded: moq.Duration, val unhealthy: moq.Duration) {
	fun debug(): kotlin.String
}
data class HangCatalogAac(val profile: kotlin.UByte) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
class HangCatalogClock internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val wallGet: moq.Timestamp
	fun wallSet(value: moq.Timestamp): moq.HangCatalogClock
	fun wallClock(pts: moq.Timestamp): moq.ClockWall
	companion object {
		fun new(wall: moq.Timestamp): moq.HangCatalogClock
	}
}
@kotlin.jvm.JvmInline value class HangCatalogPriorities internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	val catalogGet: kotlin.Byte
	fun catalogSet(value: kotlin.Byte): moq.HangCatalogPriorities
	val textGet: kotlin.Byte
	fun textSet(value: kotlin.Byte): moq.HangCatalogPriorities
	val audioGet: kotlin.Byte
	fun audioSet(value: kotlin.Byte): moq.HangCatalogPriorities
	val videoGet: kotlin.Byte
	fun videoSet(value: kotlin.Byte): moq.HangCatalogPriorities
}
data class HangCatalogAv1(val profile: kotlin.UByte, val level: kotlin.UByte, val tier: kotlin.Int, val bitdepth: kotlin.UByte, val monoChrome: kotlin.Boolean, val chromaSubsamplingX: kotlin.Boolean, val chromaSubsamplingY: kotlin.Boolean, val chromaSamplePosition: kotlin.UByte, val colorPrimaries: kotlin.UByte, val transferCharacteristics: kotlin.UByte, val matrixCoefficients: kotlin.UByte, val fullRange: kotlin.Boolean) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.HangCatalogAv1
		val PREFIX: kotlin.String
	}
}
data class HangCatalogMastering(val red: moq.HangCatalogChromaticity, val green: moq.HangCatalogChromaticity, val blue: moq.HangCatalogChromaticity, val white: moq.HangCatalogChromaticity, val maxLuminance: kotlin.Double, val minLuminance: kotlin.Double) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class HangCatalogChromaticity(val x: kotlin.Double, val y: kotlin.Double) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class HangCatalogContentLight(val maxCll: kotlin.UShort, val maxFall: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class HangCatalogH264(val inline: kotlin.Boolean, val profile: kotlin.UByte, val constraints: kotlin.UByte, val level: kotlin.UByte) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
data class HangCatalogH265(val inBand: kotlin.Boolean, val profileSpace: kotlin.UByte, val profileIdc: kotlin.UByte, val profileCompatibilityFlags: ArrayU8x4, val tierFlag: kotlin.Boolean, val levelIdc: kotlin.UByte, val constraintFlags: ArrayU8x6) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
data class HangCatalogVp9(val profile: kotlin.UByte, val level: kotlin.UByte, val bitDepth: kotlin.UByte, val chromaSubsampling: kotlin.UByte, val colorPrimaries: kotlin.UByte, val transferCharacteristics: kotlin.UByte, val matrixCoefficients: kotlin.UByte, val fullRange: kotlin.Boolean) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.HangCatalogVp9
	}
}
data class HangCatalogDisplay(val width: kotlin.UInt, val height: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class HangContainerFrame(val timestamp: moq.Timestamp, val payload: moq.Bytes) {
	fun debug(): kotlin.String
	fun encode(buf: moq.Mut<moq.Bytes>)
	fun writeTo(group: moq.GroupProducer)
	companion object {
		fun decode(buf: moq.Bytes): moq.HangContainerFrame
	}
}
data class MuxContainerFrame(val timestamp: moq.Timestamp, val duration: moq.Timestamp?, val payload: moq.Bytes, val keyframe: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class MuxContainerTimestampRewind(val timestamp: moq.Timestamp, val floor: moq.Timestamp) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
data class MuxSelectSelected(val video: kotlin.collections.List<kotlin.String>, val audio: kotlin.collections.List<kotlin.String>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.MuxSelectSelected
		fun ofExtra(catalog: moq.HangCatalogExtra): moq.MuxSelectSelected
		fun ofExt(catalog: moq.HangCatalogExt): moq.MuxSelectSelected
		fun ofUnit(catalog: moq.HangCatalog): moq.MuxSelectSelected
	}
}
@kotlin.jvm.JvmInline value class MuxTimelineTransition internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
}
data class MuxTimelineAudio(val end: moq.Duration, val buffered: moq.Duration, val target: moq.Duration) {
	fun debug(): kotlin.String
}
data class MuxCodecAacConfig(val profile: kotlin.UByte, val sampleRate: kotlin.UInt, val channelCount: kotlin.UInt) {
	fun encode(): moq.Bytes
	companion object {
		fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecAacConfig
	}
}
data class MuxCodecFlacConfig(val minBlockSize: kotlin.UShort, val maxBlockSize: kotlin.UShort, val minFrameSize: kotlin.UInt, val maxFrameSize: kotlin.UInt, val sampleRate: kotlin.UInt, val channelCount: kotlin.UInt, val bitsPerSample: kotlin.UInt, val totalSamples: kotlin.ULong, val md5: ArrayU8x16) {
	fun debug(): kotlin.String
	fun encodeStreamInfo(): moq.Bytes
	fun description(): moq.Bytes
	companion object {
		fun parseStreamInfo(buf: moq.Mut<moq.Bytes>): moq.MuxCodecFlacConfig
		fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecFlacConfig
	}
}
data class MuxCodecH264Sps(val profile: kotlin.UByte, val constraints: kotlin.UByte, val level: kotlin.UByte, val codedWidth: kotlin.UInt, val codedHeight: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun parse(nal: moq.Bytes): moq.MuxCodecH264Sps
	}
}
data class MuxCodecMp3Config(val sampleRate: kotlin.UInt, val channelCount: kotlin.UInt) {
	companion object {
		fun parse(data: moq.Bytes): moq.MuxCodecMp3Config
	}
}
class MuxCodecOpusMapping internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun family(): kotlin.UByte
	fun streams(): kotlin.UByte
	fun coupled(): kotlin.UByte
	fun table(): moq.Bytes
}
data class MuxContainerFmp4Fragment(val data: moq.Bytes, val independent: kotlin.Boolean, val duration: moq.Duration) {
	fun debug(): kotlin.String
}
data class MuxContainerLegacyWire(val _0: moq.MuxContainerKind)
data class MuxContainerLocWire(val _0: moq.MuxContainerKind)
data class MuxContainerMp4Finish(val moovOffset: kotlin.ULong, val moov: moq.Bytes, val mdatOffset: kotlin.ULong, val mdat: moq.Bytes) {
	fun debug(): kotlin.String
}
data class MuxContainerTsDescriptor(val tag: kotlin.UByte, val data: moq.Bytes) {
	fun debug(): kotlin.String
}
class MuxContainerFmp4FragmentConfig internal constructor() {
	fun debug(): kotlin.String
	val missingDurationGet: moq.MuxContainerFmp4FragmentMissingDuration
	fun missingDurationSet(value: moq.MuxContainerFmp4FragmentMissingDuration): moq.MuxContainerFmp4FragmentConfig
	companion object {
		fun default(): moq.MuxContainerFmp4FragmentConfig
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomFourCc internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun new(value: ArrayU8x4): moq.Mp4AtomFourCc
	}
}
data class Mp4AtomMp4a(val audio: moq.Mp4AtomAudio, val esds: moq.Mp4AtomEsds, val btrt: moq.Mp4AtomBtrt?, val taic: moq.Mp4AtomTaic?) {
	fun debug(): kotlin.String
}
data class Mp4AtomMp3(val audio: moq.Mp4AtomAudio) {
	fun debug(): kotlin.String
}
data class Mp4AtomOpus(val audio: moq.Mp4AtomAudio, val dops: moq.Mp4AtomDops, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
}
data class Mp4AtomFlac(val audio: moq.Mp4AtomAudio, val dfla: moq.Mp4AtomDfla) {
	fun debug(): kotlin.String
}
data class Mp4AtomAc3(val audio: moq.Mp4AtomAudio, val dac3: moq.Mp4AtomAc3SpecificBox) {
	fun debug(): kotlin.String
}
data class Mp4AtomEac3(val audio: moq.Mp4AtomAudio, val dec3: moq.Mp4AtomEc3SpecificBox) {
	fun debug(): kotlin.String
}
data class Mp4AtomIpcm(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomFpcm(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomSowt(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomTwos(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomLpcm(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomIn24(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomIn32(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomFl32(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomFl64(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomS16l(val audio: moq.Mp4AtomAudio, val pcmc: moq.Mp4AtomPcmC?, val chnl: moq.Mp4AtomChnl?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
	fun format(): moq.Mp4AtomPcmFormat?
}
data class Mp4AtomWvtt(val plaintext: moq.Mp4AtomPlainText, val config: moq.Mp4AtomVttC, val label: moq.Mp4AtomVlab?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
}
data class Mp4AtomSamr(val amrsampleentry: moq.Mp4AtomAmrSampleEntry, val damr: moq.Mp4AtomDamr) {
	fun debug(): kotlin.String
}
data class Mp4AtomMett(val metadata: moq.Mp4AtomMetaData, val contentEncoding: kotlin.String, val mimeFormat: kotlin.String, val btrt: moq.Mp4AtomBtrt?, val txtc: moq.Mp4AtomTxtC?) {
	fun debug(): kotlin.String
}
data class Mp4AtomMetx(val metadata: moq.Mp4AtomMetaData, val contentEncoding: kotlin.String, val namespace: kotlin.String, val schemaLocation: kotlin.String, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
}
data class Mp4AtomUrim(val metadata: moq.Mp4AtomMetaData, val theLabel: moq.Mp4AtomUri, val init: moq.Mp4AtomUriI?, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
}
data class Mp4AtomCamm(val metadata: moq.Mp4AtomMetaData) {
	fun debug(): kotlin.String
}
data class Mp4AtomEvte(val metadata: moq.Mp4AtomMetaData, val btrt: moq.Mp4AtomBtrt?, val silb: moq.Mp4AtomSilb?) {
	fun debug(): kotlin.String
}
data class Mp4AtomMebx(val metadata: moq.Mp4AtomMetaData, val keys: kotlin.collections.List<moq.Mp4AtomMebxKey>, val btrt: moq.Mp4AtomBtrt?) {
	fun debug(): kotlin.String
}
data class Mp4AtomTkhd(val creationTime: kotlin.ULong, val modificationTime: kotlin.ULong, val trackId: kotlin.UInt, val duration: kotlin.ULong, val layer: kotlin.UShort, val alternateGroup: kotlin.UShort, val enabled: kotlin.Boolean, val inMovie: kotlin.Boolean, val sizeIsAspectRatio: kotlin.Boolean, val volume: moq.Mp4AtomFixedPointU8, val matrix: moq.Mp4AtomMatrix, val width: moq.Mp4AtomFixedPointU16, val height: moq.Mp4AtomFixedPointU16) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTkhd
	}
}
data class Mp4AtomEdts(val elst: moq.Mp4AtomElst?) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEdts
	}
}
data class Mp4AtomSenc(val version: moq.Mp4AtomSencBoxVersion, val useSubsamples: kotlin.Boolean, val data: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSenc
	}
}
data class Mp4AtomTref(val trackReferenceTypeBoxes: kotlin.collections.List<moq.Mp4AtomTrackReferenceTypeBox>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTref
	}
}
data class Mp4AtomAudio(val dataReferenceIndex: kotlin.UShort, val channelCount: kotlin.UShort, val sampleSize: kotlin.UShort, val sampleRate: moq.Mp4AtomFixedPointU16) {
	fun debug(): kotlin.String
}
data class Mp4AtomMetaData(val dataReferenceIndex: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class RustlsClientTicketRequest(val newSessionCount: kotlin.UByte, val resumptionCount: kotlin.UByte) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class MoqQuicVarInt internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun intoInner(): kotlin.ULong
	fun encode(output: moq.Mut<moq.Bytes>)
	companion object {
		fun default(): moq.MoqQuicVarInt
		val MAX: moq.MoqQuicVarInt
		fun fromU32(x: kotlin.UInt): moq.MoqQuicVarInt
		fun fromU64(x: kotlin.ULong): moq.MoqQuicVarInt
	}
}
data class Mp4AtomAvcc(val configurationVersion: kotlin.UByte, val avcProfileIndication: kotlin.UByte, val profileCompatibility: kotlin.UByte, val avcLevelIndication: kotlin.UByte, val lengthSize: kotlin.UByte, val sequenceParameterSets: kotlin.collections.List<moq.Bytes>, val pictureParameterSets: kotlin.collections.List<moq.Bytes>, val ext: moq.Mp4AtomAvccExt?) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomAvcc
		fun new(sps: moq.Bytes, pps: moq.Bytes): moq.Mp4AtomAvcc
	}
}
data class Mp4AtomBtrt(val bufferSizeDb: kotlin.UInt, val maxBitrate: kotlin.UInt, val avgBitrate: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomBtrt
		fun new(bufferSizeDb: kotlin.UInt, maxBitrate: kotlin.UInt, avgBitrate: kotlin.UInt): moq.Mp4AtomBtrt
	}
}
data class Mp4AtomPasp(val hSpacing: kotlin.UInt, val vSpacing: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPasp
		fun new(hSpacing: kotlin.UInt, vSpacing: kotlin.UInt): moq.Mp4AtomPasp
	}
}
data class Mp4AtomTaic(val timeUncertainty: kotlin.ULong, val clockResolution: kotlin.UInt, val clockDriftRate: kotlin.Int, val clockType: moq.Mp4AtomClockType) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTaic
		fun new(timeUncertainty: kotlin.ULong, clockResolution: kotlin.UInt, clockDriftRate: kotlin.Int, clockType: moq.Mp4AtomClockType): moq.Mp4AtomTaic
	}
}
data class Mp4AtomFiel(val fieldCount: kotlin.UByte, val fieldOrder: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomFiel
		fun new(fieldCount: kotlin.UByte, fieldOrder: kotlin.UByte): moq.Mp4AtomFiel
	}
}
data class Mp4AtomHvcc(val configurationVersion: kotlin.UByte, val generalProfileSpace: kotlin.UByte, val generalTierFlag: kotlin.Boolean, val generalProfileIdc: kotlin.UByte, val generalProfileCompatibilityFlags: ArrayU8x4, val generalConstraintIndicatorFlags: ArrayU8x6, val generalLevelIdc: kotlin.UByte, val minSpatialSegmentationIdc: kotlin.UShort, val parallelismType: kotlin.UByte, val chromaFormatIdc: kotlin.UByte, val bitDepthLumaMinus8: kotlin.UByte, val bitDepthChromaMinus8: kotlin.UByte, val avgFrameRate: kotlin.UShort, val constantFrameRate: kotlin.UByte, val numTemporalLayers: kotlin.UByte, val temporalIdNested: kotlin.Boolean, val lengthSizeMinusOne: kotlin.UByte, val arrays: kotlin.collections.List<moq.Mp4AtomHvcCArray>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHvcc
		fun new(): moq.Mp4AtomHvcc
	}
}
data class Mp4AtomLhvc(val configurationVersion: kotlin.UByte, val minSpatialSegmentationIdc: kotlin.UShort, val parallelismType: kotlin.UByte, val numTemporalLayers: kotlin.UByte, val temporalIdNested: kotlin.Boolean, val lengthSizeMinusOne: kotlin.UByte, val arrays: kotlin.collections.List<moq.Mp4AtomHvcCArray>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomLhvc
	}
}
data class Mp4AtomCcst(val allRefPicsIntra: kotlin.Boolean, val intraPredUsed: kotlin.Boolean, val maxRefPerPic: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCcst
	}
}
data class Mp4AtomVpcC(val profile: kotlin.UByte, val level: kotlin.UByte, val bitDepth: kotlin.UByte, val chromaSubsampling: kotlin.UByte, val videoFullRangeFlag: kotlin.Boolean, val colorPrimaries: kotlin.UByte, val transferCharacteristics: kotlin.UByte, val matrixCoefficients: kotlin.UByte, val codecInitializationData: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomVpcC
	}
}
data class Mp4AtomAv1c(val seqProfile: kotlin.UByte, val seqLevelIdx0: kotlin.UByte, val seqTier0: kotlin.Boolean, val highBitdepth: kotlin.Boolean, val twelveBit: kotlin.Boolean, val monochrome: kotlin.Boolean, val chromaSubsamplingX: kotlin.Boolean, val chromaSubsamplingY: kotlin.Boolean, val chromaSamplePosition: kotlin.UByte, val initialPresentationDelay: kotlin.UByte?, val configObus: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomAv1c
	}
}
data class Mp4AtomEsds(val esDesc: moq.Mp4AtomEsdsEsDescriptor) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEsds
	}
}
data class Mp4AtomRgbaColor(val red: kotlin.UByte, val green: kotlin.UByte, val blue: kotlin.UByte, val alpha: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomRgbaColor
	}
}
data class Mp4AtomFtab(val fontEntries: kotlin.collections.List<moq.Mp4AtomFontEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomFtab
	}
}
data class Mp4AtomDisp(val disparityShiftIn16thPel: kotlin.Short) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomDisp
	}
}
data class Mp4AtomDops(val outputChannelCount: kotlin.UByte, val preSkip: kotlin.UShort, val inputSampleRate: kotlin.UInt, val outputGain: kotlin.Short, val channelMapping: moq.Mp4AtomOpusChannelMapping?) {
	fun debug(): kotlin.String
}
data class Mp4AtomCmpd(val components: kotlin.collections.List<moq.Mp4AtomComponent>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCmpd
	}
}
data class Mp4AtomDfla(val blocks: kotlin.collections.List<moq.Mp4AtomFlacMetadataBlock>) {
	fun debug(): kotlin.String
}
data class Mp4AtomAc3SpecificBox(val fscod: kotlin.UByte, val bsid: kotlin.UByte, val bsmod: kotlin.UByte, val acmod: kotlin.UByte, val lfeon: kotlin.Boolean, val bitRateCode: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Mp4AtomEc3SpecificBox(val dataRate: kotlin.UShort, val substreams: kotlin.collections.List<moq.Mp4AtomEc3IndependentSubstream>) {
	fun debug(): kotlin.String
}
data class Mp4AtomPcmC(val bigEndian: kotlin.Boolean, val sampleSize: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Mp4AtomChnl(val channelStructure: moq.Mp4AtomChannelStructure?, val objectCount: kotlin.UByte?, val formatOrdering: kotlin.UByte?, val baseChannelCount: kotlin.UByte?) {
	fun debug(): kotlin.String
	companion object {
		fun decodeBodyWithChannelCount(buf: moq.Mut<moq.Bytes>, channelCount: kotlin.UShort): moq.Mp4AtomChnl
	}
}
data class Mp4AtomPlainText(val dataReferenceIndex: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class Mp4AtomVttC(val config: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomVlab(val sourceLabel: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomAmrSampleEntry(val dataReferenceIndex: kotlin.UShort, val timescale: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class Mp4AtomDamr(val vendor: moq.Mp4AtomFourCc, val decoderVersion: kotlin.UByte, val modeSet: kotlin.UShort, val modeChangePeriod: kotlin.UByte, val framesPerSample: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Mp4AtomTxtC(val textConfig: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomUri(val theUri: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomUriI(val uriInitializationData: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomSilb(val schemes: kotlin.collections.List<moq.Mp4AtomSchemeIdEntry>, val otherSchemes: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class Mp4AtomMebxKey(val localKeyId: moq.Mp4AtomFourCc, val keyd: moq.Mp4AtomKeyd, val dtyp: moq.Mp4AtomDtyp?, val loca: moq.Mp4AtomLoca?, val setu: moq.Mp4AtomSetu?) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class Mp4AtomFixedPointI16 internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun integer(): kotlin.Short
	fun decimal(): kotlin.Short
	companion object {
		fun default(): moq.Mp4AtomFixedPointI16
		fun new(int: kotlin.Short, dec: kotlin.Short): moq.Mp4AtomFixedPointI16
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomFixedPointI8 internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun integer(): kotlin.Byte
	fun decimal(): kotlin.Byte
	companion object {
		fun default(): moq.Mp4AtomFixedPointI8
		fun new(int: kotlin.Byte, dec: kotlin.Byte): moq.Mp4AtomFixedPointI8
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomFixedPointU16 internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun integer(): kotlin.UShort
	fun decimal(): kotlin.UShort
	companion object {
		fun default(): moq.Mp4AtomFixedPointU16
		fun new(int: kotlin.UShort, dec: kotlin.UShort): moq.Mp4AtomFixedPointU16
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomFixedPointU8 internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun integer(): kotlin.UByte
	fun decimal(): kotlin.UByte
	companion object {
		fun default(): moq.Mp4AtomFixedPointU8
		fun new(int: kotlin.UByte, dec: kotlin.UByte): moq.Mp4AtomFixedPointU8
	}
}
data class Mp4AtomMatrix(val a: kotlin.Int, val b: kotlin.Int, val u: kotlin.Int, val c: kotlin.Int, val d: kotlin.Int, val v: kotlin.Int, val x: kotlin.Int, val y: kotlin.Int, val w: kotlin.Int) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMatrix
	}
}
data class Mp4AtomElst(val entries: kotlin.collections.List<moq.Mp4AtomElstEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomElst
	}
}
data class Mp4AtomHdlr(val handler: moq.Mp4AtomFourCc, val name: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHdlr
	}
}
data class Mp4AtomMdhd(val creationTime: kotlin.ULong, val modificationTime: kotlin.ULong, val timescale: kotlin.UInt, val duration: kotlin.ULong, val language: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMdhd
	}
}
data class Mp4AtomTrackReferenceTypeBox(val referenceType: moq.Mp4AtomFourCc, val trackIds: kotlin.UIntArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Mp4AtomChpl(val entries: kotlin.collections.List<moq.Mp4AtomChplEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomChpl
	}
}
data class Mp4AtomCprt(val language: kotlin.String, val notice: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCprt
	}
}
data class Mp4AtomKind(val schemeUri: kotlin.String, val value: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomKind
	}
}
data class Mp4AtomRtng(val entity: moq.Mp4AtomFourCc, val criteria: moq.Mp4AtomFourCc, val language: kotlin.String, val ratingInfo: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomPcmFormat(val bigEndian: kotlin.Boolean, val sampleSize: kotlin.UShort, val float: kotlin.Boolean) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class MoqQuicStreamId internal constructor(internal val bits: Long) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun initiator(): moq.MoqQuicSide
	fun dir(): moq.MoqQuicStreamDir
	fun index(): kotlin.ULong
	companion object {
		fun new(initiator: moq.MoqQuicSide, dir: moq.MoqQuicStreamDir, index: kotlin.ULong): moq.MoqQuicStreamId
	}
}
class MoqQuicUdpStats internal constructor() {
	fun debug(): kotlin.String
	val datagramsGet: kotlin.ULong
	fun datagramsSet(value: kotlin.ULong): moq.MoqQuicUdpStats
	val bytesGet: kotlin.ULong
	fun bytesSet(value: kotlin.ULong): moq.MoqQuicUdpStats
	val iosGet: kotlin.ULong
	fun iosSet(value: kotlin.ULong): moq.MoqQuicUdpStats
	companion object {
		fun default(): moq.MoqQuicUdpStats
	}
}
class MoqQuicCreditStats internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val leasedGet: kotlin.ULong
	fun leasedSet(value: kotlin.ULong): moq.MoqQuicCreditStats
	val availableGet: kotlin.ULong
	fun availableSet(value: kotlin.ULong): moq.MoqQuicCreditStats
	val evictedGet: kotlin.ULong
	fun evictedSet(value: kotlin.ULong): moq.MoqQuicCreditStats
	val peakSpentGet: kotlin.ULong
	fun peakSpentSet(value: kotlin.ULong): moq.MoqQuicCreditStats
	companion object {
		fun default(): moq.MoqQuicCreditStats
	}
}
data class Mp4AtomAvccExt(val chromaFormat: kotlin.UByte, val bitDepthLuma: kotlin.UByte, val bitDepthChroma: kotlin.UByte, val sequenceParameterSetsExt: kotlin.collections.List<moq.Bytes>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomAvccExt
	}
}
data class Mp4AtomSt3d(val stereoMode: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSt3d
	}
}
data class Mp4AtomHfov(val fieldOfView: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHfov
	}
}
data class Mp4AtomHvcCArray(val completeness: kotlin.Boolean, val nalUnitType: kotlin.UByte, val nalus: kotlin.collections.List<moq.Bytes>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHvcCArray
	}
}
data class Mp4AtomEsdsEsDescriptor(val esId: kotlin.UShort, val dependsOnEsId: kotlin.UShort?, val url: moq.Bytes?, val ocrEsId: kotlin.UShort?, val streamPriority: kotlin.UByte, val decConfig: moq.Mp4AtomEsdsDecoderConfig, val slConfig: moq.Mp4AtomEsdsSlConfig) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEsdsEsDescriptor
	}
}
data class Mp4AtomFontEntry(val fontId: kotlin.UShort, val font: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomFontEntry
	}
}
data class Mp4AtomOpusChannelMapping(val family: kotlin.UByte, val streamCount: kotlin.UByte, val coupledCount: kotlin.UByte, val channelMapping: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomComponent(val componentType: kotlin.UShort, val componentTypeUri: kotlin.String?) {
	fun debug(): kotlin.String
}
data class Mp4AtomUncompressedComponent(val componentIndex: kotlin.UShort, val componentBitDepthMinusOne: kotlin.UByte, val componentFormat: kotlin.UByte, val componentAlignSize: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Mp4AtomEc3IndependentSubstream(val fscod: kotlin.UByte, val bsid: kotlin.UByte, val asvc: kotlin.Boolean, val bsmod: kotlin.UByte, val acmod: kotlin.UByte, val lfeon: kotlin.Boolean, val numDepSub: kotlin.UByte, val chanLoc: kotlin.UShort?) {
	fun debug(): kotlin.String
}
data class Mp4AtomSchemeIdEntry(val schemeIdUri: kotlin.String, val value: kotlin.String, val atleastOnce: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class Mp4AtomKeyd(val keyNamespace: moq.Mp4AtomFourCc, val keyValue: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomDtyp(val dataType: moq.Mp4AtomDataType) {
	fun debug(): kotlin.String
}
data class Mp4AtomLoca(val locale: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomSetu(val data: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomElstEntry(val segmentDuration: kotlin.ULong, val mediaTime: kotlin.ULong?, val mediaRate: moq.Mp4AtomFixedPointI16) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomElstEntry
	}
}
data class Mp4AtomFtyp(val majorBrand: moq.Mp4AtomFourCc, val minorVersion: kotlin.UInt, val compatibleBrands: kotlin.collections.List<moq.Mp4AtomFourCc>) {
	fun debug(): kotlin.String
}
data class Mp4AtomStyp(val majorBrand: moq.Mp4AtomFourCc, val minorVersion: kotlin.UInt, val compatibleBrands: kotlin.collections.List<moq.Mp4AtomFourCc>) {
	fun debug(): kotlin.String
}
data class Mp4AtomPitm(val itemId: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPitm
	}
}
data class Mp4AtomIloc(val itemLocations: kotlin.collections.List<moq.Mp4AtomItemLocation>) {
	fun debug(): kotlin.String
}
data class Mp4AtomIinf(val itemInfos: kotlin.collections.List<moq.Mp4AtomItemInfoEntry>) {
	fun debug(): kotlin.String
}
data class Mp4AtomAuxc(val auxType: kotlin.String, val auxSubtype: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomAuxc
	}
}
data class Mp4AtomClap(val cleanApertureWidthN: kotlin.UInt, val cleanApertureWidthD: kotlin.UInt, val cleanApertureHeightN: kotlin.UInt, val cleanApertureHeightD: kotlin.UInt, val horizOffN: kotlin.Int, val horizOffD: kotlin.UInt, val vertOffN: kotlin.Int, val vertOffD: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomClap
	}
}
data class Mp4AtomImir(val axis: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomImir
	}
}
data class Mp4AtomIrot(val angle: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomIrot
	}
}
data class Mp4AtomIscl(val targetWidthNumerator: kotlin.UShort, val targetWidthDenominator: kotlin.UShort, val targetHeightNumerator: kotlin.UShort, val targetHeightDenominator: kotlin.UShort) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomIscl
	}
}
data class Mp4AtomIspe(val width: kotlin.UInt, val height: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomIspe
	}
}
data class Mp4AtomPixi(val bitsPerChannel: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPixi
	}
}
data class Mp4AtomRref(val referenceTypes: kotlin.collections.List<moq.Mp4AtomFourCc>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomRref
	}
}
data class Mp4AtomIpma(val itemProperties: kotlin.collections.List<moq.Mp4AtomPropertyAssociations>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomIpma
	}
}
data class Mp4AtomIref(val references: kotlin.collections.List<moq.Mp4AtomReference>) {
	fun debug(): kotlin.String
}
data class Mp4AtomIdat(val data: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Tuple2<T1, T2>(val first: T1, val second: T2) {
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Mp4AtomIlst(val name: moq.Mp4AtomName?, val year: moq.Mp4AtomYear?, val covr: moq.Mp4AtomCovr?, val desc: moq.Mp4AtomDesc?, val ctoo: moq.Mp4AtomTool?, val cprt: moq.Mp4AtomCopyright?, val mdta: kotlin.collections.List<moq.Tuple2<kotlin.UInt, moq.Mp4AtomIlstData>>) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomIlst
	}
}
data class Mp4AtomCovr(val _0: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomDesc(val countryIndicator: kotlin.UShort, val languageIndicator: kotlin.UShort, val text: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomTool(val countryIndicator: kotlin.UShort, val languageIndicator: kotlin.UShort, val text: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun new(countryIndicator: kotlin.UShort, languageIndicator: kotlin.UShort, text: kotlin.String): moq.Mp4AtomTool
	}
}
data class Mp4AtomName(val _0: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomYear(val _0: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomKeys(val entries: kotlin.collections.List<moq.Mp4AtomKeyEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomKeys
	}
}
data class Mp4AtomMvhd(val creationTime: kotlin.ULong, val modificationTime: kotlin.ULong, val timescale: kotlin.UInt, val duration: kotlin.ULong, val rate: moq.Mp4AtomFixedPointU16, val volume: moq.Mp4AtomFixedPointU8, val matrix: moq.Mp4AtomMatrix, val nextTrackId: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMvhd
	}
}
data class Mp4AtomAinf(val hidden: kotlin.Boolean, val profileVersion: kotlin.UInt, val apid: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomAinf
	}
}
data class Mp4AtomSkip(val zeroed: moq.Mp4AtomZeroed) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSkip
	}
}
data class Mp4AtomStts(val entries: kotlin.collections.List<moq.Mp4AtomSttsEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStts
	}
}
data class Mp4AtomStsc(val entries: kotlin.collections.List<moq.Mp4AtomStscEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStsc
	}
}
data class Mp4AtomStsz(val samples: moq.Mp4AtomStszSamples) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStsz
	}
}
data class Mp4AtomStss(val entries: kotlin.UIntArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomStss
	}
}
data class Mp4AtomStco(val entries: kotlin.UIntArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomStco
	}
}
data class Mp4AtomCo64(val entries: kotlin.ULongArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomCo64
	}
}
data class Mp4AtomCslg(val compositionToDtsShift: kotlin.Long, val leastDecodeToDisplayShift: kotlin.Long, val greatestDecodeToDisplayDelta: kotlin.Long, val compositionStartTime: kotlin.Long, val compositionEndTime: kotlin.Long) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCslg
	}
}
data class Mp4AtomCtts(val entries: kotlin.collections.List<moq.Mp4AtomCttsEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCtts
	}
}
data class Mp4AtomSbgp(val groupingType: moq.Mp4AtomFourCc, val groupingTypeParameter: kotlin.UInt?, val entries: kotlin.collections.List<moq.Mp4AtomSbgpEntry>) {
	fun debug(): kotlin.String
}
data class Mp4AtomSgpd(val groupingType: moq.Mp4AtomFourCc, val defaultLength: kotlin.UInt?, val defaultGroupDescriptionIndex: kotlin.UInt?, val staticGroupDescription: kotlin.Boolean, val staticMapping: kotlin.Boolean, val essential: kotlin.Boolean, val entries: kotlin.collections.List<moq.Mp4AtomSgpdEntry>) {
	fun debug(): kotlin.String
}
data class Mp4AtomSubs(val flags: ArrayU8x3, val entries: kotlin.collections.List<moq.Mp4AtomSubsEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSubs
	}
}
data class Mp4AtomSaio(val auxInfo: moq.Mp4AtomAuxInfo?, val offsets: kotlin.ULongArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomSaio
	}
}
data class Mp4AtomSaiz(val auxInfo: moq.Mp4AtomAuxInfo?, val defaultSampleInfoSize: kotlin.UByte, val sampleCount: kotlin.UInt, val sampleInfoSize: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSaiz
	}
}
data class Mp4AtomDinf(val dref: moq.Mp4AtomDref) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomDinf
	}
}
data class Mp4AtomDref(val entries: kotlin.collections.List<moq.Mp4AtomDrefEntry>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomDref
	}
}
data class Mp4AtomHmhd(val maxPduSize: kotlin.UShort, val avgPduSize: kotlin.UShort, val maxBitrate: kotlin.UInt, val avgBitrate: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHmhd
	}
}
data class Mp4AtomSmhd(val balance: moq.Mp4AtomFixedPointI8) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSmhd
	}
}
data class Mp4AtomVmhd(val graphicsMode: kotlin.UShort, val opColor: moq.Mp4AtomRgbColor) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomVmhd
	}
}
data class Mp4AtomMvex(val mehd: moq.Mp4AtomMehd?, val trex: kotlin.collections.List<moq.Mp4AtomTrex>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMvex
	}
}
data class Mp4AtomMehd(val fragmentDuration: kotlin.ULong) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMehd
	}
}
data class Mp4AtomTrex(val trackId: kotlin.UInt, val defaultSampleDescriptionIndex: kotlin.UInt, val defaultSampleDuration: kotlin.UInt, val defaultSampleSize: kotlin.UInt, val defaultSampleFlags: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTrex
	}
}
data class Mp4AtomEmsg(val timescale: kotlin.UInt, val presentationTime: moq.Mp4AtomEmsgTimestamp, val eventDuration: kotlin.UInt, val id: kotlin.UInt, val schemeIdUri: kotlin.String, val value: kotlin.String, val messageData: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomMfhd(val sequenceNumber: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMfhd
	}
}
data class Mp4AtomTfhd(val trackId: kotlin.UInt, val baseDataOffset: kotlin.ULong?, val sampleDescriptionIndex: kotlin.UInt?, val defaultSampleDuration: kotlin.UInt?, val defaultSampleSize: kotlin.UInt?, val defaultSampleFlags: kotlin.UInt?, val durationIsEmpty: kotlin.Boolean, val defaultBaseIsMoof: kotlin.Boolean) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTfhd
	}
}
data class Mp4AtomTfdt(val baseMediaDecodeTime: kotlin.ULong) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTfdt
	}
}
data class Mp4AtomTrun(val dataOffset: kotlin.Int?, val samples: moq.Mp4AtomTrunSamples) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTrun
	}
}
data class Mp4AtomMdat(val data: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomFree(val zeroed: moq.Mp4AtomZeroed) {
	fun debug(): kotlin.String
}
data class Mp4AtomSidx(val referenceId: kotlin.UInt, val timescale: kotlin.UInt, val earliestPresentationTime: kotlin.ULong, val firstOffset: kotlin.ULong, val references: kotlin.collections.List<moq.Mp4AtomSegmentReference>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSidx
	}
}
data class Mp4AtomPrft(val referenceTrackId: kotlin.UInt, val ntpTimestamp: kotlin.ULong, val mediaTime: kotlin.ULong, val utcTimeSemantics: moq.Mp4AtomReferenceTime) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPrft
	}
}
data class Mp4AtomMfra(val tfra: kotlin.collections.List<moq.Mp4AtomTfra>, val mfro: moq.Mp4AtomMfro) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMfra
	}
}
data class Mp4AtomTfra(val trackId: kotlin.UInt, val entries: kotlin.collections.List<moq.Mp4AtomFragmentInfo>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTfra
	}
}
data class Mp4AtomMfro(val parentSize: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMfro
	}
}
data class Mp4AtomChplEntry(val start: kotlin.ULong, val title: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomSvhd(val metadataSource: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSvhd
	}
}
data class Mp4AtomMust(val requiredBoxTypes: kotlin.collections.List<moq.Mp4AtomFourCc>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomMust
	}
}
data class Mp4AtomEsdsDecoderConfig(val objectTypeIndication: kotlin.UByte, val streamType: kotlin.UByte, val upStream: kotlin.UByte, val bufferSizeDb: moq.Mp4AtomU24, val maxBitrate: kotlin.UInt, val avgBitrate: kotlin.UInt, val decSpecific: moq.Mp4AtomEsdsDecoderSpecific?) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEsdsDecoderConfig
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomEsdsSlConfig internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEsdsSlConfig
	}
}
@kotlin.jvm.JvmInline value class Mp4AtomU24 internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomU24
	}
}
data class Mp4AtomItemLocation(val itemId: kotlin.UInt, val constructionMethod: kotlin.UByte, val dataReferenceIndex: kotlin.UShort, val baseOffset: kotlin.ULong, val extents: kotlin.collections.List<moq.Mp4AtomItemLocationExtent>) {
	fun debug(): kotlin.String
}
data class Mp4AtomItemInfoEntry(val itemId: kotlin.UInt, val itemProtectionIndex: kotlin.UShort, val itemType: moq.Mp4AtomFourCc?, val itemName: kotlin.String, val contentType: kotlin.String?, val contentEncoding: kotlin.String?, val itemUriType: kotlin.String?, val itemNotInPresentation: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class Mp4AtomPropertyAssociations(val itemId: kotlin.UInt, val associations: kotlin.collections.List<moq.Mp4AtomPropertyAssociation>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPropertyAssociations
	}
}
data class Mp4AtomReference(val referenceType: moq.Mp4AtomFourCc, val fromItemId: kotlin.UInt, val toItemIds: kotlin.UIntArray) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Mp4AtomCopyright(val countryIndicator: kotlin.UShort, val languageIndicator: kotlin.UShort, val text: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomIlstData(val countryIndicator: kotlin.UShort, val languageIndicator: kotlin.UShort, val value: moq.Mp4AtomIlstDataValue) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Mp4AtomKeyEntry(val keyNamespace: moq.Mp4AtomFourCc, val keyValue: kotlin.String) {
	fun debug(): kotlin.String
}
data class Mp4AtomZeroed(val size: kotlin.ULong) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomZeroed
		fun new(size: kotlin.ULong): moq.Mp4AtomZeroed
	}
}
data class Mp4AtomSttsEntry(val sampleCount: kotlin.UInt, val sampleDelta: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSttsEntry
	}
}
data class Mp4AtomStscEntry(val firstChunk: kotlin.UInt, val samplesPerChunk: kotlin.UInt, val sampleDescriptionIndex: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStscEntry
	}
}
data class Mp4AtomCttsEntry(val sampleCount: kotlin.UInt, val sampleOffset: kotlin.Long) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCttsEntry
	}
}
data class Mp4AtomSbgpEntry(val sampleCount: kotlin.UInt, val groupDescriptionIndex: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Mp4AtomSgpdEntry(val descriptionLength: kotlin.UInt?, val entry: moq.Mp4AtomAnySampleGroupEntry) {
	fun debug(): kotlin.String
}
data class Mp4AtomSubsEntry(val sampleDelta: kotlin.UInt, val subsamples: kotlin.collections.List<moq.Mp4AtomSubsSubsample>) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSubsEntry
	}
}
data class Mp4AtomAuxInfo(val auxInfoType: moq.Mp4AtomFourCc, val auxInfoTypeParameter: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Mp4AtomRgbColor(val red: kotlin.UShort, val green: kotlin.UShort, val blue: kotlin.UShort) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomRgbColor
	}
}
data class Mp4AtomSegmentReference(val referenceType: kotlin.Boolean, val referencedSize: kotlin.UInt, val subsegmentDuration: kotlin.UInt, val startsWithSap: kotlin.Boolean, val sapType: kotlin.UByte, val sapDeltaTime: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Mp4AtomFragmentInfo(val time: kotlin.ULong, val moofOffset: kotlin.ULong, val trafNumber: kotlin.UInt, val trunNumber: kotlin.UInt, val sampleDelta: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomFragmentInfo
	}
}
data class Mp4AtomC2pa(val boxPurpose: kotlin.String, val data: moq.Bytes) {
	fun debug(): kotlin.String
}
class Mp4AtomExtendedType internal constructor() {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomExtendedType
		fun new(value: ArrayU8x16): moq.Mp4AtomExtendedType
	}
}
data class Mp4AtomPrhd(val poseYawDegrees: kotlin.Int, val posePitchDegrees: kotlin.Int, val poseRollDegrees: kotlin.Int) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPrhd
	}
}
data class Mp4AtomPrji(val projectionKind: moq.Mp4AtomFourCc) {
	fun debug(): kotlin.String
	companion object {
		val RECTILINEAR: moq.Mp4AtomFourCc
		val EQUIRECTANGULAR: moq.Mp4AtomFourCc
		val HALFEQUIRECTANGULAR: moq.Mp4AtomFourCc
		val FISHEYE: moq.Mp4AtomFourCc
		val PARAMETRICIMMERSIVE: moq.Mp4AtomFourCc
	}
}
data class Mp4AtomStri(val eyeViewsReversed: kotlin.Boolean, val hasAdditionalViews: kotlin.Boolean, val hasRightEyeView: kotlin.Boolean, val hasLeftEyeView: kotlin.Boolean) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStri
	}
}
data class Mp4AtomHero(val heroEyeIndicator: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomHero
	}
}
data class Mp4AtomPkin(val viewPackingKind: moq.Mp4AtomFourCc) {
	fun debug(): kotlin.String
	companion object {
		val SIDEBYSIDE: moq.Mp4AtomFourCc
		val OVERUNDER: moq.Mp4AtomFourCc
	}
}
data class Mp4AtomEsdsDecoderSpecific(val profile: kotlin.UByte, val freqIndex: kotlin.UByte, val chanConf: kotlin.UByte, val raw: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEsdsDecoderSpecific
	}
}
data class Mp4AtomExplicitSpeakerPosition(val azimuth: kotlin.Short, val elevation: kotlin.Byte) {
	fun debug(): kotlin.String
}
data class Mp4AtomItemLocationExtent(val itemReferenceIndex: kotlin.ULong, val offset: kotlin.ULong, val length: kotlin.ULong) {
	fun debug(): kotlin.String
}
data class Mp4AtomPropertyAssociation(val essential: kotlin.Boolean, val propertyIndex: kotlin.UShort) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomPropertyAssociation
	}
}
data class Mp4AtomSubsSubsample(val size: moq.Mp4AtomSubsSubsampleSize, val priority: kotlin.UByte, val discardable: kotlin.Boolean, val codecSpecificParameters: moq.Bytes) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSubsSubsample
	}
}
data class Mp4AtomUrl(val location: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomUrl
	}
}
data class Mp4AtomUrn(val name: kotlin.String, val location: kotlin.String) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomUrn
	}
}
data class Mp4AtomTrunEntry(val duration: kotlin.UInt?, val size: kotlin.UInt?, val flags: kotlin.UInt?, val cts: kotlin.Long?) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomTrunEntry
	}
}
data class RustlsCryptoHpkeHpkePublicKey(val _0: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomEqui(val projectionBoundsTop: kotlin.UInt, val projectionBoundsBottom: kotlin.UInt, val projectionBoundsLeft: kotlin.UInt, val projectionBoundsRight: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomEqui
	}
}
data class Mp4AtomCbmp(val layout: kotlin.UInt, val padding: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomCbmp
	}
}
data class Mp4AtomMshp(val crc: kotlin.UInt, val encodingFourCc: moq.Mp4AtomFourCc, val data: moq.Bytes) {
	fun debug(): kotlin.String
}
data class Mp4AtomBlin(val baselineValue: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomBlin
	}
}
data class Mp4AtomDadj(val disparityAdjustment: kotlin.Int) {
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomDadj
	}
}
enum class EncodeError(internal val value: Int) {
	BOUNDS_EXCEEDED(0),
	TOO_LARGE(1),
	SHORT(2),
	INVALID_STATE(3),
	TOO_MANY(4),
	VERSION(5),
	UNSUPPORTED(6);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class InvalidPattern {
	data object EmptySegment : InvalidPattern()
	data class InvalidSegment(val _0: kotlin.String) : InvalidPattern()
	data object MultipleGlobstars : InvalidPattern()
	data object TooManySegments : InvalidPattern()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class SessionError {
	data object Cancel : SessionError()
	data object Internal : SessionError()
	data object Unauthorized : SessionError()
	data object Expired : SessionError()
	data object ProtocolViolation : SessionError()
	data object KeyValueFormatting : SessionError()
	data object TooManyRequests : SessionError()
	data object GoawayTimeout : SessionError()
	data object Timeout : SessionError()
	data object AuthTokenCacheOverflow : SessionError()
	data object Version : SessionError()
	data object MalformedAuthToken : SessionError()
	data object UnknownAuthTokenAlias : SessionError()
	data object TooManyRequestUpdates : SessionError()
	data class App(val _0: kotlin.UShort) : SessionError()
	data class Unknown(val _0: kotlin.UInt) : SessionError()
	data object Unsupported : SessionError()
	data object InvalidRequestId : SessionError()
	data object InvalidPath : SessionError()
	data object InvalidAuthority : SessionError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun toCode(): kotlin.UInt
	companion object {
		fun fromCode(code: kotlin.UInt): moq.SessionError
	}
}
sealed class StreamError {
	data class Session(val _0: moq.SessionError) : StreamError()
	data object Internal : StreamError()
	data object Cancel : StreamError()
	data object DeliveryTimeout : StreamError()
	data object ControlTimeout : StreamError()
	data object GoingAway : StreamError()
	data object TooFarBehind : StreamError()
	data object MalformedTrack : StreamError()
	data object NotFound : StreamError()
	data object Unroutable : StreamError()
	data object Old : StreamError()
	data object Evicted : StreamError()
	data object WrongSize : StreamError()
	data object FrameTooLarge : StreamError()
	data object GroupTooLarge : StreamError()
	data object TimestampMismatch : StreamError()
	data object Unauthorized : StreamError()
	data class App(val _0: kotlin.UShort) : StreamError()
	data class Unknown(val _0: kotlin.UInt) : StreamError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun toCode(): kotlin.UInt
	companion object {
		fun fromCode(code: kotlin.UInt): moq.StreamError
	}
}
enum class InvalidHop(internal val value: Int) {
	RANGE(0),
	TOO_MANY(1),
	DUPLICATE(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class IntersectionError(internal val value: Int) {
	TOO_MANY_PATTERNS(0);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class Segment {
	data class Literal(val _0: kotlin.String) : Segment()
	data object Wildcard : Segment()
	data class Partial(val prefix: kotlin.String, val suffix: kotlin.String) : Segment()
	data object Globstar : Segment()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class ClockMissedTickBehavior(internal val value: Int) {
	BURST(0),
	DELAY(1),
	SKIP(2);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.ClockMissedTickBehavior
	}
}
enum class TokioConnectError(internal val value: Int) {
	UNAUTHORIZED(0),
	FORBIDDEN(1);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun isAuth(): kotlin.Boolean
	companion object {}
}
enum class TokioRestart(internal val value: Int) {
	REDIAL(0),
	WAIT(1);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.TokioRestart
	}
}
enum class TokioStatus(internal val value: Int) {
	CONNECTED(0),
	DISCONNECTED(1),
	MIGRATING(2);
	fun debug(): kotlin.String
	companion object {}
}
enum class TokioTransport(internal val value: Int) {
	QUIC(0),
	WEB_SOCKET(1),
	TCP(2),
	UNIX(3),
	WEB_TRANSPORT(4),
	WEB_RTC(5);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {}
}
enum class AuthAnswer(internal val value: Int) {
	GRANTED(0),
	REQUESTS(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class AuthRequestKind(internal val value: Int) {
	SUBSCRIBE(0),
	PUBLISH_NAMESPACE(1),
	DISCOVER(2),
	PUBLISH(3);
	fun debug(): kotlin.String
	companion object {}
}
enum class PathError(internal val value: Int) {
	TOO_LONG(0),
	EMPTY_SEGMENT(1),
	INVALID_ESCAPE(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class SetupRole(internal val value: Int) {
	PUBLISHER(0),
	SUBSCRIBER(1);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {}
}
enum class StatsRole(internal val value: Int) {
	PUBLISHER(0),
	SUBSCRIBER(1);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {}
}
enum class StatsMessage(internal val value: Int) {
	SESSION(0),
	ANNOUNCE(1),
	TRACK_INFO(2),
	SUBSCRIBE(3),
	FETCH(4),
	PROBE(5),
	AUTH(6),
	GOAWAY(7),
	OTHER(8);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {}
}
enum class StatsFailure(internal val value: Int) {
	IDLE_TIMEOUT(0),
	TRANSPORT(1),
	APPLICATION(2),
	EXPIRED(3);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		val ALL: kotlin.collections.List<moq.StatsFailure>
	}
}
enum class StatsRefusalKind(internal val value: Int) {
	REFUSED(0),
	UNAVAILABLE(1),
	REQUEST(2),
	FORBIDDEN(3),
	LAN(4),
	EXPIRED(5);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		val ALL: kotlin.collections.List<moq.StatsRefusalKind>
	}
}
enum class TransportAddress(internal val value: Int) {
	URI(0),
	SETUP(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class TransportCloseKind(internal val value: Int) {
	TRANSPORT(0),
	IDLE_TIMEOUT(1),
	CLOSED(2),
	EVICTED(3);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.TransportCloseKind
	}
}
sealed class TrackAnchor {
	data class Absolute(val _0: kotlin.ULong) : TrackAnchor()
	data class Relative(val _0: kotlin.ULong) : TrackAnchor()
	data class Time(val _0: moq.Timestamp) : TrackAnchor()
	fun debug(): kotlin.String
}
enum class TrackOrder(internal val value: Int) {
	ASCENDING(0),
	DESCENDING(1);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.TrackOrder
	}
}
enum class OriginLink(internal val value: Int) {
	LOCAL(0),
	PEER(1),
	UPSTREAM(2);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.OriginLink
	}
}
sealed class OriginSource {
	data object Local : OriginSource()
	data class Peer(val _0: moq.Hop) : OriginSource()
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.OriginSource
	}
}
enum class HangHealthVerdict(internal val value: Int) {
	UNKNOWN(0),
	HEALTHY(1),
	DEGRADED(2),
	UNHEALTHY(3);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.HangHealthVerdict
	}
}
enum class HangHealthRole(internal val value: Int) {
	PUBLISHER(0),
	VIEWER(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class HangHealthReset(internal val value: Int) {
	DECREASE(0),
	REPLACED(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class HangHealthInput(internal val value: Int) {
	RTT(0),
	LOSS(1),
	HEADROOM(2),
	SKIPPED(3),
	LATE(4),
	ERRORS(5),
	STALLED(6);
	fun debug(): kotlin.String
	companion object {}
}
sealed class HangCatalogAudioCodec {
	data class Aac(val _0: moq.HangCatalogAac) : HangCatalogAudioCodec()
	data object Opus : HangCatalogAudioCodec()
	data object Pcm : HangCatalogAudioCodec()
	data object Flac : HangCatalogAudioCodec()
	data object Mp3 : HangCatalogAudioCodec()
	data object Mp2 : HangCatalogAudioCodec()
	data object Ac3 : HangCatalogAudioCodec()
	data object Ec3 : HangCatalogAudioCodec()
	data class Unknown(val _0: kotlin.String) : HangCatalogAudioCodec()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun kind(): moq.HangCatalogAudioCodecKind
}
enum class HangCatalogAudioCodecKind(internal val value: Int) {
	AAC(0),
	OPUS(1),
	UNKNOWN(2),
	FLAC(3),
	MP3(4),
	PCM(5),
	AC3(6),
	EC3(7);
	fun debug(): kotlin.String
	companion object {}
}
sealed class HangCatalogAudioLayout {
	data object Ambisonic : HangCatalogAudioLayout()
	data class Unknown(val _0: kotlin.String) : HangCatalogAudioLayout()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class HangCatalogMode {
	data object Snapshot : HangCatalogMode()
	data object Stream : HangCatalogMode()
	data class Unknown(val _0: kotlin.String) : HangCatalogMode()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class HangCatalogTextFormat {
	data object Vtt : HangCatalogTextFormat()
	data object Ttml : HangCatalogTextFormat()
	data object Utf8 : HangCatalogTextFormat()
	data class Unknown(val _0: kotlin.String) : HangCatalogTextFormat()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.HangCatalogTextFormat
	}
}
sealed class HangCatalogTextRole {
	data object Subtitle : HangCatalogTextRole()
	data object Caption : HangCatalogTextRole()
	data class Unknown(val _0: kotlin.String) : HangCatalogTextRole()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.HangCatalogTextRole
	}
}
sealed class HangCatalogVideoCodec {
	data class H264(val _0: moq.HangCatalogH264) : HangCatalogVideoCodec()
	data class H265(val _0: moq.HangCatalogH265) : HangCatalogVideoCodec()
	data class Vp9(val _0: moq.HangCatalogVp9) : HangCatalogVideoCodec()
	data class Av1(val _0: moq.HangCatalogAv1) : HangCatalogVideoCodec()
	data object Vp8 : HangCatalogVideoCodec()
	data class Unknown(val _0: kotlin.String) : HangCatalogVideoCodec()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun kind(): moq.HangCatalogVideoCodecKind
}
enum class HangCatalogVideoCodecKind(internal val value: Int) {
	H264(0),
	H265(1),
	VP8(2),
	VP9(3),
	AV1(4),
	UNKNOWN(5);
	fun debug(): kotlin.String
	companion object {}
}
enum class HangCatalogRange(internal val value: Int) {
	LIMITED(0),
	FULL(1);
	fun debug(): kotlin.String
	companion object {}
}
sealed class HangCatalogStereo {
	data object Mono : HangCatalogStereo()
	data object LeftRight : HangCatalogStereo()
	data object TopBottom : HangCatalogStereo()
	data class Unknown(val _0: kotlin.String) : HangCatalogStereo()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class MuxCatalogCatalogFormat(internal val value: Int) {
	HANG(0),
	MSF(1);
	fun debug(): kotlin.String
	fun extension(): kotlin.String
	companion object {
		fun default(): moq.MuxCatalogCatalogFormat
		val DEFAULT: moq.MuxCatalogCatalogFormat
		fun detect(name: kotlin.String): moq.MuxCatalogCatalogFormat?
	}
}
enum class MuxContainerKind(internal val value: Int) {
	AUDIO(0),
	VIDEO(1),
	DATA(2);
	fun debug(): kotlin.String
	companion object {}
}
sealed class MuxContainerRead {
	data class Frame(val group: kotlin.ULong, val frame: moq.MuxContainerFrame) : MuxContainerRead()
	data class Gap(val timestamp: moq.Timestamp, val duration: moq.Timestamp) : MuxContainerRead()
	data class End(val group: kotlin.ULong, val end: moq.Timestamp) : MuxContainerRead()
	fun debug(): kotlin.String
}
enum class MuxImportAudioFormat(internal val value: Int) {
	AAC(0),
	OPUS(1),
	FLAC(2),
	MP3(3);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class MuxImportContainerFormat(internal val value: Int) {
	FMP4(0),
	MKV(1),
	TS(2),
	FLV(3);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class MuxImportFormat {
	data class Audio(val _0: moq.MuxImportAudioFormat) : MuxImportFormat()
	data class Video(val _0: moq.MuxImportVideoFormat) : MuxImportFormat()
	data class Container(val _0: moq.MuxImportContainerFormat) : MuxImportFormat()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun kind(): moq.MuxImportKind
}
enum class MuxImportKind(internal val value: Int) {
	AUDIO(0),
	VIDEO(1),
	CONTAINER(2);
	fun debug(): kotlin.String
	fun name_(): kotlin.String
	companion object {}
}
enum class MuxImportVideoFormat(internal val value: Int) {
	AVC1(0),
	AVC3(1),
	HVC1(2),
	HEV1(3),
	AV01(4),
	VP8(5),
	VP9(6);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class MuxRateTarget {
	data class Enabled(val _0: moq.BandwidthRate) : MuxRateTarget()
	data object Disabled : MuxRateTarget()
	fun debug(): kotlin.String
}
enum class MuxTimelineKind(internal val value: Int) {
	VIDEO(0),
	AUDIO(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class MuxTimelineError(internal val value: Int) {
	OVERFLOW(0),
	BEFORE_ORIGIN(1),
	DELAY_OVERFLOW(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class TokioListenBind {
	data class Addr(val _0: moq.SocketAddr) : TokioListenBind()
	data class Host(val _0: kotlin.String, val _1: kotlin.UShort) : TokioListenBind()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class TransportMemoryError {
	data class Session(val code: kotlin.UInt, val reason: kotlin.String) : TransportMemoryError()
	data class Reset(val _0: kotlin.UInt) : TransportMemoryError()
	data class Stopped(val _0: kotlin.UInt) : TransportMemoryError()
	data object Closed : TransportMemoryError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class MuxCatalogMsfError {
	data class UnresolvableNamespace(val name: kotlin.String, val namespace: kotlin.String) : MuxCatalogMsfError()
	data object InvalidUtf8 : MuxCatalogMsfError()
	data object ParseFrame : MuxCatalogMsfError()
	data class MissingCmafInit(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MalformedInitData(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MissingVideoCodec(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MissingAudioCodec(val _0: kotlin.String) : MuxCatalogMsfError()
	data class UnsupportedPackaging(val name: kotlin.String, val packaging: kotlin.String) : MuxCatalogMsfError()
	data class MissingTextCodec(val _0: kotlin.String) : MuxCatalogMsfError()
	data class InvalidVideoCodec(val name: kotlin.String, val codec: kotlin.String) : MuxCatalogMsfError()
	data class InvalidAudioCodec(val name: kotlin.String, val codec: kotlin.String) : MuxCatalogMsfError()
	data class MissingAudioParams(val _0: kotlin.String) : MuxCatalogMsfError()
	data class UnsupportedDerivationPackaging(val name: kotlin.String, val packaging: kotlin.String) : MuxCatalogMsfError()
	data class MalformedAac(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MalformedOpus(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MalformedFlac(val _0: kotlin.String) : MuxCatalogMsfError()
	data class OpusTrailingBytes(val _0: kotlin.String) : MuxCatalogMsfError()
	data class UnsupportedDerivationCodec(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MalformedInitSegment(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MissingInitMoov(val _0: kotlin.String) : MuxCatalogMsfError()
	data class MissingAudioSampleEntry(val _0: kotlin.String) : MuxCatalogMsfError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class MuxCodecFlacError(internal val value: Int) {
	SHORT(0),
	INVALID_MARKER(1),
	MISSING_STREAM_INFO(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class MuxCodecMp3Error(internal val value: Int) {
	HEADER_TOO_SHORT(0),
	MISSING_SYNC(1),
	RESERVED_VERSION(2),
	NOT_LAYER3(3),
	RESERVED_SAMPLE_RATE(4);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class MuxCodecOpusError {
	data object HeadTooShort : MuxCodecOpusError()
	data object InvalidSignature : MuxCodecOpusError()
	data class UnsupportedVersion(val _0: kotlin.UByte) : MuxCodecOpusError()
	data class UnsupportedChannelCount(val _0: kotlin.UInt) : MuxCodecOpusError()
	data object MappingTableTooShort : MuxCodecOpusError()
	data object InvalidMappingTable : MuxCodecOpusError()
	data class UnsupportedMappingFamily(val _0: kotlin.UByte) : MuxCodecOpusError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class MuxCodecVp8Error(internal val value: Int) {
	FRAME_TOO_SHORT(0),
	KEYFRAME_HEADER_TOO_SHORT(1),
	START_CODE_MISMATCH(2),
	EMPTY_FRAME(3);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class MuxCodecVp9Error(internal val value: Int) {
	INVALID_FRAME_MARKER(0),
	INVALID_SYNC_CODE(1),
	TRUNCATED(2),
	EMPTY_FRAME(3);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class MuxContainerFmp4Chunk {
	data class Init(val _0: moq.Bytes) : MuxContainerFmp4Chunk()
	data class Fragment(val _0: moq.MuxContainerFmp4Fragment) : MuxContainerFmp4Chunk()
	fun debug(): kotlin.String
	fun data(): moq.Bytes
	fun intoData(): moq.Bytes
	fun init(): moq.Bytes?
	fun fragment(): moq.MuxContainerFmp4Fragment?
}
enum class MuxContainerTsFraming(internal val value: Int) {
	SECTION(0);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.MuxContainerTsFraming
	}
}
sealed class MuxContainerTsStep {
	data class Rewind(val by: moq.Duration, val flagged: kotlin.Boolean) : MuxContainerTsStep()
	data class Leap(val by: moq.Duration) : MuxContainerTsStep()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class MuxCodecAacAdtsError(internal val value: Int) {
	TRUNCATED(0),
	SYNCWORD(1),
	SAMPLE_RATE(2),
	FRAME_LENGTH(3),
	PACKET_LENGTH(4);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class MuxContainerFmp4FragmentMissingDuration(internal val value: Int) {
	REJECT(0),
	INFER_FROM_PRESENTATION_TIME(1);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.MuxContainerFmp4FragmentMissingDuration
	}
}
enum class MuxContainerTsStatsClass(internal val value: Int) {
	AUDIO(1),
	VIDEO(2),
	DATA(0);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.MuxContainerTsStatsClass
	}
}
enum class MoqTokioQuicCongestionControl(internal val value: Int) {
	LOSS(0),
	DELAY(1),
	NONE(2);
	fun debug(): kotlin.String
	companion object {}
}
enum class MoqTokioConnectionPhase(internal val value: Int) {
	CONNECTING(0),
	CONNECTED(1),
	DISCONNECTED(2),
	MIGRATING(3),
	CLOSED(4);
	fun debug(): kotlin.String
	companion object {}
}
enum class MoqTokioAcceptFailure(internal val value: Int) {
	CONNECTION(0),
	EXHAUSTED(1),
	UNKNOWN(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		val ALL: kotlin.collections.List<moq.MoqTokioAcceptFailure>
	}
}
enum class MoqTokioRedirect(internal val value: Int) {
	FOLLOW(0),
	SAME_HOST(1),
	IGNORE(2);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.MoqTokioRedirect
	}
}
sealed class MoqQuicRate {
	data object Unlimited : MoqQuicRate()
	data class BytesPerSecond(val _0: kotlin.ULong) : MoqQuicRate()
	fun debug(): kotlin.String
}
sealed class Mp4AtomColr {
	data class Nclx(val colourPrimaries: kotlin.UShort, val transferCharacteristics: kotlin.UShort, val matrixCoefficients: kotlin.UShort, val fullRangeFlag: kotlin.Boolean) : Mp4AtomColr()
	data class Nclc(val colourPrimaries: kotlin.UShort, val transferCharacteristics: kotlin.UShort, val matrixCoefficients: kotlin.UShort) : Mp4AtomColr()
	data class Ricc(val profile: moq.Bytes) : Mp4AtomColr()
	data class Prof(val profile: moq.Bytes) : Mp4AtomColr()
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomColr
		fun new(colourPrimaries: kotlin.UShort, transferCharacteristics: kotlin.UShort, matrixCoefficients: kotlin.UShort, fullRangeFlag: kotlin.Boolean): moq.Mp4AtomColr
	}
}
sealed class Mp4AtomUncC {
	data class V1(val profile: moq.Mp4AtomFourCc) : Mp4AtomUncC()
	data class V0(val profile: moq.Mp4AtomFourCc, val components: kotlin.collections.List<moq.Mp4AtomUncompressedComponent>, val samplingType: kotlin.UByte, val interleaveType: kotlin.UByte, val blockSize: kotlin.UByte, val componentsLittleEndian: kotlin.Boolean, val blockPadLsb: kotlin.Boolean, val blockLittleEndian: kotlin.Boolean, val blockReversed: kotlin.Boolean, val padUnknown: kotlin.Boolean, val pixelSize: kotlin.UInt, val rowAlignSize: kotlin.UInt, val tileAlignSize: kotlin.UInt, val numTileColsMinusOne: kotlin.UInt, val numTileRowsMinusOne: kotlin.UInt) : Mp4AtomUncC()
	fun debug(): kotlin.String
}
enum class Mp4AtomSencBoxVersion(internal val value: Int) {
	V0(0),
	V1(1),
	V2(2);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomSencBoxVersion
	}
}
sealed class MoqTokioConnectError {
	data object EmptyAddresses : MoqTokioConnectError()
	data class UnsupportedScheme(val _0: kotlin.String) : MoqTokioConnectError()
	data object MissingHost : MoqTokioConnectError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class RustlsProtocolVersion {
	data object SsLv2 : RustlsProtocolVersion()
	data object SsLv3 : RustlsProtocolVersion()
	data object TlSv10 : RustlsProtocolVersion()
	data object TlSv11 : RustlsProtocolVersion()
	data object TlSv12 : RustlsProtocolVersion()
	data object TlSv13 : RustlsProtocolVersion()
	data object DtlSv10 : RustlsProtocolVersion()
	data object DtlSv12 : RustlsProtocolVersion()
	data object DtlSv13 : RustlsProtocolVersion()
	data class Unknown(val _0: kotlin.UShort) : RustlsProtocolVersion()
	fun debug(): kotlin.String
	fun toArray(): ArrayU8x2
	fun asStr(): kotlin.String?
}
enum class RustlsClientTls12Resumption(internal val value: Int) {
	DISABLED(0),
	SESSION_ID_ONLY(1),
	SESSION_ID_OR_TICKETS(2);
	fun debug(): kotlin.String
	companion object {}
}
enum class MoqSockDemuxDiscard(internal val value: Int) {
	STUN(0),
	UNKNOWN(1),
	UNCLAIMED(2),
	OVERFLOW(3);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		val ALL: kotlin.collections.List<moq.MoqSockDemuxDiscard>
	}
}
enum class Mp4AtomClockType(internal val value: Int) {
	UNKNOWN(0),
	DOES_NOT_SYNC(1),
	CAN_SYNC(2),
	RESERVED(3);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomClockType
	}
}
sealed class Mp4AtomFlacMetadataBlock {
	data class StreamInfo(val minimumBlockSize: kotlin.UShort, val maximumBlockSize: kotlin.UShort, val minimumFrameSize: moq.Mp4AtomU24, val maximumFrameSize: moq.Mp4AtomU24, val sampleRate: kotlin.UInt, val numChannelsMinusOne: kotlin.UByte, val bitsPerSampleMinusOne: kotlin.UByte, val numberOfInterchannelSamples: kotlin.ULong, val md5Checksum: moq.Bytes) : Mp4AtomFlacMetadataBlock()
	data object Padding : Mp4AtomFlacMetadataBlock()
	data object Application : Mp4AtomFlacMetadataBlock()
	data object SeekTable : Mp4AtomFlacMetadataBlock()
	data class VorbisComment(val vendorString: kotlin.String, val comments: kotlin.collections.List<kotlin.String>) : Mp4AtomFlacMetadataBlock()
	data object CueSheet : Mp4AtomFlacMetadataBlock()
	data object Picture : Mp4AtomFlacMetadataBlock()
	data object Reserved : Mp4AtomFlacMetadataBlock()
	data object Forbidden : Mp4AtomFlacMetadataBlock()
	fun debug(): kotlin.String
}
sealed class Mp4AtomChannelStructure {
	data class ExplicitPositions(val positions: kotlin.collections.List<moq.Mp4AtomSpeakerPosition>) : Mp4AtomChannelStructure()
	data class DefinedLayout(val layout: kotlin.UByte, val omittedChannelsMap: kotlin.ULong?, val channelOrderDefinition: kotlin.UByte?) : Mp4AtomChannelStructure()
	fun debug(): kotlin.String
}
sealed class Mp4AtomUuid {
	data class C2pa(val _0: moq.Mp4AtomC2pa) : Mp4AtomUuid()
	data class Unknown(val _0: moq.Mp4AtomExtendedType, val _1: moq.Bytes) : Mp4AtomUuid()
	fun debug(): kotlin.String
}
enum class MoqSockUdpEcnCodepoint(internal val value: Int) {
	ECT0(2),
	ECT1(1),
	CE(3);
	fun debug(): kotlin.String
	companion object {
		fun fromBits(x: kotlin.UByte): moq.MoqSockUdpEcnCodepoint?
	}
}
enum class MoqQuicSide(internal val value: Int) {
	CLIENT(0),
	SERVER(1);
	fun debug(): kotlin.String
	fun isClient(): kotlin.Boolean
	fun isServer(): kotlin.Boolean
	companion object {}
}
enum class MoqQuicStreamDir(internal val value: Int) {
	BI(0),
	UNI(1);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
sealed class Mp4AtomSpeakerPosition {
	data class Standard(val _0: moq.Mp4AtomAudioChannelPosition) : Mp4AtomSpeakerPosition()
	data class Explicit(val _0: moq.Mp4AtomExplicitSpeakerPosition) : Mp4AtomSpeakerPosition()
	fun debug(): kotlin.String
}
sealed class Mp4AtomDataType {
	data class WellKnown(val _0: kotlin.UInt) : Mp4AtomDataType()
	data class Custom(val _0: kotlin.String) : Mp4AtomDataType()
	data class Unknown(val _0: kotlin.UInt, val _1: moq.Bytes) : Mp4AtomDataType()
	fun debug(): kotlin.String
}
sealed class Mp4AtomStszSamples {
	data class Identical(val count: kotlin.UInt, val size: kotlin.UInt) : Mp4AtomStszSamples()
	data class Different(val sizes: kotlin.UIntArray) : Mp4AtomStszSamples() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomStszSamples
	}
}
sealed class Mp4AtomDrefEntry {
	data class Url(val _0: moq.Mp4AtomUrl) : Mp4AtomDrefEntry()
	data class Urn(val _0: moq.Mp4AtomUrn) : Mp4AtomDrefEntry()
	data class Unknown(val _0: moq.Mp4AtomFourCc, val _1: moq.Bytes) : Mp4AtomDrefEntry()
	fun debug(): kotlin.String
	fun kind(): moq.Mp4AtomFourCc
}
sealed class Mp4AtomEmsgTimestamp {
	data class Relative(val _0: kotlin.UInt) : Mp4AtomEmsgTimestamp()
	data class Absolute(val _0: kotlin.ULong) : Mp4AtomEmsgTimestamp()
	fun debug(): kotlin.String
}
sealed class Mp4AtomTrunSamples {
	data class Inherited(val count: kotlin.UInt, val firstFlags: kotlin.UInt?) : Mp4AtomTrunSamples()
	data class Explicit(val _0: kotlin.collections.List<moq.Mp4AtomTrunEntry>) : Mp4AtomTrunSamples()
	fun debug(): kotlin.String
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	fun get(index: kotlin.ULong): moq.Mp4AtomTrunEntry?
	companion object {
		fun default(): moq.Mp4AtomTrunSamples
	}
}
enum class Mp4AtomReferenceTime(internal val value: Int) {
	INPUT(0),
	OUTPUT(1),
	FINALISED(2),
	WRITTEN(3),
	CONSISTENT(4),
	REAL_TIME(5);
	fun debug(): kotlin.String
	companion object {
		fun default(): moq.Mp4AtomReferenceTime
	}
}
sealed class Mp4AtomProjKind {
	data class Equi(val _0: moq.Mp4AtomEqui) : Mp4AtomProjKind()
	data class Cbmp(val _0: moq.Mp4AtomCbmp) : Mp4AtomProjKind()
	data class Mshp(val _0: moq.Mp4AtomMshp) : Mp4AtomProjKind()
	fun debug(): kotlin.String
}
enum class Mp4AtomVexuProjKind(internal val value: Int) {
	RECTILINEAR(0),
	EQUIRECTANGULAR(1),
	HALF_EQUIRECTANGULAR(2),
	FISHEYE(3),
	PARAMETRIC_IMMERSIVE(4);
	fun debug(): kotlin.String
	fun kind(): moq.Mp4AtomFourCc
	companion object {}
}
enum class Mp4AtomAudioChannelPosition(internal val value: Int) {
	FRONT_LEFT(0),
	FRONT_RIGHT(1),
	FRONT_CENTER(2),
	LFE1(3),
	LEFT_SURROUND(4),
	RIGHT_SURROUND(5),
	FRONT_LEFT_OF_CENTER(6),
	FRONT_RIGHT_OF_CENTER(7),
	REAR_LEFT(8),
	REAR_RIGHT(9),
	REAR_CENTER(10),
	SURROUND_LEFT(11),
	SURROUND_RIGHT(12),
	SIDE_LEFT(13),
	SIDE_RIGHT(14),
	FRONT_LEFT_WIDE(15),
	FRONT_RIGHT_WIDE(16),
	TOP_FRONT_LEFT(17),
	TOP_FRONT_RIGHT(18),
	TOP_FRONT_CENTER(19),
	TOP_REAR_LEFT(20),
	TOP_REAR_RIGHT(21),
	TOP_REAR_CENTER(22),
	TOP_SIDE_LEFT(23),
	TOP_SIDE_RIGHT(24),
	TOP_CENTER(25),
	LFE2(26),
	BOTTOM_FRONT_LEFT(27),
	BOTTOM_FRONT_RIGHT(28),
	BOTTOM_FRONT_CENTER(29),
	TOP_SURROUND_LEFT(30),
	TOP_SURROUND_RIGHT(31),
	LFE3(36),
	LEOS(37),
	REOS(38),
	HWBCAL(39),
	HWBCAR(40),
	LBS(41),
	RBS(42),
	LEFT_EAR(43),
	RIGHT_EAR(44),
	UNKNOWN(127);
	fun debug(): kotlin.String
	companion object {
		fun fromU8(value: kotlin.UByte): moq.Mp4AtomAudioChannelPosition?
	}
}
sealed class Mp4AtomIlstDataValue {
	data class Reserved(val _0: moq.Bytes) : Mp4AtomIlstDataValue()
	data class Utf8(val _0: kotlin.String) : Mp4AtomIlstDataValue()
	data class Utf16(val _0: kotlin.String) : Mp4AtomIlstDataValue()
	data class Jpeg(val _0: moq.Bytes) : Mp4AtomIlstDataValue()
	data class Png(val _0: moq.Bytes) : Mp4AtomIlstDataValue()
	data class Bmp(val _0: moq.Bytes) : Mp4AtomIlstDataValue()
	data class BeSignedInt(val _0: kotlin.Long) : Mp4AtomIlstDataValue()
	data class BeUnsignedInt(val _0: kotlin.ULong) : Mp4AtomIlstDataValue()
	data class BeFloat32(val _0: kotlin.Float) : Mp4AtomIlstDataValue() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class BeFloat64(val _0: kotlin.Double) : Mp4AtomIlstDataValue() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class Unknown(val _0: kotlin.UInt, val _1: moq.Bytes) : Mp4AtomIlstDataValue()
	fun debug(): kotlin.String
}
sealed class Mp4AtomAnySampleGroupEntry {
	data class DirectReferenceSampleList(val _0: kotlin.UInt, val _1: kotlin.UIntArray) : Mp4AtomAnySampleGroupEntry() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class UnknownGroupingType(val _0: moq.Mp4AtomFourCc, val _1: moq.Bytes) : Mp4AtomAnySampleGroupEntry()
	fun debug(): kotlin.String
}
sealed class Mp4AtomSubsSubsampleSize {
	data class U16(val _0: kotlin.UShort) : Mp4AtomSubsSubsampleSize()
	data class U32(val _0: kotlin.UInt) : Mp4AtomSubsSubsampleSize()
	fun debug(): kotlin.String
	fun value(): kotlin.UInt
	companion object {
		fun default(): moq.Mp4AtomSubsSubsampleSize
	}
}
enum class Level(internal val value: Int) {
	TRACE(0),
	DEBUG(1),
	INFO(2),
	WARN(3),
	ERROR(4);
	final override fun toString(): kotlin.String
	companion object {}
}
class Park internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Park
	fun hold(cx: moq.Waker): moq.Waiter
	companion object {
		fun default(): moq.Park
		fun new(waiter: moq.Waiter): moq.Park
	}
}
class Waiter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Waiter
	fun register(list: moq.WaiterList)
	fun waker(): moq.Waker
	fun context(): moq.Waker
	fun batch(poll: () -> kotlin.String): kotlin.String
	companion object {
		fun new(waker: moq.Waker): moq.Waiter
		fun noop(): moq.Waiter
		suspend fun pollFuture(future: suspend () -> kotlin.Unit)
	}
}
class WaiterList internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun register(waiter: moq.Waiter)
	fun take(): moq.WaiterList
	fun isEmpty(): kotlin.Boolean
	fun wake()
	companion object {
		fun default(): moq.WaiterList
		fun new(): moq.WaiterList
	}
}
class DecodeError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.DecodeError
}
class Driver internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun poll(now: moq.ClockInstant, waiter: moq.Waiter): moq.ClockInstant?
}
class Instance internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Instance
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun asStr(): kotlin.String
	fun isMinted(): kotlin.Boolean
	companion object {
		fun random(): moq.Instance
	}
}
class Path internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Path
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun hasPrefix(prefix: moq.Path): kotlin.Boolean
	fun stripPrefix(prefix: moq.Path): moq.Path?
	fun head(n: kotlin.ULong): moq.Path?
	fun parts(): kotlin.collections.List<kotlin.String>
	fun asStr(): kotlin.String
	fun isEmpty(): kotlin.Boolean
	fun len(): kotlin.ULong
	fun join(other: moq.Path): moq.Path
	fun resolve(rel: moq.PathRelative): moq.Path
	fun tryResolve(rel: moq.PathRelative): moq.Path?
	fun relative(base: moq.Path): moq.PathRelative?
	companion object {
		fun default(): moq.Path
		fun new(s: kotlin.String): moq.Path
		fun fromStatic(s: kotlin.String): moq.Path
		fun empty(): moq.Path
	}
}
class Pattern internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Pattern
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun asStr(): kotlin.String
	fun segments(): kotlin.collections.List<moq.Segment>
	fun head(): kotlin.String
	fun isLiteral(): kotlin.Boolean
	fun asPrefix(): kotlin.String?
	fun hasGlobstar(): kotlin.Boolean
	fun matches(path: kotlin.String): kotlin.Boolean
	fun contains(other: moq.Pattern): kotlin.Boolean
	fun overlaps(other: moq.Pattern): kotlin.Boolean
	fun specificity(): moq.Specificity
	fun rebase(root: kotlin.String): moq.Patterns
	fun intersect(other: moq.Pattern): moq.Patterns
	fun captures(matched: moq.Pattern): kotlin.collections.List<moq.Pattern>?
	fun rooted(root: kotlin.String): moq.Pattern
	companion object {
		fun default(): moq.Pattern
		fun new(segments: kotlin.collections.List<moq.Segment>): moq.Pattern
		fun all(): moq.Pattern
		fun literal(path: kotlin.String): moq.Pattern
		fun subtree(path: kotlin.String): moq.Pattern
	}
}
class Patterns internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Patterns
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun insert(pattern: moq.Pattern): kotlin.Boolean
	fun matches(path: kotlin.String): kotlin.Boolean
	fun contains(pattern: moq.Pattern): kotlin.Boolean
	fun covers(other: moq.Patterns): kotlin.Boolean
	fun overlaps(pattern: moq.Pattern): kotlin.Boolean
	fun intersect(other: moq.Patterns): moq.Patterns
	fun rebase(root: kotlin.String): moq.Patterns
	fun rooted(root: kotlin.String): moq.Patterns
	fun iter(): kotlin.collections.List<moq.Pattern>
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.Patterns
		fun new(): moq.Patterns
	}
}
class Server internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Server
	fun withPublisherProducer(publish: moq.OriginProducer): moq.Server
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.Server
	fun withSubscriber(subscribe: moq.OriginProducer): moq.Server
	fun withStats(stats: moq.StatsSession): moq.Server
	fun withLimits(limits: moq.SessionLimits): moq.Server
	fun withVersions(versions: moq.Versions): moq.Server
	fun withExtensions(extensions: moq.SetupExtensions): moq.Server
	fun withImplementation(implementation: kotlin.String): moq.Server
	suspend fun acceptSession(now: moq.ClockInstant, session: moq.TransportMemorySession): kotlin.Pair<moq.Session, moq.Driver>
	suspend fun acceptRequestSession(now: moq.ClockInstant, session: moq.TransportMemorySession): moq.ServerHandshake
	companion object {
		fun default(): moq.Server
		fun new(): moq.Server
	}
}
class Session internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Session
	fun version(): moq.Version
	suspend fun peerImplementation(): kotlin.String?
	fun sendBandwidth(): moq.BandwidthConsumer?
	fun allocator(): moq.BandwidthAllocator
	fun recvBandwidth(): moq.BandwidthConsumer?
	fun viewers(): moq.BandwidthViewers
	fun stats(): moq.SessionStats
	fun abort(err: moq.Error)
	suspend fun close_()
	suspend fun closed(): moq.Error
	fun drain(): moq.GoawayProducer
	fun draining(): moq.GoawayConsumer
	suspend fun idle()
	fun auth(): moq.AuthHandle
}
class Client internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Client
	fun withPublisherProducer(publish: moq.OriginProducer): moq.Client
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.Client
	fun withSubscriber(subscribe: moq.OriginProducer): moq.Client
	fun withStats(stats: moq.StatsSession): moq.Client
	fun withOrigin(origin: moq.OriginProducer): moq.Client
	fun withLimits(limits: moq.SessionLimits): moq.Client
	fun withAllocator(allocator: moq.BandwidthAllocator): moq.Client
	fun withViewers(viewers: moq.BandwidthViewers): moq.Client
	fun withVersions(versions: moq.Versions): moq.Client
	fun withUrl(url: moq.Url): moq.Client
	fun withToken(token: moq.SetupToken): moq.Client
	fun withAuth(auth: moq.AuthHandle): moq.Client
	fun withExtensions(extensions: moq.SetupExtensions): moq.Client
	fun withImplementation(implementation: kotlin.String): moq.Client
	fun withCost(cost: moq.OriginCost): moq.Client
	fun withPeerHop(hop: moq.Hop): moq.Client
	suspend fun connectSession(now: moq.ClockInstant, session: moq.TransportMemorySession): kotlin.Pair<moq.Session, moq.Driver>
	companion object {
		fun default(): moq.Client
		fun new(): moq.Client
	}
}
class Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Error
	fun session(): moq.SessionError?
	fun stream(): moq.StreamError?
	companion object {
		fun fromTransportError(err: moq.Error): moq.Error
		fun fromTransportMemoryError(err: moq.TransportMemoryError): moq.Error
	}
}
class Version internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Version
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun hasSetupToken(): kotlin.Boolean
	fun asStr(): kotlin.String
	fun code(): kotlin.ULong
	fun alpn(): kotlin.String
	fun isLite(): kotlin.Boolean
	companion object {
		fun fromCode(code: kotlin.ULong): moq.Version?
	}
}
class Versions internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Versions
	fun alpns(): kotlin.collections.List<kotlin.String>
	fun filter(other: moq.Versions): moq.Versions?
	fun select(version: moq.Version): moq.Version?
	fun contains(version: moq.Version): kotlin.Boolean
	fun iter(): kotlin.collections.List<moq.Version>
	companion object {
		fun default(): moq.Versions
		fun all(): moq.Versions
	}
}
class TimedJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TimedJson
	val valueGet: moq.Json
	fun valueSet(value: moq.Json)
	val atGet: moq.Timestamp?
	fun atSet(value: moq.Timestamp?)
	fun at(at: moq.Timestamp): moq.TimedJson
}
class Hops internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Hops
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun push(hop: moq.Hop)
	fun contains(hop: moq.Hop): kotlin.Boolean
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	fun iter(): kotlin.collections.List<moq.Hop>
	fun asSlice(): kotlin.collections.List<moq.Hop>
	companion object {
		fun default(): moq.Hops
		fun new(): moq.Hops
	}
}
class Specificity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Specificity
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class ClockClock internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ClockClock
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun now(): moq.ClockInstant
	fun wall(): moq.ClockWall
	companion object {
		fun current(): moq.ClockClock
	}
}
class ClockDeadline internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun set(at: moq.ClockInstant?)
	fun deadline(): moq.ClockInstant?
	suspend fun poll()
	companion object {
		fun new(deadlines: moq.ClockDeadlines): moq.ClockDeadline
		fun after(deadlines: moq.ClockDeadlines, duration: moq.Duration): moq.ClockDeadline
	}
}
class ClockDeadlines internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.ClockDeadlines
	fun advance(now: moq.ClockInstant)
	fun park(waiter: moq.Waiter): moq.ClockInstant?
	fun next(): moq.ClockInstant?
	fun now(): moq.ClockInstant
	fun enter(): moq.ClockEntered
	companion object {
		fun new(now: moq.ClockInstant): moq.ClockDeadlines
	}
}
class ClockEntered internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
}
class ClockManual internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ClockManual
	fun now(): moq.ClockInstant
	fun wall(): moq.ClockWall
	fun advance(duration: moq.Duration)
	fun stepWall(wall: moq.ClockWall)
	companion object {
		fun default(): moq.ClockManual
		val WALLSTART: moq.ClockWall
		fun new(): moq.ClockManual
	}
}
class ClockRng internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class ClockInterval internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun tick(): moq.ClockInstant
	fun setMissedTickBehavior(missed: moq.ClockMissedTickBehavior)
	fun period(): moq.Duration
}
class ClockWallMap internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ClockWallMap
	fun sample(now: moq.ClockInstant, wall: moq.ClockWall)
	fun map(at: moq.ClockWall): moq.ClockInstant?
	companion object {
		fun default(): moq.ClockWallMap
		val TOLERANCE: moq.Duration
	}
}
class CoopBudget internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.CoopBudget
	suspend fun pollYield()
	companion object {
		fun new(passes: kotlin.UInt): moq.CoopBudget
	}
}
class HangCatalog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalog
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
	companion object {
		fun default(): moq.HangCatalog
		val DEFAULTNAME: kotlin.String
		fun defaultTrackInfo(): moq.TrackInfo
		fun defaultSubscription(): moq.TrackSubscription
		suspend fun subscribe(broadcast: moq.BroadcastConsumer): moq.HangCatalogConsumer
		fun fromSlice(v: moq.Bytes): moq.HangCatalog
		fun fromReader(reader: moq.Bytes): moq.HangCatalog
	}
}
class HangCatalogExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogExtra
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	val extGet: moq.MuxCatalogHangExtra
	fun extSet(value: moq.MuxCatalogHangExtra)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
	companion object {
		fun default(): moq.HangCatalogExtra
		suspend fun subscribe(broadcast: moq.BroadcastConsumer): moq.HangCatalogConsumerExtra
		fun fromSlice(v: moq.Bytes): moq.HangCatalogExtra
		fun fromReader(reader: moq.Bytes): moq.HangCatalogExtra
	}
}
class HangCatalogExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogExt
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	val extGet: moq.MuxContainerTsExt
	fun extSet(value: moq.MuxContainerTsExt)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
	companion object {
		fun default(): moq.HangCatalogExt
		suspend fun subscribe(broadcast: moq.BroadcastConsumer): moq.HangCatalogConsumerExt
		fun fromSlice(v: moq.Bytes): moq.HangCatalogExt
		fun fromReader(reader: moq.Bytes): moq.HangCatalogExt
	}
}
class HangError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.HangError
}
class MuxClock internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxClock
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun now(): moq.Timestamp
	fun capture(at: moq.ClockInstant): moq.Timestamp
	fun timescale(): moq.Timescale
	fun wall(): moq.HangCatalogClock?
	fun wallClock(pts: moq.Timestamp): moq.ClockWall
	companion object {
		fun default(): moq.MuxClock
		val TIMESCALE: moq.Timescale
		fun new(): moq.MuxClock
		fun at(epoch: moq.ClockInstant, wall: moq.ClockWall): moq.MuxClock
	}
}
class MuxLingerUnit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun wait()
	companion object {
		val DEFAULT: moq.Duration
		fun new(broadcasts: kotlin.collections.List<moq.BroadcastProducer>, duration: moq.Duration): moq.MuxLingerUnit
	}
}
class MuxPacer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withLead(lead: moq.Duration): moq.MuxPacer
	fun withDelay(delay: moq.Duration): moq.MuxPacer
	fun pace(ts: moq.Timestamp, now: moq.ClockInstant): moq.ClockInstant
	fun due(ts: moq.Timestamp): moq.ClockInstant?
	fun slack(): moq.Duration
	fun hurry(ts: moq.Timestamp, now: moq.ClockInstant): moq.ClockInstant
	companion object {
		fun default(): moq.MuxPacer
	}
}
class MuxBinding internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun broadcast(): moq.BroadcastConsumer
	companion object {
		fun new(broadcast: moq.BroadcastConsumer): moq.MuxBinding
	}
}
class MuxSource internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxSource
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>
	fun broadcast(): moq.BroadcastConsumer
	suspend fun catalogExtra(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExtra
	suspend fun catalogExt(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExt
	suspend fun catalogUnit(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumer
	fun resolveReference(rel: moq.PathRelative?): moq.Path?
	suspend fun resolve(rel: moq.PathRelative?, instance: moq.Instance?): moq.BroadcastConsumer
	fun bind(rel: moq.PathRelative?, instance: moq.Instance?): moq.MuxBinding
	suspend fun subscribeTrack(rel: moq.PathRelative?, instance: moq.Instance?, name: kotlin.String): moq.TrackSubscriber
	fun follow(rel: moq.PathRelative?, instance: moq.Instance?): moq.MuxFollowInstances
	companion object {
		fun new(origin: moq.OriginConsumer, broadcast: moq.BroadcastConsumer): moq.MuxSource
	}
}
class MuxError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxError
}
class TokioDuration internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.TokioDuration
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.TokioDuration
	}
}
class TokioBackoff internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioBackoff
	val initialGet: moq.TokioDuration?
	fun initialSet(value: moq.TokioDuration?)
	val multiplierGet: kotlin.UInt
	fun multiplierSet(value: kotlin.UInt)
	val maxGet: moq.TokioDuration?
	fun maxSet(value: moq.TokioDuration?)
	val healthyGet: moq.TokioDuration?
	fun healthySet(value: moq.TokioDuration?)
	val timeoutGet: moq.TokioDuration?
	fun timeoutSet(value: moq.TokioDuration?)
	fun initial(): moq.Duration
	fun max(): moq.Duration
	fun timeout(): moq.Duration
	companion object {
		fun default(): moq.TokioBackoff
	}
}
class TokioClient internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioClient
	fun withPublisherProducer(publish: moq.OriginProducer): moq.TokioClient
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.TokioClient
	fun withOrigin(origin: moq.OriginProducer): moq.TokioClient
	fun withSubscriber(subscribe: moq.OriginProducer): moq.TokioClient
	fun withLimits(limits: moq.SessionLimits): moq.TokioClient
	fun withStats(stats: moq.StatsSession): moq.TokioClient
	fun withCost(cost: kotlin.ULong): moq.TokioClient
	fun withPeerHop(hop: moq.Hop): moq.TokioClient
	fun withReconnect(reconnect: kotlin.Boolean): moq.TokioClient
	fun withRestart(restart: moq.TokioRestart): moq.TokioClient
	fun connect(addrs: moq.MoqTokioAddrs): moq.TokioConnection
	suspend fun close_()
	fun publish(origin: moq.OriginConsumer): moq.TokioConnection?
	fun consume(origin: moq.OriginProducer): moq.TokioConnection?
	companion object {
		fun new(config: moq.TokioClientConfig): moq.TokioClient
	}
}
class TokioConnection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioConnection
	fun abort(err: moq.Error)
	suspend fun close_()
	suspend fun established(): moq.TokioConnection
	suspend fun status(): moq.TokioStatus
	fun connected(): kotlin.Boolean
	fun connects(): kotlin.ULong
	fun version(): moq.Version?
	fun transport(): moq.TokioTransport?
	fun draining(): moq.GoawayConsumer?
	fun sendBandwidth(): moq.BandwidthConsumer
	fun allocator(): moq.BandwidthAllocator
	fun recvBandwidth(): moq.BandwidthConsumer
	fun viewers(): moq.BandwidthViewers
	suspend fun closed()
	fun auth(): moq.AuthHandle
	fun tokens(): moq.AuthTokens
	fun monitor(): moq.TokioConnectionMonitor
}
class TokioError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.TokioError
	fun connectError(): moq.TokioConnectError?
	fun isAuth(): kotlin.Boolean
	fun status(): kotlin.UShort?
}
class TokioLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioLog
	val levelGet: moq.Level
	fun levelSet(value: moq.Level)
	fun level(): moq.Level?
	fun init()
	companion object {
		fun default(): moq.TokioLog
		fun new(level: moq.Level): moq.TokioLog
	}
}
class TokioServer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun servePublish(origin: moq.OriginConsumer)
	suspend fun serveConsume(origin: moq.OriginProducer)
	suspend fun serveBoth(publish: moq.OriginConsumer, subscribe: moq.OriginProducer)
	fun tlsAcceptor(): moq.MoqTokioTcpTlsAcceptor?
	fun certificates(): moq.TokioTlsCertificates
	fun acceptHealth(): kotlin.collections.List<moq.MoqTokioAcceptHealth>
	suspend fun bind(): moq.TokioServerListener
	suspend fun listen(): moq.TokioServerListener
	fun localAddr(): moq.SocketAddr
	fun websocketLocalAddr(): moq.SocketAddr?
	companion object {
		fun new(config: moq.TokioServerConfig): moq.TokioServer
	}
}
class AuthWatch internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun peek(): moq.AuthGrant?
	suspend fun changed(): moq.AuthGrant?
}
class AuthRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.AuthRequest?
}
class AuthRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun token(): moq.Bytes
	fun kind(): moq.AuthRequestKind?
	fun path(): moq.Path?
	suspend fun closed(): moq.Error
	fun accept(grant: moq.AuthGrant): moq.AuthIssued
	fun reject(code: moq.SessionError, reason: kotlin.String)
}
class AuthIssued internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun update(grant: moq.AuthGrant)
	fun revoke(code: moq.SessionError, reason: kotlin.String)
	suspend fun closed(): moq.Error
}
class AuthGrant internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AuthGrant
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val publishGet: moq.Patterns
	fun publishSet(value: moq.Patterns)
	val subscribeGet: moq.Patterns
	fun subscribeSet(value: moq.Patterns)
	val expiresGet: moq.ClockInstant?
	fun expiresSet(value: moq.ClockInstant?)
	companion object {
		fun default(): moq.AuthGrant
		fun all(): moq.AuthGrant
	}
}
class AuthHandle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.AuthHandle
	fun grant(): moq.AuthWatch
	suspend fun add(token: moq.Bytes): kotlin.Pair<moq.AuthCredential, moq.AuthAnswer>
	fun authorize(grant: moq.AuthGrant)
	fun abort(err: moq.Error)
	companion object {
		fun default(): moq.AuthHandle
		fun new(): moq.AuthHandle
	}
}
class AuthCredential internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AuthCredential
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun grant(): moq.AuthWatch
	suspend fun closed(): moq.Error
}
class AuthDial internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val urlGet: moq.Url
	fun urlSet(value: moq.Url)
	val tokenGet: moq.SetupToken?
	fun tokenSet(value: moq.SetupToken?)
}
class AuthToken internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AuthToken
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun asStr(): kotlin.String
}
class AuthTokens internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.AuthTokens
	fun handle_(): moq.AuthHandle
	fun set(tokens: kotlin.collections.List<moq.AuthToken>)
	suspend fun answered()
	fun dial(url: moq.Url, versions: moq.Versions): moq.AuthDial
	fun connected(session: moq.Session)
	fun changed(dial: moq.AuthDial): kotlin.Boolean
	companion object {
		fun new(tokens: kotlin.collections.List<moq.AuthToken>): moq.AuthTokens
	}
}
class GoawayProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun send(goaway: moq.GoawayGoaway)
}
class GoawayGoaway internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.GoawayGoaway
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun withTimeout(timeout: moq.Duration): moq.GoawayGoaway
	fun uri(): kotlin.String
	fun timeout(): moq.Duration?
	companion object {
		fun default(): moq.GoawayGoaway
		fun new(): moq.GoawayGoaway
		fun redirect(uri: kotlin.String): moq.GoawayGoaway
	}
}
class GoawayConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GoawayConsumer
	suspend fun poll(): moq.GoawayGoaway
}
class PathRelative internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.PathRelative
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun asStr(): kotlin.String
	fun isEmpty(): kotlin.Boolean
	fun len(): kotlin.ULong
	companion object {
		fun default(): moq.PathRelative
		fun new(s: kotlin.String): moq.PathRelative
		fun empty(): moq.PathRelative
	}
}
class ServerHandshake internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun path(): kotlin.String
	fun role(): moq.SetupRole?
	fun peerHop(): moq.Hop?
	fun peerImplementation(): kotlin.String?
	fun auth(): moq.AuthHandle
	fun requests(): moq.AuthRequests
	fun token(): moq.SetupToken?
	fun withPublisherProducer(publish: moq.OriginProducer): moq.ServerHandshake
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.ServerHandshake
	fun withSubscriber(subscribe: moq.OriginProducer): moq.ServerHandshake
	fun withPeerHop(hop: moq.Hop): moq.ServerHandshake
	fun withStats(stats: moq.StatsSession): moq.ServerHandshake
	fun withLimits(limits: moq.SessionLimits): moq.ServerHandshake
	fun withDeadline(at: moq.ClockInstant): moq.ServerHandshake
	suspend fun ok(): kotlin.Pair<moq.Session, moq.Driver>
	fun close_(err: moq.Error)
}
class ServerActivity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ServerActivity
	fun enter(): moq.ServerActive?
	suspend fun close_()
	companion object {
		fun default(): moq.ServerActivity
	}
}
class ServerActive internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
}
class SessionStats internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.SessionStats
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val rttGet: moq.Duration?
	fun rttSet(value: moq.Duration?)
	val estimatedSendRateGet: moq.BandwidthRate?
	fun estimatedSendRateSet(value: moq.BandwidthRate?)
	val bytesSentGet: kotlin.ULong?
	fun bytesSentSet(value: kotlin.ULong?)
	val bytesReceivedGet: kotlin.ULong?
	fun bytesReceivedSet(value: kotlin.ULong?)
	val bytesLostGet: kotlin.ULong?
	fun bytesLostSet(value: kotlin.ULong?)
	val packetsSentGet: kotlin.ULong?
	fun packetsSentSet(value: kotlin.ULong?)
	val packetsReceivedGet: kotlin.ULong?
	fun packetsReceivedSet(value: kotlin.ULong?)
	val packetsLostGet: kotlin.ULong?
	fun packetsLostSet(value: kotlin.ULong?)
	val streamsEvictedGet: kotlin.ULong?
	fun streamsEvictedSet(value: kotlin.ULong?)
	companion object {
		fun default(): moq.SessionStats
	}
}
class StatsTier internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.StatsTier
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun label(): moq.Path
	fun asStr(): kotlin.String
	companion object {
		fun default(): moq.StatsTier
		fun new(label: moq.Path): moq.StatsTier
	}
}
class StatsSnapshot internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsSnapshot
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun control(): kotlin.collections.List<kotlin.Triple<moq.StatsTier, moq.StatsMessage, moq.StatsTotals>>
	fun traffic(): kotlin.collections.List<kotlin.Triple<moq.StatsTier, moq.StatsRole, moq.StatsTraffic>>
	fun sessions(): kotlin.collections.List<kotlin.Pair<moq.StatsTier, moq.StatsPresence>>
	companion object {
		fun default(): moq.StatsSnapshot
	}
}
class StatsReport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsReport
	val trafficGet: kotlin.collections.List<moq.StatsTrafficEntry>
	fun trafficSet(value: kotlin.collections.List<moq.StatsTrafficEntry>)
	val sessionsGet: kotlin.collections.List<moq.StatsSessionEntry>
	fun sessionsSet(value: kotlin.collections.List<moq.StatsSessionEntry>)
	companion object {
		fun default(): moq.StatsReport
	}
}
class StatsTrafficEntry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsTrafficEntry
	val pathGet: moq.Path
	fun pathSet(value: moq.Path)
	val tierGet: moq.StatsTier
	fun tierSet(value: moq.StatsTier)
	val publisherGet: moq.StatsTraffic
	fun publisherSet(value: moq.StatsTraffic)
	val subscriberGet: moq.StatsTraffic
	fun subscriberSet(value: moq.StatsTraffic)
	val closedGet: kotlin.Boolean
	fun closedSet(value: kotlin.Boolean)
}
class StatsSessionEntry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsSessionEntry
	val tierGet: moq.StatsTier
	fun tierSet(value: moq.StatsTier)
	val labelGet: moq.Path
	fun labelSet(value: moq.Path)
	val presenceGet: moq.StatsPresence
	fun presenceSet(value: moq.StatsPresence)
	val closedGet: kotlin.Boolean
	fun closedSet(value: kotlin.Boolean)
}
class StatsConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsConfig
	val excludeGet: moq.Patterns
	fun excludeSet(value: moq.Patterns)
	fun withExclude(pattern: moq.Pattern): moq.StatsConfig
	companion object {
		fun default(): moq.StatsConfig
		fun new(): moq.StatsConfig
	}
}
class StatsRegistry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsRegistry
	fun isEnabled(): kotlin.Boolean
	fun tier(tier: moq.StatsTier): moq.StatsHandle
	fun snapshot(): moq.StatsSnapshot
	fun report(report: moq.StatsReport)
	companion object {
		fun default(): moq.StatsRegistry
		fun new(config: moq.StatsConfig): moq.StatsRegistry
		fun disabled(): moq.StatsRegistry
	}
}
class StatsHandle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsHandle
	fun refuse(label: moq.Path, reason: moq.StatsRefusalKind)
	fun session(label: moq.Path): moq.StatsSession
	fun request(label: moq.Path): moq.StatsSession
	companion object {
		fun default(): moq.StatsHandle
	}
}
class StatsWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		fun default(): moq.StatsWire
	}
}
class StatsSession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsSession
	fun wire(counters: moq.TransportWire): moq.StatsWire
	fun retag(tier: moq.StatsTier, label: moq.Path)
	companion object {
		fun default(): moq.StatsSession
	}
}
class TransportWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TransportWire
	fun sentBytes(): kotlin.ULong
	fun receivedBytes(): kotlin.ULong
	fun setSentBytes(total: kotlin.ULong)
	fun setReceivedBytes(total: kotlin.ULong)
	companion object {
		fun default(): moq.TransportWire
		fun new(): moq.TransportWire
	}
}
class TransportProbe internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TransportProbe
	fun target(): moq.BandwidthRate
	fun setTarget(rate: moq.BandwidthRate)
	companion object {
		fun default(): moq.TransportProbe
		fun new(): moq.TransportProbe
	}
}
class BandwidthDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthDemand
	fun wanted(): kotlin.Boolean
	suspend fun wantedChanged(): kotlin.Boolean
}
class BandwidthReservation internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun rate(): moq.BandwidthRate?
	fun consume(): moq.BandwidthConsumer
	fun update(max: moq.BandwidthRate)
}
class BandwidthViewer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun rate(): moq.BandwidthRate?
	fun consume(): moq.BandwidthConsumer
	fun update(max: moq.BandwidthRate)
}
class BandwidthProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthProducer
	fun set(bitrate: moq.BandwidthRate?)
	fun consume(): moq.BandwidthConsumer
	fun weak(): moq.BandwidthWeak
	fun demand(): moq.BandwidthDemand
	companion object {
		fun default(): moq.BandwidthProducer
		fun new(): moq.BandwidthProducer
	}
}
class BandwidthWeak internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthWeak
	fun consume(): moq.BandwidthConsumer
}
class BandwidthAllocator internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BandwidthAllocator
	fun setTarget(target: moq.BandwidthRate?)
	fun reserve(track: moq.TrackDemand, max: moq.BandwidthRate): moq.BandwidthReservation
	companion object {
		fun default(): moq.BandwidthAllocator
		fun new(estimate: moq.BandwidthConsumer): moq.BandwidthAllocator
		fun unlimited(): moq.BandwidthAllocator
	}
}
class BandwidthViewers internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BandwidthViewers
	fun watch(max: moq.BandwidthRate): moq.BandwidthViewer
	fun setTarget(target: moq.BandwidthRate?)
	companion object {
		fun default(): moq.BandwidthViewers
		val MAXTARGET: moq.BandwidthRate
		fun new(estimate: moq.BandwidthWeak): moq.BandwidthViewers
	}
}
class BandwidthConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthConsumer
	fun peek(): moq.BandwidthRate?
	fun probed(): kotlin.Boolean
	suspend fun changed(): moq.BandwidthRate?
}
class BroadcastDatagramRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastDatagramRequests
	suspend fun requested(): moq.DatagramRequest
}
class BroadcastConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastConsumer
	fun tracks(): moq.BroadcastTracks
	fun info(): moq.BroadcastInfo
	fun track(name: kotlin.String): moq.TrackConsumer
	fun datagrams(name: kotlin.String): moq.DatagramConsumer
	fun isClosed(): kotlin.Boolean
	suspend fun closed()
	fun isClone(other: moq.BroadcastConsumer): kotlin.Boolean
}
class BroadcastDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastDemand
	fun wanted(): kotlin.Boolean
	suspend fun wantedChanged(): kotlin.Boolean
}
class BroadcastInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BroadcastInfo
	val poolGet: moq.CachePool
	fun poolSet(value: moq.CachePool)
	val cacheDurationGet: moq.Duration
	fun cacheDurationSet(value: moq.Duration)
	val pathGet: moq.Path
	fun pathSet(value: moq.Path)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	fun produce(): moq.BroadcastProducer
	companion object {
		fun default(): moq.BroadcastInfo
		fun new(): moq.BroadcastInfo
	}
}
class BroadcastProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastProducer
	fun info(): moq.BroadcastInfo
	fun demand(): moq.BroadcastDemand
	fun createTrack(name: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	fun createDatagrams(name: kotlin.String, info: moq.DatagramInfo): moq.DatagramProducer
	fun reserveTrack(name: kotlin.String): moq.TrackRequest
	fun uniqueTrack(suffix: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	fun uniqueName(suffix: kotlin.String): kotlin.String
	fun requests(): moq.BroadcastRequests
	fun consume(): moq.BroadcastConsumer
	fun close_()
	fun isClone(other: moq.BroadcastProducer): kotlin.Boolean
	companion object {
		fun new(info: moq.BroadcastInfo): moq.BroadcastProducer
	}
}
class BroadcastRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastRequests
	fun info(): moq.BroadcastInfo
	suspend fun requestedTrack(): moq.TrackRequest
	fun consume(): moq.BroadcastConsumer
	fun datagrams(): moq.BroadcastDatagramRequests
	fun lister(): moq.BroadcastLister
	suspend fun closed()
	fun isClone(other: moq.BroadcastRequests): kotlin.Boolean
}
class BroadcastLister internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun pollWanted()
	suspend fun pollUnwanted()
	fun add(name: kotlin.String, info: moq.TrackInfo)
	fun remove(name: kotlin.String)
	fun finish(name: kotlin.String)
	fun synced()
	fun clear()
	fun end()
	fun abort(err: moq.Error)
}
class BroadcastTracks internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.BroadcastTracksEvent?
	fun broadcast(): moq.BroadcastConsumer
}
class BroadcastTracksEvent internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BroadcastTracksEvent
}
class CacheConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.CacheConfig
	fun withCapacity(capacity: kotlin.ULong?): moq.CacheConfig
	fun withExpiry(expiry: moq.Duration?): moq.CacheConfig
	companion object {
		fun default(): moq.CacheConfig
	}
}
class CachePool internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.CachePool
	fun capacity(): kotlin.ULong?
	fun used(): kotlin.ULong
	fun resize(capacity: kotlin.ULong?)
	fun expiry(): moq.Duration?
	fun poll(now: moq.ClockInstant, waiter: moq.Waiter): moq.ClockInstant?
	fun downgrade(): moq.CachePoolWeak
	companion object {
		fun default(): moq.CachePool
		fun new(config: moq.CacheConfig): moq.CachePool
		fun unbounded(): moq.CachePool
	}
}
class CachePoolWeak internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.CachePoolWeak
	fun upgrade(): moq.CachePool?
}
class DatagramProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.DatagramProducer
	fun name(): kotlin.String
	fun info(): moq.DatagramInfo
	fun append(timestamp: moq.Timestamp?, payload: moq.Bytes): kotlin.ULong
	fun insert(datagram: moq.Datagram)
	fun consume(): moq.DatagramConsumer
	fun finish()
	fun abort(err: moq.Error)
	fun priority(): kotlin.Byte?
	suspend fun pollPriority(last: kotlin.Byte?): kotlin.Byte?
	fun isClone(other: moq.DatagramProducer): kotlin.Boolean
}
class DatagramRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun priority(): kotlin.Byte?
	fun accept(info: moq.DatagramInfo): moq.DatagramProducer
	fun reject(err: moq.Error)
}
class DatagramConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.DatagramConsumer
	fun name(): kotlin.String
	suspend fun info(): moq.DatagramInfo
	suspend fun recv(): moq.Datagram?
	fun priority(): kotlin.Byte
	fun setPriority(priority: kotlin.Byte)
	fun isClone(other: moq.DatagramConsumer): kotlin.Boolean
}
class DatagramInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.DatagramInfo
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val timescaleGet: moq.Timescale?
	fun timescaleSet(value: moq.Timescale?)
	val priorityGet: kotlin.Byte
	fun prioritySet(value: kotlin.Byte)
	fun withTimescale(timescale: moq.Timescale?): moq.DatagramInfo
	fun withPriority(priority: kotlin.Byte): moq.DatagramInfo
	fun rescale(timestamp: moq.Timestamp?): moq.Timestamp?
	fun produce(name: kotlin.String): moq.DatagramProducer
	companion object {
		fun default(): moq.DatagramInfo
	}
}
class FrameProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun group(): moq.GroupProducer
	fun remaining(): kotlin.ULong
	fun write(chunk: moq.Bytes)
	fun finish(): moq.GroupProducer
	fun abort(err: moq.Error)
	val sizeGet: kotlin.ULong
	val timestampGet: moq.Timestamp?
}
class FrameFrame internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.FrameFrame
	val timestampGet: moq.Timestamp?
	fun timestampSet(value: moq.Timestamp?)
	val payloadGet: moq.Bytes
	fun payloadSet(value: moq.Bytes)
	fun arrived(): moq.ClockInstant
	companion object {
		fun new(timestamp: moq.Timestamp?, payload: moq.Bytes): moq.FrameFrame
	}
}
class FrameBuffer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun capacity(): kotlin.ULong
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	fun filled(): kotlin.collections.List<moq.FrameFrame>
	fun push(frame: moq.FrameFrame)
	fun clear()
	companion object {
		fun default(): moq.FrameBuffer
		fun new(): moq.FrameBuffer
	}
}
class FrameConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.FrameConsumer
	suspend fun readChunk(): moq.Bytes?
	suspend fun readAll(): moq.Bytes
	val sizeGet: kotlin.ULong
	val timestampGet: moq.Timestamp?
}
class GroupProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun timescale(): moq.Timescale?
	fun startAt(index: kotlin.ULong)
	fun writeFrame(timestamp: moq.Timestamp?, data: moq.Bytes)
	fun writeFrames8(frames: moq.FrameBuffer)
	fun createFrame(frame: moq.FrameInfo): moq.FrameProducer
	fun finish()
	fun abort(err: moq.Error)
	suspend fun closed()
	suspend fun ready(frame: moq.FrameInfo)
	fun frameCount(): kotlin.ULong
	fun consume(): moq.GroupConsumer
	fun demand(): moq.GroupDemand
	val sequenceGet: kotlin.ULong
}
class GroupDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GroupDemand
	fun sequence(): kotlin.ULong
	fun wanted(): kotlin.Boolean
	suspend fun wantedChanged(): kotlin.Boolean
	suspend fun closed()
}
class GroupConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GroupConsumer
	fun keepAlive()
	fun timescale(): moq.Timescale?
	fun arrived(): moq.ClockInstant
	fun index(): kotlin.ULong
	fun withRange(frames: moq.BoundsU64): moq.GroupConsumer
	fun skipTo(index: kotlin.ULong)
	fun frameCount(): kotlin.ULong
	suspend fun nextFrame(): moq.FrameConsumer?
	suspend fun readFrame(): moq.FrameFrame?
	suspend fun pollReadFrames8(out: moq.FrameBuffer): kotlin.ULong
	suspend fun readFrames8(out: moq.FrameBuffer): kotlin.collections.List<moq.FrameFrame>
	suspend fun finished(): kotlin.ULong
	val sequenceGet: kotlin.ULong
}
class GroupFetch internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.GroupFetch
	val priorityGet: kotlin.Byte
	fun prioritySet(value: kotlin.Byte)
	val maxDelayGet: moq.Duration?
	fun maxDelaySet(value: moq.Duration?)
	fun withPriority(priority: kotlin.Byte): moq.GroupFetch
	fun withMaxDelay(maxDelay: moq.Duration): moq.GroupFetch
	companion object {
		fun default(): moq.GroupFetch
	}
}
class TrackRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackRequests
	fun name(): kotlin.String
	suspend fun requestedFill(): moq.TrackFill
	fun demand(): moq.TrackDemand
}
class TrackDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackDemand
	fun name(): kotlin.String
	fun wanted(): kotlin.ULong?
	suspend fun wantedChanged(): kotlin.ULong?
	suspend fun closed()
}
class TrackSubscriber internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun info(): moq.TrackInfo
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	fun control(): moq.TrackControl
	suspend fun recvGroup(): moq.GroupConsumer?
	fun ordered(): moq.TrackOrdered
	fun isClone(other: moq.TrackSubscriber): kotlin.Boolean
	fun floor(): kotlin.ULong?
	fun nextInRange(from: kotlin.ULong): kotlin.ULong?
	suspend fun finished(): moq.TrackPosition
	fun withRange(groups: moq.BoundsU64): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription
	fun update(subscription: moq.TrackSubscription)
	suspend fun largest(): moq.TrackPosition?
}
class TrackOrdered internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun info(): moq.TrackInfo
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	suspend fun nextGroup(): moq.GroupConsumer?
	fun withRange(groups: moq.BoundsU64): moq.TrackOrdered
	fun control(): moq.TrackControl
	fun subscription(): moq.TrackSubscription
	fun update(subscription: moq.TrackSubscription)
	suspend fun finished(): moq.TrackPosition
	suspend fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackOrdered): kotlin.Boolean
}
class TrackRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun consume(): moq.TrackConsumer
	fun requests(): moq.TrackRequests
	fun demand(): moq.TrackDemand
	fun accept(info: moq.TrackInfo?): moq.TrackProducer
	fun reject(err: moq.Error)
	fun subscription(): moq.TrackSubscription?
	suspend fun subscriptionChanged(): moq.TrackSubscription?
}
class TrackInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TrackInfo
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val timescaleGet: moq.Timescale?
	fun timescaleSet(value: moq.Timescale?)
	val maxAgeGet: moq.Duration?
	fun maxAgeSet(value: moq.Duration?)
	val priorityGet: kotlin.Byte
	fun prioritySet(value: kotlin.Byte)
	val compressGet: kotlin.Boolean
	fun compressSet(value: kotlin.Boolean)
	val refreshGet: kotlin.UInt?
	fun refreshSet(value: kotlin.UInt?)
	fun withTimescale(timescale: moq.Timescale?): moq.TrackInfo
	fun withMaxAge(maxAge: moq.Duration?): moq.TrackInfo
	fun withPriority(priority: kotlin.Byte): moq.TrackInfo
	fun withCompress(compress: kotlin.Boolean): moq.TrackInfo
	fun withRefresh(refresh: kotlin.UInt?): moq.TrackInfo
	fun rescale(timestamp: moq.Timestamp?): moq.Timestamp?
	companion object {
		fun default(): moq.TrackInfo
	}
}
class TrackProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackProducer
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun writeFrame(timestamp: moq.Timestamp?, frame: moq.Bytes)
	fun finish()
	fun finishAt(end: moq.TrackPosition, error: moq.Error?)
	fun startAt(sequence: kotlin.ULong?)
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	fun abort(err: moq.Error)
	fun abortUnwanted(err: moq.Error)
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun demand(): moq.TrackDemand
	fun consume(): moq.TrackConsumer
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	suspend fun subscriptionChanged(): moq.TrackSubscription?
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class TrackConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackConsumer
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	suspend fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	suspend fun fetchGroup(sequence: kotlin.ULong, options: moq.GroupFetch?): moq.GroupConsumer
	suspend fun info(): moq.TrackInfo
	suspend fun largest(): moq.TrackPosition?
}
class TrackControl internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackControl
	fun subscription(): moq.TrackSubscription
	fun update(subscription: moq.TrackSubscription)
}
class TrackSubscription internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TrackSubscription
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val priorityGet: kotlin.Byte
	fun prioritySet(value: kotlin.Byte)
	val delayGet: moq.TrackDelay
	fun delaySet(value: moq.TrackDelay)
	val rangesGet: kotlin.collections.List<moq.TrackRange>
	fun rangesSet(value: kotlin.collections.List<moq.TrackRange>)
	val orderGet: moq.TrackOrder
	fun orderSet(value: moq.TrackOrder)
	fun withPriority(priority: kotlin.Byte): moq.TrackSubscription
	fun withDelay(delay: moq.TrackDelay): moq.TrackSubscription
	fun withRanges(ranges: kotlin.collections.List<moq.TrackRange>): moq.TrackSubscription
	fun withRange(groups: moq.BoundsU64): moq.TrackSubscription
	fun withOrder(order: moq.TrackOrder): moq.TrackSubscription
	companion object {
		fun default(): moq.TrackSubscription
	}
}
class TrackFill internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackFill
	fun start(): moq.TrackPosition
	fun frameStart(sequence: kotlin.ULong): kotlin.ULong
	fun end(): kotlin.ULong
	fun time(): moq.Timestamp?
	fun priority(): kotlin.Byte
	fun maxDelay(): moq.Duration?
	fun order(): moq.TrackOrder
	fun accept(sequence: kotlin.ULong, info: moq.TrackInfo?): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64)
	fun refuseGroups(groups: moq.RangeU64, err: moq.Error)
	fun isWanted(sequence: kotlin.ULong): kotlin.Boolean
	suspend fun unwanted(sequence: kotlin.ULong)
	fun wanted(): kotlin.ULong?
	suspend fun wantedChanged(): kotlin.ULong?
}
class OriginFollowing internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun available(): moq.BroadcastConsumer
}
class OriginConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginConfig
	val hopGet: moq.Hop
	fun hopSet(value: moq.Hop)
	val maxHopsGet: moq.OriginHopLimit
	fun maxHopsSet(value: moq.OriginHopLimit)
	val poolGet: moq.CachePool
	fun poolSet(value: moq.CachePool)
	val cacheDurationGet: moq.Duration
	fun cacheDurationSet(value: moq.Duration)
	val updateHoldGet: moq.Duration
	fun updateHoldSet(value: moq.Duration)
	companion object {
		fun default(): moq.OriginConfig
		fun new(hop: moq.Hop): moq.OriginConfig
	}
}
class OriginConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginConsumer
	fun hop(): moq.Hop
	fun local(): moq.OriginConsumer
	fun withHidden(hidden: kotlin.Boolean): moq.OriginConsumer
	fun covering(): moq.OriginConsumer
	fun exclude(prefix: kotlin.String): moq.OriginConsumer
	fun withStats(session: moq.StatsSession): moq.OriginConsumer
	fun announced(): moq.AnnounceConsumer
	fun consume(): moq.OriginConsumer
	fun follow(path: kotlin.String, instance: moq.Instance?): moq.OriginFollowing
	fun scope(root: kotlin.String, patterns: moq.Patterns): moq.OriginConsumer
	suspend fun request(path: kotlin.String, instance: moq.Instance?): moq.BroadcastConsumer
	fun root(): moq.Path
	fun allowed(): moq.Patterns
	fun absolute(path: kotlin.String): moq.Path
}
class OriginDriver internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun poll(now: moq.ClockInstant, waiter: moq.Waiter): moq.ClockInstant?
}
class OriginPrepared internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun broadcast(): moq.BroadcastProducer
	fun publish(): moq.OriginPublication
}
class OriginProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginProducer
	fun withStats(session: moq.StatsSession): moq.OriginProducer
	fun withLink(link: moq.OriginLink): moq.OriginProducer
	fun withPreference(preference: moq.OriginPreference): moq.OriginProducer
	fun config(): moq.OriginConfig
	fun hop(): moq.Hop
	fun createBroadcast(path: kotlin.String): moq.OriginPublication
	fun publish(path: kotlin.String, route: moq.OriginRoute): moq.OriginPublication
	fun requests(prefix: kotlin.String): moq.OriginRequests
	fun scope(root: kotlin.String, patterns: moq.Patterns): moq.OriginProducer
	fun mount(at: kotlin.String, target: kotlin.String): moq.OriginProducer
	fun consume(): moq.OriginConsumer
	fun root(): moq.Path
	fun allowed(): moq.Patterns
	fun absolute(path: kotlin.String): moq.Path
	companion object {
		fun new(config: moq.OriginConfig): kotlin.Pair<moq.OriginProducer, moq.OriginDriver>
	}
}
class OriginPublication internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginPublication
	fun broadcast(): moq.BroadcastProducer
	fun announce(route: moq.OriginRoute)
	fun unannounce()
	fun route(): moq.OriginRoute?
	fun info(): moq.BroadcastInfo
	fun demand(): moq.BroadcastDemand
	fun createTrack(name: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	fun createDatagrams(name: kotlin.String, info: moq.DatagramInfo): moq.DatagramProducer
	fun reserveTrack(name: kotlin.String): moq.TrackRequest
	fun uniqueTrack(suffix: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	fun uniqueName(suffix: kotlin.String): kotlin.String
	fun requests(): moq.BroadcastRequests
	fun consume(): moq.BroadcastConsumer
	fun close_()
	fun isClone(other: moq.BroadcastProducer): kotlin.Boolean
}
class OriginRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun path(): moq.Path
	fun prepare(): moq.OriginPrepared
	fun serveProducer(broadcast: moq.BroadcastProducer)
	fun serveConsumer(broadcast: moq.BroadcastConsumer)
	fun reject(err: moq.Error)
}
class OriginRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun route(): moq.OriginRoute
	suspend fun requestedBroadcast(): moq.OriginRequest
}
class OriginRoute internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginRoute
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val hopsGet: moq.Hops
	fun hopsSet(value: moq.Hops)
	val costGet: moq.OriginCost
	fun costSet(value: moq.OriginCost)
	val authGet: moq.AuthCredential?
	fun authSet(value: moq.AuthCredential?)
	fun withInstance(instance: moq.Instance, epoch: kotlin.ULong): moq.OriginRoute
	fun withHops(hops: moq.Hops): moq.OriginRoute
	fun withCost(cost: moq.OriginCost): moq.OriginRoute
	fun withAuth(auth: moq.AuthCredential): moq.OriginRoute
	fun isAnonymous(): kotlin.Boolean
	fun source(): moq.OriginSource
	companion object {
		fun default(): moq.OriginRoute
	}
}
class AnnounceConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.AnnounceEvent?
	fun tryNext(): moq.AnnounceEvent?
	fun isClosed(): kotlin.Boolean
	suspend fun pollSynced()
	fun root(): moq.Path
	fun absolute(prefix: kotlin.String): moq.Path
}
class AnnounceEvent internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AnnounceEvent
}
class HangStatsSnapshot internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangStatsSnapshot
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val transportGet: moq.HangStatsTransport
	fun transportSet(value: moq.HangStatsTransport)
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>)
	companion object {
		fun default(): moq.HangStatsSnapshot
	}
}
class HangStatsSnapshotString internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangStatsSnapshotString
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val transportGet: moq.HangStatsTransport
	fun transportSet(value: moq.HangStatsTransport)
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>)
	val extGet: kotlin.String
	fun extSet(value: kotlin.String)
	companion object {
		fun default(): moq.HangStatsSnapshotString
	}
}
class HangStatsRendition internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangStatsRendition
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val framesGet: kotlin.ULong
	fun framesSet(value: kotlin.ULong)
	val bytesGet: kotlin.ULong
	fun bytesSet(value: kotlin.ULong)
	val keyframesGet: kotlin.ULong
	fun keyframesSet(value: kotlin.ULong)
	val skippedGet: kotlin.ULong
	fun skippedSet(value: kotlin.ULong)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	companion object {
		fun default(): moq.HangStatsRendition
	}
}
class HangStatsTransport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangStatsTransport
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val rttGet: moq.Duration?
	fun rttSet(value: moq.Duration?)
	val rateGet: kotlin.ULong?
	fun rateSet(value: kotlin.ULong?)
	val lostBytesGet: kotlin.ULong?
	fun lostBytesSet(value: kotlin.ULong?)
	val lostPacketsGet: kotlin.ULong?
	fun lostPacketsSet(value: kotlin.ULong?)
	val ageGet: moq.Duration?
	fun ageSet(value: moq.Duration?)
	companion object {
		fun default(): moq.HangStatsTransport
	}
}
class HangEchoSnapshot internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangEchoSnapshot
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val transportGet: moq.HangStatsTransport
	fun transportSet(value: moq.HangStatsTransport)
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>)
	companion object {
		fun default(): moq.HangEchoSnapshot
	}
}
class HangEchoSnapshotString internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangEchoSnapshotString
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val transportGet: moq.HangStatsTransport
	fun transportSet(value: moq.HangStatsTransport)
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>)
	val extGet: kotlin.String
	fun extSet(value: kotlin.String)
	companion object {
		fun default(): moq.HangEchoSnapshotString
	}
}
class HangEchoRendition internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangEchoRendition
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val framesGet: kotlin.ULong
	fun framesSet(value: kotlin.ULong)
	val bytesGet: kotlin.ULong
	fun bytesSet(value: kotlin.ULong)
	val decodedGet: kotlin.ULong
	fun decodedSet(value: kotlin.ULong)
	val lateGet: kotlin.ULong
	fun lateSet(value: kotlin.ULong)
	val errorsGet: kotlin.ULong
	fun errorsSet(value: kotlin.ULong)
	val stallsGet: kotlin.ULong
	fun stallsSet(value: kotlin.ULong)
	val underrunsGet: kotlin.ULong
	fun underrunsSet(value: kotlin.ULong)
	val stalledGet: moq.Duration
	fun stalledSet(value: moq.Duration)
	val arrivalGet: moq.HangEchoArrival?
	fun arrivalSet(value: moq.HangEchoArrival?)
	val latencyGet: moq.Duration?
	fun latencySet(value: moq.Duration?)
	companion object {
		fun default(): moq.HangEchoRendition
	}
}
class HangEchoArrival internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangEchoArrival
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val timestampGet: moq.Timestamp
	fun timestampSet(value: moq.Timestamp)
	val wallGet: moq.ClockWall
	fun wallSet(value: moq.ClockWall)
	companion object {
		fun new(timestamp: moq.Timestamp, wall: moq.ClockWall): moq.HangEchoArrival
	}
}
class HangHealthObserver internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthObserver
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val pathGet: moq.Path
	fun pathSet(value: moq.Path)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	companion object {
		fun new(path: moq.Path, instance: moq.Instance?): moq.HangHealthObserver
	}
}
class HangHealthReportEchoSnapshotString internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthReportEchoSnapshotString
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val observerGet: moq.HangHealthObserver
	fun observerSet(value: moq.HangHealthObserver)
	val snapshotGet: moq.HangEchoSnapshotString
	fun snapshotSet(value: moq.HangEchoSnapshotString)
	companion object {
		fun new(observer: moq.HangHealthObserver, snapshot: moq.HangEchoSnapshotString): moq.HangHealthReportEchoSnapshotString
	}
}
class HangHealthReportStatsSnapshotString internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthReportStatsSnapshotString
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val observerGet: moq.HangHealthObserver
	fun observerSet(value: moq.HangHealthObserver)
	val snapshotGet: moq.HangStatsSnapshotString
	fun snapshotSet(value: moq.HangStatsSnapshotString)
	companion object {
		fun new(observer: moq.HangHealthObserver, snapshot: moq.HangStatsSnapshotString): moq.HangHealthReportStatsSnapshotString
	}
}
class HangHealthThresholds internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthThresholds
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val staleGet: moq.Duration
	fun staleSet(value: moq.Duration)
	val transportGet: moq.HangHealthTransportThresholds
	fun transportSet(value: moq.HangHealthTransportThresholds)
	val publisherGet: moq.HangHealthPublisherThresholds
	fun publisherSet(value: moq.HangHealthPublisherThresholds)
	val viewerGet: moq.HangHealthViewerThresholds
	fun viewerSet(value: moq.HangHealthViewerThresholds)
	companion object {
		fun default(): moq.HangHealthThresholds
	}
}
class HangHealthTransportThresholds internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthTransportThresholds
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val rttGet: moq.HangHealthLimitsDuration
	fun rttSet(value: moq.HangHealthLimitsDuration)
	val lossGet: moq.HangHealthLimitsF64
	fun lossSet(value: moq.HangHealthLimitsF64)
	val headroomGet: moq.HangHealthLimitsF64
	fun headroomSet(value: moq.HangHealthLimitsF64)
}
class HangHealthPublisherThresholds internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthPublisherThresholds
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val skippedGet: moq.HangHealthLimitsF64
	fun skippedSet(value: moq.HangHealthLimitsF64)
}
class HangHealthViewerThresholds internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthViewerThresholds
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val lateGet: moq.HangHealthLimitsF64
	fun lateSet(value: moq.HangHealthLimitsF64)
	val errorsGet: moq.HangHealthLimitsF64
	fun errorsSet(value: moq.HangHealthLimitsF64)
	val stalledGet: moq.HangHealthLimitsF64
	fun stalledSet(value: moq.HangHealthLimitsF64)
}
class HangHealthEvidence internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthEvidence
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val inputGet: moq.HangHealthInput
	fun inputSet(value: moq.HangHealthInput)
	val valueGet: kotlin.Double?
	fun valueSet(value: kotlin.Double?)
	val verdictGet: moq.HangHealthVerdict
	fun verdictSet(value: moq.HangHealthVerdict)
}
class HangHealthConnection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthConnection
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val verdictGet: moq.HangHealthVerdict
	fun verdictSet(value: moq.HangHealthVerdict)
	val evidenceGet: kotlin.collections.List<moq.HangHealthEvidence>
	fun evidenceSet(value: kotlin.collections.List<moq.HangHealthEvidence>)
	val staleGet: kotlin.Boolean
	fun staleSet(value: kotlin.Boolean)
	val lostBytesGet: kotlin.ULong?
	fun lostBytesSet(value: kotlin.ULong?)
	val lostPacketsGet: kotlin.ULong?
	fun lostPacketsSet(value: kotlin.ULong?)
	companion object {
		fun default(): moq.HangHealthConnection
	}
}
class HangHealthRendition internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangHealthRendition
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val verdictGet: moq.HangHealthVerdict
	fun verdictSet(value: moq.HangHealthVerdict)
	val evidenceGet: kotlin.collections.List<moq.HangHealthEvidence>
	fun evidenceSet(value: kotlin.collections.List<moq.HangHealthEvidence>)
	val framesGet: kotlin.ULong?
	fun framesSet(value: kotlin.ULong?)
	val bytesGet: kotlin.ULong?
	fun bytesSet(value: kotlin.ULong?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	companion object {
		fun default(): moq.HangHealthRendition
	}
}
class HangCatalogConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalog?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.HangCatalogConsumer
	}
}
class HangCatalogConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExtra?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.HangCatalogConsumerExtra
	}
}
class HangCatalogConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExt?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.HangCatalogConsumerExt
	}
}
class HangCatalogAudio internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogAudio
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>)
	fun insert(name: kotlin.String, config: moq.HangCatalogAudioConfig)
	fun remove(name: kotlin.String): moq.HangCatalogAudioConfig?
	fun isEmpty(): kotlin.Boolean
	fun enabled(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>
	fun ranked(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>
	companion object {
		fun default(): moq.HangCatalogAudio
	}
}
class HangCatalogAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogAudioConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative?
	fun broadcastSet(value: moq.PathRelative?)
	val trackGet: kotlin.String?
	fun trackSet(value: kotlin.String?)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val labelGet: kotlin.String?
	fun labelSet(value: kotlin.String?)
	val codecGet: moq.HangCatalogAudioCodec
	fun codecSet(value: moq.HangCatalogAudioCodec)
	val sampleRateGet: kotlin.UInt
	fun sampleRateSet(value: kotlin.UInt)
	val channelCountGet: kotlin.UInt
	fun channelCountSet(value: kotlin.UInt)
	val layoutGet: moq.HangCatalogAudioLayout?
	fun layoutSet(value: moq.HangCatalogAudioLayout?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val enabledGet: kotlin.Boolean
	fun enabledSet(value: kotlin.Boolean)
	val descriptionGet: moq.Bytes?
	fun descriptionSet(value: moq.Bytes?)
	val containerGet: moq.HangCatalogContainer
	fun containerSet(value: moq.HangCatalogContainer)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val warmupGet: moq.Duration?
	fun warmupSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	fun sameIdentity(other: moq.HangCatalogAudioConfig): kotlin.Boolean
	companion object {
		fun new(codec: moq.HangCatalogAudioCodec, sampleRate: kotlin.UInt, channelCount: kotlin.UInt): moq.HangCatalogAudioConfig
	}
}
class HangCatalogBinary internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogBinary
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val tracksGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogBinaryConfig>>
	fun tracksSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogBinaryConfig>>)
	fun insert(name: kotlin.String, config: moq.HangCatalogBinaryConfig)
	fun remove(name: kotlin.String): moq.HangCatalogBinaryConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.HangCatalogBinary
	}
}
class HangCatalogBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogBinaryConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative?
	fun broadcastSet(value: moq.PathRelative?)
	val trackGet: kotlin.String?
	fun trackSet(value: kotlin.String?)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val modeGet: moq.HangCatalogMode
	fun modeSet(value: moq.HangCatalogMode)
	val mimeGet: kotlin.String?
	fun mimeSet(value: kotlin.String?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	val extraGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.Json>>
	fun extraSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.Json>>)
	companion object {
		fun new(mode: moq.HangCatalogMode): moq.HangCatalogBinaryConfig
	}
}
class HangCatalogContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogContainer
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.HangCatalogContainer
	}
}
class HangCatalogContribution internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogContribution
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative
	fun broadcastSet(value: moq.PathRelative)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	companion object {
		fun new(broadcast: moq.PathRelative): moq.HangCatalogContribution
	}
}
class HangCatalogEcho internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogEcho
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val pathGet: moq.PathRelative
	fun pathSet(value: moq.PathRelative)
	companion object {
		fun new(path: moq.PathRelative): moq.HangCatalogEcho
	}
}
class HangCatalogJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogJson
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val tracksGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogJsonConfig>>
	fun tracksSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogJsonConfig>>)
	fun insert(name: kotlin.String, config: moq.HangCatalogJsonConfig)
	fun remove(name: kotlin.String): moq.HangCatalogJsonConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.HangCatalogJson
	}
}
class HangCatalogJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogJsonConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative?
	fun broadcastSet(value: moq.PathRelative?)
	val trackGet: kotlin.String?
	fun trackSet(value: kotlin.String?)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val modeGet: moq.HangCatalogMode
	fun modeSet(value: moq.HangCatalogMode)
	val schemaGet: kotlin.String?
	fun schemaSet(value: kotlin.String?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	val extraGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.Json>>
	fun extraSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.Json>>)
	companion object {
		fun new(mode: moq.HangCatalogMode): moq.HangCatalogJsonConfig
	}
}
class HangCatalogM2ts internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogM2ts
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val trackGet: kotlin.String
	fun trackSet(value: kotlin.String)
	val randomAccessGet: kotlin.Boolean
	fun randomAccessSet(value: kotlin.Boolean)
	val muxRateGet: kotlin.ULong?
	fun muxRateSet(value: kotlin.ULong?)
	companion object {
		fun new(track: kotlin.String): moq.HangCatalogM2ts
	}
}
class HangCatalogMedia internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogMedia
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val clockGet: moq.HangCatalogClock?
	fun clockSet(value: moq.HangCatalogClock?)
	val joinGet: moq.HangCatalogJoin?
	fun joinSet(value: moq.HangCatalogJoin?)
	fun isEmpty(): kotlin.Boolean
	fun interval(): moq.Duration?
	companion object {
		fun default(): moq.HangCatalogMedia
	}
}
class HangCatalogJoin internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogJoin
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val intervalGet: moq.Duration
	fun intervalSet(value: moq.Duration)
	companion object {
		fun new(interval: moq.Duration): moq.HangCatalogJoin?
	}
}
class HangCatalogStats internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogStats
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val trackGet: kotlin.String
	fun trackSet(value: kotlin.String)
	companion object {
		fun new(track: kotlin.String): moq.HangCatalogStats
	}
}
class HangCatalogText internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogText
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogTextConfig>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogTextConfig>>)
	fun insert(name: kotlin.String, config: moq.HangCatalogTextConfig)
	fun remove(name: kotlin.String): moq.HangCatalogTextConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.HangCatalogText
	}
}
class HangCatalogTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogTextConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative?
	fun broadcastSet(value: moq.PathRelative?)
	val trackGet: kotlin.String?
	fun trackSet(value: kotlin.String?)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val formatGet: moq.HangCatalogTextFormat
	fun formatSet(value: moq.HangCatalogTextFormat)
	val roleGet: moq.HangCatalogTextRole
	fun roleSet(value: moq.HangCatalogTextRole)
	val langGet: kotlin.String?
	fun langSet(value: kotlin.String?)
	val labelGet: kotlin.String?
	fun labelSet(value: kotlin.String?)
	val containerGet: moq.HangCatalogContainer
	fun containerSet(value: moq.HangCatalogContainer)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	companion object {
		fun new(format: moq.HangCatalogTextFormat): moq.HangCatalogTextConfig
	}
}
class HangCatalogColor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogColor
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val primariesGet: kotlin.UByte?
	fun primariesSet(value: kotlin.UByte?)
	val transferGet: kotlin.UByte?
	fun transferSet(value: kotlin.UByte?)
	val matrixGet: kotlin.UByte?
	fun matrixSet(value: kotlin.UByte?)
	val rangeGet: moq.HangCatalogRange?
	fun rangeSet(value: moq.HangCatalogRange?)
	val masteringGet: moq.HangCatalogMastering?
	fun masteringSet(value: moq.HangCatalogMastering?)
	val contentLightGet: moq.HangCatalogContentLight?
	fun contentLightSet(value: moq.HangCatalogContentLight?)
	companion object {
		fun default(): moq.HangCatalogColor
	}
}
class HangCatalogProjection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogProjection
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.HangCatalogProjection
	}
}
class HangCatalogVideo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogVideo
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val renditionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>
	fun renditionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>)
	val displayGet: moq.HangCatalogDisplay?
	fun displaySet(value: moq.HangCatalogDisplay?)
	val rotationGet: kotlin.Double?
	fun rotationSet(value: kotlin.Double?)
	val flipGet: kotlin.Boolean?
	fun flipSet(value: kotlin.Boolean?)
	fun insert(name: kotlin.String, config: moq.HangCatalogVideoConfig)
	fun remove(name: kotlin.String): moq.HangCatalogVideoConfig?
	fun isEmpty(): kotlin.Boolean
	fun enabled(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>
	fun ranked(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>
	fun setProperties(properties: moq.HangCatalogVideoProperties)
	companion object {
		fun default(): moq.HangCatalogVideo
	}
}
class HangCatalogVideoProperties internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogVideoProperties
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val displayGet: moq.HangCatalogDisplay?
	fun displaySet(value: moq.HangCatalogDisplay?)
	val rotationGet: kotlin.Double?
	fun rotationSet(value: kotlin.Double?)
	val flipGet: kotlin.Boolean?
	fun flipSet(value: kotlin.Boolean?)
	companion object {
		fun default(): moq.HangCatalogVideoProperties
	}
}
class HangCatalogVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogVideoConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative?
	fun broadcastSet(value: moq.PathRelative?)
	val trackGet: kotlin.String?
	fun trackSet(value: kotlin.String?)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	val labelGet: kotlin.String?
	fun labelSet(value: kotlin.String?)
	val codecGet: moq.HangCatalogVideoCodec
	fun codecSet(value: moq.HangCatalogVideoCodec)
	val descriptionGet: moq.Bytes?
	fun descriptionSet(value: moq.Bytes?)
	val codedWidthGet: kotlin.UInt?
	fun codedWidthSet(value: kotlin.UInt?)
	val codedHeightGet: kotlin.UInt?
	fun codedHeightSet(value: kotlin.UInt?)
	val displayAspectWidthGet: kotlin.UInt?
	fun displayAspectWidthSet(value: kotlin.UInt?)
	val displayAspectHeightGet: kotlin.UInt?
	fun displayAspectHeightSet(value: kotlin.UInt?)
	val colorGet: moq.HangCatalogColor?
	fun colorSet(value: moq.HangCatalogColor?)
	val projectionGet: moq.HangCatalogProjection?
	fun projectionSet(value: moq.HangCatalogProjection?)
	val stereoGet: moq.HangCatalogStereo?
	fun stereoSet(value: moq.HangCatalogStereo?)
	val interlacedGet: kotlin.Boolean
	fun interlacedSet(value: kotlin.Boolean)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val enabledGet: kotlin.Boolean
	fun enabledSet(value: kotlin.Boolean)
	val preferenceGet: kotlin.Int
	fun preferenceSet(value: kotlin.Int)
	val framerateGet: kotlin.Double?
	fun framerateSet(value: kotlin.Double?)
	val reorderGet: kotlin.UByte?
	fun reorderSet(value: kotlin.UByte?)
	val optimizeForLatencyGet: kotlin.Boolean?
	fun optimizeForLatencySet(value: kotlin.Boolean?)
	val containerGet: moq.HangCatalogContainer
	fun containerSet(value: moq.HangCatalogContainer)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val warmupGet: moq.Duration?
	fun warmupSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	fun sameIdentity(other: moq.HangCatalogVideoConfig): kotlin.Boolean
	companion object {
		fun new(codec: moq.HangCatalogVideoCodec): moq.HangCatalogVideoConfig
	}
}
class MuxCatalogConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumer
	}
}
class MuxCatalogConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExtra
	}
}
class MuxCatalogConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExt
	}
}
class MuxCatalogEstimate internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCatalogEstimate
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	val framerateGet: kotlin.Double?
	fun framerateSet(value: kotlin.Double?)
	fun withJitter(jitter: moq.Duration?): moq.MuxCatalogEstimate
	fun withBitrate(bitrate: kotlin.ULong?): moq.MuxCatalogEstimate
	fun withDelay(delay: moq.Duration?): moq.MuxCatalogEstimate
	fun withFramerate(framerate: kotlin.Double?): moq.MuxCatalogEstimate
	companion object {
		fun default(): moq.MuxCatalogEstimate
	}
}
class MuxCatalogEstimator internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun write(timestamp: moq.Timestamp, bytes: kotlin.ULong)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun reorder(delay: moq.Timestamp)
	fun estimate(): moq.MuxCatalogEstimate
	companion object {
		fun default(): moq.MuxCatalogEstimator
		fun new(): moq.MuxCatalogEstimator
	}
}
class MuxCatalogConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogConfig
	fun withCatalogExtra(catalog: moq.HangCatalogExtra): moq.MuxCatalogConfigExtra
	fun withCatalogExt(catalog: moq.HangCatalogExt): moq.MuxCatalogConfigExt
	fun withCatalogUnit(catalog: moq.HangCatalog): moq.MuxCatalogConfig
	fun withClock(clock: moq.MuxClock): moq.MuxCatalogConfig
	fun withMaxAge(maxAge: moq.Duration?): moq.MuxCatalogConfig
	fun withBandwidth(bandwidth: moq.BandwidthAllocator): moq.MuxCatalogConfig
	companion object {
		fun default(): moq.MuxCatalogConfig
	}
}
class MuxCatalogConfigExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogConfigExtra
	fun withCatalogExtra(catalog: moq.HangCatalogExtra): moq.MuxCatalogConfigExtra
	fun withCatalogExt(catalog: moq.HangCatalogExt): moq.MuxCatalogConfigExt
	fun withCatalogUnit(catalog: moq.HangCatalog): moq.MuxCatalogConfig
	fun withClock(clock: moq.MuxClock): moq.MuxCatalogConfigExtra
	fun withMaxAge(maxAge: moq.Duration?): moq.MuxCatalogConfigExtra
	fun withBandwidth(bandwidth: moq.BandwidthAllocator): moq.MuxCatalogConfigExtra
}
class MuxCatalogConfigExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogConfigExt
	fun withCatalogExtra(catalog: moq.HangCatalogExtra): moq.MuxCatalogConfigExtra
	fun withCatalogExt(catalog: moq.HangCatalogExt): moq.MuxCatalogConfigExt
	fun withCatalogUnit(catalog: moq.HangCatalog): moq.MuxCatalogConfig
	fun withClock(clock: moq.MuxClock): moq.MuxCatalogConfigExt
	fun withMaxAge(maxAge: moq.Duration?): moq.MuxCatalogConfigExt
	fun withBandwidth(bandwidth: moq.BandwidthAllocator): moq.MuxCatalogConfigExt
}
class MuxCatalogGuard internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun commit()
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
}
class MuxCatalogGuardExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun commit()
	fun setSection(name: kotlin.String, value: moq.Json)
	fun removeSection(name: kotlin.String): moq.Json?
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	val extGet: moq.MuxCatalogHangExtra
	fun extSet(value: moq.MuxCatalogHangExtra)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
}
class MuxCatalogGuardExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun commit()
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val mediaGet: moq.HangCatalogMedia
	fun mediaSet(value: moq.HangCatalogMedia)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val m2tsGet: moq.HangCatalogM2ts?
	fun m2tsSet(value: moq.HangCatalogM2ts?)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val contributionsGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>
	fun contributionsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogContribution>>)
	val extGet: moq.MuxContainerTsExt
	fun extSet(value: moq.MuxContainerTsExt)
	fun checkRenditions()
	fun media(): moq.HangCatalog
	fun toJson(): kotlin.String
	fun toJsonPretty(): kotlin.String
	fun toVec(): moq.Bytes
	fun toWriter(writer: moq.Bytes)
}
class MuxCatalogProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducer
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun mutate(f: (moq.HangCatalog) -> kotlin.Unit)
	fun modify(): moq.MuxCatalogGuard
	fun snapshot(): moq.HangCatalog
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebase
	fun reserve(): moq.MuxCatalogReserved
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	fun consume(): moq.MuxCatalogHangConsumer
	fun abort(err: moq.Error)
	fun finish()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfig): moq.MuxCatalogProducer
	}
}
class MuxCatalogProducerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducerExtra
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun mutate(f: (moq.HangCatalogExtra) -> kotlin.Unit)
	fun modify(): moq.MuxCatalogGuardExtra
	fun snapshot(): moq.HangCatalogExtra
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebaseExtra
	fun reserve(): moq.MuxCatalogReservedExtra
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	fun consume(): moq.MuxCatalogHangConsumerExtra
	fun abort(err: moq.Error)
	fun finish()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfigExtra): moq.MuxCatalogProducerExtra
	}
}
class MuxCatalogProducerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducerExt
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun mutate(f: (moq.HangCatalogExt) -> kotlin.Unit)
	fun modify(): moq.MuxCatalogGuardExt
	fun snapshot(): moq.HangCatalogExt
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebaseExt
	fun reserve(): moq.MuxCatalogReservedExt
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	fun consume(): moq.MuxCatalogHangConsumerExt
	fun abort(err: moq.Error)
	fun finish()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfigExt): moq.MuxCatalogProducerExt
	}
}
class MuxCatalogTimebase internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebase
	fun reserve(): moq.MuxCatalogReserved
	fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogTimebaseExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebaseExtra
	fun reserve(): moq.MuxCatalogReservedExtra
	fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogTimebaseExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebaseExt
	fun reserve(): moq.MuxCatalogReservedExt
	fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogReserved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReserved
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReserved
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
}
class MuxCatalogReservedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReservedExtra
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReservedExtra
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
}
class MuxCatalogReservedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReservedExt
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReservedExt
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
}
class MuxCatalogVideoHint internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCatalogVideoHint
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val codecGet: moq.HangCatalogVideoCodec?
	fun codecSet(value: moq.HangCatalogVideoCodec?)
	val codedWidthGet: kotlin.UInt?
	fun codedWidthSet(value: kotlin.UInt?)
	val codedHeightGet: kotlin.UInt?
	fun codedHeightSet(value: kotlin.UInt?)
	val displayAspectWidthGet: kotlin.UInt?
	fun displayAspectWidthSet(value: kotlin.UInt?)
	val displayAspectHeightGet: kotlin.UInt?
	fun displayAspectHeightSet(value: kotlin.UInt?)
	val colorGet: moq.HangCatalogColor?
	fun colorSet(value: moq.HangCatalogColor?)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	val framerateGet: kotlin.Double?
	fun framerateSet(value: kotlin.Double?)
	val reorderGet: kotlin.UByte?
	fun reorderSet(value: kotlin.UByte?)
	val optimizeForLatencyGet: kotlin.Boolean?
	fun optimizeForLatencySet(value: kotlin.Boolean?)
	val jitterGet: moq.Duration?
	fun jitterSet(value: moq.Duration?)
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	val containerGet: moq.HangCatalogContainer
	fun containerSet(value: moq.HangCatalogContainer)
	val preferenceGet: kotlin.Int
	fun preferenceSet(value: kotlin.Int)
	fun apply(config: moq.HangCatalogVideoConfig)
	fun toConfig(): moq.HangCatalogVideoConfig?
	companion object {
		fun default(): moq.MuxCatalogVideoHint
	}
}
class MuxContainerConsumerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withWarmup(warmup: moq.Duration): moq.MuxContainerConsumerContainer
	fun withDelay(delay: moq.TrackDelay): moq.MuxContainerConsumerContainer
	suspend fun read(): moq.MuxContainerRead?
	fun setDelay(delay: moq.TrackDelay)
	fun setMaxBuffer(bytes: kotlin.ULong)
	companion object {
		fun new(track: moq.TrackSubscriber, format: moq.MuxCatalogHangContainer): moq.MuxContainerConsumerContainer
	}
}
class MuxContainerConsumerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withWarmup(warmup: moq.Duration): moq.MuxContainerConsumerFmp4Wire
	fun withDelay(delay: moq.TrackDelay): moq.MuxContainerConsumerFmp4Wire
	suspend fun read(): moq.MuxContainerRead?
	fun setDelay(delay: moq.TrackDelay)
	fun setMaxBuffer(bytes: kotlin.ULong)
	companion object {
		fun new(track: moq.TrackSubscriber, format: moq.MuxContainerFmp4Wire): moq.MuxContainerConsumerFmp4Wire
	}
}
class MuxContainerConsumerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withWarmup(warmup: moq.Duration): moq.MuxContainerConsumerLegacyWire
	fun withDelay(delay: moq.TrackDelay): moq.MuxContainerConsumerLegacyWire
	suspend fun read(): moq.MuxContainerRead?
	fun setDelay(delay: moq.TrackDelay)
	fun setMaxBuffer(bytes: kotlin.ULong)
	companion object {
		fun new(track: moq.TrackSubscriber, format: moq.MuxContainerLegacyWire): moq.MuxContainerConsumerLegacyWire
	}
}
class MuxContainerConsumerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withWarmup(warmup: moq.Duration): moq.MuxContainerConsumerLocWire
	fun withDelay(delay: moq.TrackDelay): moq.MuxContainerConsumerLocWire
	suspend fun read(): moq.MuxContainerRead?
	fun setDelay(delay: moq.TrackDelay)
	fun setMaxBuffer(bytes: kotlin.ULong)
	companion object {
		fun new(track: moq.TrackSubscriber, format: moq.MuxContainerLocWire): moq.MuxContainerConsumerLocWire
	}
}
class MuxContainerGroupConsumerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sequence(): kotlin.ULong
	suspend fun read(): moq.MuxContainerFrame?
	companion object {
		fun new(group: moq.GroupConsumer, format: moq.MuxCatalogHangContainer): moq.MuxContainerGroupConsumerContainer
	}
}
class MuxContainerGroupConsumerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sequence(): kotlin.ULong
	suspend fun read(): moq.MuxContainerFrame?
	companion object {
		fun new(group: moq.GroupConsumer, format: moq.MuxContainerFmp4Wire): moq.MuxContainerGroupConsumerFmp4Wire
	}
}
class MuxContainerGroupConsumerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sequence(): kotlin.ULong
	suspend fun read(): moq.MuxContainerFrame?
	companion object {
		fun new(group: moq.GroupConsumer, format: moq.MuxContainerLegacyWire): moq.MuxContainerGroupConsumerLegacyWire
	}
}
class MuxContainerGroupConsumerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sequence(): kotlin.ULong
	suspend fun read(): moq.MuxContainerFrame?
	companion object {
		fun new(group: moq.GroupConsumer, format: moq.MuxContainerLocWire): moq.MuxContainerGroupConsumerLocWire
	}
}
class MuxContainerProducerContainerAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogAudioConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainerAudioConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainerAudioConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogBinaryConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainerBinaryConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainerBinaryConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogJsonConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainerJsonConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainerJsonConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogTextConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainerTextConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainerTextConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogVideoConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainerVideoConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainerVideoConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set()
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerContainer
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerContainer
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		fun new(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer): moq.MuxContainerProducerContainer
	}
}
class MuxContainerProducerFmp4WireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogAudioConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4WireAudioConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4WireAudioConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogBinaryConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4WireBinaryConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogJsonConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4WireJsonConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4WireJsonConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogTextConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4WireTextConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4WireTextConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogVideoConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4WireVideoConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4WireVideoConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set()
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerFmp4Wire
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerFmp4Wire
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		fun new(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire): moq.MuxContainerProducerFmp4Wire
	}
}
class MuxContainerProducerLegacyWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogAudioConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWireAudioConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWireAudioConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogBinaryConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWireBinaryConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogJsonConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWireJsonConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWireJsonConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogTextConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWireTextConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWireTextConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogVideoConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWireVideoConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWireVideoConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set()
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLegacyWire
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLegacyWire
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		fun new(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire): moq.MuxContainerProducerLegacyWire
	}
}
class MuxContainerProducerLocWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogAudioConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWireAudioConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWireAudioConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogBinaryConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWireBinaryConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWireBinaryConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogJsonConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWireJsonConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWireJsonConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogTextConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWireTextConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWireTextConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set(config: moq.HangCatalogVideoConfig)
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWireVideoConfig
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWireVideoConfig
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun set()
	fun setBitrate(bitrate: kotlin.ULong)
	fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun estimate(): moq.MuxCatalogEstimate
	fun reorder(delay: moq.Timestamp)
	fun history(until: moq.Timestamp)
	fun needsKeyframe(): kotlin.Boolean
	fun withBuffer(duration: moq.Duration): moq.MuxContainerProducerLocWire
	fun withBandwidth(allocator: moq.BandwidthAllocator): moq.MuxContainerProducerLocWire
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun track(): moq.TrackProducer
	fun liveEdge(): moq.Timestamp?
	fun write(frame: moq.MuxContainerFrame)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	fun finish()
	fun fail(err: moq.Error)
	fun abort(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	fun appendGroup(): moq.GroupProducer
	fun dropGroups(groups: moq.RangeU64, err: moq.StreamError)
	fun finished(): moq.TrackPosition?
	suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		fun new(track: moq.TrackProducer, container: moq.MuxContainerLocWire): moq.MuxContainerProducerLocWire
	}
}
class MuxFollowInstances internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.MuxFollowEvent
	companion object {
		fun new(origin: moq.OriginConsumer, path: kotlin.String, instance: moq.Instance?): moq.MuxFollowInstances
	}
}
class MuxFollowEvent internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class MuxImportAudioInit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxImportAudioInit
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val formatGet: moq.MuxImportAudioFormat
	fun formatSet(value: moq.MuxImportAudioFormat)
	val dataGet: moq.Bytes
	fun dataSet(value: moq.Bytes)
	val labelGet: kotlin.String?
	fun labelSet(value: kotlin.String?)
	val containerGet: moq.HangCatalogContainer
	fun containerSet(value: moq.HangCatalogContainer)
	val bitrateGet: kotlin.ULong?
	fun bitrateSet(value: kotlin.ULong?)
	companion object {
		fun new(format: moq.MuxImportAudioFormat, data: moq.Bytes): moq.MuxImportAudioInit
	}
}
class MuxImportContainerInit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxImportContainerInit
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val formatGet: moq.MuxImportContainerFormat
	fun formatSet(value: moq.MuxImportContainerFormat)
	val dataGet: moq.Bytes
	fun dataSet(value: moq.Bytes)
	companion object {
		fun new(format: moq.MuxImportContainerFormat, data: moq.Bytes): moq.MuxImportContainerInit
	}
}
class MuxImportVideoInit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxImportVideoInit
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val formatGet: moq.MuxImportVideoFormat
	fun formatSet(value: moq.MuxImportVideoFormat)
	val dataGet: moq.Bytes
	fun dataSet(value: moq.Bytes)
	val labelGet: kotlin.String?
	fun labelSet(value: kotlin.String?)
	val hintGet: moq.MuxCatalogVideoHint
	fun hintSet(value: moq.MuxCatalogVideoHint)
	val priorityGet: kotlin.Byte
	fun prioritySet(value: kotlin.Byte)
	companion object {
		fun new(format: moq.MuxImportVideoFormat, data: moq.Bytes): moq.MuxImportVideoInit
	}
}
class MuxImportContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun cut()
	fun seek(sequence: kotlin.ULong)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved, init: moq.MuxImportContainerInit): moq.MuxImportContainer
	}
}
class MuxImportContainerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun cut()
	fun seek(sequence: kotlin.ULong)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportContainerInit): moq.MuxImportContainerExtra
	}
}
class MuxImportContainerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun cut()
	fun seek(sequence: kotlin.ULong)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportContainerInit): moq.MuxImportContainerExt
	}
}
class MuxImportTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun seek(sequence: kotlin.ULong)
	fun demand(): moq.TrackDemand
	fun name(): kotlin.String
	companion object {
		fun audioExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		fun audioExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		fun audioUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		fun videoExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportVideoInit): moq.MuxImportTrack
		fun videoExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportVideoInit): moq.MuxImportTrack
		fun videoUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportVideoInit): moq.MuxImportTrack
	}
}
class MuxImportTrackStream internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(data: moq.Bytes)
	fun decode(data: moq.Bytes)
	fun finish()
	fun setEnabled(enabled: kotlin.Boolean)
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun seek(sequence: kotlin.ULong)
	fun demand(): moq.TrackDemand
	fun name(): kotlin.String
	companion object {
		fun videoExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
		fun videoExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
		fun videoUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
	}
}
class MuxJsonSnapshotJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun consume(): moq.TrackSubscriber
	fun demand(): moq.TrackDemand
	suspend fun refresh()
	fun update(value: moq.Json, timestamp: moq.Timestamp)
	fun finish()
	fun abort(err: moq.Error)
}
class MuxJsonStreamJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun consume(): moq.TrackSubscriber
	fun demand(): moq.TrackDemand
	fun append(value: moq.Json, timestamp: moq.Timestamp)
	fun finish()
	fun abort(err: moq.Error)
}
class MuxJsonConsumerJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun mode(): moq.HangCatalogMode
	suspend fun next(): moq.TimedJson?
	suspend fun latest(): moq.TimedJson?
	companion object {
		fun fromTrack(track: moq.TrackSubscriber, config: moq.HangCatalogJsonConfig): moq.MuxJsonConsumerJson
	}
}
class MuxJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxJsonConfig
	val schemaGet: kotlin.String?
	fun schemaSet(value: kotlin.String?)
	fun withSchema(schema: kotlin.String): moq.MuxJsonConfig
	companion object {
		fun default(): moq.MuxJsonConfig
	}
}
class MuxJsonOptions internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxJsonOptions
	val deltasGet: kotlin.Boolean
	fun deltasSet(value: kotlin.Boolean)
	fun withDeltas(deltas: kotlin.Boolean): moq.MuxJsonOptions
	companion object {
		fun default(): moq.MuxJsonOptions
	}
}
class MuxRateFollower internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun policy(): moq.MuxRatePolicy
	fun setPolicy(policy: moq.MuxRatePolicy)
	suspend fun next(): moq.MuxRateTarget
	fun failed()
	companion object {
		fun new(grant: moq.BandwidthConsumer, policy: moq.MuxRatePolicy): moq.MuxRateFollower
	}
}
class MuxRatePolicy internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxRatePolicy
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val maxGet: moq.BandwidthRate
	fun maxSet(value: moq.BandwidthRate)
	val minGet: moq.BandwidthRate
	fun minSet(value: moq.BandwidthRate)
	val lowerGet: moq.BandwidthRate?
	fun lowerSet(value: moq.BandwidthRate?)
	companion object {
		fun new(max: moq.BandwidthRate): moq.MuxRatePolicy
	}
}
class MuxRateControl internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxRateControl
	fun target(): moq.MuxRateTarget
	fun setProbed(probed: kotlin.Boolean)
	fun update(estimate: moq.BandwidthRate?, now: moq.ClockInstant): moq.MuxRateTarget?
	companion object {
		fun new(policy: moq.MuxRatePolicy): moq.MuxRateControl
	}
}
class MuxSelectBroadcast internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSelectBroadcast
	fun video(video: moq.MuxSelectVideo): moq.MuxSelectBroadcast
	fun audio(audio: moq.MuxSelectAudio): moq.MuxSelectBroadcast
	fun hasVideo(): kotlin.Boolean
	fun hasAudio(): kotlin.Boolean
	fun retainExtra(catalog: moq.HangCatalogExtra, selected: moq.MuxSelectSelected)
	fun retainExt(catalog: moq.HangCatalogExt, selected: moq.MuxSelectSelected)
	fun retainUnit(catalog: moq.HangCatalog, selected: moq.MuxSelectSelected)
	companion object {
		fun default(): moq.MuxSelectBroadcast
	}
}
class MuxSelectVideo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSelectVideo
	fun name(name: kotlin.String): moq.MuxSelectVideo
	fun codec(codec: moq.HangCatalogVideoCodecKind): moq.MuxSelectVideo
	companion object {
		fun default(): moq.MuxSelectVideo
	}
}
class MuxSelectAudio internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSelectAudio
	fun name(name: kotlin.String): moq.MuxSelectAudio
	fun codec(codec: moq.HangCatalogAudioCodecKind): moq.MuxSelectAudio
	companion object {
		fun default(): moq.MuxSelectAudio
	}
}
class MuxTimelineTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun retire(drained: moq.ClockInstant?)
	fun map(source: moq.Timestamp): moq.Timestamp
	fun declare(): moq.MuxTimelineTransition
	fun end(source: moq.Timestamp)
	fun plan(transition: moq.MuxTimelineTransition, source: moq.Timestamp, delay: moq.Duration?)
	fun gap()
	fun abandon(transition: moq.MuxTimelineTransition)
}
class MuxTimelineProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxTimelineProducer
	fun reset(delay: moq.Duration): moq.MuxTimelineProducer
	fun consume(): moq.MuxTimelineConsumer
	fun track(kind: moq.MuxTimelineKind): moq.MuxTimelineTrack
	fun video(timestamp: moq.Timestamp, now: moq.ClockInstant): kotlin.Boolean
	fun videoRestarted()
	fun audio(audio: moq.MuxTimelineAudio, now: moq.ClockInstant): kotlin.Boolean
	fun audioRestarted()
	fun audioStopped()
	fun due(timestamp: moq.Timestamp): moq.ClockInstant?
	companion object {
		fun new(delay: moq.Duration): moq.MuxTimelineProducer
	}
}
class MuxTimelineConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxTimelineConsumer
	fun playhead(now: moq.ClockInstant): moq.Duration?
	fun target(): moq.Duration?
	fun due(timestamp: moq.Timestamp): moq.ClockInstant?
	fun present(timestamps: kotlin.collections.List<moq.Timestamp>, now: moq.ClockInstant): kotlin.ULong?
}
class TokioTlsError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
class TokioTlsIdentity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsIdentity
	fun fingerprint(): kotlin.String
	companion object {
		fun generate(hostnames: kotlin.collections.List<kotlin.String>): moq.TokioTlsIdentity
	}
}
class TokioTlsPeers internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsPeers
	fun insert(fingerprint: kotlin.String)
	fun remove(fingerprint: kotlin.String)
	fun contains(fingerprint: kotlin.String): kotlin.Boolean
	companion object {
		fun default(): moq.TokioTlsPeers
		fun new(): moq.TokioTlsPeers
	}
}
class TokioTlsConnect internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsConnect
	val rootGet: kotlin.collections.List<moq.PathBuf>
	fun rootSet(value: kotlin.collections.List<moq.PathBuf>)
	val systemRootsGet: kotlin.Boolean?
	fun systemRootsSet(value: kotlin.Boolean?)
	val fingerprintGet: kotlin.collections.List<kotlin.String>
	fun fingerprintSet(value: kotlin.collections.List<kotlin.String>)
	val identityGet: moq.TokioTlsIdentity?
	fun identitySet(value: moq.TokioTlsIdentity?)
	val certGet: moq.PathBuf?
	fun certSet(value: moq.PathBuf?)
	val keyGet: moq.PathBuf?
	fun keySet(value: moq.PathBuf?)
	val insecureGet: kotlin.Boolean?
	fun insecureSet(value: kotlin.Boolean?)
	val hostNameGet: kotlin.String?
	fun hostNameSet(value: kotlin.String?)
	fun deprecated(): moq.MoqTokioCliDeprecated
	fun build(): moq.RustlsClientConfig
	companion object {
		fun default(): moq.TokioTlsConnect
	}
}
class TokioTlsListen internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsListen
	val certGet: kotlin.collections.List<moq.PathBuf>
	fun certSet(value: kotlin.collections.List<moq.PathBuf>)
	val keyGet: kotlin.collections.List<moq.PathBuf>
	fun keySet(value: kotlin.collections.List<moq.PathBuf>)
	val generateGet: kotlin.collections.List<kotlin.String>
	fun generateSet(value: kotlin.collections.List<kotlin.String>)
	val identityGet: moq.TokioTlsIdentity?
	fun identitySet(value: moq.TokioTlsIdentity?)
	val peersGet: moq.TokioTlsPeers?
	fun peersSet(value: moq.TokioTlsPeers?)
	val rootGet: kotlin.collections.List<moq.PathBuf>
	fun rootSet(value: kotlin.collections.List<moq.PathBuf>)
	fun load(): moq.TokioTlsServe
	fun deprecated(): moq.MoqTokioCliDeprecated
	fun loadRoots(): moq.RustlsRootCertStore
	fun serverConfig(alpn: kotlin.collections.List<moq.Bytes>): moq.RustlsServerConfig
	companion object {
		fun default(): moq.TokioTlsListen
	}
}
class TokioTlsServe internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsServe
	fun certificates(): moq.TokioTlsCertificates
	fun config(alpn: kotlin.collections.List<moq.Bytes>): moq.RustlsServerConfig
}
class TokioTlsPeerIdentity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsPeerIdentity
	fun chain(): kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>
	fun fingerprint(): kotlin.String?
	fun name(): kotlin.String?
	fun issuer(): kotlin.String?
	fun expiry(): moq.ClockWall?
	companion object {
		fun fromChain(chain: kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>): moq.TokioTlsPeerIdentity
	}
}
class TokioTlsCertificates internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsCertificates
	fun fingerprints(): kotlin.collections.List<kotlin.String>
	companion object {
		fun fromPem(chain: moq.Bytes): moq.TokioTlsCertificates
	}
}
class TokioTlsKeyLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	companion object {
		fun open(path: moq.PathBuf): moq.TokioTlsKeyLog
	}
}
class TokioClientConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioClientConfig
	val connectGet: moq.TokioConnectConfig
	fun connectSet(value: moq.TokioConnectConfig)
	val quicGet: moq.TokioQuicConfig
	fun quicSet(value: moq.TokioQuicConfig)
	fun init(): moq.TokioClient
	companion object {
		fun default(): moq.TokioClientConfig
	}
}
class TokioConnectConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioConnectConfig
	val urlGet: moq.Url?
	fun urlSet(value: moq.Url?)
	val tokensGet: kotlin.collections.List<moq.AuthToken>
	fun tokensSet(value: kotlin.collections.List<moq.AuthToken>)
	val bindGet: moq.SocketAddr?
	fun bindSet(value: moq.SocketAddr?)
	val raceGet: moq.TokioDuration?
	fun raceSet(value: moq.TokioDuration?)
	val resolutionDelayGet: moq.TokioDuration?
	fun resolutionDelaySet(value: moq.TokioDuration?)
	val timeoutGet: moq.TokioDuration?
	fun timeoutSet(value: moq.TokioDuration?)
	val versionGet: kotlin.collections.List<moq.Version>
	fun versionSet(value: kotlin.collections.List<moq.Version>)
	val extensionsGet: moq.SetupExtensions
	fun extensionsSet(value: moq.SetupExtensions)
	val implementationGet: kotlin.String?
	fun implementationSet(value: kotlin.String?)
	val tlsGet: moq.TokioTlsConnect
	fun tlsSet(value: moq.TokioTlsConnect)
	val backoffGet: moq.TokioBackoff
	fun backoffSet(value: moq.TokioBackoff)
	val goawayGet: moq.MoqTokioConnectionGoaway
	fun goawaySet(value: moq.MoqTokioConnectionGoaway)
	val websocketGet: moq.MoqTokioWebsocketConfig
	fun websocketSet(value: moq.MoqTokioWebsocketConfig)
	fun deprecated(): moq.MoqTokioCliDeprecated
	fun init(quic: moq.TokioQuicConfig): moq.TokioClient
	fun versions(): moq.Versions
	fun resolve(): moq.MoqTokioConnectResolved
	companion object {
		fun default(): moq.TokioConnectConfig
	}
}
class TokioConnectionMonitor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioConnectionMonitor
	fun state(): moq.MoqTokioConnectionObservation
	suspend fun stateChanged(after: kotlin.ULong): moq.MoqTokioConnectionObservation
	fun presence(): moq.StatsPresence
	suspend fun pollPresence(): moq.StatsPresence
	suspend fun presenceChanged(): moq.StatsPresence
	fun stats(): moq.SessionStats?
	fun failure(): moq.TokioError?
	fun snapshot(): moq.MoqTokioConnectionSnapshot?
}
class TokioListenConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioListenConfig
	val bindGet: moq.TokioListenBind?
	fun bindSet(value: moq.TokioListenBind?)
	val tcpGet: moq.MoqTokioTcpConfig
	fun tcpSet(value: moq.MoqTokioTcpConfig)
	val unixGet: moq.MoqTokioUnixConfig
	fun unixSet(value: moq.MoqTokioUnixConfig)
	val versionGet: kotlin.collections.List<moq.Version>
	fun versionSet(value: kotlin.collections.List<moq.Version>)
	val extensionsGet: moq.SetupExtensions
	fun extensionsSet(value: moq.SetupExtensions)
	val implementationGet: kotlin.String?
	fun implementationSet(value: kotlin.String?)
	val timeoutGet: moq.TokioDuration?
	fun timeoutSet(value: moq.TokioDuration?)
	val tlsGet: moq.TokioTlsListen
	fun tlsSet(value: moq.TokioTlsListen)
	val preferredV4Get: moq.SocketAddrV4?
	fun preferredV4Set(value: moq.SocketAddrV4?)
	val preferredV6Get: moq.SocketAddrV6?
	fun preferredV6Set(value: moq.SocketAddrV6?)
	val lbGet: moq.MoqTokioListenLoadBalancer
	fun lbSet(value: moq.MoqTokioListenLoadBalancer)
	val resetGet: moq.MoqTokioListenReset
	fun resetSet(value: moq.MoqTokioListenReset)
	fun deprecated(): moq.MoqTokioCliDeprecated
	fun resolvedTimeout(): moq.Duration?
	fun resolve(): moq.MoqTokioListenResolved
	fun init(quic: moq.TokioQuicConfig): moq.TokioServer
	fun initStreams(): moq.TokioServer
	fun versions(): moq.Versions
	fun hasExplicitBind(): kotlin.Boolean
	companion object {
		fun default(): moq.TokioListenConfig
	}
}
class TokioQuicConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioQuicConfig
	val maxStreamsGet: kotlin.ULong?
	fun maxStreamsSet(value: kotlin.ULong?)
	val gsoGet: kotlin.Boolean?
	fun gsoSet(value: kotlin.Boolean?)
	val idleTimeoutGet: moq.TokioDuration?
	fun idleTimeoutSet(value: moq.TokioDuration?)
	val keepAliveGet: moq.TokioDuration?
	fun keepAliveSet(value: moq.TokioDuration?)
	val mtuDiscoveryGet: kotlin.Boolean?
	fun mtuDiscoverySet(value: kotlin.Boolean?)
	val congestionControlGet: moq.MoqTokioQuicCongestionControl?
	fun congestionControlSet(value: moq.MoqTokioQuicCongestionControl?)
	val controllerGet: moq.MoqTokioQuicController?
	fun controllerSet(value: moq.MoqTokioQuicController?)
	val receiveWindowGet: kotlin.ULong?
	fun receiveWindowSet(value: kotlin.ULong?)
	val streamReceiveWindowGet: kotlin.ULong?
	fun streamReceiveWindowSet(value: kotlin.ULong?)
	val sendWindowGet: kotlin.ULong?
	fun sendWindowSet(value: kotlin.ULong?)
	val qlogGet: moq.PathBuf?
	fun qlogSet(value: moq.PathBuf?)
	val keylogGet: moq.PathBuf?
	fun keylogSet(value: moq.PathBuf?)
	fun deprecated(): moq.MoqTokioCliDeprecated
	fun validate()
	fun resolve(): moq.MoqTokioQuicResolved
	companion object {
		fun default(): moq.TokioQuicConfig
	}
}
class TokioServerConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val tlsGet: moq.TokioTlsServe?
	fun tlsSet(value: moq.TokioTlsServe?)
	val activityGet: moq.ServerActivity
	fun activitySet(value: moq.ServerActivity)
	val listenGet: moq.TokioListenConfig
	fun listenSet(value: moq.TokioListenConfig)
	val quicGet: moq.TokioQuicConfig
	fun quicSet(value: moq.TokioQuicConfig)
	val websocketGet: moq.MoqTokioWebsocketListener?
	fun websocketSet(value: moq.MoqTokioWebsocketListener?)
	val publisherGet: moq.OriginConsumer?
	fun publisherSet(value: moq.OriginConsumer?)
	val subscriberGet: moq.OriginProducer?
	fun subscriberSet(value: moq.OriginProducer?)
	val statsGet: moq.StatsSession
	fun statsSet(value: moq.StatsSession)
	val demuxGet: moq.MoqTokioQuicDemuxDemux
	fun demuxSet(value: moq.MoqTokioQuicDemuxDemux)
	fun init(): moq.TokioServer
	fun initStreams(): moq.TokioServer
	companion object {
		fun default(): moq.TokioServerConfig
	}
}
class TokioServerListener internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun accept(): moq.TokioServerRequest
	suspend fun close_()
	fun localAddr(): moq.SocketAddr
	fun websocketLocalAddr(): moq.SocketAddr?
	fun tcpLocalAddr(): moq.SocketAddr?
	fun certificates(): moq.TokioTlsCertificates
	fun acceptHealth(): kotlin.collections.List<moq.MoqTokioAcceptHealth>
}
class TokioServerRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun reject(code: moq.SessionError)
	fun withPublisherProducer(publish: moq.OriginProducer): moq.TokioServerRequest
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.TokioServerRequest
	fun withSubscriber(subscribe: moq.OriginProducer): moq.TokioServerRequest
	fun withPeerHop(hop: moq.Hop): moq.TokioServerRequest
	fun withStats(stats: moq.StatsSession): moq.TokioServerRequest
	fun withLimits(limits: moq.SessionLimits): moq.TokioServerRequest
	fun quic(): moq.MoqTokioQuicDriverConnectionConnection?
	fun withQuicLimits(limits: moq.MoqQuicLimits): moq.TokioServerRequest
	suspend fun ok(): moq.Session
	fun transport(): moq.TokioTransport
	fun url(): moq.Url?
	fun authority(): kotlin.String?
	fun path(): kotlin.String
	fun query(): kotlin.String?
	fun remoteAddr(): moq.SocketAddr?
	fun localAddr(): moq.SocketAddr?
	fun serverName(): kotlin.String?
	fun alpn(): kotlin.String?
	fun role(): moq.SetupRole?
	fun auth(): moq.AuthHandle
	fun requests(): moq.AuthRequests
	fun peerHop(): moq.Hop?
	fun peerImplementation(): kotlin.String?
	fun token(): moq.SetupToken?
	fun peerIdentity(): moq.TokioTlsPeerIdentity?
}
class TransportMemoryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TransportMemoryConfig
	val streamBufferGet: kotlin.ULong
	fun streamBufferSet(value: kotlin.ULong)
	val protocolGet: kotlin.String?
	fun protocolSet(value: kotlin.String?)
	val maxConcurrentUniStreamsGet: kotlin.ULong?
	fun maxConcurrentUniStreamsSet(value: kotlin.ULong?)
	val maxConcurrentBiStreamsGet: kotlin.ULong?
	fun maxConcurrentBiStreamsSet(value: kotlin.ULong?)
	companion object {
		fun default(): moq.TransportMemoryConfig
	}
}
class TransportMemorySession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TransportMemorySession
}
class MuxCatalogHangConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalog?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogHangConsumer
	}
}
class MuxCatalogHangConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExtra?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogHangConsumerExtra
	}
}
class MuxCatalogHangConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExt?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogHangConsumerExt
	}
}
class MuxCatalogHangContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		fun new(container: moq.HangCatalogContainer, kind: moq.MuxContainerKind): moq.MuxCatalogHangContainer
	}
}
class MuxCatalogHangExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCatalogHangExtra
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun get(name: kotlin.String): moq.Json?
	fun iter(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.Json>>
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	fun set(name: kotlin.String, value: moq.Json)
	fun remove(name: kotlin.String): moq.Json?
	companion object {
		fun default(): moq.MuxCatalogHangExtra
	}
}
class MuxCatalogMsfConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalog?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumer
	}
}
class MuxCatalogMsfConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExtra?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumerExtra
	}
}
class MuxCatalogMsfConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.HangCatalogExt?
	companion object {
		fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumerExt
	}
}
class MuxCodecAacError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecAacError
}
class MuxCodecAacImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun seek(sequence: kotlin.ULong)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
	}
}
class MuxCodecAnnexbError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecAnnexbError
}
class MuxCodecAv1Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(buf: moq.Bytes)
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun seek(sequence: kotlin.ULong)
	fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
	}
}
class MuxCodecAv1Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		fun default(): moq.MuxCodecAv1Split
		fun new(): moq.MuxCodecAv1Split
	}
}
class MuxCodecFlacImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun seek(sequence: kotlin.ULong)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
	}
}
class MuxCodecH264Avc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun avcc(): moq.Bytes?
	fun transform(payload: moq.Bytes): moq.Bytes?
	companion object {
		fun default(): moq.MuxCodecH264Avc1
		fun new(): moq.MuxCodecH264Avc1
	}
}
class MuxCodecH264Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecH264Error
}
class MuxCodecH264Avcc internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecH264Avcc
	val profileGet: kotlin.UByte
	fun profileSet(value: kotlin.UByte)
	val constraintsGet: kotlin.UByte
	fun constraintsSet(value: kotlin.UByte)
	val levelGet: kotlin.UByte
	fun levelSet(value: kotlin.UByte)
	val lengthSizeGet: kotlin.ULong
	fun lengthSizeSet(value: kotlin.ULong)
	val spsGet: kotlin.collections.List<moq.Bytes>
	fun spsSet(value: kotlin.collections.List<moq.Bytes>)
	val ppsGet: kotlin.collections.List<moq.Bytes>
	fun ppsSet(value: kotlin.collections.List<moq.Bytes>)
	val codedWidthGet: kotlin.UInt?
	fun codedWidthSet(value: kotlin.UInt?)
	val codedHeightGet: kotlin.UInt?
	fun codedHeightSet(value: kotlin.UInt?)
	companion object {
		fun parse(avcc: moq.Bytes): moq.MuxCodecH264Avcc
	}
}
class MuxCodecH264Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(buf: moq.Bytes)
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun track(): moq.TrackProducer
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun seek(sequence: kotlin.ULong)
	fun observeReorder(reorder: moq.Timestamp)
	fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
	}
}
class MuxCodecH264Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		fun default(): moq.MuxCodecH264Split
		fun new(): moq.MuxCodecH264Split
	}
}
class MuxCodecH265Hvc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun hvcc(): moq.Bytes?
	fun transform(payload: moq.Bytes): moq.Bytes?
	companion object {
		fun default(): moq.MuxCodecH265Hvc1
		fun new(): moq.MuxCodecH265Hvc1
	}
}
class MuxCodecH265Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecH265Error
}
class MuxCodecH265Hvcc internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecH265Hvcc
	val lengthSizeGet: kotlin.ULong
	fun lengthSizeSet(value: kotlin.ULong)
	val vpsGet: kotlin.collections.List<moq.Bytes>
	fun vpsSet(value: kotlin.collections.List<moq.Bytes>)
	val spsGet: kotlin.collections.List<moq.Bytes>
	fun spsSet(value: kotlin.collections.List<moq.Bytes>)
	val ppsGet: kotlin.collections.List<moq.Bytes>
	fun ppsSet(value: kotlin.collections.List<moq.Bytes>)
	companion object {
		fun parse(hvcc: moq.Bytes): moq.MuxCodecH265Hvcc
	}
}
class MuxCodecH265Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(buf: moq.Bytes)
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun track(): moq.TrackProducer
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	fun seek(sequence: kotlin.ULong)
	fun observeReorder(reorder: moq.Timestamp)
	fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
	}
}
class MuxCodecH265Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		fun default(): moq.MuxCodecH265Split
		fun new(): moq.MuxCodecH265Split
	}
}
class MuxCodecMp3Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun seek(sequence: kotlin.ULong)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
	}
}
class MuxCodecOpusConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecOpusConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val sampleRateGet: kotlin.UInt
	fun sampleRateSet(value: kotlin.UInt)
	val channelCountGet: kotlin.UInt
	fun channelCountSet(value: kotlin.UInt)
	val preSkipGet: kotlin.UShort
	fun preSkipSet(value: kotlin.UShort)
	val outputGainGet: kotlin.Short
	fun outputGainSet(value: kotlin.Short)
	val mappingGet: moq.MuxCodecOpusMapping?
	fun mappingSet(value: moq.MuxCodecOpusMapping?)
	fun withPreSkip(preSkip: kotlin.UShort): moq.MuxCodecOpusConfig
	fun encode(): moq.Bytes
	companion object {
		fun new(sampleRate: kotlin.UInt, channelCount: kotlin.UInt): moq.MuxCodecOpusConfig
		fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecOpusConfig
	}
}
class MuxCodecOpusImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun seek(sequence: kotlin.ULong)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
	}
}
class MuxCodecVp8Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(buf: moq.Bytes)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun seek(sequence: kotlin.ULong)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
	}
}
class MuxCodecVp9Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun initialize(buf: moq.Bytes)
	fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	fun demand(): moq.TrackDemand
	fun finish()
	fun abort(err: moq.Error)
	fun cut(end: moq.Timestamp?)
	fun discontinuity()
	fun setEnabled(enabled: kotlin.Boolean)
	fun seek(sequence: kotlin.ULong)
	companion object {
		fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
		fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
		fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
	}
}
class MuxContainerFlvImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImport
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerFlvImport
	}
}
class MuxContainerFlvImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImportExtra
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerFlvImportExtra
	}
}
class MuxContainerFlvImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImportExt
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerFlvImportExt
	}
}
class MuxContainerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun trak(): moq.Mp4AtomTrak
	companion object {
		fun new(trak: moq.Mp4AtomTrak): moq.MuxContainerFmp4Wire
		fun fromInit(initData: moq.Bytes): moq.MuxContainerFmp4Wire
	}
}
class MuxContainerFmp4Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerFmp4Error
}
class MuxContainerFmp4Fragmenter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun push(frame: moq.MuxContainerFrame): kotlin.collections.List<moq.MuxContainerFmp4Fragment>
	fun pushGroup(frame: moq.MuxContainerFrame): kotlin.collections.List<moq.MuxContainerFmp4Fragment>
	fun flush(): moq.MuxContainerFmp4Fragment?
}
class MuxContainerFmp4Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4Import
	fun withPassthrough(passthrough: kotlin.Boolean): moq.MuxContainerFmp4Import
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerFmp4Import
	}
}
class MuxContainerFmp4ImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4ImportExtra
	fun withPassthrough(passthrough: kotlin.Boolean): moq.MuxContainerFmp4ImportExtra
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerFmp4ImportExtra
	}
}
class MuxContainerFmp4ImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4ImportExt
	fun withPassthrough(passthrough: kotlin.Boolean): moq.MuxContainerFmp4ImportExt
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	fun seek(sequence: kotlin.ULong)
	fun discontinuity()
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerFmp4ImportExt
	}
}
class MuxContainerFmp4Muxer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun timescale(): moq.Timescale
	fun withTimescale(timescale: moq.Timescale): moq.MuxContainerFmp4Muxer
	fun withFramerate(framerate: kotlin.Double?): moq.MuxContainerFmp4Muxer
	suspend fun read(group: moq.GroupConsumer): kotlin.collections.List<moq.MuxContainerFrame>
	fun init(): moq.Bytes?
	fun fragment(sequence: kotlin.UInt, frames: kotlin.collections.List<moq.MuxContainerFrame>, end: moq.Timestamp?): moq.Bytes
	fun fragmenter(config: moq.MuxContainerFmp4FragmentConfig): moq.MuxContainerFmp4Fragmenter
	companion object {
		fun video(config: moq.HangCatalogVideoConfig): moq.MuxContainerFmp4Muxer
		fun audio(config: moq.HangCatalogAudioConfig): moq.MuxContainerFmp4Muxer
	}
}
class MuxContainerMkvImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImport
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerMkvImport
	}
}
class MuxContainerMkvImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImportExtra
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerMkvImportExtra
	}
}
class MuxContainerMkvImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImportExt
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerMkvImportExt
	}
}
class MuxContainerTsExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsExt
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mpegtsGet: moq.MuxContainerTsMpegts
	fun mpegtsSet(value: moq.MuxContainerTsMpegts)
	companion object {
		fun default(): moq.MuxContainerTsExt
	}
}
class MuxContainerTsMpegts internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsMpegts
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val tracksGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.MuxContainerTsTrack>>
	fun tracksSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.MuxContainerTsTrack>>)
	val programDescriptorsGet: kotlin.collections.List<moq.MuxContainerTsDescriptor>
	fun programDescriptorsSet(value: kotlin.collections.List<moq.MuxContainerTsDescriptor>)
	val programGet: moq.MuxContainerTsProgram?
	fun programSet(value: moq.MuxContainerTsProgram?)
	val siGet: kotlin.collections.List<kotlin.Pair<kotlin.UShort, kotlin.collections.List<kotlin.Pair<kotlin.UByte, moq.MuxContainerTsSiEntry>>>>
	fun siSet(value: kotlin.collections.List<kotlin.Pair<kotlin.UShort, kotlin.collections.List<kotlin.Pair<kotlin.UByte, moq.MuxContainerTsSiEntry>>>>)
	val muxRateGet: kotlin.ULong?
	fun muxRateSet(value: kotlin.ULong?)
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.MuxContainerTsMpegts
	}
}
class MuxContainerTsPesPacket internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsPesPacket
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val streamTypeGet: kotlin.UByte
	fun streamTypeSet(value: kotlin.UByte)
	companion object {
		fun default(): moq.MuxContainerTsPesPacket
		fun new(streamType: kotlin.UByte): moq.MuxContainerTsPesPacket
	}
}
class MuxContainerTsProgram internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsProgram
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val transportStreamIdGet: kotlin.UShort
	fun transportStreamIdSet(value: kotlin.UShort)
	val programNumberGet: kotlin.UShort
	fun programNumberSet(value: kotlin.UShort)
	val pmtPidGet: kotlin.UShort
	fun pmtPidSet(value: kotlin.UShort)
	companion object {
		fun default(): moq.MuxContainerTsProgram
	}
}
class MuxContainerTsSiEntry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsSiEntry
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val trackGet: kotlin.String
	fun trackSet(value: kotlin.String)
	val intervalGet: moq.Duration?
	fun intervalSet(value: moq.Duration?)
	companion object {
		fun default(): moq.MuxContainerTsSiEntry
	}
}
class MuxContainerTsTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsTrack
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val pidGet: kotlin.UShort
	fun pidSet(value: kotlin.UShort)
	val descriptorsGet: kotlin.collections.List<moq.MuxContainerTsDescriptor>
	fun descriptorsSet(value: kotlin.collections.List<moq.MuxContainerTsDescriptor>)
	val verbatimGet: moq.MuxContainerTsVerbatim?
	fun verbatimSet(value: moq.MuxContainerTsVerbatim?)
	val pesPacketGet: moq.MuxContainerTsPesPacket?
	fun pesPacketSet(value: moq.MuxContainerTsPesPacket?)
	companion object {
		fun default(): moq.MuxContainerTsTrack
		fun new(pid: kotlin.UShort): moq.MuxContainerTsTrack
	}
}
class MuxContainerTsVerbatim internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsVerbatim
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val streamTypeGet: kotlin.UByte
	fun streamTypeSet(value: kotlin.UByte)
	val framingGet: moq.MuxContainerTsFraming
	fun framingSet(value: moq.MuxContainerTsFraming)
	companion object {
		fun default(): moq.MuxContainerTsVerbatim
		fun new(streamType: kotlin.UByte, framing: moq.MuxContainerTsFraming): moq.MuxContainerTsVerbatim
	}
}
class MuxContainerTsExportConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumerExtra
	fun withDelay(delay: moq.Duration): moq.MuxContainerTsExportConsumerExtra
	suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumerExtra): moq.MuxContainerTsExportConsumerExtra
	companion object {
		fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumerExtra): moq.MuxContainerTsExportConsumerExtra
	}
}
class MuxContainerTsExportConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumerExt
	fun withDelay(delay: moq.Duration): moq.MuxContainerTsExportConsumerExt
	suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumerExt): moq.MuxContainerTsExportConsumerExt
	companion object {
		fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumerExt): moq.MuxContainerTsExportConsumerExt
	}
}
class MuxContainerTsExportConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumer
	fun withDelay(delay: moq.Duration): moq.MuxContainerTsExportConsumer
	suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumer): moq.MuxContainerTsExportConsumer
	companion object {
		fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumer): moq.MuxContainerTsExportConsumer
	}
}
class MuxContainerTsFollower internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollower
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollower
	fun export(): moq.MuxContainerTsExportConsumer
	suspend fun next(): moq.MuxContainerFrame?
	companion object {
		fun new(export: moq.MuxContainerTsExportConsumer): moq.MuxContainerTsFollower
	}
}
class MuxContainerTsFollowerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollowerExtra
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollowerExtra
	fun export(): moq.MuxContainerTsExportConsumerExtra
	suspend fun next(): moq.MuxContainerFrame?
	companion object {
		fun new(export: moq.MuxContainerTsExportConsumerExtra): moq.MuxContainerTsFollowerExtra
	}
}
class MuxContainerTsFollowerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollowerExt
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollowerExt
	fun export(): moq.MuxContainerTsExportConsumerExt
	suspend fun next(): moq.MuxContainerFrame?
	companion object {
		fun new(export: moq.MuxContainerTsExportConsumerExt): moq.MuxContainerTsFollowerExt
	}
}
class MuxContainerTsPassthrough internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthrough
	fun track(): kotlin.String
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerTsPassthrough
	}
}
class MuxContainerTsPassthroughExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthroughExtra
	fun track(): kotlin.String
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerTsPassthroughExtra
	}
}
class MuxContainerTsPassthroughExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthroughExt
	fun track(): kotlin.String
	fun decode(data: moq.Bytes)
	fun finish()
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerTsPassthroughExt
	}
}
class MuxContainerTsPrograms internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun decode(data: moq.Bytes)
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun finish(): kotlin.collections.List<moq.BroadcastProducer>
	fun abort(err: moq.Error)
	companion object {
		fun new(origin: moq.OriginProducer, name: moq.Path, config: moq.MuxCatalogConfig): moq.MuxContainerTsPrograms
	}
}
class MuxContainerTsRelease internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withDelay(delay: moq.Duration): moq.MuxContainerTsRelease
	fun dropped(): kotlin.ULong
	fun drift(): kotlin.Double?
	fun outOfTolerance(): kotlin.ULong
	fun stats(): moq.MuxContainerTsStatsRelease
	suspend fun next(): moq.MuxContainerFrame?
	companion object {
		suspend fun new(source: moq.MuxSource, catalogFormat: moq.MuxCatalogCatalogFormat): moq.MuxContainerTsRelease
	}
}
class MuxContainerTsImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImport
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImport
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerTsImport
	}
}
class MuxContainerTsImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImportExtra
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImportExtra
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerTsImportExtra
	}
}
class MuxContainerTsImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImportExt
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImportExt
	fun decode(data: moq.Bytes)
	fun seek(sequence: kotlin.ULong)
	fun finish()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerTsImportExt
	}
}
class MuxCodecAacAdtsHeader internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecAacAdtsHeader
	val objectTypeGet: kotlin.UByte
	fun objectTypeSet(value: kotlin.UByte)
	val sampleRateGet: kotlin.UInt
	fun sampleRateSet(value: kotlin.UInt)
	val channelConfigGet: kotlin.UByte
	fun channelConfigSet(value: kotlin.UByte)
	val blocksGet: kotlin.UByte
	fun blocksSet(value: kotlin.UByte)
	fun payload(packet: moq.Bytes): moq.Bytes
	companion object {
		fun parse(data: moq.Bytes): moq.MuxCodecAacAdtsHeader
	}
}
class MuxContainerTsStatsReport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsStatsReport
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mpegtsGet: moq.MuxContainerTsStatsSnapshot?
	fun mpegtsSet(value: moq.MuxContainerTsStatsSnapshot?)
	val releaseGet: moq.MuxContainerTsStatsRelease?
	fun releaseSet(value: moq.MuxContainerTsStatsRelease?)
	companion object {
		fun default(): moq.MuxContainerTsStatsReport
	}
}
class MuxContainerTsStatsSnapshot internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsStatsSnapshot
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val streamsGet: kotlin.collections.List<kotlin.Pair<kotlin.UShort, moq.MuxContainerTsStatsStream>>
	fun streamsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.UShort, moq.MuxContainerTsStatsStream>>)
	val crcErrorGet: kotlin.ULong
	fun crcErrorSet(value: kotlin.ULong)
	val tsSyncLossGet: kotlin.ULong
	fun tsSyncLossSet(value: kotlin.ULong)
	val syncByteErrorGet: kotlin.ULong
	fun syncByteErrorSet(value: kotlin.ULong)
	val patErrorGet: kotlin.ULong
	fun patErrorSet(value: kotlin.ULong)
	val continuityCountErrorGet: kotlin.ULong
	fun continuityCountErrorSet(value: kotlin.ULong)
	val pmtErrorGet: kotlin.ULong
	fun pmtErrorSet(value: kotlin.ULong)
	val transportErrorGet: kotlin.ULong
	fun transportErrorSet(value: kotlin.ULong)
	val pcrRepetitionErrorGet: kotlin.ULong
	fun pcrRepetitionErrorSet(value: kotlin.ULong)
	val pcrDiscontinuityIndicatorErrorGet: kotlin.ULong
	fun pcrDiscontinuityIndicatorErrorSet(value: kotlin.ULong)
	val ptsErrorGet: kotlin.ULong
	fun ptsErrorSet(value: kotlin.ULong)
	fun isEmpty(): kotlin.Boolean
	companion object {
		fun default(): moq.MuxContainerTsStatsSnapshot
	}
}
class MuxContainerTsStatsStream internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsStatsStream
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val trackGet: kotlin.String
	fun trackSet(value: kotlin.String)
	val classGet: moq.MuxContainerTsStatsClass
	fun classSet(value: moq.MuxContainerTsStatsClass)
	val unitsGet: kotlin.ULong
	fun unitsSet(value: kotlin.ULong)
	val quietGet: moq.Duration?
	fun quietSet(value: moq.Duration?)
	val resyncsGet: kotlin.ULong
	fun resyncsSet(value: kotlin.ULong)
	val discardedGet: kotlin.ULong
	fun discardedSet(value: kotlin.ULong)
	val unconfirmedGet: kotlin.ULong
	fun unconfirmedSet(value: kotlin.ULong)
	val damagedGet: kotlin.ULong
	fun damagedSet(value: kotlin.ULong)
	val unplacedGet: kotlin.ULong
	fun unplacedSet(value: kotlin.ULong)
	val refusedGet: kotlin.ULong
	fun refusedSet(value: kotlin.ULong)
	val continuityCountErrorGet: kotlin.ULong
	fun continuityCountErrorSet(value: kotlin.ULong)
	val transportErrorGet: kotlin.ULong
	fun transportErrorSet(value: kotlin.ULong)
	val ptsErrorGet: kotlin.ULong
	fun ptsErrorSet(value: kotlin.ULong)
	companion object {
		fun default(): moq.MuxContainerTsStatsStream
	}
}
class MuxContainerTsStatsExport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsStatsExport
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val streamsGet: kotlin.collections.List<kotlin.Pair<kotlin.UShort, moq.MuxContainerTsStatsStream>>
	fun streamsSet(value: kotlin.collections.List<kotlin.Pair<kotlin.UShort, moq.MuxContainerTsStatsStream>>)
	val releaseGet: moq.MuxContainerTsStatsRelease
	fun releaseSet(value: moq.MuxContainerTsStatsRelease)
	companion object {
		fun default(): moq.MuxContainerTsStatsExport
	}
}
class MuxContainerTsStatsRelease internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsStatsRelease
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val droppedGet: kotlin.ULong
	fun droppedSet(value: kotlin.ULong)
	val trimmedGet: kotlin.ULong
	fun trimmedSet(value: kotlin.ULong)
	val gapsGet: kotlin.ULong
	fun gapsSet(value: kotlin.ULong)
	val marginGet: kotlin.Long?
	fun marginSet(value: kotlin.Long?)
	val offsetGet: kotlin.Long?
	fun offsetSet(value: kotlin.Long?)
	val driftGet: kotlin.Double?
	fun driftSet(value: kotlin.Double?)
	val outOfToleranceGet: kotlin.ULong
	fun outOfToleranceSet(value: kotlin.ULong)
	companion object {
		fun default(): moq.MuxContainerTsStatsRelease
	}
}
class MuxContainerTsStatsLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sample(latest: moq.MuxContainerTsStatsSnapshot)
	fun finish(latest: moq.MuxContainerTsStatsSnapshot)
	companion object {
		fun default(): moq.MuxContainerTsStatsLog
		val INTERVAL: moq.Duration
	}
}
class MoqTokioAddrs internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioAddrs
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun or(addr: moq.MoqTokioConnectAddr): moq.MoqTokioAddrs
	fun asSlice(): kotlin.collections.List<moq.MoqTokioConnectAddr>
	companion object {
		fun new(addr: moq.MoqTokioConnectAddr): moq.MoqTokioAddrs
		fun collect(urls: kotlin.collections.List<moq.MoqTokioConnectAddr>): moq.MoqTokioAddrs?
	}
}
class MoqTokioCliDeprecated internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioCliDeprecated
	fun isEmpty(): kotlin.Boolean
	fun extend(other: moq.MoqTokioCliDeprecated)
	fun flag(old: kotlin.String, env: kotlin.String?, new: kotlin.String)
	fun changed(old: kotlin.String, env: kotlin.String?, new: kotlin.String, note: kotlin.String)
	fun removed(old: kotlin.String, env: kotlin.String?, note: kotlin.String)
	fun toml(old: kotlin.String, new: kotlin.String, note: kotlin.String?)
	companion object {
		fun default(): moq.MoqTokioCliDeprecated
	}
}
class Mp4AtomCodec internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomCodec
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun visual(): moq.Mp4AtomVisual?
	fun audio(): moq.Mp4AtomAudio?
	fun metadata(): moq.Mp4AtomMetaData?
}
class MoqTokioTcpTlsAcceptor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MoqTokioTcpTlsAcceptor
	fun config(): moq.RustlsServerConfig
	fun offered(alpns: kotlin.collections.List<moq.Bytes>): kotlin.Boolean
}
class MoqTokioAcceptHealth internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioAcceptHealth
	fun listener(): kotlin.String
	fun accepted()
	fun failures(failure: moq.MoqTokioAcceptFailure): kotlin.ULong
	fun stalled(): moq.Duration?
	companion object {
		fun new(listener: kotlin.String): moq.MoqTokioAcceptHealth
	}
}
class RustlsClientConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsClientConfig
	val alpnProtocolsGet: kotlin.collections.List<moq.Bytes>
	fun alpnProtocolsSet(value: kotlin.collections.List<moq.Bytes>)
	val checkSelectedAlpnGet: kotlin.Boolean
	fun checkSelectedAlpnSet(value: kotlin.Boolean)
	val resumptionGet: moq.RustlsClientResumption
	fun resumptionSet(value: moq.RustlsClientResumption)
	val maxFragmentSizeGet: kotlin.ULong?
	fun maxFragmentSizeSet(value: kotlin.ULong?)
	val clientAuthCertResolverGet: moq.RustlsResolvesClientCert
	fun clientAuthCertResolverSet(value: moq.RustlsResolvesClientCert)
	val enableSniGet: kotlin.Boolean
	fun enableSniSet(value: kotlin.Boolean)
	val keyLogGet: moq.RustlsKeyLog
	fun keyLogSet(value: moq.RustlsKeyLog)
	val enableSecretExtractionGet: kotlin.Boolean
	fun enableSecretExtractionSet(value: kotlin.Boolean)
	val enableEarlyDataGet: kotlin.Boolean
	fun enableEarlyDataSet(value: kotlin.Boolean)
	val requireEmsGet: kotlin.Boolean
	fun requireEmsSet(value: kotlin.Boolean)
	val timeProviderGet: moq.RustlsTimeProvider
	fun timeProviderSet(value: moq.RustlsTimeProvider)
	val certDecompressorsGet: kotlin.collections.List<moq.RustlsCertDecompressor>
	fun certDecompressorsSet(value: kotlin.collections.List<moq.RustlsCertDecompressor>)
	val certCompressorsGet: kotlin.collections.List<moq.RustlsCertCompressor>
	fun certCompressorsSet(value: kotlin.collections.List<moq.RustlsCertCompressor>)
	val certCompressionCacheGet: moq.RustlsCompressionCache
	fun certCompressionCacheSet(value: moq.RustlsCompressionCache)
	val sendTicketRequestGet: moq.RustlsClientTicketRequest?
	fun sendTicketRequestSet(value: moq.RustlsClientTicketRequest?)
	fun fips(): kotlin.Boolean
	fun cryptoProvider(): moq.RustlsCryptoProvider
	fun dangerous(): moq.RustlsClientDangerDangerousClientConfig
}
class RustlsRootCertStore internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsRootCertStore
	val rootsGet: kotlin.collections.List<moq.RustlsPkiTypesTrustAnchor>
	fun rootsSet(value: kotlin.collections.List<moq.RustlsPkiTypesTrustAnchor>)
	fun addParsableCertificates(derCerts: kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>): kotlin.Pair<kotlin.ULong, kotlin.ULong>
	fun add(der: moq.RustlsPkiTypesCertificateDer)
	fun subjects(): kotlin.collections.List<moq.RustlsDistinguishedName>
	fun isEmpty(): kotlin.Boolean
	fun len(): kotlin.ULong
	companion object {
		fun empty(): moq.RustlsRootCertStore
	}
}
class RustlsPkiTypesCertificateDer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsPkiTypesCertificateDer
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun intoOwned(): moq.RustlsPkiTypesCertificateDer
	companion object {
		fun fromSlice(bytes: moq.Bytes): moq.RustlsPkiTypesCertificateDer
	}
}
class MoqTokioConnectResolved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectResolved
	val bindGet: moq.SocketAddr
	fun bindSet(value: moq.SocketAddr)
	val raceGet: moq.Duration
	fun raceSet(value: moq.Duration)
	val resolutionDelayGet: moq.Duration
	fun resolutionDelaySet(value: moq.Duration)
	val timeoutGet: moq.Duration
	fun timeoutSet(value: moq.Duration)
}
class MoqTokioConnectionGoaway internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectionGoaway
	val redirectGet: moq.MoqTokioRedirect
	fun redirectSet(value: moq.MoqTokioRedirect)
	val handoverGet: moq.TokioDuration?
	fun handoverSet(value: moq.TokioDuration?)
	companion object {
		fun default(): moq.MoqTokioConnectionGoaway
	}
}
class MoqTokioWebsocketConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioWebsocketConfig
	val enabledGet: kotlin.Boolean?
	fun enabledSet(value: kotlin.Boolean?)
	val delayGet: moq.TokioDuration?
	fun delaySet(value: moq.TokioDuration?)
	fun resolve(): moq.MoqTokioWebsocketResolved
	companion object {
		fun default(): moq.MoqTokioWebsocketConfig
	}
}
class MoqTokioConnectionObservation internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectionObservation
	val revisionGet: kotlin.ULong
	fun revisionSet(value: kotlin.ULong)
	val phaseGet: moq.MoqTokioConnectionPhase
	fun phaseSet(value: moq.MoqTokioConnectionPhase)
	val connectsGet: kotlin.ULong
	fun connectsSet(value: kotlin.ULong)
	val lastConnectedGet: moq.ClockInstant?
	fun lastConnectedSet(value: moq.ClockInstant?)
	val transportGet: moq.TokioTransport?
	fun transportSet(value: moq.TokioTransport?)
	val failureGet: moq.TokioError?
	fun failureSet(value: moq.TokioError?)
	val versionGet: moq.Version?
	fun versionSet(value: moq.Version?)
}
class MoqTokioConnectionSnapshot internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val statsGet: moq.SessionStats
	fun statsSet(value: moq.SessionStats)
	val versionGet: moq.Version
	fun versionSet(value: moq.Version)
}
class MoqTokioListenResolved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioListenResolved
	val bindGet: moq.TokioListenBind?
	fun bindSet(value: moq.TokioListenBind?)
	val versionsGet: moq.Versions
	fun versionsSet(value: moq.Versions)
	val extensionsGet: moq.SetupExtensions
	fun extensionsSet(value: moq.SetupExtensions)
	val implementationGet: kotlin.String?
	fun implementationSet(value: kotlin.String?)
	val timeoutGet: moq.Duration?
	fun timeoutSet(value: moq.Duration?)
	val tlsGet: moq.TokioTlsListen
	fun tlsSet(value: moq.TokioTlsListen)
	val preferredV4Get: moq.SocketAddrV4?
	fun preferredV4Set(value: moq.SocketAddrV4?)
	val preferredV6Get: moq.SocketAddrV6?
	fun preferredV6Set(value: moq.SocketAddrV6?)
	val lbGet: moq.MoqTokioListenLoadBalancer
	fun lbSet(value: moq.MoqTokioListenLoadBalancer)
	val resetSecretGet: moq.PathBuf?
	fun resetSecretSet(value: moq.PathBuf?)
}
class MoqTokioTcpConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioTcpConfig
	val bindGet: moq.SocketAddr?
	fun bindSet(value: moq.SocketAddr?)
	val tlsGet: kotlin.Boolean?
	fun tlsSet(value: kotlin.Boolean?)
	companion object {
		fun default(): moq.MoqTokioTcpConfig
	}
}
class MoqTokioUnixConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioUnixConfig
	val bindGet: moq.PathBuf?
	fun bindSet(value: moq.PathBuf?)
	val allowGet: moq.MoqTokioUnixAllow
	fun allowSet(value: moq.MoqTokioUnixAllow)
	companion object {
		fun default(): moq.MoqTokioUnixConfig
	}
}
class MoqTokioListenLoadBalancer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioListenLoadBalancer
	val idGet: moq.MoqTokioQuicServerId?
	fun idSet(value: moq.MoqTokioQuicServerId?)
	val nonceGet: kotlin.ULong?
	fun nonceSet(value: kotlin.ULong?)
	val forwardGet: moq.MoqTokioListenForward
	fun forwardSet(value: moq.MoqTokioListenForward)
	companion object {
		fun default(): moq.MoqTokioListenLoadBalancer
	}
}
class MoqTokioListenReset internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioListenReset
	val secretGet: moq.PathBuf?
	fun secretSet(value: moq.PathBuf?)
	companion object {
		fun default(): moq.MoqTokioListenReset
	}
}
class MoqTokioQuicResolved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicResolved
	val maxStreamsGet: kotlin.ULong
	fun maxStreamsSet(value: kotlin.ULong)
	val gsoGet: kotlin.Boolean?
	fun gsoSet(value: kotlin.Boolean?)
	val idleTimeoutGet: moq.Duration
	fun idleTimeoutSet(value: moq.Duration)
	val keepAliveGet: moq.Duration?
	fun keepAliveSet(value: moq.Duration?)
	val mtuDiscoveryGet: kotlin.Boolean
	fun mtuDiscoverySet(value: kotlin.Boolean)
	val congestionControlGet: moq.MoqTokioQuicCongestionControl?
	fun congestionControlSet(value: moq.MoqTokioQuicCongestionControl?)
	val controllerGet: moq.MoqTokioQuicController?
	fun controllerSet(value: moq.MoqTokioQuicController?)
	val receiveWindowGet: kotlin.ULong
	fun receiveWindowSet(value: kotlin.ULong)
	val streamReceiveWindowGet: kotlin.ULong
	fun streamReceiveWindowSet(value: kotlin.ULong)
	val sendWindowGet: kotlin.ULong
	fun sendWindowSet(value: kotlin.ULong)
	val qlogGet: moq.PathBuf?
	fun qlogSet(value: moq.PathBuf?)
	val keylogGet: moq.PathBuf?
	fun keylogSet(value: moq.PathBuf?)
	fun hasController(): kotlin.Boolean
	fun keepAliveInterval(): moq.Duration
	companion object {
		fun default(): moq.MoqTokioQuicResolved
	}
}
class MoqTokioQuicController internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicController
	val _0_get: moq.MoqCongestionFactory
	fun _0_set(value: moq.MoqCongestionFactory)
}
class MoqTokioWebsocketListener internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProtocols(protocols: kotlin.collections.List<kotlin.String>): moq.MoqTokioWebsocketListener
	fun withQuic(quic: moq.MoqTokioQuicResolved): moq.MoqTokioWebsocketListener
	fun localAddr(): moq.SocketAddr
	fun acceptHealth(): moq.MoqTokioAcceptHealth
	companion object {
		suspend fun bind(addr: moq.SocketAddr): moq.MoqTokioWebsocketListener
	}
}
class MoqTokioQuicDemuxDemux internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDemuxDemux
	fun counters(): moq.MoqSockDemuxCounters
	fun webrtc(): moq.MoqTokioQuicDemuxWebRtc
	companion object {
		fun default(): moq.MoqTokioQuicDemuxDemux
	}
}
class MoqTokioQuicDriverConnectionConnection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverConnectionConnection
	suspend fun openUni(options: moq.MoqTokioQuicDriverSendOptions): moq.MoqTokioQuicDriverSendStream
	fun sendGroup(priority: kotlin.Long): moq.MoqTokioQuicDriverSendGroup
	suspend fun openBi(options: moq.MoqTokioQuicDriverSendOptions): kotlin.Pair<moq.MoqTokioQuicDriverSendStream, moq.MoqTokioQuicDriverRecvStream>
	suspend fun acceptUni(): moq.MoqTokioQuicDriverRecvStream
	suspend fun acceptBi(): kotlin.Pair<moq.MoqTokioQuicDriverSendStream, moq.MoqTokioQuicDriverRecvStream>
	suspend fun readDatagram(): moq.Bytes
	suspend fun pollRecvDatagram(): moq.Bytes
	suspend fun closed(): moq.MoqQuicError
	fun closeReason(): moq.MoqQuicError?
	fun close_(errorCode: moq.MoqQuicVarInt, reason: kotlin.String)
	fun sendDatagram(data: moq.Bytes, options: moq.MoqTokioQuicDriverSendOptions)
	fun maxDatagramSize(): kotlin.ULong?
	fun datagramSendBufferSpace(): kotlin.ULong
	fun remoteAddress(): moq.SocketAddr
	fun rtt(): moq.Duration
	fun stats(): moq.MoqQuicConnectionStats
	fun handshakeData(): moq.MoqQuicCryptoHandshakeData?
	fun peerIdentity(): kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>?
	fun stableId(): kotlin.ULong
	fun setLimits(limits: moq.MoqQuicLimits)
}
class MoqQuicLimits internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqQuicLimits
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val maxStreamsGet: kotlin.ULong?
	fun maxStreamsSet(value: kotlin.ULong?)
	val receiveWindowGet: kotlin.ULong?
	fun receiveWindowSet(value: kotlin.ULong?)
	val streamReceiveWindowGet: kotlin.ULong?
	fun streamReceiveWindowSet(value: kotlin.ULong?)
	val sendWindowGet: kotlin.ULong?
	fun sendWindowSet(value: kotlin.ULong?)
	val receiveRateGet: moq.MoqQuicRate?
	fun receiveRateSet(value: moq.MoqQuicRate?)
	val sendRateGet: moq.MoqQuicRate?
	fun sendRateSet(value: moq.MoqQuicRate?)
	fun validate()
	companion object {
		fun default(): moq.MoqQuicLimits
	}
}
class Mp4AtomTrak internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomTrak
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val tkhdGet: moq.Mp4AtomTkhd
	fun tkhdSet(value: moq.Mp4AtomTkhd)
	val edtsGet: moq.Mp4AtomEdts?
	fun edtsSet(value: moq.Mp4AtomEdts?)
	val metaGet: moq.Mp4AtomMeta?
	fun metaSet(value: moq.Mp4AtomMeta?)
	val mdiaGet: moq.Mp4AtomMdia
	fun mdiaSet(value: moq.Mp4AtomMdia)
	val sencGet: moq.Mp4AtomSenc?
	fun sencSet(value: moq.Mp4AtomSenc?)
	val trefGet: moq.Mp4AtomTref?
	fun trefSet(value: moq.Mp4AtomTref?)
	val udtaGet: moq.Mp4AtomUdta?
	fun udtaSet(value: moq.Mp4AtomUdta?)
	companion object {
		fun default(): moq.Mp4AtomTrak
	}
}
class MoqTokioConnectAddr internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectAddr
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun url(): moq.Url
	fun addresses(): kotlin.collections.List<moq.SocketAddr>?
	companion object {
		fun new(url: moq.Url): moq.MoqTokioConnectAddr
		fun pinned(url: moq.Url, addrs: kotlin.collections.List<moq.SocketAddr>): moq.MoqTokioConnectAddr
	}
}
class Mp4AtomAvc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomAvc1
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val avccGet: moq.Mp4AtomAvcc
	fun avccSet(value: moq.Mp4AtomAvcc)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val taicGet: moq.Mp4AtomTaic?
	fun taicSet(value: moq.Mp4AtomTaic?)
	val fielGet: moq.Mp4AtomFiel?
	fun fielSet(value: moq.Mp4AtomFiel?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomAvc1
	}
}
class Mp4AtomAvc3 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomAvc3
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val avccGet: moq.Mp4AtomAvcc
	fun avccSet(value: moq.Mp4AtomAvcc)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val taicGet: moq.Mp4AtomTaic?
	fun taicSet(value: moq.Mp4AtomTaic?)
	val fielGet: moq.Mp4AtomFiel?
	fun fielSet(value: moq.Mp4AtomFiel?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomAvc3
	}
}
class Mp4AtomJpeg internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomJpeg
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val fielGet: moq.Mp4AtomFiel?
	fun fielSet(value: moq.Mp4AtomFiel?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomJpeg
	}
}
class Mp4AtomHev1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomHev1
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val hvccGet: moq.Mp4AtomHvcc
	fun hvccSet(value: moq.Mp4AtomHvcc)
	val lhvcGet: moq.Mp4AtomLhvc?
	fun lhvcSet(value: moq.Mp4AtomLhvc?)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val taicGet: moq.Mp4AtomTaic?
	fun taicSet(value: moq.Mp4AtomTaic?)
	val fielGet: moq.Mp4AtomFiel?
	fun fielSet(value: moq.Mp4AtomFiel?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomHev1
	}
}
class Mp4AtomHvc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomHvc1
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val hvccGet: moq.Mp4AtomHvcc
	fun hvccSet(value: moq.Mp4AtomHvcc)
	val lhvcGet: moq.Mp4AtomLhvc?
	fun lhvcSet(value: moq.Mp4AtomLhvc?)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val taicGet: moq.Mp4AtomTaic?
	fun taicSet(value: moq.Mp4AtomTaic?)
	val fielGet: moq.Mp4AtomFiel?
	fun fielSet(value: moq.Mp4AtomFiel?)
	val ccstGet: moq.Mp4AtomCcst?
	fun ccstSet(value: moq.Mp4AtomCcst?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomHvc1
	}
}
class Mp4AtomVp08 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomVp08
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val vpccGet: moq.Mp4AtomVpcC
	fun vpccSet(value: moq.Mp4AtomVpcC)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomVp08
	}
}
class Mp4AtomVp09 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomVp09
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val vpccGet: moq.Mp4AtomVpcC
	fun vpccSet(value: moq.Mp4AtomVpcC)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomVp09
	}
}
class Mp4AtomAv01 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomAv01
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val visualGet: moq.Mp4AtomVisual
	fun visualSet(value: moq.Mp4AtomVisual)
	val av1cGet: moq.Mp4AtomAv1c
	fun av1cSet(value: moq.Mp4AtomAv1c)
	val btrtGet: moq.Mp4AtomBtrt?
	fun btrtSet(value: moq.Mp4AtomBtrt?)
	val ccstGet: moq.Mp4AtomCcst?
	fun ccstSet(value: moq.Mp4AtomCcst?)
	val colrGet: moq.Mp4AtomColr?
	fun colrSet(value: moq.Mp4AtomColr?)
	val paspGet: moq.Mp4AtomPasp?
	fun paspSet(value: moq.Mp4AtomPasp?)
	val taicGet: moq.Mp4AtomTaic?
	fun taicSet(value: moq.Mp4AtomTaic?)
	val spatialGet: moq.Mp4AtomSpatial?
	fun spatialSet(value: moq.Mp4AtomSpatial?)
	companion object {
		fun default(): moq.Mp4AtomAv01
	}
}
class Mp4AtomTx3g internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomTx3g
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val dataReferenceIndexGet: kotlin.UShort
	fun dataReferenceIndexSet(value: kotlin.UShort)
	val displayFlagsGet: kotlin.UInt
	fun displayFlagsSet(value: kotlin.UInt)
	val horizontalJustificationGet: kotlin.Byte
	fun horizontalJustificationSet(value: kotlin.Byte)
	val verticalJustificationGet: kotlin.Byte
	fun verticalJustificationSet(value: kotlin.Byte)
	val bgColorRgbaGet: moq.Mp4AtomRgbaColor
	fun bgColorRgbaSet(value: moq.Mp4AtomRgbaColor)
	val styleRecordGet: ArrayU8x12
	fun styleRecordSet(value: ArrayU8x12)
	val ftabGet: moq.Mp4AtomFtab?
	fun ftabSet(value: moq.Mp4AtomFtab?)
	val dispGet: moq.Mp4AtomDisp?
	fun dispSet(value: moq.Mp4AtomDisp?)
	companion object {
		fun default(): moq.Mp4AtomTx3g
	}
}
class RustlsPkiTypesTrustAnchor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsPkiTypesTrustAnchor
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val subjectGet: moq.RustlsPkiTypesDer
	fun subjectSet(value: moq.RustlsPkiTypesDer)
	val subjectPublicKeyInfoGet: moq.RustlsPkiTypesDer
	fun subjectPublicKeyInfoSet(value: moq.RustlsPkiTypesDer)
	val nameConstraintsGet: moq.RustlsPkiTypesDer?
	fun nameConstraintsSet(value: moq.RustlsPkiTypesDer?)
	fun toOwned(): moq.RustlsPkiTypesTrustAnchor
}
class MoqTokioQuicServerId internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicServerId
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class MoqTokioListenForward internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioListenForward
	val bindGet: moq.SocketAddr?
	fun bindSet(value: moq.SocketAddr?)
	val keyGet: moq.PathBuf?
	fun keySet(value: moq.PathBuf?)
	val peersGet: moq.PathBuf?
	fun peersSet(value: moq.PathBuf?)
	fun configured(): kotlin.Boolean
	companion object {
		fun default(): moq.MoqTokioListenForward
	}
}
class Mp4AtomMeta internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomMeta
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val hdlrGet: moq.Mp4AtomHdlr
	fun hdlrSet(value: moq.Mp4AtomHdlr)
	val itemsGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun itemsSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	fun getPitm(): moq.Mp4AtomPitm?
	fun getDinf(): moq.Mp4AtomDinf?
	fun getIloc(): moq.Mp4AtomIloc?
	fun getIinf(): moq.Mp4AtomIinf?
	fun getIprp(): moq.Mp4AtomIprp?
	fun getIref(): moq.Mp4AtomIref?
	fun getIdat(): moq.Mp4AtomIdat?
	fun getIlst(): moq.Mp4AtomIlst?
	fun getKeys(): moq.Mp4AtomKeys?
	fun getMutPitm(): moq.Mp4AtomPitm?
	fun getMutDinf(): moq.Mp4AtomDinf?
	fun getMutIloc(): moq.Mp4AtomIloc?
	fun getMutIinf(): moq.Mp4AtomIinf?
	fun getMutIprp(): moq.Mp4AtomIprp?
	fun getMutIref(): moq.Mp4AtomIref?
	fun getMutIdat(): moq.Mp4AtomIdat?
	fun getMutIlst(): moq.Mp4AtomIlst?
	fun getMutKeys(): moq.Mp4AtomKeys?
	fun pushPitm(atom: moq.Mp4AtomPitm)
	fun pushDinf(atom: moq.Mp4AtomDinf)
	fun pushIloc(atom: moq.Mp4AtomIloc)
	fun pushIinf(atom: moq.Mp4AtomIinf)
	fun pushIprp(atom: moq.Mp4AtomIprp)
	fun pushIref(atom: moq.Mp4AtomIref)
	fun pushIdat(atom: moq.Mp4AtomIdat)
	fun pushIlst(atom: moq.Mp4AtomIlst)
	fun pushKeys(atom: moq.Mp4AtomKeys)
	fun removePitm(): moq.Mp4AtomPitm?
	fun removeDinf(): moq.Mp4AtomDinf?
	fun removeIloc(): moq.Mp4AtomIloc?
	fun removeIinf(): moq.Mp4AtomIinf?
	fun removeIprp(): moq.Mp4AtomIprp?
	fun removeIref(): moq.Mp4AtomIref?
	fun removeIdat(): moq.Mp4AtomIdat?
	fun removeIlst(): moq.Mp4AtomIlst?
	fun removeKeys(): moq.Mp4AtomKeys?
}
class Mp4AtomMdia internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomMdia
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mdhdGet: moq.Mp4AtomMdhd
	fun mdhdSet(value: moq.Mp4AtomMdhd)
	val hdlrGet: moq.Mp4AtomHdlr
	fun hdlrSet(value: moq.Mp4AtomHdlr)
	val minfGet: moq.Mp4AtomMinf
	fun minfSet(value: moq.Mp4AtomMinf)
	companion object {
		fun default(): moq.Mp4AtomMdia
	}
}
class Mp4AtomUdta internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomUdta
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val chplGet: moq.Mp4AtomChpl?
	fun chplSet(value: moq.Mp4AtomChpl?)
	val cprtGet: kotlin.collections.List<moq.Mp4AtomCprt>
	fun cprtSet(value: kotlin.collections.List<moq.Mp4AtomCprt>)
	val kindGet: kotlin.collections.List<moq.Mp4AtomKind>
	fun kindSet(value: kotlin.collections.List<moq.Mp4AtomKind>)
	val metaGet: moq.Mp4AtomMeta?
	fun metaSet(value: moq.Mp4AtomMeta?)
	val rtngGet: kotlin.collections.List<moq.Mp4AtomRtng>
	fun rtngSet(value: kotlin.collections.List<moq.Mp4AtomRtng>)
	companion object {
		fun default(): moq.Mp4AtomUdta
	}
}
class Mp4AtomVisual internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomVisual
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val dataReferenceIndexGet: kotlin.UShort
	fun dataReferenceIndexSet(value: kotlin.UShort)
	val widthGet: kotlin.UShort
	fun widthSet(value: kotlin.UShort)
	val heightGet: kotlin.UShort
	fun heightSet(value: kotlin.UShort)
	val horizresolutionGet: moq.Mp4AtomFixedPointU16
	fun horizresolutionSet(value: moq.Mp4AtomFixedPointU16)
	val vertresolutionGet: moq.Mp4AtomFixedPointU16
	fun vertresolutionSet(value: moq.Mp4AtomFixedPointU16)
	val frameCountGet: kotlin.UShort
	fun frameCountSet(value: kotlin.UShort)
	val compressorGet: moq.Mp4AtomCompressor
	fun compressorSet(value: moq.Mp4AtomCompressor)
	val depthGet: kotlin.UShort
	fun depthSet(value: kotlin.UShort)
	companion object {
		fun default(): moq.Mp4AtomVisual
	}
}
class RustlsClientDangerDangerousClientConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
}
class RustlsClientResumption internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsClientResumption
	fun tls12Resumption(tls12: moq.RustlsClientTls12Resumption): moq.RustlsClientResumption
	companion object {
		fun default(): moq.RustlsClientResumption
		fun inMemorySessions(num: kotlin.ULong): moq.RustlsClientResumption
		fun disabled(): moq.RustlsClientResumption
	}
}
class RustlsDistinguishedName internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsDistinguishedName
	companion object {
		fun inSequence(bytes: moq.Bytes): moq.RustlsDistinguishedName
	}
}
class MoqTokioWebsocketResolved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioWebsocketResolved
	val enabledGet: kotlin.Boolean
	fun enabledSet(value: kotlin.Boolean)
	val delayGet: moq.Duration
	fun delaySet(value: moq.Duration)
}
class MoqTokioUnixAllow internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioUnixAllow
	val uidGet: kotlin.UIntArray
	fun uidSet(value: kotlin.UIntArray)
	val gidGet: kotlin.UIntArray
	fun gidSet(value: kotlin.UIntArray)
	val pidGet: kotlin.IntArray
	fun pidSet(value: kotlin.IntArray)
	companion object {
		fun default(): moq.MoqTokioUnixAllow
	}
}
class MoqTokioWebsocketError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
class MoqSockDemuxCounters internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqSockDemuxCounters
	fun discard(reason: moq.MoqSockDemuxDiscard)
	fun get(reason: moq.MoqSockDemuxDiscard): kotlin.ULong
	companion object {
		fun default(): moq.MoqSockDemuxCounters
	}
}
class MoqTokioQuicDemuxWebRtc internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	suspend fun recv(): moq.MoqTokioQuicDemuxDatagram?
	fun pin(remote: moq.SocketAddr): moq.MoqSockDemuxPin
}
class MoqTokioQuicDriverSendOptions internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverSendOptions
	val priorityGet: kotlin.Long
	fun prioritySet(value: kotlin.Long)
	companion object {
		fun default(): moq.MoqTokioQuicDriverSendOptions
	}
}
class MoqTokioQuicDriverSendStream internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	suspend fun writeAll(buf: moq.Bytes)
	suspend fun finish(): moq.MoqQuicVarInt?
	fun reset(errorCode: moq.MoqQuicVarInt)
	fun resetAt(offset: moq.MoqTokioQuicDriverSendOffset, errorCode: moq.MoqQuicVarInt)
	fun offset(): moq.MoqTokioQuicDriverSendOffset
	fun setPriority(priority: kotlin.Long)
	fun priority(): kotlin.Long
	fun setGroup(group: moq.MoqTokioQuicDriverSendGroup?)
	suspend fun stopped(): moq.MoqQuicVarInt?
	suspend fun pollClosed(): moq.MoqQuicVarInt?
	fun id(): moq.MoqQuicStreamId
}
class MoqTokioQuicDriverSendGroup internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverSendGroup
	fun setPriority(priority: kotlin.Long)
	fun priority(): kotlin.Long
}
class MoqTokioQuicDriverRecvStream internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	suspend fun readChunk(max: kotlin.ULong): moq.Bytes?
	suspend fun readToEnd(limit: kotlin.ULong): moq.Bytes
	fun setPriority(priority: kotlin.Long)
	fun stop(errorCode: moq.MoqQuicVarInt)
	fun id(): moq.MoqQuicStreamId
	suspend fun receivedReset(): moq.MoqQuicVarInt?
}
class MoqTokioQuicDriverSendError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverSendError
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class MoqQuicConnectionStats internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqQuicConnectionStats
	val udpTxGet: moq.MoqQuicUdpStats
	fun udpTxSet(value: moq.MoqQuicUdpStats)
	val udpRxGet: moq.MoqQuicUdpStats
	fun udpRxSet(value: moq.MoqQuicUdpStats)
	val frameTxGet: moq.MoqQuicFrameStats
	fun frameTxSet(value: moq.MoqQuicFrameStats)
	val frameRxGet: moq.MoqQuicFrameStats
	fun frameRxSet(value: moq.MoqQuicFrameStats)
	val pathGet: moq.MoqQuicPathStats
	fun pathSet(value: moq.MoqQuicPathStats)
	val creditGet: moq.MoqQuicCreditStats
	fun creditSet(value: moq.MoqQuicCreditStats)
	companion object {
		fun default(): moq.MoqQuicConnectionStats
	}
}
class MoqQuicCryptoHandshakeData internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqQuicCryptoHandshakeData
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val alpnGet: moq.Bytes?
	fun alpnSet(value: moq.Bytes?)
	val serverNameGet: kotlin.String?
	fun serverNameSet(value: kotlin.String?)
}
class Mp4AtomSpatial internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomSpatial
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val st3dGet: moq.Mp4AtomSt3d?
	fun st3dSet(value: moq.Mp4AtomSt3d?)
	val sv3dGet: moq.Mp4AtomSv3d?
	fun sv3dSet(value: moq.Mp4AtomSv3d?)
	val vexuGet: moq.Mp4AtomVexu?
	fun vexuSet(value: moq.Mp4AtomVexu?)
	val hfovGet: moq.Mp4AtomHfov?
	fun hfovSet(value: moq.Mp4AtomHfov?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomSpatial
	}
}
class Mp4AtomAny internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomAny
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun kind(): moq.Mp4AtomFourCc
}
class Mp4AtomMinf internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomMinf
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val vmhdGet: moq.Mp4AtomVmhd?
	fun vmhdSet(value: moq.Mp4AtomVmhd?)
	val smhdGet: moq.Mp4AtomSmhd?
	fun smhdSet(value: moq.Mp4AtomSmhd?)
	val nmhdGet: moq.Mp4AtomNmhd?
	fun nmhdSet(value: moq.Mp4AtomNmhd?)
	val sthdGet: moq.Mp4AtomSthd?
	fun sthdSet(value: moq.Mp4AtomSthd?)
	val hmhdGet: moq.Mp4AtomHmhd?
	fun hmhdSet(value: moq.Mp4AtomHmhd?)
	val dinfGet: moq.Mp4AtomDinf
	fun dinfSet(value: moq.Mp4AtomDinf)
	val stblGet: moq.Mp4AtomStbl
	fun stblSet(value: moq.Mp4AtomStbl)
	companion object {
		fun default(): moq.Mp4AtomMinf
	}
}
class Mp4AtomCompressor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomCompressor
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomCompressor
	}
}
class RustlsPkiTypesDer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsPkiTypesDer
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun fromSlice(der: moq.Bytes): moq.RustlsPkiTypesDer
	}
}
class MoqTokioQuicDemuxDatagram internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDemuxDatagram
	val remoteGet: moq.SocketAddr
	fun remoteSet(value: moq.SocketAddr)
	val localGet: moq.IpAddr?
	fun localSet(value: moq.IpAddr?)
	val dataGet: moq.Bytes
	fun dataSet(value: moq.Bytes)
}
class MoqSockDemuxPin internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
}
class MoqTokioQuicDriverSendOffset internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverSendOffset
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class MoqTokioQuicDriverRecvError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDriverRecvError
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class MoqQuicFrameStats internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqQuicFrameStats
	val acksGet: kotlin.ULong
	fun acksSet(value: kotlin.ULong)
	val ackFrequencyGet: kotlin.ULong
	fun ackFrequencySet(value: kotlin.ULong)
	val cryptoGet: kotlin.ULong
	fun cryptoSet(value: kotlin.ULong)
	val connectionCloseGet: kotlin.ULong
	fun connectionCloseSet(value: kotlin.ULong)
	val dataBlockedGet: kotlin.ULong
	fun dataBlockedSet(value: kotlin.ULong)
	val datagramGet: kotlin.ULong
	fun datagramSet(value: kotlin.ULong)
	val handshakeDoneGet: kotlin.UByte
	fun handshakeDoneSet(value: kotlin.UByte)
	val immediateAckGet: kotlin.ULong
	fun immediateAckSet(value: kotlin.ULong)
	val maxDataGet: kotlin.ULong
	fun maxDataSet(value: kotlin.ULong)
	val maxStreamDataGet: kotlin.ULong
	fun maxStreamDataSet(value: kotlin.ULong)
	val maxStreamsBidiGet: kotlin.ULong
	fun maxStreamsBidiSet(value: kotlin.ULong)
	val maxStreamsUniGet: kotlin.ULong
	fun maxStreamsUniSet(value: kotlin.ULong)
	val newConnectionIdGet: kotlin.ULong
	fun newConnectionIdSet(value: kotlin.ULong)
	val newTokenGet: kotlin.ULong
	fun newTokenSet(value: kotlin.ULong)
	val pathChallengeGet: kotlin.ULong
	fun pathChallengeSet(value: kotlin.ULong)
	val pathResponseGet: kotlin.ULong
	fun pathResponseSet(value: kotlin.ULong)
	val pingGet: kotlin.ULong
	fun pingSet(value: kotlin.ULong)
	val resetStreamGet: kotlin.ULong
	fun resetStreamSet(value: kotlin.ULong)
	val retireConnectionIdGet: kotlin.ULong
	fun retireConnectionIdSet(value: kotlin.ULong)
	val streamDataBlockedGet: kotlin.ULong
	fun streamDataBlockedSet(value: kotlin.ULong)
	val streamsBlockedBidiGet: kotlin.ULong
	fun streamsBlockedBidiSet(value: kotlin.ULong)
	val streamsBlockedUniGet: kotlin.ULong
	fun streamsBlockedUniSet(value: kotlin.ULong)
	val stopSendingGet: kotlin.ULong
	fun stopSendingSet(value: kotlin.ULong)
	val streamGet: kotlin.ULong
	fun streamSet(value: kotlin.ULong)
	companion object {
		fun default(): moq.MoqQuicFrameStats
	}
}
class MoqQuicPathStats internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqQuicPathStats
	val rttGet: moq.Duration
	fun rttSet(value: moq.Duration)
	val minRttGet: moq.Duration
	fun minRttSet(value: moq.Duration)
	val cwndGet: kotlin.ULong
	fun cwndSet(value: kotlin.ULong)
	val bandwidthEstimateGet: kotlin.ULong?
	fun bandwidthEstimateSet(value: kotlin.ULong?)
	val congestionEventsGet: kotlin.ULong
	fun congestionEventsSet(value: kotlin.ULong)
	val spuriousCongestionEventsGet: kotlin.ULong
	fun spuriousCongestionEventsSet(value: kotlin.ULong)
	val lostPacketsGet: kotlin.ULong
	fun lostPacketsSet(value: kotlin.ULong)
	val lostBytesGet: kotlin.ULong
	fun lostBytesSet(value: kotlin.ULong)
	val sentPacketsGet: kotlin.ULong
	fun sentPacketsSet(value: kotlin.ULong)
	val sentPlpmtudProbesGet: kotlin.ULong
	fun sentPlpmtudProbesSet(value: kotlin.ULong)
	val lostPlpmtudProbesGet: kotlin.ULong
	fun lostPlpmtudProbesSet(value: kotlin.ULong)
	val sentBandwidthProbesGet: kotlin.ULong
	fun sentBandwidthProbesSet(value: kotlin.ULong)
	val sentBandwidthProbeBytesGet: kotlin.ULong
	fun sentBandwidthProbeBytesSet(value: kotlin.ULong)
	val blackHolesDetectedGet: kotlin.ULong
	fun blackHolesDetectedSet(value: kotlin.ULong)
	val currentMtuGet: kotlin.UShort
	fun currentMtuSet(value: kotlin.UShort)
	companion object {
		fun default(): moq.MoqQuicPathStats
	}
}
class Mp4AtomSv3d internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomSv3d
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val svhdGet: moq.Mp4AtomSvhd?
	fun svhdSet(value: moq.Mp4AtomSvhd?)
	val projGet: moq.Mp4AtomProj?
	fun projSet(value: moq.Mp4AtomProj?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomSv3d
	}
}
class Mp4AtomVexu internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomVexu
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val projGet: moq.Mp4AtomVexuProj?
	fun projSet(value: moq.Mp4AtomVexuProj?)
	val eyesGet: moq.Mp4AtomEyes?
	fun eyesSet(value: moq.Mp4AtomEyes?)
	val packGet: moq.Mp4AtomPack?
	fun packSet(value: moq.Mp4AtomPack?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomVexu
	}
}
class Mp4AtomIprp internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomIprp
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val ipcoGet: moq.Mp4AtomIpco
	fun ipcoSet(value: moq.Mp4AtomIpco)
	val ipmaGet: kotlin.collections.List<moq.Mp4AtomIpma>
	fun ipmaSet(value: kotlin.collections.List<moq.Mp4AtomIpma>)
	companion object {
		fun default(): moq.Mp4AtomIprp
	}
}
class Mp4AtomIpco internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomIpco
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val propertiesGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun propertiesSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomIpco
	}
}
class Mp4AtomMoov internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomMoov
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mvhdGet: moq.Mp4AtomMvhd
	fun mvhdSet(value: moq.Mp4AtomMvhd)
	val metaGet: moq.Mp4AtomMeta?
	fun metaSet(value: moq.Mp4AtomMeta?)
	val mvexGet: moq.Mp4AtomMvex?
	fun mvexSet(value: moq.Mp4AtomMvex?)
	val trakGet: kotlin.collections.List<moq.Mp4AtomTrak>
	fun trakSet(value: kotlin.collections.List<moq.Mp4AtomTrak>)
	val udtaGet: moq.Mp4AtomUdta?
	fun udtaSet(value: moq.Mp4AtomUdta?)
	val ainfGet: moq.Mp4AtomAinf?
	fun ainfSet(value: moq.Mp4AtomAinf?)
	companion object {
		fun default(): moq.Mp4AtomMoov
	}
}
class Mp4AtomStbl internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomStbl
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val stsdGet: moq.Mp4AtomStsd
	fun stsdSet(value: moq.Mp4AtomStsd)
	val sttsGet: moq.Mp4AtomStts
	fun sttsSet(value: moq.Mp4AtomStts)
	val cttsGet: moq.Mp4AtomCtts?
	fun cttsSet(value: moq.Mp4AtomCtts?)
	val stssGet: moq.Mp4AtomStss?
	fun stssSet(value: moq.Mp4AtomStss?)
	val stscGet: moq.Mp4AtomStsc
	fun stscSet(value: moq.Mp4AtomStsc)
	val stszGet: moq.Mp4AtomStsz
	fun stszSet(value: moq.Mp4AtomStsz)
	val stcoGet: moq.Mp4AtomStco?
	fun stcoSet(value: moq.Mp4AtomStco?)
	val co64Get: moq.Mp4AtomCo64?
	fun co64Set(value: moq.Mp4AtomCo64?)
	val sbgpGet: kotlin.collections.List<moq.Mp4AtomSbgp>
	fun sbgpSet(value: kotlin.collections.List<moq.Mp4AtomSbgp>)
	val sgpdGet: kotlin.collections.List<moq.Mp4AtomSgpd>
	fun sgpdSet(value: kotlin.collections.List<moq.Mp4AtomSgpd>)
	val subsGet: kotlin.collections.List<moq.Mp4AtomSubs>
	fun subsSet(value: kotlin.collections.List<moq.Mp4AtomSubs>)
	val saizGet: kotlin.collections.List<moq.Mp4AtomSaiz>
	fun saizSet(value: kotlin.collections.List<moq.Mp4AtomSaiz>)
	val saioGet: kotlin.collections.List<moq.Mp4AtomSaio>
	fun saioSet(value: kotlin.collections.List<moq.Mp4AtomSaio>)
	val cslgGet: moq.Mp4AtomCslg?
	fun cslgSet(value: moq.Mp4AtomCslg?)
	companion object {
		fun default(): moq.Mp4AtomStbl
	}
}
class Mp4AtomStsd internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomStsd
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val codecsGet: kotlin.collections.List<moq.Mp4AtomCodec>
	fun codecsSet(value: kotlin.collections.List<moq.Mp4AtomCodec>)
	companion object {
		fun default(): moq.Mp4AtomStsd
	}
}
class Mp4AtomNmhd internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomNmhd
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomNmhd
	}
}
class Mp4AtomSthd internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomSthd
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun default(): moq.Mp4AtomSthd
	}
}
class Mp4AtomMoof internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomMoof
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mfhdGet: moq.Mp4AtomMfhd
	fun mfhdSet(value: moq.Mp4AtomMfhd)
	val trafGet: kotlin.collections.List<moq.Mp4AtomTraf>
	fun trafSet(value: kotlin.collections.List<moq.Mp4AtomTraf>)
	companion object {
		fun default(): moq.Mp4AtomMoof
	}
}
class Mp4AtomTraf internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomTraf
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val tfhdGet: moq.Mp4AtomTfhd
	fun tfhdSet(value: moq.Mp4AtomTfhd)
	val tfdtGet: moq.Mp4AtomTfdt?
	fun tfdtSet(value: moq.Mp4AtomTfdt?)
	val trunGet: kotlin.collections.List<moq.Mp4AtomTrun>
	fun trunSet(value: kotlin.collections.List<moq.Mp4AtomTrun>)
	val sbgpGet: kotlin.collections.List<moq.Mp4AtomSbgp>
	fun sbgpSet(value: kotlin.collections.List<moq.Mp4AtomSbgp>)
	val sgpdGet: kotlin.collections.List<moq.Mp4AtomSgpd>
	fun sgpdSet(value: kotlin.collections.List<moq.Mp4AtomSgpd>)
	val subsGet: kotlin.collections.List<moq.Mp4AtomSubs>
	fun subsSet(value: kotlin.collections.List<moq.Mp4AtomSubs>)
	val saizGet: kotlin.collections.List<moq.Mp4AtomSaiz>
	fun saizSet(value: kotlin.collections.List<moq.Mp4AtomSaiz>)
	val saioGet: kotlin.collections.List<moq.Mp4AtomSaio>
	fun saioSet(value: kotlin.collections.List<moq.Mp4AtomSaio>)
	val metaGet: moq.Mp4AtomMeta?
	fun metaSet(value: moq.Mp4AtomMeta?)
	val sencGet: moq.Mp4AtomSenc?
	fun sencSet(value: moq.Mp4AtomSenc?)
	val udtaGet: moq.Mp4AtomUdta?
	fun udtaSet(value: moq.Mp4AtomUdta?)
	companion object {
		fun default(): moq.Mp4AtomTraf
	}
}
class Mp4AtomProj internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomProj
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val prhdGet: moq.Mp4AtomPrhd?
	fun prhdSet(value: moq.Mp4AtomPrhd?)
	val kindGet: moq.Mp4AtomProjKind?
	fun kindSet(value: moq.Mp4AtomProjKind?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomProj
	}
}
class Mp4AtomVexuProj internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomVexuProj
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val prjiGet: moq.Mp4AtomPrji?
	fun prjiSet(value: moq.Mp4AtomPrji?)
	val kindGet: moq.Mp4AtomVexuProjKind?
	fun kindSet(value: moq.Mp4AtomVexuProjKind?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomVexuProj
	}
}
class Mp4AtomEyes internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomEyes
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val striGet: moq.Mp4AtomStri?
	fun striSet(value: moq.Mp4AtomStri?)
	val heroGet: moq.Mp4AtomHero?
	fun heroSet(value: moq.Mp4AtomHero?)
	val camsGet: moq.Mp4AtomCams?
	fun camsSet(value: moq.Mp4AtomCams?)
	val cmfyGet: moq.Mp4AtomCmfy?
	fun cmfySet(value: moq.Mp4AtomCmfy?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomEyes
	}
}
class Mp4AtomPack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomPack
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val pkinGet: moq.Mp4AtomPkin?
	fun pkinSet(value: moq.Mp4AtomPkin?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomPack
	}
}
class Mp4AtomCams internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomCams
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val blinGet: moq.Mp4AtomBlin?
	fun blinSet(value: moq.Mp4AtomBlin?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomCams
	}
}
class Mp4AtomCmfy internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Mp4AtomCmfy
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val mustGet: moq.Mp4AtomMust?
	fun mustSet(value: moq.Mp4AtomMust?)
	val dadjGet: moq.Mp4AtomDadj?
	fun dadjSet(value: moq.Mp4AtomDadj?)
	val unknownGet: kotlin.collections.List<moq.Mp4AtomAny>
	fun unknownSet(value: kotlin.collections.List<moq.Mp4AtomAny>)
	companion object {
		fun default(): moq.Mp4AtomCmfy
	}
}
class Url internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Url
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		fun parse(input: kotlin.String): moq.Url
	}
}
class Json internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun toText(): kotlin.String
	companion object {
		fun parse(input: kotlin.String): moq.Json
	}
}
class Instant internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Instant
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class RustlsServerConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsServerConfig
}
class RustlsResolvesClientCert internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsResolvesClientCert
}
class RustlsKeyLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsKeyLog
}
class RustlsTimeProvider internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsTimeProvider
}
class RustlsCertDecompressor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class RustlsCertCompressor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class RustlsCompressionCache internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsCompressionCache
}
class RustlsCryptoProvider internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.RustlsCryptoProvider
}
class MoqCongestionFactory internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MoqCongestionFactory
}
class MoqQuicError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
interface TransportMetrics {}
class ClockElapsedException(val error: moq.ClockElapsed) : Exception() {
	override val message: String
}
class DecodeErrorException(val error: moq.DecodeError) : Exception() {
	override val message: String
}
class EncodeErrorException(val error: moq.EncodeError) : Exception() {
	override val message: String
}
class ErrorException(val error: moq.Error) : Exception() {
	override val message: String
}
class FrameFrameException(val error: moq.FrameFrame) : Exception() {
	override val message: String
}
class HangErrorException(val error: moq.HangError) : Exception() {
	override val message: String
}
class IntersectionErrorException(val error: moq.IntersectionError) : Exception() {
	override val message: String
}
class InvalidHopException(val error: moq.InvalidHop) : Exception() {
	override val message: String
}
class InvalidPatternException(val error: moq.InvalidPattern) : Exception() {
	override val message: String
}
class MoqTokioConnectErrorException(val error: moq.MoqTokioConnectError) : Exception() {
	override val message: String
}
class MoqTokioQuicDriverRecvErrorException(val error: moq.MoqTokioQuicDriverRecvError) : Exception() {
	override val message: String
}
class MoqTokioQuicDriverSendErrorException(val error: moq.MoqTokioQuicDriverSendError) : Exception() {
	override val message: String
}
class MoqTokioWebsocketErrorException(val error: moq.MoqTokioWebsocketError) : Exception() {
	override val message: String
}
class MuxCodecAacAdtsErrorException(val error: moq.MuxCodecAacAdtsError) : Exception() {
	override val message: String
}
class MuxCodecAacErrorException(val error: moq.MuxCodecAacError) : Exception() {
	override val message: String
}
class MuxCodecAnnexbErrorException(val error: moq.MuxCodecAnnexbError) : Exception() {
	override val message: String
}
class MuxCodecFlacErrorException(val error: moq.MuxCodecFlacError) : Exception() {
	override val message: String
}
class MuxCodecH264ErrorException(val error: moq.MuxCodecH264Error) : Exception() {
	override val message: String
}
class MuxCodecH265ErrorException(val error: moq.MuxCodecH265Error) : Exception() {
	override val message: String
}
class MuxCodecMp3ErrorException(val error: moq.MuxCodecMp3Error) : Exception() {
	override val message: String
}
class MuxCodecOpusErrorException(val error: moq.MuxCodecOpusError) : Exception() {
	override val message: String
}
class MuxContainerFmp4ErrorException(val error: moq.MuxContainerFmp4Error) : Exception() {
	override val message: String
}
class MuxErrorException(val error: moq.MuxError) : Exception() {
	override val message: String
}
class MuxTimelineErrorException(val error: moq.MuxTimelineError) : Exception() {
	override val message: String
}
class PathErrorException(val error: moq.PathError) : Exception() {
	override val message: String
}
class StringException(val error: kotlin.String) : Exception() {
	override val message: String
}
class TimeOverflowException(val error: moq.TimeOverflow) : Exception() {
	override val message: String
}
class TokioErrorException(val error: moq.TokioError) : Exception() {
	override val message: String
}
class TokioTlsErrorException(val error: moq.TokioTlsError) : Exception() {
	override val message: String
}
class TrackProducerException(val error: moq.TrackProducer) : Exception() {
	override val message: String
}
const val PATH_MAX_PARTS: ULong
const val PATH_MAX_BYTES: ULong
const val PATTERN_MAX_SEGMENTS: ULong
const val PATTERN_MAX_INTERSECTIONS: ULong
const val SETUP_TOKEN_OUT_OF_BAND: ULong
const val TRANSPORT_MIN_RECEIVE_WINDOW: ULong
const val TRANSPORT_SETUP_RECEIVE_WINDOW: ULong
const val GROUP_MAX_CACHE_BYTES: ULong
const val HANG_CATALOG_MOQ_EPOCH_UNIX_MILLIS: ULong
const val HANG_CATALOG_MAX_SAFE_INTEGER: ULong
const val MUX_CONTAINER_DEFAULT_MAX_BUFFER: ULong
const val MUX_CONTAINER_FMP4_MAX_HELD_BYTES: ULong
const val MUX_CONTAINER_FMP4_MAX_METADATA_BYTES: ULong
const val MUX_CONTAINER_FMP4_MAX_FRAGMENT_SAMPLES: ULong
const val MUX_CONTAINER_FMP4_MAX_TRACKS: ULong
const val MOQ_QUIC_VARINT_MAX_SIZE: ULong
const val MP4_ATOM_ST3D_MONO: UByte
const val MP4_ATOM_ST3D_TOP_BOTTOM: UByte
const val MP4_ATOM_ST3D_LEFT_RIGHT: UByte
const val MP4_ATOM_ST3D_STEREO_CUSTOM: UByte
const val MP4_ATOM_ST3D_RIGHT_LEFT: UByte
const val MP4_ATOM_HFOV_DENOMINATOR: UInt
const val MP4_ATOM_ESDS_ESDESCRIPTOR_TAG: UByte
const val MP4_ATOM_ESDS_DECODERCONFIG_TAG: UByte
const val MP4_ATOM_ESDS_SLCONFIG_TAG: UByte
const val MP4_ATOM_U24_MAX: UInt
const val MP4_ATOM_HERO_NONE: UByte
const val MP4_ATOM_HERO_LEFT: UByte
const val MP4_ATOM_HERO_RIGHT: UByte
const val MP4_ATOM_ESDS_DECODERSPECIFIC_TAG: UByte
const val MP4_ATOM_DADJ_DENOMINATOR: Int
val IMPLEMENTATION: kotlin.String
val ALPNS: kotlin.collections.List<kotlin.String>
fun clockNow(): moq.ClockInstant
fun clockWall(): moq.ClockWall
fun clockOrigin(): kotlin.Pair<moq.ClockInstant, moq.ClockWall>
fun clockRng(): moq.ClockRng
fun clockSeed(): kotlin.ULong
suspend fun clockDrive(poll: (moq.ClockInstant, moq.Waiter) -> moq.ClockInstant?): kotlin.String
fun clockInterval(period: moq.Duration): moq.ClockInterval
fun clockIntervalAt(start: moq.ClockInstant, period: moq.Duration): moq.ClockInterval
suspend fun clockSleep(duration: moq.Duration)
suspend fun clockSleepUntil(at: moq.ClockInstant)
suspend fun clockTimeout(duration: moq.Duration, future: suspend () -> kotlin.Unit)
suspend fun clockTimeoutAt(deadline: moq.ClockInstant, future: suspend () -> kotlin.Unit)
suspend fun timeRunDriver(driver: moq.Driver): moq.Error
suspend fun timeRunOriginDriver(driver: moq.OriginDriver): moq.Error
fun varintDecodeQuic(r: moq.Mut<moq.Bytes>): kotlin.ULong
fun varintEncodeQuic(value: kotlin.ULong, w: moq.Mut<moq.Bytes>)
val CACHEDEFAULTEXPIRY: moq.Duration
val HANGECHOSUFFIX: kotlin.String
val HANGECHOTRACK: kotlin.String
fun hangEchoIsEcho(path: moq.Path): kotlin.Boolean
val HANGCATALOGPRIORITY: moq.HangCatalogPriorities
val HANGCONTAINERMAXAGE: moq.Duration
val HANGCONTAINERTIMESCALE: moq.Timescale
fun hangContainerTrackInfo(priority: kotlin.Byte): moq.TrackInfo
fun tokioTlsParseFingerprint(value: kotlin.String): ArrayU8x32
fun tokioOriginSpawn(): moq.OriginProducer
fun transportMemoryPair(config: moq.TransportMemoryConfig): kotlin.Pair<moq.TransportMemorySession, moq.TransportMemorySession>
fun muxCodecAacConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
val MUXCODECANNEXBSTARTCODE: moq.Bytes
fun muxCodecAnnexbFromLengthPrefixed(payload: moq.Bytes, lengthSize: kotlin.ULong, prefix: moq.Bytes?): moq.Bytes
fun muxCodecAnnexbBuildPrefix(nals: kotlin.collections.List<moq.Bytes>): moq.Bytes
fun muxCodecAnnexbLengthPrefixedToAnnexb(data: moq.Bytes, lengthSize: kotlin.ULong, out: moq.Mut<moq.Bytes>)
fun muxCodecAnnexbAfterStartCode(b: moq.Bytes): kotlin.ULong?
fun muxCodecAnnexbFindStartCode(b: moq.Bytes): kotlin.Pair<kotlin.ULong, kotlin.ULong>?
fun muxCodecFlacConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
fun muxCodecH264Config(buf: moq.Bytes): moq.HangCatalogVideoConfig
fun muxCodecH265Config(buf: moq.Bytes): moq.HangCatalogVideoConfig
fun muxCodecMp3Config(init: moq.Bytes): moq.HangCatalogAudioConfig
val MUXCODECOPUSWARMUP: moq.Duration
fun muxCodecOpusPacketSamples(packet: moq.Bytes): kotlin.UInt?
fun muxCodecOpusConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
fun muxContainerMp4Repair(file: java.nio.channels.SeekableByteChannel): moq.MuxContainerMp4Finish?
interface StartSeekable {
	fun seekStart(offset: ULong): ULong
}
