// The runtime every generated Kotlin file includes in its `_Native` object, so no name it
// takes can clash with an exported type's: the codec values cross JNI in, the handles
// objects own, what frees them once collected, and the waker futures park on.

/** Encodes values as the library's JNI glue decodes them: little-endian, field by field. */
class __Writer {
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

	/** A block of numbers: its length, then each little-endian, copied in one call. */
	fun block(v: ByteArray) {
		room(4 + v.size)
		b.putInt(v.size)
		b.put(v)
	}

	fun block(v: ShortArray) {
		room(4 + v.size * 2)
		b.putInt(v.size)
		b.asShortBuffer().put(v)
		b.position(b.position() + v.size * 2)
	}

	fun block(v: IntArray) {
		room(4 + v.size * 4)
		b.putInt(v.size)
		b.asIntBuffer().put(v)
		b.position(b.position() + v.size * 4)
	}

	fun block(v: LongArray) {
		room(4 + v.size * 8)
		b.putInt(v.size)
		b.asLongBuffer().put(v)
		b.position(b.position() + v.size * 8)
	}

	fun block(v: FloatArray) {
		room(4 + v.size * 4)
		b.putInt(v.size)
		b.asFloatBuffer().put(v)
		b.position(b.position() + v.size * 4)
	}

	fun block(v: DoubleArray) {
		room(4 + v.size * 8)
		b.putInt(v.size)
		b.asDoubleBuffer().put(v)
		b.position(b.position() + v.size * 8)
	}

	/**
	 * Bytes a record holds, or -1 for none: Kotlin's as [KOTLIN], or a direct buffer's as
	 * [DIRECT], their array's or buffer's place in [refs], where they start, and their
	 * length, which Rust copies once from the array or the buffer's memory, and
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
		if (owner != 0L && owner != __DIRECT) {
			b.putInt(LENT)
			b.putLong(owner)
			b.putInt(v.start)
			b.putInt(v.size)
			(kept ?: java.util.ArrayList<Any?>().also { kept = it }).add(v.held)
			return
		}
		b.putInt(if (owner == __DIRECT) DIRECT else KOTLIN)
		b.putInt(ref(v.held))
		b.putInt(v.start)
		b.putInt(v.size)
	}

	/**
	 * Text as [TEXT], added to [refs], or -1 for none: Rust encodes each in [refs] once,
	 * before it reads any of the value, so a lone surrogate throws before anything moves.
	 */
	fun string(v: String?) {
		if (v == null) {
			i32(-1)
			return
		}
		i32(TEXT)
		ref(v)
	}

	/** A path's bytes as [bytes] encodes Kotlin's, or -1 for none. */
	fun path(v: PathBuf?) {
		if (v == null) {
			i32(-1)
			return
		}
		room(16)
		b.putInt(KOTLIN)
		b.putInt(ref(v.bytes))
		b.putInt(0)
		b.putInt(v.bytes.size)
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

	/** Whether it is IPv6, then the address of its family. */
	fun socketAddr(v: SocketAddr) = when (v) {
		is SocketAddrV4 -> {
			bool(false)
			socketAddrV4(v)
		}
		is SocketAddrV6 -> {
			bool(true)
			socketAddrV6(v)
		}
	}

	fun socketAddrV4(v: SocketAddrV4) {
		ipv4Addr(v.ip)
		i16(v.port.toShort())
	}

	fun socketAddrV6(v: SocketAddrV6) {
		ipv6Addr(v.ip)
		i16(v.port.toShort())
		i32(v.flowinfo.toInt())
		i32(v.scopeId.toInt())
	}

	/** Whether it is IPv6, then the address of its family. */
	fun ipAddr(v: IpAddr) = when (v) {
		is Ipv4Addr -> {
			bool(false)
			ipv4Addr(v)
		}
		is Ipv6Addr -> {
			bool(true)
			ipv6Addr(v)
		}
	}

	/** Its octets, the first highest. */
	fun ipv4Addr(v: Ipv4Addr) = i32(java.lang.Integer.reverseBytes(v.bits.toInt()))

	fun ipv6Addr(v: Ipv6Addr) {
		i64(java.lang.Long.reverseBytes(v.hi))
		i64(java.lang.Long.reverseBytes(v.lo))
	}

	/** A `u128`, from 0 through 2^128 - 1. */
	fun u128(v: java.math.BigInteger) {
		if (v.signum() < 0 || v.bitLength() > 128) throw __Invalid("$v is out of range for Rust's u128")
		i64(v.toLong())
		i64(v.shiftRight(64).toLong())
	}

	fun duration(v: Duration) {
		i64(v.secs.toLong())
		i32(v.nanos.toInt())
	}

	private var held: java.util.ArrayList<__Handle>? = null

	// The slot each of [held] went to, as [__Handle.hold] says.
	private var slots = IntArray(0)

	// What holds Rust's bytes encoded in place, which the GC must not free before [release].
	private var kept: java.util.ArrayList<Any?>? = null

	/** An object's handle, held until [release], or 0 for none. */
	fun handle(v: __Handle?) {
		if (v == null) {
			i64(0L)
			return
		}
		val slot = v.hold()
		i64(v.raw)
		val held = held ?: java.util.ArrayList<__Handle>().also { held = it }
		if (held.size == slots.size) slots = slots.copyOf(kotlin.math.max(4, held.size * 2))
		slots[held.size] = slot
		held.add(v)
	}

	// Whether each of [held] is one [moved] encoded, up to the last it did.
	private var moves: BooleanArray? = null

	/** An object's handle whose value the call takes, as [handle] encodes it, which [abandon] closes. */
	fun moved(v: __Handle?) {
		handle(v)
		if (v == null) return
		val at = held!!.size - 1
		var moves = moves ?: BooleanArray(slots.size)
		if (moves.size <= at) moves = moves.copyOf(slots.size)
		moves[at] = true
		this.moves = moves
	}

	/**
	 * Abandon each handle [moved] encoded, once the call took their values: those encoded,
	 * not those the value holds now, as a callback may have changed it.
	 */
	fun abandon() {
		val moves = moves ?: return
		held?.forEachIndexed { i, h -> if (i < moves.size && moves[i]) h.abandon() }
	}

	/**
	 * The encoding, then where [refs] names any, the place in it of each text, in order, and
	 * how many, which Rust reads first.
	 */
	fun finish(): ByteArray {
		val objects = objects ?: return b.array().copyOf(b.position())
		var texts = 0
		for (at in objects.indices) {
			if (objects[at] is String) {
				i32(at)
				texts++
			}
		}
		i32(texts)
		return b.array().copyOf(b.position())
	}

	/** Release the handles encoded, once the call that reads them returns. */
	fun release() {
		held?.forEachIndexed { i, h -> h.release(slots[i]) }
		// A fence on each, which keeps it reachable up to here.
		kept?.forEach { if (it != null) __Keep.fence(it) }
	}

	/** Run [block] with this writer, releasing its handles once it returns. */
	inline fun <R> use(block: (__Writer) -> R): R {
		try {
			return block(this)
		} finally {
			release()
		}
	}
}

/** Decodes what the library's JNI glue encodes. */
class __Reader(private val b: java.nio.ByteBuffer) {
	constructor(bytes: ByteArray) : this(java.nio.ByteBuffer.wrap(bytes))

