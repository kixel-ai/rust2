// The api fixture's Kotlin test: the script fixtures/api/examples/transcript.rs runs
// natively, through the Kotlin tests/fixtures/transcript writes from it, which calls the
// functions here that transcript.rs defines. tests/fixtures.rs compares the transcripts.
//
// With an argument, it instead checks the futures' semantics, what the GC frees, what
// only Kotlin's types bound, or breaks a contract, which must fail.

@file:OptIn(ExperimentalUnsignedTypes::class)

import api.*
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.Executor
import kotlin.concurrent.thread
import kotlin.time.Duration.Companion.milliseconds
import kotlin.time.Duration.Companion.seconds
import kotlinx.coroutines.CompletableDeferred
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.asCoroutineDispatcher
import kotlinx.coroutines.awaitCancellation
import kotlinx.coroutines.cancel
import kotlinx.coroutines.currentCoroutineContext
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.takeWhile
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout

// What a function marked so holds lives in its own frame: the Android test keeps it from
// R8's inlining, since ART's interpreter keeps every reference a frame's registers last
// held reachable until the frame returns, which would keep it from the GC checks after.
annotation class OwnFrame

// The Log callback: keeps even levels.
class Print : Log, AutoCloseable {
	override fun log(level: UByte, line: String): Boolean {
		println("log $level $line")
		return level.toInt() % 2 == 0
	}

	// Rust closes it once it drops the implementation.
	override fun close() = println("freed")
}

// The Undo callback: each method returns its own digit.
class Edits : Undo {
	override fun delete(n: UInt): UInt = n - 1u

	override fun free(): UInt = 2u

	override fun type(): UByte = 3u
}

// Print each of `n`'s lists, its floats as their bits, as transcript.rs does.
fun numbers(what: String, n: Numbers) {
	println("$what u16 ${n.u16s.contentToString()}")
	println("$what u32 ${n.u32s.contentToString()}")
	println("$what u64 ${n.u64s.contentToString()}")
	println("$what i8 ${n.i8s.contentToString()}")
	println("$what i16 ${n.i16s.contentToString()}")
	println("$what i32 ${n.i32s.contentToString()}")
	println("$what i64 ${n.i64s.contentToString()}")
	println("$what f32 ${n.f32s.map { it.toRawBits().toUInt() }}")
	println("$what f64 ${n.f64s.map { it.toRawBits().toULong() }}")
	println("$what some ${n.some?.let { "Some(${it.contentToString()})" } ?: "None"}")
}

/** A path's bytes as Rust's `escape_ascii` prints them. */
fun escaped(path: PathBuf): String = path.toByteArray().joinToString("") {
	when (val b = it.toInt() and 0xff) {
		'\t'.code -> "\\t"
		'\r'.code -> "\\r"
		'\n'.code -> "\\n"
		'\\'.code, '\''.code, '"'.code -> "\\${b.toChar()}"
		in 0x20 until 0x7f -> b.toChar().toString()
		else -> "\\x%02x".format(b)
	}
}

/** Bytes in hex, as transcript.rs prints them. */
fun hex(bytes: ByteArray): String = bytes.joinToString("") { "%02x".format(it) }

fun hex(array: ArrayU8x4): String = hex(array.toByteArray())

fun pathHex(path: PathBuf): String = hex(path.toByteArray())

/** A duration as its seconds and nine digits of nanoseconds. */
fun seconds(d: Duration): String = "${d.secs}.${d.nanos.toString().padStart(9, '0')}"

/** A duration in whole milliseconds. */
fun millis(d: Duration): ULong = d.secs * 1000uL + d.nanos / 1_000_000u

/** A char's code point, which Kotlin holds it as. */
fun code(c: Int): UInt = c.toUInt()

fun reversed(list: List<TlsCodec>): List<TlsCodec> = list.reversed()

fun f64Bits(floats: DoubleArray): ULongArray = ULongArray(floats.size) { floats[it].toRawBits().toULong() }

/** An address's octets in hex, so their length shows its family. */
fun ipHex(ip: IpAddr): String = hex(
	when (ip) {
		is Ipv4Addr -> ip.octets()
		is Ipv6Addr -> ip.octets()
	},
)

/** A socket address as its IP's octets in hex, its port, flow label, and scope, each IPv6's alone. */
fun socketHex(addr: SocketAddr): String = when (addr) {
	is SocketAddrV4 -> "${hex(addr.ip.octets())}:${addr.port}"
	is SocketAddrV6 -> "${hex(addr.ip.octets())}:${addr.port}:${addr.flowinfo}:${addr.scopeId}"
}

/** A path no Rust string spells: bytes that are not UTF-8, or on Windows, WTF-8's lone surrogate. */
fun unspelled(): PathBuf = when (java.io.File.separatorChar) {
	'\\' -> PathBuf(byteArrayOf(0xed.toByte(), 0xa0.toByte(), 0x80.toByte()))
	else -> stray()
}

/** The bytes as text. */
fun Bytes.text(): String = toByteArray().decodeToString()

fun check(ok: Boolean, what: String) {
	if (!ok) {
		System.err.println("failed: $what")
		kotlin.system.exitProcess(1)
	}
}

/** Whether [f] throws `IllegalArgumentException`, as a value its Rust type cannot hold does. */
fun rejects(f: () -> Any?): Boolean = runCatching(f).exceptionOrNull() is IllegalArgumentException

/** The library's `usize::MAX`, which its target's pointer width sets: 32 bits on armv7 and x86 Android. */
fun usizeMax(): ULong = if (pointerBits(platform()) == 64u) ULong.MAX_VALUE else UInt.MAX_VALUE.toULong()

/** The library's `isize::MIN` and `isize::MAX`. */
fun isizeRange(): LongRange = if (pointerBits(platform()) == 64u) Long.MIN_VALUE..Long.MAX_VALUE else Int.MIN_VALUE.toLong()..Int.MAX_VALUE.toLong()

fun script() = transcript()

// A channel's futures, on a dispatcher the script drains, so it sees the receive pend and
// its wake queue the resume.
fun channel() {
	val channel = Channel.new()
	var wakes = 0
	val waker = Waker { wakes++ }
	val q = Queue()
	var got: Bytes? = null
	CoroutineScope(q.asCoroutineDispatcher()).launch { got = channel.recv() }
	q.drain()
	println("recv pending ${got == null}")
	channel.send(Bytes("hi".encodeToByteArray()))
	wakes += q.size
	println("woken $wakes")
	q.drain()
	println("recv ${got!!.text()}")
	runBlocking {
		channel.send(Bytes(byteArrayOf()))
		try {
			println("nonempty ${channel.recvNonempty().size}")
		} catch (e: ErrorException) {
			println("nonempty err ${e.message}")
		}
		Batch.new().use { b4 ->
			pair().use { b2 -> println("fill " + (fill4(b4, 9u) + fill2(b2, 1u)).joinToString(" ")) }
		}
		// The send happens at the call: the future is dropped unawaited.
		channel.sendLater(Bytes("eager".encodeToByteArray()))
		println("recv ${channel.recv().text()}")
		// The send happens at the call, and the future borrows the channel and the tag.
		println("tagged ${channel.tagged("tag")}")
		println("itemized ${channel.itemized(uintArrayOf(1u, 2u, 3u))}")
		println("byte_len ${byteLen(Bytes("abc".encodeToByteArray()))}")
		channel.send(Bytes("sized".encodeToByteArray()))
		val (payload, size) = channel.recvSized()
		println("sized ${payload.text()} $size")
	}
	// The waker moves into Rust, which keeps it until the send wakes it.
	channel.notify(waker)
	channel.send(Bytes("rung".encodeToByteArray()))
	println("woken $wakes")
	runBlocking {
		// A poll with no async twin is a future.
		println("take ${channel.pollTake().text()}")
		channel.send(Bytes("measured".encodeToByteArray()))
		println("measure ${channel.pollMeasure(2u)}")
	}
	// A context is made of the waker, which Rust keeps until the send wakes it.
	channel.listen(waker)
	channel.send(Bytes("heard".encodeToByteArray()))
	println("woken $wakes")
	// A returned waker wakes the waiter's task through Rust until the GC drops it.
	Waiter.new(waker).use { waiter ->
		waiter.waker().wake()
		waiter.context().wake()
	}
	println("woken $wakes")
}

// A kept closure: called again at the instant it asks for, then once its waiter wakes.
fun drive() {
	val start = Moment.new(0u, 1000u)!!
	var calls = 0
	val stopped = runBlocking {
		drive(start) { now, waiter ->
			calls += 1
			when (calls) {
				1 -> now.after(5u)
				2 -> {
					println("drive ${now.millis() - start.millis()}")
					waiter.waker().let { waker ->
						waker.wake()
						(waker as AutoCloseable).close()
					}
					null
				}
				else -> throw StringException("stopped after $calls")
			}
		}
	}
	println("drive $stopped")
}

