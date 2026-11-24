// bench/export.c's cases through the generated Kotlin, so the two differ by what JNI and
// the Kotlin wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the JIT's and GC's threads included, or under
// Valgrind, runs once for rust2-bench to count. It runs on HotSpot and, shrunk by R8,
// on ART.

import java.lang.management.ManagementFactory
import java.util.concurrent.CountDownLatch
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.launch
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.runBlocking
import moq.*

// A tenth under Valgrind, which counts every call exactly.
var GROUPS = 10000
const val FRAMES = 100
val N get() = GROUPS * FRAMES
const val THREADS = 4
val SIZES = listOf(100, 1024, 16384, 65536, 1048576)

// Wall and CPU nanoseconds: a point, or a span per call.
data class Span(val wall: Double, val cpu: Double)

// ART, which has no java.lang.management, reports itself as Dalvik.
val art = System.getProperty("java.vm.name") == "Dalvik"

// HotSpot's count of the process's CPU time, apart so that ART never resolves its class.
object HotSpot {
	val os = ManagementFactory.getOperatingSystemMXBean() as com.sun.management.OperatingSystemMXBean
}

// The process's CPU nanoseconds, every thread's, ended ones included: on ART, the kernel's
// user and system ticks, at the 100 Hz USER_HZ every Linux ABI fixes, which is HotSpot's
// own clock on Linux too.
fun cpu(): Long = when (art) {
	true -> java.io.File("/proc/self/stat").readText().substringAfterLast(") ").split(' ').let { stat ->
		(stat[11].toLong() + stat[12].toLong()) * 10_000_000
	}
	false -> HotSpot.os.processCpuTime
}

fun now() = Span(System.nanoTime().toDouble(), cpu().toDouble())

// Nanoseconds per call since `start`, over `n` calls.
fun per(start: Span, n: Double = N.toDouble()): Span {
	val end = now()
	return Span((end.wall - start.wall) / n, (end.cpu - start.cpu) / n)
}

// Whether Valgrind counts this, as `just ffi count` says: only then is its library loaded,
// which ART's run has none of.
val codspeed = System.getenv("RUST2_CODSPEED") != null

// moq/bench/codspeed.h, through the JNI functions in the shared library bench-build makes.
object Codspeed {
	init {
		System.loadLibrary("codspeed")
	}

	external fun init(bench: String): Boolean

	external fun start()

	external fun stop(name: String)
}

// A case's start: now, and Valgrind counting from here.
fun begin(): Span {
	val start = now()
	if (codspeed) Codspeed.start()
	return start
}

// The case `name`'s nanoseconds per call since `start`, over `n` calls.
fun end(start: Span, name: String, n: Double = N.toDouble()): Span {
	if (codspeed) Codspeed.stop(name)
	return per(start, n)
}

// What the loops sum, kept so the JIT cannot drop the calls.
@Volatile var sink = 0

// `call` made N times on each of THREADS threads at once, thread i on handles[i]: wall
// nanoseconds per call on one thread, and CPU nanoseconds per call.
fun <T> concurrent(handles: List<T>, call: (T) -> Int): Span {
	val ready = CountDownLatch(THREADS)
	val go = CountDownLatch(1)
	val failures = java.util.concurrent.ConcurrentLinkedQueue<Throwable>()
	val workers = handles.map { handle ->
		Thread {
			var sum = 0
			ready.countDown()
			go.await()
			try {
				repeat(N) { sum += call(handle) }
			} catch (e: Throwable) {
				failures.add(e)
			}
			sink += sum
		}.apply { start() }
	}
	ready.await()
	val start = now()
	go.countDown()
	workers.forEach { it.join() }
	// A worker that died early would otherwise report a time for fewer than N calls.
	failures.peek()?.let { throw it }
	return per(start, N.toDouble()).let { Span(it.wall, it.cpu / THREADS) }
}