	init { b.order(java.nio.ByteOrder.LITTLE_ENDIAN) }

	fun bool(): Boolean = b.get() != 0.toByte()

	fun i8(): Byte = b.get()

	fun i16(): Short = b.getShort()

	fun i32(): Int = b.getInt()

	fun i64(): Long = b.getLong()

	fun f32(): Float = b.getFloat()

	fun f64(): Double = b.getDouble()

	/** `n` bytes, of a length both sides know, such as an array's. */
	fun raw(n: Int): ByteArray = ByteArray(n).also { b.get(it) }

	/** A block of [n] numbers, copied in one call. */
	fun i8s(n: Int): ByteArray = ByteArray(n).also { b.get(it) }

	fun i16s(n: Int): ShortArray = ShortArray(n).also {
		b.asShortBuffer().get(it)
		b.position(b.position() + n * 2)
	}

	fun i32s(n: Int): IntArray = IntArray(n).also {
		b.asIntBuffer().get(it)
		b.position(b.position() + n * 4)
	}

	fun i64s(n: Int): LongArray = LongArray(n).also {
		b.asLongBuffer().get(it)
		b.position(b.position() + n * 8)
	}

	fun f32s(n: Int): FloatArray = FloatArray(n).also {
		b.asFloatBuffer().get(it)
		b.position(b.position() + n * 4)
	}

	fun f64s(n: Int): DoubleArray = DoubleArray(n).also {
		b.asDoubleBuffer().get(it)
		b.position(b.position() + n * 8)
	}

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
		LENT -> Bytes(__Lent(b.getLong()), b.getInt())
		else -> Bytes(ByteArray(n).also { b.get(it) })
	}

	fun string(): String? {
		val n = b.getInt()
		if (n < 0) return null
		// String's constructor beats Charset.decode 2-5x, direct buffers included, even with the copy.
		if (!b.hasArray()) return String(ByteArray(n).also { b.get(it) }, kotlin.text.Charsets.UTF_8)
		val v = String(b.array(), b.arrayOffset() + b.position(), n, kotlin.text.Charsets.UTF_8)
		b.position(b.position() + n)
		return v
	}

	/** A path, as Rust's bytes of it, or null for none. */
	fun path(): PathBuf? = bytes()?.let { PathBuf.of(it) }

	/** Whether it is IPv6, then the address of its family. */
	fun socketAddr(): SocketAddr = if (bool()) socketAddrV6() else socketAddrV4()

	fun socketAddrV4(): SocketAddrV4 = SocketAddrV4(ipv4Addr(), b.getShort().toUShort())