// Readers over bytes, whose reads fail, and that seek past `Long.MAX_VALUE`.
fun readers() {
	for (file in listOf(Memory("hello world".encodeToByteArray()), Broken())) {
		try {
			val (tail, at) = tail(file, 5u)
			println("tail ${tail.text()} $at")
		} catch (e: StringException) {
			println("tail error ${e.error}")
		}
	}
	// A seek from the start reaches the reader whole, past `Long.MAX_VALUE`, as `Cursor` takes one.
	try {
		println("sought ${seekTo(Far(), ULong.MAX_VALUE)}")
	} catch (e: StringException) {
		println("sought error ${e.error}")
	}
}

// Streams, each collected as a flow: to its end, left early, and to an error, past which
// the stream goes on.
fun streams() = runBlocking {
	Countdown.new(3u).use { c -> println("countdown " + c.asFlow().toList().joinToString(" ")) }
	Countdown.new(5u).use { c ->
		c.asFlow().takeWhile { it != 4u }.collect {}
		println("countdown after break ${c.next()}")
	}
	val channel = Channel.new()
	channel.feed().use { feed ->
		for (first in listOf("a", "c")) {
			channel.send(Bytes(first.encodeToByteArray()))
			try {
				feed.asFlow().collect { payload ->
					println("feed ${payload.text()}")
					val reply = when (payload.text()) {
						"a" -> "b"
						"b" -> "!"
						else -> ""
					}
					channel.send(Bytes(reply.encodeToByteArray()))
				}
				println("feed end")
			} catch (e: ErrorException) {
				println("feed error ${e.message}")
			}
		}
	}
}

// Futures of a value that is not `Send`, run on this thread.
fun localLater(local: Local) {
	println("local later ${runBlocking { localOrNewLater(local) }.bump()}")
	println("local later fresh ${runBlocking { localOrNewLater(null) }.bump()}")
}

// A channel whose reads fail, which seeks anywhere from its start, as Rust's `Cursor` does.
class Far : Broken(), StartSeekable {
	override fun seekStart(offset: ULong): ULong = offset
}

// A channel over bytes in memory, as Rust's `Cursor` reads them.
class Memory(private val bytes: ByteArray) : java.nio.channels.SeekableByteChannel {
	private var at = 0L

	override fun read(dst: java.nio.ByteBuffer): Int {
		if (at >= bytes.size) return -1
		val n = minOf(dst.remaining().toLong(), bytes.size - at).toInt()
		dst.put(bytes, at.toInt(), n)
		at += n
		return n
	}

	override fun write(src: java.nio.ByteBuffer): Int = throw java.nio.channels.NonWritableChannelException()

	override fun position(): Long = at

	override fun position(newPosition: Long) = apply { at = newPosition }

	override fun size(): Long = bytes.size.toLong()

	override fun truncate(size: Long) = throw java.nio.channels.NonWritableChannelException()

	override fun isOpen() = true

	override fun close() {}
}

// A channel whose reads fail, as the native script's broken reader's do: each seek lands
// at 0.
open class Broken : java.nio.channels.SeekableByteChannel {
	override fun read(dst: java.nio.ByteBuffer): Int = throw java.io.IOException("broken")

	override fun write(src: java.nio.ByteBuffer): Int = throw java.nio.channels.NonWritableChannelException()

	override fun position(): Long = 0

	override fun position(newPosition: Long) = this

	override fun size(): Long = 0

	override fun truncate(size: Long) = throw java.nio.channels.NonWritableChannelException()

	override fun isOpen() = true

	override fun close() {}
}

/**
 * What the transcript's script says only in Rust: Kotlin's own errors, equality, and
 * conversions, each value the Kotlin type holds and Rust's cannot, which throws, and each
 * conversion to and from the idiomatic type, which throws where the other cannot hold a
 * value exactly.
 */
