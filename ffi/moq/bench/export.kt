// bench/export.c's cases through the generated Kotlin, so the two differ by what JNI and
// the Kotlin wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the JIT's and GC's threads included, or under
// CodSpeed, runs once for its CPU simulation to count.

import java.lang.management.ManagementFactory
import java.util.concurrent.CountDownLatch
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.launch
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.runBlocking
import moq.*

// A tenth under CodSpeed, whose simulation counts every call exactly.
var GROUPS = 10000
const val FRAMES = 100
val N get() = GROUPS * FRAMES
const val THREADS = 4
val SIZES = listOf(100, 1024, 16384, 65536, 1048576)

// Wall and CPU nanoseconds: a point, or a span per call.
data class Span(val wall: Double, val cpu: Double)

val os = ManagementFactory.getOperatingSystemMXBean() as com.sun.management.OperatingSystemMXBean

fun now() = Span(System.nanoTime().toDouble(), os.processCpuTime.toDouble())

// Nanoseconds per call since `start`, over `n` calls.
fun per(start: Span, n: Double = N.toDouble()): Span {
	val end = now()
	return Span((end.wall - start.wall) / n, (end.cpu - start.cpu) / n)
}

// moq/bench/codspeed.h, through the JNI functions in the shared library bench-build makes.
object Codspeed {
	init {
		System.loadLibrary("codspeed")
	}

	external fun init(bench: String): Boolean

	external fun start()

	external fun stop(name: String)
}

// A case's start: now, and CodSpeed counting from here.
fun begin(): Span {
	val start = now()
	Codspeed.start()
	return start
}

// The case `name`'s nanoseconds per call since `start`, over `n` calls.
fun end(start: Span, name: String, n: Double = N.toDouble()): Span {
	Codspeed.stop(name)
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

// Groups of 100 frames of `size` bytes written and each read back, its payload's first
// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
suspend fun throughput(track: TrackProducer, subscriber: TrackSubscriber, at: Timestamp, size: Int): Span {
	val groups = ((64 shl 20) / (size * FRAMES)).coerceIn(3, 2000)
	val frame = Bytes(ByteArray(size))
	val name = "throughput[$size]"
	var sum = 0
	val start = begin()
	repeat(groups) {
		track.appendGroup().use { group ->
			subscriber.recvGroup()!!.use { reader ->
				repeat(FRAMES) {
					group.writeFrame(at, frame)
					reader.readFrame()!!.use { read -> sum += read.payloadGet[0] }
				}
			}
			group.finish()
		}
	}
	sink += sum
	val ns = end(start, name, groups.toDouble() * FRAMES * size)
	return Span(1e3 / ns.wall, 1e3 / ns.cpu)
}

fun round(measured: Boolean): Map<String, Span> {
	val out = linkedMapOf<String, Span>()
	val broadcast = BroadcastInfo.new().produce()
	val track = broadcast.createTrack("bench", TrackInfo.default().withTimescale(Timescale.MICRO))
	val frame = Bytes(ByteArray(100))
	var sum = 0

	var start = begin()
	repeat(N) { if (track.isClosed()) sum++ }
	out["call"] = end(start, "call")

	track.appendGroup().use { group ->
		group.demand().use { demand ->
			start = begin()
			repeat(N) { sum += demand.sequence().toInt() }
			out["getter"] = end(start, "getter")

			// The handle call and the getter from THREADS threads: on one shared handle, then
			// on a clone each, a handle of its own to the same value. CodSpeed's simulation runs
			// one thread at a time, which contends for nothing, so it skips them.
			if (!measured) {
				val isClosed = { t: TrackProducer -> if (t.isClosed()) 1 else 0 }
				val sequence = { d: GroupDemand -> d.sequence().toInt() }
				val tracks = List(THREADS) { track.clone() }
				val demands = List(THREADS) { demand.clone() }
				out["shared:call"] = concurrent(List(THREADS) { track }, isClosed)
				out["own:call"] = concurrent(tracks, isClosed)
				out["shared:getter"] = concurrent(List(THREADS) { demand }, sequence)
				out["own:getter"] = concurrent(demands, sequence)
				tracks.forEach { it.close() }
				demands.forEach { it.close() }
			}
		}
		group.finish()
	}

	start = begin()
	repeat(N) { if (track.largest() != null) sum++ }
	out["record"] = end(start, "record")

	start = begin()
	repeat(N) { track.info().close() }
	out["object"] = end(start, "object")

	// The same from THREADS threads at once, so objects made and freed together contend
	// for whatever the binding shares between them. Kotlin's alone: the others share none.
	// CodSpeed skips it as it does the other threaded cases.
	if (!measured) {
		val began = now()
		val failures = java.util.concurrent.ConcurrentLinkedQueue<Throwable>()
		List(THREADS) { java.lang.Thread { try { repeat(N / THREADS) { track.info().close() } } catch (e: Throwable) { failures.add(e) } }.apply { start() } }.forEach { it.join() }
		// A worker that died early would otherwise report a time for fewer than N calls.
		failures.peek()?.let { throw it }
		out["objects"] = per(began)
	}

	// Each frame stamped through a fresh timestamp, as a caller holding a u64 must.
	var micros = 0uL
	start = begin()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) {
				group.writeFrame(Timestamp.fromMicros(micros++), frame)
			}
			group.finish()
		}
	}
	out["stamped"] = end(start, "stamped")

	val at = Timestamp.fromMicros(micros)
	start = begin()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) { group.writeFrame(at, frame) }
			group.finish()
		}
	}
	out["write"] = end(start, "write")

	var last: FrameFrame? = null
	runBlocking {
		val subscriber = track.consume().use { it.subscribe(null) }

		// Each frame written, then read through a suspend fun whose future is ready at its
		// first poll, so it never suspends.
		var t0 = begin()
		repeat(GROUPS) {
			track.appendGroup().use { group ->
				subscriber.recvGroup()!!.use { reader ->
					repeat(FRAMES) {
						group.writeFrame(at, frame)
						last?.close()
						last = reader.readFrame()!!
					}
				}
				group.finish()
			}
		}
		out["ready"] = end(t0, "ready")

		// Each read started before its frame is written, so it suspends, and the write's wake
		// resumes it through runBlocking's event loop.
		t0 = begin()
		repeat(GROUPS) {
			track.appendGroup().use { group ->
				subscriber.recvGroup()!!.use { reader ->
					repeat(FRAMES) {
						val pending = async(start = CoroutineStart.UNDISPATCHED) { reader.readFrame() }
						group.writeFrame(at, frame)
						pending.await()!!.close()
					}
				}
				group.finish()
			}
		}
		out["woken"] = end(t0, "woken")

		// One writer coroutine and one reader, with at most one frame permitted.
		t0 = begin()
		val permits = Channel<Unit>()
		val writer = launch {
			repeat(GROUPS) {
				track.appendGroup().use { group ->
					repeat(FRAMES) {
						permits.receive()
						group.writeFrame(at, frame)
					}
					permits.receive()
					group.finish()
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

		for (size in SIZES) out["mbps:$size"] = throughput(track, subscriber, at, size)
		subscriber.close()
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
	// Under CodSpeed, one round, which prints nothing: its simulation counts each case.
	if (Codspeed.init("ffi/moq/bench/export.kt::kotlin")) {
		GROUPS /= 10
		round(true)
		return
	}
	round(false)
	for ((case, value) in round(false)) println("$case\t${value.wall}\t${value.cpu}")
}
