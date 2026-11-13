// The runtime every generated Kotlin file includes in its `Native` object, so no name it
// takes can clash with an exported type's: the codec values cross JNI in, the handles
// objects own, what frees them once collected, and the waker futures park on.

/** Encodes values as the library's JNI glue decodes them: little-endian, field by field. */
class Writer {
	private var b: java.nio.ByteBuffer = java.nio.ByteBuffer.allocate(64).order(java.nio.ByteOrder.LITTLE_ENDIAN)

	private fun room(n: Int) {
		if (b.remaining() < n) {
			val next = java.nio.ByteBuffer.allocate(kotlin.comparisons.maxOf(b.capacity() * 2, b.position() + n)).order(java.nio.ByteOrder.LITTLE_ENDIAN)
			next.put(b.array(), 0, b.position())
			b = next
		}
	}

	fun bool(v: Boolean) = i8(if (v) 1 else 0)

	fun i8(v: Byte) {
		room(1)
		b.put(v)
	}

	fun i16(v: Short) {
		room(2)
		b.putShort(v)
	}

	fun i32(v: Int) {
		room(4)
		b.putInt(v)
	}

	fun i64(v: Long) {
		room(8)
		b.putLong(v)
	}

	fun f32(v: Float) {
		room(4)
		b.putFloat(v)
	}

	fun f64(v: Double) {
		room(8)
		b.putDouble(v)
	}

	/** Bytes of a length both sides know, such as an array's. */
	fun raw(v: ByteArray) {
		room(v.size)
		b.put(v)
	}

	/**
	 * Bytes a record holds, or -1 for none: Kotlin's as [KOTLIN], their array's place in
	 * [refs], where they start, and their length, which Rust copies once from the array, and
	 * Rust's as [LENT], their owner, where they start, and their length, which Rust reads in
	 * place, kept until [release].
	 */
	fun bytes(v: Bytes?) {
		if (v == null) {
			i32(-1)
			return
		}
		val owner = v.owner
		room(20)
		if (owner != 0L) {
			b.putInt(LENT)
			b.putLong(owner)
			b.putInt(v.start)
			b.putInt(v.size)
			(kept ?: java.util.ArrayList<Any?>().also { kept = it }).add(v.held)
			return
		}
		b.putInt(KOTLIN)
		b.putInt(ref(v.held))
		b.putInt(v.start)
		b.putInt(v.size)
	}

	/** Text as [TEXT] and its place in [refs], which Rust encodes once from, or -1 for none. */
	fun string(v: String?) {
		if (v == null) {
			i32(-1)
			return
		}
		room(8)
		b.putInt(TEXT)
		b.putInt(ref(v))
	}

	private var objects: java.util.ArrayList<Any?>? = null

	// The place in [refs] of [v], added to it.
	private fun ref(v: Any?): Int {
		val objects = objects ?: java.util.ArrayList<Any?>().also { objects = it }
		objects.add(v)
		return objects.size - 1
	}

	/** The arrays and text the encoding refers to, which Rust reads in the call, or null for none. */
	fun refs(): kotlin.Array<Any?>? = objects?.toArray()

	/** An address as Rust's `SocketAddr` holds it, which an unresolved one has no form of. */
	fun socketAddr(v: java.net.InetSocketAddress) {
		val ip = kotlin.requireNotNull(v.address) { "an unresolved address has no Rust SocketAddr" }
		room(16)
		b.put(ip.address)
		repeat(16 - ip.address.size) { b.put(0) }
		i16(v.port.toShort())
		bool(ip is java.net.Inet6Address)
		// Java keeps no flow label.
		i32(0)
		i32((ip as? java.net.Inet6Address)?.scopeId ?: 0)
	}

	/** An IPv4 address, for Rust's `SocketAddrV4`. */
	fun socketAddrV4(v: java.net.InetSocketAddress) {
		kotlin.require(v.address !is java.net.Inet6Address) { "an IPv6 address has no Rust SocketAddrV4" }
		socketAddr(v)
	}

