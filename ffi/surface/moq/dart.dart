// moq.dart
extension type const U64.fromBits(int bits) {
	U64(int value);
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
final class ArrayU8x12 {
	ArrayU8x12(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class ArrayU8x16 {
	ArrayU8x16(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class ArrayU8x2 {
	ArrayU8x2(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class ArrayU8x3 {
	ArrayU8x3(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class ArrayU8x32 {
	ArrayU8x32(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
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
final class ArrayU8x6 {
	ArrayU8x6(List<int> bytes);
	int operator [](int index);
	typed.Uint8List toBytes();
	@override bool operator ==(Object other);
	@override int get hashCode;
	@override String toString();
}
final class TimeOverflow implements Exception {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Timescale {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Timescale get SECOND;
	static Timescale get MILLI;
	static Timescale get MICRO;
	static Timescale get NANO;
	factory Timescale(U64 unitsPerSecond);
	U64 asU64();
}
final class Timestamp {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Timestamp get ZERO;
	factory Timestamp(U64 value, Timescale scale);
	factory Timestamp.fromScale(U64 value, U64 unitsPerSecond);
	factory Timestamp.fromSecs(U64 seconds);
	factory Timestamp.fromMillis(U64 millis);
	factory Timestamp.fromMicros(U64 micros);
	factory Timestamp.fromNanos(U64 nanos);
	U64 value();
	Timescale scale();
	bool isZero();
	Timestamp convert(Timescale newScale);
	BigInt asScale(Timescale target);
	U64 asSecs();
	BigInt asMillis();
	BigInt asMicros();
	BigInt asNanos();
	Timestamp checkedAdd(Timestamp rhs);
	Timestamp checkedSub(Timestamp rhs);
	factory Timestamp.now();
}
final class Datagram {
	final U64 sequence;
	final Timestamp? timestamp;
	final typed.Uint8List payload;
	const Datagram({required this.sequence, required this.timestamp, required this.payload});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Hop {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Hop get UNKNOWN;
	factory Hop(U64 id);
	factory Hop.random();
	U64 id();
}
final class ClockInstant {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static ClockInstant get ZERO;
	factory ClockInstant.now();
	RustDuration elapsed();
	RustDuration durationSince(ClockInstant earlier);
	RustDuration? checkedDurationSince(ClockInstant earlier);
	RustDuration saturatingDurationSince(ClockInstant earlier);
	ClockInstant? checkedAdd(RustDuration duration);
	ClockInstant? checkedSub(RustDuration duration);
	Instant intoStd();
	factory ClockInstant.fromStd(Instant at);
}
final class ClockElapsed implements Exception {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClockWall {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static ClockWall get UNIX_EPOCH;
	factory ClockWall.now();
	RustDuration? checkedDurationSince(ClockWall earlier);
	RustDuration saturatingDurationSince(ClockWall earlier);
	ClockWall? checkedAdd(RustDuration duration);
	ClockWall saturatingAdd(RustDuration duration);
	ClockWall? checkedSub(RustDuration duration);
}
final class SetupExtensions {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory SetupExtensions.default_();
	bool get authGet;
	SetupExtensions authSet(bool value);
	bool get solicitGet;
	SetupExtensions solicitSet(bool value);
	bool get ietfGet;
	SetupExtensions ietfSet(bool value);
}
final class SetupToken {
	final U64 kind;
	final typed.Uint8List value;
	const SetupToken({required this.kind, required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class SessionLimits {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory SessionLimits.default_();
	U64 get announcesGet;
	SessionLimits announcesSet(U64 value);
}
final class StatsContent {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsContent.default_();
	U64 get bytesGet;
	StatsContent bytesSet(U64 value);
	U64 get framesGet;
	StatsContent framesSet(U64 value);
	U64 get groupsGet;
	StatsContent groupsSet(U64 value);
}
final class StatsTraffic {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsTraffic.default_();
	U64 get announcesStartedGet;
	StatsTraffic announcesStartedSet(U64 value);
	U64 get announcesEndedGet;
	StatsTraffic announcesEndedSet(U64 value);
	U64 get announcedBytesGet;
	StatsTraffic announcedBytesSet(U64 value);
	U64 get broadcastsStartedGet;
	StatsTraffic broadcastsStartedSet(U64 value);
	U64 get broadcastsEndedGet;
	StatsTraffic broadcastsEndedSet(U64 value);
	U64 get subscriptionsStartedGet;
	StatsTraffic subscriptionsStartedSet(U64 value);
	U64 get subscriptionsEndedGet;
	StatsTraffic subscriptionsEndedSet(U64 value);
	U64 get fetchesGet;
	StatsTraffic fetchesSet(U64 value);
	U64 get bytesGet;
	StatsTraffic bytesSet(U64 value);
	U64 get framesGet;
	StatsTraffic framesSet(U64 value);
	U64 get groupsGet;
	StatsTraffic groupsSet(U64 value);
	U64 get datagramsGet;
	StatsTraffic datagramsSet(U64 value);
	U64 get inflatedGet;
	StatsTraffic inflatedSet(U64 value);
	StatsContent get staleGet;
	StatsTraffic staleSet(StatsContent value);
	StatsTraffic add(StatsTraffic other);
	U64 activeBroadcasts();
	U64 activeSubscriptions();
	bool isIdle();
}
final class StatsPresence {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsPresence.default_();
	StatsRefusals get sessionsRefusedGet;
	StatsPresence sessionsRefusedSet(StatsRefusals value);
	StatsFailures get sessionsFailedGet;
	StatsPresence sessionsFailedSet(StatsFailures value);
	U64 get requestsGet;
	StatsPresence requestsSet(U64 value);
	U64 get sessionsStartedGet;
	StatsPresence sessionsStartedSet(U64 value);
	U64 get sessionsEndedGet;
	StatsPresence sessionsEndedSet(U64 value);
	U64 get sentBytesGet;
	StatsPresence sentBytesSet(U64 value);
	U64 get receivedBytesGet;
	StatsPresence receivedBytesSet(U64 value);
	U64 get announcesPeakGet;
	StatsPresence announcesPeakSet(U64 value);
	U64 get subscriptionsPeakGet;
	StatsPresence subscriptionsPeakSet(U64 value);
	U64 get wireTxGet;
	StatsPresence wireTxSet(U64 value);
	U64 get wireRxGet;
	StatsPresence wireRxSet(U64 value);
	StatsPresence add(StatsPresence other);
	U64 active();
}
final class StatsTotals {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsTotals.default_();
	U64 get txGet;
	StatsTotals txSet(U64 value);
	U64 get rxGet;
	StatsTotals rxSet(U64 value);
}
final class StatsFailures {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsFailures.default_();
	U64 get idleTimeoutGet;
	StatsFailures idleTimeoutSet(U64 value);
	U64 get transportGet;
	StatsFailures transportSet(U64 value);
	U64 get applicationGet;
	StatsFailures applicationSet(U64 value);
	U64 get expiredGet;
	StatsFailures expiredSet(U64 value);
	U64 count(StatsFailure kind);
	StatsFailures add(StatsFailures other);
	bool regressed(StatsFailures previous);
}
final class StatsRefusals {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsRefusals.default_();
	U64 get refusedGet;
	StatsRefusals refusedSet(U64 value);
	U64 get unavailableGet;
	StatsRefusals unavailableSet(U64 value);
	U64 get requestGet;
	StatsRefusals requestSet(U64 value);
	U64 get forbiddenGet;
	StatsRefusals forbiddenSet(U64 value);
	U64 get lanGet;
	StatsRefusals lanSet(U64 value);
	U64 get expiredGet;
	StatsRefusals expiredSet(U64 value);
	U64 count(StatsRefusalKind kind);
	StatsRefusals add(StatsRefusals other);
	bool regressed(StatsRefusals previous);
}
final class TransportReceiveWindow {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64 get connectionGet;
	TransportReceiveWindow connectionSet(U64 value);
	U64 get streamGet;
	TransportReceiveWindow streamSet(U64 value);
	factory TransportReceiveWindow(U64 connection, U64 stream);
}
final class BandwidthRate {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory BandwidthRate.default_();
	static BandwidthRate get ZERO;
	factory BandwidthRate.fromBps(U64 bps);
	factory BandwidthRate.fromKbps(U64 kbps);
	factory BandwidthRate.fromMbps(U64 mbps);
	U64 asBps();
	BandwidthRate scaled(double factor);
	BandwidthRate absDiff(BandwidthRate other);
}
final class FrameInfo {
	final U64 size;
	final Timestamp? timestamp;
	const FrameInfo({required this.size, required this.timestamp});
	String debug();
}
final class GroupInfo {
	final U64 sequence;
	const GroupInfo({required this.sequence});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class TrackDelay {
	final RustDuration? min;
	final RustDuration? max;
	const TrackDelay({required this.min, required this.max});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackDelay.default_();
	TrackDelay withMin(RustDuration? min);
	TrackDelay withMax(RustDuration? max);
}
final class TrackPosition {
	final U64 group;
	final U64 frame;
	const TrackPosition({required this.group, required this.frame});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackPosition.default_();
	factory TrackPosition.group(U64 group);
	static TrackPosition? after(U64 group, U64 frame);
	static TrackPosition? afterGroup(U64 group);
	BoundU64 groupEnd();
}
final class TrackRange {
	final TrackAnchor start;
	final TrackAnchor? end;
	const TrackRange({required this.start, required this.end});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackRange.live();
	factory TrackRange.oldest();
	factory TrackRange.groups(BoundsU64 groups);
}
final class OriginCost {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginCost.default_();
	factory OriginCost(U64 cost);
	U64 value();
	static OriginCost get MAX;
	static OriginCost get DRAIN;
}
final class OriginHopLimit {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginHopLimit.default_();
	static OriginHopLimit? new_(U64 max);
	U64 get();
}
final class OriginPreference {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginPreference.default_();
	factory OriginPreference(int preference);
	int value();
}
final class HangHealthLimitsF64 {
	final double degraded;
	final double unhealthy;
	const HangHealthLimitsF64({required this.degraded, required this.unhealthy});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangHealthLimitsDuration {
	final RustDuration degraded;
	final RustDuration unhealthy;
	const HangHealthLimitsDuration({required this.degraded, required this.unhealthy});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogAac {
	final int profile;
	const HangCatalogAac({required this.profile});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogClock {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Timestamp get wallGet;
	HangCatalogClock wallSet(Timestamp value);
	factory HangCatalogClock(Timestamp wall);
	ClockWall wallClock(Timestamp pts);
}
final class HangCatalogPriorities {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int get catalogGet;
	HangCatalogPriorities catalogSet(int value);
	int get textGet;
	HangCatalogPriorities textSet(int value);
	int get audioGet;
	HangCatalogPriorities audioSet(int value);
	int get videoGet;
	HangCatalogPriorities videoSet(int value);
}
final class HangCatalogAv1 {
	final int profile;
	final int level;
	final String tier;
	final int bitdepth;
	final bool monoChrome;
	final bool chromaSubsamplingX;
	final bool chromaSubsamplingY;
	final int chromaSamplePosition;
	final int colorPrimaries;
	final int transferCharacteristics;
	final int matrixCoefficients;
	final bool fullRange;
	const HangCatalogAv1({required this.profile, required this.level, required this.tier, required this.bitdepth, required this.monoChrome, required this.chromaSubsamplingX, required this.chromaSubsamplingY, required this.chromaSamplePosition, required this.colorPrimaries, required this.transferCharacteristics, required this.matrixCoefficients, required this.fullRange});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogAv1.default_();
	static String get PREFIX;
}
final class HangCatalogMastering {
	final HangCatalogChromaticity red;
	final HangCatalogChromaticity green;
	final HangCatalogChromaticity blue;
	final HangCatalogChromaticity white;
	final double maxLuminance;
	final double minLuminance;
	const HangCatalogMastering({required this.red, required this.green, required this.blue, required this.white, required this.maxLuminance, required this.minLuminance});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogChromaticity {
	final double x;
	final double y;
	const HangCatalogChromaticity({required this.x, required this.y});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogContentLight {
	final int maxCll;
	final int maxFall;
	const HangCatalogContentLight({required this.maxCll, required this.maxFall});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogH264 {
	final bool inline;
	final int profile;
	final int constraints;
	final int level;
	const HangCatalogH264({required this.inline, required this.profile, required this.constraints, required this.level});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogH265 {
	final bool inBand;
	final int profileSpace;
	final int profileIdc;
	final ArrayU8x4 profileCompatibilityFlags;
	final bool tierFlag;
	final int levelIdc;
	final ArrayU8x6 constraintFlags;
	const HangCatalogH265({required this.inBand, required this.profileSpace, required this.profileIdc, required this.profileCompatibilityFlags, required this.tierFlag, required this.levelIdc, required this.constraintFlags});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogVp9 {
	final int profile;
	final int level;
	final int bitDepth;
	final int chromaSubsampling;
	final int colorPrimaries;
	final int transferCharacteristics;
	final int matrixCoefficients;
	final bool fullRange;
	const HangCatalogVp9({required this.profile, required this.level, required this.bitDepth, required this.chromaSubsampling, required this.colorPrimaries, required this.transferCharacteristics, required this.matrixCoefficients, required this.fullRange});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogVp9.default_();
}
final class HangCatalogDisplay {
	final int width;
	final int height;
	const HangCatalogDisplay({required this.width, required this.height});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangContainerFrame {
	final Timestamp timestamp;
	final typed.Uint8List payload;
	const HangContainerFrame({required this.timestamp, required this.payload});
	String debug();
	void encode(Mut<typed.Uint8List> buf);
	factory HangContainerFrame.decode(typed.Uint8List buf);
	void writeTo(GroupProducer group);
}
final class MuxContainerFrame {
	final Timestamp timestamp;
	final Timestamp? duration;
	final typed.Uint8List payload;
	final bool keyframe;
	const MuxContainerFrame({required this.timestamp, required this.duration, required this.payload, required this.keyframe});
	String debug();
}
final class MuxContainerTimestampRewind {
	final Timestamp timestamp;
	final Timestamp floor;
	const MuxContainerTimestampRewind({required this.timestamp, required this.floor});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxSelectSelected {
	final List<String> video;
	final List<String> audio;
	const MuxSelectSelected({required this.video, required this.audio});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxSelectSelected.default_();
	factory MuxSelectSelected.ofExtra(HangCatalogExtra catalog);
	factory MuxSelectSelected.ofExt(HangCatalogExt catalog);
	factory MuxSelectSelected.ofUnit(HangCatalog catalog);
}
final class MuxTimelineTransition {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxTimelineAudio {
	final RustDuration end;
	final RustDuration buffered;
	final RustDuration target;
	const MuxTimelineAudio({required this.end, required this.buffered, required this.target});
	String debug();
}
final class MuxCodecAacConfig {
	final int profile;
	final int sampleRate;
	final int channelCount;
	const MuxCodecAacConfig({required this.profile, required this.sampleRate, required this.channelCount});
	factory MuxCodecAacConfig.parse(Mut<typed.Uint8List> buf);
	typed.Uint8List encode();
}
final class MuxCodecFlacConfig {
	final int minBlockSize;
	final int maxBlockSize;
	final int minFrameSize;
	final int maxFrameSize;
	final int sampleRate;
	final int channelCount;
	final int bitsPerSample;
	final U64 totalSamples;
	final ArrayU8x16 md5;
	const MuxCodecFlacConfig({required this.minBlockSize, required this.maxBlockSize, required this.minFrameSize, required this.maxFrameSize, required this.sampleRate, required this.channelCount, required this.bitsPerSample, required this.totalSamples, required this.md5});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxCodecFlacConfig.parseStreamInfo(Mut<typed.Uint8List> buf);
	factory MuxCodecFlacConfig.parse(Mut<typed.Uint8List> buf);
	typed.Uint8List encodeStreamInfo();
	typed.Uint8List description();
}
final class MuxCodecH264Sps {
	final int profile;
	final int constraints;
	final int level;
	final int codedWidth;
	final int codedHeight;
	const MuxCodecH264Sps({required this.profile, required this.constraints, required this.level, required this.codedWidth, required this.codedHeight});
	String debug();
	factory MuxCodecH264Sps.parse(typed.Uint8List nal);
}
final class MuxCodecMp3Config {
	final int sampleRate;
	final int channelCount;
	const MuxCodecMp3Config({required this.sampleRate, required this.channelCount});
	factory MuxCodecMp3Config.parse(typed.Uint8List data);
}
final class MuxCodecOpusMapping {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int family();
	int streams();
	int coupled();
	typed.Uint8List table();
}
final class MuxContainerFmp4Fragment {
	final typed.Uint8List data;
	final bool independent;
	final RustDuration duration;
	const MuxContainerFmp4Fragment({required this.data, required this.independent, required this.duration});
	String debug();
}
final class MuxContainerLegacyWire {
	final MuxContainerKind $0;
	const MuxContainerLegacyWire(this.$0);
}
final class MuxContainerLocWire {
	final MuxContainerKind $0;
	const MuxContainerLocWire(this.$0);
}
final class MuxContainerMp4Finish {
	final U64 moovOffset;
	final typed.Uint8List moov;
	final U64 mdatOffset;
	final typed.Uint8List mdat;
	const MuxContainerMp4Finish({required this.moovOffset, required this.moov, required this.mdatOffset, required this.mdat});
	String debug();
}
final class MuxContainerTsDescriptor {
	final int tag;
	final typed.Uint8List data;
	const MuxContainerTsDescriptor({required this.tag, required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxContainerFmp4FragmentConfig {
	String debug();
	factory MuxContainerFmp4FragmentConfig.default_();
	MuxContainerFmp4FragmentMissingDuration get missingDurationGet;
	MuxContainerFmp4FragmentConfig missingDurationSet(MuxContainerFmp4FragmentMissingDuration value);
}
final class Mp4AtomFourCc {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFourCc(ArrayU8x4 value);
}
final class Mp4AtomMp4a {
	final Mp4AtomAudio audio;
	final Mp4AtomEsds esds;
	final Mp4AtomBtrt? btrt;
	final Mp4AtomTaic? taic;
	const Mp4AtomMp4a({required this.audio, required this.esds, required this.btrt, required this.taic});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMp3 {
	final Mp4AtomAudio audio;
	const Mp4AtomMp3({required this.audio});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomOpus {
	final Mp4AtomAudio audio;
	final Mp4AtomDops dops;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomOpus({required this.audio, required this.dops, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomFlac {
	final Mp4AtomAudio audio;
	final Mp4AtomDfla dfla;
	const Mp4AtomFlac({required this.audio, required this.dfla});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomAc3 {
	final Mp4AtomAudio audio;
	final Mp4AtomAc3SpecificBox dac3;
	const Mp4AtomAc3({required this.audio, required this.dac3});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomEac3 {
	final Mp4AtomAudio audio;
	final Mp4AtomEc3SpecificBox dec3;
	const Mp4AtomEac3({required this.audio, required this.dec3});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIpcm {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomIpcm({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomFpcm {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomFpcm({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomSowt {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomSowt({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomTwos {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomTwos({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomLpcm {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomLpcm({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomIn24 {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomIn24({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomIn32 {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomIn32({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomFl32 {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomFl32({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomFl64 {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomFl64({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomS16l {
	final Mp4AtomAudio audio;
	final Mp4AtomPcmC? pcmc;
	final Mp4AtomChnl? chnl;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomS16l({required this.audio, required this.pcmc, required this.chnl, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomPcmFormat? format();
}
final class Mp4AtomWvtt {
	final Mp4AtomPlainText plaintext;
	final Mp4AtomVttC config;
	final Mp4AtomVlab? label;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomWvtt({required this.plaintext, required this.config, required this.label, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSamr {
	final Mp4AtomAmrSampleEntry amrsampleentry;
	final Mp4AtomDamr damr;
	const Mp4AtomSamr({required this.amrsampleentry, required this.damr});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMett {
	final Mp4AtomMetaData metadata;
	final String contentEncoding;
	final String mimeFormat;
	final Mp4AtomBtrt? btrt;
	final Mp4AtomTxtC? txtc;
	const Mp4AtomMett({required this.metadata, required this.contentEncoding, required this.mimeFormat, required this.btrt, required this.txtc});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMetx {
	final Mp4AtomMetaData metadata;
	final String contentEncoding;
	final String namespace;
	final String schemaLocation;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomMetx({required this.metadata, required this.contentEncoding, required this.namespace, required this.schemaLocation, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUrim {
	final Mp4AtomMetaData metadata;
	final Mp4AtomUri theLabel;
	final Mp4AtomUriI? init;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomUrim({required this.metadata, required this.theLabel, required this.init, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomCamm {
	final Mp4AtomMetaData metadata;
	const Mp4AtomCamm({required this.metadata});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomEvte {
	final Mp4AtomMetaData metadata;
	final Mp4AtomBtrt? btrt;
	final Mp4AtomSilb? silb;
	const Mp4AtomEvte({required this.metadata, required this.btrt, required this.silb});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMebx {
	final Mp4AtomMetaData metadata;
	final List<Mp4AtomMebxKey> keys;
	final Mp4AtomBtrt? btrt;
	const Mp4AtomMebx({required this.metadata, required this.keys, required this.btrt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomTkhd {
	final U64 creationTime;
	final U64 modificationTime;
	final int trackId;
	final U64 duration;
	final int layer;
	final int alternateGroup;
	final bool enabled;
	final bool inMovie;
	final bool sizeIsAspectRatio;
	final Mp4AtomFixedPointU8 volume;
	final Mp4AtomMatrix matrix;
	final Mp4AtomFixedPointU16 width;
	final Mp4AtomFixedPointU16 height;
	const Mp4AtomTkhd({required this.creationTime, required this.modificationTime, required this.trackId, required this.duration, required this.layer, required this.alternateGroup, required this.enabled, required this.inMovie, required this.sizeIsAspectRatio, required this.volume, required this.matrix, required this.width, required this.height});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTkhd.default_();
}
final class Mp4AtomEdts {
	final Mp4AtomElst? elst;
	const Mp4AtomEdts({required this.elst});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEdts.default_();
}
final class Mp4AtomSenc {
	final Mp4AtomSencBoxVersion version;
	final bool useSubsamples;
	final typed.Uint8List data;
	const Mp4AtomSenc({required this.version, required this.useSubsamples, required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSenc.default_();
}
final class Mp4AtomTref {
	final List<Mp4AtomTrackReferenceTypeBox> trackReferenceTypeBoxes;
	const Mp4AtomTref({required this.trackReferenceTypeBoxes});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTref.default_();
}
final class Mp4AtomAudio {
	final int dataReferenceIndex;
	final int channelCount;
	final int sampleSize;
	final Mp4AtomFixedPointU16 sampleRate;
	const Mp4AtomAudio({required this.dataReferenceIndex, required this.channelCount, required this.sampleSize, required this.sampleRate});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMetaData {
	final int dataReferenceIndex;
	const Mp4AtomMetaData({required this.dataReferenceIndex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class RustlsClientTicketRequest {
	final int newSessionCount;
	final int resumptionCount;
	const RustlsClientTicketRequest({required this.newSessionCount, required this.resumptionCount});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqQuicVarInt {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqQuicVarInt.default_();
	static MoqQuicVarInt get MAX;
	factory MoqQuicVarInt.fromU32(int x);
	factory MoqQuicVarInt.fromU64(U64 x);
	U64 intoInner();
	void encode(Mut<typed.Uint8List> output);
}
final class Mp4AtomAvcc {
	final int configurationVersion;
	final int avcProfileIndication;
	final int profileCompatibility;
	final int avcLevelIndication;
	final int lengthSize;
	final List<typed.Uint8List> sequenceParameterSets;
	final List<typed.Uint8List> pictureParameterSets;
	final Mp4AtomAvccExt? ext;
	const Mp4AtomAvcc({required this.configurationVersion, required this.avcProfileIndication, required this.profileCompatibility, required this.avcLevelIndication, required this.lengthSize, required this.sequenceParameterSets, required this.pictureParameterSets, required this.ext});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAvcc.default_();
	factory Mp4AtomAvcc.new_(typed.Uint8List sps, typed.Uint8List pps);
}
final class Mp4AtomBtrt {
	final int bufferSizeDb;
	final int maxBitrate;
	final int avgBitrate;
	const Mp4AtomBtrt({required this.bufferSizeDb, required this.maxBitrate, required this.avgBitrate});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomBtrt.default_();
	factory Mp4AtomBtrt.new_(int bufferSizeDb, int maxBitrate, int avgBitrate);
}
final class Mp4AtomPasp {
	final int hSpacing;
	final int vSpacing;
	const Mp4AtomPasp({required this.hSpacing, required this.vSpacing});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPasp.default_();
	factory Mp4AtomPasp.new_(int hSpacing, int vSpacing);
}
final class Mp4AtomTaic {
	final U64 timeUncertainty;
	final int clockResolution;
	final int clockDriftRate;
	final Mp4AtomClockType clockType;
	const Mp4AtomTaic({required this.timeUncertainty, required this.clockResolution, required this.clockDriftRate, required this.clockType});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTaic.default_();
	factory Mp4AtomTaic.new_(U64 timeUncertainty, int clockResolution, int clockDriftRate, Mp4AtomClockType clockType);
}
final class Mp4AtomFiel {
	final int fieldCount;
	final int fieldOrder;
	const Mp4AtomFiel({required this.fieldCount, required this.fieldOrder});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFiel.default_();
	factory Mp4AtomFiel.new_(int fieldCount, int fieldOrder);
}
final class Mp4AtomHvcc {
	final int configurationVersion;
	final int generalProfileSpace;
	final bool generalTierFlag;
	final int generalProfileIdc;
	final ArrayU8x4 generalProfileCompatibilityFlags;
	final ArrayU8x6 generalConstraintIndicatorFlags;
	final int generalLevelIdc;
	final int minSpatialSegmentationIdc;
	final int parallelismType;
	final int chromaFormatIdc;
	final int bitDepthLumaMinus8;
	final int bitDepthChromaMinus8;
	final int avgFrameRate;
	final int constantFrameRate;
	final int numTemporalLayers;
	final bool temporalIdNested;
	final int lengthSizeMinusOne;
	final List<Mp4AtomHvcCArray> arrays;
	const Mp4AtomHvcc({required this.configurationVersion, required this.generalProfileSpace, required this.generalTierFlag, required this.generalProfileIdc, required this.generalProfileCompatibilityFlags, required this.generalConstraintIndicatorFlags, required this.generalLevelIdc, required this.minSpatialSegmentationIdc, required this.parallelismType, required this.chromaFormatIdc, required this.bitDepthLumaMinus8, required this.bitDepthChromaMinus8, required this.avgFrameRate, required this.constantFrameRate, required this.numTemporalLayers, required this.temporalIdNested, required this.lengthSizeMinusOne, required this.arrays});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHvcc.default_();
	factory Mp4AtomHvcc.new_();
}
final class Mp4AtomLhvc {
	final int configurationVersion;
	final int minSpatialSegmentationIdc;
	final int parallelismType;
	final int numTemporalLayers;
	final bool temporalIdNested;
	final int lengthSizeMinusOne;
	final List<Mp4AtomHvcCArray> arrays;
	const Mp4AtomLhvc({required this.configurationVersion, required this.minSpatialSegmentationIdc, required this.parallelismType, required this.numTemporalLayers, required this.temporalIdNested, required this.lengthSizeMinusOne, required this.arrays});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomLhvc.default_();
}
final class Mp4AtomCcst {
	final bool allRefPicsIntra;
	final bool intraPredUsed;
	final int maxRefPerPic;
	const Mp4AtomCcst({required this.allRefPicsIntra, required this.intraPredUsed, required this.maxRefPerPic});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCcst.default_();
}
final class Mp4AtomVpcC {
	final int profile;
	final int level;
	final int bitDepth;
	final int chromaSubsampling;
	final bool videoFullRangeFlag;
	final int colorPrimaries;
	final int transferCharacteristics;
	final int matrixCoefficients;
	final typed.Uint8List codecInitializationData;
	const Mp4AtomVpcC({required this.profile, required this.level, required this.bitDepth, required this.chromaSubsampling, required this.videoFullRangeFlag, required this.colorPrimaries, required this.transferCharacteristics, required this.matrixCoefficients, required this.codecInitializationData});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVpcC.default_();
}
final class Mp4AtomAv1c {
	final int seqProfile;
	final int seqLevelIdx0;
	final bool seqTier0;
	final bool highBitdepth;
	final bool twelveBit;
	final bool monochrome;
	final bool chromaSubsamplingX;
	final bool chromaSubsamplingY;
	final int chromaSamplePosition;
	final int? initialPresentationDelay;
	final typed.Uint8List configObus;
	const Mp4AtomAv1c({required this.seqProfile, required this.seqLevelIdx0, required this.seqTier0, required this.highBitdepth, required this.twelveBit, required this.monochrome, required this.chromaSubsamplingX, required this.chromaSubsamplingY, required this.chromaSamplePosition, required this.initialPresentationDelay, required this.configObus});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAv1c.default_();
}
final class Mp4AtomEsds {
	final Mp4AtomEsdsEsDescriptor esDesc;
	const Mp4AtomEsds({required this.esDesc});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEsds.default_();
}
final class Mp4AtomRgbaColor {
	final int red;
	final int green;
	final int blue;
	final int alpha;
	const Mp4AtomRgbaColor({required this.red, required this.green, required this.blue, required this.alpha});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomRgbaColor.default_();
}
final class Mp4AtomFtab {
	final List<Mp4AtomFontEntry> fontEntries;
	const Mp4AtomFtab({required this.fontEntries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFtab.default_();
}
final class Mp4AtomDisp {
	final int disparityShiftIn16thPel;
	const Mp4AtomDisp({required this.disparityShiftIn16thPel});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomDisp.default_();
}
final class Mp4AtomDops {
	final int outputChannelCount;
	final int preSkip;
	final int inputSampleRate;
	final int outputGain;
	final Mp4AtomOpusChannelMapping? channelMapping;
	const Mp4AtomDops({required this.outputChannelCount, required this.preSkip, required this.inputSampleRate, required this.outputGain, required this.channelMapping});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomCmpd {
	final List<Mp4AtomComponent> components;
	const Mp4AtomCmpd({required this.components});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCmpd.default_();
}
final class Mp4AtomDfla {
	final List<Mp4AtomFlacMetadataBlock> blocks;
	const Mp4AtomDfla({required this.blocks});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomAc3SpecificBox {
	final int fscod;
	final int bsid;
	final int bsmod;
	final int acmod;
	final bool lfeon;
	final int bitRateCode;
	const Mp4AtomAc3SpecificBox({required this.fscod, required this.bsid, required this.bsmod, required this.acmod, required this.lfeon, required this.bitRateCode});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomEc3SpecificBox {
	final int dataRate;
	final List<Mp4AtomEc3IndependentSubstream> substreams;
	const Mp4AtomEc3SpecificBox({required this.dataRate, required this.substreams});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomPcmC {
	final bool bigEndian;
	final int sampleSize;
	const Mp4AtomPcmC({required this.bigEndian, required this.sampleSize});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomChnl {
	final Mp4AtomChannelStructure? channelStructure;
	final int? objectCount;
	final int? formatOrdering;
	final int? baseChannelCount;
	const Mp4AtomChnl({required this.channelStructure, required this.objectCount, required this.formatOrdering, required this.baseChannelCount});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomChnl.decodeBodyWithChannelCount(Mut<typed.Uint8List> buf, int channelCount);
}
final class Mp4AtomPlainText {
	final int dataReferenceIndex;
	const Mp4AtomPlainText({required this.dataReferenceIndex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomVttC {
	final String config;
	const Mp4AtomVttC({required this.config});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomVlab {
	final String sourceLabel;
	const Mp4AtomVlab({required this.sourceLabel});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomAmrSampleEntry {
	final int dataReferenceIndex;
	final int timescale;
	const Mp4AtomAmrSampleEntry({required this.dataReferenceIndex, required this.timescale});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomDamr {
	final Mp4AtomFourCc vendor;
	final int decoderVersion;
	final int modeSet;
	final int modeChangePeriod;
	final int framesPerSample;
	const Mp4AtomDamr({required this.vendor, required this.decoderVersion, required this.modeSet, required this.modeChangePeriod, required this.framesPerSample});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomTxtC {
	final String textConfig;
	const Mp4AtomTxtC({required this.textConfig});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUri {
	final String theUri;
	const Mp4AtomUri({required this.theUri});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUriI {
	final typed.Uint8List uriInitializationData;
	const Mp4AtomUriI({required this.uriInitializationData});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSilb {
	final List<Mp4AtomSchemeIdEntry> schemes;
	final bool otherSchemes;
	const Mp4AtomSilb({required this.schemes, required this.otherSchemes});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMebxKey {
	final Mp4AtomFourCc localKeyId;
	final Mp4AtomKeyd keyd;
	final Mp4AtomDtyp? dtyp;
	final Mp4AtomLoca? loca;
	final Mp4AtomSetu? setu;
	const Mp4AtomMebxKey({required this.localKeyId, required this.keyd, required this.dtyp, required this.loca, required this.setu});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomFixedPointI16 {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFixedPointI16.default_();
	factory Mp4AtomFixedPointI16(int int, int dec);
	int integer();
	int decimal();
}
final class Mp4AtomFixedPointI8 {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFixedPointI8.default_();
	factory Mp4AtomFixedPointI8(int int, int dec);
	int integer();
	int decimal();
}
final class Mp4AtomFixedPointU16 {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFixedPointU16.default_();
	factory Mp4AtomFixedPointU16(int int, int dec);
	int integer();
	int decimal();
}
final class Mp4AtomFixedPointU8 {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFixedPointU8.default_();
	factory Mp4AtomFixedPointU8(int int, int dec);
	int integer();
	int decimal();
}
final class Mp4AtomMatrix {
	final int a;
	final int b;
	final int u;
	final int c;
	final int d;
	final int v;
	final int x;
	final int y;
	final int w;
	const Mp4AtomMatrix({required this.a, required this.b, required this.u, required this.c, required this.d, required this.v, required this.x, required this.y, required this.w});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMatrix.default_();
}
final class Mp4AtomElst {
	final List<Mp4AtomElstEntry> entries;
	const Mp4AtomElst({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomElst.default_();
}
final class Mp4AtomHdlr {
	final Mp4AtomFourCc handler;
	final String name;
	const Mp4AtomHdlr({required this.handler, required this.name});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHdlr.default_();
}
final class Mp4AtomMdhd {
	final U64 creationTime;
	final U64 modificationTime;
	final int timescale;
	final U64 duration;
	final String language;
	const Mp4AtomMdhd({required this.creationTime, required this.modificationTime, required this.timescale, required this.duration, required this.language});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMdhd.default_();
}
final class Mp4AtomTrackReferenceTypeBox {
	final Mp4AtomFourCc referenceType;
	final typed.Uint32List trackIds;
	const Mp4AtomTrackReferenceTypeBox({required this.referenceType, required this.trackIds});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomChpl {
	final List<Mp4AtomChplEntry> entries;
	const Mp4AtomChpl({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomChpl.default_();
}
final class Mp4AtomCprt {
	final String language;
	final String notice;
	const Mp4AtomCprt({required this.language, required this.notice});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCprt.default_();
}
final class Mp4AtomKind {
	final String schemeUri;
	final String value;
	const Mp4AtomKind({required this.schemeUri, required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomKind.default_();
}
final class Mp4AtomRtng {
	final Mp4AtomFourCc entity;
	final Mp4AtomFourCc criteria;
	final String language;
	final String ratingInfo;
	const Mp4AtomRtng({required this.entity, required this.criteria, required this.language, required this.ratingInfo});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomPcmFormat {
	final bool bigEndian;
	final int sampleSize;
	final bool float;
	const Mp4AtomPcmFormat({required this.bigEndian, required this.sampleSize, required this.float});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqQuicStreamId {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqQuicStreamId(MoqQuicSide initiator, MoqQuicStreamDir dir, U64 index);
	MoqQuicSide initiator();
	MoqQuicStreamDir dir();
	U64 index();
}
final class MoqQuicUdpStats {
	String debug();
	factory MoqQuicUdpStats.default_();
	U64 get datagramsGet;
	MoqQuicUdpStats datagramsSet(U64 value);
	U64 get bytesGet;
	MoqQuicUdpStats bytesSet(U64 value);
	U64 get iosGet;
	MoqQuicUdpStats iosSet(U64 value);
}
final class MoqQuicCreditStats {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqQuicCreditStats.default_();
	U64 get leasedGet;
	MoqQuicCreditStats leasedSet(U64 value);
	U64 get availableGet;
	MoqQuicCreditStats availableSet(U64 value);
	U64 get evictedGet;
	MoqQuicCreditStats evictedSet(U64 value);
	U64 get peakSpentGet;
	MoqQuicCreditStats peakSpentSet(U64 value);
}
final class Mp4AtomAvccExt {
	final int chromaFormat;
	final int bitDepthLuma;
	final int bitDepthChroma;
	final List<typed.Uint8List> sequenceParameterSetsExt;
	const Mp4AtomAvccExt({required this.chromaFormat, required this.bitDepthLuma, required this.bitDepthChroma, required this.sequenceParameterSetsExt});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAvccExt.default_();
}
final class Mp4AtomSt3d {
	final int stereoMode;
	const Mp4AtomSt3d({required this.stereoMode});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSt3d.default_();
}
final class Mp4AtomHfov {
	final int fieldOfView;
	const Mp4AtomHfov({required this.fieldOfView});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHfov.default_();
}
final class Mp4AtomHvcCArray {
	final bool completeness;
	final int nalUnitType;
	final List<typed.Uint8List> nalus;
	const Mp4AtomHvcCArray({required this.completeness, required this.nalUnitType, required this.nalus});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHvcCArray.default_();
}
final class Mp4AtomEsdsEsDescriptor {
	final int esId;
	final int? dependsOnEsId;
	final typed.Uint8List? url;
	final int? ocrEsId;
	final int streamPriority;
	final Mp4AtomEsdsDecoderConfig decConfig;
	final Mp4AtomEsdsSlConfig slConfig;
	const Mp4AtomEsdsEsDescriptor({required this.esId, required this.dependsOnEsId, required this.url, required this.ocrEsId, required this.streamPriority, required this.decConfig, required this.slConfig});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEsdsEsDescriptor.default_();
}
final class Mp4AtomFontEntry {
	final int fontId;
	final String font;
	const Mp4AtomFontEntry({required this.fontId, required this.font});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFontEntry.default_();
}
final class Mp4AtomOpusChannelMapping {
	final int family;
	final int streamCount;
	final int coupledCount;
	final typed.Uint8List channelMapping;
	const Mp4AtomOpusChannelMapping({required this.family, required this.streamCount, required this.coupledCount, required this.channelMapping});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomComponent {
	final int componentType;
	final String? componentTypeUri;
	const Mp4AtomComponent({required this.componentType, required this.componentTypeUri});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUncompressedComponent {
	final int componentIndex;
	final int componentBitDepthMinusOne;
	final int componentFormat;
	final int componentAlignSize;
	const Mp4AtomUncompressedComponent({required this.componentIndex, required this.componentBitDepthMinusOne, required this.componentFormat, required this.componentAlignSize});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomEc3IndependentSubstream {
	final int fscod;
	final int bsid;
	final bool asvc;
	final int bsmod;
	final int acmod;
	final bool lfeon;
	final int numDepSub;
	final int? chanLoc;
	const Mp4AtomEc3IndependentSubstream({required this.fscod, required this.bsid, required this.asvc, required this.bsmod, required this.acmod, required this.lfeon, required this.numDepSub, required this.chanLoc});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSchemeIdEntry {
	final String schemeIdUri;
	final String value;
	final bool atleastOnce;
	const Mp4AtomSchemeIdEntry({required this.schemeIdUri, required this.value, required this.atleastOnce});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomKeyd {
	final Mp4AtomFourCc keyNamespace;
	final typed.Uint8List keyValue;
	const Mp4AtomKeyd({required this.keyNamespace, required this.keyValue});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomDtyp {
	final Mp4AtomDataType dataType;
	const Mp4AtomDtyp({required this.dataType});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomLoca {
	final String locale;
	const Mp4AtomLoca({required this.locale});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSetu {
	final typed.Uint8List data;
	const Mp4AtomSetu({required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomElstEntry {
	final U64 segmentDuration;
	final U64? mediaTime;
	final Mp4AtomFixedPointI16 mediaRate;
	const Mp4AtomElstEntry({required this.segmentDuration, required this.mediaTime, required this.mediaRate});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomElstEntry.default_();
}
final class Mp4AtomFtyp {
	final Mp4AtomFourCc majorBrand;
	final int minorVersion;
	final List<Mp4AtomFourCc> compatibleBrands;
	const Mp4AtomFtyp({required this.majorBrand, required this.minorVersion, required this.compatibleBrands});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomStyp {
	final Mp4AtomFourCc majorBrand;
	final int minorVersion;
	final List<Mp4AtomFourCc> compatibleBrands;
	const Mp4AtomStyp({required this.majorBrand, required this.minorVersion, required this.compatibleBrands});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomPitm {
	final int itemId;
	const Mp4AtomPitm({required this.itemId});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPitm.default_();
}
final class Mp4AtomIloc {
	final List<Mp4AtomItemLocation> itemLocations;
	const Mp4AtomIloc({required this.itemLocations});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIinf {
	final List<Mp4AtomItemInfoEntry> itemInfos;
	const Mp4AtomIinf({required this.itemInfos});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomAuxc {
	final String auxType;
	final typed.Uint8List auxSubtype;
	const Mp4AtomAuxc({required this.auxType, required this.auxSubtype});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAuxc.default_();
}
final class Mp4AtomClap {
	final int cleanApertureWidthN;
	final int cleanApertureWidthD;
	final int cleanApertureHeightN;
	final int cleanApertureHeightD;
	final int horizOffN;
	final int horizOffD;
	final int vertOffN;
	final int vertOffD;
	const Mp4AtomClap({required this.cleanApertureWidthN, required this.cleanApertureWidthD, required this.cleanApertureHeightN, required this.cleanApertureHeightD, required this.horizOffN, required this.horizOffD, required this.vertOffN, required this.vertOffD});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomClap.default_();
}
final class Mp4AtomImir {
	final int axis;
	const Mp4AtomImir({required this.axis});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomImir.default_();
}
final class Mp4AtomIrot {
	final int angle;
	const Mp4AtomIrot({required this.angle});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIrot.default_();
}
final class Mp4AtomIscl {
	final int targetWidthNumerator;
	final int targetWidthDenominator;
	final int targetHeightNumerator;
	final int targetHeightDenominator;
	const Mp4AtomIscl({required this.targetWidthNumerator, required this.targetWidthDenominator, required this.targetHeightNumerator, required this.targetHeightDenominator});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIscl.default_();
}
final class Mp4AtomIspe {
	final int width;
	final int height;
	const Mp4AtomIspe({required this.width, required this.height});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIspe.default_();
}
final class Mp4AtomPixi {
	final typed.Uint8List bitsPerChannel;
	const Mp4AtomPixi({required this.bitsPerChannel});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPixi.default_();
}
final class Mp4AtomRref {
	final List<Mp4AtomFourCc> referenceTypes;
	const Mp4AtomRref({required this.referenceTypes});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomRref.default_();
}
final class Mp4AtomIpma {
	final List<Mp4AtomPropertyAssociations> itemProperties;
	const Mp4AtomIpma({required this.itemProperties});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIpma.default_();
}
final class Mp4AtomIref {
	final List<Mp4AtomReference> references;
	const Mp4AtomIref({required this.references});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIdat {
	final typed.Uint8List data;
	const Mp4AtomIdat({required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIlst {
	final Mp4AtomName? name;
	final Mp4AtomYear? year;
	final Mp4AtomCovr? covr;
	final Mp4AtomDesc? desc;
	final Mp4AtomTool? ctoo;
	final Mp4AtomCopyright? cprt;
	final List<(int, Mp4AtomIlstData)> mdta;
	const Mp4AtomIlst({required this.name, required this.year, required this.covr, required this.desc, required this.ctoo, required this.cprt, required this.mdta});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIlst.default_();
}
final class Mp4AtomCovr {
	final typed.Uint8List $0;
	const Mp4AtomCovr(this.$0);
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomDesc {
	final int countryIndicator;
	final int languageIndicator;
	final String text;
	const Mp4AtomDesc({required this.countryIndicator, required this.languageIndicator, required this.text});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomTool {
	final int countryIndicator;
	final int languageIndicator;
	final String text;
	const Mp4AtomTool({required this.countryIndicator, required this.languageIndicator, required this.text});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTool.new_(int countryIndicator, int languageIndicator, String text);
}
final class Mp4AtomName {
	final String $0;
	const Mp4AtomName(this.$0);
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomYear {
	final String $0;
	const Mp4AtomYear(this.$0);
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomKeys {
	final List<Mp4AtomKeyEntry> entries;
	const Mp4AtomKeys({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomKeys.default_();
}
final class Mp4AtomMvhd {
	final U64 creationTime;
	final U64 modificationTime;
	final int timescale;
	final U64 duration;
	final Mp4AtomFixedPointU16 rate;
	final Mp4AtomFixedPointU8 volume;
	final Mp4AtomMatrix matrix;
	final int nextTrackId;
	const Mp4AtomMvhd({required this.creationTime, required this.modificationTime, required this.timescale, required this.duration, required this.rate, required this.volume, required this.matrix, required this.nextTrackId});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMvhd.default_();
}
final class Mp4AtomAinf {
	final bool hidden;
	final int profileVersion;
	final String apid;
	const Mp4AtomAinf({required this.hidden, required this.profileVersion, required this.apid});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAinf.default_();
}
final class Mp4AtomSkip {
	final Mp4AtomZeroed zeroed;
	const Mp4AtomSkip({required this.zeroed});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSkip.default_();
}
final class Mp4AtomStts {
	final List<Mp4AtomSttsEntry> entries;
	const Mp4AtomStts({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStts.default_();
}
final class Mp4AtomStsc {
	final List<Mp4AtomStscEntry> entries;
	const Mp4AtomStsc({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStsc.default_();
}
final class Mp4AtomStsz {
	final Mp4AtomStszSamples samples;
	const Mp4AtomStsz({required this.samples});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStsz.default_();
}
final class Mp4AtomStss {
	final typed.Uint32List entries;
	const Mp4AtomStss({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStss.default_();
}
final class Mp4AtomStco {
	final typed.Uint32List entries;
	const Mp4AtomStco({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStco.default_();
}
final class Mp4AtomCo64 {
	final List<U64> entries;
	const Mp4AtomCo64({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCo64.default_();
}
final class Mp4AtomCslg {
	final int compositionToDtsShift;
	final int leastDecodeToDisplayShift;
	final int greatestDecodeToDisplayDelta;
	final int compositionStartTime;
	final int compositionEndTime;
	const Mp4AtomCslg({required this.compositionToDtsShift, required this.leastDecodeToDisplayShift, required this.greatestDecodeToDisplayDelta, required this.compositionStartTime, required this.compositionEndTime});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCslg.default_();
}
final class Mp4AtomCtts {
	final List<Mp4AtomCttsEntry> entries;
	const Mp4AtomCtts({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCtts.default_();
}
final class Mp4AtomSbgp {
	final Mp4AtomFourCc groupingType;
	final int? groupingTypeParameter;
	final List<Mp4AtomSbgpEntry> entries;
	const Mp4AtomSbgp({required this.groupingType, required this.groupingTypeParameter, required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSgpd {
	final Mp4AtomFourCc groupingType;
	final int? defaultLength;
	final int? defaultGroupDescriptionIndex;
	final bool staticGroupDescription;
	final bool staticMapping;
	final bool essential;
	final List<Mp4AtomSgpdEntry> entries;
	const Mp4AtomSgpd({required this.groupingType, required this.defaultLength, required this.defaultGroupDescriptionIndex, required this.staticGroupDescription, required this.staticMapping, required this.essential, required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSubs {
	final ArrayU8x3 flags;
	final List<Mp4AtomSubsEntry> entries;
	const Mp4AtomSubs({required this.flags, required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSubs.default_();
}
final class Mp4AtomSaio {
	final Mp4AtomAuxInfo? auxInfo;
	final List<U64> offsets;
	const Mp4AtomSaio({required this.auxInfo, required this.offsets});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSaio.default_();
}
final class Mp4AtomSaiz {
	final Mp4AtomAuxInfo? auxInfo;
	final int defaultSampleInfoSize;
	final int sampleCount;
	final typed.Uint8List sampleInfoSize;
	const Mp4AtomSaiz({required this.auxInfo, required this.defaultSampleInfoSize, required this.sampleCount, required this.sampleInfoSize});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSaiz.default_();
}
final class Mp4AtomDinf {
	final Mp4AtomDref dref;
	const Mp4AtomDinf({required this.dref});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomDinf.default_();
}
final class Mp4AtomDref {
	final List<Mp4AtomDrefEntry> entries;
	const Mp4AtomDref({required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomDref.default_();
}
final class Mp4AtomHmhd {
	final int maxPduSize;
	final int avgPduSize;
	final int maxBitrate;
	final int avgBitrate;
	const Mp4AtomHmhd({required this.maxPduSize, required this.avgPduSize, required this.maxBitrate, required this.avgBitrate});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHmhd.default_();
}
final class Mp4AtomSmhd {
	final Mp4AtomFixedPointI8 balance;
	const Mp4AtomSmhd({required this.balance});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSmhd.default_();
}
final class Mp4AtomVmhd {
	final int graphicsMode;
	final Mp4AtomRgbColor opColor;
	const Mp4AtomVmhd({required this.graphicsMode, required this.opColor});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVmhd.default_();
}
final class Mp4AtomMvex {
	final Mp4AtomMehd? mehd;
	final List<Mp4AtomTrex> trex;
	const Mp4AtomMvex({required this.mehd, required this.trex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMvex.default_();
}
final class Mp4AtomMehd {
	final U64 fragmentDuration;
	const Mp4AtomMehd({required this.fragmentDuration});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMehd.default_();
}
final class Mp4AtomTrex {
	final int trackId;
	final int defaultSampleDescriptionIndex;
	final int defaultSampleDuration;
	final int defaultSampleSize;
	final int defaultSampleFlags;
	const Mp4AtomTrex({required this.trackId, required this.defaultSampleDescriptionIndex, required this.defaultSampleDuration, required this.defaultSampleSize, required this.defaultSampleFlags});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTrex.default_();
}
final class Mp4AtomEmsg {
	final int timescale;
	final Mp4AtomEmsgTimestamp presentationTime;
	final int eventDuration;
	final int id;
	final String schemeIdUri;
	final String value;
	final typed.Uint8List messageData;
	const Mp4AtomEmsg({required this.timescale, required this.presentationTime, required this.eventDuration, required this.id, required this.schemeIdUri, required this.value, required this.messageData});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomMfhd {
	final int sequenceNumber;
	const Mp4AtomMfhd({required this.sequenceNumber});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMfhd.default_();
}
final class Mp4AtomTfhd {
	final int trackId;
	final U64? baseDataOffset;
	final int? sampleDescriptionIndex;
	final int? defaultSampleDuration;
	final int? defaultSampleSize;
	final int? defaultSampleFlags;
	final bool durationIsEmpty;
	final bool defaultBaseIsMoof;
	const Mp4AtomTfhd({required this.trackId, required this.baseDataOffset, required this.sampleDescriptionIndex, required this.defaultSampleDuration, required this.defaultSampleSize, required this.defaultSampleFlags, required this.durationIsEmpty, required this.defaultBaseIsMoof});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTfhd.default_();
}
final class Mp4AtomTfdt {
	final U64 baseMediaDecodeTime;
	const Mp4AtomTfdt({required this.baseMediaDecodeTime});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTfdt.default_();
}
final class Mp4AtomTrun {
	final int? dataOffset;
	final Mp4AtomTrunSamples samples;
	const Mp4AtomTrun({required this.dataOffset, required this.samples});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTrun.default_();
}
final class Mp4AtomMdat {
	final typed.Uint8List data;
	const Mp4AtomMdat({required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomFree {
	final Mp4AtomZeroed zeroed;
	const Mp4AtomFree({required this.zeroed});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSidx {
	final int referenceId;
	final int timescale;
	final U64 earliestPresentationTime;
	final U64 firstOffset;
	final List<Mp4AtomSegmentReference> references;
	const Mp4AtomSidx({required this.referenceId, required this.timescale, required this.earliestPresentationTime, required this.firstOffset, required this.references});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSidx.default_();
}
final class Mp4AtomPrft {
	final int referenceTrackId;
	final U64 ntpTimestamp;
	final U64 mediaTime;
	final Mp4AtomReferenceTime utcTimeSemantics;
	const Mp4AtomPrft({required this.referenceTrackId, required this.ntpTimestamp, required this.mediaTime, required this.utcTimeSemantics});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPrft.default_();
}
final class Mp4AtomMfra {
	final List<Mp4AtomTfra> tfra;
	final Mp4AtomMfro mfro;
	const Mp4AtomMfra({required this.tfra, required this.mfro});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMfra.default_();
}
final class Mp4AtomTfra {
	final int trackId;
	final List<Mp4AtomFragmentInfo> entries;
	const Mp4AtomTfra({required this.trackId, required this.entries});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTfra.default_();
}
final class Mp4AtomMfro {
	final int parentSize;
	const Mp4AtomMfro({required this.parentSize});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMfro.default_();
}
final class Mp4AtomChplEntry {
	final U64 start;
	final String title;
	const Mp4AtomChplEntry({required this.start, required this.title});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSvhd {
	final String metadataSource;
	const Mp4AtomSvhd({required this.metadataSource});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSvhd.default_();
}
final class Mp4AtomMust {
	final List<Mp4AtomFourCc> requiredBoxTypes;
	const Mp4AtomMust({required this.requiredBoxTypes});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMust.default_();
}
final class Mp4AtomEsdsDecoderConfig {
	final int objectTypeIndication;
	final int streamType;
	final int upStream;
	final Mp4AtomU24 bufferSizeDb;
	final int maxBitrate;
	final int avgBitrate;
	final Mp4AtomEsdsDecoderSpecific? decSpecific;
	const Mp4AtomEsdsDecoderConfig({required this.objectTypeIndication, required this.streamType, required this.upStream, required this.bufferSizeDb, required this.maxBitrate, required this.avgBitrate, required this.decSpecific});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEsdsDecoderConfig.default_();
}
final class Mp4AtomEsdsSlConfig {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEsdsSlConfig.default_();
}
final class Mp4AtomU24 {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomU24.default_();
}
final class Mp4AtomItemLocation {
	final int itemId;
	final int constructionMethod;
	final int dataReferenceIndex;
	final U64 baseOffset;
	final List<Mp4AtomItemLocationExtent> extents;
	const Mp4AtomItemLocation({required this.itemId, required this.constructionMethod, required this.dataReferenceIndex, required this.baseOffset, required this.extents});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomItemInfoEntry {
	final int itemId;
	final int itemProtectionIndex;
	final Mp4AtomFourCc? itemType;
	final String itemName;
	final String? contentType;
	final String? contentEncoding;
	final String? itemUriType;
	final bool itemNotInPresentation;
	const Mp4AtomItemInfoEntry({required this.itemId, required this.itemProtectionIndex, required this.itemType, required this.itemName, required this.contentType, required this.contentEncoding, required this.itemUriType, required this.itemNotInPresentation});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomPropertyAssociations {
	final int itemId;
	final List<Mp4AtomPropertyAssociation> associations;
	const Mp4AtomPropertyAssociations({required this.itemId, required this.associations});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPropertyAssociations.default_();
}
final class Mp4AtomReference {
	final Mp4AtomFourCc referenceType;
	final int fromItemId;
	final typed.Uint32List toItemIds;
	const Mp4AtomReference({required this.referenceType, required this.fromItemId, required this.toItemIds});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomCopyright {
	final int countryIndicator;
	final int languageIndicator;
	final String text;
	const Mp4AtomCopyright({required this.countryIndicator, required this.languageIndicator, required this.text});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIlstData {
	final int countryIndicator;
	final int languageIndicator;
	final Mp4AtomIlstDataValue value;
	const Mp4AtomIlstData({required this.countryIndicator, required this.languageIndicator, required this.value});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomKeyEntry {
	final Mp4AtomFourCc keyNamespace;
	final String keyValue;
	const Mp4AtomKeyEntry({required this.keyNamespace, required this.keyValue});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomZeroed {
	final U64 size;
	const Mp4AtomZeroed({required this.size});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomZeroed.default_();
	factory Mp4AtomZeroed.new_(U64 size);
}
final class Mp4AtomSttsEntry {
	final int sampleCount;
	final int sampleDelta;
	const Mp4AtomSttsEntry({required this.sampleCount, required this.sampleDelta});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSttsEntry.default_();
}
final class Mp4AtomStscEntry {
	final int firstChunk;
	final int samplesPerChunk;
	final int sampleDescriptionIndex;
	const Mp4AtomStscEntry({required this.firstChunk, required this.samplesPerChunk, required this.sampleDescriptionIndex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStscEntry.default_();
}
final class Mp4AtomCttsEntry {
	final int sampleCount;
	final int sampleOffset;
	const Mp4AtomCttsEntry({required this.sampleCount, required this.sampleOffset});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCttsEntry.default_();
}
final class Mp4AtomSbgpEntry {
	final int sampleCount;
	final int groupDescriptionIndex;
	const Mp4AtomSbgpEntry({required this.sampleCount, required this.groupDescriptionIndex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSgpdEntry {
	final int? descriptionLength;
	final Mp4AtomAnySampleGroupEntry entry;
	const Mp4AtomSgpdEntry({required this.descriptionLength, required this.entry});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSubsEntry {
	final int sampleDelta;
	final List<Mp4AtomSubsSubsample> subsamples;
	const Mp4AtomSubsEntry({required this.sampleDelta, required this.subsamples});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSubsEntry.default_();
}
final class Mp4AtomAuxInfo {
	final Mp4AtomFourCc auxInfoType;
	final int auxInfoTypeParameter;
	const Mp4AtomAuxInfo({required this.auxInfoType, required this.auxInfoTypeParameter});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomRgbColor {
	final int red;
	final int green;
	final int blue;
	const Mp4AtomRgbColor({required this.red, required this.green, required this.blue});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomRgbColor.default_();
}
final class Mp4AtomSegmentReference {
	final bool referenceType;
	final int referencedSize;
	final int subsegmentDuration;
	final bool startsWithSap;
	final int sapType;
	final int sapDeltaTime;
	const Mp4AtomSegmentReference({required this.referenceType, required this.referencedSize, required this.subsegmentDuration, required this.startsWithSap, required this.sapType, required this.sapDeltaTime});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomFragmentInfo {
	final U64 time;
	final U64 moofOffset;
	final int trafNumber;
	final int trunNumber;
	final int sampleDelta;
	const Mp4AtomFragmentInfo({required this.time, required this.moofOffset, required this.trafNumber, required this.trunNumber, required this.sampleDelta});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomFragmentInfo.default_();
}
final class Mp4AtomC2pa {
	final String boxPurpose;
	final typed.Uint8List data;
	const Mp4AtomC2pa({required this.boxPurpose, required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomExtendedType {
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomExtendedType.default_();
	factory Mp4AtomExtendedType(ArrayU8x16 value);
}
final class Mp4AtomPrhd {
	final int poseYawDegrees;
	final int posePitchDegrees;
	final int poseRollDegrees;
	const Mp4AtomPrhd({required this.poseYawDegrees, required this.posePitchDegrees, required this.poseRollDegrees});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPrhd.default_();
}
final class Mp4AtomPrji {
	final Mp4AtomFourCc projectionKind;
	const Mp4AtomPrji({required this.projectionKind});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Mp4AtomFourCc get RECTILINEAR;
	static Mp4AtomFourCc get EQUIRECTANGULAR;
	static Mp4AtomFourCc get HALF_EQUIRECTANGULAR;
	static Mp4AtomFourCc get FISHEYE;
	static Mp4AtomFourCc get PARAMETRIC_IMMERSIVE;
}
final class Mp4AtomStri {
	final bool eyeViewsReversed;
	final bool hasAdditionalViews;
	final bool hasRightEyeView;
	final bool hasLeftEyeView;
	const Mp4AtomStri({required this.eyeViewsReversed, required this.hasAdditionalViews, required this.hasRightEyeView, required this.hasLeftEyeView});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStri.default_();
}
final class Mp4AtomHero {
	final int heroEyeIndicator;
	const Mp4AtomHero({required this.heroEyeIndicator});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHero.default_();
}
final class Mp4AtomPkin {
	final Mp4AtomFourCc viewPackingKind;
	const Mp4AtomPkin({required this.viewPackingKind});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Mp4AtomFourCc get SIDE_BY_SIDE;
	static Mp4AtomFourCc get OVER_UNDER;
}
final class Mp4AtomEsdsDecoderSpecific {
	final int profile;
	final int freqIndex;
	final int chanConf;
	final typed.Uint8List raw;
	const Mp4AtomEsdsDecoderSpecific({required this.profile, required this.freqIndex, required this.chanConf, required this.raw});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEsdsDecoderSpecific.default_();
}
final class Mp4AtomExplicitSpeakerPosition {
	final int azimuth;
	final int elevation;
	const Mp4AtomExplicitSpeakerPosition({required this.azimuth, required this.elevation});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomItemLocationExtent {
	final U64 itemReferenceIndex;
	final U64 offset;
	final U64 length;
	const Mp4AtomItemLocationExtent({required this.itemReferenceIndex, required this.offset, required this.length});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomPropertyAssociation {
	final bool essential;
	final int propertyIndex;
	const Mp4AtomPropertyAssociation({required this.essential, required this.propertyIndex});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPropertyAssociation.default_();
}
final class Mp4AtomSubsSubsample {
	final Mp4AtomSubsSubsampleSize size;
	final int priority;
	final bool discardable;
	final typed.Uint8List codecSpecificParameters;
	const Mp4AtomSubsSubsample({required this.size, required this.priority, required this.discardable, required this.codecSpecificParameters});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSubsSubsample.default_();
}
final class Mp4AtomUrl {
	final String location;
	const Mp4AtomUrl({required this.location});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomUrl.default_();
}
final class Mp4AtomUrn {
	final String name;
	final String location;
	const Mp4AtomUrn({required this.name, required this.location});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomUrn.default_();
}
final class Mp4AtomTrunEntry {
	final int? duration;
	final int? size;
	final int? flags;
	final int? cts;
	const Mp4AtomTrunEntry({required this.duration, required this.size, required this.flags, required this.cts});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTrunEntry.default_();
}
final class RustlsCryptoHpkeHpkePublicKey {
	final typed.Uint8List $0;
	const RustlsCryptoHpkeHpkePublicKey(this.$0);
	String debug();
}
final class Mp4AtomEqui {
	final int projectionBoundsTop;
	final int projectionBoundsBottom;
	final int projectionBoundsLeft;
	final int projectionBoundsRight;
	const Mp4AtomEqui({required this.projectionBoundsTop, required this.projectionBoundsBottom, required this.projectionBoundsLeft, required this.projectionBoundsRight});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEqui.default_();
}
final class Mp4AtomCbmp {
	final int layout;
	final int padding;
	const Mp4AtomCbmp({required this.layout, required this.padding});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCbmp.default_();
}
final class Mp4AtomMshp {
	final int crc;
	final Mp4AtomFourCc encodingFourCc;
	final typed.Uint8List data;
	const Mp4AtomMshp({required this.crc, required this.encodingFourCc, required this.data});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomBlin {
	final int baselineValue;
	const Mp4AtomBlin({required this.baselineValue});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomBlin.default_();
}
final class Mp4AtomDadj {
	final int disparityAdjustment;
	const Mp4AtomDadj({required this.disparityAdjustment});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomDadj.default_();
}
enum EncodeError implements Exception {
	boundsExceeded(0),
	tooLarge(1),
	short(2),
	invalidState(3),
	tooMany(4),
	version(5),
	unsupported(6),
	const EncodeError(this._c);
	@override String toString();
	String debug();
}
sealed class InvalidPattern implements Exception {
	const InvalidPattern();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class InvalidPatternEmptySegment extends InvalidPattern {
	const InvalidPatternEmptySegment();
}
final class InvalidPatternInvalidSegment extends InvalidPattern {
	final String $0;
	const InvalidPatternInvalidSegment(this.$0);
}
final class InvalidPatternMultipleGlobstars extends InvalidPattern {
	const InvalidPatternMultipleGlobstars();
}
final class InvalidPatternTooManySegments extends InvalidPattern {
	const InvalidPatternTooManySegments();
}
sealed class SessionError {
	const SessionError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int toCode();
	factory SessionError.fromCode(int code);
}
final class SessionErrorCancel extends SessionError {
	const SessionErrorCancel();
}
final class SessionErrorInternal extends SessionError {
	const SessionErrorInternal();
}
final class SessionErrorUnauthorized extends SessionError {
	const SessionErrorUnauthorized();
}
final class SessionErrorExpired extends SessionError {
	const SessionErrorExpired();
}
final class SessionErrorProtocolViolation extends SessionError {
	const SessionErrorProtocolViolation();
}
final class SessionErrorKeyValueFormatting extends SessionError {
	const SessionErrorKeyValueFormatting();
}
final class SessionErrorTooManyRequests extends SessionError {
	const SessionErrorTooManyRequests();
}
final class SessionErrorGoawayTimeout extends SessionError {
	const SessionErrorGoawayTimeout();
}
final class SessionErrorTimeout extends SessionError {
	const SessionErrorTimeout();
}
final class SessionErrorAuthTokenCacheOverflow extends SessionError {
	const SessionErrorAuthTokenCacheOverflow();
}
final class SessionErrorVersion extends SessionError {
	const SessionErrorVersion();
}
final class SessionErrorMalformedAuthToken extends SessionError {
	const SessionErrorMalformedAuthToken();
}
final class SessionErrorUnknownAuthTokenAlias extends SessionError {
	const SessionErrorUnknownAuthTokenAlias();
}
final class SessionErrorTooManyRequestUpdates extends SessionError {
	const SessionErrorTooManyRequestUpdates();
}
final class SessionErrorApp extends SessionError {
	final int $0;
	const SessionErrorApp(this.$0);
}
final class SessionErrorUnknown extends SessionError {
	final int $0;
	const SessionErrorUnknown(this.$0);
}
final class SessionErrorUnsupported extends SessionError {
	const SessionErrorUnsupported();
}
final class SessionErrorInvalidRequestId extends SessionError {
	const SessionErrorInvalidRequestId();
}
final class SessionErrorInvalidPath extends SessionError {
	const SessionErrorInvalidPath();
}
final class SessionErrorInvalidAuthority extends SessionError {
	const SessionErrorInvalidAuthority();
}
sealed class StreamError {
	const StreamError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int toCode();
	factory StreamError.fromCode(int code);
}
final class StreamErrorSession extends StreamError {
	final SessionError $0;
	const StreamErrorSession(this.$0);
}
final class StreamErrorInternal extends StreamError {
	const StreamErrorInternal();
}
final class StreamErrorCancel extends StreamError {
	const StreamErrorCancel();
}
final class StreamErrorDeliveryTimeout extends StreamError {
	const StreamErrorDeliveryTimeout();
}
final class StreamErrorControlTimeout extends StreamError {
	const StreamErrorControlTimeout();
}
final class StreamErrorGoingAway extends StreamError {
	const StreamErrorGoingAway();
}
final class StreamErrorTooFarBehind extends StreamError {
	const StreamErrorTooFarBehind();
}
final class StreamErrorMalformedTrack extends StreamError {
	const StreamErrorMalformedTrack();
}
final class StreamErrorNotFound extends StreamError {
	const StreamErrorNotFound();
}
final class StreamErrorUnroutable extends StreamError {
	const StreamErrorUnroutable();
}
final class StreamErrorOld extends StreamError {
	const StreamErrorOld();
}
final class StreamErrorEvicted extends StreamError {
	const StreamErrorEvicted();
}
final class StreamErrorWrongSize extends StreamError {
	const StreamErrorWrongSize();
}
final class StreamErrorFrameTooLarge extends StreamError {
	const StreamErrorFrameTooLarge();
}
final class StreamErrorGroupTooLarge extends StreamError {
	const StreamErrorGroupTooLarge();
}
final class StreamErrorTimestampMismatch extends StreamError {
	const StreamErrorTimestampMismatch();
}
final class StreamErrorUnauthorized extends StreamError {
	const StreamErrorUnauthorized();
}
final class StreamErrorApp extends StreamError {
	final int $0;
	const StreamErrorApp(this.$0);
}
final class StreamErrorUnknown extends StreamError {
	final int $0;
	const StreamErrorUnknown(this.$0);
}
enum InvalidHop implements Exception {
	range(0),
	tooMany(1),
	duplicate(2),
	const InvalidHop(this._c);
	@override String toString();
	String debug();
}
enum IntersectionError implements Exception {
	tooManyPatterns(0),
	const IntersectionError(this._c);
	@override String toString();
	String debug();
}
sealed class Segment {
	const Segment();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class SegmentLiteral extends Segment {
	final String $0;
	const SegmentLiteral(this.$0);
}
final class SegmentWildcard extends Segment {
	const SegmentWildcard();
}
final class SegmentPartial extends Segment {
	final String prefix;
	final String suffix;
	const SegmentPartial({required this.prefix, required this.suffix});
}
final class SegmentGlobstar extends Segment {
	const SegmentGlobstar();
}
enum ClockMissedTickBehavior {
	burst(0),
	delay(1),
	skip(2),
	const ClockMissedTickBehavior(this._c);
	String debug();
	factory ClockMissedTickBehavior.default_();
}
enum TokioConnectError {
	unauthorized(0),
	forbidden(1),
	const TokioConnectError(this._c);
	@override String toString();
	String debug();
	bool isAuth();
}
enum TokioRestart {
	redial(0),
	wait(1),
	const TokioRestart(this._c);
	String debug();
	factory TokioRestart.default_();
}
enum TokioStatus {
	connected(0),
	disconnected(1),
	migrating(2),
	const TokioStatus(this._c);
	String debug();
}
enum TokioTransport {
	quic(0),
	webSocket(1),
	tcp(2),
	unix(3),
	webTransport(4),
	webRtc(5),
	const TokioTransport(this._c);
	@override String toString();
	String debug();
	String asStr();
}
enum AuthAnswer {
	granted(0),
	requests(1),
	const AuthAnswer(this._c);
	String debug();
}
enum AuthRequestKind {
	subscribe(0),
	publishNamespace(1),
	discover(2),
	publish(3),
	const AuthRequestKind(this._c);
	String debug();
}
enum PathError implements Exception {
	tooLong(0),
	emptySegment(1),
	invalidEscape(2),
	const PathError(this._c);
	@override String toString();
	String debug();
}
enum SetupRole {
	publisher(0),
	subscriber(1),
	const SetupRole(this._c);
	@override String toString();
	String debug();
	String asStr();
}
enum StatsRole {
	publisher(0),
	subscriber(1),
	const StatsRole(this._c);
	String debug();
	String asStr();
}
enum StatsMessage {
	session(0),
	announce(1),
	trackInfo(2),
	subscribe(3),
	fetch(4),
	probe(5),
	auth(6),
	goaway(7),
	other(8),
	const StatsMessage(this._c);
	String debug();
	String asStr();
}
enum StatsFailure {
	idleTimeout(0),
	transport(1),
	application(2),
	expired(3),
	const StatsFailure(this._c);
	String debug();
	static List<StatsFailure> get ALL;
	String asStr();
}
enum StatsRefusalKind {
	refused(0),
	unavailable(1),
	request(2),
	forbidden(3),
	lan(4),
	expired(5),
	const StatsRefusalKind(this._c);
	String debug();
	static List<StatsRefusalKind> get ALL;
	String asStr();
}
enum TransportAddress {
	uri(0),
	setup(1),
	const TransportAddress(this._c);
	String debug();
}
enum TransportCloseKind {
	transport(0),
	idleTimeout(1),
	closed(2),
	evicted(3),
	const TransportCloseKind(this._c);
	String debug();
	factory TransportCloseKind.default_();
}
sealed class TrackAnchor {
	const TrackAnchor();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class TrackAnchorAbsolute extends TrackAnchor {
	final U64 $0;
	const TrackAnchorAbsolute(this.$0);
}
final class TrackAnchorRelative extends TrackAnchor {
	final U64 $0;
	const TrackAnchorRelative(this.$0);
}
final class TrackAnchorTime extends TrackAnchor {
	final Timestamp $0;
	const TrackAnchorTime(this.$0);
}
enum TrackOrder {
	ascending(0),
	descending(1),
	const TrackOrder(this._c);
	String debug();
	factory TrackOrder.default_();
}
enum OriginLink {
	local(0),
	peer(1),
	upstream(2),
	const OriginLink(this._c);
	String debug();
	factory OriginLink.default_();
}
sealed class OriginSource {
	const OriginSource();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginSource.default_();
}
final class OriginSourceLocal extends OriginSource {
	const OriginSourceLocal();
}
final class OriginSourcePeer extends OriginSource {
	final Hop $0;
	const OriginSourcePeer(this.$0);
}
enum HangHealthVerdict {
	unknown(0),
	healthy(1),
	degraded(2),
	unhealthy(3),
	const HangHealthVerdict(this._c);
	String debug();
	factory HangHealthVerdict.default_();
}
enum HangHealthRole {
	publisher(0),
	viewer(1),
	const HangHealthRole(this._c);
	String debug();
}
enum HangHealthReset {
	decrease(0),
	replaced(1),
	const HangHealthReset(this._c);
	String debug();
}
enum HangHealthInput {
	rtt(0),
	loss(1),
	headroom(2),
	skipped(3),
	late(4),
	errors(5),
	stalled(6),
	const HangHealthInput(this._c);
	String debug();
}
sealed class HangCatalogAudioCodec {
	const HangCatalogAudioCodec();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangCatalogAudioCodecKind kind();
}
final class HangCatalogAudioCodecAAC extends HangCatalogAudioCodec {
	final HangCatalogAac $0;
	const HangCatalogAudioCodecAAC(this.$0);
}
final class HangCatalogAudioCodecOpus extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecOpus();
}
final class HangCatalogAudioCodecPcm extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecPcm();
}
final class HangCatalogAudioCodecFlac extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecFlac();
}
final class HangCatalogAudioCodecMp3 extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecMp3();
}
final class HangCatalogAudioCodecMp2 extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecMp2();
}
final class HangCatalogAudioCodecAc3 extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecAc3();
}
final class HangCatalogAudioCodecEc3 extends HangCatalogAudioCodec {
	const HangCatalogAudioCodecEc3();
}
final class HangCatalogAudioCodecUnknown extends HangCatalogAudioCodec {
	final String $0;
	const HangCatalogAudioCodecUnknown(this.$0);
}
enum HangCatalogAudioCodecKind {
	aac(0),
	opus(1),
	unknown(2),
	flac(3),
	mp3(4),
	pcm(5),
	ac3(6),
	ec3(7),
	const HangCatalogAudioCodecKind(this._c);
	String debug();
}
sealed class HangCatalogAudioLayout {
	const HangCatalogAudioLayout();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogAudioLayoutAmbisonic extends HangCatalogAudioLayout {
	const HangCatalogAudioLayoutAmbisonic();
}
final class HangCatalogAudioLayoutUnknown extends HangCatalogAudioLayout {
	final String $0;
	const HangCatalogAudioLayoutUnknown(this.$0);
}
sealed class HangCatalogMode {
	const HangCatalogMode();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogModeSnapshot extends HangCatalogMode {
	const HangCatalogModeSnapshot();
}
final class HangCatalogModeStream extends HangCatalogMode {
	const HangCatalogModeStream();
}
final class HangCatalogModeUnknown extends HangCatalogMode {
	final String $0;
	const HangCatalogModeUnknown(this.$0);
}
sealed class HangCatalogTextFormat {
	const HangCatalogTextFormat();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogTextFormat.default_();
}
final class HangCatalogTextFormatVtt extends HangCatalogTextFormat {
	const HangCatalogTextFormatVtt();
}
final class HangCatalogTextFormatTtml extends HangCatalogTextFormat {
	const HangCatalogTextFormatTtml();
}
final class HangCatalogTextFormatUtf8 extends HangCatalogTextFormat {
	const HangCatalogTextFormatUtf8();
}
final class HangCatalogTextFormatUnknown extends HangCatalogTextFormat {
	final String $0;
	const HangCatalogTextFormatUnknown(this.$0);
}
sealed class HangCatalogTextRole {
	const HangCatalogTextRole();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogTextRole.default_();
}
final class HangCatalogTextRoleSubtitle extends HangCatalogTextRole {
	const HangCatalogTextRoleSubtitle();
}
final class HangCatalogTextRoleCaption extends HangCatalogTextRole {
	const HangCatalogTextRoleCaption();
}
final class HangCatalogTextRoleUnknown extends HangCatalogTextRole {
	final String $0;
	const HangCatalogTextRoleUnknown(this.$0);
}
sealed class HangCatalogVideoCodec {
	const HangCatalogVideoCodec();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangCatalogVideoCodecKind kind();
}
final class HangCatalogVideoCodecH264 extends HangCatalogVideoCodec {
	final HangCatalogH264 $0;
	const HangCatalogVideoCodecH264(this.$0);
}
final class HangCatalogVideoCodecH265 extends HangCatalogVideoCodec {
	final HangCatalogH265 $0;
	const HangCatalogVideoCodecH265(this.$0);
}
final class HangCatalogVideoCodecVP9 extends HangCatalogVideoCodec {
	final HangCatalogVp9 $0;
	const HangCatalogVideoCodecVP9(this.$0);
}
final class HangCatalogVideoCodecAV1 extends HangCatalogVideoCodec {
	final HangCatalogAv1 $0;
	const HangCatalogVideoCodecAV1(this.$0);
}
final class HangCatalogVideoCodecVP8 extends HangCatalogVideoCodec {
	const HangCatalogVideoCodecVP8();
}
final class HangCatalogVideoCodecUnknown extends HangCatalogVideoCodec {
	final String $0;
	const HangCatalogVideoCodecUnknown(this.$0);
}
enum HangCatalogVideoCodecKind {
	h264(0),
	h265(1),
	vp8(2),
	vp9(3),
	av1(4),
	unknown(5),
	const HangCatalogVideoCodecKind(this._c);
	String debug();
}
enum HangCatalogRange {
	limited(0),
	full(1),
	const HangCatalogRange(this._c);
	String debug();
}
sealed class HangCatalogStereo {
	const HangCatalogStereo();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class HangCatalogStereoMono extends HangCatalogStereo {
	const HangCatalogStereoMono();
}
final class HangCatalogStereoLeftRight extends HangCatalogStereo {
	const HangCatalogStereoLeftRight();
}
final class HangCatalogStereoTopBottom extends HangCatalogStereo {
	const HangCatalogStereoTopBottom();
}
final class HangCatalogStereoUnknown extends HangCatalogStereo {
	final String $0;
	const HangCatalogStereoUnknown(this.$0);
}
enum MuxCatalogCatalogFormat {
	hang(0),
	msf(1),
	const MuxCatalogCatalogFormat(this._c);
	String debug();
	factory MuxCatalogCatalogFormat.default_();
	static MuxCatalogCatalogFormat get DEFAULT;
	String extension();
	static MuxCatalogCatalogFormat? detect(String name);
}
enum MuxContainerKind {
	audio(0),
	video(1),
	data(2),
	const MuxContainerKind(this._c);
	String debug();
}
sealed class MuxContainerRead {
	const MuxContainerRead();
	String debug();
}
final class MuxContainerReadFrame extends MuxContainerRead {
	final U64 group;
	final MuxContainerFrame frame;
	const MuxContainerReadFrame({required this.group, required this.frame});
}
final class MuxContainerReadGap extends MuxContainerRead {
	final Timestamp timestamp;
	final Timestamp duration;
	const MuxContainerReadGap({required this.timestamp, required this.duration});
}
final class MuxContainerReadEnd extends MuxContainerRead {
	final U64 group;
	final Timestamp end;
	const MuxContainerReadEnd({required this.group, required this.end});
}
enum MuxImportAudioFormat {
	aac(0),
	opus(1),
	flac(2),
	mp3(3),
	const MuxImportAudioFormat(this._c);
	@override String toString();
	String debug();
}
enum MuxImportContainerFormat {
	fmp4(0),
	mkv(1),
	ts(2),
	flv(3),
	const MuxImportContainerFormat(this._c);
	@override String toString();
	String debug();
}
sealed class MuxImportFormat {
	const MuxImportFormat();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	MuxImportKind kind();
}
final class MuxImportFormatAudio extends MuxImportFormat {
	final MuxImportAudioFormat $0;
	const MuxImportFormatAudio(this.$0);
}
final class MuxImportFormatVideo extends MuxImportFormat {
	final MuxImportVideoFormat $0;
	const MuxImportFormatVideo(this.$0);
}
final class MuxImportFormatContainer extends MuxImportFormat {
	final MuxImportContainerFormat $0;
	const MuxImportFormatContainer(this.$0);
}
enum MuxImportKind {
	audio(0),
	video(1),
	container(2),
	const MuxImportKind(this._c);
	String debug();
	String name();
}
enum MuxImportVideoFormat {
	avc1(0),
	avc3(1),
	hvc1(2),
	hev1(3),
	av01(4),
	vp8(5),
	vp9(6),
	const MuxImportVideoFormat(this._c);
	@override String toString();
	String debug();
}
sealed class MuxRateTarget {
	const MuxRateTarget();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxRateTargetEnabled extends MuxRateTarget {
	final BandwidthRate $0;
	const MuxRateTargetEnabled(this.$0);
}
final class MuxRateTargetDisabled extends MuxRateTarget {
	const MuxRateTargetDisabled();
}
enum MuxTimelineKind {
	video(0),
	audio(1),
	const MuxTimelineKind(this._c);
	String debug();
}
enum MuxTimelineError implements Exception {
	overflow(0),
	beforeOrigin(1),
	delayOverflow(2),
	const MuxTimelineError(this._c);
	@override String toString();
	String debug();
}
sealed class TokioListenBind {
	const TokioListenBind();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class TokioListenBindAddr extends TokioListenBind {
	final SocketAddr $0;
	const TokioListenBindAddr(this.$0);
}
final class TokioListenBindHost extends TokioListenBind {
	final String $0;
	final int $1;
	const TokioListenBindHost(this.$0, this.$1);
}
sealed class TransportMemoryError {
	const TransportMemoryError();
	@override String toString();
	String debug();
}
final class TransportMemoryErrorSession extends TransportMemoryError {
	final int code;
	final String reason;
	const TransportMemoryErrorSession({required this.code, required this.reason});
}
final class TransportMemoryErrorReset extends TransportMemoryError {
	final int $0;
	const TransportMemoryErrorReset(this.$0);
}
final class TransportMemoryErrorStopped extends TransportMemoryError {
	final int $0;
	const TransportMemoryErrorStopped(this.$0);
}
final class TransportMemoryErrorClosed extends TransportMemoryError {
	const TransportMemoryErrorClosed();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
sealed class MuxCatalogMsfError {
	const MuxCatalogMsfError();
	@override String toString();
	String debug();
}
final class MuxCatalogMsfErrorUnresolvableNamespace extends MuxCatalogMsfError {
	final String name;
	final String namespace;
	const MuxCatalogMsfErrorUnresolvableNamespace({required this.name, required this.namespace});
}
final class MuxCatalogMsfErrorInvalidUtf8 extends MuxCatalogMsfError {
	const MuxCatalogMsfErrorInvalidUtf8();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCatalogMsfErrorParseFrame extends MuxCatalogMsfError {
	const MuxCatalogMsfErrorParseFrame();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCatalogMsfErrorMissingCmafInit extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingCmafInit(this.$0);
}
final class MuxCatalogMsfErrorMalformedInitData extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMalformedInitData(this.$0);
}
final class MuxCatalogMsfErrorMissingVideoCodec extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingVideoCodec(this.$0);
}
final class MuxCatalogMsfErrorMissingAudioCodec extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingAudioCodec(this.$0);
}
final class MuxCatalogMsfErrorUnsupportedPackaging extends MuxCatalogMsfError {
	final String name;
	final String packaging;
	const MuxCatalogMsfErrorUnsupportedPackaging({required this.name, required this.packaging});
}
final class MuxCatalogMsfErrorMissingTextCodec extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingTextCodec(this.$0);
}
final class MuxCatalogMsfErrorInvalidVideoCodec extends MuxCatalogMsfError {
	final String name;
	final String codec;
	const MuxCatalogMsfErrorInvalidVideoCodec({required this.name, required this.codec});
}
final class MuxCatalogMsfErrorInvalidAudioCodec extends MuxCatalogMsfError {
	final String name;
	final String codec;
	const MuxCatalogMsfErrorInvalidAudioCodec({required this.name, required this.codec});
}
final class MuxCatalogMsfErrorMissingAudioParams extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingAudioParams(this.$0);
}
final class MuxCatalogMsfErrorUnsupportedDerivationPackaging extends MuxCatalogMsfError {
	final String name;
	final String packaging;
	const MuxCatalogMsfErrorUnsupportedDerivationPackaging({required this.name, required this.packaging});
}
final class MuxCatalogMsfErrorMalformedAac extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMalformedAac(this.$0);
}
final class MuxCatalogMsfErrorMalformedOpus extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMalformedOpus(this.$0);
}
final class MuxCatalogMsfErrorMalformedFlac extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMalformedFlac(this.$0);
}
final class MuxCatalogMsfErrorOpusTrailingBytes extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorOpusTrailingBytes(this.$0);
}
final class MuxCatalogMsfErrorUnsupportedDerivationCodec extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorUnsupportedDerivationCodec(this.$0);
}
final class MuxCatalogMsfErrorMalformedInitSegment extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMalformedInitSegment(this.$0);
}
final class MuxCatalogMsfErrorMissingInitMoov extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingInitMoov(this.$0);
}
final class MuxCatalogMsfErrorMissingAudioSampleEntry extends MuxCatalogMsfError {
	final String $0;
	const MuxCatalogMsfErrorMissingAudioSampleEntry(this.$0);
}
enum MuxCodecFlacError implements Exception {
	short(0),
	invalidMarker(1),
	missingStreamInfo(2),
	const MuxCodecFlacError(this._c);
	@override String toString();
	String debug();
}
enum MuxCodecMp3Error implements Exception {
	headerTooShort(0),
	missingSync(1),
	reservedVersion(2),
	notLayer3(3),
	reservedSampleRate(4),
	const MuxCodecMp3Error(this._c);
	@override String toString();
	String debug();
}
sealed class MuxCodecOpusError implements Exception {
	const MuxCodecOpusError();
	@override String toString();
	String debug();
}
final class MuxCodecOpusErrorHeadTooShort extends MuxCodecOpusError {
	const MuxCodecOpusErrorHeadTooShort();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCodecOpusErrorInvalidSignature extends MuxCodecOpusError {
	const MuxCodecOpusErrorInvalidSignature();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCodecOpusErrorUnsupportedVersion extends MuxCodecOpusError {
	final int $0;
	const MuxCodecOpusErrorUnsupportedVersion(this.$0);
}
final class MuxCodecOpusErrorUnsupportedChannelCount extends MuxCodecOpusError {
	final int $0;
	const MuxCodecOpusErrorUnsupportedChannelCount(this.$0);
}
final class MuxCodecOpusErrorMappingTableTooShort extends MuxCodecOpusError {
	const MuxCodecOpusErrorMappingTableTooShort();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCodecOpusErrorInvalidMappingTable extends MuxCodecOpusError {
	const MuxCodecOpusErrorInvalidMappingTable();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCodecOpusErrorUnsupportedMappingFamily extends MuxCodecOpusError {
	final int $0;
	const MuxCodecOpusErrorUnsupportedMappingFamily(this.$0);
}
enum MuxCodecVp8Error {
	frameTooShort(0),
	keyframeHeaderTooShort(1),
	startCodeMismatch(2),
	emptyFrame(3),
	const MuxCodecVp8Error(this._c);
	@override String toString();
	String debug();
}
enum MuxCodecVp9Error {
	invalidFrameMarker(0),
	invalidSyncCode(1),
	truncated(2),
	emptyFrame(3),
	const MuxCodecVp9Error(this._c);
	@override String toString();
	String debug();
}
sealed class MuxContainerFmp4Chunk {
	const MuxContainerFmp4Chunk();
	String debug();
	typed.Uint8List data();
	typed.Uint8List intoData();
	typed.Uint8List? init();
	MuxContainerFmp4Fragment? fragment();
}
final class MuxContainerFmp4ChunkInit extends MuxContainerFmp4Chunk {
	final typed.Uint8List $0;
	const MuxContainerFmp4ChunkInit(this.$0);
}
final class MuxContainerFmp4ChunkFragment extends MuxContainerFmp4Chunk {
	final MuxContainerFmp4Fragment $0;
	const MuxContainerFmp4ChunkFragment(this.$0);
}
enum MuxContainerTsFraming {
	section(0),
	const MuxContainerTsFraming(this._c);
	String debug();
	factory MuxContainerTsFraming.default_();
}
sealed class MuxContainerTsStep {
	const MuxContainerTsStep();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxContainerTsStepRewind extends MuxContainerTsStep {
	final RustDuration by;
	final bool flagged;
	const MuxContainerTsStepRewind({required this.by, required this.flagged});
}
final class MuxContainerTsStepLeap extends MuxContainerTsStep {
	final RustDuration by;
	const MuxContainerTsStepLeap({required this.by});
}
enum MuxCodecAacAdtsError implements Exception {
	truncated(0),
	syncword(1),
	sampleRate(2),
	frameLength(3),
	packetLength(4),
	const MuxCodecAacAdtsError(this._c);
	@override String toString();
	String debug();
}
enum MuxContainerFmp4FragmentMissingDuration {
	reject(0),
	inferFromPresentationTime(1),
	const MuxContainerFmp4FragmentMissingDuration(this._c);
	String debug();
	factory MuxContainerFmp4FragmentMissingDuration.default_();
}
enum MuxContainerTsStatsClass {
	audio(1),
	video(2),
	data(0),
	const MuxContainerTsStatsClass(this._c);
	String debug();
	factory MuxContainerTsStatsClass.default_();
}
enum MoqTokioQuicCongestionControl {
	loss(0),
	delay(1),
	none(2),
	const MoqTokioQuicCongestionControl(this._c);
	String debug();
}
enum MoqTokioConnectionPhase {
	connecting(0),
	connected(1),
	disconnected(2),
	migrating(3),
	closed(4),
	const MoqTokioConnectionPhase(this._c);
	String debug();
}
enum MoqTokioAcceptFailure {
	connection(0),
	exhausted(1),
	unknown(2),
	const MoqTokioAcceptFailure(this._c);
	@override String toString();
	String debug();
	static List<MoqTokioAcceptFailure> get ALL;
	String asStr();
}
enum MoqTokioRedirect {
	follow(0),
	sameHost(1),
	ignore(2),
	const MoqTokioRedirect(this._c);
	String debug();
	factory MoqTokioRedirect.default_();
}
sealed class MoqQuicRate {
	const MoqQuicRate();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqQuicRateUnlimited extends MoqQuicRate {
	const MoqQuicRateUnlimited();
}
final class MoqQuicRateBytesPerSecond extends MoqQuicRate {
	final U64 $0;
	const MoqQuicRateBytesPerSecond(this.$0);
}
sealed class Mp4AtomColr {
	const Mp4AtomColr();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomColr.default_();
	factory Mp4AtomColr.new_(int colourPrimaries, int transferCharacteristics, int matrixCoefficients, bool fullRangeFlag);
}
final class Mp4AtomColrNclx extends Mp4AtomColr {
	final int colourPrimaries;
	final int transferCharacteristics;
	final int matrixCoefficients;
	final bool fullRangeFlag;
	const Mp4AtomColrNclx({required this.colourPrimaries, required this.transferCharacteristics, required this.matrixCoefficients, required this.fullRangeFlag});
}
final class Mp4AtomColrNclc extends Mp4AtomColr {
	final int colourPrimaries;
	final int transferCharacteristics;
	final int matrixCoefficients;
	const Mp4AtomColrNclc({required this.colourPrimaries, required this.transferCharacteristics, required this.matrixCoefficients});
}
final class Mp4AtomColrRicc extends Mp4AtomColr {
	final typed.Uint8List profile;
	const Mp4AtomColrRicc({required this.profile});
}
final class Mp4AtomColrProf extends Mp4AtomColr {
	final typed.Uint8List profile;
	const Mp4AtomColrProf({required this.profile});
}
sealed class Mp4AtomUncC {
	const Mp4AtomUncC();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUncCV1 extends Mp4AtomUncC {
	final Mp4AtomFourCc profile;
	const Mp4AtomUncCV1({required this.profile});
}
final class Mp4AtomUncCV0 extends Mp4AtomUncC {
	final Mp4AtomFourCc profile;
	final List<Mp4AtomUncompressedComponent> components;
	final int samplingType;
	final int interleaveType;
	final int blockSize;
	final bool componentsLittleEndian;
	final bool blockPadLsb;
	final bool blockLittleEndian;
	final bool blockReversed;
	final bool padUnknown;
	final int pixelSize;
	final int rowAlignSize;
	final int tileAlignSize;
	final int numTileColsMinusOne;
	final int numTileRowsMinusOne;
	const Mp4AtomUncCV0({required this.profile, required this.components, required this.samplingType, required this.interleaveType, required this.blockSize, required this.componentsLittleEndian, required this.blockPadLsb, required this.blockLittleEndian, required this.blockReversed, required this.padUnknown, required this.pixelSize, required this.rowAlignSize, required this.tileAlignSize, required this.numTileColsMinusOne, required this.numTileRowsMinusOne});
}
enum Mp4AtomSencBoxVersion {
	v0(0),
	v1(1),
	v2(2),
	const Mp4AtomSencBoxVersion(this._c);
	String debug();
	factory Mp4AtomSencBoxVersion.default_();
}
sealed class MoqTokioConnectError implements Exception {
	const MoqTokioConnectError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqTokioConnectErrorEmptyAddresses extends MoqTokioConnectError {
	const MoqTokioConnectErrorEmptyAddresses();
}
final class MoqTokioConnectErrorUnsupportedScheme extends MoqTokioConnectError {
	final String $0;
	const MoqTokioConnectErrorUnsupportedScheme(this.$0);
}
final class MoqTokioConnectErrorMissingHost extends MoqTokioConnectError {
	const MoqTokioConnectErrorMissingHost();
}
sealed class RustlsProtocolVersion {
	const RustlsProtocolVersion();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	ArrayU8x2 toArray();
	String? asStr();
}
final class RustlsProtocolVersionSSLv2 extends RustlsProtocolVersion {
	const RustlsProtocolVersionSSLv2();
}
final class RustlsProtocolVersionSSLv3 extends RustlsProtocolVersion {
	const RustlsProtocolVersionSSLv3();
}
final class RustlsProtocolVersionTLSv1_0 extends RustlsProtocolVersion {
	const RustlsProtocolVersionTLSv1_0();
}
final class RustlsProtocolVersionTLSv1_1 extends RustlsProtocolVersion {
	const RustlsProtocolVersionTLSv1_1();
}
final class RustlsProtocolVersionTLSv1_2 extends RustlsProtocolVersion {
	const RustlsProtocolVersionTLSv1_2();
}
final class RustlsProtocolVersionTLSv1_3 extends RustlsProtocolVersion {
	const RustlsProtocolVersionTLSv1_3();
}
final class RustlsProtocolVersionDTLSv1_0 extends RustlsProtocolVersion {
	const RustlsProtocolVersionDTLSv1_0();
}
final class RustlsProtocolVersionDTLSv1_2 extends RustlsProtocolVersion {
	const RustlsProtocolVersionDTLSv1_2();
}
final class RustlsProtocolVersionDTLSv1_3 extends RustlsProtocolVersion {
	const RustlsProtocolVersionDTLSv1_3();
}
final class RustlsProtocolVersionUnknown extends RustlsProtocolVersion {
	final int $0;
	const RustlsProtocolVersionUnknown(this.$0);
}
enum RustlsClientTls12Resumption {
	disabled(0),
	sessionIdOnly(1),
	sessionIdOrTickets(2),
	const RustlsClientTls12Resumption(this._c);
	String debug();
}
enum MoqSockDemuxDiscard {
	stun(0),
	unknown(1),
	unclaimed(2),
	overflow(3),
	const MoqSockDemuxDiscard(this._c);
	String debug();
	static List<MoqSockDemuxDiscard> get ALL;
	String asStr();
}
enum Mp4AtomClockType {
	unknown(0),
	doesNotSync(1),
	canSync(2),
	reserved(3),
	const Mp4AtomClockType(this._c);
	String debug();
	factory Mp4AtomClockType.default_();
}
sealed class Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlock();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomFlacMetadataBlockStreamInfo extends Mp4AtomFlacMetadataBlock {
	final int minimumBlockSize;
	final int maximumBlockSize;
	final Mp4AtomU24 minimumFrameSize;
	final Mp4AtomU24 maximumFrameSize;
	final int sampleRate;
	final int numChannelsMinusOne;
	final int bitsPerSampleMinusOne;
	final U64 numberOfInterchannelSamples;
	final typed.Uint8List md5Checksum;
	const Mp4AtomFlacMetadataBlockStreamInfo({required this.minimumBlockSize, required this.maximumBlockSize, required this.minimumFrameSize, required this.maximumFrameSize, required this.sampleRate, required this.numChannelsMinusOne, required this.bitsPerSampleMinusOne, required this.numberOfInterchannelSamples, required this.md5Checksum});
}
final class Mp4AtomFlacMetadataBlockPadding extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockPadding();
}
final class Mp4AtomFlacMetadataBlockApplication extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockApplication();
}
final class Mp4AtomFlacMetadataBlockSeekTable extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockSeekTable();
}
final class Mp4AtomFlacMetadataBlockVorbisComment extends Mp4AtomFlacMetadataBlock {
	final String vendorString;
	final List<String> comments;
	const Mp4AtomFlacMetadataBlockVorbisComment({required this.vendorString, required this.comments});
}
final class Mp4AtomFlacMetadataBlockCueSheet extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockCueSheet();
}
final class Mp4AtomFlacMetadataBlockPicture extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockPicture();
}
final class Mp4AtomFlacMetadataBlockReserved extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockReserved();
}
final class Mp4AtomFlacMetadataBlockForbidden extends Mp4AtomFlacMetadataBlock {
	const Mp4AtomFlacMetadataBlockForbidden();
}
sealed class Mp4AtomChannelStructure {
	const Mp4AtomChannelStructure();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomChannelStructureExplicitPositions extends Mp4AtomChannelStructure {
	final List<Mp4AtomSpeakerPosition> positions;
	const Mp4AtomChannelStructureExplicitPositions({required this.positions});
}
final class Mp4AtomChannelStructureDefinedLayout extends Mp4AtomChannelStructure {
	final int layout;
	final U64? omittedChannelsMap;
	final int? channelOrderDefinition;
	const Mp4AtomChannelStructureDefinedLayout({required this.layout, required this.omittedChannelsMap, required this.channelOrderDefinition});
}
sealed class Mp4AtomUuid {
	const Mp4AtomUuid();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomUuidC2pa extends Mp4AtomUuid {
	final Mp4AtomC2pa $0;
	const Mp4AtomUuidC2pa(this.$0);
}
final class Mp4AtomUuidUnknown extends Mp4AtomUuid {
	final Mp4AtomExtendedType $0;
	final typed.Uint8List $1;
	const Mp4AtomUuidUnknown(this.$0, this.$1);
}
enum MoqSockUdpEcnCodepoint {
	ect0(2),
	ect1(1),
	ce(3),
	const MoqSockUdpEcnCodepoint(this._c);
	String debug();
	static MoqSockUdpEcnCodepoint? fromBits(int x);
}
enum MoqQuicSide {
	client(0),
	server(1),
	const MoqQuicSide(this._c);
	String debug();
	bool isClient();
	bool isServer();
}
enum MoqQuicStreamDir {
	bi(0),
	uni(1),
	const MoqQuicStreamDir(this._c);
	@override String toString();
	String debug();
}
sealed class Mp4AtomSpeakerPosition {
	const Mp4AtomSpeakerPosition();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomSpeakerPositionStandard extends Mp4AtomSpeakerPosition {
	final Mp4AtomAudioChannelPosition $0;
	const Mp4AtomSpeakerPositionStandard(this.$0);
}
final class Mp4AtomSpeakerPositionExplicit extends Mp4AtomSpeakerPosition {
	final Mp4AtomExplicitSpeakerPosition $0;
	const Mp4AtomSpeakerPositionExplicit(this.$0);
}
sealed class Mp4AtomDataType {
	const Mp4AtomDataType();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomDataTypeWellKnown extends Mp4AtomDataType {
	final int $0;
	const Mp4AtomDataTypeWellKnown(this.$0);
}
final class Mp4AtomDataTypeCustom extends Mp4AtomDataType {
	final String $0;
	const Mp4AtomDataTypeCustom(this.$0);
}
final class Mp4AtomDataTypeUnknown extends Mp4AtomDataType {
	final int $0;
	final typed.Uint8List $1;
	const Mp4AtomDataTypeUnknown(this.$0, this.$1);
}
sealed class Mp4AtomStszSamples {
	const Mp4AtomStszSamples();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStszSamples.default_();
}
final class Mp4AtomStszSamplesIdentical extends Mp4AtomStszSamples {
	final int count;
	final int size;
	const Mp4AtomStszSamplesIdentical({required this.count, required this.size});
}
final class Mp4AtomStszSamplesDifferent extends Mp4AtomStszSamples {
	final typed.Uint32List sizes;
	const Mp4AtomStszSamplesDifferent({required this.sizes});
}
sealed class Mp4AtomDrefEntry {
	const Mp4AtomDrefEntry();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomFourCc kind();
}
final class Mp4AtomDrefEntryUrl extends Mp4AtomDrefEntry {
	final Mp4AtomUrl $0;
	const Mp4AtomDrefEntryUrl(this.$0);
}
final class Mp4AtomDrefEntryUrn extends Mp4AtomDrefEntry {
	final Mp4AtomUrn $0;
	const Mp4AtomDrefEntryUrn(this.$0);
}
final class Mp4AtomDrefEntryUnknown extends Mp4AtomDrefEntry {
	final Mp4AtomFourCc $0;
	final typed.Uint8List $1;
	const Mp4AtomDrefEntryUnknown(this.$0, this.$1);
}
sealed class Mp4AtomEmsgTimestamp {
	const Mp4AtomEmsgTimestamp();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomEmsgTimestampRelative extends Mp4AtomEmsgTimestamp {
	final int $0;
	const Mp4AtomEmsgTimestampRelative(this.$0);
}
final class Mp4AtomEmsgTimestampAbsolute extends Mp4AtomEmsgTimestamp {
	final U64 $0;
	const Mp4AtomEmsgTimestampAbsolute(this.$0);
}
sealed class Mp4AtomTrunSamples {
	const Mp4AtomTrunSamples();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTrunSamples.default_();
	U64 len();
	bool isEmpty();
	Mp4AtomTrunEntry? get(U64 index);
}
final class Mp4AtomTrunSamplesInherited extends Mp4AtomTrunSamples {
	final int count;
	final int? firstFlags;
	const Mp4AtomTrunSamplesInherited({required this.count, required this.firstFlags});
}
final class Mp4AtomTrunSamplesExplicit extends Mp4AtomTrunSamples {
	final List<Mp4AtomTrunEntry> $0;
	const Mp4AtomTrunSamplesExplicit(this.$0);
}
enum Mp4AtomReferenceTime {
	input(0),
	output(1),
	finalised(2),
	written(3),
	consistent(4),
	realTime(5),
	const Mp4AtomReferenceTime(this._c);
	String debug();
	factory Mp4AtomReferenceTime.default_();
}
sealed class Mp4AtomProjKind {
	const Mp4AtomProjKind();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomProjKindEqui extends Mp4AtomProjKind {
	final Mp4AtomEqui $0;
	const Mp4AtomProjKindEqui(this.$0);
}
final class Mp4AtomProjKindCbmp extends Mp4AtomProjKind {
	final Mp4AtomCbmp $0;
	const Mp4AtomProjKindCbmp(this.$0);
}
final class Mp4AtomProjKindMshp extends Mp4AtomProjKind {
	final Mp4AtomMshp $0;
	const Mp4AtomProjKindMshp(this.$0);
}
enum Mp4AtomVexuProjKind {
	rectilinear(0),
	equirectangular(1),
	halfEquirectangular(2),
	fisheye(3),
	parametricImmersive(4),
	const Mp4AtomVexuProjKind(this._c);
	String debug();
	Mp4AtomFourCc kind();
}
enum Mp4AtomAudioChannelPosition {
	frontLeft(0),
	frontRight(1),
	frontCenter(2),
	lfe1(3),
	leftSurround(4),
	rightSurround(5),
	frontLeftOfCenter(6),
	frontRightOfCenter(7),
	rearLeft(8),
	rearRight(9),
	rearCenter(10),
	surroundLeft(11),
	surroundRight(12),
	sideLeft(13),
	sideRight(14),
	frontLeftWide(15),
	frontRightWide(16),
	topFrontLeft(17),
	topFrontRight(18),
	topFrontCenter(19),
	topRearLeft(20),
	topRearRight(21),
	topRearCenter(22),
	topSideLeft(23),
	topSideRight(24),
	topCenter(25),
	lfe2(26),
	bottomFrontLeft(27),
	bottomFrontRight(28),
	bottomFrontCenter(29),
	topSurroundLeft(30),
	topSurroundRight(31),
	lfe3(36),
	leos(37),
	reos(38),
	hwbcal(39),
	hwbcar(40),
	lbs(41),
	rbs(42),
	leftEar(43),
	rightEar(44),
	unknown(127),
	const Mp4AtomAudioChannelPosition(this._c);
	String debug();
	static Mp4AtomAudioChannelPosition? fromU8(int value);
}
sealed class Mp4AtomIlstDataValue {
	const Mp4AtomIlstDataValue();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomIlstDataValueReserved extends Mp4AtomIlstDataValue {
	final typed.Uint8List $0;
	const Mp4AtomIlstDataValueReserved(this.$0);
}
final class Mp4AtomIlstDataValueUtf8 extends Mp4AtomIlstDataValue {
	final String $0;
	const Mp4AtomIlstDataValueUtf8(this.$0);
}
final class Mp4AtomIlstDataValueUtf16 extends Mp4AtomIlstDataValue {
	final String $0;
	const Mp4AtomIlstDataValueUtf16(this.$0);
}
final class Mp4AtomIlstDataValueJpeg extends Mp4AtomIlstDataValue {
	final typed.Uint8List $0;
	const Mp4AtomIlstDataValueJpeg(this.$0);
}
final class Mp4AtomIlstDataValuePng extends Mp4AtomIlstDataValue {
	final typed.Uint8List $0;
	const Mp4AtomIlstDataValuePng(this.$0);
}
final class Mp4AtomIlstDataValueBmp extends Mp4AtomIlstDataValue {
	final typed.Uint8List $0;
	const Mp4AtomIlstDataValueBmp(this.$0);
}
final class Mp4AtomIlstDataValueBeSignedInt extends Mp4AtomIlstDataValue {
	final int $0;
	const Mp4AtomIlstDataValueBeSignedInt(this.$0);
}
final class Mp4AtomIlstDataValueBeUnsignedInt extends Mp4AtomIlstDataValue {
	final U64 $0;
	const Mp4AtomIlstDataValueBeUnsignedInt(this.$0);
}
final class Mp4AtomIlstDataValueBeFloat32 extends Mp4AtomIlstDataValue {
	final double $0;
	const Mp4AtomIlstDataValueBeFloat32(this.$0);
}
final class Mp4AtomIlstDataValueBeFloat64 extends Mp4AtomIlstDataValue {
	final double $0;
	const Mp4AtomIlstDataValueBeFloat64(this.$0);
}
final class Mp4AtomIlstDataValueUnknown extends Mp4AtomIlstDataValue {
	final int $0;
	final typed.Uint8List $1;
	const Mp4AtomIlstDataValueUnknown(this.$0, this.$1);
}
sealed class Mp4AtomAnySampleGroupEntry {
	const Mp4AtomAnySampleGroupEntry();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class Mp4AtomAnySampleGroupEntryDirectReferenceSampleList extends Mp4AtomAnySampleGroupEntry {
	final int $0;
	final typed.Uint32List $1;
	const Mp4AtomAnySampleGroupEntryDirectReferenceSampleList(this.$0, this.$1);
}
final class Mp4AtomAnySampleGroupEntryUnknownGroupingType extends Mp4AtomAnySampleGroupEntry {
	final Mp4AtomFourCc $0;
	final typed.Uint8List $1;
	const Mp4AtomAnySampleGroupEntryUnknownGroupingType(this.$0, this.$1);
}
sealed class Mp4AtomSubsSubsampleSize {
	const Mp4AtomSubsSubsampleSize();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSubsSubsampleSize.default_();
	int value();
}
final class Mp4AtomSubsSubsampleSizeU16 extends Mp4AtomSubsSubsampleSize {
	final int $0;
	const Mp4AtomSubsSubsampleSizeU16(this.$0);
}
final class Mp4AtomSubsSubsampleSizeU32 extends Mp4AtomSubsSubsampleSize {
	final int $0;
	const Mp4AtomSubsSubsampleSizeU32(this.$0);
}
enum Level {
	trace(0),
	debug(1),
	info(2),
	warn(3),
	error(4),
	const Level(this._c);
	@override String toString();
}
final class Park extends _Object {
	String debug();
	Park clone();
	factory Park.default_();
	factory Park(Waiter waiter);
	Waiter hold(void Function() cx);
}
final class Waiter extends _Object {
	Waiter clone();
	factory Waiter(void Function() waker);
	factory Waiter.noop();
	void register(WaiterList list);
	Waker waker();
	Waker context();
	String batch(String Function() poll);
	static Cancelable<void> pollFuture(Future<void> future);
}
final class WaiterList extends _Object {
	String debug();
	factory WaiterList.default_();
	factory WaiterList();
	void register(Waiter waiter);
	WaiterList take();
	bool isEmpty();
	void wake();
}
final class DecodeError extends _Object implements Exception {
	@override String toString();
	String debug();
	DecodeError clone();
}
final class Driver extends _Object {
	String debug();
	ClockInstant? poll(ClockInstant now, Waiter waiter);
}
final class Instance extends _Object {
	@override String toString();
	String debug();
	Instance clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Instance.random();
	String asStr();
	bool isMinted();
}
final class Path extends _Object {
	@override String toString();
	String debug();
	Path clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Path.default_();
	factory Path(String s);
	factory Path.fromStatic(String s);
	bool hasPrefix(Path prefix);
	Path? stripPrefix(Path prefix);
	Path? head(U64 n);
	List<String> parts();
	String asStr();
	factory Path.empty();
	bool isEmpty();
	U64 len();
	Path join(Path other);
	Path resolve(PathRelative rel);
	Path? tryResolve(PathRelative rel);
	PathRelative? relative(Path base);
}
final class Pattern extends _Object {
	@override String toString();
	String debug();
	Pattern clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Pattern.default_();
	factory Pattern(List<Segment> segments);
	factory Pattern.all();
	factory Pattern.literal(String path);
	factory Pattern.subtree(String path);
	String asStr();
	List<Segment> segments();
	String head();
	bool isLiteral();
	String? asPrefix();
	bool hasGlobstar();
	bool matches(String path);
	bool contains(Pattern other);
	bool overlaps(Pattern other);
	Specificity specificity();
	Patterns rebase(String root);
	Patterns intersect(Pattern other);
	List<Pattern>? captures(Pattern matched);
	Pattern rooted(String root);
}
final class Patterns extends _Object {
	String debug();
	Patterns clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Patterns.default_();
	factory Patterns();
	bool insert(Pattern pattern);
	bool matches(String path);
	bool contains(Pattern pattern);
	bool covers(Patterns other);
	bool overlaps(Pattern pattern);
	Patterns intersect(Patterns other);
	Patterns rebase(String root);
	Patterns rooted(String root);
	List<Pattern> iter();
	U64 len();
	bool isEmpty();
}
final class Server extends _Object {
	Server clone();
	factory Server.default_();
	factory Server();
	Server withPublisherProducer(OriginProducer publish);
	Server withPublisherConsumer(OriginConsumer publish);
	Server withSubscriber(OriginProducer subscribe);
	Server withStats(StatsSession stats);
	Server withLimits(SessionLimits limits);
	Server withVersions(Versions versions);
	Server withExtensions(SetupExtensions extensions);
	Server withImplementation(String implementation);
	Cancelable<(Session, Driver)> acceptSession(ClockInstant now, TransportMemorySession session);
	Cancelable<ServerHandshake> acceptRequestSession(ClockInstant now, TransportMemorySession session);
}
final class Session extends _Object {
	Session clone();
	Version version();
	Cancelable<String?> peerImplementation();
	BandwidthConsumer? sendBandwidth();
	BandwidthAllocator allocator();
	BandwidthConsumer? recvBandwidth();
	BandwidthViewers viewers();
	SessionStats stats();
	void abort(Error err);
	Cancelable<void> close_();
	Cancelable<Error> closed();
	GoawayProducer drain();
	GoawayConsumer draining();
	Cancelable<void> idle();
	AuthHandle auth();
}
final class Client extends _Object {
	Client clone();
	factory Client.default_();
	factory Client();
	Client withPublisherProducer(OriginProducer publish);
	Client withPublisherConsumer(OriginConsumer publish);
	Client withSubscriber(OriginProducer subscribe);
	Client withStats(StatsSession stats);
	Client withOrigin(OriginProducer origin);
	Client withLimits(SessionLimits limits);
	Client withAllocator(BandwidthAllocator allocator);
	Client withViewers(BandwidthViewers viewers);
	Client withVersions(Versions versions);
	Client withUrl(Url url);
	Client withToken(SetupToken token);
	Client withAuth(AuthHandle auth);
	Client withExtensions(SetupExtensions extensions);
	Client withImplementation(String implementation);
	Client withCost(OriginCost cost);
	Client withPeerHop(Hop hop);
	Cancelable<(Session, Driver)> connectSession(ClockInstant now, TransportMemorySession session);
}
final class Error extends _Object implements Exception {
	@override String toString();
	String debug();
	Error clone();
	SessionError? session();
	StreamError? stream();
	factory Error.fromTransportError(Error err);
	factory Error.fromTransportMemoryError(TransportMemoryError err);
}
final class Version extends _Object {
	@override String toString();
	String debug();
	Version clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	bool hasSetupToken();
	String asStr();
	static Version? fromCode(U64 code);
	U64 code();
	String alpn();
	bool isLite();
}
final class Versions extends _Object {
	String debug();
	Versions clone();
	factory Versions.default_();
	factory Versions.all();
	List<String> alpns();
	Versions? filter(Versions other);
	Version? select(Version version);
	bool contains(Version version);
	List<Version> iter();
}
final class TimedJson extends _Object {
	String debug();
	TimedJson clone();
	Json get valueGet;
	void valueSet(Json value);
	Timestamp? get atGet;
	void atSet(Timestamp? value);
	TimedJson at(Timestamp at);
}
final class Hops extends _Object {
	String debug();
	Hops clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Hops.default_();
	factory Hops();
	void push(Hop hop);
	bool contains(Hop hop);
	U64 len();
	bool isEmpty();
	List<Hop> iter();
	List<Hop> asSlice();
}
final class Specificity extends _Object {
	String debug();
	Specificity clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class ClockClock extends _Object {
	String debug();
	ClockClock clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory ClockClock.current();
	ClockInstant now();
	ClockWall wall();
}
final class ClockDeadline extends _Object {
	String debug();
	factory ClockDeadline(ClockDeadlines deadlines);
	factory ClockDeadline.after(ClockDeadlines deadlines, RustDuration duration);
	void set(ClockInstant? at);
	ClockInstant? deadline();
	Cancelable<void> poll();
}
final class ClockDeadlines extends _Object {
	ClockDeadlines clone();
	factory ClockDeadlines(ClockInstant now);
	void advance(ClockInstant now);
	ClockInstant? park(Waiter waiter);
	ClockInstant? next();
	ClockInstant now();
	ClockEntered enter();
}
final class ClockEntered extends _Object {
	String debug();
}
final class ClockManual extends _Object {
	String debug();
	ClockManual clone();
	factory ClockManual.default_();
	static ClockWall get WALL_START;
	factory ClockManual();
	ClockInstant now();
	ClockWall wall();
	void advance(RustDuration duration);
	void stepWall(ClockWall wall);
}
final class ClockRng extends _Object {
}
final class ClockInterval extends _Object {
	Cancelable<ClockInstant> tick();
	void setMissedTickBehavior(ClockMissedTickBehavior missed);
	RustDuration period();
}
final class ClockWallMap extends _Object {
	String debug();
	ClockWallMap clone();
	factory ClockWallMap.default_();
	static RustDuration get TOLERANCE;
	void sample(ClockInstant now, ClockWall wall);
	ClockInstant? map(ClockWall at);
}
final class CoopBudget extends _Object {
	String debug();
	CoopBudget clone();
	factory CoopBudget(int passes);
	Cancelable<void> pollYield();
}
final class HangCatalog extends _Object {
	String debug();
	HangCatalog clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalog.default_();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	static String get DEFAULT_NAME;
	static TrackInfo defaultTrackInfo();
	static TrackSubscription defaultSubscription();
	void checkRenditions();
	HangCatalog media();
	static Cancelable<HangCatalogConsumer> subscribe(BroadcastConsumer broadcast);
	factory HangCatalog.fromSlice(typed.Uint8List v);
	factory HangCatalog.fromReader(typed.Uint8List reader);
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class HangCatalogExtra extends _Object {
	String debug();
	HangCatalogExtra clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogExtra.default_();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	MuxCatalogHangExtra get extGet;
	void extSet(MuxCatalogHangExtra value);
	void checkRenditions();
	HangCatalog media();
	static Cancelable<HangCatalogConsumerExtra> subscribe(BroadcastConsumer broadcast);
	factory HangCatalogExtra.fromSlice(typed.Uint8List v);
	factory HangCatalogExtra.fromReader(typed.Uint8List reader);
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class HangCatalogExt extends _Object {
	String debug();
	HangCatalogExt clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogExt.default_();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	MuxContainerTsExt get extGet;
	void extSet(MuxContainerTsExt value);
	void checkRenditions();
	HangCatalog media();
	static Cancelable<HangCatalogConsumerExt> subscribe(BroadcastConsumer broadcast);
	factory HangCatalogExt.fromSlice(typed.Uint8List v);
	factory HangCatalogExt.fromReader(typed.Uint8List reader);
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class HangError extends _Object implements Exception {
	@override String toString();
	String debug();
	HangError clone();
}
final class MuxClock extends _Object {
	String debug();
	MuxClock clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxClock.default_();
	static Timescale get TIMESCALE;
	factory MuxClock();
	factory MuxClock.at(ClockInstant epoch, ClockWall wall);
	Timestamp now();
	Timestamp capture(ClockInstant at);
	Timescale timescale();
	HangCatalogClock? wall();
	ClockWall wallClock(Timestamp pts);
}
final class MuxLingerUnit extends _Object {
	static RustDuration get DEFAULT;
	factory MuxLingerUnit(List<BroadcastProducer> broadcasts, RustDuration duration);
	Cancelable<void> wait();
}
final class MuxPacer extends _Object {
	factory MuxPacer.default_();
	MuxPacer withLead(RustDuration lead);
	MuxPacer withDelay(RustDuration delay);
	ClockInstant pace(Timestamp ts, ClockInstant now);
	ClockInstant? due(Timestamp ts);
	RustDuration slack();
	ClockInstant hurry(Timestamp ts, ClockInstant now);
}
final class MuxBinding extends _Object {
	factory MuxBinding(BroadcastConsumer broadcast);
	Cancelable<BroadcastConsumer> broadcast();
}
final class MuxSource extends _Object {
	MuxSource clone();
	factory MuxSource(OriginConsumer origin, BroadcastConsumer broadcast);
	List<(String, HangEchoRendition)> stats();
	BroadcastConsumer broadcast();
	Cancelable<MuxCatalogConsumerExtra> catalogExtra(MuxCatalogCatalogFormat format);
	Cancelable<MuxCatalogConsumerExt> catalogExt(MuxCatalogCatalogFormat format);
	Cancelable<MuxCatalogConsumer> catalogUnit(MuxCatalogCatalogFormat format);
	Path? resolveReference(PathRelative? rel);
	Cancelable<BroadcastConsumer> resolve(PathRelative? rel, Instance? instance);
	MuxBinding bind(PathRelative? rel, Instance? instance);
	Cancelable<TrackSubscriber> subscribeTrack(PathRelative? rel, Instance? instance, String name);
	MuxFollowInstances follow(PathRelative? rel, Instance? instance);
}
final class MuxError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxError clone();
}
final class TokioDuration extends _Object {
	@override String toString();
	String debug();
	TokioDuration clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TokioDuration.default_();
}
final class TokioBackoff extends _Object {
	String debug();
	TokioBackoff clone();
	factory TokioBackoff.default_();
	TokioDuration? get initialGet;
	void initialSet(TokioDuration? value);
	int get multiplierGet;
	void multiplierSet(int value);
	TokioDuration? get maxGet;
	void maxSet(TokioDuration? value);
	TokioDuration? get healthyGet;
	void healthySet(TokioDuration? value);
	TokioDuration? get timeoutGet;
	void timeoutSet(TokioDuration? value);
	RustDuration initial();
	RustDuration max();
	RustDuration timeout();
}
final class TokioClient extends _Object {
	TokioClient clone();
	factory TokioClient(TokioClientConfig config);
	TokioClient withPublisherProducer(OriginProducer publish);
	TokioClient withPublisherConsumer(OriginConsumer publish);
	TokioClient withOrigin(OriginProducer origin);
	TokioClient withSubscriber(OriginProducer subscribe);
	TokioClient withLimits(SessionLimits limits);
	TokioClient withStats(StatsSession stats);
	TokioClient withCost(U64 cost);
	TokioClient withPeerHop(Hop hop);
	TokioClient withReconnect(bool reconnect);
	TokioClient withRestart(TokioRestart restart);
	TokioConnection connect(MoqTokioAddrs addrs);
	Cancelable<void> close_();
	TokioConnection? publish(OriginConsumer origin);
	TokioConnection? consume(OriginProducer origin);
}
final class TokioConnection extends _Object {
	TokioConnection clone();
	void abort(Error err);
	Cancelable<void> close_();
	Cancelable<TokioConnection> established();
	Cancelable<TokioStatus> status();
	bool connected();
	U64 connects();
	Version? version();
	TokioTransport? transport();
	GoawayConsumer? draining();
	BandwidthConsumer sendBandwidth();
	BandwidthAllocator allocator();
	BandwidthConsumer recvBandwidth();
	BandwidthViewers viewers();
	Cancelable<void> closed();
	AuthHandle auth();
	AuthTokens tokens();
	TokioConnectionMonitor monitor();
}
final class TokioError extends _Object implements Exception {
	@override String toString();
	String debug();
	TokioError clone();
	TokioConnectError? connectError();
	bool isAuth();
	int? status();
}
final class TokioLog extends _Object {
	String debug();
	TokioLog clone();
	factory TokioLog.default_();
	Level get levelGet;
	void levelSet(Level value);
	factory TokioLog(Level level);
	Level? level();
	void init();
}
final class TokioServer extends _Object {
	factory TokioServer(TokioServerConfig config);
	Cancelable<void> servePublish(OriginConsumer origin);
	Cancelable<void> serveConsume(OriginProducer origin);
	Cancelable<void> serveBoth(OriginConsumer publish, OriginProducer subscribe);
	MoqTokioTcpTlsAcceptor? tlsAcceptor();
	TokioTlsCertificates certificates();
	List<MoqTokioAcceptHealth> acceptHealth();
	Cancelable<TokioServerListener> bind();
	Cancelable<TokioServerListener> listen();
	SocketAddr localAddr();
	SocketAddr? websocketLocalAddr();
}
final class AuthWatch extends _Object {
	AuthGrant? peek();
	Cancelable<AuthGrant?> changed();
}
final class AuthRequests extends _Object {
	Cancelable<AuthRequest?> next();
}
final class AuthRequest extends _Object {
	typed.Uint8List token();
	AuthRequestKind? kind();
	Path? path();
	Cancelable<Error> closed();
	AuthIssued accept(AuthGrant grant);
	void reject(SessionError code, String reason);
}
final class AuthIssued extends _Object {
	void update(AuthGrant grant);
	void revoke(SessionError code, String reason);
	Cancelable<Error> closed();
}
final class AuthGrant extends _Object {
	String debug();
	AuthGrant clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory AuthGrant.default_();
	Patterns get publishGet;
	void publishSet(Patterns value);
	Patterns get subscribeGet;
	void subscribeSet(Patterns value);
	ClockInstant? get expiresGet;
	void expiresSet(ClockInstant? value);
	factory AuthGrant.all();
}
final class AuthHandle extends _Object {
	AuthHandle clone();
	factory AuthHandle.default_();
	factory AuthHandle();
	AuthWatch grant();
	Cancelable<(AuthCredential, AuthAnswer)> add(typed.Uint8List token);
	void authorize(AuthGrant grant);
	void abort(Error err);
}
final class AuthCredential extends _Object {
	String debug();
	AuthCredential clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	AuthWatch grant();
	Cancelable<Error> closed();
}
final class AuthDial extends _Object {
	Url get urlGet;
	void urlSet(Url value);
	SetupToken? get tokenGet;
	void tokenSet(SetupToken? value);
}
final class AuthToken extends _Object {
	String debug();
	AuthToken clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	String asStr();
}
final class AuthTokens extends _Object {
	AuthTokens clone();
	factory AuthTokens(List<AuthToken> tokens);
	AuthHandle handle();
	void set(List<AuthToken> tokens);
	Cancelable<void> answered();
	AuthDial dial(Url url, Versions versions);
	void connected(Session session);
	bool changed(AuthDial dial);
}
final class GoawayProducer extends _Object {
	void send(GoawayGoaway goaway);
}
final class GoawayGoaway extends _Object {
	String debug();
	GoawayGoaway clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory GoawayGoaway.default_();
	factory GoawayGoaway();
	factory GoawayGoaway.redirect(String uri);
	GoawayGoaway withTimeout(RustDuration timeout);
	String uri();
	RustDuration? timeout();
}
final class GoawayConsumer extends _Object {
	GoawayConsumer clone();
	Cancelable<GoawayGoaway> poll();
}
final class PathRelative extends _Object {
	@override String toString();
	String debug();
	PathRelative clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory PathRelative.default_();
	factory PathRelative(String s);
	String asStr();
	factory PathRelative.empty();
	bool isEmpty();
	U64 len();
}
final class ServerHandshake extends _Object {
	String path();
	SetupRole? role();
	Hop? peerHop();
	String? peerImplementation();
	AuthHandle auth();
	AuthRequests requests();
	SetupToken? token();
	ServerHandshake withPublisherProducer(OriginProducer publish);
	ServerHandshake withPublisherConsumer(OriginConsumer publish);
	ServerHandshake withSubscriber(OriginProducer subscribe);
	ServerHandshake withPeerHop(Hop hop);
	ServerHandshake withStats(StatsSession stats);
	ServerHandshake withLimits(SessionLimits limits);
	ServerHandshake withDeadline(ClockInstant at);
	Cancelable<(Session, Driver)> ok();
	void close_(Error err);
}
final class ServerActivity extends _Object {
	String debug();
	ServerActivity clone();
	factory ServerActivity.default_();
	ServerActive? enter();
	Cancelable<void> close_();
}
final class ServerActive extends _Object {
	String debug();
}
final class SessionStats extends _Object {
	String debug();
	SessionStats clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory SessionStats.default_();
	RustDuration? get rttGet;
	void rttSet(RustDuration? value);
	BandwidthRate? get estimatedSendRateGet;
	void estimatedSendRateSet(BandwidthRate? value);
	U64? get bytesSentGet;
	void bytesSentSet(U64? value);
	U64? get bytesReceivedGet;
	void bytesReceivedSet(U64? value);
	U64? get bytesLostGet;
	void bytesLostSet(U64? value);
	U64? get packetsSentGet;
	void packetsSentSet(U64? value);
	U64? get packetsReceivedGet;
	void packetsReceivedSet(U64? value);
	U64? get packetsLostGet;
	void packetsLostSet(U64? value);
	U64? get streamsEvictedGet;
	void streamsEvictedSet(U64? value);
}
final class StatsTier extends _Object {
	@override String toString();
	String debug();
	StatsTier clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsTier.default_();
	factory StatsTier(Path label);
	Path label();
	String asStr();
}
final class StatsSnapshot extends _Object {
	String debug();
	StatsSnapshot clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsSnapshot.default_();
	List<(StatsTier, StatsMessage, StatsTotals)> control();
	List<(StatsTier, StatsRole, StatsTraffic)> traffic();
	List<(StatsTier, StatsPresence)> sessions();
}
final class StatsReport extends _Object {
	String debug();
	StatsReport clone();
	factory StatsReport.default_();
	List<StatsTrafficEntry> get trafficGet;
	void trafficSet(List<StatsTrafficEntry> value);
	List<StatsSessionEntry> get sessionsGet;
	void sessionsSet(List<StatsSessionEntry> value);
}
final class StatsTrafficEntry extends _Object {
	String debug();
	StatsTrafficEntry clone();
	Path get pathGet;
	void pathSet(Path value);
	StatsTier get tierGet;
	void tierSet(StatsTier value);
	StatsTraffic get publisherGet;
	void publisherSet(StatsTraffic value);
	StatsTraffic get subscriberGet;
	void subscriberSet(StatsTraffic value);
	bool get closedGet;
	void closedSet(bool value);
}
final class StatsSessionEntry extends _Object {
	String debug();
	StatsSessionEntry clone();
	StatsTier get tierGet;
	void tierSet(StatsTier value);
	Path get labelGet;
	void labelSet(Path value);
	StatsPresence get presenceGet;
	void presenceSet(StatsPresence value);
	bool get closedGet;
	void closedSet(bool value);
}
final class StatsConfig extends _Object {
	String debug();
	StatsConfig clone();
	factory StatsConfig.default_();
	Patterns get excludeGet;
	void excludeSet(Patterns value);
	factory StatsConfig();
	StatsConfig withExclude(Pattern pattern);
}
final class StatsRegistry extends _Object {
	StatsRegistry clone();
	factory StatsRegistry.default_();
	factory StatsRegistry(StatsConfig config);
	factory StatsRegistry.disabled();
	bool isEnabled();
	StatsHandle tier(StatsTier tier);
	StatsSnapshot snapshot();
	void report(StatsReport report);
}
final class StatsHandle extends _Object {
	StatsHandle clone();
	factory StatsHandle.default_();
	void refuse(Path label, StatsRefusalKind reason);
	StatsSession session(Path label);
	StatsSession request(Path label);
}
final class StatsWire extends _Object {
	factory StatsWire.default_();
}
final class StatsSession extends _Object {
	StatsSession clone();
	factory StatsSession.default_();
	StatsWire wire(TransportWire counters);
	void retag(StatsTier tier, Path label);
}
final class TransportWire extends _Object {
	String debug();
	TransportWire clone();
	factory TransportWire.default_();
	factory TransportWire();
	U64 sentBytes();
	U64 receivedBytes();
	void setSentBytes(U64 total);
	void setReceivedBytes(U64 total);
}
final class TransportProbe extends _Object {
	String debug();
	TransportProbe clone();
	factory TransportProbe.default_();
	factory TransportProbe();
	BandwidthRate target();
	void setTarget(BandwidthRate rate);
}
final class BandwidthDemand extends _Object {
	BandwidthDemand clone();
	bool wanted();
	Cancelable<bool> wantedChanged();
}
final class BandwidthReservation extends _Object {
	BandwidthRate? rate();
	BandwidthConsumer consume();
	void update(BandwidthRate max);
}
final class BandwidthViewer extends _Object {
	BandwidthRate? rate();
	BandwidthConsumer consume();
	void update(BandwidthRate max);
}
final class BandwidthProducer extends _Object {
	BandwidthProducer clone();
	factory BandwidthProducer.default_();
	factory BandwidthProducer();
	void set(BandwidthRate? bitrate);
	BandwidthConsumer consume();
	BandwidthWeak weak();
	BandwidthDemand demand();
}
final class BandwidthWeak extends _Object {
	BandwidthWeak clone();
	BandwidthConsumer consume();
}
final class BandwidthAllocator extends _Object {
	String debug();
	BandwidthAllocator clone();
	factory BandwidthAllocator.default_();
	factory BandwidthAllocator(BandwidthConsumer estimate);
	void setTarget(BandwidthRate? target);
	factory BandwidthAllocator.unlimited();
	BandwidthReservation reserve(TrackDemand track, BandwidthRate max);
}
final class BandwidthViewers extends _Object {
	String debug();
	BandwidthViewers clone();
	factory BandwidthViewers.default_();
	static BandwidthRate get MAX_TARGET;
	factory BandwidthViewers(BandwidthWeak estimate);
	BandwidthViewer watch(BandwidthRate max);
	void setTarget(BandwidthRate? target);
}
final class BandwidthConsumer extends _Object {
	BandwidthConsumer clone();
	BandwidthRate? peek();
	bool probed();
	Cancelable<BandwidthRate?> changed();
}
final class BroadcastDatagramRequests extends _Object {
	BroadcastDatagramRequests clone();
	Cancelable<DatagramRequest> requested();
}
final class BroadcastConsumer extends _Object {
	BroadcastConsumer clone();
	BroadcastTracks tracks();
	BroadcastInfo info();
	TrackConsumer track(String name);
	DatagramConsumer datagrams(String name);
	bool isClosed();
	Cancelable<void> closed();
	bool isClone(BroadcastConsumer other);
}
final class BroadcastDemand extends _Object {
	BroadcastDemand clone();
	bool wanted();
	Cancelable<bool> wantedChanged();
}
final class BroadcastInfo extends _Object {
	String debug();
	BroadcastInfo clone();
	factory BroadcastInfo.default_();
	CachePool get poolGet;
	void poolSet(CachePool value);
	RustDuration get cacheDurationGet;
	void cacheDurationSet(RustDuration value);
	Path get pathGet;
	void pathSet(Path value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	factory BroadcastInfo();
	BroadcastProducer produce();
}
final class BroadcastProducer extends _Object {
	BroadcastProducer clone();
	factory BroadcastProducer(BroadcastInfo info);
	BroadcastInfo info();
	BroadcastDemand demand();
	TrackProducer createTrack(String name, TrackInfo? info);
	DatagramProducer createDatagrams(String name, DatagramInfo info);
	TrackRequest reserveTrack(String name);
	TrackProducer uniqueTrack(String suffix, TrackInfo? info);
	String uniqueName(String suffix);
	BroadcastRequests requests();
	BroadcastConsumer consume();
	void close_();
	bool isClone(BroadcastProducer other);
}
final class BroadcastRequests extends _Object {
	BroadcastRequests clone();
	BroadcastInfo info();
	Cancelable<TrackRequest> requestedTrack();
	BroadcastConsumer consume();
	BroadcastDatagramRequests datagrams();
	BroadcastLister lister();
	Cancelable<void> closed();
	bool isClone(BroadcastRequests other);
}
final class BroadcastLister extends _Object {
	Cancelable<void> pollWanted();
	Cancelable<void> pollUnwanted();
	void add(String name, TrackInfo info);
	void remove(String name);
	void finish(String name);
	void synced();
	void clear();
	void end();
	void abort(Error err);
}
final class BroadcastTracks extends _Object {
	Cancelable<BroadcastTracksEvent?> next();
	BroadcastConsumer broadcast();
}
final class BroadcastTracksEvent extends _Object {
	String debug();
	BroadcastTracksEvent clone();
}
final class CacheConfig extends _Object {
	String debug();
	CacheConfig clone();
	factory CacheConfig.default_();
	CacheConfig withCapacity(U64? capacity);
	CacheConfig withExpiry(RustDuration? expiry);
}
final class CachePool extends _Object {
	String debug();
	CachePool clone();
	factory CachePool.default_();
	factory CachePool(CacheConfig config);
	factory CachePool.unbounded();
	U64? capacity();
	U64 used();
	void resize(U64? capacity);
	RustDuration? expiry();
	ClockInstant? poll(ClockInstant now, Waiter waiter);
	CachePoolWeak downgrade();
}
final class CachePoolWeak extends _Object {
	String debug();
	CachePoolWeak clone();
	CachePool? upgrade();
}
final class DatagramProducer extends _Object {
	DatagramProducer clone();
	String name();
	DatagramInfo info();
	U64 append(Timestamp? timestamp, typed.Uint8List payload);
	void insert(Datagram datagram);
	DatagramConsumer consume();
	void finish();
	void abort(Error err);
	int? priority();
	Cancelable<int?> pollPriority(int? last);
	bool isClone(DatagramProducer other);
}
final class DatagramRequest extends _Object {
	String name();
	int? priority();
	DatagramProducer accept(DatagramInfo info);
	void reject(Error err);
}
final class DatagramConsumer extends _Object {
	DatagramConsumer clone();
	String name();
	Cancelable<DatagramInfo> info();
	Cancelable<Datagram?> recv();
	int priority();
	void setPriority(int priority);
	bool isClone(DatagramConsumer other);
}
final class DatagramInfo extends _Object {
	String debug();
	DatagramInfo clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory DatagramInfo.default_();
	Timescale? get timescaleGet;
	void timescaleSet(Timescale? value);
	int get priorityGet;
	void prioritySet(int value);
	DatagramInfo withTimescale(Timescale? timescale);
	DatagramInfo withPriority(int priority);
	Timestamp? rescale(Timestamp? timestamp);
	DatagramProducer produce(String name);
}
final class FrameProducer extends _Object {
	GroupProducer group();
	U64 remaining();
	void write(typed.Uint8List chunk);
	GroupProducer finish();
	void abort(Error err);
	U64 get sizeGet;
	Timestamp? get timestampGet;
}
final class FrameFrame extends _Object implements Exception {
	String debug();
	FrameFrame clone();
	Timestamp? get timestampGet;
	void timestampSet(Timestamp? value);
	typed.Uint8List get payloadGet;
	void payloadSet(typed.Uint8List value);
	factory FrameFrame(Timestamp? timestamp, typed.Uint8List payload);
	ClockInstant arrived();
}
final class FrameBuffer extends _Object {
	String debug();
	factory FrameBuffer.default_();
	factory FrameBuffer();
	U64 capacity();
	U64 len();
	bool isEmpty();
	List<FrameFrame> filled();
	void push(FrameFrame frame);
	void clear();
}
final class FrameConsumer extends _Object {
	FrameConsumer clone();
	Cancelable<typed.Uint8List?> readChunk();
	Cancelable<typed.Uint8List> readAll();
	U64 get sizeGet;
	Timestamp? get timestampGet;
}
final class GroupProducer extends _Object {
	Timescale? timescale();
	void startAt(U64 index);
	void writeFrame(Timestamp? timestamp, typed.Uint8List data);
	void writeFrames8(FrameBuffer frames);
	FrameProducer createFrame(FrameInfo frame);
	void finish();
	void abort(Error err);
	Cancelable<void> closed();
	Cancelable<void> ready(FrameInfo frame);
	U64 frameCount();
	GroupConsumer consume();
	GroupDemand demand();
	U64 get sequenceGet;
}
final class GroupDemand extends _Object {
	GroupDemand clone();
	U64 sequence();
	bool wanted();
	Cancelable<bool> wantedChanged();
	Cancelable<void> closed();
}
final class GroupConsumer extends _Object {
	GroupConsumer clone();
	void keepAlive();
	Timescale? timescale();
	ClockInstant arrived();
	U64 index();
	GroupConsumer withRange(BoundsU64 frames);
	void skipTo(U64 index);
	U64 frameCount();
	Cancelable<FrameConsumer?> nextFrame();
	Cancelable<FrameFrame?> readFrame();
	Cancelable<U64> pollReadFrames8(FrameBuffer out);
	Cancelable<List<FrameFrame>> readFrames8(FrameBuffer out);
	Cancelable<U64> finished();
	U64 get sequenceGet;
}
final class GroupFetch extends _Object {
	String debug();
	GroupFetch clone();
	factory GroupFetch.default_();
	int get priorityGet;
	void prioritySet(int value);
	RustDuration? get maxDelayGet;
	void maxDelaySet(RustDuration? value);
	GroupFetch withPriority(int priority);
	GroupFetch withMaxDelay(RustDuration maxDelay);
}
final class TrackRequests extends _Object {
	TrackRequests clone();
	String name();
	Cancelable<TrackFill> requestedFill();
	TrackDemand demand();
}
final class TrackDemand extends _Object {
	TrackDemand clone();
	String name();
	U64? wanted();
	Cancelable<U64?> wantedChanged();
	Cancelable<void> closed();
}
final class TrackSubscriber extends _Object {
	TrackInfo info();
	String name();
	BroadcastInfo broadcast();
	TrackControl control();
	Cancelable<GroupConsumer?> recvGroup();
	TrackOrdered ordered();
	bool isClone(TrackSubscriber other);
	U64? floor();
	U64? nextInRange(U64 from);
	Cancelable<TrackPosition> finished();
	TrackSubscriber withRange(BoundsU64 groups);
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
	Cancelable<TrackPosition?> largest();
}
final class TrackOrdered extends _Object {
	TrackInfo info();
	String name();
	BroadcastInfo broadcast();
	Cancelable<GroupConsumer?> nextGroup();
	TrackOrdered withRange(BoundsU64 groups);
	TrackControl control();
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
	Cancelable<TrackPosition> finished();
	Cancelable<TrackPosition?> largest();
	bool isClone(TrackOrdered other);
}
final class TrackRequest extends _Object {
	String name();
	TrackConsumer consume();
	TrackRequests requests();
	TrackDemand demand();
	TrackProducer accept(TrackInfo? info);
	void reject(Error err);
	TrackSubscription? subscription();
	Cancelable<TrackSubscription?> subscriptionChanged();
}
final class TrackInfo extends _Object {
	String debug();
	TrackInfo clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackInfo.default_();
	Timescale? get timescaleGet;
	void timescaleSet(Timescale? value);
	RustDuration? get maxAgeGet;
	void maxAgeSet(RustDuration? value);
	int get priorityGet;
	void prioritySet(int value);
	bool get compressGet;
	void compressSet(bool value);
	int? get refreshGet;
	void refreshSet(int? value);
	TrackInfo withTimescale(Timescale? timescale);
	TrackInfo withMaxAge(RustDuration? maxAge);
	TrackInfo withPriority(int priority);
	TrackInfo withCompress(bool compress);
	TrackInfo withRefresh(int? refresh);
	Timestamp? rescale(Timestamp? timestamp);
}
final class TrackProducer extends _Object implements Exception {
	TrackProducer clone();
	String name();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void writeFrame(Timestamp? timestamp, typed.Uint8List frame);
	void finish();
	void finishAt(TrackPosition end, Error? error);
	void startAt(U64? sequence);
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	void abort(Error err);
	void abortUnwanted(Error err);
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackDemand demand();
	TrackConsumer consume();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	Cancelable<TrackSubscription?> subscriptionChanged();
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class TrackConsumer extends _Object {
	TrackConsumer clone();
	String name();
	BroadcastInfo broadcast();
	Cancelable<TrackSubscriber> subscribe(TrackSubscription? subscription);
	Cancelable<GroupConsumer> fetchGroup(U64 sequence, GroupFetch? options);
	Cancelable<TrackInfo> info();
	Cancelable<TrackPosition?> largest();
}
final class TrackControl extends _Object {
	TrackControl clone();
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
}
final class TrackSubscription extends _Object {
	String debug();
	TrackSubscription clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackSubscription.default_();
	int get priorityGet;
	void prioritySet(int value);
	TrackDelay get delayGet;
	void delaySet(TrackDelay value);
	List<TrackRange> get rangesGet;
	void rangesSet(List<TrackRange> value);
	TrackOrder get orderGet;
	void orderSet(TrackOrder value);
	TrackSubscription withPriority(int priority);
	TrackSubscription withDelay(TrackDelay delay);
	TrackSubscription withRanges(List<TrackRange> ranges);
	TrackSubscription withRange(BoundsU64 groups);
	TrackSubscription withOrder(TrackOrder order);
}
final class TrackFill extends _Object {
	TrackFill clone();
	TrackPosition start();
	U64 frameStart(U64 sequence);
	U64 end();
	Timestamp? time();
	int priority();
	RustDuration? maxDelay();
	TrackOrder order();
	GroupProducer accept(U64 sequence, TrackInfo? info);
	void dropGroups(RangeU64 groups);
	void refuseGroups(RangeU64 groups, Error err);
	bool isWanted(U64 sequence);
	Cancelable<void> unwanted(U64 sequence);
	U64? wanted();
	Cancelable<U64?> wantedChanged();
}
final class OriginFollowing extends _Object {
	Cancelable<BroadcastConsumer> available();
}
final class OriginConfig extends _Object {
	String debug();
	OriginConfig clone();
	factory OriginConfig.default_();
	Hop get hopGet;
	void hopSet(Hop value);
	OriginHopLimit get maxHopsGet;
	void maxHopsSet(OriginHopLimit value);
	CachePool get poolGet;
	void poolSet(CachePool value);
	RustDuration get cacheDurationGet;
	void cacheDurationSet(RustDuration value);
	RustDuration get updateHoldGet;
	void updateHoldSet(RustDuration value);
	factory OriginConfig(Hop hop);
}
final class OriginConsumer extends _Object {
	OriginConsumer clone();
	Hop hop();
	OriginConsumer local();
	OriginConsumer withHidden(bool hidden);
	OriginConsumer covering();
	OriginConsumer exclude(String prefix);
	OriginConsumer withStats(StatsSession session);
	AnnounceConsumer announced();
	OriginConsumer consume();
	OriginFollowing follow(String path, Instance? instance);
	OriginConsumer scope(String root, Patterns patterns);
	Cancelable<BroadcastConsumer> request(String path, Instance? instance);
	Path root();
	Patterns allowed();
	Path absolute(String path);
}
final class OriginDriver extends _Object {
	ClockInstant? poll(ClockInstant now, Waiter waiter);
}
final class OriginPrepared extends _Object {
	BroadcastProducer broadcast();
	OriginPublication publish();
}
final class OriginProducer extends _Object {
	OriginProducer clone();
	static (OriginProducer, OriginDriver) new_(OriginConfig config);
	OriginProducer withStats(StatsSession session);
	OriginProducer withLink(OriginLink link);
	OriginProducer withPreference(OriginPreference preference);
	OriginConfig config();
	Hop hop();
	OriginPublication createBroadcast(String path);
	OriginPublication publish(String path, OriginRoute route);
	OriginRequests requests(String prefix);
	OriginProducer scope(String root, Patterns patterns);
	OriginProducer mount(String at, String target);
	OriginConsumer consume();
	Path root();
	Patterns allowed();
	Path absolute(String path);
}
final class OriginPublication extends _Object {
	OriginPublication clone();
	BroadcastProducer broadcast();
	void announce(OriginRoute route);
	void unannounce();
	OriginRoute? route();
	BroadcastInfo info();
	BroadcastDemand demand();
	TrackProducer createTrack(String name, TrackInfo? info);
	DatagramProducer createDatagrams(String name, DatagramInfo info);
	TrackRequest reserveTrack(String name);
	TrackProducer uniqueTrack(String suffix, TrackInfo? info);
	String uniqueName(String suffix);
	BroadcastRequests requests();
	BroadcastConsumer consume();
	void close_();
	bool isClone(BroadcastProducer other);
}
final class OriginRequest extends _Object {
	Path path();
	OriginPrepared prepare();
	void serveProducer(BroadcastProducer broadcast);
	void serveConsumer(BroadcastConsumer broadcast);
	void reject(Error err);
}
final class OriginRequests extends _Object {
	OriginRoute route();
	Cancelable<OriginRequest> requestedBroadcast();
}
final class OriginRoute extends _Object {
	String debug();
	OriginRoute clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginRoute.default_();
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	Hops get hopsGet;
	void hopsSet(Hops value);
	OriginCost get costGet;
	void costSet(OriginCost value);
	AuthCredential? get authGet;
	void authSet(AuthCredential? value);
	OriginRoute withInstance(Instance instance, U64 epoch);
	OriginRoute withHops(Hops hops);
	OriginRoute withCost(OriginCost cost);
	OriginRoute withAuth(AuthCredential auth);
	bool isAnonymous();
	OriginSource source();
}
final class AnnounceConsumer extends _Object {
	Cancelable<AnnounceEvent?> next();
	AnnounceEvent? tryNext();
	bool isClosed();
	Cancelable<void> pollSynced();
	Path root();
	Path absolute(String prefix);
}
final class AnnounceEvent extends _Object {
	String debug();
	AnnounceEvent clone();
}
final class HangStatsSnapshot extends _Object {
	String debug();
	HangStatsSnapshot clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangStatsSnapshot.default_();
	HangStatsTransport get transportGet;
	void transportSet(HangStatsTransport value);
	List<(String, HangStatsRendition)> get renditionsGet;
	void renditionsSet(List<(String, HangStatsRendition)> value);
}
final class HangStatsSnapshotString extends _Object {
	String debug();
	HangStatsSnapshotString clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangStatsSnapshotString.default_();
	HangStatsTransport get transportGet;
	void transportSet(HangStatsTransport value);
	List<(String, HangStatsRendition)> get renditionsGet;
	void renditionsSet(List<(String, HangStatsRendition)> value);
	String get extGet;
	void extSet(String value);
}
final class HangStatsRendition extends _Object {
	String debug();
	HangStatsRendition clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangStatsRendition.default_();
	U64 get framesGet;
	void framesSet(U64 value);
	U64 get bytesGet;
	void bytesSet(U64 value);
	U64 get keyframesGet;
	void keyframesSet(U64 value);
	U64 get skippedGet;
	void skippedSet(U64 value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
}
final class HangStatsTransport extends _Object {
	String debug();
	HangStatsTransport clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangStatsTransport.default_();
	RustDuration? get rttGet;
	void rttSet(RustDuration? value);
	U64? get rateGet;
	void rateSet(U64? value);
	U64? get lostBytesGet;
	void lostBytesSet(U64? value);
	U64? get lostPacketsGet;
	void lostPacketsSet(U64? value);
	RustDuration? get ageGet;
	void ageSet(RustDuration? value);
}
final class HangEchoSnapshot extends _Object {
	String debug();
	HangEchoSnapshot clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangEchoSnapshot.default_();
	HangStatsTransport get transportGet;
	void transportSet(HangStatsTransport value);
	List<(String, HangEchoRendition)> get renditionsGet;
	void renditionsSet(List<(String, HangEchoRendition)> value);
}
final class HangEchoSnapshotString extends _Object {
	String debug();
	HangEchoSnapshotString clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangEchoSnapshotString.default_();
	HangStatsTransport get transportGet;
	void transportSet(HangStatsTransport value);
	List<(String, HangEchoRendition)> get renditionsGet;
	void renditionsSet(List<(String, HangEchoRendition)> value);
	String get extGet;
	void extSet(String value);
}
final class HangEchoRendition extends _Object {
	String debug();
	HangEchoRendition clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangEchoRendition.default_();
	U64 get framesGet;
	void framesSet(U64 value);
	U64 get bytesGet;
	void bytesSet(U64 value);
	U64 get decodedGet;
	void decodedSet(U64 value);
	U64 get lateGet;
	void lateSet(U64 value);
	U64 get errorsGet;
	void errorsSet(U64 value);
	U64 get stallsGet;
	void stallsSet(U64 value);
	U64 get underrunsGet;
	void underrunsSet(U64 value);
	RustDuration get stalledGet;
	void stalledSet(RustDuration value);
	HangEchoArrival? get arrivalGet;
	void arrivalSet(HangEchoArrival? value);
	RustDuration? get latencyGet;
	void latencySet(RustDuration? value);
}
final class HangEchoArrival extends _Object {
	String debug();
	HangEchoArrival clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Timestamp get timestampGet;
	void timestampSet(Timestamp value);
	ClockWall get wallGet;
	void wallSet(ClockWall value);
	factory HangEchoArrival(Timestamp timestamp, ClockWall wall);
}
final class HangHealthObserver extends _Object {
	String debug();
	HangHealthObserver clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Path get pathGet;
	void pathSet(Path value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	factory HangHealthObserver(Path path, Instance? instance);
}
final class HangHealthReportEchoSnapshotString extends _Object {
	String debug();
	HangHealthReportEchoSnapshotString clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthObserver get observerGet;
	void observerSet(HangHealthObserver value);
	HangEchoSnapshotString get snapshotGet;
	void snapshotSet(HangEchoSnapshotString value);
	factory HangHealthReportEchoSnapshotString(HangHealthObserver observer, HangEchoSnapshotString snapshot);
}
final class HangHealthReportStatsSnapshotString extends _Object {
	String debug();
	HangHealthReportStatsSnapshotString clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthObserver get observerGet;
	void observerSet(HangHealthObserver value);
	HangStatsSnapshotString get snapshotGet;
	void snapshotSet(HangStatsSnapshotString value);
	factory HangHealthReportStatsSnapshotString(HangHealthObserver observer, HangStatsSnapshotString snapshot);
}
final class HangHealthThresholds extends _Object {
	String debug();
	HangHealthThresholds clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangHealthThresholds.default_();
	RustDuration get staleGet;
	void staleSet(RustDuration value);
	HangHealthTransportThresholds get transportGet;
	void transportSet(HangHealthTransportThresholds value);
	HangHealthPublisherThresholds get publisherGet;
	void publisherSet(HangHealthPublisherThresholds value);
	HangHealthViewerThresholds get viewerGet;
	void viewerSet(HangHealthViewerThresholds value);
}
final class HangHealthTransportThresholds extends _Object {
	String debug();
	HangHealthTransportThresholds clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthLimitsDuration get rttGet;
	void rttSet(HangHealthLimitsDuration value);
	HangHealthLimitsF64 get lossGet;
	void lossSet(HangHealthLimitsF64 value);
	HangHealthLimitsF64 get headroomGet;
	void headroomSet(HangHealthLimitsF64 value);
}
final class HangHealthPublisherThresholds extends _Object {
	String debug();
	HangHealthPublisherThresholds clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthLimitsF64 get skippedGet;
	void skippedSet(HangHealthLimitsF64 value);
}
final class HangHealthViewerThresholds extends _Object {
	String debug();
	HangHealthViewerThresholds clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthLimitsF64 get lateGet;
	void lateSet(HangHealthLimitsF64 value);
	HangHealthLimitsF64 get errorsGet;
	void errorsSet(HangHealthLimitsF64 value);
	HangHealthLimitsF64 get stalledGet;
	void stalledSet(HangHealthLimitsF64 value);
}
final class HangHealthEvidence extends _Object {
	String debug();
	HangHealthEvidence clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangHealthInput get inputGet;
	void inputSet(HangHealthInput value);
	double? get valueGet;
	void valueSet(double? value);
	HangHealthVerdict get verdictGet;
	void verdictSet(HangHealthVerdict value);
}
final class HangHealthConnection extends _Object {
	String debug();
	HangHealthConnection clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangHealthConnection.default_();
	HangHealthVerdict get verdictGet;
	void verdictSet(HangHealthVerdict value);
	List<HangHealthEvidence> get evidenceGet;
	void evidenceSet(List<HangHealthEvidence> value);
	bool get staleGet;
	void staleSet(bool value);
	U64? get lostBytesGet;
	void lostBytesSet(U64? value);
	U64? get lostPacketsGet;
	void lostPacketsSet(U64? value);
}
final class HangHealthRendition extends _Object {
	String debug();
	HangHealthRendition clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangHealthRendition.default_();
	HangHealthVerdict get verdictGet;
	void verdictSet(HangHealthVerdict value);
	List<HangHealthEvidence> get evidenceGet;
	void evidenceSet(List<HangHealthEvidence> value);
	U64? get framesGet;
	void framesSet(U64? value);
	U64? get bytesGet;
	void bytesSet(U64? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
}
final class HangCatalogConsumer extends _Object {
	factory HangCatalogConsumer(TrackSubscriber track);
	Cancelable<HangCatalog?> next();
}
final class HangCatalogConsumerExtra extends _Object {
	factory HangCatalogConsumerExtra(TrackSubscriber track);
	Cancelable<HangCatalogExtra?> next();
}
final class HangCatalogConsumerExt extends _Object {
	factory HangCatalogConsumerExt(TrackSubscriber track);
	Cancelable<HangCatalogExt?> next();
}
final class HangCatalogAudio extends _Object {
	String debug();
	HangCatalogAudio clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogAudio.default_();
	List<(String, HangCatalogAudioConfig)> get renditionsGet;
	void renditionsSet(List<(String, HangCatalogAudioConfig)> value);
	void insert(String name, HangCatalogAudioConfig config);
	HangCatalogAudioConfig? remove(String name);
	bool isEmpty();
	List<(String, HangCatalogAudioConfig)> enabled();
	List<(String, HangCatalogAudioConfig)> ranked();
}
final class HangCatalogAudioConfig extends _Object {
	String debug();
	HangCatalogAudioConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative? get broadcastGet;
	void broadcastSet(PathRelative? value);
	String? get trackGet;
	void trackSet(String? value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	String? get labelGet;
	void labelSet(String? value);
	HangCatalogAudioCodec get codecGet;
	void codecSet(HangCatalogAudioCodec value);
	int get sampleRateGet;
	void sampleRateSet(int value);
	int get channelCountGet;
	void channelCountSet(int value);
	HangCatalogAudioLayout? get layoutGet;
	void layoutSet(HangCatalogAudioLayout? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	bool get enabledGet;
	void enabledSet(bool value);
	typed.Uint8List? get descriptionGet;
	void descriptionSet(typed.Uint8List? value);
	HangCatalogContainer get containerGet;
	void containerSet(HangCatalogContainer value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get warmupGet;
	void warmupSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	factory HangCatalogAudioConfig(HangCatalogAudioCodec codec, int sampleRate, int channelCount);
	bool sameIdentity(HangCatalogAudioConfig other);
}
final class HangCatalogBinary extends _Object {
	String debug();
	HangCatalogBinary clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogBinary.default_();
	List<(String, HangCatalogBinaryConfig)> get tracksGet;
	void tracksSet(List<(String, HangCatalogBinaryConfig)> value);
	void insert(String name, HangCatalogBinaryConfig config);
	HangCatalogBinaryConfig? remove(String name);
	bool isEmpty();
}
final class HangCatalogBinaryConfig extends _Object {
	String debug();
	HangCatalogBinaryConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative? get broadcastGet;
	void broadcastSet(PathRelative? value);
	String? get trackGet;
	void trackSet(String? value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	HangCatalogMode get modeGet;
	void modeSet(HangCatalogMode value);
	String? get mimeGet;
	void mimeSet(String? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	List<(String, Json)> get extraGet;
	void extraSet(List<(String, Json)> value);
	factory HangCatalogBinaryConfig(HangCatalogMode mode);
}
final class HangCatalogContainer extends _Object {
	String debug();
	HangCatalogContainer clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogContainer.default_();
}
final class HangCatalogContribution extends _Object {
	String debug();
	HangCatalogContribution clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative get broadcastGet;
	void broadcastSet(PathRelative value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	factory HangCatalogContribution(PathRelative broadcast);
}
final class HangCatalogEcho extends _Object {
	String debug();
	HangCatalogEcho clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative get pathGet;
	void pathSet(PathRelative value);
	factory HangCatalogEcho(PathRelative path);
}
final class HangCatalogJson extends _Object {
	String debug();
	HangCatalogJson clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogJson.default_();
	List<(String, HangCatalogJsonConfig)> get tracksGet;
	void tracksSet(List<(String, HangCatalogJsonConfig)> value);
	void insert(String name, HangCatalogJsonConfig config);
	HangCatalogJsonConfig? remove(String name);
	bool isEmpty();
}
final class HangCatalogJsonConfig extends _Object {
	String debug();
	HangCatalogJsonConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative? get broadcastGet;
	void broadcastSet(PathRelative? value);
	String? get trackGet;
	void trackSet(String? value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	HangCatalogMode get modeGet;
	void modeSet(HangCatalogMode value);
	String? get schemaGet;
	void schemaSet(String? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	List<(String, Json)> get extraGet;
	void extraSet(List<(String, Json)> value);
	factory HangCatalogJsonConfig(HangCatalogMode mode);
}
final class HangCatalogM2ts extends _Object {
	String debug();
	HangCatalogM2ts clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	String get trackGet;
	void trackSet(String value);
	bool get randomAccessGet;
	void randomAccessSet(bool value);
	U64? get muxRateGet;
	void muxRateSet(U64? value);
	factory HangCatalogM2ts(String track);
}
final class HangCatalogMedia extends _Object {
	String debug();
	HangCatalogMedia clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogMedia.default_();
	HangCatalogClock? get clockGet;
	void clockSet(HangCatalogClock? value);
	HangCatalogJoin? get joinGet;
	void joinSet(HangCatalogJoin? value);
	bool isEmpty();
	RustDuration? interval();
}
final class HangCatalogJoin extends _Object {
	String debug();
	HangCatalogJoin clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	RustDuration get intervalGet;
	void intervalSet(RustDuration value);
	static HangCatalogJoin? new_(RustDuration interval);
}
final class HangCatalogStats extends _Object {
	String debug();
	HangCatalogStats clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	String get trackGet;
	void trackSet(String value);
	factory HangCatalogStats(String track);
}
final class HangCatalogText extends _Object {
	String debug();
	HangCatalogText clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogText.default_();
	List<(String, HangCatalogTextConfig)> get renditionsGet;
	void renditionsSet(List<(String, HangCatalogTextConfig)> value);
	void insert(String name, HangCatalogTextConfig config);
	HangCatalogTextConfig? remove(String name);
	bool isEmpty();
}
final class HangCatalogTextConfig extends _Object {
	String debug();
	HangCatalogTextConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative? get broadcastGet;
	void broadcastSet(PathRelative? value);
	String? get trackGet;
	void trackSet(String? value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	HangCatalogTextFormat get formatGet;
	void formatSet(HangCatalogTextFormat value);
	HangCatalogTextRole get roleGet;
	void roleSet(HangCatalogTextRole value);
	String? get langGet;
	void langSet(String? value);
	String? get labelGet;
	void labelSet(String? value);
	HangCatalogContainer get containerGet;
	void containerSet(HangCatalogContainer value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	factory HangCatalogTextConfig(HangCatalogTextFormat format);
}
final class HangCatalogColor extends _Object {
	String debug();
	HangCatalogColor clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogColor.default_();
	int? get primariesGet;
	void primariesSet(int? value);
	int? get transferGet;
	void transferSet(int? value);
	int? get matrixGet;
	void matrixSet(int? value);
	HangCatalogRange? get rangeGet;
	void rangeSet(HangCatalogRange? value);
	HangCatalogMastering? get masteringGet;
	void masteringSet(HangCatalogMastering? value);
	HangCatalogContentLight? get contentLightGet;
	void contentLightSet(HangCatalogContentLight? value);
}
final class HangCatalogProjection extends _Object {
	String debug();
	HangCatalogProjection clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogProjection.default_();
}
final class HangCatalogVideo extends _Object {
	String debug();
	HangCatalogVideo clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogVideo.default_();
	List<(String, HangCatalogVideoConfig)> get renditionsGet;
	void renditionsSet(List<(String, HangCatalogVideoConfig)> value);
	HangCatalogDisplay? get displayGet;
	void displaySet(HangCatalogDisplay? value);
	double? get rotationGet;
	void rotationSet(double? value);
	bool? get flipGet;
	void flipSet(bool? value);
	void insert(String name, HangCatalogVideoConfig config);
	HangCatalogVideoConfig? remove(String name);
	bool isEmpty();
	List<(String, HangCatalogVideoConfig)> enabled();
	List<(String, HangCatalogVideoConfig)> ranked();
	void setProperties(HangCatalogVideoProperties properties);
}
final class HangCatalogVideoProperties extends _Object {
	String debug();
	HangCatalogVideoProperties clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogVideoProperties.default_();
	HangCatalogDisplay? get displayGet;
	void displaySet(HangCatalogDisplay? value);
	double? get rotationGet;
	void rotationSet(double? value);
	bool? get flipGet;
	void flipSet(bool? value);
}
final class HangCatalogVideoConfig extends _Object {
	String debug();
	HangCatalogVideoConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative? get broadcastGet;
	void broadcastSet(PathRelative? value);
	String? get trackGet;
	void trackSet(String? value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	String? get labelGet;
	void labelSet(String? value);
	HangCatalogVideoCodec get codecGet;
	void codecSet(HangCatalogVideoCodec value);
	typed.Uint8List? get descriptionGet;
	void descriptionSet(typed.Uint8List? value);
	int? get codedWidthGet;
	void codedWidthSet(int? value);
	int? get codedHeightGet;
	void codedHeightSet(int? value);
	int? get displayAspectWidthGet;
	void displayAspectWidthSet(int? value);
	int? get displayAspectHeightGet;
	void displayAspectHeightSet(int? value);
	HangCatalogColor? get colorGet;
	void colorSet(HangCatalogColor? value);
	HangCatalogProjection? get projectionGet;
	void projectionSet(HangCatalogProjection? value);
	HangCatalogStereo? get stereoGet;
	void stereoSet(HangCatalogStereo? value);
	bool get interlacedGet;
	void interlacedSet(bool value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	bool get enabledGet;
	void enabledSet(bool value);
	int get preferenceGet;
	void preferenceSet(int value);
	double? get framerateGet;
	void framerateSet(double? value);
	int? get reorderGet;
	void reorderSet(int? value);
	bool? get optimizeForLatencyGet;
	void optimizeForLatencySet(bool? value);
	HangCatalogContainer get containerGet;
	void containerSet(HangCatalogContainer value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get warmupGet;
	void warmupSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	factory HangCatalogVideoConfig(HangCatalogVideoCodec codec);
	bool sameIdentity(HangCatalogVideoConfig other);
}
final class MuxCatalogConsumer extends _Object {
	static Cancelable<MuxCatalogConsumer> new_(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format);
}
final class MuxCatalogConsumerExtra extends _Object {
	static Cancelable<MuxCatalogConsumerExtra> new_(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format);
}
final class MuxCatalogConsumerExt extends _Object {
	static Cancelable<MuxCatalogConsumerExt> new_(BroadcastConsumer broadcast, MuxCatalogCatalogFormat format);
}
final class MuxCatalogEstimate extends _Object {
	String debug();
	MuxCatalogEstimate clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxCatalogEstimate.default_();
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	double? get framerateGet;
	void framerateSet(double? value);
	MuxCatalogEstimate withJitter(RustDuration? jitter);
	MuxCatalogEstimate withBitrate(U64? bitrate);
	MuxCatalogEstimate withDelay(RustDuration? delay);
	MuxCatalogEstimate withFramerate(double? framerate);
}
final class MuxCatalogEstimator extends _Object {
	factory MuxCatalogEstimator.default_();
	factory MuxCatalogEstimator();
	void write(Timestamp timestamp, U64 bytes);
	void cut(Timestamp? end);
	void discontinuity();
	void reorder(Timestamp delay);
	MuxCatalogEstimate estimate();
}
final class MuxCatalogConfig extends _Object {
	MuxCatalogConfig clone();
	factory MuxCatalogConfig.default_();
	MuxCatalogConfigExtra withCatalogExtra(HangCatalogExtra catalog);
	MuxCatalogConfigExt withCatalogExt(HangCatalogExt catalog);
	MuxCatalogConfig withCatalogUnit(HangCatalog catalog);
	MuxCatalogConfig withClock(MuxClock clock);
	MuxCatalogConfig withMaxAge(RustDuration? maxAge);
	MuxCatalogConfig withBandwidth(BandwidthAllocator bandwidth);
}
final class MuxCatalogConfigExtra extends _Object {
	MuxCatalogConfigExtra clone();
	MuxCatalogConfigExtra withCatalogExtra(HangCatalogExtra catalog);
	MuxCatalogConfigExt withCatalogExt(HangCatalogExt catalog);
	MuxCatalogConfig withCatalogUnit(HangCatalog catalog);
	MuxCatalogConfigExtra withClock(MuxClock clock);
	MuxCatalogConfigExtra withMaxAge(RustDuration? maxAge);
	MuxCatalogConfigExtra withBandwidth(BandwidthAllocator bandwidth);
}
final class MuxCatalogConfigExt extends _Object {
	MuxCatalogConfigExt clone();
	MuxCatalogConfigExtra withCatalogExtra(HangCatalogExtra catalog);
	MuxCatalogConfigExt withCatalogExt(HangCatalogExt catalog);
	MuxCatalogConfig withCatalogUnit(HangCatalog catalog);
	MuxCatalogConfigExt withClock(MuxClock clock);
	MuxCatalogConfigExt withMaxAge(RustDuration? maxAge);
	MuxCatalogConfigExt withBandwidth(BandwidthAllocator bandwidth);
}
final class MuxCatalogGuard extends _Object {
	void commit();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	void checkRenditions();
	HangCatalog media();
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class MuxCatalogGuardExtra extends _Object {
	void commit();
	void setSection(String name, Json value);
	Json? removeSection(String name);
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	MuxCatalogHangExtra get extGet;
	void extSet(MuxCatalogHangExtra value);
	void checkRenditions();
	HangCatalog media();
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class MuxCatalogGuardExt extends _Object {
	void commit();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogMedia get mediaGet;
	void mediaSet(HangCatalogMedia value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogM2ts? get m2tsGet;
	void m2tsSet(HangCatalogM2ts? value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogContribution)> get contributionsGet;
	void contributionsSet(List<(String, HangCatalogContribution)> value);
	MuxContainerTsExt get extGet;
	void extSet(MuxContainerTsExt value);
	void checkRenditions();
	HangCatalog media();
	String toJson();
	String toJsonPretty();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class MuxCatalogProducer extends _Object {
	MuxCatalogProducer clone();
	factory MuxCatalogProducer(BroadcastProducer broadcast, MuxCatalogConfig config);
	TrackInfo trackInfo(int priority);
	Timestamp timestamp(Timestamp? hint);
	void mutate(void Function(HangCatalog) f);
	MuxCatalogGuard modify();
	HangCatalog snapshot();
	MuxClock clock();
	MuxCatalogTimebase timebase();
	MuxCatalogReserved reserve();
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	bool isClaimedJsonConfig(String name);
	bool isClaimedBinaryConfig(String name);
	bool isClaimedVideoConfig(String name);
	bool isClaimedAudioConfig(String name);
	bool isClaimedTextConfig(String name);
	List<(String, HangStatsRendition)> stats();
	MuxJsonSnapshotJson jsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
	MuxJsonSnapshotJson jsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
	MuxJsonStreamJson jsonStreamConfig(TrackProducer track, MuxJsonConfig config);
	MuxJsonStreamJson jsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
	MuxCatalogHangConsumer consume();
	void abort(Error err);
	void finish();
}
final class MuxCatalogProducerExtra extends _Object {
	MuxCatalogProducerExtra clone();
	factory MuxCatalogProducerExtra(BroadcastProducer broadcast, MuxCatalogConfigExtra config);
	TrackInfo trackInfo(int priority);
	Timestamp timestamp(Timestamp? hint);
	void mutate(void Function(HangCatalogExtra) f);
	MuxCatalogGuardExtra modify();
	HangCatalogExtra snapshot();
	MuxClock clock();
	MuxCatalogTimebaseExtra timebase();
	MuxCatalogReservedExtra reserve();
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	bool isClaimedJsonConfig(String name);
	bool isClaimedBinaryConfig(String name);
	bool isClaimedVideoConfig(String name);
	bool isClaimedAudioConfig(String name);
	bool isClaimedTextConfig(String name);
	List<(String, HangStatsRendition)> stats();
	MuxJsonSnapshotJson jsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
	MuxJsonSnapshotJson jsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
	MuxJsonStreamJson jsonStreamConfig(TrackProducer track, MuxJsonConfig config);
	MuxJsonStreamJson jsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
	MuxCatalogHangConsumerExtra consume();
	void abort(Error err);
	void finish();
}
final class MuxCatalogProducerExt extends _Object {
	MuxCatalogProducerExt clone();
	factory MuxCatalogProducerExt(BroadcastProducer broadcast, MuxCatalogConfigExt config);
	TrackInfo trackInfo(int priority);
	Timestamp timestamp(Timestamp? hint);
	void mutate(void Function(HangCatalogExt) f);
	MuxCatalogGuardExt modify();
	HangCatalogExt snapshot();
	MuxClock clock();
	MuxCatalogTimebaseExt timebase();
	MuxCatalogReservedExt reserve();
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	bool isClaimedJsonConfig(String name);
	bool isClaimedBinaryConfig(String name);
	bool isClaimedVideoConfig(String name);
	bool isClaimedAudioConfig(String name);
	bool isClaimedTextConfig(String name);
	List<(String, HangStatsRendition)> stats();
	MuxJsonSnapshotJson jsonSnapshotConfig(TrackProducer track, MuxJsonConfig config, MuxJsonOptions options);
	MuxJsonSnapshotJson jsonSnapshotJsonConfig(TrackProducer track, HangCatalogJsonConfig config, MuxJsonOptions options);
	MuxJsonStreamJson jsonStreamConfig(TrackProducer track, MuxJsonConfig config);
	MuxJsonStreamJson jsonStreamJsonConfig(TrackProducer track, HangCatalogJsonConfig config);
	MuxCatalogHangConsumerExt consume();
	void abort(Error err);
	void finish();
}
final class MuxCatalogTimebase extends _Object {
	MuxCatalogTimebase clone();
	MuxCatalogReserved reserve();
	void place(Timestamp pts, ClockWall wall);
}
final class MuxCatalogTimebaseExtra extends _Object {
	MuxCatalogTimebaseExtra clone();
	MuxCatalogReservedExtra reserve();
	void place(Timestamp pts, ClockWall wall);
}
final class MuxCatalogTimebaseExt extends _Object {
	MuxCatalogTimebaseExt clone();
	MuxCatalogReservedExt reserve();
	void place(Timestamp pts, ClockWall wall);
}
final class MuxCatalogReserved extends _Object {
	MuxCatalogReserved clone();
	MuxCatalogReserved withInterval(RustDuration? interval);
	TrackInfo trackInfo(int priority);
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	Timestamp timestamp(Timestamp? hint);
}
final class MuxCatalogReservedExtra extends _Object {
	MuxCatalogReservedExtra clone();
	MuxCatalogReservedExtra withInterval(RustDuration? interval);
	TrackInfo trackInfo(int priority);
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	Timestamp timestamp(Timestamp? hint);
}
final class MuxCatalogReservedExt extends _Object {
	MuxCatalogReservedExt clone();
	MuxCatalogReservedExt withInterval(RustDuration? interval);
	TrackInfo trackInfo(int priority);
	MuxContainerProducerContainerVideoConfig videoContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireVideoConfig videoFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireVideoConfig videoLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireVideoConfig videoLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig audioContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireAudioConfig audioFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireAudioConfig audioLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireAudioConfig audioLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig textContainer(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireTextConfig textFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireTextConfig textLegacyWire(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireTextConfig textLocWire(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	MuxContainerProducerContainerJsonConfig trackContainerJsonConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogJsonConfig? config);
	MuxContainerProducerContainerBinaryConfig trackContainerBinaryConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogBinaryConfig? config);
	MuxContainerProducerContainerVideoConfig trackContainerVideoConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogVideoConfig? config);
	MuxContainerProducerContainerAudioConfig trackContainerAudioConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogAudioConfig? config);
	MuxContainerProducerContainerTextConfig trackContainerTextConfig(TrackProducer track, MuxCatalogHangContainer container, HangCatalogTextConfig? config);
	MuxContainerProducerFmp4WireJsonConfig trackFmp4WireJsonConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogJsonConfig? config);
	MuxContainerProducerFmp4WireBinaryConfig trackFmp4WireBinaryConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerFmp4WireVideoConfig trackFmp4WireVideoConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogVideoConfig? config);
	MuxContainerProducerFmp4WireAudioConfig trackFmp4WireAudioConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogAudioConfig? config);
	MuxContainerProducerFmp4WireTextConfig trackFmp4WireTextConfig(TrackProducer track, MuxContainerFmp4Wire container, HangCatalogTextConfig? config);
	MuxContainerProducerLegacyWireJsonConfig trackLegacyWireJsonConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLegacyWireBinaryConfig trackLegacyWireBinaryConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLegacyWireVideoConfig trackLegacyWireVideoConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLegacyWireAudioConfig trackLegacyWireAudioConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLegacyWireTextConfig trackLegacyWireTextConfig(TrackProducer track, MuxContainerLegacyWire container, HangCatalogTextConfig? config);
	MuxContainerProducerLocWireJsonConfig trackLocWireJsonConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogJsonConfig? config);
	MuxContainerProducerLocWireBinaryConfig trackLocWireBinaryConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogBinaryConfig? config);
	MuxContainerProducerLocWireVideoConfig trackLocWireVideoConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogVideoConfig? config);
	MuxContainerProducerLocWireAudioConfig trackLocWireAudioConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogAudioConfig? config);
	MuxContainerProducerLocWireTextConfig trackLocWireTextConfig(TrackProducer track, MuxContainerLocWire container, HangCatalogTextConfig? config);
	Timestamp timestamp(Timestamp? hint);
}
final class MuxCatalogVideoHint extends _Object {
	String debug();
	MuxCatalogVideoHint clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxCatalogVideoHint.default_();
	HangCatalogVideoCodec? get codecGet;
	void codecSet(HangCatalogVideoCodec? value);
	int? get codedWidthGet;
	void codedWidthSet(int? value);
	int? get codedHeightGet;
	void codedHeightSet(int? value);
	int? get displayAspectWidthGet;
	void displayAspectWidthSet(int? value);
	int? get displayAspectHeightGet;
	void displayAspectHeightSet(int? value);
	HangCatalogColor? get colorGet;
	void colorSet(HangCatalogColor? value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	double? get framerateGet;
	void framerateSet(double? value);
	int? get reorderGet;
	void reorderSet(int? value);
	bool? get optimizeForLatencyGet;
	void optimizeForLatencySet(bool? value);
	RustDuration? get jitterGet;
	void jitterSet(RustDuration? value);
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	HangCatalogContainer get containerGet;
	void containerSet(HangCatalogContainer value);
	int get preferenceGet;
	void preferenceSet(int value);
	void apply(HangCatalogVideoConfig config);
	HangCatalogVideoConfig? toConfig();
}
final class MuxContainerConsumerContainer extends _Object {
	factory MuxContainerConsumerContainer(TrackSubscriber track, MuxCatalogHangContainer format);
	MuxContainerConsumerContainer withWarmup(RustDuration warmup);
	MuxContainerConsumerContainer withDelay(TrackDelay delay);
	Cancelable<MuxContainerRead?> read();
	void setDelay(TrackDelay delay);
	void setMaxBuffer(U64 bytes);
}
final class MuxContainerConsumerFmp4Wire extends _Object {
	factory MuxContainerConsumerFmp4Wire(TrackSubscriber track, MuxContainerFmp4Wire format);
	MuxContainerConsumerFmp4Wire withWarmup(RustDuration warmup);
	MuxContainerConsumerFmp4Wire withDelay(TrackDelay delay);
	Cancelable<MuxContainerRead?> read();
	void setDelay(TrackDelay delay);
	void setMaxBuffer(U64 bytes);
}
final class MuxContainerConsumerLegacyWire extends _Object {
	factory MuxContainerConsumerLegacyWire(TrackSubscriber track, MuxContainerLegacyWire format);
	MuxContainerConsumerLegacyWire withWarmup(RustDuration warmup);
	MuxContainerConsumerLegacyWire withDelay(TrackDelay delay);
	Cancelable<MuxContainerRead?> read();
	void setDelay(TrackDelay delay);
	void setMaxBuffer(U64 bytes);
}
final class MuxContainerConsumerLocWire extends _Object {
	factory MuxContainerConsumerLocWire(TrackSubscriber track, MuxContainerLocWire format);
	MuxContainerConsumerLocWire withWarmup(RustDuration warmup);
	MuxContainerConsumerLocWire withDelay(TrackDelay delay);
	Cancelable<MuxContainerRead?> read();
	void setDelay(TrackDelay delay);
	void setMaxBuffer(U64 bytes);
}
final class MuxContainerGroupConsumerContainer extends _Object {
	factory MuxContainerGroupConsumerContainer(GroupConsumer group, MuxCatalogHangContainer format);
	U64 sequence();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerFmp4Wire extends _Object {
	factory MuxContainerGroupConsumerFmp4Wire(GroupConsumer group, MuxContainerFmp4Wire format);
	U64 sequence();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerLegacyWire extends _Object {
	factory MuxContainerGroupConsumerLegacyWire(GroupConsumer group, MuxContainerLegacyWire format);
	U64 sequence();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerLocWire extends _Object {
	factory MuxContainerGroupConsumerLocWire(GroupConsumer group, MuxContainerLocWire format);
	U64 sequence();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerProducerContainerAudioConfig extends _Object {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainerAudioConfig withBuffer(RustDuration duration);
	MuxContainerProducerContainerAudioConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerBinaryConfig extends _Object {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainerBinaryConfig withBuffer(RustDuration duration);
	MuxContainerProducerContainerBinaryConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerJsonConfig extends _Object {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainerJsonConfig withBuffer(RustDuration duration);
	MuxContainerProducerContainerJsonConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerTextConfig extends _Object {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainerTextConfig withBuffer(RustDuration duration);
	MuxContainerProducerContainerTextConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerVideoConfig extends _Object {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainerVideoConfig withBuffer(RustDuration duration);
	MuxContainerProducerContainerVideoConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainer extends _Object {
	factory MuxContainerProducerContainer(TrackProducer track, MuxCatalogHangContainer container);
	void set();
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerContainer withBuffer(RustDuration duration);
	MuxContainerProducerContainer withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireAudioConfig extends _Object {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4WireAudioConfig withBuffer(RustDuration duration);
	MuxContainerProducerFmp4WireAudioConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireBinaryConfig extends _Object {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4WireBinaryConfig withBuffer(RustDuration duration);
	MuxContainerProducerFmp4WireBinaryConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireJsonConfig extends _Object {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4WireJsonConfig withBuffer(RustDuration duration);
	MuxContainerProducerFmp4WireJsonConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireTextConfig extends _Object {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4WireTextConfig withBuffer(RustDuration duration);
	MuxContainerProducerFmp4WireTextConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireVideoConfig extends _Object {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4WireVideoConfig withBuffer(RustDuration duration);
	MuxContainerProducerFmp4WireVideoConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4Wire extends _Object {
	factory MuxContainerProducerFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container);
	void set();
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerFmp4Wire withBuffer(RustDuration duration);
	MuxContainerProducerFmp4Wire withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireAudioConfig extends _Object {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWireAudioConfig withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWireAudioConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireBinaryConfig extends _Object {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWireBinaryConfig withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWireBinaryConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireJsonConfig extends _Object {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWireJsonConfig withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWireJsonConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireTextConfig extends _Object {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWireTextConfig withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWireTextConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireVideoConfig extends _Object {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWireVideoConfig withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWireVideoConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWire extends _Object {
	factory MuxContainerProducerLegacyWire(TrackProducer track, MuxContainerLegacyWire container);
	void set();
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLegacyWire withBuffer(RustDuration duration);
	MuxContainerProducerLegacyWire withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireAudioConfig extends _Object {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWireAudioConfig withBuffer(RustDuration duration);
	MuxContainerProducerLocWireAudioConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireBinaryConfig extends _Object {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWireBinaryConfig withBuffer(RustDuration duration);
	MuxContainerProducerLocWireBinaryConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireJsonConfig extends _Object {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWireJsonConfig withBuffer(RustDuration duration);
	MuxContainerProducerLocWireJsonConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireTextConfig extends _Object {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWireTextConfig withBuffer(RustDuration duration);
	MuxContainerProducerLocWireTextConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireVideoConfig extends _Object {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWireVideoConfig withBuffer(RustDuration duration);
	MuxContainerProducerLocWireVideoConfig withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWire extends _Object {
	factory MuxContainerProducerLocWire(TrackProducer track, MuxContainerLocWire container);
	void set();
	void setBitrate(U64 bitrate);
	Timestamp timestamp(Timestamp? hint);
	String name();
	TrackDemand demand();
	MuxCatalogEstimate estimate();
	void reorder(Timestamp delay);
	void history(Timestamp until);
	bool needsKeyframe();
	MuxContainerProducerLocWire withBuffer(RustDuration duration);
	MuxContainerProducerLocWire withBandwidth(BandwidthAllocator allocator);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	TrackProducer track();
	Timestamp? liveEdge();
	void write(MuxContainerFrame frame);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	void discontinuity();
	void finish();
	void fail(Error err);
	void abort(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup();
	void dropGroups(RangeU64 groups, StreamError err);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxFollowInstances extends _Object {
	factory MuxFollowInstances(OriginConsumer origin, String path, Instance? instance);
	Cancelable<MuxFollowEvent> next();
}
final class MuxFollowEvent extends _Object {
}
final class MuxImportAudioInit extends _Object {
	String debug();
	MuxImportAudioInit clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	MuxImportAudioFormat get formatGet;
	void formatSet(MuxImportAudioFormat value);
	typed.Uint8List get dataGet;
	void dataSet(typed.Uint8List value);
	String? get labelGet;
	void labelSet(String? value);
	HangCatalogContainer get containerGet;
	void containerSet(HangCatalogContainer value);
	U64? get bitrateGet;
	void bitrateSet(U64? value);
	factory MuxImportAudioInit(MuxImportAudioFormat format, typed.Uint8List data);
}
final class MuxImportContainerInit extends _Object {
	String debug();
	MuxImportContainerInit clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	MuxImportContainerFormat get formatGet;
	void formatSet(MuxImportContainerFormat value);
	typed.Uint8List get dataGet;
	void dataSet(typed.Uint8List value);
	factory MuxImportContainerInit(MuxImportContainerFormat format, typed.Uint8List data);
}
final class MuxImportVideoInit extends _Object {
	String debug();
	MuxImportVideoInit clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	MuxImportVideoFormat get formatGet;
	void formatSet(MuxImportVideoFormat value);
	typed.Uint8List get dataGet;
	void dataSet(typed.Uint8List value);
	String? get labelGet;
	void labelSet(String? value);
	MuxCatalogVideoHint get hintGet;
	void hintSet(MuxCatalogVideoHint value);
	int get priorityGet;
	void prioritySet(int value);
	factory MuxImportVideoInit(MuxImportVideoFormat format, typed.Uint8List data);
}
final class MuxImportContainer extends _Object {
	factory MuxImportContainer(BroadcastProducer broadcast, MuxCatalogReserved reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void cut();
	void seek(U64 sequence);
}
final class MuxImportContainerExtra extends _Object {
	factory MuxImportContainerExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void cut();
	void seek(U64 sequence);
}
final class MuxImportContainerExt extends _Object {
	factory MuxImportContainerExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void cut();
	void seek(U64 sequence);
}
final class MuxImportTrack extends _Object {
	factory MuxImportTrack.audioExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportAudioInit init);
	factory MuxImportTrack.audioExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportAudioInit init);
	factory MuxImportTrack.audioUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportAudioInit init);
	factory MuxImportTrack.videoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
	factory MuxImportTrack.videoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
	factory MuxImportTrack.videoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
	void decode(typed.Uint8List frame, Timestamp? pts);
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 sequence);
	TrackDemand demand();
	String name();
}
final class MuxImportTrackStream extends _Object {
	factory MuxImportTrackStream.videoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
	factory MuxImportTrackStream.videoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
	factory MuxImportTrackStream.videoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
	void initialize(typed.Uint8List data);
	void decode(typed.Uint8List data);
	void finish();
	void setEnabled(bool enabled);
	void abort(Error err);
	void cut(Timestamp? end);
	void seek(U64 sequence);
	TrackDemand demand();
	String name();
}
final class MuxJsonSnapshotJson extends _Object {
	String name();
	TrackSubscriber consume();
	TrackDemand demand();
	Cancelable<void> refresh();
	void update(Json value, Timestamp timestamp);
	void finish();
	void abort(Error err);
}
final class MuxJsonStreamJson extends _Object {
	String name();
	TrackSubscriber consume();
	TrackDemand demand();
	void append(Json value, Timestamp timestamp);
	void finish();
	void abort(Error err);
}
final class MuxJsonConsumerJson extends _Object {
	factory MuxJsonConsumerJson.fromTrack(TrackSubscriber track, HangCatalogJsonConfig config);
	HangCatalogMode mode();
	Cancelable<TimedJson?> next();
	Cancelable<TimedJson?> latest();
}
final class MuxJsonConfig extends _Object {
	String debug();
	MuxJsonConfig clone();
	factory MuxJsonConfig.default_();
	String? get schemaGet;
	void schemaSet(String? value);
	MuxJsonConfig withSchema(String schema);
}
final class MuxJsonOptions extends _Object {
	String debug();
	MuxJsonOptions clone();
	factory MuxJsonOptions.default_();
	bool get deltasGet;
	void deltasSet(bool value);
	MuxJsonOptions withDeltas(bool deltas);
}
final class MuxRateFollower extends _Object {
	factory MuxRateFollower(BandwidthConsumer grant, MuxRatePolicy policy);
	MuxRatePolicy policy();
	void setPolicy(MuxRatePolicy policy);
	Cancelable<MuxRateTarget> next();
	void failed();
}
final class MuxRatePolicy extends _Object {
	String debug();
	MuxRatePolicy clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	BandwidthRate get maxGet;
	void maxSet(BandwidthRate value);
	BandwidthRate get minGet;
	void minSet(BandwidthRate value);
	BandwidthRate? get lowerGet;
	void lowerSet(BandwidthRate? value);
	factory MuxRatePolicy(BandwidthRate max);
}
final class MuxRateControl extends _Object {
	String debug();
	MuxRateControl clone();
	factory MuxRateControl(MuxRatePolicy policy);
	MuxRateTarget target();
	void setProbed(bool probed);
	MuxRateTarget? update(BandwidthRate? estimate, ClockInstant now);
}
final class MuxSelectBroadcast extends _Object {
	String debug();
	MuxSelectBroadcast clone();
	factory MuxSelectBroadcast.default_();
	MuxSelectBroadcast video(MuxSelectVideo video);
	MuxSelectBroadcast audio(MuxSelectAudio audio);
	bool hasVideo();
	bool hasAudio();
	void retainExtra(HangCatalogExtra catalog, MuxSelectSelected selected);
	void retainExt(HangCatalogExt catalog, MuxSelectSelected selected);
	void retainUnit(HangCatalog catalog, MuxSelectSelected selected);
}
final class MuxSelectVideo extends _Object {
	String debug();
	MuxSelectVideo clone();
	factory MuxSelectVideo.default_();
	MuxSelectVideo name(String name);
	MuxSelectVideo codec(HangCatalogVideoCodecKind codec);
}
final class MuxSelectAudio extends _Object {
	String debug();
	MuxSelectAudio clone();
	factory MuxSelectAudio.default_();
	MuxSelectAudio name(String name);
	MuxSelectAudio codec(HangCatalogAudioCodecKind codec);
}
final class MuxTimelineTrack extends _Object {
	void retire(ClockInstant? drained);
	Timestamp map(Timestamp source);
	MuxTimelineTransition declare();
	void end(Timestamp source);
	void plan(MuxTimelineTransition transition, Timestamp source, RustDuration? delay);
	void gap();
	void abandon(MuxTimelineTransition transition);
}
final class MuxTimelineProducer extends _Object {
	MuxTimelineProducer clone();
	factory MuxTimelineProducer(RustDuration delay);
	MuxTimelineProducer reset(RustDuration delay);
	MuxTimelineConsumer consume();
	MuxTimelineTrack track(MuxTimelineKind kind);
	bool video(Timestamp timestamp, ClockInstant now);
	void videoRestarted();
	bool audio(MuxTimelineAudio audio, ClockInstant now);
	void audioRestarted();
	void audioStopped();
	ClockInstant? due(Timestamp timestamp);
}
final class MuxTimelineConsumer extends _Object {
	MuxTimelineConsumer clone();
	RustDuration? playhead(ClockInstant now);
	RustDuration? target();
	ClockInstant? due(Timestamp timestamp);
	U64? present(List<Timestamp> timestamps, ClockInstant now);
}
final class TokioTlsError extends _Object implements Exception {
	@override String toString();
	String debug();
}
final class TokioTlsIdentity extends _Object {
	String debug();
	TokioTlsIdentity clone();
	factory TokioTlsIdentity.generate(List<String> hostnames);
	String fingerprint();
}
final class TokioTlsPeers extends _Object {
	String debug();
	TokioTlsPeers clone();
	factory TokioTlsPeers.default_();
	factory TokioTlsPeers();
	void insert(String fingerprint);
	void remove(String fingerprint);
	bool contains(String fingerprint);
}
final class TokioTlsConnect extends _Object {
	String debug();
	TokioTlsConnect clone();
	factory TokioTlsConnect.default_();
	List<PathBuf> get rootGet;
	void rootSet(List<PathBuf> value);
	bool? get systemRootsGet;
	void systemRootsSet(bool? value);
	List<String> get fingerprintGet;
	void fingerprintSet(List<String> value);
	TokioTlsIdentity? get identityGet;
	void identitySet(TokioTlsIdentity? value);
	PathBuf? get certGet;
	void certSet(PathBuf? value);
	PathBuf? get keyGet;
	void keySet(PathBuf? value);
	bool? get insecureGet;
	void insecureSet(bool? value);
	String? get hostNameGet;
	void hostNameSet(String? value);
	MoqTokioCliDeprecated deprecated();
	RustlsClientConfig build();
}
final class TokioTlsListen extends _Object {
	String debug();
	TokioTlsListen clone();
	factory TokioTlsListen.default_();
	List<PathBuf> get certGet;
	void certSet(List<PathBuf> value);
	List<PathBuf> get keyGet;
	void keySet(List<PathBuf> value);
	List<String> get generateGet;
	void generateSet(List<String> value);
	TokioTlsIdentity? get identityGet;
	void identitySet(TokioTlsIdentity? value);
	TokioTlsPeers? get peersGet;
	void peersSet(TokioTlsPeers? value);
	List<PathBuf> get rootGet;
	void rootSet(List<PathBuf> value);
	TokioTlsServe load();
	MoqTokioCliDeprecated deprecated();
	RustlsRootCertStore loadRoots();
	RustlsServerConfig serverConfig(List<typed.Uint8List> alpn);
}
final class TokioTlsServe extends _Object {
	String debug();
	TokioTlsServe clone();
	TokioTlsCertificates certificates();
	RustlsServerConfig config(List<typed.Uint8List> alpn);
}
final class TokioTlsPeerIdentity extends _Object {
	String debug();
	TokioTlsPeerIdentity clone();
	factory TokioTlsPeerIdentity.fromChain(List<RustlsPkiTypesCertificateDer> chain);
	List<RustlsPkiTypesCertificateDer> chain();
	String? fingerprint();
	String? name();
	String? issuer();
	ClockWall? expiry();
}
final class TokioTlsCertificates extends _Object {
	String debug();
	TokioTlsCertificates clone();
	factory TokioTlsCertificates.fromPem(typed.Uint8List chain);
	List<String> fingerprints();
}
final class TokioTlsKeyLog extends _Object {
	String debug();
	factory TokioTlsKeyLog.open(PathBuf path);
}
final class TokioClientConfig extends _Object {
	String debug();
	TokioClientConfig clone();
	factory TokioClientConfig.default_();
	TokioConnectConfig get connectGet;
	void connectSet(TokioConnectConfig value);
	TokioQuicConfig get quicGet;
	void quicSet(TokioQuicConfig value);
	TokioClient init();
}
final class TokioConnectConfig extends _Object {
	String debug();
	TokioConnectConfig clone();
	factory TokioConnectConfig.default_();
	Url? get urlGet;
	void urlSet(Url? value);
	List<AuthToken> get tokensGet;
	void tokensSet(List<AuthToken> value);
	SocketAddr? get bindGet;
	void bindSet(SocketAddr? value);
	TokioDuration? get raceGet;
	void raceSet(TokioDuration? value);
	TokioDuration? get resolutionDelayGet;
	void resolutionDelaySet(TokioDuration? value);
	TokioDuration? get timeoutGet;
	void timeoutSet(TokioDuration? value);
	List<Version> get versionGet;
	void versionSet(List<Version> value);
	SetupExtensions get extensionsGet;
	void extensionsSet(SetupExtensions value);
	String? get implementationGet;
	void implementationSet(String? value);
	TokioTlsConnect get tlsGet;
	void tlsSet(TokioTlsConnect value);
	TokioBackoff get backoffGet;
	void backoffSet(TokioBackoff value);
	MoqTokioConnectionGoaway get goawayGet;
	void goawaySet(MoqTokioConnectionGoaway value);
	MoqTokioWebsocketConfig get websocketGet;
	void websocketSet(MoqTokioWebsocketConfig value);
	MoqTokioCliDeprecated deprecated();
	TokioClient init(TokioQuicConfig quic);
	Versions versions();
	MoqTokioConnectResolved resolve();
}
final class TokioConnectionMonitor extends _Object {
	TokioConnectionMonitor clone();
	MoqTokioConnectionObservation state();
	Cancelable<MoqTokioConnectionObservation> stateChanged(U64 after);
	StatsPresence presence();
	Cancelable<StatsPresence> pollPresence();
	Cancelable<StatsPresence> presenceChanged();
	SessionStats? stats();
	TokioError? failure();
	MoqTokioConnectionSnapshot? snapshot();
}
final class TokioListenConfig extends _Object {
	String debug();
	TokioListenConfig clone();
	factory TokioListenConfig.default_();
	TokioListenBind? get bindGet;
	void bindSet(TokioListenBind? value);
	MoqTokioTcpConfig get tcpGet;
	void tcpSet(MoqTokioTcpConfig value);
	MoqTokioUnixConfig get unixGet;
	void unixSet(MoqTokioUnixConfig value);
	List<Version> get versionGet;
	void versionSet(List<Version> value);
	SetupExtensions get extensionsGet;
	void extensionsSet(SetupExtensions value);
	String? get implementationGet;
	void implementationSet(String? value);
	TokioDuration? get timeoutGet;
	void timeoutSet(TokioDuration? value);
	TokioTlsListen get tlsGet;
	void tlsSet(TokioTlsListen value);
	SocketAddrV4? get preferredV4Get;
	void preferredV4Set(SocketAddrV4? value);
	SocketAddrV6? get preferredV6Get;
	void preferredV6Set(SocketAddrV6? value);
	MoqTokioListenLoadBalancer get lbGet;
	void lbSet(MoqTokioListenLoadBalancer value);
	MoqTokioListenReset get resetGet;
	void resetSet(MoqTokioListenReset value);
	MoqTokioCliDeprecated deprecated();
	RustDuration? resolvedTimeout();
	MoqTokioListenResolved resolve();
	TokioServer init(TokioQuicConfig quic);
	TokioServer initStreams();
	Versions versions();
	bool hasExplicitBind();
}
final class TokioQuicConfig extends _Object {
	String debug();
	TokioQuicConfig clone();
	factory TokioQuicConfig.default_();
	U64? get maxStreamsGet;
	void maxStreamsSet(U64? value);
	bool? get gsoGet;
	void gsoSet(bool? value);
	TokioDuration? get idleTimeoutGet;
	void idleTimeoutSet(TokioDuration? value);
	TokioDuration? get keepAliveGet;
	void keepAliveSet(TokioDuration? value);
	bool? get mtuDiscoveryGet;
	void mtuDiscoverySet(bool? value);
	MoqTokioQuicCongestionControl? get congestionControlGet;
	void congestionControlSet(MoqTokioQuicCongestionControl? value);
	MoqTokioQuicController? get controllerGet;
	void controllerSet(MoqTokioQuicController? value);
	U64? get receiveWindowGet;
	void receiveWindowSet(U64? value);
	U64? get streamReceiveWindowGet;
	void streamReceiveWindowSet(U64? value);
	U64? get sendWindowGet;
	void sendWindowSet(U64? value);
	PathBuf? get qlogGet;
	void qlogSet(PathBuf? value);
	PathBuf? get keylogGet;
	void keylogSet(PathBuf? value);
	MoqTokioCliDeprecated deprecated();
	void validate();
	MoqTokioQuicResolved resolve();
}
final class TokioServerConfig extends _Object {
	factory TokioServerConfig.default_();
	TokioTlsServe? get tlsGet;
	void tlsSet(TokioTlsServe? value);
	ServerActivity get activityGet;
	void activitySet(ServerActivity value);
	TokioListenConfig get listenGet;
	void listenSet(TokioListenConfig value);
	TokioQuicConfig get quicGet;
	void quicSet(TokioQuicConfig value);
	MoqTokioWebsocketListener? get websocketGet;
	void websocketSet(MoqTokioWebsocketListener? value);
	OriginConsumer? get publisherGet;
	void publisherSet(OriginConsumer? value);
	OriginProducer? get subscriberGet;
	void subscriberSet(OriginProducer? value);
	StatsSession get statsGet;
	void statsSet(StatsSession value);
	MoqTokioQuicDemuxDemux get demuxGet;
	void demuxSet(MoqTokioQuicDemuxDemux value);
	TokioServer init();
	TokioServer initStreams();
}
final class TokioServerListener extends _Object {
	Cancelable<TokioServerRequest> accept();
	Cancelable<void> close_();
	SocketAddr localAddr();
	SocketAddr? websocketLocalAddr();
	SocketAddr? tcpLocalAddr();
	TokioTlsCertificates certificates();
	List<MoqTokioAcceptHealth> acceptHealth();
}
final class TokioServerRequest extends _Object {
	Cancelable<void> reject(SessionError code);
	TokioServerRequest withPublisherProducer(OriginProducer publish);
	TokioServerRequest withPublisherConsumer(OriginConsumer publish);
	TokioServerRequest withSubscriber(OriginProducer subscribe);
	TokioServerRequest withPeerHop(Hop hop);
	TokioServerRequest withStats(StatsSession stats);
	TokioServerRequest withLimits(SessionLimits limits);
	MoqTokioQuicDriverConnectionConnection? quic();
	TokioServerRequest withQuicLimits(MoqQuicLimits limits);
	Cancelable<Session> ok();
	TokioTransport transport();
	Url? url();
	String? authority();
	String path();
	String? query();
	SocketAddr? remoteAddr();
	SocketAddr? localAddr();
	String? serverName();
	String? alpn();
	SetupRole? role();
	AuthHandle auth();
	AuthRequests requests();
	Hop? peerHop();
	String? peerImplementation();
	SetupToken? token();
	TokioTlsPeerIdentity? peerIdentity();
}
final class TransportMemoryConfig extends _Object {
	String debug();
	TransportMemoryConfig clone();
	factory TransportMemoryConfig.default_();
	U64 get streamBufferGet;
	void streamBufferSet(U64 value);
	String? get protocolGet;
	void protocolSet(String? value);
	U64? get maxConcurrentUniStreamsGet;
	void maxConcurrentUniStreamsSet(U64? value);
	U64? get maxConcurrentBiStreamsGet;
	void maxConcurrentBiStreamsSet(U64? value);
}
final class TransportMemorySession extends _Object {
	TransportMemorySession clone();
}
final class MuxCatalogHangConsumer extends _Object {
	factory MuxCatalogHangConsumer(TrackSubscriber track);
	Cancelable<HangCatalog?> next();
}
final class MuxCatalogHangConsumerExtra extends _Object {
	factory MuxCatalogHangConsumerExtra(TrackSubscriber track);
	Cancelable<HangCatalogExtra?> next();
}
final class MuxCatalogHangConsumerExt extends _Object {
	factory MuxCatalogHangConsumerExt(TrackSubscriber track);
	Cancelable<HangCatalogExt?> next();
}
final class MuxCatalogHangContainer extends _Object {
	factory MuxCatalogHangContainer(HangCatalogContainer container, MuxContainerKind kind);
}
final class MuxCatalogHangExtra extends _Object {
	String debug();
	MuxCatalogHangExtra clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxCatalogHangExtra.default_();
	Json? get(String name);
	List<(String, Json)> iter();
	U64 len();
	bool isEmpty();
	void set(String name, Json value);
	Json? remove(String name);
}
final class MuxCatalogMsfConsumer extends _Object {
	factory MuxCatalogMsfConsumer(TrackSubscriber track);
	Cancelable<HangCatalog?> next();
}
final class MuxCatalogMsfConsumerExtra extends _Object {
	factory MuxCatalogMsfConsumerExtra(TrackSubscriber track);
	Cancelable<HangCatalogExtra?> next();
}
final class MuxCatalogMsfConsumerExt extends _Object {
	factory MuxCatalogMsfConsumerExt(TrackSubscriber track);
	Cancelable<HangCatalogExt?> next();
}
final class MuxCodecAacError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecAacError clone();
}
final class MuxCodecAacImport extends _Object {
	factory MuxCodecAacImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecAacImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecAacImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	String name();
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 sequence);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecAnnexbError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecAnnexbError clone();
}
final class MuxCodecAv1Import extends _Object {
	factory MuxCodecAv1Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecAv1Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecAv1Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 sequence);
	void decode(List<MuxContainerFrame> frames);
}
final class MuxCodecAv1Split extends _Object {
	factory MuxCodecAv1Split.default_();
	factory MuxCodecAv1Split();
	List<MuxContainerFrame> decode(typed.Uint8List data, Timestamp? pts);
	List<MuxContainerFrame> flush(Timestamp? pts);
	void reset();
}
final class MuxCodecFlacImport extends _Object {
	factory MuxCodecFlacImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecFlacImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecFlacImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 sequence);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecH264Avc1 extends _Object {
	factory MuxCodecH264Avc1.default_();
	factory MuxCodecH264Avc1();
	typed.Uint8List? avcc();
	typed.Uint8List? transform(typed.Uint8List payload);
}
final class MuxCodecH264Error extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecH264Error clone();
}
final class MuxCodecH264Avcc extends _Object {
	String debug();
	MuxCodecH264Avcc clone();
	int get profileGet;
	void profileSet(int value);
	int get constraintsGet;
	void constraintsSet(int value);
	int get levelGet;
	void levelSet(int value);
	U64 get lengthSizeGet;
	void lengthSizeSet(U64 value);
	List<typed.Uint8List> get spsGet;
	void spsSet(List<typed.Uint8List> value);
	List<typed.Uint8List> get ppsGet;
	void ppsSet(List<typed.Uint8List> value);
	int? get codedWidthGet;
	void codedWidthSet(int? value);
	int? get codedHeightGet;
	void codedHeightSet(int? value);
	factory MuxCodecH264Avcc.parse(typed.Uint8List avcc);
}
final class MuxCodecH264Import extends _Object {
	factory MuxCodecH264Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH264Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH264Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	String name();
	TrackDemand demand();
	TrackProducer track();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	void seek(U64 sequence);
	void observeReorder(Timestamp reorder);
	void decode(List<MuxContainerFrame> frames);
}
final class MuxCodecH264Split extends _Object {
	factory MuxCodecH264Split.default_();
	factory MuxCodecH264Split();
	List<MuxContainerFrame> decode(typed.Uint8List data, Timestamp? pts);
	List<MuxContainerFrame> flush(Timestamp? pts);
	void reset();
}
final class MuxCodecH265Hvc1 extends _Object {
	factory MuxCodecH265Hvc1.default_();
	factory MuxCodecH265Hvc1();
	typed.Uint8List? hvcc();
	typed.Uint8List? transform(typed.Uint8List payload);
}
final class MuxCodecH265Error extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecH265Error clone();
}
final class MuxCodecH265Hvcc extends _Object {
	String debug();
	MuxCodecH265Hvcc clone();
	U64 get lengthSizeGet;
	void lengthSizeSet(U64 value);
	List<typed.Uint8List> get vpsGet;
	void vpsSet(List<typed.Uint8List> value);
	List<typed.Uint8List> get spsGet;
	void spsSet(List<typed.Uint8List> value);
	List<typed.Uint8List> get ppsGet;
	void ppsSet(List<typed.Uint8List> value);
	factory MuxCodecH265Hvcc.parse(typed.Uint8List hvcc);
}
final class MuxCodecH265Import extends _Object {
	factory MuxCodecH265Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH265Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH265Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	String name();
	TrackDemand demand();
	TrackProducer track();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	void seek(U64 sequence);
	void observeReorder(Timestamp reorder);
	void decode(List<MuxContainerFrame> frames);
}
final class MuxCodecH265Split extends _Object {
	factory MuxCodecH265Split.default_();
	factory MuxCodecH265Split();
	List<MuxContainerFrame> decode(typed.Uint8List data, Timestamp? pts);
	List<MuxContainerFrame> flush(Timestamp? pts);
	void reset();
}
final class MuxCodecMp3Import extends _Object {
	factory MuxCodecMp3Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecMp3Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecMp3Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 sequence);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecOpusConfig extends _Object {
	String debug();
	MuxCodecOpusConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	int get sampleRateGet;
	void sampleRateSet(int value);
	int get channelCountGet;
	void channelCountSet(int value);
	int get preSkipGet;
	void preSkipSet(int value);
	int get outputGainGet;
	void outputGainSet(int value);
	MuxCodecOpusMapping? get mappingGet;
	void mappingSet(MuxCodecOpusMapping? value);
	factory MuxCodecOpusConfig(int sampleRate, int channelCount);
	MuxCodecOpusConfig withPreSkip(int preSkip);
	factory MuxCodecOpusConfig.parse(Mut<typed.Uint8List> buf);
	typed.Uint8List encode();
}
final class MuxCodecOpusImport extends _Object {
	factory MuxCodecOpusImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecOpusImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecOpusImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	String name();
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 sequence);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecVp8Import extends _Object {
	factory MuxCodecVp8Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp8Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp8Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	void decode(typed.Uint8List frame, Timestamp? pts);
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 sequence);
}
final class MuxCodecVp9Import extends _Object {
	factory MuxCodecVp9Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp9Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp9Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	void decode(typed.Uint8List frame, Timestamp? pts);
	TrackDemand demand();
	void finish();
	void abort(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 sequence);
}
final class MuxContainerFlvImport extends _Object {
	factory MuxContainerFlvImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerFlvImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerFlvImportExtra extends _Object {
	factory MuxContainerFlvImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerFlvImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerFlvImportExt extends _Object {
	factory MuxContainerFlvImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerFlvImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerFmp4Wire extends _Object {
	factory MuxContainerFmp4Wire(Mp4AtomTrak trak);
	factory MuxContainerFmp4Wire.fromInit(typed.Uint8List initData);
	Mp4AtomTrak trak();
}
final class MuxContainerFmp4Error extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxContainerFmp4Error clone();
}
final class MuxContainerFmp4Fragmenter extends _Object {
	List<MuxContainerFmp4Fragment> push(MuxContainerFrame frame);
	List<MuxContainerFmp4Fragment> pushGroup(MuxContainerFrame frame);
	MuxContainerFmp4Fragment? flush();
}
final class MuxContainerFmp4Import extends _Object {
	factory MuxContainerFmp4Import(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	void cut();
	MuxContainerFmp4Import withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4Import withPassthrough(bool passthrough);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void seek(U64 sequence);
	void discontinuity();
}
final class MuxContainerFmp4ImportExtra extends _Object {
	factory MuxContainerFmp4ImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	void cut();
	MuxContainerFmp4ImportExtra withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4ImportExtra withPassthrough(bool passthrough);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void seek(U64 sequence);
	void discontinuity();
}
final class MuxContainerFmp4ImportExt extends _Object {
	factory MuxContainerFmp4ImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	void cut();
	MuxContainerFmp4ImportExt withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4ImportExt withPassthrough(bool passthrough);
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
	void seek(U64 sequence);
	void discontinuity();
}
final class MuxContainerFmp4Muxer extends _Object {
	factory MuxContainerFmp4Muxer.video(HangCatalogVideoConfig config);
	factory MuxContainerFmp4Muxer.audio(HangCatalogAudioConfig config);
	Timescale timescale();
	MuxContainerFmp4Muxer withTimescale(Timescale timescale);
	MuxContainerFmp4Muxer withFramerate(double? framerate);
	Cancelable<List<MuxContainerFrame>> read(GroupConsumer group);
	typed.Uint8List? init();
	typed.Uint8List fragment(int sequence, List<MuxContainerFrame> frames, Timestamp? end);
	MuxContainerFmp4Fragmenter fragmenter(MuxContainerFmp4FragmentConfig config);
}
final class MuxContainerMkvImport extends _Object {
	factory MuxContainerMkvImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerMkvImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerMkvImportExtra extends _Object {
	factory MuxContainerMkvImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerMkvImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerMkvImportExt extends _Object {
	factory MuxContainerMkvImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerMkvImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	void abort(Error err);
}
final class MuxContainerTsExt extends _Object {
	String debug();
	MuxContainerTsExt clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsExt.default_();
	MuxContainerTsMpegts get mpegtsGet;
	void mpegtsSet(MuxContainerTsMpegts value);
}
final class MuxContainerTsMpegts extends _Object {
	String debug();
	MuxContainerTsMpegts clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsMpegts.default_();
	List<(String, MuxContainerTsTrack)> get tracksGet;
	void tracksSet(List<(String, MuxContainerTsTrack)> value);
	List<MuxContainerTsDescriptor> get programDescriptorsGet;
	void programDescriptorsSet(List<MuxContainerTsDescriptor> value);
	MuxContainerTsProgram? get programGet;
	void programSet(MuxContainerTsProgram? value);
	List<(int, List<(int, MuxContainerTsSiEntry)>)> get siGet;
	void siSet(List<(int, List<(int, MuxContainerTsSiEntry)>)> value);
	U64? get muxRateGet;
	void muxRateSet(U64? value);
	bool isEmpty();
}
final class MuxContainerTsPesPacket extends _Object {
	String debug();
	MuxContainerTsPesPacket clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsPesPacket.default_();
	int get streamTypeGet;
	void streamTypeSet(int value);
	factory MuxContainerTsPesPacket(int streamType);
}
final class MuxContainerTsProgram extends _Object {
	String debug();
	MuxContainerTsProgram clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsProgram.default_();
	int get transportStreamIdGet;
	void transportStreamIdSet(int value);
	int get programNumberGet;
	void programNumberSet(int value);
	int get pmtPidGet;
	void pmtPidSet(int value);
}
final class MuxContainerTsSiEntry extends _Object {
	String debug();
	MuxContainerTsSiEntry clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsSiEntry.default_();
	String get trackGet;
	void trackSet(String value);
	RustDuration? get intervalGet;
	void intervalSet(RustDuration? value);
}
final class MuxContainerTsTrack extends _Object {
	String debug();
	MuxContainerTsTrack clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsTrack.default_();
	int get pidGet;
	void pidSet(int value);
	List<MuxContainerTsDescriptor> get descriptorsGet;
	void descriptorsSet(List<MuxContainerTsDescriptor> value);
	MuxContainerTsVerbatim? get verbatimGet;
	void verbatimSet(MuxContainerTsVerbatim? value);
	MuxContainerTsPesPacket? get pesPacketGet;
	void pesPacketSet(MuxContainerTsPesPacket? value);
	factory MuxContainerTsTrack(int pid);
}
final class MuxContainerTsVerbatim extends _Object {
	String debug();
	MuxContainerTsVerbatim clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsVerbatim.default_();
	int get streamTypeGet;
	void streamTypeSet(int value);
	MuxContainerTsFraming get framingGet;
	void framingSet(MuxContainerTsFraming value);
	factory MuxContainerTsVerbatim(int streamType, MuxContainerTsFraming framing);
}
final class MuxContainerTsExportConsumerExtra extends _Object {
	factory MuxContainerTsExportConsumerExtra(MuxSource source, MuxCatalogConsumerExtra catalog);
	MuxContainerTsExportConsumerExtra withMuxRate(U64 muxRate);
	MuxContainerTsExportConsumerExtra withDelay(RustDuration delay);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumerExtra follow(BroadcastConsumer broadcast, MuxCatalogConsumerExtra catalog);
}
final class MuxContainerTsExportConsumerExt extends _Object {
	factory MuxContainerTsExportConsumerExt(MuxSource source, MuxCatalogConsumerExt catalog);
	MuxContainerTsExportConsumerExt withMuxRate(U64 muxRate);
	MuxContainerTsExportConsumerExt withDelay(RustDuration delay);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumerExt follow(BroadcastConsumer broadcast, MuxCatalogConsumerExt catalog);
}
final class MuxContainerTsExportConsumer extends _Object {
	factory MuxContainerTsExportConsumer(MuxSource source, MuxCatalogConsumer catalog);
	MuxContainerTsExportConsumer withMuxRate(U64 muxRate);
	MuxContainerTsExportConsumer withDelay(RustDuration delay);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumer follow(BroadcastConsumer broadcast, MuxCatalogConsumer catalog);
}
final class MuxContainerTsFollower extends _Object {
	factory MuxContainerTsFollower(MuxContainerTsExportConsumer export);
	MuxContainerTsFollower withFollow(RustDuration follow);
	MuxContainerTsFollower withStitch(bool stitch);
	MuxContainerTsExportConsumer export();
	Cancelable<MuxContainerFrame?> next();
}
final class MuxContainerTsFollowerExtra extends _Object {
	factory MuxContainerTsFollowerExtra(MuxContainerTsExportConsumerExtra export);
	MuxContainerTsFollowerExtra withFollow(RustDuration follow);
	MuxContainerTsFollowerExtra withStitch(bool stitch);
	MuxContainerTsExportConsumerExtra export();
	Cancelable<MuxContainerFrame?> next();
}
final class MuxContainerTsFollowerExt extends _Object {
	factory MuxContainerTsFollowerExt(MuxContainerTsExportConsumerExt export);
	MuxContainerTsFollowerExt withFollow(RustDuration follow);
	MuxContainerTsFollowerExt withStitch(bool stitch);
	MuxContainerTsExportConsumerExt export();
	Cancelable<MuxContainerFrame?> next();
}
final class MuxContainerTsPassthrough extends _Object {
	factory MuxContainerTsPassthrough(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerTsPassthrough withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
}
final class MuxContainerTsPassthroughExtra extends _Object {
	factory MuxContainerTsPassthroughExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerTsPassthroughExtra withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
}
final class MuxContainerTsPassthroughExt extends _Object {
	factory MuxContainerTsPassthroughExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerTsPassthroughExt withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	void finish();
	void abort(Error err);
}
final class MuxContainerTsPrograms extends _Object {
	factory MuxContainerTsPrograms(OriginProducer origin, Path name, MuxCatalogConfig config);
	void decode(typed.Uint8List data);
	MuxContainerTsStatsSnapshot stats();
	List<BroadcastProducer> finish();
	void abort(Error err);
}
final class MuxContainerTsRelease extends _Object {
	static Cancelable<MuxContainerTsRelease> new_(MuxSource source, MuxCatalogCatalogFormat catalogFormat);
	MuxContainerTsRelease withDelay(RustDuration delay);
	U64 dropped();
	double? drift();
	U64 outOfTolerance();
	MuxContainerTsStatsRelease stats();
	Cancelable<MuxContainerFrame?> next();
}
final class MuxContainerTsImport extends _Object {
	factory MuxContainerTsImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerTsImport withProgram(int program);
	MuxContainerTsImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsImportExtra extends _Object {
	factory MuxContainerTsImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerTsImportExtra withProgram(int program);
	MuxContainerTsImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsImportExt extends _Object {
	factory MuxContainerTsImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerTsImportExt withProgram(int program);
	MuxContainerTsImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	void finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxCodecAacAdtsHeader extends _Object {
	String debug();
	MuxCodecAacAdtsHeader clone();
	int get objectTypeGet;
	void objectTypeSet(int value);
	int get sampleRateGet;
	void sampleRateSet(int value);
	int get channelConfigGet;
	void channelConfigSet(int value);
	int get blocksGet;
	void blocksSet(int value);
	factory MuxCodecAacAdtsHeader.parse(typed.Uint8List data);
	typed.Uint8List payload(typed.Uint8List packet);
}
final class MuxContainerTsStatsReport extends _Object {
	String debug();
	MuxContainerTsStatsReport clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsStatsReport.default_();
	MuxContainerTsStatsSnapshot? get mpegtsGet;
	void mpegtsSet(MuxContainerTsStatsSnapshot? value);
	MuxContainerTsStatsRelease? get releaseGet;
	void releaseSet(MuxContainerTsStatsRelease? value);
}
final class MuxContainerTsStatsSnapshot extends _Object {
	String debug();
	MuxContainerTsStatsSnapshot clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsStatsSnapshot.default_();
	List<(int, MuxContainerTsStatsStream)> get streamsGet;
	void streamsSet(List<(int, MuxContainerTsStatsStream)> value);
	U64 get crcErrorGet;
	void crcErrorSet(U64 value);
	U64 get tsSyncLossGet;
	void tsSyncLossSet(U64 value);
	U64 get syncByteErrorGet;
	void syncByteErrorSet(U64 value);
	U64 get patErrorGet;
	void patErrorSet(U64 value);
	U64 get continuityCountErrorGet;
	void continuityCountErrorSet(U64 value);
	U64 get pmtErrorGet;
	void pmtErrorSet(U64 value);
	U64 get transportErrorGet;
	void transportErrorSet(U64 value);
	U64 get pcrRepetitionErrorGet;
	void pcrRepetitionErrorSet(U64 value);
	U64 get pcrDiscontinuityIndicatorErrorGet;
	void pcrDiscontinuityIndicatorErrorSet(U64 value);
	U64 get ptsErrorGet;
	void ptsErrorSet(U64 value);
	bool isEmpty();
}
final class MuxContainerTsStatsStream extends _Object {
	String debug();
	MuxContainerTsStatsStream clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsStatsStream.default_();
	String get trackGet;
	void trackSet(String value);
	MuxContainerTsStatsClass get classGet;
	void classSet(MuxContainerTsStatsClass value);
	U64 get unitsGet;
	void unitsSet(U64 value);
	RustDuration? get quietGet;
	void quietSet(RustDuration? value);
	U64 get resyncsGet;
	void resyncsSet(U64 value);
	U64 get discardedGet;
	void discardedSet(U64 value);
	U64 get unconfirmedGet;
	void unconfirmedSet(U64 value);
	U64 get damagedGet;
	void damagedSet(U64 value);
	U64 get unplacedGet;
	void unplacedSet(U64 value);
	U64 get refusedGet;
	void refusedSet(U64 value);
	U64 get continuityCountErrorGet;
	void continuityCountErrorSet(U64 value);
	U64 get transportErrorGet;
	void transportErrorSet(U64 value);
	U64 get ptsErrorGet;
	void ptsErrorSet(U64 value);
}
final class MuxContainerTsStatsExport extends _Object {
	String debug();
	MuxContainerTsStatsExport clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsStatsExport.default_();
	List<(int, MuxContainerTsStatsStream)> get streamsGet;
	void streamsSet(List<(int, MuxContainerTsStatsStream)> value);
	MuxContainerTsStatsRelease get releaseGet;
	void releaseSet(MuxContainerTsStatsRelease value);
}
final class MuxContainerTsStatsRelease extends _Object {
	String debug();
	MuxContainerTsStatsRelease clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerTsStatsRelease.default_();
	U64 get droppedGet;
	void droppedSet(U64 value);
	U64 get trimmedGet;
	void trimmedSet(U64 value);
	U64 get gapsGet;
	void gapsSet(U64 value);
	int? get marginGet;
	void marginSet(int? value);
	int? get offsetGet;
	void offsetSet(int? value);
	double? get driftGet;
	void driftSet(double? value);
	U64 get outOfToleranceGet;
	void outOfToleranceSet(U64 value);
}
final class MuxContainerTsStatsLog extends _Object {
	factory MuxContainerTsStatsLog.default_();
	static RustDuration get INTERVAL;
	void sample(MuxContainerTsStatsSnapshot latest);
	void finish(MuxContainerTsStatsSnapshot latest);
}
final class MoqTokioAddrs extends _Object {
	String debug();
	MoqTokioAddrs clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqTokioAddrs(MoqTokioConnectAddr addr);
	MoqTokioAddrs or(MoqTokioConnectAddr addr);
	static MoqTokioAddrs? collect(List<MoqTokioConnectAddr> urls);
	List<MoqTokioConnectAddr> asSlice();
}
final class MoqTokioCliDeprecated extends _Object {
	@override String toString();
	String debug();
	MoqTokioCliDeprecated clone();
	factory MoqTokioCliDeprecated.default_();
	bool isEmpty();
	void extend(MoqTokioCliDeprecated other);
	void flag(String old, String? env, String new_);
	void changed(String old, String? env, String new_, String note);
	void removed(String old, String? env, String note);
	void toml(String old, String new_, String? note);
}
final class Mp4AtomCodec extends _Object {
	String debug();
	Mp4AtomCodec clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomVisual? visual();
	Mp4AtomAudio? audio();
	Mp4AtomMetaData? metadata();
}
final class MoqTokioTcpTlsAcceptor extends _Object {
	MoqTokioTcpTlsAcceptor clone();
	RustlsServerConfig config();
	bool offered(List<typed.Uint8List> alpns);
}
final class MoqTokioAcceptHealth extends _Object {
	String debug();
	MoqTokioAcceptHealth clone();
	factory MoqTokioAcceptHealth(String listener);
	String listener();
	void accepted();
	U64 failures(MoqTokioAcceptFailure failure);
	RustDuration? stalled();
}
final class RustlsClientConfig extends _Object {
	String debug();
	RustlsClientConfig clone();
	List<typed.Uint8List> get alpnProtocolsGet;
	void alpnProtocolsSet(List<typed.Uint8List> value);
	bool get checkSelectedAlpnGet;
	void checkSelectedAlpnSet(bool value);
	RustlsClientResumption get resumptionGet;
	void resumptionSet(RustlsClientResumption value);
	U64? get maxFragmentSizeGet;
	void maxFragmentSizeSet(U64? value);
	RustlsResolvesClientCert get clientAuthCertResolverGet;
	void clientAuthCertResolverSet(RustlsResolvesClientCert value);
	bool get enableSniGet;
	void enableSniSet(bool value);
	RustlsKeyLog get keyLogGet;
	void keyLogSet(RustlsKeyLog value);
	bool get enableSecretExtractionGet;
	void enableSecretExtractionSet(bool value);
	bool get enableEarlyDataGet;
	void enableEarlyDataSet(bool value);
	bool get requireEmsGet;
	void requireEmsSet(bool value);
	RustlsTimeProvider get timeProviderGet;
	void timeProviderSet(RustlsTimeProvider value);
	List<RustlsCertDecompressor> get certDecompressorsGet;
	void certDecompressorsSet(List<RustlsCertDecompressor> value);
	List<RustlsCertCompressor> get certCompressorsGet;
	void certCompressorsSet(List<RustlsCertCompressor> value);
	RustlsCompressionCache get certCompressionCacheGet;
	void certCompressionCacheSet(RustlsCompressionCache value);
	RustlsClientTicketRequest? get sendTicketRequestGet;
	void sendTicketRequestSet(RustlsClientTicketRequest? value);
	bool fips();
	RustlsCryptoProvider cryptoProvider();
	RustlsClientDangerDangerousClientConfig dangerous();
}
final class RustlsRootCertStore extends _Object {
	String debug();
	RustlsRootCertStore clone();
	List<RustlsPkiTypesTrustAnchor> get rootsGet;
	void rootsSet(List<RustlsPkiTypesTrustAnchor> value);
	factory RustlsRootCertStore.empty();
	(U64, U64) addParsableCertificates(List<RustlsPkiTypesCertificateDer> derCerts);
	void add(RustlsPkiTypesCertificateDer der);
	List<RustlsDistinguishedName> subjects();
	bool isEmpty();
	U64 len();
}
final class RustlsPkiTypesCertificateDer extends _Object {
	String debug();
	RustlsPkiTypesCertificateDer clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory RustlsPkiTypesCertificateDer.fromSlice(typed.Uint8List bytes);
	RustlsPkiTypesCertificateDer intoOwned();
}
final class MoqTokioConnectResolved extends _Object {
	String debug();
	MoqTokioConnectResolved clone();
	SocketAddr get bindGet;
	void bindSet(SocketAddr value);
	RustDuration get raceGet;
	void raceSet(RustDuration value);
	RustDuration get resolutionDelayGet;
	void resolutionDelaySet(RustDuration value);
	RustDuration get timeoutGet;
	void timeoutSet(RustDuration value);
}
final class MoqTokioConnectionGoaway extends _Object {
	String debug();
	MoqTokioConnectionGoaway clone();
	factory MoqTokioConnectionGoaway.default_();
	MoqTokioRedirect get redirectGet;
	void redirectSet(MoqTokioRedirect value);
	TokioDuration? get handoverGet;
	void handoverSet(TokioDuration? value);
}
final class MoqTokioWebsocketConfig extends _Object {
	String debug();
	MoqTokioWebsocketConfig clone();
	factory MoqTokioWebsocketConfig.default_();
	bool? get enabledGet;
	void enabledSet(bool? value);
	TokioDuration? get delayGet;
	void delaySet(TokioDuration? value);
	MoqTokioWebsocketResolved resolve();
}
final class MoqTokioConnectionObservation extends _Object {
	String debug();
	MoqTokioConnectionObservation clone();
	U64 get revisionGet;
	void revisionSet(U64 value);
	MoqTokioConnectionPhase get phaseGet;
	void phaseSet(MoqTokioConnectionPhase value);
	U64 get connectsGet;
	void connectsSet(U64 value);
	ClockInstant? get lastConnectedGet;
	void lastConnectedSet(ClockInstant? value);
	TokioTransport? get transportGet;
	void transportSet(TokioTransport? value);
	TokioError? get failureGet;
	void failureSet(TokioError? value);
	Version? get versionGet;
	void versionSet(Version? value);
}
final class MoqTokioConnectionSnapshot extends _Object {
	SessionStats get statsGet;
	void statsSet(SessionStats value);
	Version get versionGet;
	void versionSet(Version value);
}
final class MoqTokioListenResolved extends _Object {
	String debug();
	MoqTokioListenResolved clone();
	TokioListenBind? get bindGet;
	void bindSet(TokioListenBind? value);
	Versions get versionsGet;
	void versionsSet(Versions value);
	SetupExtensions get extensionsGet;
	void extensionsSet(SetupExtensions value);
	String? get implementationGet;
	void implementationSet(String? value);
	RustDuration? get timeoutGet;
	void timeoutSet(RustDuration? value);
	TokioTlsListen get tlsGet;
	void tlsSet(TokioTlsListen value);
	SocketAddrV4? get preferredV4Get;
	void preferredV4Set(SocketAddrV4? value);
	SocketAddrV6? get preferredV6Get;
	void preferredV6Set(SocketAddrV6? value);
	MoqTokioListenLoadBalancer get lbGet;
	void lbSet(MoqTokioListenLoadBalancer value);
	PathBuf? get resetSecretGet;
	void resetSecretSet(PathBuf? value);
}
final class MoqTokioTcpConfig extends _Object {
	String debug();
	MoqTokioTcpConfig clone();
	factory MoqTokioTcpConfig.default_();
	SocketAddr? get bindGet;
	void bindSet(SocketAddr? value);
	bool? get tlsGet;
	void tlsSet(bool? value);
}
final class MoqTokioUnixConfig extends _Object {
	String debug();
	MoqTokioUnixConfig clone();
	factory MoqTokioUnixConfig.default_();
	PathBuf? get bindGet;
	void bindSet(PathBuf? value);
	MoqTokioUnixAllow get allowGet;
	void allowSet(MoqTokioUnixAllow value);
}
final class MoqTokioListenLoadBalancer extends _Object {
	String debug();
	MoqTokioListenLoadBalancer clone();
	factory MoqTokioListenLoadBalancer.default_();
	MoqTokioQuicServerId? get idGet;
	void idSet(MoqTokioQuicServerId? value);
	U64? get nonceGet;
	void nonceSet(U64? value);
	MoqTokioListenForward get forwardGet;
	void forwardSet(MoqTokioListenForward value);
}
final class MoqTokioListenReset extends _Object {
	String debug();
	MoqTokioListenReset clone();
	factory MoqTokioListenReset.default_();
	PathBuf? get secretGet;
	void secretSet(PathBuf? value);
}
final class MoqTokioQuicResolved extends _Object {
	String debug();
	MoqTokioQuicResolved clone();
	factory MoqTokioQuicResolved.default_();
	U64 get maxStreamsGet;
	void maxStreamsSet(U64 value);
	bool? get gsoGet;
	void gsoSet(bool? value);
	RustDuration get idleTimeoutGet;
	void idleTimeoutSet(RustDuration value);
	RustDuration? get keepAliveGet;
	void keepAliveSet(RustDuration? value);
	bool get mtuDiscoveryGet;
	void mtuDiscoverySet(bool value);
	MoqTokioQuicCongestionControl? get congestionControlGet;
	void congestionControlSet(MoqTokioQuicCongestionControl? value);
	MoqTokioQuicController? get controllerGet;
	void controllerSet(MoqTokioQuicController? value);
	U64 get receiveWindowGet;
	void receiveWindowSet(U64 value);
	U64 get streamReceiveWindowGet;
	void streamReceiveWindowSet(U64 value);
	U64 get sendWindowGet;
	void sendWindowSet(U64 value);
	PathBuf? get qlogGet;
	void qlogSet(PathBuf? value);
	PathBuf? get keylogGet;
	void keylogSet(PathBuf? value);
	bool hasController();
	RustDuration keepAliveInterval();
}
final class MoqTokioQuicController extends _Object {
	String debug();
	MoqTokioQuicController clone();
	MoqCongestionFactory get $0_get;
	void $0_set(MoqCongestionFactory value);
}
final class MoqTokioWebsocketListener extends _Object {
	static Cancelable<MoqTokioWebsocketListener> bind(SocketAddr addr);
	MoqTokioWebsocketListener withProtocols(List<String> protocols);
	MoqTokioWebsocketListener withQuic(MoqTokioQuicResolved quic);
	SocketAddr localAddr();
	MoqTokioAcceptHealth acceptHealth();
}
final class MoqTokioQuicDemuxDemux extends _Object {
	String debug();
	MoqTokioQuicDemuxDemux clone();
	factory MoqTokioQuicDemuxDemux.default_();
	MoqSockDemuxCounters counters();
	MoqTokioQuicDemuxWebRtc webrtc();
}
final class MoqTokioQuicDriverConnectionConnection extends _Object {
	String debug();
	MoqTokioQuicDriverConnectionConnection clone();
	Cancelable<MoqTokioQuicDriverSendStream> openUni(MoqTokioQuicDriverSendOptions options);
	MoqTokioQuicDriverSendGroup sendGroup(int priority);
	Cancelable<(MoqTokioQuicDriverSendStream, MoqTokioQuicDriverRecvStream)> openBi(MoqTokioQuicDriverSendOptions options);
	Cancelable<MoqTokioQuicDriverRecvStream> acceptUni();
	Cancelable<(MoqTokioQuicDriverSendStream, MoqTokioQuicDriverRecvStream)> acceptBi();
	Cancelable<typed.Uint8List> readDatagram();
	Cancelable<typed.Uint8List> pollRecvDatagram();
	Cancelable<MoqQuicError> closed();
	MoqQuicError? closeReason();
	void close_(MoqQuicVarInt errorCode, String reason);
	void sendDatagram(typed.Uint8List data, MoqTokioQuicDriverSendOptions options);
	U64? maxDatagramSize();
	U64 datagramSendBufferSpace();
	SocketAddr remoteAddress();
	RustDuration rtt();
	MoqQuicConnectionStats stats();
	MoqQuicCryptoHandshakeData? handshakeData();
	List<RustlsPkiTypesCertificateDer>? peerIdentity();
	U64 stableId();
	void setLimits(MoqQuicLimits limits);
}
final class MoqQuicLimits extends _Object {
	String debug();
	MoqQuicLimits clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqQuicLimits.default_();
	U64? get maxStreamsGet;
	void maxStreamsSet(U64? value);
	U64? get receiveWindowGet;
	void receiveWindowSet(U64? value);
	U64? get streamReceiveWindowGet;
	void streamReceiveWindowSet(U64? value);
	U64? get sendWindowGet;
	void sendWindowSet(U64? value);
	MoqQuicRate? get receiveRateGet;
	void receiveRateSet(MoqQuicRate? value);
	MoqQuicRate? get sendRateGet;
	void sendRateSet(MoqQuicRate? value);
	void validate();
}
final class Mp4AtomTrak extends _Object {
	String debug();
	Mp4AtomTrak clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTrak.default_();
	Mp4AtomTkhd get tkhdGet;
	void tkhdSet(Mp4AtomTkhd value);
	Mp4AtomEdts? get edtsGet;
	void edtsSet(Mp4AtomEdts? value);
	Mp4AtomMeta? get metaGet;
	void metaSet(Mp4AtomMeta? value);
	Mp4AtomMdia get mdiaGet;
	void mdiaSet(Mp4AtomMdia value);
	Mp4AtomSenc? get sencGet;
	void sencSet(Mp4AtomSenc? value);
	Mp4AtomTref? get trefGet;
	void trefSet(Mp4AtomTref? value);
	Mp4AtomUdta? get udtaGet;
	void udtaSet(Mp4AtomUdta? value);
}
final class MoqTokioConnectAddr extends _Object {
	String debug();
	MoqTokioConnectAddr clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MoqTokioConnectAddr(Url url);
	factory MoqTokioConnectAddr.pinned(Url url, List<SocketAddr> addrs);
	Url url();
	List<SocketAddr>? addresses();
}
final class Mp4AtomAvc1 extends _Object {
	String debug();
	Mp4AtomAvc1 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAvc1.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomAvcc get avccGet;
	void avccSet(Mp4AtomAvcc value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomTaic? get taicGet;
	void taicSet(Mp4AtomTaic? value);
	Mp4AtomFiel? get fielGet;
	void fielSet(Mp4AtomFiel? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomAvc3 extends _Object {
	String debug();
	Mp4AtomAvc3 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAvc3.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomAvcc get avccGet;
	void avccSet(Mp4AtomAvcc value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomTaic? get taicGet;
	void taicSet(Mp4AtomTaic? value);
	Mp4AtomFiel? get fielGet;
	void fielSet(Mp4AtomFiel? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomJpeg extends _Object {
	String debug();
	Mp4AtomJpeg clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomJpeg.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomFiel? get fielGet;
	void fielSet(Mp4AtomFiel? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomHev1 extends _Object {
	String debug();
	Mp4AtomHev1 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHev1.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomHvcc get hvccGet;
	void hvccSet(Mp4AtomHvcc value);
	Mp4AtomLhvc? get lhvcGet;
	void lhvcSet(Mp4AtomLhvc? value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomTaic? get taicGet;
	void taicSet(Mp4AtomTaic? value);
	Mp4AtomFiel? get fielGet;
	void fielSet(Mp4AtomFiel? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomHvc1 extends _Object {
	String debug();
	Mp4AtomHvc1 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomHvc1.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomHvcc get hvccGet;
	void hvccSet(Mp4AtomHvcc value);
	Mp4AtomLhvc? get lhvcGet;
	void lhvcSet(Mp4AtomLhvc? value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomTaic? get taicGet;
	void taicSet(Mp4AtomTaic? value);
	Mp4AtomFiel? get fielGet;
	void fielSet(Mp4AtomFiel? value);
	Mp4AtomCcst? get ccstGet;
	void ccstSet(Mp4AtomCcst? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomVp08 extends _Object {
	String debug();
	Mp4AtomVp08 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVp08.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomVpcC get vpccGet;
	void vpccSet(Mp4AtomVpcC value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomVp09 extends _Object {
	String debug();
	Mp4AtomVp09 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVp09.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomVpcC get vpccGet;
	void vpccSet(Mp4AtomVpcC value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomAv01 extends _Object {
	String debug();
	Mp4AtomAv01 clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomAv01.default_();
	Mp4AtomVisual get visualGet;
	void visualSet(Mp4AtomVisual value);
	Mp4AtomAv1c get av1cGet;
	void av1cSet(Mp4AtomAv1c value);
	Mp4AtomBtrt? get btrtGet;
	void btrtSet(Mp4AtomBtrt? value);
	Mp4AtomCcst? get ccstGet;
	void ccstSet(Mp4AtomCcst? value);
	Mp4AtomColr? get colrGet;
	void colrSet(Mp4AtomColr? value);
	Mp4AtomPasp? get paspGet;
	void paspSet(Mp4AtomPasp? value);
	Mp4AtomTaic? get taicGet;
	void taicSet(Mp4AtomTaic? value);
	Mp4AtomSpatial? get spatialGet;
	void spatialSet(Mp4AtomSpatial? value);
}
final class Mp4AtomTx3g extends _Object {
	String debug();
	Mp4AtomTx3g clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTx3g.default_();
	int get dataReferenceIndexGet;
	void dataReferenceIndexSet(int value);
	int get displayFlagsGet;
	void displayFlagsSet(int value);
	int get horizontalJustificationGet;
	void horizontalJustificationSet(int value);
	int get verticalJustificationGet;
	void verticalJustificationSet(int value);
	Mp4AtomRgbaColor get bgColorRgbaGet;
	void bgColorRgbaSet(Mp4AtomRgbaColor value);
	ArrayU8x12 get styleRecordGet;
	void styleRecordSet(ArrayU8x12 value);
	Mp4AtomFtab? get ftabGet;
	void ftabSet(Mp4AtomFtab? value);
	Mp4AtomDisp? get dispGet;
	void dispSet(Mp4AtomDisp? value);
}
final class RustlsPkiTypesTrustAnchor extends _Object {
	String debug();
	RustlsPkiTypesTrustAnchor clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	RustlsPkiTypesDer get subjectGet;
	void subjectSet(RustlsPkiTypesDer value);
	RustlsPkiTypesDer get subjectPublicKeyInfoGet;
	void subjectPublicKeyInfoSet(RustlsPkiTypesDer value);
	RustlsPkiTypesDer? get nameConstraintsGet;
	void nameConstraintsSet(RustlsPkiTypesDer? value);
	RustlsPkiTypesTrustAnchor toOwned();
}
final class MoqTokioQuicServerId extends _Object {
	String debug();
	MoqTokioQuicServerId clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqTokioListenForward extends _Object {
	String debug();
	MoqTokioListenForward clone();
	factory MoqTokioListenForward.default_();
	SocketAddr? get bindGet;
	void bindSet(SocketAddr? value);
	PathBuf? get keyGet;
	void keySet(PathBuf? value);
	PathBuf? get peersGet;
	void peersSet(PathBuf? value);
	bool configured();
}
final class Mp4AtomMeta extends _Object {
	String debug();
	Mp4AtomMeta clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomHdlr get hdlrGet;
	void hdlrSet(Mp4AtomHdlr value);
	List<Mp4AtomAny> get itemsGet;
	void itemsSet(List<Mp4AtomAny> value);
	Mp4AtomPitm? getPitm();
	Mp4AtomDinf? getDinf();
	Mp4AtomIloc? getIloc();
	Mp4AtomIinf? getIinf();
	Mp4AtomIprp? getIprp();
	Mp4AtomIref? getIref();
	Mp4AtomIdat? getIdat();
	Mp4AtomIlst? getIlst();
	Mp4AtomKeys? getKeys();
	Mp4AtomPitm? getMutPitm();
	Mp4AtomDinf? getMutDinf();
	Mp4AtomIloc? getMutIloc();
	Mp4AtomIinf? getMutIinf();
	Mp4AtomIprp? getMutIprp();
	Mp4AtomIref? getMutIref();
	Mp4AtomIdat? getMutIdat();
	Mp4AtomIlst? getMutIlst();
	Mp4AtomKeys? getMutKeys();
	void pushPitm(Mp4AtomPitm atom);
	void pushDinf(Mp4AtomDinf atom);
	void pushIloc(Mp4AtomIloc atom);
	void pushIinf(Mp4AtomIinf atom);
	void pushIprp(Mp4AtomIprp atom);
	void pushIref(Mp4AtomIref atom);
	void pushIdat(Mp4AtomIdat atom);
	void pushIlst(Mp4AtomIlst atom);
	void pushKeys(Mp4AtomKeys atom);
	Mp4AtomPitm? removePitm();
	Mp4AtomDinf? removeDinf();
	Mp4AtomIloc? removeIloc();
	Mp4AtomIinf? removeIinf();
	Mp4AtomIprp? removeIprp();
	Mp4AtomIref? removeIref();
	Mp4AtomIdat? removeIdat();
	Mp4AtomIlst? removeIlst();
	Mp4AtomKeys? removeKeys();
}
final class Mp4AtomMdia extends _Object {
	String debug();
	Mp4AtomMdia clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMdia.default_();
	Mp4AtomMdhd get mdhdGet;
	void mdhdSet(Mp4AtomMdhd value);
	Mp4AtomHdlr get hdlrGet;
	void hdlrSet(Mp4AtomHdlr value);
	Mp4AtomMinf get minfGet;
	void minfSet(Mp4AtomMinf value);
}
final class Mp4AtomUdta extends _Object {
	String debug();
	Mp4AtomUdta clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomUdta.default_();
	Mp4AtomChpl? get chplGet;
	void chplSet(Mp4AtomChpl? value);
	List<Mp4AtomCprt> get cprtGet;
	void cprtSet(List<Mp4AtomCprt> value);
	List<Mp4AtomKind> get kindGet;
	void kindSet(List<Mp4AtomKind> value);
	Mp4AtomMeta? get metaGet;
	void metaSet(Mp4AtomMeta? value);
	List<Mp4AtomRtng> get rtngGet;
	void rtngSet(List<Mp4AtomRtng> value);
}
final class Mp4AtomVisual extends _Object {
	String debug();
	Mp4AtomVisual clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVisual.default_();
	int get dataReferenceIndexGet;
	void dataReferenceIndexSet(int value);
	int get widthGet;
	void widthSet(int value);
	int get heightGet;
	void heightSet(int value);
	Mp4AtomFixedPointU16 get horizresolutionGet;
	void horizresolutionSet(Mp4AtomFixedPointU16 value);
	Mp4AtomFixedPointU16 get vertresolutionGet;
	void vertresolutionSet(Mp4AtomFixedPointU16 value);
	int get frameCountGet;
	void frameCountSet(int value);
	Mp4AtomCompressor get compressorGet;
	void compressorSet(Mp4AtomCompressor value);
	int get depthGet;
	void depthSet(int value);
}
final class RustlsClientDangerDangerousClientConfig extends _Object {
	String debug();
}
final class RustlsClientResumption extends _Object {
	String debug();
	RustlsClientResumption clone();
	factory RustlsClientResumption.default_();
	factory RustlsClientResumption.inMemorySessions(U64 num);
	factory RustlsClientResumption.disabled();
	RustlsClientResumption tls12Resumption(RustlsClientTls12Resumption tls12);
}
final class RustlsDistinguishedName extends _Object {
	String debug();
	RustlsDistinguishedName clone();
	factory RustlsDistinguishedName.inSequence(typed.Uint8List bytes);
}
final class MoqTokioWebsocketResolved extends _Object {
	String debug();
	MoqTokioWebsocketResolved clone();
	bool get enabledGet;
	void enabledSet(bool value);
	RustDuration get delayGet;
	void delaySet(RustDuration value);
}
final class MoqTokioUnixAllow extends _Object {
	String debug();
	MoqTokioUnixAllow clone();
	factory MoqTokioUnixAllow.default_();
	typed.Uint32List get uidGet;
	void uidSet(typed.Uint32List value);
	typed.Uint32List get gidGet;
	void gidSet(typed.Uint32List value);
	typed.Int32List get pidGet;
	void pidSet(typed.Int32List value);
}
final class MoqTokioWebsocketError extends _Object implements Exception {
	@override String toString();
	String debug();
}
final class MoqSockDemuxCounters extends _Object {
	String debug();
	MoqSockDemuxCounters clone();
	factory MoqSockDemuxCounters.default_();
	void discard(MoqSockDemuxDiscard reason);
	U64 get(MoqSockDemuxDiscard reason);
}
final class MoqTokioQuicDemuxWebRtc extends _Object {
	String debug();
	Cancelable<MoqTokioQuicDemuxDatagram?> recv();
	MoqSockDemuxPin pin(SocketAddr remote);
}
final class MoqTokioQuicDriverSendOptions extends _Object {
	String debug();
	MoqTokioQuicDriverSendOptions clone();
	factory MoqTokioQuicDriverSendOptions.default_();
	int get priorityGet;
	void prioritySet(int value);
}
final class MoqTokioQuicDriverSendStream extends _Object {
	String debug();
	Cancelable<void> writeAll(typed.Uint8List buf);
	Cancelable<MoqQuicVarInt?> finish();
	void reset(MoqQuicVarInt errorCode);
	void resetAt(MoqTokioQuicDriverSendOffset offset, MoqQuicVarInt errorCode);
	MoqTokioQuicDriverSendOffset offset();
	void setPriority(int priority);
	int priority();
	void setGroup(MoqTokioQuicDriverSendGroup? group);
	Cancelable<MoqQuicVarInt?> stopped();
	Cancelable<MoqQuicVarInt?> pollClosed();
	MoqQuicStreamId id();
}
final class MoqTokioQuicDriverSendGroup extends _Object {
	String debug();
	MoqTokioQuicDriverSendGroup clone();
	void setPriority(int priority);
	int priority();
}
final class MoqTokioQuicDriverRecvStream extends _Object {
	String debug();
	Cancelable<typed.Uint8List?> readChunk(U64 max);
	Cancelable<typed.Uint8List> readToEnd(U64 limit);
	void setPriority(int priority);
	void stop(MoqQuicVarInt errorCode);
	MoqQuicStreamId id();
	Cancelable<MoqQuicVarInt?> receivedReset();
}
final class MoqTokioQuicDriverSendError extends _Object implements Exception {
	@override String toString();
	String debug();
	MoqTokioQuicDriverSendError clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqQuicConnectionStats extends _Object {
	String debug();
	MoqQuicConnectionStats clone();
	factory MoqQuicConnectionStats.default_();
	MoqQuicUdpStats get udpTxGet;
	void udpTxSet(MoqQuicUdpStats value);
	MoqQuicUdpStats get udpRxGet;
	void udpRxSet(MoqQuicUdpStats value);
	MoqQuicFrameStats get frameTxGet;
	void frameTxSet(MoqQuicFrameStats value);
	MoqQuicFrameStats get frameRxGet;
	void frameRxSet(MoqQuicFrameStats value);
	MoqQuicPathStats get pathGet;
	void pathSet(MoqQuicPathStats value);
	MoqQuicCreditStats get creditGet;
	void creditSet(MoqQuicCreditStats value);
}
final class MoqQuicCryptoHandshakeData extends _Object {
	String debug();
	MoqQuicCryptoHandshakeData clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	typed.Uint8List? get alpnGet;
	void alpnSet(typed.Uint8List? value);
	String? get serverNameGet;
	void serverNameSet(String? value);
}
final class Mp4AtomSpatial extends _Object {
	String debug();
	Mp4AtomSpatial clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSpatial.default_();
	Mp4AtomSt3d? get st3dGet;
	void st3dSet(Mp4AtomSt3d? value);
	Mp4AtomSv3d? get sv3dGet;
	void sv3dSet(Mp4AtomSv3d? value);
	Mp4AtomVexu? get vexuGet;
	void vexuSet(Mp4AtomVexu? value);
	Mp4AtomHfov? get hfovGet;
	void hfovSet(Mp4AtomHfov? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomAny extends _Object {
	String debug();
	Mp4AtomAny clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Mp4AtomFourCc kind();
}
final class Mp4AtomMinf extends _Object {
	String debug();
	Mp4AtomMinf clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMinf.default_();
	Mp4AtomVmhd? get vmhdGet;
	void vmhdSet(Mp4AtomVmhd? value);
	Mp4AtomSmhd? get smhdGet;
	void smhdSet(Mp4AtomSmhd? value);
	Mp4AtomNmhd? get nmhdGet;
	void nmhdSet(Mp4AtomNmhd? value);
	Mp4AtomSthd? get sthdGet;
	void sthdSet(Mp4AtomSthd? value);
	Mp4AtomHmhd? get hmhdGet;
	void hmhdSet(Mp4AtomHmhd? value);
	Mp4AtomDinf get dinfGet;
	void dinfSet(Mp4AtomDinf value);
	Mp4AtomStbl get stblGet;
	void stblSet(Mp4AtomStbl value);
}
final class Mp4AtomCompressor extends _Object {
	String debug();
	Mp4AtomCompressor clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCompressor.default_();
}
final class RustlsPkiTypesDer extends _Object {
	String debug();
	RustlsPkiTypesDer clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory RustlsPkiTypesDer.fromSlice(typed.Uint8List der);
}
final class MoqTokioQuicDemuxDatagram extends _Object {
	String debug();
	MoqTokioQuicDemuxDatagram clone();
	SocketAddr get remoteGet;
	void remoteSet(SocketAddr value);
	IpAddr? get localGet;
	void localSet(IpAddr? value);
	typed.Uint8List get dataGet;
	void dataSet(typed.Uint8List value);
}
final class MoqSockDemuxPin extends _Object {
	String debug();
}
final class MoqTokioQuicDriverSendOffset extends _Object {
	String debug();
	MoqTokioQuicDriverSendOffset clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqTokioQuicDriverRecvError extends _Object implements Exception {
	@override String toString();
	String debug();
	MoqTokioQuicDriverRecvError clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MoqQuicFrameStats extends _Object {
	String debug();
	MoqQuicFrameStats clone();
	factory MoqQuicFrameStats.default_();
	U64 get acksGet;
	void acksSet(U64 value);
	U64 get ackFrequencyGet;
	void ackFrequencySet(U64 value);
	U64 get cryptoGet;
	void cryptoSet(U64 value);
	U64 get connectionCloseGet;
	void connectionCloseSet(U64 value);
	U64 get dataBlockedGet;
	void dataBlockedSet(U64 value);
	U64 get datagramGet;
	void datagramSet(U64 value);
	int get handshakeDoneGet;
	void handshakeDoneSet(int value);
	U64 get immediateAckGet;
	void immediateAckSet(U64 value);
	U64 get maxDataGet;
	void maxDataSet(U64 value);
	U64 get maxStreamDataGet;
	void maxStreamDataSet(U64 value);
	U64 get maxStreamsBidiGet;
	void maxStreamsBidiSet(U64 value);
	U64 get maxStreamsUniGet;
	void maxStreamsUniSet(U64 value);
	U64 get newConnectionIdGet;
	void newConnectionIdSet(U64 value);
	U64 get newTokenGet;
	void newTokenSet(U64 value);
	U64 get pathChallengeGet;
	void pathChallengeSet(U64 value);
	U64 get pathResponseGet;
	void pathResponseSet(U64 value);
	U64 get pingGet;
	void pingSet(U64 value);
	U64 get resetStreamGet;
	void resetStreamSet(U64 value);
	U64 get retireConnectionIdGet;
	void retireConnectionIdSet(U64 value);
	U64 get streamDataBlockedGet;
	void streamDataBlockedSet(U64 value);
	U64 get streamsBlockedBidiGet;
	void streamsBlockedBidiSet(U64 value);
	U64 get streamsBlockedUniGet;
	void streamsBlockedUniSet(U64 value);
	U64 get stopSendingGet;
	void stopSendingSet(U64 value);
	U64 get streamGet;
	void streamSet(U64 value);
}
final class MoqQuicPathStats extends _Object {
	String debug();
	MoqQuicPathStats clone();
	factory MoqQuicPathStats.default_();
	RustDuration get rttGet;
	void rttSet(RustDuration value);
	RustDuration get minRttGet;
	void minRttSet(RustDuration value);
	U64 get cwndGet;
	void cwndSet(U64 value);
	U64? get bandwidthEstimateGet;
	void bandwidthEstimateSet(U64? value);
	U64 get congestionEventsGet;
	void congestionEventsSet(U64 value);
	U64 get spuriousCongestionEventsGet;
	void spuriousCongestionEventsSet(U64 value);
	U64 get lostPacketsGet;
	void lostPacketsSet(U64 value);
	U64 get lostBytesGet;
	void lostBytesSet(U64 value);
	U64 get sentPacketsGet;
	void sentPacketsSet(U64 value);
	U64 get sentPlpmtudProbesGet;
	void sentPlpmtudProbesSet(U64 value);
	U64 get lostPlpmtudProbesGet;
	void lostPlpmtudProbesSet(U64 value);
	U64 get sentBandwidthProbesGet;
	void sentBandwidthProbesSet(U64 value);
	U64 get sentBandwidthProbeBytesGet;
	void sentBandwidthProbeBytesSet(U64 value);
	U64 get blackHolesDetectedGet;
	void blackHolesDetectedSet(U64 value);
	int get currentMtuGet;
	void currentMtuSet(int value);
}
final class Mp4AtomSv3d extends _Object {
	String debug();
	Mp4AtomSv3d clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSv3d.default_();
	Mp4AtomSvhd? get svhdGet;
	void svhdSet(Mp4AtomSvhd? value);
	Mp4AtomProj? get projGet;
	void projSet(Mp4AtomProj? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomVexu extends _Object {
	String debug();
	Mp4AtomVexu clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVexu.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomVexuProj? get projGet;
	void projSet(Mp4AtomVexuProj? value);
	Mp4AtomEyes? get eyesGet;
	void eyesSet(Mp4AtomEyes? value);
	Mp4AtomPack? get packGet;
	void packSet(Mp4AtomPack? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomIprp extends _Object {
	String debug();
	Mp4AtomIprp clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIprp.default_();
	Mp4AtomIpco get ipcoGet;
	void ipcoSet(Mp4AtomIpco value);
	List<Mp4AtomIpma> get ipmaGet;
	void ipmaSet(List<Mp4AtomIpma> value);
}
final class Mp4AtomIpco extends _Object {
	String debug();
	Mp4AtomIpco clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomIpco.default_();
	List<Mp4AtomAny> get propertiesGet;
	void propertiesSet(List<Mp4AtomAny> value);
}
final class Mp4AtomMoov extends _Object {
	String debug();
	Mp4AtomMoov clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMoov.default_();
	Mp4AtomMvhd get mvhdGet;
	void mvhdSet(Mp4AtomMvhd value);
	Mp4AtomMeta? get metaGet;
	void metaSet(Mp4AtomMeta? value);
	Mp4AtomMvex? get mvexGet;
	void mvexSet(Mp4AtomMvex? value);
	List<Mp4AtomTrak> get trakGet;
	void trakSet(List<Mp4AtomTrak> value);
	Mp4AtomUdta? get udtaGet;
	void udtaSet(Mp4AtomUdta? value);
	Mp4AtomAinf? get ainfGet;
	void ainfSet(Mp4AtomAinf? value);
}
final class Mp4AtomStbl extends _Object {
	String debug();
	Mp4AtomStbl clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStbl.default_();
	Mp4AtomStsd get stsdGet;
	void stsdSet(Mp4AtomStsd value);
	Mp4AtomStts get sttsGet;
	void sttsSet(Mp4AtomStts value);
	Mp4AtomCtts? get cttsGet;
	void cttsSet(Mp4AtomCtts? value);
	Mp4AtomStss? get stssGet;
	void stssSet(Mp4AtomStss? value);
	Mp4AtomStsc get stscGet;
	void stscSet(Mp4AtomStsc value);
	Mp4AtomStsz get stszGet;
	void stszSet(Mp4AtomStsz value);
	Mp4AtomStco? get stcoGet;
	void stcoSet(Mp4AtomStco? value);
	Mp4AtomCo64? get co64Get;
	void co64Set(Mp4AtomCo64? value);
	List<Mp4AtomSbgp> get sbgpGet;
	void sbgpSet(List<Mp4AtomSbgp> value);
	List<Mp4AtomSgpd> get sgpdGet;
	void sgpdSet(List<Mp4AtomSgpd> value);
	List<Mp4AtomSubs> get subsGet;
	void subsSet(List<Mp4AtomSubs> value);
	List<Mp4AtomSaiz> get saizGet;
	void saizSet(List<Mp4AtomSaiz> value);
	List<Mp4AtomSaio> get saioGet;
	void saioSet(List<Mp4AtomSaio> value);
	Mp4AtomCslg? get cslgGet;
	void cslgSet(Mp4AtomCslg? value);
}
final class Mp4AtomStsd extends _Object {
	String debug();
	Mp4AtomStsd clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomStsd.default_();
	List<Mp4AtomCodec> get codecsGet;
	void codecsSet(List<Mp4AtomCodec> value);
}
final class Mp4AtomNmhd extends _Object {
	String debug();
	Mp4AtomNmhd clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomNmhd.default_();
}
final class Mp4AtomSthd extends _Object {
	String debug();
	Mp4AtomSthd clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomSthd.default_();
}
final class Mp4AtomMoof extends _Object {
	String debug();
	Mp4AtomMoof clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomMoof.default_();
	Mp4AtomMfhd get mfhdGet;
	void mfhdSet(Mp4AtomMfhd value);
	List<Mp4AtomTraf> get trafGet;
	void trafSet(List<Mp4AtomTraf> value);
}
final class Mp4AtomTraf extends _Object {
	String debug();
	Mp4AtomTraf clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomTraf.default_();
	Mp4AtomTfhd get tfhdGet;
	void tfhdSet(Mp4AtomTfhd value);
	Mp4AtomTfdt? get tfdtGet;
	void tfdtSet(Mp4AtomTfdt? value);
	List<Mp4AtomTrun> get trunGet;
	void trunSet(List<Mp4AtomTrun> value);
	List<Mp4AtomSbgp> get sbgpGet;
	void sbgpSet(List<Mp4AtomSbgp> value);
	List<Mp4AtomSgpd> get sgpdGet;
	void sgpdSet(List<Mp4AtomSgpd> value);
	List<Mp4AtomSubs> get subsGet;
	void subsSet(List<Mp4AtomSubs> value);
	List<Mp4AtomSaiz> get saizGet;
	void saizSet(List<Mp4AtomSaiz> value);
	List<Mp4AtomSaio> get saioGet;
	void saioSet(List<Mp4AtomSaio> value);
	Mp4AtomMeta? get metaGet;
	void metaSet(Mp4AtomMeta? value);
	Mp4AtomSenc? get sencGet;
	void sencSet(Mp4AtomSenc? value);
	Mp4AtomUdta? get udtaGet;
	void udtaSet(Mp4AtomUdta? value);
}
final class Mp4AtomProj extends _Object {
	String debug();
	Mp4AtomProj clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomProj.default_();
	Mp4AtomPrhd? get prhdGet;
	void prhdSet(Mp4AtomPrhd? value);
	Mp4AtomProjKind? get kindGet;
	void kindSet(Mp4AtomProjKind? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomVexuProj extends _Object {
	String debug();
	Mp4AtomVexuProj clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomVexuProj.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomPrji? get prjiGet;
	void prjiSet(Mp4AtomPrji? value);
	Mp4AtomVexuProjKind? get kindGet;
	void kindSet(Mp4AtomVexuProjKind? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomEyes extends _Object {
	String debug();
	Mp4AtomEyes clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomEyes.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomStri? get striGet;
	void striSet(Mp4AtomStri? value);
	Mp4AtomHero? get heroGet;
	void heroSet(Mp4AtomHero? value);
	Mp4AtomCams? get camsGet;
	void camsSet(Mp4AtomCams? value);
	Mp4AtomCmfy? get cmfyGet;
	void cmfySet(Mp4AtomCmfy? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomPack extends _Object {
	String debug();
	Mp4AtomPack clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomPack.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomPkin? get pkinGet;
	void pkinSet(Mp4AtomPkin? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomCams extends _Object {
	String debug();
	Mp4AtomCams clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCams.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomBlin? get blinGet;
	void blinSet(Mp4AtomBlin? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Mp4AtomCmfy extends _Object {
	String debug();
	Mp4AtomCmfy clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Mp4AtomCmfy.default_();
	Mp4AtomMust? get mustGet;
	void mustSet(Mp4AtomMust? value);
	Mp4AtomDadj? get dadjGet;
	void dadjSet(Mp4AtomDadj? value);
	List<Mp4AtomAny> get unknownGet;
	void unknownSet(List<Mp4AtomAny> value);
}
final class Url extends _Object {
	@override String toString();
	String debug();
	Url clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory Url.parse(String input);
}
final class Json extends _Object {
	factory Json.parse(String input);
	String toText();
}
final class Instant extends _Object {
	String debug();
	Instant clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class RustlsServerConfig extends _Object {
	RustlsServerConfig clone();
}
final class RustlsResolvesClientCert extends _Object {
	RustlsResolvesClientCert clone();
}
final class RustlsKeyLog extends _Object {
	RustlsKeyLog clone();
}
final class RustlsTimeProvider extends _Object {
	RustlsTimeProvider clone();
}
final class RustlsCertDecompressor extends _Object {
}
final class RustlsCertCompressor extends _Object {
}
final class RustlsCompressionCache extends _Object {
	RustlsCompressionCache clone();
}
final class RustlsCryptoProvider extends _Object {
	RustlsCryptoProvider clone();
}
final class MoqCongestionFactory extends _Object {
	MoqCongestionFactory clone();
}
final class MoqQuicError extends _Object {
}
abstract class TransportMetrics {
	void close();
}
final class StringException implements Exception {
	final String error;
	const StringException(this.error);
	@override String toString();
}
const PATH_MAX_PARTS = U64.fromBits(32);
const PATH_MAX_BYTES = U64.fromBits(4096);
const PATTERN_MAX_SEGMENTS = U64.fromBits(32);
const PATTERN_MAX_INTERSECTIONS = U64.fromBits(1024);
const SETUP_TOKEN_OUT_OF_BAND = U64.fromBits(0);
const TRANSPORT_MIN_RECEIVE_WINDOW = U64.fromBits(1048576);
const TRANSPORT_SETUP_RECEIVE_WINDOW = U64.fromBits(131102);
const GROUP_MAX_CACHE_BYTES = U64.fromBits(33554432);
const HANG_CATALOG_MOQ_EPOCH_UNIX_MILLIS = U64.fromBits(1577836800000);
const HANG_CATALOG_MAX_SAFE_INTEGER = U64.fromBits(9007199254740991);
const MUX_CONTAINER_DEFAULT_MAX_BUFFER = U64.fromBits(33554432);
const MUX_CONTAINER_FMP4_MAX_HELD_BYTES = U64.fromBits(33554432);
const MUX_CONTAINER_FMP4_MAX_METADATA_BYTES = U64.fromBits(2097152);
const MUX_CONTAINER_FMP4_MAX_FRAGMENT_SAMPLES = U64.fromBits(131072);
const MUX_CONTAINER_FMP4_MAX_TRACKS = U64.fromBits(256);
const MOQ_QUIC_VAR_INT_MAX_SIZE = U64.fromBits(8);
const MP4_ATOM_ST3D_MONO = 0;
const MP4_ATOM_ST3D_TOP_BOTTOM = 1;
const MP4_ATOM_ST3D_LEFT_RIGHT = 2;
const MP4_ATOM_ST3D_STEREO_CUSTOM = 3;
const MP4_ATOM_ST3D_RIGHT_LEFT = 4;
const MP4_ATOM_HFOV_DENOMINATOR = 1000;
const MP4_ATOM_ESDS_ES_DESCRIPTOR_TAG = 3;
const MP4_ATOM_ESDS_DECODER_CONFIG_TAG = 4;
const MP4_ATOM_ESDS_SL_CONFIG_TAG = 6;
const MP4_ATOM_U24_MAX = 16777215;
const MP4_ATOM_HERO_NONE = 0;
const MP4_ATOM_HERO_LEFT = 1;
const MP4_ATOM_HERO_RIGHT = 2;
const MP4_ATOM_ESDS_DECODER_SPECIFIC_TAG = 5;
const MP4_ATOM_DADJ_DENOMINATOR = 10000;
String get IMPLEMENTATION;
List<String> get ALPNS;
ClockInstant clockNow();
ClockWall clockWall();
(ClockInstant, ClockWall) clockOrigin();
ClockRng clockRng();
U64 clockSeed();
Cancelable<String> clockDrive(ClockInstant? Function(ClockInstant, Waiter) poll);
ClockInterval clockInterval(RustDuration period);
ClockInterval clockIntervalAt(ClockInstant start, RustDuration period);
Cancelable<void> clockSleep(RustDuration duration);
Cancelable<void> clockSleepUntil(ClockInstant at);
Cancelable<void> clockTimeout(RustDuration duration, Future<void> future);
Cancelable<void> clockTimeoutAt(ClockInstant deadline, Future<void> future);
Cancelable<Error> timeRunDriver(Driver driver);
Cancelable<Error> timeRunOriginDriver(OriginDriver driver);
U64 varintDecodeQuic(Mut<typed.Uint8List> r);
void varintEncodeQuic(U64 value, Mut<typed.Uint8List> w);
RustDuration get CACHE_DEFAULT_EXPIRY;
String get HANG_ECHO_SUFFIX;
String get HANG_ECHO_TRACK;
bool hangEchoIsEcho(Path path);
HangCatalogPriorities get HANG_CATALOG_PRIORITY;
RustDuration get HANG_CONTAINER_MAX_AGE;
Timescale get HANG_CONTAINER_TIMESCALE;
TrackInfo hangContainerTrackInfo(int priority);
ArrayU8x32 tokioTlsParseFingerprint(String value);
OriginProducer tokioOriginSpawn();
(TransportMemorySession, TransportMemorySession) transportMemoryPair(TransportMemoryConfig config);
HangCatalogAudioConfig muxCodecAacConfig(typed.Uint8List init);
typed.Uint8List get MUX_CODEC_ANNEXB_START_CODE;
typed.Uint8List muxCodecAnnexbFromLengthPrefixed(typed.Uint8List payload, U64 lengthSize, typed.Uint8List? prefix);
typed.Uint8List muxCodecAnnexbBuildPrefix(List<typed.Uint8List> nals);
void muxCodecAnnexbLengthPrefixedToAnnexb(typed.Uint8List data, U64 lengthSize, Mut<typed.Uint8List> out);
U64? muxCodecAnnexbAfterStartCode(typed.Uint8List b);
(U64, U64)? muxCodecAnnexbFindStartCode(typed.Uint8List b);
HangCatalogAudioConfig muxCodecFlacConfig(typed.Uint8List init);
HangCatalogVideoConfig muxCodecH264Config(typed.Uint8List buf);
HangCatalogVideoConfig muxCodecH265Config(typed.Uint8List buf);
HangCatalogAudioConfig muxCodecMp3Config(typed.Uint8List init);
RustDuration get MUX_CODEC_OPUS_WARMUP;
int? muxCodecOpusPacketSamples(typed.Uint8List packet);
HangCatalogAudioConfig muxCodecOpusConfig(typed.Uint8List init);
MuxContainerMp4Finish? muxContainerMp4Repair(SeekableReader file);
