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
	ClockInstant saturatingAdd(RustDuration duration);
	ClockInstant saturatingSub(RustDuration duration);
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
	static ClockWall get MOQ_EPOCH;
	factory ClockWall.now();
	RustDuration? checkedDurationSince(ClockWall earlier);
	RustDuration saturatingDurationSince(ClockWall earlier);
	ClockWall? checkedAdd(RustDuration duration);
	ClockWall saturatingAdd(RustDuration duration);
	ClockWall? checkedSub(RustDuration duration);
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
	U64 get routesRefusedGet;
	StatsPresence routesRefusedSet(U64 value);
	StatsRoutes get routesGet;
	StatsPresence routesSet(StatsRoutes value);
	StatsPresence add(StatsPresence other);
	U64 active();
}
final class StatsActivity {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsActivity.default_();
	U64 get idleGet;
	StatsActivity idleSet(U64 value);
	U64 get activeGet;
	StatsActivity activeSet(U64 value);
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
	U64 get txMessagesGet;
	StatsTotals txMessagesSet(U64 value);
	U64 get rxMessagesGet;
	StatsTotals rxMessagesSet(U64 value);
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
	U64 get membershipGet;
	StatsRefusals membershipSet(U64 value);
	U64 get expiredGet;
	StatsRefusals expiredSet(U64 value);
	U64 count(StatsRefusalKind kind);
	StatsRefusals add(StatsRefusals other);
	bool regressed(StatsRefusals previous);
}
final class StatsRouteCloses {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsRouteCloses.default_();
	U64 get stalledGet;
	StatsRouteCloses stalledSet(U64 value);
	U64 get overrunGet;
	StatsRouteCloses overrunSet(U64 value);
	U64 get malformedGet;
	StatsRouteCloses malformedSet(U64 value);
	U64 get roleGet;
	StatsRouteCloses roleSet(U64 value);
	U64 get unaskedGet;
	StatsRouteCloses unaskedSet(U64 value);
	U64 get regressedGet;
	StatsRouteCloses regressedSet(U64 value);
	U64 get otherGet;
	StatsRouteCloses otherSet(U64 value);
	U64 count(StatsRouteClose kind);
	U64 total();
	StatsRouteCloses add(StatsRouteCloses other);
	bool regressed(StatsRouteCloses previous);
}
final class StatsRouteRefusals {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsRouteRefusals.default_();
	U64 get pathGet;
	StatsRouteRefusals pathSet(U64 value);
	U64 get scopeGet;
	StatsRouteRefusals scopeSet(U64 value);
	U64 get loopedGet;
	StatsRouteRefusals loopedSet(U64 value);
	U64 count(StatsRouteRefusal kind);
	U64 total();
	StatsRouteRefusals add(StatsRouteRefusals other);
	bool regressed(StatsRouteRefusals previous);
}
final class StatsRoutes {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsRoutes.default_();
	U64 get changedGet;
	StatsRoutes changedSet(U64 value);
	U64 get rewrittenGet;
	StatsRoutes rewrittenSet(U64 value);
	StatsWithdrawals get sentGet;
	StatsRoutes sentSet(StatsWithdrawals value);
	StatsWithdrawals get receivedGet;
	StatsRoutes receivedSet(StatsWithdrawals value);
	StatsRouteRefusals get refusedGet;
	StatsRoutes refusedSet(StatsRouteRefusals value);
	StatsRouteCloses get closedGet;
	StatsRoutes closedSet(StatsRouteCloses value);
	StatsRoutes add(StatsRoutes other);
	bool regressed(StatsRoutes previous);
}
final class StatsWithdrawals {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory StatsWithdrawals.default_();
	U64 get deadGet;
	StatsWithdrawals deadSet(U64 value);
	U64 get drainGet;
	StatsWithdrawals drainSet(U64 value);
	U64 count(StatsWithdrawal kind);
	U64 total();
	StatsWithdrawals add(StatsWithdrawals other);
	bool regressed(StatsWithdrawals previous);
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
final class DatagramDatagram {
	final U64 sequence;
	final Timestamp timestamp;
	final typed.Uint8List payload;
	const DatagramDatagram({required this.sequence, required this.timestamp, required this.payload});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class FrameInfo {
	final U64 size;
	final Timestamp timestamp;
	const FrameInfo({required this.size, required this.timestamp});
	String debug();
}
final class GroupInfo {
	final U64 start;
	const GroupInfo({required this.start});
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
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackPosition.default_();
	U64 get groupGet;
	TrackPosition groupSet(U64 value);
	U64 get frameGet;
	TrackPosition frameSet(U64 value);
	factory TrackPosition(U64 group, U64 frame);
	factory TrackPosition.group(U64 group);
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
}
final class OriginCost {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginCost.default_();
	factory OriginCost(U64 cost);
	U64 value();
}
final class OriginPriority {
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory OriginPriority.default_();
	factory OriginPriority(int priority);
	int value();
}
final class HangCatalogAac {
	final int profile;
	const HangCatalogAac({required this.profile});
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
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
final class MuxContainerPlayout {
	final TrackDelay delay;
	final RustDuration warmup;
	const MuxContainerPlayout({required this.delay, required this.warmup});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory MuxContainerPlayout.default_();
	factory MuxContainerPlayout.new_(TrackDelay delay, RustDuration warmup);
	TrackSubscription subscription(TrackSubscription subscription);
	MuxContainerConsumerContainer consumerContainer(TrackSubscriber track, MuxCatalogHangContainer format);
	MuxContainerConsumerFmp4Wire consumerFmp4Wire(TrackSubscriber track, MuxContainerFmp4Wire format);
	MuxContainerConsumerLegacyWire consumerLegacyWire(TrackSubscriber track, MuxContainerLegacyWire format);
	MuxContainerConsumerLocWire consumerLocWire(TrackSubscriber track, MuxContainerLocWire format);
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
final class MuxCodecH265VideoSignal {
	final int primaries;
	final int transfer;
	final int matrix;
	final bool fullRange;
	const MuxCodecH265VideoSignal({required this.primaries, required this.transfer, required this.matrix, required this.fullRange});
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
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
final class MuxContainerTsDescriptor {
	final int tag;
	final typed.Uint8List data;
	const MuxContainerTsDescriptor({required this.tag, required this.data});
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
final class RustlsCryptoHpkeHpkePublicKey {
	final typed.Uint8List $0;
	const RustlsCryptoHpkeHpkePublicKey(this.$0);
	String debug();
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
final class SessionErrorTooManyRequests extends SessionError {
	const SessionErrorTooManyRequests();
}
final class SessionErrorGoawayTimeout extends SessionError {
	const SessionErrorGoawayTimeout();
}
final class SessionErrorTimeout extends SessionError {
	const SessionErrorTimeout();
}
final class SessionErrorVersion extends SessionError {
	const SessionErrorVersion();
}
final class SessionErrorMalformedAuthToken extends SessionError {
	const SessionErrorMalformedAuthToken();
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
enum Retry {
	same(0),
	reroute(1),
	never(2),
	const Retry(this._c);
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
	handoff(1),
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
	tls(5),
	const TokioTransport(this._c);
	@override String toString();
	String debug();
	String asStr();
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
enum ReconnectVerdict {
	retry(0),
	unusable(1),
	settled(2),
	const ReconnectVerdict(this._c);
	String debug();
}
enum ReconnectConfigError {
	empty(0),
	healthy(1),
	const ReconnectConfigError(this._c);
	@override String toString();
	String debug();
}
enum ReconnectRedirect {
	follow(0),
	sameHost(1),
	ignore(2),
	const ReconnectRedirect(this._c);
	String debug();
	factory ReconnectRedirect.default_();
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
	membership(4),
	expired(5),
	const StatsRefusalKind(this._c);
	String debug();
	static List<StatsRefusalKind> get ALL;
	String asStr();
}
enum StatsRouteClose {
	stalled(0),
	overrun(1),
	malformed(2),
	role(3),
	unasked(4),
	regressed(5),
	other(6),
	const StatsRouteClose(this._c);
	String debug();
	static List<StatsRouteClose> get ALL;
	String asStr();
}
enum StatsRouteRefusal {
	path(0),
	scope(1),
	looped(2),
	const StatsRouteRefusal(this._c);
	String debug();
	static List<StatsRouteRefusal> get ALL;
	String asStr();
}
enum StatsWithdrawal {
	dead(0),
	drain(1),
	const StatsWithdrawal(this._c);
	String debug();
	static List<StatsWithdrawal> get ALL;
	String asStr();
}
enum TransportAddress {
	uri(0),
	setup(1),
	const TransportAddress(this._c);
	String debug();
}
sealed class TransportLimitsError {
	const TransportLimitsError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class TransportLimitsErrorUnsupported extends TransportLimitsError {
	const TransportLimitsErrorUnsupported();
}
final class TransportLimitsErrorRefused extends TransportLimitsError {
	final String $0;
	const TransportLimitsErrorRefused(this.$0);
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
enum BroadcastKind {
	track(0),
	datagrams(1),
	const BroadcastKind(this._c);
	String debug();
}
enum TrackCompression {
	none(0),
	deflate(1),
	const TrackCompression(this._c);
	String debug();
	factory TrackCompression.default_();
}
sealed class TrackAnchor {
	const TrackAnchor();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class TrackAnchorRelative extends TrackAnchor {
	final U64 $0;
	const TrackAnchorRelative(this.$0);
}
final class TrackAnchorOldest extends TrackAnchor {
	const TrackAnchorOldest();
}
final class TrackAnchorTime extends TrackAnchor {
	final U64 $0;
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
enum OriginRouteKind {
	start(0),
	update(1),
	end(2),
	offline(3),
	restart(4),
	const OriginRouteKind(this._c);
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
enum HangCatalogColorRange {
	limited(0),
	full(1),
	const HangCatalogColorRange(this._c);
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
sealed class MuxContainerReorderError {
	const MuxContainerReorderError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxContainerReorderErrorDepth extends MuxContainerReorderError {
	final int reorder;
	final int max;
	final String codec;
	const MuxContainerReorderErrorDepth({required this.reorder, required this.max, required this.codec});
}
final class MuxContainerReorderErrorDeepened extends MuxContainerReorderError {
	final int declared;
	final int reorder;
	const MuxContainerReorderErrorDeepened({required this.declared, required this.reorder});
}
final class MuxContainerReorderErrorReordered extends MuxContainerReorderError {
	final U64 $0;
	const MuxContainerReorderErrorReordered(this.$0);
}
final class MuxContainerReorderErrorPresentation extends MuxContainerReorderError {
	const MuxContainerReorderErrorPresentation();
}
final class MuxContainerReorderErrorTie extends MuxContainerReorderError {
	const MuxContainerReorderErrorTie();
}
final class MuxContainerReorderErrorUndrained extends MuxContainerReorderError {
	const MuxContainerReorderErrorUndrained();
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
sealed class TokioListenBind {
	const TokioListenBind();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
	SocketAddr resolve();
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
enum TransportPollAcked {
	delivered(0),
	unsupported(1),
	sent(2),
	const TransportPollAcked(this._c);
	String debug();
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
final class MuxContainerTsStepIdentity extends MuxContainerTsStep {
	final int transportStreamId;
	final int programNumber;
	const MuxContainerTsStepIdentity({required this.transportStreamId, required this.programNumber});
}
sealed class MuxCodecAacAdtsError implements Exception {
	const MuxCodecAacAdtsError();
	@override String toString();
	String debug();
	@override bool operator ==(Object other);
	@override int get hashCode;
}
final class MuxCodecAacAdtsErrorTruncated extends MuxCodecAacAdtsError {
	const MuxCodecAacAdtsErrorTruncated();
}
final class MuxCodecAacAdtsErrorSyncword extends MuxCodecAacAdtsError {
	const MuxCodecAacAdtsErrorSyncword();
}
final class MuxCodecAacAdtsErrorSampleRate extends MuxCodecAacAdtsError {
	const MuxCodecAacAdtsErrorSampleRate();
}
final class MuxCodecAacAdtsErrorFrameLength extends MuxCodecAacAdtsError {
	const MuxCodecAacAdtsErrorFrameLength();
}
final class MuxCodecAacAdtsErrorPacketLength extends MuxCodecAacAdtsError {
	const MuxCodecAacAdtsErrorPacketLength();
}
final class MuxCodecAacAdtsErrorObjectType extends MuxCodecAacAdtsError {
	final int $0;
	const MuxCodecAacAdtsErrorObjectType(this.$0);
}
final class MuxCodecAacAdtsErrorRate extends MuxCodecAacAdtsError {
	final int $0;
	const MuxCodecAacAdtsErrorRate(this.$0);
}
final class MuxCodecAacAdtsErrorChannelConfig extends MuxCodecAacAdtsError {
	final int $0;
	const MuxCodecAacAdtsErrorChannelConfig(this.$0);
}
final class MuxCodecAacAdtsErrorTooLarge extends MuxCodecAacAdtsError {
	final U64 $0;
	const MuxCodecAacAdtsErrorTooLarge(this.$0);
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
	bbr(0),
	cubic(1),
	unlimited(2),
	const MoqTokioQuicCongestionControl(this._c);
	String debug();
	factory MoqTokioQuicCongestionControl.default_();
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
enum MoqQuicConnectionMuxVersion {
	qMux00(0),
	qMux01(1),
	qMux02(2),
	const MoqQuicConnectionMuxVersion(this._c);
	String debug();
	static List<MoqQuicConnectionMuxVersion> get ALL;
	bool usesRecords();
	String alpn();
	String prefix();
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
enum MoqSockUdpEcnCodepoint {
	ect0(2),
	ect1(1),
	ce(3),
	const MoqSockUdpEcnCodepoint(this._c);
	String debug();
	static MoqSockUdpEcnCodepoint? fromBits(int x);
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
	Server withAllocator(BandwidthAllocator allocator);
	Server withStats(StatsSession stats);
	Server withLimits(SessionLimits limits);
	Server withVersions(Versions versions);
	Server withImplementation(String implementation);
	Server withTimeout(RustDuration? timeout);
	ClockInstant? deadline(ClockInstant arrived);
	Server withReceiveWindow(U64 window);
	Cancelable<(Session, Driver)> acceptMemorySession(ClockInstant now, TransportMemorySession session);
	Cancelable<(Session, Driver)> acceptQmuxSession(ClockInstant now, QmuxSession session);
	Cancelable<ServerHandshake> handshakeMemorySession(ClockInstant now, TransportMemorySession session);
	Cancelable<ServerHandshake> handshakeQmuxSession(ClockInstant now, QmuxSession session);
}
final class Session extends _Object {
	String debug();
	Session clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
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
	void goaway(GoawayGoaway goaway);
	GoawayConsumer draining();
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
	Client withLimits(SessionLimits limits);
	Client withAllocator(BandwidthAllocator allocator);
	Client withViewers(BandwidthViewers viewers);
	Client withVersions(Versions versions);
	Client withUrl(Url url);
	Url? url();
	Client withToken(SetupToken token);
	Client withAuth(AuthHandle auth);
	Client withImplementation(String implementation);
	Client withCost(OriginCost cost);
	Client withPeerNode(U64 node);
	Cancelable<(Session, Driver)> connectMemorySession(ClockInstant now, TransportMemorySession session);
	Cancelable<(Session, Driver)> connectQmuxSession(ClockInstant now, QmuxSession session);
}
final class Error extends _Object implements Exception {
	@override String toString();
	String debug();
	Error clone();
	SessionError? session();
	StreamError? stream();
	factory Error.fromTransportError(Error err);
	factory Error.fromTransportMemoryError(TransportMemoryError err);
	Retry retry();
}
final class Timestamp extends _Object {
	String debug();
	Timestamp clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static Timestamp get ZERO;
	factory Timestamp(U64 value, Timescale scale);
	factory Timestamp.unscaled(U64 key);
	factory Timestamp.fromScale(U64 value, U64 unitsPerSecond);
	factory Timestamp.fromSecs(U64 seconds);
	factory Timestamp.fromMillis(U64 millis);
	factory Timestamp.fromMicros(U64 micros);
	factory Timestamp.fromNanos(U64 nanos);
	U64 value();
	Timescale? scale();
	bool isZero();
	Timestamp convert(Timescale newScale);
	BigInt asScale(Timescale target);
	U64 asSecs();
	BigInt asMillis();
	BigInt asMicros();
	BigInt asNanos();
	Timestamp checkedAdd(Timestamp rhs);
	Timestamp checkedSub(Timestamp rhs);
}
final class Version extends _Object {
	@override String toString();
	String debug();
	Version clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static List<Version> get ALL;
	bool hasRequestPath();
	String asStr();
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
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	static String get DEFAULT_NAME;
	static TrackInfo defaultTrackInfo();
	static TrackSubscription defaultSubscription();
	void checkRenditions();
	HangCatalog withoutExt();
	factory HangCatalog.fromSlice(typed.Uint8List v);
	factory HangCatalog.fromReader(typed.Uint8List reader);
	String toJson();
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
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	MuxCatalogHangExtra get extGet;
	void extSet(MuxCatalogHangExtra value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	void checkRenditions();
	HangCatalog withoutExt();
	factory HangCatalogExtra.fromSlice(typed.Uint8List v);
	factory HangCatalogExtra.fromReader(typed.Uint8List reader);
	String toJson();
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
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	MuxContainerTsExt get extGet;
	void extSet(MuxContainerTsExt value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	void checkRenditions();
	HangCatalog withoutExt();
	factory HangCatalogExt.fromSlice(typed.Uint8List v);
	factory HangCatalogExt.fromReader(typed.Uint8List reader);
	String toJson();
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
final class MuxIngest extends _Object {
	String debug();
	MuxIngest clone();
	factory MuxIngest.default_();
	RustDuration? get maxAgeGet;
	void maxAgeSet(RustDuration? value);
	BandwidthAllocator get bandwidthGet;
	void bandwidthSet(BandwidthAllocator value);
	RustDuration get lingerGet;
	void lingerSet(RustDuration value);
	BandwidthLimit? get rateGet;
	void rateSet(BandwidthLimit? value);
	MuxCatalogConfig catalog();
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
	MoqTokioCliBound? get maxGet;
	void maxSet(MoqTokioCliBound? value);
	TokioDuration? get healthyGet;
	void healthySet(TokioDuration? value);
	MoqTokioCliBound? get timeoutGet;
	void timeoutSet(MoqTokioCliBound? value);
	RustDuration initial();
	RustDuration? max();
	RustDuration? timeout();
}
final class TokioClient extends _Object {
	TokioClient clone();
	factory TokioClient(TokioClientConfig config);
	TokioClient withPublisherProducer(OriginProducer publish);
	TokioClient withPublisherConsumer(OriginConsumer publish);
	TokioClient withSubscriber(OriginProducer subscribe);
	TokioClient withLimits(SessionLimits limits);
	TokioClient withStats(StatsSession stats);
	TokioClient withCost(U64 cost);
	TokioClient withPeerNode(U64 node);
	TokioClient withToken(SetupToken token);
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
	dasync.Stream<AuthRequest> asStream();
}
final class AuthRequest extends _Object {
	typed.Uint8List token();
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
	AuthCredential add(typed.Uint8List token);
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
	Cancelable<void> answered();
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
}
final class GoawayGoaway extends _Object {
	String debug();
	GoawayGoaway clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory GoawayGoaway.default_();
	factory GoawayGoaway();
	factory GoawayGoaway.redirect(String uri);
	GoawayGoaway withTimeout(RustDuration? timeout);
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
final class ReconnectBackoff extends _Object {
	String debug();
	ReconnectBackoff clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory ReconnectBackoff.default_();
	RustDuration get initialGet;
	void initialSet(RustDuration value);
	int get multiplierGet;
	void multiplierSet(int value);
	RustDuration? get maxGet;
	void maxSet(RustDuration? value);
	RustDuration get healthyGet;
	void healthySet(RustDuration value);
	RustDuration? get timeoutGet;
	void timeoutSet(RustDuration? value);
}
final class ReconnectHandover extends _Object {
	String debug();
	ReconnectHandover clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory ReconnectHandover.default_();
	ReconnectRedirect get redirectGet;
	void redirectSet(ReconnectRedirect value);
	RustDuration get handoverGet;
	void handoverSet(RustDuration value);
}
final class ReconnectConfig extends _Object {
	String debug();
	ReconnectConfig clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory ReconnectConfig.default_();
	ReconnectBackoff get backoffGet;
	void backoffSet(ReconnectBackoff value);
	ReconnectHandover get goawayGet;
	void goawaySet(ReconnectHandover value);
	TokioRestart get restartGet;
	void restartSet(TokioRestart value);
	RustDuration? get timeoutGet;
	void timeoutSet(RustDuration? value);
	bool get onceGet;
	void onceSet(bool value);
	bool get pinnedGet;
	void pinnedSet(bool value);
}
final class ServerHandshake extends _Object {
	String path();
	SetupRole? role();
	U64? peerNode();
	String? peerImplementation();
	AuthHandle auth();
	AuthRequests requests();
	SetupToken? token();
	ServerHandshake withPublisherProducer(OriginProducer publish);
	ServerHandshake withPublisherConsumer(OriginConsumer publish);
	ServerHandshake withSubscriber(OriginProducer subscribe);
	ServerHandshake withPeerNode(U64 node);
	ServerHandshake withStats(StatsSession stats);
	ServerHandshake withLimits(SessionLimits limits);
	ServerHandshake withReceiveWindow(U64 window);
	Cancelable<void> expired();
	Cancelable<(Session, Driver)> ok();
	void close_(Error err);
}
final class ServerAdmission extends _Object {
	String debug();
	ServerAdmission clone();
	factory ServerAdmission.default_();
	ServerAdmitted? enter();
	void drain();
	Cancelable<ClockInstant> close_();
}
final class ServerAdmitted extends _Object {
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
	U64? get datagramsUnroutedGet;
	void datagramsUnroutedSet(U64? value);
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
	List<(StatsTier, StatsActivity)> activity();
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
	void report(StatsReport report, ClockInstant now);
}
final class StatsHandle extends _Object {
	StatsHandle clone();
	factory StatsHandle.default_();
	void refuse(StatsRefusal refusal);
	StatsSession session(StatsSessionInfo session);
	StatsSession request(Path label);
}
final class StatsRefusal extends _Object {
	String debug();
	StatsRefusal clone();
	Path get labelGet;
	void labelSet(Path value);
	StatsRefusalKind get reasonGet;
	void reasonSet(StatsRefusalKind value);
	factory StatsRefusal(Path label, StatsRefusalKind reason);
}
final class StatsSessionInfo extends _Object {
	String debug();
	StatsSessionInfo clone();
	Path get labelGet;
	void labelSet(Path value);
	factory StatsSessionInfo(Path label);
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
final class StatsBill extends _Object {
	String debug();
	StatsBill clone();
	factory StatsBill.default_();
	void deliver(U64 bytes);
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
final class TransportReceiveWindow extends _Object {
	String debug();
	TransportReceiveWindow clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	U64? get connectionGet;
	void connectionSet(U64? value);
	U64 get streamGet;
	void streamSet(U64 value);
	factory TransportReceiveWindow(U64? connection, U64 stream);
}
final class TransportLimits extends _Object {
	String debug();
	TransportLimits clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TransportLimits.default_();
	U64? get receiveWindowGet;
	void receiveWindowSet(U64? value);
}
final class BandwidthLimit extends _Object {
	String debug();
	BandwidthLimit clone();
	factory BandwidthLimit.default_();
	factory BandwidthLimit(BandwidthCap? cap);
	void set(BandwidthCap? cap);
	BandwidthCap? get();
	Cancelable<BandwidthCap?> changed();
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
final class BandwidthCap extends _Object {
	String debug();
	BandwidthCap clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	BandwidthRate get rateGet;
	void rateSet(BandwidthRate value);
	RustDuration get burstGet;
	void burstSet(RustDuration value);
	static RustDuration get DEFAULT_BURST;
	factory BandwidthCap(BandwidthRate rate);
	BandwidthCap withBurst(RustDuration burst);
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
	RustDuration? get cacheDurationGet;
	void cacheDurationSet(RustDuration? value);
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
	TrackRequest reserveUnlistedTrack(String name);
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
	void remove(BroadcastKind kind, String name);
	void finish(BroadcastKind kind, String name);
	void synced();
	void clear();
	void end();
	void abort(Error err);
}
final class BroadcastTracks extends _Object {
	Cancelable<BroadcastTracksEvent?> next();
	BroadcastConsumer broadcast();
	dasync.Stream<BroadcastTracksEvent> asStream();
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
	U64 append(Timestamp timestamp, typed.Uint8List payload);
	void insert(DatagramDatagram datagram);
	DatagramConsumer consume();
	void finish();
	void abort(Error err);
	int? priority();
	Cancelable<int?> priorityChanged();
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
	Cancelable<DatagramDatagram?> recv();
	int priority();
	void setPriority(int priority);
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
	Timestamp rescale(Timestamp timestamp);
}
final class FrameProducer extends _Object {
	GroupProducer group();
	U64 remaining();
	void write(typed.Uint8List chunk);
	GroupProducer finish();
	void abort(Error err);
	U64 get sizeGet;
	Timestamp get timestampGet;
}
final class FrameFrame extends _Object {
	String debug();
	FrameFrame clone();
	Timestamp get timestampGet;
	void timestampSet(Timestamp value);
	typed.Uint8List get payloadGet;
	void payloadSet(typed.Uint8List value);
	factory FrameFrame(Timestamp timestamp, typed.Uint8List payload);
	ClockInstant arrived();
}
final class FrameConsumer extends _Object {
	FrameConsumer clone();
	Cancelable<typed.Uint8List?> readChunk();
	Cancelable<typed.Uint8List> readAll();
	U64 get sizeGet;
	Timestamp get timestampGet;
}
final class GroupProducer extends _Object {
	Timescale? timescale();
	void startAt(U64 index);
	void writeFrame(Timestamp timestamp, typed.Uint8List data);
	FrameProducer createFrame(FrameInfo frame);
	GroupFinished finishAt(U64 end);
	GroupFinished finish();
	void abort(Error err);
	Cancelable<void> ready(FrameInfo frame);
	U64 frameCount();
	GroupConsumer consume();
	Cancelable<void> closed();
	GroupDemand demand();
	U64 get startGet;
}
final class GroupFinished extends _Object {
	void abort(Error err);
	U64 frameCount();
	GroupConsumer consume();
	U64 get startGet;
}
final class GroupDemand extends _Object {
	GroupDemand clone();
	U64 start();
	bool wanted();
	Cancelable<bool> wantedChanged();
	Cancelable<void> closed();
}
final class GroupConsumer extends _Object {
	GroupConsumer clone();
	Timescale? timescale();
	ClockInstant arrived();
	U64 index();
	GroupConsumer withRange(BoundsU64 frames);
	void skipTo(U64 index);
	U64 frameCount();
	U64? end();
	Cancelable<FrameConsumer?> nextFrame();
	Cancelable<FrameFrame?> readFrame();
	Cancelable<U64> finished();
	U64 get startGet;
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
final class TrackFinished extends _Object {
	String name();
	BroadcastInfo broadcast();
	TrackInfo info();
	TrackRequests requests();
	GroupProducer createGroup(GroupInfo group);
	void abort(Error err);
	Cancelable<void> closed();
	TrackConsumer consume();
	TrackSubscriber subscribe(TrackSubscription? subscription);
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
	bool wanted();
	Cancelable<bool> wantedChanged();
	Cancelable<void> refresh();
	Cancelable<void> closed();
}
final class TrackSubscriber extends _Object {
	StatsBill bill();
	TrackInfo info();
	String name();
	BroadcastInfo broadcast();
	TrackControl control();
	Cancelable<GroupConsumer?> recvGroup();
	TrackOrdered ordered();
	bool isClone(TrackSubscriber other);
	Cancelable<U64?> start();
	U64? floor();
	U64? nextInRange(U64 from);
	bool isSkipped(RangeU64 groups);
	Cancelable<TrackPosition> finished();
	TrackSubscriber withRange(BoundsU64 groups);
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
	Cancelable<TrackPosition?> largest();
	Cancelable<void> refresh();
}
final class TrackOrdered extends _Object {
	TrackInfo info();
	String name();
	BroadcastInfo broadcast();
	Cancelable<TrackItem?> nextGroup();
	TrackControl control();
	Cancelable<void> refresh();
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
	Cancelable<TrackPosition> finished();
	Cancelable<TrackPosition?> largest();
	bool isSkipped(RangeU64 groups);
	bool isClone(TrackOrdered other);
}
final class TrackItem extends _Object {
	GroupConsumer? group();
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
	TrackCompression get compressionGet;
	void compressionSet(TrackCompression value);
	TrackRefresh get refreshGet;
	void refreshSet(TrackRefresh value);
	TrackInfo withTimescale(Timescale? timescale);
	TrackInfo withMaxAge(RustDuration? maxAge);
	TrackInfo withPriority(int priority);
	TrackInfo withCompression(TrackCompression compression);
	TrackInfo withRefresh(TrackRefresh refresh);
	Timestamp rescale(Timestamp timestamp);
	Timestamp rescaleIn(Timestamp timestamp, U64 key);
}
final class TrackRefresh extends _Object {
	String debug();
	TrackRefresh clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory TrackRefresh.default_();
	int? get ratioGet;
	void ratioSet(int? value);
	RustDuration? get intervalGet;
	void intervalSet(RustDuration? value);
	TrackRefresh withRatio(int? ratio);
	TrackRefresh withInterval(RustDuration? interval);
}
final class TrackProducer extends _Object implements Exception {
	TrackProducer clone();
	String name();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void writeFrame(Timestamp timestamp, typed.Uint8List frame);
	TrackFinished finish();
	TrackFinished fail(Error err);
	void finishAt(TrackPosition end);
	void startAt(U64? key);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	void abort(Error err);
	void abortUnwanted(Error err);
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackDemand demand();
	TrackConsumer consume();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	Cancelable<TrackSubscription?> subscriptionChanged();
	TrackRequests requests();
}
final class TrackAbortHandle extends _Object {
	TrackAbortHandle clone();
	bool isClosed();
	void abort(Error err);
}
final class TrackConsumer extends _Object {
	TrackConsumer clone();
	TrackConsumer unmetered();
	String name();
	BroadcastInfo broadcast();
	Cancelable<TrackSubscriber> subscribe(TrackSubscription? subscription);
	Cancelable<GroupConsumer> fetchGroup(U64 key, GroupFetch? options);
	Cancelable<TrackInfo> info();
	Cancelable<TrackPosition?> largest();
}
final class TrackControl extends _Object {
	TrackControl clone();
	TrackSubscription subscription();
	void update(TrackSubscription subscription);
	Cancelable<void> refresh();
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
	TrackSubscription withOrder(TrackOrder order);
}
final class TrackFill extends _Object {
}
final class OriginFollowing extends _Object {
	Cancelable<BroadcastConsumer?> available();
}
final class OriginClaim extends _Object {
	OriginClaim clone();
	Instance instance();
	void announce();
	void withdraw();
	bool isAnnounced();
	void setCost(OriginCost cost);
	void setAuth(AuthCredential? auth);
	Cancelable<OriginRequest?> requested();
}
final class OriginConfig extends _Object {
	String debug();
	OriginConfig clone();
	factory OriginConfig.default_();
	U64 get nodeGet;
	void nodeSet(U64 value);
	U64 get maxHopsGet;
	void maxHopsSet(U64 value);
	CachePool get poolGet;
	void poolSet(CachePool value);
	RustDuration? get cacheDurationGet;
	void cacheDurationSet(RustDuration? value);
	factory OriginConfig(U64 node);
}
final class OriginConsumer extends _Object {
	OriginConsumer clone();
	U64 node();
	OriginConsumer withHidden(bool hidden);
	OriginConsumer covering();
	OriginConsumer exclude(String prefix);
	OriginConsumer withStats(StatsSession session);
	AnnounceConsumer announced();
	OriginRoutes routes();
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
final class OriginProducer extends _Object {
	OriginProducer clone();
	static (OriginProducer, OriginDriver) new_(OriginConfig config);
	OriginProducer withStats(StatsSession session);
	OriginProducer withLink(OriginLink link);
	OriginProducer withPriority(OriginPriority priority);
	OriginProducer withProxy(bool proxy);
	OriginConfig config();
	U64 node();
	bool isOwn(U64 id);
	OriginRoute route(String path, OriginRouteOptions options);
	OriginClaim claim(String prefix, OriginRouteOptions options);
	OriginProducer scope(String root, Patterns patterns);
	OriginProducer exclude(String prefix);
	OriginProducer mount(String at, String target);
	OriginConsumer consume();
	Path root();
	Patterns allowed();
	Path absolute(String path);
}
final class OriginRequest extends _Object {
	Path path();
	OriginRoute route(OriginRouteOptions options);
	void serveProducer(BroadcastProducer broadcast);
	void serveConsumer(BroadcastConsumer broadcast);
	void reject(Error err);
}
final class OriginRoute extends _Object {
	OriginRoute clone();
	BroadcastProducer broadcast();
	Instance instance();
	void announce();
	void withdraw();
	bool isAnnounced();
	void setCost(OriginCost cost);
	void setAuth(AuthCredential? auth);
}
final class OriginRouteInfo extends _Object {
	String debug();
	OriginRouteInfo clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Instance instance();
	OriginCost cost();
	bool available();
}
final class OriginRouteOptions extends _Object {
	String debug();
	OriginRouteOptions clone();
	factory OriginRouteOptions.default_();
	factory OriginRouteOptions.hidden();
	OriginRouteOptions withCost(OriginCost cost);
	OriginRouteOptions withAuth(AuthCredential auth);
	OriginRouteOptions withInstance(Instance instance, U64 epoch);
}
final class OriginRouteEvent extends _Object {
	String debug();
	OriginRouteEvent clone();
	OriginRouteKind get kindGet;
	void kindSet(OriginRouteKind value);
	AnnounceAnnounce get announceGet;
	void announceSet(AnnounceAnnounce value);
}
final class OriginRoutes extends _Object {
	Cancelable<OriginRouteEvent?> next();
	OriginRouteEvent? tryNext();
	bool isClosed();
	Cancelable<void> pollSynced();
	Path root();
	Path absolute(String prefix);
	dasync.Stream<OriginRouteEvent> asStream();
}
final class AnnounceConsumer extends _Object {
	Cancelable<AnnounceEvent?> next();
	AnnounceEvent? tryNext();
	bool isClosed();
	Cancelable<void> pollSynced();
	Path root();
	Path absolute(String prefix);
	dasync.Stream<AnnounceEvent> asStream();
}
final class AnnounceEvent extends _Object {
	String debug();
	AnnounceEvent clone();
}
final class AnnounceAnnounce extends _Object {
	String debug();
	AnnounceAnnounce clone();
	Path get prefixGet;
	void prefixSet(Path value);
	List<Pattern>? get capturesGet;
	void capturesSet(List<Pattern>? value);
	OriginRouteInfo get routeGet;
	void routeSet(OriginRouteInfo value);
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
final class HangCatalogSkipped extends _Object {
	String debug();
	HangCatalogSkipped clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogSkipped.default_();
	List<(String, U64)> get videoGet;
	void videoSet(List<(String, U64)> value);
	List<(String, U64)> get audioGet;
	void audioSet(List<(String, U64)> value);
	List<(String, U64)> get textGet;
	void textSet(List<(String, U64)> value);
	List<(String, U64)> get jsonGet;
	void jsonSet(List<(String, U64)> value);
	List<(String, U64)> get binaryGet;
	void binarySet(List<(String, U64)> value);
	List<(String, U64)> get includeGet;
	void includeSet(List<(String, U64)> value);
	bool isEmpty();
	U64? version();
}
final class HangCatalogConsumer extends _Object {
	factory HangCatalogConsumer(TrackConsumer track, RustDuration delay);
	Cancelable<MoqJsonTimedCatalog?> pollNext();
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
final class HangCatalogClock extends _Object {
	String debug();
	HangCatalogClock clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Timestamp get wallGet;
	void wallSet(Timestamp value);
	factory HangCatalogClock(Timestamp wall);
	ClockWall wallClock(Timestamp pts);
}
final class HangCatalogContainer extends _Object {
	String debug();
	HangCatalogContainer clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogContainer.default_();
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
final class HangCatalogInclude extends _Object {
	String debug();
	HangCatalogInclude clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	PathRelative get broadcastGet;
	void broadcastSet(PathRelative value);
	Instance? get instanceGet;
	void instanceSet(Instance? value);
	factory HangCatalogInclude(PathRelative broadcast);
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
final class HangCatalogTimeline extends _Object {
	String debug();
	HangCatalogTimeline clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	factory HangCatalogTimeline.default_();
	HangCatalogClock? get clockGet;
	void clockSet(HangCatalogClock? value);
	RustDuration? get intervalGet;
	void intervalSet(RustDuration? value);
	bool isEmpty();
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
	HangCatalogColorRange? get rangeGet;
	void rangeSet(HangCatalogColorRange? value);
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
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	void checkRenditions();
	HangCatalog withoutExt();
	String toJson();
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
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	MuxCatalogHangExtra get extGet;
	void extSet(MuxCatalogHangExtra value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	void checkRenditions();
	HangCatalog withoutExt();
	String toJson();
	typed.Uint8List toVec();
	void toWriter(typed.Uint8List writer);
}
final class MuxCatalogGuardExt extends _Object {
	void commit();
	HangCatalogVideo get videoGet;
	void videoSet(HangCatalogVideo value);
	HangCatalogAudio get audioGet;
	void audioSet(HangCatalogAudio value);
	HangCatalogTimeline get syncGet;
	void syncSet(HangCatalogTimeline value);
	HangCatalogText get textGet;
	void textSet(HangCatalogText value);
	HangCatalogJson get jsonGet;
	void jsonSet(HangCatalogJson value);
	HangCatalogBinary get binaryGet;
	void binarySet(HangCatalogBinary value);
	HangCatalogStats? get statsGet;
	void statsSet(HangCatalogStats? value);
	HangCatalogEcho? get echoGet;
	void echoSet(HangCatalogEcho? value);
	List<(String, HangCatalogInclude)> get includeGet;
	void includeSet(List<(String, HangCatalogInclude)> value);
	MuxContainerTsExt get extGet;
	void extSet(MuxContainerTsExt value);
	HangCatalogSkipped get skippedGet;
	void skippedSet(HangCatalogSkipped value);
	void checkRenditions();
	HangCatalog withoutExt();
	String toJson();
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
}
final class MuxContainerConsumerContainer extends _Object {
	factory MuxContainerConsumerContainer(TrackSubscriber track, MuxCatalogHangContainer format);
	ClockInstant? arrival();
	ClockInstant? backlog();
	Cancelable<MuxContainerRead?> read();
	bool isSkipped(RangeU64 groups);
	void setDelay(TrackDelay delay);
	void setPlayhead(MuxSyncConsumer playhead);
}
final class MuxContainerConsumerFmp4Wire extends _Object {
	factory MuxContainerConsumerFmp4Wire(TrackSubscriber track, MuxContainerFmp4Wire format);
	ClockInstant? arrival();
	ClockInstant? backlog();
	Cancelable<MuxContainerRead?> read();
	bool isSkipped(RangeU64 groups);
	void setDelay(TrackDelay delay);
	void setPlayhead(MuxSyncConsumer playhead);
}
final class MuxContainerConsumerLegacyWire extends _Object {
	factory MuxContainerConsumerLegacyWire(TrackSubscriber track, MuxContainerLegacyWire format);
	ClockInstant? arrival();
	ClockInstant? backlog();
	Cancelable<MuxContainerRead?> read();
	bool isSkipped(RangeU64 groups);
	void setDelay(TrackDelay delay);
	void setPlayhead(MuxSyncConsumer playhead);
}
final class MuxContainerConsumerLocWire extends _Object {
	factory MuxContainerConsumerLocWire(TrackSubscriber track, MuxContainerLocWire format);
	ClockInstant? arrival();
	ClockInstant? backlog();
	Cancelable<MuxContainerRead?> read();
	bool isSkipped(RangeU64 groups);
	void setDelay(TrackDelay delay);
	void setPlayhead(MuxSyncConsumer playhead);
}
final class MuxContainerGroupConsumerContainer extends _Object {
	factory MuxContainerGroupConsumerContainer(GroupConsumer group, MuxCatalogHangContainer format);
	U64 start();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerFmp4Wire extends _Object {
	factory MuxContainerGroupConsumerFmp4Wire(GroupConsumer group, MuxContainerFmp4Wire format);
	U64 start();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerLegacyWire extends _Object {
	factory MuxContainerGroupConsumerLegacyWire(GroupConsumer group, MuxContainerLegacyWire format);
	U64 start();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerGroupConsumerLocWire extends _Object {
	factory MuxContainerGroupConsumerLocWire(GroupConsumer group, MuxContainerLocWire format);
	U64 start();
	Cancelable<MuxContainerFrame?> read();
}
final class MuxContainerFinishedContainerAudioConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedContainerBinaryConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedContainerJsonConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedContainerTextConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedContainerVideoConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedContainer extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4WireAudioConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4WireBinaryConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4WireJsonConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4WireTextConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4WireVideoConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedFmp4Wire extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWireAudioConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWireBinaryConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWireJsonConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWireTextConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWireVideoConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLegacyWire extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWireAudioConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWireBinaryConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWireJsonConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWireTextConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWireVideoConfig extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerFinishedLocWire extends _Object {
	String name();
	TrackDemand demand();
	TrackInfo info();
	TrackSubscriber consume();
	MuxCatalogEstimate estimate();
	void abort(Error err);
}
final class MuxContainerGuardUnit extends _Object {
	void commit();
}
final class MuxContainerGuardAudioConfig extends _Object {
	void commit();
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
	bool sameIdentity(HangCatalogAudioConfig other);
}
final class MuxContainerGuardBinaryConfig extends _Object {
	void commit();
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
}
final class MuxContainerGuardJsonConfig extends _Object {
	void commit();
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
}
final class MuxContainerGuardTextConfig extends _Object {
	void commit();
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
}
final class MuxContainerGuardVideoConfig extends _Object {
	void commit();
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
	RustDuration? get delayGet;
	void delaySet(RustDuration? value);
	bool sameIdentity(HangCatalogVideoConfig other);
}
final class MuxContainerProducerContainerAudioConfig extends _Object implements Exception {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardAudioConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainerAudioConfig finish();
	MuxContainerFinishedContainerAudioConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerBinaryConfig extends _Object implements Exception {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardBinaryConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainerBinaryConfig finish();
	MuxContainerFinishedContainerBinaryConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerJsonConfig extends _Object implements Exception {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardJsonConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainerJsonConfig finish();
	MuxContainerFinishedContainerJsonConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerTextConfig extends _Object implements Exception {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardTextConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainerTextConfig finish();
	MuxContainerFinishedContainerTextConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainerVideoConfig extends _Object implements Exception {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardVideoConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainerVideoConfig finish();
	MuxContainerFinishedContainerVideoConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerContainer extends _Object implements Exception {
	factory MuxContainerProducerContainer(TrackProducer track, MuxCatalogHangContainer container);
	void set();
	void setBitrate(U64 bitrate);
	MuxContainerGuardUnit modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedContainer finish();
	MuxContainerFinishedContainer fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireAudioConfig extends _Object implements Exception {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardAudioConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4WireAudioConfig finish();
	MuxContainerFinishedFmp4WireAudioConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireBinaryConfig extends _Object implements Exception {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardBinaryConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4WireBinaryConfig finish();
	MuxContainerFinishedFmp4WireBinaryConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireJsonConfig extends _Object implements Exception {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardJsonConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4WireJsonConfig finish();
	MuxContainerFinishedFmp4WireJsonConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireTextConfig extends _Object implements Exception {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardTextConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4WireTextConfig finish();
	MuxContainerFinishedFmp4WireTextConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4WireVideoConfig extends _Object implements Exception {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardVideoConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4WireVideoConfig finish();
	MuxContainerFinishedFmp4WireVideoConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerFmp4Wire extends _Object implements Exception {
	factory MuxContainerProducerFmp4Wire(TrackProducer track, MuxContainerFmp4Wire container);
	void set();
	void setBitrate(U64 bitrate);
	MuxContainerGuardUnit modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedFmp4Wire finish();
	MuxContainerFinishedFmp4Wire fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireAudioConfig extends _Object implements Exception {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardAudioConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWireAudioConfig finish();
	MuxContainerFinishedLegacyWireAudioConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireBinaryConfig extends _Object implements Exception {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardBinaryConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWireBinaryConfig finish();
	MuxContainerFinishedLegacyWireBinaryConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireJsonConfig extends _Object implements Exception {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardJsonConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWireJsonConfig finish();
	MuxContainerFinishedLegacyWireJsonConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireTextConfig extends _Object implements Exception {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardTextConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWireTextConfig finish();
	MuxContainerFinishedLegacyWireTextConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWireVideoConfig extends _Object implements Exception {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardVideoConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWireVideoConfig finish();
	MuxContainerFinishedLegacyWireVideoConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLegacyWire extends _Object implements Exception {
	factory MuxContainerProducerLegacyWire(TrackProducer track, MuxContainerLegacyWire container);
	void set();
	void setBitrate(U64 bitrate);
	MuxContainerGuardUnit modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLegacyWire finish();
	MuxContainerFinishedLegacyWire fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireAudioConfig extends _Object implements Exception {
	void set(HangCatalogAudioConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardAudioConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWireAudioConfig finish();
	MuxContainerFinishedLocWireAudioConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireBinaryConfig extends _Object implements Exception {
	void set(HangCatalogBinaryConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardBinaryConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWireBinaryConfig finish();
	MuxContainerFinishedLocWireBinaryConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireJsonConfig extends _Object implements Exception {
	void set(HangCatalogJsonConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardJsonConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWireJsonConfig finish();
	MuxContainerFinishedLocWireJsonConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireTextConfig extends _Object implements Exception {
	void set(HangCatalogTextConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardTextConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWireTextConfig finish();
	MuxContainerFinishedLocWireTextConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWireVideoConfig extends _Object implements Exception {
	void set(HangCatalogVideoConfig config);
	void setBitrate(U64 bitrate);
	MuxContainerGuardVideoConfig modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWireVideoConfig finish();
	MuxContainerFinishedLocWireVideoConfig fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxContainerProducerLocWire extends _Object implements Exception {
	factory MuxContainerProducerLocWire(TrackProducer track, MuxContainerLocWire container);
	void set();
	void setBitrate(U64 bitrate);
	MuxContainerGuardUnit modify();
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
	void seek(U64 start);
	void discontinuity();
	MuxContainerFinishedLocWire finish();
	MuxContainerFinishedLocWire fail(Error err);
	void abort(Error err);
	void abortUnwanted(Error err);
	TrackSubscriber consume();
	BroadcastInfo broadcast();
	TrackInfo info();
	GroupProducer createGroup(GroupInfo group);
	GroupProducer appendGroup(Timestamp start);
	void skipGroups(RangeU64 groups);
	TrackPosition? finished();
	Cancelable<void> closed();
	bool isClosed();
	TrackPosition? largest();
	bool isClone(TrackProducer other);
	TrackAbortHandle abortHandle();
	TrackSubscriber subscribe(TrackSubscription? subscription);
	TrackSubscription? subscription();
	TrackRequests requests();
}
final class MuxFollowInstances extends _Object {
	Cancelable<BroadcastConsumer?> next();
	dasync.Stream<BroadcastConsumer> asStream();
}
final class MuxImportContainer extends _Object {
	factory MuxImportContainer(BroadcastProducer broadcast, MuxCatalogReserved reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	MuxImportContainerFinished finish();
	void abort(Error err);
	void cut();
	void seek(U64 start);
}
final class MuxImportContainerExtra extends _Object {
	factory MuxImportContainerExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	MuxImportContainerFinishedExtra finish();
	void abort(Error err);
	void cut();
	void seek(U64 start);
}
final class MuxImportContainerExt extends _Object {
	factory MuxImportContainerExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved, MuxImportContainerInit init);
	void decode(typed.Uint8List data);
	MuxImportContainerFinishedExt finish();
	void abort(Error err);
	void cut();
	void seek(U64 start);
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
final class MuxImportTrack extends _Object implements Exception {
	factory MuxImportTrack.audioExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportAudioInit init);
	factory MuxImportTrack.audioExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportAudioInit init);
	factory MuxImportTrack.audioUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportAudioInit init);
	factory MuxImportTrack.videoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
	factory MuxImportTrack.videoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
	factory MuxImportTrack.videoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
	void decode(typed.Uint8List frame, Timestamp? pts);
	MuxImportTrackFinished finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 start);
	TrackDemand demand();
	String name();
}
final class MuxImportTrackStream extends _Object implements Exception {
	factory MuxImportTrackStream.videoExtra(TrackRequest request, MuxCatalogReservedExtra reserved, MuxImportVideoInit init);
	factory MuxImportTrackStream.videoExt(TrackRequest request, MuxCatalogReservedExt reserved, MuxImportVideoInit init);
	factory MuxImportTrackStream.videoUnit(TrackRequest request, MuxCatalogReserved reserved, MuxImportVideoInit init);
	void initialize(typed.Uint8List data);
	void decode(typed.Uint8List data);
	MuxImportTrackFinished finish();
	void setEnabled(bool enabled);
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void seek(U64 start);
	TrackDemand demand();
	String name();
}
final class MuxJsonSnapshotJson extends _Object implements Exception {
	String name();
	TrackSubscriber consume();
	TrackDemand demand();
	Cancelable<void> refresh();
	void update(Json value, Timestamp timestamp);
	void finish();
	void abort(Error err);
	void abortUnwanted(Error err);
}
final class MuxJsonStreamJson extends _Object implements Exception {
	String name();
	TrackSubscriber consume();
	TrackDemand demand();
	void append(Json value, Timestamp timestamp);
	void finish();
	void abort(Error err);
	void abortUnwanted(Error err);
}
final class MuxJsonConsumerJson extends _Object {
	static Cancelable<MuxJsonConsumerJson> fromTrack(TrackConsumer track, HangCatalogJsonConfig config);
	HangCatalogMode mode();
	Cancelable<MoqJsonTimedJson?> next();
	Cancelable<MoqJsonTimedJson?> latest();
	dasync.Stream<MoqJsonTimedJson> asStream();
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
final class MuxRateGrant extends _Object {
	factory MuxRateGrant(BandwidthConsumer grant, MuxRatePolicy policy);
	void assume(MuxRateTarget target);
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
	void retainExtra(HangCatalogExtra catalog);
	void retainExt(HangCatalogExt catalog);
	void retainUnit(HangCatalog catalog);
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
final class MuxSyncTrack extends _Object {
	bool received(Timestamp timestamp, ClockInstant now);
	void reset();
	bool setTarget(RustDuration target);
	Cancelable<RustDuration> delayChanged(RustDuration seen);
}
final class MuxSyncSchedule extends _Object {
	String debug();
	MuxSyncSchedule clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	RustDuration delay();
	ClockInstant? due(Timestamp timestamp);
}
final class MuxSyncProducer extends _Object {
	MuxSyncProducer clone();
	factory MuxSyncProducer(RustDuration delay);
	MuxSyncProducer reset(RustDuration delay);
	MuxSyncConsumer consume();
	MuxSyncTrack track(MuxContainerKind kind);
	ClockInstant? due(Timestamp timestamp);
}
final class MuxSyncConsumer extends _Object {
	MuxSyncConsumer clone();
	RustDuration delay();
	ClockInstant? due(Timestamp timestamp);
	RustDuration? position(ClockInstant at);
	MuxSyncSchedule schedule();
	Cancelable<MuxSyncSchedule> changed(MuxSyncSchedule seen);
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
	TokioTlsPeerIdentity peer(List<RustlsPkiTypesCertificateDer> chain);
}
final class TokioTlsPeerIdentity extends _Object {
	String debug();
	TokioTlsPeerIdentity clone();
	List<RustlsPkiTypesCertificateDer> chain();
	String? fingerprint();
	String? name();
	List<String> lineage();
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
	String? get implementationGet;
	void implementationSet(String? value);
	TokioTlsConnect get tlsGet;
	void tlsSet(TokioTlsConnect value);
	TokioBackoff get backoffGet;
	void backoffSet(TokioBackoff value);
	MoqTokioConnectionHandover get goawayGet;
	void goawaySet(MoqTokioConnectionHandover value);
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
	Server server();
	bool hasExplicitBind();
	bool servesQuic();
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
	ServerAdmission get admissionGet;
	void admissionSet(ServerAdmission value);
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
	MoqTokioServerStreams get streamsGet;
	void streamsSet(MoqTokioServerStreams value);
	TokioServer init();
	TokioServer initStreams();
}
final class TokioServerListener extends _Object {
	Cancelable<TokioServerRequest> accept();
	Cancelable<void> close_();
	Cancelable<void> closeStreams();
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
	TokioServerRequest withPeerNode(U64 node);
	TokioServerRequest withStats(StatsSession stats);
	TokioServerRequest withLimits(SessionLimits limits);
	MoqTokioServerQuicLimits? quicLimits();
	TokioServerRequest withQuicLimits(MoqQuicLimits limits);
	Cancelable<void> expired();
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
	U64? peerNode();
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
	RustDuration get latencyGet;
	void latencySet(RustDuration value);
}
final class TransportMemorySession extends _Object {
	TransportMemorySession clone();
	TransportMemoryCloser closer();
}
final class TransportMemoryCloser extends _Object {
	TransportMemoryCloser clone();
	void close_(int code, String reason);
	bool isClosed();
}
final class MuxCatalogHangConsumer extends _Object {
	factory MuxCatalogHangConsumer(TrackConsumer track);
	Cancelable<HangCatalog?> next();
	dasync.Stream<HangCatalog> asStream();
}
final class MuxCatalogHangConsumerExtra extends _Object {
	factory MuxCatalogHangConsumerExtra(TrackConsumer track);
	Cancelable<HangCatalogExtra?> next();
	dasync.Stream<HangCatalogExtra> asStream();
}
final class MuxCatalogHangConsumerExt extends _Object {
	factory MuxCatalogHangConsumerExt(TrackConsumer track);
	Cancelable<HangCatalogExt?> next();
	dasync.Stream<HangCatalogExt> asStream();
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
	dasync.Stream<HangCatalog> asStream();
}
final class MuxCatalogMsfConsumerExtra extends _Object {
	factory MuxCatalogMsfConsumerExtra(TrackSubscriber track);
	Cancelable<HangCatalogExtra?> next();
	dasync.Stream<HangCatalogExtra> asStream();
}
final class MuxCatalogMsfConsumerExt extends _Object {
	factory MuxCatalogMsfConsumerExt(TrackSubscriber track);
	Cancelable<HangCatalogExt?> next();
	dasync.Stream<HangCatalogExt> asStream();
}
final class MuxCodecAacConfig extends _Object {
	int get profileGet;
	void profileSet(int value);
	int get sampleRateGet;
	void sampleRateSet(int value);
	int get channelCountGet;
	void channelCountSet(int value);
	factory MuxCodecAacConfig(int profile, int sampleRate, int channelCount);
	factory MuxCodecAacConfig.parse(Mut<typed.Uint8List> buf);
	typed.Uint8List encode();
}
final class MuxCodecAacError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecAacError clone();
}
final class MuxCodecAacImport extends _Object implements Exception {
	factory MuxCodecAacImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecAacImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecAacImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	String name();
	TrackDemand demand();
	MuxContainerFinishedContainerAudioConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 start);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecAnnexbError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxCodecAnnexbError clone();
}
final class MuxCodecAv1Import extends _Object implements Exception {
	factory MuxCodecAv1Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecAv1Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecAv1Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	TrackDemand demand();
	MuxContainerFinishedContainerVideoConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 start);
	void decode(List<MuxContainerFrame> frames);
}
final class MuxCodecAv1Split extends _Object {
	factory MuxCodecAv1Split.default_();
	factory MuxCodecAv1Split();
	List<MuxContainerFrame> decode(typed.Uint8List data, Timestamp? pts);
	List<MuxContainerFrame> flush(Timestamp? pts);
	void reset();
}
final class MuxCodecFlacImport extends _Object implements Exception {
	factory MuxCodecFlacImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecFlacImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecFlacImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	TrackDemand demand();
	MuxContainerFinishedContainerAudioConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 start);
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
final class MuxCodecH264Import extends _Object implements Exception {
	factory MuxCodecH264Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH264Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH264Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	String name();
	TrackDemand demand();
	TrackProducer track();
	MuxContainerFinishedContainerVideoConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	void seek(U64 start);
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
final class MuxCodecH265Sps extends _Object {
	String debug();
	MuxCodecH265Sps clone();
	factory MuxCodecH265Sps.parse(typed.Uint8List nal);
	(int, int) size();
	MuxCodecH265VideoSignal? videoSignal();
}
final class MuxCodecH265Import extends _Object implements Exception {
	factory MuxCodecH265Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH265Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecH265Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	String name();
	TrackDemand demand();
	TrackProducer track();
	MuxContainerFinishedContainerVideoConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void reportSkipped(U64 frames);
	void reportBitrate(BandwidthRate bitrate);
	void seek(U64 start);
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
final class MuxCodecMp3Import extends _Object implements Exception {
	factory MuxCodecMp3Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecMp3Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecMp3Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	TrackDemand demand();
	MuxContainerFinishedContainerAudioConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 start);
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
final class MuxCodecOpusImport extends _Object implements Exception {
	factory MuxCodecOpusImport.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, HangCatalogAudioConfig config);
	factory MuxCodecOpusImport.newExt(TrackProducer track, MuxCatalogReservedExt reserved, HangCatalogAudioConfig config);
	factory MuxCodecOpusImport.newUnit(TrackProducer track, MuxCatalogReserved reserved, HangCatalogAudioConfig config);
	String name();
	TrackDemand demand();
	MuxContainerFinishedContainerAudioConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void seek(U64 start);
	void decode(typed.Uint8List frame, Timestamp? pts);
}
final class MuxCodecVp8Import extends _Object implements Exception {
	factory MuxCodecVp8Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp8Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp8Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	void decode(typed.Uint8List frame, Timestamp? pts);
	TrackDemand demand();
	MuxContainerFinishedContainerVideoConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 start);
}
final class MuxCodecVp9Import extends _Object implements Exception {
	factory MuxCodecVp9Import.newExtra(TrackProducer track, MuxCatalogReservedExtra reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp9Import.newExt(TrackProducer track, MuxCatalogReservedExt reserved, MuxCatalogVideoHint hint);
	factory MuxCodecVp9Import.newUnit(TrackProducer track, MuxCatalogReserved reserved, MuxCatalogVideoHint hint);
	void initialize(typed.Uint8List buf);
	void decode(typed.Uint8List frame, Timestamp? pts);
	TrackDemand demand();
	MuxContainerFinishedContainerVideoConfig finish();
	void abort(Error err);
	void abortUnwanted(Error err);
	void cut(Timestamp? end);
	void discontinuity();
	void setEnabled(bool enabled);
	void seek(U64 start);
}
final class MuxContainerFlvImport extends _Object {
	factory MuxContainerFlvImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerFlvImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerFlvFinished finish();
	void abort(Error err);
}
final class MuxContainerFlvImportExtra extends _Object {
	factory MuxContainerFlvImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerFlvImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerFlvFinishedExtra finish();
	void abort(Error err);
}
final class MuxContainerFlvImportExt extends _Object {
	factory MuxContainerFlvImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerFlvImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerFlvFinishedExt finish();
	void abort(Error err);
}
final class MuxContainerFlvFinished extends _Object {
	void abort(Error err);
}
final class MuxContainerFlvFinishedExtra extends _Object {
	void abort(Error err);
}
final class MuxContainerFlvFinishedExt extends _Object {
	void abort(Error err);
}
final class MuxContainerFmp4Wire extends _Object {
	factory MuxContainerFmp4Wire.fromInit(typed.Uint8List initData);
}
final class MuxContainerFmp4Error extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxContainerFmp4Error clone();
}
final class MuxContainerFmp4Import extends _Object {
	factory MuxContainerFmp4Import(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	void cut();
	MuxContainerFmp4Import withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4Import withCmaf(bool cmaf);
	void decode(typed.Uint8List data);
	MuxContainerFmp4Finished finish();
	void abort(Error err);
	void seek(U64 start);
	void rejoin();
	void discontinuity();
}
final class MuxContainerFmp4ImportExtra extends _Object {
	factory MuxContainerFmp4ImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	void cut();
	MuxContainerFmp4ImportExtra withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4ImportExtra withCmaf(bool cmaf);
	void decode(typed.Uint8List data);
	MuxContainerFmp4FinishedExtra finish();
	void abort(Error err);
	void seek(U64 start);
	void rejoin();
	void discontinuity();
}
final class MuxContainerFmp4ImportExt extends _Object {
	factory MuxContainerFmp4ImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	void cut();
	MuxContainerFmp4ImportExt withSelect(MuxSelectBroadcast select);
	MuxContainerFmp4ImportExt withCmaf(bool cmaf);
	void decode(typed.Uint8List data);
	MuxContainerFmp4FinishedExt finish();
	void abort(Error err);
	void seek(U64 start);
	void rejoin();
	void discontinuity();
}
final class MuxContainerFmp4Finished extends _Object {
	void abort(Error err);
}
final class MuxContainerFmp4FinishedExtra extends _Object {
	void abort(Error err);
}
final class MuxContainerFmp4FinishedExt extends _Object {
	void abort(Error err);
}
final class MuxContainerFmp4Muxer extends _Object {
	factory MuxContainerFmp4Muxer.video(HangCatalogVideoConfig config);
	factory MuxContainerFmp4Muxer.audio(HangCatalogAudioConfig config);
	Timescale timescale();
	MuxContainerFmp4Muxer withTimescale(Timescale timescale);
	Cancelable<List<MuxContainerFrame>> read(GroupConsumer group);
	(int, int)? pictureSize(MuxContainerFrame keyframe);
	typed.Uint8List? init();
	typed.Uint8List fragment(int sequence, List<MuxContainerFrame> frames, Timestamp? end);
}
final class MuxContainerLegacyAudio extends _Object {
	String debug();
	MuxContainerLegacyAudio clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static MuxContainerLegacyAudio? new_(HangCatalogAudioConfig config);
	Timestamp? duration(typed.Uint8List payload);
}
final class MuxContainerMkvImport extends _Object {
	factory MuxContainerMkvImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerMkvImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerMkvFinished finish();
	void abort(Error err);
}
final class MuxContainerMkvImportExtra extends _Object {
	factory MuxContainerMkvImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerMkvImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerMkvFinishedExtra finish();
	void abort(Error err);
}
final class MuxContainerMkvImportExt extends _Object {
	factory MuxContainerMkvImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerMkvImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 start);
	MuxContainerMkvFinishedExt finish();
	void abort(Error err);
}
final class MuxContainerMkvFinished extends _Object {
	void abort(Error err);
}
final class MuxContainerMkvFinishedExtra extends _Object {
	void abort(Error err);
}
final class MuxContainerMkvFinishedExt extends _Object {
	void abort(Error err);
}
final class MuxContainerMp4Finish extends _Object {
	String debug();
	MuxContainerMp4Finish clone();
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
	MuxContainerTsPassthroughTrack? get passthroughGet;
	void passthroughSet(MuxContainerTsPassthroughTrack? value);
	bool isEmpty();
}
final class MuxContainerTsPassthroughTrack extends _Object {
	String debug();
	MuxContainerTsPassthroughTrack clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	String get trackGet;
	void trackSet(String value);
	bool get randomAccessGet;
	void randomAccessSet(bool value);
	factory MuxContainerTsPassthroughTrack(String track);
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
final class MuxContainerTsError extends _Object implements Exception {
	@override String toString();
	String debug();
	MuxContainerTsError clone();
}
final class MuxContainerTsExportConsumerExtra extends _Object {
	factory MuxContainerTsExportConsumerExtra(MuxSource source, MuxCatalogConsumerExtra catalog, RustDuration delay);
	MuxContainerTsExportConsumerExtra withMuxRate(U64 muxRate);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumerExtra follow(BroadcastConsumer broadcast, MuxCatalogConsumerExtra catalog);
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsExportConsumerExt extends _Object {
	factory MuxContainerTsExportConsumerExt(MuxSource source, MuxCatalogConsumerExt catalog, RustDuration delay);
	MuxContainerTsExportConsumerExt withMuxRate(U64 muxRate);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumerExt follow(BroadcastConsumer broadcast, MuxCatalogConsumerExt catalog);
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsExportConsumer extends _Object {
	factory MuxContainerTsExportConsumer(MuxSource source, MuxCatalogConsumer catalog, RustDuration delay);
	MuxContainerTsExportConsumer withMuxRate(U64 muxRate);
	Cancelable<MuxContainerFrame?> next();
	MuxContainerTsStatsExport stats();
	U64 discontinuity();
	MuxContainerTsExportConsumer follow(BroadcastConsumer broadcast, MuxCatalogConsumer catalog);
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsFollower extends _Object {
	factory MuxContainerTsFollower(MuxContainerTsExportConsumer export);
	MuxContainerTsFollower withFollow(RustDuration follow);
	MuxContainerTsFollower withStitch(bool stitch);
	MuxContainerTsExportConsumer export();
	Cancelable<MuxContainerFrame?> next();
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsFollowerExtra extends _Object {
	factory MuxContainerTsFollowerExtra(MuxContainerTsExportConsumerExtra export);
	MuxContainerTsFollowerExtra withFollow(RustDuration follow);
	MuxContainerTsFollowerExtra withStitch(bool stitch);
	MuxContainerTsExportConsumerExtra export();
	Cancelable<MuxContainerFrame?> next();
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsFollowerExt extends _Object {
	factory MuxContainerTsFollowerExt(MuxContainerTsExportConsumerExt export);
	MuxContainerTsFollowerExt withFollow(RustDuration follow);
	MuxContainerTsFollowerExt withStitch(bool stitch);
	MuxContainerTsExportConsumerExt export();
	Cancelable<MuxContainerFrame?> next();
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsPassthrough extends _Object {
	factory MuxContainerTsPassthrough(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerTsPassthrough withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	MuxContainerTsPassthroughFinished finish();
	void abort(Error err);
}
final class MuxContainerTsPassthroughUnit extends _Object {
	factory MuxContainerTsPassthroughUnit(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerTsPassthroughUnit withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	MuxContainerTsPassthroughFinishedUnit finish();
	void abort(Error err);
}
final class MuxContainerTsPassthroughExtra extends _Object {
	factory MuxContainerTsPassthroughExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerTsPassthroughExtra withPcrPid(int pid);
	String track();
	void decode(typed.Uint8List data);
	MuxContainerTsPassthroughFinishedExtra finish();
	void abort(Error err);
}
final class MuxContainerTsPrograms extends _Object {
	factory MuxContainerTsPrograms(OriginProducer origin, Path name, MuxCatalogConfig config);
	void decode(typed.Uint8List data);
	MuxContainerTsStatsSnapshot stats();
	MuxContainerTsProgramsFinished finish();
	void abort(Error err);
}
final class MuxContainerTsRelease extends _Object {
	static Cancelable<MuxContainerTsRelease> new_(MuxSource source, MuxCatalogCatalogFormat catalogFormat, RustDuration delay);
	U64 dropped();
	double? drift();
	U64 outOfTolerance();
	MuxContainerTsStatsRelease stats();
	Cancelable<MuxContainerFrame?> next();
	dasync.Stream<MuxContainerFrame> asStream();
}
final class MuxContainerTsRepublish extends _Object {
	factory MuxContainerTsRepublish(OriginProducer origin, Path name, Instance instance, MuxCatalogConfig config);
	MuxContainerTsRepublish withProgram(int program);
	BroadcastProducer broadcast();
	MuxCatalogProducerExt catalog();
	Instance instance();
	MuxContainerTsRepublished? decode(typed.Uint8List data);
	MuxContainerTsStatsSnapshot stats();
	MuxContainerTsRepublishFinished finish();
	void abort(Error err);
}
final class MuxContainerTsRepublished extends _Object {
	String debug();
	MuxContainerTsRepublished clone();
	MuxContainerTsStep get stepGet;
	void stepSet(MuxContainerTsStep value);
	MuxContainerTsStatsSnapshot get statsGet;
	void statsSet(MuxContainerTsStatsSnapshot value);
}
final class MuxContainerTsImport extends _Object {
	factory MuxContainerTsImport(BroadcastProducer broadcast, MuxCatalogReserved reserved);
	MuxContainerTsImport withProgram(int program);
	MuxContainerTsImport withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	MuxContainerTsFinished finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsImportExtra extends _Object {
	factory MuxContainerTsImportExtra(BroadcastProducer broadcast, MuxCatalogReservedExtra reserved);
	MuxContainerTsImportExtra withProgram(int program);
	MuxContainerTsImportExtra withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	MuxContainerTsFinishedExtra finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsImportExt extends _Object {
	factory MuxContainerTsImportExt(BroadcastProducer broadcast, MuxCatalogReservedExt reserved);
	MuxContainerTsImportExt withProgram(int program);
	MuxContainerTsImportExt withContainer(HangCatalogContainer container);
	void decode(typed.Uint8List data);
	void seek(U64 sequence);
	MuxContainerTsFinishedExt finish();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsFinished extends _Object {
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsFinishedExtra extends _Object {
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsFinishedExt extends _Object {
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxImportContainerFinished extends _Object {
	void abort(Error err);
}
final class MuxImportContainerFinishedExtra extends _Object {
	void abort(Error err);
}
final class MuxImportContainerFinishedExt extends _Object {
	void abort(Error err);
}
final class MuxImportTrackFinished extends _Object {
	String name();
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
final class MuxContainerTsPassthroughFinished extends _Object {
	String track();
	void abort(Error err);
}
final class MuxContainerTsPassthroughFinishedUnit extends _Object {
	String track();
	void abort(Error err);
}
final class MuxContainerTsPassthroughFinishedExtra extends _Object {
	String track();
	void abort(Error err);
}
final class MuxContainerTsProgramsFinished extends _Object {
	List<BroadcastProducer> broadcasts();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
}
final class MuxContainerTsRepublishFinished extends _Object {
	BroadcastProducer broadcast();
	MuxContainerTsStatsSnapshot stats();
	void abort(Error err);
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
final class MoqTokioCliBound extends _Object {
	@override String toString();
	String debug();
	MoqTokioCliBound clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	static MoqTokioCliBound get NONE;
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
final class MoqJsonTimedCatalog extends _Object {
	String debug();
	MoqJsonTimedCatalog clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	HangCatalog get valueGet;
	void valueSet(HangCatalog value);
	Timestamp get atGet;
	void atSet(Timestamp value);
	factory MoqJsonTimedCatalog(HangCatalog value, Timestamp at);
}
final class MoqJsonTimedJson extends _Object {
	String debug();
	MoqJsonTimedJson clone();
	@override bool operator ==(Object other);
	@override int get hashCode;
	Json get valueGet;
	void valueSet(Json value);
	Timestamp get atGet;
	void atSet(Timestamp value);
	factory MoqJsonTimedJson(Json value, Timestamp at);
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
	RustDuration? get timeoutGet;
	void timeoutSet(RustDuration? value);
}
final class MoqTokioConnectionHandover extends _Object {
	String debug();
	MoqTokioConnectionHandover clone();
	factory MoqTokioConnectionHandover.default_();
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
	U64 get connectsGet;
	void connectsSet(U64 value);
	ClockInstant? get lastConnectedGet;
	void lastConnectedSet(ClockInstant? value);
	MoqTokioConnectionPhase get phaseGet;
	void phaseSet(MoqTokioConnectionPhase value);
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
	RustDuration? get idleTimeoutGet;
	void idleTimeoutSet(RustDuration? value);
	RustDuration? get keepAliveGet;
	void keepAliveSet(RustDuration? value);
	bool get mtuDiscoveryGet;
	void mtuDiscoverySet(bool value);
	MoqTokioQuicCongestionControl get congestionControlGet;
	void congestionControlSet(MoqTokioQuicCongestionControl value);
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
	RustDuration keepAliveInterval();
	MoqQuicLimits limits();
}
final class MoqTokioWebsocketListener extends _Object {
	static Cancelable<MoqTokioWebsocketListener> bind(SocketAddr addr);
	MoqTokioWebsocketListener withProtocols(List<String> protocols);
	MoqTokioWebsocketListener withQuic(MoqTokioQuicResolved quic);
	SocketAddr localAddr();
	MoqTokioAcceptHealth acceptHealth();
	Cancelable<QmuxSession> accept();
}
final class MoqTokioQuicDemuxDemux extends _Object {
	String debug();
	MoqTokioQuicDemuxDemux clone();
	factory MoqTokioQuicDemuxDemux.default_();
	MoqSockDemuxCounters counters();
	MoqTokioQuicDemuxWebRtc webrtc();
}
final class MoqTokioServerStreams extends _Object {
	String debug();
	factory MoqTokioServerStreams.default_();
}
final class MoqTokioServerQuicLimits extends _Object {
	String debug();
	MoqTokioServerQuicLimits clone();
	void set(MoqQuicLimits limits);
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
final class MoqTokioConnectionPhase extends _Object {
	String debug();
	MoqTokioConnectionPhase clone();
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
}
final class RustlsClientDangerDangerousClientConfig extends _Object {
	String debug();
}
final class RustlsClientResumption extends _Object {
	String debug();
	RustlsClientResumption clone();
	factory RustlsClientResumption.default_();
	factory RustlsClientResumption.inMemorySessions(U64 num_);
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
final class QmuxSession extends _Object {
	String debug();
	QmuxSession clone();
	String? protocol();
	MoqQuicConnectionMuxVersion version();
	void setLimits(MoqQuicLimits limits);
	List<RustlsPkiTypesCertificateDer>? peerCertificates();
	String? serverName();
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
const HANG_CATALOG_MAX_SAFE_INTEGER = U64.fromBits(9007199254740991);
String get IMPLEMENTATION;
List<String> get ALPNS;
ClockInstant clockNow();
ClockWall clockWall();
(ClockInstant, ClockWall) clockOrigin();
ClockRng clockRng();
U64 clockSeed();
void clockHostTime();
Cancelable<String> clockDrive(ClockInstant? Function(ClockInstant, Waiter) poll);
ClockInterval clockInterval(RustDuration period);
ClockInterval clockIntervalAt(ClockInstant start, RustDuration period);
Cancelable<void> clockSleep(RustDuration duration);
Cancelable<void> clockSleepUntil(ClockInstant at);
Cancelable<void> clockTimeout(RustDuration duration, Future<void> future);
Cancelable<void> clockTimeoutAt(ClockInstant deadline, Future<void> future);
RustDuration get SERVER_DEFAULT_TIMEOUT;
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
RustDuration get MUX_CONTAINER_DELAY_MAX;
RustDuration get MUX_RATE_RETRY;
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