	/** An IPv6 address, for Rust's `SocketAddrV6`. */
	fun socketAddrV6(v: java.net.InetSocketAddress) {
		kotlin.require(v.address !is java.net.Inet4Address) { "an IPv4 address has no Rust SocketAddrV6" }
		socketAddr(v)
	}

	/** A `u128`, from 0 through 2^128 - 1. */
	fun u128(v: java.math.BigInteger) {
		kotlin.require(v.signum() >= 0 && v.bitLength() <= 128) { "$v is out of range for Rust's u128" }
		i64(v.toLong())
		i64(v.shiftRight(64).toLong())
	}

	fun duration(v: kotlin.time.Duration) {
		kotlin.require(!v.isNegative()) { "a negative duration has no Rust Duration" }
		v.toComponents { secs, nanos ->
			i64(secs)
			i32(nanos)
		}
	}

	private var held: java.util.ArrayList<Handle>? = null

	// What holds Rust's bytes encoded in place, which the GC must not free before [release].
	private var kept: java.util.ArrayList<Any?>? = null

	/** An object's handle, held until [release], or 0 for none. */
	fun handle(v: Handle?) {
		if (v == null) {
			i64(0L)
			return
		}
		i64(v.hold())
		(held ?: java.util.ArrayList<Handle>().also { held = it }).add(v)
	}

	fun finish(): ByteArray = b.array().copyOf(b.position())

	/** Release the handles encoded, once the call that reads them returns. */
	fun release() {
		held?.forEach { it.release() }
		java.lang.ref.Reference.reachabilityFence(kept)
	}

	/** Run [block] with this writer, releasing its handles once it returns. */
	inline fun <R> use(block: (Writer) -> R): R {
		try {
			return block(this)
		} finally {
			release()
		}
	}
}

/** Decodes what the library's JNI glue encodes. */
class Reader(bytes: ByteArray) {
	private val b: java.nio.ByteBuffer = java.nio.ByteBuffer.wrap(bytes).order(java.nio.ByteOrder.LITTLE_ENDIAN)

	fun bool(): Boolean = b.get() != 0.toByte()

	fun i8(): Byte = b.get()

	fun i16(): Short = b.getShort()

	fun i32(): Int = b.getInt()

	fun i64(): Long = b.getLong()

	fun f32(): Float = b.getFloat()

	fun f64(): Double = b.getDouble()

	/** `n` bytes, of a length both sides know, such as an array's. */
	fun raw(n: Int): ByteArray = ByteArray(n).also { b.get(it) }

	fun bytes(): ByteArray? {
		val n = b.getInt()
		if (n < 0) return null
		val v = ByteArray(n)
		b.get(v)
		return v
	}

	/** Bytes Rust lent a call, copied, or null for none. */
	fun lent(): Bytes? = bytes()?.let { Bytes(it) }

	/** Bytes Rust handed over, copied or lent by their owner, or null for none. */
	fun owned(): Bytes? = when (val n = b.getInt()) {
		-1 -> null
		LENT -> Bytes(Lent(b.getLong()), b.getInt())
		else -> Bytes(ByteArray(n).also { b.get(it) })
	}

	fun string(): String? {
		val n = b.getInt()
		if (n < 0) return null
		val v = String(b.array(), b.arrayOffset() + b.position(), n, kotlin.text.Charsets.UTF_8)
		b.position(b.position() + n)
		return v
	}

	/** A path, which on Unix may hold bytes no String holds: those throw. */
	fun path(): String? = bytes()?.decodeToString(throwOnInvalidSequence = true)

	fun socketAddr(): java.net.InetSocketAddress {
		val ip = ByteArray(16)
		b.get(ip)
		val port = b.getShort().toInt() and 0xffff
		val v6 = bool()
		val flowinfo = b.getInt()
		val scope = b.getInt()
		kotlin.check(flowinfo == 0) { "an IPv6 flow label has no Java form" }
		// Built as Inet6Address directly, as InetAddress would read an IPv4-mapped one as IPv4.
		val address = when {
			!v6 -> java.net.InetAddress.getByAddress(ip.copyOf(4))
			scope == 0 -> java.net.Inet6Address.getByAddress(null, ip, null as java.net.NetworkInterface?)
			else -> java.net.Inet6Address.getByAddress(null, ip, scope)
		}
		return java.net.InetSocketAddress(address, port)
	}

