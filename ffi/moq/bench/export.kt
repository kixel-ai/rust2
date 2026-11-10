// bench/export.c's cases through the generated Kotlin, so the two differ by what JNI and
// the Kotlin wrapper add. Each round runs twice, the first to warm the JIT.

import kotlinx.coroutines.CoroutineStart
import kotlinx.coroutines.async
import kotlinx.coroutines.runBlocking
import moq.*

const val GROUPS = 10000
const val FRAMES = 100
const val N = GROUPS * FRAMES
val SIZES = listOf(100, 1024, 16384, 65536, 1048576)

// Nanoseconds per call since `start`, over `n` calls.
fun per(start: Long, n: Int = N): Double = (System.nanoTime() - start).toDouble() / n

// What the loops sum, kept so the JIT cannot drop the calls.
@Volatile var sink = 0

// Groups of 100 frames of `size` bytes written and each read back, its payload's first
// byte touched, in about 64 MiB and at least 300 frames: megabytes a second.
suspend fun throughput(track: TrackProducer, subscriber: TrackSubscriber, at: Timestamp, size: Int): Double {
	val groups = ((64 shl 20) / (size * FRAMES)).coerceIn(3, 2000)
	val frame = ByteArray(size)
	var sum = 0
	val start = System.nanoTime()
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
	return groups.toDouble() * FRAMES * size * 1e3 / (System.nanoTime() - start)
}

fun round(): Map<String, Double> {
	val out = linkedMapOf<String, Double>()
	val broadcast = BroadcastInfo.new().produce()
	val track = Timescale.MICRO.use { micro ->
		broadcast.createTrack("bench", TrackInfo.default().withTimescale(micro))
	}
	val frame = ByteArray(100)
	var sum = 0

	var start = System.nanoTime()
	repeat(N) { if (track.isClosed()) sum++ }
	out["call"] = per(start)

	track.appendGroup().use { group ->
		group.demand().use { demand ->
			start = System.nanoTime()
			repeat(N) { sum += demand.sequence().toInt() }
			out["getter"] = per(start)
		}
		group.finish()
	}

	start = System.nanoTime()
	repeat(N) { if (track.largest() != null) sum++ }
	out["record"] = per(start)

	start = System.nanoTime()
	repeat(N) { track.info().close() }
	out["object"] = per(start)

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var micros = 0uL
	start = System.nanoTime()
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
	start = System.nanoTime()
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
		var t0 = System.nanoTime()
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
		t0 = System.nanoTime()
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

		for (size in SIZES) out["mbps:$size"] = throughput(track, subscriber, at, size)
		subscriber.close()
	}

	// The last frame's 100 bytes, copied across, and one of them read.
	start = System.nanoTime()
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
	for ((case, value) in round()) println("$case\t$value")
}