	fun socketAddrV6(): SocketAddrV6 = SocketAddrV6(ipv6Addr(), b.getShort().toUShort(), b.getInt().toUInt(), b.getInt().toUInt())

	/** Whether it is IPv6, then the address of its family. */
	fun ipAddr(): IpAddr = if (bool()) ipv6Addr() else ipv4Addr()

	/** Its octets, the first highest. */
	fun ipv4Addr(): Ipv4Addr = Ipv4Addr(java.lang.Integer.reverseBytes(b.getInt()).toUInt())

	fun ipv6Addr(): Ipv6Addr = Ipv6Addr(java.lang.Long.reverseBytes(b.getLong()), java.lang.Long.reverseBytes(b.getLong()))

	fun u128(): java.math.BigInteger {
		val lo = b.getLong()
		val hi = b.getLong()
		return java.math.BigInteger(1, java.nio.ByteBuffer.allocate(16).putLong(hi).putLong(lo).array())
	}

	fun duration(): Duration = Duration(b.getLong().toULong(), b.getInt().toUInt())
}

/**
 * Loads the library [name]: the copy for this host a desktop jar bundles beside this class,
 * which its index `<name>.natives` names by Rust target, or else, with no index, as on
 * Android, the one on the library path. A bundled copy is loaded from a private directory,
 * since the JVM loads only a file.
 */
private fun __load(name: String) {
	// ART names itself Dalvik; an APK bundles no index, and looking for one would index its
	// resources on the class loader's first lookup.
	val index = if (System.getProperty("java.vm.name") == "Dalvik") null else _Native::class.java.getResourceAsStream("$name.natives")
	if (index == null) {
		System.loadLibrary(name)
		return
	}
	val bundled = index.use { it.readBytes().decodeToString() }.lines().filter { it.isNotEmpty() }.associate {
		val (target, file) = it.split(' ', limit = 2)
		target to file
	}
	val host = __host()
	val file = bundled[host]
		?: throw java.lang.UnsatisfiedLinkError("this jar bundles $name for ${bundled.keys.sorted()}, not this host's $host")
	val library = _Native::class.java.getResourceAsStream("native/$host/$file")
		?: throw java.lang.UnsatisfiedLinkError("this jar's index names native/$host/$file, which it lacks")
	val dir = java.nio.file.Files.createTempDirectory("rust2-$name-")
	val path = dir.resolve(file)
	try {
		library.use { java.nio.file.Files.copy(it, path) }
		System.load(path.toString())
	} catch (e: Throwable) {
		// A library that failed to load is not mapped, so even Windows lets it go.
		path.toFile().delete()
		dir.toFile().delete()
		throw e
	}
	// The file can go once loaded, as its mapping stays, except where the system refuses, as
	// Windows does while the process runs: there the directory is marked stale, and each such
	// load sweeps the stale directories whose process ended, whose files it can delete.
	if (!(path.toFile().delete() && dir.toFile().delete())) {
		java.io.File(dir.toFile(), "stale").createNewFile()
		dir.parent.toFile().listFiles()?.forEach { stale ->
			if (stale.name.startsWith("rust2-$name-") && stale != dir.toFile() && java.io.File(stale, "stale").isFile) {
				val files = stale.listFiles().orEmpty().filter { it.name != "stale" }
				if (files.all { it.delete() }) {
					java.io.File(stale, "stale").delete()
					stale.delete()
				}
			}
		}
	}
}

/** This host as a Rust target names it, for the bundled libraries built for one. */
private fun __host(): String {
	val os = System.getProperty("os.name").orEmpty()
	val arch = when (val arch = System.getProperty("os.arch").orEmpty()) {
		"amd64", "x86_64" -> "x86_64"
		"aarch64", "arm64" -> "aarch64"
		else -> arch
	}
	return when {
		os == "Linux" -> "$arch-unknown-linux-gnu"
		os == "Mac OS X" -> "$arch-apple-darwin"
		os.startsWith("Windows") -> "$arch-pc-windows-msvc"
		else -> "$arch $os"
	}
}

/** The most a `usize` holds on the library's target, whose `isize` holds half. */
private val USIZE_MAX: ULong = __usize_max().toULong()

/** A `usize` as JNI passes it, throwing past the library's target's, which may be 32-bit. */
fun __usize(v: ULong): Long {
	if (v > USIZE_MAX) throw __Invalid("$v is out of range for this target's usize")
	return v.toLong()
}

/** An `isize`, throwing past the library's target's, which may be 32-bit. */
fun __isize(v: Long): Long {
	val max = (USIZE_MAX shr 1).toLong()
	if (v > max || v < -max - 1) throw __Invalid("$v is out of range for this target's isize")
	return v
}

/** A `NonZero`'s integer as JNI passes it, throwing if it is zero, as no `NonZero` is. */
fun __nonZero(v: Byte): Byte = if (v != 0.toByte()) v else throw __Invalid("a NonZero is zero")

fun __nonZero(v: Short): Short = if (v != 0.toShort()) v else throw __Invalid("a NonZero is zero")