	fun u128(): java.math.BigInteger {
		val lo = b.getLong()
		val hi = b.getLong()
		return java.math.BigInteger(1, java.nio.ByteBuffer.allocate(16).putLong(hi).putLong(lo).array())
	}

	fun duration(): kotlin.time.Duration {
		val secs = b.getLong()
		val nanos = b.getInt()
		// Past Long.MAX_VALUE seconds, as no Kotlin Duration but its infinite one holds.
		if (secs < 0) return kotlin.time.Duration.INFINITE
		return secs.toDuration(kotlin.time.DurationUnit.SECONDS) + nanos.toDuration(kotlin.time.DurationUnit.NANOSECONDS)
	}
}

/**
 * Frees a Rust value once the GC collects its owner, unless freed first: a phantom
 * reference, which one daemon thread takes off one queue, as every JVM and Android level
 * has. Freeing first is the fast path, and [untrack] makes it the only one.
 */
abstract class Cleanup(owner: Any?) : java.lang.ref.PhantomReference<Any>(owner, QUEUE) {
	// The GC enqueues only a reference still reachable, so each is linked from [head]
	// until freed.
	private var prev: Cleanup? = null
	private var next: Cleanup? = null

	protected fun track() = synchronized(LOCK) {
		next = head
		head?.prev = this
		head = this
	}

	/** Stop tracking, true if this call did: so whatever frees after it frees once. */
	protected fun untrack(): Boolean = synchronized(LOCK) {
		if (head !== this && prev == null) return false
		prev?.next = next
		next?.prev = prev
		if (head === this) head = next
		prev = null
		next = null
		true
	}

	/** Run on the cleaner's thread once the owner is collected. */
	protected abstract fun collected()

	private companion object {
		val QUEUE = java.lang.ref.ReferenceQueue<Any>()
		val LOCK = Any()
		var head: Cleanup? = null

		init {
			val thread = java.lang.Thread({
				while (true) (QUEUE.remove() as Cleanup).collected()
			}, "rust2-ffi cleaner")
			thread.isDaemon = true
			thread.start()
		}
	}
}

/**
 * A handle [owner] owns, or is lent for a call: each call holds it, so [close] frees it
 * only once no call does, and a call after [close] throws. An owned one is closed once
 * the GC collects [owner], if nothing closed it.
 */
class Handle(owner: Any, internal val raw: Long, private val free: Drop?, private val type: String) :
	Cleanup(if (free == null) null else owner) {
	// The holds, one for the owner until closed and one per call, shifted past a bit set
	// once closed: one word, so a handle allocates nothing beside itself.
	@Volatile @JvmField internal var state = HELD

	init {
		if (free != null) track()
	}

	fun hold(): Long {
		while (true) {
			val n = state
			kotlin.check(n and CLOSED == 0L) { "$type is closed" }
			if (STATE.compareAndSet(this, n, n + HELD)) return raw
		}
	}

	fun release() = freed(STATE.addAndGet(this, -HELD))

	fun close() {
		while (true) {
			val n = state
			if (n and CLOSED != 0L) return
			if (STATE.compareAndSet(this, n, (n or CLOSED) - HELD)) return freed((n or CLOSED) - HELD)
		}
	}

	/**
	 * End a lend, closing the handle, which Rust frees next: a call still holding it would
	 * reach the freed handle, so it throws, as Rust's own check fails once a call borrowed.
	 */
	fun end() {
		close()
		kotlin.check(state == CLOSED) { "$type: a call still holds it when its lend ends" }
	}

	// Free it once no hold is left.
	private fun freed(n: Long) {
		if (n < HELD && free != null && untrack()) free.drop(raw)
	}

	// A call still holding it frees it once it returns.
	override fun collected() = close()

	/** Run [block] with the raw handle, held until it returns. */
	inline fun <R> hold(block: (Long) -> R): R {
		val h = hold()
		try {
			return block(h)
		} finally {
			release()
		}
	}

	private companion object {
		const val CLOSED = 1L
		const val HELD = 2L
		val STATE: java.util.concurrent.atomic.AtomicLongFieldUpdater<Handle> =
			java.util.concurrent.atomic.AtomicLongFieldUpdater.newUpdater(Handle::class.java, "state")
	}
}

