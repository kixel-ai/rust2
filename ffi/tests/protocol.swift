// Runs a script tests/protocol/model.rs writes for Swift through the api fixture's Swift:
// each step on one of two threads, printing what it returned a line. An object frees as
// its slot lets go of it, and a call that consumes one takes it from its slot whether or
// not its check passes. A future is an async call, awaited on its step's thread through a
// task that prefers that thread's executor, as the step after the one that makes it reads.

import Api
import ApiFFI
import Foundation

/// A thread running what it is handed, and the jobs of the tasks preferring it.
final class Worker: TaskExecutor, @unchecked Sendable {
	private let condition = NSCondition()
	private var jobs: [() -> Void] = []

	init() {
		let thread = Thread { [unowned self] in self.serve() }
		thread.start()
	}

	private func serve() {
		while true {
			condition.lock()
			while jobs.isEmpty {
				condition.wait()
			}
			let job = jobs.removeFirst()
			condition.unlock()
			job()
		}
	}

	func push(_ job: @escaping () -> Void) {
		condition.lock()
		jobs.append(job)
		condition.signal()
		condition.unlock()
	}

	func enqueue(_ job: UnownedJob) {
		push { job.runSynchronously(on: self.asUnownedTaskExecutor()) }
	}
}

/// One slot's handle, of whichever kind it holds, which any thread reaches as its step runs.
final class Slot: @unchecked Sendable {
	let kind: String
	var counter: Counter?
	var ledger: Ledger?
	var draft: Draft?
	var receipt: Receipt?
	var tally: Tally?
	var local: Local?
	var meters: Meters?
	var meter: Meter?
	/// What a future's call returned, which the step awaiting it prints.
	var awaited: String?

	init(_ kind: String) {
		self.kind = kind
	}

	func free() {
		counter = nil
		ledger = nil
		draft = nil
		receipt = nil
		tally = nil
		local = nil
		meters = nil
		meter = nil
		awaited = nil
	}
}

/// The status each check's message names.
let statuses: [(String, String)] = [
	("a handle's value was moved out", "moved"),
	("a handle is borrowed exclusively", "exclusive"),
	("a handle is already borrowed", "borrowed"),
	("a handle is borrowed on another thread", "thread"),
	("a handle a call lent is borrowed exclusively or moved out", "lent"),
	("a handle lent from another outlived it", "outlived"),
	("a value that is not Send is used on another thread", "not_send"),
]

/// What a step that threw prints: its check's status, or `error` for Rust's own `Err`.
func outcome(of error: Swift.Error) -> String {
	guard let check = error as? CheckError else { return "error" }
	for (message, name) in statuses where check.message.hasPrefix(message) {
		return name
	}
	return "unknown \(check)"
}

func caught(_ body: () throws -> String) -> String {
	do {
		return try body()
	} catch {
		return outcome(of: error)
	}
}

/// Run a step that makes a future: await its call, keeping what it returned for the next
/// step.
func later(_ slots: [Slot], _ op: String, _ a: Int, _ b: Int, _ c: Int) async -> String {
	do {
		let value: UInt64
		switch op {
		case "plus_later":
			if c < 0 {
				value = try await slots[b].counter!.plusLater()
			} else {
				value = try await slots[b].counter!.plusLater(other: slots[c].counter!)
			}
		case "bump_later": value = UInt64(try await slots[b].tally!.bumpLater())
		default: value = UInt64(try await slots[b].local!.later())
		}
		slots[a].awaited = String(value)
		return "ok"
	} catch {
		return outcome(of: error)
	}
}

