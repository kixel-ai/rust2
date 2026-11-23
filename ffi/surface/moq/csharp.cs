// moq.cs
#nullable enable
namespace Moq {
	public abstract unsafe class RustObject : IDisposable {
		public bool IsClosed { get; }
		public void Dispose();
	}
	public readonly struct Str : IEquatable<Str>, IComparable<Str> {
		public ReadOnlyMemory<byte> Utf8 { get; }
		public static Str From(string text);
		public static Str FromUtf8(ReadOnlySpan<byte> utf8);
		public static implicit operator Str(string text);
		public int Length { get; }
		public override string ToString();
		public bool Equals(Str other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(Str other);
		public static bool operator ==(Str a, Str b);
		public static bool operator !=(Str a, Str b);
	}
	public readonly struct PathBuf : IEquatable<PathBuf> {
		public ReadOnlyMemory<byte> Bytes { get; }
		public PathBuf(ReadOnlySpan<byte> bytes);
		public PathBuf(string text);
		public static implicit operator PathBuf(string text);
		public string ToText();
		public override string ToString();
		public bool Equals(PathBuf other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct Bytes : IDisposable, IEquatable<Bytes> {
		public ReadOnlyMemory<byte> Memory { get; }
		public Bytes(ReadOnlyMemory<byte> memory);
		public static implicit operator Bytes(byte[] bytes);
		public static implicit operator Bytes(ReadOnlyMemory<byte> memory);
		public int Length { get; }
		public ReadOnlySpan<byte> Span { get; }
		public byte[] ToArray();
		public void Dispose();
		public bool Equals(Bytes other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Bytes a, Bytes b);
		public static bool operator !=(Bytes a, Bytes b);
	}
	public readonly struct Scalar : IEquatable<Scalar>, IComparable<Scalar> {
		public uint Value { get; }
		public Scalar(uint value);
		public static implicit operator Scalar(char c);
		public override string ToString();
		public bool Equals(Scalar other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(Scalar other);
		public static bool operator ==(Scalar a, Scalar b);
		public static bool operator !=(Scalar a, Scalar b);
	}
	public readonly struct U128 : IEquatable<U128>, IComparable<U128> {
		public ulong High { get; }
		public ulong Low { get; }
		public U128(ulong high, ulong low);
		public static implicit operator U128(ulong v);
		public System.Numerics.BigInteger ToBigInteger();
		public static U128 From(System.Numerics.BigInteger v);
		#if NET7_0_OR_GREATER
		public static implicit operator UInt128(U128 v);
		public static implicit operator U128(UInt128 v);
		#endif
		public string ToHex(int width = 0);
		public override string ToString();
		public bool Equals(U128 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(U128 other);
		public static bool operator ==(U128 a, U128 b);
		public static bool operator !=(U128 a, U128 b);
	}
	public readonly struct Duration : IEquatable<Duration>, IComparable<Duration> {
		public ulong Secs { get; }
		public uint Nanos { get; }
		public Duration(ulong secs, uint nanos = 0);
		public static Duration From(TimeSpan span);
		public TimeSpan ToTimeSpan();
		public bool Equals(Duration other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(Duration other);
		public override string ToString();
		public static bool operator ==(Duration a, Duration b);
		public static bool operator !=(Duration a, Duration b);
	}
	public readonly struct Ipv4Addr : IEquatable<Ipv4Addr> {
		public uint Bits { get; }
		public Ipv4Addr(uint bits);
		public Ipv4Addr(byte a, byte b, byte c, byte d);
		public byte[] Octets();
		public System.Net.IPAddress ToIPAddress();
		public static Ipv4Addr From(System.Net.IPAddress address);
		public override string ToString();
		public bool Equals(Ipv4Addr other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct Ipv6Addr : IEquatable<Ipv6Addr> {
		public Ipv6Addr(ReadOnlySpan<byte> octets);
		public byte this[int k] { get; }
		public byte[] Octets();
		public System.Net.IPAddress ToIPAddress();
		public static Ipv6Addr From(System.Net.IPAddress address);
		public override string ToString();
		public bool Equals(Ipv6Addr other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct IpAddr : IEquatable<IpAddr> {
		public Ipv4Addr? V4 { get; }
		public Ipv6Addr? V6 { get; }
		public IpAddr(Ipv4Addr v4);
		public IpAddr(Ipv6Addr v6);
		public static implicit operator IpAddr(Ipv4Addr v4);
		public static implicit operator IpAddr(Ipv6Addr v6);
		public override string ToString();
		public bool Equals(IpAddr other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct SocketAddrV4 : IEquatable<SocketAddrV4> {
		public Ipv4Addr Ip { get; }
		public ushort Port { get; }
		public SocketAddrV4(Ipv4Addr ip, ushort port);
		public override string ToString();
		public bool Equals(SocketAddrV4 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct SocketAddrV6 : IEquatable<SocketAddrV6> {
		public Ipv6Addr Ip { get; }
		public ushort Port { get; }
		public uint FlowInfo { get; }
		public uint ScopeId { get; }
		public SocketAddrV6(Ipv6Addr ip, ushort port, uint flowInfo = 0, uint scopeId = 0);
		public override string ToString();
		public bool Equals(SocketAddrV6 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public readonly struct SocketAddr : IEquatable<SocketAddr> {
		public SocketAddrV4? V4 { get; }
		public SocketAddrV6? V6 { get; }
		public SocketAddr(SocketAddrV4 v4);
		public SocketAddr(SocketAddrV6 v6);
		public SocketAddr(IpAddr ip, ushort port);
		public static implicit operator SocketAddr(SocketAddrV4 v4);
		public static implicit operator SocketAddr(SocketAddrV6 v6);
		public ushort Port { get; }
		public override string ToString();
		public bool Equals(SocketAddr other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public sealed unsafe class Waker : IDisposable {
		public void Wake();
		public void Dispose();
	}
	public abstract class RustException : Exception {
		protected RustException(string message);
	}
	public interface ISeekableReader {
		int Read(Span<byte> buffer);
		ulong Seek(long offset, SeekOrigin origin);
	}
	public sealed class SeekableStream : ISeekableReader {
		public SeekableStream(Stream stream);
		public int Read(Span<byte> buffer);
		public ulong Seek(long offset, SeekOrigin origin);
	}
	public enum Bound {
		Unbounded = 0,
		Included = 1,
		Excluded = 2,
	}
	public struct BoundU64 : IEquatable<BoundU64> {
		public Bound Kind;
		public ulong Value;
		public BoundU64(Bound kind, ulong value = 0);
		public static BoundU64 Unbounded { get; }
		public static BoundU64 Included(ulong value);
		public static BoundU64 Excluded(ulong value);
		public bool Equals(BoundU64 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public override string ToString();
	}
	public struct BoundsU64 : IEquatable<BoundsU64> {
		public BoundU64 Start;
		public BoundU64 End;
		public BoundsU64(BoundU64 start, BoundU64 end);
		public bool Equals(BoundsU64 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public struct RangeU64 : IEquatable<RangeU64> {
		public ulong Start;
		public ulong End;
		public RangeU64(ulong start, ulong end);
		public bool Equals(RangeU64 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe struct ArrayU8x2 : IEquatable<ArrayU8x2> {
		public ArrayU8x2(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x2 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe struct ArrayU8x4 : IEquatable<ArrayU8x4> {
		public ArrayU8x4(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x4 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe struct ArrayU8x6 : IEquatable<ArrayU8x6> {
		public ArrayU8x6(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x6 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe struct ArrayU8x16 : IEquatable<ArrayU8x16> {
		public ArrayU8x16(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x16 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe struct ArrayU8x32 : IEquatable<ArrayU8x32> {
		public ArrayU8x32(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x32 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe partial struct TimeOverflow : IEquatable<TimeOverflow> {
		public bool Equals(TimeOverflow other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TimeOverflow a, TimeOverflow b);
		public static bool operator !=(TimeOverflow a, TimeOverflow b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct Timescale : IEquatable<Timescale>, IComparable<Timescale> {
		public bool Equals(Timescale other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Timescale a, Timescale b);
		public static bool operator !=(Timescale a, Timescale b);
		public int? PartialCompareTo(Timescale other);
		public static bool operator <(Timescale a, Timescale b);
		public static bool operator <=(Timescale a, Timescale b);
		public static bool operator >(Timescale a, Timescale b);
		public static bool operator >=(Timescale a, Timescale b);
		public int CompareTo(Timescale other);
		public override string ToString();
		public string Debug();
		public static Timescale SECOND { get; }
		public static Timescale MILLI { get; }
		public static Timescale MICRO { get; }
		public static Timescale NANO { get; }
		public static Timescale New(ulong unitsPerSecond);
		public ulong AsU64();
	}
	public unsafe partial struct Timestamp : IEquatable<Timestamp> {
		public bool Equals(Timestamp other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Timestamp a, Timestamp b);
		public static bool operator !=(Timestamp a, Timestamp b);
		public int? PartialCompareTo(Timestamp other);
		public static bool operator <(Timestamp a, Timestamp b);
		public static bool operator <=(Timestamp a, Timestamp b);
		public static bool operator >(Timestamp a, Timestamp b);
		public static bool operator >=(Timestamp a, Timestamp b);
		public string Debug();
		public static Timestamp ZERO { get; }
		public static Timestamp New(ulong value, Timescale scale);
		public static Timestamp Unscaled(ulong key);
		public static Timestamp FromScale(ulong value, ulong unitsPerSecond);
		public static Timestamp FromSecs(ulong seconds);
		public static Timestamp FromMillis(ulong millis);
		public static Timestamp FromMicros(ulong micros);
		public static Timestamp FromNanos(ulong nanos);
		public ulong Value();
		public Timescale? Scale();
		public bool IsZero();
		public Timestamp Convert(Timescale newScale);
		public U128 AsScale(Timescale target);
		public ulong AsSecs();
		public U128 AsMillis();
		public U128 AsMicros();
		public U128 AsNanos();
		public Timestamp CheckedAdd(Timestamp rhs);
		public Timestamp CheckedSub(Timestamp rhs);
	}
	public unsafe partial struct ClockInstant : IEquatable<ClockInstant>, IComparable<ClockInstant> {
		public bool Equals(ClockInstant other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClockInstant a, ClockInstant b);
		public static bool operator !=(ClockInstant a, ClockInstant b);
		public int? PartialCompareTo(ClockInstant other);
		public static bool operator <(ClockInstant a, ClockInstant b);
		public static bool operator <=(ClockInstant a, ClockInstant b);
		public static bool operator >(ClockInstant a, ClockInstant b);
		public static bool operator >=(ClockInstant a, ClockInstant b);
		public int CompareTo(ClockInstant other);
		public string Debug();
		public static ClockInstant ZERO { get; }
		public static ClockInstant Now();
		public Duration Elapsed();
		public Duration DurationSince(ClockInstant earlier);
		public Duration? CheckedDurationSince(ClockInstant earlier);
		public Duration SaturatingDurationSince(ClockInstant earlier);
		public ClockInstant? CheckedAdd(Duration duration);
		public ClockInstant? CheckedSub(Duration duration);
		public ClockInstant SaturatingAdd(Duration duration);
		public ClockInstant SaturatingSub(Duration duration);
		public Instant IntoStd();
		public static ClockInstant FromStd(Instant at);
	}
	public unsafe partial struct ClockElapsed : IEquatable<ClockElapsed> {
		public bool Equals(ClockElapsed other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClockElapsed a, ClockElapsed b);
		public static bool operator !=(ClockElapsed a, ClockElapsed b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct ClockWall : IEquatable<ClockWall>, IComparable<ClockWall> {
		public bool Equals(ClockWall other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClockWall a, ClockWall b);
		public static bool operator !=(ClockWall a, ClockWall b);
		public int? PartialCompareTo(ClockWall other);
		public static bool operator <(ClockWall a, ClockWall b);
		public static bool operator <=(ClockWall a, ClockWall b);
		public static bool operator >(ClockWall a, ClockWall b);
		public static bool operator >=(ClockWall a, ClockWall b);
		public int CompareTo(ClockWall other);
		public string Debug();
		public static ClockWall UNIX_EPOCH { get; }
		public static ClockWall MOQ_EPOCH { get; }
		public static ClockWall Now();
		public Duration? CheckedDurationSince(ClockWall earlier);
		public Duration SaturatingDurationSince(ClockWall earlier);
		public ClockWall? CheckedAdd(Duration duration);
		public ClockWall SaturatingAdd(Duration duration);
		public ClockWall? CheckedSub(Duration duration);
	}
	public unsafe partial struct SetupToken : IEquatable<SetupToken> {
		public ulong Kind;
		public Bytes Value;
		public SetupToken(ulong kind, Bytes value);
		public bool Equals(SetupToken other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(SetupToken a, SetupToken b);
		public static bool operator !=(SetupToken a, SetupToken b);
		public string Debug();
	}
	public unsafe partial struct SessionLimits : IEquatable<SessionLimits> {
		public bool Equals(SessionLimits other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(SessionLimits a, SessionLimits b);
		public static bool operator !=(SessionLimits a, SessionLimits b);
		public string Debug();
		public static SessionLimits Default();
		public nuint Announces { get; set; }
	}
	public unsafe partial struct StatsContent : IEquatable<StatsContent> {
		public bool Equals(StatsContent other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsContent a, StatsContent b);
		public static bool operator !=(StatsContent a, StatsContent b);
		public string Debug();
		public static StatsContent Default();
		public ulong Bytes { get; set; }
		public ulong Frames { get; set; }
		public ulong Groups { get; set; }
	}
	public unsafe partial struct StatsTraffic : IEquatable<StatsTraffic> {
		public bool Equals(StatsTraffic other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsTraffic a, StatsTraffic b);
		public static bool operator !=(StatsTraffic a, StatsTraffic b);
		public string Debug();
		public static StatsTraffic Default();
		public ulong AnnouncesStarted { get; set; }
		public ulong AnnouncesEnded { get; set; }
		public ulong AnnouncedBytes { get; set; }
		public ulong BroadcastsStarted { get; set; }
		public ulong BroadcastsEnded { get; set; }
		public ulong SubscriptionsStarted { get; set; }
		public ulong SubscriptionsEnded { get; set; }
		public ulong Fetches { get; set; }
		public ulong Bytes { get; set; }
		public ulong Frames { get; set; }
		public ulong Groups { get; set; }
		public ulong Datagrams { get; set; }
		public ulong Inflated { get; set; }
		public StatsContent Stale { get; set; }
		public void Add(StatsTraffic other);
		public ulong ActiveBroadcasts();
		public ulong ActiveSubscriptions();
		public bool IsIdle();
	}
	public unsafe partial struct StatsPresence : IEquatable<StatsPresence> {
		public bool Equals(StatsPresence other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsPresence a, StatsPresence b);
		public static bool operator !=(StatsPresence a, StatsPresence b);
		public string Debug();
		public static StatsPresence Default();
		public StatsRefusals SessionsRefused { get; set; }
		public StatsFailures SessionsFailed { get; set; }
		public ulong Requests { get; set; }
		public ulong SessionsStarted { get; set; }
		public ulong SessionsEnded { get; set; }
		public ulong SentBytes { get; set; }
		public ulong ReceivedBytes { get; set; }
		public ulong AnnouncesPeak { get; set; }
		public ulong SubscriptionsPeak { get; set; }
		public ulong WireTx { get; set; }
		public ulong WireRx { get; set; }
		public ulong RoutesRefused { get; set; }
		public StatsRoutes Routes { get; set; }
		public void Add(StatsPresence other);
		public ulong Active();
	}
	public unsafe partial struct StatsActivity : IEquatable<StatsActivity> {
		public bool Equals(StatsActivity other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsActivity a, StatsActivity b);
		public static bool operator !=(StatsActivity a, StatsActivity b);
		public string Debug();
		public static StatsActivity Default();
		public ulong Idle { get; set; }
		public ulong Active { get; set; }
	}
	public unsafe partial struct StatsTotals : IEquatable<StatsTotals> {
		public bool Equals(StatsTotals other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsTotals a, StatsTotals b);
		public static bool operator !=(StatsTotals a, StatsTotals b);
		public string Debug();
		public static StatsTotals Default();
		public ulong Tx { get; set; }
		public ulong Rx { get; set; }
		public ulong TxMessages { get; set; }
		public ulong RxMessages { get; set; }
	}
	public unsafe partial struct StatsFailures : IEquatable<StatsFailures> {
		public bool Equals(StatsFailures other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsFailures a, StatsFailures b);
		public static bool operator !=(StatsFailures a, StatsFailures b);
		public string Debug();
		public static StatsFailures Default();
		public ulong IdleTimeout { get; set; }
		public ulong Transport { get; set; }
		public ulong Application { get; set; }
		public ulong Expired { get; set; }
		public ulong Count(StatsFailure kind);
		public void Add(StatsFailures other);
		public bool Regressed(StatsFailures previous);
	}
	public unsafe partial struct StatsRefusals : IEquatable<StatsRefusals> {
		public bool Equals(StatsRefusals other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsRefusals a, StatsRefusals b);
		public static bool operator !=(StatsRefusals a, StatsRefusals b);
		public string Debug();
		public static StatsRefusals Default();
		public ulong Refused { get; set; }
		public ulong Unavailable { get; set; }
		public ulong Request { get; set; }
		public ulong Forbidden { get; set; }
		public ulong Membership { get; set; }
		public ulong Expired { get; set; }
		public ulong Count(StatsRefusalKind kind);
		public void Add(StatsRefusals other);
		public bool Regressed(StatsRefusals previous);
	}
	public unsafe partial struct StatsRouteCloses : IEquatable<StatsRouteCloses> {
		public bool Equals(StatsRouteCloses other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsRouteCloses a, StatsRouteCloses b);
		public static bool operator !=(StatsRouteCloses a, StatsRouteCloses b);
		public string Debug();
		public static StatsRouteCloses Default();
		public ulong Stalled { get; set; }
		public ulong Overrun { get; set; }
		public ulong Malformed { get; set; }
		public ulong Role { get; set; }
		public ulong Unasked { get; set; }
		public ulong Regressed { get; set; }
		public ulong Other { get; set; }
		public ulong Count(StatsRouteClose kind);
		public ulong Total();
		public void Add(StatsRouteCloses other);
		public bool Regressed_(StatsRouteCloses previous);
	}
	public unsafe partial struct StatsRouteRefusals : IEquatable<StatsRouteRefusals> {
		public bool Equals(StatsRouteRefusals other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsRouteRefusals a, StatsRouteRefusals b);
		public static bool operator !=(StatsRouteRefusals a, StatsRouteRefusals b);
		public string Debug();
		public static StatsRouteRefusals Default();
		public ulong Path { get; set; }
		public ulong Scope { get; set; }
		public ulong Looped { get; set; }
		public ulong Count(StatsRouteRefusal kind);
		public ulong Total();
		public void Add(StatsRouteRefusals other);
		public bool Regressed(StatsRouteRefusals previous);
	}
	public unsafe partial struct StatsRoutes : IEquatable<StatsRoutes> {
		public bool Equals(StatsRoutes other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsRoutes a, StatsRoutes b);
		public static bool operator !=(StatsRoutes a, StatsRoutes b);
		public string Debug();
		public static StatsRoutes Default();
		public ulong Changed { get; set; }
		public ulong Rewritten { get; set; }
		public StatsWithdrawals Sent { get; set; }
		public StatsWithdrawals Received { get; set; }
		public StatsRouteRefusals Refused { get; set; }
		public StatsRouteCloses Closed { get; set; }
		public void Add(StatsRoutes other);
		public bool Regressed(StatsRoutes previous);
	}
	public unsafe partial struct StatsWithdrawals : IEquatable<StatsWithdrawals> {
		public bool Equals(StatsWithdrawals other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsWithdrawals a, StatsWithdrawals b);
		public static bool operator !=(StatsWithdrawals a, StatsWithdrawals b);
		public string Debug();
		public static StatsWithdrawals Default();
		public ulong Dead { get; set; }
		public ulong Drain { get; set; }
		public ulong Count(StatsWithdrawal kind);
		public ulong Total();
		public void Add(StatsWithdrawals other);
		public bool Regressed(StatsWithdrawals previous);
	}
	public readonly struct NonZeroU64 : IEquatable<NonZeroU64>, IComparable<NonZeroU64> {
		public ulong Value { get; }
		public NonZeroU64(ulong value);
		public static explicit operator NonZeroU64(ulong value);
		public static implicit operator ulong(NonZeroU64 value);
		public override string ToString();
		public bool Equals(NonZeroU64 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroU64 other);
	}
	public unsafe partial struct TransportReceiveWindow : IEquatable<TransportReceiveWindow> {
		public bool Equals(TransportReceiveWindow other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TransportReceiveWindow a, TransportReceiveWindow b);
		public static bool operator !=(TransportReceiveWindow a, TransportReceiveWindow b);
		public string Debug();
		public NonZeroU64? Connection { get; set; }
		public ulong Stream { get; set; }
		public static TransportReceiveWindow New(NonZeroU64? connection, ulong stream);
	}
	public unsafe partial struct TransportLimits : IEquatable<TransportLimits> {
		public bool Equals(TransportLimits other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TransportLimits a, TransportLimits b);
		public static bool operator !=(TransportLimits a, TransportLimits b);
		public string Debug();
		public static TransportLimits Default();
		public NonZeroU64? ReceiveWindow { get; set; }
	}
	public unsafe partial struct BandwidthRate : IEquatable<BandwidthRate>, IComparable<BandwidthRate> {
		public bool Equals(BandwidthRate other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(BandwidthRate a, BandwidthRate b);
		public static bool operator !=(BandwidthRate a, BandwidthRate b);
		public int? PartialCompareTo(BandwidthRate other);
		public static bool operator <(BandwidthRate a, BandwidthRate b);
		public static bool operator <=(BandwidthRate a, BandwidthRate b);
		public static bool operator >(BandwidthRate a, BandwidthRate b);
		public static bool operator >=(BandwidthRate a, BandwidthRate b);
		public int CompareTo(BandwidthRate other);
		public string Debug();
		public static BandwidthRate Default();
		public static BandwidthRate ZERO { get; }
		public static BandwidthRate FromBps(ulong bps);
		public static BandwidthRate FromKbps(ulong kbps);
		public static BandwidthRate FromMbps(ulong mbps);
		public ulong AsBps();
		public BandwidthRate Scaled(double factor);
		public BandwidthRate AbsDiff(BandwidthRate other);
	}
	public unsafe partial struct DatagramDatagram : IEquatable<DatagramDatagram> {
		public ulong Sequence;
		public Timestamp Timestamp;
		public Bytes Payload;
		public DatagramDatagram(ulong sequence, Timestamp timestamp, Bytes payload);
		public bool Equals(DatagramDatagram other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(DatagramDatagram a, DatagramDatagram b);
		public static bool operator !=(DatagramDatagram a, DatagramDatagram b);
		public string Debug();
	}
	public unsafe partial struct FrameInfo {
		public ulong Size;
		public Timestamp Timestamp;
		public FrameInfo(ulong size, Timestamp timestamp);
		public string Debug();
	}
	public unsafe partial struct GroupInfo : IEquatable<GroupInfo>, IComparable<GroupInfo> {
		public ulong Start;
		public GroupInfo(ulong start);
		public bool Equals(GroupInfo other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(GroupInfo a, GroupInfo b);
		public static bool operator !=(GroupInfo a, GroupInfo b);
		public int? PartialCompareTo(GroupInfo other);
		public static bool operator <(GroupInfo a, GroupInfo b);
		public static bool operator <=(GroupInfo a, GroupInfo b);
		public static bool operator >(GroupInfo a, GroupInfo b);
		public static bool operator >=(GroupInfo a, GroupInfo b);
		public int CompareTo(GroupInfo other);
		public string Debug();
	}
	public unsafe partial struct TrackDelay : IEquatable<TrackDelay> {
		public Duration? Min;
		public Duration? Max;
		public TrackDelay(Duration? min, Duration? max);
		public bool Equals(TrackDelay other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackDelay a, TrackDelay b);
		public static bool operator !=(TrackDelay a, TrackDelay b);
		public string Debug();
		public static TrackDelay Default();
		public TrackDelay WithMin(Duration? min);
		public TrackDelay WithMax(Duration? max);
	}
	public unsafe partial struct TrackPosition : IEquatable<TrackPosition>, IComparable<TrackPosition> {
		public bool Equals(TrackPosition other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackPosition a, TrackPosition b);
		public static bool operator !=(TrackPosition a, TrackPosition b);
		public int? PartialCompareTo(TrackPosition other);
		public static bool operator <(TrackPosition a, TrackPosition b);
		public static bool operator <=(TrackPosition a, TrackPosition b);
		public static bool operator >(TrackPosition a, TrackPosition b);
		public static bool operator >=(TrackPosition a, TrackPosition b);
		public int CompareTo(TrackPosition other);
		public string Debug();
		public static TrackPosition Default();
		public ulong Group { get; set; }
		public ulong Frame { get; set; }
		public static TrackPosition New(ulong group, ulong frame);
		public static TrackPosition Group_(ulong group);
	}
	public unsafe partial struct TrackRange : IEquatable<TrackRange>, IComparable<TrackRange> {
		public TrackAnchor Start;
		public TrackAnchor? End;
		public TrackRange(TrackAnchor start, TrackAnchor? end);
		public bool Equals(TrackRange other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackRange a, TrackRange b);
		public static bool operator !=(TrackRange a, TrackRange b);
		public int? PartialCompareTo(TrackRange other);
		public static bool operator <(TrackRange a, TrackRange b);
		public static bool operator <=(TrackRange a, TrackRange b);
		public static bool operator >(TrackRange a, TrackRange b);
		public static bool operator >=(TrackRange a, TrackRange b);
		public int CompareTo(TrackRange other);
		public string Debug();
		public static TrackRange Live();
		public static TrackRange Oldest();
	}
	public unsafe partial struct OriginCost : IEquatable<OriginCost>, IComparable<OriginCost> {
		public bool Equals(OriginCost other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(OriginCost a, OriginCost b);
		public static bool operator !=(OriginCost a, OriginCost b);
		public int? PartialCompareTo(OriginCost other);
		public static bool operator <(OriginCost a, OriginCost b);
		public static bool operator <=(OriginCost a, OriginCost b);
		public static bool operator >(OriginCost a, OriginCost b);
		public static bool operator >=(OriginCost a, OriginCost b);
		public int CompareTo(OriginCost other);
		public string Debug();
		public static OriginCost Default();
		public static OriginCost New(ulong cost);
		public ulong Value();
	}
	public unsafe partial struct OriginPriority : IEquatable<OriginPriority>, IComparable<OriginPriority> {
		public bool Equals(OriginPriority other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(OriginPriority a, OriginPriority b);
		public static bool operator !=(OriginPriority a, OriginPriority b);
		public int? PartialCompareTo(OriginPriority other);
		public static bool operator <(OriginPriority a, OriginPriority b);
		public static bool operator <=(OriginPriority a, OriginPriority b);
		public static bool operator >(OriginPriority a, OriginPriority b);
		public static bool operator >=(OriginPriority a, OriginPriority b);
		public int CompareTo(OriginPriority other);
		public string Debug();
		public static OriginPriority Default();
		public static OriginPriority New(sbyte priority);
		public sbyte Value();
	}
	public unsafe partial struct HangCatalogAac : IEquatable<HangCatalogAac> {
		public byte Profile;
		public HangCatalogAac(byte profile);
		public bool Equals(HangCatalogAac other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAac a, HangCatalogAac b);
		public static bool operator !=(HangCatalogAac a, HangCatalogAac b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct HangCatalogClock : IEquatable<HangCatalogClock> {
		public bool Equals(HangCatalogClock other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogClock a, HangCatalogClock b);
		public static bool operator !=(HangCatalogClock a, HangCatalogClock b);
		public string Debug();
		public Timestamp Wall { get; set; }
		public static HangCatalogClock New(Timestamp wall);
		public ClockWall WallClock(Timestamp pts);
	}
	public unsafe partial struct HangCatalogPriorities : IEquatable<HangCatalogPriorities> {
		public bool Equals(HangCatalogPriorities other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogPriorities a, HangCatalogPriorities b);
		public static bool operator !=(HangCatalogPriorities a, HangCatalogPriorities b);
		public string Debug();
		public sbyte Catalog { get; set; }
		public sbyte Text { get; set; }
		public sbyte Audio { get; set; }
		public sbyte Video { get; set; }
	}
	public unsafe partial struct HangCatalogAv1 : IEquatable<HangCatalogAv1> {
		public byte Profile;
		public byte Level;
		public Scalar Tier;
		public byte Bitdepth;
		public bool MonoChrome;
		public bool ChromaSubsamplingX;
		public bool ChromaSubsamplingY;
		public byte ChromaSamplePosition;
		public byte ColorPrimaries;
		public byte TransferCharacteristics;
		public byte MatrixCoefficients;
		public bool FullRange;
		public HangCatalogAv1(byte profile, byte level, Scalar tier, byte bitdepth, bool monoChrome, bool chromaSubsamplingX, bool chromaSubsamplingY, byte chromaSamplePosition, byte colorPrimaries, byte transferCharacteristics, byte matrixCoefficients, bool fullRange);
		public bool Equals(HangCatalogAv1 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAv1 a, HangCatalogAv1 b);
		public static bool operator !=(HangCatalogAv1 a, HangCatalogAv1 b);
		public override string ToString();
		public string Debug();
		public static HangCatalogAv1 Default();
		public static Str PREFIX { get; }
	}
	public unsafe partial struct HangCatalogMastering : IEquatable<HangCatalogMastering> {
		public HangCatalogChromaticity Red;
		public HangCatalogChromaticity Green;
		public HangCatalogChromaticity Blue;
		public HangCatalogChromaticity White;
		public double MaxLuminance;
		public double MinLuminance;
		public HangCatalogMastering(HangCatalogChromaticity red, HangCatalogChromaticity green, HangCatalogChromaticity blue, HangCatalogChromaticity white, double maxLuminance, double minLuminance);
		public bool Equals(HangCatalogMastering other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogMastering a, HangCatalogMastering b);
		public static bool operator !=(HangCatalogMastering a, HangCatalogMastering b);
		public string Debug();
	}
	public unsafe partial struct HangCatalogChromaticity : IEquatable<HangCatalogChromaticity> {
		public double X;
		public double Y;
		public HangCatalogChromaticity(double x, double y);
		public bool Equals(HangCatalogChromaticity other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogChromaticity a, HangCatalogChromaticity b);
		public static bool operator !=(HangCatalogChromaticity a, HangCatalogChromaticity b);
		public string Debug();
	}
	public unsafe partial struct HangCatalogContentLight : IEquatable<HangCatalogContentLight> {
		public ushort MaxCll;
		public ushort MaxFall;
		public HangCatalogContentLight(ushort maxCll, ushort maxFall);
		public bool Equals(HangCatalogContentLight other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogContentLight a, HangCatalogContentLight b);
		public static bool operator !=(HangCatalogContentLight a, HangCatalogContentLight b);
		public string Debug();
	}
	public unsafe partial struct HangCatalogH264 : IEquatable<HangCatalogH264> {
		public bool Inline;
		public byte Profile;
		public byte Constraints;
		public byte Level;
		public HangCatalogH264(bool inline, byte profile, byte constraints, byte level);
		public bool Equals(HangCatalogH264 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogH264 a, HangCatalogH264 b);
		public static bool operator !=(HangCatalogH264 a, HangCatalogH264 b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct HangCatalogH265 : IEquatable<HangCatalogH265> {
		public bool InBand;
		public byte ProfileSpace;
		public byte ProfileIdc;
		public ArrayU8x4 ProfileCompatibilityFlags;
		public bool TierFlag;
		public byte LevelIdc;
		public ArrayU8x6 ConstraintFlags;
		public HangCatalogH265(bool inBand, byte profileSpace, byte profileIdc, ArrayU8x4 profileCompatibilityFlags, bool tierFlag, byte levelIdc, ArrayU8x6 constraintFlags);
		public bool Equals(HangCatalogH265 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogH265 a, HangCatalogH265 b);
		public static bool operator !=(HangCatalogH265 a, HangCatalogH265 b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct HangCatalogVp9 : IEquatable<HangCatalogVp9> {
		public byte Profile;
		public byte Level;
		public byte BitDepth;
		public byte ChromaSubsampling;
		public byte ColorPrimaries;
		public byte TransferCharacteristics;
		public byte MatrixCoefficients;
		public bool FullRange;
		public HangCatalogVp9(byte profile, byte level, byte bitDepth, byte chromaSubsampling, byte colorPrimaries, byte transferCharacteristics, byte matrixCoefficients, bool fullRange);
		public bool Equals(HangCatalogVp9 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogVp9 a, HangCatalogVp9 b);
		public static bool operator !=(HangCatalogVp9 a, HangCatalogVp9 b);
		public override string ToString();
		public string Debug();
		public static HangCatalogVp9 Default();
	}
	public unsafe partial struct HangCatalogDisplay : IEquatable<HangCatalogDisplay> {
		public uint Width;
		public uint Height;
		public HangCatalogDisplay(uint width, uint height);
		public bool Equals(HangCatalogDisplay other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogDisplay a, HangCatalogDisplay b);
		public static bool operator !=(HangCatalogDisplay a, HangCatalogDisplay b);
		public string Debug();
	}
	public unsafe partial struct HangContainerFrame {
		public Timestamp Timestamp;
		public Bytes Payload;
		public HangContainerFrame(Timestamp timestamp, Bytes payload);
		public string Debug();
		public void Encode(ref Bytes buf);
		public static HangContainerFrame Decode(ReadOnlySpan<byte> buf);
		public void WriteTo(GroupProducer group);
	}
	public unsafe partial struct MuxContainerFrame {
		public Timestamp Timestamp;
		public Timestamp? Duration;
		public Bytes Payload;
		public bool Keyframe;
		public MuxContainerFrame(Timestamp timestamp, Timestamp? duration, Bytes payload, bool keyframe);
		public string Debug();
	}
	public unsafe partial struct MuxContainerTimestampRewind : IEquatable<MuxContainerTimestampRewind> {
		public Timestamp Timestamp;
		public Timestamp Floor;
		public MuxContainerTimestampRewind(Timestamp timestamp, Timestamp floor);
		public bool Equals(MuxContainerTimestampRewind other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTimestampRewind a, MuxContainerTimestampRewind b);
		public static bool operator !=(MuxContainerTimestampRewind a, MuxContainerTimestampRewind b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct MuxContainerPlayout : IEquatable<MuxContainerPlayout> {
		public TrackDelay Delay;
		public Duration Warmup;
		public MuxContainerPlayout(TrackDelay delay, Duration warmup);
		public bool Equals(MuxContainerPlayout other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerPlayout a, MuxContainerPlayout b);
		public static bool operator !=(MuxContainerPlayout a, MuxContainerPlayout b);
		public string Debug();
		public static MuxContainerPlayout Default();
		public static MuxContainerPlayout New(TrackDelay delay, Duration warmup);
		public TrackSubscription Subscription(TrackSubscription subscription);
		public MuxContainerConsumerContainer ConsumerContainer(TrackSubscriber track, MuxCatalogHangContainer format);
		public MuxContainerConsumerFmp4Wire ConsumerFmp4Wire(TrackSubscriber track, MuxContainerFmp4Wire format);
		public MuxContainerConsumerLegacyWire ConsumerLegacyWire(TrackSubscriber track, MuxContainerLegacyWire format);
		public MuxContainerConsumerLocWire ConsumerLocWire(TrackSubscriber track, MuxContainerLocWire format);
	}
	public unsafe partial struct MuxCodecFlacConfig : IEquatable<MuxCodecFlacConfig> {
		public ushort MinBlockSize;
		public ushort MaxBlockSize;
		public uint MinFrameSize;
		public uint MaxFrameSize;
		public uint SampleRate;
		public uint ChannelCount;
		public uint BitsPerSample;
		public ulong TotalSamples;
		public ArrayU8x16 Md5;
		public MuxCodecFlacConfig(ushort minBlockSize, ushort maxBlockSize, uint minFrameSize, uint maxFrameSize, uint sampleRate, uint channelCount, uint bitsPerSample, ulong totalSamples, ArrayU8x16 md5);
		public bool Equals(MuxCodecFlacConfig other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCodecFlacConfig a, MuxCodecFlacConfig b);
		public static bool operator !=(MuxCodecFlacConfig a, MuxCodecFlacConfig b);
		public string Debug();
		public static MuxCodecFlacConfig ParseStreamInfo(ref Bytes buf);
		public static MuxCodecFlacConfig Parse(ref Bytes buf);
		public Bytes EncodeStreamInfo();
		public Bytes Description();
	}
	public unsafe partial struct MuxCodecH264Sps {
		public byte Profile;
		public byte Constraints;
		public byte Level;
		public uint CodedWidth;
		public uint CodedHeight;
		public MuxCodecH264Sps(byte profile, byte constraints, byte level, uint codedWidth, uint codedHeight);
		public string Debug();
		public static MuxCodecH264Sps Parse(ReadOnlySpan<byte> nal);
	}
	public unsafe partial struct MuxCodecH265VideoSignal : IEquatable<MuxCodecH265VideoSignal> {
		public byte Primaries;
		public byte Transfer;
		public byte Matrix;
		public bool FullRange;
		public MuxCodecH265VideoSignal(byte primaries, byte transfer, byte matrix, bool fullRange);
		public bool Equals(MuxCodecH265VideoSignal other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCodecH265VideoSignal a, MuxCodecH265VideoSignal b);
		public static bool operator !=(MuxCodecH265VideoSignal a, MuxCodecH265VideoSignal b);
		public string Debug();
	}
	public unsafe partial struct MuxCodecMp3Config {
		public uint SampleRate;
		public uint ChannelCount;
		public MuxCodecMp3Config(uint sampleRate, uint channelCount);
		public static MuxCodecMp3Config Parse(ReadOnlySpan<byte> data);
	}
	public unsafe partial struct MuxCodecOpusMapping : IEquatable<MuxCodecOpusMapping> {
		public bool Equals(MuxCodecOpusMapping other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCodecOpusMapping a, MuxCodecOpusMapping b);
		public static bool operator !=(MuxCodecOpusMapping a, MuxCodecOpusMapping b);
		public string Debug();
		public byte Family();
		public byte Streams_();
		public byte Coupled();
		public Bytes Table();
	}
	public unsafe partial struct MuxContainerFmp4Fragment {
		public Bytes Data;
		public bool Independent;
		public Duration Duration;
		public MuxContainerFmp4Fragment(Bytes data, bool independent, Duration duration);
		public string Debug();
	}
	public unsafe partial struct MuxContainerLegacyWire {
		public MuxContainerKind _0;
		public MuxContainerLegacyWire(MuxContainerKind _0);
	}
	public unsafe partial struct MuxContainerLocWire {
		public MuxContainerKind _0;
		public MuxContainerLocWire(MuxContainerKind _0);
	}
	public unsafe partial struct MuxContainerTsDescriptor : IEquatable<MuxContainerTsDescriptor> {
		public byte Tag;
		public Bytes Data;
		public MuxContainerTsDescriptor(byte tag, Bytes data);
		public bool Equals(MuxContainerTsDescriptor other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsDescriptor a, MuxContainerTsDescriptor b);
		public static bool operator !=(MuxContainerTsDescriptor a, MuxContainerTsDescriptor b);
		public string Debug();
	}
	public unsafe partial struct RustlsClientTicketRequest : IEquatable<RustlsClientTicketRequest> {
		public byte NewSessionCount;
		public byte ResumptionCount;
		public RustlsClientTicketRequest(byte newSessionCount, byte resumptionCount);
		public bool Equals(RustlsClientTicketRequest other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RustlsClientTicketRequest a, RustlsClientTicketRequest b);
		public static bool operator !=(RustlsClientTicketRequest a, RustlsClientTicketRequest b);
		public string Debug();
	}
	public unsafe partial struct RustlsCryptoHpkeHpkePublicKey {
		public Bytes _0;
		public RustlsCryptoHpkeHpkePublicKey(Bytes _0);
		public string Debug();
	}
	public enum EncodeError {
		BoundsExceeded = 0,
		TooLarge = 1,
		Short = 2,
		InvalidState = 3,
		TooMany = 4,
		Version = 5,
		Unsupported = 6,
	}
	public static unsafe partial class EncodeErrorExtensions {
		public static string Display(this EncodeError self);
		public static string Debug(this EncodeError self);
	}
	public abstract unsafe partial class InvalidPattern : IEquatable<InvalidPattern> {
		public sealed class EmptySegment : InvalidPattern {
		}
		public sealed class InvalidSegment : InvalidPattern {
			public Str _0;
			public InvalidSegment(Str _0);
		}
		public sealed class MultipleGlobstars : InvalidPattern {
		}
		public sealed class TooManySegments : InvalidPattern {
		}
		public bool Equals(InvalidPattern? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(InvalidPattern? a, InvalidPattern? b);
		public static bool operator !=(InvalidPattern? a, InvalidPattern? b);
		public override string ToString();
		public string Debug();
	}
	public abstract unsafe partial class SessionError : IEquatable<SessionError> {
		public sealed class Cancel : SessionError {
		}
		public sealed class Internal : SessionError {
		}
		public sealed class Unauthorized : SessionError {
		}
		public sealed class Expired : SessionError {
		}
		public sealed class ProtocolViolation : SessionError {
		}
		public sealed class TooManyRequests : SessionError {
		}
		public sealed class GoawayTimeout : SessionError {
		}
		public sealed class Timeout : SessionError {
		}
		public sealed class Version : SessionError {
		}
		public sealed class MalformedAuthToken : SessionError {
		}
		public sealed class App : SessionError {
			public ushort _0;
			public App(ushort _0);
		}
		public sealed class Unknown : SessionError {
			public uint _0;
			public Unknown(uint _0);
		}
		public sealed class Unsupported : SessionError {
		}
		public bool Equals(SessionError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(SessionError? a, SessionError? b);
		public static bool operator !=(SessionError? a, SessionError? b);
		public override string ToString();
		public string Debug();
		public uint ToCode();
		public static SessionError FromCode(uint code);
	}
	public abstract unsafe partial class StreamError : IEquatable<StreamError> {
		public sealed class Session : StreamError {
			public SessionError _0;
			public Session(SessionError _0);
		}
		public sealed class Internal : StreamError {
		}
		public sealed class Cancel : StreamError {
		}
		public sealed class DeliveryTimeout : StreamError {
		}
		public sealed class ControlTimeout : StreamError {
		}
		public sealed class GoingAway : StreamError {
		}
		public sealed class TooFarBehind : StreamError {
		}
		public sealed class MalformedTrack : StreamError {
		}
		public sealed class NotFound : StreamError {
		}
		public sealed class Unroutable : StreamError {
		}
		public sealed class Old : StreamError {
		}
		public sealed class Evicted : StreamError {
		}
		public sealed class WrongSize : StreamError {
		}
		public sealed class FrameTooLarge : StreamError {
		}
		public sealed class GroupTooLarge : StreamError {
		}
		public sealed class TimestampMismatch : StreamError {
		}
		public sealed class Unauthorized : StreamError {
		}
		public sealed class App : StreamError {
			public ushort _0;
			public App(ushort _0);
		}
		public sealed class Unknown : StreamError {
			public uint _0;
			public Unknown(uint _0);
		}
		public bool Equals(StreamError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StreamError? a, StreamError? b);
		public static bool operator !=(StreamError? a, StreamError? b);
		public override string ToString();
		public string Debug();
		public uint ToCode();
		public static StreamError FromCode(uint code);
	}
	public enum Retry {
		Same = 0,
		Reroute = 1,
		Never = 2,
	}
	public static unsafe partial class RetryExtensions {
		public static string Debug(this Retry self);
	}
	public enum IntersectionError {
		TooManyPatterns = 0,
	}
	public static unsafe partial class IntersectionErrorExtensions {
		public static string Display(this IntersectionError self);
		public static string Debug(this IntersectionError self);
	}
	public abstract unsafe partial class Segment : IEquatable<Segment> {
		public sealed class Literal : Segment {
			public Str _0;
			public Literal(Str _0);
		}
		public sealed class Wildcard : Segment {
		}
		public sealed class Partial : Segment {
			public Str Prefix;
			public Str Suffix;
			public Partial(Str prefix, Str suffix);
		}
		public sealed class Globstar : Segment {
		}
		public bool Equals(Segment? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Segment? a, Segment? b);
		public static bool operator !=(Segment? a, Segment? b);
		public override string ToString();
		public string Debug();
	}
	public enum ClockMissedTickBehavior {
		Burst = 0,
		Delay = 1,
		Skip = 2,
	}
	public static unsafe partial class ClockMissedTickBehaviorExtensions {
		public static string Debug(this ClockMissedTickBehavior self);
		public static ClockMissedTickBehavior Default();
	}
	public enum TokioConnectError {
		Unauthorized = 0,
		Forbidden = 1,
	}
	public static unsafe partial class TokioConnectErrorExtensions {
		public static string Display(this TokioConnectError self);
		public static string Debug(this TokioConnectError self);
		public static bool IsAuth(this TokioConnectError self);
	}
	public enum TokioRestart {
		Redial = 0,
		Handoff = 1,
	}
	public static unsafe partial class TokioRestartExtensions {
		public static string Debug(this TokioRestart self);
		public static TokioRestart Default();
	}
	public enum TokioStatus {
		Connected = 0,
		Disconnected = 1,
		Migrating = 2,
	}
	public static unsafe partial class TokioStatusExtensions {
		public static string Debug(this TokioStatus self);
	}
	public enum TokioTransport {
		Quic = 0,
		WebSocket = 1,
		Tcp = 2,
		Unix = 3,
		WebTransport = 4,
		Tls = 5,
	}
	public static unsafe partial class TokioTransportExtensions {
		public static string Display(this TokioTransport self);
		public static string Debug(this TokioTransport self);
		public static Str AsStr(this TokioTransport self);
	}
	public enum PathError {
		TooLong = 0,
		EmptySegment = 1,
		InvalidEscape = 2,
	}
	public static unsafe partial class PathErrorExtensions {
		public static string Display(this PathError self);
		public static string Debug(this PathError self);
	}
	public enum SetupRole {
		Publisher = 0,
		Subscriber = 1,
	}
	public static unsafe partial class SetupRoleExtensions {
		public static string Display(this SetupRole self);
		public static string Debug(this SetupRole self);
		public static Str AsStr(this SetupRole self);
	}
	public enum ReconnectVerdict {
		Retry = 0,
		Unusable = 1,
		Settled = 2,
	}
	public static unsafe partial class ReconnectVerdictExtensions {
		public static string Debug(this ReconnectVerdict self);
	}
	public enum ReconnectConfigError {
		Empty = 0,
		Healthy = 1,
	}
	public static unsafe partial class ReconnectConfigErrorExtensions {
		public static string Display(this ReconnectConfigError self);
		public static string Debug(this ReconnectConfigError self);
	}
	public enum ReconnectRedirect {
		Follow = 0,
		SameHost = 1,
		Ignore = 2,
	}
	public static unsafe partial class ReconnectRedirectExtensions {
		public static string Debug(this ReconnectRedirect self);
		public static ReconnectRedirect Default();
	}
	public enum StatsRole {
		Publisher = 0,
		Subscriber = 1,
	}
	public static unsafe partial class StatsRoleExtensions {
		public static string Debug(this StatsRole self);
		public static Str AsStr(this StatsRole self);
	}
	public enum StatsMessage {
		Session = 0,
		Announce = 1,
		TrackInfo = 2,
		Subscribe = 3,
		Fetch = 4,
		Probe = 5,
		Auth = 6,
		Goaway = 7,
		Other = 8,
	}
	public static unsafe partial class StatsMessageExtensions {
		public static string Debug(this StatsMessage self);
		public static Str AsStr(this StatsMessage self);
	}
	public enum StatsFailure {
		IdleTimeout = 0,
		Transport = 1,
		Application = 2,
		Expired = 3,
	}
	public static unsafe partial class StatsFailureExtensions {
		public static string Debug(this StatsFailure self);
		public static StatsFailure[] ALL { get; }
		public static Str AsStr(this StatsFailure self);
	}
	public enum StatsRefusalKind {
		Refused = 0,
		Unavailable = 1,
		Request = 2,
		Forbidden = 3,
		Membership = 4,
		Expired = 5,
	}
	public static unsafe partial class StatsRefusalKindExtensions {
		public static string Debug(this StatsRefusalKind self);
		public static StatsRefusalKind[] ALL { get; }
		public static Str AsStr(this StatsRefusalKind self);
	}
	public enum StatsRouteClose {
		Stalled = 0,
		Overrun = 1,
		Malformed = 2,
		Role = 3,
		Unasked = 4,
		Regressed = 5,
		Other = 6,
	}
	public static unsafe partial class StatsRouteCloseExtensions {
		public static string Debug(this StatsRouteClose self);
		public static StatsRouteClose[] ALL { get; }
		public static Str AsStr(this StatsRouteClose self);
	}
	public enum StatsRouteRefusal {
		Path = 0,
		Scope = 1,
		Looped = 2,
	}
	public static unsafe partial class StatsRouteRefusalExtensions {
		public static string Debug(this StatsRouteRefusal self);
		public static StatsRouteRefusal[] ALL { get; }
		public static Str AsStr(this StatsRouteRefusal self);
	}
	public enum StatsWithdrawal {
		Dead = 0,
		Drain = 1,
	}
	public static unsafe partial class StatsWithdrawalExtensions {
		public static string Debug(this StatsWithdrawal self);
		public static StatsWithdrawal[] ALL { get; }
		public static Str AsStr(this StatsWithdrawal self);
	}
	public enum TransportAddress {
		Uri = 0,
		Setup = 1,
	}
	public static unsafe partial class TransportAddressExtensions {
		public static string Debug(this TransportAddress self);
	}
	public abstract unsafe partial class TransportLimitsError : IEquatable<TransportLimitsError> {
		public sealed class Unsupported : TransportLimitsError {
		}
		public sealed class Refused : TransportLimitsError {
			public Str _0;
			public Refused(Str _0);
		}
		public bool Equals(TransportLimitsError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TransportLimitsError? a, TransportLimitsError? b);
		public static bool operator !=(TransportLimitsError? a, TransportLimitsError? b);
		public override string ToString();
		public string Debug();
	}
	public enum TransportCloseKind {
		Transport = 0,
		IdleTimeout = 1,
		Closed = 2,
		Evicted = 3,
	}
	public static unsafe partial class TransportCloseKindExtensions {
		public static string Debug(this TransportCloseKind self);
		public static TransportCloseKind Default();
	}
	public enum BroadcastKind {
		Track = 0,
		Datagrams = 1,
	}
	public static unsafe partial class BroadcastKindExtensions {
		public static string Debug(this BroadcastKind self);
	}
	public enum TrackCompression {
		None = 0,
		Deflate = 1,
	}
	public static unsafe partial class TrackCompressionExtensions {
		public static string Debug(this TrackCompression self);
		public static TrackCompression Default();
	}
	public abstract unsafe partial class TrackAnchor : IEquatable<TrackAnchor>, IComparable<TrackAnchor> {
		public sealed class Relative : TrackAnchor {
			public ulong _0;
			public Relative(ulong _0);
		}
		public sealed class Oldest : TrackAnchor {
		}
		public sealed class Time : TrackAnchor {
			public ulong _0;
			public Time(ulong _0);
		}
		public bool Equals(TrackAnchor? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackAnchor? a, TrackAnchor? b);
		public static bool operator !=(TrackAnchor? a, TrackAnchor? b);
		public int? PartialCompareTo(TrackAnchor other);
		public static bool operator <(TrackAnchor a, TrackAnchor b);
		public static bool operator <=(TrackAnchor a, TrackAnchor b);
		public static bool operator >(TrackAnchor a, TrackAnchor b);
		public static bool operator >=(TrackAnchor a, TrackAnchor b);
		public int CompareTo(TrackAnchor? other);
		public string Debug();
	}
	public enum TrackOrder {
		Ascending = 0,
		Descending = 1,
	}
	public static unsafe partial class TrackOrderExtensions {
		public static string Debug(this TrackOrder self);
		public static TrackOrder Default();
	}
	public enum OriginLink {
		Local = 0,
		Peer = 1,
		Upstream = 2,
	}
	public static unsafe partial class OriginLinkExtensions {
		public static string Debug(this OriginLink self);
		public static OriginLink Default();
	}
	public enum OriginRouteKind {
		Start = 0,
		Update = 1,
		End = 2,
		Offline = 3,
		Restart = 4,
	}
	public static unsafe partial class OriginRouteKindExtensions {
		public static string Debug(this OriginRouteKind self);
	}
	public abstract unsafe partial class HangCatalogAudioCodec : IEquatable<HangCatalogAudioCodec> {
		public sealed class Aac : HangCatalogAudioCodec {
			public HangCatalogAac _0;
			public Aac(HangCatalogAac _0);
		}
		public sealed class Opus : HangCatalogAudioCodec {
		}
		public sealed class Pcm : HangCatalogAudioCodec {
		}
		public sealed class Flac : HangCatalogAudioCodec {
		}
		public sealed class Mp3 : HangCatalogAudioCodec {
		}
		public sealed class Mp2 : HangCatalogAudioCodec {
		}
		public sealed class Ac3 : HangCatalogAudioCodec {
		}
		public sealed class Ec3 : HangCatalogAudioCodec {
		}
		public sealed class Unknown : HangCatalogAudioCodec {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogAudioCodec? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAudioCodec? a, HangCatalogAudioCodec? b);
		public static bool operator !=(HangCatalogAudioCodec? a, HangCatalogAudioCodec? b);
		public override string ToString();
		public string Debug();
		public HangCatalogAudioCodecKind Kind();
	}
	public enum HangCatalogAudioCodecKind {
		Aac = 0,
		Opus = 1,
		Unknown = 2,
		Flac = 3,
		Mp3 = 4,
		Pcm = 5,
		Ac3 = 6,
		Ec3 = 7,
	}
	public static unsafe partial class HangCatalogAudioCodecKindExtensions {
		public static string Debug(this HangCatalogAudioCodecKind self);
	}
	public abstract unsafe partial class HangCatalogAudioLayout : IEquatable<HangCatalogAudioLayout> {
		public sealed class Ambisonic : HangCatalogAudioLayout {
		}
		public sealed class Unknown : HangCatalogAudioLayout {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogAudioLayout? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAudioLayout? a, HangCatalogAudioLayout? b);
		public static bool operator !=(HangCatalogAudioLayout? a, HangCatalogAudioLayout? b);
		public override string ToString();
		public string Debug();
	}
	public abstract unsafe partial class HangCatalogMode : IEquatable<HangCatalogMode> {
		public sealed class Snapshot : HangCatalogMode {
		}
		public sealed class Stream_ : HangCatalogMode {
		}
		public sealed class Unknown : HangCatalogMode {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogMode? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogMode? a, HangCatalogMode? b);
		public static bool operator !=(HangCatalogMode? a, HangCatalogMode? b);
		public override string ToString();
		public string Debug();
	}
	public abstract unsafe partial class HangCatalogTextFormat : IEquatable<HangCatalogTextFormat> {
		public sealed class Vtt : HangCatalogTextFormat {
		}
		public sealed class Ttml : HangCatalogTextFormat {
		}
		public sealed class Utf8 : HangCatalogTextFormat {
		}
		public sealed class Unknown : HangCatalogTextFormat {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogTextFormat? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogTextFormat? a, HangCatalogTextFormat? b);
		public static bool operator !=(HangCatalogTextFormat? a, HangCatalogTextFormat? b);
		public override string ToString();
		public string Debug();
		public static HangCatalogTextFormat Default();
	}
	public abstract unsafe partial class HangCatalogTextRole : IEquatable<HangCatalogTextRole> {
		public sealed class Subtitle : HangCatalogTextRole {
		}
		public sealed class Caption : HangCatalogTextRole {
		}
		public sealed class Unknown : HangCatalogTextRole {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogTextRole? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogTextRole? a, HangCatalogTextRole? b);
		public static bool operator !=(HangCatalogTextRole? a, HangCatalogTextRole? b);
		public override string ToString();
		public string Debug();
		public static HangCatalogTextRole Default();
	}
	public abstract unsafe partial class HangCatalogVideoCodec : IEquatable<HangCatalogVideoCodec> {
		public sealed class H264 : HangCatalogVideoCodec {
			public HangCatalogH264 _0;
			public H264(HangCatalogH264 _0);
		}
		public sealed class H265 : HangCatalogVideoCodec {
			public HangCatalogH265 _0;
			public H265(HangCatalogH265 _0);
		}
		public sealed class Vp9 : HangCatalogVideoCodec {
			public HangCatalogVp9 _0;
			public Vp9(HangCatalogVp9 _0);
		}
		public sealed class Av1 : HangCatalogVideoCodec {
			public HangCatalogAv1 _0;
			public Av1(HangCatalogAv1 _0);
		}
		public sealed class Vp8 : HangCatalogVideoCodec {
		}
		public sealed class Unknown : HangCatalogVideoCodec {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogVideoCodec? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogVideoCodec? a, HangCatalogVideoCodec? b);
		public static bool operator !=(HangCatalogVideoCodec? a, HangCatalogVideoCodec? b);
		public override string ToString();
		public string Debug();
		public HangCatalogVideoCodecKind Kind();
	}
	public enum HangCatalogVideoCodecKind {
		H264 = 0,
		H265 = 1,
		Vp8 = 2,
		Vp9 = 3,
		Av1 = 4,
		Unknown = 5,
	}
	public static unsafe partial class HangCatalogVideoCodecKindExtensions {
		public static string Debug(this HangCatalogVideoCodecKind self);
	}
	public enum HangCatalogColorRange {
		Limited = 0,
		Full = 1,
	}
	public static unsafe partial class HangCatalogColorRangeExtensions {
		public static string Debug(this HangCatalogColorRange self);
	}
	public abstract unsafe partial class HangCatalogStereo : IEquatable<HangCatalogStereo> {
		public sealed class Mono : HangCatalogStereo {
		}
		public sealed class LeftRight : HangCatalogStereo {
		}
		public sealed class TopBottom : HangCatalogStereo {
		}
		public sealed class Unknown : HangCatalogStereo {
			public Str _0;
			public Unknown(Str _0);
		}
		public bool Equals(HangCatalogStereo? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogStereo? a, HangCatalogStereo? b);
		public static bool operator !=(HangCatalogStereo? a, HangCatalogStereo? b);
		public override string ToString();
		public string Debug();
	}
	public enum MuxCatalogCatalogFormat {
		Hang = 0,
		Msf = 1,
	}
	public static unsafe partial class MuxCatalogCatalogFormatExtensions {
		public static string Debug(this MuxCatalogCatalogFormat self);
		public static MuxCatalogCatalogFormat Default();
		public static MuxCatalogCatalogFormat DEFAULT { get; }
		public static Str Extension(this MuxCatalogCatalogFormat self);
		public static MuxCatalogCatalogFormat? Detect(Str name);
	}
	public enum MuxContainerKind {
		Audio = 0,
		Video = 1,
		Data = 2,
	}
	public static unsafe partial class MuxContainerKindExtensions {
		public static string Debug(this MuxContainerKind self);
	}
	public abstract unsafe partial class MuxContainerRead {
		public sealed class Frame : MuxContainerRead {
			public ulong Group;
			public MuxContainerFrame Frame_;
			public Frame(ulong group, MuxContainerFrame frame);
		}
		public sealed class Gap : MuxContainerRead {
			public Timestamp Timestamp;
			public Timestamp Duration;
			public Gap(Timestamp timestamp, Timestamp duration);
		}
		public sealed class End : MuxContainerRead {
			public ulong Group;
			public Timestamp End_;
			public End(ulong group, Timestamp end);
		}
		public string Debug();
	}
	public abstract unsafe partial class MuxContainerReorderError : IEquatable<MuxContainerReorderError> {
		public sealed class Depth : MuxContainerReorderError {
			public byte Reorder;
			public uint Max;
			public Str Codec;
			public Depth(byte reorder, uint max, Str codec);
		}
		public sealed class Deepened : MuxContainerReorderError {
			public byte Declared;
			public byte Reorder;
			public Deepened(byte declared, byte reorder);
		}
		public sealed class Reordered : MuxContainerReorderError {
			public nuint _0;
			public Reordered(nuint _0);
		}
		public sealed class Presentation : MuxContainerReorderError {
		}
		public sealed class Tie : MuxContainerReorderError {
		}
		public sealed class Undrained : MuxContainerReorderError {
		}
		public bool Equals(MuxContainerReorderError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerReorderError? a, MuxContainerReorderError? b);
		public static bool operator !=(MuxContainerReorderError? a, MuxContainerReorderError? b);
		public override string ToString();
		public string Debug();
	}
	public enum MuxImportAudioFormat {
		Aac = 0,
		Opus = 1,
		Flac = 2,
		Mp3 = 3,
	}
	public static unsafe partial class MuxImportAudioFormatExtensions {
		public static string Display(this MuxImportAudioFormat self);
		public static string Debug(this MuxImportAudioFormat self);
	}
	public enum MuxImportContainerFormat {
		Fmp4 = 0,
		Mkv = 1,
		Ts = 2,
		Flv = 3,
	}
	public static unsafe partial class MuxImportContainerFormatExtensions {
		public static string Display(this MuxImportContainerFormat self);
		public static string Debug(this MuxImportContainerFormat self);
	}
	public abstract unsafe partial class MuxImportFormat : IEquatable<MuxImportFormat> {
		public sealed class Audio : MuxImportFormat {
			public MuxImportAudioFormat _0;
			public Audio(MuxImportAudioFormat _0);
		}
		public sealed class Video : MuxImportFormat {
			public MuxImportVideoFormat _0;
			public Video(MuxImportVideoFormat _0);
		}
		public sealed class Container : MuxImportFormat {
			public MuxImportContainerFormat _0;
			public Container(MuxImportContainerFormat _0);
		}
		public bool Equals(MuxImportFormat? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxImportFormat? a, MuxImportFormat? b);
		public static bool operator !=(MuxImportFormat? a, MuxImportFormat? b);
		public override string ToString();
		public string Debug();
		public MuxImportKind Kind();
	}
	public enum MuxImportKind {
		Audio = 0,
		Video = 1,
		Container = 2,
	}
	public static unsafe partial class MuxImportKindExtensions {
		public static string Debug(this MuxImportKind self);
		public static Str Name(this MuxImportKind self);
	}
	public enum MuxImportVideoFormat {
		Avc1 = 0,
		Avc3 = 1,
		Hvc1 = 2,
		Hev1 = 3,
		Av01 = 4,
		Vp8 = 5,
		Vp9 = 6,
	}
	public static unsafe partial class MuxImportVideoFormatExtensions {
		public static string Display(this MuxImportVideoFormat self);
		public static string Debug(this MuxImportVideoFormat self);
	}
	public abstract unsafe partial class MuxRateTarget : IEquatable<MuxRateTarget> {
		public sealed class Enabled : MuxRateTarget {
			public BandwidthRate _0;
			public Enabled(BandwidthRate _0);
		}
		public sealed class Disabled : MuxRateTarget {
		}
		public bool Equals(MuxRateTarget? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxRateTarget? a, MuxRateTarget? b);
		public static bool operator !=(MuxRateTarget? a, MuxRateTarget? b);
		public string Debug();
	}
	public abstract unsafe partial class TokioListenBind : IEquatable<TokioListenBind> {
		public sealed class Addr : TokioListenBind {
			public SocketAddr _0;
			public Addr(SocketAddr _0);
		}
		public sealed class Host : TokioListenBind {
			public Str _0;
			public ushort _1;
			public Host(Str _0, ushort _1);
		}
		public bool Equals(TokioListenBind? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TokioListenBind? a, TokioListenBind? b);
		public static bool operator !=(TokioListenBind? a, TokioListenBind? b);
		public override string ToString();
		public string Debug();
		public SocketAddr Resolve();
	}
	public abstract unsafe partial class TransportMemoryError {
		public sealed class Session : TransportMemoryError {
			public uint Code;
			public Str Reason;
			public Session(uint code, Str reason);
		}
		public sealed class Reset : TransportMemoryError {
			public uint _0;
			public Reset(uint _0);
		}
		public sealed class Stopped : TransportMemoryError {
			public uint _0;
			public Stopped(uint _0);
		}
		public sealed class Closed : TransportMemoryError {
		}
		public override string ToString();
		public string Debug();
	}
	public enum TransportPollAcked {
		Delivered = 0,
		Unsupported = 1,
		Sent = 2,
	}
	public static unsafe partial class TransportPollAckedExtensions {
		public static string Debug(this TransportPollAcked self);
	}
	public abstract unsafe partial class MuxCatalogMsfError {
		public sealed class UnresolvableNamespace : MuxCatalogMsfError {
			public Str Name;
			public Str Namespace;
			public UnresolvableNamespace(Str name, Str @namespace);
		}
		public sealed class InvalidUtf8 : MuxCatalogMsfError {
		}
		public sealed class ParseFrame : MuxCatalogMsfError {
		}
		public sealed class MissingCmafInit : MuxCatalogMsfError {
			public Str _0;
			public MissingCmafInit(Str _0);
		}
		public sealed class MalformedInitData : MuxCatalogMsfError {
			public Str _0;
			public MalformedInitData(Str _0);
		}
		public sealed class MissingVideoCodec : MuxCatalogMsfError {
			public Str _0;
			public MissingVideoCodec(Str _0);
		}
		public sealed class MissingAudioCodec : MuxCatalogMsfError {
			public Str _0;
			public MissingAudioCodec(Str _0);
		}
		public sealed class UnsupportedPackaging : MuxCatalogMsfError {
			public Str Name;
			public Str Packaging;
			public UnsupportedPackaging(Str name, Str packaging);
		}
		public sealed class MissingTextCodec : MuxCatalogMsfError {
			public Str _0;
			public MissingTextCodec(Str _0);
		}
		public sealed class InvalidVideoCodec : MuxCatalogMsfError {
			public Str Name;
			public Str Codec;
			public InvalidVideoCodec(Str name, Str codec);
		}
		public sealed class InvalidAudioCodec : MuxCatalogMsfError {
			public Str Name;
			public Str Codec;
			public InvalidAudioCodec(Str name, Str codec);
		}
		public sealed class MissingAudioParams : MuxCatalogMsfError {
			public Str _0;
			public MissingAudioParams(Str _0);
		}
		public sealed class UnsupportedDerivationPackaging : MuxCatalogMsfError {
			public Str Name;
			public Str Packaging;
			public UnsupportedDerivationPackaging(Str name, Str packaging);
		}
		public sealed class MalformedAac : MuxCatalogMsfError {
			public Str _0;
			public MalformedAac(Str _0);
		}
		public sealed class MalformedOpus : MuxCatalogMsfError {
			public Str _0;
			public MalformedOpus(Str _0);
		}
		public sealed class MalformedFlac : MuxCatalogMsfError {
			public Str _0;
			public MalformedFlac(Str _0);
		}
		public sealed class OpusTrailingBytes : MuxCatalogMsfError {
			public Str _0;
			public OpusTrailingBytes(Str _0);
		}
		public sealed class UnsupportedDerivationCodec : MuxCatalogMsfError {
			public Str _0;
			public UnsupportedDerivationCodec(Str _0);
		}
		public sealed class MalformedInitSegment : MuxCatalogMsfError {
			public Str _0;
			public MalformedInitSegment(Str _0);
		}
		public sealed class MissingInitMoov : MuxCatalogMsfError {
			public Str _0;
			public MissingInitMoov(Str _0);
		}
		public sealed class MissingAudioSampleEntry : MuxCatalogMsfError {
			public Str _0;
			public MissingAudioSampleEntry(Str _0);
		}
		public override string ToString();
		public string Debug();
	}
	public enum MuxCodecFlacError {
		Short = 0,
		InvalidMarker = 1,
		MissingStreamInfo = 2,
	}
	public static unsafe partial class MuxCodecFlacErrorExtensions {
		public static string Display(this MuxCodecFlacError self);
		public static string Debug(this MuxCodecFlacError self);
	}
	public enum MuxCodecMp3Error {
		HeaderTooShort = 0,
		MissingSync = 1,
		ReservedVersion = 2,
		NotLayer3 = 3,
		ReservedSampleRate = 4,
	}
	public static unsafe partial class MuxCodecMp3ErrorExtensions {
		public static string Display(this MuxCodecMp3Error self);
		public static string Debug(this MuxCodecMp3Error self);
	}
	public abstract unsafe partial class MuxCodecOpusError {
		public sealed class HeadTooShort : MuxCodecOpusError {
		}
		public sealed class InvalidSignature : MuxCodecOpusError {
		}
		public sealed class UnsupportedVersion : MuxCodecOpusError {
			public byte _0;
			public UnsupportedVersion(byte _0);
		}
		public sealed class UnsupportedChannelCount : MuxCodecOpusError {
			public uint _0;
			public UnsupportedChannelCount(uint _0);
		}
		public sealed class MappingTableTooShort : MuxCodecOpusError {
		}
		public sealed class InvalidMappingTable : MuxCodecOpusError {
		}
		public sealed class UnsupportedMappingFamily : MuxCodecOpusError {
			public byte _0;
			public UnsupportedMappingFamily(byte _0);
		}
		public override string ToString();
		public string Debug();
	}
	public enum MuxCodecVp8Error {
		FrameTooShort = 0,
		KeyframeHeaderTooShort = 1,
		StartCodeMismatch = 2,
		EmptyFrame = 3,
	}
	public static unsafe partial class MuxCodecVp8ErrorExtensions {
		public static string Display(this MuxCodecVp8Error self);
		public static string Debug(this MuxCodecVp8Error self);
	}
	public enum MuxCodecVp9Error {
		InvalidFrameMarker = 0,
		InvalidSyncCode = 1,
		Truncated = 2,
		EmptyFrame = 3,
	}
	public static unsafe partial class MuxCodecVp9ErrorExtensions {
		public static string Display(this MuxCodecVp9Error self);
		public static string Debug(this MuxCodecVp9Error self);
	}
	public abstract unsafe partial class MuxContainerFmp4Chunk {
		public sealed class Init : MuxContainerFmp4Chunk {
			public Bytes _0;
			public Init(Bytes _0);
		}
		public sealed class Fragment : MuxContainerFmp4Chunk {
			public MuxContainerFmp4Fragment _0;
			public Fragment(MuxContainerFmp4Fragment _0);
		}
		public string Debug();
		public Bytes Data();
		public Bytes IntoData();
		public Bytes? Init_();
		public MuxContainerFmp4Fragment? Fragment_();
	}
	public enum MuxContainerTsFraming {
		Section = 0,
	}
	public static unsafe partial class MuxContainerTsFramingExtensions {
		public static string Debug(this MuxContainerTsFraming self);
		public static MuxContainerTsFraming Default();
	}
	public abstract unsafe partial class MuxContainerTsStep : IEquatable<MuxContainerTsStep> {
		public sealed class Rewind : MuxContainerTsStep {
			public Duration By;
			public bool Flagged;
			public Rewind(Duration by, bool flagged);
		}
		public sealed class Leap : MuxContainerTsStep {
			public Duration By;
			public Leap(Duration by);
		}
		public sealed class Identity : MuxContainerTsStep {
			public ushort TransportStreamId;
			public ushort ProgramNumber;
			public Identity(ushort transportStreamId, ushort programNumber);
		}
		public bool Equals(MuxContainerTsStep? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStep? a, MuxContainerTsStep? b);
		public static bool operator !=(MuxContainerTsStep? a, MuxContainerTsStep? b);
		public override string ToString();
		public string Debug();
	}
	public abstract unsafe partial class MuxCodecAacAdtsError : IEquatable<MuxCodecAacAdtsError> {
		public sealed class Truncated : MuxCodecAacAdtsError {
		}
		public sealed class Syncword : MuxCodecAacAdtsError {
		}
		public sealed class SampleRate : MuxCodecAacAdtsError {
		}
		public sealed class FrameLength : MuxCodecAacAdtsError {
		}
		public sealed class PacketLength : MuxCodecAacAdtsError {
		}
		public sealed class ObjectType : MuxCodecAacAdtsError {
			public byte _0;
			public ObjectType(byte _0);
		}
		public sealed class Rate : MuxCodecAacAdtsError {
			public uint _0;
			public Rate(uint _0);
		}
		public sealed class ChannelConfig : MuxCodecAacAdtsError {
			public byte _0;
			public ChannelConfig(byte _0);
		}
		public sealed class TooLarge : MuxCodecAacAdtsError {
			public nuint _0;
			public TooLarge(nuint _0);
		}
		public bool Equals(MuxCodecAacAdtsError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCodecAacAdtsError? a, MuxCodecAacAdtsError? b);
		public static bool operator !=(MuxCodecAacAdtsError? a, MuxCodecAacAdtsError? b);
		public override string ToString();
		public string Debug();
	}
	public enum MuxContainerTsStatsClass {
		Audio = 1,
		Video = 2,
		Data = 0,
	}
	public static unsafe partial class MuxContainerTsStatsClassExtensions {
		public static string Debug(this MuxContainerTsStatsClass self);
		public static MuxContainerTsStatsClass Default();
	}
	public enum MoqTokioQuicCongestionControl {
		Bbr = 0,
		Cubic = 1,
		Unlimited = 2,
	}
	public static unsafe partial class MoqTokioQuicCongestionControlExtensions {
		public static string Debug(this MoqTokioQuicCongestionControl self);
		public static MoqTokioQuicCongestionControl Default();
	}
	public enum MoqTokioAcceptFailure {
		Connection = 0,
		Exhausted = 1,
		Unknown = 2,
	}
	public static unsafe partial class MoqTokioAcceptFailureExtensions {
		public static string Display(this MoqTokioAcceptFailure self);
		public static string Debug(this MoqTokioAcceptFailure self);
		public static MoqTokioAcceptFailure[] ALL { get; }
		public static Str AsStr(this MoqTokioAcceptFailure self);
	}
	public enum MoqTokioRedirect {
		Follow = 0,
		SameHost = 1,
		Ignore = 2,
	}
	public static unsafe partial class MoqTokioRedirectExtensions {
		public static string Debug(this MoqTokioRedirect self);
		public static MoqTokioRedirect Default();
	}
	public abstract unsafe partial class MoqQuicRate : IEquatable<MoqQuicRate> {
		public sealed class Unlimited : MoqQuicRate {
		}
		public sealed class BytesPerSecond : MoqQuicRate {
			public NonZeroU64 _0;
			public BytesPerSecond(NonZeroU64 _0);
		}
		public bool Equals(MoqQuicRate? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqQuicRate? a, MoqQuicRate? b);
		public static bool operator !=(MoqQuicRate? a, MoqQuicRate? b);
		public string Debug();
	}
	public abstract unsafe partial class MoqTokioConnectError : IEquatable<MoqTokioConnectError> {
		public sealed class EmptyAddresses : MoqTokioConnectError {
		}
		public sealed class UnsupportedScheme : MoqTokioConnectError {
			public Str _0;
			public UnsupportedScheme(Str _0);
		}
		public sealed class MissingHost : MoqTokioConnectError {
		}
		public bool Equals(MoqTokioConnectError? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqTokioConnectError? a, MoqTokioConnectError? b);
		public static bool operator !=(MoqTokioConnectError? a, MoqTokioConnectError? b);
		public override string ToString();
		public string Debug();
	}
	public abstract unsafe partial class RustlsProtocolVersion : IEquatable<RustlsProtocolVersion> {
		public sealed class SsLv2 : RustlsProtocolVersion {
		}
		public sealed class SsLv3 : RustlsProtocolVersion {
		}
		public sealed class TlSv10 : RustlsProtocolVersion {
		}
		public sealed class TlSv11 : RustlsProtocolVersion {
		}
		public sealed class TlSv12 : RustlsProtocolVersion {
		}
		public sealed class TlSv13 : RustlsProtocolVersion {
		}
		public sealed class DtlSv10 : RustlsProtocolVersion {
		}
		public sealed class DtlSv12 : RustlsProtocolVersion {
		}
		public sealed class DtlSv13 : RustlsProtocolVersion {
		}
		public sealed class Unknown : RustlsProtocolVersion {
			public ushort _0;
			public Unknown(ushort _0);
		}
		public bool Equals(RustlsProtocolVersion? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RustlsProtocolVersion? a, RustlsProtocolVersion? b);
		public static bool operator !=(RustlsProtocolVersion? a, RustlsProtocolVersion? b);
		public string Debug();
		public ArrayU8x2 ToArray();
		public Str? AsStr();
	}
	public enum RustlsClientTls12Resumption {
		Disabled = 0,
		SessionIdOnly = 1,
		SessionIdOrTickets = 2,
	}
	public static unsafe partial class RustlsClientTls12ResumptionExtensions {
		public static string Debug(this RustlsClientTls12Resumption self);
	}
	public enum MoqQuicConnectionMuxVersion {
		QMux00 = 0,
		QMux01 = 1,
		QMux02 = 2,
	}
	public static unsafe partial class MoqQuicConnectionMuxVersionExtensions {
		public static string Debug(this MoqQuicConnectionMuxVersion self);
		public static MoqQuicConnectionMuxVersion[] ALL { get; }
		public static bool UsesRecords(this MoqQuicConnectionMuxVersion self);
		public static Str Alpn(this MoqQuicConnectionMuxVersion self);
		public static Str Prefix(this MoqQuicConnectionMuxVersion self);
	}
	public enum MoqSockDemuxDiscard {
		Stun = 0,
		Unknown = 1,
		Unclaimed = 2,
		Overflow = 3,
	}
	public static unsafe partial class MoqSockDemuxDiscardExtensions {
		public static string Debug(this MoqSockDemuxDiscard self);
		public static MoqSockDemuxDiscard[] ALL { get; }
		public static Str AsStr(this MoqSockDemuxDiscard self);
	}
	public enum MoqSockUdpEcnCodepoint {
		Ect0 = 2,
		Ect1 = 1,
		Ce = 3,
	}
	public static unsafe partial class MoqSockUdpEcnCodepointExtensions {
		public static string Debug(this MoqSockUdpEcnCodepoint self);
		public static MoqSockUdpEcnCodepoint? FromBits(byte x);
	}
	public enum Level {
		Error = 0,
		Warn = 1,
		Info = 2,
		Debug = 3,
		Trace = 4,
	}
	public static unsafe partial class LevelExtensions {
		public static int? PartialCmp(this Level self, Level other);
		public static int Cmp(this Level self, Level other);
		public static string Display(this Level self);
	}
	public sealed unsafe partial class Park : RustObject {
		public Park Clone();
		public string Debug();
		public static Park Default();
		public static Park New(Waiter waiter);
		public Waiter Hold(Action cx);
	}
	public sealed unsafe partial class Waiter : RustObject {
		public Waiter Clone();
		public static Waiter New(Action waker);
		public static Waiter Noop();
		public void Register(WaiterList list);
		public Waker Waker();
		public Waker Context();
		public Str Batch(Func<Str> poll);
		public static ValueTask PollFuture(Func<CancellationToken, Task> future, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class WaiterList : RustObject {
		public string Debug();
		public static WaiterList Default();
		public static WaiterList New();
		public void Register(Waiter waiter);
		public WaiterList Take();
		public bool IsEmpty();
		public void Wake();
	}
	public sealed unsafe partial class DecodeError : RustObject {
		public DecodeError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class Driver : RustObject {
		public string Debug();
		public ClockInstant? Poll(ClockInstant now, Waiter waiter);
	}
	public sealed unsafe partial class Instance : RustObject, IEquatable<Instance>, IComparable<Instance> {
		public Instance Clone();
		public bool Equals(Instance? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Instance? a, Instance? b);
		public static bool operator !=(Instance? a, Instance? b);
		public int? PartialCompareTo(Instance other);
		public static bool operator <(Instance a, Instance b);
		public static bool operator <=(Instance a, Instance b);
		public static bool operator >(Instance a, Instance b);
		public static bool operator >=(Instance a, Instance b);
		public int CompareTo(Instance? other);
		public override string ToString();
		public string Debug();
		public static Instance Random();
		public Str AsStr();
		public bool IsMinted();
	}
	public sealed unsafe partial class Path : RustObject, IEquatable<Path>, IComparable<Path> {
		public Path Clone();
		public bool Equals(Path? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Path? a, Path? b);
		public static bool operator !=(Path? a, Path? b);
		public int? PartialCompareTo(Path other);
		public static bool operator <(Path a, Path b);
		public static bool operator <=(Path a, Path b);
		public static bool operator >(Path a, Path b);
		public static bool operator >=(Path a, Path b);
		public int CompareTo(Path? other);
		public override string ToString();
		public string Debug();
		public static Path Default();
		public static Path New(Str s);
		public static Path FromStatic(Str s);
		public bool HasPrefix(Path prefix);
		public Path? StripPrefix(Path prefix);
		public Path? Head(nuint n);
		public Str AsStr();
		public static Path Empty();
		public bool IsEmpty();
		public nuint Len();
		public Path Join(Path other);
		public Path Resolve(PathRelative rel);
		public Path? TryResolve(PathRelative rel);
		public PathRelative? Relative(Path @base);
	}
	public sealed unsafe partial class Pattern : RustObject, IEquatable<Pattern>, IComparable<Pattern> {
		public Pattern Clone();
		public bool Equals(Pattern? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Pattern? a, Pattern? b);
		public static bool operator !=(Pattern? a, Pattern? b);
		public int? PartialCompareTo(Pattern other);
		public static bool operator <(Pattern a, Pattern b);
		public static bool operator <=(Pattern a, Pattern b);
		public static bool operator >(Pattern a, Pattern b);
		public static bool operator >=(Pattern a, Pattern b);
		public int CompareTo(Pattern? other);
		public override string ToString();
		public string Debug();
		public static Pattern Default();
		public static Pattern New(Segment[] segments);
		public static Pattern All();
		public static Pattern Literal(Str path);
		public static Pattern Subtree(Str path);
		public Str AsStr();
		public Segment[] Segments();
		public Str Head();
		public bool IsLiteral();
		public Str? AsPrefix();
		public bool HasGlobstar();
		public bool Matches(Str path);
		public bool Contains(Pattern other);
		public bool Overlaps(Pattern other);
		public Specificity Specificity();
		public Patterns Rebase(Str root);
		public Patterns Intersect(Pattern other);
		public Pattern[]? Captures(Pattern matched);
		public Pattern Rooted(Str root);
	}
	public sealed unsafe partial class Patterns : RustObject, IEquatable<Patterns> {
		public Patterns Clone();
		public bool Equals(Patterns? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Patterns? a, Patterns? b);
		public static bool operator !=(Patterns? a, Patterns? b);
		public string Debug();
		public static Patterns Default();
		public static Patterns New();
		public bool Insert(Pattern pattern);
		public bool Matches(Str path);
		public bool Contains(Pattern pattern);
		public bool Covers(Patterns other);
		public bool Overlaps(Pattern pattern);
		public Patterns Intersect(Patterns other);
		public Patterns Rebase(Str root);
		public Patterns Rooted(Str root);
		public Pattern[] Iter();
		public nuint Len();
		public bool IsEmpty();
	}
	public sealed unsafe partial class Server : RustObject {
		public Server Clone();
		public static Server Default();
		public static Server New();
		public Server WithPublisherProducer(OriginProducer publish);
		public Server WithPublisherConsumer(OriginConsumer publish);
		public Server WithSubscriber(OriginProducer subscribe);
		public Server WithAllocator(BandwidthAllocator allocator);
		public Server WithStats(StatsSession stats);
		public Server WithLimits(SessionLimits limits);
		public Server WithVersions(Versions versions);
		public Server WithImplementation(Str implementation);
		public Server WithTimeout(Duration? timeout);
		public ClockInstant? Deadline(ClockInstant arrived);
		public Server WithReceiveWindow(NonZeroU64 window);
		public ValueTask<(Session, Driver)> AcceptMemorySession(ClockInstant now, TransportMemorySession session, CancellationToken cancellationToken = default);
		public ValueTask<(Session, Driver)> AcceptQmuxSession(ClockInstant now, QmuxSession session, CancellationToken cancellationToken = default);
		public ValueTask<ServerHandshake> HandshakeMemorySession(ClockInstant now, TransportMemorySession session, CancellationToken cancellationToken = default);
		public ValueTask<ServerHandshake> HandshakeQmuxSession(ClockInstant now, QmuxSession session, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Session : RustObject, IEquatable<Session> {
		public Session Clone();
		public bool Equals(Session? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Session? a, Session? b);
		public static bool operator !=(Session? a, Session? b);
		public string Debug();
		public Version Version();
		public ValueTask<Str?> PeerImplementation(CancellationToken cancellationToken = default);
		public BandwidthConsumer? SendBandwidth();
		public BandwidthAllocator Allocator();
		public BandwidthConsumer? RecvBandwidth();
		public BandwidthViewers Viewers();
		public SessionStats Stats();
		public void Abort(Error err);
		public ValueTask Close(CancellationToken cancellationToken = default);
		public ValueTask<Error> Closed(CancellationToken cancellationToken = default);
		public void Goaway(GoawayGoaway goaway);
		public GoawayConsumer Draining();
		public AuthHandle Auth();
	}
	public sealed unsafe partial class Client : RustObject {
		public Client Clone();
		public static Client Default();
		public static Client New();
		public Client WithPublisherProducer(OriginProducer publish);
		public Client WithPublisherConsumer(OriginConsumer publish);
		public Client WithSubscriber(OriginProducer subscribe);
		public Client WithStats(StatsSession stats);
		public Client WithLimits(SessionLimits limits);
		public Client WithAllocator(BandwidthAllocator allocator);
		public Client WithViewers(BandwidthViewers viewers);
		public Client WithVersions(Versions versions);
		public Client WithUrl(Url url);
		public Url? Url();
		public Client WithToken(SetupToken token);
		public Client WithAuth(AuthHandle auth);
		public Client WithImplementation(Str implementation);
		public Client WithCost(OriginCost cost);
		public Client WithPeerNode(NonZeroU64 node);
		public ValueTask<(Session, Driver)> ConnectMemorySession(ClockInstant now, TransportMemorySession session, CancellationToken cancellationToken = default);
		public ValueTask<(Session, Driver)> ConnectQmuxSession(ClockInstant now, QmuxSession session, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Error : RustObject {
		public Error Clone();
		public override string ToString();
		public string Debug();
		public SessionError? Session();
		public StreamError? Stream();
		public static Error FromTransportError(Error err);
		public static Error FromTransportMemoryError(TransportMemoryError err);
		public Retry Retry();
	}
	public sealed unsafe partial class Version : RustObject, IEquatable<Version> {
		public Version Clone();
		public bool Equals(Version? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Version? a, Version? b);
		public static bool operator !=(Version? a, Version? b);
		public override string ToString();
		public string Debug();
		public static Version[] ALL { get; }
		public bool HasRequestPath();
		public Str AsStr();
		public Str Alpn();
		public bool IsLite();
	}
	public sealed unsafe partial class Versions : RustObject {
		public Versions Clone();
		public string Debug();
		public static Versions Default();
		public static Versions All();
		public Str[] Alpns();
		public Versions? Filter(Versions other);
		public Version? Select(Version version);
		public bool Contains(Version version);
		public Version[] Iter();
	}
	public sealed unsafe partial class Specificity : RustObject, IEquatable<Specificity>, IComparable<Specificity> {
		public Specificity Clone();
		public bool Equals(Specificity? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Specificity? a, Specificity? b);
		public static bool operator !=(Specificity? a, Specificity? b);
		public int? PartialCompareTo(Specificity other);
		public static bool operator <(Specificity a, Specificity b);
		public static bool operator <=(Specificity a, Specificity b);
		public static bool operator >(Specificity a, Specificity b);
		public static bool operator >=(Specificity a, Specificity b);
		public int CompareTo(Specificity? other);
		public string Debug();
	}
	public sealed unsafe partial class ClockClock : RustObject, IEquatable<ClockClock> {
		public ClockClock Clone();
		public bool Equals(ClockClock? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClockClock? a, ClockClock? b);
		public static bool operator !=(ClockClock? a, ClockClock? b);
		public string Debug();
		public static ClockClock Current();
		public ClockInstant Now();
		public ClockWall Wall();
	}
	public sealed unsafe partial class ClockDeadline : RustObject {
		public string Debug();
		public static ClockDeadline New(ClockDeadlines deadlines);
		public static ClockDeadline After(ClockDeadlines deadlines, Duration duration);
		public void Set(ClockInstant? at);
		public ClockInstant? Deadline();
		public ValueTask Poll(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class ClockDeadlines : RustObject {
		public ClockDeadlines Clone();
		public static ClockDeadlines New(ClockInstant now);
		public void Advance(ClockInstant now);
		public ClockInstant? Park(Waiter waiter);
		public ClockInstant? Next();
		public ClockInstant Now();
		public ClockEntered Enter();
	}
	public sealed unsafe partial class ClockEntered : RustObject {
		public string Debug();
	}
	public sealed unsafe partial class ClockManual : RustObject {
		public ClockManual Clone();
		public string Debug();
		public static ClockManual Default();
		public static ClockWall WALL_START { get; }
		public static ClockManual New();
		public ClockInstant Now();
		public ClockWall Wall();
		public void Advance(Duration duration);
		public void StepWall(ClockWall wall);
	}
	public sealed unsafe partial class ClockRng : RustObject {
	}
	public sealed unsafe partial class ClockInterval : RustObject {
		public ValueTask<ClockInstant> Tick(CancellationToken cancellationToken = default);
		public void SetMissedTickBehavior(ClockMissedTickBehavior missed);
		public Duration Period();
	}
	public sealed unsafe partial class ClockWallMap : RustObject {
		public ClockWallMap Clone();
		public string Debug();
		public static ClockWallMap Default();
		public static Duration TOLERANCE { get; }
		public void Sample(ClockInstant now, ClockWall wall);
		public ClockInstant? Map(ClockWall at);
	}
	public sealed unsafe partial class CoopBudget : RustObject {
		public CoopBudget Clone();
		public string Debug();
		public static CoopBudget New(uint passes);
		public ValueTask PollYield(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class HangCatalog : RustObject, IEquatable<HangCatalog> {
		public HangCatalog Clone();
		public bool Equals(HangCatalog? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalog? a, HangCatalog? b);
		public static bool operator !=(HangCatalog? a, HangCatalog? b);
		public string Debug();
		public static HangCatalog Default();
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public static Str DEFAULT_NAME { get; }
		public static TrackInfo DefaultTrackInfo();
		public static TrackSubscription DefaultSubscription();
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public static HangCatalog FromSlice(ReadOnlySpan<byte> v);
		public static HangCatalog FromReader(ReadOnlySpan<byte> reader);
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class HangCatalogExtra : RustObject, IEquatable<HangCatalogExtra> {
		public HangCatalogExtra Clone();
		public bool Equals(HangCatalogExtra? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogExtra? a, HangCatalogExtra? b);
		public static bool operator !=(HangCatalogExtra? a, HangCatalogExtra? b);
		public string Debug();
		public static HangCatalogExtra Default();
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public MuxCatalogHangExtra Ext { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public static HangCatalogExtra FromSlice(ReadOnlySpan<byte> v);
		public static HangCatalogExtra FromReader(ReadOnlySpan<byte> reader);
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class HangCatalogExt : RustObject, IEquatable<HangCatalogExt> {
		public HangCatalogExt Clone();
		public bool Equals(HangCatalogExt? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogExt? a, HangCatalogExt? b);
		public static bool operator !=(HangCatalogExt? a, HangCatalogExt? b);
		public string Debug();
		public static HangCatalogExt Default();
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public MuxContainerTsExt Ext { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public static HangCatalogExt FromSlice(ReadOnlySpan<byte> v);
		public static HangCatalogExt FromReader(ReadOnlySpan<byte> reader);
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class HangError : RustObject {
		public HangError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxClock : RustObject, IEquatable<MuxClock> {
		public MuxClock Clone();
		public bool Equals(MuxClock? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxClock? a, MuxClock? b);
		public static bool operator !=(MuxClock? a, MuxClock? b);
		public string Debug();
		public static MuxClock Default();
		public static Timescale TIMESCALE { get; }
		public static MuxClock New();
		public static MuxClock At(ClockInstant epoch, ClockWall wall);
		public Timestamp Now();
		public Timestamp Capture(ClockInstant at);
		public Timescale Timescale();
		public HangCatalogClock? Wall();
		public ClockWall WallClock(Timestamp pts);
	}
	public sealed unsafe partial class MuxIngest : RustObject {
		public MuxIngest Clone();
		public string Debug();
		public static MuxIngest Default();
		public Duration? MaxAge { get; set; }
		public BandwidthAllocator Bandwidth { get; set; }
		public Duration Linger { get; set; }
		public BandwidthLimit? Rate { get; set; }
		public MuxCatalogConfig Catalog();
	}
	public sealed unsafe partial class MuxLingerUnit : RustObject {
		public static Duration DEFAULT { get; }
		public static MuxLingerUnit New(BroadcastProducer[] broadcasts, Duration duration);
		public ValueTask Wait(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxPacer : RustObject {
		public static MuxPacer Default();
		public MuxPacer WithLead(Duration lead);
		public MuxPacer WithDelay(Duration delay);
		public ClockInstant Pace(Timestamp ts, ClockInstant now);
		public ClockInstant? Due(Timestamp ts);
		public Duration Slack();
		public ClockInstant Hurry(Timestamp ts, ClockInstant now);
	}
	public sealed unsafe partial class MuxBinding : RustObject {
		public static MuxBinding New(BroadcastConsumer broadcast);
		public ValueTask<BroadcastConsumer> Broadcast(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxSource : RustObject {
		public MuxSource Clone();
		public static MuxSource New(OriginConsumer origin, BroadcastConsumer broadcast);
		public (Str, HangEchoRendition)[] Stats();
		public BroadcastConsumer Broadcast();
		public ValueTask<MuxCatalogConsumerExtra> CatalogExtra(MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
		public ValueTask<MuxCatalogConsumerExt> CatalogExt(MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
		public ValueTask<MuxCatalogConsumer> CatalogUnit(MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
		public Path? ResolveReference(PathRelative? rel);
		public ValueTask<BroadcastConsumer> Resolve(PathRelative? rel, Instance? instance, CancellationToken cancellationToken = default);
		public MuxBinding Bind(PathRelative? rel, Instance? instance);
		public ValueTask<TrackSubscriber> SubscribeTrack(PathRelative? rel, Instance? instance, Str name, CancellationToken cancellationToken = default);
		public MuxFollowInstances Follow(PathRelative? rel, Instance? instance);
	}
	public sealed unsafe partial class MuxError : RustObject {
		public MuxError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class TokioDuration : RustObject, IEquatable<TokioDuration>, IComparable<TokioDuration> {
		public TokioDuration Clone();
		public bool Equals(TokioDuration? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TokioDuration? a, TokioDuration? b);
		public static bool operator !=(TokioDuration? a, TokioDuration? b);
		public int? PartialCompareTo(TokioDuration other);
		public static bool operator <(TokioDuration a, TokioDuration b);
		public static bool operator <=(TokioDuration a, TokioDuration b);
		public static bool operator >(TokioDuration a, TokioDuration b);
		public static bool operator >=(TokioDuration a, TokioDuration b);
		public int CompareTo(TokioDuration? other);
		public override string ToString();
		public string Debug();
		public static TokioDuration Default();
	}
	public sealed unsafe partial class TokioBackoff : RustObject {
		public TokioBackoff Clone();
		public string Debug();
		public static TokioBackoff Default();
		public TokioDuration? Initial { get; set; }
		public uint Multiplier { get; set; }
		public MoqTokioCliBound? Max { get; set; }
		public TokioDuration? Healthy { get; set; }
		public MoqTokioCliBound? Timeout { get; set; }
		public Duration Initial_();
		public Duration? Max_();
		public Duration? Timeout_();
	}
	public sealed unsafe partial class TokioClient : RustObject {
		public TokioClient Clone();
		public static TokioClient New(TokioClientConfig config);
		public TokioClient WithPublisherProducer(OriginProducer publish);
		public TokioClient WithPublisherConsumer(OriginConsumer publish);
		public TokioClient WithSubscriber(OriginProducer subscribe);
		public TokioClient WithLimits(SessionLimits limits);
		public TokioClient WithStats(StatsSession stats);
		public TokioClient WithCost(ulong cost);
		public TokioClient WithPeerNode(NonZeroU64 node);
		public TokioClient WithToken(SetupToken token);
		public TokioClient WithReconnect(bool reconnect);
		public TokioClient WithRestart(TokioRestart restart);
		public TokioConnection Connect(MoqTokioAddrs addrs);
		public ValueTask Close(CancellationToken cancellationToken = default);
		public TokioConnection? Publish(OriginConsumer origin);
		public TokioConnection? Consume(OriginProducer origin);
	}
	public sealed unsafe partial class TokioConnection : RustObject {
		public TokioConnection Clone();
		public void Abort(Error err);
		public ValueTask Close(CancellationToken cancellationToken = default);
		public ValueTask<TokioConnection> Established(CancellationToken cancellationToken = default);
		public ValueTask<TokioStatus> Status(CancellationToken cancellationToken = default);
		public bool Connected();
		public ulong Connects();
		public Version? Version();
		public TokioTransport? Transport();
		public GoawayConsumer? Draining();
		public BandwidthConsumer SendBandwidth();
		public BandwidthAllocator Allocator();
		public BandwidthConsumer RecvBandwidth();
		public BandwidthViewers Viewers();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public AuthHandle Auth();
		public AuthTokens Tokens();
		public TokioConnectionMonitor Monitor();
	}
	public sealed unsafe partial class TokioError : RustObject {
		public TokioError Clone();
		public override string ToString();
		public string Debug();
		public TokioConnectError? ConnectError();
		public bool IsAuth();
		public ushort? Status();
	}
	public sealed unsafe partial class TokioLog : RustObject {
		public TokioLog Clone();
		public string Debug();
		public static TokioLog Default();
		public Level Level { get; set; }
		public static TokioLog New(Level level);
		public Level? Level_();
		public void Init();
	}
	public sealed unsafe partial class TokioServer : RustObject {
		public static TokioServer New(TokioServerConfig config);
		public ValueTask ServePublish(OriginConsumer origin, CancellationToken cancellationToken = default);
		public MoqTokioTcpTlsAcceptor? TlsAcceptor();
		public TokioTlsCertificates Certificates();
		public MoqTokioAcceptHealth[] AcceptHealth();
		public ValueTask<TokioServerListener> Bind(CancellationToken cancellationToken = default);
		public ValueTask<TokioServerListener> Listen(CancellationToken cancellationToken = default);
		public SocketAddr LocalAddr();
		public SocketAddr? WebsocketLocalAddr();
	}
	public sealed unsafe partial class AuthWatch : RustObject {
		public AuthGrant? Peek();
		public ValueTask<AuthGrant?> Changed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AuthRequests : RustObject, IAsyncEnumerable<AuthRequest> {
		public ValueTask<AuthRequest?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<AuthRequest> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AuthRequest : RustObject {
		public Bytes Token();
		public ValueTask<Error> Closed(CancellationToken cancellationToken = default);
		public AuthIssued Accept(AuthGrant grant);
		public void Reject(SessionError code, Str reason);
	}
	public sealed unsafe partial class AuthIssued : RustObject {
		public void Update(AuthGrant grant);
		public void Revoke(SessionError code, Str reason);
		public ValueTask<Error> Closed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AuthGrant : RustObject, IEquatable<AuthGrant> {
		public AuthGrant Clone();
		public bool Equals(AuthGrant? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(AuthGrant? a, AuthGrant? b);
		public static bool operator !=(AuthGrant? a, AuthGrant? b);
		public string Debug();
		public static AuthGrant Default();
		public Patterns Publish { get; set; }
		public Patterns Subscribe { get; set; }
		public ClockInstant? Expires { get; set; }
		public static AuthGrant All();
	}
	public sealed unsafe partial class AuthHandle : RustObject {
		public AuthHandle Clone();
		public static AuthHandle Default();
		public static AuthHandle New();
		public AuthWatch Grant();
		public AuthCredential Add(ReadOnlySpan<byte> token);
		public void Authorize(AuthGrant grant);
		public void Abort(Error err);
	}
	public sealed unsafe partial class AuthCredential : RustObject, IEquatable<AuthCredential> {
		public AuthCredential Clone();
		public bool Equals(AuthCredential? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(AuthCredential? a, AuthCredential? b);
		public static bool operator !=(AuthCredential? a, AuthCredential? b);
		public string Debug();
		public AuthWatch Grant();
		public ValueTask<Error> Closed(CancellationToken cancellationToken = default);
		public ValueTask Answered(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AuthToken : RustObject, IEquatable<AuthToken> {
		public AuthToken Clone();
		public bool Equals(AuthToken? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(AuthToken? a, AuthToken? b);
		public static bool operator !=(AuthToken? a, AuthToken? b);
		public string Debug();
		public Str AsStr();
	}
	public sealed unsafe partial class AuthTokens : RustObject {
		public AuthTokens Clone();
		public static AuthTokens New(AuthToken[] tokens);
		public AuthHandle Handle();
		public void Set(AuthToken[] tokens);
		public ValueTask Answered(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class GoawayGoaway : RustObject, IEquatable<GoawayGoaway> {
		public GoawayGoaway Clone();
		public bool Equals(GoawayGoaway? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(GoawayGoaway? a, GoawayGoaway? b);
		public static bool operator !=(GoawayGoaway? a, GoawayGoaway? b);
		public string Debug();
		public static GoawayGoaway Default();
		public static GoawayGoaway New();
		public static GoawayGoaway Redirect(Str uri);
		public GoawayGoaway WithTimeout(Duration? timeout);
		public Str Uri();
		public Duration? Timeout();
	}
	public sealed unsafe partial class GoawayConsumer : RustObject {
		public GoawayConsumer Clone();
		public ValueTask<GoawayGoaway> Poll(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class PathRelative : RustObject, IEquatable<PathRelative> {
		public PathRelative Clone();
		public bool Equals(PathRelative? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PathRelative? a, PathRelative? b);
		public static bool operator !=(PathRelative? a, PathRelative? b);
		public override string ToString();
		public string Debug();
		public static PathRelative Default();
		public static PathRelative New(Str s);
		public Str AsStr();
		public static PathRelative Empty();
		public bool IsEmpty();
		public nuint Len();
	}
	public sealed unsafe partial class ReconnectBackoff : RustObject, IEquatable<ReconnectBackoff> {
		public ReconnectBackoff Clone();
		public bool Equals(ReconnectBackoff? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ReconnectBackoff? a, ReconnectBackoff? b);
		public static bool operator !=(ReconnectBackoff? a, ReconnectBackoff? b);
		public string Debug();
		public static ReconnectBackoff Default();
		public Duration Initial { get; set; }
		public uint Multiplier { get; set; }
		public Duration? Max { get; set; }
		public Duration Healthy { get; set; }
		public Duration? Timeout { get; set; }
	}
	public sealed unsafe partial class ReconnectHandover : RustObject, IEquatable<ReconnectHandover> {
		public ReconnectHandover Clone();
		public bool Equals(ReconnectHandover? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ReconnectHandover? a, ReconnectHandover? b);
		public static bool operator !=(ReconnectHandover? a, ReconnectHandover? b);
		public string Debug();
		public static ReconnectHandover Default();
		public ReconnectRedirect Redirect { get; set; }
		public Duration Handover { get; set; }
	}
	public sealed unsafe partial class ReconnectConfig : RustObject, IEquatable<ReconnectConfig> {
		public ReconnectConfig Clone();
		public bool Equals(ReconnectConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ReconnectConfig? a, ReconnectConfig? b);
		public static bool operator !=(ReconnectConfig? a, ReconnectConfig? b);
		public string Debug();
		public static ReconnectConfig Default();
		public ReconnectBackoff Backoff { get; set; }
		public ReconnectHandover Goaway { get; set; }
		public TokioRestart Restart { get; set; }
		public Duration? Timeout { get; set; }
		public bool Once { get; set; }
		public bool Pinned { get; set; }
	}
	public sealed unsafe partial class ServerHandshake : RustObject {
		public Str Path();
		public SetupRole? Role();
		public NonZeroU64? PeerNode();
		public Str? PeerImplementation();
		public AuthHandle Auth();
		public AuthRequests Requests();
		public SetupToken? Token();
		public ServerHandshake WithPublisherProducer(OriginProducer publish);
		public ServerHandshake WithPublisherConsumer(OriginConsumer publish);
		public ServerHandshake WithSubscriber(OriginProducer subscribe);
		public ServerHandshake WithPeerNode(NonZeroU64 node);
		public ServerHandshake WithStats(StatsSession stats);
		public ServerHandshake WithLimits(SessionLimits limits);
		public ServerHandshake WithReceiveWindow(NonZeroU64 window);
		public ValueTask Expired(CancellationToken cancellationToken = default);
		public ValueTask<(Session, Driver)> Ok(CancellationToken cancellationToken = default);
		public void Close(Error err);
	}
	public sealed unsafe partial class ServerAdmission : RustObject {
		public ServerAdmission Clone();
		public string Debug();
		public static ServerAdmission Default();
		public ServerAdmitted? Enter();
		public void Drain();
		public ValueTask<ClockInstant> Close(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class ServerAdmitted : RustObject {
		public string Debug();
	}
	public sealed unsafe partial class SessionStats : RustObject, IEquatable<SessionStats> {
		public SessionStats Clone();
		public bool Equals(SessionStats? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(SessionStats? a, SessionStats? b);
		public static bool operator !=(SessionStats? a, SessionStats? b);
		public string Debug();
		public static SessionStats Default();
		public Duration? Rtt { get; set; }
		public BandwidthRate? EstimatedSendRate { get; set; }
		public ulong? BytesSent { get; set; }
		public ulong? BytesReceived { get; set; }
		public ulong? BytesLost { get; set; }
		public ulong? PacketsSent { get; set; }
		public ulong? PacketsReceived { get; set; }
		public ulong? PacketsLost { get; set; }
		public ulong? StreamsEvicted { get; set; }
		public ulong? DatagramsUnrouted { get; set; }
	}
	public sealed unsafe partial class StatsTier : RustObject, IEquatable<StatsTier> {
		public StatsTier Clone();
		public bool Equals(StatsTier? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsTier? a, StatsTier? b);
		public static bool operator !=(StatsTier? a, StatsTier? b);
		public override string ToString();
		public string Debug();
		public static StatsTier Default();
		public static StatsTier New(Path label);
		public Path Label();
		public Str AsStr();
	}
	public sealed unsafe partial class StatsSnapshot : RustObject, IEquatable<StatsSnapshot> {
		public StatsSnapshot Clone();
		public bool Equals(StatsSnapshot? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StatsSnapshot? a, StatsSnapshot? b);
		public static bool operator !=(StatsSnapshot? a, StatsSnapshot? b);
		public string Debug();
		public static StatsSnapshot Default();
		public (StatsTier, StatsMessage, StatsTotals)[] Control();
		public (StatsTier, StatsRole, StatsTraffic)[] Traffic();
		public (StatsTier, StatsPresence)[] Sessions();
		public (StatsTier, StatsActivity)[] Activity();
	}
	public sealed unsafe partial class StatsReport : RustObject {
		public StatsReport Clone();
		public string Debug();
		public static StatsReport Default();
		public StatsTrafficEntry[] Traffic { get; set; }
		public StatsSessionEntry[] Sessions { get; set; }
	}
	public sealed unsafe partial class StatsTrafficEntry : RustObject {
		public StatsTrafficEntry Clone();
		public string Debug();
		public Path Path { get; set; }
		public StatsTier Tier { get; set; }
		public StatsTraffic Publisher { get; set; }
		public StatsTraffic Subscriber { get; set; }
		public bool Closed { get; set; }
	}
	public sealed unsafe partial class StatsSessionEntry : RustObject {
		public StatsSessionEntry Clone();
		public string Debug();
		public StatsTier Tier { get; set; }
		public Path Label { get; set; }
		public StatsPresence Presence { get; set; }
		public bool Closed { get; set; }
	}
	public sealed unsafe partial class StatsConfig : RustObject {
		public StatsConfig Clone();
		public string Debug();
		public static StatsConfig Default();
		public Patterns Exclude { get; set; }
		public static StatsConfig New();
		public StatsConfig WithExclude(Pattern pattern);
	}
	public sealed unsafe partial class StatsRegistry : RustObject {
		public StatsRegistry Clone();
		public static StatsRegistry Default();
		public static StatsRegistry New(StatsConfig config);
		public static StatsRegistry Disabled();
		public bool IsEnabled();
		public StatsHandle Tier(StatsTier tier);
		public StatsSnapshot Snapshot();
		public void Report(StatsReport report, ClockInstant now);
	}
	public sealed unsafe partial class StatsHandle : RustObject {
		public StatsHandle Clone();
		public static StatsHandle Default();
		public void Refuse(StatsRefusal refusal);
		public StatsSession Session(StatsSessionInfo session);
		public StatsSession Request(Path label);
	}
	public sealed unsafe partial class StatsRefusal : RustObject {
		public StatsRefusal Clone();
		public string Debug();
		public Path Label { get; set; }
		public StatsRefusalKind Reason { get; set; }
		public static StatsRefusal New(Path label, StatsRefusalKind reason);
	}
	public sealed unsafe partial class StatsSessionInfo : RustObject {
		public StatsSessionInfo Clone();
		public string Debug();
		public Path Label { get; set; }
		public static StatsSessionInfo New(Path label);
	}
	public sealed unsafe partial class StatsWire : RustObject {
		public static StatsWire Default();
	}
	public sealed unsafe partial class StatsSession : RustObject {
		public StatsSession Clone();
		public static StatsSession Default();
		public StatsWire Wire(TransportWire counters);
		public void Retag(StatsTier tier, Path label);
	}
	public sealed unsafe partial class StatsBill : RustObject {
		public StatsBill Clone();
		public string Debug();
		public static StatsBill Default();
		public void Deliver(ulong bytes);
	}
	public sealed unsafe partial class TransportWire : RustObject {
		public TransportWire Clone();
		public string Debug();
		public static TransportWire Default();
		public static TransportWire New();
		public ulong SentBytes();
		public ulong ReceivedBytes();
		public void SetSentBytes(ulong total);
		public void SetReceivedBytes(ulong total);
	}
	public sealed unsafe partial class TransportProbe : RustObject {
		public TransportProbe Clone();
		public string Debug();
		public static TransportProbe Default();
		public static TransportProbe New();
		public BandwidthRate Target();
		public void SetTarget(BandwidthRate rate);
	}
	public sealed unsafe partial class BandwidthLimit : RustObject {
		public BandwidthLimit Clone();
		public string Debug();
		public static BandwidthLimit Default();
		public static BandwidthLimit New(BandwidthCap? cap);
		public void Set(BandwidthCap? cap);
		public BandwidthCap? Get();
		public ValueTask<BandwidthCap?> Changed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BandwidthDemand : RustObject {
		public BandwidthDemand Clone();
		public bool Wanted();
		public ValueTask<bool> WantedChanged(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BandwidthReservation : RustObject {
		public BandwidthRate? Rate();
		public BandwidthConsumer Consume();
		public void Update(BandwidthRate max);
	}
	public sealed unsafe partial class BandwidthViewer : RustObject {
		public BandwidthRate? Rate();
		public BandwidthConsumer Consume();
		public void Update(BandwidthRate max);
	}
	public sealed unsafe partial class BandwidthCap : RustObject, IEquatable<BandwidthCap> {
		public BandwidthCap Clone();
		public bool Equals(BandwidthCap? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(BandwidthCap? a, BandwidthCap? b);
		public static bool operator !=(BandwidthCap? a, BandwidthCap? b);
		public string Debug();
		public BandwidthRate Rate { get; set; }
		public Duration Burst { get; set; }
		public static Duration DEFAULT_BURST { get; }
		public static BandwidthCap New(BandwidthRate rate);
		public BandwidthCap WithBurst(Duration burst);
	}
	public sealed unsafe partial class BandwidthProducer : RustObject {
		public BandwidthProducer Clone();
		public static BandwidthProducer Default();
		public static BandwidthProducer New();
		public void Set(BandwidthRate? bitrate);
		public BandwidthConsumer Consume();
		public BandwidthWeak Weak();
		public BandwidthDemand Demand();
	}
	public sealed unsafe partial class BandwidthWeak : RustObject {
		public BandwidthWeak Clone();
		public BandwidthConsumer Consume();
	}
	public sealed unsafe partial class BandwidthAllocator : RustObject {
		public BandwidthAllocator Clone();
		public string Debug();
		public static BandwidthAllocator Default();
		public static BandwidthAllocator New(BandwidthConsumer estimate);
		public void SetTarget(BandwidthRate? target);
		public static BandwidthAllocator Unlimited();
		public BandwidthReservation Reserve(TrackDemand track, BandwidthRate max);
	}
	public sealed unsafe partial class BandwidthViewers : RustObject {
		public BandwidthViewers Clone();
		public string Debug();
		public static BandwidthViewers Default();
		public static BandwidthRate MAX_TARGET { get; }
		public static BandwidthViewers New(BandwidthWeak estimate);
		public BandwidthViewer Watch(BandwidthRate max);
		public void SetTarget(BandwidthRate? target);
	}
	public sealed unsafe partial class BandwidthConsumer : RustObject {
		public BandwidthConsumer Clone();
		public BandwidthRate? Peek();
		public bool Probed();
		public ValueTask<BandwidthRate?> Changed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BroadcastDatagramRequests : RustObject {
		public BroadcastDatagramRequests Clone();
		public ValueTask<DatagramRequest> Requested(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BroadcastConsumer : RustObject {
		public BroadcastConsumer Clone();
		public BroadcastTracks Tracks();
		public BroadcastInfo Info();
		public TrackConsumer Track(Str name);
		public DatagramConsumer Datagrams(Str name);
		public bool IsClosed_();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClone(BroadcastConsumer other);
	}
	public sealed unsafe partial class BroadcastDemand : RustObject {
		public BroadcastDemand Clone();
		public bool Wanted();
		public ValueTask<bool> WantedChanged(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BroadcastInfo : RustObject {
		public BroadcastInfo Clone();
		public string Debug();
		public static BroadcastInfo Default();
		public CachePool Pool { get; set; }
		public Duration? CacheDuration { get; set; }
		public Path Path { get; set; }
		public Instance? Instance { get; set; }
		public static BroadcastInfo New();
		public BroadcastProducer Produce();
	}
	public sealed unsafe partial class BroadcastProducer : RustObject {
		public BroadcastProducer Clone();
		public static BroadcastProducer New(BroadcastInfo info);
		public BroadcastInfo Info();
		public BroadcastDemand Demand();
		public TrackProducer CreateTrack(Str name, TrackInfo? info);
		public DatagramProducer CreateDatagrams(Str name, DatagramInfo info);
		public TrackRequest ReserveTrack(Str name);
		public TrackRequest ReserveUnlistedTrack(Str name);
		public TrackProducer UniqueTrack(Str suffix, TrackInfo? info);
		public Str UniqueName(Str suffix);
		public BroadcastRequests Requests();
		public BroadcastConsumer Consume();
		public void Close();
		public bool IsClone(BroadcastProducer other);
	}
	public sealed unsafe partial class BroadcastRequests : RustObject {
		public BroadcastRequests Clone();
		public BroadcastInfo Info();
		public ValueTask<TrackRequest> RequestedTrack(CancellationToken cancellationToken = default);
		public BroadcastConsumer Consume();
		public BroadcastDatagramRequests Datagrams();
		public BroadcastLister Lister();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClone(BroadcastRequests other);
	}
	public sealed unsafe partial class BroadcastLister : RustObject {
		public ValueTask PollWanted(CancellationToken cancellationToken = default);
		public ValueTask PollUnwanted(CancellationToken cancellationToken = default);
		public void Add(Str name, TrackInfo info);
		public void Remove(BroadcastKind kind, Str name);
		public void Finish(BroadcastKind kind, Str name);
		public void Synced();
		public void Clear();
		public void End();
		public void Abort(Error err);
	}
	public sealed unsafe partial class BroadcastTracks : RustObject, IAsyncEnumerable<BroadcastTracksEvent> {
		public ValueTask<BroadcastTracksEvent?> Next(CancellationToken cancellationToken = default);
		public BroadcastConsumer Broadcast();
		public IAsyncEnumerator<BroadcastTracksEvent> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class BroadcastTracksEvent : RustObject {
		public BroadcastTracksEvent Clone();
		public string Debug();
	}
	public sealed unsafe partial class CacheConfig : RustObject {
		public CacheConfig Clone();
		public string Debug();
		public static CacheConfig Default();
		public CacheConfig WithCapacity(ulong? capacity);
		public CacheConfig WithExpiry(Duration? expiry);
	}
	public sealed unsafe partial class CachePool : RustObject {
		public CachePool Clone();
		public string Debug();
		public static CachePool Default();
		public static CachePool New(CacheConfig config);
		public static CachePool Unbounded();
		public ulong? Capacity();
		public ulong Used();
		public void Resize(ulong? capacity);
		public Duration? Expiry();
		public ClockInstant? Poll(ClockInstant now, Waiter waiter);
		public CachePoolWeak Downgrade();
	}
	public sealed unsafe partial class CachePoolWeak : RustObject {
		public CachePoolWeak Clone();
		public string Debug();
		public CachePool? Upgrade();
	}
	public sealed unsafe partial class DatagramProducer : RustObject {
		public DatagramProducer Clone();
		public Str Name();
		public DatagramInfo Info();
		public ulong Append(Timestamp timestamp, ReadOnlySpan<byte> payload);
		public void Insert(DatagramDatagram datagram);
		public DatagramConsumer Consume();
		public void Finish();
		public void Abort(Error err);
		public sbyte? Priority();
		public ValueTask<sbyte?> PriorityChanged(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class DatagramRequest : RustObject {
		public Str Name();
		public sbyte? Priority();
		public DatagramProducer Accept(DatagramInfo info);
		public void Reject(Error err);
	}
	public sealed unsafe partial class DatagramConsumer : RustObject {
		public DatagramConsumer Clone();
		public Str Name();
		public ValueTask<DatagramInfo> Info(CancellationToken cancellationToken = default);
		public ValueTask<DatagramDatagram?> Recv(CancellationToken cancellationToken = default);
		public sbyte Priority();
		public void SetPriority(sbyte priority);
	}
	public sealed unsafe partial class DatagramInfo : RustObject, IEquatable<DatagramInfo> {
		public DatagramInfo Clone();
		public bool Equals(DatagramInfo? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(DatagramInfo? a, DatagramInfo? b);
		public static bool operator !=(DatagramInfo? a, DatagramInfo? b);
		public string Debug();
		public static DatagramInfo Default();
		public Timescale? Timescale { get; set; }
		public sbyte Priority { get; set; }
		public DatagramInfo WithTimescale(Timescale? timescale);
		public DatagramInfo WithPriority(sbyte priority);
		public Timestamp Rescale(Timestamp timestamp);
	}
	public sealed unsafe partial class FrameProducer : RustObject {
		public GroupProducer Group();
		public nuint Remaining();
		public void Write(ReadOnlySpan<byte> chunk);
		public GroupProducer Finish();
		public void Abort(Error err);
		public ulong Size { get; }
		public Timestamp Timestamp { get; }
	}
	public sealed unsafe partial class FrameFrame : RustObject {
		public FrameFrame Clone();
		public string Debug();
		public Timestamp Timestamp { get; set; }
		public Bytes Payload { get; set; }
		public static FrameFrame New(Timestamp timestamp, ReadOnlySpan<byte> payload);
		public ClockInstant Arrived();
	}
	public sealed unsafe partial class FrameConsumer : RustObject {
		public FrameConsumer Clone();
		public ValueTask<Bytes?> ReadChunk(CancellationToken cancellationToken = default);
		public ValueTask<Bytes> ReadAll(CancellationToken cancellationToken = default);
		public ulong Size { get; }
		public Timestamp Timestamp { get; }
	}
	public sealed unsafe partial class GroupProducer : RustObject {
		public Timescale? Timescale();
		public void StartAt(ulong index);
		public void WriteFrame(Timestamp timestamp, ReadOnlySpan<byte> data);
		public FrameProducer CreateFrame(FrameInfo frame);
		public GroupFinished FinishAt(ulong end);
		public GroupFinished Finish();
		public void Abort(Error err);
		public ValueTask Ready(FrameInfo frame, CancellationToken cancellationToken = default);
		public nuint FrameCount();
		public GroupConsumer Consume();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public GroupDemand Demand();
		public ulong Start { get; }
	}
	public sealed unsafe partial class GroupFinished : RustObject {
		public void Abort(Error err);
		public nuint FrameCount();
		public GroupConsumer Consume();
		public ulong Start { get; }
	}
	public sealed unsafe partial class GroupDemand : RustObject {
		public GroupDemand Clone();
		public ulong Start();
		public bool Wanted();
		public ValueTask<bool> WantedChanged(CancellationToken cancellationToken = default);
		public ValueTask Closed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class GroupConsumer : RustObject {
		public GroupConsumer Clone();
		public Timescale? Timescale();
		public ClockInstant Arrived();
		public ulong Index();
		public GroupConsumer WithRange(BoundsU64 frames);
		public void SkipTo(ulong index);
		public nuint FrameCount();
		public ulong? End();
		public ValueTask<FrameConsumer?> NextFrame(CancellationToken cancellationToken = default);
		public ValueTask<FrameFrame?> ReadFrame(CancellationToken cancellationToken = default);
		public ValueTask<ulong> Finished(CancellationToken cancellationToken = default);
		public ulong Start { get; }
	}
	public sealed unsafe partial class GroupFetch : RustObject {
		public GroupFetch Clone();
		public string Debug();
		public static GroupFetch Default();
		public sbyte Priority { get; set; }
		public Duration? MaxDelay { get; set; }
		public GroupFetch WithPriority(sbyte priority);
		public GroupFetch WithMaxDelay(Duration maxDelay);
	}
	public sealed unsafe partial class TrackFinished : RustObject {
		public Str Name();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public TrackRequests Requests();
		public GroupProducer CreateGroup(GroupInfo group);
		public void Abort(Error err);
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public TrackConsumer Consume();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
	}
	public sealed unsafe partial class TrackRequests : RustObject {
		public TrackRequests Clone();
		public Str Name();
		public ValueTask<TrackFill> RequestedFill(CancellationToken cancellationToken = default);
		public TrackDemand Demand();
	}
	public sealed unsafe partial class TrackDemand : RustObject {
		public TrackDemand Clone();
		public Str Name();
		public bool Wanted();
		public ValueTask<bool> WantedChanged(CancellationToken cancellationToken = default);
		public ValueTask Refresh(CancellationToken cancellationToken = default);
		public ValueTask Closed(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class TrackSubscriber : RustObject {
		public StatsBill Bill();
		public TrackInfo Info();
		public Str Name();
		public BroadcastInfo Broadcast();
		public TrackControl Control();
		public ValueTask<GroupConsumer?> RecvGroup(CancellationToken cancellationToken = default);
		public TrackOrdered Ordered();
		public bool IsClone(TrackSubscriber other);
		public ValueTask<ulong?> Start(CancellationToken cancellationToken = default);
		public ulong? Floor();
		public ulong? NextInRange(ulong from);
		public bool IsSkipped(RangeU64 groups);
		public ValueTask<TrackPosition> Finished(CancellationToken cancellationToken = default);
		public TrackSubscriber WithRange(BoundsU64 groups);
		public TrackSubscription Subscription();
		public void Update(TrackSubscription subscription);
		public ValueTask<TrackPosition?> Largest(CancellationToken cancellationToken = default);
		public ValueTask Refresh(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class TrackOrdered : RustObject {
		public TrackInfo Info();
		public Str Name();
		public BroadcastInfo Broadcast();
		public ValueTask<TrackItem?> NextGroup(CancellationToken cancellationToken = default);
		public TrackControl Control();
		public ValueTask Refresh(CancellationToken cancellationToken = default);
		public TrackSubscription Subscription();
		public void Update(TrackSubscription subscription);
		public ValueTask<TrackPosition> Finished(CancellationToken cancellationToken = default);
		public ValueTask<TrackPosition?> Largest(CancellationToken cancellationToken = default);
		public bool IsSkipped(RangeU64 groups);
		public bool IsClone(TrackOrdered other);
	}
	public sealed unsafe partial class TrackItem : RustObject {
		public GroupConsumer? Group();
	}
	public sealed unsafe partial class TrackRequest : RustObject {
		public Str Name();
		public TrackConsumer Consume();
		public TrackRequests Requests();
		public TrackDemand Demand();
		public TrackProducer Accept(TrackInfo? info);
		public void Reject(Error err);
		public TrackSubscription? Subscription();
		public ValueTask<TrackSubscription?> SubscriptionChanged(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class TrackInfo : RustObject, IEquatable<TrackInfo> {
		public TrackInfo Clone();
		public bool Equals(TrackInfo? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackInfo? a, TrackInfo? b);
		public static bool operator !=(TrackInfo? a, TrackInfo? b);
		public string Debug();
		public static TrackInfo Default();
		public Timescale? Timescale { get; set; }
		public Duration? MaxAge { get; set; }
		public sbyte Priority { get; set; }
		public TrackCompression Compression { get; set; }
		public TrackRefresh Refresh { get; set; }
		public TrackInfo WithTimescale(Timescale? timescale);
		public TrackInfo WithMaxAge(Duration? maxAge);
		public TrackInfo WithPriority(sbyte priority);
		public TrackInfo WithCompression(TrackCompression compression);
		public TrackInfo WithRefresh(TrackRefresh refresh);
		public Timestamp Rescale(Timestamp timestamp);
		public Timestamp RescaleIn(Timestamp timestamp, ulong key);
	}
	public sealed unsafe partial class TrackRefresh : RustObject, IEquatable<TrackRefresh> {
		public TrackRefresh Clone();
		public bool Equals(TrackRefresh? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackRefresh? a, TrackRefresh? b);
		public static bool operator !=(TrackRefresh? a, TrackRefresh? b);
		public string Debug();
		public static TrackRefresh Default();
		public uint? Ratio { get; set; }
		public Duration? Interval { get; set; }
		public TrackRefresh WithRatio(uint? ratio);
		public TrackRefresh WithInterval(Duration? interval);
	}
	public sealed unsafe partial class TrackProducer : RustObject {
		public TrackProducer Clone();
		public Str Name();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void WriteFrame(Timestamp timestamp, ReadOnlySpan<byte> frame);
		public TrackFinished Finish();
		public TrackFinished Fail(Error err);
		public void FinishAt(TrackPosition end);
		public void StartAt(ulong? key);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackDemand Demand();
		public TrackConsumer Consume();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public ValueTask<TrackSubscription?> SubscriptionChanged(CancellationToken cancellationToken = default);
		public TrackRequests Requests();
	}
	public sealed unsafe partial class TrackAbortHandle : RustObject {
		public TrackAbortHandle Clone();
		public bool IsClosed_();
		public void Abort(Error err);
	}
	public sealed unsafe partial class TrackConsumer : RustObject {
		public TrackConsumer Clone();
		public TrackConsumer Unmetered();
		public Str Name();
		public BroadcastInfo Broadcast();
		public ValueTask<TrackSubscriber> Subscribe(TrackSubscription? subscription, CancellationToken cancellationToken = default);
		public ValueTask<GroupConsumer> FetchGroup(ulong key, GroupFetch? options, CancellationToken cancellationToken = default);
		public ValueTask<TrackInfo> Info(CancellationToken cancellationToken = default);
		public ValueTask<TrackPosition?> Largest(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class TrackControl : RustObject {
		public TrackControl Clone();
		public TrackSubscription Subscription();
		public void Update(TrackSubscription subscription);
		public ValueTask Refresh(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class TrackSubscription : RustObject, IEquatable<TrackSubscription> {
		public TrackSubscription Clone();
		public bool Equals(TrackSubscription? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TrackSubscription? a, TrackSubscription? b);
		public static bool operator !=(TrackSubscription? a, TrackSubscription? b);
		public string Debug();
		public static TrackSubscription Default();
		public sbyte Priority { get; set; }
		public TrackDelay Delay { get; set; }
		public TrackRange[] Ranges { get; set; }
		public TrackOrder Order { get; set; }
		public TrackSubscription WithPriority(sbyte priority);
		public TrackSubscription WithDelay(TrackDelay delay);
		public TrackSubscription WithRanges(TrackRange[] ranges);
		public TrackSubscription WithOrder(TrackOrder order);
	}
	public sealed unsafe partial class TrackFill : RustObject {
	}
	public sealed unsafe partial class OriginFollowing : RustObject {
		public ValueTask<BroadcastConsumer?> Available(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class OriginClaim : RustObject {
		public OriginClaim Clone();
		public Instance Instance();
		public void Announce();
		public void Withdraw();
		public bool IsAnnounced();
		public void SetCost(OriginCost cost);
		public void SetAuth(AuthCredential? auth);
		public ValueTask<OriginRequest?> Requested(CancellationToken cancellationToken = default);
	}
	public readonly struct NonZeroUsize : IEquatable<NonZeroUsize>, IComparable<NonZeroUsize> {
		public nuint Value { get; }
		public NonZeroUsize(nuint value);
		public static explicit operator NonZeroUsize(nuint value);
		public static implicit operator nuint(NonZeroUsize value);
		public override string ToString();
		public bool Equals(NonZeroUsize other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroUsize other);
	}
	public sealed unsafe partial class OriginConfig : RustObject {
		public OriginConfig Clone();
		public string Debug();
		public static OriginConfig Default();
		public NonZeroU64 Node { get; set; }
		public NonZeroUsize MaxHops { get; set; }
		public CachePool Pool { get; set; }
		public Duration? CacheDuration { get; set; }
		public static OriginConfig New(NonZeroU64 node);
	}
	public sealed unsafe partial class OriginConsumer : RustObject {
		public OriginConsumer Clone();
		public NonZeroU64 Node();
		public OriginConsumer WithHidden(bool hidden);
		public OriginConsumer Covering();
		public OriginConsumer Exclude(Str prefix);
		public OriginConsumer WithStats(StatsSession session);
		public AnnounceConsumer Announced();
		public OriginRoutes Routes();
		public OriginConsumer Consume();
		public OriginFollowing Follow(Str path, Instance? instance);
		public OriginConsumer Scope(Str root, Patterns patterns);
		public ValueTask<BroadcastConsumer> Request(Str path, Instance? instance, CancellationToken cancellationToken = default);
		public Path Root();
		public Patterns Allowed();
		public Path Absolute(Str path);
	}
	public sealed unsafe partial class OriginDriver : RustObject {
		public ClockInstant? Poll(ClockInstant now, Waiter waiter);
	}
	public sealed unsafe partial class OriginProducer : RustObject {
		public OriginProducer Clone();
		public static (OriginProducer, OriginDriver) New(OriginConfig config);
		public OriginProducer WithStats(StatsSession session);
		public OriginProducer WithLink(OriginLink link);
		public OriginProducer WithPriority(OriginPriority priority);
		public OriginProducer WithProxy(bool proxy);
		public OriginConfig Config();
		public NonZeroU64 Node();
		public bool IsOwn(NonZeroU64 id);
		public OriginRoute Route(Str path, OriginRouteOptions options);
		public OriginClaim Claim(Str prefix, OriginRouteOptions options);
		public OriginProducer Scope(Str root, Patterns patterns);
		public OriginProducer Exclude(Str prefix);
		public OriginProducer Mount(Str at, Str target);
		public OriginConsumer Consume();
		public Path Root();
		public Patterns Allowed();
		public Path Absolute(Str path);
	}
	public sealed unsafe partial class OriginRequest : RustObject {
		public Path Path();
		public OriginRoute Route(OriginRouteOptions options);
		public void ServeProducer(BroadcastProducer broadcast);
		public void ServeConsumer(BroadcastConsumer broadcast);
		public void Reject(Error err);
	}
	public sealed unsafe partial class OriginRoute : RustObject {
		public OriginRoute Clone();
		public BroadcastProducer Broadcast();
		public Instance Instance();
		public void Announce();
		public void Withdraw();
		public bool IsAnnounced();
		public void SetCost(OriginCost cost);
		public void SetAuth(AuthCredential? auth);
	}
	public sealed unsafe partial class OriginRouteInfo : RustObject, IEquatable<OriginRouteInfo> {
		public OriginRouteInfo Clone();
		public bool Equals(OriginRouteInfo? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(OriginRouteInfo? a, OriginRouteInfo? b);
		public static bool operator !=(OriginRouteInfo? a, OriginRouteInfo? b);
		public string Debug();
		public Instance Instance();
		public OriginCost Cost();
		public bool Available();
	}
	public sealed unsafe partial class OriginRouteOptions : RustObject {
		public OriginRouteOptions Clone();
		public string Debug();
		public static OriginRouteOptions Default();
		public static OriginRouteOptions Hidden();
		public OriginRouteOptions WithCost(OriginCost cost);
		public OriginRouteOptions WithAuth(AuthCredential auth);
		public OriginRouteOptions WithInstance(Instance instance, ulong epoch);
	}
	public sealed unsafe partial class OriginRouteEvent : RustObject {
		public OriginRouteEvent Clone();
		public string Debug();
		public OriginRouteKind Kind { get; set; }
		public AnnounceAnnounce Announce { get; set; }
	}
	public sealed unsafe partial class OriginRoutes : RustObject, IAsyncEnumerable<OriginRouteEvent> {
		public ValueTask<OriginRouteEvent?> Next(CancellationToken cancellationToken = default);
		public OriginRouteEvent? TryNext();
		public bool IsClosed_();
		public ValueTask PollSynced(CancellationToken cancellationToken = default);
		public Path Root();
		public Path Absolute(Str prefix);
		public IAsyncEnumerator<OriginRouteEvent> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AnnounceConsumer : RustObject, IAsyncEnumerable<AnnounceEvent> {
		public ValueTask<AnnounceEvent?> Next(CancellationToken cancellationToken = default);
		public AnnounceEvent? TryNext();
		public bool IsClosed_();
		public ValueTask PollSynced(CancellationToken cancellationToken = default);
		public Path Root();
		public Path Absolute(Str prefix);
		public IAsyncEnumerator<AnnounceEvent> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class AnnounceEvent : RustObject {
		public AnnounceEvent Clone();
		public string Debug();
	}
	public sealed unsafe partial class AnnounceAnnounce : RustObject {
		public AnnounceAnnounce Clone();
		public string Debug();
		public Path Prefix { get; set; }
		public Pattern[]? Captures { get; set; }
		public OriginRouteInfo Route { get; set; }
	}
	public sealed unsafe partial class HangStatsSnapshot : RustObject, IEquatable<HangStatsSnapshot> {
		public HangStatsSnapshot Clone();
		public bool Equals(HangStatsSnapshot? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangStatsSnapshot? a, HangStatsSnapshot? b);
		public static bool operator !=(HangStatsSnapshot? a, HangStatsSnapshot? b);
		public string Debug();
		public static HangStatsSnapshot Default();
		public HangStatsTransport Transport { get; set; }
		public (Str, HangStatsRendition)[] Renditions { get; set; }
	}
	public sealed unsafe partial class HangStatsRendition : RustObject, IEquatable<HangStatsRendition> {
		public HangStatsRendition Clone();
		public bool Equals(HangStatsRendition? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangStatsRendition? a, HangStatsRendition? b);
		public static bool operator !=(HangStatsRendition? a, HangStatsRendition? b);
		public string Debug();
		public static HangStatsRendition Default();
		public ulong Frames { get; set; }
		public ulong Bytes { get; set; }
		public ulong Keyframes { get; set; }
		public ulong Skipped { get; set; }
		public ulong? Bitrate { get; set; }
	}
	public sealed unsafe partial class HangStatsTransport : RustObject, IEquatable<HangStatsTransport> {
		public HangStatsTransport Clone();
		public bool Equals(HangStatsTransport? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangStatsTransport? a, HangStatsTransport? b);
		public static bool operator !=(HangStatsTransport? a, HangStatsTransport? b);
		public string Debug();
		public static HangStatsTransport Default();
		public Duration? Rtt { get; set; }
		public ulong? Rate { get; set; }
		public ulong? LostBytes { get; set; }
		public ulong? LostPackets { get; set; }
		public Duration? Age { get; set; }
	}
	public sealed unsafe partial class HangEchoSnapshot : RustObject, IEquatable<HangEchoSnapshot> {
		public HangEchoSnapshot Clone();
		public bool Equals(HangEchoSnapshot? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangEchoSnapshot? a, HangEchoSnapshot? b);
		public static bool operator !=(HangEchoSnapshot? a, HangEchoSnapshot? b);
		public string Debug();
		public static HangEchoSnapshot Default();
		public HangStatsTransport Transport { get; set; }
		public (Str, HangEchoRendition)[] Renditions { get; set; }
	}
	public sealed unsafe partial class HangEchoRendition : RustObject, IEquatable<HangEchoRendition> {
		public HangEchoRendition Clone();
		public bool Equals(HangEchoRendition? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangEchoRendition? a, HangEchoRendition? b);
		public static bool operator !=(HangEchoRendition? a, HangEchoRendition? b);
		public string Debug();
		public static HangEchoRendition Default();
		public ulong Frames { get; set; }
		public ulong Bytes { get; set; }
		public ulong Decoded { get; set; }
		public ulong Late { get; set; }
		public ulong Errors { get; set; }
		public ulong Stalls { get; set; }
		public ulong Underruns { get; set; }
		public Duration Stalled { get; set; }
		public HangEchoArrival? Arrival { get; set; }
	}
	public sealed unsafe partial class HangEchoArrival : RustObject, IEquatable<HangEchoArrival> {
		public HangEchoArrival Clone();
		public bool Equals(HangEchoArrival? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangEchoArrival? a, HangEchoArrival? b);
		public static bool operator !=(HangEchoArrival? a, HangEchoArrival? b);
		public string Debug();
		public Timestamp Timestamp { get; set; }
		public ClockWall Wall { get; set; }
		public static HangEchoArrival New(Timestamp timestamp, ClockWall wall);
	}
	public sealed unsafe partial class HangCatalogSkipped : RustObject, IEquatable<HangCatalogSkipped> {
		public HangCatalogSkipped Clone();
		public bool Equals(HangCatalogSkipped? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogSkipped? a, HangCatalogSkipped? b);
		public static bool operator !=(HangCatalogSkipped? a, HangCatalogSkipped? b);
		public string Debug();
		public static HangCatalogSkipped Default();
		public (Str, ulong)[] Video { get; set; }
		public (Str, ulong)[] Audio { get; set; }
		public (Str, ulong)[] Text { get; set; }
		public (Str, ulong)[] Json { get; set; }
		public (Str, ulong)[] Binary { get; set; }
		public (Str, ulong)[] Include { get; set; }
		public bool IsEmpty();
		public ulong? Version();
	}
	public sealed unsafe partial class HangCatalogConsumer : RustObject {
		public static HangCatalogConsumer New(TrackConsumer track, Duration delay);
		public ValueTask<MoqJsonTimedCatalog?> PollNext(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class HangCatalogAudio : RustObject, IEquatable<HangCatalogAudio> {
		public HangCatalogAudio Clone();
		public bool Equals(HangCatalogAudio? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAudio? a, HangCatalogAudio? b);
		public static bool operator !=(HangCatalogAudio? a, HangCatalogAudio? b);
		public string Debug();
		public static HangCatalogAudio Default();
		public (Str, HangCatalogAudioConfig)[] Renditions { get; set; }
		public void Insert(Str name, HangCatalogAudioConfig config);
		public HangCatalogAudioConfig? Remove(Str name);
		public bool IsEmpty();
		public (Str, HangCatalogAudioConfig)[] Enabled();
		public (Str, HangCatalogAudioConfig)[] Ranked();
	}
	public sealed unsafe partial class HangCatalogAudioConfig : RustObject, IEquatable<HangCatalogAudioConfig> {
		public HangCatalogAudioConfig Clone();
		public bool Equals(HangCatalogAudioConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogAudioConfig? a, HangCatalogAudioConfig? b);
		public static bool operator !=(HangCatalogAudioConfig? a, HangCatalogAudioConfig? b);
		public string Debug();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public Str? Label { get; set; }
		public HangCatalogAudioCodec Codec { get; set; }
		public uint SampleRate { get; set; }
		public uint ChannelCount { get; set; }
		public HangCatalogAudioLayout? Layout { get; set; }
		public ulong? Bitrate { get; set; }
		public bool Enabled { get; set; }
		public Bytes? Description { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Warmup { get; set; }
		public Duration? Delay { get; set; }
		public static HangCatalogAudioConfig New(HangCatalogAudioCodec codec, uint sampleRate, uint channelCount);
		public bool SameIdentity(HangCatalogAudioConfig other);
	}
	public sealed unsafe partial class HangCatalogBinary : RustObject, IEquatable<HangCatalogBinary> {
		public HangCatalogBinary Clone();
		public bool Equals(HangCatalogBinary? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogBinary? a, HangCatalogBinary? b);
		public static bool operator !=(HangCatalogBinary? a, HangCatalogBinary? b);
		public string Debug();
		public static HangCatalogBinary Default();
		public (Str, HangCatalogBinaryConfig)[] Tracks { get; set; }
		public void Insert(Str name, HangCatalogBinaryConfig config);
		public HangCatalogBinaryConfig? Remove(Str name);
		public bool IsEmpty();
	}
	public sealed unsafe partial class HangCatalogBinaryConfig : RustObject, IEquatable<HangCatalogBinaryConfig> {
		public HangCatalogBinaryConfig Clone();
		public bool Equals(HangCatalogBinaryConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogBinaryConfig? a, HangCatalogBinaryConfig? b);
		public static bool operator !=(HangCatalogBinaryConfig? a, HangCatalogBinaryConfig? b);
		public string Debug();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogMode Mode { get; set; }
		public Str? Mime { get; set; }
		public ulong? Bitrate { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public (Str, Json)[] Extra { get; set; }
		public static HangCatalogBinaryConfig New(HangCatalogMode mode);
	}
	public sealed unsafe partial class HangCatalogContainer : RustObject, IEquatable<HangCatalogContainer> {
		public HangCatalogContainer Clone();
		public bool Equals(HangCatalogContainer? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogContainer? a, HangCatalogContainer? b);
		public static bool operator !=(HangCatalogContainer? a, HangCatalogContainer? b);
		public string Debug();
		public static HangCatalogContainer Default();
	}
	public sealed unsafe partial class HangCatalogEcho : RustObject, IEquatable<HangCatalogEcho> {
		public HangCatalogEcho Clone();
		public bool Equals(HangCatalogEcho? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogEcho? a, HangCatalogEcho? b);
		public static bool operator !=(HangCatalogEcho? a, HangCatalogEcho? b);
		public string Debug();
		public PathRelative Path { get; set; }
		public static HangCatalogEcho New(PathRelative path);
	}
	public sealed unsafe partial class HangCatalogInclude : RustObject, IEquatable<HangCatalogInclude> {
		public HangCatalogInclude Clone();
		public bool Equals(HangCatalogInclude? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogInclude? a, HangCatalogInclude? b);
		public static bool operator !=(HangCatalogInclude? a, HangCatalogInclude? b);
		public string Debug();
		public PathRelative Broadcast { get; set; }
		public Instance? Instance { get; set; }
		public static HangCatalogInclude New(PathRelative broadcast);
	}
	public sealed unsafe partial class HangCatalogJson : RustObject, IEquatable<HangCatalogJson> {
		public HangCatalogJson Clone();
		public bool Equals(HangCatalogJson? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogJson? a, HangCatalogJson? b);
		public static bool operator !=(HangCatalogJson? a, HangCatalogJson? b);
		public string Debug();
		public static HangCatalogJson Default();
		public (Str, HangCatalogJsonConfig)[] Tracks { get; set; }
		public void Insert(Str name, HangCatalogJsonConfig config);
		public HangCatalogJsonConfig? Remove(Str name);
		public bool IsEmpty();
	}
	public sealed unsafe partial class HangCatalogJsonConfig : RustObject, IEquatable<HangCatalogJsonConfig> {
		public HangCatalogJsonConfig Clone();
		public bool Equals(HangCatalogJsonConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogJsonConfig? a, HangCatalogJsonConfig? b);
		public static bool operator !=(HangCatalogJsonConfig? a, HangCatalogJsonConfig? b);
		public string Debug();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogMode Mode { get; set; }
		public Str? Schema { get; set; }
		public ulong? Bitrate { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public (Str, Json)[] Extra { get; set; }
		public static HangCatalogJsonConfig New(HangCatalogMode mode);
	}
	public sealed unsafe partial class HangCatalogStats : RustObject, IEquatable<HangCatalogStats> {
		public HangCatalogStats Clone();
		public bool Equals(HangCatalogStats? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogStats? a, HangCatalogStats? b);
		public static bool operator !=(HangCatalogStats? a, HangCatalogStats? b);
		public string Debug();
		public Str Track { get; set; }
		public static HangCatalogStats New(Str track);
	}
	public sealed unsafe partial class HangCatalogText : RustObject, IEquatable<HangCatalogText> {
		public HangCatalogText Clone();
		public bool Equals(HangCatalogText? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogText? a, HangCatalogText? b);
		public static bool operator !=(HangCatalogText? a, HangCatalogText? b);
		public string Debug();
		public static HangCatalogText Default();
		public (Str, HangCatalogTextConfig)[] Renditions { get; set; }
		public void Insert(Str name, HangCatalogTextConfig config);
		public HangCatalogTextConfig? Remove(Str name);
		public bool IsEmpty();
	}
	public sealed unsafe partial class HangCatalogTextConfig : RustObject, IEquatable<HangCatalogTextConfig> {
		public HangCatalogTextConfig Clone();
		public bool Equals(HangCatalogTextConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogTextConfig? a, HangCatalogTextConfig? b);
		public static bool operator !=(HangCatalogTextConfig? a, HangCatalogTextConfig? b);
		public string Debug();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogTextFormat Format { get; set; }
		public HangCatalogTextRole Role { get; set; }
		public Str? Lang { get; set; }
		public Str? Label { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public static HangCatalogTextConfig New(HangCatalogTextFormat format);
	}
	public sealed unsafe partial class HangCatalogTimeline : RustObject, IEquatable<HangCatalogTimeline> {
		public HangCatalogTimeline Clone();
		public bool Equals(HangCatalogTimeline? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogTimeline? a, HangCatalogTimeline? b);
		public static bool operator !=(HangCatalogTimeline? a, HangCatalogTimeline? b);
		public string Debug();
		public static HangCatalogTimeline Default();
		public HangCatalogClock? Clock { get; set; }
		public Duration? Interval { get; set; }
		public bool IsEmpty();
	}
	public sealed unsafe partial class HangCatalogColor : RustObject, IEquatable<HangCatalogColor> {
		public HangCatalogColor Clone();
		public bool Equals(HangCatalogColor? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogColor? a, HangCatalogColor? b);
		public static bool operator !=(HangCatalogColor? a, HangCatalogColor? b);
		public string Debug();
		public static HangCatalogColor Default();
		public byte? Primaries { get; set; }
		public byte? Transfer { get; set; }
		public byte? Matrix { get; set; }
		public HangCatalogColorRange? Range { get; set; }
		public HangCatalogMastering? Mastering { get; set; }
		public HangCatalogContentLight? ContentLight { get; set; }
	}
	public sealed unsafe partial class HangCatalogProjection : RustObject, IEquatable<HangCatalogProjection> {
		public HangCatalogProjection Clone();
		public bool Equals(HangCatalogProjection? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogProjection? a, HangCatalogProjection? b);
		public static bool operator !=(HangCatalogProjection? a, HangCatalogProjection? b);
		public string Debug();
		public static HangCatalogProjection Default();
	}
	public sealed unsafe partial class HangCatalogVideo : RustObject, IEquatable<HangCatalogVideo> {
		public HangCatalogVideo Clone();
		public bool Equals(HangCatalogVideo? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogVideo? a, HangCatalogVideo? b);
		public static bool operator !=(HangCatalogVideo? a, HangCatalogVideo? b);
		public string Debug();
		public static HangCatalogVideo Default();
		public (Str, HangCatalogVideoConfig)[] Renditions { get; set; }
		public HangCatalogDisplay? Display { get; set; }
		public double? Rotation { get; set; }
		public bool? Flip { get; set; }
		public void Insert(Str name, HangCatalogVideoConfig config);
		public HangCatalogVideoConfig? Remove(Str name);
		public bool IsEmpty();
		public (Str, HangCatalogVideoConfig)[] Enabled();
		public (Str, HangCatalogVideoConfig)[] Ranked();
		public void SetProperties(HangCatalogVideoProperties properties);
	}
	public sealed unsafe partial class HangCatalogVideoProperties : RustObject, IEquatable<HangCatalogVideoProperties> {
		public HangCatalogVideoProperties Clone();
		public bool Equals(HangCatalogVideoProperties? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogVideoProperties? a, HangCatalogVideoProperties? b);
		public static bool operator !=(HangCatalogVideoProperties? a, HangCatalogVideoProperties? b);
		public string Debug();
		public static HangCatalogVideoProperties Default();
		public HangCatalogDisplay? Display { get; set; }
		public double? Rotation { get; set; }
		public bool? Flip { get; set; }
	}
	public sealed unsafe partial class HangCatalogVideoConfig : RustObject, IEquatable<HangCatalogVideoConfig> {
		public HangCatalogVideoConfig Clone();
		public bool Equals(HangCatalogVideoConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(HangCatalogVideoConfig? a, HangCatalogVideoConfig? b);
		public static bool operator !=(HangCatalogVideoConfig? a, HangCatalogVideoConfig? b);
		public string Debug();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public Str? Label { get; set; }
		public HangCatalogVideoCodec Codec { get; set; }
		public Bytes? Description { get; set; }
		public uint? CodedWidth { get; set; }
		public uint? CodedHeight { get; set; }
		public uint? DisplayAspectWidth { get; set; }
		public uint? DisplayAspectHeight { get; set; }
		public HangCatalogColor? Color { get; set; }
		public HangCatalogProjection? Projection { get; set; }
		public HangCatalogStereo? Stereo { get; set; }
		public bool Interlaced { get; set; }
		public ulong? Bitrate { get; set; }
		public bool Enabled { get; set; }
		public int Preference { get; set; }
		public double? Framerate { get; set; }
		public byte? Reorder { get; set; }
		public bool? OptimizeForLatency { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public static HangCatalogVideoConfig New(HangCatalogVideoCodec codec);
		public bool SameIdentity(HangCatalogVideoConfig other);
	}
	public sealed unsafe partial class MuxCatalogConsumer : RustObject {
		public static ValueTask<MuxCatalogConsumer> New(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogConsumerExtra : RustObject {
		public static ValueTask<MuxCatalogConsumerExtra> New(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogConsumerExt : RustObject {
		public static ValueTask<MuxCatalogConsumerExt> New(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogEstimate : RustObject, IEquatable<MuxCatalogEstimate> {
		public MuxCatalogEstimate Clone();
		public bool Equals(MuxCatalogEstimate? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCatalogEstimate? a, MuxCatalogEstimate? b);
		public static bool operator !=(MuxCatalogEstimate? a, MuxCatalogEstimate? b);
		public string Debug();
		public static MuxCatalogEstimate Default();
		public Duration? Jitter { get; set; }
		public ulong? Bitrate { get; set; }
		public Duration? Delay { get; set; }
		public double? Framerate { get; set; }
		public MuxCatalogEstimate WithJitter(Duration? jitter);
		public MuxCatalogEstimate WithBitrate(ulong? bitrate);
		public MuxCatalogEstimate WithDelay(Duration? delay);
		public MuxCatalogEstimate WithFramerate(double? framerate);
	}
	public sealed unsafe partial class MuxCatalogConfig : RustObject {
		public MuxCatalogConfig Clone();
		public static MuxCatalogConfig Default();
		public MuxCatalogConfigExtra WithCatalogExtra(HangCatalogExtra catalog);
		public MuxCatalogConfigExt WithCatalogExt(HangCatalogExt catalog);
		public MuxCatalogConfig WithCatalogUnit(HangCatalog catalog);
		public MuxCatalogConfig WithClock(MuxClock clock);
		public MuxCatalogConfig WithMaxAge(Duration? maxAge);
		public MuxCatalogConfig WithBandwidth(BandwidthAllocator bandwidth);
	}
	public sealed unsafe partial class MuxCatalogConfigExtra : RustObject {
		public MuxCatalogConfigExtra Clone();
		public MuxCatalogConfigExtra WithCatalogExtra(HangCatalogExtra catalog);
		public MuxCatalogConfigExt WithCatalogExt(HangCatalogExt catalog);
		public MuxCatalogConfig WithCatalogUnit(HangCatalog catalog);
		public MuxCatalogConfigExtra WithClock(MuxClock clock);
		public MuxCatalogConfigExtra WithMaxAge(Duration? maxAge);
		public MuxCatalogConfigExtra WithBandwidth(BandwidthAllocator bandwidth);
	}
	public sealed unsafe partial class MuxCatalogConfigExt : RustObject {
		public MuxCatalogConfigExt Clone();
		public MuxCatalogConfigExtra WithCatalogExtra(HangCatalogExtra catalog);
		public MuxCatalogConfigExt WithCatalogExt(HangCatalogExt catalog);
		public MuxCatalogConfig WithCatalogUnit(HangCatalog catalog);
		public MuxCatalogConfigExt WithClock(MuxClock clock);
		public MuxCatalogConfigExt WithMaxAge(Duration? maxAge);
		public MuxCatalogConfigExt WithBandwidth(BandwidthAllocator bandwidth);
	}
	public sealed unsafe partial class MuxCatalogGuard : RustObject {
		public void Commit();
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class MuxCatalogGuardExtra : RustObject {
		public void Commit();
		public void SetSection(Str name, Json value);
		public Json? RemoveSection(Str name);
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public MuxCatalogHangExtra Ext { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class MuxCatalogGuardExt : RustObject {
		public void Commit();
		public HangCatalogVideo Video { get; set; }
		public HangCatalogAudio Audio { get; set; }
		public HangCatalogTimeline Sync { get; set; }
		public HangCatalogText Text { get; set; }
		public HangCatalogJson Json { get; set; }
		public HangCatalogBinary Binary { get; set; }
		public HangCatalogStats? Stats { get; set; }
		public HangCatalogEcho? Echo { get; set; }
		public (Str, HangCatalogInclude)[] Include { get; set; }
		public MuxContainerTsExt Ext { get; set; }
		public HangCatalogSkipped Skipped { get; set; }
		public void CheckRenditions();
		public HangCatalog WithoutExt();
		public Str ToJson();
		public Bytes ToVec();
		public void ToWriter(ReadOnlySpan<byte> writer);
	}
	public sealed unsafe partial class MuxCatalogProducer : RustObject {
		public MuxCatalogProducer Clone();
		public static MuxCatalogProducer New(BroadcastProducer broadcast, MuxCatalogConfig config);
		public TrackInfo TrackInfo(sbyte priority);
		public Timestamp Timestamp(Timestamp? hint);
		public void Mutate(Action<HangCatalog> f);
		public MuxCatalogGuard Modify();
		public HangCatalog Snapshot();
		public MuxClock Clock();
		public MuxCatalogTimebase Timebase();
		public MuxCatalogReserved Reserve();
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public bool IsClaimedJsonConfig(Str name);
		public bool IsClaimedBinaryConfig(Str name);
		public bool IsClaimedVideoConfig(Str name);
		public bool IsClaimedAudioConfig(Str name);
		public bool IsClaimedTextConfig(Str name);
		public (Str, HangStatsRendition)[] Stats();
		public MuxJsonSnapshotJson JsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
		public MuxJsonSnapshotJson JsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
		public MuxJsonStreamJson JsonStreamConfig(TrackProducer track, MuxJsonConfig config);
		public MuxJsonStreamJson JsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
		public MuxCatalogHangConsumer Consume();
		public void Abort(Error err);
		public void Finish();
	}
	public sealed unsafe partial class MuxCatalogProducerExtra : RustObject {
		public MuxCatalogProducerExtra Clone();
		public static MuxCatalogProducerExtra New(BroadcastProducer broadcast, MuxCatalogConfigExtra config);
		public TrackInfo TrackInfo(sbyte priority);
		public Timestamp Timestamp(Timestamp? hint);
		public void Mutate(Action<HangCatalogExtra> f);
		public MuxCatalogGuardExtra Modify();
		public HangCatalogExtra Snapshot();
		public MuxClock Clock();
		public MuxCatalogTimebaseExtra Timebase();
		public MuxCatalogReservedExtra Reserve();
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public bool IsClaimedJsonConfig(Str name);
		public bool IsClaimedBinaryConfig(Str name);
		public bool IsClaimedVideoConfig(Str name);
		public bool IsClaimedAudioConfig(Str name);
		public bool IsClaimedTextConfig(Str name);
		public (Str, HangStatsRendition)[] Stats();
		public MuxJsonSnapshotJson JsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
		public MuxJsonSnapshotJson JsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
		public MuxJsonStreamJson JsonStreamConfig(TrackProducer track, MuxJsonConfig config);
		public MuxJsonStreamJson JsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
		public MuxCatalogHangConsumerExtra Consume();
		public void Abort(Error err);
		public void Finish();
	}
	public sealed unsafe partial class MuxCatalogProducerExt : RustObject {
		public MuxCatalogProducerExt Clone();
		public static MuxCatalogProducerExt New(BroadcastProducer broadcast, MuxCatalogConfigExt config);
		public TrackInfo TrackInfo(sbyte priority);
		public Timestamp Timestamp(Timestamp? hint);
		public void Mutate(Action<HangCatalogExt> f);
		public MuxCatalogGuardExt Modify();
		public HangCatalogExt Snapshot();
		public MuxClock Clock();
		public MuxCatalogTimebaseExt Timebase();
		public MuxCatalogReservedExt Reserve();
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public bool IsClaimedJsonConfig(Str name);
		public bool IsClaimedBinaryConfig(Str name);
		public bool IsClaimedVideoConfig(Str name);
		public bool IsClaimedAudioConfig(Str name);
		public bool IsClaimedTextConfig(Str name);
		public (Str, HangStatsRendition)[] Stats();
		public MuxJsonSnapshotJson JsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
		public MuxJsonSnapshotJson JsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
		public MuxJsonStreamJson JsonStreamConfig(TrackProducer track, MuxJsonConfig config);
		public MuxJsonStreamJson JsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
		public MuxCatalogHangConsumerExt Consume();
		public void Abort(Error err);
		public void Finish();
	}
	public sealed unsafe partial class MuxCatalogTimebase : RustObject {
		public MuxCatalogTimebase Clone();
		public MuxCatalogReserved Reserve();
		public void Place(Timestamp pts, ClockWall wall);
	}
	public sealed unsafe partial class MuxCatalogTimebaseExtra : RustObject {
		public MuxCatalogTimebaseExtra Clone();
		public MuxCatalogReservedExtra Reserve();
		public void Place(Timestamp pts, ClockWall wall);
	}
	public sealed unsafe partial class MuxCatalogTimebaseExt : RustObject {
		public MuxCatalogTimebaseExt Clone();
		public MuxCatalogReservedExt Reserve();
		public void Place(Timestamp pts, ClockWall wall);
	}
	public sealed unsafe partial class MuxCatalogReserved : RustObject {
		public MuxCatalogReserved Clone();
		public MuxCatalogReserved WithInterval(Duration? interval);
		public TrackInfo TrackInfo(sbyte priority);
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public Timestamp Timestamp(Timestamp? hint);
	}
	public sealed unsafe partial class MuxCatalogReservedExtra : RustObject {
		public MuxCatalogReservedExtra Clone();
		public MuxCatalogReservedExtra WithInterval(Duration? interval);
		public TrackInfo TrackInfo(sbyte priority);
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public Timestamp Timestamp(Timestamp? hint);
	}
	public sealed unsafe partial class MuxCatalogReservedExt : RustObject {
		public MuxCatalogReservedExt Clone();
		public MuxCatalogReservedExt WithInterval(Duration? interval);
		public TrackInfo TrackInfo(sbyte priority);
		public MuxContainerProducerContainerVideoConfig VideoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig VideoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig VideoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireVideoConfig VideoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig AudioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig AudioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig AudioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireAudioConfig AudioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TextContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TextFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TextLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireTextConfig TextLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerContainerJsonConfig TrackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
		public MuxContainerProducerContainerBinaryConfig TrackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerContainerVideoConfig TrackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
		public MuxContainerProducerContainerAudioConfig TrackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
		public MuxContainerProducerContainerTextConfig TrackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
		public MuxContainerProducerFmp4WireJsonConfig TrackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerFmp4WireBinaryConfig TrackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerFmp4WireVideoConfig TrackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerFmp4WireAudioConfig TrackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerFmp4WireTextConfig TrackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLegacyWireJsonConfig TrackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLegacyWireBinaryConfig TrackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLegacyWireVideoConfig TrackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLegacyWireAudioConfig TrackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLegacyWireTextConfig TrackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
		public MuxContainerProducerLocWireJsonConfig TrackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
		public MuxContainerProducerLocWireBinaryConfig TrackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
		public MuxContainerProducerLocWireVideoConfig TrackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
		public MuxContainerProducerLocWireAudioConfig TrackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
		public MuxContainerProducerLocWireTextConfig TrackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
		public Timestamp Timestamp(Timestamp? hint);
	}
	public sealed unsafe partial class MuxCatalogVideoHint : RustObject, IEquatable<MuxCatalogVideoHint> {
		public MuxCatalogVideoHint Clone();
		public bool Equals(MuxCatalogVideoHint? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCatalogVideoHint? a, MuxCatalogVideoHint? b);
		public static bool operator !=(MuxCatalogVideoHint? a, MuxCatalogVideoHint? b);
		public string Debug();
		public static MuxCatalogVideoHint Default();
		public HangCatalogVideoCodec? Codec { get; set; }
		public uint? CodedWidth { get; set; }
		public uint? CodedHeight { get; set; }
		public uint? DisplayAspectWidth { get; set; }
		public uint? DisplayAspectHeight { get; set; }
		public HangCatalogColor? Color { get; set; }
		public ulong? Bitrate { get; set; }
		public double? Framerate { get; set; }
		public byte? Reorder { get; set; }
		public bool? OptimizeForLatency { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public HangCatalogContainer Container { get; set; }
		public int Preference { get; set; }
		public void Apply(HangCatalogVideoConfig config);
	}
	public sealed unsafe partial class MuxContainerConsumerContainer : RustObject {
		public static MuxContainerConsumerContainer New(TrackSubscriber track, MuxCatalogHangContainer format);
		public ClockInstant? Arrival();
		public ClockInstant? Backlog();
		public ValueTask<MuxContainerRead?> Read(CancellationToken cancellationToken = default);
		public bool IsSkipped(RangeU64 groups);
		public void SetDelay(TrackDelay delay);
		public void SetPlayhead(MuxSyncConsumer playhead);
	}
	public sealed unsafe partial class MuxContainerConsumerFmp4Wire : RustObject {
		public static MuxContainerConsumerFmp4Wire New(TrackSubscriber track, MuxContainerFmp4Wire format);
		public ClockInstant? Arrival();
		public ClockInstant? Backlog();
		public ValueTask<MuxContainerRead?> Read(CancellationToken cancellationToken = default);
		public bool IsSkipped(RangeU64 groups);
		public void SetDelay(TrackDelay delay);
		public void SetPlayhead(MuxSyncConsumer playhead);
	}
	public sealed unsafe partial class MuxContainerConsumerLegacyWire : RustObject {
		public static MuxContainerConsumerLegacyWire New(TrackSubscriber track, MuxContainerLegacyWire format);
		public ClockInstant? Arrival();
		public ClockInstant? Backlog();
		public ValueTask<MuxContainerRead?> Read(CancellationToken cancellationToken = default);
		public bool IsSkipped(RangeU64 groups);
		public void SetDelay(TrackDelay delay);
		public void SetPlayhead(MuxSyncConsumer playhead);
	}
	public sealed unsafe partial class MuxContainerConsumerLocWire : RustObject {
		public static MuxContainerConsumerLocWire New(TrackSubscriber track, MuxContainerLocWire format);
		public ClockInstant? Arrival();
		public ClockInstant? Backlog();
		public ValueTask<MuxContainerRead?> Read(CancellationToken cancellationToken = default);
		public bool IsSkipped(RangeU64 groups);
		public void SetDelay(TrackDelay delay);
		public void SetPlayhead(MuxSyncConsumer playhead);
	}
	public sealed unsafe partial class MuxContainerGroupConsumerContainer : RustObject {
		public static MuxContainerGroupConsumerContainer New(GroupConsumer group, MuxCatalogHangContainer format);
		public ulong Start();
		public ValueTask<MuxContainerFrame?> Read(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerGroupConsumerFmp4Wire : RustObject {
		public static MuxContainerGroupConsumerFmp4Wire New(GroupConsumer group, MuxContainerFmp4Wire format);
		public ulong Start();
		public ValueTask<MuxContainerFrame?> Read(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerGroupConsumerLegacyWire : RustObject {
		public static MuxContainerGroupConsumerLegacyWire New(GroupConsumer group, MuxContainerLegacyWire format);
		public ulong Start();
		public ValueTask<MuxContainerFrame?> Read(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerGroupConsumerLocWire : RustObject {
		public static MuxContainerGroupConsumerLocWire New(GroupConsumer group, MuxContainerLocWire format);
		public ulong Start();
		public ValueTask<MuxContainerFrame?> Read(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerFinishedContainerAudioConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedContainerBinaryConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedContainerJsonConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedContainerTextConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedContainerVideoConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedContainer : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4WireAudioConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4WireBinaryConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4WireJsonConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4WireTextConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4WireVideoConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedFmp4Wire : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWireAudioConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWireBinaryConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWireJsonConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWireTextConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWireVideoConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLegacyWire : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWireAudioConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWireBinaryConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWireJsonConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWireTextConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWireVideoConfig : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFinishedLocWire : RustObject {
		public Str Name();
		public TrackDemand Demand();
		public TrackInfo Info();
		public TrackSubscriber Consume();
		public MuxCatalogEstimate Estimate();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerGuardUnit : RustObject {
		public void Commit();
	}
	public sealed unsafe partial class MuxContainerGuardAudioConfig : RustObject {
		public void Commit();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public Str? Label { get; set; }
		public HangCatalogAudioCodec Codec { get; set; }
		public uint SampleRate { get; set; }
		public uint ChannelCount { get; set; }
		public HangCatalogAudioLayout? Layout { get; set; }
		public ulong? Bitrate { get; set; }
		public bool Enabled { get; set; }
		public Bytes? Description { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Warmup { get; set; }
		public Duration? Delay { get; set; }
		public bool SameIdentity(HangCatalogAudioConfig other);
	}
	public sealed unsafe partial class MuxContainerGuardBinaryConfig : RustObject {
		public void Commit();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogMode Mode { get; set; }
		public Str? Mime { get; set; }
		public ulong? Bitrate { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public (Str, Json)[] Extra { get; set; }
	}
	public sealed unsafe partial class MuxContainerGuardJsonConfig : RustObject {
		public void Commit();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogMode Mode { get; set; }
		public Str? Schema { get; set; }
		public ulong? Bitrate { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public (Str, Json)[] Extra { get; set; }
	}
	public sealed unsafe partial class MuxContainerGuardTextConfig : RustObject {
		public void Commit();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public HangCatalogTextFormat Format { get; set; }
		public HangCatalogTextRole Role { get; set; }
		public Str? Lang { get; set; }
		public Str? Label { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
	}
	public sealed unsafe partial class MuxContainerGuardVideoConfig : RustObject {
		public void Commit();
		public PathRelative? Broadcast { get; set; }
		public Str? Track { get; set; }
		public Instance? Instance { get; set; }
		public Str? Label { get; set; }
		public HangCatalogVideoCodec Codec { get; set; }
		public Bytes? Description { get; set; }
		public uint? CodedWidth { get; set; }
		public uint? CodedHeight { get; set; }
		public uint? DisplayAspectWidth { get; set; }
		public uint? DisplayAspectHeight { get; set; }
		public HangCatalogColor? Color { get; set; }
		public HangCatalogProjection? Projection { get; set; }
		public HangCatalogStereo? Stereo { get; set; }
		public bool Interlaced { get; set; }
		public ulong? Bitrate { get; set; }
		public bool Enabled { get; set; }
		public int Preference { get; set; }
		public double? Framerate { get; set; }
		public byte? Reorder { get; set; }
		public bool? OptimizeForLatency { get; set; }
		public HangCatalogContainer Container { get; set; }
		public Duration? Jitter { get; set; }
		public Duration? Delay { get; set; }
		public bool SameIdentity(HangCatalogVideoConfig other);
	}
	public sealed unsafe partial class MuxContainerProducerContainerAudioConfig : RustObject {
		public void Set(HangCatalogAudioConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardAudioConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainerAudioConfig WithBuffer(Duration duration);
		public MuxContainerProducerContainerAudioConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainerAudioConfig Finish();
		public MuxContainerFinishedContainerAudioConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerContainerBinaryConfig : RustObject {
		public void Set(HangCatalogBinaryConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardBinaryConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainerBinaryConfig WithBuffer(Duration duration);
		public MuxContainerProducerContainerBinaryConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainerBinaryConfig Finish();
		public MuxContainerFinishedContainerBinaryConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerContainerJsonConfig : RustObject {
		public void Set(HangCatalogJsonConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardJsonConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainerJsonConfig WithBuffer(Duration duration);
		public MuxContainerProducerContainerJsonConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainerJsonConfig Finish();
		public MuxContainerFinishedContainerJsonConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerContainerTextConfig : RustObject {
		public void Set(HangCatalogTextConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardTextConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainerTextConfig WithBuffer(Duration duration);
		public MuxContainerProducerContainerTextConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainerTextConfig Finish();
		public MuxContainerFinishedContainerTextConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerContainerVideoConfig : RustObject {
		public void Set(HangCatalogVideoConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardVideoConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainerVideoConfig WithBuffer(Duration duration);
		public MuxContainerProducerContainerVideoConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public MuxContainerFinishedContainerVideoConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerContainer : RustObject {
		public static MuxContainerProducerContainer New(TrackProducer track, MuxCatalogHangContainer container);
		public void Set();
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardUnit Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerContainer WithBuffer(Duration duration);
		public MuxContainerProducerContainer WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedContainer Finish();
		public MuxContainerFinishedContainer Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4WireAudioConfig : RustObject {
		public void Set(HangCatalogAudioConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardAudioConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4WireAudioConfig WithBuffer(Duration duration);
		public MuxContainerProducerFmp4WireAudioConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4WireAudioConfig Finish();
		public MuxContainerFinishedFmp4WireAudioConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4WireBinaryConfig : RustObject {
		public void Set(HangCatalogBinaryConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardBinaryConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4WireBinaryConfig WithBuffer(Duration duration);
		public MuxContainerProducerFmp4WireBinaryConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4WireBinaryConfig Finish();
		public MuxContainerFinishedFmp4WireBinaryConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4WireJsonConfig : RustObject {
		public void Set(HangCatalogJsonConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardJsonConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4WireJsonConfig WithBuffer(Duration duration);
		public MuxContainerProducerFmp4WireJsonConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4WireJsonConfig Finish();
		public MuxContainerFinishedFmp4WireJsonConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4WireTextConfig : RustObject {
		public void Set(HangCatalogTextConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardTextConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4WireTextConfig WithBuffer(Duration duration);
		public MuxContainerProducerFmp4WireTextConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4WireTextConfig Finish();
		public MuxContainerFinishedFmp4WireTextConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4WireVideoConfig : RustObject {
		public void Set(HangCatalogVideoConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardVideoConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4WireVideoConfig WithBuffer(Duration duration);
		public MuxContainerProducerFmp4WireVideoConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4WireVideoConfig Finish();
		public MuxContainerFinishedFmp4WireVideoConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerFmp4Wire : RustObject {
		public static MuxContainerProducerFmp4Wire New(TrackProducer track, MuxContainerFmp4Wire container);
		public void Set();
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardUnit Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerFmp4Wire WithBuffer(Duration duration);
		public MuxContainerProducerFmp4Wire WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedFmp4Wire Finish();
		public MuxContainerFinishedFmp4Wire Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWireAudioConfig : RustObject {
		public void Set(HangCatalogAudioConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardAudioConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWireAudioConfig WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWireAudioConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWireAudioConfig Finish();
		public MuxContainerFinishedLegacyWireAudioConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWireBinaryConfig : RustObject {
		public void Set(HangCatalogBinaryConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardBinaryConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWireBinaryConfig WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWireBinaryConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWireBinaryConfig Finish();
		public MuxContainerFinishedLegacyWireBinaryConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWireJsonConfig : RustObject {
		public void Set(HangCatalogJsonConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardJsonConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWireJsonConfig WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWireJsonConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWireJsonConfig Finish();
		public MuxContainerFinishedLegacyWireJsonConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWireTextConfig : RustObject {
		public void Set(HangCatalogTextConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardTextConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWireTextConfig WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWireTextConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWireTextConfig Finish();
		public MuxContainerFinishedLegacyWireTextConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWireVideoConfig : RustObject {
		public void Set(HangCatalogVideoConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardVideoConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWireVideoConfig WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWireVideoConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWireVideoConfig Finish();
		public MuxContainerFinishedLegacyWireVideoConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLegacyWire : RustObject {
		public static MuxContainerProducerLegacyWire New(TrackProducer track, MuxContainerLegacyWire container);
		public void Set();
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardUnit Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLegacyWire WithBuffer(Duration duration);
		public MuxContainerProducerLegacyWire WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLegacyWire Finish();
		public MuxContainerFinishedLegacyWire Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWireAudioConfig : RustObject {
		public void Set(HangCatalogAudioConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardAudioConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWireAudioConfig WithBuffer(Duration duration);
		public MuxContainerProducerLocWireAudioConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWireAudioConfig Finish();
		public MuxContainerFinishedLocWireAudioConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWireBinaryConfig : RustObject {
		public void Set(HangCatalogBinaryConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardBinaryConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWireBinaryConfig WithBuffer(Duration duration);
		public MuxContainerProducerLocWireBinaryConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWireBinaryConfig Finish();
		public MuxContainerFinishedLocWireBinaryConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWireJsonConfig : RustObject {
		public void Set(HangCatalogJsonConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardJsonConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWireJsonConfig WithBuffer(Duration duration);
		public MuxContainerProducerLocWireJsonConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWireJsonConfig Finish();
		public MuxContainerFinishedLocWireJsonConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWireTextConfig : RustObject {
		public void Set(HangCatalogTextConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardTextConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWireTextConfig WithBuffer(Duration duration);
		public MuxContainerProducerLocWireTextConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWireTextConfig Finish();
		public MuxContainerFinishedLocWireTextConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWireVideoConfig : RustObject {
		public void Set(HangCatalogVideoConfig config);
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardVideoConfig Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWireVideoConfig WithBuffer(Duration duration);
		public MuxContainerProducerLocWireVideoConfig WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWireVideoConfig Finish();
		public MuxContainerFinishedLocWireVideoConfig Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxContainerProducerLocWire : RustObject {
		public static MuxContainerProducerLocWire New(TrackProducer track, MuxContainerLocWire container);
		public void Set();
		public void SetBitrate(ulong bitrate);
		public MuxContainerGuardUnit Modify();
		public Timestamp Timestamp(Timestamp? hint);
		public Str Name();
		public TrackDemand Demand();
		public MuxCatalogEstimate Estimate();
		public void Reorder(Timestamp delay);
		public void History(Timestamp until);
		public bool NeedsKeyframe();
		public MuxContainerProducerLocWire WithBuffer(Duration duration);
		public MuxContainerProducerLocWire WithBandwidth(BandwidthAllocator allocator);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public TrackProducer Track();
		public Timestamp? LiveEdge();
		public void Write(MuxContainerFrame frame);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public void Discontinuity();
		public MuxContainerFinishedLocWire Finish();
		public MuxContainerFinishedLocWire Fail(Error err);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public TrackSubscriber Consume();
		public BroadcastInfo Broadcast();
		public TrackInfo Info();
		public GroupProducer CreateGroup(GroupInfo group);
		public GroupProducer AppendGroup(Timestamp start);
		public void SkipGroups(RangeU64 groups);
		public TrackPosition? Finished();
		public ValueTask Closed(CancellationToken cancellationToken = default);
		public bool IsClosed_();
		public TrackPosition? Largest();
		public bool IsClone(TrackProducer other);
		public TrackAbortHandle AbortHandle();
		public TrackSubscriber Subscribe(TrackSubscription? subscription);
		public TrackSubscription? Subscription();
		public TrackRequests Requests();
	}
	public sealed unsafe partial class MuxFollowInstances : RustObject, IAsyncEnumerable<BroadcastConsumer> {
		public ValueTask<BroadcastConsumer?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<BroadcastConsumer> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxImportContainer : RustObject {
		public static MuxImportContainer New(BroadcastProducer broadcast, MuxCatalogReserved reserved, MuxImportContainerInit init);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxImportContainerFinished Finish();
		public void Abort(Error err);
		public void Cut();
		public void Seek(ulong start);
	}
	public sealed unsafe partial class MuxImportContainerExtra : RustObject {
		public static MuxImportContainerExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved, MuxImportContainerInit init);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxImportContainerFinishedExtra Finish();
		public void Abort(Error err);
		public void Cut();
		public void Seek(ulong start);
	}
	public sealed unsafe partial class MuxImportContainerExt : RustObject {
		public static MuxImportContainerExt New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved, MuxImportContainerInit init);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxImportContainerFinishedExt Finish();
		public void Abort(Error err);
		public void Cut();
		public void Seek(ulong start);
	}
	public sealed unsafe partial class MuxImportAudioInit : RustObject, IEquatable<MuxImportAudioInit> {
		public MuxImportAudioInit Clone();
		public bool Equals(MuxImportAudioInit? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxImportAudioInit? a, MuxImportAudioInit? b);
		public static bool operator !=(MuxImportAudioInit? a, MuxImportAudioInit? b);
		public string Debug();
		public MuxImportAudioFormat Format { get; set; }
		public Bytes Data { get; set; }
		public Str? Label { get; set; }
		public HangCatalogContainer Container { get; set; }
		public ulong? Bitrate { get; set; }
		public static MuxImportAudioInit New(MuxImportAudioFormat format, ReadOnlySpan<byte> data);
	}
	public sealed unsafe partial class MuxImportContainerInit : RustObject, IEquatable<MuxImportContainerInit> {
		public MuxImportContainerInit Clone();
		public bool Equals(MuxImportContainerInit? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxImportContainerInit? a, MuxImportContainerInit? b);
		public static bool operator !=(MuxImportContainerInit? a, MuxImportContainerInit? b);
		public string Debug();
		public MuxImportContainerFormat Format { get; set; }
		public Bytes Data { get; set; }
		public static MuxImportContainerInit New(MuxImportContainerFormat format, ReadOnlySpan<byte> data);
	}
	public sealed unsafe partial class MuxImportVideoInit : RustObject, IEquatable<MuxImportVideoInit> {
		public MuxImportVideoInit Clone();
		public bool Equals(MuxImportVideoInit? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxImportVideoInit? a, MuxImportVideoInit? b);
		public static bool operator !=(MuxImportVideoInit? a, MuxImportVideoInit? b);
		public string Debug();
		public MuxImportVideoFormat Format { get; set; }
		public Bytes Data { get; set; }
		public Str? Label { get; set; }
		public MuxCatalogVideoHint Hint { get; set; }
		public sbyte Priority { get; set; }
		public static MuxImportVideoInit New(MuxImportVideoFormat format, ReadOnlySpan<byte> data);
	}
	public sealed unsafe partial class MuxImportTrack : RustObject {
		public static MuxImportTrack AudioExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportAudioInit init);
		public static MuxImportTrack AudioExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportAudioInit init);
		public static MuxImportTrack AudioUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportAudioInit init);
		public static MuxImportTrack VideoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
		public static MuxImportTrack VideoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
		public static MuxImportTrack VideoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
		public MuxImportTrackFinished Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void Seek(ulong start);
		public TrackDemand Demand();
		public Str Name();
	}
	public sealed unsafe partial class MuxImportTrackStream : RustObject {
		public static MuxImportTrackStream VideoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
		public static MuxImportTrackStream VideoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
		public static MuxImportTrackStream VideoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
		public void Initialize(ReadOnlySpan<byte> data);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxImportTrackFinished Finish();
		public void SetEnabled(bool enabled);
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Seek(ulong start);
		public TrackDemand Demand();
		public Str Name();
	}
	public sealed unsafe partial class MuxJsonSnapshotJson : RustObject {
		public Str Name();
		public TrackSubscriber Consume();
		public TrackDemand Demand();
		public ValueTask Refresh(CancellationToken cancellationToken = default);
		public void Update(Json value, Timestamp timestamp);
		public void Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
	}
	public sealed unsafe partial class MuxJsonStreamJson : RustObject {
		public Str Name();
		public TrackSubscriber Consume();
		public TrackDemand Demand();
		public void Append(Json value, Timestamp timestamp);
		public void Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
	}
	public sealed unsafe partial class MuxJsonConsumerJson : RustObject, IAsyncEnumerable<MoqJsonTimedJson> {
		public static ValueTask<MuxJsonConsumerJson> FromTrack(TrackConsumer track, HangCatalogJsonConfig config, CancellationToken cancellationToken = default);
		public HangCatalogMode Mode();
		public ValueTask<MoqJsonTimedJson?> Next(CancellationToken cancellationToken = default);
		public ValueTask<MoqJsonTimedJson?> Latest(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<MoqJsonTimedJson> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxJsonConfig : RustObject {
		public MuxJsonConfig Clone();
		public string Debug();
		public static MuxJsonConfig Default();
		public Str? Schema { get; set; }
		public MuxJsonConfig WithSchema(Str schema);
	}
	public sealed unsafe partial class MuxJsonOptions : RustObject {
		public MuxJsonOptions Clone();
		public string Debug();
		public static MuxJsonOptions Default();
		public bool Deltas { get; set; }
		public MuxJsonOptions WithDeltas(bool deltas);
	}
	public sealed unsafe partial class MuxRateGrant : RustObject {
		public static MuxRateGrant New(BandwidthConsumer grant, MuxRatePolicy policy);
		public void Assume(MuxRateTarget target);
		public MuxRatePolicy Policy();
		public void SetPolicy(MuxRatePolicy policy);
		public ValueTask<MuxRateTarget> Next(CancellationToken cancellationToken = default);
		public void Failed();
	}
	public sealed unsafe partial class MuxRatePolicy : RustObject, IEquatable<MuxRatePolicy> {
		public MuxRatePolicy Clone();
		public bool Equals(MuxRatePolicy? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxRatePolicy? a, MuxRatePolicy? b);
		public static bool operator !=(MuxRatePolicy? a, MuxRatePolicy? b);
		public string Debug();
		public BandwidthRate Max { get; set; }
		public BandwidthRate Min { get; set; }
		public BandwidthRate? Lower { get; set; }
		public static MuxRatePolicy New(BandwidthRate max);
	}
	public sealed unsafe partial class MuxRateControl : RustObject {
		public MuxRateControl Clone();
		public string Debug();
		public static MuxRateControl New(MuxRatePolicy policy);
		public MuxRateTarget Target();
		public void SetProbed(bool probed);
		public MuxRateTarget? Update(BandwidthRate? estimate, ClockInstant now);
	}
	public sealed unsafe partial class MuxSelectBroadcast : RustObject {
		public MuxSelectBroadcast Clone();
		public string Debug();
		public static MuxSelectBroadcast Default();
		public MuxSelectBroadcast Video(MuxSelectVideo video);
		public MuxSelectBroadcast Audio(MuxSelectAudio audio);
		public bool HasVideo();
		public bool HasAudio();
		public void RetainExtra(HangCatalogExtra catalog);
		public void RetainExt(HangCatalogExt catalog);
		public void RetainUnit(HangCatalog catalog);
	}
	public sealed unsafe partial class MuxSelectVideo : RustObject {
		public MuxSelectVideo Clone();
		public string Debug();
		public static MuxSelectVideo Default();
		public MuxSelectVideo Name(Str name);
		public MuxSelectVideo Codec(HangCatalogVideoCodecKind codec);
	}
	public sealed unsafe partial class MuxSelectAudio : RustObject {
		public MuxSelectAudio Clone();
		public string Debug();
		public static MuxSelectAudio Default();
		public MuxSelectAudio Name(Str name);
		public MuxSelectAudio Codec(HangCatalogAudioCodecKind codec);
	}
	public sealed unsafe partial class MuxSyncTrack : RustObject {
		public bool Received(Timestamp timestamp, ClockInstant now);
		public void Reset();
		public bool SetTarget(Duration target);
		public ValueTask<Duration> DelayChanged(Duration seen, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxSyncSchedule : RustObject, IEquatable<MuxSyncSchedule> {
		public MuxSyncSchedule Clone();
		public bool Equals(MuxSyncSchedule? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxSyncSchedule? a, MuxSyncSchedule? b);
		public static bool operator !=(MuxSyncSchedule? a, MuxSyncSchedule? b);
		public string Debug();
		public Duration Delay();
		public ClockInstant? Due(Timestamp timestamp);
	}
	public sealed unsafe partial class MuxSyncProducer : RustObject {
		public MuxSyncProducer Clone();
		public static MuxSyncProducer New(Duration delay);
		public MuxSyncProducer Reset(Duration delay);
		public MuxSyncConsumer Consume();
		public MuxSyncTrack Track(MuxContainerKind kind);
		public ClockInstant? Due(Timestamp timestamp);
	}
	public sealed unsafe partial class MuxSyncConsumer : RustObject {
		public MuxSyncConsumer Clone();
		public Duration Delay();
		public ClockInstant? Due(Timestamp timestamp);
		public Duration? Position(ClockInstant at);
		public MuxSyncSchedule Schedule();
		public ValueTask<MuxSyncSchedule> Changed(MuxSyncSchedule seen, CancellationToken cancellationToken = default);
		public nuint? Present(Timestamp[] timestamps, ClockInstant now);
	}
	public sealed unsafe partial class TokioTlsError : RustObject {
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class TokioTlsIdentity : RustObject {
		public TokioTlsIdentity Clone();
		public string Debug();
		public static TokioTlsIdentity Generate(Str[] hostnames);
		public Str Fingerprint();
	}
	public sealed unsafe partial class TokioTlsConnect : RustObject {
		public TokioTlsConnect Clone();
		public string Debug();
		public static TokioTlsConnect Default();
		public PathBuf[] Root { get; set; }
		public bool? SystemRoots { get; set; }
		public Str[] Fingerprint { get; set; }
		public TokioTlsIdentity? Identity { get; set; }
		public PathBuf? Cert { get; set; }
		public PathBuf? Key { get; set; }
		public bool? Insecure { get; set; }
		public Str? HostName { get; set; }
		public MoqTokioCliDeprecated Deprecated();
		public RustlsClientConfig Build();
	}
	public sealed unsafe partial class TokioTlsListen : RustObject {
		public TokioTlsListen Clone();
		public string Debug();
		public static TokioTlsListen Default();
		public PathBuf[] Cert { get; set; }
		public PathBuf[] Key { get; set; }
		public Str[] Generate { get; set; }
		public TokioTlsIdentity? Identity { get; set; }
		public PathBuf[] Root { get; set; }
		public TokioTlsServe Load();
		public MoqTokioCliDeprecated Deprecated();
		public RustlsRootCertStore LoadRoots();
		public RustlsServerConfig ServerConfig(Bytes[] alpn);
	}
	public sealed unsafe partial class TokioTlsServe : RustObject {
		public TokioTlsServe Clone();
		public string Debug();
		public TokioTlsCertificates Certificates();
		public RustlsServerConfig Config(Bytes[] alpn);
		public TokioTlsPeerIdentity Peer(RustlsPkiTypesCertificateDer[] chain);
	}
	public sealed unsafe partial class TokioTlsPeerIdentity : RustObject {
		public TokioTlsPeerIdentity Clone();
		public string Debug();
		public RustlsPkiTypesCertificateDer[] Chain();
		public Str? Fingerprint();
		public Str? Name();
		public Str[] Lineage();
		public Str? Issuer();
		public ClockWall? Expiry();
	}
	public sealed unsafe partial class TokioTlsCertificates : RustObject {
		public TokioTlsCertificates Clone();
		public string Debug();
		public static TokioTlsCertificates FromPem(ReadOnlySpan<byte> chain);
		public Str[] Fingerprints();
	}
	public sealed unsafe partial class TokioTlsKeyLog : RustObject {
		public string Debug();
		public static TokioTlsKeyLog Open(PathBuf path);
	}
	public sealed unsafe partial class TokioClientConfig : RustObject {
		public TokioClientConfig Clone();
		public string Debug();
		public static TokioClientConfig Default();
		public TokioConnectConfig Connect { get; set; }
		public TokioQuicConfig Quic { get; set; }
		public TokioClient Init();
	}
	public sealed unsafe partial class TokioConnectConfig : RustObject {
		public TokioConnectConfig Clone();
		public string Debug();
		public static TokioConnectConfig Default();
		public Url? Url { get; set; }
		public AuthToken[] Tokens { get; set; }
		public SocketAddr? Bind { get; set; }
		public TokioDuration? Race { get; set; }
		public TokioDuration? ResolutionDelay { get; set; }
		public TokioDuration? Timeout { get; set; }
		public Version[] Version { get; set; }
		public Str? Implementation { get; set; }
		public TokioTlsConnect Tls { get; set; }
		public TokioBackoff Backoff { get; set; }
		public MoqTokioConnectionHandover Goaway { get; set; }
		public MoqTokioWebsocketConfig Websocket { get; set; }
		public MoqTokioCliDeprecated Deprecated();
		public TokioClient Init(TokioQuicConfig quic);
		public Versions Versions();
		public MoqTokioConnectResolved Resolve();
	}
	public sealed unsafe partial class TokioConnectionMonitor : RustObject {
		public TokioConnectionMonitor Clone();
		public MoqTokioConnectionObservation State();
		public ValueTask<MoqTokioConnectionObservation> StateChanged(ulong after, CancellationToken cancellationToken = default);
		public StatsPresence Presence();
		public ValueTask<StatsPresence> PollPresence(CancellationToken cancellationToken = default);
		public ValueTask<StatsPresence> PresenceChanged(CancellationToken cancellationToken = default);
		public SessionStats? Stats();
		public TokioError? Failure();
		public MoqTokioConnectionSnapshot? Snapshot();
	}
	public sealed unsafe partial class TokioListenConfig : RustObject {
		public TokioListenConfig Clone();
		public string Debug();
		public static TokioListenConfig Default();
		public TokioListenBind? Bind { get; set; }
		public MoqTokioTcpConfig Tcp { get; set; }
		public MoqTokioUnixConfig Unix { get; set; }
		public Version[] Version { get; set; }
		public Str? Implementation { get; set; }
		public TokioDuration? Timeout { get; set; }
		public TokioTlsListen Tls { get; set; }
		public SocketAddrV4? PreferredV4 { get; set; }
		public SocketAddrV6? PreferredV6 { get; set; }
		public MoqTokioListenLoadBalancer Lb { get; set; }
		public MoqTokioListenReset Reset { get; set; }
		public MoqTokioCliDeprecated Deprecated();
		public Duration? ResolvedTimeout();
		public MoqTokioListenResolved Resolve();
		public TokioServer Init(TokioQuicConfig quic);
		public TokioServer InitStreams();
		public Versions Versions();
		public Server Server();
		public bool HasExplicitBind();
		public bool ServesQuic();
	}
	public sealed unsafe partial class TokioQuicConfig : RustObject {
		public TokioQuicConfig Clone();
		public string Debug();
		public static TokioQuicConfig Default();
		public ulong? MaxStreams { get; set; }
		public bool? Gso { get; set; }
		public TokioDuration? IdleTimeout { get; set; }
		public TokioDuration? KeepAlive { get; set; }
		public bool? MtuDiscovery { get; set; }
		public MoqTokioQuicCongestionControl? CongestionControl { get; set; }
		public ulong? ReceiveWindow { get; set; }
		public ulong? StreamReceiveWindow { get; set; }
		public ulong? SendWindow { get; set; }
		public PathBuf? Qlog { get; set; }
		public PathBuf? Keylog { get; set; }
		public MoqTokioCliDeprecated Deprecated();
		public void Validate();
		public MoqTokioQuicResolved Resolve();
	}
	public sealed unsafe partial class TokioServerConfig : RustObject {
		public static TokioServerConfig Default();
		public TokioTlsServe? Tls { get; set; }
		public ServerAdmission Admission { get; set; }
		public TokioListenConfig Listen { get; set; }
		public TokioQuicConfig Quic { get; set; }
		public MoqTokioWebsocketListener? Websocket { get; set; }
		public OriginConsumer? Publisher { get; set; }
		public OriginProducer? Subscriber { get; set; }
		public StatsSession Stats { get; set; }
		public MoqTokioQuicDemuxDemux Demux { get; set; }
		public MoqTokioServerStreams Streams_ { get; set; }
		public TokioServer Init();
		public TokioServer InitStreams();
	}
	public sealed unsafe partial class TokioServerListener : RustObject {
		public ValueTask<TokioServerRequest> Accept(CancellationToken cancellationToken = default);
		public ValueTask Close(CancellationToken cancellationToken = default);
		public ValueTask CloseStreams(CancellationToken cancellationToken = default);
		public SocketAddr LocalAddr();
		public SocketAddr? WebsocketLocalAddr();
		public SocketAddr? TcpLocalAddr();
		public TokioTlsCertificates Certificates();
		public MoqTokioAcceptHealth[] AcceptHealth();
	}
	public sealed unsafe partial class TokioServerRequest : RustObject {
		public ValueTask Reject(SessionError code, CancellationToken cancellationToken = default);
		public TokioServerRequest WithPublisherProducer(OriginProducer publish);
		public TokioServerRequest WithPublisherConsumer(OriginConsumer publish);
		public TokioServerRequest WithSubscriber(OriginProducer subscribe);
		public TokioServerRequest WithPeerNode(NonZeroU64 node);
		public TokioServerRequest WithStats(StatsSession stats);
		public TokioServerRequest WithLimits(SessionLimits limits);
		public MoqTokioServerQuicLimits? QuicLimits();
		public TokioServerRequest WithQuicLimits(MoqQuicLimits limits);
		public ValueTask Expired(CancellationToken cancellationToken = default);
		public ValueTask<Session> Ok(CancellationToken cancellationToken = default);
		public TokioTransport Transport();
		public Url? Url();
		public Str? Authority();
		public Str Path();
		public Str? Query();
		public SocketAddr? RemoteAddr();
		public SocketAddr? LocalAddr();
		public Str? ServerName();
		public Str? Alpn();
		public SetupRole? Role();
		public AuthHandle Auth();
		public AuthRequests Requests();
		public NonZeroU64? PeerNode();
		public Str? PeerImplementation();
		public SetupToken? Token();
		public TokioTlsPeerIdentity? PeerIdentity();
	}
	public sealed unsafe partial class TransportMemoryConfig : RustObject {
		public TransportMemoryConfig Clone();
		public string Debug();
		public static TransportMemoryConfig Default();
		public nuint StreamBuffer { get; set; }
		public Str? Protocol { get; set; }
		public nuint? MaxConcurrentUniStreams { get; set; }
		public nuint? MaxConcurrentBiStreams { get; set; }
		public Duration Latency { get; set; }
	}
	public sealed unsafe partial class TransportMemorySession : RustObject {
		public TransportMemorySession Clone();
		public TransportMemoryCloser Closer();
	}
	public sealed unsafe partial class TransportMemoryCloser : RustObject {
		public TransportMemoryCloser Clone();
		public void Close(uint code, Str reason);
		public bool IsClosed_();
	}
	public sealed unsafe partial class MuxCatalogHangConsumer : RustObject, IAsyncEnumerable<HangCatalog> {
		public static MuxCatalogHangConsumer New(TrackConsumer track);
		public ValueTask<HangCatalog?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalog> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogHangConsumerExtra : RustObject, IAsyncEnumerable<HangCatalogExtra> {
		public static MuxCatalogHangConsumerExtra New(TrackConsumer track);
		public ValueTask<HangCatalogExtra?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalogExtra> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogHangConsumerExt : RustObject, IAsyncEnumerable<HangCatalogExt> {
		public static MuxCatalogHangConsumerExt New(TrackConsumer track);
		public ValueTask<HangCatalogExt?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalogExt> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogHangContainer : RustObject {
		public static MuxCatalogHangContainer New(HangCatalogContainer container, MuxContainerKind kind);
	}
	public sealed unsafe partial class MuxCatalogHangExtra : RustObject, IEquatable<MuxCatalogHangExtra> {
		public MuxCatalogHangExtra Clone();
		public bool Equals(MuxCatalogHangExtra? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCatalogHangExtra? a, MuxCatalogHangExtra? b);
		public static bool operator !=(MuxCatalogHangExtra? a, MuxCatalogHangExtra? b);
		public string Debug();
		public static MuxCatalogHangExtra Default();
		public Json? Get(Str name);
		public (Str, Json)[] Iter();
		public nuint Len();
		public bool IsEmpty();
		public void Set(Str name, Json value);
		public Json? Remove(Str name);
	}
	public sealed unsafe partial class MuxCatalogMsfConsumer : RustObject, IAsyncEnumerable<HangCatalog> {
		public static MuxCatalogMsfConsumer New(TrackSubscriber track);
		public ValueTask<HangCatalog?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalog> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogMsfConsumerExtra : RustObject, IAsyncEnumerable<HangCatalogExtra> {
		public static MuxCatalogMsfConsumerExtra New(TrackSubscriber track);
		public ValueTask<HangCatalogExtra?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalogExtra> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCatalogMsfConsumerExt : RustObject, IAsyncEnumerable<HangCatalogExt> {
		public static MuxCatalogMsfConsumerExt New(TrackSubscriber track);
		public ValueTask<HangCatalogExt?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<HangCatalogExt> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxCodecAacConfig : RustObject {
		public byte Profile { get; set; }
		public uint SampleRate { get; set; }
		public uint ChannelCount { get; set; }
		public static MuxCodecAacConfig New(byte profile, uint sampleRate, uint channelCount);
		public static MuxCodecAacConfig Parse(ref Bytes buf);
		public Bytes Encode();
	}
	public sealed unsafe partial class MuxCodecAacError : RustObject {
		public MuxCodecAacError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxCodecAacImport : RustObject {
		public static MuxCodecAacImport NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
		public static MuxCodecAacImport NewExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
		public static MuxCodecAacImport NewUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
		public Str Name();
		public TrackDemand Demand();
		public MuxContainerFinishedContainerAudioConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void Seek(ulong start);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
	}
	public sealed unsafe partial class MuxCodecAnnexbError : RustObject {
		public MuxCodecAnnexbError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxCodecAv1Import : RustObject {
		public static MuxCodecAv1Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
		public static MuxCodecAv1Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
		public static MuxCodecAv1Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
		public void Initialize(ReadOnlySpan<byte> buf);
		public TrackDemand Demand();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void Seek(ulong start);
		public void Decode(MuxContainerFrame[] frames);
	}
	public sealed unsafe partial class MuxCodecAv1Split : RustObject {
		public static MuxCodecAv1Split Default();
		public static MuxCodecAv1Split New();
		public MuxContainerFrame[] Decode(ReadOnlySpan<byte> data, Timestamp? pts);
		public MuxContainerFrame[] Flush(Timestamp? pts);
		public void Reset();
	}
	public sealed unsafe partial class MuxCodecFlacImport : RustObject {
		public static MuxCodecFlacImport NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
		public static MuxCodecFlacImport NewExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
		public static MuxCodecFlacImport NewUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
		public TrackDemand Demand();
		public MuxContainerFinishedContainerAudioConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void Seek(ulong start);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
	}
	public sealed unsafe partial class MuxCodecH264Avc1 : RustObject {
		public static MuxCodecH264Avc1 Default();
		public static MuxCodecH264Avc1 New();
		public Bytes? Avcc();
		public Bytes? Transform(ReadOnlySpan<byte> payload);
	}
	public sealed unsafe partial class MuxCodecH264Error : RustObject {
		public MuxCodecH264Error Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxCodecH264Avcc : RustObject {
		public MuxCodecH264Avcc Clone();
		public string Debug();
		public byte Profile { get; set; }
		public byte Constraints { get; set; }
		public byte Level { get; set; }
		public nuint LengthSize { get; set; }
		public Bytes[] Sps { get; set; }
		public Bytes[] Pps { get; set; }
		public uint? CodedWidth { get; set; }
		public uint? CodedHeight { get; set; }
		public static MuxCodecH264Avcc Parse(ReadOnlySpan<byte> avcc);
	}
	public sealed unsafe partial class MuxCodecH264Import : RustObject {
		public static MuxCodecH264Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
		public static MuxCodecH264Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
		public static MuxCodecH264Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
		public void Initialize(ReadOnlySpan<byte> buf);
		public Str Name();
		public TrackDemand Demand();
		public TrackProducer Track();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public void Seek(ulong start);
		public void ObserveReorder(Timestamp reorder);
		public void Decode(MuxContainerFrame[] frames);
	}
	public sealed unsafe partial class MuxCodecH264Split : RustObject {
		public static MuxCodecH264Split Default();
		public static MuxCodecH264Split New();
		public MuxContainerFrame[] Decode(ReadOnlySpan<byte> data, Timestamp? pts);
		public MuxContainerFrame[] Flush(Timestamp? pts);
		public void Reset();
	}
	public sealed unsafe partial class MuxCodecH265Hvc1 : RustObject {
		public static MuxCodecH265Hvc1 Default();
		public static MuxCodecH265Hvc1 New();
		public Bytes? Hvcc();
		public Bytes? Transform(ReadOnlySpan<byte> payload);
	}
	public sealed unsafe partial class MuxCodecH265Error : RustObject {
		public MuxCodecH265Error Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxCodecH265Hvcc : RustObject {
		public MuxCodecH265Hvcc Clone();
		public string Debug();
		public nuint LengthSize { get; set; }
		public Bytes[] Vps { get; set; }
		public Bytes[] Sps { get; set; }
		public Bytes[] Pps { get; set; }
		public static MuxCodecH265Hvcc Parse(ReadOnlySpan<byte> hvcc);
	}
	public sealed unsafe partial class MuxCodecH265Sps : RustObject {
		public MuxCodecH265Sps Clone();
		public string Debug();
		public static MuxCodecH265Sps Parse(ReadOnlySpan<byte> nal);
		public (uint, uint) Size();
		public MuxCodecH265VideoSignal? VideoSignal();
	}
	public sealed unsafe partial class MuxCodecH265Import : RustObject {
		public static MuxCodecH265Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
		public static MuxCodecH265Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
		public static MuxCodecH265Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
		public void Initialize(ReadOnlySpan<byte> buf);
		public Str Name();
		public TrackDemand Demand();
		public TrackProducer Track();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void ReportSkipped(ulong frames);
		public void ReportBitrate(BandwidthRate bitrate);
		public void Seek(ulong start);
		public void ObserveReorder(Timestamp reorder);
		public void Decode(MuxContainerFrame[] frames);
	}
	public sealed unsafe partial class MuxCodecH265Split : RustObject {
		public static MuxCodecH265Split Default();
		public static MuxCodecH265Split New();
		public MuxContainerFrame[] Decode(ReadOnlySpan<byte> data, Timestamp? pts);
		public MuxContainerFrame[] Flush(Timestamp? pts);
		public void Reset();
	}
	public sealed unsafe partial class MuxCodecMp3Import : RustObject {
		public static MuxCodecMp3Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
		public static MuxCodecMp3Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
		public static MuxCodecMp3Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
		public TrackDemand Demand();
		public MuxContainerFinishedContainerAudioConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void Seek(ulong start);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
	}
	public sealed unsafe partial class MuxCodecOpusConfig : RustObject, IEquatable<MuxCodecOpusConfig> {
		public MuxCodecOpusConfig Clone();
		public bool Equals(MuxCodecOpusConfig? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxCodecOpusConfig? a, MuxCodecOpusConfig? b);
		public static bool operator !=(MuxCodecOpusConfig? a, MuxCodecOpusConfig? b);
		public string Debug();
		public uint SampleRate { get; set; }
		public uint ChannelCount { get; set; }
		public ushort PreSkip { get; set; }
		public short OutputGain { get; set; }
		public MuxCodecOpusMapping? Mapping { get; set; }
		public static MuxCodecOpusConfig New(uint sampleRate, uint channelCount);
		public MuxCodecOpusConfig WithPreSkip(ushort preSkip);
		public static MuxCodecOpusConfig Parse(ref Bytes buf);
		public Bytes Encode();
	}
	public sealed unsafe partial class MuxCodecOpusImport : RustObject {
		public static MuxCodecOpusImport NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
		public static MuxCodecOpusImport NewExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
		public static MuxCodecOpusImport NewUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
		public Str Name();
		public TrackDemand Demand();
		public MuxContainerFinishedContainerAudioConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void Seek(ulong start);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
	}
	public sealed unsafe partial class MuxCodecVp8Import : RustObject {
		public static MuxCodecVp8Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
		public static MuxCodecVp8Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
		public static MuxCodecVp8Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
		public void Initialize(ReadOnlySpan<byte> buf);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
		public TrackDemand Demand();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void Seek(ulong start);
	}
	public sealed unsafe partial class MuxCodecVp9Import : RustObject {
		public static MuxCodecVp9Import NewExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
		public static MuxCodecVp9Import NewExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
		public static MuxCodecVp9Import NewUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
		public void Initialize(ReadOnlySpan<byte> buf);
		public void Decode(ReadOnlySpan<byte> frame, Timestamp? pts);
		public TrackDemand Demand();
		public MuxContainerFinishedContainerVideoConfig Finish();
		public void Abort(Error err);
		public void AbortUnwanted(Error err);
		public void Cut(Timestamp? end);
		public void Discontinuity();
		public void SetEnabled(bool enabled);
		public void Seek(ulong start);
	}
	public sealed unsafe partial class MuxContainerFlvImport : RustObject {
		public static MuxContainerFlvImport New(BroadcastProducer broadcast, MuxCatalogReserved reserved);
		public MuxContainerFlvImport WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerFlvFinished Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFlvImportExtra : RustObject {
		public static MuxContainerFlvImportExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
		public MuxContainerFlvImportExtra WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerFlvFinishedExtra Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFlvImportExt : RustObject {
		public static MuxContainerFlvImportExt New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
		public MuxContainerFlvImportExt WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerFlvFinishedExt Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFlvFinished : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFlvFinishedExtra : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFlvFinishedExt : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFmp4Wire : RustObject {
		public static MuxContainerFmp4Wire FromInit(ReadOnlySpan<byte> initData);
	}
	public sealed unsafe partial class MuxContainerFmp4Error : RustObject {
		public MuxContainerFmp4Error Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxContainerFmp4Import : RustObject {
		public static MuxContainerFmp4Import New(BroadcastProducer broadcast, MuxCatalogReserved reserved);
		public void Cut();
		public MuxContainerFmp4Import WithSelect(MuxSelectBroadcast select);
		public MuxContainerFmp4Import WithCmaf(bool cmaf);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerFmp4Finished Finish();
		public void Abort(Error err);
		public void Seek(ulong start);
		public void Rejoin();
		public void Discontinuity();
	}
	public sealed unsafe partial class MuxContainerFmp4ImportExtra : RustObject {
		public static MuxContainerFmp4ImportExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
		public void Cut();
		public MuxContainerFmp4ImportExtra WithSelect(MuxSelectBroadcast select);
		public MuxContainerFmp4ImportExtra WithCmaf(bool cmaf);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerFmp4FinishedExtra Finish();
		public void Abort(Error err);
		public void Seek(ulong start);
		public void Rejoin();
		public void Discontinuity();
	}
	public sealed unsafe partial class MuxContainerFmp4ImportExt : RustObject {
		public static MuxContainerFmp4ImportExt New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
		public void Cut();
		public MuxContainerFmp4ImportExt WithSelect(MuxSelectBroadcast select);
		public MuxContainerFmp4ImportExt WithCmaf(bool cmaf);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerFmp4FinishedExt Finish();
		public void Abort(Error err);
		public void Seek(ulong start);
		public void Rejoin();
		public void Discontinuity();
	}
	public sealed unsafe partial class MuxContainerFmp4Finished : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFmp4FinishedExtra : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFmp4FinishedExt : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerFmp4Muxer : RustObject {
		public static MuxContainerFmp4Muxer Video(HangCatalogVideoConfig config);
		public static MuxContainerFmp4Muxer Audio(HangCatalogAudioConfig config);
		public Timescale Timescale();
		public MuxContainerFmp4Muxer WithTimescale(Timescale timescale);
		public ValueTask<MuxContainerFrame[]> Read(GroupConsumer group, CancellationToken cancellationToken = default);
		public (uint, uint)? PictureSize(MuxContainerFrame keyframe);
		public Bytes? Init();
		public Bytes Fragment(uint sequence, MuxContainerFrame[] frames, Timestamp? end);
	}
	public sealed unsafe partial class MuxContainerLegacyAudio : RustObject, IEquatable<MuxContainerLegacyAudio> {
		public MuxContainerLegacyAudio Clone();
		public bool Equals(MuxContainerLegacyAudio? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerLegacyAudio? a, MuxContainerLegacyAudio? b);
		public static bool operator !=(MuxContainerLegacyAudio? a, MuxContainerLegacyAudio? b);
		public string Debug();
		public static MuxContainerLegacyAudio? New(HangCatalogAudioConfig config);
		public Timestamp? Duration(ReadOnlySpan<byte> payload);
	}
	public sealed unsafe partial class MuxContainerMkvImport : RustObject {
		public static MuxContainerMkvImport New(BroadcastProducer broadcast, MuxCatalogReserved reserved);
		public MuxContainerMkvImport WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerMkvFinished Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMkvImportExtra : RustObject {
		public static MuxContainerMkvImportExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
		public MuxContainerMkvImportExtra WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerMkvFinishedExtra Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMkvImportExt : RustObject {
		public static MuxContainerMkvImportExt New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
		public MuxContainerMkvImportExt WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong start);
		public MuxContainerMkvFinishedExt Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMkvFinished : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMkvFinishedExtra : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMkvFinishedExt : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerMp4Finish : RustObject {
		public MuxContainerMp4Finish Clone();
		public string Debug();
	}
	public sealed unsafe partial class MuxContainerTsExt : RustObject, IEquatable<MuxContainerTsExt> {
		public MuxContainerTsExt Clone();
		public bool Equals(MuxContainerTsExt? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsExt? a, MuxContainerTsExt? b);
		public static bool operator !=(MuxContainerTsExt? a, MuxContainerTsExt? b);
		public string Debug();
		public static MuxContainerTsExt Default();
		public MuxContainerTsMpegts Mpegts { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsMpegts : RustObject, IEquatable<MuxContainerTsMpegts> {
		public MuxContainerTsMpegts Clone();
		public bool Equals(MuxContainerTsMpegts? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsMpegts? a, MuxContainerTsMpegts? b);
		public static bool operator !=(MuxContainerTsMpegts? a, MuxContainerTsMpegts? b);
		public string Debug();
		public static MuxContainerTsMpegts Default();
		public (Str, MuxContainerTsTrack)[] Tracks { get; set; }
		public MuxContainerTsDescriptor[] ProgramDescriptors { get; set; }
		public MuxContainerTsProgram? Program { get; set; }
		public (ushort, (byte, MuxContainerTsSiEntry)[])[] Si { get; set; }
		public ulong? MuxRate { get; set; }
		public MuxContainerTsPassthroughTrack? Passthrough { get; set; }
		public static MuxContainerTsMpegts Deserialize(Json deserializer);
		public bool IsEmpty();
	}
	public sealed unsafe partial class MuxContainerTsPassthroughTrack : RustObject, IEquatable<MuxContainerTsPassthroughTrack> {
		public MuxContainerTsPassthroughTrack Clone();
		public bool Equals(MuxContainerTsPassthroughTrack? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsPassthroughTrack? a, MuxContainerTsPassthroughTrack? b);
		public static bool operator !=(MuxContainerTsPassthroughTrack? a, MuxContainerTsPassthroughTrack? b);
		public string Debug();
		public Str Track { get; set; }
		public bool RandomAccess { get; set; }
		public static MuxContainerTsPassthroughTrack New(Str track);
	}
	public sealed unsafe partial class MuxContainerTsPesPacket : RustObject, IEquatable<MuxContainerTsPesPacket> {
		public MuxContainerTsPesPacket Clone();
		public bool Equals(MuxContainerTsPesPacket? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsPesPacket? a, MuxContainerTsPesPacket? b);
		public static bool operator !=(MuxContainerTsPesPacket? a, MuxContainerTsPesPacket? b);
		public string Debug();
		public static MuxContainerTsPesPacket Default();
		public byte StreamType { get; set; }
		public static MuxContainerTsPesPacket New(byte streamType);
	}
	public sealed unsafe partial class MuxContainerTsProgram : RustObject, IEquatable<MuxContainerTsProgram> {
		public MuxContainerTsProgram Clone();
		public bool Equals(MuxContainerTsProgram? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsProgram? a, MuxContainerTsProgram? b);
		public static bool operator !=(MuxContainerTsProgram? a, MuxContainerTsProgram? b);
		public string Debug();
		public static MuxContainerTsProgram Default();
		public ushort TransportStreamId { get; set; }
		public ushort ProgramNumber { get; set; }
		public ushort PmtPid { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsSiEntry : RustObject, IEquatable<MuxContainerTsSiEntry> {
		public MuxContainerTsSiEntry Clone();
		public bool Equals(MuxContainerTsSiEntry? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsSiEntry? a, MuxContainerTsSiEntry? b);
		public static bool operator !=(MuxContainerTsSiEntry? a, MuxContainerTsSiEntry? b);
		public string Debug();
		public static MuxContainerTsSiEntry Default();
		public Str Track { get; set; }
		public Duration? Interval { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsTrack : RustObject, IEquatable<MuxContainerTsTrack> {
		public MuxContainerTsTrack Clone();
		public bool Equals(MuxContainerTsTrack? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsTrack? a, MuxContainerTsTrack? b);
		public static bool operator !=(MuxContainerTsTrack? a, MuxContainerTsTrack? b);
		public string Debug();
		public static MuxContainerTsTrack Default();
		public ushort Pid { get; set; }
		public MuxContainerTsDescriptor[] Descriptors { get; set; }
		public MuxContainerTsVerbatim? Verbatim { get; set; }
		public MuxContainerTsPesPacket? PesPacket { get; set; }
		public static MuxContainerTsTrack New(ushort pid);
	}
	public sealed unsafe partial class MuxContainerTsVerbatim : RustObject, IEquatable<MuxContainerTsVerbatim> {
		public MuxContainerTsVerbatim Clone();
		public bool Equals(MuxContainerTsVerbatim? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsVerbatim? a, MuxContainerTsVerbatim? b);
		public static bool operator !=(MuxContainerTsVerbatim? a, MuxContainerTsVerbatim? b);
		public string Debug();
		public static MuxContainerTsVerbatim Default();
		public byte StreamType { get; set; }
		public MuxContainerTsFraming Framing { get; set; }
		public static MuxContainerTsVerbatim New(byte streamType, MuxContainerTsFraming framing);
	}
	public sealed unsafe partial class MuxContainerTsError : RustObject {
		public MuxContainerTsError Clone();
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class MuxContainerTsExportConsumerExtra : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsExportConsumerExtra New(MuxSource source, MuxCatalogConsumerExtra catalog, Duration delay);
		public MuxContainerTsExportConsumerExtra WithMuxRate(ulong muxRate);
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public MuxContainerTsStatsExport Stats();
		public ulong Discontinuity();
		public MuxContainerTsExportConsumerExtra Follow(BroadcastConsumer broadcast, MuxCatalogConsumerExtra catalog);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsExportConsumerExt : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsExportConsumerExt New(MuxSource source, MuxCatalogConsumerExt catalog, Duration delay);
		public MuxContainerTsExportConsumerExt WithMuxRate(ulong muxRate);
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public MuxContainerTsStatsExport Stats();
		public ulong Discontinuity();
		public MuxContainerTsExportConsumerExt Follow(BroadcastConsumer broadcast, MuxCatalogConsumerExt catalog);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsExportConsumer : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsExportConsumer New(MuxSource source, MuxCatalogConsumer catalog, Duration delay);
		public MuxContainerTsExportConsumer WithMuxRate(ulong muxRate);
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public MuxContainerTsStatsExport Stats();
		public ulong Discontinuity();
		public MuxContainerTsExportConsumer Follow(BroadcastConsumer broadcast, MuxCatalogConsumer catalog);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsFollower : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsFollower New(MuxContainerTsExportConsumer export);
		public MuxContainerTsFollower WithFollow(Duration follow);
		public MuxContainerTsFollower WithStitch(bool stitch);
		public MuxContainerTsExportConsumer Export();
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsFollowerExtra : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsFollowerExtra New(MuxContainerTsExportConsumerExtra export);
		public MuxContainerTsFollowerExtra WithFollow(Duration follow);
		public MuxContainerTsFollowerExtra WithStitch(bool stitch);
		public MuxContainerTsExportConsumerExtra Export();
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsFollowerExt : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static MuxContainerTsFollowerExt New(MuxContainerTsExportConsumerExt export);
		public MuxContainerTsFollowerExt WithFollow(Duration follow);
		public MuxContainerTsFollowerExt WithStitch(bool stitch);
		public MuxContainerTsExportConsumerExt Export();
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsPassthrough : RustObject {
		public static MuxContainerTsPassthrough New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
		public MuxContainerTsPassthrough WithPcrPid(ushort pid);
		public Str Track();
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerTsPassthroughFinished Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsPassthroughUnit : RustObject {
		public static MuxContainerTsPassthroughUnit New(BroadcastProducer broadcast, MuxCatalogReserved reserved);
		public MuxContainerTsPassthroughUnit WithPcrPid(ushort pid);
		public Str Track();
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerTsPassthroughFinishedUnit Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsPassthroughExtra : RustObject {
		public static MuxContainerTsPassthroughExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
		public MuxContainerTsPassthroughExtra WithPcrPid(ushort pid);
		public Str Track();
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerTsPassthroughFinishedExtra Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsPrograms : RustObject {
		public static MuxContainerTsPrograms New(OriginProducer origin, Path name, MuxCatalogConfig config);
		public void Decode(ReadOnlySpan<byte> data);
		public MuxContainerTsStatsSnapshot Stats();
		public MuxContainerTsProgramsFinished Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsRelease : RustObject, IAsyncEnumerable<MuxContainerFrame> {
		public static ValueTask<MuxContainerTsRelease> New(MuxSource source, MuxCatalogCatalogFormat catalogFormat, Duration delay, CancellationToken cancellationToken = default);
		public ulong Dropped();
		public double? Drift();
		public ulong OutOfTolerance();
		public MuxContainerTsStatsRelease Stats();
		public ValueTask<MuxContainerFrame?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<MuxContainerFrame> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MuxContainerTsRepublish : RustObject {
		public static MuxContainerTsRepublish New(OriginProducer origin, Path name, Instance instance, MuxCatalogConfig config);
		public MuxContainerTsRepublish WithProgram(ushort program);
		public BroadcastProducer Broadcast();
		public MuxCatalogProducerExt Catalog();
		public Instance Instance();
		public MuxContainerTsRepublished? Decode(ReadOnlySpan<byte> data);
		public MuxContainerTsStatsSnapshot Stats();
		public MuxContainerTsRepublishFinished Finish();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsRepublished : RustObject {
		public MuxContainerTsRepublished Clone();
		public string Debug();
		public MuxContainerTsStep Step { get; set; }
		public MuxContainerTsStatsSnapshot Stats { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsImport : RustObject {
		public static MuxContainerTsImport New(BroadcastProducer broadcast, MuxCatalogReserved reserved);
		public MuxContainerTsImport WithProgram(ushort program);
		public MuxContainerTsImport WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong sequence);
		public MuxContainerTsFinished Finish();
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsImportExtra : RustObject {
		public static MuxContainerTsImportExtra New(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
		public MuxContainerTsImportExtra WithProgram(ushort program);
		public MuxContainerTsImportExtra WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong sequence);
		public MuxContainerTsFinishedExtra Finish();
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsImportExt : RustObject {
		public static MuxContainerTsImportExt New(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
		public MuxContainerTsImportExt WithProgram(ushort program);
		public MuxContainerTsImportExt WithContainer(HangCatalogContainer container);
		public void Decode(ReadOnlySpan<byte> data);
		public void Seek(ulong sequence);
		public MuxContainerTsFinishedExt Finish();
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsFinished : RustObject {
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsFinishedExtra : RustObject {
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsFinishedExt : RustObject {
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxImportContainerFinished : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxImportContainerFinishedExtra : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxImportContainerFinishedExt : RustObject {
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxImportTrackFinished : RustObject {
		public Str Name();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxCodecAacAdtsHeader : RustObject {
		public MuxCodecAacAdtsHeader Clone();
		public string Debug();
		public byte ObjectType { get; set; }
		public uint SampleRate { get; set; }
		public byte ChannelConfig { get; set; }
		public byte Blocks { get; set; }
		public static MuxCodecAacAdtsHeader Parse(ReadOnlySpan<byte> data);
		public Bytes Payload(ReadOnlySpan<byte> packet);
	}
	public sealed unsafe partial class MuxContainerTsPassthroughFinished : RustObject {
		public Str Track();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsPassthroughFinishedUnit : RustObject {
		public Str Track();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsPassthroughFinishedExtra : RustObject {
		public Str Track();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsProgramsFinished : RustObject {
		public BroadcastProducer[] Broadcasts();
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsRepublishFinished : RustObject {
		public BroadcastProducer Broadcast();
		public MuxContainerTsStatsSnapshot Stats();
		public void Abort(Error err);
	}
	public sealed unsafe partial class MuxContainerTsStatsReport : RustObject, IEquatable<MuxContainerTsStatsReport> {
		public MuxContainerTsStatsReport Clone();
		public bool Equals(MuxContainerTsStatsReport? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStatsReport? a, MuxContainerTsStatsReport? b);
		public static bool operator !=(MuxContainerTsStatsReport? a, MuxContainerTsStatsReport? b);
		public string Debug();
		public static MuxContainerTsStatsReport Default();
		public MuxContainerTsStatsSnapshot? Mpegts { get; set; }
		public MuxContainerTsStatsRelease? Release { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsStatsSnapshot : RustObject, IEquatable<MuxContainerTsStatsSnapshot> {
		public MuxContainerTsStatsSnapshot Clone();
		public bool Equals(MuxContainerTsStatsSnapshot? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStatsSnapshot? a, MuxContainerTsStatsSnapshot? b);
		public static bool operator !=(MuxContainerTsStatsSnapshot? a, MuxContainerTsStatsSnapshot? b);
		public string Debug();
		public static MuxContainerTsStatsSnapshot Default();
		public (ushort, MuxContainerTsStatsStream)[] Streams_ { get; set; }
		public ulong CrcError { get; set; }
		public ulong TsSyncLoss { get; set; }
		public ulong SyncByteError { get; set; }
		public ulong PatError { get; set; }
		public ulong ContinuityCountError { get; set; }
		public ulong PmtError { get; set; }
		public ulong TransportError { get; set; }
		public ulong PcrRepetitionError { get; set; }
		public ulong PcrDiscontinuityIndicatorError { get; set; }
		public ulong PtsError { get; set; }
		public bool IsEmpty();
	}
	public sealed unsafe partial class MuxContainerTsStatsStream : RustObject, IEquatable<MuxContainerTsStatsStream> {
		public MuxContainerTsStatsStream Clone();
		public bool Equals(MuxContainerTsStatsStream? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStatsStream? a, MuxContainerTsStatsStream? b);
		public static bool operator !=(MuxContainerTsStatsStream? a, MuxContainerTsStatsStream? b);
		public string Debug();
		public static MuxContainerTsStatsStream Default();
		public Str Track { get; set; }
		public MuxContainerTsStatsClass Class { get; set; }
		public ulong Units { get; set; }
		public Duration? Quiet { get; set; }
		public ulong Resyncs { get; set; }
		public ulong Discarded { get; set; }
		public ulong Unconfirmed { get; set; }
		public ulong Damaged { get; set; }
		public ulong Unplaced { get; set; }
		public ulong Refused { get; set; }
		public ulong ContinuityCountError { get; set; }
		public ulong TransportError { get; set; }
		public ulong PtsError { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsStatsExport : RustObject, IEquatable<MuxContainerTsStatsExport> {
		public MuxContainerTsStatsExport Clone();
		public bool Equals(MuxContainerTsStatsExport? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStatsExport? a, MuxContainerTsStatsExport? b);
		public static bool operator !=(MuxContainerTsStatsExport? a, MuxContainerTsStatsExport? b);
		public string Debug();
		public static MuxContainerTsStatsExport Default();
		public (ushort, MuxContainerTsStatsStream)[] Streams_ { get; set; }
		public MuxContainerTsStatsRelease Release { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsStatsRelease : RustObject, IEquatable<MuxContainerTsStatsRelease> {
		public MuxContainerTsStatsRelease Clone();
		public bool Equals(MuxContainerTsStatsRelease? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MuxContainerTsStatsRelease? a, MuxContainerTsStatsRelease? b);
		public static bool operator !=(MuxContainerTsStatsRelease? a, MuxContainerTsStatsRelease? b);
		public string Debug();
		public static MuxContainerTsStatsRelease Default();
		public ulong Dropped { get; set; }
		public ulong Trimmed { get; set; }
		public ulong Gaps { get; set; }
		public long? Margin { get; set; }
		public long? Offset { get; set; }
		public double? Drift { get; set; }
		public ulong OutOfTolerance { get; set; }
	}
	public sealed unsafe partial class MuxContainerTsStatsLog : RustObject {
		public static MuxContainerTsStatsLog Default();
		public static Duration INTERVAL { get; }
		public void Sample(MuxContainerTsStatsSnapshot latest);
		public void Finish(MuxContainerTsStatsSnapshot latest);
	}
	public sealed unsafe partial class MoqTokioAddrs : RustObject, IEquatable<MoqTokioAddrs> {
		public MoqTokioAddrs Clone();
		public bool Equals(MoqTokioAddrs? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqTokioAddrs? a, MoqTokioAddrs? b);
		public static bool operator !=(MoqTokioAddrs? a, MoqTokioAddrs? b);
		public string Debug();
		public static MoqTokioAddrs New(MoqTokioConnectAddr addr);
		public MoqTokioAddrs Or(MoqTokioConnectAddr addr);
		public static MoqTokioAddrs? Collect(MoqTokioConnectAddr[] urls);
		public MoqTokioConnectAddr[] AsSlice();
	}
	public sealed unsafe partial class MoqTokioCliDeprecated : RustObject {
		public MoqTokioCliDeprecated Clone();
		public override string ToString();
		public string Debug();
		public static MoqTokioCliDeprecated Default();
		public bool IsEmpty();
		public void Extend(MoqTokioCliDeprecated other);
		public void Flag(Str old, Str? env, Str @new);
		public void Changed(Str old, Str? env, Str @new, Str note);
		public void Removed(Str old, Str? env, Str note);
		public void Toml(Str old, Str @new, Str? note);
	}
	public sealed unsafe partial class MoqTokioCliBound : RustObject, IEquatable<MoqTokioCliBound> {
		public MoqTokioCliBound Clone();
		public bool Equals(MoqTokioCliBound? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqTokioCliBound? a, MoqTokioCliBound? b);
		public static bool operator !=(MoqTokioCliBound? a, MoqTokioCliBound? b);
		public override string ToString();
		public string Debug();
		public static MoqTokioCliBound NONE { get; }
	}
	public sealed unsafe partial class MoqTokioTcpTlsAcceptor : RustObject {
		public MoqTokioTcpTlsAcceptor Clone();
		public RustlsServerConfig Config();
		public bool Offered(Bytes[] alpns);
	}
	public sealed unsafe partial class MoqTokioAcceptHealth : RustObject {
		public MoqTokioAcceptHealth Clone();
		public string Debug();
		public static MoqTokioAcceptHealth New(Str listener);
		public Str Listener();
		public void Accepted();
		public ulong Failures(MoqTokioAcceptFailure failure);
		public Duration? Stalled();
	}
	public sealed unsafe partial class MoqJsonTimedCatalog : RustObject, IEquatable<MoqJsonTimedCatalog> {
		public MoqJsonTimedCatalog Clone();
		public bool Equals(MoqJsonTimedCatalog? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqJsonTimedCatalog? a, MoqJsonTimedCatalog? b);
		public static bool operator !=(MoqJsonTimedCatalog? a, MoqJsonTimedCatalog? b);
		public string Debug();
		public HangCatalog Value { get; set; }
		public Timestamp At { get; set; }
		public static MoqJsonTimedCatalog New(HangCatalog value, Timestamp at);
	}
	public sealed unsafe partial class MoqJsonTimedJson : RustObject, IEquatable<MoqJsonTimedJson> {
		public MoqJsonTimedJson Clone();
		public bool Equals(MoqJsonTimedJson? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqJsonTimedJson? a, MoqJsonTimedJson? b);
		public static bool operator !=(MoqJsonTimedJson? a, MoqJsonTimedJson? b);
		public string Debug();
		public Json Value { get; set; }
		public Timestamp At { get; set; }
		public static MoqJsonTimedJson New(Json value, Timestamp at);
	}
	public sealed unsafe partial class RustlsClientConfig : RustObject {
		public RustlsClientConfig Clone();
		public string Debug();
		public Bytes[] AlpnProtocols { get; set; }
		public bool CheckSelectedAlpn { get; set; }
		public RustlsClientResumption Resumption { get; set; }
		public nuint? MaxFragmentSize { get; set; }
		public RustlsResolvesClientCert ClientAuthCertResolver { get; set; }
		public bool EnableSni { get; set; }
		public RustlsKeyLog KeyLog { get; set; }
		public bool EnableSecretExtraction { get; set; }
		public bool EnableEarlyData { get; set; }
		public bool RequireEms { get; set; }
		public RustlsTimeProvider TimeProvider { get; set; }
		public RustlsCertDecompressor[] CertDecompressors { get; set; }
		public RustlsCertCompressor[] CertCompressors { get; set; }
		public RustlsCompressionCache CertCompressionCache { get; set; }
		public RustlsClientTicketRequest? SendTicketRequest { get; set; }
		public bool Fips();
		public RustlsCryptoProvider CryptoProvider();
		public RustlsClientDangerDangerousClientConfig Dangerous();
	}
	public sealed unsafe partial class RustlsRootCertStore : RustObject {
		public RustlsRootCertStore Clone();
		public string Debug();
		public RustlsPkiTypesTrustAnchor[] Roots { get; set; }
		public static RustlsRootCertStore Empty();
		public (nuint, nuint) AddParsableCertificates(RustlsPkiTypesCertificateDer[] derCerts);
		public void Add(RustlsPkiTypesCertificateDer der);
		public RustlsDistinguishedName[] Subjects();
		public bool IsEmpty();
		public nuint Len();
	}
	public sealed unsafe partial class RustlsPkiTypesCertificateDer : RustObject, IEquatable<RustlsPkiTypesCertificateDer> {
		public RustlsPkiTypesCertificateDer Clone();
		public bool Equals(RustlsPkiTypesCertificateDer? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RustlsPkiTypesCertificateDer? a, RustlsPkiTypesCertificateDer? b);
		public static bool operator !=(RustlsPkiTypesCertificateDer? a, RustlsPkiTypesCertificateDer? b);
		public string Debug();
		public static RustlsPkiTypesCertificateDer FromSlice(ReadOnlySpan<byte> bytes);
		public RustlsPkiTypesCertificateDer IntoOwned();
	}
	public sealed unsafe partial class MoqTokioConnectResolved : RustObject {
		public MoqTokioConnectResolved Clone();
		public string Debug();
		public SocketAddr Bind { get; set; }
		public Duration Race { get; set; }
		public Duration ResolutionDelay { get; set; }
		public Duration? Timeout { get; set; }
	}
	public sealed unsafe partial class MoqTokioConnectionHandover : RustObject {
		public MoqTokioConnectionHandover Clone();
		public string Debug();
		public static MoqTokioConnectionHandover Default();
		public MoqTokioRedirect Redirect { get; set; }
		public TokioDuration? Handover { get; set; }
	}
	public sealed unsafe partial class MoqTokioWebsocketConfig : RustObject {
		public MoqTokioWebsocketConfig Clone();
		public string Debug();
		public static MoqTokioWebsocketConfig Default();
		public bool? Enabled { get; set; }
		public TokioDuration? Delay { get; set; }
		public MoqTokioWebsocketResolved Resolve();
	}
	public sealed unsafe partial class MoqTokioConnectionObservation : RustObject {
		public MoqTokioConnectionObservation Clone();
		public string Debug();
		public ulong Revision { get; set; }
		public ulong Connects { get; set; }
		public ClockInstant? LastConnected { get; set; }
		public MoqTokioConnectionPhase Phase { get; set; }
	}
	public sealed unsafe partial class MoqTokioConnectionSnapshot : RustObject {
		public SessionStats Stats { get; set; }
		public Version Version { get; set; }
	}
	public sealed unsafe partial class MoqTokioListenResolved : RustObject {
		public MoqTokioListenResolved Clone();
		public string Debug();
		public TokioListenBind? Bind { get; set; }
		public Versions Versions { get; set; }
		public Str? Implementation { get; set; }
		public Duration? Timeout { get; set; }
		public TokioTlsListen Tls { get; set; }
		public SocketAddrV4? PreferredV4 { get; set; }
		public SocketAddrV6? PreferredV6 { get; set; }
		public MoqTokioListenLoadBalancer Lb { get; set; }
		public PathBuf? ResetSecret { get; set; }
	}
	public sealed unsafe partial class MoqTokioTcpConfig : RustObject {
		public MoqTokioTcpConfig Clone();
		public string Debug();
		public static MoqTokioTcpConfig Default();
		public SocketAddr? Bind { get; set; }
		public bool? Tls { get; set; }
	}
	public sealed unsafe partial class MoqTokioUnixConfig : RustObject {
		public MoqTokioUnixConfig Clone();
		public string Debug();
		public static MoqTokioUnixConfig Default();
		public PathBuf? Bind { get; set; }
		public MoqTokioUnixAllow Allow { get; set; }
	}
	public sealed unsafe partial class MoqTokioListenLoadBalancer : RustObject {
		public MoqTokioListenLoadBalancer Clone();
		public string Debug();
		public static MoqTokioListenLoadBalancer Default();
		public MoqTokioQuicServerId? Id { get; set; }
		public nuint? Nonce { get; set; }
		public MoqTokioListenForward Forward { get; set; }
	}
	public sealed unsafe partial class MoqTokioListenReset : RustObject {
		public MoqTokioListenReset Clone();
		public string Debug();
		public static MoqTokioListenReset Default();
		public PathBuf? Secret { get; set; }
	}
	public sealed unsafe partial class MoqTokioQuicResolved : RustObject {
		public MoqTokioQuicResolved Clone();
		public string Debug();
		public static MoqTokioQuicResolved Default();
		public ulong MaxStreams { get; set; }
		public bool? Gso { get; set; }
		public Duration? IdleTimeout { get; set; }
		public Duration? KeepAlive { get; set; }
		public bool MtuDiscovery { get; set; }
		public MoqTokioQuicCongestionControl CongestionControl { get; set; }
		public ulong ReceiveWindow { get; set; }
		public ulong StreamReceiveWindow { get; set; }
		public ulong SendWindow { get; set; }
		public PathBuf? Qlog { get; set; }
		public PathBuf? Keylog { get; set; }
		public Duration KeepAliveInterval();
		public MoqQuicLimits Limits();
	}
	public sealed unsafe partial class MoqTokioWebsocketListener : RustObject {
		public static ValueTask<MoqTokioWebsocketListener> Bind(SocketAddr addr, CancellationToken cancellationToken = default);
		public MoqTokioWebsocketListener WithProtocols(Str[] protocols);
		public MoqTokioWebsocketListener WithQuic(MoqTokioQuicResolved quic);
		public SocketAddr LocalAddr();
		public MoqTokioAcceptHealth AcceptHealth();
		public ValueTask<QmuxSession> Accept(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class MoqTokioQuicDemuxDemux : RustObject {
		public MoqTokioQuicDemuxDemux Clone();
		public string Debug();
		public static MoqTokioQuicDemuxDemux Default();
		public MoqSockDemuxCounters Counters();
		public MoqTokioQuicDemuxWebRtc Webrtc();
	}
	public sealed unsafe partial class MoqTokioServerStreams : RustObject {
		public string Debug();
		public static MoqTokioServerStreams Default();
	}
	public sealed unsafe partial class MoqTokioServerQuicLimits : RustObject {
		public MoqTokioServerQuicLimits Clone();
		public string Debug();
		public void Set(MoqQuicLimits limits);
	}
	public sealed unsafe partial class MoqQuicLimits : RustObject, IEquatable<MoqQuicLimits> {
		public MoqQuicLimits Clone();
		public bool Equals(MoqQuicLimits? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqQuicLimits? a, MoqQuicLimits? b);
		public static bool operator !=(MoqQuicLimits? a, MoqQuicLimits? b);
		public string Debug();
		public static MoqQuicLimits Default();
		public NonZeroU64? MaxStreams { get; set; }
		public NonZeroU64? ReceiveWindow { get; set; }
		public NonZeroU64? StreamReceiveWindow { get; set; }
		public NonZeroU64? SendWindow { get; set; }
		public MoqQuicRate? ReceiveRate { get; set; }
		public MoqQuicRate? SendRate { get; set; }
		public void Validate();
	}
	public sealed unsafe partial class MoqTokioConnectAddr : RustObject, IEquatable<MoqTokioConnectAddr> {
		public MoqTokioConnectAddr Clone();
		public bool Equals(MoqTokioConnectAddr? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqTokioConnectAddr? a, MoqTokioConnectAddr? b);
		public static bool operator !=(MoqTokioConnectAddr? a, MoqTokioConnectAddr? b);
		public string Debug();
		public static MoqTokioConnectAddr New(Url url);
		public static MoqTokioConnectAddr Pinned(Url url, SocketAddr[] addrs);
		public Url Url();
		public SocketAddr[]? Addresses();
	}
	public sealed unsafe partial class RustlsPkiTypesTrustAnchor : RustObject, IEquatable<RustlsPkiTypesTrustAnchor> {
		public RustlsPkiTypesTrustAnchor Clone();
		public bool Equals(RustlsPkiTypesTrustAnchor? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RustlsPkiTypesTrustAnchor? a, RustlsPkiTypesTrustAnchor? b);
		public static bool operator !=(RustlsPkiTypesTrustAnchor? a, RustlsPkiTypesTrustAnchor? b);
		public string Debug();
		public RustlsPkiTypesDer Subject { get; set; }
		public RustlsPkiTypesDer SubjectPublicKeyInfo { get; set; }
		public RustlsPkiTypesDer? NameConstraints { get; set; }
		public RustlsPkiTypesTrustAnchor ToOwned();
	}
	public sealed unsafe partial class MoqTokioConnectionPhase : RustObject {
		public MoqTokioConnectionPhase Clone();
		public string Debug();
	}
	public sealed unsafe partial class MoqTokioQuicServerId : RustObject, IEquatable<MoqTokioQuicServerId> {
		public MoqTokioQuicServerId Clone();
		public bool Equals(MoqTokioQuicServerId? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MoqTokioQuicServerId? a, MoqTokioQuicServerId? b);
		public static bool operator !=(MoqTokioQuicServerId? a, MoqTokioQuicServerId? b);
		public string Debug();
	}
	public sealed unsafe partial class MoqTokioListenForward : RustObject {
		public MoqTokioListenForward Clone();
		public string Debug();
		public static MoqTokioListenForward Default();
		public SocketAddr? Bind { get; set; }
		public PathBuf? Key { get; set; }
		public PathBuf? Peers { get; set; }
	}
	public sealed unsafe partial class RustlsClientDangerDangerousClientConfig : RustObject {
		public string Debug();
		public RustlsClientConfig Cfg { get; }
	}
	public sealed unsafe partial class RustlsClientResumption : RustObject {
		public RustlsClientResumption Clone();
		public string Debug();
		public static RustlsClientResumption Default();
		public static RustlsClientResumption InMemorySessions(nuint num);
		public static RustlsClientResumption Disabled();
		public RustlsClientResumption Tls12Resumption(RustlsClientTls12Resumption tls12);
	}
	public sealed unsafe partial class RustlsDistinguishedName : RustObject {
		public RustlsDistinguishedName Clone();
		public string Debug();
		public static RustlsDistinguishedName InSequence(ReadOnlySpan<byte> bytes);
	}
	public sealed unsafe partial class MoqTokioWebsocketResolved : RustObject {
		public MoqTokioWebsocketResolved Clone();
		public string Debug();
		public bool Enabled { get; set; }
		public Duration Delay { get; set; }
	}
	public sealed unsafe partial class MoqTokioUnixAllow : RustObject {
		public MoqTokioUnixAllow Clone();
		public string Debug();
		public static MoqTokioUnixAllow Default();
		public uint[] Uid { get; set; }
		public uint[] Gid { get; set; }
		public int[] Pid { get; set; }
	}
	public sealed unsafe partial class MoqTokioWebsocketError : RustObject {
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class QmuxSession : RustObject {
		public QmuxSession Clone();
		public string Debug();
		public Str? Protocol();
		public MoqQuicConnectionMuxVersion Version();
		public void SetLimits(MoqQuicLimits limits);
		public RustlsPkiTypesCertificateDer[]? PeerCertificates();
		public Str? ServerName();
	}
	public sealed unsafe partial class MoqSockDemuxCounters : RustObject {
		public MoqSockDemuxCounters Clone();
		public string Debug();
		public static MoqSockDemuxCounters Default();
		public void Discard(MoqSockDemuxDiscard reason);
		public ulong Get(MoqSockDemuxDiscard reason);
	}
	public sealed unsafe partial class MoqTokioQuicDemuxWebRtc : RustObject {
		public string Debug();
		public ValueTask<MoqTokioQuicDemuxDatagram?> Recv(CancellationToken cancellationToken = default);
		public MoqSockDemuxPin Pin(SocketAddr remote);
	}
	public sealed unsafe partial class RustlsPkiTypesDer : RustObject, IEquatable<RustlsPkiTypesDer> {
		public RustlsPkiTypesDer Clone();
		public bool Equals(RustlsPkiTypesDer? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RustlsPkiTypesDer? a, RustlsPkiTypesDer? b);
		public static bool operator !=(RustlsPkiTypesDer? a, RustlsPkiTypesDer? b);
		public string Debug();
		public static RustlsPkiTypesDer FromSlice(ReadOnlySpan<byte> der);
	}
	public sealed unsafe partial class MoqTokioQuicDemuxDatagram : RustObject {
		public MoqTokioQuicDemuxDatagram Clone();
		public string Debug();
		public SocketAddr Remote { get; set; }
		public IpAddr? Local { get; set; }
		public Bytes Data { get; set; }
	}
	public sealed unsafe partial class MoqSockDemuxPin : RustObject {
		public string Debug();
	}
	public sealed unsafe partial class Url : RustObject, IEquatable<Url>, IComparable<Url> {
		public Url Clone();
		public bool Equals(Url? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Url? a, Url? b);
		public static bool operator !=(Url? a, Url? b);
		public int? PartialCompareTo(Url other);
		public static bool operator <(Url a, Url b);
		public static bool operator <=(Url a, Url b);
		public static bool operator >(Url a, Url b);
		public static bool operator >=(Url a, Url b);
		public int CompareTo(Url? other);
		public override string ToString();
		public string Debug();
		public static Url Parse(Str input);
	}
	public sealed unsafe partial class Json : RustObject {
		public static Json Parse(Str input);
		public Str ToText();
	}
	public sealed unsafe partial class Instant : RustObject, IEquatable<Instant>, IComparable<Instant> {
		public Instant Clone();
		public bool Equals(Instant? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Instant? a, Instant? b);
		public static bool operator !=(Instant? a, Instant? b);
		public int? PartialCompareTo(Instant other);
		public static bool operator <(Instant a, Instant b);
		public static bool operator <=(Instant a, Instant b);
		public static bool operator >(Instant a, Instant b);
		public static bool operator >=(Instant a, Instant b);
		public int CompareTo(Instant? other);
		public string Debug();
	}
	public sealed unsafe partial class RustlsServerConfig : RustObject {
		public RustlsServerConfig Clone();
	}
	public sealed unsafe partial class RustlsResolvesClientCert : RustObject {
		public RustlsResolvesClientCert Clone();
	}
	public sealed unsafe partial class RustlsKeyLog : RustObject {
		public RustlsKeyLog Clone();
	}
	public sealed unsafe partial class RustlsTimeProvider : RustObject {
		public RustlsTimeProvider Clone();
	}
	public sealed unsafe partial class RustlsCertDecompressor : RustObject {
	}
	public sealed unsafe partial class RustlsCertCompressor : RustObject {
	}
	public sealed unsafe partial class RustlsCompressionCache : RustObject {
		public RustlsCompressionCache Clone();
	}
	public sealed unsafe partial class RustlsCryptoProvider : RustObject {
		public RustlsCryptoProvider Clone();
	}
	public interface TransportMetrics {
	}
	public sealed class ClockElapsedException : RustException {
		public ClockElapsed Error { get; }
		public ClockElapsedException(ClockElapsed error);
	}
	public sealed class DecodeErrorException : RustException {
		public DecodeError Error { get; }
		public DecodeErrorException(DecodeError error);
	}
	public sealed class EncodeErrorException : RustException {
		public EncodeError Error { get; }
		public EncodeErrorException(EncodeError error);
	}
	public sealed class ErrorException : RustException {
		public Error Error { get; }
		public ErrorException(Error error);
	}
	public sealed class HangErrorException : RustException {
		public HangError Error { get; }
		public HangErrorException(HangError error);
	}
	public sealed class IntersectionErrorException : RustException {
		public IntersectionError Error { get; }
		public IntersectionErrorException(IntersectionError error);
	}
	public sealed class InvalidPatternException : RustException {
		public InvalidPattern Error { get; }
		public InvalidPatternException(InvalidPattern error);
	}
	public sealed class MoqTokioConnectErrorException : RustException {
		public MoqTokioConnectError Error { get; }
		public MoqTokioConnectErrorException(MoqTokioConnectError error);
	}
	public sealed class MoqTokioWebsocketErrorException : RustException {
		public MoqTokioWebsocketError Error { get; }
		public MoqTokioWebsocketErrorException(MoqTokioWebsocketError error);
	}
	public sealed class MuxCodecAacAdtsErrorException : RustException {
		public MuxCodecAacAdtsError Error { get; }
		public MuxCodecAacAdtsErrorException(MuxCodecAacAdtsError error);
	}
	public sealed class MuxCodecAacErrorException : RustException {
		public MuxCodecAacError Error { get; }
		public MuxCodecAacErrorException(MuxCodecAacError error);
	}
	public sealed class MuxCodecAacImportException : RustException {
		public MuxCodecAacImport Error { get; }
		public MuxCodecAacImportException(MuxCodecAacImport error);
	}
	public sealed class MuxCodecAnnexbErrorException : RustException {
		public MuxCodecAnnexbError Error { get; }
		public MuxCodecAnnexbErrorException(MuxCodecAnnexbError error);
	}
	public sealed class MuxCodecAv1ImportException : RustException {
		public MuxCodecAv1Import Error { get; }
		public MuxCodecAv1ImportException(MuxCodecAv1Import error);
	}
	public sealed class MuxCodecFlacErrorException : RustException {
		public MuxCodecFlacError Error { get; }
		public MuxCodecFlacErrorException(MuxCodecFlacError error);
	}
	public sealed class MuxCodecFlacImportException : RustException {
		public MuxCodecFlacImport Error { get; }
		public MuxCodecFlacImportException(MuxCodecFlacImport error);
	}
	public sealed class MuxCodecH264ErrorException : RustException {
		public MuxCodecH264Error Error { get; }
		public MuxCodecH264ErrorException(MuxCodecH264Error error);
	}
	public sealed class MuxCodecH264ImportException : RustException {
		public MuxCodecH264Import Error { get; }
		public MuxCodecH264ImportException(MuxCodecH264Import error);
	}
	public sealed class MuxCodecH265ErrorException : RustException {
		public MuxCodecH265Error Error { get; }
		public MuxCodecH265ErrorException(MuxCodecH265Error error);
	}
	public sealed class MuxCodecH265ImportException : RustException {
		public MuxCodecH265Import Error { get; }
		public MuxCodecH265ImportException(MuxCodecH265Import error);
	}
	public sealed class MuxCodecMp3ErrorException : RustException {
		public MuxCodecMp3Error Error { get; }
		public MuxCodecMp3ErrorException(MuxCodecMp3Error error);
	}
	public sealed class MuxCodecMp3ImportException : RustException {
		public MuxCodecMp3Import Error { get; }
		public MuxCodecMp3ImportException(MuxCodecMp3Import error);
	}
	public sealed class MuxCodecOpusErrorException : RustException {
		public MuxCodecOpusError Error { get; }
		public MuxCodecOpusErrorException(MuxCodecOpusError error);
	}
	public sealed class MuxCodecOpusImportException : RustException {
		public MuxCodecOpusImport Error { get; }
		public MuxCodecOpusImportException(MuxCodecOpusImport error);
	}
	public sealed class MuxCodecVp8ImportException : RustException {
		public MuxCodecVp8Import Error { get; }
		public MuxCodecVp8ImportException(MuxCodecVp8Import error);
	}
	public sealed class MuxCodecVp9ImportException : RustException {
		public MuxCodecVp9Import Error { get; }
		public MuxCodecVp9ImportException(MuxCodecVp9Import error);
	}
	public sealed class MuxContainerFmp4ErrorException : RustException {
		public MuxContainerFmp4Error Error { get; }
		public MuxContainerFmp4ErrorException(MuxContainerFmp4Error error);
	}
	public sealed class MuxContainerProducerContainerAudioConfigException : RustException {
		public MuxContainerProducerContainerAudioConfig Error { get; }
		public MuxContainerProducerContainerAudioConfigException(MuxContainerProducerContainerAudioConfig error);
	}
	public sealed class MuxContainerProducerContainerBinaryConfigException : RustException {
		public MuxContainerProducerContainerBinaryConfig Error { get; }
		public MuxContainerProducerContainerBinaryConfigException(MuxContainerProducerContainerBinaryConfig error);
	}
	public sealed class MuxContainerProducerContainerException : RustException {
		public MuxContainerProducerContainer Error { get; }
		public MuxContainerProducerContainerException(MuxContainerProducerContainer error);
	}
	public sealed class MuxContainerProducerContainerJsonConfigException : RustException {
		public MuxContainerProducerContainerJsonConfig Error { get; }
		public MuxContainerProducerContainerJsonConfigException(MuxContainerProducerContainerJsonConfig error);
	}
	public sealed class MuxContainerProducerContainerTextConfigException : RustException {
		public MuxContainerProducerContainerTextConfig Error { get; }
		public MuxContainerProducerContainerTextConfigException(MuxContainerProducerContainerTextConfig error);
	}
	public sealed class MuxContainerProducerContainerVideoConfigException : RustException {
		public MuxContainerProducerContainerVideoConfig Error { get; }
		public MuxContainerProducerContainerVideoConfigException(MuxContainerProducerContainerVideoConfig error);
	}
	public sealed class MuxContainerProducerFmp4WireAudioConfigException : RustException {
		public MuxContainerProducerFmp4WireAudioConfig Error { get; }
		public MuxContainerProducerFmp4WireAudioConfigException(MuxContainerProducerFmp4WireAudioConfig error);
	}
	public sealed class MuxContainerProducerFmp4WireBinaryConfigException : RustException {
		public MuxContainerProducerFmp4WireBinaryConfig Error { get; }
		public MuxContainerProducerFmp4WireBinaryConfigException(MuxContainerProducerFmp4WireBinaryConfig error);
	}
	public sealed class MuxContainerProducerFmp4WireException : RustException {
		public MuxContainerProducerFmp4Wire Error { get; }
		public MuxContainerProducerFmp4WireException(MuxContainerProducerFmp4Wire error);
	}
	public sealed class MuxContainerProducerFmp4WireJsonConfigException : RustException {
		public MuxContainerProducerFmp4WireJsonConfig Error { get; }
		public MuxContainerProducerFmp4WireJsonConfigException(MuxContainerProducerFmp4WireJsonConfig error);
	}
	public sealed class MuxContainerProducerFmp4WireTextConfigException : RustException {
		public MuxContainerProducerFmp4WireTextConfig Error { get; }
		public MuxContainerProducerFmp4WireTextConfigException(MuxContainerProducerFmp4WireTextConfig error);
	}
	public sealed class MuxContainerProducerFmp4WireVideoConfigException : RustException {
		public MuxContainerProducerFmp4WireVideoConfig Error { get; }
		public MuxContainerProducerFmp4WireVideoConfigException(MuxContainerProducerFmp4WireVideoConfig error);
	}
	public sealed class MuxContainerProducerLegacyWireAudioConfigException : RustException {
		public MuxContainerProducerLegacyWireAudioConfig Error { get; }
		public MuxContainerProducerLegacyWireAudioConfigException(MuxContainerProducerLegacyWireAudioConfig error);
	}
	public sealed class MuxContainerProducerLegacyWireBinaryConfigException : RustException {
		public MuxContainerProducerLegacyWireBinaryConfig Error { get; }
		public MuxContainerProducerLegacyWireBinaryConfigException(MuxContainerProducerLegacyWireBinaryConfig error);
	}
	public sealed class MuxContainerProducerLegacyWireException : RustException {
		public MuxContainerProducerLegacyWire Error { get; }
		public MuxContainerProducerLegacyWireException(MuxContainerProducerLegacyWire error);
	}
	public sealed class MuxContainerProducerLegacyWireJsonConfigException : RustException {
		public MuxContainerProducerLegacyWireJsonConfig Error { get; }
		public MuxContainerProducerLegacyWireJsonConfigException(MuxContainerProducerLegacyWireJsonConfig error);
	}
	public sealed class MuxContainerProducerLegacyWireTextConfigException : RustException {
		public MuxContainerProducerLegacyWireTextConfig Error { get; }
		public MuxContainerProducerLegacyWireTextConfigException(MuxContainerProducerLegacyWireTextConfig error);
	}
	public sealed class MuxContainerProducerLegacyWireVideoConfigException : RustException {
		public MuxContainerProducerLegacyWireVideoConfig Error { get; }
		public MuxContainerProducerLegacyWireVideoConfigException(MuxContainerProducerLegacyWireVideoConfig error);
	}
	public sealed class MuxContainerProducerLocWireAudioConfigException : RustException {
		public MuxContainerProducerLocWireAudioConfig Error { get; }
		public MuxContainerProducerLocWireAudioConfigException(MuxContainerProducerLocWireAudioConfig error);
	}
	public sealed class MuxContainerProducerLocWireBinaryConfigException : RustException {
		public MuxContainerProducerLocWireBinaryConfig Error { get; }
		public MuxContainerProducerLocWireBinaryConfigException(MuxContainerProducerLocWireBinaryConfig error);
	}
	public sealed class MuxContainerProducerLocWireException : RustException {
		public MuxContainerProducerLocWire Error { get; }
		public MuxContainerProducerLocWireException(MuxContainerProducerLocWire error);
	}
	public sealed class MuxContainerProducerLocWireJsonConfigException : RustException {
		public MuxContainerProducerLocWireJsonConfig Error { get; }
		public MuxContainerProducerLocWireJsonConfigException(MuxContainerProducerLocWireJsonConfig error);
	}
	public sealed class MuxContainerProducerLocWireTextConfigException : RustException {
		public MuxContainerProducerLocWireTextConfig Error { get; }
		public MuxContainerProducerLocWireTextConfigException(MuxContainerProducerLocWireTextConfig error);
	}
	public sealed class MuxContainerProducerLocWireVideoConfigException : RustException {
		public MuxContainerProducerLocWireVideoConfig Error { get; }
		public MuxContainerProducerLocWireVideoConfigException(MuxContainerProducerLocWireVideoConfig error);
	}
	public sealed class MuxContainerTsErrorException : RustException {
		public MuxContainerTsError Error { get; }
		public MuxContainerTsErrorException(MuxContainerTsError error);
	}
	public sealed class MuxErrorException : RustException {
		public MuxError Error { get; }
		public MuxErrorException(MuxError error);
	}
	public sealed class MuxImportTrackException : RustException {
		public MuxImportTrack Error { get; }
		public MuxImportTrackException(MuxImportTrack error);
	}
	public sealed class MuxImportTrackStreamException : RustException {
		public MuxImportTrackStream Error { get; }
		public MuxImportTrackStreamException(MuxImportTrackStream error);
	}
	public sealed class MuxJsonSnapshotJsonException : RustException {
		public MuxJsonSnapshotJson Error { get; }
		public MuxJsonSnapshotJsonException(MuxJsonSnapshotJson error);
	}
	public sealed class MuxJsonStreamJsonException : RustException {
		public MuxJsonStreamJson Error { get; }
		public MuxJsonStreamJsonException(MuxJsonStreamJson error);
	}
	public sealed class PathErrorException : RustException {
		public PathError Error { get; }
		public PathErrorException(PathError error);
	}
	public sealed class StringException : RustException {
		public Str Error { get; }
		public StringException(Str error);
	}
	public sealed class TimeOverflowException : RustException {
		public TimeOverflow Error { get; }
		public TimeOverflowException(TimeOverflow error);
	}
	public sealed class TokioErrorException : RustException {
		public TokioError Error { get; }
		public TokioErrorException(TokioError error);
	}
	public sealed class TokioTlsErrorException : RustException {
		public TokioTlsError Error { get; }
		public TokioTlsErrorException(TokioTlsError error);
	}
	public sealed class TrackProducerException : RustException {
		public TrackProducer Error { get; }
		public TrackProducerException(TrackProducer error);
	}
	public static unsafe partial class Lib {
		public const nuint PATH_MAX_PARTS = 32U;
		public const nuint PATH_MAX_BYTES = 4096U;
		public const nuint PATTERN_MAX_SEGMENTS = 32U;
		public const nuint PATTERN_MAX_INTERSECTIONS = 1024U;
		public const ulong SETUP_TOKEN_OUT_OF_BAND = 0U;
		public const ulong TRANSPORT_MIN_RECEIVE_WINDOW = 1048576U;
		public const ulong TRANSPORT_SETUP_RECEIVE_WINDOW = 131102U;
		public const ulong HANG_CATALOG_MAX_SAFE_INTEGER = 9007199254740991U;
		public static Str IMPLEMENTATION { get; }
		public static Str[] ALPNS { get; }
		public static Str Escape(Str content);
		public static Str Unescape(Str segment);
		public static ClockInstant ClockNow();
		public static ClockWall ClockWall();
		public static (ClockInstant, ClockWall) ClockOrigin();
		public static ClockRng ClockRng();
		public static ulong ClockSeed();
		public static void ClockHostTime();
		public static ValueTask<Str> ClockDrive(Func<ClockInstant, Waiter, ClockInstant?> poll, CancellationToken cancellationToken = default);
		public static ClockInterval ClockInterval(Duration period);
		public static ClockInterval ClockIntervalAt(ClockInstant start, Duration period);
		public static ValueTask ClockSleep(Duration duration, CancellationToken cancellationToken = default);
		public static ValueTask ClockSleepUntil(ClockInstant at, CancellationToken cancellationToken = default);
		public static ValueTask ClockTimeout(Duration duration, Func<CancellationToken, Task> future, CancellationToken cancellationToken = default);
		public static ValueTask ClockTimeoutAt(ClockInstant deadline, Func<CancellationToken, Task> future, CancellationToken cancellationToken = default);
		public static Duration server_DEFAULT_TIMEOUT { get; }
		public static ValueTask<Error> TimeRunDriver(Driver driver, CancellationToken cancellationToken = default);
		public static ValueTask<Error> TimeRunOriginDriver(OriginDriver driver, CancellationToken cancellationToken = default);
		public static ulong VarintDecodeQuic(ref Bytes r);
		public static void VarintEncodeQuic(ulong value, ref Bytes w);
		public static Duration cache_DEFAULT_EXPIRY { get; }
		public static Str hang_echo_SUFFIX { get; }
		public static Str hang_echo_TRACK { get; }
		public static bool HangEchoIsEcho(Path path);
		public static HangCatalogPriorities hang_catalog_PRIORITY { get; }
		public static Duration hang_container_MAX_AGE { get; }
		public static Timescale hang_container_TIMESCALE { get; }
		public static TrackInfo HangContainerTrackInfo(sbyte priority);
		public static Duration mux_container_DELAY_MAX { get; }
		public static Duration mux_rate_RETRY { get; }
		public static ArrayU8x32 TokioTlsParseFingerprint(Str value);
		public static OriginProducer TokioOriginSpawn();
		public static (TransportMemorySession, TransportMemorySession) TransportMemoryPair(TransportMemoryConfig config);
		public static HangCatalogAudioConfig MuxCodecAacConfig(ReadOnlySpan<byte> init);
		public static Bytes mux_codec_annexb_START_CODE { get; }
		public static Bytes MuxCodecAnnexbFromLengthPrefixed(ReadOnlySpan<byte> payload, nuint lengthSize, Bytes? prefix);
		public static Bytes MuxCodecAnnexbBuildPrefix(Bytes[] nals);
		public static void MuxCodecAnnexbLengthPrefixedToAnnexb(ReadOnlySpan<byte> data, nuint lengthSize, ref Bytes @out);
		public static nuint? MuxCodecAnnexbAfterStartCode(ReadOnlySpan<byte> b);
		public static (nuint, nuint)? MuxCodecAnnexbFindStartCode(ReadOnlySpan<byte> b);
		public static HangCatalogAudioConfig MuxCodecFlacConfig(ReadOnlySpan<byte> init);
		public static HangCatalogVideoConfig MuxCodecH264Config(ReadOnlySpan<byte> buf);
		public static HangCatalogVideoConfig MuxCodecH265Config(ReadOnlySpan<byte> buf);
		public static HangCatalogAudioConfig MuxCodecMp3Config(ReadOnlySpan<byte> init);
		public static Duration mux_codec_opus_WARMUP { get; }
		public static uint? MuxCodecOpusPacketSamples(ReadOnlySpan<byte> packet);
		public static HangCatalogAudioConfig MuxCodecOpusConfig(ReadOnlySpan<byte> init);
		public static MuxContainerMp4Finish? MuxContainerMp4Repair(ISeekableReader file);
	}
}
