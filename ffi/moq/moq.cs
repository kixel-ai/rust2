// moq.c's script through the generated C#: a broadcast's track, published and subscribed to
// in one process, with every read a `ValueTask`, and an origin, with moq-pattern, kio,
// moq-tokio, hang, and moq-mux exported into the same library, and the catalog lent to C#
// for a call.
//
// The local reads need no runtime: each read that must wait is polled again on the
// synchronization context the script drains, after the write that wakes it. A session over
// QUIC runs on moq-tokio's fallback runtime, whose threads wake C#'s, which then polls on
// the thread pool.

#nullable enable

using System;
using System.Collections.Concurrent;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Moq;

static class Program
{
	static int failures;

	static void Expect(bool ok, string what)
	{
		if (!ok)
		{
			Console.Error.WriteLine("failed: " + what);
			failures++;
		}
	}

	// The polls a wake schedules, run when the script drains them.
	sealed class Queue : SynchronizationContext
	{
		readonly ConcurrentQueue<(SendOrPostCallback, object?)> jobs = new ConcurrentQueue<(SendOrPostCallback, object?)>();

		public override void Post(SendOrPostCallback d, object? state) => jobs.Enqueue((d, state));

		public int Count => jobs.Count;

		public void Drain()
		{
			while (jobs.TryDequeue(out var job))
			{
				job.Item1(job.Item2);
			}
		}
	}

	static readonly Queue Q = new Queue();

	// What `task` returns once the polls queued so far ran, which must finish it.
	static T Ready<T>(ValueTask<T> task)
	{
		Q.Drain();
		if (!task.IsCompleted)
		{
			throw new InvalidOperationException("a local read pended with nothing left to wake it");
		}
		return task.Result;
	}

	static byte[] Utf8(string text) => Encoding.UTF8.GetBytes(text);

	static string Text(Bytes bytes) => Encoding.UTF8.GetString(bytes.Span);