/// Run one step that makes no future on this thread: what it returned. A step that makes a
/// handle makes it before naming its slot, which a failed one never fills.
func run(_ slots: [Slot], _ op: String, _ a: Int, _ b: Int, _ c: Int) -> String {
	caught {
		switch op {
		case "counter_new": slots[a].counter = Counter.new("c", UInt64(b))
		case "counter_get": return String(try slots[a].counter!.get())
		case "counter_add":
			do {
				return String(try slots[a].counter!.add(UInt64(b)))
			} catch let check as CheckError {
				throw check
			} catch {
				return "error"
			}
		case "counter_into_sum": return String(try slots[a].counter.take()!.intoSum(UInt64(b)))
		case "counter_plus":
			return String(b < 0 ? try slots[a].counter!.plus() : try slots[a].counter!.plus(other: slots[b].counter!))
		case "counter_add_to":
			if b < 0 {
				var none: Counter? = nil
				return try slots[a].counter!.addTo(&none) ? "1" : "0"
			}
			return try slots[a].counter!.addTo(&slots[b].counter) ? "1" : "0"
		case "counter_clone":
			let made = try slots[b].counter!.clone()
			slots[a].counter = consume made
		case "ledger_new": slots[a].ledger = Ledger.new()
		case "ledger_edit":
			do {
				let made = try slots[b].ledger!.edit()
				slots[a].draft = consume made
				return "1"
			} catch let check as CheckError {
				throw check
			} catch {
				return "error"
			}
		case "ledger_published": return String(try slots[a].ledger!.published())
		case "ledger_seal": try slots[a].ledger!.seal()
		case "ledger_receipt":
			let made = try slots[b].ledger!.receipt()
			slots[a].receipt = consume made
		case "draft_write": try slots[a].draft!.write("line")
		case "draft_lines": return String(try slots[a].draft!.lines())
		case "draft_commit": return String(try slots[a].draft.take()!.commit())
		case "tallied": return String(try tallied(slots[a].receipt.take()!))
		case "stored": return String(try stored(slots[a].receipt.take()!))
		case "receipt_count": return String(try slots[a].receipt!.count())
		case "receipt_clone":
			let made = try slots[b].receipt!.clone()
			slots[a].receipt = consume made
		case "tally_new": slots[a].tally = Tally.new()
		case "tally_bump": return String(try slots[a].tally!.bump())
		case "local_new": slots[a].local = Local.new()
		case "local_bump": return String(try slots[a].local!.bump())
		case "local_share":
			let made = try slots[b].local!.share()
			slots[a].local = consume made
		case "meters_new": slots[a].meters = Meters.new()
		case "meters_first":
			let made = try slots[b].meters!.first()
			slots[a].meter = consume made
		case "meters_clear": try slots[a].meters!.clear()
		case "meter_new": slots[a].meter = Meter.default()
		case "meter_add": return String(try slots[a].meter!.add(UInt64(b)))
		case "meter_reset": try slots[a].meter!.reset()
		case "free": slots[a].free()
		case "await": return slots[a].awaited!
		case "drain": api_thread_drain()
		default: fatalError("unknown op \(op)")
		}
		return "ok"
	}
}

/// Holds what a step on another thread returned until the main thread reads it.
final class Out: @unchecked Sendable {
	var line = ""
}

@main
struct Protocol {
	static func main() {
		let args = CommandLine.arguments
		guard args.count == 2, let text = try? String(contentsOfFile: args[1], encoding: .utf8) else {
			FileHandle.standardError.write("usage: protocol <script>\n".data(using: .utf8)!)
			exit(2)
		}
		// The first line names each slot's kind, and is empty in a script making none.
		var lines = text.split(separator: "\n", omittingEmptySubsequences: false)[...]
		let slots = lines.removeFirst().split(separator: " ").map { Slot(String($0)) }
		let workers = [Worker(), Worker()]
		for line in lines where !line.isEmpty {
			let tokens = line.split(separator: " ").map(String.init)
			let worker = workers[Int(tokens[0])!]
			let op = tokens[1]
			let args = tokens.dropFirst(2).map { $0 == "-" ? -1 : Int($0)! } + [-1, -1, -1]
			let (a, b, c) = (args[0], args[1], args[2])
			let out = Out()
			let done = DispatchSemaphore(value: 0)
			switch op {
			case "plus_later", "bump_later", "local_later":
				worker.push {
					Task(executorPreference: worker) {
						out.line = await later(slots, op, a, b, c)
						done.signal()
					}
				}
			default:
				worker.push {
					out.line = run(slots, op, a, b, c)
					done.signal()
				}
			}
			done.wait()
			print(out.line)
			fflush(stdout)
		}
	}
}