/** A waker Rust returned: [wake] wakes it through Rust until [close], or the GC, drops it. */
class Returned(raw: Long) : Waker, AutoCloseable {
	private val handle = Handle(this, raw, Drop { __waker_drop(it) }, "Waker")

	override fun wake() = handle.hold { __waker_wake(it) }

	override fun close() = handle.close()
}

/** Frees a raw handle: a `fun interface`, so the handle crosses unboxed. */
fun interface Drop {
	fun drop(raw: Long)
}

/**
 * Rust's buffer, which [owner] holds, freed once the GC collects this. Each read passes it
 * to the JNI function reading the buffer, as JNI keeps an argument reachable until the call
 * returns: a fence on every JVM and Android level, where `Reference.reachabilityFence`
 * needs API 28.
 */
class Lent(internal val owner: Long) {
	init {
		Free(this, owner)
	}
}

private class Free(lent: Lent, private val owner: Long) : Cleanup(lent) {
	init {
		track()
	}

	override fun collected() {
		untrack()
		__bytes_free(owner)
	}
}

/** What precedes a lent buffer's owner and length where an encoded one's length would. */
const val LENT = -2

/** What precedes the place of Kotlin's array of bytes in a writer's `refs`, where an encoded one's length would. */
const val KOTLIN = -3

/** What precedes the place of Kotlin's text in a writer's `refs`, where an encoded one's length would. */
const val TEXT = -4

/** Bytes a call returned: copied, lent by the owner and length a `LongArray` holds, or null for none. */
fun owned(given: Any?): Bytes? = when (given) {
	null -> null
	is ByteArray -> Bytes(given)
	else -> (given as kotlin.LongArray).let { Bytes(Lent(it[0]), it[1].toInt()) }
}

/** Run [block] with the raw handle of [handle], or 0 for none, held until it returns. */
inline fun <R> hold(handle: Handle?, block: (Long) -> R): R = when (handle) {
	null -> block(0L)
	else -> handle.hold(block)
}

/** A waker a future parks on, which latches a wake that lands before it parks. */
// The atomic itself, so a call allocates one object for it.
class Signal : java.util.concurrent.atomic.AtomicReference<Any?>(null), Waker {
	override fun wake() {
		val parked = getAndSet(WOKEN)
		if (parked !is kotlinx.coroutines.CancellableContinuation<*>) return
		@Suppress("UNCHECKED_CAST")
		val c = parked as kotlinx.coroutines.CancellableContinuation<Unit>
		// Never resumed in place, since the next poll would run inside Rust's wake, which may
		// hold a lock that poll takes: a dispatcher that would resume here is made to defer,
		// and one that cannot, as `Unconfined`, resumes on `Default`.
		val dispatcher = c.context[kotlin.coroutines.ContinuationInterceptor] as? kotlinx.coroutines.CoroutineDispatcher
		when {
			dispatcher != null && dispatcher.isDispatchNeeded(c.context) -> c.resume(Unit)
			dispatcher != null && dispatcher !== kotlinx.coroutines.Dispatchers.Unconfined ->
				dispatcher.dispatch(c.context, java.lang.Runnable { c.resume(Unit) })
			else -> kotlinx.coroutines.Dispatchers.Default.dispatch(c.context, java.lang.Runnable { c.resume(Unit) })
		}
	}

	/** Forget any wake before the next poll, which sees whatever it was for. */
	fun reset() = set(null)

	/** Suspend until a wake since [reset], at once if one already landed. */
	suspend fun park() = kotlinx.coroutines.suspendCancellableCoroutine<Unit> { c ->
		if (!compareAndSet(null, c)) c.resume(Unit)
	}

	private companion object {
		val WOKEN = Any()
	}
}

