// The api fixture driven from C#: the transcript, which `fixtures.rs` generates as the
// other half of `Script`, then what only C# says, each a case of its own.
#nullable enable
// `api` aliases the namespace as Rust spells the crate.
#pragma warning disable CS8981

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using api = Api;

/// Keeps even levels, as each language's implementation does, and says when Rust drops it.
sealed class Print : api.Log, IDisposable
{
	public bool Log(byte level, api.Str line)
	{
		Console.WriteLine($"log {level} {line}");
		return level % 2 == 0;
	}

	public void Dispose() => Console.WriteLine("freed");
}

/// An edit history whose methods each return their own digit.
sealed class Edits : api.Undo
{
	public uint Delete(uint n) => n - 1;

	public uint Free() => 2;

	public byte Type() => 3;
}

/// A log calling `f` for each line, keeping none.
sealed class CountLog : api.Log
{
	readonly Action f;

	public CountLog(Action f) => this.f = f;

	public bool Log(byte level, api.Str line)
	{
		f();
		return false;
	}
}

/// A reader whose reads fail, and each seek lands at 0.
sealed class Broken : api.ISeekableReader
{
	public int Read(Span<byte> buffer) => throw new IOException("broken");

	public ulong Seek(long offset, SeekOrigin origin) => 0;
}

/// A reader that seeks anywhere, and is empty there.
sealed class Far : api.ISeekableReader
{
	public int Read(Span<byte> buffer) => 0;

	public ulong Seek(long offset, SeekOrigin origin) => origin == SeekOrigin.Begin ? unchecked((ulong)offset) : 0;
}

/// A reader whose seek throws what is no I/O error.
sealed class Thrower : api.ISeekableReader
{
	public int Read(Span<byte> buffer) => 0;

	public ulong Seek(long offset, SeekOrigin origin) => throw new InvalidOperationException("a seek failed");
}

static partial class Script
{
	static int Main(string[] args)
	{
		Console.OutputEncoding = new UTF8Encoding(false);
		switch (args)
		{
			case []:
				Transcript();
				break;
			case ["checks"]:
				Checks();
				break;
			case ["futures"]:
				Futures().GetAwaiter().GetResult();
				break;
			case [var which]:
				Fail(which);
				break;
			default:
				Check(false, "no case " + string.Join(" ", args));
				break;
		}
		Console.Out.Flush();
		return 0;
	}

	static void Check(bool ok, string what)
	{
		if (!ok)
		{
			Console.Error.WriteLine("failed: " + what);
			Environment.Exit(1);
		}
	}

	/// `f` throws a `T`, whose message `test` accepts.
	static void Throws<T>(Action f, string what, Func<T, bool>? test = null)
		where T : Exception
	{
		try
		{
			f();
		}
		catch (T e)
		{
			Check(test == null || test(e), $"{what}: {e.GetType().Name}: {e.Message}");
			return;
		}
		Check(false, what);
	}

	/// Waits for `done`, which another thread makes true, failing after ten seconds.
	static void Until(Func<bool> done, string what) => Check(SpinWait.SpinUntil(done, TimeSpan.FromSeconds(10)), what);

	static T Within<T>(ValueTask<T> task, string what)
	{
		var t = task.AsTask();
		Check(t.Wait(TimeSpan.FromSeconds(10)), what);
		return t.Result;
	}

	static byte[] Utf8(string text) => Encoding.UTF8.GetBytes(text);

	static string _Hex(ReadOnlySpan<byte> bytes)
	{
		var out_ = new StringBuilder();
		foreach (var b in bytes)
		{
			out_.Append(b.ToString("x2"));
		}
		return out_.ToString();
	}

	static string Hex(ReadOnlyMemory<byte> bytes) => _Hex(bytes.Span);

	static string Hex(api.ArrayU8x4 bytes) => _Hex(bytes.ToArray());

