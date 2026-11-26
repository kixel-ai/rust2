// bench/export.c's cases through the generated C#, so the two differ by what P/Invoke and
// the C# wrapper add. Each round runs twice, the first to warm the JIT. Each case prints
// its wall time and the process's CPU time, the JIT's and GC's threads included, or under
// Valgrind, runs once for swole to count. Its argument is
// moq/bench/hooks.h's shared library.

#nullable enable

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Globalization;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;
using Moq;

/// Wall and CPU nanoseconds: a point, or a span per call.
readonly record struct Span(double Wall, double Cpu);

/// The clocks and Callgrind's hooks, apart from the bench's `async` code, which C# keeps
/// out of an unsafe context.
static unsafe class Clock
{
	[StructLayout(LayoutKind.Sequential)]
	struct Timespec
	{
		public long Sec;
		public long Nsec;
	}

	[DllImport("libc", EntryPoint = "clock_gettime")]
	static extern int ClockGettime(int clock, Timespec* ts);

	// CLOCK_PROCESS_CPUTIME_ID, which .NET does not expose.
	static readonly int ProcessCpu = OperatingSystem.IsMacOS() ? 12 : 2;
	static readonly Stopwatch Wall = Stopwatch.StartNew();

	static delegate* unmanaged<byte*, byte> init;
	static delegate* unmanaged<void> start;
	static delegate* unmanaged<byte*, void> stop;

	public static Span Now()
	{
		Timespec ts;
		if (ClockGettime(ProcessCpu, &ts) != 0)
		{
			throw new InvalidOperationException("clock_gettime");
		}
		return new Span(Wall.ElapsedTicks * 1e9 / Stopwatch.Frequency, ts.Sec * 1e9 + ts.Nsec);
	}

	/// Opens moq/bench/hooks.h's library, returning whether Valgrind counts this.
	public static bool Open(string path, string bench)
	{
		var lib = NativeLibrary.Load(path);
		init = (delegate* unmanaged<byte*, byte>)NativeLibrary.GetExport(lib, "hooks_init");
		start = (delegate* unmanaged<void>)NativeLibrary.GetExport(lib, "hooks_start");
		stop = (delegate* unmanaged<byte*, void>)NativeLibrary.GetExport(lib, "hooks_stop");
		fixed (byte* name = Ascii(bench))
		{
			return init(name) != 0;
		}
	}

	public static void Start() => start();

	public static void Stop(string name)
	{
		fixed (byte* c = Ascii(name))
		{
			stop(c);
		}
	}

	static byte[] Ascii(string s) => System.Text.Encoding.ASCII.GetBytes(s + "\0");
}

static class Bench
{
	/// A tenth under Valgrind, which counts every call exactly.
	static int Groups = 10000;
	const int Frames = 100;
	static int N => Groups * Frames;
	const int Threads = 4;
	static readonly int[] Sizes = { 100, 1024, 16384, 65536, 1048576 };

	/// The sizes `frame.payload` is read at, as TypeScript's.
	static readonly int[] Payloads = { 16, 64, 100, 256, 1024, 4096, 16384, 65536, 262144, 1048576 };

	/// What the loops sum, kept so the JIT cannot drop the calls.
	static long sink;

	/// The round's clock in microseconds on its timed tracks, where each group starts at its
	/// first frame's time, so the starts a track's groups are appended at strictly increase.
	static ulong micros;

	/// Nanoseconds per call since `start`, over `calls` calls.
	static Span Per(Span start, double calls)
	{
		var end = Clock.Now();
		return new Span((end.Wall - start.Wall) / calls, (end.Cpu - start.Cpu) / calls);
	}

	/// A case's start: now, and Valgrind counting from here.
	static Span Begin()
	{
		var start = Clock.Now();
		Clock.Start();
		return start;
	}

	/// The case `name`'s nanoseconds per call since `start`, over `calls` calls, N by default.
	static Span End(Span start, string name, double calls = 0)
	{
		Clock.Stop(name);
		return Per(start, calls == 0 ? N : calls);
	}

	/// A new group's start, at which each of its frames is stamped: one conversion a group.
	static Timestamp GroupStart() => Timestamp.FromMicros(micros++);

	/// A timed track named `name` on `broadcast`. It caches every group written to it until it
	/// closes, as the broadcast's pool, which nothing polls, evicts none, so each case that
	/// writes groups writes them to a track of its own, disposed once it ends.
	static TrackProducer Track(BroadcastProducer broadcast, string name)
	{
		using var info = TrackInfo.Default();
		return broadcast.CreateTrack(name, info.WithTimescale(Timescale.MICRO));
	}

	/// A subscriber to `track`.
	static async Task<TrackSubscriber> Subscribe(TrackProducer track)
	{
		using var consumer = track.Consume();
		return await consumer.Subscribe(null);
	}

