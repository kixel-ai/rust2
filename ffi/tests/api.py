# The api fixture's Python test: the script fixtures/api/examples/transcript.rs runs
# natively, through the generated Python. tests/fixtures.rs compares the transcripts.
#
# With an argument, it instead checks the futures' semantics, what frees what nobody
# closed, the values only Python's types bound, or how Python calls into the extension,
# or breaks a contract, which must fail.

import asyncio
import datetime
import gc
import os
import sys
import threading

import api

# The transcript's lines end and encode as Rust's do, which Windows' text mode and code
# page would turn into CRLF and the locale's encoding.
sys.stdout.reconfigure(encoding="utf-8", newline="\n")


class PrintLog(api.Log):
	"""The Log callback: keeps even levels."""

	def log(self, level, line):
		print(f"log {level} {line}")
		return level % 2 == 0

	# Rust closes it once it drops the implementation.
	def close(self):
		print("freed")


def bound(b):
	match b.kind:
		case api.Bound.INCLUDED:
			return f"Included({b.value})"
		case api.Bound.EXCLUDED:
			return f"Excluded({b.value})"
		case api.Bound.UNBOUNDED:
			return "Unbounded"


def flag(value):
	"""A bool as Rust prints it."""
	return "true" if value else "false"


def data(view):
	"""The bytes a memoryview lends, releasing it."""
	with view:
		return bytes(view)


def text(view):
	return data(view).decode()


def debugs(values):
	return "[" + ", ".join(repr(v) for v in values) + "]"


def quoted(values):
	return "[" + ", ".join(f'"{v}"' for v in values) + "]"


def escaped(path):
	"""A path's bytes as Rust's `escape_ascii` prints them."""
	out = ""
	for b in os.fsencode(path):
		match b:
			case 0x09:
				out += "\\t"
			case 0x0D:
				out += "\\r"
			case 0x0A:
				out += "\\n"
			case 0x5C | 0x27 | 0x22:
				out += "\\" + chr(b)
			case b if 0x20 <= b < 0x7F:
				out += chr(b)
			case b:
				out += f"\\x{b:02x}"
	return out


def check(ok, what):
	if not ok:
		print(f"failed: {what}", file=sys.stderr)
		sys.exit(1)


def step(loop):
	"""Run the loop until what is ready has run."""
	loop.run_until_complete(asyncio.sleep(0))


def until(loop, done):
	"""Run the loop until `done()`."""

	async def wait():
		while not done():
			await asyncio.sleep(0)

	loop.run_until_complete(asyncio.wait_for(wait(), 5))