/**
 * A suspend function Rust polls as a future, one poll at a time from any thread: its first
 * poll launches it in [context], on a job of its own rather than a child of the caller's,
 * since Rust decides when it ends, and never in place, since a poll runs inside Rust. The
 * caller's job cancelling cancels it too. Once it returns, the latest poll's waker is
 * woken; if it throws, the next poll rethrows, which aborts as a callback that throws
 * does. Rust closes it once it drops it, which cancels it. Once cancelled, by the caller
 * or by Rust, it ends unfinished: Rust never sees it done, so a call the caller cancelled
 * reports the caller's cancel.
 */
class Pollable(
	private val body: suspend () -> Unit,
	private val context: kotlin.coroutines.CoroutineContext,
) : AutoCloseable {
	private var job: kotlinx.coroutines.Job? = null
	// What passes the caller's cancel on to [job], disposed once it ends or Rust closes it.
	private var watch: kotlinx.coroutines.DisposableHandle? = null
	private var done = false
	private var failure: Throwable? = null
	// The latest poll's waker, a pointer to Rust's, or 0.
	private var waker = 0L

	// The caller's job reaches it as it starts cancelling, not once its children finish,
	// which only the internal overload of invokeOnCompletion tells.
	@OptIn(kotlinx.coroutines.InternalCoroutinesApi::class)
	fun poll(given: Long): Boolean {
		val old: Long
		synchronized(this) {
			failure?.let { throw it }
			if (done) {
				__waker_drop(given)
				return true
			}
			old = waker
			waker = given
			if (job == null) {
				val launched = kotlinx.coroutines.CoroutineScope(dispatched()).launch { run() }
				job = launched
				watch = context[kotlinx.coroutines.Job]?.invokeOnCompletion(onCancelling = true, invokeImmediately = true) { cause ->
					if (cause != null) launched.cancel()
				}
			}
		}
		if (old != 0L) __waker_drop(old)
		return false
	}

	/** [context] without its job, on a dispatcher that defers, as [Signal] resumes on. */
	private fun dispatched(): kotlin.coroutines.CoroutineContext {
		val bare = context.minusKey(kotlinx.coroutines.Job)
		val dispatcher = bare[kotlin.coroutines.ContinuationInterceptor] as? kotlinx.coroutines.CoroutineDispatcher
		return when {
			dispatcher != null && dispatcher !== kotlinx.coroutines.Dispatchers.Unconfined && dispatcher.isDispatchNeeded(bare) -> bare
			else -> bare + kotlinx.coroutines.Dispatchers.Default
		}
	}

	private suspend fun run() {
		val self = kotlin.coroutines.coroutineContext[kotlinx.coroutines.Job]!!
		try {
			body()
		} catch (e: Throwable) {
			// Cancelled, it ends unfinished: a throw is how it stopped.
			if (self.isCancelled) throw e
			synchronized(this) { failure = e }
		}
		// Cancelled as it returned, even one that ignored the cancel, it ends unfinished.
		if (self.isCancelled) return
		val w: Long
		val watched: kotlinx.coroutines.DisposableHandle?
		synchronized(this) {
			done = true
			w = waker
			waker = 0L
			watched = watch
			watch = null
		}
		watched?.dispose()
		if (w != 0L) {
			__waker_wake(w)
			__waker_drop(w)
		}
	}

	override fun close() {
		val w: Long
		val watched: kotlinx.coroutines.DisposableHandle?
		synchronized(this) {
			w = waker
			waker = 0L
			watched = watch
			watch = null
		}
		watched?.dispose()
		job?.cancel()
		if (w != 0L) __waker_drop(w)
	}
}

/**
 * Await the future the call that returned [first] made and polled once with [signal]:
 * poll it until it finishes, then lift its result. A cancel frees it, which cancels it in
 * Rust and loses nothing.
 */
suspend fun <T> await(
	first: ByteArray,
	signal: Signal,
	poll: (Long, Long) -> ByteArray?,
	free: (Long) -> Unit,
	lift: (Reader) -> T,
): T {
	val r = Reader(first)
	if (r.bool()) return lift(r)
	val raw = r.i64()
	val node = r.i64()
	try {
		while (true) {
			signal.park()
			signal.reset()
			val done = poll(raw, node)
			if (done != null) return lift(Reader(done))
		}
	} finally {
		free(raw)
		__waker_free(node)
	}
}