// A track named `name` on `broadcast`. It caches every group written to it until it closes,
// as the broadcast's pool, which nothing polls, evicts none, so each case that writes groups
// writes them to a track of its own, closed once it ends: a round holds one case's frames at
// a time instead of every case's, about 2 GB.
fun track(broadcast: BroadcastProducer, name: String) = broadcast.createTrack(name, TrackInfo.default().withTimescale(Timescale.MICRO))

// [block] on a track of its own named `name` and a subscriber to it, closed once it returns.
suspend fun <R> subscribed(broadcast: BroadcastProducer, name: String, block: suspend (TrackProducer, TrackSubscriber) -> R): R =
	track(broadcast, name).use { track -> track.consume().use { it.subscribe(null) }.use { block(track, it) } }

// Groups of 100 frames of `size` bytes written and each read back, its payload's first
// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
// Each group starts at `next()`, the time its frames are written at.
suspend fun throughput(broadcast: BroadcastProducer, next: () -> Timestamp, size: Int): Span {
	val groups = ((64 shl 20) / (size * FRAMES)).coerceIn(3, 2000)
	val frame = Bytes(ByteArray(size))
	val name = "throughput[$size]"
	var sum = 0
	return subscribed(broadcast, name) { track, subscriber ->
		val start = begin()
		repeat(groups) {
			val at = next()
			track.appendGroup(at).use { group ->
				subscriber.recvGroup()!!.use { reader ->
					repeat(FRAMES) {
						group.writeFrame(at, frame)
						reader.readFrame()!!.use { read -> sum += read.payloadGet[0] }
					}
				}
				group.finish().close()
			}
		}
		sink += sum
		val ns = end(start, name, groups.toDouble() * FRAMES * size)
		Span(1e3 / ns.wall, 1e3 / ns.cpu)
	}
}