def script():
	print(f"limit {api.LIMIT}")

	p = api.Point(1, 2).offset(2, 3)
	print(f"offset {p.x} {p.y}")
	print(f"debug {p!r}")
	print(f"eq {flag(p == api.Point(3, 5))}")
	d = api.Point.default()
	print(f"default {d.x} {d.y}")

	c = api.Counter.new("c", 5)
	print(f"get {c.get()}")
	print(f"name {c.name()}")
	print(f"add ok {c.add(10)}")
	try:
		c.add(2000)
	except api.ErrorException as e:
		print(f"add err {e} {e.error!r}")
	try:
		c.sub(20)
	except api.ErrorException as e:
		print(f"sub err {e} {e.error!r}")
	c.rename("d")
	print(f"name {c.name()}")
	print(f"is d {flag(c.is_('d'))}")
	copy = c.clone()
	copy.add(1)
	print(f"clone {copy.get()} original {c.get()}")
	print(f"debug {c!r}")
	label = c.into_label(p)
	print(f"label {label!r}")
	print(f"note {flag(api.has_text(api.Note('')))} {flag(api.has_text(api.Note(None)))}")

	below, above = api.Counter.new("s", 7).split(5)
	print(f"split {below.name()} {below.get()} {above.name()} {above.get()}")
	(only,) = api.only(api.Counter.new("o", 3))
	print(f"only {only.name()} {only.get()}")
	w8, w16, w32, w64 = api.widths(98305)
	print(f"widths {w8} {w16} {w32} {w64}")
	for b in [2, 0]:
		try:
			q, r = api.divide(7, b)
			print(f"divide {q} {r}")
		except api.ErrorException as e:
			print(f"divide err {e}")
	for n in [98305, 0]:
		spread = api.spread(n)
		print("spread none" if spread is None else f"spread {spread[0]} {spread[1]} {spread[2]} {spread[3]}")
	for needle in [b"lo", b"x"]:
		found = api.find(b"hello", needle)
		print("find none" if found is None else f"find {found[0]} {found[1]}")
	stat_rows = api.rows([api.Point(1, 2), api.Point(3, 3)])
	print("rows " + ", ".join(f"{name} {shape!r} {p.x} {p.y}" for name, shape, p in stat_rows))
	# Each handle in a tuple moves in, as one taken by value does.
	board = api.Board.new([("low", api.Counter.new("l", 1)), ("high", api.Counter.new("h", 9))])
	ranked = board.ranked()
	print("ranked " + ", ".join(f"{name} {counter.name()} {counter.get()}" for name, counter in ranked))
	# A map crosses as its pairs and a set as its items, which Rust collects in order, each
	# key once, the last kept.
	atlas = api.Atlas([("b", api.Point(1, 1)), ("ab", api.Point(2, 3)), ("b", api.Point(4, 4))], [3, 1, 3])
	atlas = atlas.with_("c", api.Point(5, 6))
	atlas_points = ", ".join(f"{name} {p.x} {p.y}" for name, p in atlas.points)
	print(f"atlas {atlas_points}; ids {' '.join(str(i) for i in atlas.ids)}")
	# Another crate's error crosses as its message.
	for port_text in ["80", "x"]:
		try:
			print(f"port {api.port(port_text)}")
		except api.StringException as e:
			print(f"port err {e.error}")
	span = api.Span((2, 10), ("s", api.Shape.SQUARE))
	lower_half, upper_half = api.halves(span)
	print(f"halves {lower_half[0]} {lower_half[1]} {upper_half[0]} {upper_half[1]} {span!r}")
	swapped_n, swapped_text = api.swapped(("t", 4))
	print(f"swapped {swapped_n} {swapped_text}")
	for value in [None, (None,), (3,)]:
		doubled = api.doubled(value)
		if doubled is None:
			print("doubled None")
		else:
			print(f"doubled Some(({'None' if doubled[0] is None else f'Some({doubled[0]})'},))")
	# A borrowed or boxed tuple of one is a tuple, not its element.
	lent_one = list(data(api.lent_back((bytes([1, 2]),))[0]))
	boxed_one = list(data(api.boxed_tuple(bytes([3]))[0]))
	print(f"lent back {lent_one} boxed {boxed_one}")
	# A borrowed string and an owned one share a tuple struct.
	print(
		f'title Some(("{api.title(True)[0]}",)) {"None" if api.title(False) is None else "Some"} '
		f'named Some(("{api.named("x")[0]}",))'
	)

	# Each lent counter is a new handle to a clone.
	registry = api.Registry.new(api.Counter.new("first", 1))
	print(f"last {flag(registry.last() is not None)}")
	registry.push(api.Counter.new("last", 2))
	first = registry.first_get
	first.add(10)
	print(f"first {registry.first_get.get()} lent {first.get()}")
	last = registry.last()
	last.add(10)
	print(f"last {registry.last().get()} lent {last.get()}")
	# A closure lent for the call edits the first counter through a handle.
	by = 5

	def edit(counter):
		counter.add(by)
		return counter.edited() and counter.get() == 6

	print(f"edit {flag(registry.edit(edit))} {registry.first_get.get()}")
	# A setter moves an object in, as assignment does, closing it.
	registry.first_set(api.Counter.new("second", 3))
	print(f"set first {registry.first_get.get()} {registry.first_get.name()}")
	print(f"walk {api.walk(3, lambda cursor: cursor.at() * 2)}")
	print(f"each {api.each(2, lambda at: print(f'visit {at}'))}")
	print(f"offset {api.offset(lambda: 7, 3)}")
	print(f"shift {api.shift(lambda: 7, 4)}")

	name = api.Name.owned("a")
	joined = name.join(api.Name.owned("b"))
	print(f"joined {joined.as_str()} {joined!r} {flag(joined == name)}")
	joined.rename("c")
	print(f"renamed {joined.as_str()}")
	long = api.longer(joined, api.Name.owned("dd"))
	print(f"longer {long.as_str()}")
	print(f"unnamed [{api.unnamed().as_str()}]")
	lit = api.Name.borrowed("lit")
	shares = [str(lit.shares(api.Name.borrowed(text))).lower() for text in ("lit", "other")]
	print(f"borrowed {lit.as_str()} {' '.join(shares)}")

	print(f"total {api.total(range(2, 5))}")
	print(f"reach {api.reach(2, 5)}")
	scaled = [api.scaled(2**128 - 1, 1), api.scaled(2**64 + 1, 2**63), api.scaled(2**127, 2)]
	print("scaled " + " ".join("none" if v is None else f"{v:032x}" for v in scaled))
	batch, batch2 = api.Batch.new(), api.pair()
	accepted = sum(batch.push(v) for v in range(1, 6))
	paired = sum(batch2.push(v) for v in range(1, 4))
	emptied = api.drain_4(batch)
	print(f"batch {batch.capacity()} {accepted} {emptied} {api.drain_4(batch)} {batch2.capacity()} {paired} {api.drain_2(batch2)}")
	wide = api.widen(range(2, 5), 1)
	print(f"widen {wide.start} {wide.stop}")
	inc, exc, unb = api.Bound.INCLUDED, api.Bound.EXCLUDED, api.Bound.UNBOUNDED
	counts = [
		api.count(api.BoundsU64(api.BoundU64(start, s), api.BoundU64(end, e)))
		for (start, s), (end, e) in [
			((inc, 2), (exc, 5)),
			((inc, 2), (inc, 5)),
			((inc, 3), (unb, 0)),
			((unb, 0), (unb, 0)),
			((exc, 2), (inc, 4)),
		]
	]
	print("count " + " ".join(str(n) for n in counts))
	print(f"end {bound(api.end(3))} {bound(api.end(0))}")
	ends_start, ends_end = api.ends(3)
	print(f"ends {bound(ends_start)} {bound(ends_end)}")
	for n in [2, 5]:
		try:
			print(f"below {api.below(n, 3)}")
		except api.BoundsU64Exception as e:
			print(f"below err {bound(e.error.start)} {bound(e.error.end)}")
	window = api.Window(range(2, 8), api.end(5))
	print(f"window {flag(window.contains(4))} {flag(window.contains(6))} {window!r}")

	print(api.greet("x", api.Shape.SQUARE))
	print(f"len {api.len(b'abc')}")
	binary = bytes([0xFF, 0, ord("a")])
	print(f"hex {api.hex(binary)}")
	print(f"timed hex {api.timed_hex(binary)}")
	print(f"sample hex {api.sample_hex(binary)}")
	print(f"queued hex {api.queued_hex(binary)}")
	print(f"tick {api.tick_name(api.Label('tick', None, None))}")
	print(f"rooted {api.rooted_name(api.Label('rooted', None, None))}")
	print(f"stored {api.stored_text('stored')}")
	print(f"layer {api.layer_name(api.Label('layer', None, None))}")
	for event in [api.Event.Moved(p), api.Event.Named("n"), api.Event.Done()]:
		print(api.describe(event))

	# A `Copy` handle passed by value is copied, so it stays usable.
	stamp = api.Stamp.from_millis(2500)
	print(f"secs {stamp.secs()}")
	print(f"later {flag(api.Stamp.from_millis(3000).later(stamp))}")
	print(f"secs {stamp.secs()}")
	message = api.Message.new(stamp, b"body")
	print(f"payload {text(message.payload_get)}")
	print(f"stamp {message.stamp_get!r}")
	print(f"sent {flag(message.sent())}")
	# Setters assign in place: the bytes are copied in, and a `Copy` handle copied.
	reset = api.Stamp.from_millis(4000)
	message.stamp_set(reset)
	message.payload_set(b"edited")
	print(f"payload {text(message.payload_get)}")
	print(f"stamp {message.stamp_get!r}")
	print(f"secs {reset.secs()}")
	message.stamp_set(None)
	print(f"stamp {flag(message.stamp_get is not None)}")

	print(f"kept {api.emit(PrintLog(), bytes([1, 2, 3, 4]))}")
	logger = api.Logger.new(PrintLog())
	print(f"write {flag(logger.write('hi'))}")
	del logger
	print("dropped")
	# Rust calls it on another thread, before the call returns.
	threshold = 2

	def keep(level, line):
		print(f"filter {level} {line}")
		return level > threshold

	print(f"filtered {api.filter(bytes([1, 2, 3, 4]), keep)}")

	loop = asyncio.new_event_loop()
	channel = api.Channel.new()
	wakes = 0

	def wake():
		nonlocal wakes
		wakes += 1

	waker = api.Waker(wake, loop)
	# On a loop the script steps, so it sees the receive pend, and its wake resume it.
	got = None

	async def recv():
		nonlocal got
		got = await channel.recv()

	task = loop.create_task(recv())
	step(loop)
	print(f"recv pending {flag(got is None)}")
	channel.send(b"hi")
	check(got is None, "recv never resumes inside the wake")
	loop.run_until_complete(task)
	wakes += 1
	print(f"woken {wakes}")
	print(f"recv {text(got)}")

	async def block():
		channel.send(b"")
		try:
			with await channel.recv_nonempty() as payload:
				print(f"nonempty {payload.nbytes}")
		except api.ErrorException as e:
			print(f"nonempty err {e}")
		filled = await api.fill_4(api.Batch.new(), 9) + await api.fill_2(api.pair(), 1)
		print("fill " + " ".join(map(str, filled)))
		# The send happens at the call: the future is dropped unawaited.
		channel.send_later(b"eager")
		print(f"recv {text(await channel.recv())}")
		channel.send(b"sized")
		payload, size = await channel.recv_sized()
		print(f"sized {text(payload)} {size}")

	loop.run_until_complete(block())
	# The waker moves into Rust, which keeps it until the send wakes it.
	channel.notify(waker)
	channel.send(b"rung")
	until(loop, lambda: wakes == 2)
	print(f"woken {wakes}")
	loop.close()

	nodes = api.Node.new(1, None)
	nodes.push(2)
	print(f"node {nodes.sum()}")

	tally = api.Tally.new()
	tally.bump()
	print(f"tally {tally.bump()} {flag(tally == tally)}")
	print(f"both {api.bump_both(tally, tally)}")
	print(f"peek {api.peek(tally, lambda lent: lent.bump())}")

	area = api.grow(api.Area.new(api.Size(2, 3)), 1)
	size = area.size()
	print(f"area {size.width} {size.height} {area.pixels()}")

	# Generic types, each at the instances it infers, named by their arguments.
	pair = api.PairU32(2, 5)
	swapped = pair.swap()
	print(f"pair {swapped.left} {swapped.right} sum {pair.sum()} {pair!r} {flag(pair == swapped)}")
	print(f"even {flag(pair.even())} {flag(api.PairU32(7, 7).even())}")
	# An instance renamed alone, beside another of `Pair` that keeps its name.
	sized = api.tag(api.Size(1, 2), 3)
	flipped = sized.swap()
	print(f"sized {flipped.left} {flipped.right.width} {flipped!r}")
	beside = api.PairU32(258, 9).beside_size(sized.left)
	print(f"beside {beside.left} {beside!r}")
	# At its default, named as the type itself.
	stack = api.Stack.new()
	for i, n in enumerate(["a", "b"]):
		stack.push(api.Counter.new(n, i + 1))
	top = stack.pop().get()
	print(f"stack {stack.depth()} Some({top}) {flag(stack.pop() is not None)} {flag(stack.pop() is None)}")
	# Generic functions and methods, at each implementation of the trait bounding them, by
	# reference, which leaves Python its object.
	from_point = api.Counter.seeded_point("p", api.Point(-3, 4))
	from_size = api.Counter.seeded_size("s", api.Size(5, 6))
	from_counter = api.Counter.seeded_counter("c", from_point)
	from_polar = api.Counter.seeded_polar_point("q", api.PolarPoint(1, 2))
	print(
		f"seeded {from_point.get()} {from_size.get()} {from_counter.get()} {from_polar.get()} "
		f"past {stack.depth_past_point(api.Point(1, 2))} seeds {api.seeds_point_size(api.Size(2, 3))}"
	)

	# A trait implemented for `Point` and `&Point` apart: one instance.
	print(f"weigh {api.weigh_point(api.Point(-1, 2))}")
	# A function named as the type it returns, and one named as a type it does not.
	polar = api.polar_point(2)
	check(polar is not None, "a point off the origin")
	print(f"polar {polar.distance} {flag(api.polar_point(0) is None)} area {api.area(api.Size(2, 3))}")
	# Two types whose paths join alike, the second kept apart where its path splits.
	print(f"wind {api.wind(api.PolarTurn(2), api.Polar_Turn(3))}")
	# Instances that name each other, and one made only by an impl of it.
	cross = api.cross(api.LeftRightU8(api.RightU8(1)), api.RightLeftU8(api.LeftU8(2)))
	print(f"cross {cross} outer {api.OuterInnerU8.new(4).get()}")
	# A bound that outlives the call takes a value, and an instance only a constant names.
	print(f"kept {api.kept_point(api.Point(1, -2))} boxed {api.BOXED.value}")
	print(f"keep {api.keep_point(api.Point(1, -2))}")
	five = api.Point(5, 2)
	print(f"kept or {api.kept_or_point(five)} {api.kept_or_point(None)} keeper {api.keeper_point(five)}")
	# Inferred instances taking a lent closure and a foreign implementation, and one only
	# its impl of a trait names.
	sides = api.PairU32(2, 5)
	print(f"apply {api.apply_u32(sides, lambda side: side * 3)}")
	print(f"report {api.report_u32(sides, PrintLog())}")
	print(f"wrapped {api.kept_wrap_u32(api.WrapU32(4))}")
	print(f"keeps {api.keeps(PrintLog()).value}")

	# Its Drop runs when its last reference goes, not before.
	token = api.Token.new(7)
	print(f"token {token.id_get} dropped {api.dropped()}")
	del token
	print(f"dropped {api.dropped()}")

	# A field and the method of its name, each its own member.
	retry = api.Retry.new(5)
	limit = retry.limit_get
	print(f"retry {0 if limit is None else 1} {limit or 0} {retry.limit()} {retry.delay()}")
	# The renamed field's setter, named as its getter.
	retry.limit_set(None)
	print(f"retry {retry.limit()}")
	print(f"slot {api.SlotU8.new(200).value()}")

	# Foreign types: a URL parsed from text, socket addresses, OS paths, and `Vec`s.
	base = api.Url.parse("https://example.com/a/b")
	host = api.host(base)
	host = "None" if host is None else f'Some("{host}")'
	print(f"url {base} host {host}")
	url = api.resolve(base.clone(), "c?d")
	print(f"resolve {url} {flag(url == base)}")
	try:
		api.Url.parse("no scheme")
	except api.StringException as e:
		print(f"parse err {e.error}")
	v4 = ("127.0.0.1", 8080)
	v6 = ("::1", 443, 0, 3)
	print(f"ports {api.ports([v4, v6])}")
	peer = api.Peer(v6, ["a", "b"]).next("c")
	print(f"peer {peer!r} {quoted(peer.aliases())}")
	file = api.file("dir", "f\\'\"\t")
	print(f"file {escaped(file)}")
	print("parents " + " ".join("none" if p is None else escaped(p) for p in api.parents([file, "/"])))
	words = api.words(" one two  three ")
	print(f"words {len(words)} {quoted(words)}")
	print(f"sentence {api.sentence(words)} {api.sentence(None)}")
	print(f"sentence {api.sentence([])}|")
	print(f"either {api.either('abc')} {api.either(None)}")
	print(f"prefixed {bytes(api.prefixed(b'cd', b'ab')).decode()} {bytes(api.prefixed(b'cd', None)).decode()}")
	print(f"coordinates {api.coordinates(api.Point(2, 3))} {api.coordinates(None)}")
	two, three = api.Counter.new("two", 2), api.Counter.new("three", 3)
	print(f"plus {two.plus(three)} {two.plus(None)}")
	print(f"add_to {str(two.add_to(three)).lower()} {str(two.add_to(None)).lower()} {three.get()}")
	encoded = api.Mut(memoryview(b"0,"))
	api.put_number(1, encoded)
	api.put_number(23, encoded)
	print(f"put_number {bytes(encoded.value).decode()}")
	entry = api.Mut((memoryview(b"ab"), 1))
	stamps = api.push_counted(entry, ord("c"))
	print(f"push_counted {stamps} {bytes(entry.value[0]).decode()} {entry.value[1]}")
	digits = api.Mut(memoryview(b"7x9"))
	first_digit = api.digit(digits)
	try:
		api.digit(digits)
		skipped = False
	except api.ErrorException:
		skipped = True
	print(f"digit {first_digit} {str(skipped).lower()} {api.digit(digits)} {len(digits.value)}")
	nudged = api.Mut(api.Point(1, 2))
	api.nudge(nudged, 3)
	print(f"nudge {nudged.value.x} {nudged.value.y}")
	config = api.secure("h3")
	print(f"secure {api.alpn_lent(config)} {api.alpn(config.clone())}")
	meters = api.Meters.new()
	first = meters.first()
	first.add(2)
	print(f"lent {first.add(3)} {meters.first().count()}")
	meters.clear()
	print(f"cleared {meters.first().count()}")
	tallies = api.Tallies.new()
	tallies.first().bump()
	print(f"tallied {tallies.first().bump()}")
	print(f"chunks {[list(data(chunk)) for chunk in api.chunks(b'abcde', 2)]}")
	counters = api.counters(["x", "y"])
	print("counters " + ", ".join(f"{c.name()} {c.get()}" for c in counters))
	# Each counter moves in, so Python closes it.
	print(f"worth {api.worth([counters[0]], [counters[1]])} {api.worth(None, [])}")
	for counter in counters:
		try:
			counter.get()
			check(False, "a counter moved in is closed")
		except ValueError:
			pass
	held = [api.held_count((api.Counter.new("h", 4),)), api.held_count(None)]
	print("held " + " ".join("None" if n is None else f"Some({n})" for n in held))
	print(f'boxed Some("{api.boxed_name(api.boxed_counter("b"))}") {api.boxed_name(None)} {str(api.boxed_counter(None) is None).lower()}')
	print(f"less {api.less(5, 2)}")
	print(f"sums {api.sums([[1, 2], [], [3]])}")
	summed = api.summed([api.Point(1, 2), api.Point(-4, 6)])
	none_summed = api.summed([])
	print(f"summed {summed.x} {summed.y} {none_summed.x} {none_summed.y}")
	print(f"longest {api.longest(['a', 'abc'])} {api.longest([])}")
	print(f'first of Some("{api.first_of(["a", "b"])}") {api.first_of([]) or "None"}')
	limits = api.byte_limits(b'a', b'bc')
	waits = api.LimitsDuration(datetime.timedelta(milliseconds=5), datetime.timedelta(seconds=2))
	breadth = api.breadth(waits) // datetime.timedelta(milliseconds=1)
	print(f"limits {bytes(limits.low).decode()} {bytes(limits.high).decode()} breadth {breadth}")
	joined = bytes(api.joined([b'ab', b'', b'c'])).decode()
	print(f"joined {joined} {len(api.joined([]))} read {api.read_len(b'hello')}")
	doc = api.Json.parse('{"b": [1, 2.5, null], "a": "é"}')
	print(f"json {api.encode(doc)}")
	print(f"timed text {api.timed_text(doc)}")
	tagged = api.tagged(doc, "t")
	print(f"tagged {tagged.to_text()}")
	# A JSON object's entries cross as pairs of a key and its document, each moved in.
	entries = api.entries([("b", api.Json.parse("[1]")), ("a", api.Json.parse("2"))], "t")
	print("entries " + ", ".join(f"{key} {value.to_text()}" for key, value in entries))
	try:
		api.Json.parse('{"a":')
	except api.StringException as e:
		print(f"json parse err {e.error}")
	site = api.Site.new(base)
	print(f"site {site.home_get} {site.home_get}")

	# Slices, passed in and returned as clones, and the socket address of each family.
	print(f"shapes {debugs(api.Shape.ALL)} greetings {quoted(api.GREETINGS)}")
	trail = api.Trail.new([api.Point(1, 2), api.Point(3, 4)], ["a", "b"])
	print(f"trail {debugs(trail.points())} {debugs(trail.labels())}")
	# Returned iterators, each collected at the call: drained labels are gone after.
	print(f"iterated {debugs(trail.iter())} {quoted(trail.names())} {list(data(trail.bytes()))}")
	print(f"drained {debugs(trail.drain())} then {debugs(trail.labels())}")
	numbered = api.numbered(["p", "q"])
	print("numbered " + ", ".join(f"{c.name()} {c.get()}" for c in numbered))
	next4, next6 = api.nexts(v4, v6)
	print(f"nexts {next4[0]} {next4[1]} {next6[1]} {next6[3]}")


