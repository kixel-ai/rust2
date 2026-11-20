// api.kt
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
class ArrayU8x4(bytes: ByteArray) {
	operator fun get(index: Int): Byte
	fun toByteArray(): ByteArray
	override fun equals(other: Any?): Boolean
	override fun hashCode(): Int
	override fun toString(): String
}
data class Parcel(val mark: api.Mark, val payload: api.Bytes)
data class Weighted(val score: api.Score, val weight: kotlin.UInt) {
	fun total(): kotlin.ULong
}
data class Stamped(val at: api.Stamp, val count: kotlin.UInt) {
	fun total(): kotlin.ULong
}
data class CancellationError(val code: kotlin.UInt)
data class Packet(val seq: kotlin.UInt, val payload: api.Bytes)
data class Hasher(val seed: kotlin.UByte)
data class Size(val width: kotlin.UInt, val height: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Point(val x: kotlin.Int, val y: kotlin.Int) {
	fun debug(): kotlin.String
	fun offset(dx: kotlin.Int, dy: kotlin.Int): api.Point
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Point
		@kotlin.jvm.JvmStatic @kotlin.jvm.JvmName("getX_") fun getX(): kotlin.Int
	}
}
data class Label(val name: kotlin.String, val at: api.Point?, val ttl: api.Duration?) {
	fun debug(): kotlin.String
}
data class Note(val text: kotlin.String?) {
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Note
	}
}
data class Reading(val ok: kotlin.Boolean, val level: kotlin.Float, val ratio: kotlin.Double, val mark: kotlin.Int, val shape: api.Shape, val at: api.Point, val note: kotlin.String, val count: kotlin.ULong, val offset: kotlin.Byte, val len: kotlin.ULong, val tag: kotlin.UShort) {
	fun debug(): kotlin.String
	fun next(): api.Reading?
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Packaged(val api: kotlin.UInt) {
	fun debug(): kotlin.String
	fun point(): api.Point
	fun shape(): api.Shape
}
data class Atlas(val points: kotlin.collections.List<kotlin.Pair<kotlin.String, api.Point>>, val ids: kotlin.collections.List<kotlin.UInt>) {
	fun debug(): kotlin.String
	fun with(name: kotlin.String, at: api.Point): api.Atlas
}
data class Span(val bounds: kotlin.Pair<kotlin.UInt, kotlin.UInt>, val named: kotlin.Pair<kotlin.String, api.Shape>?) {
	fun debug(): kotlin.String
}
data class Window(val range: api.RangeU64, val end: api.BoundU64) {
	fun debug(): kotlin.String
	fun contains(n: kotlin.ULong): kotlin.Boolean
}
@kotlin.jvm.JvmInline value class Ticket internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun number(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(number: kotlin.UInt): api.Ticket
		@kotlin.jvm.JvmStatic fun clones(): kotlin.UInt
	}
}
data class Seat(val mark: api.Mark, val row: kotlin.UInt) {
	fun debug(): kotlin.String
	fun count(): kotlin.ULong
}
@kotlin.jvm.JvmInline value class Stamp internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun secs(): kotlin.ULong
	fun whole(): api.Stamp
	fun later(other: api.Stamp): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun fromMillis(millis: kotlin.ULong): api.Stamp
	}
}
class Moment internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun millis(): kotlin.ULong
	fun after(ticks: kotlin.ULong): api.Moment
	companion object {
		@kotlin.jvm.JvmStatic fun new(ticks: kotlin.ULong, rate: kotlin.ULong): api.Moment?
	}
}
class LongArray_ internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun c(): kotlin.ULong
	fun also(f: (kotlin.ULong) -> kotlin.Unit)
	companion object {
		@kotlin.jvm.JvmStatic fun new(low: kotlin.ULong, high: kotlin.ULong): api.LongArray_
	}
}
data class JvmField(val n: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class JvmStatic(val n: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class OptIn(val n: kotlin.UInt) {
	fun debug(): kotlin.String
}
class Wide internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun sum(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(seed: kotlin.ULong): api.Wide
	}
}
data class Clocked(val at: api.Moment, val count: kotlin.UInt) {
	fun debug(): kotlin.String
}
@kotlin.jvm.JvmInline value class Score internal constructor(internal val bits: Long) {
	fun debug(): kotlin.String
	fun count(): kotlin.ULong
	fun add(other: api.Score): api.Score
	fun doubled(): api.Score
	fun take(): kotlin.Pair<api.Score, kotlin.ULong>
	fun note(mark: api.Mark): kotlin.Pair<api.Score, kotlin.ULong>
	fun absorb(other: api.Mut<api.Score>): kotlin.Triple<api.Score, kotlin.ULong, kotlin.ULong>
	companion object {
		@kotlin.jvm.JvmStatic fun of(count: kotlin.ULong): api.Score
	}
}
class Hits internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val hitsGet: kotlin.ULong
	fun hitsSet(value: kotlin.ULong): api.Hits
	val missesGet: kotlin.ULong
	fun missesSet(value: kotlin.ULong): api.Hits
	fun add(other: api.Hits): api.Hits
	fun eq(other: api.Hits): kotlin.Pair<api.Hits, kotlin.Boolean>
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Hits
		@kotlin.jvm.JvmStatic fun of(hits: kotlin.ULong, misses: kotlin.ULong): api.Hits
	}
}
class Extent internal constructor() {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val startGet: kotlin.ULong
	fun startSet(value: kotlin.ULong): api.Extent
	fun end(): kotlin.ULong
	fun slide(by: kotlin.ULong): kotlin.Pair<api.Extent, kotlin.ULong>
	companion object {
		@kotlin.jvm.JvmStatic fun new(start: kotlin.ULong, len: kotlin.ULong): api.Extent
	}
}
data class PairSizeU8(val left: api.Size, val right: kotlin.UByte) {
	fun debug(): kotlin.String
	fun swap(): api.PairU8Size
}
data class PairNonZeroU16(val left: kotlin.UShort, val right: kotlin.UShort) {
	fun debug(): kotlin.String
	fun swap(): api.PairNonZeroU16
	fun even(): kotlin.Boolean
}
data class PairU32(val left: kotlin.UInt, val right: kotlin.UInt) {
	fun debug(): kotlin.String
	fun swap(): api.PairU32
	fun even(): kotlin.Boolean
	fun sum(): kotlin.UInt
	fun besideSize(right: api.Size): api.PairU8Size
}
data class PairU8Size(val left: kotlin.UByte, val right: api.Size) {
	fun debug(): kotlin.String
}
data class LeftRightU8(val value: api.RightU8) {
	fun debug(): kotlin.String
}
data class LeftU8(val value: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class RightLeftU8(val value: api.LeftU8) {
	fun debug(): kotlin.String
}
data class RightU8(val value: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class InnerU8(val value: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class BoxedU16(val value: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class BoxedU8(val value: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class LogCount(val kept: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class WrapU32(val value: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class PolarTurn(val whole: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Peer(val addr: api.SocketAddr, val names: kotlin.collections.List<kotlin.String>) {
	fun debug(): kotlin.String
	fun aliases(): kotlin.collections.List<kotlin.String>
	fun next(name: kotlin.String): api.Peer
}
data class Shadowed(val other: kotlin.IntArray, val a: kotlin.UByte, val b: kotlin.UByte) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Numbers(val u16s: kotlin.UShortArray, val u32s: kotlin.UIntArray, val u64s: kotlin.ULongArray, val i8s: kotlin.ByteArray, val i16s: kotlin.ShortArray, val i32s: kotlin.IntArray, val i64s: kotlin.LongArray, val f32s: kotlin.FloatArray, val f64s: kotlin.DoubleArray, val some: kotlin.IntArray?) {
	fun debug(): kotlin.String
	fun reversed(): api.Numbers
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun edges(): api.Numbers
		@kotlin.jvm.JvmStatic fun zero(negative: kotlin.Boolean): api.Numbers
	}
}
data class Measure(val value: kotlin.Double, val numbers: api.Numbers) {
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.Double, negative: kotlin.Boolean): api.Measure
	}
}
data class LimitsBytes(val low: api.Bytes, val high: api.Bytes) {
	fun debug(): kotlin.String
}
data class LimitsDuration(val low: api.Duration, val high: api.Duration) {
	fun debug(): kotlin.String
}
data class Page(val size: kotlin.UInt) {
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Page
	}
}
data class Booth(val mark: api.Mark) {
	fun debug(): kotlin.String
}
data class Cancelled(val step: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class StartSeekable_(val at: kotlin.ULong) {
	fun debug(): kotlin.String
}
data class DoubleArray_(val len: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class NonZeros(val u8: kotlin.UByte, val u16: kotlin.UShort, val u32: kotlin.UInt, val u64: kotlin.ULong, val usize: kotlin.ULong, val i8: kotlin.Byte, val i16: kotlin.Short, val i32: kotlin.Int, val i64: kotlin.Long, val isize: kotlin.Long) {
	fun debug(): kotlin.String
}
data class A(val b: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class Qualifiers(val api: kotlin.Double, val kotlin: kotlin.Double) {
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class PolarPoint(val turns: kotlin.UByte, val distance: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Polar_Turn(val part: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class ClashTone(val hz: kotlin.UShort) {
	fun debug(): kotlin.String
}
data class ClashTone_(val db: kotlin.UByte) {
	fun debug(): kotlin.String
}
data class ClashSpan(val width: kotlin.UInt) {
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun width(): kotlin.UInt
	fun toString_(): kotlin.String
}
data class ClashAny(val typeUrl: kotlin.String) {
	fun debug(): kotlin.String
}
data class ClashBreak(val _0: kotlin.UByte) {
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val SP: api.ClashBreak
		@kotlin.jvm.JvmStatic val SP_: api.ClashBreak
	}
}
data class TlsPolicyLifetime(val seconds: kotlin.UInt) {
	fun debug(): kotlin.String
}
data class Core(val id: kotlin.UInt)
sealed class Choice {
	data object Empty : Choice()
	data class Picked(val _0: api.Mark) : Choice()
}
sealed class Array {
	data class U8x4(val _0: kotlin.UByte) : Array()
}
sealed class Platform {
	data class Wasm(val bits: kotlin.UInt) : Platform()
	data class Native(val _0: kotlin.UInt) : Platform()
}
sealed class Placement {
	data object Anywhere : Placement()
	data class Pinned(val _0: api.Core) : Placement()
}
enum class Shape(internal val value: Int) {
	CIRCLE(1),
	SQUARE(4);
	fun debug(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic val ALL: kotlin.collections.List<api.Shape>
		@kotlin.jvm.JvmStatic val BOTH: kotlin.collections.List<api.Shape>
	}
}
sealed class Event {
	data class Moved(val _0: api.Point) : Event()
	data class Named(val name: kotlin.String) : Event()
	data object Done : Event()
	fun debug(): kotlin.String
}
sealed class Clash {
	data class Point(val _0: api.Point) : Clash()
	data class Shape(val _0: api.Shape) : Clash()
	data class Event(val _0: api.Event) : Clash()
	data class Double(val _0: kotlin.Double) : Clash() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class Float(val _0: kotlin.Float) : Clash() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class String(val _0: kotlin.String) : Clash()
	data class Int(val _0: kotlin.Int) : Clash()
	data class Boolean(val _0: kotlin.Boolean) : Clash()
	data class Native(val _0: kotlin.UInt) : Clash()
	data class Bytes(val _0: api.Bytes) : Clash()
	data object Any : Clash()
	data object Unit : Clash()
	data object Waker : Clash()
	data object Mut : Clash()
	data class Pair(val _0: kotlin.UByte, val _1: kotlin.UByte) : Clash()
	data class Stamp(val _0: kotlin.Long) : Clash()
	data class Moment(val _0: api.Moment) : Clash()
	fun debug(): kotlin.String
	fun point(): api.Point?
	fun bytes(): api.Bytes
	fun reading(kotlin: kotlin.UInt): api.Reading
	fun joined(lent: api.Bytes, owned: api.Bytes, maybe: api.Bytes?): api.Bytes
	suspend fun tagged(tag: kotlin.String): kotlin.String
	fun stamp(kotlin: kotlin.ULong): api.Stamp
	fun moment(kotlin: kotlin.ULong): api.Moment
	fun describe(text: kotlin.String, n: kotlin.UInt, shape: api.Shape, out: api.Mut<kotlin.UInt>): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun of(value: kotlin.Double): api.Clash
		@kotlin.jvm.JvmStatic fun listed(java: kotlin.UIntArray, kotlin: api.Mut<kotlin.UInt>): kotlin.collections.List<kotlin.UIntArray>
		@kotlin.jvm.JvmStatic suspend fun after(first: suspend () -> kotlin.Unit, kotlin: kotlin.Double): api.Clash
		@kotlin.jvm.JvmStatic fun at(api: api.Point): api.Clash
	}
}
sealed class Stage {
	data class Init(val _0: kotlin.UInt) : Stage()
	data object Done : Stage()
	fun debug(): kotlin.String
}
sealed class Error {
	data class Underflow(val by: kotlin.ULong) : Error()
	data object Overflow : Error()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
enum class Status(internal val value: Int) {
	MOVED(0),
	KEPT(1);
	fun debug(): kotlin.String
	companion object {}
}
sealed class Labelled {
	data class Tagged(val tag: kotlin.UInt) : Labelled()
	data object Bare : Labelled()
	fun debug(): kotlin.String
}
enum class Family(internal open val value: Int) {
	NATIVE(0),
	WASM(1)
	fun debug(): kotlin.String
	companion object {}
}
enum class VecClashTone(internal val value: Int) {
	ERROR(0);
	fun debug(): kotlin.String
	companion object {}
}
sealed class B {
	data object One : B()
	data class Two(val _0: kotlin.UByte) : B()
	fun debug(): kotlin.String
}
sealed class Builtin {
	data class Any(val _0: kotlin.IntArray) : Builtin() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class Boolean(val _0: kotlin.Double) : Builtin() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class Int(val _0: kotlin.Double) : Builtin() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
	data class Native(val _0: kotlin.Double) : Builtin() {
		override fun equals(other: kotlin.Any?): kotlin.Boolean
		override fun hashCode(): kotlin.Int
	}
}
sealed class ClashCandidate {
	data object None : ClashCandidate()
	data class Match(val _0: kotlin.UByte) : ClashCandidate()
	fun debug(): kotlin.String
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
class Board internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun intoCounters(): kotlin.collections.List<kotlin.Pair<kotlin.String, api.Counter>>
	fun ranked(): kotlin.collections.List<kotlin.Pair<kotlin.String, api.Counter>>
	fun best(): api.Counter
	companion object {
		@kotlin.jvm.JvmStatic fun new(counters: kotlin.collections.List<kotlin.Pair<kotlin.String, api.Counter>>): api.Board
	}
}
class Receipt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Receipt
	fun count(): kotlin.UInt
}
class Draft internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun edited(): kotlin.Boolean
	fun describe(): kotlin.String
	fun commit(): kotlin.UInt
	val titleGet: kotlin.String
	fun titleSet(value: kotlin.String)
	fun lines(): kotlin.UInt
	fun write(line: kotlin.String)
}
class FiledReceipt internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.FiledReceipt
	fun count(): kotlin.UInt
}
class PadU16 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun tag(): kotlin.UShort
	fun title(): kotlin.UShort
	fun describe(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun wide(): api.PadU16
	}
}
class PadU8 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun describe(): kotlin.String
	val titleGet: kotlin.String
	fun lines(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun small(): api.PadU8
	}
}
class Layered internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val titleGet: kotlin.String
	fun lines(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(): api.Layered
	}
}
class Marker internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun label(): kotlin.String
	val xGet: kotlin.Int
	fun xSet(value: kotlin.Int)
	val yGet: kotlin.Int
	fun ySet(value: kotlin.Int)
	fun offset(dx: kotlin.Int, dy: kotlin.Int): api.Point
	companion object {
		@kotlin.jvm.JvmStatic fun new(label: kotlin.String, x: kotlin.Int, y: kotlin.Int): api.Marker
	}
}
class Batch internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun push(v: kotlin.UInt): kotlin.Boolean
	fun capacity(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(): api.Batch
	}
}
class Batch2 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun push(v: kotlin.UInt): kotlin.Boolean
	fun capacity(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(): api.Batch2
	}
}
class Waiter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun waker(): api.Waker
	fun context(): api.Waker
	companion object {
		@kotlin.jvm.JvmStatic fun new(waker: api.Waker): api.Waiter
		@kotlin.jvm.JvmStatic suspend fun pollFuture(future: suspend () -> kotlin.Unit)
	}
}
class Logger internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun write(line: kotlin.String): kotlin.Boolean
	fun writeFromThread(line: kotlin.String): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(sink: api.Log): api.Logger
	}
}
class Journal internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun redirect(sink: api.Log)
	fun write(line: kotlin.String): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(sink: api.Log): api.Journal
	}
}
class Local internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun bump(): kotlin.UInt
	fun share(): api.Local
	suspend fun later(): kotlin.UInt
	fun visit(f: () -> kotlin.UInt): kotlin.UInt
	suspend fun pollCount(): kotlin.UInt
	fun threadNumber(): kotlin.ULong
	fun onOwnThread(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Local
		@kotlin.jvm.JvmStatic fun new(): api.Local
	}
}
class LocalWaiter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic fun new(waker: api.Waker): api.LocalWaiter
		@kotlin.jvm.JvmStatic suspend fun pollReady(n: kotlin.UInt): kotlin.UInt
	}
}
class Retry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val limitGet: kotlin.UInt?
	fun limitSet(value: kotlin.UInt?)
	fun limit(): kotlin.UInt
	fun delay(): kotlin.UInt
	fun setLimit(limit: kotlin.UInt)
	companion object {
		@kotlin.jvm.JvmStatic fun new(limit: kotlin.UInt?): api.Retry
	}
}
class Digest internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val sumGet: ArrayU8x4
	fun sumSet(value: ArrayU8x4)
	fun flipped(mask: ArrayU8x4): ArrayU8x4
	companion object {
		@kotlin.jvm.JvmStatic fun new(): api.Digest
	}
}
class Cursor internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun at(): kotlin.ULong
}
class OuterInnerU8 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun get(): kotlin.UByte
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.UByte): api.OuterInnerU8
	}
}
class SlotU32 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.UInt): api.SlotU32
	}
}
class SlotU8 internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun value(): kotlin.UShort
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.UByte): api.SlotU8
	}
}
class Resolving internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val _0_get: api.TlsResolver
	fun _0_set(value: api.TlsResolver)
	companion object {
		@kotlin.jvm.JvmStatic fun new(name: kotlin.String): api.Resolving
	}
}
class Codecs internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val listGet: kotlin.collections.List<api.TlsCodec>
	fun listSet(value: kotlin.collections.List<api.TlsCodec>)
	fun ids(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Codecs
		@kotlin.jvm.JvmStatic fun new(): api.Codecs
	}
}
class Der internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun bytes(): api.Bytes
	companion object {
		@kotlin.jvm.JvmStatic fun fromSlice(bytes: api.Bytes): api.Der
	}
}
class Verbosity internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val levelGet: api.Level
	fun levelSet(value: api.Level)
	fun filter(): api.Level?
	companion object {
		@kotlin.jvm.JvmStatic fun new(level: api.Level): api.Verbosity
	}
}
class Site internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val homeGet: api.Url
	fun homeSet(value: api.Url)
	companion object {
		@kotlin.jvm.JvmStatic fun new(home: api.Url): api.Site
	}
}
class Trail internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun points(): kotlin.collections.List<api.Point>
	fun labels(): kotlin.collections.List<api.Label>
	fun iter(): kotlin.collections.List<api.Point>
	fun names(): kotlin.collections.List<kotlin.String>
	fun beyond(x: kotlin.Int): kotlin.collections.List<api.Point>
	fun bytes(): api.Bytes
	fun drain(): kotlin.collections.List<api.Label>
	companion object {
		@kotlin.jvm.JvmStatic fun new(points: kotlin.collections.List<api.Point>, names: kotlin.collections.List<kotlin.String>): api.Trail
	}
}
class Worker internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun id(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(id: kotlin.UInt): api.Worker
	}
}
class Handle internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun count(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(count: kotlin.UInt): api.Handle
		@kotlin.jvm.JvmStatic suspend fun later(count: kotlin.UInt): api.Handle
	}
}
class Area internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Area
	fun size(): api.Size
	fun pixels(): kotlin.ULong
	fun scaledPoint(by: api.Point): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(size: api.Size): api.Area
	}
}
class Counter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Counter
	fun get(): kotlin.ULong
	fun offset(handle: kotlin.ULong, h: kotlin.ULong): kotlin.ULong
	fun intoSum(close: kotlin.ULong): kotlin.ULong
	fun name(): kotlin.String
	@kotlin.jvm.Throws(ErrorException::class) fun add(n: kotlin.ULong): kotlin.ULong
	@kotlin.jvm.Throws(ErrorException::class) fun sub(n: kotlin.ULong): kotlin.ULong
	fun edited(): kotlin.Boolean
	fun plus(other: api.Counter?): kotlin.ULong
	fun clamped(low: api.Counter?, high: api.Counter?): kotlin.ULong
	suspend fun plusLater(other: api.Counter?): kotlin.ULong
	fun addTo(other: api.Counter?): kotlin.Boolean
	fun intoLabel(at: api.Point?): api.Label
	fun rename(name: kotlin.String)
	fun split(at: kotlin.ULong): kotlin.Pair<api.Counter, api.Counter>
	@kotlin.jvm.Throws(StringException::class) fun plusIfString(n: kotlin.ULong, `object`: kotlin.Any?): kotlin.ULong
	fun `is`(name: kotlin.String): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Counter
		@kotlin.jvm.JvmStatic fun new(name: kotlin.String, start: kotlin.ULong): api.Counter
		@kotlin.jvm.JvmStatic fun seededPoint(name: kotlin.String, from: api.Point): api.Counter
		@kotlin.jvm.JvmStatic fun seededCounter(name: kotlin.String, from: api.Counter): api.Counter
		@kotlin.jvm.JvmStatic fun seededSize(name: kotlin.String, from: api.Size): api.Counter
		@kotlin.jvm.JvmStatic fun seededWrapU32(name: kotlin.String, from: api.WrapU32): api.Counter
		@kotlin.jvm.JvmStatic fun seededPolarPoint(name: kotlin.String, from: api.PolarPoint): api.Counter
	}
}
class Meter internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun add(n: kotlin.ULong): kotlin.ULong
	fun count(): kotlin.ULong
	fun reset()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Meter
	}
}
class Meters internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val lastGet: api.Meter
	fun lastSet(value: api.Meter)
	val spareGet: api.Meter?
	fun spareSet(value: api.Meter?)
	fun first(): api.Meter
	fun clear()
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Meters
		@kotlin.jvm.JvmStatic fun new(): api.Meters
	}
}
class Registry internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val firstGet: api.Counter
	fun firstSet(value: api.Counter)
	fun last(): api.Counter?
	fun push(counter: api.Counter)
	fun edit(edit: (api.Counter) -> kotlin.Boolean): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Registry
		@kotlin.jvm.JvmStatic fun new(first: api.Counter): api.Registry
	}
}
class Ledger internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	@kotlin.jvm.Throws(ErrorException::class) fun edit(): api.Draft
	fun published(): kotlin.UInt
	fun title(): kotlin.String
	fun seal()
	fun receipt(): api.Receipt
	fun noted(): api.Receipt
	fun fileReceipt(item: api.Receipt): api.FiledReceipt
	fun slip(): api.Slip
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Ledger
		@kotlin.jvm.JvmStatic fun new(): api.Ledger
	}
}
class Slip internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Slip
	val receiptGet: api.Receipt
	fun receiptSet(value: api.Receipt)
	val globalGet: api.Receipt
	fun globalSet(value: api.Receipt)
	fun count(): kotlin.UInt
}
class Sheet internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Sheet
	val titleGet: kotlin.String
	fun titleSet(value: kotlin.String)
	fun lines(): kotlin.UInt
	fun write(line: kotlin.String)
	fun describe(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Sheet
		@kotlin.jvm.JvmStatic fun new(): api.Sheet
	}
}
class Name internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Name
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun asStr(): kotlin.String
	fun join(other: api.Name): api.Name
	fun shares(other: api.Name): kotlin.Boolean
	fun rename(text: kotlin.String)
	companion object {
		@kotlin.jvm.JvmStatic fun owned(text: kotlin.String): api.Name
		@kotlin.jvm.JvmStatic fun borrowed(text: kotlin.String): api.Name
	}
}
class Channel internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Channel
	fun clear()
	fun send(payload: api.Bytes)
	suspend fun recv(): api.Bytes
	fun notify(waker: api.Waker)
	suspend fun sendLater(payload: api.Bytes)
	suspend fun tagged(tag: kotlin.String): kotlin.String
	suspend fun itemized(items: kotlin.UIntArray?): kotlin.ULong
	suspend fun pollTake(): api.Bytes
	@kotlin.jvm.Throws(ErrorException::class) suspend fun pollMeasure(extra: kotlin.ULong): kotlin.ULong
	fun listen(cx: api.Waker)
	suspend fun recvSized(): kotlin.Pair<api.Bytes, kotlin.ULong>
	@kotlin.jvm.Throws(ErrorException::class) suspend fun recvNonempty(): api.Bytes
	fun sendFromThread(payload: api.Bytes)
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Channel
		@kotlin.jvm.JvmStatic fun new(): api.Channel
	}
}
class Gap internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Gap
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun sum(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(wide: kotlin.ULong, narrow: kotlin.UByte): api.Gap
	}
}
class Berth internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Berth
	fun at(): kotlin.UInt?
	companion object {
		@kotlin.jvm.JvmStatic fun new(at: kotlin.UInt?): api.Berth
	}
}
class Mark internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Mark
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun count(): kotlin.ULong
	fun add(by: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic fun of(count: kotlin.ULong): api.Mark
	}
}
class Message internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Message
	val stampGet: api.Stamp?
	fun stampSet(value: api.Stamp?)
	val payloadGet: api.Bytes
	fun payloadSet(value: api.Bytes)
	fun sent(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(stamp: api.Stamp?, payload: api.Bytes): api.Message
	}
}
class Node internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	val valueGet: kotlin.ULong
	fun valueSet(value: kotlin.ULong)
	fun push(value: kotlin.ULong)
	fun sum(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.ULong, next: api.Node?): api.Node
	}
}
class Tally internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun bump(): kotlin.UInt
	suspend fun bumpLater(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Tally
		@kotlin.jvm.JvmStatic fun new(): api.Tally
	}
}
class Tallies internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun first(): api.Tally
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Tallies
		@kotlin.jvm.JvmStatic fun new(): api.Tallies
	}
}
class Plain internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val nGet: kotlin.ULong
	fun nSet(value: kotlin.ULong)
	val onGet: kotlin.Boolean
	fun onSet(value: kotlin.Boolean)
	companion object {
		@kotlin.jvm.JvmStatic fun new(n: kotlin.ULong, on: kotlin.Boolean): api.Plain
	}
}
class Labeled internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun new(label: kotlin.String): api.Labeled
	}
}
class Gauge internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Gauge
	fun set(level: kotlin.ULong)
	fun level(): kotlin.ULong
	fun intoLevel(): kotlin.ULong
	suspend fun later(): kotlin.ULong
	suspend fun reaches(`this`: kotlin.ULong): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(level: kotlin.ULong): api.Gauge
	}
}
class Latch internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Latch
	fun set(value: kotlin.UInt): kotlin.Boolean
	fun get(): kotlin.UInt?
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Latch
	}
}
class Token internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.Token
	val idGet: kotlin.UInt
	fun idSet(value: kotlin.UInt)
	fun twice(): kotlin.collections.List<api.Token>
	companion object {
		@kotlin.jvm.JvmStatic fun new(id: kotlin.UInt): api.Token
	}
}
class Stack internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun push(item: api.Counter)
	fun pop(): api.Counter?
	fun depth(): kotlin.ULong
	fun depthPastPoint(from: api.Point): kotlin.ULong
	fun depthPastCounter(from: api.Counter): kotlin.ULong
	fun depthPastSize(from: api.Size): kotlin.ULong
	fun depthPastWrapU32(from: api.WrapU32): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.Stack
		@kotlin.jvm.JvmStatic fun new(): api.Stack
	}
}
class Refusal internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
}
class C internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.UByte): api.C
	}
}
class Key internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Key
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	fun hashOf(): kotlin.ULong
	companion object {
		@kotlin.jvm.JvmStatic fun new(name: kotlin.String): api.Key
	}
}
class ClashSeed internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun value(): kotlin.UInt
	fun bool(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun default(): api.ClashSeed
		@kotlin.jvm.JvmStatic fun default_(): api.ClashSeed
	}
}
class ClashEmptyString internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun isEmpty(): kotlin.Boolean
}
class ClashStore internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun free(): kotlin.UInt
	fun close_(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(count: kotlin.UInt): api.ClashStore
	}
}
class TlsSession internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun alpn(): kotlin.String
	fun lifetime(): api.TlsPolicyLifetime
}
class Url internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	final override fun toString(): kotlin.String
	fun debug(): kotlin.String
	fun clone(): api.Url
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(StringException::class) fun parse(input: kotlin.String): api.Url
	}
}
class Json internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun toText(): kotlin.String
	companion object {
		@kotlin.jvm.JvmStatic @kotlin.jvm.Throws(StringException::class) fun parse(input: kotlin.String): api.Json
	}
}
class Instant internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	fun clone(): api.Instant
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class StdIoError internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class TlsConfig internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.TlsConfig
}
class TlsCert internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class TlsResolver internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun clone(): api.TlsResolver
}
class TlsCodec internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
}
class Spawned internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun join(): kotlin.UInt
	companion object {
		@kotlin.jvm.JvmStatic fun new(task: () -> kotlin.UInt): api.Spawned
	}
}
class LogThread internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun join(): kotlin.Boolean
	companion object {
		@kotlin.jvm.JvmStatic fun new(log: api.Log, line: kotlin.String): api.LogThread
	}
}
class Pinned internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	val valueGet: kotlin.ULong
	fun valueSet(value: kotlin.ULong)
	fun part(): api.Part
	companion object {
		@kotlin.jvm.JvmStatic fun new(value: kotlin.ULong): api.Pinned
	}
}
class Hooked internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val nGet: kotlin.ULong
	fun nSet(value: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic fun new(n: kotlin.ULong): api.Hooked
	}
}
class Forged internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val nGet: kotlin.ULong
	fun nSet(value: kotlin.ULong)
	companion object {
		@kotlin.jvm.JvmStatic fun new(n: kotlin.ULong): api.Forged
	}
}
class Part internal constructor(raw: Long, lent: Boolean = false) : AutoCloseable {
	override fun close()
	fun debug(): kotlin.String
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
	val valueGet: kotlin.ULong
	fun valueSet(value: kotlin.ULong)
}
interface Log {
	fun log(level: kotlin.UByte, line: kotlin.String): kotlin.Boolean
}
interface Beat {
	fun beat(): kotlin.UInt
}
data class Tuple1<T1>(val first: T1) {
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Tuple4<T1, T2, T3, T4>(val first: T1, val second: T2, val third: T3, val fourth: T4) {
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
data class Tuple2<T1, T2>(val first: T1, val second: T2) {
	override fun equals(other: kotlin.Any?): kotlin.Boolean
	override fun hashCode(): kotlin.Int
}
class BoundsU64Exception(val error: api.BoundsU64) : Exception() {
	override val message: String
}
class ErrorException(val error: api.Error) : Exception() {
	override val message: String
}
class RefusalException(val error: api.Refusal) : Exception() {
	override val message: String
}
class StringException(val error: kotlin.String) : Exception() {
	override val message: String
}
class kotlincollectionsListClashToneException(val error: kotlin.collections.List<api.ClashTone>) : Exception() {
	override val message: String
}
const val LIMIT: ULong
const val FAR: Long
fun hasText(note: api.Note): kotlin.Boolean
fun marked(apiMarked: api.Mark): kotlin.ULong
fun reading(n: kotlin.UInt): api.Reading?
fun readingDebug(n: kotlin.UInt): kotlin.String
fun readingEcho(reading: api.Reading?): api.Reading?
fun readingBump(reading: api.Mut<api.Reading?>)
fun advance(stage: api.Stage): api.Stage
@kotlin.jvm.Throws(StringException::class) fun sameObject(a: kotlin.Any?, b: kotlin.Any?): kotlin.Boolean
@kotlin.jvm.Throws(StringException::class) fun thrown(fail: kotlin.Boolean): kotlin.ULong
@kotlin.jvm.Throws(StringException::class) fun thrownLogger(sink: api.Log): api.Logger
fun only(counter: api.Counter): api.Tuple1<api.Counter>
fun widths(n: kotlin.ULong): api.Tuple4<kotlin.UByte, kotlin.Short, kotlin.UInt, kotlin.ULong>
@kotlin.jvm.Throws(ErrorException::class) fun divide(a: kotlin.ULong, b: kotlin.ULong): kotlin.Pair<kotlin.ULong, kotlin.ULong>
@kotlin.jvm.Throws(ErrorException::class) fun corner(unit: kotlin.Boolean): kotlin.Pair<kotlin.UInt, kotlin.UInt>
suspend fun cornerLater(): kotlin.Pair<kotlin.UInt, kotlin.UInt>
fun spread(n: kotlin.ULong): api.Tuple4<kotlin.UByte, kotlin.Short, kotlin.UInt, kotlin.ULong>?
fun find(haystack: api.Bytes, needle: api.Bytes): kotlin.Pair<kotlin.ULong, kotlin.ULong>?
fun rows(points: kotlin.collections.List<api.Point>): kotlin.collections.List<kotlin.Triple<kotlin.String, api.Shape, api.Point>>
@kotlin.jvm.Throws(StringException::class) fun port(text: kotlin.String): kotlin.UShort
fun span(start: kotlin.UInt, end: kotlin.UInt): kotlin.UInt
fun halves(span: api.Span): kotlin.Pair<kotlin.Pair<kotlin.UInt, kotlin.UInt>, kotlin.Pair<kotlin.UInt, kotlin.UInt>>
fun doubled(value: api.Tuple1<kotlin.UInt?>?): api.Tuple1<kotlin.UInt?>?
fun title(formal: kotlin.Boolean): api.Tuple1<kotlin.String>?
fun named(name: kotlin.String): api.Tuple1<kotlin.String>?
fun lentBack(value: api.Tuple1<api.Bytes>): api.Tuple1<api.Bytes>
fun boxedTuple(bytes: api.Bytes): api.Tuple1<api.Bytes>
fun swapped(pair: kotlin.Pair<kotlin.String, kotlin.UInt>): kotlin.Pair<kotlin.UInt, kotlin.String>
fun lasting(draft: api.Draft): kotlin.Boolean
fun stored(receipt: api.Receipt): kotlin.UInt
fun tallied(receipt: api.Receipt): kotlin.UInt
fun summedUpReceipt(totaled: api.Receipt): kotlin.UInt
fun erased(receipt: api.Receipt): kotlin.UInt
fun longer(a: api.Name, b: api.Name): api.Name
fun unnamed(): api.Name
fun total(range: api.RangeU64): kotlin.ULong
fun reach(start: kotlin.UInt, end: kotlin.UInt): kotlin.UInt
fun scaled(v: java.math.BigInteger, by: kotlin.ULong): java.math.BigInteger?
fun pair(): api.Batch2
fun drain4(batch: api.Batch): kotlin.ULong
fun drain2(batch: api.Batch2): kotlin.ULong
fun capacities44(a: api.Batch, b: api.Batch): kotlin.ULong
fun capacities42(a: api.Batch, b: api.Batch2): kotlin.ULong
fun capacities24(a: api.Batch2, b: api.Batch): kotlin.ULong
fun capacities22(a: api.Batch2, b: api.Batch2): kotlin.ULong
suspend fun fill4(batch: api.Batch, v: kotlin.UInt): kotlin.UIntArray
suspend fun fill2(batch: api.Batch2, v: kotlin.UInt): kotlin.UIntArray
fun widen(range: api.RangeU64, by: kotlin.ULong): api.RangeU64
fun count(range: api.BoundsU64): kotlin.ULong
fun end(limit: kotlin.ULong): api.BoundU64
fun ends(limit: kotlin.ULong): kotlin.Pair<api.BoundU64, api.BoundU64>
@kotlin.jvm.Throws(BoundsU64Exception::class) fun below(n: kotlin.ULong, limit: kotlin.ULong): kotlin.ULong
suspend fun drive(start: api.Moment, poll: (api.Moment, api.Waiter) -> api.Moment?): kotlin.String
fun made(n: kotlin.UInt, make: (kotlin.UInt) -> kotlin.String): kotlin.String
fun tickets(entries: kotlin.collections.List<kotlin.Pair<kotlin.String, api.Ticket>>): kotlin.collections.List<kotlin.Pair<kotlin.String, api.Ticket>>
fun reissue(ticket: api.Ticket, swap: (api.Ticket) -> api.Ticket): api.Ticket
fun reissueLent(ticket: api.Ticket, swap: (api.Ticket) -> api.Ticket): api.Ticket
fun seat(mark: api.Mark, row: kotlin.UInt): api.Seat
fun admit(entry: api.Tuple4<api.Counter, api.Mark, api.Seat, api.Choice>): kotlin.ULong
fun choose(choice: api.Choice): kotlin.ULong
fun heft(marked: kotlin.Pair<api.Mark, api.Bytes>, lent: api.Parcel, moved: kotlin.Pair<api.Counter, api.Bytes>): kotlin.ULong
@kotlin.jvm.Throws(StringException::class) fun spend(e: api.Counter, it: api.Counter): kotlin.ULong
fun isKept(status: api.Status): kotlin.Boolean
@kotlin.jvm.Throws(StringException::class) fun attempt(check: () -> kotlin.Unit)
@kotlin.jvm.Throws(StringException::class) fun tail(file: java.nio.channels.SeekableByteChannel, len: kotlin.ULong): kotlin.Pair<api.Bytes, kotlin.ULong>
@kotlin.jvm.Throws(StringException::class) fun seekTo(file: java.nio.channels.SeekableByteChannel, at: kotlin.ULong): kotlin.ULong
suspend fun halved(n: kotlin.UInt): kotlin.UInt?
suspend fun yielded(value: kotlin.UInt): kotlin.UInt
suspend fun after(first: suspend () -> kotlin.Unit, value: kotlin.UInt): kotlin.UInt
@kotlin.jvm.Throws(ErrorException::class) suspend fun within(polls: kotlin.UInt, future: suspend () -> kotlin.Unit)
@kotlin.jvm.Throws(StringException::class) fun momentAt(ticks: kotlin.ULong, rate: kotlin.ULong): api.Moment
@kotlin.jvm.Throws(StringException::class) fun stampAt(millis: kotlin.ULong): api.Stamp
fun shifted(moment: api.Moment, shift: (api.Moment) -> api.Moment): api.Moment
fun jvmSum(field: api.JvmField, method: api.JvmStatic, opt: api.OptIn): kotlin.UInt
fun clockedAfter(clocked: api.Clocked, ticks: kotlin.ULong): api.Clocked
fun emit(log: api.Log, lines: api.Bytes): kotlin.UInt
@kotlin.jvm.Throws(ErrorException::class) suspend fun counted(n: kotlin.UInt, count: (kotlin.UInt) -> kotlin.Unit): kotlin.UInt
fun filter(lines: api.Bytes, keep: (kotlin.UByte, kotlin.String) -> kotlin.Boolean): kotlin.UInt
suspend fun logLater(log: api.Log): kotlin.Boolean
fun logCounts(log: api.Log, level: kotlin.UByte, counter: api.Counter, more: kotlin.collections.List<api.Counter>): kotlin.UInt
fun onFirstThread(): kotlin.Boolean
fun localsDropped(): kotlin.UInt
fun localOrNew(local: api.Local?): api.Local
suspend fun localOrNewLater(local: api.Local?): api.Local
fun bumpEither(first: api.Local?, second: api.Local): kotlin.UInt
suspend fun localAfter(): api.Local
suspend fun localLater(n: kotlin.UInt): kotlin.UInt
suspend fun pollTripled(n: kotlin.UInt): kotlin.UInt
fun watch(gauge: api.Gauge, watch: (api.Gauge) -> kotlin.ULong): kotlin.ULong
fun tokenBytes(id: kotlin.UInt, len: kotlin.UInt): api.Bytes
fun tokenSlice(id: kotlin.UInt, len: kotlin.UInt, of: kotlin.UInt): api.Bytes
fun nextPacket(packet: api.Packet): api.Packet
@kotlin.jvm.Throws(StringException::class) fun fingerprint(hex: kotlin.String): ArrayU8x4
fun dropped(): kotlin.UInt
fun bumpBoth(a: api.Tally, b: api.Tally): kotlin.UInt
fun offset(by: () -> kotlin.UInt, byData: kotlin.UInt): kotlin.UInt
fun shift(by: () -> kotlin.UInt, cBy: kotlin.UInt): kotlin.UInt
fun walk(steps: kotlin.ULong, visit: (api.Cursor) -> kotlin.ULong): kotlin.ULong
fun each(steps: kotlin.UInt, visit: (kotlin.UInt) -> kotlin.Unit): kotlin.UInt
fun peek(tally: api.Tally, peek: (api.Tally) -> kotlin.UInt): kotlin.UInt
fun len(payload: api.Bytes): kotlin.ULong
fun hex(payload: api.Bytes): kotlin.String
fun timedHex(payload: api.Bytes): kotlin.String
fun timedText(value: api.Json): kotlin.String
fun sampleHex(sample: api.Bytes): kotlin.String
fun queuedHex(queued: api.Bytes): kotlin.String
fun tickName(tick: api.Label): kotlin.String
fun rootedName(rooted: api.Label): kotlin.String
fun storedText(stored: kotlin.String): kotlin.String
fun layerName(layer: api.Label): kotlin.String
fun greet(name: kotlin.String, shape: api.Shape): kotlin.String
fun describe(event: api.Event): kotlin.String
fun tag(size: api.Size, tag: kotlin.UByte): api.PairSizeU8
fun cross(left: api.LeftRightU8, right: api.RightLeftU8): kotlin.UByte
fun keptPoint(from: api.Point): kotlin.ULong
fun keptCounter(from: api.Counter): kotlin.ULong
fun keptSize(from: api.Size): kotlin.ULong
fun keptWrapU32(from: api.WrapU32): kotlin.ULong
fun keptPolarPoint(from: api.PolarPoint): kotlin.ULong
fun keepPoint(kept: api.Point): kotlin.Int
fun keptOrPoint(kept: api.Point?): kotlin.Int
fun keeperPoint(keeper: api.Point): kotlin.Int
val BOXED: api.BoxedU16
fun keeps(log: api.Log): api.BoxedU8
fun weighPoint(from: api.Point): kotlin.UInt
fun applyNonZeroU16(pair: api.PairNonZeroU16, f: (kotlin.UShort) -> kotlin.UInt): kotlin.UInt
fun applyU32(pair: api.PairU32, f: (kotlin.UInt) -> kotlin.UInt): kotlin.UInt
fun reportNonZeroU16(pair: api.PairNonZeroU16, log: api.Log): kotlin.UInt
fun reportU32(pair: api.PairU32, log: api.Log): kotlin.UInt
fun tallyLogCount(log: api.Log): kotlin.UInt
fun wind(whole: api.PolarTurn, part: api.Polar_Turn): kotlin.UInt
fun area(size: api.Size): kotlin.ULong
fun seedsPointPoint(from: api.Point): kotlin.ULong
fun seedsPointCounter(from: api.Counter): kotlin.ULong
fun seedsPointSize(from: api.Size): kotlin.ULong
fun seedsPointWrapU32(from: api.WrapU32): kotlin.ULong
fun seedsPointPolarPoint(from: api.PolarPoint): kotlin.ULong
fun seedsCounterPoint(from: api.Point): kotlin.ULong
fun seedsCounterCounter(from: api.Counter): kotlin.ULong
fun seedsCounterSize(from: api.Size): kotlin.ULong
fun seedsCounterWrapU32(from: api.WrapU32): kotlin.ULong
fun seedsCounterPolarPoint(from: api.PolarPoint): kotlin.ULong
fun grow(area: api.Area, by: kotlin.UInt): api.Area
fun wide(value: kotlin.UInt): api.SlotU32
fun host(url: api.Url): kotlin.String?
@kotlin.jvm.Throws(StringException::class) fun resolve(base: api.Url, path: kotlin.String): api.Url
fun ports(addrs: kotlin.collections.List<api.SocketAddr>): kotlin.UShortArray
fun doubledTtl(ttl: api.Duration): api.Duration
fun file(dir: api.PathBuf, name: kotlin.String): api.PathBuf
fun stray(): api.PathBuf
fun visitStray(visit: (api.PathBuf) -> kotlin.Unit)
fun nextChar(c: kotlin.Int): kotlin.Int?
fun extended(duration: api.Duration, by: api.Duration): api.Duration?
fun parents(paths: kotlin.collections.List<api.PathBuf>): kotlin.collections.List<api.PathBuf?>
fun words(text: kotlin.String): kotlin.collections.List<kotlin.String>
fun sentence(words: kotlin.collections.List<kotlin.String>?): kotlin.String
fun putNumber(value: kotlin.UInt, buf: api.Mut<api.Bytes>)
fun secondLater(w235: api.Duration): api.Duration
@kotlin.jvm.Throws(ErrorException::class) fun digit(r: api.Mut<api.Bytes>): kotlin.UByte
fun pushCounted(entry: api.Mut<kotlin.Pair<api.Bytes, kotlin.UInt>>, byte: kotlin.UByte): kotlin.UInt
fun nudge(point: api.Mut<api.Point>, by: kotlin.Int)
fun nudgeBy(point: api.Mut<api.Point>, refusal: api.Refusal)
fun secure(alpn: kotlin.String): api.TlsConfig
fun alpn(config: api.TlsConfig): kotlin.String
fun alpnLent(config: api.TlsConfig): kotlin.String
fun certify(der: api.Bytes): api.TlsCert
suspend fun byteLen(bytes: api.Bytes): kotlin.ULong
fun certLen(cert: api.TlsCert): kotlin.ULong
fun certDer(cert: api.TlsCert): api.Bytes
fun chainLen(chain: kotlin.collections.List<api.TlsCert>): kotlin.ULong
@kotlin.jvm.Throws(StringException::class) fun verify(cert: api.TlsCert): kotlin.UInt
@kotlin.jvm.Throws(StringException::class) fun parseCert(text: kotlin.String): kotlin.UInt
@kotlin.jvm.Throws(StringException::class) suspend fun parseLater(text: kotlin.String): kotlin.UInt
fun resolver(name: kotlin.String): api.TlsResolver
fun resolved(resolver: api.TlsResolver): kotlin.String
fun failure(message: kotlin.String): api.StdIoError
fun failureMessage(error: api.StdIoError): kotlin.String
fun failureKind(error: api.StdIoError): kotlin.String
fun sumAll(items: kotlin.collections.List<kotlin.UInt>): kotlin.UInt
fun weighAllCounter(items: kotlin.collections.List<api.Counter>): kotlin.ULong
fun offeredLen(alpns: kotlin.collections.List<api.Bytes>): kotlin.ULong
fun joinWords(words: kotlin.collections.List<kotlin.String>): kotlin.String
@kotlin.jvm.Throws(StringException::class) fun resume(alpn: kotlin.String, seconds: kotlin.UInt): api.TlsSession
fun either(text: kotlin.String?): kotlin.ULong
suspend fun eitherLater(text: kotlin.String?): kotlin.ULong
fun prefixed(payload: api.Bytes, prefix: api.Bytes?): api.Bytes
fun sumLists(a: kotlin.IntArray, b: kotlin.IntArray?, c: kotlin.IntArray?): kotlin.Int
fun coordinates(point: api.Point?): kotlin.Int
fun chunks(bytes: api.Bytes, size: kotlin.ULong): kotlin.collections.List<api.Bytes>
fun counters(names: kotlin.collections.List<kotlin.String>): kotlin.collections.List<api.Counter>
fun worth(some: kotlin.collections.List<api.Counter>?, boxed: kotlin.collections.List<api.Counter>): kotlin.ULong
fun heldCount(held: api.Tuple1<api.Counter>?): kotlin.ULong?
fun boxedCounter(name: kotlin.String?): api.Counter?
fun boxedName(boxed: api.Counter?): kotlin.String?
fun less(value: kotlin.UInt, value2: kotlin.UInt): kotlin.UInt
fun sums(rows: kotlin.collections.List<kotlin.UIntArray>): kotlin.UIntArray
fun least(sets: kotlin.collections.List<kotlin.collections.List<kotlin.UInt>>): kotlin.collections.List<kotlin.collections.List<kotlin.UInt>>
fun splitLast(items: kotlin.IntArray): api.Tuple2<kotlin.IntArray, kotlin.UInt>
fun sumU32(items: kotlin.UIntArray): kotlin.ULong
fun doubledI64(items: kotlin.LongArray): kotlin.LongArray
fun boxedFloats(items: kotlin.DoubleArray): kotlin.DoubleArray
fun summed(points: kotlin.collections.List<api.Point>): api.Point
fun breadth(limits: api.LimitsDuration): api.Duration
fun byteLimits(low: api.Bytes, high: api.Bytes): api.LimitsBytes
fun joined(parts: kotlin.collections.List<api.Bytes>): api.Bytes
fun firstOf(items: kotlin.collections.List<kotlin.String>): kotlin.String?
fun readLen(reader: api.Bytes): kotlin.ULong
fun longest(words: kotlin.collections.List<kotlin.String>): kotlin.ULong
fun encode(value: api.Json): kotlin.String
fun tagged(doc: api.Json, tag: kotlin.String): api.Json
fun entries(entries: kotlin.collections.List<kotlin.Pair<kotlin.String, api.Json>>, tag: kotlin.String): kotlin.collections.List<kotlin.Pair<kotlin.String, api.Json>>
fun sections(entries: kotlin.collections.List<kotlin.Pair<kotlin.String, api.Json>>): kotlin.collections.List<kotlin.Pair<kotlin.String, api.Json>>
fun now(): api.Instant
fun later(at: api.Instant, by: api.Duration): api.Instant
fun since(later: api.Instant, earlier: api.Instant): api.Duration
fun hasInstant(at: api.Instant?): kotlin.Boolean
fun louder(filter: api.Level?): api.Level?
fun spell(filter: api.Level?): kotlin.String
val GREETINGS: kotlin.collections.List<kotlin.String>
val PRIMES: kotlin.UIntArray
fun numbered(names: kotlin.collections.List<kotlin.String>): kotlin.collections.List<api.Counter>
fun nexts(v4: api.SocketAddrV4, v6: api.SocketAddrV6): kotlin.Pair<api.SocketAddrV4, api.SocketAddrV6>
fun nextIps(v4: api.Ipv4Addr, v6: api.Ipv6Addr): kotlin.Pair<api.Ipv4Addr, api.Ipv6Addr>
fun flip(ip: api.IpAddr): api.IpAddr
fun reasons(pairs: kotlin.collections.List<kotlin.Pair<api.Counter, api.Refusal>>, last: kotlin.Pair<api.Counter, api.Refusal>?): kotlin.String
fun excuse(note: api.Bytes, entry: kotlin.Pair<api.Bytes, api.Refusal>): kotlin.ULong
fun relay(log: api.Log, refusals: kotlin.collections.List<api.Refusal>): kotlin.UInt
@kotlin.jvm.Throws(RefusalException::class) fun refuse(reason: kotlin.String): kotlin.UInt
fun platform(): api.Platform
fun pointerBits(platform: api.Platform): kotlin.UInt
fun sameLabelled(labelled: api.Labelled): api.Labelled
fun placement(): api.Placement
fun isFamily(family: api.Family): kotlin.Boolean
fun wasmOnly(): kotlin.UInt
@kotlin.jvm.Throws(ErrorException::class) fun wasmTake(counter: api.Counter): kotlin.ULong
fun wasmLog(log: api.Log)
fun boothCount(booth: api.Booth): kotlin.ULong
fun page(): api.Page
fun beats(beat: api.Beat): kotlin.UInt
fun same(a: kotlin.Double, b: kotlin.Double): kotlin.Int
@kotlin.jvm.Throws(ErrorException::class) fun returnValues(point: api.Mut<api.Point>, fail: kotlin.Boolean): kotlin.Triple<kotlin.String, api.Tuple4<kotlin.UByte, kotlin.Short, kotlin.UInt, kotlin.ULong>?, kotlin.collections.List<api.Bytes>>
@kotlin.jvm.Throws(ErrorException::class) fun returnMoved(counter: api.Counter, fail: kotlin.Boolean): api.Counter
fun nonZeros(): api.NonZeros
fun describeNonZeros(nonZeros: api.NonZeros): kotlin.String
fun nextHop(hop: kotlin.ULong?): kotlin.ULong?
fun fartherHop(hops: api.PairNonZeroU16): kotlin.UShort
fun sameDuration(v: api.Duration): api.Duration
fun sameU64(v: kotlin.ULong): kotlin.ULong
fun sameUsize(v: kotlin.ULong): kotlin.ULong
fun sameIsize(v: kotlin.Long): kotlin.Long
fun sameU128(v: java.math.BigInteger): java.math.BigInteger
fun sameChar(v: kotlin.Int): kotlin.Int
fun sameString(v: kotlin.String): kotlin.String
fun samePath(v: api.PathBuf): api.PathBuf
fun sameSocketAddr(v: api.SocketAddr): api.SocketAddr
fun sameSocketAddrV4(v: api.SocketAddrV4): api.SocketAddrV4
fun sameSocketAddrV6(v: api.SocketAddrV6): api.SocketAddrV6
fun sameIpAddr(v: api.IpAddr): api.IpAddr
fun crowded(reading: api.Reading, a: api.Stamp, b: api.Stamp, n0: kotlin.ULong, n1: kotlin.ULong, n2: kotlin.ULong, n3: kotlin.ULong, n4: kotlin.ULong, n5: kotlin.ULong, n6: kotlin.ULong, n7: kotlin.ULong, n8: kotlin.ULong, n9: kotlin.ULong, n10: kotlin.ULong, n11: kotlin.ULong, n12: kotlin.ULong, n13: kotlin.ULong, n14: kotlin.ULong, n15: kotlin.ULong, n16: kotlin.ULong, n17: kotlin.ULong, n18: kotlin.ULong, n19: kotlin.ULong, n20: kotlin.ULong, n21: kotlin.ULong, n22: kotlin.ULong, n23: kotlin.ULong, n24: kotlin.ULong, n25: kotlin.ULong, n26: kotlin.ULong, n27: kotlin.ULong, n28: kotlin.ULong, n29: kotlin.ULong, n30: kotlin.ULong, n31: kotlin.ULong, n32: kotlin.ULong, n33: kotlin.ULong, n34: kotlin.ULong, n35: kotlin.ULong, n36: kotlin.ULong, n37: kotlin.ULong, n38: kotlin.ULong, n39: kotlin.ULong, n40: kotlin.ULong, n41: kotlin.ULong, n42: kotlin.ULong, n43: kotlin.ULong, n44: kotlin.ULong, n45: kotlin.ULong, n46: kotlin.ULong, n47: kotlin.ULong, n48: kotlin.ULong, n49: kotlin.ULong, n50: kotlin.ULong, n51: kotlin.ULong, n52: kotlin.ULong, n53: kotlin.ULong, n54: kotlin.ULong, n55: kotlin.ULong, n56: kotlin.ULong, n57: kotlin.ULong, n58: kotlin.ULong, n59: kotlin.ULong, n60: kotlin.ULong, n61: kotlin.ULong, n62: kotlin.ULong, n63: kotlin.ULong, n64: kotlin.ULong, n65: kotlin.ULong, n66: kotlin.ULong, n67: kotlin.ULong, n68: kotlin.ULong, n69: kotlin.ULong, n70: kotlin.ULong, n71: kotlin.ULong, n72: kotlin.ULong, n73: kotlin.ULong, n74: kotlin.ULong, n75: kotlin.ULong, n76: kotlin.ULong, n77: kotlin.ULong, n78: kotlin.ULong, n79: kotlin.ULong, n80: kotlin.ULong, n81: kotlin.ULong, n82: kotlin.ULong, n83: kotlin.ULong, n84: kotlin.ULong, n85: kotlin.ULong, n86: kotlin.ULong, n87: kotlin.ULong, n88: kotlin.ULong, n89: kotlin.ULong, n90: kotlin.ULong, n91: kotlin.ULong, n92: kotlin.ULong, n93: kotlin.ULong, n94: kotlin.ULong, n95: kotlin.ULong, n96: kotlin.ULong, n97: kotlin.ULong, n98: kotlin.ULong, n99: kotlin.ULong, n100: kotlin.ULong, n101: kotlin.ULong, n102: kotlin.ULong, n103: kotlin.ULong, n104: kotlin.ULong, n105: kotlin.ULong, n106: kotlin.ULong, n107: kotlin.ULong, n108: kotlin.ULong, n109: kotlin.ULong, n110: kotlin.ULong, n111: kotlin.ULong, n112: kotlin.ULong, n113: kotlin.ULong, n114: kotlin.ULong, n115: kotlin.ULong, n116: kotlin.ULong, n117: kotlin.ULong, n118: kotlin.ULong): kotlin.ULong
fun crammed(a: api.Moment, b: api.Moment, n0: kotlin.ULong, n1: kotlin.ULong, n2: kotlin.ULong, n3: kotlin.ULong, n4: kotlin.ULong, n5: kotlin.ULong, n6: kotlin.ULong, n7: kotlin.ULong, n8: kotlin.ULong, n9: kotlin.ULong, n10: kotlin.ULong, n11: kotlin.ULong, n12: kotlin.ULong, n13: kotlin.ULong, n14: kotlin.ULong, n15: kotlin.ULong, n16: kotlin.ULong, n17: kotlin.ULong, n18: kotlin.ULong, n19: kotlin.ULong, n20: kotlin.ULong, n21: kotlin.ULong, n22: kotlin.ULong, n23: kotlin.ULong, n24: kotlin.ULong, n25: kotlin.ULong, n26: kotlin.ULong, n27: kotlin.ULong, n28: kotlin.ULong, n29: kotlin.ULong, n30: kotlin.ULong, n31: kotlin.ULong, n32: kotlin.ULong, n33: kotlin.ULong, n34: kotlin.ULong, n35: kotlin.ULong, n36: kotlin.ULong, n37: kotlin.ULong, n38: kotlin.ULong, n39: kotlin.ULong, n40: kotlin.ULong, n41: kotlin.ULong, n42: kotlin.ULong, n43: kotlin.ULong, n44: kotlin.ULong, n45: kotlin.ULong, n46: kotlin.ULong, n47: kotlin.ULong, n48: kotlin.ULong, n49: kotlin.ULong, n50: kotlin.ULong, n51: kotlin.ULong, n52: kotlin.ULong, n53: kotlin.ULong, n54: kotlin.ULong, n55: kotlin.ULong, n56: kotlin.ULong, n57: kotlin.ULong, n58: kotlin.ULong, n59: kotlin.ULong, n60: kotlin.ULong, n61: kotlin.ULong, n62: kotlin.ULong, n63: kotlin.ULong, n64: kotlin.ULong, n65: kotlin.ULong, n66: kotlin.ULong, n67: kotlin.ULong, n68: kotlin.ULong, n69: kotlin.ULong, n70: kotlin.ULong, n71: kotlin.ULong, n72: kotlin.ULong, n73: kotlin.ULong, n74: kotlin.ULong, n75: kotlin.ULong, n76: kotlin.ULong, n77: kotlin.ULong, n78: kotlin.ULong, n79: kotlin.ULong, n80: kotlin.ULong, n81: kotlin.ULong, n82: kotlin.ULong, n83: kotlin.ULong, n84: kotlin.ULong, n85: kotlin.ULong, n86: kotlin.ULong, n87: kotlin.ULong, n88: kotlin.ULong, n89: kotlin.ULong, n90: kotlin.ULong, n91: kotlin.ULong, n92: kotlin.ULong, n93: kotlin.ULong, n94: kotlin.ULong, n95: kotlin.ULong, n96: kotlin.ULong, n97: kotlin.ULong, n98: kotlin.ULong, n99: kotlin.ULong, n100: kotlin.ULong, n101: kotlin.ULong, n102: kotlin.ULong, n103: kotlin.ULong, n104: kotlin.ULong, n105: kotlin.ULong, n106: kotlin.ULong, n107: kotlin.ULong, n108: kotlin.ULong, n109: kotlin.ULong, n110: kotlin.ULong, n111: kotlin.ULong, n112: kotlin.ULong, n113: kotlin.ULong, n114: kotlin.ULong, n115: kotlin.ULong, n116: kotlin.ULong, n117: kotlin.ULong, n118: kotlin.ULong, n119: kotlin.ULong, n120: kotlin.ULong, n121: kotlin.ULong, n122: kotlin.ULong, n123: kotlin.ULong): kotlin.ULong
fun jniOne(): kotlin.UInt
fun polarPoint(distance: kotlin.UInt): api.PolarPoint?
fun clashAny(typeUrl: kotlin.String): api.ClashAny
fun clashCandidate(at: kotlin.UByte): api.ClashCandidate
fun clashIgnore(arg0: kotlin.UByte, arg1: kotlin.UByte, arg2: kotlin.UByte): kotlin.UByte
fun clashEmpty(): api.ClashEmptyString
@kotlin.jvm.Throws(kotlincollectionsListClashToneException::class) fun clashAudible(tone: api.ClashTone): api.ClashTone
fun clashNested(a: kotlin.Pair<kotlin.Pair<kotlin.UByte, kotlin.UShort>, kotlin.UInt>?, b: api.Tuple1<kotlin.Triple<kotlin.UByte, kotlin.UShort, kotlin.UInt>>?): kotlin.UInt
fun clashAudioEncoderFinish(): kotlin.UByte
fun clashAudioEncoderFinish_(): kotlin.UByte
fun emitFromThread(log: api.Log, lines: api.Bytes): kotlin.UInt
suspend fun emitLater(log: api.Log, lines: api.Bytes): kotlin.UInt
suspend fun logOnDrop(log: api.Log)
fun logWaker(log: api.Log): api.Waker
fun hook(f: () -> kotlin.Unit)
fun unhook()
interface StartSeekable {
	fun seekStart(offset: ULong): ULong
}