fun checks() {
	val two = java.math.BigInteger.valueOf(2)
	for (v in listOf(two.pow(128), java.math.BigInteger.ONE.negate())) {
		check(runCatching { scaled(v, 1u) }.exceptionOrNull() is IllegalArgumentException, "$v is no u128")
	}
	check(runCatching { ArrayU8x4(ByteArray(3)) }.exceptionOrNull() is IllegalArgumentException, "3 bytes are no [u8; 4]")
	// Methods named as `java.lang.Object`'s, apart from them on the JVM, or in Kotlin too where
	// an interface's would override them.
	ClashStore.new(1u).use { store ->
		store.wait()
		store.finalize()
		check(store.free() == 1u, "methods named as java.lang.Object's")
	}
	var waits = 0
	val source = object : ClashSource {
		override fun wait_() {
			waits++
		}

		override fun readByte(): UByte = 4u
	}
	check(clashWaited(source) == 4.toUByte() && waits == 1, "an interface's method named as java.lang.Object's")

	// A record holding a value past a word crosses as its words, in and out, and a call
	// with no slots left for one passes it encoded.
	val moment = Moment.new(1500u, 1000u)!!
	val shifted = moment.after(500u)
	val clocked = clockedAfter(Clocked(moment, 1u), 500u)
	check(clocked == Clocked(moment.after(500u), 2u) && clocked.at.millis() == 2000uL, "a record's moment crosses as its words")
	check(crammed(moment, shifted, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL) == 1500uL + 2000uL + 124uL, "a crammed call passes its last moment encoded")
	// A `Result` of one returns it directly, or throws.
	check(momentAt(1500u, 1000u) == moment && stampAt(5u) == Stamp.fromMillis(5u), "a result's value returns directly")
	check(runCatching { momentAt(1u, 0u) }.isFailure && runCatching { stampAt(0u) }.isFailure, "a result's error throws")
	// One of 128 words is made with no constructor of a parameter each, and passed encoded.
	val broad = Wide.new(ULong.MAX_VALUE)
	check(broad.sum() == 8001uL - 1uL && broad == Wide.new(ULong.MAX_VALUE) && broad != Wide.new(0u), "a wide value crosses")

	Counter.new("two", 2u).use { two ->
		check(two.offset(3u, 4u) == 36uL, "a method's parameters named handle and h are its own")
	}
	check(Counter.new("five", 5u).intoSum(1u) == 6uL, "a consuming method's parameter named close is its own")
	// A function taking a JNI env, which Kotlin alone exports, passing its thread's.
	val any = Any()
	check(sameObject(any, any) && !sameObject(any, Any()) && sameObject(null, null) && !sameObject(any, null), "a JObject crosses as the reference Kotlin passed")
	Counter.new("five", 5u).use { five ->
		check(five.plusIfString(2u, "s") == 7uL && five.plusIfString(2u, 1) == 5uL, "a method takes the env between parameters Kotlin passes")
		check((runCatching { five.plusIfString(2u, null) }.exceptionOrNull() as? StringException)?.error == "the object is null", "a JNI function's error throws")
	}
	for (fail in listOf(false, true)) {
		val thrown = runCatching { thrown(fail) }.exceptionOrNull()
		check(thrown is IllegalStateException && thrown.message == "thrown", "an exception the Rust left pending throws, failing or not")
	}
	var closed = false
	val sink = object : Log, AutoCloseable {
		override fun log(level: UByte, line: String): Boolean = true

		override fun close() {
			closed = true
		}
	}
	val dropped = runCatching { thrownLogger(sink) }.exceptionOrNull()
	check(dropped is IllegalStateException && dropped.message == "logger" && closed, "a result dropped for a pending exception closes its callback, then throws it")
	check(jniOne() == 1u, "a facade module named jni is the facade's")

	// A draft borrows its ledger exclusively until it closes: the ledger refuses any other
	// use meanwhile.
	Ledger.new().use { ledger ->
		ledger.edit().use { draft ->
			draft.write("first")
			val refused = runCatching { ledger.published() }.exceptionOrNull()
			check(
				refused is IllegalStateException && refused.message == "api_Ledger_published: a handle is borrowed exclusively",
				"a lent ledger refuses a read: $refused",
			)
		}
	}
	// Each counter moves in, so Kotlin closes it.
	val counters = counters(listOf("x", "y"))
	worth(listOf(counters[0]), listOf(counters[1]))
	for (counter in counters) {
		check(runCatching { counter.get() }.exceptionOrNull() is IllegalStateException, "a counter moved in is closed")
	}
	// A kept closure, called on another thread after the call that took it returned.
	check(Spawned.new { 7u }.join() == 7u, "a kept closure runs on another thread")

	check(splitLast(intArrayOf(1, 2, -3)) == splitLast(intArrayOf(1, 2, -3)), "a tuple holding an array compares by contents")
	check(splitLast(intArrayOf(1, 2, -3)).hashCode() == splitLast(intArrayOf(1, 2, -3)).hashCode(), "a tuple holding an array hashes by contents")
	check(Shadowed(intArrayOf(1), 0u, 0u) == Shadowed(intArrayOf(1), 0u, 0u) && Shadowed(intArrayOf(1), 0u, 0u) != Shadowed(intArrayOf(2), 0u, 0u), "a field named other compares as itself")
	check(Builtin.Any(intArrayOf(1)) == Builtin.Any(intArrayOf(1)) && Builtin.Any(intArrayOf(1)).hashCode() == Builtin.Any(intArrayOf(1)).hashCode(), "a variant named Any compares by contents")
	check(Builtin.Native(Double.NaN) != Builtin.Native(Double.NaN) && Builtin.Int(-0.0) == Builtin.Int(0.0) && Builtin.Boolean(1.0) != Builtin.Int(1.0), "variants named as Kotlin's builtins compare as Rust's")
	check(Qualifiers(-0.0, 1.0) == Qualifiers(0.0, 1.0) && Qualifiers(-0.0, 1.0).hashCode() == Qualifiers(0.0, 1.0).hashCode() && Qualifiers(Double.NaN, 1.0) != Qualifiers(Double.NaN, 1.0), "fields named as the package and kotlin compare as Rust's")
	val edges = Numbers.edges()
	check(Numbers.zero(true).hashCode() == Numbers.zero(false).hashCode(), "a negative zero hashes as zero does")
	check(Measure.new(-0.0, true).hashCode() == Measure.new(0.0, false).hashCode(), "a negative zero hashes as zero does in a nested record")
	val ints = edges.copy(f32s = FloatArray(0), f64s = DoubleArray(0))
	val alike = Numbers.edges().copy(f32s = FloatArray(0), f64s = DoubleArray(0))
	check(ints == alike && ints.hashCode() == alike.hashCode(), "records of arrays compare by contents")
	val back = edges.reversed().reversed()
	check(back.copy(f32s = FloatArray(0), f64s = DoubleArray(0)) == ints, "every integer edge value crosses both ways exactly")
	check(back.f32s.map { it.toRawBits() } == edges.f32s.map { it.toRawBits() } && back.f64s.map { it.toRawBits() } == edges.f64s.map { it.toRawBits() }, "every float edge value crosses both ways exactly")
	check(edges.copy(some = null).reversed().some == null, "a missing array stays missing")
	check(edges.copy(some = IntArray(0)).reversed().some?.size == 0, "an empty array is no missing one")
	check(edges.copy(some = intArrayOf(Int.MIN_VALUE, 7)).reversed().some.contentEquals(intArrayOf(7, Int.MIN_VALUE)), "an array in an option crosses in a record")

	// Kotlin's own hash of a key is Rust's `Hash`, as its `equals` is Rust's `==`.
	Key.new("a").use { a ->
		Key.new("a").use { same ->
			Key.new("b").use { b ->
				check(a == same && a != b, "keys compare as Rust's")
				check(a.hashCode() == same.hashCode() && a.hashCode() != b.hashCode(), "keys hash as Rust's")
				check(a.hashCode() == a.hashOf().toLong().hashCode(), "a key hashes as Rust's Hash")
				check(setOf(a, b).contains(same), "a key finds its equal in a set")
			}
		}
	}

	// Durations: exact both ways, and checked against kotlin.time's.
	check(rejects { Duration(0u, 1_000_000_000u) }, "a second of nanoseconds is no Duration's")
	check(rejects { Duration((-1).milliseconds) } && rejects { Duration(kotlin.time.Duration.INFINITE) }, "negative and infinite durations are no Rust Duration")
	check(Duration(1.5.seconds) == Duration(1u, 500_000_000u) && Duration(1.5.seconds).toKotlinDuration() == 1.5.seconds, "a kotlin.time Duration crosses exactly")
	check(Duration(0u, 1u) < Duration(1u) && Duration.MAX > Duration(ULong.MAX_VALUE), "durations order as Rust's")
	check(rejects { Duration.MAX.toKotlinDuration() }, "kotlin.time has no Duration::MAX")
	// Past about 146 years, kotlin.time holds milliseconds alone.
	check(rejects { Duration(5_000_000_000u, 1u).toKotlinDuration() }, "kotlin.time drops nanoseconds past 146 years")
	check(Duration(Duration(5_000_000_000u, 1_000_000u).toKotlinDuration()) == Duration(5_000_000_000u, 1_000_000u), "whole milliseconds past 146 years convert")
	check(sameDuration(Duration(ULong.MAX_VALUE, 1u)) == Duration(ULong.MAX_VALUE, 1u), "seconds past Long.MAX_VALUE cross")

	// Integers and chars Kotlin holds and Rust does not throw before they cross.
	for (c in listOf(0xd800, 0xdfff, 0x110000, -1)) check(rejects { sameChar(c) }, "$c is no char")
	check(rejects { nextHop(0UL) } && rejects { fartherHop(PairNonZeroU16(0u, 1u)) }, "a zero is no NonZero")

	// Text: a lone surrogate throws, passed alone or encoded, before anything moves.
	check(rejects { sameString("a\ud800") } && rejects { sameString("\udc00b") }, "a lone surrogate is no Rust text")
	check(sameString("😀") == "😀", "a surrogate pair is one char")
	check(sameString("x".repeat(1023) + "😀") == "x".repeat(1023) + "😀", "a pair split across chunks is one char")
	check(rejects { joinWords(listOf("a", "\ud800")) }, "a lone surrogate in a list throws")
	val kept = Counter.new("kept", 1u)
	check(rejects { Board.new(listOf(Pair("\ud800", kept))) }, "a lone surrogate beside a moved object throws")
	check(kept.get() == 1uL, "an object passed beside text that threw stays open")
	kept.close()

	// Paths: any bytes on Unix, and text only where it is UTF-8.
	val stray = stray()
	check(stray.toByteArray().contentEquals(byteArrayOf(-1)) && stray == PathBuf(byteArrayOf(-1)), "a path keeps bytes no String holds")
	check(rejects { stray.toText() } && rejects { stray.toPath() } && stray.toString() == "�", "a path that is not UTF-8 has no text")
	check(rejects { PathBuf("\ud800") }, "a lone surrogate has no path")
	check(PathBuf("a/é").toText() == "a/é", "a UTF-8 path converts")
	// `java.nio.file` is on Android from API 26.
	if (runCatching { Class.forName("java.nio.file.Paths") }.isSuccess) {
		check(PathBuf("a/é").toPath() == java.nio.file.Paths.get("a/é"), "a UTF-8 path converts to a Path")
	}
	check(_wtf8(byteArrayOf(0xed.toByte(), 0xa0.toByte(), 0x80.toByte())) && !_wtf8(byteArrayOf(0xed.toByte(), 0xa0.toByte(), 0x80.toByte(), 0xed.toByte(), 0xb0.toByte(), 0x80.toByte())), "a lone surrogate is WTF-8, a pair encoded apart is not")
	check(!_wtf8(byteArrayOf(-1)) && !_wtf8(byteArrayOf(0xc0.toByte(), 0x80.toByte())) && _wtf8("a/é😀".encodeToByteArray()), "WTF-8 is UTF-8 otherwise")

	// Addresses: exact by construction, and checked to and from java.net's.
	val loopback = ByteArray(16).also { it[15] = 1 }
	val scoped = java.net.Inet6Address.getByAddress(null, loopback, 3)
	check(rejects { Ipv6Addr(scoped) } && rejects { IpAddr.of(scoped) } && rejects { Ipv4Addr(ByteArray(3)) }, "a scope or a wrong length is no Rust IP")
	val unresolved = java.net.InetSocketAddress.createUnresolved("localhost", 1)
	check(rejects { SocketAddr.of(unresolved) } && rejects { SocketAddrV6(java.net.InetSocketAddress(java.net.InetAddress.getByAddress(byteArrayOf(1, 2, 3, 4)), 1)) }, "an unresolved or other family's address is refused")
	check(rejects { SocketAddrV6(Ipv6Addr(loopback), 1u, 1u).toInetSocketAddress() }, "Java keeps no flow label")
	check(rejects { SocketAddrV6(Ipv6Addr(loopback), 1u, 0u, UInt.MAX_VALUE).toInetSocketAddress() }, "Java keeps no scope past Int.MAX_VALUE")
	val at6 = SocketAddr.of(java.net.InetSocketAddress(scoped, 443))
	check(at6 == SocketAddrV6(Ipv6Addr(loopback), 443u, 0u, 3u) && at6.toString() == "[::1%3]:443", "an IPv6 address keeps its scope")
	check(at6.toInetSocketAddress() == java.net.InetSocketAddress(scoped, 443), "an IPv6 address converts back")
	val mapped = Ipv6Addr(ByteArray(16).also { it[10] = -1; it[11] = -1; it[12] = 127; it[15] = 1 })
	check(mapped.toString() == "::ffff:127.0.0.1" && (mapped as IpAddr).toInetAddress() is java.net.Inet6Address, "an IPv4-mapped address stays IPv6")
	check(IpAddr.of(java.net.InetAddress.getByAddress(byteArrayOf(127, 0, 0, 1))) == Ipv4Addr(127u, 0u, 0u, 1u), "an IPv4 address converts")
	check(Ipv6Addr(ByteArray(16).also { it[1] = 1; it[15] = 2 }).toString() == "1::2" && Ipv6Addr(ByteArray(16)).toString() == "::", "IPv6 prints as Rust's")
	check(SocketAddrV4(Ipv4Addr(127u, 0u, 0u, 1u), 80u).toString() == "127.0.0.1:80", "IPv4 prints as Rust's")
	println("checks ok")
}