	/// A path's bytes as Rust's `escape_ascii` prints them.
	static string Escaped(api.PathBuf path)
	{
		var out_ = new StringBuilder();
		foreach (var b in path.Bytes.Span)
		{
			switch (b)
			{
				case 0x09:
					out_.Append("\\t");
					break;
				case 0x0d:
					out_.Append("\\r");
					break;
				case 0x0a:
					out_.Append("\\n");
					break;
				case 0x5c or 0x27 or 0x22:
					out_.Append('\\').Append((char)b);
					break;
				case >= 0x20 and < 0x7f:
					out_.Append((char)b);
					break;
				default:
					out_.Append("\\x").Append(b.ToString("x2"));
					break;
			}
		}
		return out_.ToString();
	}

	static string PathHex(api.PathBuf path) => _Hex(path.Bytes.Span);

	/// A duration as its seconds and nine digits of nanoseconds.
	static string Seconds(api.Duration d) => $"{d.Secs}.{d.Nanos:D9}";

	/// A duration in whole milliseconds, cut to a `u64` as Rust's `as` does.
	static ulong Millis(api.Duration d) => unchecked(d.Secs * 1000UL + d.Nanos / 1000000);

	static uint Code(api.Scalar c) => c.Value;

	static T[] Reversed<T>(T[] list) => list.Reverse().ToArray();

	static ulong[] F64Bits(double[] floats) => floats.Select(BitConverter.DoubleToUInt64Bits).ToArray();

	/// A socket address as its IP's bytes in hex, its port, flow label, and scope, each
	/// IPv6's alone, apart by colons.
	static string SocketHex(api.SocketAddr addr) => addr.V6 is api.SocketAddrV6 v6
		? $"{_Hex(v6.Ip.Octets())}:{v6.Port}:{v6.FlowInfo}:{v6.ScopeId}"
		: $"{_Hex(addr.V4!.Value.Ip.Octets())}:{addr.V4!.Value.Port}";

	/// An address's bytes in hex, so the length shows its family.
	static string IpHex(api.IpAddr ip) => ip.V6 is api.Ipv6Addr v6 ? _Hex(v6.Octets()) : _Hex(ip.V4!.Value.Octets());

	/// A path no Rust string spells: bytes that are not UTF-8, or on Windows, WTF-8's lone
	/// surrogate.
	static api.PathBuf Unspelled() =>
		OperatingSystem.IsWindows() ? new api.PathBuf(new byte[] { 0xed, 0xa0, 0x80 }) : api.Lib.Stray();

	/// Print each of `n`'s lists, its floats as their bits.
	static void Numbers(api.Str what, api.Numbers n)
	{
		static string Of<T>(IEnumerable<T> items) => "[" + string.Join(", ", items) + "]";
		Console.WriteLine($"{what} u16 {Of(n.U16s)}");
		Console.WriteLine($"{what} u32 {Of(n.U32s)}");
		Console.WriteLine($"{what} u64 {Of(n.U64s)}");
		Console.WriteLine($"{what} i8 {Of(n.I8s)}");
		Console.WriteLine($"{what} i16 {Of(n.I16s)}");
		Console.WriteLine($"{what} i32 {Of(n.I32s)}");
		Console.WriteLine($"{what} i64 {Of(n.I64s)}");
		Console.WriteLine($"{what} f32 {Of(n.F32s.Select(BitConverter.SingleToUInt32Bits))}");
		Console.WriteLine($"{what} f64 {Of(n.F64s.Select(BitConverter.DoubleToUInt64Bits))}");
		Console.WriteLine($"{what} some {(n.Some is int[] some ? $"Some({Of(some)})" : "None")}");
	}