	/// `call` made N times on each of `Threads` threads at once, thread i on handles[i]: wall
	/// nanoseconds per call on one thread, and CPU nanoseconds per call.
	static Span Concurrent<T>(T[] handles, Func<T, long> call)
	{
		using var ready = new CountdownEvent(Threads);
		using var go = new ManualResetEventSlim();
		var failures = new System.Collections.Concurrent.ConcurrentQueue<Exception>();
		var workers = new Thread[Threads];
		for (var t = 0; t < Threads; t++)
		{
			var handle = handles[t];
			workers[t] = new Thread(() =>
			{
				long sum = 0;
				ready.Signal();
				go.Wait();
				try
				{
					for (var i = 0; i < N; i++)
					{
						sum += call(handle);
					}
				}
				catch (Exception e)
				{
					failures.Enqueue(e);
				}
				Interlocked.Add(ref sink, sum);
			});
			workers[t].Start();
		}
		ready.Wait();
		var start = Clock.Now();
		go.Set();
		foreach (var worker in workers)
		{
			worker.Join();
		}
		// A worker that died early would otherwise report a time for fewer than N calls.
		if (failures.TryPeek(out var failure))
		{
			throw failure;
		}
		var per = Per(start, N);
		return new Span(per.Wall, per.Cpu / Threads);
	}

	/// Groups of 100 frames of `size` bytes written and each read back, its payload's first
	/// byte touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second.
	static async Task<Span> Throughput(BroadcastProducer broadcast, int size)
	{
		var count = Math.Clamp((64 << 20) / (size * Frames), 3, 2000);
		var frame = new byte[size];
		var name = $"throughput[{size}]";
		using var track = Track(broadcast, name);
		using var subscriber = await Subscribe(track);
		var start = Begin();
		for (var g = 0; g < count; g++)
		{
			var at = GroupStart();
			using var group = track.AppendGroup(at);
			using (var reader = (await subscriber.RecvGroup())!)
			{
				for (var f = 0; f < Frames; f++)
				{
					group.WriteFrame(at, frame);
					using var read = (await reader.ReadFrame())!;
					using var payload = read.Payload;
					sink += payload.Span[0];
				}
			}
			group.Finish().Dispose();
		}
		var ns = End(start, name, (double)count * Frames * size);
		return new Span(1e3 / ns.Wall, 1e3 / ns.Cpu);
	}

	/// `frame.payload` of one frame of `size` bytes, read and disposed N times: up to 512 bytes
	/// a copy, past it a lend of Rust's buffer.
	static async Task<Span> Payload(BroadcastProducer broadcast, int size)
	{
		var name = $"payload[{size}]";
		using var track = Track(broadcast, name);
		using var subscriber = await Subscribe(track);
		var at = GroupStart();
		using var group = track.AppendGroup(at);
		FrameFrame read;
		using (var reader = (await subscriber.RecvGroup())!)
		{
			group.WriteFrame(at, new byte[size]);
			read = (await reader.ReadFrame())!;
		}
		group.Finish().Dispose();
		var start = Begin();
		for (var i = 0; i < N; i++)
		{
			using var payload = read.Payload;
			sink += payload.Span[0];
		}
		var ns = End(start, name);
		read.Dispose();
		return ns;
	}