fun round(measured: Boolean): Map<String, Span> {
	val out = linkedMapOf<String, Span>()
	val broadcast = BroadcastInfo.new().produce()
	val track = track(broadcast, "bench")
	val frame = Bytes(ByteArray(100))
	var sum = 0
	// A group's start is its key, so each appended group's must pass the last one's. The
	// cases that write every frame of a group at one time take the next microsecond per
	// group, and so convert a timestamp per group, not per frame.
	var micros = 0uL
	val next = { Timestamp.fromMicros(micros++) }

	var start = begin()
	repeat(N) { if (track.isClosed()) sum++ }
	out["call"] = end(start, "call")

	track.appendGroup(next()).use { group ->
		group.demand().use { demand ->
			start = begin()
			repeat(N) { sum += demand.start().toInt() }
			out["getter"] = end(start, "getter")

			// The handle call and the getter from THREADS threads: on one shared handle, then
			// on a clone each, a handle of its own to the same value. Valgrind runs
			// one thread at a time, which contends for nothing, so it skips them.
			if (!measured) {
				val isClosed = { t: TrackProducer -> if (t.isClosed()) 1 else 0 }
				val groupStart = { d: GroupDemand -> d.start().toInt() }
				val tracks = List(THREADS) { track.clone() }
				val demands = List(THREADS) { demand.clone() }
				out["shared:call"] = concurrent(List(THREADS) { track }, isClosed)
				out["own:call"] = concurrent(tracks, isClosed)
				out["shared:getter"] = concurrent(List(THREADS) { demand }, groupStart)
				out["own:getter"] = concurrent(demands, groupStart)
				tracks.forEach { it.close() }
				demands.forEach { it.close() }
			}
		}
		group.finish().close()
	}

	start = begin()
	repeat(N) { if (track.largest() != null) sum++ }
	out["record"] = end(start, "record")

	start = begin()
	repeat(N) { track.info().close() }
	out["object"] = end(start, "object")

	// A primitive pub field read, which reaches no user code.
	track.info().use { info ->
		start = begin()
		repeat(N) { sum += info.priorityGet }
		out["field"] = end(start, "field")
	}

	// The same from THREADS threads at once, so objects made and freed together contend
	// for whatever the binding shares between them. Kotlin's alone: the others share none.
	// A count skips it as it does the other threaded cases.
	if (!measured) {
		val began = now()
		val failures = java.util.concurrent.ConcurrentLinkedQueue<Throwable>()
		List(THREADS) { java.lang.Thread { try { repeat(N / THREADS) { track.info().close() } } catch (e: Throwable) { failures.add(e) } }.apply { start() } }.forEach { it.join() }
		// A worker that died early would otherwise report a time for fewer than N calls.
		failures.peek()?.let { throw it }
		out["objects"] = per(began)
	}

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must, its
	// group starting at its first frame's.
	track(broadcast, "stamped").use { track ->
		start = begin()
		repeat(GROUPS) {
			track.appendGroup(Timestamp.fromMicros(micros)).use { group ->
				repeat(FRAMES) {
					group.writeFrame(Timestamp.fromMicros(micros++), frame)
				}
				group.finish().close()
			}
		}
		out["stamped"] = end(start, "stamped")
	}

	track(broadcast, "write").use { track ->
		start = begin()
		repeat(GROUPS) {
			val at = next()
			track.appendGroup(at).use { group ->
				repeat(FRAMES) { group.writeFrame(at, frame) }
				group.finish().close()
			}
		}
		out["write"] = end(start, "write")
	}

	var last: FrameFrame? = null
	runBlocking {
		// Each frame written, then read through a suspend fun whose future is ready at its
		// first poll, so it never suspends.
		subscribed(broadcast, "ready") { track, subscriber ->
			val t0 = begin()
			repeat(GROUPS) {
				val at = next()
				track.appendGroup(at).use { group ->
					subscriber.recvGroup()!!.use { reader ->
						repeat(FRAMES) {
							group.writeFrame(at, frame)
							last?.close()
							last = reader.readFrame()!!
						}
					}
					group.finish().close()
				}
			}
			out["ready"] = end(t0, "ready")
		}

		// Each read started before its frame is written, so it suspends, and the write's wake
		// resumes it through runBlocking's event loop.
		subscribed(broadcast, "woken") { track, subscriber ->
			val t0 = begin()
			repeat(GROUPS) {
				val at = next()
				track.appendGroup(at).use { group ->
					subscriber.recvGroup()!!.use { reader ->
						repeat(FRAMES) {
							val pending = async(start = CoroutineStart.UNDISPATCHED) { reader.readFrame() }
							group.writeFrame(at, frame)
							pending.await()!!.close()
						}
					}
					group.finish().close()
				}
			}
			out["woken"] = end(t0, "woken")
		}

		// One writer coroutine and one reader, with at most one frame permitted.
		subscribed(broadcast, "loop") { track, subscriber ->
			val t0 = begin()
			val permits = Channel<Unit>()
			val writer = launch {
				repeat(GROUPS) {
					val at = next()
					track.appendGroup(at).use { group ->
						repeat(FRAMES) {
							permits.receive()
							group.writeFrame(at, frame)
						}
						permits.receive()
						group.finish().close()
					}
				}
			}
			repeat(GROUPS) {
				subscriber.recvGroup()!!.use { reader ->
					repeat(FRAMES) {
						permits.send(Unit)
						reader.readFrame()!!.close()
					}
				}
				permits.send(Unit)
			}
			writer.join()
			permits.close()
			out["loop"] = end(t0, "loop")
		}

		for (size in SIZES) out["mbps:$size"] = throughput(broadcast, next, size)
	}

	// The last frame's 100 bytes, copied across, and one of them read.
	start = begin()
	last!!.use { read ->
		repeat(N) { sum += read.payloadGet[0] }
	}
	out["payload"] = end(start, "payload")
	track.close()
	broadcast.close()
	sink += sum
	return out
}

fun main() {
	// Under Valgrind, one round, which prints nothing: rust2-bench counts each case.
	if (codspeed && Codspeed.init("ffi/moq/bench/export.kt::kotlin")) {
		GROUPS /= 10
		round(true)
		return
	}
	round(false)
	for ((case, value) in round(false)) println("$case\t${value.wall}\t${value.cpu}")
}