	/// A channel's futures, each awaited as a `ValueTask`.
	static void Channel()
	{
		using var channel = api.Channel.New();
		var wakes = 0;
		// Polled at the call, it pends, and the send's wake polls it again on the pool.
		var recv = channel.Recv();
		Console.WriteLine($"recv pending {_Bool(!recv.IsCompleted)}");
		channel.Send(Utf8("hi"));
		var got = Within(recv, "a send wakes a recv");
		wakes++;
		Console.WriteLine($"woken {wakes}");
		Console.WriteLine($"recv {_Text(got)}");
		channel.Send(Array.Empty<byte>());
		try
		{
			Console.WriteLine($"nonempty {_Block(channel.RecvNonempty()).Length}");
		}
		catch (api.ErrorException e)
		{
			Console.WriteLine($"nonempty err {e.Error}");
		}
		using (var fill4 = api.Batch.New())
		using (var fill2 = api.Lib.Pair())
		{
			var filled = _Block(api.Lib.Fill4(fill4, 9)).Concat(_Block(api.Lib.Fill2(fill2, 1)));
			Console.WriteLine($"fill {string.Join(" ", filled)}");
		}
		// The send happens at the call: the task is dropped unawaited.
		_ = channel.SendLater(Utf8("eager"));
		Console.WriteLine($"recv {_Text(_Block(channel.Recv()))}");
		// The send happens at the call, and the future borrows the channel and the tag.
		Console.WriteLine($"tagged {_Block(channel.Tagged("tag"))}");
		Console.WriteLine($"itemized {_Block(channel.Itemized(new uint[] { 1, 2, 3 }))}");
		Console.WriteLine($"byte_len {_Block(api.Lib.ByteLen(Utf8("abc")))}");
		channel.Send(Utf8("sized"));
		var (payload, size) = _Block(channel.RecvSized());
		Console.WriteLine($"sized {_Text(payload)} {size}");
		// The waker moves into Rust, which keeps it until the send wakes it.
		channel.Notify(() => Interlocked.Increment(ref wakes));
		channel.Send(Utf8("rung"));
		Until(() => Volatile.Read(ref wakes) == 2, "the send wakes the waker Rust keeps");
		Console.WriteLine($"woken {wakes}");
		// A poll with no async twin is a future.
		Console.WriteLine($"take {_Text(_Block(channel.PollTake()))}");
		channel.Send(Utf8("measured"));
		Console.WriteLine($"measure {_Block(channel.PollMeasure(2))}");
		// A context is made of the waker, which Rust keeps until the send wakes it.
		channel.Listen(() => Interlocked.Increment(ref wakes));
		channel.Send(Utf8("heard"));
		Until(() => Volatile.Read(ref wakes) == 3, "the send wakes the context Rust keeps");
		Console.WriteLine($"woken {wakes}");
		// A returned waker wakes the waiter's task through Rust until it is disposed, after
		// which a wake throws.
		var waiter = api.Waiter.New(() => Interlocked.Increment(ref wakes));
		var waker = waiter.Waker();
		var context = waiter.Context();
		waker.Wake();
		context.Wake();
		waiter.Dispose();
		waker.Dispose();
		context.Dispose();
		context.Dispose();
		Throws<ObjectDisposedException>(waker.Wake, "a disposed waker throws");
		Until(() => Volatile.Read(ref wakes) == 5, "a returned waker wakes the waiter");
		Console.WriteLine($"woken {wakes}");
	}

	/// A kept closure: called again at the instant it asks for, then once its waiter wakes,
	/// failing with the exception of Rust's error.
	static void Drive()
	{
		var start = api.Moment.New(0, 1000) ?? throw _None();
		var calls = 0;
		var stopped = Within(
			api.Lib.Drive(
				start,
				(now, waiter) =>
				{
					calls++;
					switch (calls)
					{
						case 1:
							return now.After(5);
						case 2:
							Console.WriteLine($"drive {now.Millis() - start.Millis()}");
							using (var waker = waiter.Waker())
							{
								waker.Wake();
							}
							return null;
						default:
							throw new api.StringException($"stopped after {calls}");
					}
				}
			),
			"a kept closure wakes its future"
		);
		Console.WriteLine($"drive {stopped}");
	}

