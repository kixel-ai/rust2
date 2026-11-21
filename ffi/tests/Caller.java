// Plain Java calling the api fixture's generated Kotlin, as tests/fixtures.rs compiles and
// runs it, reaching what a Kotlin caller reaches: a companion's function on its class, a
// constructor named as Java reserves, a checked exception javac only lets a `catch` take where
// the function declares it, bytes through a direct ByteBuffer, functions, fields, and
// constructors of unsigned integers, whose names Kotlin would mangle, an opaque value as its
// class, the runtime's values, unsigned integers boxed, and callback interfaces it implements.

import api.ApiKt;
import api.Berth;
import api.Bytes;
import api.ClashStore;
import api.Duration;
import api.Hits;
import api.IpAddr;
import api.Ipv4Addr;
import api.Ipv6Addr;
import api.Log;
import api.Packet;
import api.Platform;
import api.Point;
import api.RangeU64;
import api.Sensor;
import api.Shape;
import api.SocketAddrV4;
import api.SocketAddrV6;
import api.Stamp;
import api.StampException;
import api.Stamped;
import api.StartSeekable;
import api.StringException;
import api.Ticket;
import api.UIntException;
import api.Unsigned;
import api.Url;
import java.net.InetAddress;
import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.SeekableByteChannel;
import java.util.Collections;
import java.util.List;

class Caller {
	public static void main(String[] args) throws Exception {
		try (Url url = Url.parse("https://example.com/a")) {
			check(url.toString().equals("https://example.com/a"), "a companion's function is static");
		}
		try {
			Url.parse("not a url").close();
			throw new AssertionError("a relative URL parses");
		} catch (StringException e) {
			check(!e.getError().isEmpty(), "a declared exception is caught");
		}
		try {
			check(ApiKt.sameObject(args, args), "a top-level function is static");
		} catch (StringException e) {
			throw new AssertionError("the same object fails", e);
		}
		check(Point.getX_() == 0 && new Point(3, 4).getX() == 3, "a static apart from a field's getter takes a trailing _");
		check(Shape.valueOf_("square") == Shape.SQUARE && Shape.valueOf("SQUARE") == Shape.SQUARE, "a static apart from an enum's valueOf takes a trailing _");
		check(IpAddr.of(InetAddress.getByName("127.0.0.1")).toString().equals("127.0.0.1"), "the runtime's companion functions are static");
		check(Point.default_().equals(new Point(0, 0)), "Rust's Default is default_, which Java does not reserve");

		check(ApiKt.sameU64(-1L) == -1L && ApiKt.span(1, -1) == -2, "a top-level function takes and returns unsigned integers");
		Hits hits = Hits.of(-1L, 2L).hitsSet(3L);
		check(hits.getHitsGet() == 3L && hits.getMissesGet() == 2L, "a member takes and returns unsigned integers");

		Packet packet = ApiKt.nextPacket(Packet.of(-2, new Bytes(new byte[] {7})));
		check(packet.getSeq() == -1 && packet.getPayload().get(0) == 7, "a record of an unsigned field is made by of and read by its getter");
		check(Platform.Native.of(-1).get_0() == -1, "a variant of an unsigned field is made by of and read by its getter");
		RangeU64 range = ApiKt.widen(RangeU64.of(2L, -2L), 1L);
		check(range.getStart() == 1L && range.getEnd() == -1L, "a range of unsigned integers is made by of and read by its getters");
		try {
			ApiKt.odd(4);
			throw new AssertionError("4 is odd");
		} catch (UIntException e) {
			check(e.getError() == 4 && UIntException.of(-1).getError() == -1, "an exception of an unsigned error is made by of and read by its getter");
		}

		try (ClashStore store = ClashStore.new_(3)) {
			Ticket ticket = Ticket.new_(7);
			check(Ticket.number(ticket) == 7 && store.take(ticket) == 10 && ClashStore.take_(ticket) == 7, "an opaque value crosses as its class, made by new_");
			Ticket reissued = ApiKt.reissue(ticket, t -> Ticket.new_(Ticket.number(t) + 1));
			check(Ticket.number(reissued) == 8 && Ticket.debug(reissued).contains("8"), "a closure takes and returns an opaque value as its class");
		}
		Stamp early = Stamp.fromMillis(1500L);
		Stamp late = Stamp.fromMillis(2500L);
		check(Stamp.secs(early) == 1L && Stamp.later(late, early) && Stamp.compareTo(late, early) > 0, "an opaque value's members are its class's statics");
		check(Collections.max(List.of(late, early)) == late, "an opaque value is Comparable");
		check(Stamp.partialCompareTo(early, late) == -1 && Stamp.default_().equals(Stamp.fromMillis(0L)), "an opaque value's PartialOrd and Default are statics");
		check(ApiKt.past(late, early).equals(late), "a function returns an opaque value as its class");
		try {
			ApiKt.past(early, late);
			throw new AssertionError("an early stamp is past");
		} catch (StampException e) {
			check(e.getError().equals(late) && StampException.of(early).getError().equals(early), "an exception of an opaque value is read by its getter as its class");
		}
		Stamped stamped = Stamped.of(late, 3);
		check(stamped.getAt().equals(late) && stamped.getCount() == 3 && stamped.total() == 5L, "a record of an opaque field takes and reads it as its class");

		Duration duration = Duration.of(-1L, 5);
		check(duration.getSecs() == -1L && duration.getNanos() == 5 && duration.compareTo(Duration.Companion.getMAX()) < 0, "a Duration is made by of and read by its getters");
		Ipv4Addr ip = Ipv4Addr.of(new byte[] {127, 0, 0, 1});
		check(Ipv4Addr.bits(ip) == 0x7f000001 && ip.equals(Ipv4Addr.of(0x7f000001)) && ip.equals(Ipv4Addr.of((byte) 127, (byte) 0, (byte) 0, (byte) 1)), "an Ipv4Addr is made by of and read by bits");
		Ipv4Addr next = ApiKt.nextIps(ip, new Ipv6Addr(new byte[16])).getFirst();
		check(next.toString().equals("127.0.0.2") && Ipv4Addr.octets(next)[3] == 2, "an Ipv4Addr crosses as its class");
		SocketAddrV4 socket = ApiKt.sameSocketAddrV4(SocketAddrV4.of(ip, (short) -1));
		check(SocketAddrV4.ip(socket).equals(ip) && SocketAddrV4.port(socket) == -1, "a SocketAddrV4 crosses as its class, made by of and read by ip and port");
		check(SocketAddrV4.of(new InetSocketAddress(InetAddress.getByName("127.0.0.1"), 443)).toInetSocketAddress().getPort() == 443, "a SocketAddrV4 is made of an InetSocketAddress");
		SocketAddrV6 v6 = SocketAddrV6.of(new Ipv6Addr(new byte[16]), (short) -1, -1, 3);
		check(SocketAddrV6.port(v6) == -1 && v6.getFlowinfo() == -1 && v6.getScopeId() == 3, "a SocketAddrV6 is made by of and read by its getters");

		try (Berth berth = Berth.new_(Unsigned.box(-2))) {
			check(Unsigned.bits(berth.at()) == -2, "a nullable unsigned integer is boxed and unboxed");
		}
		check(ApiKt.measured(() -> Unsigned.box(-2L)) == -1L, "a closure returns an unsigned integer boxed");
		check(Unsigned.bits(Unsigned.box((byte) -1)) == -1 && Unsigned.bits(Unsigned.box((short) -1)) == -1, "each width boxes and unboxes");

		Levels levels = new Levels();
		check(ApiKt.emit(Log.of(levels), new Bytes(new byte[] {(byte) 200, 1})) == 1, "Java implements a callback interface of an unsigned integer");
		check(levels.seen == 201 && levels.closed, "Rust passes the callback each unsigned integer's bits, and closes it as it drops it");
		Dial dial = new Dial();
		check(ApiKt.sensed(Sensor.of(dial)) == -1 && dial.stopped == 1, "Rust calls a callback's own close once, apart from closing it as it drops it");
		try (Far far = new Far()) {
			check(ApiKt.seekTo(far, -1L) == -1L && far.at == -1L, "Java implements StartSeekable");
		}

		ByteBuffer frame = ByteBuffer.allocateDirect(5);
		frame.put(new byte[] {1, 2, 3, 4, 5}).flip();
		Bytes doubled = ApiKt.prefixed(new Bytes(frame), new Bytes(frame));
		ByteBuffer input = ByteBuffer.allocateDirect(10);
		doubled.copyInto(input, 0, doubled.getSize());
		check(input.position() == 10 && input.get(9) == 5 && input.get(4) == 5 && input.get(5) == 1, "bytes cross a direct ByteBuffer both ways");
		System.out.println("java");
	}

