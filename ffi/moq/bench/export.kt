// bench/export.c's cases through the generated Kotlin, so the two differ by what JNI and
// the Kotlin wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the JIT's and GC's threads included.

import java.lang.management.ManagementFactory
import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.launch
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.runBlocking
import moq.*

const val GROUPS = 10000
const val FRAMES = 100
const val N = GROUPS * FRAMES
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

// What the loops sum, kept so the JIT cannot drop the calls.
@Volatile var sink = 0

// Groups of 100 frames of `size` bytes written and each read back, its payload's first
// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
suspend fun throughput(track: TrackProducer, subscriber: TrackSubscriber, at: Timestamp, size: Int): Span {
	val groups = ((64 shl 20) / (size * FRAMES)).coerceIn(3, 2000)
	val frame = Bytes(ByteArray(size))
	var sum = 0
	val start = now()
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
	val ns = per(start, groups.toDouble() * FRAMES * size)
	return Span(1e3 / ns.wall, 1e3 / ns.cpu)
}

fun round(): Map<String, Span> {
	val out = linkedMapOf<String, Span>()
	val broadcast = BroadcastInfo.new().produce()
	val track = Timescale.MICRO.use { micro ->
		broadcast.createTrack("bench", TrackInfo.default().withTimescale(micro))
	}
	val frame = Bytes(ByteArray(100))
	var sum = 0

	var start = now()
	repeat(N) { if (track.isClosed()) sum++ }
	out["call"] = per(start)

	track.appendGroup().use { group ->
		group.demand().use { demand ->
			start = now()
			repeat(N) { sum += demand.sequence().toInt() }
			out["getter"] = per(start)
		}
		group.finish()
	}

	start = now()
	repeat(N) { if (track.largest() != null) sum++ }
	out["record"] = per(start)

	start = now()
	repeat(N) { track.info().close() }
	out["object"] = per(start)

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var micros = 0uL
	start = now()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) {
				Timestamp.fromMicros(micros++).use { at -> group.writeFrame(at, frame) }
			}
			group.finish()
		}
	}
	out["stamped"] = per(start)

	val at = Timestamp.fromMicros(micros)
	start = now()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) { group.writeFrame(at, frame) }
			group.finish()
		}
	}
	out["write"] = per(start)

	var last: FrameFrame? = null
	runBlocking {
		val subscriber = track.consume().use { it.subscribe(null) }

		// Each frame written, then read through a suspend fun whose future is ready at its
		// first poll, so it never suspends.
		var t0 = now()
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
		out["ready"] = per(t0)

		// Each read started before its frame is written, so it suspends, and the write's wake
		// resumes it through runBlocking's event loop.
		t0 = now()
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
		out["woken"] = per(t0)

		// One writer coroutine and one reader, with at most one frame permitted.
		t0 = now()
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
		out["loop"] = per(t0)

		for (size in SIZES) out["mbps:$size"] = throughput(track, subscriber, at, size)
		subscriber.close()
	}

	// The last frame's 100 bytes, copied across, and one of them read.
	start = now()
	last!!.use { read ->
		repeat(N) { sum += read.payloadGet[0] }
	}
	out["payload"] = per(start)
	at.close()
	track.close()
	broadcast.close()
	sink += sum
	return out
}

fun main() {
	round()
	for ((case, value) in round()) println("$case\t${value.wall}\t${value.cpu}")
}
