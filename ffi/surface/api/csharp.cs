// api.cs
#nullable enable
namespace Api {
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
	public unsafe struct ArrayU8x4 : IEquatable<ArrayU8x4> {
		public ArrayU8x4(ReadOnlySpan<byte> bytes);
		public byte this[int index] { get; }
		public int Length { get; }
		public byte[] ToArray();
		public bool Equals(ArrayU8x4 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
	}
	public unsafe partial struct Parcel {
		public Mark Mark;
		public Bytes Payload;
		public Parcel(Mark mark, Bytes payload);
	}
	public unsafe partial struct Weighted {
		public Score Score;
		public uint Weight;
		public Weighted(Score score, uint weight);
		public ulong Total();
	}
	public unsafe partial struct Stamped {
		public Stamp At;
		public uint Count;
		public Stamped(Stamp at, uint count);
		public ulong Total();
	}
	public unsafe partial struct CancellationError {
		public uint Code;
		public CancellationError(uint code);
	}
	public unsafe partial struct Packet {
		public uint Seq;
		public Bytes Payload;
		public Packet(uint seq, Bytes payload);
	}
	public unsafe partial struct Hasher {
		public byte Seed;
		public Hasher(byte seed);
	}
	public unsafe partial struct Size : IEquatable<Size> {
		public uint Width;
		public uint Height;
		public Size(uint width, uint height);
		public bool Equals(Size other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Size a, Size b);
		public static bool operator !=(Size a, Size b);
		public string Debug();
	}
	public unsafe partial struct Point : IEquatable<Point> {
		public int X;
		public int Y;
		public Point(int x, int y);
		public bool Equals(Point other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Point a, Point b);
		public static bool operator !=(Point a, Point b);
		public string Debug();
		public static Point Default();
		public Point Offset(int dx, int dy);
		public static int GetX();
	}
	public unsafe partial struct Label : IEquatable<Label> {
		public Str Name;
		public Point? At;
		public Duration? Ttl;
		public Label(Str name, Point? at, Duration? ttl);
		public bool Equals(Label other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Label a, Label b);
		public static bool operator !=(Label a, Label b);
		public string Debug();
	}
	public unsafe partial struct Note : IEquatable<Note> {
		public Str? Text;
		public Note(Str? text);
		public bool Equals(Note other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Note a, Note b);
		public static bool operator !=(Note a, Note b);
		public string Debug();
		public static Note Default();
	}
	public unsafe partial struct Reading : IEquatable<Reading> {
		public bool Ok;
		public float Level;
		public double Ratio;
		public Scalar Mark;
		public Shape Shape;
		public Point At;
		public Str Note;
		public ulong Count;
		public sbyte Offset;
		public nuint Len;
		public ushort Tag;
		public Reading(bool ok, float level, double ratio, Scalar mark, Shape shape, Point at, Str note, ulong count, sbyte offset, nuint len, ushort tag);
		public bool Equals(Reading other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Reading a, Reading b);
		public static bool operator !=(Reading a, Reading b);
		public string Debug();
		public Reading? Next();
	}
	public unsafe partial struct Packaged : IEquatable<Packaged> {
		public uint Api;
		public Packaged(uint api);
		public bool Equals(Packaged other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Packaged a, Packaged b);
		public static bool operator !=(Packaged a, Packaged b);
		public string Debug();
		public Point Point();
		public Shape Shape();
	}
	public unsafe partial struct Atlas : IEquatable<Atlas> {
		public (Str, Point)[] Points;
		public uint[] Ids;
		public Atlas((Str, Point)[] points, uint[] ids);
		public bool Equals(Atlas other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Atlas a, Atlas b);
		public static bool operator !=(Atlas a, Atlas b);
		public string Debug();
		public Atlas With(Str name, Point at);
	}
	public unsafe partial struct Span_ : IEquatable<Span_> {
		public (uint, uint) Bounds;
		public (Str, Shape)? Named;
		public Span_((uint, uint) bounds, (Str, Shape)? named);
		public bool Equals(Span_ other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Span_ a, Span_ b);
		public static bool operator !=(Span_ a, Span_ b);
		public string Debug();
	}
	public unsafe partial struct Window : IEquatable<Window> {
		public RangeU64 Range;
		public BoundU64 End;
		public Window(RangeU64 range, BoundU64 end);
		public bool Equals(Window other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Window a, Window b);
		public static bool operator !=(Window a, Window b);
		public string Debug();
		public bool Contains(ulong n);
	}
	public unsafe partial struct Ticket : IEquatable<Ticket> {
		public bool Equals(Ticket other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Ticket a, Ticket b);
		public static bool operator !=(Ticket a, Ticket b);
		public string Debug();
		public static Ticket New(uint number);
		public uint Number();
		public void Wait();
		public static uint Clones();
	}
	public unsafe partial struct Seat : IEquatable<Seat> {
		public Mark Mark;
		public uint Row;
		public Seat(Mark mark, uint row);
		public bool Equals(Seat other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Seat a, Seat b);
		public static bool operator !=(Seat a, Seat b);
		public string Debug();
		public ulong Count();
	}
	public unsafe partial struct Stamp : IEquatable<Stamp>, IComparable<Stamp> {
		public bool Equals(Stamp other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Stamp a, Stamp b);
		public static bool operator !=(Stamp a, Stamp b);
		public int? PartialCompareTo(Stamp other);
		public static bool operator <(Stamp a, Stamp b);
		public static bool operator <=(Stamp a, Stamp b);
		public static bool operator >(Stamp a, Stamp b);
		public static bool operator >=(Stamp a, Stamp b);
		public int CompareTo(Stamp other);
		public string Debug();
		public static Stamp Default();
		public static Stamp FromMillis(ulong millis);
		public ulong Secs();
		public Stamp Whole();
		public bool Later(Stamp other);
	}
	public unsafe partial struct Moment : IEquatable<Moment> {
		public bool Equals(Moment other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Moment a, Moment b);
		public static bool operator !=(Moment a, Moment b);
		public string Debug();
		public static Moment? New(ulong ticks, ulong rate);
		public ulong Millis();
		public Moment After(ulong ticks);
	}
	public unsafe partial struct LongArray : IEquatable<LongArray> {
		public bool Equals(LongArray other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LongArray a, LongArray b);
		public static bool operator !=(LongArray a, LongArray b);
		public string Debug();
		public static LongArray New(ulong low, ulong high);
		public ulong C();
		public void Also(Action<ulong> f);
	}
	public unsafe partial struct JvmField : IEquatable<JvmField> {
		public uint N;
		public JvmField(uint n);
		public bool Equals(JvmField other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(JvmField a, JvmField b);
		public static bool operator !=(JvmField a, JvmField b);
		public string Debug();
	}
	public unsafe partial struct JvmStatic : IEquatable<JvmStatic> {
		public uint N;
		public JvmStatic(uint n);
		public bool Equals(JvmStatic other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(JvmStatic a, JvmStatic b);
		public static bool operator !=(JvmStatic a, JvmStatic b);
		public string Debug();
	}
	public unsafe partial struct OptIn : IEquatable<OptIn> {
		public uint N;
		public OptIn(uint n);
		public bool Equals(OptIn other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(OptIn a, OptIn b);
		public static bool operator !=(OptIn a, OptIn b);
		public string Debug();
	}
	public unsafe partial struct Wide : IEquatable<Wide> {
		public bool Equals(Wide other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Wide a, Wide b);
		public static bool operator !=(Wide a, Wide b);
		public string Debug();
		public static Wide New(ulong seed);
		public ulong Sum();
	}
	public unsafe partial struct Clocked : IEquatable<Clocked> {
		public Moment At;
		public uint Count;
		public Clocked(Moment at, uint count);
		public bool Equals(Clocked other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Clocked a, Clocked b);
		public static bool operator !=(Clocked a, Clocked b);
		public string Debug();
	}
	public unsafe partial struct Rated : IEquatable<Rated> {
		public bool Equals(Rated other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Rated a, Rated b);
		public static bool operator !=(Rated a, Rated b);
		public string Debug();
		public static Rated New(ulong count, ulong scale);
		public ulong Scale();
	}
	public unsafe partial struct Score {
		public string Debug();
		public static Score Of(ulong count);
		public ulong Count();
		public void Add(Score other);
		public Score Doubled();
		public ulong Take();
		public ulong Note(Mark mark);
		public (ulong, ulong) Absorb(ref Score other);
	}
	public unsafe partial struct Hits : IEquatable<Hits> {
		public bool Equals(Hits other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Hits a, Hits b);
		public static bool operator !=(Hits a, Hits b);
		public string Debug();
		public static Hits Default();
		public ulong Hits_ { get; set; }
		public ulong Misses { get; set; }
		public static Hits Of(ulong hits, ulong misses);
		public void Add(Hits other);
		public bool Eq(Hits other);
	}
	public unsafe partial struct Regs : IEquatable<Regs> {
		public bool Equals(Regs other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Regs a, Regs b);
		public static bool operator !=(Regs a, Regs b);
		public string Debug();
		public ushort Fcw { get; set; }
		public ulong Wide { get; set; }
		public Point At { get; set; }
		public byte Tag { get; set; }
		public static Regs New(ushort fcw, ulong wide, Point at);
		public byte Seed();
	}
	public unsafe partial struct Extent : IEquatable<Extent> {
		public bool Equals(Extent other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Extent a, Extent b);
		public static bool operator !=(Extent a, Extent b);
		public string Debug();
		public ulong Start { get; set; }
		public static Extent New(ulong start, ulong len);
		public ulong End();
		public ulong Slide(ulong by);
	}
	public unsafe partial struct PairSizeU8 : IEquatable<PairSizeU8> {
		public Size Left;
		public byte Right;
		public PairSizeU8(Size left, byte right);
		public bool Equals(PairSizeU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PairSizeU8 a, PairSizeU8 b);
		public static bool operator !=(PairSizeU8 a, PairSizeU8 b);
		public string Debug();
		public PairU8Size Swap();
	}
	public readonly struct NonZeroU16 : IEquatable<NonZeroU16>, IComparable<NonZeroU16> {
		public ushort Value { get; }
		public NonZeroU16(ushort value);
		public static explicit operator NonZeroU16(ushort value);
		public static implicit operator ushort(NonZeroU16 value);
		public override string ToString();
		public bool Equals(NonZeroU16 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroU16 other);
	}
	public unsafe partial struct PairNonZeroU16 : IEquatable<PairNonZeroU16> {
		public NonZeroU16 Left;
		public NonZeroU16 Right;
		public PairNonZeroU16(NonZeroU16 left, NonZeroU16 right);
		public bool Equals(PairNonZeroU16 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PairNonZeroU16 a, PairNonZeroU16 b);
		public static bool operator !=(PairNonZeroU16 a, PairNonZeroU16 b);
		public string Debug();
		public PairNonZeroU16 Swap();
		public bool Even();
	}
	public unsafe partial struct PairU32 : IEquatable<PairU32> {
		public uint Left;
		public uint Right;
		public PairU32(uint left, uint right);
		public bool Equals(PairU32 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PairU32 a, PairU32 b);
		public static bool operator !=(PairU32 a, PairU32 b);
		public string Debug();
		public PairU32 Swap();
		public bool Even();
		public uint Sum();
		public PairU8Size BesideSize(Size right);
	}
	public unsafe partial struct PairU8Size : IEquatable<PairU8Size> {
		public byte Left;
		public Size Right;
		public PairU8Size(byte left, Size right);
		public bool Equals(PairU8Size other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PairU8Size a, PairU8Size b);
		public static bool operator !=(PairU8Size a, PairU8Size b);
		public string Debug();
		public PairSizeU8 Swap();
	}
	public unsafe partial struct LeftRightU8 : IEquatable<LeftRightU8> {
		public RightU8 Value;
		public LeftRightU8(RightU8 value);
		public bool Equals(LeftRightU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LeftRightU8 a, LeftRightU8 b);
		public static bool operator !=(LeftRightU8 a, LeftRightU8 b);
		public string Debug();
	}
	public unsafe partial struct LeftU8 : IEquatable<LeftU8> {
		public byte Value;
		public LeftU8(byte value);
		public bool Equals(LeftU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LeftU8 a, LeftU8 b);
		public static bool operator !=(LeftU8 a, LeftU8 b);
		public string Debug();
	}
	public unsafe partial struct RightLeftU8 : IEquatable<RightLeftU8> {
		public LeftU8 Value;
		public RightLeftU8(LeftU8 value);
		public bool Equals(RightLeftU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RightLeftU8 a, RightLeftU8 b);
		public static bool operator !=(RightLeftU8 a, RightLeftU8 b);
		public string Debug();
	}
	public unsafe partial struct RightU8 : IEquatable<RightU8> {
		public byte Value;
		public RightU8(byte value);
		public bool Equals(RightU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(RightU8 a, RightU8 b);
		public static bool operator !=(RightU8 a, RightU8 b);
		public string Debug();
	}
	public unsafe partial struct InnerU8 : IEquatable<InnerU8> {
		public byte Value;
		public InnerU8(byte value);
		public bool Equals(InnerU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(InnerU8 a, InnerU8 b);
		public static bool operator !=(InnerU8 a, InnerU8 b);
		public string Debug();
	}
	public unsafe partial struct BoxedU16 : IEquatable<BoxedU16> {
		public ushort Value;
		public BoxedU16(ushort value);
		public bool Equals(BoxedU16 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(BoxedU16 a, BoxedU16 b);
		public static bool operator !=(BoxedU16 a, BoxedU16 b);
		public string Debug();
	}
	public unsafe partial struct BoxedU8 : IEquatable<BoxedU8> {
		public byte Value;
		public BoxedU8(byte value);
		public bool Equals(BoxedU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(BoxedU8 a, BoxedU8 b);
		public static bool operator !=(BoxedU8 a, BoxedU8 b);
		public string Debug();
	}
	public unsafe partial struct LogCount : IEquatable<LogCount> {
		public uint Kept;
		public LogCount(uint kept);
		public bool Equals(LogCount other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LogCount a, LogCount b);
		public static bool operator !=(LogCount a, LogCount b);
		public string Debug();
	}
	public unsafe partial struct WrapU32 : IEquatable<WrapU32> {
		public uint Value;
		public WrapU32(uint value);
		public bool Equals(WrapU32 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(WrapU32 a, WrapU32 b);
		public static bool operator !=(WrapU32 a, WrapU32 b);
		public string Debug();
	}
	public unsafe partial struct PolarTurn : IEquatable<PolarTurn> {
		public byte Whole;
		public PolarTurn(byte whole);
		public bool Equals(PolarTurn other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PolarTurn a, PolarTurn b);
		public static bool operator !=(PolarTurn a, PolarTurn b);
		public string Debug();
	}
	public unsafe partial struct Peer : IEquatable<Peer> {
		public SocketAddr Addr;
		public Str[] Names;
		public Peer(SocketAddr addr, Str[] names);
		public bool Equals(Peer other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Peer a, Peer b);
		public static bool operator !=(Peer a, Peer b);
		public string Debug();
		public Str[] Aliases();
		public Peer Next(Str name);
	}
	public unsafe partial struct Shadowed : IEquatable<Shadowed> {
		public int[] Other;
		public byte A;
		public byte B;
		public Shadowed(int[] other, byte a, byte b);
		public bool Equals(Shadowed other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Shadowed a, Shadowed b);
		public static bool operator !=(Shadowed a, Shadowed b);
		public string Debug();
	}
	public unsafe partial struct Numbers : IEquatable<Numbers> {
		public ushort[] U16s;
		public uint[] U32s;
		public ulong[] U64s;
		public sbyte[] I8s;
		public short[] I16s;
		public int[] I32s;
		public long[] I64s;
		public float[] F32s;
		public double[] F64s;
		public int[]? Some;
		public Numbers(ushort[] u16s, uint[] u32s, ulong[] u64s, sbyte[] i8s, short[] i16s, int[] i32s, long[] i64s, float[] f32s, double[] f64s, int[]? some);
		public bool Equals(Numbers other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Numbers a, Numbers b);
		public static bool operator !=(Numbers a, Numbers b);
		public string Debug();
		public static Numbers Edges();
		public static Numbers Zero(bool negative);
		public Numbers Reversed();
	}
	public unsafe partial struct Measure : IEquatable<Measure> {
		public double Value;
		public Numbers Numbers;
		public Measure(double value, Numbers numbers);
		public bool Equals(Measure other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Measure a, Measure b);
		public static bool operator !=(Measure a, Measure b);
		public string Debug();
		public static Measure New(double value, bool negative);
	}
	public unsafe partial struct LimitsBytes : IEquatable<LimitsBytes> {
		public Bytes Low;
		public Bytes High;
		public LimitsBytes(Bytes low, Bytes high);
		public bool Equals(LimitsBytes other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LimitsBytes a, LimitsBytes b);
		public static bool operator !=(LimitsBytes a, LimitsBytes b);
		public string Debug();
	}
	public unsafe partial struct LimitsDuration : IEquatable<LimitsDuration> {
		public Duration Low;
		public Duration High;
		public LimitsDuration(Duration low, Duration high);
		public bool Equals(LimitsDuration other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(LimitsDuration a, LimitsDuration b);
		public static bool operator !=(LimitsDuration a, LimitsDuration b);
		public string Debug();
	}
	public unsafe partial struct Page : IEquatable<Page> {
		public uint Size;
		public Page(uint size);
		public bool Equals(Page other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Page a, Page b);
		public static bool operator !=(Page a, Page b);
		public string Debug();
		public static Page Default();
	}
	public unsafe partial struct Booth : IEquatable<Booth> {
		public Mark Mark;
		public Booth(Mark mark);
		public bool Equals(Booth other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Booth a, Booth b);
		public static bool operator !=(Booth a, Booth b);
		public string Debug();
	}
	public unsafe partial struct Cancelled : IEquatable<Cancelled> {
		public byte Step;
		public Cancelled(byte step);
		public bool Equals(Cancelled other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Cancelled a, Cancelled b);
		public static bool operator !=(Cancelled a, Cancelled b);
		public string Debug();
	}
	public unsafe partial struct StartSeekable : IEquatable<StartSeekable> {
		public ulong At;
		public StartSeekable(ulong at);
		public bool Equals(StartSeekable other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(StartSeekable a, StartSeekable b);
		public static bool operator !=(StartSeekable a, StartSeekable b);
		public string Debug();
	}
	public unsafe partial struct DoubleArray : IEquatable<DoubleArray> {
		public uint Len;
		public DoubleArray(uint len);
		public bool Equals(DoubleArray other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(DoubleArray a, DoubleArray b);
		public static bool operator !=(DoubleArray a, DoubleArray b);
		public string Debug();
	}
	public readonly struct NonZeroU8 : IEquatable<NonZeroU8>, IComparable<NonZeroU8> {
		public byte Value { get; }
		public NonZeroU8(byte value);
		public static explicit operator NonZeroU8(byte value);
		public static implicit operator byte(NonZeroU8 value);
		public override string ToString();
		public bool Equals(NonZeroU8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroU8 other);
	}
	public readonly struct NonZeroU32 : IEquatable<NonZeroU32>, IComparable<NonZeroU32> {
		public uint Value { get; }
		public NonZeroU32(uint value);
		public static explicit operator NonZeroU32(uint value);
		public static implicit operator uint(NonZeroU32 value);
		public override string ToString();
		public bool Equals(NonZeroU32 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroU32 other);
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
	public readonly struct NonZeroI8 : IEquatable<NonZeroI8>, IComparable<NonZeroI8> {
		public sbyte Value { get; }
		public NonZeroI8(sbyte value);
		public static explicit operator NonZeroI8(sbyte value);
		public static implicit operator sbyte(NonZeroI8 value);
		public override string ToString();
		public bool Equals(NonZeroI8 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroI8 other);
	}
	public readonly struct NonZeroI16 : IEquatable<NonZeroI16>, IComparable<NonZeroI16> {
		public short Value { get; }
		public NonZeroI16(short value);
		public static explicit operator NonZeroI16(short value);
		public static implicit operator short(NonZeroI16 value);
		public override string ToString();
		public bool Equals(NonZeroI16 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroI16 other);
	}
	public readonly struct NonZeroI32 : IEquatable<NonZeroI32>, IComparable<NonZeroI32> {
		public int Value { get; }
		public NonZeroI32(int value);
		public static explicit operator NonZeroI32(int value);
		public static implicit operator int(NonZeroI32 value);
		public override string ToString();
		public bool Equals(NonZeroI32 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroI32 other);
	}
	public readonly struct NonZeroI64 : IEquatable<NonZeroI64>, IComparable<NonZeroI64> {
		public long Value { get; }
		public NonZeroI64(long value);
		public static explicit operator NonZeroI64(long value);
		public static implicit operator long(NonZeroI64 value);
		public override string ToString();
		public bool Equals(NonZeroI64 other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroI64 other);
	}
	public readonly struct NonZeroIsize : IEquatable<NonZeroIsize>, IComparable<NonZeroIsize> {
		public nint Value { get; }
		public NonZeroIsize(nint value);
		public static explicit operator NonZeroIsize(nint value);
		public static implicit operator nint(NonZeroIsize value);
		public override string ToString();
		public bool Equals(NonZeroIsize other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public int CompareTo(NonZeroIsize other);
	}
	public unsafe partial struct NonZeros {
		public NonZeroU8 U8;
		public NonZeroU16 U16;
		public NonZeroU32 U32;
		public NonZeroU64 U64;
		public NonZeroUsize Usize;
		public NonZeroI8 I8;
		public NonZeroI16 I16;
		public NonZeroI32 I32;
		public NonZeroI64 I64;
		public NonZeroIsize Isize;
		public NonZeros(NonZeroU8 u8, NonZeroU16 u16, NonZeroU32 u32, NonZeroU64 u64, NonZeroUsize usize, NonZeroI8 i8, NonZeroI16 i16, NonZeroI32 i32, NonZeroI64 i64, NonZeroIsize isize);
		public string Debug();
	}
	public unsafe partial struct A : IEquatable<A> {
		public byte B;
		public A(byte b);
		public bool Equals(A other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(A a, A b);
		public static bool operator !=(A a, A b);
		public string Debug();
	}
	public unsafe partial struct Qualifiers : IEquatable<Qualifiers> {
		public double Api;
		public double Kotlin;
		public Qualifiers(double api, double kotlin);
		public bool Equals(Qualifiers other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Qualifiers a, Qualifiers b);
		public static bool operator !=(Qualifiers a, Qualifiers b);
	}
	public unsafe partial struct Version : IEquatable<Version>, IComparable<Version> {
		public ushort Major;
		public ushort Minor;
		public Version(ushort major, ushort minor);
		public bool Equals(Version other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Version a, Version b);
		public static bool operator !=(Version a, Version b);
		public int? PartialCompareTo(Version other);
		public static bool operator <(Version a, Version b);
		public static bool operator <=(Version a, Version b);
		public static bool operator >(Version a, Version b);
		public static bool operator >=(Version a, Version b);
		public int CompareTo(Version other);
		public string Debug();
	}
	public unsafe partial struct Spot : IEquatable<Spot> {
		public uint X;
		public uint Y;
		public Spot(uint x, uint y);
		public bool Equals(Spot other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Spot a, Spot b);
		public static bool operator !=(Spot a, Spot b);
		public string Debug();
		public static Spot Deserialize(Json deserializer);
	}
	public unsafe partial struct Deprecated : IEquatable<Deprecated> {
		public bool On;
		public uint Lower;
		public bool Deprecated_;
		public Deprecated(bool on, uint lower, bool deprecated);
		public bool Equals(Deprecated other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Deprecated a, Deprecated b);
		public static bool operator !=(Deprecated a, Deprecated b);
		public string Debug();
		public static Deprecated Default();
	}
	public unsafe partial struct Dial : IEquatable<Dial> {
		public ulong Level;
		public ulong Before;
		public Dial(ulong level, ulong before);
		public bool Equals(Dial other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Dial a, Dial b);
		public static bool operator !=(Dial a, Dial b);
		public string Debug();
		public static Dial Default();
		public Dial Turn(ulong by);
		public Dial Max();
	}
	public unsafe partial struct Knob : IEquatable<Knob> {
		public ulong Level;
		public Knob(ulong level);
		public bool Equals(Knob other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Knob a, Knob b);
		public static bool operator !=(Knob a, Knob b);
		public string Debug();
		public static Knob Default();
		public ulong Doubled();
	}
	public unsafe partial struct PolarPoint : IEquatable<PolarPoint> {
		public byte Turns;
		public uint Distance;
		public PolarPoint(byte turns, uint distance);
		public bool Equals(PolarPoint other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(PolarPoint a, PolarPoint b);
		public static bool operator !=(PolarPoint a, PolarPoint b);
		public string Debug();
	}
	public unsafe partial struct Polar_Turn : IEquatable<Polar_Turn> {
		public byte Part;
		public Polar_Turn(byte part);
		public bool Equals(Polar_Turn other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Polar_Turn a, Polar_Turn b);
		public static bool operator !=(Polar_Turn a, Polar_Turn b);
		public string Debug();
	}
	public unsafe partial struct MacrosStreams : IEquatable<MacrosStreams> {
		public byte Stdin;
		public byte Stdout;
		public byte Stderr;
		public byte Errno;
		public MacrosStreams(byte stdin, byte stdout, byte stderr, byte errno);
		public bool Equals(MacrosStreams other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(MacrosStreams a, MacrosStreams b);
		public static bool operator !=(MacrosStreams a, MacrosStreams b);
		public string Debug();
	}
	public unsafe partial struct ClashTone : IEquatable<ClashTone> {
		public ushort Hz;
		public ClashTone(ushort hz);
		public bool Equals(ClashTone other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashTone a, ClashTone b);
		public static bool operator !=(ClashTone a, ClashTone b);
		public string Debug();
	}
	public unsafe partial struct ClashTone_ : IEquatable<ClashTone_> {
		public byte Db;
		public ClashTone_(byte db);
		public bool Equals(ClashTone_ other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashTone_ a, ClashTone_ b);
		public static bool operator !=(ClashTone_ a, ClashTone_ b);
		public string Debug();
	}
	public unsafe partial struct ClashSpan : IEquatable<ClashSpan> {
		public uint Width;
		public ClashSpan(uint width);
		public bool Equals(ClashSpan other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashSpan a, ClashSpan b);
		public static bool operator !=(ClashSpan a, ClashSpan b);
		public override string ToString();
		public string Debug();
		public uint Width_();
		public Str ToString_();
	}
	public unsafe partial struct ClashAny : IEquatable<ClashAny> {
		public Str TypeUrl;
		public ClashAny(Str typeUrl);
		public bool Equals(ClashAny other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashAny a, ClashAny b);
		public static bool operator !=(ClashAny a, ClashAny b);
		public string Debug();
	}
	public unsafe partial struct ClashBreak : IEquatable<ClashBreak> {
		public byte _0;
		public ClashBreak(byte _0);
		public bool Equals(ClashBreak other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashBreak a, ClashBreak b);
		public static bool operator !=(ClashBreak a, ClashBreak b);
		public string Debug();
		public static ClashBreak Sp { get; }
		public static ClashBreak SP { get; }
	}
	public unsafe partial struct ClashMalformed : IEquatable<ClashMalformed> {
		public byte At;
		public ClashMalformed(byte at);
		public bool Equals(ClashMalformed other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashMalformed a, ClashMalformed b);
		public static bool operator !=(ClashMalformed a, ClashMalformed b);
		public override string ToString();
		public string Debug();
		public bool Is(byte at);
	}
	public unsafe partial struct ClashDescribed : IEquatable<ClashDescribed> {
		public byte Description;
		public byte DebugDescription;
		public ClashDescribed(byte description, byte debugDescription);
		public bool Equals(ClashDescribed other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashDescribed a, ClashDescribed b);
		public static bool operator !=(ClashDescribed a, ClashDescribed b);
		public override string ToString();
		public string Debug();
	}
	public unsafe partial struct ClashErrno : IEquatable<ClashErrno> {
		public int Errno;
		public ClashErrno(int errno);
		public bool Equals(ClashErrno other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashErrno a, ClashErrno b);
		public static bool operator !=(ClashErrno a, ClashErrno b);
		public string Debug();
		public static ClashErrno NULL { get; }
	}
	public unsafe partial struct ClashEscaped : IEquatable<ClashEscaped> {
		public byte New;
		public ClashEscaped(byte @new);
		public bool Equals(ClashEscaped other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashEscaped a, ClashEscaped b);
		public static bool operator !=(ClashEscaped a, ClashEscaped b);
		public string Debug();
		public byte Delete_(byte @new, byte @new2);
		public byte Delete();
	}
	public unsafe partial struct ClashUnderscored : IEquatable<ClashUnderscored> {
		public byte New_;
		public byte New;
		public byte Debug_;
		public byte Default_;
		public ClashUnderscored(byte @new2, byte @new, byte debug, byte @default);
		public bool Equals(ClashUnderscored other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashUnderscored a, ClashUnderscored b);
		public static bool operator !=(ClashUnderscored a, ClashUnderscored b);
		public string Debug();
		public static ClashUnderscored Default();
	}
	public unsafe partial struct TlsPolicyLifetime : IEquatable<TlsPolicyLifetime> {
		public uint Seconds;
		public TlsPolicyLifetime(uint seconds);
		public bool Equals(TlsPolicyLifetime other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(TlsPolicyLifetime a, TlsPolicyLifetime b);
		public static bool operator !=(TlsPolicyLifetime a, TlsPolicyLifetime b);
		public string Debug();
	}
	public unsafe partial struct Core {
		public uint Id;
		public Core(uint id);
	}
	public abstract unsafe partial class Choice {
		public sealed class Empty : Choice {
		}
		public sealed class Picked : Choice {
			public Mark _0;
			public Picked(Mark _0);
		}
	}
	public abstract unsafe partial class Array_ {
		public sealed class U8x4 : Array_ {
			public byte _0;
			public U8x4(byte _0);
		}
	}
	public abstract unsafe partial class Platform {
		public sealed class Wasm : Platform {
			public uint Bits;
			public Wasm(uint bits);
		}
		public sealed class Native_ : Platform {
			public uint _0;
			public Native_(uint _0);
		}
	}
	public abstract unsafe partial class Placement {
		public sealed class Anywhere : Placement {
		}
		public sealed class Pinned : Placement {
			public Core _0;
			public Pinned(Core _0);
		}
	}
	public enum Shape {
		Circle = 1,
		Square = 4,
	}
	public static unsafe partial class ShapeExtensions {
		public static string Debug(this Shape self);
		public static Shape? ValueOf(Str name);
		public static Shape[] ALL { get; }
		public static Shape[] BOTH { get; }
	}
	public abstract unsafe partial class Event : IEquatable<Event> {
		public sealed class Moved : Event {
			public Point _0;
			public Moved(Point _0);
		}
		public sealed class Named : Event {
			public Str Name;
			public Named(Str name);
		}
		public sealed class Done : Event {
		}
		public bool Equals(Event? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Event? a, Event? b);
		public static bool operator !=(Event? a, Event? b);
		public string Debug();
	}
	public abstract unsafe partial class Clash : IEquatable<Clash> {
		public sealed class Point : Clash {
			public global::Api.Point _0;
			public Point(global::Api.Point _0);
		}
		public sealed class Shape : Clash {
			public global::Api.Shape _0;
			public Shape(global::Api.Shape _0);
		}
		public sealed class Event : Clash {
			public global::Api.Event _0;
			public Event(global::Api.Event _0);
		}
		public sealed class Double : Clash {
			public double _0;
			public Double(double _0);
		}
		public sealed class Float : Clash {
			public float _0;
			public Float(float _0);
		}
		public sealed class String_ : Clash {
			public Str _0;
			public String_(Str _0);
		}
		public sealed class Int : Clash {
			public int _0;
			public Int(int _0);
		}
		public sealed class Boolean : Clash {
			public bool _0;
			public Boolean(bool _0);
		}
		public sealed class Native_ : Clash {
			public uint _0;
			public Native_(uint _0);
		}
		public sealed class Bytes_ : Clash {
			public Bytes _0;
			public Bytes_(Bytes _0);
		}
		public sealed class Any : Clash {
		}
		public sealed class Unit : Clash {
		}
		public sealed class Waker_ : Clash {
		}
		public sealed class Mut : Clash {
		}
		public sealed class Pair : Clash {
			public byte _0;
			public byte _1;
			public Pair(byte _0, byte _1);
		}
		public sealed class Stamp : Clash {
			public long _0;
			public Stamp(long _0);
		}
		public sealed class Moment : Clash {
			public global::Api.Moment _0;
			public Moment(global::Api.Moment _0);
		}
		public bool Equals(Clash? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Clash? a, Clash? b);
		public static bool operator !=(Clash? a, Clash? b);
		public string Debug();
		public global::Api.Point? Point_();
		public static Clash Of(double value);
		public Bytes Bytes();
		public Reading Reading(uint kotlin);
		public static uint[][] Listed(ReadOnlySpan<uint> java, ref uint kotlin);
		public static ValueTask<Clash> After(Func<CancellationToken, Task> first, double kotlin, CancellationToken cancellationToken = default);
		public Bytes Joined(ReadOnlySpan<byte> lent, ReadOnlySpan<byte> owned, Bytes? maybe);
		public ValueTask<Str> Tagged(Str tag, CancellationToken cancellationToken = default);
		public global::Api.Stamp Stamp_(ulong kotlin);
		public global::Api.Moment Moment_(ulong kotlin);
		public static Clash At(global::Api.Point api);
		public Str Describe(Str text, uint n, global::Api.Shape shape, ref uint @out);
	}
	public abstract unsafe partial class Stage : IEquatable<Stage> {
		public sealed class Init : Stage {
			public uint _0;
			public Init(uint _0);
		}
		public sealed class Done : Stage {
		}
		public bool Equals(Stage? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Stage? a, Stage? b);
		public static bool operator !=(Stage? a, Stage? b);
		public string Debug();
	}
	public abstract unsafe partial class Error : IEquatable<Error> {
		public sealed class Underflow : Error {
			public ulong By;
			public Underflow(ulong by);
		}
		public sealed class Overflow : Error {
		}
		public bool Equals(Error? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Error? a, Error? b);
		public static bool operator !=(Error? a, Error? b);
		public override string ToString();
		public string Debug();
	}
	public enum Status {
		Moved = 0,
		Kept = 1,
	}
	public static unsafe partial class StatusExtensions {
		public static string Debug(this Status self);
	}
	public abstract unsafe partial class Labelled : IEquatable<Labelled> {
		public sealed class Tagged : Labelled {
			public uint Tag;
			public Tagged(uint tag);
		}
		public sealed class Bare : Labelled {
		}
		public bool Equals(Labelled? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Labelled? a, Labelled? b);
		public static bool operator !=(Labelled? a, Labelled? b);
		public string Debug();
	}
	public enum Family {
		Native = 0,
		Wasm = 1,
	}
	public static unsafe partial class FamilyExtensions {
		public static string Debug(this Family self);
	}
	public enum VecClashTone {
		Error = 0,
	}
	public static unsafe partial class VecClashToneExtensions {
		public static string Debug(this VecClashTone self);
	}
	public abstract unsafe partial class B : IEquatable<B> {
		public sealed class One : B {
		}
		public sealed class Two : B {
			public byte _0;
			public Two(byte _0);
		}
		public bool Equals(B? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(B? a, B? b);
		public static bool operator !=(B? a, B? b);
		public string Debug();
	}
	public abstract unsafe partial class Builtin : IEquatable<Builtin> {
		public sealed class Any : Builtin {
			public int[] _0;
			public Any(int[] _0);
		}
		public sealed class Boolean : Builtin {
			public double _0;
			public Boolean(double _0);
		}
		public sealed class Int : Builtin {
			public double _0;
			public Int(double _0);
		}
		public sealed class Native_ : Builtin {
			public double _0;
			public Native_(double _0);
		}
		public bool Equals(Builtin? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Builtin? a, Builtin? b);
		public static bool operator !=(Builtin? a, Builtin? b);
	}
	public abstract unsafe partial class Rank : IEquatable<Rank>, IComparable<Rank> {
		public sealed class Unranked : Rank {
		}
		public sealed class Ranked : Rank {
			public byte Compare;
			public byte PartialCompare;
			public byte CompareTo_;
			public byte PartialCompareTo_;
			public Ranked(byte compare, byte partialCompare, byte compareTo, byte partialCompareTo);
		}
		public bool Equals(Rank? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Rank? a, Rank? b);
		public static bool operator !=(Rank? a, Rank? b);
		public int? PartialCompareTo(Rank other);
		public static bool operator <(Rank a, Rank b);
		public static bool operator <=(Rank a, Rank b);
		public static bool operator >(Rank a, Rank b);
		public static bool operator >=(Rank a, Rank b);
		public int CompareTo(Rank? other);
		public string Debug();
	}
	public enum Tier {
		Low = 0,
		High = 1,
		CompareTo = 2,
		PartialCompareTo = 3,
	}
	public static unsafe partial class TierExtensions {
		public static int? PartialCmp(this Tier self, Tier other);
		public static int Cmp(this Tier self, Tier other);
		public static string Debug(this Tier self);
	}
	public abstract unsafe partial class ClashCandidate : IEquatable<ClashCandidate> {
		public sealed class None : ClashCandidate {
		}
		public sealed class Match : ClashCandidate {
			public byte _0;
			public Match(byte _0);
		}
		public bool Equals(ClashCandidate? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashCandidate? a, ClashCandidate? b);
		public static bool operator !=(ClashCandidate? a, ClashCandidate? b);
		public string Debug();
	}
	public enum ClashLevel {
		Default = 0,
		Debug = 1,
	}
	public static unsafe partial class ClashLevelExtensions {
		public static string Debug_(this ClashLevel self);
		public static ClashLevel Default();
		public static byte Index(this ClashLevel self);
	}
	public abstract unsafe partial class ClashTag : IEquatable<ClashTag> {
		public sealed class Application : ClashTag {
			public uint Number_;
			public Application(uint number);
		}
		public sealed class Private : ClashTag {
			public uint Number_;
			public Private(uint number);
		}
		public bool Equals(ClashTag? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashTag? a, ClashTag? b);
		public static bool operator !=(ClashTag? a, ClashTag? b);
		public string Debug();
		public uint Number();
	}
	public enum ClashOp {
		Delete_ = 0,
		Delete = 1,
	}
	public static unsafe partial class ClashOpExtensions {
		public static string Debug(this ClashOp self);
	}
	public abstract unsafe partial class ClashChange : IEquatable<ClashChange> {
		public sealed class Set : ClashChange {
			public byte New_;
			public byte New;
			public Set(byte @new2, byte @new);
		}
		public sealed class Clear : ClashChange {
		}
		public bool Equals(ClashChange? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashChange? a, ClashChange? b);
		public static bool operator !=(ClashChange? a, ClashChange? b);
		public string Debug();
	}
	public abstract unsafe partial class ClashShift : IEquatable<ClashShift> {
		public sealed class Up : ClashShift {
			public byte _0;
			public Up(byte _0);
		}
		public sealed class Down : ClashShift {
		}
		public sealed class Tag : ClashShift {
			public byte _0;
			public Tag(byte _0);
		}
		public bool Equals(ClashShift? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashShift? a, ClashShift? b);
		public static bool operator !=(ClashShift? a, ClashShift? b);
		public string Debug();
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
	public sealed unsafe partial class Board : RustObject {
		public static Board New((Str, Counter)[] counters);
		public (Str, Counter)[] IntoCounters();
		public (Str, Counter)[] Ranked();
		public Counter Best();
	}
	public sealed unsafe partial class Tab : RustObject {
		public Ledger Ledger { get; }
		public Str Label { get; }
	}
	public sealed unsafe partial class Receipt : RustObject {
		public Receipt Clone();
		public uint Count();
	}
	public sealed unsafe partial class Draft : RustObject {
		public bool Edited();
		public Str Describe();
		public uint Commit();
		public Str Title { get; set; }
		public uint Lines();
		public void Write(Str line);
	}
	public sealed unsafe partial class FiledReceipt : RustObject {
		public FiledReceipt Clone();
		public uint Count();
	}
	public sealed unsafe partial class PadU16 : RustObject {
		public static PadU16 Wide();
		public ushort Tag();
		public ushort Title();
		public Str Title_ { get; }
		public uint Lines();
		public Str Describe();
	}
	public sealed unsafe partial class PadU8 : RustObject {
		public static PadU8 Small();
		public Str Describe();
		public Str Title { get; }
		public uint Lines();
	}
	public sealed unsafe partial class Layered : RustObject {
		public static Layered New();
		public Str Title { get; }
		public uint Lines();
	}
	public sealed unsafe partial class Marker : RustObject {
		public static Marker New(Str label, int x, int y);
		public Str Label();
		public int X();
		public static int Y();
		public int X_ { get; set; }
		public int Y_ { get; set; }
		public Point Offset(int dx, int dy);
	}
	public sealed unsafe partial class Batch : RustObject {
		public static Batch New();
		public bool Push(uint v);
		public nuint Capacity();
	}
	public sealed unsafe partial class Batch2 : RustObject {
		public static Batch2 New();
		public bool Push(uint v);
		public nuint Capacity();
	}
	public sealed unsafe partial class Waiter : RustObject {
		public static Waiter New(Action waker);
		public Waker Waker();
		public Waker Context();
		public static ValueTask PollFuture(Func<CancellationToken, Task> future, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Feed : RustObject, IAsyncEnumerable<Bytes> {
		public ValueTask<Bytes?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<Bytes> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Countdown : RustObject, IAsyncEnumerable<uint> {
		public static Countdown New(uint from);
		public static Countdown Told(uint from, Log log);
		public ValueTask<uint?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<uint> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Retired : RustObject, IAsyncEnumerable<uint> {
		public static Retired New(uint from);
		public ValueTask<uint?> Next(CancellationToken cancellationToken = default);
		public IAsyncEnumerator<uint> GetAsyncEnumerator(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Folio : RustObject {
		public ushort Page { get; set; }
		public Str Title { get; set; }
		public Str? Note { get; set; }
		public Hasp Hasp { get; set; }
		public Hasp Dock { get; }
		public static Folio New(ushort page, Str title);
		public byte Seed();
	}
	public sealed unsafe partial class Logger : RustObject {
		public static Logger New(Log sink);
		public bool Write(Str line);
		public bool WriteFromThread(Str line);
	}
	public sealed unsafe partial class Journal : RustObject {
		public static Journal New(Log sink);
		public void Redirect(Log sink);
		public bool Write(Str line);
	}
	public sealed unsafe partial class Epilogue : RustObject {
		public static Epilogue New(Log sink, uint id);
		public void Redirect(Log sink);
	}
	public sealed unsafe partial class Local : RustObject, IEquatable<Local> {
		public bool Equals(Local? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Local? a, Local? b);
		public static bool operator !=(Local? a, Local? b);
		public static Local Default();
		public static Local New();
		public uint Bump();
		public Local Share();
		public ValueTask<uint> Later(CancellationToken cancellationToken = default);
		public uint Visit(Func<uint> f);
		public ValueTask<uint> PollCount(CancellationToken cancellationToken = default);
		public ulong ThreadNumber();
		public bool OnOwnThread();
	}
	public sealed unsafe partial class Wrapped : RustObject {
		public static Wrapped New(ulong n);
		public ulong N { get; set; }
		public bool On { get; set; }
		public ulong Count();
		public void Recount(ulong n);
		public ulong Steps();
		public uint Locals();
	}
	public sealed unsafe partial class Buffer_ : RustObject {
		public static Buffer_ New(uint len);
		public void Blank();
		public bool Zeroed();
	}
	public sealed unsafe partial class LocalWaiter : RustObject {
		public static LocalWaiter New(Action waker);
		public static ValueTask<uint> PollReady(uint n, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Retry : RustObject {
		public uint? Limit { get; set; }
		public uint? Delay { get; set; }
		public static Retry New(uint? limit);
		public uint Limit_();
		public uint Delay_();
		public void SetLimit(uint limit);
	}
	public sealed unsafe partial class Digest : RustObject {
		public ArrayU8x4 Sum { get; set; }
		public static Digest New();
		public ArrayU8x4 Flipped(ArrayU8x4 mask);
	}
	public sealed unsafe partial class Cursor : RustObject {
		public ulong At();
	}
	public sealed unsafe partial class OuterInnerU8 : RustObject {
		public static OuterInnerU8 New(byte value);
		public byte Get();
	}
	public sealed unsafe partial class SlotU32 : RustObject {
		public uint Value { get; set; }
		public static SlotU32 New(uint value);
	}
	public sealed unsafe partial class SlotU8 : RustObject {
		public byte Value { get; set; }
		public static SlotU8 New(byte value);
		public ushort Value_();
	}
	public sealed unsafe partial class Resolving : RustObject {
		public TlsResolver _0 { get; set; }
		public static Resolving New(Str name);
	}
	public sealed unsafe partial class Codecs : RustObject {
		public static Codecs Default();
		public TlsCodec[] List { get; set; }
		public static Codecs New();
		public uint Ids();
	}
	public sealed unsafe partial class Der : RustObject {
		public static Der FromSlice(ReadOnlySpan<byte> bytes);
		public Bytes Bytes();
	}
	public sealed unsafe partial class Verbosity : RustObject {
		public Level Level { get; set; }
		public static Verbosity New(Level level);
		public Level? Filter();
	}
	public sealed unsafe partial class Site : RustObject {
		public Url Home { get; set; }
		public static Site New(Url home);
	}
	public sealed unsafe partial class Trail : RustObject {
		public static Trail New(Point[] points, Str[] names);
		public Point[] Points();
		public Label[] Labels();
		public Point[] Iter();
		public Str[] Names();
		public Point[] Beyond(int x);
		public Bytes Bytes();
		public Label[] Drain();
	}
	public sealed unsafe partial class Worker : RustObject {
		public static Worker New(uint id);
		public uint Id();
	}
	public sealed unsafe partial class Handle : RustObject {
		public static Handle New(uint count);
		public uint Count();
		public static ValueTask<Handle> Later(uint count, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Holder : RustObject {
		public static Holder New(ulong level);
		public ulong Level { get; }
		public ulong Doubled();
	}
	public sealed unsafe partial class Tag : RustObject {
		public static Tag Default();
		public static Tag New();
		public Str Tagged(ref uint n, Func<Scalar> f);
	}
	public sealed unsafe partial class Area : RustObject {
		public Area Clone();
		public string Debug();
		public static Area New(Size size);
		public Size Size();
		public ulong Pixels();
		public ulong ScaledPoint(Point by);
	}
	public sealed unsafe partial class Counter : RustObject {
		public Counter Clone();
		public string Debug();
		public static Counter Default();
		public static Counter New(Str name, ulong start);
		public ulong Get();
		public ulong Offset(ulong handle, ulong h);
		public ulong IntoSum(ulong close);
		public Str Name();
		public ulong Add(ulong n);
		public ulong Sub(ulong n);
		public bool Edited();
		public ulong Plus(Counter? other);
		public ulong Clamped(Counter? low, Counter? high);
		public ValueTask<ulong> PlusLater(Counter? other, CancellationToken cancellationToken = default);
		public bool AddTo(Counter? other);
		public Label IntoLabel(Point? at);
		public void Rename(Str name);
		public (Counter, Counter) Split(ulong at);
		public bool Is(Str name);
		public static Counter SeededPoint(Str name, Point from);
		public static Counter SeededCounter(Str name, Counter from);
		public static Counter SeededSize(Str name, Size from);
		public static Counter SeededWrapU32(Str name, WrapU32 from);
		public static Counter SeededPolarPoint(Str name, PolarPoint from);
	}
	public sealed unsafe partial class Meter : RustObject {
		public static Meter Default();
		public ulong Add(ulong n);
		public ulong Count();
		public void Reset();
	}
	public sealed unsafe partial class Meters : RustObject {
		public static Meters Default();
		public Meter Last { get; set; }
		public Meter? Spare { get; set; }
		public static Meters New();
		public Meter First();
		public void Clear();
	}
	public sealed unsafe partial class Registry : RustObject {
		public static Registry Default();
		public Counter First { get; set; }
		public static Registry New(Counter first);
		public Counter? Last();
		public void Push(Counter counter);
		public bool Edit(Func<Counter, bool> edit);
	}
	public sealed unsafe partial class Ledger : RustObject {
		public static Ledger Default();
		public static Ledger New();
		public Draft Edit();
		public uint Published();
		public Str Title();
		public void Seal();
		public Receipt Receipt();
		public Receipt Noted();
		public FiledReceipt FileReceipt(Receipt item);
		public Tab Tab();
		public Slip Slip();
	}
	public sealed unsafe partial class Slip : RustObject {
		public Slip Clone();
		public Receipt Receipt { get; set; }
		public Receipt Global { get; set; }
		public uint Count();
	}
	public sealed unsafe partial class Sheet : RustObject {
		public Sheet Clone();
		public string Debug();
		public static Sheet Default();
		public Str Title { get; set; }
		public static Sheet New();
		public uint Lines();
		public void Write(Str line);
		public Str Describe();
	}
	public sealed unsafe partial class Name : RustObject, IEquatable<Name> {
		public Name Clone();
		public bool Equals(Name? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Name? a, Name? b);
		public static bool operator !=(Name? a, Name? b);
		public string Debug();
		public static Name Owned(Str text);
		public Str AsStr();
		public Name Join(Name other);
		public static Name Borrowed(Str text);
		public bool Shares(Name other);
		public void Rename(Str text);
	}
	public sealed unsafe partial class Channel : RustObject {
		public Channel Clone();
		public static Channel Default();
		public static Channel New();
		public void Clear();
		public void Send(ReadOnlySpan<byte> payload);
		public ValueTask<Bytes> Recv(CancellationToken cancellationToken = default);
		public void Notify(Action waker);
		public ValueTask SendLater(ReadOnlySpan<byte> payload, CancellationToken cancellationToken = default);
		public ValueTask<Str> Tagged(Str tag, CancellationToken cancellationToken = default);
		public ValueTask<nuint> Itemized(uint[]? items, CancellationToken cancellationToken = default);
		public ValueTask<Bytes> PollTake(CancellationToken cancellationToken = default);
		public ValueTask<nuint> PollMeasure(nuint extra, CancellationToken cancellationToken = default);
		public void Listen(Action cx);
		public ValueTask<(Bytes, nuint)> RecvSized(CancellationToken cancellationToken = default);
		public ValueTask<Bytes> RecvNonempty(CancellationToken cancellationToken = default);
		public Feed Feed();
		public void SendFromThread(ReadOnlySpan<byte> payload);
	}
	public sealed unsafe partial class Gap : RustObject, IEquatable<Gap> {
		public Gap Clone();
		public bool Equals(Gap? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Gap? a, Gap? b);
		public static bool operator !=(Gap? a, Gap? b);
		public string Debug();
		public static Gap New(ulong wide, byte narrow);
		public ulong Sum();
	}
	public sealed unsafe partial class Berth : RustObject {
		public Berth Clone();
		public string Debug();
		public static Berth New(uint? at);
		public uint? At();
	}
	public sealed unsafe partial class Mark : RustObject, IEquatable<Mark> {
		public Mark Clone();
		public bool Equals(Mark? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Mark? a, Mark? b);
		public static bool operator !=(Mark? a, Mark? b);
		public string Debug();
		public static Mark Of(ulong count);
		public ulong Count();
		public void Add(ulong by);
	}
	public sealed unsafe partial class Hasp : RustObject {
		public static Hasp Default();
		public bool Set();
	}
	public sealed unsafe partial class Message : RustObject {
		public Message Clone();
		public string Debug();
		public Stamp? Stamp { get; set; }
		public Bytes Payload { get; set; }
		public static Message New(Stamp? stamp, ReadOnlySpan<byte> payload);
		public bool Sent();
	}
	public sealed unsafe partial class Node : RustObject {
		public string Debug();
		public ulong Value { get; set; }
		public Node? Next { get; set; }
		public static Node New(ulong value, Node? next);
		public void Push(ulong value);
		public ulong Sum();
	}
	public sealed unsafe partial class Tally : RustObject, IEquatable<Tally> {
		public bool Equals(Tally? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Tally? a, Tally? b);
		public static bool operator !=(Tally? a, Tally? b);
		public string Debug();
		public static Tally Default();
		public static Tally New();
		public uint Bump();
		public ValueTask<uint> BumpLater(CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Tallies : RustObject {
		public static Tallies Default();
		public static Tallies New();
		public Tally First();
	}
	public sealed unsafe partial class Plain : RustObject, IEquatable<Plain> {
		public bool Equals(Plain? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Plain? a, Plain? b);
		public static bool operator !=(Plain? a, Plain? b);
		public string Debug();
		public ulong N { get; set; }
		public bool On { get; set; }
		public static Plain New(ulong n, bool on);
		public ulong Count();
		public void Recount(ulong n);
		public ulong Steps();
		public uint Locals();
	}
	public sealed unsafe partial class Labeled : RustObject, IEquatable<Labeled> {
		public bool Equals(Labeled? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Labeled? a, Labeled? b);
		public static bool operator !=(Labeled? a, Labeled? b);
		public string Debug();
		public static Labeled New(Str label);
	}
	public sealed unsafe partial class Gauge : RustObject {
		public Gauge Clone();
		public static Gauge New(ulong level);
		public void Set(ulong level);
		public ulong Level();
		public ulong IntoLevel();
		public ValueTask<ulong> Later(CancellationToken cancellationToken = default);
		public ValueTask<bool> Reaches(ulong @this, CancellationToken cancellationToken = default);
	}
	public sealed unsafe partial class Latch : RustObject {
		public Latch Clone();
		public static Latch Default();
		public bool Set(uint value);
		public uint? Get();
	}
	public sealed unsafe partial class Token : RustObject {
		public Token Clone();
		public uint Id { get; set; }
		public static Token New(uint id);
		public Token[] Twice();
	}
	public sealed unsafe partial class Stack : RustObject {
		public string Debug();
		public static Stack Default();
		public static Stack New();
		public void Push(Counter item);
		public Counter? Pop();
		public nuint Depth();
		public ulong DepthPastPoint(Point from);
		public ulong DepthPastCounter(Counter from);
		public ulong DepthPastSize(Size from);
		public ulong DepthPastWrapU32(WrapU32 from);
		public ulong DepthPastPolarPoint(PolarPoint from);
	}
	public sealed unsafe partial class Refusal : RustObject {
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class Fault : RustObject {
		public override string ToString();
		public string Debug();
	}
	public sealed unsafe partial class C : RustObject, IEquatable<C> {
		public bool Equals(C? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(C? a, C? b);
		public static bool operator !=(C? a, C? b);
		public string Debug();
		public static C New(byte value);
	}
	public sealed unsafe partial class Quantity : RustObject, IEquatable<Quantity> {
		public bool Equals(Quantity? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Quantity? a, Quantity? b);
		public static bool operator !=(Quantity? a, Quantity? b);
		public int? PartialCompareTo(Quantity other);
		public static bool operator <(Quantity a, Quantity b);
		public static bool operator <=(Quantity a, Quantity b);
		public static bool operator >(Quantity a, Quantity b);
		public static bool operator >=(Quantity a, Quantity b);
		public string Debug();
		public static Quantity New(long value, Scalar unit);
	}
	public sealed unsafe partial class Pass : RustObject, IEquatable<Pass>, IComparable<Pass> {
		public bool Equals(Pass? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Pass? a, Pass? b);
		public static bool operator !=(Pass? a, Pass? b);
		public int? PartialCompareTo(Pass other);
		public static bool operator <(Pass a, Pass b);
		public static bool operator <=(Pass a, Pass b);
		public static bool operator >(Pass a, Pass b);
		public static bool operator >=(Pass a, Pass b);
		public int CompareTo(Pass? other);
		public string Debug();
		public static Pass New(uint number);
		public uint Redeem();
	}
	public sealed unsafe partial class Key : RustObject, IEquatable<Key> {
		public Key Clone();
		public bool Equals(Key? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Key? a, Key? b);
		public static bool operator !=(Key? a, Key? b);
		public string Debug();
		public static Key New(Str name);
		public ulong HashOf();
	}
	public sealed unsafe partial class ClashSeed : RustObject {
		public static ClashSeed Default();
		public static ClashSeed Default_();
		public uint Value();
		public bool Bool();
	}
	public sealed unsafe partial class ClashEmptyString : RustObject {
		public bool IsEmpty();
	}
	public sealed unsafe partial class ClashStore : RustObject {
		public static ClashStore New(uint count);
		public uint Free();
		public uint Close();
		public void Wait();
		public void Finalize_();
		public uint Take(Ticket ticket);
		public static uint Take_(Ticket ticket);
		public static uint Count_();
		public uint Count();
		public static ClashStore Open_();
		public uint Open();
	}
	public sealed unsafe partial class ClashReader : RustObject {
		public static ClashReader New(ReadOnlySpan<byte> bytes);
		public byte ReadByte();
		public void UnreadByte();
		public void Wait(ulong ms);
	}
	public sealed unsafe partial class ClashNotch : RustObject, IEquatable<ClashNotch>, IComparable<ClashNotch> {
		public ClashNotch Clone();
		public bool Equals(ClashNotch? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(ClashNotch? a, ClashNotch? b);
		public static bool operator !=(ClashNotch? a, ClashNotch? b);
		public int? PartialCompareTo(ClashNotch other);
		public static bool operator <(ClashNotch a, ClashNotch b);
		public static bool operator <=(ClashNotch a, ClashNotch b);
		public static bool operator >(ClashNotch a, ClashNotch b);
		public static bool operator >=(ClashNotch a, ClashNotch b);
		public int CompareTo(ClashNotch? other);
		public override string ToString();
		public string Debug();
		public static ClashNotch Default();
		public static ClashNotch New(uint count);
		public uint Count();
	}
	public sealed unsafe partial class TlsSession : RustObject {
		public Str Alpn();
		public TlsPolicyLifetime Lifetime();
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
	public sealed unsafe partial class StdIoError : RustObject {
	}
	public sealed unsafe partial class TlsConfig : RustObject {
		public TlsConfig Clone();
	}
	public sealed unsafe partial class TlsCert : RustObject {
	}
	public sealed unsafe partial class TlsResolver : RustObject {
		public TlsResolver Clone();
	}
	public sealed unsafe partial class TlsCodec : RustObject {
	}
	public sealed unsafe partial class Spawned : RustObject {
		public static Spawned New(Func<uint> task);
		public uint Join();
	}
	public sealed unsafe partial class LogThread : RustObject {
		public static LogThread New(Log log, Str line);
		public bool Join();
	}
	public sealed unsafe partial class Pinned : RustObject {
		public ulong Value { get; set; }
		public static Pinned New(ulong value);
		public Part Part();
	}
	public sealed unsafe partial class Guarded : RustObject {
		public static Guarded New(ulong n);
		public ulong N { get; }
		public bool On { get; }
		public ulong Count();
		public ulong Steps();
		public uint Locals();
	}
	public sealed unsafe partial class Hooked : RustObject, IEquatable<Hooked> {
		public bool Equals(Hooked? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Hooked? a, Hooked? b);
		public static bool operator !=(Hooked? a, Hooked? b);
		public string Debug();
		public ulong N { get; set; }
		public static Hooked New(ulong n);
		public ulong Count();
	}
	public sealed unsafe partial class Forged : RustObject, IEquatable<Forged> {
		public bool Equals(Forged? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Forged? a, Forged? b);
		public static bool operator !=(Forged? a, Forged? b);
		public string Debug();
		public ulong N { get; set; }
		public static Forged New(ulong n);
	}
	public sealed unsafe partial class Part : RustObject, IEquatable<Part> {
		public bool Equals(Part? other);
		public override bool Equals(object? obj);
		public override int GetHashCode();
		public static bool operator ==(Part? a, Part? b);
		public static bool operator !=(Part? a, Part? b);
		public string Debug();
		public ulong Value { get; set; }
	}
	public interface Log {
		bool Log(byte level, Str line);
	}
	public interface Sensor {
		uint Read();
		byte Interface();
		void Close();
	}
	public interface Scan {
		ulong Scan(Bytes payload, Bytes? rest);
	}
	public interface Beat {
		uint Beat();
	}
	public interface Turner {
		ulong By();
		ulong Once();
	}
	public interface Undo {
		uint Delete(uint n);
		uint Free();
		byte Type();
	}
	public interface Stretch {
		nuint Stretch();
	}
	public interface ClashSource {
		void Wait();
		byte ReadByte();
	}
	public interface ClashEraser {
		byte Delete_(byte @new2, byte @new);
		byte Delete();
	}
	public interface ClashCount {
		uint Next();
	}
	public sealed class BoundsU64Exception : RustException {
		public BoundsU64 Error { get; }
		public BoundsU64Exception(BoundsU64 error);
	}
	public sealed class ClashMalformedException : RustException {
		public ClashMalformed Error { get; }
		public ClashMalformedException(ClashMalformed error);
	}
	public sealed class ErrorException : RustException {
		public Error Error { get; }
		public ErrorException(Error error);
	}
	public sealed class FaultException : RustException {
		public Fault Error { get; }
		public FaultException(Fault error);
	}
	public sealed class RefusalException : RustException {
		public Refusal Error { get; }
		public RefusalException(Refusal error);
	}
	public sealed class StampException : RustException {
		public Stamp Error { get; }
		public StampException(Stamp error);
	}
	public sealed class StringException : RustException {
		public Str Error { get; }
		public StringException(Str error);
	}
	public sealed class U32Exception : RustException {
		public uint Error { get; }
		public U32Exception(uint error);
	}
	public sealed class VecClashToneException : RustException {
		public ClashTone[] Error { get; }
		public VecClashToneException(ClashTone[] error);
	}
	public static unsafe partial class Lib {
		public const ulong LIMIT = 1000U;
		public const long FAR = 9223372036854775807;
		public const int EOF = -1;
		public const ulong URL = 1000U;
		public const ulong STEP = 1U;
		public static nuint WORD { get; }
		public static bool HasText(Note note);
		public static ulong Marked(Mark apiMarked);
		public static Reading? Reading(uint n);
		public static Str ReadingDebug(uint n);
		public static Reading? ReadingEcho(Reading? reading);
		public static void ReadingBump(ref Reading? reading);
		public static Stage Advance(Stage stage);
		public static ValueTuple<Counter> Only(Counter counter);
		public static (byte, short, uint, ulong) Widths(ulong n);
		public static (ulong, ulong) Divide(ulong a, ulong b);
		public static (uint, uint) Corner(bool unit);
		public static ValueTask<(uint, uint)> CornerLater(CancellationToken cancellationToken = default);
		public static (byte, short, uint, ulong)? Spread(ulong n);
		public static (nuint, nuint)? Find(ReadOnlySpan<byte> haystack, ReadOnlySpan<byte> needle);
		public static (Str, Shape, Point)[] Rows(Point[] points);
		public static ushort Port(Str text);
		public static uint Span(uint start, uint end);
		public static ((uint, uint), (uint, uint)) Halves(Span_ span);
		public static ValueTuple<uint?>? Doubled(ValueTuple<uint?>? value);
		public static ValueTuple<Str>? Title(bool formal);
		public static ValueTuple<Str>? Named(Str name);
		public static ValueTuple<Bytes> LentBack(ValueTuple<Bytes> value);
		public static ValueTuple<Bytes> BoxedTuple(ReadOnlySpan<byte> bytes);
		public static (uint, Str) Swapped((Str, uint) pair);
		public static bool Lasting(Draft draft);
		public static uint Stored(Receipt receipt);
		public static uint Tallied(Receipt receipt);
		public static uint SummedUpReceipt(Receipt totaled);
		public static uint Erased(Receipt receipt);
		public static Name Longer(Name a, Name b);
		public static Name Unnamed();
		public static ulong Total(RangeU64 range);
		public static uint Reach(uint start, uint end);
		public static U128? Scaled(U128 v, ulong by);
		public static Batch2 Pair();
		public static ulong Drain4(Batch batch);
		public static ulong Drain2(Batch2 batch);
		public static nuint Capacities44(Batch a, Batch b);
		public static nuint Capacities42(Batch a, Batch2 b);
		public static nuint Capacities24(Batch2 a, Batch b);
		public static nuint Capacities22(Batch2 a, Batch2 b);
		public static ValueTask<uint[]> Fill4(Batch batch, uint v, CancellationToken cancellationToken = default);
		public static ValueTask<uint[]> Fill2(Batch2 batch, uint v, CancellationToken cancellationToken = default);
		public static RangeU64 Widen(RangeU64 range, ulong by);
		public static nuint Count(BoundsU64 range);
		public static BoundU64 End(ulong limit);
		public static (BoundU64, BoundU64) Ends(ulong limit);
		public static ulong Below(ulong n, ulong limit);
		public static ValueTask<Str> Drive(Moment start, Func<Moment, Waiter, Moment?> poll, CancellationToken cancellationToken = default);
		public static Str Made(uint n, Func<uint, Str> make);
		public static (Str, Ticket)[] Tickets((Str, Ticket)[] entries);
		public static Ticket Reissue(Ticket ticket, Func<Ticket, Ticket> swap);
		public static Ticket ReissueLent(Ticket ticket, Func<Ticket, Ticket> swap);
		public static Seat Seat(Mark mark, uint row);
		public static ulong Admit((Counter, Mark, Seat, Choice) entry);
		public static ulong Choose(Choice choice);
		public static ulong Heft((Mark, Bytes) marked, Parcel lent, (Counter, Bytes) moved);
		public static ulong Spend(Counter e, Counter it);
		public static bool IsKept(Status status);
		public static void Attempt(Action check);
		public static (Bytes, ulong) Tail(ISeekableReader file, ulong len);
		public static ulong SeekTo(ISeekableReader file, ulong at);
		public static ValueTask<uint?> Halved(uint n, CancellationToken cancellationToken = default);
		public static ValueTask<uint> Yielded(uint value, CancellationToken cancellationToken = default);
		public static ValueTask<uint> After(Func<CancellationToken, Task> first, uint value, CancellationToken cancellationToken = default);
		public static ValueTask Within(uint polls, Func<CancellationToken, Task> future, CancellationToken cancellationToken = default);
		public static Stamp Past(Stamp at, Stamp deadline);
		public static uint Odd(uint n);
		public static Moment MomentAt(ulong ticks, ulong rate);
		public static Stamp StampAt(ulong millis);
		public static Moment Shifted(Moment moment, Func<Moment, Moment> shift);
		public static uint JvmSum(JvmField field, JvmStatic method, OptIn opt);
		public static Clocked ClockedAfter(Clocked clocked, ulong ticks);
		public static uint Emit(Log log, ReadOnlySpan<byte> lines);
		public static uint Sensed(Sensor sensor);
		public static ValueTask<uint> Counted(uint n, Action<uint> count, CancellationToken cancellationToken = default);
		public static uint Filter(ReadOnlySpan<byte> lines, Func<byte, Str, bool> keep);
		public static ValueTask<bool> LogLater(Log log, CancellationToken cancellationToken = default);
		public static uint LogCounts(Log log, byte level, Counter counter, Counter[] more);
		public static bool OnFirstThread();
		public static uint LocalsDropped();
		public static Local LocalOrNew(Local? local);
		public static ValueTask<Local> LocalOrNewLater(Local? local, CancellationToken cancellationToken = default);
		public static uint BumpEither(Local? first, Local second);
		public static ValueTask<Local> LocalAfter(CancellationToken cancellationToken = default);
		public static ValueTask<uint> LocalLater(uint n, CancellationToken cancellationToken = default);
		public static ValueTask<uint> PollTripled(uint n, CancellationToken cancellationToken = default);
		public static ulong Watch(Gauge gauge, Func<Gauge, ulong> watch);
		public static Bytes TokenBytes(uint id, uint len);
		public static Bytes TokenSlice(uint id, uint len, uint of);
		public static Packet NextPacket(Packet packet);
		public static ArrayU8x4 Fingerprint(Str hex);
		public static uint Dropped();
		public static uint BumpBoth(Tally a, Tally b);
		public static uint Offset(Func<uint> by, uint byData);
		public static uint Shift(Func<uint> by, uint cBy);
		public static ulong Walk(ulong steps, Func<Cursor, ulong> visit);
		public static uint Each(uint steps, Action<uint> visit);
		public static uint Peek(Tally tally, Func<Tally, uint> peek);
		public static nuint Len(ReadOnlySpan<byte> payload);
		public static Str Hex(ReadOnlySpan<byte> payload);
		public static Str TimedHex(ReadOnlySpan<byte> payload);
		public static Str SampleHex(ReadOnlySpan<byte> sample);
		public static Str QueuedHex(ReadOnlySpan<byte> queued);
		public static Str TickName(Label tick);
		public static Str RootedName(Label rooted);
		public static Str StoredText(Str stored);
		public static Str LayerName(Label layer);
		public static Str Greet(Str name, Shape shape);
		public static Str Describe(Event @event);
		public static PairSizeU8 Tag(Size size, byte tag);
		public static byte Cross(LeftRightU8 left, RightLeftU8 right);
		public static ulong KeptPoint(Point from);
		public static ulong KeptCounter(Counter from);
		public static ulong KeptSize(Size from);
		public static ulong KeptWrapU32(WrapU32 from);
		public static ulong KeptPolarPoint(PolarPoint from);
		public static int KeepPoint(Point kept);
		public static int KeptOrPoint(Point? kept);
		public static int KeeperPoint(Point keeper);
		public static BoxedU16 BOXED { get; }
		public static BoxedU8 Keeps(Log log);
		public static uint WeighPoint(Point from);
		public static uint ApplyNonZeroU16(PairNonZeroU16 pair, Func<NonZeroU16, uint> f);
		public static uint ApplyU32(PairU32 pair, Func<uint, uint> f);
		public static uint ReportNonZeroU16(PairNonZeroU16 pair, Log log);
		public static uint ReportU32(PairU32 pair, Log log);
		public static uint TallyLogCount(Log log);
		public static uint Wind(PolarTurn whole, Polar_Turn part);
		public static ulong Area(Size size);
		public static ulong SeedsPointPoint(Point from);
		public static ulong SeedsPointCounter(Counter from);
		public static ulong SeedsPointSize(Size from);
		public static ulong SeedsPointWrapU32(WrapU32 from);
		public static ulong SeedsPointPolarPoint(PolarPoint from);
		public static ulong SeedsCounterPoint(Point from);
		public static ulong SeedsCounterCounter(Counter from);
		public static ulong SeedsCounterSize(Size from);
		public static ulong SeedsCounterWrapU32(WrapU32 from);
		public static ulong SeedsCounterPolarPoint(PolarPoint from);
		public static Area Grow(Area area, uint by);
		public static SlotU32 Wide(uint value);
		public static Str? Host(Url url);
		public static Url Resolve(Url @base, Str path);
		public static ushort[] Ports(SocketAddr[] addrs);
		public static Duration DoubledTtl(Duration ttl);
		public static PathBuf File(PathBuf dir, Str name);
		public static PathBuf Stray();
		public static void VisitStray(Action<PathBuf> visit);
		public static Scalar? NextChar(Scalar c);
		public static Duration? Extended(Duration duration, Duration by);
		public static PathBuf?[] Parents(PathBuf[] paths);
		public static Str[] Words(Str text);
		public static Str Sentence(Str[]? words);
		public static void PutNumber(uint value, ref Bytes buf);
		public static Duration SecondLater(Duration w235);
		public static byte Digit(ref Bytes r);
		public static uint PushCounted(ref (Bytes, uint) entry, byte @byte);
		public static void Nudge(ref Point point, int by);
		public static void NudgeBy(ref Point point, Refusal refusal);
		public static TlsConfig Secure(Str alpn);
		public static Str Alpn(TlsConfig config);
		public static Str AlpnLent(TlsConfig config);
		public static TlsCert Certify(ReadOnlySpan<byte> der);
		public static ValueTask<nuint> ByteLen(ReadOnlySpan<byte> bytes, CancellationToken cancellationToken = default);
		public static nuint CertLen(TlsCert cert);
		public static Bytes CertDer(TlsCert cert);
		public static nuint ChainLen(TlsCert[] chain);
		public static uint Verify(TlsCert cert);
		public static uint ParseCert(Str text);
		public static ValueTask<uint> ParseLater(Str text, CancellationToken cancellationToken = default);
		public static TlsResolver Resolver(Str name);
		public static Str Resolved(TlsResolver resolver);
		public static StdIoError Failure(Str message);
		public static Str FailureMessage(StdIoError error);
		public static Str FailureKind(StdIoError error);
		public static uint SumAll(uint[] items);
		public static ulong WeighAllCounter(Counter[] items);
		public static nuint OfferedLen(Bytes[] alpns);
		public static Str JoinWords(Str[] words);
		public static TlsSession Resume(Str alpn, uint seconds);
		public static nuint Either(Str? text);
		public static ValueTask<nuint> EitherLater(Str? text, CancellationToken cancellationToken = default);
		public static Bytes Prefixed(ReadOnlySpan<byte> payload, Bytes? prefix);
		public static int SumLists(ReadOnlySpan<int> a, int[]? b, int[]? c);
		public static int Coordinates(Point? point);
		public static Bytes[] Chunks(ReadOnlySpan<byte> bytes, nuint size);
		public static Counter[] Counters(Str[] names);
		public static ulong Worth(Counter[]? some, Counter[] boxed);
		public static ulong? HeldCount(ValueTuple<Counter>? held);
		public static Counter? BoxedCounter(Str? name);
		public static Str? BoxedName(Counter? boxed);
		public static uint Less(uint value, uint value2);
		public static uint[] Sums(uint[][] rows);
		public static uint[][] Least(uint[][] sets);
		public static (int[], uint) SplitLast(ReadOnlySpan<int> items);
		public static ulong SumU32(ReadOnlySpan<uint> items);
		public static long[] DoubledI64(ReadOnlySpan<long> items);
		public static ulong[] VisitChunks(ReadOnlySpan<byte> payload, nuint size, Func<Bytes?, ulong> visit);
		public static ulong Scanned(Scan scan, ReadOnlySpan<byte> payload);
		public static double[] BoxedFloats(ReadOnlySpan<double> items);
		public static Point Summed(Point[] points);
		public static Duration Breadth(LimitsDuration limits);
		public static LimitsBytes ByteLimits(ReadOnlySpan<byte> low, ReadOnlySpan<byte> high);
		public static Bytes Joined(Bytes[] parts);
		public static Str? FirstOf(Str[] items);
		public static ulong ReadLen(ReadOnlySpan<byte> reader);
		public static nuint Longest(Str[] words);
		public static Str Encode(Json value);
		public static Json Tagged(Json doc, Str tag);
		public static (Str, Json)[] Entries((Str, Json)[] entries, Str tag);
		public static (Str, Json)[] Sections((Str, Json)[] entries);
		public static Instant Now();
		public static Instant Later(Instant at, Duration by);
		public static Duration Since(Instant later, Instant earlier);
		public static bool HasInstant(Instant? at);
		public static Level? Louder(Level? filter);
		public static Str Spell(Level? filter);
		public static Str[] GREETINGS { get; }
		public static uint[] PRIMES { get; }
		public static Counter[] Numbered(Str[] names);
		public static (SocketAddrV4, SocketAddrV6) Nexts(SocketAddrV4 v4, SocketAddrV6 v6);
		public static (Ipv4Addr, Ipv6Addr) NextIps(Ipv4Addr v4, Ipv6Addr v6);
		public static IpAddr Flip(IpAddr ip);
		public static Str Reasons((Counter, Refusal)[] pairs, (Counter, Refusal)? last);
		public static nuint Excuse(ReadOnlySpan<byte> note, (Bytes, Refusal) entry);
		public static uint Relay(Log log, Refusal[] refusals);
		public static uint Refuse(Str reason);
		public static uint Jam(uint n);
		public static Platform Platform();
		public static uint PointerBits(Platform platform);
		public static Labelled SameLabelled(Labelled labelled);
		public static Placement Placement();
		public static bool IsFamily(Family family);
		public static uint WasmOnly();
		public static ulong WasmTake(Counter counter);
		public static void WasmLog(Log log);
		public static ulong BoothCount(Booth booth);
		public static Page Page();
		public static uint Beats(Beat beat);
		public static byte Paired(Func<byte> f, Func<byte> fUnwind);
		public static int Same(double a, double b);
		public static int Errno();
		public static bool Assert(bool holds);
		public static byte Offsetof(byte field);
		public static (Str, (byte, short, uint, ulong)?, Bytes[]) ReturnValues(ref Point point, bool fail);
		public static Counter ReturnMoved(Counter counter, bool fail);
		public static NonZeros NonZeros();
		public static Str DescribeNonZeros(NonZeros nonZeros);
		public static NonZeroU64? NextHop(NonZeroU64? hop);
		public static NonZeroU16 FartherHop(PairNonZeroU16 hops);
		public static int Wexitstatus(int status);
		public static Duration SameDuration(Duration v);
		public static ulong SameU64(ulong v);
		public static nuint SameUsize(nuint v);
		public static nint SameIsize(nint v);
		public static U128 SameU128(U128 v);
		public static Scalar SameChar(Scalar v);
		public static Str SameString(Str v);
		public static PathBuf SamePath(PathBuf v);
		public static SocketAddr SameSocketAddr(SocketAddr v);
		public static SocketAddrV4 SameSocketAddrV4(SocketAddrV4 v);
		public static SocketAddrV6 SameSocketAddrV6(SocketAddrV6 v);
		public static IpAddr SameIpAddr(IpAddr v);
		public static Str Unescaped(Str text);
		public static ValueTask<Str> UnescapedLater(Str text, CancellationToken cancellationToken = default);
		public static ValueTask<Str?> UnescapedMaybe(Str? text, CancellationToken cancellationToken = default);
		public static Bytes Unzeroed(ReadOnlySpan<byte> bytes);
		public static byte Deprecated_(byte note);
		public static byte Deprecated(byte note);
		public static byte All(byte note);
		public static byte All_(byte note);
		public static Knob Knob(Dial dial);
		public static ValueTask<ulong> LevelLater(Dial dial, CancellationToken cancellationToken = default);
		public static Dial Turned(Dial dial, Turner turner);
		public static uint Undone(Undo undo, uint n);
		public static uint Unwound();
		public static nuint Measured(Func<nuint> f);
		public static nint Negated(Func<nint> f);
		public static Scalar Lettered(Func<Scalar> f);
		public static Scalar Bumped(ref uint n, Func<Scalar> f);
		public static nuint Stretched(Stretch stretch);
		public static ulong Crowded(Reading reading, Stamp a, Stamp b, ulong n0, ulong n1, ulong n2, ulong n3, ulong n4, ulong n5, ulong n6, ulong n7, ulong n8, ulong n9, ulong n10, ulong n11, ulong n12, ulong n13, ulong n14, ulong n15, ulong n16, ulong n17, ulong n18, ulong n19, ulong n20, ulong n21, ulong n22, ulong n23, ulong n24, ulong n25, ulong n26, ulong n27, ulong n28, ulong n29, ulong n30, ulong n31, ulong n32, ulong n33, ulong n34, ulong n35, ulong n36, ulong n37, ulong n38, ulong n39, ulong n40, ulong n41, ulong n42, ulong n43, ulong n44, ulong n45, ulong n46, ulong n47, ulong n48, ulong n49, ulong n50, ulong n51, ulong n52, ulong n53, ulong n54, ulong n55, ulong n56, ulong n57, ulong n58, ulong n59, ulong n60, ulong n61, ulong n62, ulong n63, ulong n64, ulong n65, ulong n66, ulong n67, ulong n68, ulong n69, ulong n70, ulong n71, ulong n72, ulong n73, ulong n74, ulong n75, ulong n76, ulong n77, ulong n78, ulong n79, ulong n80, ulong n81, ulong n82, ulong n83, ulong n84, ulong n85, ulong n86, ulong n87, ulong n88, ulong n89, ulong n90, ulong n91, ulong n92, ulong n93, ulong n94, ulong n95, ulong n96, ulong n97, ulong n98, ulong n99, ulong n100, ulong n101, ulong n102, ulong n103, ulong n104, ulong n105, ulong n106, ulong n107, ulong n108, ulong n109, ulong n110, ulong n111, ulong n112, ulong n113, ulong n114, ulong n115, ulong n116, ulong n117, ulong n118);
		public static ulong Crammed(Moment a, Moment b, ulong n0, ulong n1, ulong n2, ulong n3, ulong n4, ulong n5, ulong n6, ulong n7, ulong n8, ulong n9, ulong n10, ulong n11, ulong n12, ulong n13, ulong n14, ulong n15, ulong n16, ulong n17, ulong n18, ulong n19, ulong n20, ulong n21, ulong n22, ulong n23, ulong n24, ulong n25, ulong n26, ulong n27, ulong n28, ulong n29, ulong n30, ulong n31, ulong n32, ulong n33, ulong n34, ulong n35, ulong n36, ulong n37, ulong n38, ulong n39, ulong n40, ulong n41, ulong n42, ulong n43, ulong n44, ulong n45, ulong n46, ulong n47, ulong n48, ulong n49, ulong n50, ulong n51, ulong n52, ulong n53, ulong n54, ulong n55, ulong n56, ulong n57, ulong n58, ulong n59, ulong n60, ulong n61, ulong n62, ulong n63, ulong n64, ulong n65, ulong n66, ulong n67, ulong n68, ulong n69, ulong n70, ulong n71, ulong n72, ulong n73, ulong n74, ulong n75, ulong n76, ulong n77, ulong n78, ulong n79, ulong n80, ulong n81, ulong n82, ulong n83, ulong n84, ulong n85, ulong n86, ulong n87, ulong n88, ulong n89, ulong n90, ulong n91, ulong n92, ulong n93, ulong n94, ulong n95, ulong n96, ulong n97, ulong n98, ulong n99, ulong n100, ulong n101, ulong n102, ulong n103, ulong n104, ulong n105, ulong n106, ulong n107, ulong n108, ulong n109, ulong n110, ulong n111, ulong n112, ulong n113, ulong n114, ulong n115, ulong n116, ulong n117, ulong n118, ulong n119, ulong n120, ulong n121, ulong n122, ulong n123);
		public static uint JniOne();
		public static PolarPoint? PolarPoint(uint distance);
		public static uint MacrosStreamed(MacrosStreams streams, byte errno);
		public static ClashAny ClashAny(Str typeUrl);
		public static ClashCandidate ClashCandidate(byte at);
		public static byte ClashIgnore(byte arg0, byte arg1, byte arg2);
		public static ClashEmptyString ClashEmpty();
		public static uint ClashDescribed(ClashDescribed described);
		public static byte ClashWaited(ClashSource source);
		public static uint ClashUnderscored(ClashUnderscored underscored, ClashOp op, ClashChange change);
		public static (ClashUnderscored, ClashChange) ClashUnderscoredNext(ClashUnderscored underscored, ClashChange change);
		public static uint ClashErased(ClashEraser eraser);
		public static ClashNotch[] ClashNotches(uint n);
		public static byte ClashShifted(byte at, ClashShift shift);
		public static uint ClashCounted(ClashCount count);
		public static byte ClashErase_();
		public static byte ClashErase();
		public static uint ClashLend_(ClashStore? store);
		public static uint ClashLend();
		public static uint ClashRun(Func<byte> work2, byte work);
		public static ClashTone ClashAudible(ClashTone tone);
		public static uint ClashNested(((byte, ushort), uint)? a, ValueTuple<(byte, ushort, uint)>? b);
		public static byte ClashAudioEncoderFinish();
		public static byte ClashAudioEncoderFinish_();
		public static uint EmitFromThread(Log log, ReadOnlySpan<byte> lines);
		public static ValueTask<uint> EmitLater(Log log, ReadOnlySpan<byte> lines, CancellationToken cancellationToken = default);
		public static ValueTask LogOnDrop(Log log, CancellationToken cancellationToken = default);
		public static Waker LogWaker(Log log);
		public static void Hook(Action f);
		public static void Unhook();
	}
}