def futures():
	"""The futures' semantics beyond the transcript, each checked by an assertion."""
	channel = api.Channel.new()

	# The send lands from another thread while the receive waits.
	async def threaded():
		sender = threading.Thread(target=lambda: channel.send(b"blocked"))
		sender.start()
		payload = await asyncio.wait_for(channel.recv(), 5)
		sender.join()
		check(text(payload) == "blocked", "recv returns the payload another thread sent")

	asyncio.run(threaded())

	loop = asyncio.new_event_loop()

	# Resumed on its loop, never inside the wake.
	got = None

	async def recv():
		nonlocal got
		got = text(await channel.recv())

	task = loop.create_task(recv())
	step(loop)
	check(got is None, "recv waits for a send")
	channel.send(b"then")
	check(got is None, "recv never polls inside the wake")
	loop.run_until_complete(asyncio.wait_for(task, 5))
	check(got == "then", "recv resumes with the payload")

	# A wake that lands while the future polls, before it returns pending, is kept: the
	# future polls again rather than waiting for a wake that already happened.
	check(loop.run_until_complete(asyncio.wait_for(api.yielded(7), 5)) == 7, "a wake during the poll polls again")
	check(loop.run_until_complete(api.corner_later()) == (1, 1), "a future's borrowed tuple returns whole")

	# Cancelled between its wake and its poll, a receive loses nothing: the payload stays
	# for the next.
	called = False

	async def cancelled():
		nonlocal called
		await channel.recv()
		called = True

	task = loop.create_task(cancelled())
	step(loop)
	channel.send(b"kept")
	task.cancel()
	until(loop, task.done)
	check(not called and task.cancelled(), "a cancelled receive never resumes")
	payload = loop.run_until_complete(asyncio.wait_for(channel.recv(), 5))
	check(text(payload) == "kept", "a cancelled receive loses nothing")

	# An object whose last reference goes while a call awaits on it is freed once the
	# call ends.
	other = api.Channel.new()
	sender = other.clone()
	task = loop.create_task(other.recv())
	step(loop)
	del other
	sender.send(b"held")
	got = loop.run_until_complete(asyncio.wait_for(task, 5))
	check(text(got) == "held", "the call awaiting on an object holds it")

	# An awaitable from Rust is awaited once, as a coroutine is.
	later = channel.send_later(b"once")

	async def wait(awaitable):
		return await awaitable

	loop.run_until_complete(asyncio.wait_for(wait(later), 5))
	try:
		loop.run_until_complete(wait(later))
		check(False, "a second await raises")
	except RuntimeError:
		pass
	check(text(loop.run_until_complete(asyncio.wait_for(channel.recv(), 5))) == "once", "the send landed once")

	# Python run while a closing future is freed, as by its drop's callback, finds it ended.
	class Polls(api.Log):
		def log(self, level, line):
			return True

		def close(self):
			try:
				later.poll(api._n.Queue(loop))
				check(False, "a future polled while it is freed raises")
			except ValueError:
				pass

	later = api.log_later(Polls())
	later.close()

	# A waker woken from another thread calls back on its loop, once for wakes that land
	# before the loop gets to them.
	wakes = []
	waker = api.Waker(lambda: wakes.append(threading.get_ident()), loop)
	channel.notify(waker)
	sender = threading.Thread(target=lambda: channel.send(b"rung"))
	sender.start()
	sender.join()
	until(loop, lambda: wakes)
	check(wakes == [threading.get_ident()], "a waker calls back on its loop's thread")
	loop.run_until_complete(asyncio.wait_for(on_loop(), 5))
	loop.close()
	print("futures ok")