fun __nonZero(v: Int): Int = if (v != 0) v else throw __Invalid("a NonZero is zero")

fun __nonZero(v: Long): Long = if (v != 0L) v else throw __Invalid("a NonZero is zero")

/** A `char` as JNI passes it, throwing unless [v] is a Unicode scalar value, as every Rust `char` is. */
fun __char(v: Int): Int {
	if (v.toUInt() > 0x10ffffu || v ushr 11 == 0x1b) throw __Invalid("$v is no Unicode scalar value, as a Rust char is")
	return v
}

/**
 * Frees a Rust value once the GC collects its owner, unless freed first: a phantom
 * reference, which one daemon thread takes off one queue, as every JVM and Android level
 * has. Freeing first is the fast path, and [untrack] makes it the only one.
 */
abstract class __Cleanup(owner: Any?) : java.lang.ref.PhantomReference<Any>(owner, QUEUE) {
	// The GC enqueues only a reference still reachable, so each is linked from its stripe's
	// head until freed. The stripe is the making thread's, so threads making objects
	// at once rarely share a lock, and a free from any thread finds it in [stripe].
	private var prev: __Cleanup? = null
	private var next: __Cleanup? = null
	// Final, so a thread the object reaches unsafely still sees it.
	private val stripe = STRIPES[java.lang.Thread.currentThread().hashCode() and (STRIPES.size - 1)]

	protected fun track() {
		val s = stripe
		synchronized(s) {
			next = s.head
			s.head?.prev = this
			s.head = this
		}
	}

	/** Stop tracking, true if this call did: so whatever frees after it frees once. */
	protected fun untrack(): Boolean {
		val s = stripe
		synchronized(s) {
			if (s.head !== this && prev == null) return false
			prev?.next = next
			next?.prev = prev
			if (s.head === this) s.head = next
			prev = null
			next = null
			return true
		}
	}

	/** Run on the cleaner's thread once the owner is collected. */
	protected abstract fun collected()

	private companion object {
		val QUEUE = java.lang.ref.ReferenceQueue<Any>()
		/** One lock and list of the tracked. */
		class Stripe {
			@kotlin.jvm.JvmField var head: __Cleanup? = null
		}

		// A power of two, past the cores a phone or most servers have.
		val STRIPES = Array(64) { Stripe() }

		init {
			val thread = java.lang.Thread({
				while (true) (QUEUE.remove() as __Cleanup).collected()
			}, "rust2-ffi cleaner")
			thread.isDaemon = true
			thread.start()
		}
	}
}

/**
 * A handle [owner] owns, or is lent for a call: each call holds it, so it is freed only
 * once no call does, and a call after [close] throws. [close] of one whose value Rust
 * checks, which [drop] drops, drops the value as Rust's own drop does, throwing while a
 * call borrows it in Rust; of any other, it throws while a call holds it. An owned one is
 * freed once the GC collects [owner], if nothing closed it: it starts in its thread's
 * [__Nursery], and registers with the GC only once it leaves that open.
 */