// A queue of jobs that a test runs when it chooses: the dispatcher coroutines resume on.
class Queue : Executor {
	private val jobs = ConcurrentLinkedQueue<Runnable>()

	override fun execute(job: Runnable) {
		jobs.add(job)
	}

	val size: Int
		get() = jobs.size

	fun drain() {
		while (true) {
			(jobs.poll() ?: return).run()
		}
	}
}

// The futures' semantics beyond the transcript, each checked by an assertion.
fun futures() {
	val channel = Channel.new()

	// The send lands from another thread while the receive waits.
	runBlocking {
		val sender = thread { channel.send(Bytes("blocked".encodeToByteArray())) }
		val payload = channel.recv()
		sender.join()
		check(payload.text() == "blocked", "recv returns the payload another thread sent")
	}

	val q = Queue()
	val scope = CoroutineScope(q.asCoroutineDispatcher())

	// Resumed on its dispatcher, never inside the wake.
	run {
		var got: String? = null
		scope.launch { got = channel.recv().text() }
		q.drain()
		check(got == null, "recv waits for a send")
		channel.send(Bytes("then".encodeToByteArray()))
		check(got == null, "recv never polls inside the wake")
		q.drain()
		check(got == "then", "recv resumes with the payload")
	}

	// On a dispatcher that resumes in place, too: the send wakes while it holds the lock the
	// receive's poll takes.
	runBlocking {
		var got: String? = null
		val job = launch(Dispatchers.Unconfined) { got = channel.recv().text() }
		channel.send(Bytes("unconfined".encodeToByteArray()))
		job.join()
		check(got == "unconfined", "an unconfined recv resumes outside the wake")
	}

	// A wake that lands while the future polls, before it returns pending, is kept: the
	// future polls again rather than waiting for a wake that already happened.
	run {
		var got = 0u
		scope.launch { got = yielded(7u) }
		q.drain()
		check(got == 7u, "a wake during the poll polls again")
	}

	// A value that is not Send stays on the thread that made it: a use elsewhere throws, and
	// its future polls on its thread. Closed elsewhere, it drops there at its next call. One
	// such future made from none lives on a home, so it polls from any thread.
	run {
		val local = Local.new()
		val shared = local.share()
		local.bump()
		check(shared.bump() == 2u, "a shared local counts together")
		var got = 0u
		scope.launch { got = local.later() }
		q.drain()
		check(got == 3u, "a local's future runs on its thread")
		got = runBlocking(Dispatchers.Default) { localLater(5u) }
		check(got == 10u, "a future that is not Send made from none polls on a pool")
		var thrown: Throwable? = null
		thread { thrown = runCatching { local.bump() }.exceptionOrNull() }.join()
		check(
			thrown is IllegalStateException && thrown?.message == "api_Local_bump: a value that is not Send is used on another thread than the one that made it",
			"a local used elsewhere throws: $thrown",
		)
		val before = localsDropped()
		thread { local.close() }.join()
		check(localsDropped() == before, "a local closed elsewhere waits for its thread")
		check(shared.bump() == 4u && localsDropped() == before + 1u, "a local closed elsewhere drops on its thread")
		shared.close()
	}

	// Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	// for the next.
	run {
		var called = false
		val job = scope.launch {
			channel.recv()
			called = true
		}
		q.drain()
		channel.send(Bytes("kept".encodeToByteArray()))
		job.cancel()
		q.drain()
		check(!called && job.isCancelled, "a cancelled receive never resumes")
		val payload = runBlocking { withTimeout(5.seconds) { channel.recv() } }
		check(payload.text() == "kept", "a cancelled receive loses nothing")
	}

	// Closing a handle a call awaiting on it borrows throws, as Rust refuses a drop under a
	// borrow, and changes nothing: the call ends as it would, and then the close passes.
	run {
		val other = Channel.new()
		var got: String? = null
		scope.launch { got = other.recv().text() }
		q.drain()
		val sender = other.clone()
		val refused = runCatching { other.close() }.exceptionOrNull()
		check(refused is IllegalStateException && refused.message == "api_Channel_drop: a handle is already borrowed", "a close under an awaiting call throws: $refused")
		sender.send(Bytes("held".encodeToByteArray()))
		q.drain()
		check(got == "held", "the awaiting call ends as it would")
		other.close()
		val moved = runCatching { other.clear() }.exceptionOrNull()
		check(moved is IllegalStateException && moved.message == "Channel is closed", "a call after close throws: $moved")
		sender.close()
	}

	// Suspending lambdas Rust polls as futures, each launched in the caller's context.
	run {
		val ran = mutableListOf<String>()
		var got = 0u
		scope.launch { got = after({ ran += "ready" }, 4u) }
		q.drain()
		check(got == 8u && ran == listOf("ready"), "Rust awaits a suspending lambda")
	}
	run {
		val gate = CompletableDeferred<Unit>()
		var got = 0u
		scope.launch { got = after({ gate.await() }, 5u) }
		q.drain()
		check(got == 0u, "Rust waits on a lambda that suspends")
		gate.complete(Unit)
		q.drain()
		check(got == 10u, "a lambda's end wakes Rust")
	}
	run {
		var error: ErrorException? = null
		var ran = false
		scope.launch {
			try {
				within(0u) { ran = true }
			} catch (e: ErrorException) {
				error = e
			}
		}
		q.drain()
		check(error != null && !ran, "Rust gives up on a lambda before it starts, which never runs")
	}
	run {
		var cancelled = false
		val job = scope.launch {
			within(1000u) {
				try {
					awaitCancellation()
				} finally {
					cancelled = true
				}
			}
		}
		q.drain()
		check(!cancelled, "a lambda Rust polls runs until Rust drops it")
		job.cancel()
		q.drain()
		check(cancelled, "cancelling the call cancels the lambda Rust polls")
	}
	run {
		// A lambda that returns once the caller cancelled ends unfinished: the call
		// reports the cancel, never Rust's success.
		var result: String? = null
		scope.launch {
			val caller = coroutineContext[kotlinx.coroutines.Job]!!
			result = try {
				"returned ${after({ caller.cancel() }, 1u)}"
			} catch (e: kotlinx.coroutines.CancellationException) {
				"cancelled"
			}
		}
		q.drain()
		check(result == "cancelled", "a caller's cancel the lambda sees reports the cancel")
	}

	// A lambda a poll borrows, which the future it makes owns: polled until it ends, and
	// cancelled with the call.
	run {
		val gate = CompletableDeferred<Unit>()
		var done = false
		scope.launch {
			Waiter.pollFuture { gate.await() }
			done = true
		}
		q.drain()
		check(!done, "a poll of a lambda that suspends waits")
		gate.complete(Unit)
		q.drain()
		check(done, "a borrowed lambda's end finishes the poll")
	}
	run {
		var cancelled = false
		val job = scope.launch {
			Waiter.pollFuture {
				try {
					awaitCancellation()
				} finally {
					cancelled = true
				}
			}
		}
		q.drain()
		check(!cancelled, "a lambda a poll borrows runs until Rust drops it")
		job.cancel()
		q.drain()
		check(cancelled, "cancelling the poll cancels the lambda it borrows")
	}

	// Rust lets go of a closure it keeps once it drops it, done or not, so the GC collects it.
	released(spawned(9u), "a spawned closure is released once it ran")
	run {
		val (job, weak) = idle(scope)
		q.drain()
		check(held(weak), "a closure a pending future keeps stays held")
		job.cancel()
		q.drain()
		released(weak, "a closure a cancelled future kept is released")
	}
	// Cancelling a flow's collection cancels its pending `next`, which loses nothing: the
	// payload sent after is the next call's. A collector leaving early reads nothing ahead.
	val fed = Channel.new()
	fed.feed().use { feed ->
		var got: String? = null
		val job = scope.launch { feed.asFlow().collect { got = it.text() } }
		q.drain()
		job.cancel()
		q.drain()
		check(job.isCancelled && got == null, "a flow's collection cancels while next pends")
		fed.send(Bytes("kept".encodeToByteArray()))
		val next = runBlocking { feed.next() }
		check(next?.text() == "kept", "cancelling a flow's collection loses no payload")
		// A collector cancelling its job in its handler calls no `next` after, though the next
		// payload is ready at once.
		fed.send(Bytes("first".encodeToByteArray()))
		got = null
		val stopped = scope.launch {
			feed.asFlow().collect {
				got = it.text()
				fed.send(Bytes("second".encodeToByteArray()))
				currentCoroutineContext().cancel()
			}
		}
		q.drain()
		check(stopped.isCancelled && got == "first", "a flow's collection cancels in its handler")
		// Awaited on the queue, so a lost payload fails the check rather than waiting forever.
		var after: String? = null
		val direct = scope.launch { after = feed.next()?.text() }
		q.drain()
		direct.cancel()
		q.drain()
		check(after == "second", "a flow cancelled in its handler reads nothing ahead")
		Countdown.new(3u).use { c ->
			val first = runBlocking { c.asFlow().first() }
			check(first == 3u && runBlocking { c.next() } == 2u, "a flow left early reads nothing ahead")
		}
	}
	println("futures ok")
}