async def on_loop():
	"""Wakes during a call on the running loop's own thread, which write no byte to its
	socket and dispatch once the call returns, beside wakes from other threads, which do."""
	channel = api.Channel.new()
	got = []

	async def recv():
		got.append(text(await channel.recv()))

	async def parked():
		task = asyncio.ensure_future(recv())
		await asyncio.sleep(0)
		return task

	def signaled():
		return wakeup.queue.signaled()

	async def settle():
		for _ in range(4):
			await asyncio.sleep(0)

	task = await parked()
	wakeup = api._wakeups[asyncio.get_running_loop()]
	channel.send(b"near")
	check(not got, "a wake on the loop's thread never resumes inside the call")
	check(not signaled(), "a wake on the loop's thread writes no byte")
	await task
	check(got == ["near"], "a wake on the loop's thread resumes the receive")

	task = await parked()
	sender = threading.Thread(target=lambda: channel.send(b"far"))
	sender.start()
	sender.join()
	check(signaled(), "a wake from another thread writes a byte")
	await task
	check(got == ["near", "far"], "a wake from another thread resumes the receive")

	# Before the first await, the call's poll with no waker leaves nothing to wake.
	early = channel.recv()
	channel.send(b"early")
	check(text(await early) == "early", "a send before the first await is received")

	# Cancelled after its wake, or before it, in the same step, a receive never resumes and
	# leaves the payload.
	for send_first in (True, False):
		task = await parked()
		if send_first:
			channel.send(b"kept")
			task.cancel()
		else:
			task.cancel()
			channel.send(b"kept")
		await settle()
		check(task.cancelled() and len(got) == 2, "a cancelled receive never resumes")
		check(text(await channel.recv()) == "kept", "a cancelled receive loses nothing")
	check(not signaled(), "wakes on the loop's thread wrote no byte")

	# Wakes that land before the loop gets to them call the waker once.
	wakes = []
	waker = api.Waker(lambda: wakes.append(1))
	for _ in range(2):
		channel.notify(waker)
		channel.send(b"rung")
	check(not wakes, "a waker is never called inside the call")
	check(not signaled(), "a waker's wakes on the loop's thread, with nothing parked, write no byte")
	await settle()
	check(wakes == [1], "wakes on the loop's thread call the waker once")
	await channel.recv()

	# Many receives, each woken once from the loop's thread or another, while both wake.
	channels = [api.Channel.new() for _ in range(64)]
	for _ in range(20):
		tasks = [asyncio.ensure_future(c.recv()) for c in channels]
		await asyncio.sleep(0)
		sender = threading.Thread(target=lambda: [c.send(b"far") for c in channels[1::2]])
		sender.start()
		for c in channels[::2]:
			c.send(b"near")
		payloads = await asyncio.gather(*tasks)
		sender.join()
		expected = ["near", "far"] * (len(channels) // 2)
		check([text(p) for p in payloads] == expected, "every receive resumes once with its payload")


def owned():
	"""What frees an object or bytes: its last reference going, or the cycle collector.
	Rust's bytes cross lent, until they and every view of them are released."""
	for n in range(1023, 1026):
		payload = bytes(i % 256 for i in range(n))
		packet = api.next_packet(api.Packet(1, payload))
		check(packet.seq == 2 and data(packet.payload) == payload, f"a record's {n} bytes cross both ways")
	freed = api.dropped()
	for n in range(1023, 1026):
		view = api.token_bytes(n, n)
		check(api.dropped() == freed, f"{n} bytes are lent, not copied")
		check(bytes(view) == bytes([n % 256]) * n, f"{n} bytes read back")
		view.release()
		freed += 1
		check(api.dropped() == freed, f"releasing {n} bytes frees them")
		view.release()
		check(api.dropped() == freed, f"releasing {n} bytes again frees nothing")
		try:
			bytes(view)
			check(False, f"released {n} bytes refuse a read")
		except ValueError:
			pass

	before = api.dropped()
	api.Token.new(3)
	api.token_bytes(4, 1025)
	check(api.dropped() == before + 2, "the last reference going frees at once")

	# A view kept past what it was read from keeps Rust's buffer.
	part = api.token_bytes(7, 1025)[1:]
	packet = api.next_packet(api.Packet(1, bytes(range(256)) * 5))
	payload = packet.payload
	del packet
	check(api.dropped() == before + 2, "a view keeps Rust's buffer")
	check(bytes(part) == bytes([7]) * 1024, "a slice outlives the bytes it was cut from")
	check(bytes(payload) == bytes(range(256)) * 5, "bytes outlive their record")
	del part
	check(api.dropped() == before + 3, "the last view going frees Rust's buffer")

	cycle = [api.Token.new(5), api.token_bytes(6, 1)]
	cycle.append(cycle)
	del cycle
	gc.collect()
	check(api.dropped() == before + 5, "the cycle collector frees what a cycle held")
	print("owned ok")


def values():
	"""The values Python's types bound: integers checked against their Rust type's range,
	durations truncated to the microsecond up to `timedelta.max`, which stands for longer,
	and on Unix, a path whose bytes are not UTF-8, which crosses as `os.fsdecode` decodes it,
	and back as it came. Also a borrowed tuple, returned whole."""

	def raises(error, f, what):
		try:
			f()
			check(False, what)
		except error:
			pass

	raises(OverflowError, lambda: api.widths(2**64), "an int past u64 raises")
	raises(OverflowError, lambda: api.widths(-1), "a negative int for a u64 raises")
	raises(TypeError, lambda: api.title(1), "an int is no bool")
	raises(OverflowError, lambda: api.scaled(2**128, 1), "an int past u128 raises")
	raises(OverflowError, lambda: api.scaled(-1, 1), "a negative int for a u128 raises")

	# A borrowed tuple is one C value, not one out per element.
	check(api.corner(True) == (1, 1), "a borrowed tuple returns whole")
	raises(api.ErrorException, lambda: api.corner(False), "a borrowed tuple's error raises")

	td = datetime.timedelta
	check(api.doubled_ttl(td(seconds=1, microseconds=5)) == td(seconds=2, microseconds=10), "a duration crosses")
	check(api.doubled_ttl(td(days=600_000_000)) == td.max, "a duration past timedelta's is its max")
	check(api.doubled_ttl(td.max) == td.max, "timedelta's max is Duration::MAX")
	raises(ValueError, lambda: api.doubled_ttl(td(seconds=-1)), "a negative duration raises")

	if os.name == "posix":
		path = api.stray()
		check(os.fsencode(path) == b"\xff", "a path that is not UTF-8 keeps its bytes")
		check(api.parents([path, os.fsdecode(b"\xfe/\xff")])[1] == os.fsdecode(b"\xfe"), "and crosses back")
	print("values ok")


def calls():
	"""Calls Python makes straight into the extension, with no Python frame between, as a
	`def` of the same signature takes them: by position or keyword, raising its errors,
	with its docstring and the signature `inspect` reads."""
	import inspect

	def raises(f, message):
		try:
			f()
			check(False, message)
		except TypeError as e:
			check(str(e) == message, f"{e!r} is {message!r}")

	kinds = {
		(api.Counter, "get"): "method_descriptor",
		(api.Counter, "new"): "staticmethod",
		(api.Counter, "__repr__"): "method_descriptor",
		(api.Registry, "first_get"): "getset_descriptor",
		(api.Point, "offset"): "method_descriptor",
		(api.Channel, "send_later"): "method_descriptor",
		# An `async def` stays a coroutine function.
		(api.Gauge, "later"): "function",
	}
	for (cls, name), kind in kinds.items():
		got = type(inspect.getattr_static(cls, name)).__name__
		check(got == kind, f"{cls.__name__}.{name} is a {got}")
	check(type(api.has_text).__name__ == "builtin_function_or_method", "a function is the extension's")

	c = api.Counter.new(start=2, name="c")
	check(c.add(n=3) == 5 and c.sub(1) == 4, "a method takes its arguments by keyword")
	check(api.Point(1, 2).offset(dy=3, dx=4) == api.Point(5, 5), "a record's method takes keywords")
	check(api.has_text(note=api.Note.default()) is False, "a function takes keywords")
	check(api.Registry.new(first=c).edit(edit=lambda counter: True), "a closure passes by keyword")
	loop = asyncio.new_event_loop()
	check(loop.run_until_complete(api.halved(n=4)) == 2, "an async def takes keywords")
	channel = api.Channel.new()
	loop.run_until_complete(channel.send_later(payload=b"kw"))
	check(bytes(loop.run_until_complete(channel.recv())) == b"kw", "an eager future takes keywords")
	check(loop.run_until_complete(api.either_later("ab")) == 2, "a future owns an optional borrow")
	check(loop.run_until_complete(api.either_later(None)) == 0, "a future owns a None borrow")
	two, three = api.Counter.new("two", 2), api.Counter.new("three", 3)
	check(loop.run_until_complete(two.plus_later(three)) == 5, "a future holds an optional handle")
	check(loop.run_until_complete(two.plus_later(None)) == 2, "a future holds a None handle")
	loop.close()

	# C callers may pass no argument array at all for no arguments.
	import ctypes

	no_args = ctypes.pythonapi.PyObject_CallNoArgs
	no_args.restype, no_args.argtypes = ctypes.py_object, [ctypes.py_object]
	check(no_args(api.Note.default) == api.Note.default(), "a call with no argument array")

	raises(lambda: c.add(), "Counter.add() missing 1 required positional argument: 'n'")
	raises(lambda: c.add(1, 2), "Counter.add() takes 2 positional arguments but 3 were given")
	raises(lambda: c.add(1, n=2), "Counter.add() got multiple values for argument 'n'")
	raises(lambda: c.add(m=1), "Counter.add() got an unexpected keyword argument 'm'")
	raises(lambda: c.get(1), "Counter.get() takes 1 positional argument but 2 were given")
	raises(lambda: api.Counter.new(), "Counter.new() missing 2 required positional arguments: 'name' and 'start'")
	raises(lambda: api.has_text(1, 2), "has_text() takes 1 positional argument but 2 were given")

	check(str(inspect.signature(c.add)) == "(n)", "a bound method's signature")
	check(str(inspect.signature(api.Counter.new)) == "(name, start)", "a static method's signature")
	check(api.has_text.__doc__ == "Whether `note` has text, even empty.", "a function's docstring")
	check(api.Gauge.set.__doc__ == "Set the level each clone reads.", "a method's docstring")
	check(api.Counter.get.__doc__ is None, "a method without docs has no docstring")
	check(api.Registry.first_get.__doc__ is None, "a property without docs has no docstring")
	print("calls ok")


def broken(which):
	"""Each breaks a contract, which must fail with a message naming it."""
	match which:
		case "moved":
			c = api.Counter.new("c", 0)
			c.into_label(None)
			c.get()
		case "boxed moved":
			tail = api.Node.new(1, None)
			api.Node.new(2, tail)
			tail.sum()
		case "outlived":
			meters = api.Meters.new()
			first = meters.first()
			meters.clear()
			first.count()
		case "lent moved":
			api.Registry.new(api.Counter.new("c", 0)).edit(lambda counter: counter.into_label(None) and False)
		case "lent kept":
			kept = []
			api.Registry.new(api.Counter.new("c", 0)).edit(lambda counter: kept.append(counter) or False)
			kept[0].get()
		# A thread holds the lent counter while converting an argument, which waits, the GIL
		# released, until the lend has ended; the call would then reach a freed handle.
		case "lent held":
			converting, ended = threading.Event(), threading.Event()

			class Slow:
				def __index__(self):
					converting.set()
					ended.wait()
					return 1

			def edit(counter):
				edit.thread = threading.Thread(target=counter.add, args=(Slow(),))
				edit.thread.start()
				converting.wait()
				return False

			api.Registry.new(api.Counter.new("c", 0)).edit(edit)
			ended.set()
			edit.thread.join()
		# An IPv6 address where Rust takes `SocketAddrV4`.
		case "family":
			api.nexts(("::", 1, 0, 0), ("::", 1, 0, 0))
		case "callback threw":

			class Threw(api.Log):
				def log(self, level, line):
					raise RuntimeError("a log failed")

			api.emit(Threw(), bytes([1]))
		case _:
			raise ValueError(f"no case {which}")


match sys.argv[1:]:
	case []:
		script()
	case ["futures"]:
		futures()
	case ["owned"]:
		owned()
	case ["values"]:
		values()
	case ["calls"]:
		calls()
	case [which]:
		broken(which)