class __Handle(
	owner: Any,
	@PublishedApi internal val raw: Long,
	private val free: __Drop?,
	private val type: String,
	private val drop: __Close? = null,
) {
	// The holds, one for the owner until closed and one per call, shifted past a bit set
	// once closed, one set while a [close] that may yet fail decides, and one set once calls
	// contended for them, after which they hold it in [SLOTS]: one word, so a call reads
	// them all at once, and below [HELD] only once no hold is left.
	@kotlin.concurrent.Volatile @kotlin.jvm.JvmField internal var state = HELD

	// What a slot holds for this handle, given once calls first contend for it, before
	// [BIAS]: unique while the handle lives, unlike [raw], which Rust reuses once freed
	// even while a closed handle is still reachable.
	@kotlin.concurrent.Volatile @kotlin.jvm.JvmField internal var key = 0L

	// What frees it once the GC collects its owner, from when it leaves its nursery open.
	// Plain: a free that misses an eviction's write leaves a cleanup that finds it closed.
	private var cleanup: __Collected? = null

	init {
		if (free != null) __Nursery.push(owner, this)
	}

	/**
	 * Register it with the GC as it leaves its nursery, unless closed, when its last hold
	 * frees it. Racing that close, it may register one closed meanwhile, whose cleanup then
	 * finds it closed and frees nothing: [CLOSED], claimed once, decides the one free.
	 */
	internal fun evict(owner: Any) {
		// A close still deciding may leave it open.
		if (state and (CLOSED or CLOSING) == CLOSED) return
		cleanup = __Collected(owner, this)
	}

	/** Hold it for a call, until [release] takes what this returns. Throws once closed. */
	fun hold(): Int {
		val n = state
		if (n and (CLOSED or BIAS) == 0L && STATE.compareAndSet(this, n, n + HELD)) return -1
		return contend()
	}

	// [hold] once calls contend for it: in the table, unless another handle or thread has
	// its slot, then in the count, after which later calls go to the table.
	private fun contend(): Int {
		if (state and BIAS != 0L) {
			val i = slot()
			val k = key
			if (SLOTS.compareAndSet(i, 0L, k)) {
				if (state and CLOSED == 0L) return i
				// A close claimed it meanwhile, and may have counted this hold.
				if (!SLOTS.compareAndSet(i, k, 0L)) {
					SLOTS.set(i, 0L)
					release(-1)
				}
				throw IllegalStateException("$type is closed")
			}
		}
		while (true) {
			val n = state
			kotlin.check(n and CLOSED == 0L) { "$type is closed" }
			if (STATE.compareAndSet(this, n, n + HELD)) break
		}
		if (key == 0L) KEY.compareAndSet(this, 0L, KEYS.addAndGet(2))
		while (true) {
			val n = state
			if (n and BIAS != 0L || STATE.compareAndSet(this, n, n or BIAS)) return -1
		}
	}

	// This thread's slot of the table for this handle: a column of a cache line per thread,
	// and a row in it per handle.
	private fun slot(): Int {
		val column = (java.lang.Thread.currentThread().hashCode() * FIBONACCI ushr (32 - COLUMN_BITS))
		return column * STRIDE + row()
	}

	private fun row(): Int = ((raw ushr 4) * FIBONACCI_LONG ushr (64 - ROW_BITS)).toInt()

	fun release(slot: Int) {
		if (slot >= 0 && SLOTS.getAndSet(slot, 0L) == key) return
		freed(STATE.addAndGet(this, -HELD))
	}

	/**
	 * Close it as [__Handle] says, [owner] reachable until it returns, so a cleanup of the
	 * owner never waits out a drop; again, it does nothing.
	 */
	fun close(owner: Any) {
		if (drop != null) {
			try {
				return closeChecked(drop)
			} finally {
				__Keep.fence(owner)
			}
		}
		while (true) {
			val n = state
			// Another close is deciding: wait for its outcome, which may leave it open.
			if (n and CLOSING != 0L) {
				java.lang.Thread.yield()
				continue
			}
			if (n and CLOSED != 0L) return
			kotlin.check(n and BIAS.inv() == HELD) { "$type: a handle is already borrowed" }
			if (STATE.compareAndSet(this, n, n or CLOSED or CLOSING)) break
		}
		// Claimed closed, so a call holding it in the table after this finds it closed, and
		// one before is in the table. A call that left the table finding it closed held
		// nothing, so the count is still the owner's alone.
		if (listed()) {
			STATE.addAndGet(this, -(CLOSED or CLOSING))
			throw IllegalStateException("$type: a handle is already borrowed")
		}
		state = CLOSED
		freed(CLOSED)
	}

	// [close] of a handle whose value Rust checks.
	private fun closeChecked(drop: __Close) {
		// Held by no call, as soon after its return: claimed closed so none starts, then the
		// value dropped and the handle freed in one crossing. A call meanwhile throws as
		// closed, as one racing any close may. A drop that passed stored CLOSED itself.
		while (true) {
			while (!STATE.compareAndSet(this, HELD, CLOSED or CLOSING)) {
				if (closeMet(drop)) return
			}
			try {
				drop.close(raw, this)
			} catch (e: Throwable) {
				// Rust refused the drop, before it committed: reopened. It refused before
				// draining what other threads left this one, which may end the borrow that
				// refused it: drained, it tries again.
				if (STATE.compareAndSet(this, CLOSED or CLOSING, HELD)) {
					if (__close_drain(raw)) continue
					throw e
				}
				// Committed, then a `Drop` unwound: Rust freed the handle all the same.
				cleanup?.stop()
				throw e
			}
			cleanup?.stop()
			return
		}
	}

	// [closeChecked] once a call holds it or another close is deciding: whether it is done,
	// rather than to try the one crossing again.
	private fun closeMet(drop: __Close): Boolean {
		val n = state
		// Another close is deciding: wait for its outcome, which may leave it open. Rust
		// clears CLOSING once the drop can no longer fail, and runs none of the program's
		// code on the deciding thread before then, so a close a `Drop` makes, from any
		// thread, finds it closed rather than waiting on itself. A home runs one queued
		// there only as it drains, which the wait does, for a callback on that home.
		if (n and CLOSING != 0L) {
			__close_drain(0L)
			java.lang.Thread.yield()
			return false
		}
		if (n and CLOSED != 0L) return true
		val slot = try {
			hold()
		} catch (_: IllegalStateException) {
			return false
		}
		try {
			drop.close(raw, null)
		} finally {
			release(slot)
		}
		abandon()
		return true
	}

	// Whether a call holds it in the table.
	private fun listed(): Boolean {
		if (state and BIAS == 0L) return false
		val k = key
		var i = row()
		while (i < SLOTS.length()) {
			if (SLOTS.get(i) == k) return true
			i += STRIDE
		}
		return false
	}

	/**
	 * End a lend, closing the handle, which Rust frees next: a call still holding it would
	 * reach the freed handle, so it throws, as Rust's own check fails once a call borrowed.
	 */
	fun end() {
		abandon()
		kotlin.check(state and BIAS.inv() == CLOSED) { "$type: a call still holds it when its lend ends" }
	}

	/**
	 * Give up the owner's hold, freeing it once no call holds one: once a call moved its
	 * value out, or the GC collected the owner.
	 */
	fun abandon() {
		while (true) {
			val n = state
			// A close is deciding: wait for its outcome, which may leave it open.
			if (n and CLOSING != 0L) {
				java.lang.Thread.yield()
				continue
			}
			if (n and CLOSED != 0L) return
			if (STATE.compareAndSet(this, n, n or CLOSED)) break
		}
		// Claimed closed, so each call holding it in the table did so before: count them.
		if (state and BIAS != 0L) {
			val k = key
			var i = row()
			while (i < SLOTS.length()) {
				if (SLOTS.get(i) == k) {
					STATE.addAndGet(this, HELD)
					if (!SLOTS.compareAndSet(i, k, k or REVOKED)) STATE.addAndGet(this, -HELD)
				}
				i += STRIDE
			}
		}
		release(-1)
	}

	// Free it once no hold is left, which happens once.
	private fun freed(n: Long) {
		if (n < HELD && free != null) {
			cleanup?.stop()
			free.drop(raw)
		}
	}

	/** Run [block] with the raw handle, held until it returns. */
	inline fun <R> hold(block: (Long) -> R): R {
		val slot = hold()
		try {
			return block(raw)
		} finally {
			release(slot)
		}
	}

	private companion object {
		const val CLOSED = 1L
		const val BIAS = 2L
		const val CLOSING = 4L
		const val HELD = 8L
		// Set in a slot whose hold a close counted.
		const val REVOKED = 1L
		// Once calls contend for a handle's count, each holds it in a slot of SLOTS instead,
		// so threads sharing a handle write nothing in common. A close that would free the
		// handle claims it closed first, then counts each hold it finds there, which lets go
		// through the count, so the last to let go frees it: ffi/tests/hold.rs models this.
		// A cache line of rows to a column, which the thread picks, and a row per handle: two
		// threads in one column, or two handles in one row on one thread, use the count.
		const val ROW_BITS = 3
		const val COLUMN_BITS = 8
		// Two cache lines to a column, of which it uses one's worth, so a column shares no line
		// with the next wherever the array starts.
		const val STRIDE = 2 shl ROW_BITS
		const val FIBONACCI = -0x61c88647
		const val FIBONACCI_LONG = -0x61c8864680b583ebL
		val SLOTS = java.util.concurrent.atomic.AtomicLongArray(STRIDE shl COLUMN_BITS)
		val STATE: java.util.concurrent.atomic.AtomicLongFieldUpdater<__Handle> =
			java.util.concurrent.atomic.AtomicLongFieldUpdater.newUpdater(__Handle::class.java, "state")
		val KEY: java.util.concurrent.atomic.AtomicLongFieldUpdater<__Handle> =
			java.util.concurrent.atomic.AtomicLongFieldUpdater.newUpdater(__Handle::class.java, "key")
		// The last key given, in steps of two, so a key's low bit is clear for [REVOKED].
		val KEYS = java.util.concurrent.atomic.AtomicLong()
	}
}