// A closure Spawned keeps, which returns [n], once Spawned ran it.
@OwnFrame
fun spawned(n: UInt): java.lang.ref.WeakReference<Any> {
	val task = { n }
	check(Spawned.new(task).join() == n, "a kept closure runs on Rust's thread")
	return java.lang.ref.WeakReference(task)
}

// A drive whose closure waits for a wake that never comes, launched on [scope].
@OwnFrame
fun idle(scope: CoroutineScope): Pair<kotlinx.coroutines.Job, java.lang.ref.WeakReference<Any>> {
	val start = Moment.new(0u, 1000u)!!
	val poll = { _: Moment, _: Waiter -> start.let { null } }
	val job = scope.launch { drive(start, poll) }
	return Pair(job, java.lang.ref.WeakReference(poll))
}

// Whether [weak] still reaches its value, which never reaches the caller's frame.
@OwnFrame
fun held(weak: java.lang.ref.WeakReference<*>): Boolean = weak.get() != null

// Collect until [weak] is gone, as once Rust lets go of the closure it held.
fun released(weak: java.lang.ref.WeakReference<*>, what: String) {
	val deadline = System.nanoTime() + 10.seconds.inWholeNanoseconds
	while (held(weak)) {
		check(System.nanoTime() < deadline, what)
		Runtime.getRuntime().gc()
		Thread.sleep(1)
	}
}

// Each breaks a contract, which must fail with a message naming it.
fun broken(which: String) {
	when (which) {
		"moved" -> {
			val c = Counter.new("c", 0u)
			c.intoLabel(null)
			c.get()
		}
		"boxed moved" -> {
			val tail = Node.new(1u, null)
			Node.new(2u, tail)
			tail.sum()
		}
		// A Rust panic cannot unwind into the JVM, so it aborts.
		"panicked" -> Clash.Unit.reading(0u)
		"outlived" -> Meters.new().let { meters ->
			val first = meters.first()
			meters.clear()
			first.count()
		}
		"lent moved" -> Registry.new(Counter.new("c", 0u)).edit { counter ->
			counter.intoLabel(null)
			false
		}
		"lent kept" -> {
			var kept: Counter? = null
			Registry.new(Counter.new("c", 0u)).edit { counter ->
				kept = counter
				false
			}
			kept!!.get()
		}
		// Any exception but a closure's own error, and a reader's IOException, aborts.
		"closure threw" -> made(1u) { throw IllegalStateException("a make failed") }
		"closure failed" -> Moment.new(0u, 1000u)!!.let { start ->
			runBlocking { drive(start) { _, _ -> throw IllegalStateException("a poll failed") } }
		}
		"reader threw" -> tail(
			object : Broken() {
				override fun read(dst: java.nio.ByteBuffer): Int = throw IllegalStateException("a read failed")
			},
			1u,
		)
		"callback threw" -> emit(
			object : Log {
				override fun log(level: UByte, line: String): Boolean = throw IllegalStateException("a log failed")
			},
			Bytes(byteArrayOf(1)),
		)
	}
}

// What frees what: an object's close, or the GC once it is collected if never closed, and
// Rust's bytes only the GC, once no Bytes reaching them is reachable. Rust's bytes up to
// 1 KiB cross copied, freed at the call, and longer ones lent.
// Kotlin's text and bytes, which Rust copies or encodes once from where Kotlin keeps them:
// each reads back as it was, at a call's top level and in a record, and text holding a lone
// surrogate, which UTF-8 has no form of, throws.
fun encoded() {
	val texts = listOf("", "a\u0000b", "é\u4e2d\uD83D\uDE00", "a".repeat(1023) + "\uD83D\uDE00", "\u4e2d\uD83D\uDE00".repeat(1500))
	for (text in texts) {
		check(greet(text, Shape.SQUARE) == "hello $text, a Square", "text crosses as its UTF-8")
		check(tickName(Label(text, null, null)) == text, "a record's text crosses as its UTF-8")
	}
	for (text in listOf("\uD800x", "x\uDC00", "end\uD83D", "\uDE00\uD83D", "a".repeat(1023) + "\uD83Dz")) {
		check(rejects { greet(text, Shape.SQUARE) }, "a lone surrogate throws")
		check(rejects { tickName(Label(text, null, null)) }, "a record's lone surrogate throws")
	}
	val body = ByteArray(300) { it.toByte() }
	val sliced = nextPacket(Packet(1u, Bytes(body).slice(7, 290))).payload
	check(sliced.toByteArray().contentEquals(body.copyOfRange(7, 290)), "a slice of Kotlin's bytes in a record crosses")
}

/** A record's scalars cross apart from its other fields, each exactly, both ways. */
fun readings() {
	check(reading(0u) == null && readingDebug(0u) == "", "no reading crosses as none")
	check(readingEcho(null) == null, "no reading crosses back")
	for (n in listOf(1u, 2u, 3u, 127u)) {
		val r = reading(n)!!
		check(r.debug() == readingDebug(n), "reading $n crosses as Rust holds it: ${r.debug()}")
		check(r.count == ULong.MAX_VALUE - n && r.len == usizeMax() - n && r.tag == (UShort.MAX_VALUE - n.toUShort()).toUShort(), "reading $n's unsigned scalars")
		check(r.at.x == Int.MIN_VALUE + n.toInt() && r.offset == (Byte.MIN_VALUE + n.toInt()).toByte(), "reading $n's signed scalars")
		check(r.level == -n.toFloat() / 2 && r.ratio == Double.MAX_VALUE / n.toDouble() && r.mark == 0x10FFFF - n.toInt(), "reading $n's floats and char")
		check(r.ok == (n % 2u == 1u) && r.shape == (if (n % 2u == 1u) Shape.CIRCLE else Shape.SQUARE) && r.note == "reading $n", "reading $n's bool, enum, and text")
		check(readingEcho(r) == r, "reading $n crosses back")
		check(r.next() == reading(n + 1u), "reading $n's receiver crosses")
		val held = Mut<Reading?>(r)
		readingBump(held)
		check(held.value == r.copy(count = r.count + 1u, note = r.note + "+"), "reading $n crosses through a Mut both ways")
	}
	val none = Mut<Reading?>(null)
	readingBump(none)
	check(none.value == null, "no reading crosses through a Mut")
}

/** A call near the JVM's 255 parameter slots, which a record's flattening must not pass. */
fun crowding() {
	val r = reading(1u)!!
	val sum = crowded(r, Stamp.fromMillis(2uL), Stamp.fromMillis(3uL), 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL, 1uL)
	check(sum == r.count + 2uL + 3uL + 119uL, "a crowded call crosses every parameter: $sum")
}

