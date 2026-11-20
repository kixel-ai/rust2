// Plain Java calling the api fixture's generated Kotlin, as tests/fixtures.rs compiles and
// runs it: a companion's function on its class, a checked exception javac only lets a
// `catch` take where the function declares it, and bytes through a direct ByteBuffer.

import api.ApiKt;
import api.Bytes;
import api.IpAddr;
import api.StringException;
import api.Url;
import java.net.InetAddress;
import java.nio.ByteBuffer;

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
		check(IpAddr.of(InetAddress.getByName("127.0.0.1")).toString().equals("127.0.0.1"), "the runtime's companion functions are static");

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
}
