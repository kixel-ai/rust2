// The runtime's public types, which every generated Kotlin file declares at its top level:
// Rust's std values no Kotlin or Java type holds exactly, each exact by construction, with
// checked conversions to and from the idiomatic type that throw `IllegalArgumentException`
// for a value the other cannot hold. Every free function is qualified, as an export may
// take its name.

/**
 * A Rust `Duration`: whole seconds, any `u64`, and nanoseconds below one second, so it holds
 * every Rust duration exactly, where a `kotlin.time.Duration` past about 146 years drops
 * what is below a millisecond and one past `Long.MAX_VALUE` seconds is infinite.
 */
class Duration(val secs: ULong, val nanos: UInt = 0u) : kotlin.Comparable<Duration> {
	init {
		kotlin.require(nanos < NANOS) { "$nanos nanoseconds is not below a second" }
	}

	/** [duration] exactly, throwing `IllegalArgumentException` for a negative or infinite one, which Rust's has no form of. */
	constructor(duration: kotlin.time.Duration) : this(seconds(duration), duration.toComponents { _, nanos -> nanos.toUInt() })

	/** This as a `kotlin.time.Duration`, throwing `IllegalArgumentException` where one cannot hold it exactly. */
	fun toKotlinDuration(): kotlin.time.Duration {
		val d = when {
			secs > Long.MAX_VALUE.toULong() -> kotlin.time.Duration.INFINITE
			else -> secs.toLong().toDuration(kotlin.time.DurationUnit.SECONDS) + nanos.toInt().toDuration(kotlin.time.DurationUnit.NANOSECONDS)
		}
		val exact = d.isFinite() && d.toComponents { s, n -> s.toULong() == secs && n.toUInt() == nanos }
		kotlin.require(exact) { "$this has no exact kotlin.time.Duration" }
		return d
	}

	override fun compareTo(other: Duration): Int = when (val c = secs.compareTo(other.secs)) {
		0 -> nanos.compareTo(other.nanos)
		else -> c
	}

	override fun equals(other: Any?): Boolean = other is Duration && secs == other.secs && nanos == other.nanos

	override fun hashCode(): Int = 31 * secs.hashCode() + nanos.hashCode()

	/** Its seconds and nanoseconds, as `1.500000000s`. */
	override fun toString(): String = "$secs.${nanos.toString().padStart(9, '0')}s"

	companion object {
		private const val NANOS = 1_000_000_000u

		/** No time. */
		val ZERO = Duration(0u)

		/** The longest, as Rust's `Duration::MAX`. */
		val MAX = Duration(ULong.MAX_VALUE, NANOS - 1u)

		private fun seconds(duration: kotlin.time.Duration): ULong {
			kotlin.require(duration.isFinite() && !duration.isNegative()) { "$duration has no Rust Duration" }
			return duration.inWholeSeconds.toULong()
		}
	}
}

/** A Rust `IpAddr`: an [Ipv4Addr] or an [Ipv6Addr], each of its own family, as Rust keeps it. */
sealed interface IpAddr {
	/** This as a `java.net.InetAddress` of its family: an IPv4-mapped IPv6 one stays IPv6. */
	fun toInetAddress(): java.net.InetAddress

	companion object {
		/** [address] exactly, throwing `IllegalArgumentException` for an IPv6 one with a scope, which Rust's `IpAddr` has no room for. */
		@kotlin.jvm.JvmStatic
		fun of(address: java.net.InetAddress): IpAddr = when (address) {
			is java.net.Inet6Address -> Ipv6Addr(address)
			else -> Ipv4Addr(address.address)
		}
	}
}

