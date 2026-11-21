// api.dart
extension type const U64.fromBits(int bits) {
	U64(int value);
	factory U64.fromBigInt(BigInt value);
	static const max = U64.fromBits(-1);
	int toInt();
	BigInt toBigInt();
	String toRadixString(int radix);
	int compareTo(U64 other);
	bool operator <(U64 other);
	bool operator <=(U64 other);
	bool operator >(U64 other);
	bool operator >=(U64 other);
}
final class RustDuration implements Comparable<RustDuration> {
	final U64 secs;
	final int nanos;
	RustDuration(this.secs, [this.nanos = 0]);
	static const zero = RustDuration._(U64.fromBits(0), 0);
	factory RustDuration.from(Duration duration);
	Duration toDuration();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override int compareTo(RustDuration other);
	@override String toString();
}
final class PathBuf {
	PathBuf(String path);
	PathBuf.fromBytes(List<int> bytes);
	typed.Uint8List get bytes;
	String toFilePath();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
sealed class IpAddr {
	factory IpAddr.from(io.InternetAddress address);
	typed.Uint8List get octets;
	io.InternetAddress toInternetAddress();
	@override String toString();
}
final class Ipv4Addr extends IpAddr {
	factory Ipv4Addr.fromOctets(List<int> octets);
	factory Ipv4Addr.from(io.InternetAddress address);
	@override typed.Uint8List get octets;
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Ipv6Addr extends IpAddr {
	factory Ipv6Addr.fromOctets(List<int> octets);
	factory Ipv6Addr.from(io.InternetAddress address);
	@override typed.Uint8List get octets;
	@override bool operator ==(Object other);
	@override int get hashCode;
}
sealed class SocketAddr {
	factory SocketAddr(IpAddr ip, int port);
	IpAddr get ip;
	int get port;
}
final class SocketAddrV4 extends SocketAddr {
	@override final Ipv4Addr ip;
	@override final int port;
	SocketAddrV4(this.ip, this.port);
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class SocketAddrV6 extends SocketAddr {
	@override final Ipv6Addr ip;
	@override final int port;
	final int flowinfo;
	final int scopeId;
	SocketAddrV6(this.ip, this.port, {this.flowinfo = 0, this.scopeId = 0});
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class Mut<T> {
	T value;
	Mut(this.value);
}
final class Cancelled implements Exception {
	const Cancelled();
	@override String toString();
}
extension type Cancelable<T>._(Future<T> _future) implements Future<T> {
	void cancel();
}
enum SeekFrom {
	start,
	current,
	end,
}
abstract interface class SeekableReader {
	factory SeekableReader.file(io.RandomAccessFile file);
	int readInto(typed.Uint8List buffer);
	int seek(int offset, SeekFrom from);
}
final class Waker extends _Object {
	void call();
}
sealed class BoundU64 {
	const BoundU64();
}
final class BoundU64Unbounded extends BoundU64 {
	const BoundU64Unbounded();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BoundU64Included extends BoundU64 {
	final U64 value;
	const BoundU64Included(this.value);
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BoundU64Excluded extends BoundU64 {
	final U64 value;
	const BoundU64Excluded(this.value);
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BoundsU64 {
	final BoundU64 start;
	final BoundU64 end;
	const BoundsU64(this.start, this.end);
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class RangeU64 {
	final U64 start;
	final U64 end;
	const RangeU64(this.start, this.end);
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class ArrayU8x4 {
	ArrayU8x4(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class Parcel {
	final Mark mark;
	final typed.Uint8List payload;
	const Parcel({required this.mark, required this.payload});
}
final class Weighted {
	final Score score;
	final int weight;
	const Weighted({required this.score, required this.weight});
	U64 total();
}
final class Stamped {
	final Stamp at;
	final int count;
	const Stamped({required this.at, required this.count});
	U64 total();
}
final class CancellationError {
	final int code;
	const CancellationError({required this.code});
}
final class Packet {
	final int seq;
	final typed.Uint8List payload;
	const Packet({required this.seq, required this.payload});
}
final class Hasher {
	final int seed;
	const Hasher({required this.seed});
}
final class Size {
	final int width;
	final int height;
	const Size({required this.width, required this.height});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Point {
	final int x;
	final int y;
	const Point({required this.x, required this.y});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Point.default_();
	Point offset(int dx, int dy);
	static int getX();
}
final class Label {
	final String name;
	final Point? at;
	final RustDuration? ttl;
	const Label({required this.name, required this.at, required this.ttl});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Note {
	final String? text;
	const Note({required this.text});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Note.default_();
}
final class Reading {
	final bool ok;
	final double level;
	final double ratio;
	final String mark;
	final Shape shape;
	final Point at;
	final String note;
	final U64 count;
	final int offset;
	final U64 len;
	final int tag;
	const Reading({required this.ok, required this.level, required this.ratio, required this.mark, required this.shape, required this.at, required this.note, required this.count, required this.offset, required this.len, required this.tag});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Reading? next();
}
final class Packaged {
	final int api;
	const Packaged({required this.api});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Point point();
	Shape shape();
}
final class Atlas {
	final List<(String, Point)> points;
	final List<int> ids;
	const Atlas({required this.points, required this.ids});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Atlas with_(String name, Point at);
}
final class Span {
	final (int, int) bounds;
	final (String, Shape)? named;
	const Span({required this.bounds, required this.named});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Window {
	final RangeU64 range;
	final BoundU64 end;
	const Window({required this.range, required this.end});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	bool contains(U64 n);
}
final class Ticket {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Ticket(int number);
	int number();
	void wait();
	static int clones();
}
final class Seat {
	final Mark mark;
	final int row;
	const Seat({required this.mark, required this.row});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 count();
}
final class Stamp {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Stamp.fromMillis(U64 millis);
	U64 secs();
	Stamp whole();
	bool later(Stamp other);
}
final class Moment {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Moment? new_(U64 ticks, U64 rate);
	U64 millis();
	Moment after(U64 ticks);
}
final class LongArray {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory LongArray(U64 low, U64 high);
	U64 c();
	void also(void Function(U64) f);
}
final class JvmField {
	final int n;
	const JvmField({required this.n});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class JvmStatic {
	final int n;
	const JvmStatic({required this.n});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class OptIn {
	final int n;
	const OptIn({required this.n});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Wide {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Wide(U64 seed);
	U64 sum();
}
final class Clocked {
	final Moment at;
	final int count;
	const Clocked({required this.at, required this.count});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Rated {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Rated(U64 count, U64 scale);
	U64 scale();
}
final class Score {
	String debug();
	factory Score.of(U64 count);
	U64 count();
	Score add(Score other);
	Score doubled();
	(Score, U64) take();
	(Score, U64) note(Mark mark);
	(Score, U64, U64) absorb(Mut<Score> other);
}
final class Hits {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Hits.default_();
	U64 get hitsGet;
	Hits hitsSet(U64 value);
	U64 get missesGet;
	Hits missesSet(U64 value);
	factory Hits.of(U64 hits, U64 misses);
	Hits add(Hits other);
	(Hits, bool) eq(Hits other);
}
final class Regs {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int get fcwGet;
	Regs fcwSet(int value);
	U64 get wideGet;
	Regs wideSet(U64 value);
	Point get atGet;
	Regs atSet(Point value);
	int get tagGet;
	Regs tagSet(int value);
	factory Regs(int fcw, U64 wide, Point at);
	int seed();
}
final class Extent {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get startGet;
	Extent startSet(U64 value);
	factory Extent(U64 start, U64 len);
	U64 end();
	(Extent, U64) slide(U64 by);
}
final class PairSizeU8 {
	final Size left;
	final int right;
	const PairSizeU8({required this.left, required this.right});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PairU8Size swap();
}
final class PairNonZeroU16 {
	final int left;
	final int right;
	const PairNonZeroU16({required this.left, required this.right});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PairNonZeroU16 swap();
	bool even();
}
final class PairU32 {
	final int left;
	final int right;
	const PairU32({required this.left, required this.right});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PairU32 swap();
	bool even();
	int sum();
	PairU8Size besideSize(Size right);
}
final class PairU8Size {
	final int left;
	final Size right;
	const PairU8Size({required this.left, required this.right});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PairSizeU8 swap();
}
final class LeftRightU8 {
	final RightU8 value;
	const LeftRightU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class LeftU8 {
	final int value;
	const LeftU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class RightLeftU8 {
	final LeftU8 value;
	const RightLeftU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class RightU8 {
	final int value;
	const RightU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class InnerU8 {
	final int value;
	const InnerU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BoxedU16 {
	final int value;
	const BoxedU16({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BoxedU8 {
	final int value;
	const BoxedU8({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class LogCount {
	final int kept;
	const LogCount({required this.kept});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class WrapU32 {
	final int value;
	const WrapU32({required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class PolarTurn {
	final int whole;
	const PolarTurn({required this.whole});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Peer {
	final SocketAddr addr;
	final List<String> names;
	const Peer({required this.addr, required this.names});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	List<String> aliases();
	Peer next(String name);
}
final class Shadowed {
	final typed.Int32List other;
	final int a;
	final int b;
	const Shadowed({required this.other, required this.a, required this.b});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Numbers {
	final typed.Uint16List u16s;
	final typed.Uint32List u32s;
	final List<U64> u64s;
	final typed.Int8List i8s;
	final typed.Int16List i16s;
	final typed.Int32List i32s;
	final typed.Int64List i64s;
	final typed.Float32List f32s;
	final typed.Float64List f64s;
	final typed.Int32List? some;
	const Numbers({required this.u16s, required this.u32s, required this.u64s, required this.i8s, required this.i16s, required this.i32s, required this.i64s, required this.f32s, required this.f64s, required this.some});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Numbers.edges();
	factory Numbers.zero(bool negative);
	Numbers reversed();
}
final class Measure {
	final double value;
	final Numbers numbers;
	const Measure({required this.value, required this.numbers});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Measure.new_(double value, bool negative);
}
final class LimitsBytes {
	final typed.Uint8List low;
	final typed.Uint8List high;
	const LimitsBytes({required this.low, required this.high});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class LimitsDuration {
	final RustDuration low;
	final RustDuration high;
	const LimitsDuration({required this.low, required this.high});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Page {
	final int size;
	const Page({required this.size});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Page.default_();
}
final class Booth {
	final Mark mark;
	const Booth({required this.mark});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Cancelled2 {
	final int step;
	const Cancelled2({required this.step});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class StartSeekable {
	final U64 at;
	const StartSeekable({required this.at});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class DoubleArray {
	final int len;
	const DoubleArray({required this.len});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class NonZeros {
	final int u8;
	final int u16;
	final int u32;
	final U64 u64;
	final U64 usize;
	final int i8;
	final int i16;
	final int i32;
	final int i64;
	final int isize;
	const NonZeros({required this.u8, required this.u16, required this.u32, required this.u64, required this.usize, required this.i8, required this.i16, required this.i32, required this.i64, required this.isize});
	String debug();
}
final class A {
	final int b;
	const A({required this.b});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Qualifiers {
	final double api;
	final double kotlin;
	const Qualifiers({required this.api, required this.kotlin});
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Version implements Comparable<Version> {
	final int major;
	final int minor;
	const Version({required this.major, required this.minor});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Version other);
	bool operator <(Version other);
	bool operator <=(Version other);
	bool operator >(Version other);
	bool operator >=(Version other);
	@override int compareTo(Version other);
}
final class Spot {
	final int x;
	final int y;
	const Spot({required this.x, required this.y});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Spot.deserialize(Json deserializer);
}
final class Deprecated2 {
	final bool on;
	@Deprecated('') final int lower;
	final bool deprecated;
	const Deprecated2({required this.on, required this.lower, required this.deprecated});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Deprecated2.default_();
}
final class Dial {
	final U64 level;
	@Deprecated('keep the `Dial` from before the turn') final U64 before;
	const Dial({required this.level, required this.before});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Dial.default_();
	Dial turn(U64 by);
	@Deprecated('turn it by `LIMIT` with `Dial::turn`') Dial max();
}
@Deprecated('use `Dial`, whose \'level\' is "the same" at \$0\nand past it') final class Knob {
	final U64 level;
	const Knob({required this.level});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Knob.default_();
	U64 doubled();
}
final class PolarPoint {
	final int turns;
	final int distance;
	const PolarPoint({required this.turns, required this.distance});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class PolarTurn2 {
	final int part;
	const PolarTurn2({required this.part});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MacrosStreams {
	final int stdin;
	final int stdout;
	final int stderr;
	final int errno;
	const MacrosStreams({required this.stdin, required this.stdout, required this.stderr, required this.errno});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashTone {
	final int hz;
	const ClashTone({required this.hz});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashTone2 {
	final int db;
	const ClashTone2({required this.db});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashSpan {
	final int width;
	const ClashSpan({required this.width});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int width_();
	String toString_();
}
final class ClashAny {
	final String typeUrl;
	const ClashAny({required this.typeUrl});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashBreak {
	final int $0;
	const ClashBreak(this.$0);
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static ClashBreak get sp_;
	static ClashBreak get sp;
}
final class ClashMalformed implements Exception {
	final int at;
	const ClashMalformed({required this.at});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	bool is_(int at);
}
final class ClashDescribed {
	final int description;
	final int debugDescription;
	const ClashDescribed({required this.description, required this.debugDescription});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashErrno {
	final int errno;
	const ClashErrno({required this.errno});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static ClashErrno get null_;
}
final class ClashEscaped {
	final int new_;
	const ClashEscaped({required this.new_});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int delete_(int new_2, int new_);
	int delete();
}
final class ClashUnderscored {
	final int new_;
	final int new2_;
	final int debug_;
	final int default_;
	const ClashUnderscored({required this.new_, required this.new2_, required this.debug_, required this.default_});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory ClashUnderscored.default_();
}
final class TlsPolicyLifetime {
	final int seconds;
	const TlsPolicyLifetime({required this.seconds});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Core {
	final int id;
	const Core({required this.id});
}
sealed class Choice {
	const Choice();
}
final class ChoiceEmpty extends Choice {
	const ChoiceEmpty();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ChoicePicked extends Choice {
	final Mark $0;
	const ChoicePicked(this.$0);
}
sealed class Array {
	const Array();
}
final class ArrayU8x42 extends Array {
	final int $0;
	const ArrayU8x42(this.$0);
}
sealed class Platform {
	const Platform();
}
final class PlatformWasm extends Platform {
	final int bits;
	const PlatformWasm({required this.bits});
}
final class PlatformNative extends Platform {
	final int $0;
	const PlatformNative(this.$0);
}
sealed class Placement {
	const Placement();
}
final class PlacementAnywhere extends Placement {
	const PlacementAnywhere();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class PlacementPinned extends Placement {
	final Core $0;
	const PlacementPinned(this.$0);
}
enum Shape {
	circle(1),
	square(4),
	const Shape(this._c);
	String debug();
	static Shape? valueOf(String name);
	static List<Shape> get all;
	static List<Shape> get both;
}
sealed class Event {
	const Event();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class EventMoved extends Event {
	final Point $0;
	const EventMoved(this.$0);
}
final class EventNamed extends Event {
	final String name;
	const EventNamed({required this.name});
}
final class EventDone extends Event {
	const EventDone();
}
sealed class Clash {
	const Clash();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Point? point();
	factory Clash.of(double value);
	typed.Uint8List bytes();
	Reading reading(int kotlin);
	static List<typed.Uint32List> listed(typed.Uint32List java, Mut<int> kotlin);
	static Cancelable<Clash> after(Future<void> first, double kotlin);
	typed.Uint8List joined(typed.Uint8List lent, typed.Uint8List owned, typed.Uint8List? maybe);
	Cancelable<String> tagged(String tag);
	Stamp stamp(U64 kotlin);
	Moment moment(U64 kotlin);
	factory Clash.at(Point api);
	String describe(String text, int n, Shape shape, Mut<int> out);
}
final class ClashPoint extends Clash {
	final Point $0;
	const ClashPoint(this.$0);
}
final class ClashShape extends Clash {
	final Shape $0;
	const ClashShape(this.$0);
}
final class ClashEvent extends Clash {
	final Event $0;
	const ClashEvent(this.$0);
}
final class ClashDouble extends Clash {
	final double $0;
	const ClashDouble(this.$0);
}
final class ClashFloat extends Clash {
	final double $0;
	const ClashFloat(this.$0);
}
final class ClashString extends Clash {
	final String $0;
	const ClashString(this.$0);
}
final class ClashInt extends Clash {
	final int $0;
	const ClashInt(this.$0);
}
final class ClashBoolean extends Clash {
	final bool $0;
	const ClashBoolean(this.$0);
}
final class ClashNative extends Clash {
	final int $0;
	const ClashNative(this.$0);
}
final class ClashBytes extends Clash {
	final typed.Uint8List $0;
	const ClashBytes(this.$0);
}
final class ClashAny2 extends Clash {
	const ClashAny2();
}
final class ClashUnit extends Clash {
	const ClashUnit();
}
final class ClashWaker extends Clash {
	const ClashWaker();
}
final class ClashMut extends Clash {
	const ClashMut();
}
final class ClashPair extends Clash {
	final int $0;
	final int $1;
	const ClashPair(this.$0, this.$1);
}
final class ClashStamp extends Clash {
	final int $0;
	const ClashStamp(this.$0);
}
final class ClashMoment extends Clash {
	final Moment $0;
	const ClashMoment(this.$0);
}
sealed class Stage {
	const Stage();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class StageInit extends Stage {
	final int $0;
	const StageInit(this.$0);
}
final class StageDone extends Stage {
	const StageDone();
}
sealed class Error implements Exception {
	const Error();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ErrorUnderflow extends Error {
	final U64 by;
	const ErrorUnderflow({required this.by});
}
final class ErrorOverflow extends Error {
	const ErrorOverflow();
}
enum Status {
	moved(0),
	kept(1),
	const Status(this._c);
	String debug();
}
sealed class Labelled {
	const Labelled();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class LabelledTagged extends Labelled {
	final int tag;
	const LabelledTagged({required this.tag});
}
final class LabelledBare extends Labelled {
	const LabelledBare();
}
enum Family {
	native(0, 0x3, 'api_Family.Native'),
	wasm(1, 0x1, 'api_Family.Wasm'),
	const Family(this._value, this._targets, this._what);
	String debug();
}
enum VecClashTone {
	error(0),
	const VecClashTone(this._c);
	String debug();
}
sealed class B {
	const B();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BOne extends B {
	const BOne();
}
final class BTwo extends B {
	final int $0;
	const BTwo(this.$0);
}
sealed class Builtin {
	const Builtin();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class BuiltinAny extends Builtin {
	final typed.Int32List $0;
	const BuiltinAny(this.$0);
}
final class BuiltinBoolean extends Builtin {
	final double $0;
	const BuiltinBoolean(this.$0);
}
final class BuiltinInt extends Builtin {
	final double $0;
	const BuiltinInt(this.$0);
}
final class BuiltinNative extends Builtin {
	final double $0;
	const BuiltinNative(this.$0);
}
sealed class Rank implements Comparable<Rank> {
	const Rank();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Rank other);
	bool operator <(Rank other);
	bool operator <=(Rank other);
	bool operator >(Rank other);
	bool operator >=(Rank other);
	@override int compareTo(Rank other);
}
final class RankUnranked extends Rank {
	const RankUnranked();
}
final class RankRanked extends Rank {
	final int compare;
	final int partialCompare;
	final int compareTo_;
	final int partialCompareTo_;
	const RankRanked({required this.compare, required this.partialCompare, required this.compareTo_, required this.partialCompareTo_});
}
enum Tier implements Comparable<Tier> {
	low(0),
	high(1),
	compareTo_(2),
	partialCompareTo_(3),
	const Tier(this._c);
	String debug();
	int? partialCompareTo(Tier other);
	bool operator <(Tier other);
	bool operator <=(Tier other);
	bool operator >(Tier other);
	bool operator >=(Tier other);
	@override int compareTo(Tier other);
}
sealed class ClashCandidate {
	const ClashCandidate();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashCandidateNone extends ClashCandidate {
	const ClashCandidateNone();
}
final class ClashCandidateMatch extends ClashCandidate {
	final int $0;
	const ClashCandidateMatch(this.$0);
}
enum ClashLevel {
	default__(0),
	debug_(1),
	const ClashLevel(this._c);
	String debug();
	factory ClashLevel.default_();
	int index_();
}
sealed class ClashTag {
	const ClashTag();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int number_();
}
final class ClashTagApplication extends ClashTag {
	final int number;
	const ClashTagApplication({required this.number});
}
final class ClashTagPrivate extends ClashTag {
	final int number;
	const ClashTagPrivate({required this.number});
}
enum ClashOp {
	delete_(0),
	delete(1),
	const ClashOp(this._c);
	String debug();
}
sealed class ClashChange {
	const ClashChange();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashChangeSet extends ClashChange {
	final int new_;
	final int new2_;
	const ClashChangeSet({required this.new_, required this.new2_});
}
final class ClashChangeClear extends ClashChange {
	const ClashChangeClear();
}
sealed class ClashShift {
	const ClashShift();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClashShiftUp extends ClashShift {
	final int $0;
	const ClashShiftUp(this.$0);
}
final class ClashShiftDown extends ClashShift {
	const ClashShiftDown();
}
enum Level implements Comparable<Level> {
	error(0),
	warn(1),
	info(2),
	debug(3),
	trace(4),
	const Level(this._c);
	@override String toString();
	int? partialCompareTo(Level other);
	bool operator <(Level other);
	bool operator <=(Level other);
	bool operator >(Level other);
	bool operator >=(Level other);
	@override int compareTo(Level other);
}
final class Board extends _Object {
	factory Board(List<(String, Counter)> counters);
	List<(String, Counter)> intoCounters();
	List<(String, Counter)> ranked();
	Counter best();
}
final class Tab extends _Object {
	Ledger get ledgerGet;
	String get labelGet;
}
final class Receipt extends _Object {
	Receipt clone();
	int count();
}
final class Draft extends _Object {
	bool edited();
	String describe();
	int commit();
	String get titleGet;
	void titleSet(String value);
	int lines();
	void write(String line);
}
final class FiledReceipt extends _Object {
	FiledReceipt clone();
	int count();
}
final class PadU16 extends _Object {
	factory PadU16.wide();
	int tag();
	int title();
	String get titleGet;
	int lines();
	String describe();
}
final class PadU8 extends _Object {
	factory PadU8.small();
	String describe();
	String get titleGet;
	int lines();
}
final class Layered extends _Object {
	factory Layered();
	String get titleGet;
	int lines();
}
final class Marker extends _Object {
	factory Marker(String label, int x, int y);
	String label();
	int x();
	static int y();
	int get xGet;
	void xSet(int value);
	int get yGet;
	void ySet(int value);
	Point offset(int dx, int dy);
}
final class Batch extends _Object {
	factory Batch();
	bool push(int v);
	U64 capacity();
}
final class Batch2 extends _Object {
	factory Batch2();
	bool push(int v);
	U64 capacity();
}
final class Waiter extends _Object {
	factory Waiter(void Function() waker);
	Waker waker();
	Waker context();
	static Cancelable<void> pollFuture(Future<void> future);
}
final class Feed extends _Object {
	Cancelable<typed.Uint8List?> next();
	dasync.Stream<typed.Uint8List> asStream();
}
final class Countdown extends _Object {
	factory Countdown(int from);
	factory Countdown.told(int from, Log log);
	Cancelable<int?> next();
	dasync.Stream<int> asStream();
}
final class Retired extends _Object {
	factory Retired(int from);
	@Deprecated('count with `Countdown`') Cancelable<int?> next();
	@Deprecated('count with `Countdown`') dasync.Stream<int> asStream();
}
final class Folio extends _Object {
	int get pageGet;
	void pageSet(int value);
	String get titleGet;
	void titleSet(String value);
	String? get noteGet;
	void noteSet(String? value);
	Hasp get haspGet;
	void haspSet(Hasp value);
	Hasp get dockGet;
	factory Folio(int page, String title);
	int seed();
}
final class Logger extends _Object {
	factory Logger(Log sink);
	bool write(String line);
	bool writeFromThread(String line);
}
final class Journal extends _Object {
	factory Journal(Log sink);
	void redirect(Log sink);
	bool write(String line);
}
final class Local extends _Object {
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Local.default_();
	factory Local();
	int bump();
	Local share();
	Cancelable<int> later();
	int visit(int Function() f);
	Cancelable<int> pollCount();
	U64 threadNumber();
	bool onOwnThread();
}
final class Wrapped extends _Object {
	factory Wrapped(U64 n);
	U64 get nGet;
	void nSet(U64 value);
	bool get onGet;
	void onSet(bool value);
	U64 count();
	void recount(U64 n);
	U64 steps();
	int locals();
}
final class Buffer extends _Object {
	factory Buffer(int len);
	void blank();
	bool zeroed();
}
final class LocalWaiter extends _Object {
	factory LocalWaiter(void Function() waker);
	static Cancelable<int> pollReady(int n);
}
final class Retry extends _Object {
	int? get limitGet;
	void limitSet(int? value);
	int? get delayGet;
	void delaySet(int? value);
	factory Retry(int? limit);
	int limit();
	int delay();
	void setLimit(int limit);
}
final class Digest extends _Object {
	ArrayU8x4 get sumGet;
	void sumSet(ArrayU8x4 value);
	factory Digest();
	ArrayU8x4 flipped(ArrayU8x4 mask);
}
final class Cursor extends _Object {
	U64 at();
}
final class OuterInnerU8 extends _Object {
	factory OuterInnerU8(int value);
	int get();
}
final class SlotU32 extends _Object {
	int get valueGet;
	void valueSet(int value);
	factory SlotU32(int value);
}
final class SlotU8 extends _Object {
	int get valueGet;
	void valueSet(int value);
	factory SlotU8(int value);
	int value();
}
final class Resolving extends _Object {
	TlsResolver get $0Get;
	void $0Set(TlsResolver value);
	factory Resolving(String name);
}
final class Codecs extends _Object {
	factory Codecs.default_();
	List<TlsCodec> get listGet;
	void listSet(List<TlsCodec> value);
	factory Codecs();
	int ids();
}
final class Der extends _Object {
	factory Der.fromSlice(typed.Uint8List bytes);
	typed.Uint8List bytes();
}
final class Verbosity extends _Object {
	Level get levelGet;
	void levelSet(Level value);
	factory Verbosity(Level level);
	Level? filter();
}
final class Site extends _Object {
	Url get homeGet;
	void homeSet(Url value);
	factory Site(Url home);
}
final class Trail extends _Object {
	factory Trail(List<Point> points, List<String> names);
	List<Point> points();
	List<Label> labels();
	List<Point> iter();
	List<String> names();
	List<Point> beyond(int x);
	typed.Uint8List bytes();
	List<Label> drain();
}
final class Worker extends _Object {
	factory Worker(int id);
	int id();
}
final class Handle extends _Object {
	factory Handle(int count);
	int count();
	static Cancelable<Handle> later(int count);
}
final class Holder extends _Object {
	factory Holder(U64 level);
	@Deprecated('use `Dial`, whose \'level\' is "the same" at \$0\nand past it') U64 get levelGet;
	@Deprecated('use `Dial`, whose \'level\' is "the same" at \$0\nand past it') U64 doubled();
}
final class Tag extends _Object {
	factory Tag.default_();
	factory Tag();
	String tagged(Mut<int> n, String Function() f);
}
final class Area extends _Object {
	String debug();
	Area clone();
	factory Area(Size size);
	Size size();
	U64 pixels();
	U64 scaledPoint(Point by);
}
final class Counter extends _Object {
	String debug();
	Counter clone();
	factory Counter.default_();
	factory Counter(String name, U64 start);
	U64 get();
	U64 offset(U64 handle, U64 h);
	U64 intoSum(U64 close);
	String name();
	U64 add(U64 n);
	U64 sub(U64 n);
	bool edited();
	U64 plus(Counter? other);
	U64 clamped(Counter? low, Counter? high);
	Cancelable<U64> plusLater(Counter? other);
	bool addTo(Counter? other);
	Label intoLabel(Point? at);
	void rename(String name);
	(Counter, Counter) split(U64 at);
	bool is_(String name);
	factory Counter.seededPoint(String name, Point from);
	factory Counter.seededCounter(String name, Counter from);
	factory Counter.seededSize(String name, Size from);
	factory Counter.seededWrapU32(String name, WrapU32 from);
	factory Counter.seededPolarPoint(String name, PolarPoint from);
}
final class Meter extends _Object {
	factory Meter.default_();
	U64 add(U64 n);
	U64 count();
	void reset();
}
final class Meters extends _Object {
	factory Meters.default_();
	Meter get lastGet;
	void lastSet(Meter value);
	Meter? get spareGet;
	void spareSet(Meter? value);
	factory Meters();
	Meter first();
	void clear();
}
final class Registry extends _Object {
	factory Registry.default_();
	Counter get firstGet;
	void firstSet(Counter value);
	factory Registry(Counter first);
	Counter? last();
	void push(Counter counter);
	bool edit(bool Function(Counter) edit);
}
final class Ledger extends _Object {
	factory Ledger.default_();
	factory Ledger();
	Draft edit();
	int published();
	String title();
	void seal();
	Receipt receipt();
	Receipt noted();
	FiledReceipt fileReceipt(Receipt item);
	Tab tab();
	Slip slip();
}
final class Slip extends _Object {
	Slip clone();
	Receipt get receiptGet;
	void receiptSet(Receipt value);
	Receipt get globalGet;
	void globalSet(Receipt value);
	int count();
}
final class Sheet extends _Object {
	String debug();
	Sheet clone();
	factory Sheet.default_();
	String get titleGet;
	void titleSet(String value);
	factory Sheet();
	int lines();
	void write(String line);
	String describe();
}
final class Name extends _Object {
	String debug();
	Name clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Name.owned(String text);
	String asStr();
	Name join(Name other);
	factory Name.borrowed(String text);
	bool shares(Name other);
	void rename(String text);
}
final class Channel extends _Object {
	Channel clone();
	factory Channel.default_();
	factory Channel();
	void clear();
	void send(typed.Uint8List payload);
	Cancelable<typed.Uint8List> recv();
	void notify(void Function() waker);
	Cancelable<void> sendLater(typed.Uint8List payload);
	Cancelable<String> tagged(String tag);
	Cancelable<U64> itemized(typed.Uint32List? items);
	Cancelable<typed.Uint8List> pollTake();
	Cancelable<U64> pollMeasure(U64 extra);
	void listen(void Function() cx);
	Cancelable<(typed.Uint8List, U64)> recvSized();
	Cancelable<typed.Uint8List> recvNonempty();
	Feed feed();
	void sendFromThread(typed.Uint8List payload);
}
final class Gap extends _Object {
	String debug();
	Gap clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Gap(U64 wide, int narrow);
	U64 sum();
}
final class Berth extends _Object {
	String debug();
	Berth clone();
	factory Berth(int? at);
	int? at();
}
final class Mark extends _Object {
	String debug();
	Mark clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mark.of(U64 count);
	U64 count();
	void add(U64 by);
}
final class Hasp extends _Object {
	factory Hasp.default_();
	bool set();
}
final class Message extends _Object {
	String debug();
	Message clone();
	Stamp? get stampGet;
	void stampSet(Stamp? value);
	typed.Uint8List get payloadGet;
	void payloadSet(typed.Uint8List value);
	factory Message(Stamp? stamp, typed.Uint8List payload);
	bool sent();
}
final class Node extends _Object {
	String debug();
	U64 get valueGet;
	void valueSet(U64 value);
	Node? get nextGet;
	void nextSet(Node? value);
	factory Node(U64 value, Node? next);
	void push(U64 value);
	U64 sum();
}
final class Tally extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Tally.default_();
	factory Tally();
	int bump();
	Cancelable<int> bumpLater();
}
final class Tallies extends _Object {
	factory Tallies.default_();
	factory Tallies();
	Tally first();
}
final class Plain extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get nGet;
	void nSet(U64 value);
	bool get onGet;
	void onSet(bool value);
	factory Plain(U64 n, bool on);
	U64 count();
	void recount(U64 n);
	U64 steps();
	int locals();
}
final class Labeled extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Labeled(String label);
}
final class Gauge extends _Object {
	Gauge clone();
	factory Gauge(U64 level);
	void set(U64 level);
	U64 level();
	U64 intoLevel();
	Cancelable<U64> later();
	Cancelable<bool> reaches(U64 this_);
}
final class Latch extends _Object {
	Latch clone();
	factory Latch.default_();
	bool set(int value);
	int? get();
}
final class Token extends _Object {
	Token clone();
	int get idGet;
	void idSet(int value);
	factory Token(int id);
	List<Token> twice();
}
final class Stack extends _Object {
	String debug();
	factory Stack.default_();
	factory Stack();
	void push(Counter item);
	Counter? pop();
	U64 depth();
	U64 depthPastPoint(Point from);
	U64 depthPastCounter(Counter from);
	U64 depthPastSize(Size from);
	U64 depthPastWrapU32(WrapU32 from);
	U64 depthPastPolarPoint(PolarPoint from);
}
final class Refusal extends _Object implements Exception {
	@override String toString();
	String debug();
}
final class Fault extends _Object implements Exception {
	@override String toString();
	String debug();
}
final class C extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory C(int value);
}
final class Quantity extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Quantity other);
	bool operator <(Quantity other);
	bool operator <=(Quantity other);
	bool operator >(Quantity other);
	bool operator >=(Quantity other);
	factory Quantity(int value, String unit);
}
final class Pass extends _Object implements Comparable<Pass> {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Pass other);
	bool operator <(Pass other);
	bool operator <=(Pass other);
	bool operator >(Pass other);
	bool operator >=(Pass other);
	@override int compareTo(Pass other);
	factory Pass(int number);
	int redeem();
}
final class Key extends _Object {
	String debug();
	Key clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Key(String name);
	U64 hashOf();
}
final class ClashSeed extends _Object {
	factory ClashSeed.default_();
	factory ClashSeed.default2_();
	int value();
	bool bool_();
}
final class ClashEmptyString extends _Object {
	bool isEmpty();
}
final class ClashStore extends _Object {
	factory ClashStore(int count);
	int free();
	int close_();
	void wait();
	void finalize();
	int take(Ticket ticket);
	static int take_(Ticket ticket);
	static int count_();
	int count();
	factory ClashStore.open();
	int open();
}
final class ClashReader extends _Object {
	factory ClashReader(typed.Uint8List bytes);
	int readByte();
	void unreadByte();
	void wait(U64 ms);
}
final class ClashNotch extends _Object implements Comparable<ClashNotch> {
	@override String toString();
	String debug();
	ClashNotch clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(ClashNotch other);
	bool operator <(ClashNotch other);
	bool operator <=(ClashNotch other);
	bool operator >(ClashNotch other);
	bool operator >=(ClashNotch other);
	@override int compareTo(ClashNotch other);
	factory ClashNotch.default_();
	factory ClashNotch(int count);
	int count();
}
final class TlsSession extends _Object {
	String alpn();
	TlsPolicyLifetime lifetime();
}
final class Url extends _Object implements Comparable<Url> {
	@override String toString();
	String debug();
	Url clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Url other);
	bool operator <(Url other);
	bool operator <=(Url other);
	bool operator >(Url other);
	bool operator >=(Url other);
	@override int compareTo(Url other);
	factory Url.parse(String input);
}
final class Json extends _Object {
	factory Json.parse(String input);
	String toText();
}
final class Instant extends _Object implements Comparable<Instant> {
	String debug();
	Instant clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int? partialCompareTo(Instant other);
	bool operator <(Instant other);
	bool operator <=(Instant other);
	bool operator >(Instant other);
	bool operator >=(Instant other);
	@override int compareTo(Instant other);
}
final class StdIoError extends _Object {
}
final class TlsConfig extends _Object {
	TlsConfig clone();
}
final class TlsCert extends _Object {
}
final class TlsResolver extends _Object {
	TlsResolver clone();
}
final class TlsCodec extends _Object {
}
final class Spawned extends _Object {
	factory Spawned(int Function() task);
	int join();
}
final class LogThread extends _Object {
	factory LogThread(Log log, String line);
	bool join();
}
final class Pinned extends _Object {
	U64 get valueGet;
	void valueSet(U64 value);
	factory Pinned(U64 value);
	Part part();
}
final class Guarded extends _Object {
	factory Guarded(U64 n);
	U64 get nGet;
	bool get onGet;
	U64 count();
	U64 steps();
	int locals();
}
final class Hooked extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get nGet;
	void nSet(U64 value);
	factory Hooked(U64 n);
	U64 count();
}
final class Forged extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get nGet;
	void nSet(U64 value);
	factory Forged(U64 n);
}
final class Part extends _Object {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get valueGet;
	void valueSet(U64 value);
}
abstract class Log {
	bool log(int level, String line);
	void close();
}
abstract class Scan {
	U64 scan(typed.Uint8List payload, typed.Uint8List? rest);
	void close();
}
abstract class Beat {
	int beat();
	void close();
}
abstract class Turner {
	U64 by();
	@Deprecated('turn `Turner::by` instead') U64 once();
	void close();
}
abstract class Undo {
	int delete(int n);
	int free();
	int type();
	void close();
}
abstract class Stretch {
	U64 stretch();
	void close();
}
abstract class ClashSource {
	void wait();
	int readByte();
	void close();
}
abstract class ClashEraser {
	int delete_(int new_, int new_2);
	int delete();
	void close();
}
abstract class ClashCount {
	int next();
	void close();
}
final class BoundsU64Exception implements Exception {
	final BoundsU64 error;
	const BoundsU64Exception(this.error);
	@override String toString();
}
final class StringException implements Exception {
	final String error;
	const StringException(this.error);
	@override String toString();
}
final class VecClashToneException implements Exception {
	final List<ClashTone> error;
	const VecClashToneException(this.error);
	@override String toString();
}
const limit = U64.fromBits(1000);
const far = 9223372036854775807;
const eof = -1;
const url = U64.fromBits(1000);
@Deprecated('pass a step to `Dial::turn`, whose `}` or `{` alone stays in the note') const step = U64.fromBits(1);
bool hasText(Note note);
U64 marked(Mark apiMarked);
Reading? reading(int n);
String readingDebug(int n);
Reading? readingEcho(Reading? reading);
void readingBump(Mut<Reading?> reading);
Stage advance(Stage stage);
(Counter,) only(Counter counter);
(int, int, int, U64) widths(U64 n);
(U64, U64) divide(U64 a, U64 b);
(int, int) corner(bool unit);
Cancelable<(int, int)> cornerLater();
(int, int, int, U64)? spread(U64 n);
(U64, U64)? find(typed.Uint8List haystack, typed.Uint8List needle);
List<(String, Shape, Point)> rows(List<Point> points);
int port(String text);
int span(int start, int end);
((int, int), (int, int)) halves(Span span);
(int?,)? doubled((int?,)? value);
(String,)? title(bool formal);
(String,)? named(String name);
(typed.Uint8List,) lentBack((typed.Uint8List,) value);
(typed.Uint8List,) boxedTuple(typed.Uint8List bytes);
(int, String) swapped((String, int) pair);
bool lasting(Draft draft);
int stored(Receipt receipt);
int tallied(Receipt receipt);
int summedUpReceipt(Receipt totaled);
int erased(Receipt receipt);
Name longer(Name a, Name b);
Name unnamed();
U64 total(RangeU64 range);
int reach(int start, int end);
BigInt? scaled(BigInt v, U64 by);
Batch2 pair();
U64 drain4(Batch batch);
U64 drain2(Batch2 batch);
U64 capacities44(Batch a, Batch b);
U64 capacities42(Batch a, Batch2 b);
U64 capacities24(Batch2 a, Batch b);
U64 capacities22(Batch2 a, Batch2 b);
Cancelable<typed.Uint32List> fill4(Batch batch, int v);
Cancelable<typed.Uint32List> fill2(Batch2 batch, int v);
RangeU64 widen(RangeU64 range, U64 by);
U64 count(BoundsU64 range);
BoundU64 end(U64 limit);
(BoundU64, BoundU64) ends(U64 limit);
U64 below(U64 n, U64 limit);
Cancelable<String> drive(Moment start, Moment? Function(Moment, Waiter) poll);
String made(int n, String Function(int) make);
List<(String, Ticket)> tickets(List<(String, Ticket)> entries);
Ticket reissue(Ticket ticket, Ticket Function(Ticket) swap);
Ticket reissueLent(Ticket ticket, Ticket Function(Ticket) swap);
Seat seat(Mark mark, int row);
U64 admit((Counter, Mark, Seat, Choice) entry);
U64 choose(Choice choice);
U64 heft((Mark, typed.Uint8List) marked, Parcel lent, (Counter, typed.Uint8List) moved);
U64 spend(Counter e, Counter it);
bool isKept(Status status);
void attempt(void Function() check);
(typed.Uint8List, U64) tail(SeekableReader file, U64 len);
U64 seekTo(SeekableReader file, U64 at);
Cancelable<int?> halved(int n);
Cancelable<int> yielded(int value);
Cancelable<int> after(Future<void> first, int value);
Cancelable<void> within(int polls, Future<void> future);
Moment momentAt(U64 ticks, U64 rate);
Stamp stampAt(U64 millis);
Moment shifted(Moment moment, Moment Function(Moment) shift);
int jvmSum(JvmField field, JvmStatic method, OptIn opt);
Clocked clockedAfter(Clocked clocked, U64 ticks);
int emit(Log log, typed.Uint8List lines);
Cancelable<int> counted(int n, void Function(int) count);
int filter(typed.Uint8List lines, bool Function(int, String) keep);
Cancelable<bool> logLater(Log log);
int logCounts(Log log, int level, Counter counter, List<Counter> more);
bool onFirstThread();
int localsDropped();
Local localOrNew(Local? local);
Cancelable<Local> localOrNewLater(Local? local);
int bumpEither(Local? first, Local second);
Cancelable<Local> localAfter();
Cancelable<int> localLater(int n);
Cancelable<int> pollTripled(int n);
U64 watch(Gauge gauge, U64 Function(Gauge) watch);
typed.Uint8List tokenBytes(int id, int len);
typed.Uint8List tokenSlice(int id, int len, int of);
Packet nextPacket(Packet packet);
ArrayU8x4 fingerprint(String hex);
int dropped();
int bumpBoth(Tally a, Tally b);
int offset(int Function() by, int byData);
int shift(int Function() by, int cBy);
U64 walk(U64 steps, U64 Function(Cursor) visit);
int each(int steps, void Function(int) visit);
int peek(Tally tally, int Function(Tally) peek);
U64 len(typed.Uint8List payload);
String hex(typed.Uint8List payload);
String timedHex(typed.Uint8List payload);
String sampleHex(typed.Uint8List sample);
String queuedHex(typed.Uint8List queued);
String tickName(Label tick);
String rootedName(Label rooted);
String storedText(String stored);
String layerName(Label layer);
String greet(String name, Shape shape);
String describe(Event event);
PairSizeU8 tag(Size size, int tag);
int cross(LeftRightU8 left, RightLeftU8 right);
U64 keptPoint(Point from);
U64 keptCounter(Counter from);
U64 keptSize(Size from);
U64 keptWrapU32(WrapU32 from);
U64 keptPolarPoint(PolarPoint from);
int keepPoint(Point kept);
int keptOrPoint(Point? kept);
int keeperPoint(Point keeper);
BoxedU16 get boxed;
BoxedU8 keeps(Log log);
int weighPoint(Point from);
int applyNonZeroU16(PairNonZeroU16 pair, int Function(int) f);
int applyU32(PairU32 pair, int Function(int) f);
int reportNonZeroU16(PairNonZeroU16 pair, Log log);
int reportU32(PairU32 pair, Log log);
int tallyLogCount(Log log);
int wind(PolarTurn whole, PolarTurn2 part);
U64 area(Size size);
U64 seedsPointPoint(Point from);
U64 seedsPointCounter(Counter from);
U64 seedsPointSize(Size from);
U64 seedsPointWrapU32(WrapU32 from);
U64 seedsPointPolarPoint(PolarPoint from);
U64 seedsCounterPoint(Point from);
U64 seedsCounterCounter(Counter from);
U64 seedsCounterSize(Size from);
U64 seedsCounterWrapU32(WrapU32 from);
U64 seedsCounterPolarPoint(PolarPoint from);
Area grow(Area area, int by);
SlotU32 wide(int value);
String? host(Url url);
Url resolve(Url base, String path);
typed.Uint16List ports(List<SocketAddr> addrs);
RustDuration doubledTtl(RustDuration ttl);
PathBuf file(PathBuf dir, String name);
PathBuf stray();
void visitStray(void Function(PathBuf) visit);
String? nextChar(String c);
RustDuration? extended(RustDuration duration, RustDuration by);
List<PathBuf?> parents(List<PathBuf> paths);
List<String> words(String text);
String sentence(List<String>? words);
void putNumber(int value, Mut<typed.Uint8List> buf);
RustDuration secondLater(RustDuration w235);
int digit(Mut<typed.Uint8List> r);
int pushCounted(Mut<(typed.Uint8List, int)> entry, int byte);
void nudge(Mut<Point> point, int by);
void nudgeBy(Mut<Point> point, Refusal refusal);
TlsConfig secure(String alpn);
String alpn(TlsConfig config);
String alpnLent(TlsConfig config);
TlsCert certify(typed.Uint8List der);
Cancelable<U64> byteLen(typed.Uint8List bytes);
U64 certLen(TlsCert cert);
typed.Uint8List certDer(TlsCert cert);
U64 chainLen(List<TlsCert> chain);
int verify(TlsCert cert);
int parseCert(String text);
Cancelable<int> parseLater(String text);
TlsResolver resolver(String name);
String resolved(TlsResolver resolver);
StdIoError failure(String message);
String failureMessage(StdIoError error);
String failureKind(StdIoError error);
int sumAll(List<int> items);
U64 weighAllCounter(List<Counter> items);
U64 offeredLen(List<typed.Uint8List> alpns);
String joinWords(List<String> words);
TlsSession resume(String alpn, int seconds);
U64 either(String? text);
Cancelable<U64> eitherLater(String? text);
typed.Uint8List prefixed(typed.Uint8List payload, typed.Uint8List? prefix);
int sumLists(typed.Int32List a, typed.Int32List? b, typed.Int32List? c);
int coordinates(Point? point);
List<typed.Uint8List> chunks(typed.Uint8List bytes, U64 size);
List<Counter> counters(List<String> names);
U64 worth(List<Counter>? some, List<Counter> boxed);
U64? heldCount((Counter,)? held);
Counter? boxedCounter(String? name);
String? boxedName(Counter? boxed);
int less(int value, int value2);
typed.Uint32List sums(List<typed.Uint32List> rows);
List<List<int>> least(List<List<int>> sets);
(typed.Int32List, int) splitLast(typed.Int32List items);
U64 sumU32(typed.Uint32List items);
typed.Int64List doubledI64(typed.Int64List items);
List<U64> visitChunks(typed.Uint8List payload, U64 size, U64 Function(typed.Uint8List?) visit);
U64 scanned(Scan scan, typed.Uint8List payload);
typed.Float64List boxedFloats(typed.Float64List items);
Point summed(List<Point> points);
RustDuration breadth(LimitsDuration limits);
LimitsBytes byteLimits(typed.Uint8List low, typed.Uint8List high);
typed.Uint8List joined(List<typed.Uint8List> parts);
String? firstOf(List<String> items);
U64 readLen(typed.Uint8List reader);
U64 longest(List<String> words);
String encode(Json value);
Json tagged(Json doc, String tag);
List<(String, Json)> entries(List<(String, Json)> entries, String tag);
List<(String, Json)> sections(List<(String, Json)> entries);
Instant now();
Instant later(Instant at, RustDuration by);
RustDuration since(Instant later, Instant earlier);
bool hasInstant(Instant? at);
Level? louder(Level? filter);
String spell(Level? filter);
List<String> get greetings;
typed.Uint32List get primes;
List<Counter> numbered(List<String> names);
(SocketAddrV4, SocketAddrV6) nexts(SocketAddrV4 v4, SocketAddrV6 v6);
(Ipv4Addr, Ipv6Addr) nextIps(Ipv4Addr v4, Ipv6Addr v6);
IpAddr flip(IpAddr ip);
String reasons(List<(Counter, Refusal)> pairs, (Counter, Refusal)? last);
U64 excuse(typed.Uint8List note, (typed.Uint8List, Refusal) entry);
int relay(Log log, List<Refusal> refusals);
int refuse(String reason);
int jam(int n);
Platform platform();
int pointerBits(Platform platform);
Labelled sameLabelled(Labelled labelled);
Placement placement();
bool isFamily(Family family);
int wasmOnly();
U64 wasmTake(Counter counter);
void wasmLog(Log log);
U64 boothCount(Booth booth);
Page page();
int beats(Beat beat);
int paired(int Function() f, int Function() fUnwind);
int same(double a, double b);
int errno();
bool assert_(bool holds);
int offsetof(int field);
(String, (int, int, int, U64)?, List<typed.Uint8List>) returnValues(Mut<Point> point, bool fail);
Counter returnMoved(Counter counter, bool fail);
NonZeros nonZeros();
String describeNonZeros(NonZeros nonZeros);
U64? nextHop(U64? hop);
int fartherHop(PairNonZeroU16 hops);
int wexitstatus(int status);
RustDuration sameDuration(RustDuration v);
U64 sameU64(U64 v);
U64 sameUsize(U64 v);
int sameIsize(int v);
BigInt sameU128(BigInt v);
String sameChar(String v);
String sameString(String v);
PathBuf samePath(PathBuf v);
SocketAddr sameSocketAddr(SocketAddr v);
SocketAddrV4 sameSocketAddrV4(SocketAddrV4 v);
SocketAddrV6 sameSocketAddrV6(SocketAddrV6 v);
IpAddr sameIpAddr(IpAddr v);
String unescaped(String text);
Cancelable<String> unescapedLater(String text);
Cancelable<String?> unescapedMaybe(String? text);
typed.Uint8List unzeroed(typed.Uint8List bytes);
int deprecated(int note);
int deprecated_(int note);
int all(int note);
int all_(int note);
@Deprecated('') Knob knob(Dial dial);
@Deprecated('read its `level`') Cancelable<U64> levelLater(Dial dial);
Dial turned(Dial dial, Turner turner);
int undone(Undo undo, int n);
int unwound();
U64 measured(U64 Function() f);
int negated(int Function() f);
String lettered(String Function() f);
String bumped(Mut<int> n, String Function() f);
U64 stretched(Stretch stretch);
U64 crowded(Reading reading, Stamp a, Stamp b, U64 n0, U64 n1, U64 n2, U64 n3, U64 n4, U64 n5, U64 n6, U64 n7, U64 n8, U64 n9, U64 n10, U64 n11, U64 n12, U64 n13, U64 n14, U64 n15, U64 n16, U64 n17, U64 n18, U64 n19, U64 n20, U64 n21, U64 n22, U64 n23, U64 n24, U64 n25, U64 n26, U64 n27, U64 n28, U64 n29, U64 n30, U64 n31, U64 n32, U64 n33, U64 n34, U64 n35, U64 n36, U64 n37, U64 n38, U64 n39, U64 n40, U64 n41, U64 n42, U64 n43, U64 n44, U64 n45, U64 n46, U64 n47, U64 n48, U64 n49, U64 n50, U64 n51, U64 n52, U64 n53, U64 n54, U64 n55, U64 n56, U64 n57, U64 n58, U64 n59, U64 n60, U64 n61, U64 n62, U64 n63, U64 n64, U64 n65, U64 n66, U64 n67, U64 n68, U64 n69, U64 n70, U64 n71, U64 n72, U64 n73, U64 n74, U64 n75, U64 n76, U64 n77, U64 n78, U64 n79, U64 n80, U64 n81, U64 n82, U64 n83, U64 n84, U64 n85, U64 n86, U64 n87, U64 n88, U64 n89, U64 n90, U64 n91, U64 n92, U64 n93, U64 n94, U64 n95, U64 n96, U64 n97, U64 n98, U64 n99, U64 n100, U64 n101, U64 n102, U64 n103, U64 n104, U64 n105, U64 n106, U64 n107, U64 n108, U64 n109, U64 n110, U64 n111, U64 n112, U64 n113, U64 n114, U64 n115, U64 n116, U64 n117, U64 n118);
U64 crammed(Moment a, Moment b, U64 n0, U64 n1, U64 n2, U64 n3, U64 n4, U64 n5, U64 n6, U64 n7, U64 n8, U64 n9, U64 n10, U64 n11, U64 n12, U64 n13, U64 n14, U64 n15, U64 n16, U64 n17, U64 n18, U64 n19, U64 n20, U64 n21, U64 n22, U64 n23, U64 n24, U64 n25, U64 n26, U64 n27, U64 n28, U64 n29, U64 n30, U64 n31, U64 n32, U64 n33, U64 n34, U64 n35, U64 n36, U64 n37, U64 n38, U64 n39, U64 n40, U64 n41, U64 n42, U64 n43, U64 n44, U64 n45, U64 n46, U64 n47, U64 n48, U64 n49, U64 n50, U64 n51, U64 n52, U64 n53, U64 n54, U64 n55, U64 n56, U64 n57, U64 n58, U64 n59, U64 n60, U64 n61, U64 n62, U64 n63, U64 n64, U64 n65, U64 n66, U64 n67, U64 n68, U64 n69, U64 n70, U64 n71, U64 n72, U64 n73, U64 n74, U64 n75, U64 n76, U64 n77, U64 n78, U64 n79, U64 n80, U64 n81, U64 n82, U64 n83, U64 n84, U64 n85, U64 n86, U64 n87, U64 n88, U64 n89, U64 n90, U64 n91, U64 n92, U64 n93, U64 n94, U64 n95, U64 n96, U64 n97, U64 n98, U64 n99, U64 n100, U64 n101, U64 n102, U64 n103, U64 n104, U64 n105, U64 n106, U64 n107, U64 n108, U64 n109, U64 n110, U64 n111, U64 n112, U64 n113, U64 n114, U64 n115, U64 n116, U64 n117, U64 n118, U64 n119, U64 n120, U64 n121, U64 n122, U64 n123);
int jniOne();
PolarPoint? polarPoint(int distance);
int macrosStreamed(MacrosStreams streams, int errno);
ClashAny clashAny(String typeUrl);
ClashCandidate clashCandidate(int at);
int clashIgnore(int arg0, int arg1, int arg2);
ClashEmptyString clashEmpty();
int clashDescribed(ClashDescribed described);
int clashWaited(ClashSource source);
int clashUnderscored(ClashUnderscored underscored, ClashOp op, ClashChange change);
(ClashUnderscored, ClashChange) clashUnderscoredNext(ClashUnderscored underscored, ClashChange change);
int clashErased(ClashEraser eraser);
List<ClashNotch> clashNotches(int n);
int clashShifted(int at, ClashShift shift);
int clashCounted(ClashCount count);
int clashErase_();
int clashErase();
int clashLend_(ClashStore? store);
int clashLend();
int clashRun(int Function() work2, int work);
ClashTone clashAudible(ClashTone tone);
int clashNested(((int, int), int)? a, ((int, int, int),)? b);
int clashAudioEncoderFinish();
int clashAudioEncoderFinish_();
int emitFromThread(Log log, typed.Uint8List lines);
Cancelable<int> emitLater(Log log, typed.Uint8List lines);
Cancelable<void> logOnDrop(Log log);
Waker logWaker(Log log);
void hook(void Function() f);
void unhook();