	static void Main()
	{
		SynchronizationContext.SetSynchronizationContext(Q);

		var broadcast = BroadcastInfo.New().Produce();
		var track = broadcast.CreateTrack("video", null);
		Expect(track.Name() == "video", "a track has its name");

		// A second track of the same name is an error, printed as Rust prints it.
		try
		{
			broadcast.CreateTrack("video", null);
			Expect(false, "a duplicate track is an error");
		}
		catch (ErrorException e)
		{
			Expect(e.Message == "duplicate", "a duplicate track's error is Rust's");
		}

		var subscriber = Ready(track.Consume().Subscribe(null));

		// A group's arrival wakes the subscriber's pending receive.
		var receive = subscriber.RecvGroup();
		Q.Drain();
		Expect(!receive.IsCompleted, "a receive waits for a group");
		// The track is untimed, so its group's start is a bare key.
		var writer = track.AppendGroup(Timestamp.Unscaled(0));
		Expect(Q.Count == 1, "a group's arrival wakes the receive");
		var first = Ready(receive)!;
		Expect(first.Index() == 0, "the first group is 0");

		// So does a frame's.
		var read = first.ReadFrame();
		Q.Drain();
		Expect(!read.IsCompleted, "a read waits for a frame");
		writer.WriteFrame(Timestamp.Unscaled(0), Utf8("hello"));
		Expect(Q.Count == 1, "a frame's arrival wakes the read");
		var hello = Ready(read)!;
		Expect(Text(hello.Payload) == "hello", "a frame has its payload");
		Expect(hello.Timestamp.Scale() == null, "an untimed frame's timestamp has no scale");
		// When it arrived: kio's clock's `Instant`, from kio's own export.
		Expect(Lib.ClockNow().CheckedDurationSince(hello.Arrived()) != null, "a frame arrived before now");

		// A timestamp is a `Copy` value copied across, which allocates nothing. The track is
		// untimed, so a timed frame is refused.
		var at = Timestamp.FromMillis(1500);
		Expect(at.AsSecs() == 1, "1500 ms is 1 s");
		// Timestamps order in time across scales, and a scaled one and an unscaled key not at all.
		var second = Timestamp.FromSecs(1);
		var key = Timestamp.Unscaled(0);
		Expect(second.PartialCompareTo(at) == -1 && at.PartialCompareTo(second) == 1, "1 s is before 1500 ms");
		Expect(key.PartialCompareTo(at) == null, "an unscaled key is unordered against a time");
		try
		{
			writer.WriteFrame(at, Utf8("late"));
			Expect(false, "a timed frame on an untimed track is an error");
		}
		catch (ErrorException e)
		{
			Expect(e.Message == "frame timestamp doesn't fit its track", "the late frame's error is Rust's");
		}
		Expect(at.AsSecs() == 1, "a copied timestamp stays usable");

		// The last frame, bytes that are not UTF-8, then the group's end, which reads as null.
		var world = new byte[] { 0xff }.Concat(Utf8("world")).ToArray();
		writer.WriteFrame(Timestamp.Unscaled(0), world);
		writer.Finish().Dispose();
		Expect(Ready(first.ReadFrame())?.Payload.ToArray().SequenceEqual(world) == true, "the last frame reads");
		Expect(Ready(first.ReadFrame()) == null, "a finished group reads null");

		// An origin and the driver that runs it, a pair. Its root, which it lends, is returned
		// as a new handle to a clone.
		var (origin, driver) = OriginProducer.New(OriginConfig.Default());
		Expect(origin.Root().AsStr() == "", "an origin's root is empty");

		// Scoped beneath a root to patterns, which moq-pattern's own export builds.
		using (var patterns = Patterns.New())
		{
			Expect(patterns.Insert(Pattern.Subtree("alice")), "a pattern inserts");
			using var scoped = origin.Scope("room", patterns);
			Expect(scoped.Root().AsStr() == "room", "a scope has its root");
			Expect(scoped.Absolute("alice").AsStr() == "room/alice", "a scope's paths are absolute");
			var allowed = scoped.Allowed();
			Expect(allowed.Matches("alice/cam"), "a scope allows its patterns");
			Expect(!allowed.Matches("bob"), "a scope allows nothing else");
			using var consumer = scoped.Consume();
			Expect(consumer.Absolute("bob").AsStr() == "room/bob", "a scope's consumer shares its root");
		}

		// A broadcast the origin routes is stamped with its path, which its info lends. Each
		// handle to it is disposed, since an open one keeps it, and so the origin, alive.
		using (var publication = origin.Route("room/alice", OriginRouteOptions.Default()))
		{
			Moq.Path path;
			using (var routed = publication.Broadcast())
			{
				path = routed.Info().Path;
			}
			Expect(path.AsStr() == "room/alice", "a broadcast has its path");

			// Paths own their segments, so C# builds them, and relative ones, from a string.
			try
			{
				Moq.Path.New("/cam/");
				Expect(false, "a path with an empty segment is an error");
			}
			catch (PathErrorException e)
			{
				Expect(e.Error == PathError.EmptySegment, "an empty segment is its own error");
			}
			var cam = Moq.Path.New("cam");
			Expect(cam.AsStr() == "cam", "a path builds from its segments");
			var joined = path.Join(cam);
			Expect(joined.AsStr() == "room/alice/cam", "paths join");
			Expect(cam.Equals(joined.StripPrefix(path)), "a path strips its prefix");
			Expect(path.StripPrefix(joined) == null, "a path strips no longer prefix");
			Expect(joined.Resolve(PathRelative.New("../bob")).AsStr() == "room/bob", "a relative path resolves");
		}

		// The driver, polled as a language's event loop would: at the time, and with a waiter
		// made from the waker and held across polls, since what a poll registers lives only as
		// long as its waiter. It runs until every owner of the origin is gone.
		var wakes = 0;
		var waiter = Waiter.New(() => Interlocked.Increment(ref wakes));
		driver.Poll(Lib.ClockNow(), waiter);
		// A client publishes the origin through moq-net's generic `with_publisher`, at each
		// type that implements its bound, by reference, so C# keeps its handle.
		Client.New().WithPublisherProducer(origin).Dispose();
		var before = Volatile.Read(ref wakes);
		origin.Dispose();
		// The last owner's dispose wakes the driver, which then finishes, cleanly.
		Expect(Volatile.Read(ref wakes) > before, "the last owner's dispose wakes the driver");
		try
		{
			driver.Poll(Lib.ClockNow(), waiter);
			Expect(false, "a driver whose origin is gone finishes");
		}
		catch (ErrorException e)
		{
			Expect(e.Message == "closed", "a finished driver's error is Rust's");
		}

		// moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which the
		// catalog lists at the decoder's 48 kHz, keeping the head as its description.
		var head = Utf8("OpusHead").Concat(new byte[] { 1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0 }).ToArray();
		var audio = Lib.MuxCodecOpusConfig(head);
		Expect(audio.SampleRate == 48000, "Opus decodes at 48 kHz");
		Expect(audio.ChannelCount == 2, "the head is stereo");
		Expect(audio.Codec is HangCatalogAudioCodec.Opus, "the codec is Opus");
		Expect(audio.Description?.ToArray().SequenceEqual(head) == true, "the head is the description");
		// A truncated head is moq-mux's error.
		try
		{
			Lib.MuxCodecOpusConfig(head.AsSpan(0, 8));
			Expect(false, "a truncated head is an error");
		}
		catch (MuxErrorException e)
		{
			Expect(e.Message.Length > 0, "a truncated head's error has a message");
		}

		// A subscription's ranges, from an array of them.
		var times = new TrackRange(new TrackAnchor.Time(2), new TrackAnchor.Time(6));
		var ranged = TrackSubscription.Default().WithRanges(new[] { TrackRange.Live(), times }).Ranges;
		Expect(ranged.SequenceEqual(new[] { TrackRange.Live(), times }), "a subscription keeps its ranges");
		Expect(ranged[1].Start.Equals(new TrackAnchor.Time(2)), "a range starts at its first time");
		Expect(new TrackAnchor.Time(6).Equals(ranged[1].End), "a range ends at its last time, included");

		// A catalog at moq-mux's untyped extension, from the config C# builds, whose generic
		// `is_claimed` is named by the last segment of its argument, and whose `mutate` lends
		// the catalog to C#.
		var publisher = BroadcastInfo.New().Produce();
		var extra = MuxCatalogConfig.Default().WithCatalogExtra(HangCatalogExtra.Default());
		var catalog = MuxCatalogProducerExtra.New(publisher, extra);
		Expect(!catalog.IsClaimedBinaryConfig("state"), "an unclaimed name is not claimed");
		var lends = 0;
		catalog.Mutate(lent =>
		{
			if (lent.ToJson().ToString().StartsWith("{", StringComparison.Ordinal))
			{
				lends++;
			}
		});
		Expect(lends == 1, "mutate lends the catalog once");

		// A JSON track the catalog lists, published through moq-mux's generic `json_snapshot` at
		// a JSON document, and a section C# parses from text, set in the lent catalog and read
		// back as text.
		var status = catalog.JsonSnapshotConfig(
			publisher.CreateTrack("status", TrackInfo.Default().WithTimescale(Timescale.MILLI)),
			MuxJsonConfig.Default(),
			MuxJsonOptions.Default());
		Expect(status.Name() == "status", "a JSON snapshot has its track's name");
		Expect(catalog.IsClaimedJsonConfig("status"), "the JSON snapshot claims its name");
		// A value C# parses is published as the track's latest, which a subscriber reads back
		// as JSON text.
		using (var on = Json.Parse("{\"on\": true}"))
		{
			status.Update(on, Timestamp.FromMillis(1));
		}
		var latest = Ready(status.Consume().RecvGroup());
		var payload = latest == null ? null : Ready(latest.ReadFrame());
		Expect(payload != null && Text(payload.Payload) == "{\"on\":true}", "a subscriber reads the latest value");
		catalog.Mutate(lent =>
		{
			var sections = lent.Ext;
			sections.Set("app", Json.Parse("{\"v\": [1, true]}"));
			lent.Ext = sections;
		});
		string? app = null;
		catalog.Mutate(lent =>
		{
			Expect(lent.ToJson().ToString().Contains("\"status\""), "the catalog lists the JSON track");
			using var section = lent.Ext.Get("app");
			app = section?.ToText().ToString();
		});
		Expect(app == "{\"v\":[1,true]}", "the catalog reads its section back as text");
		// Malformed text fails to parse, and a reserved section is moq-mux's error.
		try
		{
			Json.Parse("{\"v\":").Dispose();
			Expect(false, "malformed JSON fails to parse");
		}
		catch (StringException e)
		{
			Expect(e.Error == "EOF while parsing a value at line 1 column 5", "a parse error is serde_json's");
		}
		try
		{
			using var sections = MuxCatalogHangExtra.Default();
			sections.Set("json", Json.Parse("1"));
			Expect(false, "a reserved section is an error");
		}
		catch (MuxErrorException e)
		{
			Expect(e.Message == "reserved catalog section: json", "a reserved section is moq-mux's error");
		}

		// moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with.
		Expect(TokioConnectError.Unauthorized.IsAuth(), "unauthorized is an auth error");
		Expect(TokioConnectError.Unauthorized.Display() == "unauthorized", "a connect error displays as Rust's");

		// Session config's foreign types. A URL is parsed from text, failing with why.
		try
		{
			Url.Parse("relay.example").Dispose();
			Expect(false, "a relative URL is an error");
		}
		catch (StringException e)
		{
			Expect(e.Message == "relative URL without a base", "a URL's parse error is Rust's");
		}
		// A session's config, its URL and pinned fingerprints set as Rust assigns them.
		var connect = TokioConnectConfig.Default();
		using (var tls = connect.Tls)
		{
			tls.Fingerprint = new Str[] { "ab:cd" };
			connect.Tls = tls;
		}
		connect.Url = Url.Parse("https://relay.example:4443/demo");
		using (var tls = connect.Tls)
		{
			Expect(tls.Fingerprint.SequenceEqual(new Str[] { "ab:cd" }), "a TLS config keeps its fingerprints");
		}
		var url = connect.Url!;
		connect.Dispose();
		Expect(url.ToString() == "https://relay.example:4443/demo", "a URL prints as Rust's");
		url.Dispose();
		// A `Vec<String>`, as an array.
		Expect(Lib.ALPNS.Length > 0, "a constant slice crosses as an array");
		using (var versions = Versions.Default())
		{
			var alpns = versions.Alpns();
			Expect(alpns.Length > 0 && alpns.All(a => a.Length > 0), "each version has an ALPN");
		}
		// A socket address, crossing in a listener's bind.
		var loopback = new byte[16];
		loopback[15] = 1;
		var bind = new TokioListenBind.Addr(new SocketAddrV6(new Ipv6Addr(loopback), 4443));
		Expect(bind.ToString() == "[::1]:4443", "an IPv6 bind prints as Rust's");
		var preferred = new SocketAddrV4(new Ipv4Addr(127, 0, 0, 1), 0);
		Expect(new TokioListenBind.Addr(preferred).ToString() == "127.0.0.1:0", "an IPv4 bind prints as Rust's");
		using (var listen = TokioListenConfig.Default())
		{
			listen.Bind = bind;
			// The preferred address of each family, which takes only that family.
			listen.PreferredV4 = preferred;
			Expect(preferred.Equals(listen.PreferredV4) && listen.PreferredV6 == null, "a listener keeps its preferred IPv4 address");
			Expect(bind.Equals(listen.Bind), "a listener's config keeps its bind");
		}
		// TLS files, as OS paths, and fingerprints, as strings: none by default.
		using (var tls = TokioTlsConnect.Default())
		{
			Expect(tls.Root.Length == 0 && tls.Fingerprint.Length == 0, "a default TLS config lists none");
		}

		// A session, as moq.c runs it, polled on the thread pool as moq-tokio's threads wake it.
		SynchronizationContext.SetSynchronizationContext(null);
		var session = Task.Run(Session);
		if (!session.Wait(TimeSpan.FromSeconds(30)))
		{
			Expect(false, "a session reads its frame within 30 s");
		}

		if (failures == 0)
		{
			Console.WriteLine("ok");
		}
		Environment.Exit(failures == 0 ? 0 : 1);
	}