	/// Readers the caller implements: a file through `SeekableStream`, one whose reads fail,
	/// and one that seeks past `long.MaxValue`.
	static void Readers()
	{
		var dir = Directory.CreateTempSubdirectory("api");
		var path = Path.Combine(dir.FullName, "tail");
		File.WriteAllText(path, "hello world");
		using (var file = File.OpenRead(path))
		{
			foreach (var reader in new api.ISeekableReader[] { new api.SeekableStream(file), new Broken() })
			{
				try
				{
					var (tail, at) = api.Lib.Tail(reader, 5);
					Console.WriteLine($"tail {_Text(tail)} {at}");
				}
				catch (api.StringException e)
				{
					Console.WriteLine($"tail error {e.Error}");
				}
			}
		}
		dir.Delete(true);
		// A seek from the start reaches the reader whole, past `long.MaxValue`, as `Cursor`
		// takes one.
		try
		{
			Console.WriteLine($"sought {api.Lib.SeekTo(new Far(), ulong.MaxValue)}");
		}
		catch (api.StringException e)
		{
			Console.WriteLine($"sought error {e.Error}");
		}
	}

	static void Streams() => StreamsAsync().GetAwaiter().GetResult();

	/// Streams, each iterated with `await foreach`: to its end, left early, and to an error,
	/// past which the stream goes on.
	static async Task StreamsAsync()
	{
		var counts = new List<uint>();
		using (var countdown = api.Countdown.New(3))
		{
			await foreach (var count in countdown)
			{
				counts.Add(count);
			}
		}
		Console.WriteLine($"countdown {string.Join(" ", counts)}");
		// A loop left early takes nothing past the item it stopped at.
		using (var rest = api.Countdown.New(5))
		{
			await foreach (var count in rest)
			{
				if (count == 4)
				{
					break;
				}
			}
			Console.WriteLine($"countdown after break {await rest.Next()}");
		}
		// Each payload sends the next, which `!` fails and an empty one ends.
		using var channel = api.Channel.New();
		using var feed = channel.Feed();
		foreach (var first in new[] { "a", "c" })
		{
			channel.Send(Utf8(first));
			try
			{
				await foreach (var payload in feed)
				{
					Console.WriteLine($"feed {_Text(payload)}");
					var reply = _Text(payload) switch
					{
						"a" => "b",
						"b" => "!",
						_ => "",
					};
					channel.Send(Utf8(reply));
				}
				Console.WriteLine("feed end");
			}
			catch (api.ErrorException e)
			{
				Console.WriteLine($"feed error {e.Error}");
			}
		}
	}

	/// Futures of a value that is not `Send`, awaited on this thread's turn.
	static void LocalLater(api.Local local)
	{
		using (var later = _Block(api.Lib.LocalOrNewLater(local)))
		{
			Console.WriteLine($"local later {later.Bump()}");
		}
		using (var fresh = _Block(api.Lib.LocalOrNewLater(null)))
		{
			Console.WriteLine($"local later fresh {fresh.Bump()}");
		}
	}

	/// What the transcript cannot say: C#'s own checks of what Rust cannot hold, and of
	/// handles used past their life.
	static void Checks()
	{
		// A disposed handle throws C#'s own exception, naming its type.
		var counter = api.Counter.New("c", 1);
		counter.Dispose();
		counter.Dispose();
		Throws<ObjectDisposedException>(() => counter.Get(), "a disposed handle throws");
		// A string Rust cannot hold fails where it converts, not in the call.
		Throws<ArgumentException>(() => _ = (api.Str)"\ud800", "a lone surrogate is no Str");
		// A call taking the value disposes the handle.
		var moved = api.Counter.New("m", 0);
		moved.IntoLabel(null);
		Check(moved.IsClosed, "a call taking the value disposes the handle");
		Throws<ObjectDisposedException>(() => moved.Get(), "a moved handle throws", e => e.ObjectName == "Counter");
		Disposes();
		Raced();
	}

