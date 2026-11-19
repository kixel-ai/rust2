// Runs scripts tests/protocol/model.rs writes through the api fixture's Kotlin: each step on
// the main thread or a worker, writing what it returned a line to the script's path with
// `.out` added. Each script's transcript from the model must match it line for line.
//
// A suspend call runs the fixture's futures, which wake themselves, to their end in place,
// so the model awaits each in the step after the one that made it: that step prints what
// the call returned.

import api.*
import java.io.File
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.runBlocking

// What a refused check's message ends with, after the function's name, by the name the
// model prints.
private val STATUSES = listOf(
	"a handle's value was moved out" to "moved",
	"a handle is borrowed exclusively" to "exclusive",
	"a handle is already borrowed" to "borrowed",
	"a handle is borrowed on another thread" to "thread",
	"a handle a call lent is borrowed exclusively or moved out" to "lent",
	"a handle lent from another outlived it: the other was borrowed exclusively, moved out, or freed" to "outlived",
	"a value that is not Send is used on another thread than the one that made it" to "not_send",
)

// The line for what a step threw: Rust's `Err`, a refused check, or a closed handle, which
// a moved-out one is too.
private fun failure(e: Throwable): String = when (e) {
	is ErrorException -> "error"
	is IllegalStateException -> {
		val message = e.message ?: throw e
		when {
			message.endsWith(" is closed") -> "moved"
			else -> STATUSES.firstOrNull { (text, _) -> message.endsWith(": $text") }?.second ?: throw e
		}
	}
	else -> throw e
}

private class Script(lines: List<String>) {
	val slots = arrayOfNulls<Any>(lines.first().split(' ').size)
	// What each future's call returned, which the step awaiting it prints.
	val results = HashMap<Int, ULong>()
	val steps = lines.drop(1)

	inline fun <reified T> at(h: String): T = slots[h.toInt()] as T

	fun opt(h: String): Counter? = if (h == "-") null else at<Counter>(h)

	fun made(h: String, value: Any): String {
		slots[h.toInt()] = value
		return "ok"
	}

	fun future(h: String, call: suspend () -> ULong): String {
		results[h.toInt()] = runBlocking { call() }
		slots[h.toInt()] = h
		return "ok"
	}

	// Run one step on this thread: what it returned.
	fun run(op: String, a: List<String>): String = try {
		when (op) {
			"counter_new" -> made(a[0], Counter.new("c", a[1].toULong()))
			"counter_get" -> at<Counter>(a[0]).get().toString()
			"counter_add" -> at<Counter>(a[0]).add(a[1].toULong()).toString()
			"counter_into_sum" -> at<Counter>(a[0]).intoSum(a[1].toULong()).toString()
			"counter_plus" -> at<Counter>(a[0]).plus(opt(a[1])).toString()
			"counter_add_to" -> if (at<Counter>(a[0]).addTo(opt(a[1]))) "1" else "0"
			"counter_clone" -> made(a[0], at<Counter>(a[1]).clone())
			"plus_later" -> {
				val (counter, other) = at<Counter>(a[1]) to opt(a[2])
				future(a[0]) { counter.plusLater(other) }
			}
			"ledger_new" -> made(a[0], Ledger.new())
			"ledger_edit" -> {
				made(a[0], at<Ledger>(a[1]).edit())
				"1"
			}
			"ledger_published" -> at<Ledger>(a[0]).published().toString()
			"ledger_seal" -> at<Ledger>(a[0]).seal().let { "ok" }
			"ledger_receipt" -> made(a[0], at<Ledger>(a[1]).receipt())
			"draft_write" -> at<Draft>(a[0]).write("line").let { "ok" }
			"draft_lines" -> at<Draft>(a[0]).lines().toString()
			"draft_commit" -> at<Draft>(a[0]).commit().toString()
			"tallied" -> tallied(at<Receipt>(a[0])).toString()
			"stored" -> stored(at<Receipt>(a[0])).toString()
			"receipt_count" -> at<Receipt>(a[0]).count().toString()
			"receipt_clone" -> made(a[0], at<Receipt>(a[1]).clone())
			"tally_new" -> made(a[0], Tally.new())
			"tally_bump" -> at<Tally>(a[0]).bump().toString()
			"bump_later" -> {
				val tally = at<Tally>(a[1])
				future(a[0]) { tally.bumpLater().toULong() }
			}
			"local_new" -> {
				check(a[1] == "0") { "Kotlin makes no local on a home" }
				made(a[0], Local.new())
			}
			"local_bump" -> at<Local>(a[0]).bump().toString()
			"local_share" -> made(a[0], at<Local>(a[1]).share())
			"local_later" -> {
				val local = at<Local>(a[1])
				future(a[0]) { local.later().toULong() }
			}
			"meters_new" -> made(a[0], Meters.new())
			"meters_first" -> made(a[0], at<Meters>(a[1]).first())
			"meters_clear" -> at<Meters>(a[0]).clear().let { "ok" }
			"meter_new" -> made(a[0], Meter.default())
			"meter_add" -> at<Meter>(a[0]).add(a[1].toULong()).toString()
			"meter_reset" -> at<Meter>(a[0]).reset().let { "ok" }
			"drop" -> at<AutoCloseable>(a[0]).close().let { "ok" }
			"await" -> results.getValue(a[0].toInt()).toString()
			else -> error("an op Kotlin does not run: $op")
		}
	} catch (e: Throwable) {
		failure(e)
	}

	fun run(): String {
		val out = StringBuilder()
		// The worker: a thread of its own per script, which exits once it ends.
		// A daemon, so a script that throws ends the run rather than leaving it waiting.
		val worker = Executors.newSingleThreadExecutor { task -> Thread(task).apply { isDaemon = true } }
		try {
			for (line in steps) {
				val words = line.split(' ')
				val (op, a) = words[1] to words.drop(2)
				val got = when (words[0]) {
					"0" -> run(op, a)
					else -> worker.submit<String> { run(op, a) }.get()
				}
				out.append(got).append('\n')
			}
		} finally {
			worker.shutdown()
		}
		check(worker.awaitTermination(1, TimeUnit.MINUTES)) { "the worker never exited" }
		return out.toString()
	}
}

fun main(args: Array<String>) {
	for (path in args) {
		File("$path.out").writeText(Script(File(path).readLines()).run())
	}
}