/** A Rust `Ipv4Addr`: its 32 bits, the first octet highest, as Rust's `to_bits` has them. */
@kotlin.jvm.JvmInline
value class Ipv4Addr(val bits: UInt) : IpAddr {
	/** The address of [octets], which throws `IllegalArgumentException` unless there are 4. */
	constructor(octets: ByteArray) : this(_octets(octets, 4).let { java.nio.ByteBuffer.wrap(it).getInt().toUInt() })

	/** The address of [a].[b].[c].[d]. */
	constructor(a: UByte, b: UByte, c: UByte, d: UByte) : this((a.toUInt() shl 24) or (b.toUInt() shl 16) or (c.toUInt() shl 8) or d.toUInt())

	/** Its 4 octets. */
	fun octets(): ByteArray = java.nio.ByteBuffer.allocate(4).putInt(bits.toInt()).array()

	override fun toInetAddress(): java.net.InetAddress = java.net.InetAddress.getByAddress(octets())

	/** As Rust prints it: `127.0.0.1`. */
	override fun toString(): String = octets().joinToString(".") { (it.toInt() and 0xff).toString() }
}

/** A Rust `Ipv6Addr`: its 16 octets. */
class Ipv6Addr internal constructor(internal val hi: Long, internal val lo: Long) : IpAddr {
	/** The address of [octets], which throws `IllegalArgumentException` unless there are 16. */
	constructor(octets: ByteArray) : this(java.nio.ByteBuffer.wrap(_octets(octets, 16)))

	/** [address], throwing `IllegalArgumentException` for one with a scope, which Rust's `Ipv6Addr` has no room for. */
	constructor(address: java.net.Inet6Address) : this(_unscoped(address).address)

	private constructor(b: java.nio.ByteBuffer) : this(b.getLong(), b.getLong())

	/** Its 16 octets. */
	fun octets(): ByteArray = java.nio.ByteBuffer.allocate(16).putLong(hi).putLong(lo).array()

	override fun toInetAddress(): java.net.Inet6Address = java.net.Inet6Address.getByAddress(null, octets(), null as java.net.NetworkInterface?)

	override fun equals(other: Any?): Boolean = other is Ipv6Addr && hi == other.hi && lo == other.lo

	override fun hashCode(): Int = 31 * hi.hashCode() + lo.hashCode()

	/** As Rust prints it: the longest run of zero segments, if more than one, as `::`, and an IPv4-mapped one as `::ffff:1.2.3.4`. */
	override fun toString(): String {
		if (hi == 0L && lo ushr 32 == 0xffffL) return "::ffff:${Ipv4Addr(lo.toUInt())}"
		val segments = IntArray(8) { k -> ((if (k < 4) hi else lo) ushr (48 - 16 * (k % 4))).toInt() and 0xffff }
		var (start, len) = 0 to 0
		var at = 0
		while (at < 8) {
			var end = at
			while (end < 8 && segments[end] == 0) end++
			if (end - at > len) {
				start = at
				len = end - at
			}
			at = end + 1
		}
		val hex = { from: Int, until: Int -> (from until until).joinToString(":") { segments[it].toString(16) } }
		return if (len > 1) "${hex(0, start)}::${hex(start + len, 8)}" else hex(0, 8)
	}
}

/** A Rust `SocketAddr`: a [SocketAddrV4] or a [SocketAddrV6]. */
sealed interface SocketAddr {
	/** Its port. */
	val port: UShort

	/** This as a `java.net.InetSocketAddress`, throwing `IllegalArgumentException` for an IPv6 one with a flow label, which Java has no room for. */
	fun toInetSocketAddress(): java.net.InetSocketAddress

	companion object {
		/** [address], throwing `IllegalArgumentException` for an unresolved one, which Rust's `SocketAddr` has no form of. */
		@kotlin.jvm.JvmStatic
		fun of(address: java.net.InetSocketAddress): SocketAddr = when (_resolved(address)) {
			is java.net.Inet6Address -> SocketAddrV6(address)
			else -> SocketAddrV4(address)
		}
	}
}