	static async Task<Dictionary<string, Span>> Round(bool measured)
	{
		var out_ = new Dictionary<string, Span>();
		using var broadcastInfo = BroadcastInfo.New();
		using var broadcast = broadcastInfo.Produce();
		using var track = Track(broadcast, "bench");
		var frame = new byte[100];
		micros = 0;

		var start = Begin();
		for (var i = 0; i < N; i++)
		{
			if (track.IsClosed_())
			{
				sink++;
			}
		}
		out_["call"] = End(start, "call");

		using (var group = track.AppendGroup(GroupStart()))
		{
			using var demand = group.Demand();
			start = Begin();
			for (var i = 0; i < N; i++)
			{
				sink += (long)demand.Start();
			}
			out_["getter"] = End(start, "getter");

			// The handle call and the getter from `Threads` threads: on one shared handle,
			// then on a clone each, a handle of its own to the same value. Valgrind
			// runs one thread at a time, which contends for nothing, so it skips them.
			if (!measured)
			{
				var tracks = new TrackProducer[Threads];
				var demands = new GroupDemand[Threads];
				for (var t = 0; t < Threads; t++)
				{
					tracks[t] = track.Clone();
					demands[t] = demand.Clone();
				}
				Func<TrackProducer, long> isClosed = t => t.IsClosed_() ? 1 : 0;
				Func<GroupDemand, long> groupStart = d => (long)d.Start();
				out_["shared:call"] = Concurrent(Array.ConvertAll(tracks, _ => track), isClosed);
				out_["own:call"] = Concurrent(tracks, isClosed);
				out_["shared:getter"] = Concurrent(Array.ConvertAll(demands, _ => demand), groupStart);
				out_["own:getter"] = Concurrent(demands, groupStart);
				foreach (var t in tracks)
				{
					t.Dispose();
				}
				foreach (var d in demands)
				{
					d.Dispose();
				}
			}
			group.Finish().Dispose();
		}

		start = Begin();
		for (var i = 0; i < N; i++)
		{
			if (track.Largest() != null)
			{
				sink++;
			}
		}
		out_["record"] = End(start, "record");

		start = Begin();
		for (var i = 0; i < N; i++)
		{
			track.Info().Dispose();
		}
		out_["object"] = End(start, "object");

		// A primitive pub field read, which reaches no user code.
		using (var info = track.Info())
		{
			start = Begin();
			for (var i = 0; i < N; i++)
			{
				sink += info.Priority;
			}
			out_["field"] = End(start, "field");
		}

		// Each frame stamped through a fresh timestamp, as a caller holding a ulong must, its
		// group starting at its first frame's.
		using (var stamped = Track(broadcast, "stamped"))
		{
			start = Begin();
			for (var g = 0; g < Groups; g++)
			{
				using var group = stamped.AppendGroup(Timestamp.FromMicros(micros));
				for (var f = 0; f < Frames; f++)
				{
					group.WriteFrame(Timestamp.FromMicros(micros++), frame);
				}
				group.Finish().Dispose();
			}
			out_["stamped"] = End(start, "stamped");
		}

		using (var written = Track(broadcast, "write"))
		{
			start = Begin();
			for (var g = 0; g < Groups; g++)
			{
				var at = GroupStart();
				using var group = written.AppendGroup(at);
				for (var f = 0; f < Frames; f++)
				{
					group.WriteFrame(at, frame);
				}
				group.Finish().Dispose();
			}
			out_["write"] = End(start, "write");
		}

		// Each frame written, then read through a call whose task is complete at its first
		// poll, so it never waits for a wake.
		FrameFrame? last = null;
		using (var ready = Track(broadcast, "ready"))
		using (var subscriber = await Subscribe(ready))
		{
			start = Begin();
			for (var g = 0; g < Groups; g++)
			{
				var at = GroupStart();
				using var group = ready.AppendGroup(at);
				using (var reader = (await subscriber.RecvGroup())!)
				{
					for (var f = 0; f < Frames; f++)
					{
						group.WriteFrame(at, frame);
						last?.Dispose();
						last = (await reader.ReadFrame())!;
					}
				}
				group.Finish().Dispose();
			}
			out_["ready"] = End(start, "ready");
		}

		// Each read called before its frame is written, so it pends, and the write's wake
		// polls it again on the thread pool, which completes the task.
		using (var woken = Track(broadcast, "woken"))
		using (var subscriber = await Subscribe(woken))
		{
			start = Begin();
			for (var g = 0; g < Groups; g++)
			{
				var at = GroupStart();
				using var group = woken.AppendGroup(at);
				using (var reader = (await subscriber.RecvGroup())!)
				{
					for (var f = 0; f < Frames; f++)
					{
						var pending = reader.ReadFrame();
						group.WriteFrame(at, frame);
						(await pending)!.Dispose();
					}
				}
				group.Finish().Dispose();
			}
			out_["woken"] = End(start, "woken");
		}

		// One writer task and one reader, with at most one frame permitted.
		using (var looped = Track(broadcast, "loop"))
		using (var subscriber = await Subscribe(looped))
		{
			start = Begin();
			using var permits = new SemaphoreSlim(0);
			var writer = Task.Run(async () =>
			{
				for (var g = 0; g < Groups; g++)
				{
					var at = GroupStart();
					using var group = looped.AppendGroup(at);
					for (var f = 0; f < Frames; f++)
					{
						await permits.WaitAsync();
						group.WriteFrame(at, frame);
					}
					await permits.WaitAsync();
					group.Finish().Dispose();
				}
			});
			for (var g = 0; g < Groups; g++)
			{
				using (var reader = (await subscriber.RecvGroup())!)
				{
					for (var f = 0; f < Frames; f++)
					{
						permits.Release();
						(await reader.ReadFrame())!.Dispose();
					}
				}
				permits.Release();
			}
			await writer;
			out_["loop"] = End(start, "loop");
		}

		foreach (var size in Sizes)
		{
			out_[$"mbps:{size}"] = await Throughput(broadcast, size);
		}

		// The last frame's 100 bytes, copied across, and one of them read.
		start = Begin();
		for (var i = 0; i < N; i++)
		{
			using var payload = last!.Payload;
			sink += payload.Span[0];
		}
		out_["payload"] = End(start, "payload");
		last!.Dispose();
		foreach (var size in Payloads)
		{
			out_[$"payload:{size}"] = await Payload(broadcast, size);
		}
		return out_;
	}

	static async Task Main(string[] args)
	{
		var measured = Clock.Open(args[0], "ffi/moq/bench/export.cs::csharp");
		// Under Valgrind, one round, which prints nothing: swole counts each case.
		if (measured)
		{
			Groups /= 10;
			await Round(true);
			return;
		}
		await Round(false);
		foreach (var (name, span) in await Round(false))
		{
			Console.WriteLine(string.Create(CultureInfo.InvariantCulture, $"{name}\t{span.Wall}\t{span.Cpu}"));
		}
		GC.KeepAlive(sink);
	}
}