	/// A dispose drops Rust's value at once, once, and Rust refuses it under a borrow.
	static void Disposes()
	{
		var before = api.Lib.Dropped();
		var token = api.Token.New(1);
		token.Dispose();
		Check(api.Lib.Dropped() == before + 1, "a dispose drops at once");
		token.Dispose();
		Check(api.Lib.Dropped() == before + 1, "a second dispose drops nothing");
		// Refused under a Rust borrow, it leaves the object open.
		var ledger = api.Ledger.New();
		var draft = ledger.Edit();
		const string borrowed = "Ledger.Dispose: a handle is already borrowed";
		Throws<InvalidOperationException>(ledger.Dispose, "a dispose under a borrow throws", e => e.Message == borrowed);
		// Two disposes under the borrow both throw: one meeting the other's crossing waits out
		// its refusal rather than report the object disposed.
		for (var k = 0; k < 2000; k++)
		{
			using var start = new ManualResetEventSlim();
			Exception? late = null;
			var second = new Thread(() =>
			{
				start.Wait();
				late = Caught(ledger.Dispose);
			});
			second.Start();
			start.Set();
			var first = Caught(ledger.Dispose);
			second.Join();
			foreach (var thrown in new[] { first, late })
			{
				Check(thrown is InvalidOperationException && thrown.Message == borrowed, $"racing disposes under a borrow both throw as borrowed: {first}, {late}");
			}
		}
		draft.Dispose();
		Check(ledger.Published() == 0, "a refused dispose leaves the object open");
		ledger.Dispose();
		Throws<ObjectDisposedException>(() => ledger.Published(), "a dispose after a refused one disposes");
	}

	static Exception? Caught(Action f)
	{
		try
		{
			f();
			return null;
		}
		catch (Exception e)
		{
			return e;
		}
	}

	/// A dispose racing calls on other threads: a latch's calls only borrow it, so C# counts
	/// them; a token's Rust checks.
	static void Raced()
	{
		var before = api.Lib.Dropped();
		var latch = api.Latch.Default();
		latch.Set(5);
		Race(latch, l => l.Get() == 5, "Latch.Dispose: a handle is already borrowed", null);
		Race(api.Token.New(5), t => t.Id == 5, "Token.Dispose: a handle is already borrowed", "api_Token_id_get: a handle's value was moved out");
		Check(api.Lib.Dropped() == before + 1, "a dispose racing calls drops Rust's value once");
	}

	/// Disposes `made` while 8 threads call `call` on it: each call runs on the live value or,
	/// once it is disposed, throws `ObjectDisposedException` or, for a value Rust checks, its
	/// moved check, met by a call that held the handle before; a dispose a running call beats
	/// throws `borrowed`, as Rust refuses a drop under a borrow, and changes nothing, so the
	/// next tries again.
	static void Race<T>(T made, Func<T, bool> call, string borrowed, string? moved)
		where T : api.RustObject
	{
		var ran = 0;
		var failed = 0;
		var callers = Enumerable.Range(0, 8).Select(_ => new Thread(() =>
		{
			while (true)
			{
				bool live;
				try
				{
					live = call(made);
				}
				catch (ObjectDisposedException)
				{
					Interlocked.Increment(ref failed);
					return;
				}
				catch (InvalidOperationException e) when (e.Message == moved)
				{
					Interlocked.Increment(ref failed);
					return;
				}
				Check(live, "a call racing a dispose reads the live value");
				Interlocked.Increment(ref ran);
			}
		})).ToList();
		callers.ForEach(t => t.Start());
		while (Volatile.Read(ref ran) < 100)
		{
			Thread.Yield();
		}
		while (true)
		{
			try
			{
				made.Dispose();
				break;
			}
			catch (InvalidOperationException e)
			{
				Check(e.Message == borrowed, $"a dispose racing a call throws: {e.Message}");
			}
		}
		callers.ForEach(t => t.Join());
		Check(failed == 8, "every call after a dispose throws");
		made.Dispose();
	}