/** What frees an owned handle once the GC collects [owner]: a call still holding it frees it once it returns. */
private class __Collected(owner: Any, private val handle: __Handle) : __Cleanup(owner) {
	init {
		track()
	}

	override fun collected() {
		untrack()
		handle.abandon()
	}

	/** Stop it, as the handle is freed first. */
	fun stop() {
		untrack()
	}
}

/**
 * The latest owned handles a thread was returned, each beside its owner, which a free
 * leaves without the GC's registration, a phantom reference linked under a lock, which
 * costs more than the rest of a return and its close together. One that leaves it open,
 * pushed out by later returns or with its nursery once the GC drops that, registers then.
 */
private class __Nursery {
	// Kept reachable while here, so an eviction can register them. Only the thread
	// whose [__Ring] reaches it touches these, until [flush], once nothing reaches that.
	@kotlin.jvm.JvmField val owners = arrayOfNulls<Any>(LEN)
	@kotlin.jvm.JvmField val handles = arrayOfNulls<__Handle>(LEN)

	/**
	 * Register each handle still open, once the GC collected the ring that reached this,
	 * and let go of them all, so none waits on the others to be collected. The collector
	 * stopped the thread that filled them after its last push, which orders its writes
	 * before this.
	 */
	fun flush() {
		for (i in 0 until LEN) {
			handles[i]?.evict(owners[i]!!)
			handles[i] = null
			owners[i] = null
		}
	}