	static void check(boolean ok, String what) {
		if (!ok) throw new AssertionError(what);
	}

	/** A log keeping the sum of the levels it saw, and the line of level 200. */
	static final class Levels implements Log.Java, AutoCloseable {
		int seen;
		boolean closed;

		@Override
		public boolean log(byte level, String line) {
			seen += level & 0xff;
			return level == (byte) 200;
		}

		@Override
		public void close() {
			closed = true;
		}
	}

	/** A sensor reading the most a `u32` holds, counting how often Rust stops it. */
	static final class Dial implements Sensor.Java {
		int stopped;

		@Override
		public int read() {
			return -1;
		}

		@Override
		public void close() {
			stopped++;
		}
	}

	/** An empty channel seeking from its start to any offset, as Rust's `SeekFrom::Start` does. */
	static final class Far implements SeekableByteChannel, StartSeekable.Java {
		long at;

		@Override
		public long seekStart(long offset) {
			at = offset;
			return offset;
		}

		@Override
		public int read(ByteBuffer dst) {
			return -1;
		}

		@Override
		public int write(ByteBuffer src) {
			throw new UnsupportedOperationException();
		}

		@Override
		public long position() {
			return 0;
		}

		@Override
		public SeekableByteChannel position(long position) {
			at = position;
			return this;
		}

		@Override
		public long size() {
			return 0;
		}

		@Override
		public SeekableByteChannel truncate(long size) {
			throw new UnsupportedOperationException();
		}

		@Override
		public boolean isOpen() {
			return true;
		}

		@Override
		public void close() {}
	}
}