	/// Futures: a cancel frees the Rust future and loses nothing, and a wake during a poll
	/// polls again.
	static async Task Futures()
	{
		using var channel = api.Channel.New();
		using (var cancel = new CancellationTokenSource())
		{
			var recv = channel.Recv(cancel.Token);
			Check(!recv.IsCompleted, "a recv pends on an empty channel");
			cancel.Cancel();
			try
			{
				await recv;
				Check(false, "a cancelled recv throws");
			}
			catch (OperationCanceledException)
			{
			}
		}
		channel.Send(Utf8("after cancel"));
		Check(_Text(await channel.Recv()) == "after cancel", "a cancelled recv loses nothing");
		// A token cancelled before the call cancels a future that pends, and not one ready at once.
		using (var cancelled = new CancellationTokenSource())
		{
			cancelled.Cancel();
			channel.Send(Utf8("ready"));
			Check(_Text(await channel.Recv(cancelled.Token)) == "ready", "a ready recv ignores its token");
			var pending = channel.Recv(cancelled.Token);
			try
			{
				await pending;
				Check(false, "a recv of a cancelled token throws");
			}
			catch (OperationCanceledException)
			{
			}
			channel.Send(Utf8("kept"));
			Check(_Text(await channel.Recv()) == "kept", "a recv of a cancelled token takes nothing");
		}
		// A dispose of a handle a pending call borrows throws, as Rust refuses a drop under a
		// borrow, and changes nothing: the call ends as it would, and then the dispose passes.
		{
			var other = api.Channel.New();
			var held = other.Recv();
			var sender = other.Clone();
			Throws<InvalidOperationException>(other.Dispose, "a dispose under a pending call throws", e => e.Message == "Channel.Dispose: a handle is already borrowed");
			sender.Send(Utf8("held"));
			Check(_Text(await held) == "held", "the pending call ends as it would");
			other.Dispose();
			Throws<ObjectDisposedException>(other.Clear, "a call after a dispose throws");
			sender.Dispose();
		}
		// A send from another thread wakes the recv pending here, each time, racing its poll.
		for (var k = 0; k < 200; k++)
		{
			var recv = channel.Recv();
			var text = k.ToString();
			await Task.Run(() => channel.Send(Utf8(text)));
			Check(_Text(await recv.AsTask().WaitAsync(TimeSpan.FromSeconds(10))) == text, "a send from another thread wakes the recv");
		}
	}

	[DllImport("libc", EntryPoint = "prctl")]
	static extern int Prctl(int option, ulong a, ulong b, ulong c, ulong d);

	/// Each broken contract, which fails loud.
	static void Fail(string which)
	{
		if (OperatingSystem.IsLinux())
		{
			// Non-dumpable, an expected abort writes no core. PR_SET_DUMPABLE is 4.
			Check(Prctl(4, 0, 0, 0, 0) == 0, "prctl");
		}
		switch (which)
		{
			case "moved":
			{
				var c = api.Counter.New("c", 0);
				c.IntoLabel(null);
				c.Get();
				break;
			}
			case "outlived":
			{
				var meters = api.Meters.New();
				var first = meters.First();
				meters.Clear();
				first.Count();
				break;
			}
			case "lent moved":
				api.Registry.New(api.Counter.New("c", 0)).Edit(counter =>
				{
					counter.IntoLabel(null);
					return false;
				});
				break;
			case "lent kept":
			{
				api.Counter? kept = null;
				api.Registry.New(api.Counter.New("c", 0)).Edit(counter =>
				{
					kept = counter;
					return false;
				});
				kept!.Get();
				break;
			}
			case "callback threw":
				api.Lib.Emit(new CountLog(() => throw new InvalidOperationException("a log failed")), new byte[] { 1 });
				break;
			// Only the exception of Rust's error fails a closure.
			case "closure threw":
				_Block(api.Lib.Drive(api.Moment.New(0, 1000)!.Value, (_, _) => throw new InvalidOperationException("a poll failed")));
				break;
			// Only an IOException fails a reader.
			case "reader threw":
				api.Lib.Tail(new Thrower(), 1);
				break;
			default:
				Check(false, "no case " + which);
				break;
		}
	}
}