	companion object {
		/**
		 * How many of the latest objects returned a nursery holds: an object dropped
		 * unclosed is freed only once this many more were returned on its thread, or the
		 * GC dropped its nursery, so it bounds what a dropped object keeps of Rust's.
		 */
		const val LEN = 64

		// Each thread's ring, held weakly, which a collection that reaches it while the
		// thread is not pushing drops: so an idle thread's nursery keeps what it holds only
		// until such a collection and one more, not forever.
		private val RINGS = java.lang.ThreadLocal<java.lang.ref.WeakReference<__Ring>>()

		/** Put [handle], which [owner] owns, in this thread's nursery, oldest out. */
		fun push(owner: Any, handle: __Handle) {
			var ring = RINGS.get()?.get()
			if (ring == null) {
				ring = __Ring(__Nursery())
				__Flush(ring)
				RINGS.set(java.lang.ref.WeakReference(ring))
			}
			ring.push(owner, handle)
		}
	}
}

/** What a thread holds of its nursery, only weakly, so the GC drops an idle one. */
private class __Ring(@kotlin.jvm.JvmField val nursery: __Nursery) {
	// The oldest slot once full.
	private var next = 0

	fun push(owner: Any, handle: __Handle) {
		val n = nursery
		val i = next
		n.handles[i]?.evict(n.owners[i]!!)
		n.owners[i] = owner
		n.handles[i] = handle
		next = (i + 1) and (__Nursery.LEN - 1)
		// Its flush must not run while this pushes, and no plain store keeps it reachable.
		__Keep.fence(this)
	}
}

/** Keeps an object reachable up to here, by the strongest means the platform has. */
private object __Keep {
	// Decided once, so the compiler folds the branch away.
	@kotlin.jvm.JvmField val REFERENCE = try {
		java.lang.ref.Reference::class.java.getMethod("reachabilityFence", Any::class.java)
		true
	} catch (e: NoSuchMethodException) {
		false
	}

	/**
	 * `Reference.reachabilityFence` where the platform has it, free on HotSpot, which
	 * Android adds at API 28. Elsewhere a monitor held on [o], which keeps [o] a root while
	 * held, and which no earlier write is ordered after.
	 */
	fun fence(o: Any) {
		if (REFERENCE) java.lang.ref.Reference.reachabilityFence(o) else synchronized(o) {}
	}
}

/** What flushes a ring's nursery once the GC collects the ring. */
private class __Flush(ring: __Ring) : __Cleanup(ring) {
	private val nursery = ring.nursery

	init {
		track()
	}

	override fun collected() {
		untrack()
		nursery.flush()
	}
}

/** A waker Rust returned: [wake] wakes it through Rust until [close], or the GC, drops it. */
class __Returned(raw: Long) : Waker, AutoCloseable {
	private val handle = __Handle(this, raw, __Drop { __waker_drop(it) }, "Waker")

	override fun wake() = handle.hold { __waker_wake(it) }

	override fun close() = handle.abandon()
}

/**
 * A callback's implementation wrapping another, as a `Java` interface's `of` does, which Rust
 * closes in its place as it drops this, where it is `AutoCloseable`. The wrapper is never
 * `AutoCloseable` itself, so a callback's own method `close()` stays the callback's.
 */
interface __Wraps {
	val __wrapped: Any
}

/** Frees a raw handle: a `fun interface`, so the handle crosses unboxed. */
fun interface __Drop {
	fun drop(raw: Long)
}

/**
 * Drops the value of a raw handle, then, given its [closing] handle, frees the handle too if
 * the drop passed, storing that handle's state closed once the drop can no longer fail.
 */
fun interface __Close {
	fun close(raw: Long, closing: __Handle?)
}

/**
 * Rust's buffer, which [owner] holds, freed once the GC collects this. Each read passes it
 * to the JNI function reading the buffer, as JNI keeps an argument reachable until the call
 * returns, on every JVM and Android level.
 */
class __Lent(internal val owner: Long) {
	init {
		__Free(this, owner)
	}
}