/** A Rust `SocketAddrV4`: an [Ipv4Addr] and a port. */
@kotlin.jvm.JvmInline
value class SocketAddrV4 private constructor(private val bits: Long) : SocketAddr {
	constructor(ip: Ipv4Addr, port: UShort) : this((ip.bits.toLong() shl 16) or port.toLong())

	/** [address], throwing `IllegalArgumentException` unless it is resolved and IPv4. */
	constructor(address: java.net.InetSocketAddress) : this(
		Ipv4Addr((_resolved(address) as? java.net.Inet4Address ?: throw kotlin.IllegalArgumentException("$address is not IPv4")).address),
		address.port.toUShort(),
	)

	val ip: Ipv4Addr get() = Ipv4Addr((bits ushr 16).toUInt())

	override val port: UShort get() = bits.toUShort()

	override fun toInetSocketAddress(): java.net.InetSocketAddress = java.net.InetSocketAddress(ip.toInetAddress(), port.toInt())

	/** As Rust prints it: `127.0.0.1:443`. */
	override fun toString(): String = "$ip:$port"
}

/** A Rust `SocketAddrV6`: an [Ipv6Addr], a port, a flow label, and a scope, which Java keeps no flow label of. */
class SocketAddrV6(val ip: Ipv6Addr, override val port: UShort, val flowinfo: UInt = 0u, val scopeId: UInt = 0u) : SocketAddr {
	/** [address], with no flow label, throwing `IllegalArgumentException` unless it is resolved and IPv6. */
	constructor(address: java.net.InetSocketAddress) : this(
		(_resolved(address) as? java.net.Inet6Address ?: throw kotlin.IllegalArgumentException("$address is not IPv6")).let { Ipv6Addr(it.address) },
		address.port.toUShort(),
		0u,
		(address.address as java.net.Inet6Address).scopeId.toUInt(),
	)

	override fun toInetSocketAddress(): java.net.InetSocketAddress {
		kotlin.require(flowinfo == 0u) { "$this has a flow label, which a java.net.InetSocketAddress has no room for" }
		// Java takes a scope of an `Int` and ignores a negative one.
		kotlin.require(scopeId <= Int.MAX_VALUE.toUInt()) { "$this has a scope past Int.MAX_VALUE, which a java.net.Inet6Address has no room for" }
		val ip = when (scopeId) {
			0u -> ip.toInetAddress()
			else -> java.net.Inet6Address.getByAddress(null, ip.octets(), scopeId.toInt())
		}
		return java.net.InetSocketAddress(ip, port.toInt())
	}

	override fun equals(other: Any?): Boolean =
		other is SocketAddrV6 && ip == other.ip && port == other.port && flowinfo == other.flowinfo && scopeId == other.scopeId

	override fun hashCode(): Int = ((ip.hashCode() * 31 + port.hashCode()) * 31 + flowinfo.hashCode()) * 31 + scopeId.hashCode()

	/** As Rust prints it: `[::1%3]:443`, its scope shown unless 0, its flow label never. */
	override fun toString(): String = if (scopeId == 0u) "[$ip]:$port" else "[$ip%$scopeId]:$port"
}

/**
 * A Rust `PathBuf`: the bytes the OS holds a path as, any bytes on Unix and WTF-8 on Windows,
 * so it holds every path exactly, where a `String` holds only one that is UTF-8.
 */
class PathBuf private constructor(bytes: ByteArray, given: Boolean) {
	// Rust's own bytes are kept as they are; a caller's are copied, so it never changes.
	internal val bytes: ByteArray = if (given) bytes.copyOf() else bytes

	init {
		kotlin.require(!given || !_windows || _wtf8(bytes)) { "a path that is not WTF-8 has no Windows form" }
	}

	/** The path of the OS's [bytes], copied: on Windows they must be WTF-8, else it throws `IllegalArgumentException`. */
	constructor(bytes: ByteArray) : this(bytes, true)

