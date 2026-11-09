// bench/export.cpp's frame writes and reads through the generated Kotlin, so the two differ
// by what JNI and the Kotlin wrapper add. Each round runs twice, the first to warm the JIT.

import kotlinx.coroutines.runBlocking
import moq.*

const val GROUPS = 10000
const val FRAMES = 100

// Nanoseconds per frame since `start`.
fun perFrame(start: Long): Double = (System.nanoTime() - start).toDouble() / (GROUPS * FRAMES)

fun round(): String {
	val broadcast = BroadcastInfo.new().produce()
	val track = Timescale.MICRO.use { micro ->
		broadcast.createTrack("bench", TrackInfo.default().withTimescale(micro))
	}

	// A frame of 100 zero bytes, as bench/export.c writes.
	val frame = ByteArray(100)
	var micros = 0uL

	// Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must.
	var start = System.nanoTime()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) {
				Timestamp.fromMicros(micros++).use { at -> group.writeFrame(at, frame) }
			}
			group.finish()
		}
	}
	val stamped = perFrame(start)

	// The same, reusing one timestamp handle: what the handle costs.
	val at = Timestamp.fromMicros(micros)
	start = System.nanoTime()
	repeat(GROUPS) {
		track.appendGroup().use { group ->
			repeat(FRAMES) { group.writeFrame(at, frame) }
			group.finish()
		}
	}
	val reused = perFrame(start)

	// The floor: a hold of a handle and a bool back.
	start = System.nanoTime()
	var closed = 0
	repeat(GROUPS * FRAMES) {
		if (track.isClosed()) closed++
	}
	val call = perFrame(start)

	// Reads: each frame written, then read through a suspend fun whose future is ready at
	// its first poll, so it never suspends.
	var last: FrameFrame? = null
	var large: FrameFrame? = null
	val read = runBlocking {
		val subscriber = track.consume().use { it.subscribe(null) }
		val start = System.nanoTime()
		repeat(GROUPS) { g ->
			track.appendGroup().use { group ->
				subscriber.recvGroup()!!.use { reader ->
					repeat(FRAMES) { f ->
						group.writeFrame(at, frame)
						val read = reader.readFrame()!!
						if (g == GROUPS - 1 && f == FRAMES - 1) last = read else read.close()
					}
				}
				group.finish()
			}
		}
		perFrame(start).also {
			// A frame long enough that Rust lends its payload rather than copying it.
			track.appendGroup().use { group ->
				subscriber.recvGroup()!!.use { reader ->
					group.writeFrame(at, ByteArray(4096))
					large = reader.readFrame()!!
				}
				group.finish()
			}
			subscriber.close()
		}
	}

	// Payload reads: the last frame's 100 bytes, copied across, and one of them read.
	start = System.nanoTime()
	last!!.use { read ->
		repeat(GROUPS * FRAMES) {
			closed += read.payloadGet[0]
		}
	}
	val payload = perFrame(start)

	// Lent payload reads: a 4 KiB frame's payload, which Rust lends until the GC frees it,
	// copied out.
	start = System.nanoTime()
	large!!.use { read ->
		repeat(GROUPS * FRAMES) {
			closed += read.payloadGet.toByteArray()[0]
		}
	}
	val lentPayload = perFrame(start)
	at.close()
	track.close()
	broadcast.close()

	val bang = if (closed != 0) "!" else ""
	return "export (Kotlin): %.0f ns per stamped 100 B frame, %.0f reusing the timestamp, %.1f ns per handle call, %.0f ns per frame written and read, %.0f ns per payload read, %.0f ns per 4 KiB payload copied out%s"
		.format(stamped, reused, call, read, payload, lentPayload, bang)
}

fun main() {
	round()
	println(round())
}