private class __Free(lent: __Lent, private val owner: Long) : __Cleanup(lent) {
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

/** What precedes the place of a direct `ByteBuffer` in a writer's `refs`, where an encoded one's length would. */
const val DIRECT = -5

/** The owner a direct buffer's bytes cross a call with, which no buffer Rust lends has. */
const val __DIRECT = -1L

/** [buffer]'s array, shared, or for a read-only one a copy of its bytes from its position. */
fun __heap(buffer: java.nio.ByteBuffer): ByteArray = when (buffer.hasArray()) {
	true -> buffer.array()
	false -> ByteArray(buffer.remaining()).also { buffer.duplicate().get(it) }
}

/** What marks Kotlin's text, which a writer's `refs` holds, where an encoded one's length would. */
const val TEXT = -4

/** Bytes a call returned: copied, lent by the owner and length a `LongArray` holds, or null for none. */
fun owned(given: Any?): Bytes? = when (given) {
	null -> null
	is ByteArray -> Bytes(given)
	else -> (given as kotlin.LongArray).let { Bytes(__Lent(it[0]), it[1].toInt()) }
}

/** Run [block] with the raw handle of [handle], or 0 for none, held until it returns. */
inline fun <R> hold(handle: __Handle?, block: (Long) -> R): R = when (handle) {
	null -> block(0L)
	else -> handle.hold(block)
}

/** A waker a future parks on, which latches a wake that lands before it parks. */
// The atomic itself, so a call allocates one object for it.
class __Signal : java.util.concurrent.atomic.AtomicReference<Any?>(null), Waker {
	override fun wake() {
		val parked = getAndSet(WOKEN)
		if (parked !is kotlinx.coroutines.CancellableContinuation<*>) return
		@kotlin.Suppress("UNCHECKED_CAST")
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
class __Pollable(
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
	@kotlin.OptIn(kotlinx.coroutines.InternalCoroutinesApi::class)
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

	/** [context] without its job, on a dispatcher that defers, as [__Signal] resumes on. */
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

/** A completed native future, including one whose value is null. */
class __Ready(val value: Any?)

/** A native future still waiting on its waker node. */
class __Pending(val raw: Long, val node: Long)

/**
 * Await the future the call that returned [first] made and polled once with [signal]:
 * poll it until it finishes, then lift its result. A cancel frees it, which cancels it in
 * Rust and loses nothing.
 */
suspend fun await(
	first: Any?,
	signal: __Signal,
	poll: (Long, Long) -> Any?,
	free: (Long) -> Unit,
): Any? {
	if (first is __Ready) return first.value
	val pending = first as __Pending
	val raw = pending.raw
	val node = pending.node
	try {
		while (true) {
			signal.park()
			signal.reset()
			val done = poll(raw, node)
			if (done != null) return (done as __Ready).value
		}
	} finally {
		free(raw)
		__waker_free(node)
	}
}

/**
 * Equality as Rust's `==` has it, apart from the exported functions, which a nested object's
 * members never overload.
 */
object __Eq {
	/**
	 * Whether [a] and [b] are equal as Rust's `==` compares them: primitive arrays, lists, and
	 * tuples by their contents, and floats by IEEE `==`, so NaN equals nothing and -0.0 equals
	 * 0.0, where `equals` and `contentEquals` compare a float's bits.
	 */
	fun same(a: Any?, b: Any?): Boolean = when {
		a is Float && b is Float -> a == b
		a is Double && b is Double -> a == b
		a is kotlin.collections.List<*> && b is kotlin.collections.List<*> -> a.size == b.size && a.indices.all { same(a[it], b[it]) }
		a is kotlin.Pair<*, *> && b is kotlin.Pair<*, *> -> same(a.first, b.first) && same(a.second, b.second)
		a is kotlin.Triple<*, *, *> && b is kotlin.Triple<*, *, *> -> same(a.first, b.first) && same(a.second, b.second) && same(a.third, b.third)
		a is FloatArray && b is FloatArray -> a.size == b.size && a.indices.all { a[it] == b[it] }
		a is DoubleArray && b is DoubleArray -> a.size == b.size && a.indices.all { a[it] == b[it] }
		a is ByteArray && b is ByteArray -> a.contentEquals(b)
		a is ShortArray && b is ShortArray -> a.contentEquals(b)
		a is IntArray && b is IntArray -> a.contentEquals(b)
		a is LongArray && b is LongArray -> a.contentEquals(b)
		a is UShortArray && b is UShortArray -> a.contentEquals(b)
		a is UIntArray && b is UIntArray -> a.contentEquals(b)
		a is ULongArray && b is ULongArray -> a.contentEquals(b)
		else -> a == b
	}

	/** A float's hash agreeing with [same]: -0.0 hashes as 0.0. */
	private fun floatHash(v: Float): Int = if (v == 0.0f) 0 else v.hashCode()

	private fun doubleHash(v: Double): Int = if (v == 0.0) 0 else v.hashCode()

	/** The hash of a class of [fields], agreeing with [same], as a list of them hashes. */
	fun hashOf(vararg fields: Any?): Int = fields.fold(1) { h, v -> 31 * h + hash(v) }

	/** A hash agreeing with [same]. */
	fun hash(a: Any?): Int = when (a) {
		is Float -> floatHash(a)
		is Double -> doubleHash(a)
		is kotlin.collections.List<*> -> a.fold(1) { h, v -> 31 * h + hash(v) }
		is kotlin.Pair<*, *> -> 31 * hash(a.first) + hash(a.second)
		is kotlin.Triple<*, *, *> -> 31 * (31 * hash(a.first) + hash(a.second)) + hash(a.third)
		is FloatArray -> a.fold(1) { h, v -> 31 * h + floatHash(v) }
		is DoubleArray -> a.fold(1) { h, v -> 31 * h + doubleHash(v) }
		is ByteArray -> a.contentHashCode()
		is ShortArray -> a.contentHashCode()
		is IntArray -> a.contentHashCode()
		is LongArray -> a.contentHashCode()
		is UShortArray -> a.contentHashCode()
		is UIntArray -> a.contentHashCode()
		is ULongArray -> a.contentHashCode()
		else -> a?.hashCode() ?: 0
	}
}