fun owned() {
	encoded()
	readings()
	crowding()
	budgeted()
	nudged()
	for (n in 1023..1025) {
		val payload = ByteArray(n) { it.toByte() }
		val packet = nextPacket(Packet(1u, Bytes(payload)))
		check(packet.seq == 2u && packet.payload.toByteArray().contentEquals(payload), "a record's $n bytes cross both ways")
		val sliced = nextPacket(Packet(1u, packet.payload.slice(1, n - 1))).payload
		check(sliced.toByteArray().contentEquals(payload.copyOfRange(1, n - 1)), "a slice of a record's $n bytes crosses as its range")
	}
	collected(lentAround() + 1u, "the GC frees lent bytes nothing reaches")
	reads(Bytes(ByteArray(1025) { 9 }), 9, "Kotlin's bytes")
	reads(Bytes(window(java.nio.ByteBuffer.allocateDirect(1031).also { b -> repeat(1031) { b.put(it, 9) } }, 3, 1025)), 9, "a direct buffer's bytes")
	buffers()
	check(Point.getX() == 0 && Point(3, 4).x == 3, "a companion's getX keeps its Kotlin name beside x's getter")
	check(Shape.Companion.valueOf("square") == Shape.SQUARE && Shape.valueOf("SQUARE") == Shape.SQUARE, "a companion's valueOf keeps its Kotlin name beside the enum's own")

	// A call after close throws, and close frees at once, once.
	val before = dropped()
	val token = Token.new(1u)
	token.close()
	check(dropped() == before + 1u, "closing frees at once")
	token.close()
	check(dropped() == before + 1u, "closing again frees nothing")
	check(runCatching { token.idGet }.exceptionOrNull()?.message == "Token is closed", "a call after close throws")

	// The file serves wasm32 too: what only wasm32 has throws on the host's library.
	check(platform() is Platform.Native && pointerBits(Platform.Native(64u)) == 64u, "the host is native")
	check(isFamily(Family.NATIVE), "and of the native family")
	for ((what, call) in listOf<Pair<String, () -> Unit>>(
		"api_wasm_only" to { wasmOnly() },
		"api_Platform.Wasm" to { pointerBits(Platform.Wasm(32u)) },
		"api_Family.Wasm" to { isFamily(Family.WASM) },
		"api_Page_default" to { Page.default() },
		"api_Page_debug" to { Page(1u).debug() },
		"api_Booth_debug" to { Booth(Mark.of(1uL)).debug() },
	)) {
		val thrown = runCatching(call).exceptionOrNull()
		check(thrown is UnsupportedOperationException && thrown.message == "$what: it is not on this library's target", "$what throws: $thrown")
	}
	Counter.new("k", 7u).use { kept ->
		check(runCatching { wasmTake(kept) }.exceptionOrNull() is UnsupportedOperationException, "api_wasm_take throws")
		check(kept.get() == 7uL, "a refused call leaves its moved argument open")
	}
	// A failed check refuses the call before it moves anything, so the lent counter stays open.
	Registry.new(Counter.new("r", 3u)).edit { lent ->
		val refused = runCatching { returnMoved(lent, false) }.exceptionOrNull()
		check(refused is IllegalStateException && refused.message == "api_return_moved: a handle a call lent is borrowed exclusively or moved out", "a lent counter refuses the move: $refused")
		check(lent.get() == 3uL, "a call a check refused leaves its moved argument open")
		false
	}

	// Bytes outlive the object that returned them and the record that held them.
	val body = ByteArray(2048) { 'b'.code.toByte() }
	val message = Message.new(Stamp.fromMillis(1u), Bytes(body))
	val payload = message.payloadGet
	message.close()
	val carried = nextPacket(Packet(1u, Bytes(body))).payload
	uncollected(before + 1u, "the GC frees nothing still reachable")
	check(payload.toByteArray().contentEquals(body), "bytes outlive the object that returned them")
	check(carried.toByteArray().contentEquals(body), "bytes outlive the record that held them")

	// Never closed, each is freed once collected, but a slice keeps its bytes.
	unclosed()
	collected(before + 3u, "the GC frees what nobody closed")
	keptBySlice(before + 3u)
	collected(before + 4u, "the GC frees bytes once no slice reaches them")

	raced()
	compared()
	forwarded()
	nursed()
}

// The nursery of the latest objects a thread was returned, which a close leaves at once
// and the GC frees only once later returns push one out unclosed, or the GC drops the
// nursery of a thread returning nothing more: each object is freed once, however its close
// and the nursery race.
fun nursed() {
	// Left in a nursery no later return pushes it out of, which the GC drops.
	var before = dropped()
	unclosedToken()
	collected(before + 1u, "the GC frees what an idle nursery held unclosed")

	// Closed after later returns pushed it out unclosed, which registered it.
	before = dropped()
	val kept = Token.new(4u)
	fill()
	kept.close()
	check(dropped() == before + 1u, "closing an object pushed out of its nursery frees it at once")
	uncollected(before + 1u, "the GC frees nothing a close freed after a push out")

	// Returned on many threads, each closing some of its own and of another's, as others'
	// returns push them out and the GC drops nurseries, and dropping the rest.
	before = dropped()
	val workers = 8
	val each = 1000
	val handoff = java.util.concurrent.LinkedBlockingQueue<Token>()
	val collecting = java.util.concurrent.atomic.AtomicBoolean(true)
	val collector = thread {
		while (collecting.get()) {
			Runtime.getRuntime().gc()
			Thread.sleep(1)
		}
	}
	List(workers) {
		thread {
			for (i in 0 until each) {
				val t = Token.new(i.toUInt())
				when (i % 3) {
					0 -> t.close()
					1 -> handoff.add(t)
				}
				if (i % 3 != 0) handoff.poll()?.close()
			}
		}
	}.forEach { it.join() }
	collecting.set(false)
	collector.join()
	while (true) handoff.poll()?.close() ?: break
	collected(before + (workers * each).toUInt(), "the GC frees every object a nursery held unclosed")
}

// A token nothing reaches once this returns, and no return after it.
@OwnFrame
fun unclosedToken() {
	Token.new(5u)
}

// Return a nursery's worth of objects, each closed, pushing out every earlier one.
fun fill() {
	repeat(64) { Tally.new().close() }
}

// A payload Rust lent passed back to Rust: it crosses as Rust's own, never copied, so the
// Bytes it went through keep nothing of their own, and its one buffer drops once, after
// the last of them.
fun forwarded() {
	val before = dropped()
	forwardedRead(before)
	collected(before + 1u, "a forwarded payload's buffer drops with the last Bytes reaching it")
}

// In its own frame, so nothing on the caller's keeps the last Bytes reachable.
@OwnFrame
fun forwardedRead(before: UInt) {
	val last = forward()
	uncollected(before, "a payload forwarded through Rust stays the buffer Rust made")
	check(last.size == 2046 && last.toByteArray().all { it == 6.toByte() }, "a forwarded payload reads as Rust made it")
}

// Rust's bytes sent, received, sliced, carried in a record, and sent again.
fun forward(): Bytes = runBlocking {
	Channel.new().use { channel ->
		channel.send(tokenBytes(6u, 2048u))
		val carried = nextPacket(Packet(1u, channel.recv().slice(1, 2047))).payload
		channel.send(carried)
		channel.recv()
	}
}

// Every read of [bytes], each of which is [value].
fun reads(bytes: Bytes, value: Byte, what: String) {
	val n = bytes.size
	val all = bytes.toByteArray()
	check(all.size == n && all.all { it == value }, "$what read back")
	check(bytes[0] == value && bytes[n - 1] == value, "$what read by index")
	val into = bytes.copyInto(ByteArray(n + 1) { 7 }, 1, 1, n)
	check(into[0] == 7.toByte() && (1 until n).all { into[it] == value } && into[n] == 7.toByte(), "$what copied into a range")
	val slice = bytes.slice(2, n - 1).slice(1)
	check(slice.size == n - 4 && slice.toByteArray().all { it == value } && slice[n - 5] == value, "$what sliced twice")
	check(bytes.slice(n).size == 0 && bytes.slice(0, 0).toByteArray().isEmpty(), "$what sliced empty")
	val outOfRange = listOf<() -> Any>(
		{ bytes[n] },
		{ bytes[-1] },
		{ slice[slice.size] },
		{ bytes.slice(2, 1) },
		{ bytes.slice(0, n + 1) },
		{ slice.slice(0, slice.size + 1) },
		{ bytes.copyInto(ByteArray(n - 1)) },
		{ bytes.copyInto(ByteArray(n), 1) },
		{ bytes.copyInto(ByteArray(n), -1, 0, 1) },
	)
	for (read in outOfRange) {
		check(runCatching(read).exceptionOrNull() is IndexOutOfBoundsException, "$what refuse a read out of range")
	}
	for (direct in listOf(true, false)) {
		val kind = if (direct) "a direct buffer" else "a heap buffer"
		val into = window(if (direct) java.nio.ByteBuffer.allocateDirect(n + 3) else java.nio.ByteBuffer.allocate(n + 3), 2, n)
		check(bytes.copyInto(into, 1) === into && into.position() == n + 1, "$what copied into $kind move its position")
		check(into.get(1) == 0.toByte() && (2 until n + 1).all { into.get(it) == value } && into.get(n + 1) == 0.toByte(), "$what copied into $kind at its position")
		val full = window(if (direct) java.nio.ByteBuffer.allocateDirect(n) else java.nio.ByteBuffer.allocate(n), 1, n - 1)
		check(runCatching { bytes.copyInto(full) }.exceptionOrNull() is java.nio.BufferOverflowException && full.position() == 1 && full.get(1) == 0.toByte(), "$what refuse $kind too short, writing nothing")
		val readOnly = window(if (direct) java.nio.ByteBuffer.allocateDirect(n) else java.nio.ByteBuffer.allocate(n), 0, n).asReadOnlyBuffer()
		check(runCatching { bytes.copyInto(readOnly) }.exceptionOrNull() is java.nio.ReadOnlyBufferException, "$what refuse $kind read-only")
		check(runCatching { bytes.copyInto(into, 2, 1) }.exceptionOrNull() is IndexOutOfBoundsException, "$what refuse a range out of order for $kind")
	}
}

// [buffer] from [position] for [n] bytes, set through `Buffer`, whose `position` and
// `limit` every JVM and Android level has.
fun window(buffer: java.nio.ByteBuffer, position: Int, n: Int): java.nio.ByteBuffer {
	val moved: java.nio.Buffer = buffer
	moved.limit(position + n).position(position)
	return buffer
}