	/** The path [path] spells, throwing `IllegalArgumentException` for a lone surrogate, which UTF-8 has no form of. */
	constructor(path: String) : this(_utf8(path), false)

	/** Its bytes, copied. */
	fun toByteArray(): ByteArray = bytes.copyOf()

	/** Its text, throwing `IllegalArgumentException` where its bytes are not UTF-8. */
	fun toText(): String = try {
		bytes.decodeToString(throwOnInvalidSequence = true)
	} catch (e: java.nio.charset.CharacterCodingException) {
		throw kotlin.IllegalArgumentException("$this is not UTF-8", e)
	}

	/** This as a `java.nio.file.Path`, throwing `IllegalArgumentException` where its bytes are not UTF-8. */
	fun toPath(): java.nio.file.Path = java.nio.file.Paths.get(toText())

	override fun equals(other: Any?): Boolean = other is PathBuf && bytes.contentEquals(other.bytes)

	override fun hashCode(): Int = bytes.contentHashCode()

	/** Its text, as Rust's `display` shows it: what is not UTF-8 as U+FFFD. */
	override fun toString(): String = bytes.decodeToString()

	internal companion object {
		/** The path of bytes Rust returned, kept without a copy. */
		fun of(bytes: ByteArray): PathBuf = PathBuf(bytes, false)
	}
}

/** [octets], throwing `IllegalArgumentException` unless there are [n]. */
internal fun _octets(octets: ByteArray, n: Int): ByteArray {
	kotlin.require(octets.size == n) { "${octets.size} octets, not $n" }
	return octets
}

/** The address of [address], throwing `IllegalArgumentException` for an unresolved one. */
internal fun _resolved(address: java.net.InetSocketAddress): java.net.InetAddress =
	address.address ?: throw kotlin.IllegalArgumentException("$address is unresolved, which Rust's SocketAddr has no form of")

/** [address], throwing `IllegalArgumentException` for one with a scope. */
internal fun _unscoped(address: java.net.Inet6Address): java.net.Inet6Address {
	kotlin.require(address.scopeId == 0 && address.scopedInterface == null) { "$address has a scope, which Rust's Ipv6Addr has no room for" }
	return address
}

/** [text]'s UTF-8, throwing `IllegalArgumentException` for a lone surrogate, which UTF-8 has no form of. */
internal fun _utf8(text: String): ByteArray = try {
	text.encodeToByteArray(throwOnInvalidSequence = true)
} catch (e: java.nio.charset.CharacterCodingException) {
	throw kotlin.IllegalArgumentException("a String with a lone surrogate has no UTF-8", e)
}

/** Whether the library runs on Windows, whose paths Rust holds as WTF-8. */
internal val _windows: Boolean = java.io.File.separatorChar == '\\'

/**
 * Whether [bytes] are WTF-8, as Rust encodes a Windows path: UTF-8, but for a surrogate
 * alone, encoded as a scalar would be, though never a lead then a trail.
 */
internal fun _wtf8(bytes: ByteArray): Boolean {
	var at = 0
	var lead = false
	while (at < bytes.size) {
		val b = bytes[at].toInt() and 0xff
		val (n, min) = when {
			b < 0x80 -> 1 to 0
			b in 0xc2..0xdf -> 2 to 0x80
			b in 0xe0..0xef -> 3 to 0x800
			b in 0xf0..0xf4 -> 4 to 0x10000
			else -> return false
		}
		if (at + n > bytes.size) return false
		var c = if (n == 1) b else b and (0x7f shr n)
		for (k in 1 until n) {
			val next = bytes[at + k].toInt() and 0xff
			if (next and 0xc0 != 0x80) return false
			c = (c shl 6) or (next and 0x3f)
		}
		if (c < min || c > 0x10ffff) return false
		val surrogate = c in 0xd800..0xdfff
		if (surrogate && c >= 0xdc00 && lead) return false
		lead = surrogate && c < 0xdc00
		at += n
	}
	return true
}