	// A server bound to port 0 reads back the port it got and serves with one call, and a
	// client dials it with one call and reads a frame back.
	static async Task Session()
	{
		var served = Lib.TokioOriginSpawn();
		var live = served.Route("live", OriginRouteOptions.Default());
		var liveTrack = live.Broadcast().CreateTrack("video", null);
		var liveGroup = liveTrack.AppendGroup(Timestamp.Unscaled(0));
		liveGroup.WriteFrame(Timestamp.Unscaled(0), Utf8("hello"));
		liveGroup.Finish().Dispose();

		var serverConfig = TokioServerConfig.Default();
		var listen = serverConfig.Listen;
		listen.Bind = new TokioListenBind.Addr(new SocketAddrV6(new Ipv6Addr(new byte[16]), 0));
		var tls = listen.Tls;
		tls.Generate = new Str[] { "localhost" };
		listen.Tls = tls;
		serverConfig.Listen = listen;
		var server = serverConfig.Init();
		var local = server.LocalAddr();
		var port = local.Port;
		Expect(port != 0, "a server bound to port 0 reads back the port it got");
		using var stop = new CancellationTokenSource();
		var serving = server.ServePublish(served.Consume(), stop.Token);

		var connect = TokioConnectConfig.Default();
		connect.Url = Url.Parse("moqt://localhost:" + port);
		var insecure = connect.Tls;
		insecure.Insecure = true;
		connect.Tls = insecure;
		var clientConfig = TokioClientConfig.Default();
		clientConfig.Connect = connect;
		var client = clientConfig.Init();
		var received = Lib.TokioOriginSpawn();
		var remote = received.Consume();
		var connection = client.Clone().WithReconnect(false).Consume(received);
		Expect(connection != null, "a client with a URL dials it");
		var established = await connection!.Established();

		var subscriber = await (await remote.Request("live", null)).Track("video").Subscribe(null);
		var group = await subscriber.RecvGroup();
		var frame = group == null ? null : await group.ReadFrame();
		Expect(frame != null && Text(frame.Payload) == "hello", "a session reads the frame the server serves");

		await established.Close();
		await client.Close();
		stop.Cancel();
		try
		{
			await serving;
		}
		catch (OperationCanceledException)
		{
		}
	}
}
