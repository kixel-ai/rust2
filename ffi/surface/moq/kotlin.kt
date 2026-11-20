// FastNative.kt
package dalvik.annotation.optimization
@Retention(AnnotationRetention.BINARY) @Target(AnnotationTarget.FUNCTION) annotation class FastNative

// moq.kt
fun interface Waker {
	fun wake()
}
class Bytes private constructor(private val array: ByteArray?, private val direct: java.nio.ByteBuffer?, private val lent: _Native.__Lent?, private val offset: Int, val size: Int) {
	constructor(bytes: ByteArray)
	constructor(buffer: java.nio.ByteBuffer)
	operator fun get(index: Int): Byte
	fun slice(startIndex: Int, endIndex: Int = size): Bytes
	fun copyInto(destination: ByteArray, destinationOffset: Int = 0, startIndex: Int = 0, endIndex: Int = size): ByteArray
	fun copyInto(destination: java.nio.ByteBuffer, startIndex: Int = 0, endIndex: Int = size): java.nio.ByteBuffer
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
		@kotlin.jvm.JvmStatic fun of(address: java.net.InetAddress): IpAddr
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
		@kotlin.jvm.JvmStatic fun of(address: java.net.InetSocketAddress): SocketAddr
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
		@kotlin.jvm.JvmStatic val SECOND: moq.Timescale
		@kotlin.jvm.JvmStatic val MILLI: moq.Timescale
		@kotlin.jvm.JvmStatic val MICRO: moq.Timescale
		@kotlin.jvm.JvmStatic val NANO: moq.Timescale
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun new(unitsPerSecond: kotlin.ULong): moq.Timescale
	}
}
class Timestamp internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun value(): kotlin.ULong
	fun scale(): moq.Timescale?
	fun isZero(): kotlin.Boolean
	@kotlin.jvm.Throws(TimeOverflowException::class) fun convert(newScale: moq.Timescale): moq.Timestamp
	@kotlin.jvm.Throws(TimeOverflowException::class) fun asScale(target: moq.Timescale): java.math.BigInteger
	@kotlin.jvm.Throws(TimeOverflowException::class) fun asSecs(): kotlin.ULong
	@kotlin.jvm.Throws(TimeOverflowException::class) fun asMillis(): java.math.BigInteger
	@kotlin.jvm.Throws(TimeOverflowException::class) fun asMicros(): java.math.BigInteger
	@kotlin.jvm.Throws(TimeOverflowException::class) fun asNanos(): java.math.BigInteger
	@kotlin.jvm.Throws(TimeOverflowException::class) fun checkedAdd(rhs: moq.Timestamp): moq.Timestamp
	@kotlin.jvm.Throws(TimeOverflowException::class) fun checkedSub(rhs: moq.Timestamp): moq.Timestamp
	companion object {
		@kotlin.jvm.JvmStatic val ZERO: moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun new(value: kotlin.ULong, scale: moq.Timescale): moq.Timestamp
		@kotlin.jvm.JvmStatic fun unscaled(key: kotlin.ULong): moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun fromScale(value: kotlin.ULong, unitsPerSecond: kotlin.ULong): moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun fromSecs(seconds: kotlin.ULong): moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun fromMillis(millis: kotlin.ULong): moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun fromMicros(micros: kotlin.ULong): moq.Timestamp
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TimeOverflowException::class) fun fromNanos(nanos: kotlin.ULong): moq.Timestamp
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
	fun saturatingAdd(duration: moq.Duration): moq.ClockInstant
	fun saturatingSub(duration: moq.Duration): moq.ClockInstant
	fun intoStd(): moq.Instant
	companion object {
		@kotlin.jvm.JvmStatic val ZERO: moq.ClockInstant
		@kotlin.jvm.JvmStatic fun now(): moq.ClockInstant
		@kotlin.jvm.JvmStatic fun fromStd(at: moq.Instant): moq.ClockInstant
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
		@kotlin.jvm.JvmStatic val UNIXEPOCH: moq.ClockWall
		@kotlin.jvm.JvmStatic val MOQEPOCH: moq.ClockWall
		@kotlin.jvm.JvmStatic fun now(): moq.ClockWall
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
		@kotlin.jvm.JvmStatic fun default(): moq.SessionLimits
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsContent
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsTraffic
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
	val routesRefusedGet: kotlin.ULong
	fun routesRefusedSet(value: kotlin.ULong): moq.StatsPresence
	val routesGet: moq.StatsRoutes
	fun routesSet(value: moq.StatsRoutes): moq.StatsPresence
	fun add(other: moq.StatsPresence): moq.StatsPresence
	fun active(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsPresence
	}
}
class StatsActivity internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val idleGet: kotlin.ULong
	fun idleSet(value: kotlin.ULong): moq.StatsActivity
	val activeGet: kotlin.ULong
	fun activeSet(value: kotlin.ULong): moq.StatsActivity
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsActivity
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
	val txMessagesGet: kotlin.ULong
	fun txMessagesSet(value: kotlin.ULong): moq.StatsTotals
	val rxMessagesGet: kotlin.ULong
	fun rxMessagesSet(value: kotlin.ULong): moq.StatsTotals
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsTotals
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsFailures
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
	val membershipGet: kotlin.ULong
	fun membershipSet(value: kotlin.ULong): moq.StatsRefusals
	val expiredGet: kotlin.ULong
	fun expiredSet(value: kotlin.ULong): moq.StatsRefusals
	fun count(kind: moq.StatsRefusalKind): kotlin.ULong
	fun add(other: moq.StatsRefusals): moq.StatsRefusals
	fun regressed(previous: moq.StatsRefusals): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsRefusals
	}
}
class StatsRouteCloses internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val stalledGet: kotlin.ULong
	fun stalledSet(value: kotlin.ULong): moq.StatsRouteCloses
	val overrunGet: kotlin.ULong
	fun overrunSet(value: kotlin.ULong): moq.StatsRouteCloses
	val malformedGet: kotlin.ULong
	fun malformedSet(value: kotlin.ULong): moq.StatsRouteCloses
	val roleGet: kotlin.ULong
	fun roleSet(value: kotlin.ULong): moq.StatsRouteCloses
	val unaskedGet: kotlin.ULong
	fun unaskedSet(value: kotlin.ULong): moq.StatsRouteCloses
	val regressedGet: kotlin.ULong
	fun regressedSet(value: kotlin.ULong): moq.StatsRouteCloses
	val otherGet: kotlin.ULong
	fun otherSet(value: kotlin.ULong): moq.StatsRouteCloses
	fun count(kind: moq.StatsRouteClose): kotlin.ULong
	fun total(): kotlin.ULong
	fun add(other: moq.StatsRouteCloses): moq.StatsRouteCloses
	fun regressed(previous: moq.StatsRouteCloses): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsRouteCloses
	}
}
class StatsRouteRefusals internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val pathGet: kotlin.ULong
	fun pathSet(value: kotlin.ULong): moq.StatsRouteRefusals
	val scopeGet: kotlin.ULong
	fun scopeSet(value: kotlin.ULong): moq.StatsRouteRefusals
	val loopedGet: kotlin.ULong
	fun loopedSet(value: kotlin.ULong): moq.StatsRouteRefusals
	fun count(kind: moq.StatsRouteRefusal): kotlin.ULong
	fun total(): kotlin.ULong
	fun add(other: moq.StatsRouteRefusals): moq.StatsRouteRefusals
	fun regressed(previous: moq.StatsRouteRefusals): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsRouteRefusals
	}
}
class StatsRoutes internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val changedGet: kotlin.ULong
	fun changedSet(value: kotlin.ULong): moq.StatsRoutes
	val rewrittenGet: kotlin.ULong
	fun rewrittenSet(value: kotlin.ULong): moq.StatsRoutes
	val sentGet: moq.StatsWithdrawals
	fun sentSet(value: moq.StatsWithdrawals): moq.StatsRoutes
	val receivedGet: moq.StatsWithdrawals
	fun receivedSet(value: moq.StatsWithdrawals): moq.StatsRoutes
	val refusedGet: moq.StatsRouteRefusals
	fun refusedSet(value: moq.StatsRouteRefusals): moq.StatsRoutes
	val closedGet: moq.StatsRouteCloses
	fun closedSet(value: moq.StatsRouteCloses): moq.StatsRoutes
	fun add(other: moq.StatsRoutes): moq.StatsRoutes
	fun regressed(previous: moq.StatsRoutes): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsRoutes
	}
}
class StatsWithdrawals internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val deadGet: kotlin.ULong
	fun deadSet(value: kotlin.ULong): moq.StatsWithdrawals
	val drainGet: kotlin.ULong
	fun drainSet(value: kotlin.ULong): moq.StatsWithdrawals
	fun count(kind: moq.StatsWithdrawal): kotlin.ULong
	fun total(): kotlin.ULong
	fun add(other: moq.StatsWithdrawals): moq.StatsWithdrawals
	fun regressed(previous: moq.StatsWithdrawals): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsWithdrawals
	}
}
class TransportReceiveWindow internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val connectionGet: kotlin.ULong?
	fun connectionSet(value: kotlin.ULong?): moq.TransportReceiveWindow
	val streamGet: kotlin.ULong
	fun streamSet(value: kotlin.ULong): moq.TransportReceiveWindow
	companion object {
		@kotlin.jvm.JvmStatic fun new(connection: kotlin.ULong?, stream: kotlin.ULong): moq.TransportReceiveWindow
	}
}
class TransportLimits internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val receiveWindowGet: kotlin.ULong?
	fun receiveWindowSet(value: kotlin.ULong?): moq.TransportLimits
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TransportLimits
	}
}
@kotlin.jvm.JvmInline value class BandwidthRate internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun asBps(): kotlin.ULong
	fun scaled(factor: kotlin.Double): moq.BandwidthRate
	fun absDiff(other: moq.BandwidthRate): moq.BandwidthRate
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.BandwidthRate
		@kotlin.jvm.JvmStatic val ZERO: moq.BandwidthRate
		@kotlin.jvm.JvmStatic fun fromBps(bps: kotlin.ULong): moq.BandwidthRate
		@kotlin.jvm.JvmStatic fun fromKbps(kbps: kotlin.ULong): moq.BandwidthRate
		@kotlin.jvm.JvmStatic fun fromMbps(mbps: kotlin.ULong): moq.BandwidthRate
	}
}
data class DatagramDatagram(val sequence: kotlin.ULong, val timestamp: moq.Timestamp, val payload: moq.Bytes) {
	fun debug(): kotlin.String
}
data class FrameInfo(val size: kotlin.ULong, val timestamp: moq.Timestamp) {
	fun debug(): kotlin.String
}
data class GroupInfo(val start: kotlin.ULong) {
	fun debug(): kotlin.String
}
data class TrackDelay(val min: moq.Duration?, val max: moq.Duration?) {
	fun debug(): kotlin.String
	fun withMin(min: moq.Duration?): moq.TrackDelay
	fun withMax(max: moq.Duration?): moq.TrackDelay
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackDelay
	}
}
class TrackPosition internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val groupGet: kotlin.ULong
	fun groupSet(value: kotlin.ULong): moq.TrackPosition
	val frameGet: kotlin.ULong
	fun frameSet(value: kotlin.ULong): moq.TrackPosition
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackPosition
		@kotlin.jvm.JvmStatic fun new(group: kotlin.ULong, frame: kotlin.ULong): moq.TrackPosition
		@kotlin.jvm.JvmStatic fun group(group: kotlin.ULong): moq.TrackPosition
	}
}
data class TrackRange(val start: moq.TrackAnchor, val end: moq.TrackAnchor?) {
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun live(): moq.TrackRange
		@kotlin.jvm.JvmStatic fun oldest(): moq.TrackRange
	}
}
@kotlin.jvm.JvmInline value class OriginCost internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun value(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.OriginCost
		@kotlin.jvm.JvmStatic fun new(cost: kotlin.ULong): moq.OriginCost
	}
}
@kotlin.jvm.JvmInline value class OriginPriority internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun value(): kotlin.Byte
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.OriginPriority
		@kotlin.jvm.JvmStatic fun new(priority: kotlin.Byte): moq.OriginPriority
	}
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
	@kotlin.jvm.Throws(HangErrorException::class) fun wallClock(pts: moq.Timestamp): moq.ClockWall
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun new(wall: moq.Timestamp): moq.HangCatalogClock
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogAv1
		@kotlin.jvm.JvmStatic val PREFIX: kotlin.String
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogVp9
	}
}
data class HangCatalogDisplay(val width: kotlin.UInt, val height: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class HangContainerFrame(val timestamp: moq.Timestamp, val payload: moq.Bytes) {
	fun debug(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun encode(buf: moq.Mut<moq.Bytes>)
	@kotlin.jvm.Throws(HangErrorException::class) fun writeTo(group: moq.GroupProducer)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun decode(buf: moq.Bytes): moq.HangContainerFrame
	}
}
data class MuxContainerFrame(val timestamp: moq.Timestamp, val duration: moq.Timestamp?, val payload: moq.Bytes, val keyframe: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class MuxContainerTimestampRewind(val timestamp: moq.Timestamp, val floor: moq.Timestamp) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
data class MuxContainerPlayout(val delay: moq.TrackDelay, val warmup: moq.Duration) {
	fun debug(): kotlin.String
	fun subscription(subscription: moq.TrackSubscription): moq.TrackSubscription
	fun consumerContainer(track: moq.TrackSubscriber, format: moq.MuxCatalogHangContainer): moq.MuxContainerConsumerContainer
	fun consumerFmp4Wire(track: moq.TrackSubscriber, format: moq.MuxContainerFmp4Wire): moq.MuxContainerConsumerFmp4Wire
	fun consumerLegacyWire(track: moq.TrackSubscriber, format: moq.MuxContainerLegacyWire): moq.MuxContainerConsumerLegacyWire
	fun consumerLocWire(track: moq.TrackSubscriber, format: moq.MuxContainerLocWire): moq.MuxContainerConsumerLocWire
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerPlayout
		@kotlin.jvm.JvmStatic fun new(delay: moq.TrackDelay, warmup: moq.Duration): moq.MuxContainerPlayout
	}
}
data class MuxCodecFlacConfig(val minBlockSize: kotlin.UShort, val maxBlockSize: kotlin.UShort, val minFrameSize: kotlin.UInt, val maxFrameSize: kotlin.UInt, val sampleRate: kotlin.UInt, val channelCount: kotlin.UInt, val bitsPerSample: kotlin.UInt, val totalSamples: kotlin.ULong, val md5: ArrayU8x16) {
	fun debug(): kotlin.String
	fun encodeStreamInfo(): moq.Bytes
	fun description(): moq.Bytes
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecFlacErrorException::class) fun parseStreamInfo(buf: moq.Mut<moq.Bytes>): moq.MuxCodecFlacConfig
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecFlacErrorException::class) fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecFlacConfig
	}
}
data class MuxCodecH264Sps(val profile: kotlin.UByte, val constraints: kotlin.UByte, val level: kotlin.UByte, val codedWidth: kotlin.UInt, val codedHeight: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecH264ErrorException::class) fun parse(nal: moq.Bytes): moq.MuxCodecH264Sps
	}
}
data class MuxCodecH265VideoSignal(val primaries: kotlin.UByte, val transfer: kotlin.UByte, val matrix: kotlin.UByte, val fullRange: kotlin.Boolean) {
	fun debug(): kotlin.String
}
data class MuxCodecMp3Config(val sampleRate: kotlin.UInt, val channelCount: kotlin.UInt) {
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecMp3ErrorException::class) fun parse(data: moq.Bytes): moq.MuxCodecMp3Config
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
data class MuxContainerTsDescriptor(val tag: kotlin.UByte, val data: moq.Bytes) {
	fun debug(): kotlin.String
}
data class RustlsClientTicketRequest(val newSessionCount: kotlin.UByte, val resumptionCount: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class RustlsCryptoHpkeHpkePublicKey(val _0: moq.Bytes) {
	fun debug(): kotlin.String
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
	data object TooManyRequests : SessionError()
	data object GoawayTimeout : SessionError()
	data object Timeout : SessionError()
	data object Version : SessionError()
	data object MalformedAuthToken : SessionError()
	data class App(val _0: kotlin.UShort) : SessionError()
	data class Unknown(val _0: kotlin.UInt) : SessionError()
	data object Unsupported : SessionError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun toCode(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun fromCode(code: kotlin.UInt): moq.SessionError
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
		@kotlin.jvm.JvmStatic fun fromCode(code: kotlin.UInt): moq.StreamError
	}
}
enum class Retry(internal val value: Int) {
	SAME(0),
	REROUTE(1),
	NEVER(2);
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
		@kotlin.jvm.JvmStatic fun default(): moq.ClockMissedTickBehavior
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
	HANDOFF(1);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioRestart
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
	TLS(5);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
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
enum class ReconnectVerdict(internal val value: Int) {
	RETRY(0),
	UNUSABLE(1),
	SETTLED(2);
	fun debug(): kotlin.String
	companion object {}
}
enum class ReconnectConfigError(internal val value: Int) {
	EMPTY(0),
	HEALTHY(1);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {}
}
enum class ReconnectRedirect(internal val value: Int) {
	FOLLOW(0),
	SAME_HOST(1),
	IGNORE(2);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.ReconnectRedirect
	}
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
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.StatsFailure>
	}
}
enum class StatsRefusalKind(internal val value: Int) {
	REFUSED(0),
	UNAVAILABLE(1),
	REQUEST(2),
	FORBIDDEN(3),
	MEMBERSHIP(4),
	EXPIRED(5);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.StatsRefusalKind>
	}
}
enum class StatsRouteClose(internal val value: Int) {
	STALLED(0),
	OVERRUN(1),
	MALFORMED(2),
	ROLE(3),
	UNASKED(4),
	REGRESSED(5),
	OTHER(6);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.StatsRouteClose>
	}
}
enum class StatsRouteRefusal(internal val value: Int) {
	PATH(0),
	SCOPE(1),
	LOOPED(2);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.StatsRouteRefusal>
	}
}
enum class StatsWithdrawal(internal val value: Int) {
	DEAD(0),
	DRAIN(1);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.StatsWithdrawal>
	}
}
enum class TransportAddress(internal val value: Int) {
	URI(0),
	SETUP(1);
	fun debug(): kotlin.String
	companion object {}
}
sealed class TransportLimitsError {
	data object Unsupported : TransportLimitsError()
	data class Refused(val _0: kotlin.String) : TransportLimitsError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class TransportCloseKind(internal val value: Int) {
	TRANSPORT(0),
	IDLE_TIMEOUT(1),
	CLOSED(2),
	EVICTED(3);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TransportCloseKind
	}
}
enum class BroadcastKind(internal val value: Int) {
	TRACK(0),
	DATAGRAMS(1);
	fun debug(): kotlin.String
	companion object {}
}
enum class TrackCompression(internal val value: Int) {
	NONE(0),
	DEFLATE(1);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackCompression
	}
}
sealed class TrackAnchor {
	data class Relative(val _0: kotlin.ULong) : TrackAnchor()
	data object Oldest : TrackAnchor()
	data class Time(val _0: kotlin.ULong) : TrackAnchor()
	fun debug(): kotlin.String
}
enum class TrackOrder(internal val value: Int) {
	ASCENDING(0),
	DESCENDING(1);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackOrder
	}
}
enum class OriginLink(internal val value: Int) {
	LOCAL(0),
	PEER(1),
	UPSTREAM(2);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.OriginLink
	}
}
enum class OriginRouteKind(internal val value: Int) {
	START(0),
	UPDATE(1),
	END(2),
	OFFLINE(3),
	RESTART(4);
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogTextFormat
	}
}
sealed class HangCatalogTextRole {
	data object Subtitle : HangCatalogTextRole()
	data object Caption : HangCatalogTextRole()
	data class Unknown(val _0: kotlin.String) : HangCatalogTextRole()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogTextRole
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
enum class HangCatalogColorRange(internal val value: Int) {
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCatalogCatalogFormat
		@kotlin.jvm.JvmStatic val DEFAULT: moq.MuxCatalogCatalogFormat
		@kotlin.jvm.JvmStatic fun detect(name: kotlin.String): moq.MuxCatalogCatalogFormat?
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
sealed class MuxContainerReorderError {
	data class Depth(val reorder: kotlin.UByte, val max: kotlin.UInt, val codec: kotlin.String) : MuxContainerReorderError()
	data class Deepened(val declared: kotlin.UByte, val reorder: kotlin.UByte) : MuxContainerReorderError()
	data class Reordered(val _0: kotlin.ULong) : MuxContainerReorderError()
	data object Presentation : MuxContainerReorderError()
	data object Tie : MuxContainerReorderError()
	data object Undrained : MuxContainerReorderError()
	final override fun toString(): kotlin.String
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
sealed class TokioListenBind {
	data class Addr(val _0: moq.SocketAddr) : TokioListenBind()
	data class Host(val _0: kotlin.String, val _1: kotlin.UShort) : TokioListenBind()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	@kotlin.jvm.Throws(StringException::class) fun resolve(): moq.SocketAddr
}
sealed class TransportMemoryError {
	data class Session(val code: kotlin.UInt, val reason: kotlin.String) : TransportMemoryError()
	data class Reset(val _0: kotlin.UInt) : TransportMemoryError()
	data class Stopped(val _0: kotlin.UInt) : TransportMemoryError()
	data object Closed : TransportMemoryError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class TransportPollAcked(internal val value: Int) {
	DELIVERED(0),
	UNSUPPORTED(1),
	SENT(2);
	fun debug(): kotlin.String
	companion object {}
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsFraming
	}
}
sealed class MuxContainerTsStep {
	data class Rewind(val by: moq.Duration, val flagged: kotlin.Boolean) : MuxContainerTsStep()
	data class Leap(val by: moq.Duration) : MuxContainerTsStep()
	data class Identity(val transportStreamId: kotlin.UShort, val programNumber: kotlin.UShort) : MuxContainerTsStep()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
sealed class MuxCodecAacAdtsError {
	data object Truncated : MuxCodecAacAdtsError()
	data object Syncword : MuxCodecAacAdtsError()
	data object SampleRate : MuxCodecAacAdtsError()
	data object FrameLength : MuxCodecAacAdtsError()
	data object PacketLength : MuxCodecAacAdtsError()
	data class ObjectType(val _0: kotlin.UByte) : MuxCodecAacAdtsError()
	data class Rate(val _0: kotlin.UInt) : MuxCodecAacAdtsError()
	data class ChannelConfig(val _0: kotlin.UByte) : MuxCodecAacAdtsError()
	data class TooLarge(val _0: kotlin.ULong) : MuxCodecAacAdtsError()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class MuxContainerTsStatsClass(internal val value: Int) {
	AUDIO(1),
	VIDEO(2),
	DATA(0);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsClass
	}
}
enum class MoqTokioQuicCongestionControl(internal val value: Int) {
	BBR(0),
	CUBIC(1),
	UNLIMITED(2);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioQuicCongestionControl
	}
}
enum class MoqTokioAcceptFailure(internal val value: Int) {
	CONNECTION(0),
	EXHAUSTED(1),
	UNKNOWN(2);
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.MoqTokioAcceptFailure>
	}
}
enum class MoqTokioRedirect(internal val value: Int) {
	FOLLOW(0),
	SAME_HOST(1),
	IGNORE(2);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioRedirect
	}
}
sealed class MoqQuicRate {
	data object Unlimited : MoqQuicRate()
	data class BytesPerSecond(val _0: kotlin.ULong) : MoqQuicRate()
	fun debug(): kotlin.String
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
enum class MoqQuicConnectionMuxVersion(internal val value: Int) {
	Q_MUX00(0),
	Q_MUX01(1),
	Q_MUX02(2);
	fun debug(): kotlin.String
	fun usesRecords(): kotlin.Boolean
	fun alpn(): kotlin.String
	fun prefix(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.MoqQuicConnectionMuxVersion>
	}
}
enum class MoqSockDemuxDiscard(internal val value: Int) {
	STUN(0),
	UNKNOWN(1),
	UNCLAIMED(2),
	OVERFLOW(3);
	fun debug(): kotlin.String
	fun asStr(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.MoqSockDemuxDiscard>
	}
}
enum class MoqSockUdpEcnCodepoint(internal val value: Int) {
	ECT0(2),
	ECT1(1),
	CE(3);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun fromBits(x: kotlin.UByte): moq.MoqSockUdpEcnCodepoint?
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
		@kotlin.jvm.JvmStatic fun default(): moq.Park
		@kotlin.jvm.JvmStatic fun new(waiter: moq.Waiter): moq.Park
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
		@kotlin.jvm.JvmStatic fun new(waker: moq.Waker): moq.Waiter
		@kotlin.jvm.JvmStatic fun noop(): moq.Waiter
		@kotlin.jvm.JvmStatic suspend fun pollFuture(future: suspend () -> kotlin.Unit)
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
		@kotlin.jvm.JvmStatic fun default(): moq.WaiterList
		@kotlin.jvm.JvmStatic fun new(): moq.WaiterList
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
	@kotlin.jvm.Throws(ErrorException::class) fun poll(now: moq.ClockInstant, waiter: moq.Waiter): moq.ClockInstant?
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
		@kotlin.jvm.JvmStatic fun random(): moq.Instance
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
	fun asStr(): kotlin.String
	fun isEmpty(): kotlin.Boolean
	fun len(): kotlin.ULong
	@kotlin.jvm.Throws(PathErrorException::class) fun join(other: moq.Path): moq.Path
	@kotlin.jvm.Throws(PathErrorException::class) fun resolve(rel: moq.PathRelative): moq.Path
	fun tryResolve(rel: moq.PathRelative): moq.Path?
	fun relative(base: moq.Path): moq.PathRelative?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.Path
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(PathErrorException::class) fun new(s: kotlin.String): moq.Path
		@kotlin.jvm.JvmStatic fun fromStatic(s: kotlin.String): moq.Path
		@kotlin.jvm.JvmStatic fun empty(): moq.Path
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
	@kotlin.jvm.Throws(IntersectionErrorException::class) fun intersect(other: moq.Pattern): moq.Patterns
	fun captures(matched: moq.Pattern): kotlin.collections.List<moq.Pattern>?
	@kotlin.jvm.Throws(InvalidPatternException::class) fun rooted(root: kotlin.String): moq.Pattern
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.Pattern
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(InvalidPatternException::class) fun new(segments: kotlin.collections.List<moq.Segment>): moq.Pattern
		@kotlin.jvm.JvmStatic fun all(): moq.Pattern
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(InvalidPatternException::class) fun literal(path: kotlin.String): moq.Pattern
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(InvalidPatternException::class) fun subtree(path: kotlin.String): moq.Pattern
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
	@kotlin.jvm.Throws(IntersectionErrorException::class) fun intersect(other: moq.Patterns): moq.Patterns
	fun rebase(root: kotlin.String): moq.Patterns
	@kotlin.jvm.Throws(InvalidPatternException::class) fun rooted(root: kotlin.String): moq.Patterns
	fun iter(): kotlin.collections.List<moq.Pattern>
	fun len(): kotlin.ULong
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.Patterns
		@kotlin.jvm.JvmStatic fun new(): moq.Patterns
	}
}
class Server internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Server
	fun withPublisherProducer(publish: moq.OriginProducer): moq.Server
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.Server
	fun withSubscriber(subscribe: moq.OriginProducer): moq.Server
	fun withAllocator(allocator: moq.BandwidthAllocator): moq.Server
	fun withStats(stats: moq.StatsSession): moq.Server
	fun withLimits(limits: moq.SessionLimits): moq.Server
	fun withVersions(versions: moq.Versions): moq.Server
	fun withImplementation(implementation: kotlin.String): moq.Server
	fun withTimeout(timeout: moq.Duration?): moq.Server
	fun deadline(arrived: moq.ClockInstant): moq.ClockInstant?
	fun withReceiveWindow(window: kotlin.ULong): moq.Server
	@kotlin.jvm.Throws(ErrorException::class) suspend fun acceptMemorySession(now: moq.ClockInstant, session: moq.TransportMemorySession): kotlin.Pair<moq.Session, moq.Driver>
	@kotlin.jvm.Throws(ErrorException::class) suspend fun acceptQmuxSession(now: moq.ClockInstant, session: moq.QmuxSession): kotlin.Pair<moq.Session, moq.Driver>
	@kotlin.jvm.Throws(ErrorException::class) suspend fun handshakeMemorySession(now: moq.ClockInstant, session: moq.TransportMemorySession): moq.ServerHandshake
	@kotlin.jvm.Throws(ErrorException::class) suspend fun handshakeQmuxSession(now: moq.ClockInstant, session: moq.QmuxSession): moq.ServerHandshake
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.Server
		@kotlin.jvm.JvmStatic fun new(): moq.Server
	}
}
class Session internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.Session
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun version(): moq.Version
	suspend fun peerImplementation(): kotlin.String?
	fun sendBandwidth(): moq.BandwidthConsumer?
	fun allocator(): moq.BandwidthAllocator
	fun recvBandwidth(): moq.BandwidthConsumer?
	fun viewers(): moq.BandwidthViewers
	fun stats(): moq.SessionStats
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun close_()
	suspend fun closed(): moq.Error
	@kotlin.jvm.Throws(ErrorException::class) fun goaway(goaway: moq.GoawayGoaway)
	fun draining(): moq.GoawayConsumer
	fun auth(): moq.AuthHandle
}
class Client internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.Client
	fun withPublisherProducer(publish: moq.OriginProducer): moq.Client
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.Client
	fun withSubscriber(subscribe: moq.OriginProducer): moq.Client
	fun withStats(stats: moq.StatsSession): moq.Client
	fun withLimits(limits: moq.SessionLimits): moq.Client
	fun withAllocator(allocator: moq.BandwidthAllocator): moq.Client
	fun withViewers(viewers: moq.BandwidthViewers): moq.Client
	fun withVersions(versions: moq.Versions): moq.Client
	fun withUrl(url: moq.Url): moq.Client
	fun url(): moq.Url?
	fun withToken(token: moq.SetupToken): moq.Client
	fun withAuth(auth: moq.AuthHandle): moq.Client
	fun withImplementation(implementation: kotlin.String): moq.Client
	fun withCost(cost: moq.OriginCost): moq.Client
	fun withPeerNode(node: kotlin.ULong): moq.Client
	@kotlin.jvm.Throws(ErrorException::class) suspend fun connectMemorySession(now: moq.ClockInstant, session: moq.TransportMemorySession): kotlin.Pair<moq.Session, moq.Driver>
	@kotlin.jvm.Throws(ErrorException::class) suspend fun connectQmuxSession(now: moq.ClockInstant, session: moq.QmuxSession): kotlin.Pair<moq.Session, moq.Driver>
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.Client
		@kotlin.jvm.JvmStatic fun new(): moq.Client
	}
}
class Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Error
	fun session(): moq.SessionError?
	fun stream(): moq.StreamError?
	fun retry(): moq.Retry
	companion object {
		@kotlin.jvm.JvmStatic fun fromTransportError(err: moq.Error): moq.Error
		@kotlin.jvm.JvmStatic fun fromTransportMemoryError(err: moq.TransportMemoryError): moq.Error
	}
}
class Version internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Version
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun hasRequestPath(): kotlin.Boolean
	fun asStr(): kotlin.String
	fun alpn(): kotlin.String
	fun isLite(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<moq.Version>
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
		@kotlin.jvm.JvmStatic fun default(): moq.Versions
		@kotlin.jvm.JvmStatic fun all(): moq.Versions
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
		@kotlin.jvm.JvmStatic fun current(): moq.ClockClock
	}
}
class ClockDeadline internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun set(at: moq.ClockInstant?)
	fun deadline(): moq.ClockInstant?
	suspend fun poll()
	companion object {
		@kotlin.jvm.JvmStatic fun new(deadlines: moq.ClockDeadlines): moq.ClockDeadline
		@kotlin.jvm.JvmStatic fun after(deadlines: moq.ClockDeadlines, duration: moq.Duration): moq.ClockDeadline
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
		@kotlin.jvm.JvmStatic fun new(now: moq.ClockInstant): moq.ClockDeadlines
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
		@kotlin.jvm.JvmStatic fun default(): moq.ClockManual
		@kotlin.jvm.JvmStatic val WALLSTART: moq.ClockWall
		@kotlin.jvm.JvmStatic fun new(): moq.ClockManual
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
		@kotlin.jvm.JvmStatic fun default(): moq.ClockWallMap
		@kotlin.jvm.JvmStatic val TOLERANCE: moq.Duration
	}
}
class CoopBudget internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.CoopBudget
	suspend fun pollYield()
	companion object {
		@kotlin.jvm.JvmStatic fun new(passes: kotlin.UInt): moq.CoopBudget
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
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalog
		@kotlin.jvm.JvmStatic val DEFAULTNAME: kotlin.String
		@kotlin.jvm.JvmStatic fun defaultTrackInfo(): moq.TrackInfo
		@kotlin.jvm.JvmStatic fun defaultSubscription(): moq.TrackSubscription
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromSlice(v: moq.Bytes): moq.HangCatalog
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromReader(reader: moq.Bytes): moq.HangCatalog
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
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val extGet: moq.MuxCatalogHangExtra
	fun extSet(value: moq.MuxCatalogHangExtra)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogExtra
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromSlice(v: moq.Bytes): moq.HangCatalogExtra
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromReader(reader: moq.Bytes): moq.HangCatalogExtra
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
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val extGet: moq.MuxContainerTsExt
	fun extSet(value: moq.MuxContainerTsExt)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogExt
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromSlice(v: moq.Bytes): moq.HangCatalogExt
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(HangErrorException::class) fun fromReader(reader: moq.Bytes): moq.HangCatalogExt
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun capture(at: moq.ClockInstant): moq.Timestamp
	fun timescale(): moq.Timescale
	fun wall(): moq.HangCatalogClock?
	@kotlin.jvm.Throws(MuxErrorException::class) fun wallClock(pts: moq.Timestamp): moq.ClockWall
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxClock
		@kotlin.jvm.JvmStatic val TIMESCALE: moq.Timescale
		@kotlin.jvm.JvmStatic fun new(): moq.MuxClock
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun at(epoch: moq.ClockInstant, wall: moq.ClockWall): moq.MuxClock
	}
}
class MuxIngest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxIngest
	val maxAgeGet: moq.Duration?
	fun maxAgeSet(value: moq.Duration?)
	val bandwidthGet: moq.BandwidthAllocator
	fun bandwidthSet(value: moq.BandwidthAllocator)
	val lingerGet: moq.Duration
	fun lingerSet(value: moq.Duration)
	val rateGet: moq.BandwidthLimit?
	fun rateSet(value: moq.BandwidthLimit?)
	fun catalog(): moq.MuxCatalogConfig
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxIngest
	}
}
class MuxLingerUnit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun wait()
	companion object {
		@kotlin.jvm.JvmStatic val DEFAULT: moq.Duration
		@kotlin.jvm.JvmStatic fun new(broadcasts: kotlin.collections.List<moq.BroadcastProducer>, duration: moq.Duration): moq.MuxLingerUnit
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxPacer
	}
}
class MuxBinding internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun broadcast(): moq.BroadcastConsumer
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastConsumer): moq.MuxBinding
	}
}
class MuxSource internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxSource
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangEchoRendition>>
	fun broadcast(): moq.BroadcastConsumer
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun catalogExtra(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExtra
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun catalogExt(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExt
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun catalogUnit(format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumer
	fun resolveReference(rel: moq.PathRelative?): moq.Path?
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun resolve(rel: moq.PathRelative?, instance: moq.Instance?): moq.BroadcastConsumer
	@kotlin.jvm.Throws(MuxErrorException::class) fun bind(rel: moq.PathRelative?, instance: moq.Instance?): moq.MuxBinding
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun subscribeTrack(rel: moq.PathRelative?, instance: moq.Instance?, name: kotlin.String): moq.TrackSubscriber
	@kotlin.jvm.Throws(MuxErrorException::class) fun follow(rel: moq.PathRelative?, instance: moq.Instance?): moq.MuxFollowInstances
	companion object {
		@kotlin.jvm.JvmStatic fun new(origin: moq.OriginConsumer, broadcast: moq.BroadcastConsumer): moq.MuxSource
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
		@kotlin.jvm.JvmStatic fun default(): moq.TokioDuration
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
	val maxGet: moq.MoqTokioCliBound?
	fun maxSet(value: moq.MoqTokioCliBound?)
	val healthyGet: moq.TokioDuration?
	fun healthySet(value: moq.TokioDuration?)
	val timeoutGet: moq.MoqTokioCliBound?
	fun timeoutSet(value: moq.MoqTokioCliBound?)
	fun initial(): moq.Duration
	fun max(): moq.Duration?
	fun timeout(): moq.Duration?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioBackoff
	}
}
class TokioClient internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioClient
	fun withPublisherProducer(publish: moq.OriginProducer): moq.TokioClient
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.TokioClient
	fun withSubscriber(subscribe: moq.OriginProducer): moq.TokioClient
	fun withLimits(limits: moq.SessionLimits): moq.TokioClient
	fun withStats(stats: moq.StatsSession): moq.TokioClient
	fun withCost(cost: kotlin.ULong): moq.TokioClient
	fun withPeerNode(node: kotlin.ULong): moq.TokioClient
	fun withToken(token: moq.SetupToken): moq.TokioClient
	fun withReconnect(reconnect: kotlin.Boolean): moq.TokioClient
	fun withRestart(restart: moq.TokioRestart): moq.TokioClient
	fun connect(addrs: moq.MoqTokioAddrs): moq.TokioConnection
	suspend fun close_()
	fun publish(origin: moq.OriginConsumer): moq.TokioConnection?
	fun consume(origin: moq.OriginProducer): moq.TokioConnection?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TokioErrorException::class) fun new(config: moq.TokioClientConfig): moq.TokioClient
	}
}
class TokioConnection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioConnection
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun close_()
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun established(): moq.TokioConnection
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun status(): moq.TokioStatus
	fun connected(): kotlin.Boolean
	fun connects(): kotlin.ULong
	fun version(): moq.Version?
	fun transport(): moq.TokioTransport?
	fun draining(): moq.GoawayConsumer?
	fun sendBandwidth(): moq.BandwidthConsumer
	fun allocator(): moq.BandwidthAllocator
	fun recvBandwidth(): moq.BandwidthConsumer
	fun viewers(): moq.BandwidthViewers
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun closed()
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
	@kotlin.jvm.Throws(TokioErrorException::class) fun init()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioLog
		@kotlin.jvm.JvmStatic fun new(level: moq.Level): moq.TokioLog
	}
}
class TokioServer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun servePublish(origin: moq.OriginConsumer)
	fun tlsAcceptor(): moq.MoqTokioTcpTlsAcceptor?
	fun certificates(): moq.TokioTlsCertificates
	fun acceptHealth(): kotlin.collections.List<moq.MoqTokioAcceptHealth>
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun bind(): moq.TokioServerListener
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun listen(): moq.TokioServerListener
	@kotlin.jvm.Throws(TokioErrorException::class) fun localAddr(): moq.SocketAddr
	fun websocketLocalAddr(): moq.SocketAddr?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TokioErrorException::class) fun new(config: moq.TokioServerConfig): moq.TokioServer
	}
}
class AuthWatch internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun peek(): moq.AuthGrant?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun changed(): moq.AuthGrant?
}
class AuthRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.AuthRequest?
}
class AuthRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun token(): moq.Bytes
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
		@kotlin.jvm.JvmStatic fun default(): moq.AuthGrant
		@kotlin.jvm.JvmStatic fun all(): moq.AuthGrant
	}
}
class AuthHandle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.AuthHandle
	fun grant(): moq.AuthWatch
	@kotlin.jvm.Throws(ErrorException::class) fun add(token: moq.Bytes): moq.AuthCredential
	fun authorize(grant: moq.AuthGrant)
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.AuthHandle
		@kotlin.jvm.JvmStatic fun new(): moq.AuthHandle
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
	@kotlin.jvm.Throws(ErrorException::class) suspend fun answered()
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
	@kotlin.jvm.Throws(ErrorException::class) suspend fun answered()
	companion object {
		@kotlin.jvm.JvmStatic fun new(tokens: kotlin.collections.List<moq.AuthToken>): moq.AuthTokens
	}
}
class GoawayGoaway internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.GoawayGoaway
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun withTimeout(timeout: moq.Duration?): moq.GoawayGoaway
	fun uri(): kotlin.String
	fun timeout(): moq.Duration?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.GoawayGoaway
		@kotlin.jvm.JvmStatic fun new(): moq.GoawayGoaway
		@kotlin.jvm.JvmStatic fun redirect(uri: kotlin.String): moq.GoawayGoaway
	}
}
class GoawayConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GoawayConsumer
	@kotlin.jvm.Throws(ErrorException::class) suspend fun poll(): moq.GoawayGoaway
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
		@kotlin.jvm.JvmStatic fun default(): moq.PathRelative
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(PathErrorException::class) fun new(s: kotlin.String): moq.PathRelative
		@kotlin.jvm.JvmStatic fun empty(): moq.PathRelative
	}
}
class ReconnectBackoff internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ReconnectBackoff
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val initialGet: moq.Duration
	fun initialSet(value: moq.Duration)
	val multiplierGet: kotlin.UInt
	fun multiplierSet(value: kotlin.UInt)
	val maxGet: moq.Duration?
	fun maxSet(value: moq.Duration?)
	val healthyGet: moq.Duration
	fun healthySet(value: moq.Duration)
	val timeoutGet: moq.Duration?
	fun timeoutSet(value: moq.Duration?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.ReconnectBackoff
	}
}
class ReconnectHandover internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ReconnectHandover
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val redirectGet: moq.ReconnectRedirect
	fun redirectSet(value: moq.ReconnectRedirect)
	val handoverGet: moq.Duration
	fun handoverSet(value: moq.Duration)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.ReconnectHandover
	}
}
class ReconnectConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ReconnectConfig
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val backoffGet: moq.ReconnectBackoff
	fun backoffSet(value: moq.ReconnectBackoff)
	val goawayGet: moq.ReconnectHandover
	fun goawaySet(value: moq.ReconnectHandover)
	val restartGet: moq.TokioRestart
	fun restartSet(value: moq.TokioRestart)
	val timeoutGet: moq.Duration?
	fun timeoutSet(value: moq.Duration?)
	val onceGet: kotlin.Boolean
	fun onceSet(value: kotlin.Boolean)
	val pinnedGet: kotlin.Boolean
	fun pinnedSet(value: kotlin.Boolean)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.ReconnectConfig
	}
}
class ServerHandshake internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun path(): kotlin.String
	fun role(): moq.SetupRole?
	fun peerNode(): kotlin.ULong?
	fun peerImplementation(): kotlin.String?
	fun auth(): moq.AuthHandle
	fun requests(): moq.AuthRequests
	fun token(): moq.SetupToken?
	fun withPublisherProducer(publish: moq.OriginProducer): moq.ServerHandshake
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.ServerHandshake
	fun withSubscriber(subscribe: moq.OriginProducer): moq.ServerHandshake
	fun withPeerNode(node: kotlin.ULong): moq.ServerHandshake
	fun withStats(stats: moq.StatsSession): moq.ServerHandshake
	fun withLimits(limits: moq.SessionLimits): moq.ServerHandshake
	fun withReceiveWindow(window: kotlin.ULong): moq.ServerHandshake
	suspend fun expired()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun ok(): kotlin.Pair<moq.Session, moq.Driver>
	fun close_(err: moq.Error)
}
class ServerAdmission internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.ServerAdmission
	fun enter(): moq.ServerAdmitted?
	fun drain()
	suspend fun close_(): moq.ClockInstant
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.ServerAdmission
	}
}
class ServerAdmitted internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
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
	val datagramsUnroutedGet: kotlin.ULong?
	fun datagramsUnroutedSet(value: kotlin.ULong?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.SessionStats
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsTier
		@kotlin.jvm.JvmStatic fun new(label: moq.Path): moq.StatsTier
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
	fun activity(): kotlin.collections.List<kotlin.Pair<moq.StatsTier, moq.StatsActivity>>
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsSnapshot
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsReport
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
		@kotlin.jvm.JvmStatic fun default(): moq.StatsConfig
		@kotlin.jvm.JvmStatic fun new(): moq.StatsConfig
	}
}
class StatsRegistry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsRegistry
	fun isEnabled(): kotlin.Boolean
	fun tier(tier: moq.StatsTier): moq.StatsHandle
	fun snapshot(): moq.StatsSnapshot
	fun report(report: moq.StatsReport, now: moq.ClockInstant)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsRegistry
		@kotlin.jvm.JvmStatic fun new(config: moq.StatsConfig): moq.StatsRegistry
		@kotlin.jvm.JvmStatic fun disabled(): moq.StatsRegistry
	}
}
class StatsHandle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsHandle
	fun refuse(refusal: moq.StatsRefusal)
	fun session(session: moq.StatsSessionInfo): moq.StatsSession
	fun request(label: moq.Path): moq.StatsSession
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsHandle
	}
}
class StatsRefusal internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsRefusal
	val labelGet: moq.Path
	fun labelSet(value: moq.Path)
	val reasonGet: moq.StatsRefusalKind
	fun reasonSet(value: moq.StatsRefusalKind)
	companion object {
		@kotlin.jvm.JvmStatic fun new(label: moq.Path, reason: moq.StatsRefusalKind): moq.StatsRefusal
	}
}
class StatsSessionInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsSessionInfo
	val labelGet: moq.Path
	fun labelSet(value: moq.Path)
	companion object {
		@kotlin.jvm.JvmStatic fun new(label: moq.Path): moq.StatsSessionInfo
	}
}
class StatsWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsWire
	}
}
class StatsSession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.StatsSession
	fun wire(counters: moq.TransportWire): moq.StatsWire
	fun retag(tier: moq.StatsTier, label: moq.Path)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsSession
	}
}
class StatsBill internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.StatsBill
	fun deliver(bytes: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.StatsBill
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
		@kotlin.jvm.JvmStatic fun default(): moq.TransportWire
		@kotlin.jvm.JvmStatic fun new(): moq.TransportWire
	}
}
class TransportProbe internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TransportProbe
	fun target(): moq.BandwidthRate
	fun setTarget(rate: moq.BandwidthRate)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TransportProbe
		@kotlin.jvm.JvmStatic fun new(): moq.TransportProbe
	}
}
class BandwidthLimit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BandwidthLimit
	fun set(cap: moq.BandwidthCap?)
	fun get(): moq.BandwidthCap?
	suspend fun changed(): moq.BandwidthCap?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.BandwidthLimit
		@kotlin.jvm.JvmStatic fun new(cap: moq.BandwidthCap?): moq.BandwidthLimit
	}
}
class BandwidthDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthDemand
	fun wanted(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun wantedChanged(): kotlin.Boolean
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
	@kotlin.jvm.Throws(ErrorException::class) fun update(max: moq.BandwidthRate)
}
class BandwidthCap internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BandwidthCap
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val rateGet: moq.BandwidthRate
	fun rateSet(value: moq.BandwidthRate)
	val burstGet: moq.Duration
	fun burstSet(value: moq.Duration)
	fun withBurst(burst: moq.Duration): moq.BandwidthCap
	companion object {
		@kotlin.jvm.JvmStatic val DEFAULTBURST: moq.Duration
		@kotlin.jvm.JvmStatic fun new(rate: moq.BandwidthRate): moq.BandwidthCap
	}
}
class BandwidthProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthProducer
	@kotlin.jvm.Throws(ErrorException::class) fun set(bitrate: moq.BandwidthRate?)
	fun consume(): moq.BandwidthConsumer
	fun weak(): moq.BandwidthWeak
	fun demand(): moq.BandwidthDemand
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.BandwidthProducer
		@kotlin.jvm.JvmStatic fun new(): moq.BandwidthProducer
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
		@kotlin.jvm.JvmStatic fun default(): moq.BandwidthAllocator
		@kotlin.jvm.JvmStatic fun new(estimate: moq.BandwidthConsumer): moq.BandwidthAllocator
		@kotlin.jvm.JvmStatic fun unlimited(): moq.BandwidthAllocator
	}
}
class BandwidthViewers internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BandwidthViewers
	@kotlin.jvm.Throws(ErrorException::class) fun watch(max: moq.BandwidthRate): moq.BandwidthViewer
	@kotlin.jvm.Throws(ErrorException::class) fun setTarget(target: moq.BandwidthRate?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.BandwidthViewers
		@kotlin.jvm.JvmStatic val MAXTARGET: moq.BandwidthRate
		@kotlin.jvm.JvmStatic fun new(estimate: moq.BandwidthWeak): moq.BandwidthViewers
	}
}
class BandwidthConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BandwidthConsumer
	fun peek(): moq.BandwidthRate?
	fun probed(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun changed(): moq.BandwidthRate?
}
class BroadcastDatagramRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastDatagramRequests
	@kotlin.jvm.Throws(ErrorException::class) suspend fun requested(): moq.DatagramRequest
}
class BroadcastConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastConsumer
	@kotlin.jvm.Throws(ErrorException::class) fun tracks(): moq.BroadcastTracks
	fun info(): moq.BroadcastInfo
	@kotlin.jvm.Throws(ErrorException::class) fun track(name: kotlin.String): moq.TrackConsumer
	@kotlin.jvm.Throws(ErrorException::class) fun datagrams(name: kotlin.String): moq.DatagramConsumer
	fun isClosed(): kotlin.Boolean
	suspend fun closed()
	fun isClone(other: moq.BroadcastConsumer): kotlin.Boolean
}
class BroadcastDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastDemand
	fun wanted(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun wantedChanged(): kotlin.Boolean
}
class BroadcastInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.BroadcastInfo
	val poolGet: moq.CachePool
	fun poolSet(value: moq.CachePool)
	val cacheDurationGet: moq.Duration?
	fun cacheDurationSet(value: moq.Duration?)
	val pathGet: moq.Path
	fun pathSet(value: moq.Path)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	fun produce(): moq.BroadcastProducer
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.BroadcastInfo
		@kotlin.jvm.JvmStatic fun new(): moq.BroadcastInfo
	}
}
class BroadcastProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastProducer
	fun info(): moq.BroadcastInfo
	fun demand(): moq.BroadcastDemand
	@kotlin.jvm.Throws(ErrorException::class) fun createTrack(name: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	@kotlin.jvm.Throws(ErrorException::class) fun createDatagrams(name: kotlin.String, info: moq.DatagramInfo): moq.DatagramProducer
	@kotlin.jvm.Throws(ErrorException::class) fun reserveTrack(name: kotlin.String): moq.TrackRequest
	@kotlin.jvm.Throws(ErrorException::class) fun reserveUnlistedTrack(name: kotlin.String): moq.TrackRequest
	@kotlin.jvm.Throws(ErrorException::class) fun uniqueTrack(suffix: kotlin.String, info: moq.TrackInfo?): moq.TrackProducer
	fun uniqueName(suffix: kotlin.String): kotlin.String
	fun requests(): moq.BroadcastRequests
	fun consume(): moq.BroadcastConsumer
	fun close_()
	fun isClone(other: moq.BroadcastProducer): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(info: moq.BroadcastInfo): moq.BroadcastProducer
	}
}
class BroadcastRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.BroadcastRequests
	fun info(): moq.BroadcastInfo
	@kotlin.jvm.Throws(ErrorException::class) suspend fun requestedTrack(): moq.TrackRequest
	fun consume(): moq.BroadcastConsumer
	fun datagrams(): moq.BroadcastDatagramRequests
	@kotlin.jvm.Throws(ErrorException::class) fun lister(): moq.BroadcastLister
	suspend fun closed()
	fun isClone(other: moq.BroadcastRequests): kotlin.Boolean
}
class BroadcastLister internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun pollWanted()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun pollUnwanted()
	@kotlin.jvm.Throws(ErrorException::class) fun add(name: kotlin.String, info: moq.TrackInfo)
	@kotlin.jvm.Throws(ErrorException::class) fun remove(kind: moq.BroadcastKind, name: kotlin.String)
	@kotlin.jvm.Throws(ErrorException::class) fun finish(kind: moq.BroadcastKind, name: kotlin.String)
	fun synced()
	fun clear()
	fun end()
	fun abort(err: moq.Error)
}
class BroadcastTracks internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun next(): moq.BroadcastTracksEvent?
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
		@kotlin.jvm.JvmStatic fun default(): moq.CacheConfig
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
		@kotlin.jvm.JvmStatic fun default(): moq.CachePool
		@kotlin.jvm.JvmStatic fun new(config: moq.CacheConfig): moq.CachePool
		@kotlin.jvm.JvmStatic fun unbounded(): moq.CachePool
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
	@kotlin.jvm.Throws(ErrorException::class) fun append(timestamp: moq.Timestamp, payload: moq.Bytes): kotlin.ULong
	@kotlin.jvm.Throws(ErrorException::class) fun insert(datagram: moq.DatagramDatagram)
	fun consume(): moq.DatagramConsumer
	fun finish()
	fun abort(err: moq.Error)
	fun priority(): kotlin.Byte?
	suspend fun priorityChanged(): kotlin.Byte?
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
	@kotlin.jvm.Throws(ErrorException::class) suspend fun info(): moq.DatagramInfo
	@kotlin.jvm.Throws(ErrorException::class) suspend fun recv(): moq.DatagramDatagram?
	fun priority(): kotlin.Byte
	fun setPriority(priority: kotlin.Byte)
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
	@kotlin.jvm.Throws(ErrorException::class) fun rescale(timestamp: moq.Timestamp): moq.Timestamp
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.DatagramInfo
	}
}
class FrameProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun group(): moq.GroupProducer
	fun remaining(): kotlin.ULong
	@kotlin.jvm.Throws(ErrorException::class) fun write(chunk: moq.Bytes)
	@kotlin.jvm.Throws(ErrorException::class) fun finish(): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun abort(err: moq.Error)
	val sizeGet: kotlin.ULong
	val timestampGet: moq.Timestamp
}
class FrameFrame internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.FrameFrame
	val timestampGet: moq.Timestamp
	fun timestampSet(value: moq.Timestamp)
	val payloadGet: moq.Bytes
	fun payloadSet(value: moq.Bytes)
	fun arrived(): moq.ClockInstant
	companion object {
		@kotlin.jvm.JvmStatic fun new(timestamp: moq.Timestamp, payload: moq.Bytes): moq.FrameFrame
	}
}
class FrameConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.FrameConsumer
	@kotlin.jvm.Throws(ErrorException::class) suspend fun readChunk(): moq.Bytes?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun readAll(): moq.Bytes
	val sizeGet: kotlin.ULong
	val timestampGet: moq.Timestamp
}
class GroupProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun timescale(): moq.Timescale?
	@kotlin.jvm.Throws(ErrorException::class) fun startAt(index: kotlin.ULong)
	@kotlin.jvm.Throws(ErrorException::class) fun writeFrame(timestamp: moq.Timestamp, data: moq.Bytes)
	@kotlin.jvm.Throws(ErrorException::class) fun createFrame(frame: moq.FrameInfo): moq.FrameProducer
	@kotlin.jvm.Throws(ErrorException::class) fun finishAt(end: kotlin.ULong): moq.GroupFinished
	@kotlin.jvm.Throws(ErrorException::class) fun finish(): moq.GroupFinished
	@kotlin.jvm.Throws(ErrorException::class) fun abort(err: moq.Error)
	suspend fun ready(frame: moq.FrameInfo)
	fun frameCount(): kotlin.ULong
	fun consume(): moq.GroupConsumer
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun demand(): moq.GroupDemand
	val startGet: kotlin.ULong
}
class GroupFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) fun abort(err: moq.Error)
	fun frameCount(): kotlin.ULong
	fun consume(): moq.GroupConsumer
	val startGet: kotlin.ULong
}
class GroupDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GroupDemand
	fun start(): kotlin.ULong
	fun wanted(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun wantedChanged(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
}
class GroupConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.GroupConsumer
	fun timescale(): moq.Timescale?
	fun arrived(): moq.ClockInstant
	fun index(): kotlin.ULong
	fun withRange(frames: moq.BoundsU64): moq.GroupConsumer
	fun skipTo(index: kotlin.ULong)
	fun frameCount(): kotlin.ULong
	fun end(): kotlin.ULong?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun nextFrame(): moq.FrameConsumer?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun readFrame(): moq.FrameFrame?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun finished(): kotlin.ULong
	val startGet: kotlin.ULong
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
		@kotlin.jvm.JvmStatic fun default(): moq.GroupFetch
	}
}
class TrackFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	fun requests(): moq.TrackRequests
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun abort(err: moq.Error)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun consume(): moq.TrackConsumer
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
}
class TrackRequests internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackRequests
	fun name(): kotlin.String
	@kotlin.jvm.Throws(ErrorException::class) suspend fun requestedFill(): moq.TrackFill
	fun demand(): moq.TrackDemand
}
class TrackDemand internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackDemand
	fun name(): kotlin.String
	fun wanted(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun wantedChanged(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun refresh()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
}
class TrackSubscriber internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun bill(): moq.StatsBill
	fun info(): moq.TrackInfo
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	fun control(): moq.TrackControl
	@kotlin.jvm.Throws(ErrorException::class) suspend fun recvGroup(): moq.GroupConsumer?
	fun ordered(): moq.TrackOrdered
	fun isClone(other: moq.TrackSubscriber): kotlin.Boolean
	suspend fun start(): kotlin.ULong?
	fun floor(): kotlin.ULong?
	fun nextInRange(from: kotlin.ULong): kotlin.ULong?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) suspend fun finished(): moq.TrackPosition
	fun withRange(groups: moq.BoundsU64): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription
	@kotlin.jvm.Throws(ErrorException::class) fun update(subscription: moq.TrackSubscription)
	suspend fun largest(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun refresh()
}
class TrackOrdered internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun info(): moq.TrackInfo
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	@kotlin.jvm.Throws(ErrorException::class) suspend fun nextGroup(): moq.TrackItem?
	fun control(): moq.TrackControl
	@kotlin.jvm.Throws(ErrorException::class) suspend fun refresh()
	fun subscription(): moq.TrackSubscription
	@kotlin.jvm.Throws(ErrorException::class) fun update(subscription: moq.TrackSubscription)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun finished(): moq.TrackPosition
	suspend fun largest(): moq.TrackPosition?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	fun isClone(other: moq.TrackOrdered): kotlin.Boolean
}
class TrackItem internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun group(): moq.GroupConsumer?
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
	val compressionGet: moq.TrackCompression
	fun compressionSet(value: moq.TrackCompression)
	val refreshGet: moq.TrackRefresh
	fun refreshSet(value: moq.TrackRefresh)
	fun withTimescale(timescale: moq.Timescale?): moq.TrackInfo
	fun withMaxAge(maxAge: moq.Duration?): moq.TrackInfo
	fun withPriority(priority: kotlin.Byte): moq.TrackInfo
	fun withCompression(compression: moq.TrackCompression): moq.TrackInfo
	fun withRefresh(refresh: moq.TrackRefresh): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun rescale(timestamp: moq.Timestamp): moq.Timestamp
	@kotlin.jvm.Throws(ErrorException::class) fun rescaleIn(timestamp: moq.Timestamp, key: kotlin.ULong): moq.Timestamp
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackInfo
	}
}
class TrackRefresh internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TrackRefresh
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val ratioGet: kotlin.UInt?
	fun ratioSet(value: kotlin.UInt?)
	val intervalGet: moq.Duration?
	fun intervalSet(value: moq.Duration?)
	fun withRatio(ratio: kotlin.UInt?): moq.TrackRefresh
	fun withInterval(interval: moq.Duration?): moq.TrackRefresh
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackRefresh
	}
}
class TrackProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackProducer
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun writeFrame(timestamp: moq.Timestamp, frame: moq.Bytes)
	@kotlin.jvm.Throws(ErrorException::class) fun finish(): moq.TrackFinished
	@kotlin.jvm.Throws(ErrorException::class) fun fail(err: moq.Error): moq.TrackFinished
	@kotlin.jvm.Throws(ErrorException::class) fun finishAt(end: moq.TrackPosition)
	@kotlin.jvm.Throws(ErrorException::class) fun startAt(key: kotlin.ULong?)
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) fun abort(err: moq.Error)
	@kotlin.jvm.Throws(TrackProducerException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun demand(): moq.TrackDemand
	fun consume(): moq.TrackConsumer
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun subscriptionChanged(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class TrackAbortHandle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackAbortHandle
	fun isClosed(): kotlin.Boolean
	fun abort(err: moq.Error)
}
class TrackConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackConsumer
	fun unmetered(): moq.TrackConsumer
	fun name(): kotlin.String
	fun broadcast(): moq.BroadcastInfo
	@kotlin.jvm.Throws(ErrorException::class) suspend fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	@kotlin.jvm.Throws(ErrorException::class) suspend fun fetchGroup(key: kotlin.ULong, options: moq.GroupFetch?): moq.GroupConsumer
	@kotlin.jvm.Throws(ErrorException::class) suspend fun info(): moq.TrackInfo
	suspend fun largest(): moq.TrackPosition?
}
class TrackControl internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TrackControl
	fun subscription(): moq.TrackSubscription
	@kotlin.jvm.Throws(ErrorException::class) fun update(subscription: moq.TrackSubscription)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun refresh()
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
	fun withOrder(order: moq.TrackOrder): moq.TrackSubscription
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TrackSubscription
	}
}
class TrackFill internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class OriginFollowing internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) suspend fun available(): moq.BroadcastConsumer?
}
class OriginClaim internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginClaim
	fun instance(): moq.Instance
	@kotlin.jvm.Throws(ErrorException::class) fun announce()
	fun withdraw()
	fun isAnnounced(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) fun setCost(cost: moq.OriginCost)
	@kotlin.jvm.Throws(ErrorException::class) fun setAuth(auth: moq.AuthCredential?)
	suspend fun requested(): moq.OriginRequest?
}
class OriginConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginConfig
	val nodeGet: kotlin.ULong
	fun nodeSet(value: kotlin.ULong)
	val maxHopsGet: kotlin.ULong
	fun maxHopsSet(value: kotlin.ULong)
	val poolGet: moq.CachePool
	fun poolSet(value: moq.CachePool)
	val cacheDurationGet: moq.Duration?
	fun cacheDurationSet(value: moq.Duration?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.OriginConfig
		@kotlin.jvm.JvmStatic fun new(node: kotlin.ULong): moq.OriginConfig
	}
}
class OriginConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginConsumer
	fun node(): kotlin.ULong
	fun withHidden(hidden: kotlin.Boolean): moq.OriginConsumer
	fun covering(): moq.OriginConsumer
	@kotlin.jvm.Throws(ErrorException::class) fun exclude(prefix: kotlin.String): moq.OriginConsumer
	fun withStats(session: moq.StatsSession): moq.OriginConsumer
	fun announced(): moq.AnnounceConsumer
	fun routes(): moq.OriginRoutes
	fun consume(): moq.OriginConsumer
	@kotlin.jvm.Throws(ErrorException::class) fun follow(path: kotlin.String, instance: moq.Instance?): moq.OriginFollowing
	@kotlin.jvm.Throws(ErrorException::class) fun scope(root: kotlin.String, patterns: moq.Patterns): moq.OriginConsumer
	@kotlin.jvm.Throws(ErrorException::class) suspend fun request(path: kotlin.String, instance: moq.Instance?): moq.BroadcastConsumer
	fun root(): moq.Path
	fun allowed(): moq.Patterns
	@kotlin.jvm.Throws(ErrorException::class) fun absolute(path: kotlin.String): moq.Path
}
class OriginDriver internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) fun poll(now: moq.ClockInstant, waiter: moq.Waiter): moq.ClockInstant?
}
class OriginProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginProducer
	fun withStats(session: moq.StatsSession): moq.OriginProducer
	fun withLink(link: moq.OriginLink): moq.OriginProducer
	fun withPriority(priority: moq.OriginPriority): moq.OriginProducer
	fun withProxy(proxy: kotlin.Boolean): moq.OriginProducer
	fun config(): moq.OriginConfig
	fun node(): kotlin.ULong
	fun isOwn(id: kotlin.ULong): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) fun route(path: kotlin.String, options: moq.OriginRouteOptions): moq.OriginRoute
	@kotlin.jvm.Throws(ErrorException::class) fun claim(prefix: kotlin.String, options: moq.OriginRouteOptions): moq.OriginClaim
	@kotlin.jvm.Throws(ErrorException::class) fun scope(root: kotlin.String, patterns: moq.Patterns): moq.OriginProducer
	@kotlin.jvm.Throws(ErrorException::class) fun exclude(prefix: kotlin.String): moq.OriginProducer
	@kotlin.jvm.Throws(ErrorException::class) fun mount(at: kotlin.String, target: kotlin.String): moq.OriginProducer
	fun consume(): moq.OriginConsumer
	fun root(): moq.Path
	fun allowed(): moq.Patterns
	@kotlin.jvm.Throws(ErrorException::class) fun absolute(path: kotlin.String): moq.Path
	companion object {
		@kotlin.jvm.JvmStatic fun new(config: moq.OriginConfig): kotlin.Pair<moq.OriginProducer, moq.OriginDriver>
	}
}
class OriginRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun path(): moq.Path
	@kotlin.jvm.Throws(ErrorException::class) fun route(options: moq.OriginRouteOptions): moq.OriginRoute
	fun serveProducer(broadcast: moq.BroadcastProducer)
	fun serveConsumer(broadcast: moq.BroadcastConsumer)
	fun reject(err: moq.Error)
}
class OriginRoute internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.OriginRoute
	fun broadcast(): moq.BroadcastProducer
	fun instance(): moq.Instance
	@kotlin.jvm.Throws(ErrorException::class) fun announce()
	fun withdraw()
	fun isAnnounced(): kotlin.Boolean
	@kotlin.jvm.Throws(ErrorException::class) fun setCost(cost: moq.OriginCost)
	@kotlin.jvm.Throws(ErrorException::class) fun setAuth(auth: moq.AuthCredential?)
}
class OriginRouteInfo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginRouteInfo
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun instance(): moq.Instance
	fun cost(): moq.OriginCost
	fun available(): kotlin.Boolean
}
class OriginRouteOptions internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginRouteOptions
	fun withCost(cost: moq.OriginCost): moq.OriginRouteOptions
	fun withAuth(auth: moq.AuthCredential): moq.OriginRouteOptions
	fun withInstance(instance: moq.Instance, epoch: kotlin.ULong): moq.OriginRouteOptions
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.OriginRouteOptions
		@kotlin.jvm.JvmStatic fun hidden(): moq.OriginRouteOptions
	}
}
class OriginRouteEvent internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.OriginRouteEvent
	val kindGet: moq.OriginRouteKind
	fun kindSet(value: moq.OriginRouteKind)
	val announceGet: moq.AnnounceAnnounce
	fun announceSet(value: moq.AnnounceAnnounce)
}
class OriginRoutes internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.OriginRouteEvent?
	fun tryNext(): moq.OriginRouteEvent?
	fun isClosed(): kotlin.Boolean
	suspend fun pollSynced()
	fun root(): moq.Path
	@kotlin.jvm.Throws(ErrorException::class) fun absolute(prefix: kotlin.String): moq.Path
}
class AnnounceConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	suspend fun next(): moq.AnnounceEvent?
	fun tryNext(): moq.AnnounceEvent?
	fun isClosed(): kotlin.Boolean
	suspend fun pollSynced()
	fun root(): moq.Path
	@kotlin.jvm.Throws(ErrorException::class) fun absolute(prefix: kotlin.String): moq.Path
}
class AnnounceEvent internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AnnounceEvent
}
class AnnounceAnnounce internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.AnnounceAnnounce
	val prefixGet: moq.Path
	fun prefixSet(value: moq.Path)
	val capturesGet: kotlin.collections.List<moq.Pattern>?
	fun capturesSet(value: kotlin.collections.List<moq.Pattern>?)
	val routeGet: moq.OriginRouteInfo
	fun routeSet(value: moq.OriginRouteInfo)
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangStatsSnapshot
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangStatsRendition
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangStatsTransport
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangEchoSnapshot
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
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangEchoRendition
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
		@kotlin.jvm.JvmStatic fun new(timestamp: moq.Timestamp, wall: moq.ClockWall): moq.HangEchoArrival
	}
}
class HangCatalogSkipped internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogSkipped
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val videoGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun videoSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	val audioGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun audioSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	val textGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun textSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	val jsonGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun jsonSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	val binaryGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun binarySet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, kotlin.ULong>>)
	fun isEmpty(): kotlin.Boolean
	fun version(): kotlin.ULong?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogSkipped
	}
}
class HangCatalogConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(HangErrorException::class) suspend fun pollNext(): moq.MoqJsonTimedCatalog?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackConsumer, delay: moq.Duration): moq.HangCatalogConsumer
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
	@kotlin.jvm.Throws(HangErrorException::class) fun insert(name: kotlin.String, config: moq.HangCatalogAudioConfig)
	fun remove(name: kotlin.String): moq.HangCatalogAudioConfig?
	fun isEmpty(): kotlin.Boolean
	fun enabled(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>
	fun ranked(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogAudioConfig>>
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogAudio
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
		@kotlin.jvm.JvmStatic fun new(codec: moq.HangCatalogAudioCodec, sampleRate: kotlin.UInt, channelCount: kotlin.UInt): moq.HangCatalogAudioConfig
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
	@kotlin.jvm.Throws(HangErrorException::class) fun insert(name: kotlin.String, config: moq.HangCatalogBinaryConfig)
	fun remove(name: kotlin.String): moq.HangCatalogBinaryConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogBinary
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
		@kotlin.jvm.JvmStatic fun new(mode: moq.HangCatalogMode): moq.HangCatalogBinaryConfig
	}
}
class HangCatalogContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogContainer
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogContainer
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
		@kotlin.jvm.JvmStatic fun new(path: moq.PathRelative): moq.HangCatalogEcho
	}
}
class HangCatalogInclude internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogInclude
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val broadcastGet: moq.PathRelative
	fun broadcastSet(value: moq.PathRelative)
	val instanceGet: moq.Instance?
	fun instanceSet(value: moq.Instance?)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.PathRelative): moq.HangCatalogInclude
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
	@kotlin.jvm.Throws(HangErrorException::class) fun insert(name: kotlin.String, config: moq.HangCatalogJsonConfig)
	fun remove(name: kotlin.String): moq.HangCatalogJsonConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogJson
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
		@kotlin.jvm.JvmStatic fun new(mode: moq.HangCatalogMode): moq.HangCatalogJsonConfig
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
		@kotlin.jvm.JvmStatic fun new(track: kotlin.String): moq.HangCatalogStats
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
	@kotlin.jvm.Throws(HangErrorException::class) fun insert(name: kotlin.String, config: moq.HangCatalogTextConfig)
	fun remove(name: kotlin.String): moq.HangCatalogTextConfig?
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogText
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
		@kotlin.jvm.JvmStatic fun new(format: moq.HangCatalogTextFormat): moq.HangCatalogTextConfig
	}
}
class HangCatalogTimeline internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogTimeline
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val clockGet: moq.HangCatalogClock?
	fun clockSet(value: moq.HangCatalogClock?)
	val intervalGet: moq.Duration?
	fun intervalSet(value: moq.Duration?)
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogTimeline
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
	val rangeGet: moq.HangCatalogColorRange?
	fun rangeSet(value: moq.HangCatalogColorRange?)
	val masteringGet: moq.HangCatalogMastering?
	fun masteringSet(value: moq.HangCatalogMastering?)
	val contentLightGet: moq.HangCatalogContentLight?
	fun contentLightSet(value: moq.HangCatalogContentLight?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogColor
	}
}
class HangCatalogProjection internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.HangCatalogProjection
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogProjection
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
	@kotlin.jvm.Throws(HangErrorException::class) fun insert(name: kotlin.String, config: moq.HangCatalogVideoConfig)
	fun remove(name: kotlin.String): moq.HangCatalogVideoConfig?
	fun isEmpty(): kotlin.Boolean
	fun enabled(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>
	fun ranked(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogVideoConfig>>
	@kotlin.jvm.Throws(HangErrorException::class) fun setProperties(properties: moq.HangCatalogVideoProperties)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogVideo
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
		@kotlin.jvm.JvmStatic fun default(): moq.HangCatalogVideoProperties
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
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	fun sameIdentity(other: moq.HangCatalogVideoConfig): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(codec: moq.HangCatalogVideoCodec): moq.HangCatalogVideoConfig
	}
}
class MuxCatalogConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumer
	}
}
class MuxCatalogConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExtra
	}
}
class MuxCatalogConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) suspend fun new(broadcast: moq.BroadcastConsumer, format: moq.MuxCatalogCatalogFormat): moq.MuxCatalogConsumerExt
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCatalogEstimate
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCatalogConfig
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
}
class MuxCatalogGuardExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setSection(name: kotlin.String, value: moq.Json)
	fun removeSection(name: kotlin.String): moq.Json?
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val extGet: moq.MuxCatalogHangExtra
	fun extSet(value: moq.MuxCatalogHangExtra)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
}
class MuxCatalogGuardExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
	val videoGet: moq.HangCatalogVideo
	fun videoSet(value: moq.HangCatalogVideo)
	val audioGet: moq.HangCatalogAudio
	fun audioSet(value: moq.HangCatalogAudio)
	val syncGet: moq.HangCatalogTimeline
	fun syncSet(value: moq.HangCatalogTimeline)
	val textGet: moq.HangCatalogText
	fun textSet(value: moq.HangCatalogText)
	val jsonGet: moq.HangCatalogJson
	fun jsonSet(value: moq.HangCatalogJson)
	val binaryGet: moq.HangCatalogBinary
	fun binarySet(value: moq.HangCatalogBinary)
	val statsGet: moq.HangCatalogStats?
	fun statsSet(value: moq.HangCatalogStats?)
	val echoGet: moq.HangCatalogEcho?
	fun echoSet(value: moq.HangCatalogEcho?)
	val includeGet: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>
	fun includeSet(value: kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangCatalogInclude>>)
	val extGet: moq.MuxContainerTsExt
	fun extSet(value: moq.MuxContainerTsExt)
	val skippedGet: moq.HangCatalogSkipped
	fun skippedSet(value: moq.HangCatalogSkipped)
	@kotlin.jvm.Throws(HangErrorException::class) fun checkRenditions()
	fun withoutExt(): moq.HangCatalog
	@kotlin.jvm.Throws(HangErrorException::class) fun toJson(): kotlin.String
	@kotlin.jvm.Throws(HangErrorException::class) fun toVec(): moq.Bytes
	@kotlin.jvm.Throws(HangErrorException::class) fun toWriter(writer: moq.Bytes)
}
class MuxCatalogProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducer
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	@kotlin.jvm.Throws(MuxErrorException::class) fun mutate(f: (moq.HangCatalog) -> kotlin.Unit)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxCatalogGuard
	fun snapshot(): moq.HangCatalog
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebase
	fun reserve(): moq.MuxCatalogReserved
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(ErrorException::class) fun consume(): moq.MuxCatalogHangConsumer
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfig): moq.MuxCatalogProducer
	}
}
class MuxCatalogProducerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducerExtra
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	@kotlin.jvm.Throws(MuxErrorException::class) fun mutate(f: (moq.HangCatalogExtra) -> kotlin.Unit)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxCatalogGuardExtra
	fun snapshot(): moq.HangCatalogExtra
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebaseExtra
	fun reserve(): moq.MuxCatalogReservedExtra
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(ErrorException::class) fun consume(): moq.MuxCatalogHangConsumerExtra
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfigExtra): moq.MuxCatalogProducerExtra
	}
}
class MuxCatalogProducerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogProducerExt
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
	@kotlin.jvm.Throws(MuxErrorException::class) fun mutate(f: (moq.HangCatalogExt) -> kotlin.Unit)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxCatalogGuardExt
	fun snapshot(): moq.HangCatalogExt
	fun clock(): moq.MuxClock
	fun timebase(): moq.MuxCatalogTimebaseExt
	fun reserve(): moq.MuxCatalogReservedExt
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	fun isClaimedJsonConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedBinaryConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedVideoConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedAudioConfig(name: kotlin.String): kotlin.Boolean
	fun isClaimedTextConfig(name: kotlin.String): kotlin.Boolean
	fun stats(): kotlin.collections.List<kotlin.Pair<kotlin.String, moq.HangStatsRendition>>
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonSnapshotJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig, options: moq.MuxJsonOptions): moq.MuxJsonSnapshotJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamConfig(track: moq.TrackProducer, config: moq.MuxJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(MuxErrorException::class) fun jsonStreamJsonConfig(track: moq.TrackProducer, config: moq.HangCatalogJsonConfig): moq.MuxJsonStreamJson
	@kotlin.jvm.Throws(ErrorException::class) fun consume(): moq.MuxCatalogHangConsumerExt
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, config: moq.MuxCatalogConfigExt): moq.MuxCatalogProducerExt
	}
}
class MuxCatalogTimebase internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebase
	fun reserve(): moq.MuxCatalogReserved
	@kotlin.jvm.Throws(MuxErrorException::class) fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogTimebaseExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebaseExtra
	fun reserve(): moq.MuxCatalogReservedExtra
	@kotlin.jvm.Throws(MuxErrorException::class) fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogTimebaseExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogTimebaseExt
	fun reserve(): moq.MuxCatalogReservedExt
	@kotlin.jvm.Throws(MuxErrorException::class) fun place(pts: moq.Timestamp, wall: moq.ClockWall)
}
class MuxCatalogReserved internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReserved
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReserved
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
}
class MuxCatalogReservedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReservedExtra
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReservedExtra
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
}
class MuxCatalogReservedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxCatalogReservedExt
	fun withInterval(interval: moq.Duration?): moq.MuxCatalogReservedExt
	fun trackInfo(priority: kotlin.Byte): moq.TrackInfo
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun videoLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun audioLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textContainer(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textFmp4Wire(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLegacyWire(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun textLocWire(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerJsonConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerBinaryConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerVideoConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerAudioConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackContainerTextConfig(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackFmp4WireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLegacyWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireJsonConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogJsonConfig?): moq.MuxContainerProducerLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireBinaryConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogBinaryConfig?): moq.MuxContainerProducerLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireVideoConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogVideoConfig?): moq.MuxContainerProducerLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireAudioConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogAudioConfig?): moq.MuxContainerProducerLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun trackLocWireTextConfig(track: moq.TrackProducer, container: moq.MuxContainerLocWire, config: moq.HangCatalogTextConfig?): moq.MuxContainerProducerLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCatalogVideoHint
	}
}
class MuxContainerConsumerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun arrival(): moq.ClockInstant?
	fun backlog(): moq.ClockInstant?
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerRead?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	fun setDelay(delay: moq.TrackDelay)
	fun setPlayhead(playhead: moq.MuxSyncConsumer)
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber, format: moq.MuxCatalogHangContainer): moq.MuxContainerConsumerContainer
	}
}
class MuxContainerConsumerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun arrival(): moq.ClockInstant?
	fun backlog(): moq.ClockInstant?
	@kotlin.jvm.Throws(MuxContainerFmp4ErrorException::class) suspend fun read(): moq.MuxContainerRead?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	fun setDelay(delay: moq.TrackDelay)
	fun setPlayhead(playhead: moq.MuxSyncConsumer)
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber, format: moq.MuxContainerFmp4Wire): moq.MuxContainerConsumerFmp4Wire
	}
}
class MuxContainerConsumerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun arrival(): moq.ClockInstant?
	fun backlog(): moq.ClockInstant?
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerRead?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	fun setDelay(delay: moq.TrackDelay)
	fun setPlayhead(playhead: moq.MuxSyncConsumer)
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber, format: moq.MuxContainerLegacyWire): moq.MuxContainerConsumerLegacyWire
	}
}
class MuxContainerConsumerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun arrival(): moq.ClockInstant?
	fun backlog(): moq.ClockInstant?
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerRead?
	fun isSkipped(groups: moq.RangeU64): kotlin.Boolean
	fun setDelay(delay: moq.TrackDelay)
	fun setPlayhead(playhead: moq.MuxSyncConsumer)
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber, format: moq.MuxContainerLocWire): moq.MuxContainerConsumerLocWire
	}
}
class MuxContainerGroupConsumerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun start(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic fun new(group: moq.GroupConsumer, format: moq.MuxCatalogHangContainer): moq.MuxContainerGroupConsumerContainer
	}
}
class MuxContainerGroupConsumerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun start(): kotlin.ULong
	@kotlin.jvm.Throws(MuxContainerFmp4ErrorException::class) suspend fun read(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic fun new(group: moq.GroupConsumer, format: moq.MuxContainerFmp4Wire): moq.MuxContainerGroupConsumerFmp4Wire
	}
}
class MuxContainerGroupConsumerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun start(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic fun new(group: moq.GroupConsumer, format: moq.MuxContainerLegacyWire): moq.MuxContainerGroupConsumerLegacyWire
	}
}
class MuxContainerGroupConsumerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun start(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic fun new(group: moq.GroupConsumer, format: moq.MuxContainerLocWire): moq.MuxContainerGroupConsumerLocWire
	}
}
class MuxContainerFinishedContainerAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedContainerBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedContainerJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedContainerTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedContainerVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4WireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4WireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4WireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4WireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4WireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerFinishedLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun info(): moq.TrackInfo
	fun consume(): moq.TrackSubscriber
	fun estimate(): moq.MuxCatalogEstimate
	fun abort(err: moq.Error)
}
class MuxContainerGuardUnit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
}
class MuxContainerGuardAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
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
}
class MuxContainerGuardBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
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
}
class MuxContainerGuardJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
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
}
class MuxContainerGuardTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
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
}
class MuxContainerGuardVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun commit()
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
	val delayGet: moq.Duration?
	fun delaySet(value: moq.Duration?)
	fun sameIdentity(other: moq.HangCatalogVideoConfig): kotlin.Boolean
}
class MuxContainerProducerContainerAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogAudioConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainerAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerAudioConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogBinaryConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainerBinaryConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerBinaryConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogJsonConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainerJsonConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerJsonConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogTextConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainerTextConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerTextConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainerVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogVideoConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerVideoConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardUnit
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainer
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedContainer
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerContainerException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackProducer, container: moq.MuxCatalogHangContainer): moq.MuxContainerProducerContainer
	}
}
class MuxContainerProducerFmp4WireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogAudioConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4WireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4WireAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireAudioConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogBinaryConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4WireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4WireBinaryConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireBinaryConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogJsonConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4WireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4WireJsonConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireJsonConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogTextConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4WireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4WireTextConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireTextConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4WireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogVideoConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4WireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4WireVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireVideoConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardUnit
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedFmp4Wire
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedFmp4Wire
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerFmp4WireException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackProducer, container: moq.MuxContainerFmp4Wire): moq.MuxContainerProducerFmp4Wire
	}
}
class MuxContainerProducerLegacyWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogAudioConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWireAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireAudioConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogBinaryConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWireBinaryConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireBinaryConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogJsonConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWireJsonConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireJsonConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogTextConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWireTextConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireTextConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogVideoConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWireVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireVideoConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLegacyWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardUnit
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLegacyWire
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLegacyWire
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLegacyWireException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackProducer, container: moq.MuxContainerLegacyWire): moq.MuxContainerProducerLegacyWire
	}
}
class MuxContainerProducerLocWireAudioConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogAudioConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWireAudioConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWireAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireAudioConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireBinaryConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogBinaryConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWireBinaryConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWireBinaryConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireBinaryConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireJsonConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogJsonConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWireJsonConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWireJsonConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireJsonConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireTextConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogTextConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWireTextConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWireTextConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireTextConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWireVideoConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(config: moq.HangCatalogVideoConfig)
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWireVideoConfig
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWireVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireVideoConfigException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
}
class MuxContainerProducerLocWire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun set()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setBitrate(bitrate: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun modify(): moq.MuxContainerGuardUnit
	@kotlin.jvm.Throws(MuxErrorException::class) fun timestamp(hint: moq.Timestamp?): moq.Timestamp
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun write(frame: moq.MuxContainerFrame)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedLocWire
	@kotlin.jvm.Throws(MuxErrorException::class) fun fail(err: moq.Error): moq.MuxContainerFinishedLocWire
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxContainerProducerLocWireException::class) fun abortUnwanted(err: moq.Error)
	fun consume(): moq.TrackSubscriber
	fun broadcast(): moq.BroadcastInfo
	fun info(): moq.TrackInfo
	@kotlin.jvm.Throws(ErrorException::class) fun createGroup(group: moq.GroupInfo): moq.GroupProducer
	@kotlin.jvm.Throws(ErrorException::class) fun appendGroup(start: moq.Timestamp): moq.GroupProducer
	fun skipGroups(groups: moq.RangeU64)
	fun finished(): moq.TrackPosition?
	@kotlin.jvm.Throws(ErrorException::class) suspend fun closed()
	fun isClosed(): kotlin.Boolean
	fun largest(): moq.TrackPosition?
	fun isClone(other: moq.TrackProducer): kotlin.Boolean
	fun abortHandle(): moq.TrackAbortHandle
	fun subscribe(subscription: moq.TrackSubscription?): moq.TrackSubscriber
	fun subscription(): moq.TrackSubscription?
	fun requests(): moq.TrackRequests
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackProducer, container: moq.MuxContainerLocWire): moq.MuxContainerProducerLocWire
	}
}
class MuxFollowInstances internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.BroadcastConsumer?
}
class MuxImportContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxImportContainerFinished
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved, init: moq.MuxImportContainerInit): moq.MuxImportContainer
	}
}
class MuxImportContainerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxImportContainerFinishedExtra
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportContainerInit): moq.MuxImportContainerExtra
	}
}
class MuxImportContainerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxImportContainerFinishedExt
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportContainerInit): moq.MuxImportContainerExt
	}
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
		@kotlin.jvm.JvmStatic fun new(format: moq.MuxImportAudioFormat, data: moq.Bytes): moq.MuxImportAudioInit
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
		@kotlin.jvm.JvmStatic fun new(format: moq.MuxImportContainerFormat, data: moq.Bytes): moq.MuxImportContainerInit
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
		@kotlin.jvm.JvmStatic fun new(format: moq.MuxImportVideoFormat, data: moq.Bytes): moq.MuxImportVideoInit
	}
}
class MuxImportTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxImportTrackFinished
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxImportTrackException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	fun demand(): moq.TrackDemand
	fun name(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun audioExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun audioExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun audioUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportAudioInit): moq.MuxImportTrack
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportVideoInit): moq.MuxImportTrack
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportVideoInit): moq.MuxImportTrack
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportVideoInit): moq.MuxImportTrack
	}
}
class MuxImportTrackStream internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxImportTrackFinished
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxImportTrackStreamException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	fun demand(): moq.TrackDemand
	fun name(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoExtra(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExtra, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoExt(request: moq.TrackRequest, reserved: moq.MuxCatalogReservedExt, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun videoUnit(request: moq.TrackRequest, reserved: moq.MuxCatalogReserved, init: moq.MuxImportVideoInit): moq.MuxImportTrackStream
	}
}
class MuxJsonSnapshotJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun consume(): moq.TrackSubscriber
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun refresh()
	@kotlin.jvm.Throws(MuxErrorException::class) fun update(value: moq.Json, timestamp: moq.Timestamp)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish()
	@kotlin.jvm.Throws(MuxErrorException::class) fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxJsonSnapshotJsonException::class) fun abortUnwanted(err: moq.Error)
}
class MuxJsonStreamJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun consume(): moq.TrackSubscriber
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun append(value: moq.Json, timestamp: moq.Timestamp)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish()
	@kotlin.jvm.Throws(MuxErrorException::class) fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxJsonStreamJsonException::class) fun abortUnwanted(err: moq.Error)
}
class MuxJsonConsumerJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun mode(): moq.HangCatalogMode
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MoqJsonTimedJson?
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun latest(): moq.MoqJsonTimedJson?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) suspend fun fromTrack(track: moq.TrackConsumer, config: moq.HangCatalogJsonConfig): moq.MuxJsonConsumerJson
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxJsonConfig
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxJsonOptions
	}
}
class MuxRateGrant internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun assume(target: moq.MuxRateTarget)
	fun policy(): moq.MuxRatePolicy
	fun setPolicy(policy: moq.MuxRatePolicy)
	@kotlin.jvm.Throws(ErrorException::class) suspend fun next(): moq.MuxRateTarget
	fun failed()
	companion object {
		@kotlin.jvm.JvmStatic fun new(grant: moq.BandwidthConsumer, policy: moq.MuxRatePolicy): moq.MuxRateGrant
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
		@kotlin.jvm.JvmStatic fun new(max: moq.BandwidthRate): moq.MuxRatePolicy
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
		@kotlin.jvm.JvmStatic fun new(policy: moq.MuxRatePolicy): moq.MuxRateControl
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
	fun retainExtra(catalog: moq.HangCatalogExtra)
	fun retainExt(catalog: moq.HangCatalogExt)
	fun retainUnit(catalog: moq.HangCatalog)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxSelectBroadcast
	}
}
class MuxSelectVideo internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSelectVideo
	fun name(name: kotlin.String): moq.MuxSelectVideo
	fun codec(codec: moq.HangCatalogVideoCodecKind): moq.MuxSelectVideo
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxSelectVideo
	}
}
class MuxSelectAudio internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSelectAudio
	fun name(name: kotlin.String): moq.MuxSelectAudio
	fun codec(codec: moq.HangCatalogAudioCodecKind): moq.MuxSelectAudio
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxSelectAudio
	}
}
class MuxSyncTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun received(timestamp: moq.Timestamp, now: moq.ClockInstant): kotlin.Boolean
	fun reset()
	fun setTarget(target: moq.Duration): kotlin.Boolean
	suspend fun delayChanged(seen: moq.Duration): moq.Duration
}
class MuxSyncSchedule internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxSyncSchedule
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun delay(): moq.Duration
	fun due(timestamp: moq.Timestamp): moq.ClockInstant?
}
class MuxSyncProducer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxSyncProducer
	fun reset(delay: moq.Duration): moq.MuxSyncProducer
	fun consume(): moq.MuxSyncConsumer
	fun track(kind: moq.MuxContainerKind): moq.MuxSyncTrack
	fun due(timestamp: moq.Timestamp): moq.ClockInstant?
	companion object {
		@kotlin.jvm.JvmStatic fun new(delay: moq.Duration): moq.MuxSyncProducer
	}
}
class MuxSyncConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.MuxSyncConsumer
	fun delay(): moq.Duration
	fun due(timestamp: moq.Timestamp): moq.ClockInstant?
	fun position(at: moq.ClockInstant): moq.Duration?
	fun schedule(): moq.MuxSyncSchedule
	suspend fun changed(seen: moq.MuxSyncSchedule): moq.MuxSyncSchedule
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
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TokioTlsErrorException::class) fun generate(hostnames: kotlin.collections.List<kotlin.String>): moq.TokioTlsIdentity
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
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun build(): moq.RustlsClientConfig
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioTlsConnect
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
	val rootGet: kotlin.collections.List<moq.PathBuf>
	fun rootSet(value: kotlin.collections.List<moq.PathBuf>)
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun load(): moq.TokioTlsServe
	fun deprecated(): moq.MoqTokioCliDeprecated
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun loadRoots(): moq.RustlsRootCertStore
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun serverConfig(alpn: kotlin.collections.List<moq.Bytes>): moq.RustlsServerConfig
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioTlsListen
	}
}
class TokioTlsServe internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsServe
	fun certificates(): moq.TokioTlsCertificates
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun config(alpn: kotlin.collections.List<moq.Bytes>): moq.RustlsServerConfig
	fun peer(chain: kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>): moq.TokioTlsPeerIdentity
}
class TokioTlsPeerIdentity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsPeerIdentity
	fun chain(): kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>
	fun fingerprint(): kotlin.String?
	fun name(): kotlin.String?
	@kotlin.jvm.Throws(TokioTlsErrorException::class) fun lineage(): kotlin.collections.List<kotlin.String>
	fun issuer(): kotlin.String?
	fun expiry(): moq.ClockWall?
}
class TokioTlsCertificates internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.TokioTlsCertificates
	fun fingerprints(): kotlin.collections.List<kotlin.String>
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TokioTlsErrorException::class) fun fromPem(chain: moq.Bytes): moq.TokioTlsCertificates
	}
}
class TokioTlsKeyLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(TokioTlsErrorException::class) fun open(path: moq.PathBuf): moq.TokioTlsKeyLog
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
	@kotlin.jvm.Throws(TokioErrorException::class) fun init(): moq.TokioClient
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioClientConfig
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
	val implementationGet: kotlin.String?
	fun implementationSet(value: kotlin.String?)
	val tlsGet: moq.TokioTlsConnect
	fun tlsSet(value: moq.TokioTlsConnect)
	val backoffGet: moq.TokioBackoff
	fun backoffSet(value: moq.TokioBackoff)
	val goawayGet: moq.MoqTokioConnectionHandover
	fun goawaySet(value: moq.MoqTokioConnectionHandover)
	val websocketGet: moq.MoqTokioWebsocketConfig
	fun websocketSet(value: moq.MoqTokioWebsocketConfig)
	fun deprecated(): moq.MoqTokioCliDeprecated
	@kotlin.jvm.Throws(TokioErrorException::class) fun init(quic: moq.TokioQuicConfig): moq.TokioClient
	fun versions(): moq.Versions
	fun resolve(): moq.MoqTokioConnectResolved
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioConnectConfig
	}
}
class TokioConnectionMonitor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TokioConnectionMonitor
	fun state(): moq.MoqTokioConnectionObservation
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun stateChanged(after: kotlin.ULong): moq.MoqTokioConnectionObservation
	fun presence(): moq.StatsPresence
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun pollPresence(): moq.StatsPresence
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun presenceChanged(): moq.StatsPresence
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
	@kotlin.jvm.Throws(TokioErrorException::class) fun resolve(): moq.MoqTokioListenResolved
	@kotlin.jvm.Throws(TokioErrorException::class) fun init(quic: moq.TokioQuicConfig): moq.TokioServer
	@kotlin.jvm.Throws(TokioErrorException::class) fun initStreams(): moq.TokioServer
	fun versions(): moq.Versions
	fun server(): moq.Server
	fun hasExplicitBind(): kotlin.Boolean
	fun servesQuic(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioListenConfig
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
	@kotlin.jvm.Throws(TokioErrorException::class) fun validate()
	fun resolve(): moq.MoqTokioQuicResolved
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioQuicConfig
	}
}
class TokioServerConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val tlsGet: moq.TokioTlsServe?
	fun tlsSet(value: moq.TokioTlsServe?)
	val admissionGet: moq.ServerAdmission
	fun admissionSet(value: moq.ServerAdmission)
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
	val streamsGet: moq.MoqTokioServerStreams
	fun streamsSet(value: moq.MoqTokioServerStreams)
	@kotlin.jvm.Throws(TokioErrorException::class) fun init(): moq.TokioServer
	@kotlin.jvm.Throws(TokioErrorException::class) fun initStreams(): moq.TokioServer
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TokioServerConfig
	}
}
class TokioServerListener internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun accept(): moq.TokioServerRequest
	suspend fun close_()
	suspend fun closeStreams()
	@kotlin.jvm.Throws(TokioErrorException::class) fun localAddr(): moq.SocketAddr
	fun websocketLocalAddr(): moq.SocketAddr?
	fun tcpLocalAddr(): moq.SocketAddr?
	fun certificates(): moq.TokioTlsCertificates
	fun acceptHealth(): kotlin.collections.List<moq.MoqTokioAcceptHealth>
}
class TokioServerRequest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun reject(code: moq.SessionError)
	fun withPublisherProducer(publish: moq.OriginProducer): moq.TokioServerRequest
	fun withPublisherConsumer(publish: moq.OriginConsumer): moq.TokioServerRequest
	fun withSubscriber(subscribe: moq.OriginProducer): moq.TokioServerRequest
	fun withPeerNode(node: kotlin.ULong): moq.TokioServerRequest
	fun withStats(stats: moq.StatsSession): moq.TokioServerRequest
	fun withLimits(limits: moq.SessionLimits): moq.TokioServerRequest
	fun quicLimits(): moq.MoqTokioServerQuicLimits?
	@kotlin.jvm.Throws(StringException::class) fun withQuicLimits(limits: moq.MoqQuicLimits): moq.TokioServerRequest
	suspend fun expired()
	@kotlin.jvm.Throws(TokioErrorException::class) suspend fun ok(): moq.Session
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
	fun peerNode(): kotlin.ULong?
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
	val latencyGet: moq.Duration
	fun latencySet(value: moq.Duration)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.TransportMemoryConfig
	}
}
class TransportMemorySession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TransportMemorySession
	fun closer(): moq.TransportMemoryCloser
}
class TransportMemoryCloser internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): moq.TransportMemoryCloser
	fun close_(code: kotlin.UInt, reason: kotlin.String)
	fun isClosed(): kotlin.Boolean
}
class MuxCatalogHangConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalog?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackConsumer): moq.MuxCatalogHangConsumer
	}
}
class MuxCatalogHangConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalogExtra?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackConsumer): moq.MuxCatalogHangConsumerExtra
	}
}
class MuxCatalogHangConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalogExt?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackConsumer): moq.MuxCatalogHangConsumerExt
	}
}
class MuxCatalogHangContainer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(container: moq.HangCatalogContainer, kind: moq.MuxContainerKind): moq.MuxCatalogHangContainer
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun set(name: kotlin.String, value: moq.Json)
	fun remove(name: kotlin.String): moq.Json?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCatalogHangExtra
	}
}
class MuxCatalogMsfConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalog?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumer
	}
}
class MuxCatalogMsfConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalogExtra?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumerExtra
	}
}
class MuxCatalogMsfConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.HangCatalogExt?
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: moq.TrackSubscriber): moq.MuxCatalogMsfConsumerExt
	}
}
class MuxCodecAacConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val profileGet: kotlin.UByte
	fun profileSet(value: kotlin.UByte)
	val sampleRateGet: kotlin.UInt
	fun sampleRateSet(value: kotlin.UInt)
	val channelCountGet: kotlin.UInt
	fun channelCountSet(value: kotlin.UInt)
	@kotlin.jvm.Throws(MuxCodecAacErrorException::class) fun encode(): moq.Bytes
	companion object {
		@kotlin.jvm.JvmStatic fun new(profile: kotlin.UByte, sampleRate: kotlin.UInt, channelCount: kotlin.UInt): moq.MuxCodecAacConfig
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecAacErrorException::class) fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecAacConfig
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecAacImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecAacImport
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
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(buf: moq.Bytes)
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecAv1ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecAv1Import
	}
}
class MuxCodecAv1Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	@kotlin.jvm.Throws(MuxErrorException::class) fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCodecAv1Split
		@kotlin.jvm.JvmStatic fun new(): moq.MuxCodecAv1Split
	}
}
class MuxCodecFlacImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecFlacImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecFlacImport
	}
}
class MuxCodecH264Avc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun avcc(): moq.Bytes?
	@kotlin.jvm.Throws(MuxCodecH264ErrorException::class) fun transform(payload: moq.Bytes): moq.Bytes?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCodecH264Avc1
		@kotlin.jvm.JvmStatic fun new(): moq.MuxCodecH264Avc1
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
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecH264ErrorException::class) fun parse(avcc: moq.Bytes): moq.MuxCodecH264Avcc
	}
}
class MuxCodecH264Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(buf: moq.Bytes)
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun track(): moq.TrackProducer
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecH264ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun observeReorder(reorder: moq.Timestamp)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH264Import
	}
}
class MuxCodecH264Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	@kotlin.jvm.Throws(MuxErrorException::class) fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCodecH264Split
		@kotlin.jvm.JvmStatic fun new(): moq.MuxCodecH264Split
	}
}
class MuxCodecH265Hvc1 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun hvcc(): moq.Bytes?
	@kotlin.jvm.Throws(MuxCodecH265ErrorException::class) fun transform(payload: moq.Bytes): moq.Bytes?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCodecH265Hvc1
		@kotlin.jvm.JvmStatic fun new(): moq.MuxCodecH265Hvc1
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
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecH265ErrorException::class) fun parse(hvcc: moq.Bytes): moq.MuxCodecH265Hvcc
	}
}
class MuxCodecH265Sps internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxCodecH265Sps
	fun size(): kotlin.Pair<kotlin.UInt, kotlin.UInt>
	fun videoSignal(): moq.MuxCodecH265VideoSignal?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecH265ErrorException::class) fun parse(nal: moq.Bytes): moq.MuxCodecH265Sps
	}
}
class MuxCodecH265Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(buf: moq.Bytes)
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	fun track(): moq.TrackProducer
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecH265ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	fun reportSkipped(frames: kotlin.ULong)
	fun reportBitrate(bitrate: moq.BandwidthRate)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun observeReorder(reorder: moq.Timestamp)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frames: kotlin.collections.List<moq.MuxContainerFrame>)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecH265Import
	}
}
class MuxCodecH265Split internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes, pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	@kotlin.jvm.Throws(MuxErrorException::class) fun flush(pts: moq.Timestamp?): kotlin.collections.List<moq.MuxContainerFrame>
	fun reset()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxCodecH265Split
		@kotlin.jvm.JvmStatic fun new(): moq.MuxCodecH265Split
	}
}
class MuxCodecMp3Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecMp3ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecMp3Import
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
	@kotlin.jvm.Throws(MuxCodecOpusErrorException::class) fun encode(): moq.Bytes
	companion object {
		@kotlin.jvm.JvmStatic fun new(sampleRate: kotlin.UInt, channelCount: kotlin.UInt): moq.MuxCodecOpusConfig
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecOpusErrorException::class) fun parse(buf: moq.Mut<moq.Bytes>): moq.MuxCodecOpusConfig
	}
}
class MuxCodecOpusImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerAudioConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecOpusImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, config: moq.HangCatalogAudioConfig): moq.MuxCodecOpusImport
	}
}
class MuxCodecVp8Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(buf: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecVp8ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp8Import
	}
}
class MuxCodecVp9Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun initialize(buf: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(frame: moq.Bytes, pts: moq.Timestamp?)
	fun demand(): moq.TrackDemand
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFinishedContainerVideoConfig
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxCodecVp9ImportException::class) fun abortUnwanted(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut(end: moq.Timestamp?)
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	@kotlin.jvm.Throws(MuxErrorException::class) fun setEnabled(enabled: kotlin.Boolean)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExtra(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExtra, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newExt(track: moq.TrackProducer, reserved: moq.MuxCatalogReservedExt, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun newUnit(track: moq.TrackProducer, reserved: moq.MuxCatalogReserved, hint: moq.MuxCatalogVideoHint): moq.MuxCodecVp9Import
	}
}
class MuxContainerFlvImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImport
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFlvFinished
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerFlvImport
	}
}
class MuxContainerFlvImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImportExtra
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFlvFinishedExtra
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerFlvImportExtra
	}
}
class MuxContainerFlvImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerFlvImportExt
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFlvFinishedExt
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerFlvImportExt
	}
}
class MuxContainerFlvFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFlvFinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFlvFinishedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFmp4Wire internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxContainerFmp4ErrorException::class) fun fromInit(initData: moq.Bytes): moq.MuxContainerFmp4Wire
	}
}
class MuxContainerFmp4Error internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerFmp4Error
}
class MuxContainerFmp4Import internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4Import
	fun withCmaf(cmaf: kotlin.Boolean): moq.MuxContainerFmp4Import
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFmp4Finished
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun rejoin()
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerFmp4Import
	}
}
class MuxContainerFmp4ImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4ImportExtra
	fun withCmaf(cmaf: kotlin.Boolean): moq.MuxContainerFmp4ImportExtra
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFmp4FinishedExtra
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun rejoin()
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerFmp4ImportExtra
	}
}
class MuxContainerFmp4ImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxErrorException::class) fun cut()
	fun withSelect(select: moq.MuxSelectBroadcast): moq.MuxContainerFmp4ImportExt
	fun withCmaf(cmaf: kotlin.Boolean): moq.MuxContainerFmp4ImportExt
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerFmp4FinishedExt
	fun abort(err: moq.Error)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun rejoin()
	@kotlin.jvm.Throws(MuxErrorException::class) fun discontinuity()
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerFmp4ImportExt
	}
}
class MuxContainerFmp4Finished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFmp4FinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFmp4FinishedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerFmp4Muxer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun timescale(): moq.Timescale
	@kotlin.jvm.Throws(MuxErrorException::class) fun withTimescale(timescale: moq.Timescale): moq.MuxContainerFmp4Muxer
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun read(group: moq.GroupConsumer): kotlin.collections.List<moq.MuxContainerFrame>
	@kotlin.jvm.Throws(MuxErrorException::class) fun pictureSize(keyframe: moq.MuxContainerFrame): kotlin.Pair<kotlin.UInt, kotlin.UInt>?
	@kotlin.jvm.Throws(MuxErrorException::class) fun init(): moq.Bytes?
	@kotlin.jvm.Throws(MuxErrorException::class) fun fragment(sequence: kotlin.UInt, frames: kotlin.collections.List<moq.MuxContainerFrame>, end: moq.Timestamp?): moq.Bytes
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun video(config: moq.HangCatalogVideoConfig): moq.MuxContainerFmp4Muxer
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun audio(config: moq.HangCatalogAudioConfig): moq.MuxContainerFmp4Muxer
	}
}
class MuxContainerLegacyAudio internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerLegacyAudio
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun duration(payload: moq.Bytes): moq.Timestamp?
	companion object {
		@kotlin.jvm.JvmStatic fun new(config: moq.HangCatalogAudioConfig): moq.MuxContainerLegacyAudio?
	}
}
class MuxContainerMkvImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImport
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerMkvFinished
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerMkvImport
	}
}
class MuxContainerMkvImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImportExtra
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerMkvFinishedExtra
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerMkvImportExtra
	}
}
class MuxContainerMkvImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerMkvImportExt
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun seek(start: kotlin.ULong)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerMkvFinishedExt
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerMkvImportExt
	}
}
class MuxContainerMkvFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerMkvFinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerMkvFinishedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxContainerMp4Finish internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerMp4Finish
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsExt
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
	val passthroughGet: moq.MuxContainerTsPassthroughTrack?
	fun passthroughSet(value: moq.MuxContainerTsPassthroughTrack?)
	fun isEmpty(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsMpegts
	}
}
class MuxContainerTsPassthroughTrack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsPassthroughTrack
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val trackGet: kotlin.String
	fun trackSet(value: kotlin.String)
	val randomAccessGet: kotlin.Boolean
	fun randomAccessSet(value: kotlin.Boolean)
	companion object {
		@kotlin.jvm.JvmStatic fun new(track: kotlin.String): moq.MuxContainerTsPassthroughTrack
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsPesPacket
		@kotlin.jvm.JvmStatic fun new(streamType: kotlin.UByte): moq.MuxContainerTsPesPacket
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsProgram
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsSiEntry
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsTrack
		@kotlin.jvm.JvmStatic fun new(pid: kotlin.UShort): moq.MuxContainerTsTrack
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsVerbatim
		@kotlin.jvm.JvmStatic fun new(streamType: kotlin.UByte, framing: moq.MuxContainerTsFraming): moq.MuxContainerTsVerbatim
	}
}
class MuxContainerTsError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsError
}
class MuxContainerTsExportConsumerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumerExtra
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumerExtra): moq.MuxContainerTsExportConsumerExtra
	companion object {
		@kotlin.jvm.JvmStatic fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumerExtra, delay: moq.Duration): moq.MuxContainerTsExportConsumerExtra
	}
}
class MuxContainerTsExportConsumerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumerExt
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumerExt): moq.MuxContainerTsExportConsumerExt
	companion object {
		@kotlin.jvm.JvmStatic fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumerExt, delay: moq.Duration): moq.MuxContainerTsExportConsumerExt
	}
}
class MuxContainerTsExportConsumer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withMuxRate(muxRate: kotlin.ULong): moq.MuxContainerTsExportConsumer
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	fun stats(): moq.MuxContainerTsStatsExport
	fun discontinuity(): kotlin.ULong
	@kotlin.jvm.Throws(MuxErrorException::class) fun follow(broadcast: moq.BroadcastConsumer, catalog: moq.MuxCatalogConsumer): moq.MuxContainerTsExportConsumer
	companion object {
		@kotlin.jvm.JvmStatic fun new(source: moq.MuxSource, catalog: moq.MuxCatalogConsumer, delay: moq.Duration): moq.MuxContainerTsExportConsumer
	}
}
class MuxContainerTsFollower internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollower
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollower
	fun export(): moq.MuxContainerTsExportConsumer
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(export: moq.MuxContainerTsExportConsumer): moq.MuxContainerTsFollower
	}
}
class MuxContainerTsFollowerExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollowerExtra
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollowerExtra
	fun export(): moq.MuxContainerTsExportConsumerExtra
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(export: moq.MuxContainerTsExportConsumerExtra): moq.MuxContainerTsFollowerExtra
	}
}
class MuxContainerTsFollowerExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withFollow(follow: moq.Duration): moq.MuxContainerTsFollowerExt
	fun withStitch(stitch: kotlin.Boolean): moq.MuxContainerTsFollowerExt
	fun export(): moq.MuxContainerTsExportConsumerExt
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(export: moq.MuxContainerTsExportConsumerExt): moq.MuxContainerTsFollowerExt
	}
}
class MuxContainerTsPassthrough internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthrough
	fun track(): kotlin.String
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerTsPassthroughFinished
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerTsPassthrough
	}
}
class MuxContainerTsPassthroughUnit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthroughUnit
	fun track(): kotlin.String
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerTsPassthroughFinishedUnit
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerTsPassthroughUnit
	}
}
class MuxContainerTsPassthroughExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withPcrPid(pid: kotlin.UShort): moq.MuxContainerTsPassthroughExtra
	fun track(): kotlin.String
	@kotlin.jvm.Throws(MuxErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxErrorException::class) fun finish(): moq.MuxContainerTsPassthroughFinishedExtra
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerTsPassthroughExtra
	}
}
class MuxContainerTsPrograms internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun decode(data: moq.Bytes)
	fun stats(): moq.MuxContainerTsStatsSnapshot
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun finish(): moq.MuxContainerTsProgramsFinished
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(origin: moq.OriginProducer, name: moq.Path, config: moq.MuxCatalogConfig): moq.MuxContainerTsPrograms
	}
}
class MuxContainerTsRelease internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun dropped(): kotlin.ULong
	fun drift(): kotlin.Double?
	fun outOfTolerance(): kotlin.ULong
	fun stats(): moq.MuxContainerTsStatsRelease
	@kotlin.jvm.Throws(MuxErrorException::class) suspend fun next(): moq.MuxContainerFrame?
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxErrorException::class) suspend fun new(source: moq.MuxSource, catalogFormat: moq.MuxCatalogCatalogFormat, delay: moq.Duration): moq.MuxContainerTsRelease
	}
}
class MuxContainerTsRepublish internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsRepublish
	fun broadcast(): moq.BroadcastProducer
	fun catalog(): moq.MuxCatalogProducerExt
	fun instance(): moq.Instance
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun decode(data: moq.Bytes): moq.MuxContainerTsRepublished?
	fun stats(): moq.MuxContainerTsStatsSnapshot
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun finish(): moq.MuxContainerTsRepublishFinished
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun new(origin: moq.OriginProducer, name: moq.Path, instance: moq.Instance, config: moq.MuxCatalogConfig): moq.MuxContainerTsRepublish
	}
}
class MuxContainerTsRepublished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MuxContainerTsRepublished
	val stepGet: moq.MuxContainerTsStep
	fun stepSet(value: moq.MuxContainerTsStep)
	val statsGet: moq.MuxContainerTsStatsSnapshot
	fun statsSet(value: moq.MuxContainerTsStatsSnapshot)
}
class MuxContainerTsImport internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImport
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImport
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun seek(sequence: kotlin.ULong)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun finish(): moq.MuxContainerTsFinished
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReserved): moq.MuxContainerTsImport
	}
}
class MuxContainerTsImportExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImportExtra
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImportExtra
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun seek(sequence: kotlin.ULong)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun finish(): moq.MuxContainerTsFinishedExtra
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExtra): moq.MuxContainerTsImportExtra
	}
}
class MuxContainerTsImportExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun withProgram(program: kotlin.UShort): moq.MuxContainerTsImportExt
	fun withContainer(container: moq.HangCatalogContainer): moq.MuxContainerTsImportExt
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun decode(data: moq.Bytes)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun seek(sequence: kotlin.ULong)
	@kotlin.jvm.Throws(MuxContainerTsErrorException::class) fun finish(): moq.MuxContainerTsFinishedExt
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
	companion object {
		@kotlin.jvm.JvmStatic fun new(broadcast: moq.BroadcastProducer, reserved: moq.MuxCatalogReservedExt): moq.MuxContainerTsImportExt
	}
}
class MuxContainerTsFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
}
class MuxContainerTsFinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
}
class MuxContainerTsFinishedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
}
class MuxImportContainerFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxImportContainerFinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxImportContainerFinishedExt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun abort(err: moq.Error)
}
class MuxImportTrackFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun name(): kotlin.String
	fun abort(err: moq.Error)
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
	@kotlin.jvm.Throws(MuxCodecAacAdtsErrorException::class) fun payload(packet: moq.Bytes): moq.Bytes
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MuxCodecAacAdtsErrorException::class) fun parse(data: moq.Bytes): moq.MuxCodecAacAdtsHeader
	}
}
class MuxContainerTsPassthroughFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun track(): kotlin.String
	fun abort(err: moq.Error)
}
class MuxContainerTsPassthroughFinishedUnit internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun track(): kotlin.String
	fun abort(err: moq.Error)
}
class MuxContainerTsPassthroughFinishedExtra internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun track(): kotlin.String
	fun abort(err: moq.Error)
}
class MuxContainerTsProgramsFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun broadcasts(): kotlin.collections.List<moq.BroadcastProducer>
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
}
class MuxContainerTsRepublishFinished internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun broadcast(): moq.BroadcastProducer
	fun stats(): moq.MuxContainerTsStatsSnapshot
	fun abort(err: moq.Error)
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsReport
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsSnapshot
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsStream
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsExport
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
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsRelease
	}
}
class MuxContainerTsStatsLog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun sample(latest: moq.MuxContainerTsStatsSnapshot)
	fun finish(latest: moq.MuxContainerTsStatsSnapshot)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MuxContainerTsStatsLog
		@kotlin.jvm.JvmStatic val INTERVAL: moq.Duration
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
		@kotlin.jvm.JvmStatic fun new(addr: moq.MoqTokioConnectAddr): moq.MoqTokioAddrs
		@kotlin.jvm.JvmStatic fun collect(urls: kotlin.collections.List<moq.MoqTokioConnectAddr>): moq.MoqTokioAddrs?
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioCliDeprecated
	}
}
class MoqTokioCliBound internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioCliBound
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic val NONE: moq.MoqTokioCliBound
	}
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
		@kotlin.jvm.JvmStatic fun new(listener: kotlin.String): moq.MoqTokioAcceptHealth
	}
}
class MoqJsonTimedCatalog internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqJsonTimedCatalog
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val valueGet: moq.HangCatalog
	fun valueSet(value: moq.HangCatalog)
	val atGet: moq.Timestamp
	fun atSet(value: moq.Timestamp)
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: moq.HangCatalog, at: moq.Timestamp): moq.MoqJsonTimedCatalog
	}
}
class MoqJsonTimedJson internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqJsonTimedJson
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val valueGet: moq.Json
	fun valueSet(value: moq.Json)
	val atGet: moq.Timestamp
	fun atSet(value: moq.Timestamp)
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: moq.Json, at: moq.Timestamp): moq.MoqJsonTimedJson
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
	@kotlin.jvm.Throws(StringException::class) fun add(der: moq.RustlsPkiTypesCertificateDer)
	fun subjects(): kotlin.collections.List<moq.RustlsDistinguishedName>
	fun isEmpty(): kotlin.Boolean
	fun len(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun empty(): moq.RustlsRootCertStore
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
		@kotlin.jvm.JvmStatic fun fromSlice(bytes: moq.Bytes): moq.RustlsPkiTypesCertificateDer
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
	val timeoutGet: moq.Duration?
	fun timeoutSet(value: moq.Duration?)
}
class MoqTokioConnectionHandover internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectionHandover
	val redirectGet: moq.MoqTokioRedirect
	fun redirectSet(value: moq.MoqTokioRedirect)
	val handoverGet: moq.TokioDuration?
	fun handoverSet(value: moq.TokioDuration?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioConnectionHandover
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioWebsocketConfig
	}
}
class MoqTokioConnectionObservation internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectionObservation
	val revisionGet: kotlin.ULong
	fun revisionSet(value: kotlin.ULong)
	val connectsGet: kotlin.ULong
	fun connectsSet(value: kotlin.ULong)
	val lastConnectedGet: moq.ClockInstant?
	fun lastConnectedSet(value: moq.ClockInstant?)
	val phaseGet: moq.MoqTokioConnectionPhase
	fun phaseSet(value: moq.MoqTokioConnectionPhase)
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioTcpConfig
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioUnixConfig
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioListenLoadBalancer
	}
}
class MoqTokioListenReset internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioListenReset
	val secretGet: moq.PathBuf?
	fun secretSet(value: moq.PathBuf?)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioListenReset
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
	val idleTimeoutGet: moq.Duration?
	fun idleTimeoutSet(value: moq.Duration?)
	val keepAliveGet: moq.Duration?
	fun keepAliveSet(value: moq.Duration?)
	val mtuDiscoveryGet: kotlin.Boolean
	fun mtuDiscoverySet(value: kotlin.Boolean)
	val congestionControlGet: moq.MoqTokioQuicCongestionControl
	fun congestionControlSet(value: moq.MoqTokioQuicCongestionControl)
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
	fun keepAliveInterval(): moq.Duration
	fun limits(): moq.MoqQuicLimits
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioQuicResolved
	}
}
class MoqTokioWebsocketListener internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(MoqTokioWebsocketErrorException::class) fun withProtocols(protocols: kotlin.collections.List<kotlin.String>): moq.MoqTokioWebsocketListener
	fun withQuic(quic: moq.MoqTokioQuicResolved): moq.MoqTokioWebsocketListener
	@kotlin.jvm.Throws(MoqTokioWebsocketErrorException::class) fun localAddr(): moq.SocketAddr
	fun acceptHealth(): moq.MoqTokioAcceptHealth
	@kotlin.jvm.Throws(MoqTokioWebsocketErrorException::class) suspend fun accept(): moq.QmuxSession
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MoqTokioWebsocketErrorException::class) suspend fun bind(addr: moq.SocketAddr): moq.MoqTokioWebsocketListener
	}
}
class MoqTokioQuicDemuxDemux internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioQuicDemuxDemux
	fun counters(): moq.MoqSockDemuxCounters
	fun webrtc(): moq.MoqTokioQuicDemuxWebRtc
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioQuicDemuxDemux
	}
}
class MoqTokioServerStreams internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioServerStreams
	}
}
class MoqTokioServerQuicLimits internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioServerQuicLimits
	@kotlin.jvm.Throws(StringException::class) fun set(limits: moq.MoqQuicLimits)
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
	@kotlin.jvm.Throws(StringException::class) fun validate()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqQuicLimits
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
		@kotlin.jvm.JvmStatic fun new(url: moq.Url): moq.MoqTokioConnectAddr
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(MoqTokioConnectErrorException::class) fun pinned(url: moq.Url, addrs: kotlin.collections.List<moq.SocketAddr>): moq.MoqTokioConnectAddr
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
class MoqTokioConnectionPhase internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqTokioConnectionPhase
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
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioListenForward
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
		@kotlin.jvm.JvmStatic fun default(): moq.RustlsClientResumption
		@kotlin.jvm.JvmStatic fun inMemorySessions(num: kotlin.ULong): moq.RustlsClientResumption
		@kotlin.jvm.JvmStatic fun disabled(): moq.RustlsClientResumption
	}
}
class RustlsDistinguishedName internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsDistinguishedName
	companion object {
		@kotlin.jvm.JvmStatic fun inSequence(bytes: moq.Bytes): moq.RustlsDistinguishedName
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
		@kotlin.jvm.JvmStatic fun default(): moq.MoqTokioUnixAllow
	}
}
class MoqTokioWebsocketError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
class QmuxSession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.QmuxSession
	fun protocol(): kotlin.String?
	fun version(): moq.MoqQuicConnectionMuxVersion
	@kotlin.jvm.Throws(StringException::class) fun setLimits(limits: moq.MoqQuicLimits)
	fun peerCertificates(): kotlin.collections.List<moq.RustlsPkiTypesCertificateDer>?
	fun serverName(): kotlin.String?
}
class MoqSockDemuxCounters internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.MoqSockDemuxCounters
	fun discard(reason: moq.MoqSockDemuxDiscard)
	fun get(reason: moq.MoqSockDemuxDiscard): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun default(): moq.MoqSockDemuxCounters
	}
}
class MoqTokioQuicDemuxWebRtc internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	suspend fun recv(): moq.MoqTokioQuicDemuxDatagram?
	fun pin(remote: moq.SocketAddr): moq.MoqSockDemuxPin
}
class RustlsPkiTypesDer internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): moq.RustlsPkiTypesDer
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun fromSlice(der: moq.Bytes): moq.RustlsPkiTypesDer
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
class Url internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): moq.Url
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(StringException::class) fun parse(input: kotlin.String): moq.Url
	}
}
class Json internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun toText(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(StringException::class) fun parse(input: kotlin.String): moq.Json
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
class HangErrorException(val error: moq.HangError) : Exception() {
	override val message: String
}
class IntersectionErrorException(val error: moq.IntersectionError) : Exception() {
	override val message: String
}
class InvalidPatternException(val error: moq.InvalidPattern) : Exception() {
	override val message: String
}
class MoqTokioConnectErrorException(val error: moq.MoqTokioConnectError) : Exception() {
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
class MuxCodecAacImportException(val error: moq.MuxCodecAacImport) : Exception() {
	override val message: String
}
class MuxCodecAnnexbErrorException(val error: moq.MuxCodecAnnexbError) : Exception() {
	override val message: String
}
class MuxCodecAv1ImportException(val error: moq.MuxCodecAv1Import) : Exception() {
	override val message: String
}
class MuxCodecFlacErrorException(val error: moq.MuxCodecFlacError) : Exception() {
	override val message: String
}
class MuxCodecFlacImportException(val error: moq.MuxCodecFlacImport) : Exception() {
	override val message: String
}
class MuxCodecH264ErrorException(val error: moq.MuxCodecH264Error) : Exception() {
	override val message: String
}
class MuxCodecH264ImportException(val error: moq.MuxCodecH264Import) : Exception() {
	override val message: String
}
class MuxCodecH265ErrorException(val error: moq.MuxCodecH265Error) : Exception() {
	override val message: String
}
class MuxCodecH265ImportException(val error: moq.MuxCodecH265Import) : Exception() {
	override val message: String
}
class MuxCodecMp3ErrorException(val error: moq.MuxCodecMp3Error) : Exception() {
	override val message: String
}
class MuxCodecMp3ImportException(val error: moq.MuxCodecMp3Import) : Exception() {
	override val message: String
}
class MuxCodecOpusErrorException(val error: moq.MuxCodecOpusError) : Exception() {
	override val message: String
}
class MuxCodecOpusImportException(val error: moq.MuxCodecOpusImport) : Exception() {
	override val message: String
}
class MuxCodecVp8ImportException(val error: moq.MuxCodecVp8Import) : Exception() {
	override val message: String
}
class MuxCodecVp9ImportException(val error: moq.MuxCodecVp9Import) : Exception() {
	override val message: String
}
class MuxContainerFmp4ErrorException(val error: moq.MuxContainerFmp4Error) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerAudioConfigException(val error: moq.MuxContainerProducerContainerAudioConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerBinaryConfigException(val error: moq.MuxContainerProducerContainerBinaryConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerException(val error: moq.MuxContainerProducerContainer) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerJsonConfigException(val error: moq.MuxContainerProducerContainerJsonConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerTextConfigException(val error: moq.MuxContainerProducerContainerTextConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerContainerVideoConfigException(val error: moq.MuxContainerProducerContainerVideoConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireAudioConfigException(val error: moq.MuxContainerProducerFmp4WireAudioConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireBinaryConfigException(val error: moq.MuxContainerProducerFmp4WireBinaryConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireException(val error: moq.MuxContainerProducerFmp4Wire) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireJsonConfigException(val error: moq.MuxContainerProducerFmp4WireJsonConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireTextConfigException(val error: moq.MuxContainerProducerFmp4WireTextConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerFmp4WireVideoConfigException(val error: moq.MuxContainerProducerFmp4WireVideoConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireAudioConfigException(val error: moq.MuxContainerProducerLegacyWireAudioConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireBinaryConfigException(val error: moq.MuxContainerProducerLegacyWireBinaryConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireException(val error: moq.MuxContainerProducerLegacyWire) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireJsonConfigException(val error: moq.MuxContainerProducerLegacyWireJsonConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireTextConfigException(val error: moq.MuxContainerProducerLegacyWireTextConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLegacyWireVideoConfigException(val error: moq.MuxContainerProducerLegacyWireVideoConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireAudioConfigException(val error: moq.MuxContainerProducerLocWireAudioConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireBinaryConfigException(val error: moq.MuxContainerProducerLocWireBinaryConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireException(val error: moq.MuxContainerProducerLocWire) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireJsonConfigException(val error: moq.MuxContainerProducerLocWireJsonConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireTextConfigException(val error: moq.MuxContainerProducerLocWireTextConfig) : Exception() {
	override val message: String
}
class MuxContainerProducerLocWireVideoConfigException(val error: moq.MuxContainerProducerLocWireVideoConfig) : Exception() {
	override val message: String
}
class MuxContainerTsErrorException(val error: moq.MuxContainerTsError) : Exception() {
	override val message: String
}
class MuxErrorException(val error: moq.MuxError) : Exception() {
	override val message: String
}
class MuxImportTrackException(val error: moq.MuxImportTrack) : Exception() {
	override val message: String
}
class MuxImportTrackStreamException(val error: moq.MuxImportTrackStream) : Exception() {
	override val message: String
}
class MuxJsonSnapshotJsonException(val error: moq.MuxJsonSnapshotJson) : Exception() {
	override val message: String
}
class MuxJsonStreamJsonException(val error: moq.MuxJsonStreamJson) : Exception() {
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
const val HANG_CATALOG_MAX_SAFE_INTEGER: ULong
val IMPLEMENTATION: kotlin.String
val ALPNS: kotlin.collections.List<kotlin.String>
fun clockNow(): moq.ClockInstant
fun clockWall(): moq.ClockWall
fun clockOrigin(): kotlin.Pair<moq.ClockInstant, moq.ClockWall>
fun clockRng(): moq.ClockRng
fun clockSeed(): kotlin.ULong
fun clockHostTime()
suspend fun clockDrive(poll: (moq.ClockInstant, moq.Waiter) -> moq.ClockInstant?): kotlin.String
fun clockInterval(period: moq.Duration): moq.ClockInterval
fun clockIntervalAt(start: moq.ClockInstant, period: moq.Duration): moq.ClockInterval
suspend fun clockSleep(duration: moq.Duration)
suspend fun clockSleepUntil(at: moq.ClockInstant)
@kotlin.jvm.Throws(ClockElapsedException::class) suspend fun clockTimeout(duration: moq.Duration, future: suspend () -> kotlin.Unit)
@kotlin.jvm.Throws(ClockElapsedException::class) suspend fun clockTimeoutAt(deadline: moq.ClockInstant, future: suspend () -> kotlin.Unit)
val SERVERDEFAULTTIMEOUT: moq.Duration
suspend fun timeRunDriver(driver: moq.Driver): moq.Error
suspend fun timeRunOriginDriver(driver: moq.OriginDriver): moq.Error
@kotlin.jvm.Throws(DecodeErrorException::class) fun varintDecodeQuic(r: moq.Mut<moq.Bytes>): kotlin.ULong
@kotlin.jvm.Throws(EncodeErrorException::class) fun varintEncodeQuic(value: kotlin.ULong, w: moq.Mut<moq.Bytes>)
val CACHEDEFAULTEXPIRY: moq.Duration
val HANGECHOSUFFIX: kotlin.String
val HANGECHOTRACK: kotlin.String
fun hangEchoIsEcho(path: moq.Path): kotlin.Boolean
val HANGCATALOGPRIORITY: moq.HangCatalogPriorities
val HANGCONTAINERMAXAGE: moq.Duration
val HANGCONTAINERTIMESCALE: moq.Timescale
fun hangContainerTrackInfo(priority: kotlin.Byte): moq.TrackInfo
val MUXCONTAINERDELAYMAX: moq.Duration
val MUXRATERETRY: moq.Duration
@kotlin.jvm.Throws(TokioTlsErrorException::class) fun tokioTlsParseFingerprint(value: kotlin.String): ArrayU8x32
fun tokioOriginSpawn(): moq.OriginProducer
fun transportMemoryPair(config: moq.TransportMemoryConfig): kotlin.Pair<moq.TransportMemorySession, moq.TransportMemorySession>
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecAacConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
val MUXCODECANNEXBSTARTCODE: moq.Bytes
@kotlin.jvm.Throws(MuxCodecAnnexbErrorException::class) fun muxCodecAnnexbFromLengthPrefixed(payload: moq.Bytes, lengthSize: kotlin.ULong, prefix: moq.Bytes?): moq.Bytes
fun muxCodecAnnexbBuildPrefix(nals: kotlin.collections.List<moq.Bytes>): moq.Bytes
@kotlin.jvm.Throws(MuxCodecAnnexbErrorException::class) fun muxCodecAnnexbLengthPrefixedToAnnexb(data: moq.Bytes, lengthSize: kotlin.ULong, out: moq.Mut<moq.Bytes>)
@kotlin.jvm.Throws(MuxCodecAnnexbErrorException::class) fun muxCodecAnnexbAfterStartCode(b: moq.Bytes): kotlin.ULong?
fun muxCodecAnnexbFindStartCode(b: moq.Bytes): kotlin.Pair<kotlin.ULong, kotlin.ULong>?
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecFlacConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecH264Config(buf: moq.Bytes): moq.HangCatalogVideoConfig
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecH265Config(buf: moq.Bytes): moq.HangCatalogVideoConfig
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecMp3Config(init: moq.Bytes): moq.HangCatalogAudioConfig
val MUXCODECOPUSWARMUP: moq.Duration
fun muxCodecOpusPacketSamples(packet: moq.Bytes): kotlin.UInt?
@kotlin.jvm.Throws(MuxErrorException::class) fun muxCodecOpusConfig(init: moq.Bytes): moq.HangCatalogAudioConfig
@kotlin.jvm.Throws(MuxErrorException::class) fun muxContainerMp4Repair(file: java.nio.channels.SeekableByteChannel): moq.MuxContainerMp4Finish?
interface StartSeekable {
	fun seekStart(offset: ULong): ULong
}