// A ByteBuffer's bytes, from its position until its limit, cross direct or heap, read-only
// or not, at a call's top level and in a record, short or long; they hold the buffer
// without a copy, so a later write shows, apart from its position and limit.
fun buffers() {
	for (n in listOf(0, 5, 1025, 4000)) {
		val payload = ByteArray(n) { (it * 7 + 1).toByte() }
		val direct = window(java.nio.ByteBuffer.allocateDirect(n + 5), 3, n)
		for (i in 0 until n) direct.put(3 + i, payload[i])
		val heap = java.nio.ByteBuffer.wrap(ByteArray(n + 4).also { payload.copyInto(it, 2) }, 2, n).slice()
		for ((kind, buffer) in listOf("direct", direct, "heap", heap, "read-only direct", direct.asReadOnlyBuffer(), "read-only heap", heap.asReadOnlyBuffer()).chunked(2)) {
			val bytes = Bytes(buffer as java.nio.ByteBuffer)
			val what = "$n bytes of a $kind buffer"
			check(bytes.size == n && bytes == Bytes(payload) && bytes.hashCode() == Bytes(payload).hashCode(), "$what equal Kotlin's")
			check(prefixed(bytes, null).toByteArray().contentEquals(payload), "$what cross a call")
			check(prefixed(Bytes(payload), bytes).toByteArray().contentEquals(payload + payload), "$what cross a call as an Option")
			check(nextPacket(Packet(1u, bytes)).payload.toByteArray().contentEquals(payload), "$what cross in a record")
			if (n > 2) {
				check(prefixed(bytes.slice(1, n - 1), null).toByteArray().contentEquals(payload.copyOfRange(1, n - 1)), "a slice of $what crosses as its range")
				check(nextPacket(Packet(1u, bytes.slice(1, n - 1))).payload.toByteArray().contentEquals(payload.copyOfRange(1, n - 1)), "a slice of $what crosses in a record as its range")
			}
		}
		if (n > 0) {
			val bytes = Bytes(direct)
			window(direct, 0, n + 5)
			direct.put(3, 42)
			window(direct, 0, 0)
			check(bytes[0] == 42.toByte() && bytes[n - 1] == payload[n - 1] && bytes.size == n && prefixed(bytes, null)[0] == 42.toByte(), "a direct buffer is held without a copy, apart from its position and limit")
			check(bytes.copyInto(ByteArray(n))[0] == 42.toByte() && bytes == Bytes(byteArrayOf(42) + payload.copyOfRange(1, n)), "a direct buffer reads apart from its limit")
		}
	}
	// Rust's bytes, short ones copied and long ones lent, copy into a buffer once.
	for (n in listOf(5, 1025, 4000)) reads(prefixed(Bytes(ByteArray(n) { 8 }), null), 8, "$n bytes Rust returned")
}

// Never closed, and long enough to be lent.
@OwnFrame
fun unclosed() {
	Token.new(3u)
	tokenBytes(4u, 1025u)
}

// A slice of Rust's bytes, made in a frame of its own so nothing keeps the Bytes it came
// from, which keeps Rust's buffer while it reads.
@OwnFrame
fun keptBySlice(freed: UInt) {
	val view = sliced(5u)
	uncollected(freed, "a slice keeps Rust's bytes")
	check(view.size == 10 && view.toByteArray().all { it == 5.toByte() }, "a slice reads Rust's bytes once the Bytes it came from are gone")
}

@OwnFrame
fun sliced(id: UInt): Bytes = tokenBytes(id, 1025u).slice(1015)

// Frees racing calls: a close racing calls on other threads, and the GC freeing what a
// call reads while it runs.
fun raced() {
	// A latch's calls only borrow it, so Kotlin counts them; a token's Rust checks.
	var before = dropped()
	val latch = Latch.default()
	latch.set(5u)
	race(latch, { it.get() == 5u }, setOf("Latch is closed"), "Latch: a handle is already borrowed")
	race(
		Token.new(5u),
		{ it.idGet == 5u },
		setOf("Token is closed", "api_Token_id_get: a handle's value was moved out"),
		"api_Token_drop: a handle is already borrowed",
	)
	check(dropped() == before + 1u, "close racing calls drops Rust's value once")

	// Calls on values nothing else reaches, while another thread collects: each reads what
	// it was given, never a buffer freed under it and reused by the next, which holds
	// another id.
	before = dropped()
	val collecting = java.util.concurrent.atomic.AtomicBoolean(true)
	val cycles = java.util.concurrent.atomic.AtomicInteger()
	val collector = thread {
		while (collecting.get()) {
			Runtime.getRuntime().gc()
			cycles.incrementAndGet()
		}
	}
	var rounds = 0u
	while (cycles.get() < 50) {
		racedRound(rounds % 100u + 1u)
		rounds++
	}
	collecting.set(false)
	collector.join()
	collected(before + 3u * rounds, "the GC frees what each call raced")
}

// Close [made] while 8 threads call [call] on it: each call runs on the live value or, once
// it is closed, throws one of [closed], which for a value Rust checks includes its moved
// check, met by a call that held the handle before close; a close a running call beats
// throws [borrowed], as Rust refuses a drop under a borrow, and changes nothing, so the next
// tries again.
fun <T : AutoCloseable> race(made: T, call: (T) -> Boolean, closed: Set<String>, borrowed: String) {
	val ran = java.util.concurrent.atomic.AtomicInteger()
	val failed = java.util.concurrent.atomic.AtomicInteger()
	val callers = List(8) {
		thread {
			while (true) {
				val live = try {
					call(made)
				} catch (e: IllegalStateException) {
					check(e.message in closed, "a call racing close throws: ${e.message}")
					failed.incrementAndGet()
					return@thread
				}
				check(live, "a call racing close reads the live value")
				ran.incrementAndGet()
			}
		}
	}
	while (ran.get() < 100) Thread.yield()
	while (true) {
		try {
			made.close()
			break
		} catch (e: IllegalStateException) {
			check(e.message == borrowed, "a close racing a call throws: ${e.message}")
		}
	}
	callers.forEach { it.join() }
	check(failed.get() == 8, "every call after close throws")
	made.close()
}

// Calls on values nothing else reaches once it returns, holding [id], racing the GC.
@OwnFrame
fun racedRound(id: UInt) {
	check(tokenBytes(id, 2048u)[2047] == id.toByte(), "a read by index races the GC")
	check(tokenBytes(id, 2048u).slice(1024).toByteArray().all { it == id.toByte() }, "a slice's copy races the GC")
	check(Token.new(id).idGet == id, "a call on an object races the GC")
}

// At most 64 MiB of Rust's bytes are lent at once, freed or not, since the GC sees none of
// them: past that, bytes cross copied, freed at the call, until the GC frees what was lent.
// First, so no bytes an earlier step lent count against it.
fun budgeted() {
	val before = dropped()
	lentToBudget(before)
	collected(before + 65u, "the GC frees bytes lent up to the budget")
	lentAgain(before + 65u)
	collected(before + 66u, "the GC frees bytes lent once the budget is free")
}

// On ART, Rust asks the GC to collect once half the budget is lent, so it frees the bytes
// nothing reaches before the budget runs out, though Kotlin makes no garbage meanwhile.
// HotSpot, which copies past the budget cheaply, is never asked.
fun nudged() {
	if (System.getProperty("java.vm.name") != "Dalvik") return
	val before = dropped()
	// A MiB at a time, none left reachable, until 32 are outstanding: a collection of
	// ART's own meanwhile only frees some sooner, and more are lent to reach half again.
	var lent = 0u
	val deadline = System.nanoTime() + 10.seconds.inWholeNanoseconds
	while (dropped() < before + 32u) {
		check(System.nanoTime() < deadline, "ART collects the bytes lent once half the budget is")
		if (lent - (dropped() - before) < 32u) {
			lentUnreached()
			lent++
		} else {
			Thread.sleep(1)
		}
	}
	collected(before + lent, "the GC frees the bytes lent to half the budget, each once")
}

// A MiB of Rust's bytes lent and read, none left reachable.
@OwnFrame
fun lentUnreached() {
	check(tokenBytes(5u, (1 shl 20).toUInt())[0] == 5.toByte(), "bytes lent toward half the budget read")
}

// Bytes lent once the GC freed those that filled the budget, with Rust's drops still [freed].
@OwnFrame
fun lentAgain(freed: UInt) {
	check(tokenBytes(3u, 2048u)[2047] == 3.toByte() && dropped() == freed, "bytes cross lent again once the GC frees them")
}

// Rust's bytes either side of the copy limit, returning the drops once the call has freed
// those it copied.
@OwnFrame
fun lentAround(): UInt {
	var freed = dropped()
	for (n in 1023..1025) {
		val bytes = tokenBytes(n.toUInt(), n.toUInt())
		if (n <= 1024) freed++
		check(dropped() == freed, "$n bytes are freed at the call only if copied")
		reads(bytes, n.toByte(), "$n bytes")
	}
	return freed
}

