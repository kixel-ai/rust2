"""Runs a script tests/protocol/model.rs writes through the api fixture's Python: each step
on the main thread or a worker thread, printing what it returned a line, which the model's
transcript must match line for line. A free lets go of the slot's last reference, which
CPython's counting frees at once; a future is an `async def`, made as it is awaited."""

import asyncio
import concurrent.futures
import gc
import sys
import typing

import api

# A failed check's message, after the function's name, and the status it names; a closed
# object's own error names none, and is as a moved one.
STATUSES = [
	("a handle a call lent is borrowed exclusively or moved out", "lent"),
	("a handle's value was moved out", "moved"),
	("a handle is borrowed exclusively", "exclusive"),
	("a handle is already borrowed", "borrowed"),
	("a handle is borrowed on another thread", "thread"),
	("a handle lent from another outlived it", "outlived"),
	("a value that is not Send is used on another thread", "not_send"),
	(" is closed", "moved"),
]


def status(e: Exception) -> str:
	message = str(e)
	for text, name in STATUSES:
		if text in message:
			return name
	raise e


slots: list[typing.Any] = []


def at(token: str) -> typing.Any:
	return None if token == "-" else slots[int(token)]


def awaited(coro: typing.Any) -> typing.Any:
	# A loop of its own, closed after, so no later free on another thread wakes a loop
	# that waits for nothing: the thread drains at its next check, as the model's does.
	loop = asyncio.new_event_loop()
	try:
		return loop.run_until_complete(coro)
	finally:
		loop.close()


async def drain() -> None:
	api._drains().drain()


def run(op: str, args: list[str]) -> str:
	a = int(args[0]) if args and args[0] != "-" else -1
	match op:
		case "counter_new":
			slots[a] = api.Counter.new("c", int(args[1]))
		case "counter_get":
			return str(at(args[0]).get())
		case "counter_add":
			return str(at(args[0]).add(int(args[1])))
		case "counter_into_sum":
			return str(at(args[0]).into_sum(int(args[1])))
		case "counter_plus":
			return str(at(args[0]).plus(at(args[1])))
		case "counter_add_to":
			return str(int(at(args[0]).add_to(at(args[1]))))
		case "counter_clone":
			slots[a] = at(args[1]).clone()
		case "plus_later":
			slots[a] = at(args[1]).plus_later(at(args[2]))
		case "ledger_new":
			slots[a] = api.Ledger.new()
		case "ledger_edit":
			slots[a] = at(args[1]).edit()
			return "1"
		case "ledger_published":
			return str(at(args[0]).published())
		case "ledger_seal":
			at(args[0]).seal()
		case "ledger_receipt":
			slots[a] = at(args[1]).receipt()
		case "draft_write":
			at(args[0]).write("line")
		case "draft_lines":
			return str(at(args[0]).lines())
		case "draft_commit":
			return str(at(args[0]).commit())
		case "receipt_count":
			return str(at(args[0]).count())
		case "receipt_clone":
			slots[a] = at(args[1]).clone()
		case "tallied":
			return str(api.tallied(at(args[0])))
		case "stored":
			return str(api.stored(at(args[0])))
		case "tally_new":
			slots[a] = api.Tally.new()
		case "tally_bump":
			return str(at(args[0]).bump())
		case "bump_later":
			slots[a] = at(args[1]).bump_later()
		case "local_new":
			slots[a] = api.Local.new()
		case "local_bump":
			return str(at(args[0]).bump())
		case "local_share":
			slots[a] = at(args[1]).share()
		case "local_later":
			slots[a] = at(args[1]).later()
		case "meters_new":
			slots[a] = api.Meters.new()
		case "meters_first":
			slots[a] = at(args[1]).first()
		case "meters_clear":
			at(args[0]).clear()
		case "meter_new":
			slots[a] = api.Meter.default()
		case "meter_add":
			return str(at(args[0]).add(int(args[1])))
		case "meter_reset":
			at(args[0]).reset()
		case "free":
			value = slots[a]
			slots[a] = None
			# A coroutine never awaited made nothing in Rust: closing it says so to Python.
			if asyncio.iscoroutine(value):
				value.close()
			del value
		case "await":
			value = slots[a]
			slots[a] = None
			return str(awaited(value))
		case "drain":
			awaited(drain())
		case _:
			raise ValueError(f"unknown op {op}")
	return "ok"


def step(op: str, args: list[str]) -> str:
	try:
		return run(op, args)
	except api.ErrorException:
		out = "error"
	except (ValueError, RuntimeError) as e:
		out = status(e)
	# What raised keeps its task and coroutine in a cycle, which holds the future's
	# arguments: freed now, once the exception is gone, as the model frees them.
	gc.collect()
	return out


def main() -> None:
	with open(sys.argv[1]) as script:
		lines = script.read().splitlines()
	slots.extend(None for _ in lines[0].split())
	out = []
	# One worker thread, kept for the whole script: a step's exception reaches main through
	# its result, and leaving the block joins the worker however main leaves it.
	with concurrent.futures.ThreadPoolExecutor(max_workers=1) as worker:
		for line in lines[1:]:
			thread_id, op, *args = line.split()
			if thread_id == "0":
				out.append(step(op, args))
			else:
				out.append(worker.submit(step, op, args).result())
	sys.stdout.write("".join(f"{line}\n" for line in out))


main()