@OwnFrame
fun lentToBudget(before: UInt) {
	val mib = 1 shl 20
	val held = List(64) { tokenBytes(1u, mib.toUInt()) }
	check(dropped() == before, "64 MiB of Rust's bytes cross lent")
	val copied = tokenBytes(2u, mib.toUInt())
	check(dropped() == before + 1u, "bytes past 64 MiB lent cross copied, freed at the call")
	check(copied.size == mib && copied[0] == 2.toByte() && copied[mib - 1] == 2.toByte() && held[63][mib - 1] == 1.toByte(), "bytes past the budget read as lent ones do")
	val carried = nextPacket(Packet(1u, Bytes(ByteArray(2048) { 4 }))).payload
	check(carried.size == 2048 && carried.toByteArray().all { it == 4.toByte() }, "a record's bytes past the budget cross copied")
}

// Bytes compare and hash by their contents wherever each keeps them, Kotlin's or Rust's,
// copied or lent, whole or sliced, and so do records holding them, as map keys too.
fun compared() {
	val body = ByteArray(2050) { (it * 7).toByte() }
	// Rust's whole are lent, its 1000 copied; the middle two differ by a shift.
	val ranges = listOf(0 to 2050, 1 to 1001, 2 to 1002, 5 to 5)
	val forms = ranges.map { (from, until) ->
		listOf(
			Bytes(body.copyOfRange(from, until)),
			Bytes(body).slice(from, until),
			nextPacket(Packet(1u, Bytes(body.copyOfRange(from, until)))).payload,
			nextPacket(Packet(1u, Bytes(body))).payload.slice(from, until),
		)
	}
	val keys = HashMap<Packet, Int>()
	forms.forEachIndexed { i, same -> keys[Packet(1u, same[0])] = i }
	val all = forms.flatMapIndexed { i, same -> same.map { i to it } }
	for ((i, a) in all) {
		for ((j, b) in all) {
			check((a == b) == (i == j) && (Packet(1u, a) == Packet(1u, b)) == (i == j), "bytes ${ranges[i]} and ${ranges[j]} compare by content")
			if (i == j) check(a.hashCode() == b.hashCode() && Packet(1u, a).hashCode() == Packet(1u, b).hashCode(), "bytes ${ranges[i]} hash by content")
		}
		check(keys[Packet(1u, a)] == i, "a record holding bytes ${ranges[i]} finds its key")
	}
	check(all.map { it.second }.toSet().size == ranges.size, "a set holds bytes once per content")
	val last = Bytes(body.copyOf().also { it[body.size - 1]++ })
	check(last != forms[0][3] && forms[0][3] != last, "bytes differing in their last byte differ")
	check(!forms[0][3].equals(body), "bytes differ from a ByteArray")
}

// Collect until Rust's values dropped count [n], and none past it.
fun collected(n: UInt, what: String) {
	val deadline = System.nanoTime() + 10.seconds.inWholeNanoseconds
	while (dropped() < n) {
		check(System.nanoTime() < deadline, what)
		// Not System.gc(), which ART skips unless finalization ran since its last call.
		Runtime.getRuntime().gc()
		Thread.sleep(1)
	}
	uncollected(n, "$what, each once")
}

// Collect a few times, checking that Rust's values dropped still count [n].
fun uncollected(n: UInt, what: String) {
	repeat(5) {
		Runtime.getRuntime().gc()
		Thread.sleep(1)
	}
	check(dropped() == n, what)
}

/** An enum whose variants are named as types its sealed class names, each round-tripped. */
fun clash() {
	val p = Point(1, 2)
	val all = listOf(
		Clash.Point(p),
		Clash.Shape(Shape.CIRCLE),
		Clash.Event(Event.Named("n")),
		Clash.Double(1.5),
		Clash.Float(0.5f),
		Clash.String("s"),
		Clash.Int(-3),
		Clash.Boolean(true),
		Clash.Native(7u),
		Clash.Bytes(Bytes("ab".encodeToByteArray())),
		Clash.Any,
		Clash.Unit,
		Clash.Waker,
		Clash.Mut,
		Clash.Pair(1u, 2u),
		Clash.Stamp(5L),
		Clash.Moment(Moment.new(3u, 1000u)!!),
	)
	for (c in all) {
		check(c.debug().isNotEmpty(), "clash $c debug")
	}
	check(Clash.Double(1.5).debug() == "Double(1.5)", "clash Double debug")
	check(Clash.String("s").debug() == "String(\"s\")", "clash String debug")
	check(Clash.Native(7u).debug() == "Native(7)", "clash Native debug")
	check(all[0].point() == p && all[3].point() == null, "clash point")
	check(Clash.of(2.5) == Clash.Double(2.5), "clash of")
	check(Clash.at(p) == Clash.Point(p), "clash at")
	check(Clash.Unit.reading(3u) == readingEcho(reading(3u)!!), "clash reading")
	val len = Mut(9u)
	check(Clash.listed(uintArrayOf(1u, 2u), len).map { it.toList() } == listOf(listOf(1u, 2u, 9u)) && len.value == 2u, "clash listed")
	check(runBlocking { Clash.after({}, 0.25) } == Clash.Double(0.25), "clash after")
	check(Packaged(3u).point() == Point(3, 3) && Packaged(3u).shape() == Shape.CIRCLE, "a field named as the package")
	val b = { t: String -> Bytes(t.encodeToByteArray()) }
	check(Clash.Unit.joined(b("a"), b("b"), b("c")).toByteArray().decodeToString() == "abc", "clash joined")
	check(Clash.Unit.joined(b("a"), b("b"), null).toByteArray().decodeToString() == "ab", "clash joined none")
	check(runBlocking { Clash.Int(1).tagged("t") } == "t Int(1)", "clash tagged")
	check(Clash.Unit.stamp(2500u).secs() == 2uL && Clash.Unit.stamp(5u) == Stamp.fromMillis(5u), "clash stamp")
	check(all[16].moment(9u) == Moment.new(3u, 1000u) && Clash.Unit.moment(4000u).millis() == 4000uL, "clash moment")
	check(Clash.String("hi").bytes().toByteArray().decodeToString() == "hi", "clash bytes")
	val out = Mut(0u)
	check(Clash.Int(4).describe("t", 3u, Shape.SQUARE, out) == "t Int(4) 3 Square" && out.value == 6u, "clash describe")
	check(Handle.new(5u).use { it.count() } == 5u && runBlocking { Handle.later(6u) }.use { it.count() } == 6u, "an object named as a former runtime class")
	// Names of C library macros, which C and C++ keep apart.
	check(errno() == 7 && assert(true) && offsetof(3u) == 3.toUByte() && MacrosStreams(1u, 2u, 3u, 4u).errno == 4.toUByte() && macrosStreamed(MacrosStreams(1u, 2u, 3u, 4u), 5u) == 15u, "names of C library macros")
}

fun returns() {
	val saved = returnValues(Mut(Point(0, 0)), false)
	val completed = java.util.concurrent.atomic.AtomicInteger()
	List(4) { thread {
		repeat(64) { returnValues(Mut(Point(it, -it)), false); Counter.new("thread", it.toULong()).close(); completed.incrementAndGet() }
	} }.forEach { it.join() }
	check(completed.get() == 256, "all concurrent return calls completed")
	check(saved.first == "hé🙂\u0000" && saved.third[1][9999].toUByte() == 15u.toUByte(), "a return owns its data after concurrent scratch reuse")
	val point = Mut(Point(3, -5))
	for (fail in listOf(false, true, false)) {
		try {
			val (text, widths, rows) = returnValues(point, fail)
			val (a, b, c, d) = widths!!
			println("values $text $a $b $c $d")
			println("rows ${rows[0].size} ${rows[0][0].toUByte()} ${rows[0][1].toUByte()} ${rows[0][2].toUByte()} ${rows[1].size} ${rows[1][9999].toUByte()}")
		} catch (e: ErrorException) { println("values error") }
		println("point ${point.value.x} ${point.value.y}")
	}
	for (fail in listOf(false, true)) {
		val counter = Counter.new("moved", ULong.MAX_VALUE)
		try {
			returnMoved(counter, fail).use { println("moved ${it.name()} ${it.get()}") }
		} catch (e: ErrorException) { println("moved error") }
		val usable = try { counter.get(); true } catch (_: IllegalStateException) { false }
		check(!usable) { "a moved counter was usable" }
	}
	for (hex in listOf("ff0080fe", "bad")) {
		try {
			val value = fingerprint(hex)
			println("array ${value[0].toUByte()} ${value[1].toUByte()} ${value[2].toUByte()} ${value[3].toUByte()}")
		} catch (e: StringException) { println("array error") }
	}
	for (unit in listOf(true, false)) {
		try { val (a, b) = corner(unit); println("corner $a $b") }
		catch (e: ErrorException) { println("corner error") }
	}
}

fun main(args: Array<String>) {
	when {
		args.isEmpty() -> script()
		args[0] == "returns" -> returns()
		args[0] == "futures" -> futures()
		args[0] == "owned" -> owned()
		args[0] == "clash" -> clash()
		args[0] == "checks" -> checks()
		else -> broken(args[0])
	}
}
