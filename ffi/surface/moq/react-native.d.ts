// AnnounceAnnounce.ts
import * as rt from "./rt.ts";
import type { OriginRouteInfo } from "./OriginRouteInfo.ts";
import type { Path } from "./Path.ts";
import type { Pattern } from "./Pattern.ts";
export type AnnounceAnnounce = rt.Handle<"AnnounceAnnounce">;
export declare function debug(self: AnnounceAnnounce): string;
export declare function clone(self: AnnounceAnnounce): AnnounceAnnounce;
export declare function prefixGet(self: AnnounceAnnounce): Path;
export declare function prefixSet(self: AnnounceAnnounce, value: Path): void;
export declare function capturesGet(self: AnnounceAnnounce): Pattern[] | undefined;
export declare function capturesSet(self: AnnounceAnnounce, value: Pattern[] | undefined): void;
export declare function routeGet(self: AnnounceAnnounce): OriginRouteInfo;
export declare function routeSet(self: AnnounceAnnounce, value: OriginRouteInfo): void;

// AnnounceConsumer.ts
import * as rt from "./rt.ts";
import type { AnnounceEvent } from "./AnnounceEvent.ts";
import type { Error_ } from "./Error_.ts";
import type { Path } from "./Path.ts";
export type AnnounceConsumer = rt.Handle<"AnnounceConsumer">;
export declare function next(self: AnnounceConsumer): rt.Call<AnnounceEvent | undefined>;
export declare function tryNext(self: AnnounceConsumer): AnnounceEvent | undefined;
export declare function isClosed(self: AnnounceConsumer): boolean;
export declare function pollSynced(self: AnnounceConsumer): rt.Call<void>;
export declare function root(self: AnnounceConsumer): Path;
export declare function absolute(self: AnnounceConsumer, prefix: string): rt.Result<Path, Error_>;
export declare function values(self: AnnounceConsumer): rt.Stream<AnnounceEvent>;

// AnnounceEvent.ts
import * as rt from "./rt.ts";
export type AnnounceEvent = rt.Handle<"AnnounceEvent">;
export declare function debug(self: AnnounceEvent): string;
export declare function clone(self: AnnounceEvent): AnnounceEvent;

// AuthCredential.ts
import * as rt from "./rt.ts";
import type { AuthWatch } from "./AuthWatch.ts";
import type { Error_ } from "./Error_.ts";
export type AuthCredential = rt.Handle<"AuthCredential">;
export declare function debug(self: AuthCredential): string;
export declare function clone(self: AuthCredential): AuthCredential;
export declare function eq(a: AuthCredential, b: AuthCredential): boolean;
export declare function grant(self: AuthCredential): AuthWatch;
export declare function closed(self: AuthCredential): rt.Call<Error_>;
export declare function answered(self: AuthCredential): rt.Call<rt.Result<undefined, Error_>>;

// AuthGrant.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Patterns } from "./Patterns.ts";
export type AuthGrant = rt.Handle<"AuthGrant">;
export declare function debug(self: AuthGrant): string;
export declare function clone(self: AuthGrant): AuthGrant;
export declare function eq(a: AuthGrant, b: AuthGrant): boolean;
export declare function default$(): AuthGrant;
export declare function publishGet(self: AuthGrant): Patterns;
export declare function publishSet(self: AuthGrant, value: Patterns): void;
export declare function subscribeGet(self: AuthGrant): Patterns;
export declare function subscribeSet(self: AuthGrant, value: Patterns): void;
export declare function expiresGet(self: AuthGrant): ClockInstant | undefined;
export declare function expiresSet(self: AuthGrant, value: ClockInstant | undefined): void;
export declare function all(): AuthGrant;
export { default$ as default };

// AuthHandle.ts
import * as rt from "./rt.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { AuthWatch } from "./AuthWatch.ts";
import type { Error_ } from "./Error_.ts";
export type AuthHandle = rt.Handle<"AuthHandle">;
export declare function clone(self: AuthHandle): AuthHandle;
export declare function default$(): AuthHandle;
export declare function new$(): AuthHandle;
export declare function grant(self: AuthHandle): AuthWatch;
export declare function add(self: AuthHandle, token: Uint8Array | rt.Bytes): rt.Result<AuthCredential, Error_>;
export declare function authorize(self: AuthHandle, grant: AuthGrant): void;
export declare function abort(self: AuthHandle, err: Error_): void;
export { default$ as default, new$ as new };

// AuthIssued.ts
import * as rt from "./rt.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { Error_ } from "./Error_.ts";
import type { SessionError } from "./SessionError.ts";
export type AuthIssued = rt.Handle<"AuthIssued">;
export declare function update(self: AuthIssued, grant: AuthGrant): void;
export declare function revoke(self: AuthIssued, code: SessionError, reason: string): void;
export declare function closed(self: AuthIssued): rt.Call<Error_>;

// AuthRequest.ts
import * as rt from "./rt.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { AuthIssued } from "./AuthIssued.ts";
import type { Error_ } from "./Error_.ts";
import type { SessionError } from "./SessionError.ts";
export type AuthRequest = rt.Handle<"AuthRequest">;
export declare function token(self: AuthRequest): rt.Bytes;
export declare function closed(self: AuthRequest): rt.Call<Error_>;
export declare function accept(self: AuthRequest, grant: AuthGrant): AuthIssued;
export declare function reject(self: AuthRequest, code: SessionError, reason: string): void;

// AuthRequests.ts
import * as rt from "./rt.ts";
import type { AuthRequest } from "./AuthRequest.ts";
export type AuthRequests = rt.Handle<"AuthRequests">;
export declare function next(self: AuthRequests): rt.Call<AuthRequest | undefined>;
export declare function values(self: AuthRequests): rt.Stream<AuthRequest>;

// AuthToken.ts
import * as rt from "./rt.ts";
export type AuthToken = rt.Handle<"AuthToken">;
export declare function debug(self: AuthToken): string;
export declare function clone(self: AuthToken): AuthToken;
export declare function eq(a: AuthToken, b: AuthToken): boolean;
export declare function asStr(self: AuthToken): string;

// AuthTokens.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { AuthToken } from "./AuthToken.ts";
import type { Error_ } from "./Error_.ts";
export type AuthTokens = rt.Handle<"AuthTokens">;
export declare function clone(self: AuthTokens): AuthTokens;
export declare function new$(tokens: AuthToken[]): AuthTokens;
export declare function handle(self: AuthTokens): AuthHandle;
export declare function set(self: AuthTokens, tokens: AuthToken[]): void;
export declare function answered(self: AuthTokens): rt.Call<rt.Result<undefined, Error_>>;
export { new$ as new };

// AuthWatch.ts
import * as rt from "./rt.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { Error_ } from "./Error_.ts";
export type AuthWatch = rt.Handle<"AuthWatch">;
export declare function peek(self: AuthWatch): AuthGrant | undefined;
export declare function changed(self: AuthWatch): rt.Call<rt.Result<AuthGrant | undefined, Error_>>;

// BandwidthAllocator.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BandwidthReservation } from "./BandwidthReservation.ts";
import type { TrackDemand } from "./TrackDemand.ts";
export type BandwidthAllocator = rt.Handle<"BandwidthAllocator">;
export declare function debug(self: BandwidthAllocator): string;
export declare function clone(self: BandwidthAllocator): BandwidthAllocator;
export declare function default$(): BandwidthAllocator;
export declare function new$(estimate: BandwidthConsumer): BandwidthAllocator;
export declare function setTarget(self: BandwidthAllocator, target: BandwidthRate | undefined): void;
export declare function unlimited(): BandwidthAllocator;
export declare function reserve(self: BandwidthAllocator, track: TrackDemand, max: BandwidthRate): BandwidthReservation;
export { default$ as default, new$ as new };

// BandwidthCap.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
export type BandwidthCap = rt.Handle<"BandwidthCap">;
export declare function debug(self: BandwidthCap): string;
export declare function clone(self: BandwidthCap): BandwidthCap;
export declare function eq(a: BandwidthCap, b: BandwidthCap): boolean;
export declare function rateGet(self: BandwidthCap): BandwidthRate;
export declare function rateSet(self: BandwidthCap, value: BandwidthRate): void;
export declare function burstGet(self: BandwidthCap): rt.Duration;
export declare function burstSet(self: BandwidthCap, value: rt.Duration): void;
export declare function DEFAULT_BURST(): rt.Duration;
export declare function new$(rate: BandwidthRate): BandwidthCap;
export declare function withBurst(self: BandwidthCap, burst: rt.Duration): BandwidthCap;
export { new$ as new };

// BandwidthConsumer.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { Error_ } from "./Error_.ts";
export type BandwidthConsumer = rt.Handle<"BandwidthConsumer">;
export declare function clone(self: BandwidthConsumer): BandwidthConsumer;
export declare function peek(self: BandwidthConsumer): BandwidthRate | undefined;
export declare function probed(self: BandwidthConsumer): boolean;
export declare function changed(self: BandwidthConsumer): rt.Call<rt.Result<BandwidthRate | undefined, Error_>>;

// BandwidthDemand.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type BandwidthDemand = rt.Handle<"BandwidthDemand">;
export declare function clone(self: BandwidthDemand): BandwidthDemand;
export declare function wanted(self: BandwidthDemand): boolean;
export declare function wantedChanged(self: BandwidthDemand): rt.Call<rt.Result<boolean, Error_>>;

// BandwidthLimit.ts
import * as rt from "./rt.ts";
import type { BandwidthCap } from "./BandwidthCap.ts";
export type BandwidthLimit = rt.Handle<"BandwidthLimit">;
export declare function debug(self: BandwidthLimit): string;
export declare function clone(self: BandwidthLimit): BandwidthLimit;
export declare function default$(): BandwidthLimit;
export declare function new$(cap: BandwidthCap | undefined): BandwidthLimit;
export declare function set(self: BandwidthLimit, cap: BandwidthCap | undefined): void;
export declare function get(self: BandwidthLimit): BandwidthCap | undefined;
export declare function changed(self: BandwidthLimit): rt.Call<BandwidthCap | undefined>;
export { default$ as default, new$ as new };

// BandwidthProducer.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthDemand } from "./BandwidthDemand.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BandwidthWeak } from "./BandwidthWeak.ts";
import type { Error_ } from "./Error_.ts";
export type BandwidthProducer = rt.Handle<"BandwidthProducer">;
export declare function clone(self: BandwidthProducer): BandwidthProducer;
export declare function default$(): BandwidthProducer;
export declare function new$(): BandwidthProducer;
export declare function set(self: BandwidthProducer, bitrate: BandwidthRate | undefined): rt.Result<undefined, Error_>;
export declare function consume(self: BandwidthProducer): BandwidthConsumer;
export declare function weak(self: BandwidthProducer): BandwidthWeak;
export declare function demand(self: BandwidthProducer): BandwidthDemand;
export { default$ as default, new$ as new };

// BandwidthRate.ts
import * as rt from "./rt.ts";
export type BandwidthRate = rt.Opaque<"BandwidthRate">;
export declare function debug(self: BandwidthRate): string;
export declare function eq(a: BandwidthRate, b: BandwidthRate): boolean;
export declare function hash(self: BandwidthRate): rt.u64;
export declare function partialCmp(a: BandwidthRate, b: BandwidthRate): number | undefined;
export declare function cmp(a: BandwidthRate, b: BandwidthRate): number;
export declare function default$(): BandwidthRate;
export declare function ZERO(): BandwidthRate;
export declare function fromBps(bps: rt.u64): BandwidthRate;
export declare function fromKbps(kbps: rt.u64): BandwidthRate;
export declare function fromMbps(mbps: rt.u64): BandwidthRate;
export declare function asBps(self: BandwidthRate): rt.u64;
export declare function scaled(self: BandwidthRate, factor: rt.f64): BandwidthRate;
export declare function absDiff(self: BandwidthRate, other: BandwidthRate): BandwidthRate;
export { default$ as default };

// BandwidthReservation.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
export type BandwidthReservation = rt.Handle<"BandwidthReservation">;
export declare function rate(self: BandwidthReservation): BandwidthRate | undefined;
export declare function consume(self: BandwidthReservation): BandwidthConsumer;
export declare function update(self: BandwidthReservation, max: BandwidthRate): void;

// BandwidthViewer.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { Error_ } from "./Error_.ts";
export type BandwidthViewer = rt.Handle<"BandwidthViewer">;
export declare function rate(self: BandwidthViewer): BandwidthRate | undefined;
export declare function consume(self: BandwidthViewer): BandwidthConsumer;
export declare function update(self: BandwidthViewer, max: BandwidthRate): rt.Result<undefined, Error_>;

// BandwidthViewers.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BandwidthViewer } from "./BandwidthViewer.ts";
import type { BandwidthWeak } from "./BandwidthWeak.ts";
import type { Error_ } from "./Error_.ts";
export type BandwidthViewers = rt.Handle<"BandwidthViewers">;
export declare function debug(self: BandwidthViewers): string;
export declare function clone(self: BandwidthViewers): BandwidthViewers;
export declare function default$(): BandwidthViewers;
export declare function MAX_TARGET(): BandwidthRate;
export declare function new$(estimate: BandwidthWeak): BandwidthViewers;
export declare function watch(self: BandwidthViewers, max: BandwidthRate): rt.Result<BandwidthViewer, Error_>;
export declare function setTarget(self: BandwidthViewers, target: BandwidthRate | undefined): rt.Result<undefined, Error_>;
export { default$ as default, new$ as new };

// BandwidthWeak.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
export type BandwidthWeak = rt.Handle<"BandwidthWeak">;
export declare function clone(self: BandwidthWeak): BandwidthWeak;
export declare function consume(self: BandwidthWeak): BandwidthConsumer;

// BroadcastConsumer.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { BroadcastTracks } from "./BroadcastTracks.ts";
import type { DatagramConsumer } from "./DatagramConsumer.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type BroadcastConsumer = rt.Handle<"BroadcastConsumer">;
export declare function clone(self: BroadcastConsumer): BroadcastConsumer;
export declare function tracks(self: BroadcastConsumer): rt.Result<BroadcastTracks, Error_>;
export declare function info(self: BroadcastConsumer): BroadcastInfo;
export declare function track(self: BroadcastConsumer, name: string): rt.Result<TrackConsumer, Error_>;
export declare function datagrams(self: BroadcastConsumer, name: string): rt.Result<DatagramConsumer, Error_>;
export declare function isClosed(self: BroadcastConsumer): boolean;
export declare function closed(self: BroadcastConsumer): rt.Call<void>;
export declare function isClone(self: BroadcastConsumer, other: BroadcastConsumer): boolean;

// BroadcastDatagramRequests.ts
import * as rt from "./rt.ts";
import type { DatagramRequest } from "./DatagramRequest.ts";
import type { Error_ } from "./Error_.ts";
export type BroadcastDatagramRequests = rt.Handle<"BroadcastDatagramRequests">;
export declare function clone(self: BroadcastDatagramRequests): BroadcastDatagramRequests;
export declare function requested(self: BroadcastDatagramRequests): rt.Call<rt.Result<DatagramRequest, Error_>>;

// BroadcastDemand.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type BroadcastDemand = rt.Handle<"BroadcastDemand">;
export declare function clone(self: BroadcastDemand): BroadcastDemand;
export declare function wanted(self: BroadcastDemand): boolean;
export declare function wantedChanged(self: BroadcastDemand): rt.Call<rt.Result<boolean, Error_>>;

// BroadcastInfo.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { CachePool } from "./CachePool.ts";
import type { Instance } from "./Instance.ts";
import type { Path } from "./Path.ts";
export type BroadcastInfo = rt.Handle<"BroadcastInfo">;
export declare function debug(self: BroadcastInfo): string;
export declare function clone(self: BroadcastInfo): BroadcastInfo;
export declare function default$(): BroadcastInfo;
export declare function poolGet(self: BroadcastInfo): CachePool;
export declare function poolSet(self: BroadcastInfo, value: CachePool): void;
export declare function cacheDurationGet(self: BroadcastInfo): rt.Duration | undefined;
export declare function cacheDurationSet(self: BroadcastInfo, value: rt.Duration | undefined): void;
export declare function pathGet(self: BroadcastInfo): Path;
export declare function pathSet(self: BroadcastInfo, value: Path): void;
export declare function instanceGet(self: BroadcastInfo): Instance | undefined;
export declare function instanceSet(self: BroadcastInfo, value: Instance | undefined): void;
export declare function new$(): BroadcastInfo;
export declare function produce(self: BroadcastInfo): BroadcastProducer;
export { default$ as default, new$ as new };

// BroadcastKind.ts
export type BroadcastKind = "Track" | "Datagrams";
export declare function debug(self: BroadcastKind): string;

// BroadcastLister.ts
import * as rt from "./rt.ts";
import type { BroadcastKind } from "./BroadcastKind.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackInfo } from "./TrackInfo.ts";
export type BroadcastLister = rt.Handle<"BroadcastLister">;
export declare function pollWanted(self: BroadcastLister): rt.Call<rt.Result<undefined, Error_>>;
export declare function pollUnwanted(self: BroadcastLister): rt.Call<rt.Result<undefined, Error_>>;
export declare function add(self: BroadcastLister, name: string, info: TrackInfo): rt.Result<undefined, Error_>;
export declare function remove(self: BroadcastLister, kind: BroadcastKind, name: string): rt.Result<undefined, Error_>;
export declare function finish(self: BroadcastLister, kind: BroadcastKind, name: string): rt.Result<undefined, Error_>;
export declare function synced(self: BroadcastLister): void;
export declare function clear(self: BroadcastLister): void;
export declare function end(self: BroadcastLister): void;
export declare function abort(self: BroadcastLister, err: Error_): void;

// BroadcastProducer.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastDemand } from "./BroadcastDemand.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { BroadcastRequests } from "./BroadcastRequests.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { DatagramProducer } from "./DatagramProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequest } from "./TrackRequest.ts";
export type BroadcastProducer = rt.Handle<"BroadcastProducer">;
export declare function clone(self: BroadcastProducer): BroadcastProducer;
export declare function new$(info: BroadcastInfo): BroadcastProducer;
export declare function info(self: BroadcastProducer): BroadcastInfo;
export declare function demand(self: BroadcastProducer): BroadcastDemand;
export declare function createTrack(self: BroadcastProducer, name: string, info: TrackInfo | undefined): rt.Result<TrackProducer, Error_>;
export declare function createDatagrams(self: BroadcastProducer, name: string, info: DatagramInfo): rt.Result<DatagramProducer, Error_>;
export declare function reserveTrack(self: BroadcastProducer, name: string): rt.Result<TrackRequest, Error_>;
export declare function reserveUnlistedTrack(self: BroadcastProducer, name: string): rt.Result<TrackRequest, Error_>;
export declare function uniqueTrack(self: BroadcastProducer, suffix: string, info: TrackInfo | undefined): rt.Result<TrackProducer, Error_>;
export declare function uniqueName(self: BroadcastProducer, suffix: string): string;
export declare function requests(self: BroadcastProducer): BroadcastRequests;
export declare function consume(self: BroadcastProducer): BroadcastConsumer;
export declare function close(self: BroadcastProducer): void;
export declare function isClone(self: BroadcastProducer, other: BroadcastProducer): boolean;
export { new$ as new };

// BroadcastRequests.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastDatagramRequests } from "./BroadcastDatagramRequests.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { BroadcastLister } from "./BroadcastLister.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackRequest } from "./TrackRequest.ts";
export type BroadcastRequests = rt.Handle<"BroadcastRequests">;
export declare function clone(self: BroadcastRequests): BroadcastRequests;
export declare function info(self: BroadcastRequests): BroadcastInfo;
export declare function requestedTrack(self: BroadcastRequests): rt.Call<rt.Result<TrackRequest, Error_>>;
export declare function consume(self: BroadcastRequests): BroadcastConsumer;
export declare function datagrams(self: BroadcastRequests): BroadcastDatagramRequests;
export declare function lister(self: BroadcastRequests): rt.Result<BroadcastLister, Error_>;
export declare function closed(self: BroadcastRequests): rt.Call<void>;
export declare function isClone(self: BroadcastRequests, other: BroadcastRequests): boolean;

// BroadcastTracks.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastTracksEvent } from "./BroadcastTracksEvent.ts";
import type { Error_ } from "./Error_.ts";
export type BroadcastTracks = rt.Handle<"BroadcastTracks">;
export declare function next(self: BroadcastTracks): rt.Call<rt.Result<BroadcastTracksEvent | undefined, Error_>>;
export declare function broadcast(self: BroadcastTracks): BroadcastConsumer;
export declare function values(self: BroadcastTracks): rt.Stream<rt.Result<BroadcastTracksEvent, Error_>>;

// BroadcastTracksEvent.ts
import * as rt from "./rt.ts";
export type BroadcastTracksEvent = rt.Handle<"BroadcastTracksEvent">;
export declare function debug(self: BroadcastTracksEvent): string;
export declare function clone(self: BroadcastTracksEvent): BroadcastTracksEvent;

// CacheConfig.ts
import * as rt from "./rt.ts";
export type CacheConfig = rt.Handle<"CacheConfig">;
export declare function debug(self: CacheConfig): string;
export declare function clone(self: CacheConfig): CacheConfig;
export declare function default$(): CacheConfig;
export declare function withCapacity(self: CacheConfig, capacity: rt.u64 | undefined): CacheConfig;
export declare function withExpiry(self: CacheConfig, expiry: rt.Duration | undefined): CacheConfig;
export { default$ as default };

// CachePool.ts
import * as rt from "./rt.ts";
import type { CacheConfig } from "./CacheConfig.ts";
import type { CachePoolWeak } from "./CachePoolWeak.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Waiter } from "./Waiter.ts";
export type CachePool = rt.Handle<"CachePool">;
export declare function debug(self: CachePool): string;
export declare function clone(self: CachePool): CachePool;
export declare function default$(): CachePool;
export declare function new$(config: CacheConfig): CachePool;
export declare function unbounded(): CachePool;
export declare function capacity(self: CachePool): rt.u64 | undefined;
export declare function used(self: CachePool): rt.u64;
export declare function resize(self: CachePool, capacity: rt.u64 | undefined): void;
export declare function expiry(self: CachePool): rt.Duration | undefined;
export declare function poll(self: CachePool, now: ClockInstant, waiter: Waiter): ClockInstant | undefined;
export declare function downgrade(self: CachePool): CachePoolWeak;
export { default$ as default, new$ as new };

// CachePoolWeak.ts
import * as rt from "./rt.ts";
import type { CachePool } from "./CachePool.ts";
export type CachePoolWeak = rt.Handle<"CachePoolWeak">;
export declare function debug(self: CachePoolWeak): string;
export declare function clone(self: CachePoolWeak): CachePoolWeak;
export declare function upgrade(self: CachePoolWeak): CachePool | undefined;

// Client.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthViewers } from "./BandwidthViewers.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Driver } from "./Driver.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginCost } from "./OriginCost.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { QmuxSession } from "./QmuxSession.ts";
import type { Session } from "./Session.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupToken } from "./SetupToken.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { TransportMemorySession } from "./TransportMemorySession.ts";
import type { Url } from "./Url.ts";
import type { Versions } from "./Versions.ts";
export type Client = rt.Handle<"Client">;
export declare function clone(self: Client): Client;
export declare function default$(): Client;
export declare function new$(): Client;
export declare function withPublisherProducer(self: Client, publish: OriginProducer): Client;
export declare function withPublisherConsumer(self: Client, publish: OriginConsumer): Client;
export declare function withSubscriber(self: Client, subscribe: OriginProducer): Client;
export declare function withStats(self: Client, stats: StatsSession): Client;
export declare function withLimits(self: Client, limits: SessionLimits): Client;
export declare function withAllocator(self: Client, allocator: BandwidthAllocator): Client;
export declare function withViewers(self: Client, viewers: BandwidthViewers): Client;
export declare function withVersions(self: Client, versions: Versions): Client;
export declare function withUrl(self: Client, url: Url): Client;
export declare function url(self: Client): Url | undefined;
export declare function withToken(self: Client, token: SetupToken): Client;
export declare function withAuth(self: Client, auth: AuthHandle): Client;
export declare function withImplementation(self: Client, implementation: string): Client;
export declare function withCost(self: Client, cost: OriginCost): Client;
export declare function withPeerNode(self: Client, node: rt.u64): Client;
export declare function connectMemorySession(self: Client, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<[Session, Driver], Error_>>;
export declare function connectQmuxSession(self: Client, now: ClockInstant, session: QmuxSession): rt.Call<rt.Result<[Session, Driver], Error_>>;
export { default$ as default, new$ as new };

// ClockClock.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockWall } from "./ClockWall.ts";
export type ClockClock = rt.Handle<"ClockClock">;
export declare function debug(self: ClockClock): string;
export declare function clone(self: ClockClock): ClockClock;
export declare function eq(a: ClockClock, b: ClockClock): boolean;
export declare function current(): ClockClock;
export declare function now(self: ClockClock): ClockInstant;
export declare function wall(self: ClockClock): ClockWall;

// ClockDeadline.ts
import * as rt from "./rt.ts";
import type { ClockDeadlines } from "./ClockDeadlines.ts";
import type { ClockInstant } from "./ClockInstant.ts";
export type ClockDeadline = rt.Handle<"ClockDeadline">;
export declare function debug(self: ClockDeadline): string;
export declare function new$(deadlines: ClockDeadlines): ClockDeadline;
export declare function after(deadlines: ClockDeadlines, duration: rt.Duration): ClockDeadline;
export declare function set(self: ClockDeadline, at: ClockInstant | undefined): void;
export declare function deadline(self: ClockDeadline): ClockInstant | undefined;
export declare function poll(self: ClockDeadline): rt.Call<void>;
export { new$ as new };

// ClockDeadlines.ts
import * as rt from "./rt.ts";
import type { ClockEntered } from "./ClockEntered.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Waiter } from "./Waiter.ts";
export type ClockDeadlines = rt.Handle<"ClockDeadlines">;
export declare function clone(self: ClockDeadlines): ClockDeadlines;
export declare function new$(now: ClockInstant): ClockDeadlines;
export declare function advance(self: ClockDeadlines, now: ClockInstant): void;
export declare function park(self: ClockDeadlines, waiter: Waiter): ClockInstant | undefined;
export declare function next(self: ClockDeadlines): ClockInstant | undefined;
export declare function now(self: ClockDeadlines): ClockInstant;
export declare function enter(self: ClockDeadlines): ClockEntered;
export { new$ as new };

// ClockElapsed.ts
import * as rt from "./rt.ts";
export type ClockElapsed = rt.Opaque<"ClockElapsed">;
export declare function toString(self: ClockElapsed): string;
export declare function debug(self: ClockElapsed): string;
export declare function eq(a: ClockElapsed, b: ClockElapsed): boolean;

// ClockEntered.ts
import * as rt from "./rt.ts";
export type ClockEntered = rt.Handle<"ClockEntered">;
export declare function debug(self: ClockEntered): string;

// ClockInstant.ts
import * as rt from "./rt.ts";
import type { Instant } from "./Instant.ts";
export type ClockInstant = rt.Opaque<"ClockInstant">;
export declare function debug(self: ClockInstant): string;
export declare function eq(a: ClockInstant, b: ClockInstant): boolean;
export declare function hash(self: ClockInstant): rt.u64;
export declare function partialCmp(a: ClockInstant, b: ClockInstant): number | undefined;
export declare function cmp(a: ClockInstant, b: ClockInstant): number;
export declare function ZERO(): ClockInstant;
export declare function now(): ClockInstant;
export declare function elapsed(self: ClockInstant): rt.Duration;
export declare function durationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration;
export declare function checkedDurationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration | undefined;
export declare function saturatingDurationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration;
export declare function checkedAdd(self: ClockInstant, duration: rt.Duration): ClockInstant | undefined;
export declare function checkedSub(self: ClockInstant, duration: rt.Duration): ClockInstant | undefined;
export declare function saturatingAdd(self: ClockInstant, duration: rt.Duration): ClockInstant;
export declare function saturatingSub(self: ClockInstant, duration: rt.Duration): ClockInstant;
export declare function intoStd(self: ClockInstant): Instant;
export declare function fromStd(at: Instant): ClockInstant;

// ClockInterval.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockMissedTickBehavior } from "./ClockMissedTickBehavior.ts";
export type ClockInterval = rt.Handle<"ClockInterval">;
export declare function tick(self: ClockInterval): rt.Call<ClockInstant>;
export declare function setMissedTickBehavior(self: ClockInterval, missed: ClockMissedTickBehavior): void;
export declare function period(self: ClockInterval): rt.Duration;

// ClockManual.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockWall } from "./ClockWall.ts";
export type ClockManual = rt.Handle<"ClockManual">;
export declare function debug(self: ClockManual): string;
export declare function clone(self: ClockManual): ClockManual;
export declare function default$(): ClockManual;
export declare function WALL_START(): ClockWall;
export declare function new$(): ClockManual;
export declare function now(self: ClockManual): ClockInstant;
export declare function wall(self: ClockManual): ClockWall;
export declare function advance(self: ClockManual, duration: rt.Duration): void;
export declare function stepWall(self: ClockManual, wall: ClockWall): void;
export { default$ as default, new$ as new };

// ClockMissedTickBehavior.ts
export type ClockMissedTickBehavior = "Burst" | "Delay" | "Skip";
export declare function debug(self: ClockMissedTickBehavior): string;
export declare function default$(): ClockMissedTickBehavior;
export { default$ as default };

// ClockRng.ts
import * as rt from "./rt.ts";
export type ClockRng = rt.Handle<"ClockRng">;

// ClockWall.ts
import * as rt from "./rt.ts";
export type ClockWall = rt.Opaque<"ClockWall">;
export declare function debug(self: ClockWall): string;
export declare function eq(a: ClockWall, b: ClockWall): boolean;
export declare function hash(self: ClockWall): rt.u64;
export declare function partialCmp(a: ClockWall, b: ClockWall): number | undefined;
export declare function cmp(a: ClockWall, b: ClockWall): number;
export declare function UNIX_EPOCH(): ClockWall;
export declare function MOQ_EPOCH(): ClockWall;
export declare function now(): ClockWall;
export declare function checkedDurationSince(self: ClockWall, earlier: ClockWall): rt.Duration | undefined;
export declare function saturatingDurationSince(self: ClockWall, earlier: ClockWall): rt.Duration;
export declare function checkedAdd(self: ClockWall, duration: rt.Duration): ClockWall | undefined;
export declare function saturatingAdd(self: ClockWall, duration: rt.Duration): ClockWall;
export declare function checkedSub(self: ClockWall, duration: rt.Duration): ClockWall | undefined;

// ClockWallMap.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockWall } from "./ClockWall.ts";
export type ClockWallMap = rt.Handle<"ClockWallMap">;
export declare function debug(self: ClockWallMap): string;
export declare function clone(self: ClockWallMap): ClockWallMap;
export declare function default$(): ClockWallMap;
export declare function TOLERANCE(): rt.Duration;
export declare function sample(self: ClockWallMap, now: ClockInstant, wall: ClockWall): void;
export declare function map(self: ClockWallMap, at: ClockWall): ClockInstant | undefined;
export { default$ as default };

// CoopBudget.ts
import * as rt from "./rt.ts";
export type CoopBudget = rt.Handle<"CoopBudget">;
export declare function debug(self: CoopBudget): string;
export declare function clone(self: CoopBudget): CoopBudget;
export declare function new$(passes: number): CoopBudget;
export declare function pollYield(self: CoopBudget): rt.Call<void>;
export { new$ as new };

// DatagramConsumer.ts
import * as rt from "./rt.ts";
import type { DatagramDatagram } from "./DatagramDatagram.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { Error_ } from "./Error_.ts";
export type DatagramConsumer = rt.Handle<"DatagramConsumer">;
export declare function clone(self: DatagramConsumer): DatagramConsumer;
export declare function name(self: DatagramConsumer): string;
export declare function info(self: DatagramConsumer): rt.Call<rt.Result<DatagramInfo, Error_>>;
export declare function recv(self: DatagramConsumer): rt.Call<rt.Result<DatagramDatagram | undefined, Error_>>;
export declare function priority(self: DatagramConsumer): number;
export declare function setPriority(self: DatagramConsumer, priority: number): void;

// DatagramDatagram.ts
import * as rt from "./rt.ts";
import type { Timestamp } from "./Timestamp.ts";
export type DatagramDatagram = {
    sequence: rt.u64;
    timestamp: Timestamp;
    payload: Uint8Array;
};
export declare function debug(self: DatagramDatagram): string;
export declare function eq(a: DatagramDatagram, b: DatagramDatagram): boolean;

// DatagramInfo.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type DatagramInfo = rt.Handle<"DatagramInfo">;
export declare function debug(self: DatagramInfo): string;
export declare function clone(self: DatagramInfo): DatagramInfo;
export declare function eq(a: DatagramInfo, b: DatagramInfo): boolean;
export declare function default$(): DatagramInfo;
export declare function timescaleGet(self: DatagramInfo): Timescale | undefined;
export declare function timescaleSet(self: DatagramInfo, value: Timescale | undefined): void;
export declare function priorityGet(self: DatagramInfo): number;
export declare function prioritySet(self: DatagramInfo, value: number): void;
export declare function withTimescale(self: DatagramInfo, timescale: Timescale | undefined): DatagramInfo;
export declare function withPriority(self: DatagramInfo, priority: number): DatagramInfo;
export declare function rescale(self: DatagramInfo, timestamp: Timestamp): rt.Result<Timestamp, Error_>;
export { default$ as default };

// DatagramProducer.ts
import * as rt from "./rt.ts";
import type { DatagramConsumer } from "./DatagramConsumer.ts";
import type { DatagramDatagram } from "./DatagramDatagram.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { Timestamp } from "./Timestamp.ts";
export type DatagramProducer = rt.Handle<"DatagramProducer">;
export declare function clone(self: DatagramProducer): DatagramProducer;
export declare function name(self: DatagramProducer): string;
export declare function info(self: DatagramProducer): DatagramInfo;
export declare function append(self: DatagramProducer, timestamp: Timestamp, payload: Uint8Array | rt.Bytes): rt.Result<rt.u64, Error_>;
export declare function insert(self: DatagramProducer, datagram: DatagramDatagram): rt.Result<undefined, Error_>;
export declare function consume(self: DatagramProducer): DatagramConsumer;
export declare function finish(self: DatagramProducer): void;
export declare function abort(self: DatagramProducer, err: Error_): void;
export declare function priority(self: DatagramProducer): number | undefined;
export declare function priorityChanged(self: DatagramProducer): rt.Call<number | undefined>;

// DatagramRequest.ts
import * as rt from "./rt.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { DatagramProducer } from "./DatagramProducer.ts";
import type { Error_ } from "./Error_.ts";
export type DatagramRequest = rt.Handle<"DatagramRequest">;
export declare function name(self: DatagramRequest): string;
export declare function priority(self: DatagramRequest): number | undefined;
export declare function accept(self: DatagramRequest, info: DatagramInfo): DatagramProducer;
export declare function reject(self: DatagramRequest, err: Error_): void;

// DecodeError.ts
import * as rt from "./rt.ts";
export type DecodeError = rt.Handle<"DecodeError">;
export declare function toString(self: DecodeError): string;
export declare function debug(self: DecodeError): string;
export declare function clone(self: DecodeError): DecodeError;

// Driver.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Error_ } from "./Error_.ts";
import type { Waiter } from "./Waiter.ts";
export type Driver = rt.Handle<"Driver">;
export declare function debug(self: Driver): string;
export declare function poll(self: Driver, now: ClockInstant, waiter: Waiter): rt.Result<ClockInstant | undefined, Error_>;

// EncodeError.ts
export type EncodeError = "BoundsExceeded" | "TooLarge" | "Short" | "InvalidState" | "TooMany" | "Version" | "Unsupported";
export declare function toString(self: EncodeError): string;
export declare function debug(self: EncodeError): string;

// Error_.ts
import * as rt from "./rt.ts";
import type { Retry } from "./Retry.ts";
import type { SessionError } from "./SessionError.ts";
import type { StreamError } from "./StreamError.ts";
import type { TransportMemoryError } from "./TransportMemoryError.ts";
export type Error_ = rt.Handle<"Error_">;
export declare function toString(self: Error_): string;
export declare function debug(self: Error_): string;
export declare function clone(self: Error_): Error_;
export declare function session(self: Error_): SessionError | undefined;
export declare function stream(self: Error_): StreamError | undefined;
export declare function fromTransportError(err: Error_): Error_;
export declare function fromTransportMemoryError(err: TransportMemoryError): Error_;
export declare function retry(self: Error_): Retry;

// FrameConsumer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameConsumer = rt.Handle<"FrameConsumer">;
export declare function clone(self: FrameConsumer): FrameConsumer;
export declare function readChunk(self: FrameConsumer): rt.Call<rt.Result<rt.Bytes | undefined, Error_>>;
export declare function readAll(self: FrameConsumer): rt.Call<rt.Result<rt.Bytes, Error_>>;
export declare function sizeGet(self: FrameConsumer): rt.u64;
export declare function timestampGet(self: FrameConsumer): Timestamp;

// FrameFrame.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameFrame = rt.Handle<"FrameFrame">;
export declare function debug(self: FrameFrame): string;
export declare function clone(self: FrameFrame): FrameFrame;
export declare function timestampGet(self: FrameFrame): Timestamp;
export declare function timestampSet(self: FrameFrame, value: Timestamp): void;
export declare function payloadGet(self: FrameFrame): rt.Bytes;
export declare function payloadSet(self: FrameFrame, value: Uint8Array | rt.Bytes): void;
export declare function new$(timestamp: Timestamp, payload: Uint8Array | rt.Bytes): FrameFrame;
export declare function arrived(self: FrameFrame): ClockInstant;
export { new$ as new };

// FrameInfo.ts
import * as rt from "./rt.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameInfo = {
    size: rt.u64;
    timestamp: Timestamp;
};
export declare function debug(self: FrameInfo): string;

// FrameProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameProducer = rt.Handle<"FrameProducer">;
export declare function group(self: FrameProducer): GroupProducer;
export declare function remaining(self: FrameProducer): rt.u64;
export declare function write(self: FrameProducer, chunk: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function finish(self: FrameProducer): rt.Result<GroupProducer, Error_>;
export declare function abort(self: FrameProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function sizeGet(self: FrameProducer): rt.u64;
export declare function timestampGet(self: FrameProducer): Timestamp;

// GoawayConsumer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GoawayGoaway } from "./GoawayGoaway.ts";
export type GoawayConsumer = rt.Handle<"GoawayConsumer">;
export declare function clone(self: GoawayConsumer): GoawayConsumer;
export declare function poll(self: GoawayConsumer): rt.Call<rt.Result<GoawayGoaway, Error_>>;

// GoawayGoaway.ts
import * as rt from "./rt.ts";
export type GoawayGoaway = rt.Handle<"GoawayGoaway">;
export declare function debug(self: GoawayGoaway): string;
export declare function clone(self: GoawayGoaway): GoawayGoaway;
export declare function eq(a: GoawayGoaway, b: GoawayGoaway): boolean;
export declare function default$(): GoawayGoaway;
export declare function new$(): GoawayGoaway;
export declare function redirect(uri: string): GoawayGoaway;
export declare function withTimeout(self: GoawayGoaway, timeout: rt.Duration | undefined): GoawayGoaway;
export declare function uri(self: GoawayGoaway): string;
export declare function timeout(self: GoawayGoaway): rt.Duration | undefined;
export { default$ as default, new$ as new };

// GroupConsumer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Error_ } from "./Error_.ts";
import type { FrameConsumer } from "./FrameConsumer.ts";
import type { FrameFrame } from "./FrameFrame.ts";
import type { Timescale } from "./Timescale.ts";
export type GroupConsumer = rt.Handle<"GroupConsumer">;
export declare function clone(self: GroupConsumer): GroupConsumer;
export declare function timescale(self: GroupConsumer): Timescale | undefined;
export declare function arrived(self: GroupConsumer): ClockInstant;
export declare function index(self: GroupConsumer): rt.u64;
export declare function withRange(self: GroupConsumer, frames: {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
}): GroupConsumer;
export declare function skipTo(self: GroupConsumer, index: rt.u64): void;
export declare function frameCount(self: GroupConsumer): rt.u64;
export declare function end(self: GroupConsumer): rt.u64 | undefined;
export declare function nextFrame(self: GroupConsumer): rt.Call<rt.Result<FrameConsumer | undefined, Error_>>;
export declare function readFrame(self: GroupConsumer): rt.Call<rt.Result<FrameFrame | undefined, Error_>>;
export declare function finished(self: GroupConsumer): rt.Call<rt.Result<rt.u64, Error_>>;
export declare function startGet(self: GroupConsumer): rt.u64;

// GroupDemand.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type GroupDemand = rt.Handle<"GroupDemand">;
export declare function clone(self: GroupDemand): GroupDemand;
export declare function start(self: GroupDemand): rt.u64;
export declare function wanted(self: GroupDemand): boolean;
export declare function wantedChanged(self: GroupDemand): rt.Call<rt.Result<boolean, Error_>>;
export declare function closed(self: GroupDemand): rt.Call<rt.Result<undefined, Error_>>;

// GroupFetch.ts
import * as rt from "./rt.ts";
export type GroupFetch = rt.Handle<"GroupFetch">;
export declare function debug(self: GroupFetch): string;
export declare function clone(self: GroupFetch): GroupFetch;
export declare function default$(): GroupFetch;
export declare function priorityGet(self: GroupFetch): number;
export declare function prioritySet(self: GroupFetch, value: number): void;
export declare function maxDelayGet(self: GroupFetch): rt.Duration | undefined;
export declare function maxDelaySet(self: GroupFetch, value: rt.Duration | undefined): void;
export declare function withPriority(self: GroupFetch, priority: number): GroupFetch;
export declare function withMaxDelay(self: GroupFetch, maxDelay: rt.Duration): GroupFetch;
export { default$ as default };

// GroupFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
export type GroupFinished = rt.Handle<"GroupFinished">;
export declare function abort(self: GroupFinished, err: Error_): rt.Result<undefined, Error_>;
export declare function frameCount(self: GroupFinished): rt.u64;
export declare function consume(self: GroupFinished): GroupConsumer;
export declare function startGet(self: GroupFinished): rt.u64;

// GroupInfo.ts
import * as rt from "./rt.ts";
export type GroupInfo = {
    start: rt.u64;
};
export declare function debug(self: GroupInfo): string;
export declare function eq(a: GroupInfo, b: GroupInfo): boolean;
export declare function hash(self: GroupInfo): rt.u64;
export declare function partialCmp(a: GroupInfo, b: GroupInfo): number | undefined;
export declare function cmp(a: GroupInfo, b: GroupInfo): number;

// GroupProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { FrameInfo } from "./FrameInfo.ts";
import type { FrameProducer } from "./FrameProducer.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { GroupDemand } from "./GroupDemand.ts";
import type { GroupFinished } from "./GroupFinished.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type GroupProducer = rt.Handle<"GroupProducer">;
export declare function timescale(self: GroupProducer): Timescale | undefined;
export declare function startAt(self: GroupProducer, index: rt.u64): rt.Result<undefined, Error_>;
export declare function writeFrame(self: GroupProducer, timestamp: Timestamp, data: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function createFrame(self: GroupProducer, frame: FrameInfo): rt.Result<FrameProducer, Error_>;
export declare function finishAt(self: GroupProducer, end: rt.u64): rt.Result<GroupFinished, Error_>;
export declare function finish(self: GroupProducer): rt.Result<GroupFinished, Error_>;
export declare function abort(self: GroupProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function ready(self: GroupProducer, frame: FrameInfo): rt.Call<void>;
export declare function frameCount(self: GroupProducer): rt.u64;
export declare function consume(self: GroupProducer): GroupConsumer;
export declare function closed(self: GroupProducer): rt.Call<rt.Result<undefined, Error_>>;
export declare function demand(self: GroupProducer): GroupDemand;
export declare function startGet(self: GroupProducer): rt.u64;

// HangCatalog.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type HangCatalog = rt.Handle<"HangCatalog">;
export declare function debug(self: HangCatalog): string;
export declare function clone(self: HangCatalog): HangCatalog;
export declare function eq(a: HangCatalog, b: HangCatalog): boolean;
export declare function default$(): HangCatalog;
export declare function videoGet(self: HangCatalog): HangCatalogVideo;
export declare function videoSet(self: HangCatalog, value: HangCatalogVideo): void;
export declare function audioGet(self: HangCatalog): HangCatalogAudio;
export declare function audioSet(self: HangCatalog, value: HangCatalogAudio): void;
export declare function syncGet(self: HangCatalog): HangCatalogTimeline;
export declare function syncSet(self: HangCatalog, value: HangCatalogTimeline): void;
export declare function textGet(self: HangCatalog): HangCatalogText;
export declare function textSet(self: HangCatalog, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalog): HangCatalogJson;
export declare function jsonSet(self: HangCatalog, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalog): HangCatalogBinary;
export declare function binarySet(self: HangCatalog, value: HangCatalogBinary): void;
export declare function statsGet(self: HangCatalog): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalog, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalog): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalog, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: HangCatalog): ([string, HangCatalogInclude])[];
export declare function includeSet(self: HangCatalog, value: ([string, HangCatalogInclude])[]): void;
export declare function skippedGet(self: HangCatalog): HangCatalogSkipped;
export declare function skippedSet(self: HangCatalog, value: HangCatalogSkipped): void;
export declare function DEFAULT_NAME(): string;
export declare function defaultTrackInfo(): TrackInfo;
export declare function defaultSubscription(): TrackSubscription;
export declare function checkRenditions(self: HangCatalog): rt.Result<undefined, HangError>;
export declare function withoutExt(self: HangCatalog): HangCatalog;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalog, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalog, HangError>;
export declare function toJson(self: HangCatalog): rt.Result<string, HangError>;
export declare function toVec(self: HangCatalog): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: HangCatalog, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;
export { default$ as default };

// HangCatalogAac.ts
export type HangCatalogAac = {
    profile: number;
};
export declare function toString(self: HangCatalogAac): string;
export declare function debug(self: HangCatalogAac): string;
export declare function eq(a: HangCatalogAac, b: HangCatalogAac): boolean;

// HangCatalogAudio.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangError } from "./HangError.ts";
export type HangCatalogAudio = rt.Handle<"HangCatalogAudio">;
export declare function debug(self: HangCatalogAudio): string;
export declare function clone(self: HangCatalogAudio): HangCatalogAudio;
export declare function eq(a: HangCatalogAudio, b: HangCatalogAudio): boolean;
export declare function default$(): HangCatalogAudio;
export declare function renditionsGet(self: HangCatalogAudio): ([string, HangCatalogAudioConfig])[];
export declare function renditionsSet(self: HangCatalogAudio, value: ([string, HangCatalogAudioConfig])[]): void;
export declare function insert(self: HangCatalogAudio, name: string, config: HangCatalogAudioConfig): rt.Result<undefined, HangError>;
export declare function remove(self: HangCatalogAudio, name: string): HangCatalogAudioConfig | undefined;
export declare function isEmpty(self: HangCatalogAudio): boolean;
export declare function enabled(self: HangCatalogAudio): ([string, HangCatalogAudioConfig])[];
export declare function ranked(self: HangCatalogAudio): ([string, HangCatalogAudioConfig])[];
export { default$ as default };

// HangCatalogAudioCodec.ts
import type { HangCatalogAac } from "./HangCatalogAac.ts";
import type { HangCatalogAudioCodecKind } from "./HangCatalogAudioCodecKind.ts";
export type HangCatalogAudioCodec = {
    tag: "AAC";
    _0: HangCatalogAac;
} | {
    tag: "Opus";
} | {
    tag: "Pcm";
} | {
    tag: "Flac";
} | {
    tag: "Mp3";
} | {
    tag: "Mp2";
} | {
    tag: "Ac3";
} | {
    tag: "Ec3";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogAudioCodec): string;
export declare function debug(self: HangCatalogAudioCodec): string;
export declare function eq(a: HangCatalogAudioCodec, b: HangCatalogAudioCodec): boolean;
export declare function kind(self: HangCatalogAudioCodec): HangCatalogAudioCodecKind;

// HangCatalogAudioCodecKind.ts
export type HangCatalogAudioCodecKind = "AAC" | "Opus" | "Unknown" | "Flac" | "Mp3" | "Pcm" | "Ac3" | "Ec3";
export declare function debug(self: HangCatalogAudioCodecKind): string;

// HangCatalogAudioConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioCodec } from "./HangCatalogAudioCodec.ts";
import type { HangCatalogAudioLayout } from "./HangCatalogAudioLayout.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { Instance } from "./Instance.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogAudioConfig = rt.Handle<"HangCatalogAudioConfig">;
export declare function debug(self: HangCatalogAudioConfig): string;
export declare function clone(self: HangCatalogAudioConfig): HangCatalogAudioConfig;
export declare function eq(a: HangCatalogAudioConfig, b: HangCatalogAudioConfig): boolean;
export declare function broadcastGet(self: HangCatalogAudioConfig): PathRelative | undefined;
export declare function broadcastSet(self: HangCatalogAudioConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: HangCatalogAudioConfig): string | undefined;
export declare function trackSet(self: HangCatalogAudioConfig, value: string | undefined): void;
export declare function instanceGet(self: HangCatalogAudioConfig): Instance | undefined;
export declare function instanceSet(self: HangCatalogAudioConfig, value: Instance | undefined): void;
export declare function labelGet(self: HangCatalogAudioConfig): string | undefined;
export declare function labelSet(self: HangCatalogAudioConfig, value: string | undefined): void;
export declare function codecGet(self: HangCatalogAudioConfig): HangCatalogAudioCodec;
export declare function codecSet(self: HangCatalogAudioConfig, value: HangCatalogAudioCodec): void;
export declare function sampleRateGet(self: HangCatalogAudioConfig): number;
export declare function sampleRateSet(self: HangCatalogAudioConfig, value: number): void;
export declare function channelCountGet(self: HangCatalogAudioConfig): number;
export declare function channelCountSet(self: HangCatalogAudioConfig, value: number): void;
export declare function layoutGet(self: HangCatalogAudioConfig): HangCatalogAudioLayout | undefined;
export declare function layoutSet(self: HangCatalogAudioConfig, value: HangCatalogAudioLayout | undefined): void;
export declare function bitrateGet(self: HangCatalogAudioConfig): rt.u64 | undefined;
export declare function bitrateSet(self: HangCatalogAudioConfig, value: rt.u64 | undefined): void;
export declare function enabledGet(self: HangCatalogAudioConfig): boolean;
export declare function enabledSet(self: HangCatalogAudioConfig, value: boolean): void;
export declare function descriptionGet(self: HangCatalogAudioConfig): rt.Bytes | undefined;
export declare function descriptionSet(self: HangCatalogAudioConfig, value: Uint8Array | rt.Bytes | undefined): void;
export declare function containerGet(self: HangCatalogAudioConfig): HangCatalogContainer;
export declare function containerSet(self: HangCatalogAudioConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: HangCatalogAudioConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogAudioConfig, value: rt.Duration | undefined): void;
export declare function warmupGet(self: HangCatalogAudioConfig): rt.Duration | undefined;
export declare function warmupSet(self: HangCatalogAudioConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: HangCatalogAudioConfig): rt.Duration | undefined;
export declare function delaySet(self: HangCatalogAudioConfig, value: rt.Duration | undefined): void;
export declare function new$(codec: HangCatalogAudioCodec, sampleRate: number, channelCount: number): HangCatalogAudioConfig;
export declare function sameIdentity(self: HangCatalogAudioConfig, other: HangCatalogAudioConfig): boolean;
export { new$ as new };

// HangCatalogAudioLayout.ts
export type HangCatalogAudioLayout = {
    tag: "Ambisonic";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogAudioLayout): string;
export declare function debug(self: HangCatalogAudioLayout): string;
export declare function eq(a: HangCatalogAudioLayout, b: HangCatalogAudioLayout): boolean;

// HangCatalogAv1.ts
export type HangCatalogAv1 = {
    profile: number;
    level: number;
    tier: number;
    bitdepth: number;
    monoChrome: boolean;
    chromaSubsamplingX: boolean;
    chromaSubsamplingY: boolean;
    chromaSamplePosition: number;
    colorPrimaries: number;
    transferCharacteristics: number;
    matrixCoefficients: number;
    fullRange: boolean;
};
export declare function toString(self: HangCatalogAv1): string;
export declare function debug(self: HangCatalogAv1): string;
export declare function eq(a: HangCatalogAv1, b: HangCatalogAv1): boolean;
export declare function default$(): HangCatalogAv1;
export declare function PREFIX(): string;
export { default$ as default };

// HangCatalogBinary.ts
import * as rt from "./rt.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangError } from "./HangError.ts";
export type HangCatalogBinary = rt.Handle<"HangCatalogBinary">;
export declare function debug(self: HangCatalogBinary): string;
export declare function clone(self: HangCatalogBinary): HangCatalogBinary;
export declare function eq(a: HangCatalogBinary, b: HangCatalogBinary): boolean;
export declare function default$(): HangCatalogBinary;
export declare function tracksGet(self: HangCatalogBinary): ([string, HangCatalogBinaryConfig])[];
export declare function tracksSet(self: HangCatalogBinary, value: ([string, HangCatalogBinaryConfig])[]): void;
export declare function insert(self: HangCatalogBinary, name: string, config: HangCatalogBinaryConfig): rt.Result<undefined, HangError>;
export declare function remove(self: HangCatalogBinary, name: string): HangCatalogBinaryConfig | undefined;
export declare function isEmpty(self: HangCatalogBinary): boolean;
export { default$ as default };

// HangCatalogBinaryConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { Instance } from "./Instance.ts";
import type { Json } from "./Json.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogBinaryConfig = rt.Handle<"HangCatalogBinaryConfig">;
export declare function debug(self: HangCatalogBinaryConfig): string;
export declare function clone(self: HangCatalogBinaryConfig): HangCatalogBinaryConfig;
export declare function eq(a: HangCatalogBinaryConfig, b: HangCatalogBinaryConfig): boolean;
export declare function broadcastGet(self: HangCatalogBinaryConfig): PathRelative | undefined;
export declare function broadcastSet(self: HangCatalogBinaryConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: HangCatalogBinaryConfig): string | undefined;
export declare function trackSet(self: HangCatalogBinaryConfig, value: string | undefined): void;
export declare function instanceGet(self: HangCatalogBinaryConfig): Instance | undefined;
export declare function instanceSet(self: HangCatalogBinaryConfig, value: Instance | undefined): void;
export declare function modeGet(self: HangCatalogBinaryConfig): HangCatalogMode;
export declare function modeSet(self: HangCatalogBinaryConfig, value: HangCatalogMode): void;
export declare function mimeGet(self: HangCatalogBinaryConfig): string | undefined;
export declare function mimeSet(self: HangCatalogBinaryConfig, value: string | undefined): void;
export declare function bitrateGet(self: HangCatalogBinaryConfig): rt.u64 | undefined;
export declare function bitrateSet(self: HangCatalogBinaryConfig, value: rt.u64 | undefined): void;
export declare function jitterGet(self: HangCatalogBinaryConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogBinaryConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: HangCatalogBinaryConfig): rt.Duration | undefined;
export declare function delaySet(self: HangCatalogBinaryConfig, value: rt.Duration | undefined): void;
export declare function extraGet(self: HangCatalogBinaryConfig): ([string, Json])[];
export declare function extraSet(self: HangCatalogBinaryConfig, value: ([string, Json])[]): void;
export declare function new$(mode: HangCatalogMode): HangCatalogBinaryConfig;
export { new$ as new };

// HangCatalogChromaticity.ts
import * as rt from "./rt.ts";
export type HangCatalogChromaticity = {
    x: rt.f64;
    y: rt.f64;
};
export declare function debug(self: HangCatalogChromaticity): string;
export declare function eq(a: HangCatalogChromaticity, b: HangCatalogChromaticity): boolean;

// HangCatalogClock.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { HangError } from "./HangError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type HangCatalogClock = rt.Opaque<"HangCatalogClock">;
export declare function debug(self: HangCatalogClock): string;
export declare function eq(a: HangCatalogClock, b: HangCatalogClock): boolean;
export declare function wallGet(self: HangCatalogClock): Timestamp;
export declare function wallSet(self: HangCatalogClock, value: Timestamp): HangCatalogClock;
export declare function new$(wall: Timestamp): rt.Result<HangCatalogClock, HangError>;
export declare function wallClock(self: HangCatalogClock, pts: Timestamp): rt.Result<ClockWall, HangError>;
export { new$ as new };

// HangCatalogColor.ts
import * as rt from "./rt.ts";
import type { HangCatalogColorRange } from "./HangCatalogColorRange.ts";
import type { HangCatalogContentLight } from "./HangCatalogContentLight.ts";
import type { HangCatalogMastering } from "./HangCatalogMastering.ts";
export type HangCatalogColor = rt.Handle<"HangCatalogColor">;
export declare function debug(self: HangCatalogColor): string;
export declare function clone(self: HangCatalogColor): HangCatalogColor;
export declare function eq(a: HangCatalogColor, b: HangCatalogColor): boolean;
export declare function default$(): HangCatalogColor;
export declare function primariesGet(self: HangCatalogColor): number | undefined;
export declare function primariesSet(self: HangCatalogColor, value: number | undefined): void;
export declare function transferGet(self: HangCatalogColor): number | undefined;
export declare function transferSet(self: HangCatalogColor, value: number | undefined): void;
export declare function matrixGet(self: HangCatalogColor): number | undefined;
export declare function matrixSet(self: HangCatalogColor, value: number | undefined): void;
export declare function rangeGet(self: HangCatalogColor): HangCatalogColorRange | undefined;
export declare function rangeSet(self: HangCatalogColor, value: HangCatalogColorRange | undefined): void;
export declare function masteringGet(self: HangCatalogColor): HangCatalogMastering | undefined;
export declare function masteringSet(self: HangCatalogColor, value: HangCatalogMastering | undefined): void;
export declare function contentLightGet(self: HangCatalogColor): HangCatalogContentLight | undefined;
export declare function contentLightSet(self: HangCatalogColor, value: HangCatalogContentLight | undefined): void;
export { default$ as default };

// HangCatalogColorRange.ts
export type HangCatalogColorRange = "Limited" | "Full";
export declare function debug(self: HangCatalogColorRange): string;

// HangCatalogConsumer.ts
import * as rt from "./rt.ts";
import type { HangError } from "./HangError.ts";
import type { MoqJsonTimedCatalog } from "./MoqJsonTimedCatalog.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type HangCatalogConsumer = rt.Handle<"HangCatalogConsumer">;
export declare function new$(track: TrackConsumer, delay: rt.Duration): HangCatalogConsumer;
export declare function pollNext(self: HangCatalogConsumer): rt.Call<rt.Result<MoqJsonTimedCatalog | undefined, HangError>>;
export { new$ as new };

// HangCatalogContainer.ts
import * as rt from "./rt.ts";
export type HangCatalogContainer = rt.Handle<"HangCatalogContainer">;
export declare function debug(self: HangCatalogContainer): string;
export declare function clone(self: HangCatalogContainer): HangCatalogContainer;
export declare function eq(a: HangCatalogContainer, b: HangCatalogContainer): boolean;
export declare function default$(): HangCatalogContainer;
export { default$ as default };

// HangCatalogContentLight.ts
export type HangCatalogContentLight = {
    maxCll: number;
    maxFall: number;
};
export declare function debug(self: HangCatalogContentLight): string;
export declare function eq(a: HangCatalogContentLight, b: HangCatalogContentLight): boolean;

// HangCatalogDisplay.ts
export type HangCatalogDisplay = {
    width: number;
    height: number;
};
export declare function debug(self: HangCatalogDisplay): string;
export declare function eq(a: HangCatalogDisplay, b: HangCatalogDisplay): boolean;

// HangCatalogEcho.ts
import * as rt from "./rt.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogEcho = rt.Handle<"HangCatalogEcho">;
export declare function debug(self: HangCatalogEcho): string;
export declare function clone(self: HangCatalogEcho): HangCatalogEcho;
export declare function eq(a: HangCatalogEcho, b: HangCatalogEcho): boolean;
export declare function pathGet(self: HangCatalogEcho): PathRelative;
export declare function pathSet(self: HangCatalogEcho, value: PathRelative): void;
export declare function new$(path: PathRelative): HangCatalogEcho;
export { new$ as new };

// HangCatalogExt.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { MuxContainerTsExt } from "./MuxContainerTsExt.ts";
export type HangCatalogExt = rt.Handle<"HangCatalogExt">;
export declare function debug(self: HangCatalogExt): string;
export declare function clone(self: HangCatalogExt): HangCatalogExt;
export declare function eq(a: HangCatalogExt, b: HangCatalogExt): boolean;
export declare function default$(): HangCatalogExt;
export declare function videoGet(self: HangCatalogExt): HangCatalogVideo;
export declare function videoSet(self: HangCatalogExt, value: HangCatalogVideo): void;
export declare function audioGet(self: HangCatalogExt): HangCatalogAudio;
export declare function audioSet(self: HangCatalogExt, value: HangCatalogAudio): void;
export declare function syncGet(self: HangCatalogExt): HangCatalogTimeline;
export declare function syncSet(self: HangCatalogExt, value: HangCatalogTimeline): void;
export declare function textGet(self: HangCatalogExt): HangCatalogText;
export declare function textSet(self: HangCatalogExt, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalogExt): HangCatalogJson;
export declare function jsonSet(self: HangCatalogExt, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalogExt): HangCatalogBinary;
export declare function binarySet(self: HangCatalogExt, value: HangCatalogBinary): void;
export declare function statsGet(self: HangCatalogExt): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalogExt, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalogExt): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalogExt, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: HangCatalogExt): ([string, HangCatalogInclude])[];
export declare function includeSet(self: HangCatalogExt, value: ([string, HangCatalogInclude])[]): void;
export declare function extGet(self: HangCatalogExt): MuxContainerTsExt;
export declare function extSet(self: HangCatalogExt, value: MuxContainerTsExt): void;
export declare function skippedGet(self: HangCatalogExt): HangCatalogSkipped;
export declare function skippedSet(self: HangCatalogExt, value: HangCatalogSkipped): void;
export declare function checkRenditions(self: HangCatalogExt): rt.Result<undefined, HangError>;
export declare function withoutExt(self: HangCatalogExt): HangCatalog;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalogExt, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalogExt, HangError>;
export declare function toJson(self: HangCatalogExt): rt.Result<string, HangError>;
export declare function toVec(self: HangCatalogExt): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: HangCatalogExt, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;
export { default$ as default };

// HangCatalogExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { MuxCatalogHangExtra } from "./MuxCatalogHangExtra.ts";
export type HangCatalogExtra = rt.Handle<"HangCatalogExtra">;
export declare function debug(self: HangCatalogExtra): string;
export declare function clone(self: HangCatalogExtra): HangCatalogExtra;
export declare function eq(a: HangCatalogExtra, b: HangCatalogExtra): boolean;
export declare function default$(): HangCatalogExtra;
export declare function videoGet(self: HangCatalogExtra): HangCatalogVideo;
export declare function videoSet(self: HangCatalogExtra, value: HangCatalogVideo): void;
export declare function audioGet(self: HangCatalogExtra): HangCatalogAudio;
export declare function audioSet(self: HangCatalogExtra, value: HangCatalogAudio): void;
export declare function syncGet(self: HangCatalogExtra): HangCatalogTimeline;
export declare function syncSet(self: HangCatalogExtra, value: HangCatalogTimeline): void;
export declare function textGet(self: HangCatalogExtra): HangCatalogText;
export declare function textSet(self: HangCatalogExtra, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalogExtra): HangCatalogJson;
export declare function jsonSet(self: HangCatalogExtra, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalogExtra): HangCatalogBinary;
export declare function binarySet(self: HangCatalogExtra, value: HangCatalogBinary): void;
export declare function statsGet(self: HangCatalogExtra): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalogExtra, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalogExtra): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalogExtra, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: HangCatalogExtra): ([string, HangCatalogInclude])[];
export declare function includeSet(self: HangCatalogExtra, value: ([string, HangCatalogInclude])[]): void;
export declare function extGet(self: HangCatalogExtra): MuxCatalogHangExtra;
export declare function extSet(self: HangCatalogExtra, value: MuxCatalogHangExtra): void;
export declare function skippedGet(self: HangCatalogExtra): HangCatalogSkipped;
export declare function skippedSet(self: HangCatalogExtra, value: HangCatalogSkipped): void;
export declare function checkRenditions(self: HangCatalogExtra): rt.Result<undefined, HangError>;
export declare function withoutExt(self: HangCatalogExtra): HangCatalog;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalogExtra, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalogExtra, HangError>;
export declare function toJson(self: HangCatalogExtra): rt.Result<string, HangError>;
export declare function toVec(self: HangCatalogExtra): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: HangCatalogExtra, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;
export { default$ as default };

// HangCatalogH264.ts
export type HangCatalogH264 = {
    inline: boolean;
    profile: number;
    constraints: number;
    level: number;
};
export declare function toString(self: HangCatalogH264): string;
export declare function debug(self: HangCatalogH264): string;
export declare function eq(a: HangCatalogH264, b: HangCatalogH264): boolean;

// HangCatalogH265.ts
export type HangCatalogH265 = {
    inBand: boolean;
    profileSpace: number;
    profileIdc: number;
    profileCompatibilityFlags: Uint8Array;
    tierFlag: boolean;
    levelIdc: number;
    constraintFlags: Uint8Array;
};
export declare function toString(self: HangCatalogH265): string;
export declare function debug(self: HangCatalogH265): string;
export declare function eq(a: HangCatalogH265, b: HangCatalogH265): boolean;

// HangCatalogInclude.ts
import * as rt from "./rt.ts";
import type { Instance } from "./Instance.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogInclude = rt.Handle<"HangCatalogInclude">;
export declare function debug(self: HangCatalogInclude): string;
export declare function clone(self: HangCatalogInclude): HangCatalogInclude;
export declare function eq(a: HangCatalogInclude, b: HangCatalogInclude): boolean;
export declare function broadcastGet(self: HangCatalogInclude): PathRelative;
export declare function broadcastSet(self: HangCatalogInclude, value: PathRelative): void;
export declare function instanceGet(self: HangCatalogInclude): Instance | undefined;
export declare function instanceSet(self: HangCatalogInclude, value: Instance | undefined): void;
export declare function new$(broadcast: PathRelative): HangCatalogInclude;
export { new$ as new };

// HangCatalogJson.ts
import * as rt from "./rt.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangError } from "./HangError.ts";
export type HangCatalogJson = rt.Handle<"HangCatalogJson">;
export declare function debug(self: HangCatalogJson): string;
export declare function clone(self: HangCatalogJson): HangCatalogJson;
export declare function eq(a: HangCatalogJson, b: HangCatalogJson): boolean;
export declare function default$(): HangCatalogJson;
export declare function tracksGet(self: HangCatalogJson): ([string, HangCatalogJsonConfig])[];
export declare function tracksSet(self: HangCatalogJson, value: ([string, HangCatalogJsonConfig])[]): void;
export declare function insert(self: HangCatalogJson, name: string, config: HangCatalogJsonConfig): rt.Result<undefined, HangError>;
export declare function remove(self: HangCatalogJson, name: string): HangCatalogJsonConfig | undefined;
export declare function isEmpty(self: HangCatalogJson): boolean;
export { default$ as default };

// HangCatalogJsonConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { Instance } from "./Instance.ts";
import type { Json } from "./Json.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogJsonConfig = rt.Handle<"HangCatalogJsonConfig">;
export declare function debug(self: HangCatalogJsonConfig): string;
export declare function clone(self: HangCatalogJsonConfig): HangCatalogJsonConfig;
export declare function eq(a: HangCatalogJsonConfig, b: HangCatalogJsonConfig): boolean;
export declare function broadcastGet(self: HangCatalogJsonConfig): PathRelative | undefined;
export declare function broadcastSet(self: HangCatalogJsonConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: HangCatalogJsonConfig): string | undefined;
export declare function trackSet(self: HangCatalogJsonConfig, value: string | undefined): void;
export declare function instanceGet(self: HangCatalogJsonConfig): Instance | undefined;
export declare function instanceSet(self: HangCatalogJsonConfig, value: Instance | undefined): void;
export declare function modeGet(self: HangCatalogJsonConfig): HangCatalogMode;
export declare function modeSet(self: HangCatalogJsonConfig, value: HangCatalogMode): void;
export declare function schemaGet(self: HangCatalogJsonConfig): string | undefined;
export declare function schemaSet(self: HangCatalogJsonConfig, value: string | undefined): void;
export declare function bitrateGet(self: HangCatalogJsonConfig): rt.u64 | undefined;
export declare function bitrateSet(self: HangCatalogJsonConfig, value: rt.u64 | undefined): void;
export declare function jitterGet(self: HangCatalogJsonConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogJsonConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: HangCatalogJsonConfig): rt.Duration | undefined;
export declare function delaySet(self: HangCatalogJsonConfig, value: rt.Duration | undefined): void;
export declare function extraGet(self: HangCatalogJsonConfig): ([string, Json])[];
export declare function extraSet(self: HangCatalogJsonConfig, value: ([string, Json])[]): void;
export declare function new$(mode: HangCatalogMode): HangCatalogJsonConfig;
export { new$ as new };

// HangCatalogMastering.ts
import * as rt from "./rt.ts";
import type { HangCatalogChromaticity } from "./HangCatalogChromaticity.ts";
export type HangCatalogMastering = {
    red: HangCatalogChromaticity;
    green: HangCatalogChromaticity;
    blue: HangCatalogChromaticity;
    white: HangCatalogChromaticity;
    maxLuminance: rt.f64;
    minLuminance: rt.f64;
};
export declare function debug(self: HangCatalogMastering): string;
export declare function eq(a: HangCatalogMastering, b: HangCatalogMastering): boolean;

// HangCatalogMode.ts
export type HangCatalogMode = {
    tag: "Snapshot";
} | {
    tag: "Stream";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogMode): string;
export declare function debug(self: HangCatalogMode): string;
export declare function eq(a: HangCatalogMode, b: HangCatalogMode): boolean;

// HangCatalogPriorities.ts
import * as rt from "./rt.ts";
export type HangCatalogPriorities = rt.Opaque<"HangCatalogPriorities">;
export declare function debug(self: HangCatalogPriorities): string;
export declare function eq(a: HangCatalogPriorities, b: HangCatalogPriorities): boolean;
export declare function catalogGet(self: HangCatalogPriorities): number;
export declare function catalogSet(self: HangCatalogPriorities, value: number): HangCatalogPriorities;
export declare function textGet(self: HangCatalogPriorities): number;
export declare function textSet(self: HangCatalogPriorities, value: number): HangCatalogPriorities;
export declare function audioGet(self: HangCatalogPriorities): number;
export declare function audioSet(self: HangCatalogPriorities, value: number): HangCatalogPriorities;
export declare function videoGet(self: HangCatalogPriorities): number;
export declare function videoSet(self: HangCatalogPriorities, value: number): HangCatalogPriorities;

// HangCatalogProjection.ts
import * as rt from "./rt.ts";
export type HangCatalogProjection = rt.Handle<"HangCatalogProjection">;
export declare function debug(self: HangCatalogProjection): string;
export declare function clone(self: HangCatalogProjection): HangCatalogProjection;
export declare function eq(a: HangCatalogProjection, b: HangCatalogProjection): boolean;
export declare function default$(): HangCatalogProjection;
export { default$ as default };

// HangCatalogSkipped.ts
import * as rt from "./rt.ts";
export type HangCatalogSkipped = rt.Handle<"HangCatalogSkipped">;
export declare function debug(self: HangCatalogSkipped): string;
export declare function clone(self: HangCatalogSkipped): HangCatalogSkipped;
export declare function eq(a: HangCatalogSkipped, b: HangCatalogSkipped): boolean;
export declare function default$(): HangCatalogSkipped;
export declare function videoGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function videoSet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function audioGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function audioSet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function textGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function textSet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function jsonGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function jsonSet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function binaryGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function binarySet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function includeGet(self: HangCatalogSkipped): ([string, rt.u64])[];
export declare function includeSet(self: HangCatalogSkipped, value: ([string, rt.u64])[]): void;
export declare function isEmpty(self: HangCatalogSkipped): boolean;
export declare function version(self: HangCatalogSkipped): rt.u64 | undefined;
export { default$ as default };

// HangCatalogStats.ts
import * as rt from "./rt.ts";
export type HangCatalogStats = rt.Handle<"HangCatalogStats">;
export declare function debug(self: HangCatalogStats): string;
export declare function clone(self: HangCatalogStats): HangCatalogStats;
export declare function eq(a: HangCatalogStats, b: HangCatalogStats): boolean;
export declare function trackGet(self: HangCatalogStats): string;
export declare function trackSet(self: HangCatalogStats, value: string): void;
export declare function new$(track: string): HangCatalogStats;
export { new$ as new };

// HangCatalogStereo.ts
export type HangCatalogStereo = {
    tag: "Mono";
} | {
    tag: "LeftRight";
} | {
    tag: "TopBottom";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogStereo): string;
export declare function debug(self: HangCatalogStereo): string;
export declare function eq(a: HangCatalogStereo, b: HangCatalogStereo): boolean;

// HangCatalogText.ts
import * as rt from "./rt.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangError } from "./HangError.ts";
export type HangCatalogText = rt.Handle<"HangCatalogText">;
export declare function debug(self: HangCatalogText): string;
export declare function clone(self: HangCatalogText): HangCatalogText;
export declare function eq(a: HangCatalogText, b: HangCatalogText): boolean;
export declare function default$(): HangCatalogText;
export declare function renditionsGet(self: HangCatalogText): ([string, HangCatalogTextConfig])[];
export declare function renditionsSet(self: HangCatalogText, value: ([string, HangCatalogTextConfig])[]): void;
export declare function insert(self: HangCatalogText, name: string, config: HangCatalogTextConfig): rt.Result<undefined, HangError>;
export declare function remove(self: HangCatalogText, name: string): HangCatalogTextConfig | undefined;
export declare function isEmpty(self: HangCatalogText): boolean;
export { default$ as default };

// HangCatalogTextConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { HangCatalogTextFormat } from "./HangCatalogTextFormat.ts";
import type { HangCatalogTextRole } from "./HangCatalogTextRole.ts";
import type { Instance } from "./Instance.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogTextConfig = rt.Handle<"HangCatalogTextConfig">;
export declare function debug(self: HangCatalogTextConfig): string;
export declare function clone(self: HangCatalogTextConfig): HangCatalogTextConfig;
export declare function eq(a: HangCatalogTextConfig, b: HangCatalogTextConfig): boolean;
export declare function broadcastGet(self: HangCatalogTextConfig): PathRelative | undefined;
export declare function broadcastSet(self: HangCatalogTextConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: HangCatalogTextConfig): string | undefined;
export declare function trackSet(self: HangCatalogTextConfig, value: string | undefined): void;
export declare function instanceGet(self: HangCatalogTextConfig): Instance | undefined;
export declare function instanceSet(self: HangCatalogTextConfig, value: Instance | undefined): void;
export declare function formatGet(self: HangCatalogTextConfig): HangCatalogTextFormat;
export declare function formatSet(self: HangCatalogTextConfig, value: HangCatalogTextFormat): void;
export declare function roleGet(self: HangCatalogTextConfig): HangCatalogTextRole;
export declare function roleSet(self: HangCatalogTextConfig, value: HangCatalogTextRole): void;
export declare function langGet(self: HangCatalogTextConfig): string | undefined;
export declare function langSet(self: HangCatalogTextConfig, value: string | undefined): void;
export declare function labelGet(self: HangCatalogTextConfig): string | undefined;
export declare function labelSet(self: HangCatalogTextConfig, value: string | undefined): void;
export declare function containerGet(self: HangCatalogTextConfig): HangCatalogContainer;
export declare function containerSet(self: HangCatalogTextConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: HangCatalogTextConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogTextConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: HangCatalogTextConfig): rt.Duration | undefined;
export declare function delaySet(self: HangCatalogTextConfig, value: rt.Duration | undefined): void;
export declare function new$(format: HangCatalogTextFormat): HangCatalogTextConfig;
export { new$ as new };

// HangCatalogTextFormat.ts
export type HangCatalogTextFormat = {
    tag: "Vtt";
} | {
    tag: "Ttml";
} | {
    tag: "Utf8";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogTextFormat): string;
export declare function debug(self: HangCatalogTextFormat): string;
export declare function eq(a: HangCatalogTextFormat, b: HangCatalogTextFormat): boolean;
export declare function default$(): HangCatalogTextFormat;
export { default$ as default };

// HangCatalogTextRole.ts
export type HangCatalogTextRole = {
    tag: "Subtitle";
} | {
    tag: "Caption";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogTextRole): string;
export declare function debug(self: HangCatalogTextRole): string;
export declare function eq(a: HangCatalogTextRole, b: HangCatalogTextRole): boolean;
export declare function default$(): HangCatalogTextRole;
export { default$ as default };

// HangCatalogTimeline.ts
import * as rt from "./rt.ts";
import type { HangCatalogClock } from "./HangCatalogClock.ts";
export type HangCatalogTimeline = rt.Handle<"HangCatalogTimeline">;
export declare function debug(self: HangCatalogTimeline): string;
export declare function clone(self: HangCatalogTimeline): HangCatalogTimeline;
export declare function eq(a: HangCatalogTimeline, b: HangCatalogTimeline): boolean;
export declare function default$(): HangCatalogTimeline;
export declare function clockGet(self: HangCatalogTimeline): HangCatalogClock | undefined;
export declare function clockSet(self: HangCatalogTimeline, value: HangCatalogClock | undefined): void;
export declare function intervalGet(self: HangCatalogTimeline): rt.Duration | undefined;
export declare function intervalSet(self: HangCatalogTimeline, value: rt.Duration | undefined): void;
export declare function isEmpty(self: HangCatalogTimeline): boolean;
export { default$ as default };

// HangCatalogVideo.ts
import * as rt from "./rt.ts";
import type { HangCatalogDisplay } from "./HangCatalogDisplay.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { HangCatalogVideoProperties } from "./HangCatalogVideoProperties.ts";
import type { HangError } from "./HangError.ts";
export type HangCatalogVideo = rt.Handle<"HangCatalogVideo">;
export declare function debug(self: HangCatalogVideo): string;
export declare function clone(self: HangCatalogVideo): HangCatalogVideo;
export declare function eq(a: HangCatalogVideo, b: HangCatalogVideo): boolean;
export declare function default$(): HangCatalogVideo;
export declare function renditionsGet(self: HangCatalogVideo): ([string, HangCatalogVideoConfig])[];
export declare function renditionsSet(self: HangCatalogVideo, value: ([string, HangCatalogVideoConfig])[]): void;
export declare function displayGet(self: HangCatalogVideo): HangCatalogDisplay | undefined;
export declare function displaySet(self: HangCatalogVideo, value: HangCatalogDisplay | undefined): void;
export declare function rotationGet(self: HangCatalogVideo): rt.f64 | undefined;
export declare function rotationSet(self: HangCatalogVideo, value: rt.f64 | undefined): void;
export declare function flipGet(self: HangCatalogVideo): boolean | undefined;
export declare function flipSet(self: HangCatalogVideo, value: boolean | undefined): void;
export declare function insert(self: HangCatalogVideo, name: string, config: HangCatalogVideoConfig): rt.Result<undefined, HangError>;
export declare function remove(self: HangCatalogVideo, name: string): HangCatalogVideoConfig | undefined;
export declare function isEmpty(self: HangCatalogVideo): boolean;
export declare function enabled(self: HangCatalogVideo): ([string, HangCatalogVideoConfig])[];
export declare function ranked(self: HangCatalogVideo): ([string, HangCatalogVideoConfig])[];
export declare function setProperties(self: HangCatalogVideo, properties: HangCatalogVideoProperties): rt.Result<undefined, HangError>;
export { default$ as default };

// HangCatalogVideoCodec.ts
import type { HangCatalogAv1 } from "./HangCatalogAv1.ts";
import type { HangCatalogH264 } from "./HangCatalogH264.ts";
import type { HangCatalogH265 } from "./HangCatalogH265.ts";
import type { HangCatalogVideoCodecKind } from "./HangCatalogVideoCodecKind.ts";
import type { HangCatalogVp9 } from "./HangCatalogVp9.ts";
export type HangCatalogVideoCodec = {
    tag: "H264";
    _0: HangCatalogH264;
} | {
    tag: "H265";
    _0: HangCatalogH265;
} | {
    tag: "VP9";
    _0: HangCatalogVp9;
} | {
    tag: "AV1";
    _0: HangCatalogAv1;
} | {
    tag: "VP8";
} | {
    tag: "Unknown";
    _0: string;
};
export declare function toString(self: HangCatalogVideoCodec): string;
export declare function debug(self: HangCatalogVideoCodec): string;
export declare function eq(a: HangCatalogVideoCodec, b: HangCatalogVideoCodec): boolean;
export declare function kind(self: HangCatalogVideoCodec): HangCatalogVideoCodecKind;

// HangCatalogVideoCodecKind.ts
export type HangCatalogVideoCodecKind = "H264" | "H265" | "VP8" | "VP9" | "AV1" | "Unknown";
export declare function debug(self: HangCatalogVideoCodecKind): string;

// HangCatalogVideoConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogColor } from "./HangCatalogColor.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { HangCatalogProjection } from "./HangCatalogProjection.ts";
import type { HangCatalogStereo } from "./HangCatalogStereo.ts";
import type { HangCatalogVideoCodec } from "./HangCatalogVideoCodec.ts";
import type { Instance } from "./Instance.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogVideoConfig = rt.Handle<"HangCatalogVideoConfig">;
export declare function debug(self: HangCatalogVideoConfig): string;
export declare function clone(self: HangCatalogVideoConfig): HangCatalogVideoConfig;
export declare function eq(a: HangCatalogVideoConfig, b: HangCatalogVideoConfig): boolean;
export declare function broadcastGet(self: HangCatalogVideoConfig): PathRelative | undefined;
export declare function broadcastSet(self: HangCatalogVideoConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: HangCatalogVideoConfig): string | undefined;
export declare function trackSet(self: HangCatalogVideoConfig, value: string | undefined): void;
export declare function instanceGet(self: HangCatalogVideoConfig): Instance | undefined;
export declare function instanceSet(self: HangCatalogVideoConfig, value: Instance | undefined): void;
export declare function labelGet(self: HangCatalogVideoConfig): string | undefined;
export declare function labelSet(self: HangCatalogVideoConfig, value: string | undefined): void;
export declare function codecGet(self: HangCatalogVideoConfig): HangCatalogVideoCodec;
export declare function codecSet(self: HangCatalogVideoConfig, value: HangCatalogVideoCodec): void;
export declare function descriptionGet(self: HangCatalogVideoConfig): rt.Bytes | undefined;
export declare function descriptionSet(self: HangCatalogVideoConfig, value: Uint8Array | rt.Bytes | undefined): void;
export declare function codedWidthGet(self: HangCatalogVideoConfig): number | undefined;
export declare function codedWidthSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function codedHeightGet(self: HangCatalogVideoConfig): number | undefined;
export declare function codedHeightSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function displayAspectWidthGet(self: HangCatalogVideoConfig): number | undefined;
export declare function displayAspectWidthSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function displayAspectHeightGet(self: HangCatalogVideoConfig): number | undefined;
export declare function displayAspectHeightSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function colorGet(self: HangCatalogVideoConfig): HangCatalogColor | undefined;
export declare function colorSet(self: HangCatalogVideoConfig, value: HangCatalogColor | undefined): void;
export declare function projectionGet(self: HangCatalogVideoConfig): HangCatalogProjection | undefined;
export declare function projectionSet(self: HangCatalogVideoConfig, value: HangCatalogProjection | undefined): void;
export declare function stereoGet(self: HangCatalogVideoConfig): HangCatalogStereo | undefined;
export declare function stereoSet(self: HangCatalogVideoConfig, value: HangCatalogStereo | undefined): void;
export declare function interlacedGet(self: HangCatalogVideoConfig): boolean;
export declare function interlacedSet(self: HangCatalogVideoConfig, value: boolean): void;
export declare function bitrateGet(self: HangCatalogVideoConfig): rt.u64 | undefined;
export declare function bitrateSet(self: HangCatalogVideoConfig, value: rt.u64 | undefined): void;
export declare function enabledGet(self: HangCatalogVideoConfig): boolean;
export declare function enabledSet(self: HangCatalogVideoConfig, value: boolean): void;
export declare function preferenceGet(self: HangCatalogVideoConfig): number;
export declare function preferenceSet(self: HangCatalogVideoConfig, value: number): void;
export declare function framerateGet(self: HangCatalogVideoConfig): rt.f64 | undefined;
export declare function framerateSet(self: HangCatalogVideoConfig, value: rt.f64 | undefined): void;
export declare function reorderGet(self: HangCatalogVideoConfig): number | undefined;
export declare function reorderSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function optimizeForLatencyGet(self: HangCatalogVideoConfig): boolean | undefined;
export declare function optimizeForLatencySet(self: HangCatalogVideoConfig, value: boolean | undefined): void;
export declare function containerGet(self: HangCatalogVideoConfig): HangCatalogContainer;
export declare function containerSet(self: HangCatalogVideoConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: HangCatalogVideoConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogVideoConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: HangCatalogVideoConfig): rt.Duration | undefined;
export declare function delaySet(self: HangCatalogVideoConfig, value: rt.Duration | undefined): void;
export declare function new$(codec: HangCatalogVideoCodec): HangCatalogVideoConfig;
export declare function sameIdentity(self: HangCatalogVideoConfig, other: HangCatalogVideoConfig): boolean;
export { new$ as new };

// HangCatalogVideoProperties.ts
import * as rt from "./rt.ts";
import type { HangCatalogDisplay } from "./HangCatalogDisplay.ts";
export type HangCatalogVideoProperties = rt.Handle<"HangCatalogVideoProperties">;
export declare function debug(self: HangCatalogVideoProperties): string;
export declare function clone(self: HangCatalogVideoProperties): HangCatalogVideoProperties;
export declare function eq(a: HangCatalogVideoProperties, b: HangCatalogVideoProperties): boolean;
export declare function default$(): HangCatalogVideoProperties;
export declare function displayGet(self: HangCatalogVideoProperties): HangCatalogDisplay | undefined;
export declare function displaySet(self: HangCatalogVideoProperties, value: HangCatalogDisplay | undefined): void;
export declare function rotationGet(self: HangCatalogVideoProperties): rt.f64 | undefined;
export declare function rotationSet(self: HangCatalogVideoProperties, value: rt.f64 | undefined): void;
export declare function flipGet(self: HangCatalogVideoProperties): boolean | undefined;
export declare function flipSet(self: HangCatalogVideoProperties, value: boolean | undefined): void;
export { default$ as default };

// HangCatalogVp9.ts
export type HangCatalogVp9 = {
    profile: number;
    level: number;
    bitDepth: number;
    chromaSubsampling: number;
    colorPrimaries: number;
    transferCharacteristics: number;
    matrixCoefficients: number;
    fullRange: boolean;
};
export declare function toString(self: HangCatalogVp9): string;
export declare function debug(self: HangCatalogVp9): string;
export declare function eq(a: HangCatalogVp9, b: HangCatalogVp9): boolean;
export declare function default$(): HangCatalogVp9;
export { default$ as default };

// HangContainerFrame.ts
import * as rt from "./rt.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangError } from "./HangError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type HangContainerFrame = {
    timestamp: Timestamp;
    payload: Uint8Array;
};
export declare function debug(self: HangContainerFrame): string;
export declare function encode(self: HangContainerFrame, buf: rt.Place<Uint8Array>): rt.Result<undefined, HangError>;
export declare function decode(buf: Uint8Array | rt.Bytes): rt.Result<HangContainerFrame, HangError>;
export declare function writeTo(self: HangContainerFrame, group: GroupProducer): rt.Result<undefined, HangError>;

// HangEchoArrival.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { Timestamp } from "./Timestamp.ts";
export type HangEchoArrival = rt.Handle<"HangEchoArrival">;
export declare function debug(self: HangEchoArrival): string;
export declare function clone(self: HangEchoArrival): HangEchoArrival;
export declare function eq(a: HangEchoArrival, b: HangEchoArrival): boolean;
export declare function timestampGet(self: HangEchoArrival): Timestamp;
export declare function timestampSet(self: HangEchoArrival, value: Timestamp): void;
export declare function wallGet(self: HangEchoArrival): ClockWall;
export declare function wallSet(self: HangEchoArrival, value: ClockWall): void;
export declare function new$(timestamp: Timestamp, wall: ClockWall): HangEchoArrival;
export { new$ as new };

// HangEchoRendition.ts
import * as rt from "./rt.ts";
import type { HangEchoArrival } from "./HangEchoArrival.ts";
export type HangEchoRendition = rt.Handle<"HangEchoRendition">;
export declare function debug(self: HangEchoRendition): string;
export declare function clone(self: HangEchoRendition): HangEchoRendition;
export declare function eq(a: HangEchoRendition, b: HangEchoRendition): boolean;
export declare function default$(): HangEchoRendition;
export declare function framesGet(self: HangEchoRendition): rt.u64;
export declare function framesSet(self: HangEchoRendition, value: rt.u64): void;
export declare function bytesGet(self: HangEchoRendition): rt.u64;
export declare function bytesSet(self: HangEchoRendition, value: rt.u64): void;
export declare function decodedGet(self: HangEchoRendition): rt.u64;
export declare function decodedSet(self: HangEchoRendition, value: rt.u64): void;
export declare function lateGet(self: HangEchoRendition): rt.u64;
export declare function lateSet(self: HangEchoRendition, value: rt.u64): void;
export declare function errorsGet(self: HangEchoRendition): rt.u64;
export declare function errorsSet(self: HangEchoRendition, value: rt.u64): void;
export declare function stallsGet(self: HangEchoRendition): rt.u64;
export declare function stallsSet(self: HangEchoRendition, value: rt.u64): void;
export declare function underrunsGet(self: HangEchoRendition): rt.u64;
export declare function underrunsSet(self: HangEchoRendition, value: rt.u64): void;
export declare function stalledGet(self: HangEchoRendition): rt.Duration;
export declare function stalledSet(self: HangEchoRendition, value: rt.Duration): void;
export declare function arrivalGet(self: HangEchoRendition): HangEchoArrival | undefined;
export declare function arrivalSet(self: HangEchoRendition, value: HangEchoArrival | undefined): void;
export { default$ as default };

// HangEchoSnapshot.ts
import * as rt from "./rt.ts";
import type { HangEchoRendition } from "./HangEchoRendition.ts";
import type { HangStatsTransport } from "./HangStatsTransport.ts";
export type HangEchoSnapshot = rt.Handle<"HangEchoSnapshot">;
export declare function debug(self: HangEchoSnapshot): string;
export declare function clone(self: HangEchoSnapshot): HangEchoSnapshot;
export declare function eq(a: HangEchoSnapshot, b: HangEchoSnapshot): boolean;
export declare function default$(): HangEchoSnapshot;
export declare function transportGet(self: HangEchoSnapshot): HangStatsTransport;
export declare function transportSet(self: HangEchoSnapshot, value: HangStatsTransport): void;
export declare function renditionsGet(self: HangEchoSnapshot): ([string, HangEchoRendition])[];
export declare function renditionsSet(self: HangEchoSnapshot, value: ([string, HangEchoRendition])[]): void;
export { default$ as default };

// HangError.ts
import * as rt from "./rt.ts";
export type HangError = rt.Handle<"HangError">;
export declare function toString(self: HangError): string;
export declare function debug(self: HangError): string;
export declare function clone(self: HangError): HangError;

// HangStatsRendition.ts
import * as rt from "./rt.ts";
export type HangStatsRendition = rt.Handle<"HangStatsRendition">;
export declare function debug(self: HangStatsRendition): string;
export declare function clone(self: HangStatsRendition): HangStatsRendition;
export declare function eq(a: HangStatsRendition, b: HangStatsRendition): boolean;
export declare function default$(): HangStatsRendition;
export declare function framesGet(self: HangStatsRendition): rt.u64;
export declare function framesSet(self: HangStatsRendition, value: rt.u64): void;
export declare function bytesGet(self: HangStatsRendition): rt.u64;
export declare function bytesSet(self: HangStatsRendition, value: rt.u64): void;
export declare function keyframesGet(self: HangStatsRendition): rt.u64;
export declare function keyframesSet(self: HangStatsRendition, value: rt.u64): void;
export declare function skippedGet(self: HangStatsRendition): rt.u64;
export declare function skippedSet(self: HangStatsRendition, value: rt.u64): void;
export declare function bitrateGet(self: HangStatsRendition): rt.u64 | undefined;
export declare function bitrateSet(self: HangStatsRendition, value: rt.u64 | undefined): void;
export { default$ as default };

// HangStatsSnapshot.ts
import * as rt from "./rt.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { HangStatsTransport } from "./HangStatsTransport.ts";
export type HangStatsSnapshot = rt.Handle<"HangStatsSnapshot">;
export declare function debug(self: HangStatsSnapshot): string;
export declare function clone(self: HangStatsSnapshot): HangStatsSnapshot;
export declare function eq(a: HangStatsSnapshot, b: HangStatsSnapshot): boolean;
export declare function default$(): HangStatsSnapshot;
export declare function transportGet(self: HangStatsSnapshot): HangStatsTransport;
export declare function transportSet(self: HangStatsSnapshot, value: HangStatsTransport): void;
export declare function renditionsGet(self: HangStatsSnapshot): ([string, HangStatsRendition])[];
export declare function renditionsSet(self: HangStatsSnapshot, value: ([string, HangStatsRendition])[]): void;
export { default$ as default };

// HangStatsTransport.ts
import * as rt from "./rt.ts";
export type HangStatsTransport = rt.Handle<"HangStatsTransport">;
export declare function debug(self: HangStatsTransport): string;
export declare function clone(self: HangStatsTransport): HangStatsTransport;
export declare function eq(a: HangStatsTransport, b: HangStatsTransport): boolean;
export declare function default$(): HangStatsTransport;
export declare function rttGet(self: HangStatsTransport): rt.Duration | undefined;
export declare function rttSet(self: HangStatsTransport, value: rt.Duration | undefined): void;
export declare function rateGet(self: HangStatsTransport): rt.u64 | undefined;
export declare function rateSet(self: HangStatsTransport, value: rt.u64 | undefined): void;
export declare function lostBytesGet(self: HangStatsTransport): rt.u64 | undefined;
export declare function lostBytesSet(self: HangStatsTransport, value: rt.u64 | undefined): void;
export declare function lostPacketsGet(self: HangStatsTransport): rt.u64 | undefined;
export declare function lostPacketsSet(self: HangStatsTransport, value: rt.u64 | undefined): void;
export declare function ageGet(self: HangStatsTransport): rt.Duration | undefined;
export declare function ageSet(self: HangStatsTransport, value: rt.Duration | undefined): void;
export { default$ as default };

// Instance.ts
import * as rt from "./rt.ts";
export type Instance = rt.Handle<"Instance">;
export declare function toString(self: Instance): string;
export declare function debug(self: Instance): string;
export declare function clone(self: Instance): Instance;
export declare function eq(a: Instance, b: Instance): boolean;
export declare function hash(self: Instance): rt.u64;
export declare function partialCmp(a: Instance, b: Instance): number | undefined;
export declare function cmp(a: Instance, b: Instance): number;
export declare function random(): Instance;
export declare function asStr(self: Instance): string;
export declare function isMinted(self: Instance): boolean;

// Instant.ts
import * as rt from "./rt.ts";
export type Instant = rt.Handle<"Instant">;
export declare function debug(self: Instant): string;
export declare function clone(self: Instant): Instant;
export declare function eq(a: Instant, b: Instant): boolean;
export declare function hash(self: Instant): rt.u64;
export declare function partialCmp(a: Instant, b: Instant): number | undefined;
export declare function cmp(a: Instant, b: Instant): number;

// IntersectionError.ts
export type IntersectionError = "TooManyPatterns";
export declare function toString(self: IntersectionError): string;
export declare function debug(self: IntersectionError): string;

// InvalidPattern.ts
export type InvalidPattern = {
    tag: "EmptySegment";
} | {
    tag: "InvalidSegment";
    _0: string;
} | {
    tag: "MultipleGlobstars";
} | {
    tag: "TooManySegments";
};
export declare function toString(self: InvalidPattern): string;
export declare function debug(self: InvalidPattern): string;
export declare function eq(a: InvalidPattern, b: InvalidPattern): boolean;

// Json.ts
import * as rt from "./rt.ts";
export type Json = rt.Handle<"Json">;
export declare function parse(input: string): rt.Result<Json, string>;
export declare function toText(self: Json): string;

// Level.ts
export type Level = "Error" | "Warn" | "Info" | "Debug" | "Trace";
export declare function toString(self: Level): string;
export declare function partialCmp(a: Level, b: Level): number | undefined;
export declare function cmp(a: Level, b: Level): number;

// MoqJsonTimedCatalog.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MoqJsonTimedCatalog = rt.Handle<"MoqJsonTimedCatalog">;
export declare function debug(self: MoqJsonTimedCatalog): string;
export declare function clone(self: MoqJsonTimedCatalog): MoqJsonTimedCatalog;
export declare function eq(a: MoqJsonTimedCatalog, b: MoqJsonTimedCatalog): boolean;
export declare function valueGet(self: MoqJsonTimedCatalog): HangCatalog;
export declare function valueSet(self: MoqJsonTimedCatalog, value: HangCatalog): void;
export declare function atGet(self: MoqJsonTimedCatalog): Timestamp;
export declare function atSet(self: MoqJsonTimedCatalog, value: Timestamp): void;
export declare function new$(value: HangCatalog, at: Timestamp): MoqJsonTimedCatalog;
export { new$ as new };

// MoqJsonTimedJson.ts
import * as rt from "./rt.ts";
import type { Json } from "./Json.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MoqJsonTimedJson = rt.Handle<"MoqJsonTimedJson">;
export declare function debug(self: MoqJsonTimedJson): string;
export declare function clone(self: MoqJsonTimedJson): MoqJsonTimedJson;
export declare function eq(a: MoqJsonTimedJson, b: MoqJsonTimedJson): boolean;
export declare function valueGet(self: MoqJsonTimedJson): Json;
export declare function valueSet(self: MoqJsonTimedJson, value: Json): void;
export declare function atGet(self: MoqJsonTimedJson): Timestamp;
export declare function atSet(self: MoqJsonTimedJson, value: Timestamp): void;
export declare function new$(value: Json, at: Timestamp): MoqJsonTimedJson;
export { new$ as new };

// MoqQuicConnectionMuxVersion.ts
export type MoqQuicConnectionMuxVersion = "QMux00" | "QMux01" | "QMux02";
export declare function debug(self: MoqQuicConnectionMuxVersion): string;
export declare function ALL(): MoqQuicConnectionMuxVersion[];
export declare function usesRecords(self: MoqQuicConnectionMuxVersion): boolean;
export declare function alpn(self: MoqQuicConnectionMuxVersion): string;
export declare function prefix(self: MoqQuicConnectionMuxVersion): string;

// MoqQuicLimits.ts
import * as rt from "./rt.ts";
import type { MoqQuicRate } from "./MoqQuicRate.ts";
export type MoqQuicLimits = rt.Handle<"MoqQuicLimits">;
export declare function debug(self: MoqQuicLimits): string;
export declare function clone(self: MoqQuicLimits): MoqQuicLimits;
export declare function eq(a: MoqQuicLimits, b: MoqQuicLimits): boolean;
export declare function default$(): MoqQuicLimits;
export declare function maxStreamsGet(self: MoqQuicLimits): rt.u64 | undefined;
export declare function maxStreamsSet(self: MoqQuicLimits, value: rt.u64 | undefined): void;
export declare function receiveWindowGet(self: MoqQuicLimits): rt.u64 | undefined;
export declare function receiveWindowSet(self: MoqQuicLimits, value: rt.u64 | undefined): void;
export declare function streamReceiveWindowGet(self: MoqQuicLimits): rt.u64 | undefined;
export declare function streamReceiveWindowSet(self: MoqQuicLimits, value: rt.u64 | undefined): void;
export declare function sendWindowGet(self: MoqQuicLimits): rt.u64 | undefined;
export declare function sendWindowSet(self: MoqQuicLimits, value: rt.u64 | undefined): void;
export declare function receiveRateGet(self: MoqQuicLimits): MoqQuicRate | undefined;
export declare function receiveRateSet(self: MoqQuicLimits, value: MoqQuicRate | undefined): void;
export declare function sendRateGet(self: MoqQuicLimits): MoqQuicRate | undefined;
export declare function sendRateSet(self: MoqQuicLimits, value: MoqQuicRate | undefined): void;
export declare function validate(self: MoqQuicLimits): rt.Result<undefined, string>;
export { default$ as default };

// MoqQuicRate.ts
import * as rt from "./rt.ts";
export type MoqQuicRate = {
    tag: "Unlimited";
} | {
    tag: "BytesPerSecond";
    _0: rt.u64;
};
export declare function debug(self: MoqQuicRate): string;
export declare function eq(a: MoqQuicRate, b: MoqQuicRate): boolean;

// MoqSockDemuxCounters.ts
import * as rt from "./rt.ts";
import type { MoqSockDemuxDiscard } from "./MoqSockDemuxDiscard.ts";
export type MoqSockDemuxCounters = rt.Handle<"MoqSockDemuxCounters">;
export declare function debug(self: MoqSockDemuxCounters): string;
export declare function clone(self: MoqSockDemuxCounters): MoqSockDemuxCounters;
export declare function default$(): MoqSockDemuxCounters;
export declare function discard(self: MoqSockDemuxCounters, reason: MoqSockDemuxDiscard): void;
export declare function get(self: MoqSockDemuxCounters, reason: MoqSockDemuxDiscard): rt.u64;
export { default$ as default };

// MoqSockDemuxDiscard.ts
export type MoqSockDemuxDiscard = "Stun" | "Unknown" | "Unclaimed" | "Overflow";
export declare function debug(self: MoqSockDemuxDiscard): string;
export declare function ALL(): MoqSockDemuxDiscard[];
export declare function asStr(self: MoqSockDemuxDiscard): string;

// MoqSockDemuxPin.ts
import * as rt from "./rt.ts";
export type MoqSockDemuxPin = rt.Handle<"MoqSockDemuxPin">;
export declare function debug(self: MoqSockDemuxPin): string;

// MoqSockUdpEcnCodepoint.ts
export type MoqSockUdpEcnCodepoint = "Ect0" | "Ect1" | "Ce";
export declare function debug(self: MoqSockUdpEcnCodepoint): string;
export declare function fromBits(x: number): MoqSockUdpEcnCodepoint | undefined;

// MoqTokioAcceptFailure.ts
export type MoqTokioAcceptFailure = "Connection" | "Exhausted" | "Unknown";
export declare function toString(self: MoqTokioAcceptFailure): string;
export declare function debug(self: MoqTokioAcceptFailure): string;
export declare function ALL(): MoqTokioAcceptFailure[];
export declare function asStr(self: MoqTokioAcceptFailure): string;

// MoqTokioAcceptHealth.ts
import * as rt from "./rt.ts";
import type { MoqTokioAcceptFailure } from "./MoqTokioAcceptFailure.ts";
export type MoqTokioAcceptHealth = rt.Handle<"MoqTokioAcceptHealth">;
export declare function debug(self: MoqTokioAcceptHealth): string;
export declare function clone(self: MoqTokioAcceptHealth): MoqTokioAcceptHealth;
export declare function new$(listener: string): MoqTokioAcceptHealth;
export declare function listener(self: MoqTokioAcceptHealth): string;
export declare function accepted(self: MoqTokioAcceptHealth): void;
export declare function failures(self: MoqTokioAcceptHealth, failure: MoqTokioAcceptFailure): rt.u64;
export declare function stalled(self: MoqTokioAcceptHealth): rt.Duration | undefined;
export { new$ as new };

// MoqTokioAddrs.ts
import * as rt from "./rt.ts";
import type { MoqTokioConnectAddr } from "./MoqTokioConnectAddr.ts";
export type MoqTokioAddrs = rt.Handle<"MoqTokioAddrs">;
export declare function debug(self: MoqTokioAddrs): string;
export declare function clone(self: MoqTokioAddrs): MoqTokioAddrs;
export declare function eq(a: MoqTokioAddrs, b: MoqTokioAddrs): boolean;
export declare function new$(addr: MoqTokioConnectAddr): MoqTokioAddrs;
export declare function or(self: MoqTokioAddrs, addr: MoqTokioConnectAddr): MoqTokioAddrs;
export declare function collect(urls: MoqTokioConnectAddr[]): MoqTokioAddrs | undefined;
export declare function asSlice(self: MoqTokioAddrs): MoqTokioConnectAddr[];
export { new$ as new };

// MoqTokioCliBound.ts
import * as rt from "./rt.ts";
export type MoqTokioCliBound = rt.Handle<"MoqTokioCliBound">;
export declare function toString(self: MoqTokioCliBound): string;
export declare function debug(self: MoqTokioCliBound): string;
export declare function clone(self: MoqTokioCliBound): MoqTokioCliBound;
export declare function eq(a: MoqTokioCliBound, b: MoqTokioCliBound): boolean;
export declare function hash(self: MoqTokioCliBound): rt.u64;
export declare function NONE(): MoqTokioCliBound;

// MoqTokioCliDeprecated.ts
import * as rt from "./rt.ts";
export type MoqTokioCliDeprecated = rt.Handle<"MoqTokioCliDeprecated">;
export declare function toString(self: MoqTokioCliDeprecated): string;
export declare function debug(self: MoqTokioCliDeprecated): string;
export declare function clone(self: MoqTokioCliDeprecated): MoqTokioCliDeprecated;
export declare function default$(): MoqTokioCliDeprecated;
export declare function isEmpty(self: MoqTokioCliDeprecated): boolean;
export declare function extend(self: MoqTokioCliDeprecated, other: MoqTokioCliDeprecated): void;
export declare function flag(self: MoqTokioCliDeprecated, old: string, env: string | undefined, new_: string): void;
export declare function changed(self: MoqTokioCliDeprecated, old: string, env: string | undefined, new_: string, note: string): void;
export declare function removed(self: MoqTokioCliDeprecated, old: string, env: string | undefined, note: string): void;
export declare function toml(self: MoqTokioCliDeprecated, old: string, new_: string, note: string | undefined): void;
export { default$ as default };

// MoqTokioConnectAddr.ts
import * as rt from "./rt.ts";
import type { MoqTokioConnectError } from "./MoqTokioConnectError.ts";
import type { Url } from "./Url.ts";
export type MoqTokioConnectAddr = rt.Handle<"MoqTokioConnectAddr">;
export declare function debug(self: MoqTokioConnectAddr): string;
export declare function clone(self: MoqTokioConnectAddr): MoqTokioConnectAddr;
export declare function eq(a: MoqTokioConnectAddr, b: MoqTokioConnectAddr): boolean;
export declare function new$(url: Url): MoqTokioConnectAddr;
export declare function pinned(url: Url, addrs: rt.SocketAddr[]): rt.Result<MoqTokioConnectAddr, MoqTokioConnectError>;
export declare function url(self: MoqTokioConnectAddr): Url;
export declare function addresses(self: MoqTokioConnectAddr): rt.SocketAddr[] | undefined;
export { new$ as new };

// MoqTokioConnectError.ts
export type MoqTokioConnectError = {
    tag: "EmptyAddresses";
} | {
    tag: "UnsupportedScheme";
    _0: string;
} | {
    tag: "MissingHost";
};
export declare function toString(self: MoqTokioConnectError): string;
export declare function debug(self: MoqTokioConnectError): string;
export declare function eq(a: MoqTokioConnectError, b: MoqTokioConnectError): boolean;

// MoqTokioConnectResolved.ts
import * as rt from "./rt.ts";
export type MoqTokioConnectResolved = rt.Handle<"MoqTokioConnectResolved">;
export declare function debug(self: MoqTokioConnectResolved): string;
export declare function clone(self: MoqTokioConnectResolved): MoqTokioConnectResolved;
export declare function bindGet(self: MoqTokioConnectResolved): rt.SocketAddr;
export declare function bindSet(self: MoqTokioConnectResolved, value: rt.SocketAddr): void;
export declare function raceGet(self: MoqTokioConnectResolved): rt.Duration;
export declare function raceSet(self: MoqTokioConnectResolved, value: rt.Duration): void;
export declare function resolutionDelayGet(self: MoqTokioConnectResolved): rt.Duration;
export declare function resolutionDelaySet(self: MoqTokioConnectResolved, value: rt.Duration): void;
export declare function timeoutGet(self: MoqTokioConnectResolved): rt.Duration | undefined;
export declare function timeoutSet(self: MoqTokioConnectResolved, value: rt.Duration | undefined): void;

// MoqTokioConnectionHandover.ts
import * as rt from "./rt.ts";
import type { MoqTokioRedirect } from "./MoqTokioRedirect.ts";
import type { TokioDuration } from "./TokioDuration.ts";
export type MoqTokioConnectionHandover = rt.Handle<"MoqTokioConnectionHandover">;
export declare function debug(self: MoqTokioConnectionHandover): string;
export declare function clone(self: MoqTokioConnectionHandover): MoqTokioConnectionHandover;
export declare function default$(): MoqTokioConnectionHandover;
export declare function redirectGet(self: MoqTokioConnectionHandover): MoqTokioRedirect;
export declare function redirectSet(self: MoqTokioConnectionHandover, value: MoqTokioRedirect): void;
export declare function handoverGet(self: MoqTokioConnectionHandover): TokioDuration | undefined;
export declare function handoverSet(self: MoqTokioConnectionHandover, value: TokioDuration | undefined): void;
export { default$ as default };

// MoqTokioConnectionObservation.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MoqTokioConnectionPhase } from "./MoqTokioConnectionPhase.ts";
export type MoqTokioConnectionObservation = rt.Handle<"MoqTokioConnectionObservation">;
export declare function debug(self: MoqTokioConnectionObservation): string;
export declare function clone(self: MoqTokioConnectionObservation): MoqTokioConnectionObservation;
export declare function revisionGet(self: MoqTokioConnectionObservation): rt.u64;
export declare function revisionSet(self: MoqTokioConnectionObservation, value: rt.u64): void;
export declare function connectsGet(self: MoqTokioConnectionObservation): rt.u64;
export declare function connectsSet(self: MoqTokioConnectionObservation, value: rt.u64): void;
export declare function lastConnectedGet(self: MoqTokioConnectionObservation): ClockInstant | undefined;
export declare function lastConnectedSet(self: MoqTokioConnectionObservation, value: ClockInstant | undefined): void;
export declare function phaseGet(self: MoqTokioConnectionObservation): MoqTokioConnectionPhase;
export declare function phaseSet(self: MoqTokioConnectionObservation, value: MoqTokioConnectionPhase): void;

// MoqTokioConnectionPhase.ts
import * as rt from "./rt.ts";
export type MoqTokioConnectionPhase = rt.Handle<"MoqTokioConnectionPhase">;
export declare function debug(self: MoqTokioConnectionPhase): string;
export declare function clone(self: MoqTokioConnectionPhase): MoqTokioConnectionPhase;

// MoqTokioConnectionSnapshot.ts
import * as rt from "./rt.ts";
import type { SessionStats } from "./SessionStats.ts";
import type { Version } from "./Version.ts";
export type MoqTokioConnectionSnapshot = rt.Handle<"MoqTokioConnectionSnapshot">;
export declare function statsGet(self: MoqTokioConnectionSnapshot): SessionStats;
export declare function statsSet(self: MoqTokioConnectionSnapshot, value: SessionStats): void;
export declare function versionGet(self: MoqTokioConnectionSnapshot): Version;
export declare function versionSet(self: MoqTokioConnectionSnapshot, value: Version): void;

// MoqTokioListenForward.ts
import * as rt from "./rt.ts";
export type MoqTokioListenForward = rt.Handle<"MoqTokioListenForward">;
export declare function debug(self: MoqTokioListenForward): string;
export declare function clone(self: MoqTokioListenForward): MoqTokioListenForward;
export declare function default$(): MoqTokioListenForward;
export declare function bindGet(self: MoqTokioListenForward): rt.SocketAddr | undefined;
export declare function bindSet(self: MoqTokioListenForward, value: rt.SocketAddr | undefined): void;
export declare function keyGet(self: MoqTokioListenForward): rt.PathBuf | undefined;
export declare function keySet(self: MoqTokioListenForward, value: rt.PathBuf | undefined): void;
export declare function peersGet(self: MoqTokioListenForward): rt.PathBuf | undefined;
export declare function peersSet(self: MoqTokioListenForward, value: rt.PathBuf | undefined): void;
export { default$ as default };

// MoqTokioListenLoadBalancer.ts
import * as rt from "./rt.ts";
import type { MoqTokioListenForward } from "./MoqTokioListenForward.ts";
import type { MoqTokioQuicServerId } from "./MoqTokioQuicServerId.ts";
export type MoqTokioListenLoadBalancer = rt.Handle<"MoqTokioListenLoadBalancer">;
export declare function debug(self: MoqTokioListenLoadBalancer): string;
export declare function clone(self: MoqTokioListenLoadBalancer): MoqTokioListenLoadBalancer;
export declare function default$(): MoqTokioListenLoadBalancer;
export declare function idGet(self: MoqTokioListenLoadBalancer): MoqTokioQuicServerId | undefined;
export declare function idSet(self: MoqTokioListenLoadBalancer, value: MoqTokioQuicServerId | undefined): void;
export declare function nonceGet(self: MoqTokioListenLoadBalancer): rt.u64 | undefined;
export declare function nonceSet(self: MoqTokioListenLoadBalancer, value: rt.u64 | undefined): void;
export declare function forwardGet(self: MoqTokioListenLoadBalancer): MoqTokioListenForward;
export declare function forwardSet(self: MoqTokioListenLoadBalancer, value: MoqTokioListenForward): void;
export { default$ as default };

// MoqTokioListenReset.ts
import * as rt from "./rt.ts";
export type MoqTokioListenReset = rt.Handle<"MoqTokioListenReset">;
export declare function debug(self: MoqTokioListenReset): string;
export declare function clone(self: MoqTokioListenReset): MoqTokioListenReset;
export declare function default$(): MoqTokioListenReset;
export declare function secretGet(self: MoqTokioListenReset): rt.PathBuf | undefined;
export declare function secretSet(self: MoqTokioListenReset, value: rt.PathBuf | undefined): void;
export { default$ as default };

// MoqTokioListenResolved.ts
import * as rt from "./rt.ts";
import type { MoqTokioListenLoadBalancer } from "./MoqTokioListenLoadBalancer.ts";
import type { TokioListenBind } from "./TokioListenBind.ts";
import type { TokioTlsListen } from "./TokioTlsListen.ts";
import type { Versions } from "./Versions.ts";
export type MoqTokioListenResolved = rt.Handle<"MoqTokioListenResolved">;
export declare function debug(self: MoqTokioListenResolved): string;
export declare function clone(self: MoqTokioListenResolved): MoqTokioListenResolved;
export declare function bindGet(self: MoqTokioListenResolved): TokioListenBind | undefined;
export declare function bindSet(self: MoqTokioListenResolved, value: TokioListenBind | undefined): void;
export declare function versionsGet(self: MoqTokioListenResolved): Versions;
export declare function versionsSet(self: MoqTokioListenResolved, value: Versions): void;
export declare function implementationGet(self: MoqTokioListenResolved): string | undefined;
export declare function implementationSet(self: MoqTokioListenResolved, value: string | undefined): void;
export declare function timeoutGet(self: MoqTokioListenResolved): rt.Duration | undefined;
export declare function timeoutSet(self: MoqTokioListenResolved, value: rt.Duration | undefined): void;
export declare function tlsGet(self: MoqTokioListenResolved): TokioTlsListen;
export declare function tlsSet(self: MoqTokioListenResolved, value: TokioTlsListen): void;
export declare function preferredV4Get(self: MoqTokioListenResolved): rt.SocketAddrV4 | undefined;
export declare function preferredV4Set(self: MoqTokioListenResolved, value: rt.SocketAddrV4 | undefined): void;
export declare function preferredV6Get(self: MoqTokioListenResolved): rt.SocketAddrV6 | undefined;
export declare function preferredV6Set(self: MoqTokioListenResolved, value: rt.SocketAddrV6 | undefined): void;
export declare function lbGet(self: MoqTokioListenResolved): MoqTokioListenLoadBalancer;
export declare function lbSet(self: MoqTokioListenResolved, value: MoqTokioListenLoadBalancer): void;
export declare function resetSecretGet(self: MoqTokioListenResolved): rt.PathBuf | undefined;
export declare function resetSecretSet(self: MoqTokioListenResolved, value: rt.PathBuf | undefined): void;

// MoqTokioQuicCongestionControl.ts
export type MoqTokioQuicCongestionControl = "Bbr" | "Cubic" | "Unlimited";
export declare function debug(self: MoqTokioQuicCongestionControl): string;
export declare function default$(): MoqTokioQuicCongestionControl;
export { default$ as default };

// MoqTokioQuicDemuxDatagram.ts
import * as rt from "./rt.ts";
export type MoqTokioQuicDemuxDatagram = rt.Handle<"MoqTokioQuicDemuxDatagram">;
export declare function debug(self: MoqTokioQuicDemuxDatagram): string;
export declare function clone(self: MoqTokioQuicDemuxDatagram): MoqTokioQuicDemuxDatagram;
export declare function remoteGet(self: MoqTokioQuicDemuxDatagram): rt.SocketAddr;
export declare function remoteSet(self: MoqTokioQuicDemuxDatagram, value: rt.SocketAddr): void;
export declare function localGet(self: MoqTokioQuicDemuxDatagram): rt.IpAddr | undefined;
export declare function localSet(self: MoqTokioQuicDemuxDatagram, value: rt.IpAddr | undefined): void;
export declare function dataGet(self: MoqTokioQuicDemuxDatagram): rt.Bytes;
export declare function dataSet(self: MoqTokioQuicDemuxDatagram, value: Uint8Array | rt.Bytes): void;

// MoqTokioQuicDemuxDemux.ts
import * as rt from "./rt.ts";
import type { MoqSockDemuxCounters } from "./MoqSockDemuxCounters.ts";
import type { MoqTokioQuicDemuxWebRtc } from "./MoqTokioQuicDemuxWebRtc.ts";
export type MoqTokioQuicDemuxDemux = rt.Handle<"MoqTokioQuicDemuxDemux">;
export declare function debug(self: MoqTokioQuicDemuxDemux): string;
export declare function clone(self: MoqTokioQuicDemuxDemux): MoqTokioQuicDemuxDemux;
export declare function default$(): MoqTokioQuicDemuxDemux;
export declare function counters(self: MoqTokioQuicDemuxDemux): MoqSockDemuxCounters;
export declare function webrtc(self: MoqTokioQuicDemuxDemux): MoqTokioQuicDemuxWebRtc;
export { default$ as default };

// MoqTokioQuicDemuxWebRtc.ts
import * as rt from "./rt.ts";
import type { MoqSockDemuxPin } from "./MoqSockDemuxPin.ts";
import type { MoqTokioQuicDemuxDatagram } from "./MoqTokioQuicDemuxDatagram.ts";
export type MoqTokioQuicDemuxWebRtc = rt.Handle<"MoqTokioQuicDemuxWebRtc">;
export declare function debug(self: MoqTokioQuicDemuxWebRtc): string;
export declare function recv(self: MoqTokioQuicDemuxWebRtc): rt.Call<MoqTokioQuicDemuxDatagram | undefined>;
export declare function pin(self: MoqTokioQuicDemuxWebRtc, remote: rt.SocketAddr): MoqSockDemuxPin;

// MoqTokioQuicResolved.ts
import * as rt from "./rt.ts";
import type { MoqQuicLimits } from "./MoqQuicLimits.ts";
import type { MoqTokioQuicCongestionControl } from "./MoqTokioQuicCongestionControl.ts";
export type MoqTokioQuicResolved = rt.Handle<"MoqTokioQuicResolved">;
export declare function debug(self: MoqTokioQuicResolved): string;
export declare function clone(self: MoqTokioQuicResolved): MoqTokioQuicResolved;
export declare function default$(): MoqTokioQuicResolved;
export declare function maxStreamsGet(self: MoqTokioQuicResolved): rt.u64;
export declare function maxStreamsSet(self: MoqTokioQuicResolved, value: rt.u64): void;
export declare function gsoGet(self: MoqTokioQuicResolved): boolean | undefined;
export declare function gsoSet(self: MoqTokioQuicResolved, value: boolean | undefined): void;
export declare function idleTimeoutGet(self: MoqTokioQuicResolved): rt.Duration | undefined;
export declare function idleTimeoutSet(self: MoqTokioQuicResolved, value: rt.Duration | undefined): void;
export declare function keepAliveGet(self: MoqTokioQuicResolved): rt.Duration | undefined;
export declare function keepAliveSet(self: MoqTokioQuicResolved, value: rt.Duration | undefined): void;
export declare function mtuDiscoveryGet(self: MoqTokioQuicResolved): boolean;
export declare function mtuDiscoverySet(self: MoqTokioQuicResolved, value: boolean): void;
export declare function congestionControlGet(self: MoqTokioQuicResolved): MoqTokioQuicCongestionControl;
export declare function congestionControlSet(self: MoqTokioQuicResolved, value: MoqTokioQuicCongestionControl): void;
export declare function receiveWindowGet(self: MoqTokioQuicResolved): rt.u64;
export declare function receiveWindowSet(self: MoqTokioQuicResolved, value: rt.u64): void;
export declare function streamReceiveWindowGet(self: MoqTokioQuicResolved): rt.u64;
export declare function streamReceiveWindowSet(self: MoqTokioQuicResolved, value: rt.u64): void;
export declare function sendWindowGet(self: MoqTokioQuicResolved): rt.u64;
export declare function sendWindowSet(self: MoqTokioQuicResolved, value: rt.u64): void;
export declare function qlogGet(self: MoqTokioQuicResolved): rt.PathBuf | undefined;
export declare function qlogSet(self: MoqTokioQuicResolved, value: rt.PathBuf | undefined): void;
export declare function keylogGet(self: MoqTokioQuicResolved): rt.PathBuf | undefined;
export declare function keylogSet(self: MoqTokioQuicResolved, value: rt.PathBuf | undefined): void;
export declare function keepAliveInterval(self: MoqTokioQuicResolved): rt.Duration;
export declare function limits(self: MoqTokioQuicResolved): MoqQuicLimits;
export { default$ as default };

// MoqTokioQuicServerId.ts
import * as rt from "./rt.ts";
export type MoqTokioQuicServerId = rt.Handle<"MoqTokioQuicServerId">;
export declare function debug(self: MoqTokioQuicServerId): string;
export declare function clone(self: MoqTokioQuicServerId): MoqTokioQuicServerId;
export declare function eq(a: MoqTokioQuicServerId, b: MoqTokioQuicServerId): boolean;
export declare function hash(self: MoqTokioQuicServerId): rt.u64;

// MoqTokioRedirect.ts
export type MoqTokioRedirect = "Follow" | "SameHost" | "Ignore";
export declare function debug(self: MoqTokioRedirect): string;
export declare function default$(): MoqTokioRedirect;
export { default$ as default };

// MoqTokioServerQuicLimits.ts
import * as rt from "./rt.ts";
import type { MoqQuicLimits } from "./MoqQuicLimits.ts";
export type MoqTokioServerQuicLimits = rt.Handle<"MoqTokioServerQuicLimits">;
export declare function debug(self: MoqTokioServerQuicLimits): string;
export declare function clone(self: MoqTokioServerQuicLimits): MoqTokioServerQuicLimits;
export declare function set(self: MoqTokioServerQuicLimits, limits: MoqQuicLimits): rt.Result<undefined, string>;

// MoqTokioServerStreams.ts
import * as rt from "./rt.ts";
export type MoqTokioServerStreams = rt.Handle<"MoqTokioServerStreams">;
export declare function debug(self: MoqTokioServerStreams): string;
export declare function default$(): MoqTokioServerStreams;
export { default$ as default };

// MoqTokioTcpConfig.ts
import * as rt from "./rt.ts";
export type MoqTokioTcpConfig = rt.Handle<"MoqTokioTcpConfig">;
export declare function debug(self: MoqTokioTcpConfig): string;
export declare function clone(self: MoqTokioTcpConfig): MoqTokioTcpConfig;
export declare function default$(): MoqTokioTcpConfig;
export declare function bindGet(self: MoqTokioTcpConfig): rt.SocketAddr | undefined;
export declare function bindSet(self: MoqTokioTcpConfig, value: rt.SocketAddr | undefined): void;
export declare function tlsGet(self: MoqTokioTcpConfig): boolean | undefined;
export declare function tlsSet(self: MoqTokioTcpConfig, value: boolean | undefined): void;
export { default$ as default };

// MoqTokioTcpTlsAcceptor.ts
import * as rt from "./rt.ts";
import type { RustlsServerConfig } from "./RustlsServerConfig.ts";
export type MoqTokioTcpTlsAcceptor = rt.Handle<"MoqTokioTcpTlsAcceptor">;
export declare function clone(self: MoqTokioTcpTlsAcceptor): MoqTokioTcpTlsAcceptor;
export declare function config(self: MoqTokioTcpTlsAcceptor): RustlsServerConfig;
export declare function offered(self: MoqTokioTcpTlsAcceptor, alpns: Uint8Array[]): boolean;

// MoqTokioUnixAllow.ts
import * as rt from "./rt.ts";
export type MoqTokioUnixAllow = rt.Handle<"MoqTokioUnixAllow">;
export declare function debug(self: MoqTokioUnixAllow): string;
export declare function clone(self: MoqTokioUnixAllow): MoqTokioUnixAllow;
export declare function default$(): MoqTokioUnixAllow;
export declare function uidGet(self: MoqTokioUnixAllow): Uint32Array;
export declare function uidSet(self: MoqTokioUnixAllow, value: Uint32Array): void;
export declare function gidGet(self: MoqTokioUnixAllow): Uint32Array;
export declare function gidSet(self: MoqTokioUnixAllow, value: Uint32Array): void;
export declare function pidGet(self: MoqTokioUnixAllow): Int32Array;
export declare function pidSet(self: MoqTokioUnixAllow, value: Int32Array): void;
export { default$ as default };

// MoqTokioUnixConfig.ts
import * as rt from "./rt.ts";
import type { MoqTokioUnixAllow } from "./MoqTokioUnixAllow.ts";
export type MoqTokioUnixConfig = rt.Handle<"MoqTokioUnixConfig">;
export declare function debug(self: MoqTokioUnixConfig): string;
export declare function clone(self: MoqTokioUnixConfig): MoqTokioUnixConfig;
export declare function default$(): MoqTokioUnixConfig;
export declare function bindGet(self: MoqTokioUnixConfig): rt.PathBuf | undefined;
export declare function bindSet(self: MoqTokioUnixConfig, value: rt.PathBuf | undefined): void;
export declare function allowGet(self: MoqTokioUnixConfig): MoqTokioUnixAllow;
export declare function allowSet(self: MoqTokioUnixConfig, value: MoqTokioUnixAllow): void;
export { default$ as default };

// MoqTokioWebsocketConfig.ts
import * as rt from "./rt.ts";
import type { MoqTokioWebsocketResolved } from "./MoqTokioWebsocketResolved.ts";
import type { TokioDuration } from "./TokioDuration.ts";
export type MoqTokioWebsocketConfig = rt.Handle<"MoqTokioWebsocketConfig">;
export declare function debug(self: MoqTokioWebsocketConfig): string;
export declare function clone(self: MoqTokioWebsocketConfig): MoqTokioWebsocketConfig;
export declare function default$(): MoqTokioWebsocketConfig;
export declare function enabledGet(self: MoqTokioWebsocketConfig): boolean | undefined;
export declare function enabledSet(self: MoqTokioWebsocketConfig, value: boolean | undefined): void;
export declare function delayGet(self: MoqTokioWebsocketConfig): TokioDuration | undefined;
export declare function delaySet(self: MoqTokioWebsocketConfig, value: TokioDuration | undefined): void;
export declare function resolve(self: MoqTokioWebsocketConfig): MoqTokioWebsocketResolved;
export { default$ as default };

// MoqTokioWebsocketError.ts
import * as rt from "./rt.ts";
export type MoqTokioWebsocketError = rt.Handle<"MoqTokioWebsocketError">;
export declare function toString(self: MoqTokioWebsocketError): string;
export declare function debug(self: MoqTokioWebsocketError): string;

// MoqTokioWebsocketListener.ts
import * as rt from "./rt.ts";
import type { MoqTokioAcceptHealth } from "./MoqTokioAcceptHealth.ts";
import type { MoqTokioQuicResolved } from "./MoqTokioQuicResolved.ts";
import type { MoqTokioWebsocketError } from "./MoqTokioWebsocketError.ts";
import type { QmuxSession } from "./QmuxSession.ts";
export type MoqTokioWebsocketListener = rt.Handle<"MoqTokioWebsocketListener">;
export declare function bind(addr: rt.SocketAddr): rt.Call<rt.Result<MoqTokioWebsocketListener, MoqTokioWebsocketError>>;
export declare function withProtocols(self: MoqTokioWebsocketListener, protocols: string[]): rt.Result<MoqTokioWebsocketListener, MoqTokioWebsocketError>;
export declare function withQuic(self: MoqTokioWebsocketListener, quic: MoqTokioQuicResolved): MoqTokioWebsocketListener;
export declare function localAddr(self: MoqTokioWebsocketListener): rt.Result<rt.SocketAddr, MoqTokioWebsocketError>;
export declare function acceptHealth(self: MoqTokioWebsocketListener): MoqTokioAcceptHealth;
export declare function accept(self: MoqTokioWebsocketListener): rt.Call<rt.Result<QmuxSession, MoqTokioWebsocketError>>;

// MoqTokioWebsocketResolved.ts
import * as rt from "./rt.ts";
export type MoqTokioWebsocketResolved = rt.Handle<"MoqTokioWebsocketResolved">;
export declare function debug(self: MoqTokioWebsocketResolved): string;
export declare function clone(self: MoqTokioWebsocketResolved): MoqTokioWebsocketResolved;
export declare function enabledGet(self: MoqTokioWebsocketResolved): boolean;
export declare function enabledSet(self: MoqTokioWebsocketResolved, value: boolean): void;
export declare function delayGet(self: MoqTokioWebsocketResolved): rt.Duration;
export declare function delaySet(self: MoqTokioWebsocketResolved, value: rt.Duration): void;

// MuxBinding.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxBinding = rt.Handle<"MuxBinding">;
export declare function new$(broadcast: BroadcastConsumer): MuxBinding;
export declare function broadcast(self: MuxBinding): rt.Call<rt.Result<BroadcastConsumer, MuxError>>;
export { new$ as new };

// MuxCatalogCatalogFormat.ts
export type MuxCatalogCatalogFormat = "Hang" | "Msf";
export declare function debug(self: MuxCatalogCatalogFormat): string;
export declare function default$(): MuxCatalogCatalogFormat;
export declare function DEFAULT(): MuxCatalogCatalogFormat;
export declare function extension(self: MuxCatalogCatalogFormat): string;
export declare function detect(name: string): MuxCatalogCatalogFormat | undefined;
export { default$ as default };

// MuxCatalogConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxCatalogConfigExt } from "./MuxCatalogConfigExt.ts";
import type { MuxCatalogConfigExtra } from "./MuxCatalogConfigExtra.ts";
import type { MuxClock } from "./MuxClock.ts";
export type MuxCatalogConfig = rt.Handle<"MuxCatalogConfig">;
export declare function clone(self: MuxCatalogConfig): MuxCatalogConfig;
export declare function default$(): MuxCatalogConfig;
export declare function withCatalogExtra(self: MuxCatalogConfig, catalog: HangCatalogExtra): MuxCatalogConfigExtra;
export declare function withCatalogExt(self: MuxCatalogConfig, catalog: HangCatalogExt): MuxCatalogConfigExt;
export declare function withCatalogUnit(self: MuxCatalogConfig, catalog: HangCatalog): MuxCatalogConfig;
export declare function withClock(self: MuxCatalogConfig, clock: MuxClock): MuxCatalogConfig;
export declare function withMaxAge(self: MuxCatalogConfig, maxAge: rt.Duration | undefined): MuxCatalogConfig;
export declare function withBandwidth(self: MuxCatalogConfig, bandwidth: BandwidthAllocator): MuxCatalogConfig;
export { default$ as default };

// MuxCatalogConfigExt.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxCatalogConfigExtra } from "./MuxCatalogConfigExtra.ts";
import type { MuxClock } from "./MuxClock.ts";
export type MuxCatalogConfigExt = rt.Handle<"MuxCatalogConfigExt">;
export declare function clone(self: MuxCatalogConfigExt): MuxCatalogConfigExt;
export declare function withCatalogExtra(self: MuxCatalogConfigExt, catalog: HangCatalogExtra): MuxCatalogConfigExtra;
export declare function withCatalogExt(self: MuxCatalogConfigExt, catalog: HangCatalogExt): MuxCatalogConfigExt;
export declare function withCatalogUnit(self: MuxCatalogConfigExt, catalog: HangCatalog): MuxCatalogConfig;
export declare function withClock(self: MuxCatalogConfigExt, clock: MuxClock): MuxCatalogConfigExt;
export declare function withMaxAge(self: MuxCatalogConfigExt, maxAge: rt.Duration | undefined): MuxCatalogConfigExt;
export declare function withBandwidth(self: MuxCatalogConfigExt, bandwidth: BandwidthAllocator): MuxCatalogConfigExt;

// MuxCatalogConfigExtra.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxCatalogConfigExt } from "./MuxCatalogConfigExt.ts";
import type { MuxClock } from "./MuxClock.ts";
export type MuxCatalogConfigExtra = rt.Handle<"MuxCatalogConfigExtra">;
export declare function clone(self: MuxCatalogConfigExtra): MuxCatalogConfigExtra;
export declare function withCatalogExtra(self: MuxCatalogConfigExtra, catalog: HangCatalogExtra): MuxCatalogConfigExtra;
export declare function withCatalogExt(self: MuxCatalogConfigExtra, catalog: HangCatalogExt): MuxCatalogConfigExt;
export declare function withCatalogUnit(self: MuxCatalogConfigExtra, catalog: HangCatalog): MuxCatalogConfig;
export declare function withClock(self: MuxCatalogConfigExtra, clock: MuxClock): MuxCatalogConfigExtra;
export declare function withMaxAge(self: MuxCatalogConfigExtra, maxAge: rt.Duration | undefined): MuxCatalogConfigExtra;
export declare function withBandwidth(self: MuxCatalogConfigExtra, bandwidth: BandwidthAllocator): MuxCatalogConfigExtra;

// MuxCatalogConsumer.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogConsumer = rt.Handle<"MuxCatalogConsumer">;
export declare function new$(broadcast: BroadcastConsumer, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumer, MuxError>>;
export { new$ as new };

// MuxCatalogConsumerExt.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogConsumerExt = rt.Handle<"MuxCatalogConsumerExt">;
export declare function new$(broadcast: BroadcastConsumer, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumerExt, MuxError>>;
export { new$ as new };

// MuxCatalogConsumerExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogConsumerExtra = rt.Handle<"MuxCatalogConsumerExtra">;
export declare function new$(broadcast: BroadcastConsumer, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumerExtra, MuxError>>;
export { new$ as new };

// MuxCatalogEstimate.ts
import * as rt from "./rt.ts";
export type MuxCatalogEstimate = rt.Handle<"MuxCatalogEstimate">;
export declare function debug(self: MuxCatalogEstimate): string;
export declare function clone(self: MuxCatalogEstimate): MuxCatalogEstimate;
export declare function eq(a: MuxCatalogEstimate, b: MuxCatalogEstimate): boolean;
export declare function default$(): MuxCatalogEstimate;
export declare function jitterGet(self: MuxCatalogEstimate): rt.Duration | undefined;
export declare function jitterSet(self: MuxCatalogEstimate, value: rt.Duration | undefined): void;
export declare function bitrateGet(self: MuxCatalogEstimate): rt.u64 | undefined;
export declare function bitrateSet(self: MuxCatalogEstimate, value: rt.u64 | undefined): void;
export declare function delayGet(self: MuxCatalogEstimate): rt.Duration | undefined;
export declare function delaySet(self: MuxCatalogEstimate, value: rt.Duration | undefined): void;
export declare function framerateGet(self: MuxCatalogEstimate): rt.f64 | undefined;
export declare function framerateSet(self: MuxCatalogEstimate, value: rt.f64 | undefined): void;
export declare function withJitter(self: MuxCatalogEstimate, jitter: rt.Duration | undefined): MuxCatalogEstimate;
export declare function withBitrate(self: MuxCatalogEstimate, bitrate: rt.u64 | undefined): MuxCatalogEstimate;
export declare function withDelay(self: MuxCatalogEstimate, delay: rt.Duration | undefined): MuxCatalogEstimate;
export declare function withFramerate(self: MuxCatalogEstimate, framerate: rt.f64 | undefined): MuxCatalogEstimate;
export { default$ as default };

// MuxCatalogGuard.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogGuard = rt.Handle<"MuxCatalogGuard">;
export declare function commit(self: MuxCatalogGuard): rt.Result<undefined, MuxError>;
export declare function videoGet(self: MuxCatalogGuard): HangCatalogVideo;
export declare function videoSet(self: MuxCatalogGuard, value: HangCatalogVideo): void;
export declare function audioGet(self: MuxCatalogGuard): HangCatalogAudio;
export declare function audioSet(self: MuxCatalogGuard, value: HangCatalogAudio): void;
export declare function syncGet(self: MuxCatalogGuard): HangCatalogTimeline;
export declare function syncSet(self: MuxCatalogGuard, value: HangCatalogTimeline): void;
export declare function textGet(self: MuxCatalogGuard): HangCatalogText;
export declare function textSet(self: MuxCatalogGuard, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuard): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuard, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuard): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuard, value: HangCatalogBinary): void;
export declare function statsGet(self: MuxCatalogGuard): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuard, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuard): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuard, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: MuxCatalogGuard): ([string, HangCatalogInclude])[];
export declare function includeSet(self: MuxCatalogGuard, value: ([string, HangCatalogInclude])[]): void;
export declare function skippedGet(self: MuxCatalogGuard): HangCatalogSkipped;
export declare function skippedSet(self: MuxCatalogGuard, value: HangCatalogSkipped): void;
export declare function checkRenditions(self: MuxCatalogGuard): rt.Result<undefined, HangError>;
export declare function withoutExt(self: MuxCatalogGuard): HangCatalog;
export declare function toJson(self: MuxCatalogGuard): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuard): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuard, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogGuardExt.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { MuxContainerTsExt } from "./MuxContainerTsExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogGuardExt = rt.Handle<"MuxCatalogGuardExt">;
export declare function commit(self: MuxCatalogGuardExt): rt.Result<undefined, MuxError>;
export declare function videoGet(self: MuxCatalogGuardExt): HangCatalogVideo;
export declare function videoSet(self: MuxCatalogGuardExt, value: HangCatalogVideo): void;
export declare function audioGet(self: MuxCatalogGuardExt): HangCatalogAudio;
export declare function audioSet(self: MuxCatalogGuardExt, value: HangCatalogAudio): void;
export declare function syncGet(self: MuxCatalogGuardExt): HangCatalogTimeline;
export declare function syncSet(self: MuxCatalogGuardExt, value: HangCatalogTimeline): void;
export declare function textGet(self: MuxCatalogGuardExt): HangCatalogText;
export declare function textSet(self: MuxCatalogGuardExt, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuardExt): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuardExt, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuardExt): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuardExt, value: HangCatalogBinary): void;
export declare function statsGet(self: MuxCatalogGuardExt): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuardExt, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuardExt): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuardExt, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: MuxCatalogGuardExt): ([string, HangCatalogInclude])[];
export declare function includeSet(self: MuxCatalogGuardExt, value: ([string, HangCatalogInclude])[]): void;
export declare function extGet(self: MuxCatalogGuardExt): MuxContainerTsExt;
export declare function extSet(self: MuxCatalogGuardExt, value: MuxContainerTsExt): void;
export declare function skippedGet(self: MuxCatalogGuardExt): HangCatalogSkipped;
export declare function skippedSet(self: MuxCatalogGuardExt, value: HangCatalogSkipped): void;
export declare function checkRenditions(self: MuxCatalogGuardExt): rt.Result<undefined, HangError>;
export declare function withoutExt(self: MuxCatalogGuardExt): HangCatalog;
export declare function toJson(self: MuxCatalogGuardExt): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuardExt): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuardExt, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogGuardExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogInclude } from "./HangCatalogInclude.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogSkipped } from "./HangCatalogSkipped.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogTimeline } from "./HangCatalogTimeline.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { Json } from "./Json.ts";
import type { MuxCatalogHangExtra } from "./MuxCatalogHangExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogGuardExtra = rt.Handle<"MuxCatalogGuardExtra">;
export declare function commit(self: MuxCatalogGuardExtra): rt.Result<undefined, MuxError>;
export declare function setSection(self: MuxCatalogGuardExtra, name: string, value: Json): rt.Result<undefined, MuxError>;
export declare function removeSection(self: MuxCatalogGuardExtra, name: string): Json | undefined;
export declare function videoGet(self: MuxCatalogGuardExtra): HangCatalogVideo;
export declare function videoSet(self: MuxCatalogGuardExtra, value: HangCatalogVideo): void;
export declare function audioGet(self: MuxCatalogGuardExtra): HangCatalogAudio;
export declare function audioSet(self: MuxCatalogGuardExtra, value: HangCatalogAudio): void;
export declare function syncGet(self: MuxCatalogGuardExtra): HangCatalogTimeline;
export declare function syncSet(self: MuxCatalogGuardExtra, value: HangCatalogTimeline): void;
export declare function textGet(self: MuxCatalogGuardExtra): HangCatalogText;
export declare function textSet(self: MuxCatalogGuardExtra, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuardExtra): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuardExtra, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuardExtra): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuardExtra, value: HangCatalogBinary): void;
export declare function statsGet(self: MuxCatalogGuardExtra): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuardExtra, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuardExtra): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuardExtra, value: HangCatalogEcho | undefined): void;
export declare function includeGet(self: MuxCatalogGuardExtra): ([string, HangCatalogInclude])[];
export declare function includeSet(self: MuxCatalogGuardExtra, value: ([string, HangCatalogInclude])[]): void;
export declare function extGet(self: MuxCatalogGuardExtra): MuxCatalogHangExtra;
export declare function extSet(self: MuxCatalogGuardExtra, value: MuxCatalogHangExtra): void;
export declare function skippedGet(self: MuxCatalogGuardExtra): HangCatalogSkipped;
export declare function skippedSet(self: MuxCatalogGuardExtra, value: HangCatalogSkipped): void;
export declare function checkRenditions(self: MuxCatalogGuardExtra): rt.Result<undefined, HangError>;
export declare function withoutExt(self: MuxCatalogGuardExtra): HangCatalog;
export declare function toJson(self: MuxCatalogGuardExtra): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuardExtra): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuardExtra, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogHangConsumer.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type MuxCatalogHangConsumer = rt.Handle<"MuxCatalogHangConsumer">;
export declare function new$(track: TrackConsumer): MuxCatalogHangConsumer;
export declare function next(self: MuxCatalogHangConsumer): rt.Call<rt.Result<HangCatalog | undefined, MuxError>>;
export declare function values(self: MuxCatalogHangConsumer): rt.Stream<rt.Result<HangCatalog, MuxError>>;
export { new$ as new };

// MuxCatalogHangConsumerExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type MuxCatalogHangConsumerExt = rt.Handle<"MuxCatalogHangConsumerExt">;
export declare function new$(track: TrackConsumer): MuxCatalogHangConsumerExt;
export declare function next(self: MuxCatalogHangConsumerExt): rt.Call<rt.Result<HangCatalogExt | undefined, MuxError>>;
export declare function values(self: MuxCatalogHangConsumerExt): rt.Stream<rt.Result<HangCatalogExt, MuxError>>;
export { new$ as new };

// MuxCatalogHangConsumerExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type MuxCatalogHangConsumerExtra = rt.Handle<"MuxCatalogHangConsumerExtra">;
export declare function new$(track: TrackConsumer): MuxCatalogHangConsumerExtra;
export declare function next(self: MuxCatalogHangConsumerExtra): rt.Call<rt.Result<HangCatalogExtra | undefined, MuxError>>;
export declare function values(self: MuxCatalogHangConsumerExtra): rt.Stream<rt.Result<HangCatalogExtra, MuxError>>;
export { new$ as new };

// MuxCatalogHangContainer.ts
import * as rt from "./rt.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxContainerKind } from "./MuxContainerKind.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogHangContainer = rt.Handle<"MuxCatalogHangContainer">;
export declare function new$(container: HangCatalogContainer, kind: MuxContainerKind): rt.Result<MuxCatalogHangContainer, MuxError>;
export { new$ as new };

// MuxCatalogHangExtra.ts
import * as rt from "./rt.ts";
import type { Json } from "./Json.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogHangExtra = rt.Handle<"MuxCatalogHangExtra">;
export declare function debug(self: MuxCatalogHangExtra): string;
export declare function clone(self: MuxCatalogHangExtra): MuxCatalogHangExtra;
export declare function eq(a: MuxCatalogHangExtra, b: MuxCatalogHangExtra): boolean;
export declare function default$(): MuxCatalogHangExtra;
export declare function get(self: MuxCatalogHangExtra, name: string): Json | undefined;
export declare function iter(self: MuxCatalogHangExtra): ([string, Json])[];
export declare function len(self: MuxCatalogHangExtra): rt.u64;
export declare function isEmpty(self: MuxCatalogHangExtra): boolean;
export declare function set(self: MuxCatalogHangExtra, name: string, value: Json): rt.Result<undefined, MuxError>;
export declare function remove(self: MuxCatalogHangExtra, name: string): Json | undefined;
export { default$ as default };

// MuxCatalogMsfConsumer.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogMsfConsumer = rt.Handle<"MuxCatalogMsfConsumer">;
export declare function new$(track: TrackSubscriber): MuxCatalogMsfConsumer;
export declare function next(self: MuxCatalogMsfConsumer): rt.Call<rt.Result<HangCatalog | undefined, MuxError>>;
export declare function values(self: MuxCatalogMsfConsumer): rt.Stream<rt.Result<HangCatalog, MuxError>>;
export { new$ as new };

// MuxCatalogMsfConsumerExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogMsfConsumerExt = rt.Handle<"MuxCatalogMsfConsumerExt">;
export declare function new$(track: TrackSubscriber): MuxCatalogMsfConsumerExt;
export declare function next(self: MuxCatalogMsfConsumerExt): rt.Call<rt.Result<HangCatalogExt | undefined, MuxError>>;
export declare function values(self: MuxCatalogMsfConsumerExt): rt.Stream<rt.Result<HangCatalogExt, MuxError>>;
export { new$ as new };

// MuxCatalogMsfConsumerExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogMsfConsumerExtra = rt.Handle<"MuxCatalogMsfConsumerExtra">;
export declare function new$(track: TrackSubscriber): MuxCatalogMsfConsumerExtra;
export declare function next(self: MuxCatalogMsfConsumerExtra): rt.Call<rt.Result<HangCatalogExtra | undefined, MuxError>>;
export declare function values(self: MuxCatalogMsfConsumerExtra): rt.Stream<rt.Result<HangCatalogExtra, MuxError>>;
export { new$ as new };

// MuxCatalogMsfError.ts
export type MuxCatalogMsfError = {
    tag: "UnresolvableNamespace";
    name: string;
    namespace: string;
} | {
    tag: "InvalidUtf8";
} | {
    tag: "ParseFrame";
} | {
    tag: "MissingCmafInit";
    _0: string;
} | {
    tag: "MalformedInitData";
    _0: string;
} | {
    tag: "MissingVideoCodec";
    _0: string;
} | {
    tag: "MissingAudioCodec";
    _0: string;
} | {
    tag: "UnsupportedPackaging";
    name: string;
    packaging: string;
} | {
    tag: "MissingTextCodec";
    _0: string;
} | {
    tag: "InvalidVideoCodec";
    name: string;
    codec: string;
} | {
    tag: "InvalidAudioCodec";
    name: string;
    codec: string;
} | {
    tag: "MissingAudioParams";
    _0: string;
} | {
    tag: "UnsupportedDerivationPackaging";
    name: string;
    packaging: string;
} | {
    tag: "MalformedAac";
    _0: string;
} | {
    tag: "MalformedOpus";
    _0: string;
} | {
    tag: "MalformedFlac";
    _0: string;
} | {
    tag: "OpusTrailingBytes";
    _0: string;
} | {
    tag: "UnsupportedDerivationCodec";
    _0: string;
} | {
    tag: "MalformedInitSegment";
    _0: string;
} | {
    tag: "MissingInitMoov";
    _0: string;
} | {
    tag: "MissingAudioSampleEntry";
    _0: string;
};
export declare function toString(self: MuxCatalogMsfError): string;
export declare function debug(self: MuxCatalogMsfError): string;

// MuxCatalogProducer.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxCatalogGuard } from "./MuxCatalogGuard.ts";
import type { MuxCatalogHangConsumer } from "./MuxCatalogHangConsumer.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogTimebase } from "./MuxCatalogTimebase.ts";
import type { MuxClock } from "./MuxClock.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxJsonConfig } from "./MuxJsonConfig.ts";
import type { MuxJsonOptions } from "./MuxJsonOptions.ts";
import type { MuxJsonSnapshotJson } from "./MuxJsonSnapshotJson.ts";
import type { MuxJsonStreamJson } from "./MuxJsonStreamJson.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogProducer = rt.Handle<"MuxCatalogProducer">;
export declare function clone(self: MuxCatalogProducer): MuxCatalogProducer;
export declare function new$(broadcast: BroadcastProducer, config: MuxCatalogConfig): rt.Result<MuxCatalogProducer, MuxError>;
export declare function trackInfo(self: MuxCatalogProducer, priority: number): TrackInfo;
export declare function timestamp(self: MuxCatalogProducer, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function mutate(self: MuxCatalogProducer, f: (a0: HangCatalog) => void): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxCatalogProducer): rt.Result<MuxCatalogGuard, MuxError>;
export declare function snapshot(self: MuxCatalogProducer): HangCatalog;
export declare function clock(self: MuxCatalogProducer): MuxClock;
export declare function timebase(self: MuxCatalogProducer): MuxCatalogTimebase;
export declare function reserve(self: MuxCatalogProducer): MuxCatalogReserved;
export declare function videoContainer(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogProducer, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function isClaimedJsonConfig(self: MuxCatalogProducer, name: string): boolean;
export declare function isClaimedBinaryConfig(self: MuxCatalogProducer, name: string): boolean;
export declare function isClaimedVideoConfig(self: MuxCatalogProducer, name: string): boolean;
export declare function isClaimedAudioConfig(self: MuxCatalogProducer, name: string): boolean;
export declare function isClaimedTextConfig(self: MuxCatalogProducer, name: string): boolean;
export declare function stats(self: MuxCatalogProducer): ([string, HangStatsRendition])[];
export declare function jsonSnapshotConfig(self: MuxCatalogProducer, track: TrackProducer, config: MuxJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonSnapshotJsonConfig(self: MuxCatalogProducer, track: TrackProducer, config: HangCatalogJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonStreamConfig(self: MuxCatalogProducer, track: TrackProducer, config: MuxJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function jsonStreamJsonConfig(self: MuxCatalogProducer, track: TrackProducer, config: HangCatalogJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function consume(self: MuxCatalogProducer): rt.Result<MuxCatalogHangConsumer, Error_>;
export declare function abort(self: MuxCatalogProducer, err: Error_): void;
export declare function finish(self: MuxCatalogProducer): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxCatalogProducerExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { MuxCatalogConfigExt } from "./MuxCatalogConfigExt.ts";
import type { MuxCatalogGuardExt } from "./MuxCatalogGuardExt.ts";
import type { MuxCatalogHangConsumerExt } from "./MuxCatalogHangConsumerExt.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogTimebaseExt } from "./MuxCatalogTimebaseExt.ts";
import type { MuxClock } from "./MuxClock.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxJsonConfig } from "./MuxJsonConfig.ts";
import type { MuxJsonOptions } from "./MuxJsonOptions.ts";
import type { MuxJsonSnapshotJson } from "./MuxJsonSnapshotJson.ts";
import type { MuxJsonStreamJson } from "./MuxJsonStreamJson.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogProducerExt = rt.Handle<"MuxCatalogProducerExt">;
export declare function clone(self: MuxCatalogProducerExt): MuxCatalogProducerExt;
export declare function new$(broadcast: BroadcastProducer, config: MuxCatalogConfigExt): rt.Result<MuxCatalogProducerExt, MuxError>;
export declare function trackInfo(self: MuxCatalogProducerExt, priority: number): TrackInfo;
export declare function timestamp(self: MuxCatalogProducerExt, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function mutate(self: MuxCatalogProducerExt, f: (a0: HangCatalogExt) => void): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxCatalogProducerExt): rt.Result<MuxCatalogGuardExt, MuxError>;
export declare function snapshot(self: MuxCatalogProducerExt): HangCatalogExt;
export declare function clock(self: MuxCatalogProducerExt): MuxClock;
export declare function timebase(self: MuxCatalogProducerExt): MuxCatalogTimebaseExt;
export declare function reserve(self: MuxCatalogProducerExt): MuxCatalogReservedExt;
export declare function videoContainer(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogProducerExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function isClaimedJsonConfig(self: MuxCatalogProducerExt, name: string): boolean;
export declare function isClaimedBinaryConfig(self: MuxCatalogProducerExt, name: string): boolean;
export declare function isClaimedVideoConfig(self: MuxCatalogProducerExt, name: string): boolean;
export declare function isClaimedAudioConfig(self: MuxCatalogProducerExt, name: string): boolean;
export declare function isClaimedTextConfig(self: MuxCatalogProducerExt, name: string): boolean;
export declare function stats(self: MuxCatalogProducerExt): ([string, HangStatsRendition])[];
export declare function jsonSnapshotConfig(self: MuxCatalogProducerExt, track: TrackProducer, config: MuxJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonSnapshotJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, config: HangCatalogJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonStreamConfig(self: MuxCatalogProducerExt, track: TrackProducer, config: MuxJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function jsonStreamJsonConfig(self: MuxCatalogProducerExt, track: TrackProducer, config: HangCatalogJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function consume(self: MuxCatalogProducerExt): rt.Result<MuxCatalogHangConsumerExt, Error_>;
export declare function abort(self: MuxCatalogProducerExt, err: Error_): void;
export declare function finish(self: MuxCatalogProducerExt): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxCatalogProducerExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { MuxCatalogConfigExtra } from "./MuxCatalogConfigExtra.ts";
import type { MuxCatalogGuardExtra } from "./MuxCatalogGuardExtra.ts";
import type { MuxCatalogHangConsumerExtra } from "./MuxCatalogHangConsumerExtra.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogTimebaseExtra } from "./MuxCatalogTimebaseExtra.ts";
import type { MuxClock } from "./MuxClock.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxJsonConfig } from "./MuxJsonConfig.ts";
import type { MuxJsonOptions } from "./MuxJsonOptions.ts";
import type { MuxJsonSnapshotJson } from "./MuxJsonSnapshotJson.ts";
import type { MuxJsonStreamJson } from "./MuxJsonStreamJson.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogProducerExtra = rt.Handle<"MuxCatalogProducerExtra">;
export declare function clone(self: MuxCatalogProducerExtra): MuxCatalogProducerExtra;
export declare function new$(broadcast: BroadcastProducer, config: MuxCatalogConfigExtra): rt.Result<MuxCatalogProducerExtra, MuxError>;
export declare function trackInfo(self: MuxCatalogProducerExtra, priority: number): TrackInfo;
export declare function timestamp(self: MuxCatalogProducerExtra, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function mutate(self: MuxCatalogProducerExtra, f: (a0: HangCatalogExtra) => void): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxCatalogProducerExtra): rt.Result<MuxCatalogGuardExtra, MuxError>;
export declare function snapshot(self: MuxCatalogProducerExtra): HangCatalogExtra;
export declare function clock(self: MuxCatalogProducerExtra): MuxClock;
export declare function timebase(self: MuxCatalogProducerExtra): MuxCatalogTimebaseExtra;
export declare function reserve(self: MuxCatalogProducerExtra): MuxCatalogReservedExtra;
export declare function videoContainer(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogProducerExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function isClaimedJsonConfig(self: MuxCatalogProducerExtra, name: string): boolean;
export declare function isClaimedBinaryConfig(self: MuxCatalogProducerExtra, name: string): boolean;
export declare function isClaimedVideoConfig(self: MuxCatalogProducerExtra, name: string): boolean;
export declare function isClaimedAudioConfig(self: MuxCatalogProducerExtra, name: string): boolean;
export declare function isClaimedTextConfig(self: MuxCatalogProducerExtra, name: string): boolean;
export declare function stats(self: MuxCatalogProducerExtra): ([string, HangStatsRendition])[];
export declare function jsonSnapshotConfig(self: MuxCatalogProducerExtra, track: TrackProducer, config: MuxJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonSnapshotJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, config: HangCatalogJsonConfig, options: MuxJsonOptions): rt.Result<MuxJsonSnapshotJson, MuxError>;
export declare function jsonStreamConfig(self: MuxCatalogProducerExtra, track: TrackProducer, config: MuxJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function jsonStreamJsonConfig(self: MuxCatalogProducerExtra, track: TrackProducer, config: HangCatalogJsonConfig): rt.Result<MuxJsonStreamJson, MuxError>;
export declare function consume(self: MuxCatalogProducerExtra): rt.Result<MuxCatalogHangConsumerExtra, Error_>;
export declare function abort(self: MuxCatalogProducerExtra, err: Error_): void;
export declare function finish(self: MuxCatalogProducerExtra): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxCatalogReserved.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogReserved = rt.Handle<"MuxCatalogReserved">;
export declare function clone(self: MuxCatalogReserved): MuxCatalogReserved;
export declare function withInterval(self: MuxCatalogReserved, interval: rt.Duration | undefined): MuxCatalogReserved;
export declare function trackInfo(self: MuxCatalogReserved, priority: number): TrackInfo;
export declare function videoContainer(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogReserved, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function timestamp(self: MuxCatalogReserved, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;

// MuxCatalogReservedExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogReservedExt = rt.Handle<"MuxCatalogReservedExt">;
export declare function clone(self: MuxCatalogReservedExt): MuxCatalogReservedExt;
export declare function withInterval(self: MuxCatalogReservedExt, interval: rt.Duration | undefined): MuxCatalogReservedExt;
export declare function trackInfo(self: MuxCatalogReservedExt, priority: number): TrackInfo;
export declare function videoContainer(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogReservedExt, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function timestamp(self: MuxCatalogReservedExt, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;

// MuxCatalogReservedExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerProducerContainerAudioConfig } from "./MuxContainerProducerContainerAudioConfig.ts";
import type { MuxContainerProducerContainerBinaryConfig } from "./MuxContainerProducerContainerBinaryConfig.ts";
import type { MuxContainerProducerContainerJsonConfig } from "./MuxContainerProducerContainerJsonConfig.ts";
import type { MuxContainerProducerContainerTextConfig } from "./MuxContainerProducerContainerTextConfig.ts";
import type { MuxContainerProducerContainerVideoConfig } from "./MuxContainerProducerContainerVideoConfig.ts";
import type { MuxContainerProducerFmp4WireAudioConfig } from "./MuxContainerProducerFmp4WireAudioConfig.ts";
import type { MuxContainerProducerFmp4WireBinaryConfig } from "./MuxContainerProducerFmp4WireBinaryConfig.ts";
import type { MuxContainerProducerFmp4WireJsonConfig } from "./MuxContainerProducerFmp4WireJsonConfig.ts";
import type { MuxContainerProducerFmp4WireTextConfig } from "./MuxContainerProducerFmp4WireTextConfig.ts";
import type { MuxContainerProducerFmp4WireVideoConfig } from "./MuxContainerProducerFmp4WireVideoConfig.ts";
import type { MuxContainerProducerLegacyWireAudioConfig } from "./MuxContainerProducerLegacyWireAudioConfig.ts";
import type { MuxContainerProducerLegacyWireBinaryConfig } from "./MuxContainerProducerLegacyWireBinaryConfig.ts";
import type { MuxContainerProducerLegacyWireJsonConfig } from "./MuxContainerProducerLegacyWireJsonConfig.ts";
import type { MuxContainerProducerLegacyWireTextConfig } from "./MuxContainerProducerLegacyWireTextConfig.ts";
import type { MuxContainerProducerLegacyWireVideoConfig } from "./MuxContainerProducerLegacyWireVideoConfig.ts";
import type { MuxContainerProducerLocWireAudioConfig } from "./MuxContainerProducerLocWireAudioConfig.ts";
import type { MuxContainerProducerLocWireBinaryConfig } from "./MuxContainerProducerLocWireBinaryConfig.ts";
import type { MuxContainerProducerLocWireJsonConfig } from "./MuxContainerProducerLocWireJsonConfig.ts";
import type { MuxContainerProducerLocWireTextConfig } from "./MuxContainerProducerLocWireTextConfig.ts";
import type { MuxContainerProducerLocWireVideoConfig } from "./MuxContainerProducerLocWireVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCatalogReservedExtra = rt.Handle<"MuxCatalogReservedExtra">;
export declare function clone(self: MuxCatalogReservedExtra): MuxCatalogReservedExtra;
export declare function withInterval(self: MuxCatalogReservedExtra, interval: rt.Duration | undefined): MuxCatalogReservedExtra;
export declare function trackInfo(self: MuxCatalogReservedExtra, priority: number): TrackInfo;
export declare function videoContainer(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function videoFmp4Wire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function videoLegacyWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function videoLocWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function audioContainer(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function audioFmp4Wire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function audioLegacyWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function audioLocWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function textContainer(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function textFmp4Wire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function textLegacyWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function textLocWire(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function trackContainerJsonConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerContainerJsonConfig, MuxError>;
export declare function trackContainerBinaryConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerContainerBinaryConfig, MuxError>;
export declare function trackContainerVideoConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerContainerVideoConfig, MuxError>;
export declare function trackContainerAudioConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerContainerAudioConfig, MuxError>;
export declare function trackContainerTextConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxCatalogHangContainer, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerContainerTextConfig, MuxError>;
export declare function trackFmp4WireJsonConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerFmp4WireJsonConfig, MuxError>;
export declare function trackFmp4WireBinaryConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerFmp4WireBinaryConfig, MuxError>;
export declare function trackFmp4WireVideoConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerFmp4WireVideoConfig, MuxError>;
export declare function trackFmp4WireAudioConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerFmp4WireAudioConfig, MuxError>;
export declare function trackFmp4WireTextConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerFmp4Wire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerFmp4WireTextConfig, MuxError>;
export declare function trackLegacyWireJsonConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLegacyWireJsonConfig, MuxError>;
export declare function trackLegacyWireBinaryConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLegacyWireBinaryConfig, MuxError>;
export declare function trackLegacyWireVideoConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLegacyWireVideoConfig, MuxError>;
export declare function trackLegacyWireAudioConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLegacyWireAudioConfig, MuxError>;
export declare function trackLegacyWireTextConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLegacyWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLegacyWireTextConfig, MuxError>;
export declare function trackLocWireJsonConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogJsonConfig | undefined): rt.Result<MuxContainerProducerLocWireJsonConfig, MuxError>;
export declare function trackLocWireBinaryConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogBinaryConfig | undefined): rt.Result<MuxContainerProducerLocWireBinaryConfig, MuxError>;
export declare function trackLocWireVideoConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogVideoConfig | undefined): rt.Result<MuxContainerProducerLocWireVideoConfig, MuxError>;
export declare function trackLocWireAudioConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogAudioConfig | undefined): rt.Result<MuxContainerProducerLocWireAudioConfig, MuxError>;
export declare function trackLocWireTextConfig(self: MuxCatalogReservedExtra, track: TrackProducer, container: MuxContainerLocWire, config: HangCatalogTextConfig | undefined): rt.Result<MuxContainerProducerLocWireTextConfig, MuxError>;
export declare function timestamp(self: MuxCatalogReservedExtra, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;

// MuxCatalogTimebase.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCatalogTimebase = rt.Handle<"MuxCatalogTimebase">;
export declare function clone(self: MuxCatalogTimebase): MuxCatalogTimebase;
export declare function reserve(self: MuxCatalogTimebase): MuxCatalogReserved;
export declare function place(self: MuxCatalogTimebase, pts: Timestamp, wall: ClockWall): rt.Result<undefined, MuxError>;

// MuxCatalogTimebaseExt.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCatalogTimebaseExt = rt.Handle<"MuxCatalogTimebaseExt">;
export declare function clone(self: MuxCatalogTimebaseExt): MuxCatalogTimebaseExt;
export declare function reserve(self: MuxCatalogTimebaseExt): MuxCatalogReservedExt;
export declare function place(self: MuxCatalogTimebaseExt, pts: Timestamp, wall: ClockWall): rt.Result<undefined, MuxError>;

// MuxCatalogTimebaseExtra.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCatalogTimebaseExtra = rt.Handle<"MuxCatalogTimebaseExtra">;
export declare function clone(self: MuxCatalogTimebaseExtra): MuxCatalogTimebaseExtra;
export declare function reserve(self: MuxCatalogTimebaseExtra): MuxCatalogReservedExtra;
export declare function place(self: MuxCatalogTimebaseExtra, pts: Timestamp, wall: ClockWall): rt.Result<undefined, MuxError>;

// MuxCatalogVideoHint.ts
import * as rt from "./rt.ts";
import type { HangCatalogColor } from "./HangCatalogColor.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { HangCatalogVideoCodec } from "./HangCatalogVideoCodec.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
export type MuxCatalogVideoHint = rt.Handle<"MuxCatalogVideoHint">;
export declare function debug(self: MuxCatalogVideoHint): string;
export declare function clone(self: MuxCatalogVideoHint): MuxCatalogVideoHint;
export declare function eq(a: MuxCatalogVideoHint, b: MuxCatalogVideoHint): boolean;
export declare function default$(): MuxCatalogVideoHint;
export declare function codecGet(self: MuxCatalogVideoHint): HangCatalogVideoCodec | undefined;
export declare function codecSet(self: MuxCatalogVideoHint, value: HangCatalogVideoCodec | undefined): void;
export declare function codedWidthGet(self: MuxCatalogVideoHint): number | undefined;
export declare function codedWidthSet(self: MuxCatalogVideoHint, value: number | undefined): void;
export declare function codedHeightGet(self: MuxCatalogVideoHint): number | undefined;
export declare function codedHeightSet(self: MuxCatalogVideoHint, value: number | undefined): void;
export declare function displayAspectWidthGet(self: MuxCatalogVideoHint): number | undefined;
export declare function displayAspectWidthSet(self: MuxCatalogVideoHint, value: number | undefined): void;
export declare function displayAspectHeightGet(self: MuxCatalogVideoHint): number | undefined;
export declare function displayAspectHeightSet(self: MuxCatalogVideoHint, value: number | undefined): void;
export declare function colorGet(self: MuxCatalogVideoHint): HangCatalogColor | undefined;
export declare function colorSet(self: MuxCatalogVideoHint, value: HangCatalogColor | undefined): void;
export declare function bitrateGet(self: MuxCatalogVideoHint): rt.u64 | undefined;
export declare function bitrateSet(self: MuxCatalogVideoHint, value: rt.u64 | undefined): void;
export declare function framerateGet(self: MuxCatalogVideoHint): rt.f64 | undefined;
export declare function framerateSet(self: MuxCatalogVideoHint, value: rt.f64 | undefined): void;
export declare function reorderGet(self: MuxCatalogVideoHint): number | undefined;
export declare function reorderSet(self: MuxCatalogVideoHint, value: number | undefined): void;
export declare function optimizeForLatencyGet(self: MuxCatalogVideoHint): boolean | undefined;
export declare function optimizeForLatencySet(self: MuxCatalogVideoHint, value: boolean | undefined): void;
export declare function jitterGet(self: MuxCatalogVideoHint): rt.Duration | undefined;
export declare function jitterSet(self: MuxCatalogVideoHint, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxCatalogVideoHint): rt.Duration | undefined;
export declare function delaySet(self: MuxCatalogVideoHint, value: rt.Duration | undefined): void;
export declare function containerGet(self: MuxCatalogVideoHint): HangCatalogContainer;
export declare function containerSet(self: MuxCatalogVideoHint, value: HangCatalogContainer): void;
export declare function preferenceGet(self: MuxCatalogVideoHint): number;
export declare function preferenceSet(self: MuxCatalogVideoHint, value: number): void;
export declare function apply(self: MuxCatalogVideoHint, config: HangCatalogVideoConfig): void;
export { default$ as default };

// MuxClock.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { HangCatalogClock } from "./HangCatalogClock.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxClock = rt.Handle<"MuxClock">;
export declare function debug(self: MuxClock): string;
export declare function clone(self: MuxClock): MuxClock;
export declare function eq(a: MuxClock, b: MuxClock): boolean;
export declare function default$(): MuxClock;
export declare function TIMESCALE(): Timescale;
export declare function new$(): MuxClock;
export declare function at(epoch: ClockInstant, wall: ClockWall): rt.Result<MuxClock, MuxError>;
export declare function now(self: MuxClock): Timestamp;
export declare function capture(self: MuxClock, at: ClockInstant): rt.Result<Timestamp, MuxError>;
export declare function timescale(self: MuxClock): Timescale;
export declare function wall(self: MuxClock): HangCatalogClock | undefined;
export declare function wallClock(self: MuxClock, pts: Timestamp): rt.Result<ClockWall, MuxError>;
export { default$ as default, new$ as new };

// MuxCodecAacAdtsError.ts
import * as rt from "./rt.ts";
export type MuxCodecAacAdtsError = {
    tag: "Truncated";
} | {
    tag: "Syncword";
} | {
    tag: "SampleRate";
} | {
    tag: "FrameLength";
} | {
    tag: "PacketLength";
} | {
    tag: "ObjectType";
    _0: number;
} | {
    tag: "Rate";
    _0: number;
} | {
    tag: "ChannelConfig";
    _0: number;
} | {
    tag: "TooLarge";
    _0: rt.u64;
};
export declare function toString(self: MuxCodecAacAdtsError): string;
export declare function debug(self: MuxCodecAacAdtsError): string;
export declare function eq(a: MuxCodecAacAdtsError, b: MuxCodecAacAdtsError): boolean;

// MuxCodecAacAdtsHeader.ts
import * as rt from "./rt.ts";
import type { MuxCodecAacAdtsError } from "./MuxCodecAacAdtsError.ts";
export type MuxCodecAacAdtsHeader = rt.Handle<"MuxCodecAacAdtsHeader">;
export declare function debug(self: MuxCodecAacAdtsHeader): string;
export declare function clone(self: MuxCodecAacAdtsHeader): MuxCodecAacAdtsHeader;
export declare function objectTypeGet(self: MuxCodecAacAdtsHeader): number;
export declare function objectTypeSet(self: MuxCodecAacAdtsHeader, value: number): void;
export declare function sampleRateGet(self: MuxCodecAacAdtsHeader): number;
export declare function sampleRateSet(self: MuxCodecAacAdtsHeader, value: number): void;
export declare function channelConfigGet(self: MuxCodecAacAdtsHeader): number;
export declare function channelConfigSet(self: MuxCodecAacAdtsHeader, value: number): void;
export declare function blocksGet(self: MuxCodecAacAdtsHeader): number;
export declare function blocksSet(self: MuxCodecAacAdtsHeader, value: number): void;
export declare function parse(data: Uint8Array | rt.Bytes): rt.Result<MuxCodecAacAdtsHeader, MuxCodecAacAdtsError>;
export declare function payload(self: MuxCodecAacAdtsHeader, packet: Uint8Array | rt.Bytes): rt.Result<rt.Bytes, MuxCodecAacAdtsError>;

// MuxCodecAacConfig.ts
import * as rt from "./rt.ts";
import type { MuxCodecAacError } from "./MuxCodecAacError.ts";
export type MuxCodecAacConfig = rt.Handle<"MuxCodecAacConfig">;
export declare function profileGet(self: MuxCodecAacConfig): number;
export declare function profileSet(self: MuxCodecAacConfig, value: number): void;
export declare function sampleRateGet(self: MuxCodecAacConfig): number;
export declare function sampleRateSet(self: MuxCodecAacConfig, value: number): void;
export declare function channelCountGet(self: MuxCodecAacConfig): number;
export declare function channelCountSet(self: MuxCodecAacConfig, value: number): void;
export declare function new$(profile: number, sampleRate: number, channelCount: number): MuxCodecAacConfig;
export declare function parse(buf: rt.Place<Uint8Array>): rt.Result<MuxCodecAacConfig, MuxCodecAacError>;
export declare function encode(self: MuxCodecAacConfig): rt.Result<rt.Bytes, MuxCodecAacError>;
export { new$ as new };

// MuxCodecAacError.ts
import * as rt from "./rt.ts";
export type MuxCodecAacError = rt.Handle<"MuxCodecAacError">;
export declare function toString(self: MuxCodecAacError): string;
export declare function debug(self: MuxCodecAacError): string;
export declare function clone(self: MuxCodecAacError): MuxCodecAacError;

// MuxCodecAacImport.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFinishedContainerAudioConfig } from "./MuxContainerFinishedContainerAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecAacImport = rt.Handle<"MuxCodecAacImport">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecAacImport, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecAacImport, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecAacImport, MuxError>;
export declare function name(self: MuxCodecAacImport): string;
export declare function demand(self: MuxCodecAacImport): TrackDemand;
export declare function finish(self: MuxCodecAacImport): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function abort(self: MuxCodecAacImport, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecAacImport, err: Error_): rt.Result<undefined, MuxCodecAacImport>;
export declare function cut(self: MuxCodecAacImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecAacImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecAacImport, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecAacImport, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;

// MuxCodecAnnexbError.ts
import * as rt from "./rt.ts";
export type MuxCodecAnnexbError = rt.Handle<"MuxCodecAnnexbError">;
export declare function toString(self: MuxCodecAnnexbError): string;
export declare function debug(self: MuxCodecAnnexbError): string;
export declare function clone(self: MuxCodecAnnexbError): MuxCodecAnnexbError;

// MuxCodecAv1Import.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecAv1Import = rt.Handle<"MuxCodecAv1Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, hint: MuxCatalogVideoHint): rt.Result<MuxCodecAv1Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, hint: MuxCatalogVideoHint): rt.Result<MuxCodecAv1Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, hint: MuxCatalogVideoHint): rt.Result<MuxCodecAv1Import, MuxError>;
export declare function initialize(self: MuxCodecAv1Import, buf: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxCodecAv1Import): TrackDemand;
export declare function finish(self: MuxCodecAv1Import): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxCodecAv1Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecAv1Import, err: Error_): rt.Result<undefined, MuxCodecAv1Import>;
export declare function cut(self: MuxCodecAv1Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecAv1Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecAv1Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecAv1Import, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecAv1Import, frames: MuxContainerFrame[]): rt.Result<undefined, MuxError>;

// MuxCodecAv1Split.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCodecAv1Split = rt.Handle<"MuxCodecAv1Split">;
export declare function default$(): MuxCodecAv1Split;
export declare function new$(): MuxCodecAv1Split;
export declare function decode(self: MuxCodecAv1Split, data: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function flush(self: MuxCodecAv1Split, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function reset(self: MuxCodecAv1Split): void;
export { default$ as default, new$ as new };

// MuxCodecFlacConfig.ts
import * as rt from "./rt.ts";
import type { MuxCodecFlacError } from "./MuxCodecFlacError.ts";
export type MuxCodecFlacConfig = {
    minBlockSize: number;
    maxBlockSize: number;
    minFrameSize: number;
    maxFrameSize: number;
    sampleRate: number;
    channelCount: number;
    bitsPerSample: number;
    totalSamples: rt.u64;
    md5: Uint8Array;
};
export declare function debug(self: MuxCodecFlacConfig): string;
export declare function eq(a: MuxCodecFlacConfig, b: MuxCodecFlacConfig): boolean;
export declare function parseStreamInfo(buf: rt.Place<Uint8Array>): rt.Result<MuxCodecFlacConfig, MuxCodecFlacError>;
export declare function parse(buf: rt.Place<Uint8Array>): rt.Result<MuxCodecFlacConfig, MuxCodecFlacError>;
export declare function encodeStreamInfo(self: MuxCodecFlacConfig): rt.Bytes;
export declare function description(self: MuxCodecFlacConfig): rt.Bytes;

// MuxCodecFlacError.ts
export type MuxCodecFlacError = "Short" | "InvalidMarker" | "MissingStreamInfo";
export declare function toString(self: MuxCodecFlacError): string;
export declare function debug(self: MuxCodecFlacError): string;

// MuxCodecFlacImport.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFinishedContainerAudioConfig } from "./MuxContainerFinishedContainerAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecFlacImport = rt.Handle<"MuxCodecFlacImport">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function demand(self: MuxCodecFlacImport): TrackDemand;
export declare function finish(self: MuxCodecFlacImport): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function abort(self: MuxCodecFlacImport, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecFlacImport, err: Error_): rt.Result<undefined, MuxCodecFlacImport>;
export declare function cut(self: MuxCodecFlacImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecFlacImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecFlacImport, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecFlacImport, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;

// MuxCodecH264Avc1.ts
import * as rt from "./rt.ts";
import type { MuxCodecH264Error } from "./MuxCodecH264Error.ts";
export type MuxCodecH264Avc1 = rt.Handle<"MuxCodecH264Avc1">;
export declare function default$(): MuxCodecH264Avc1;
export declare function new$(): MuxCodecH264Avc1;
export declare function avcc(self: MuxCodecH264Avc1): rt.Bytes | undefined;
export declare function transform(self: MuxCodecH264Avc1, payload: Uint8Array | rt.Bytes): rt.Result<rt.Bytes | undefined, MuxCodecH264Error>;
export { default$ as default, new$ as new };

// MuxCodecH264Avcc.ts
import * as rt from "./rt.ts";
import type { MuxCodecH264Error } from "./MuxCodecH264Error.ts";
export type MuxCodecH264Avcc = rt.Handle<"MuxCodecH264Avcc">;
export declare function debug(self: MuxCodecH264Avcc): string;
export declare function clone(self: MuxCodecH264Avcc): MuxCodecH264Avcc;
export declare function profileGet(self: MuxCodecH264Avcc): number;
export declare function profileSet(self: MuxCodecH264Avcc, value: number): void;
export declare function constraintsGet(self: MuxCodecH264Avcc): number;
export declare function constraintsSet(self: MuxCodecH264Avcc, value: number): void;
export declare function levelGet(self: MuxCodecH264Avcc): number;
export declare function levelSet(self: MuxCodecH264Avcc, value: number): void;
export declare function lengthSizeGet(self: MuxCodecH264Avcc): rt.u64;
export declare function lengthSizeSet(self: MuxCodecH264Avcc, value: rt.u64): void;
export declare function spsGet(self: MuxCodecH264Avcc): Uint8Array[];
export declare function spsSet(self: MuxCodecH264Avcc, value: Uint8Array[]): void;
export declare function ppsGet(self: MuxCodecH264Avcc): Uint8Array[];
export declare function ppsSet(self: MuxCodecH264Avcc, value: Uint8Array[]): void;
export declare function codedWidthGet(self: MuxCodecH264Avcc): number | undefined;
export declare function codedWidthSet(self: MuxCodecH264Avcc, value: number | undefined): void;
export declare function codedHeightGet(self: MuxCodecH264Avcc): number | undefined;
export declare function codedHeightSet(self: MuxCodecH264Avcc, value: number | undefined): void;
export declare function parse(avcc: Uint8Array | rt.Bytes): rt.Result<MuxCodecH264Avcc, MuxCodecH264Error>;

// MuxCodecH264Error.ts
import * as rt from "./rt.ts";
export type MuxCodecH264Error = rt.Handle<"MuxCodecH264Error">;
export declare function toString(self: MuxCodecH264Error): string;
export declare function debug(self: MuxCodecH264Error): string;
export declare function clone(self: MuxCodecH264Error): MuxCodecH264Error;

// MuxCodecH264Import.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecH264Import = rt.Handle<"MuxCodecH264Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH264Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH264Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH264Import, MuxError>;
export declare function initialize(self: MuxCodecH264Import, buf: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function name(self: MuxCodecH264Import): string;
export declare function demand(self: MuxCodecH264Import): TrackDemand;
export declare function track(self: MuxCodecH264Import): TrackProducer;
export declare function finish(self: MuxCodecH264Import): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxCodecH264Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecH264Import, err: Error_): rt.Result<undefined, MuxCodecH264Import>;
export declare function cut(self: MuxCodecH264Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecH264Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecH264Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function reportSkipped(self: MuxCodecH264Import, frames: rt.u64): void;
export declare function reportBitrate(self: MuxCodecH264Import, bitrate: BandwidthRate): void;
export declare function seek(self: MuxCodecH264Import, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function observeReorder(self: MuxCodecH264Import, reorder: Timestamp): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecH264Import, frames: MuxContainerFrame[]): rt.Result<undefined, MuxError>;

// MuxCodecH264Split.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCodecH264Split = rt.Handle<"MuxCodecH264Split">;
export declare function default$(): MuxCodecH264Split;
export declare function new$(): MuxCodecH264Split;
export declare function decode(self: MuxCodecH264Split, data: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function flush(self: MuxCodecH264Split, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function reset(self: MuxCodecH264Split): void;
export { default$ as default, new$ as new };

// MuxCodecH264Sps.ts
import * as rt from "./rt.ts";
import type { MuxCodecH264Error } from "./MuxCodecH264Error.ts";
export type MuxCodecH264Sps = {
    profile: number;
    constraints: number;
    level: number;
    codedWidth: number;
    codedHeight: number;
};
export declare function debug(self: MuxCodecH264Sps): string;
export declare function parse(nal: Uint8Array | rt.Bytes): rt.Result<MuxCodecH264Sps, MuxCodecH264Error>;

// MuxCodecH265Error.ts
import * as rt from "./rt.ts";
export type MuxCodecH265Error = rt.Handle<"MuxCodecH265Error">;
export declare function toString(self: MuxCodecH265Error): string;
export declare function debug(self: MuxCodecH265Error): string;
export declare function clone(self: MuxCodecH265Error): MuxCodecH265Error;

// MuxCodecH265Hvc1.ts
import * as rt from "./rt.ts";
import type { MuxCodecH265Error } from "./MuxCodecH265Error.ts";
export type MuxCodecH265Hvc1 = rt.Handle<"MuxCodecH265Hvc1">;
export declare function default$(): MuxCodecH265Hvc1;
export declare function new$(): MuxCodecH265Hvc1;
export declare function hvcc(self: MuxCodecH265Hvc1): rt.Bytes | undefined;
export declare function transform(self: MuxCodecH265Hvc1, payload: Uint8Array | rt.Bytes): rt.Result<rt.Bytes | undefined, MuxCodecH265Error>;
export { default$ as default, new$ as new };

// MuxCodecH265Hvcc.ts
import * as rt from "./rt.ts";
import type { MuxCodecH265Error } from "./MuxCodecH265Error.ts";
export type MuxCodecH265Hvcc = rt.Handle<"MuxCodecH265Hvcc">;
export declare function debug(self: MuxCodecH265Hvcc): string;
export declare function clone(self: MuxCodecH265Hvcc): MuxCodecH265Hvcc;
export declare function lengthSizeGet(self: MuxCodecH265Hvcc): rt.u64;
export declare function lengthSizeSet(self: MuxCodecH265Hvcc, value: rt.u64): void;
export declare function vpsGet(self: MuxCodecH265Hvcc): Uint8Array[];
export declare function vpsSet(self: MuxCodecH265Hvcc, value: Uint8Array[]): void;
export declare function spsGet(self: MuxCodecH265Hvcc): Uint8Array[];
export declare function spsSet(self: MuxCodecH265Hvcc, value: Uint8Array[]): void;
export declare function ppsGet(self: MuxCodecH265Hvcc): Uint8Array[];
export declare function ppsSet(self: MuxCodecH265Hvcc, value: Uint8Array[]): void;
export declare function parse(hvcc: Uint8Array | rt.Bytes): rt.Result<MuxCodecH265Hvcc, MuxCodecH265Error>;

// MuxCodecH265Import.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecH265Import = rt.Handle<"MuxCodecH265Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH265Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH265Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, hint: MuxCatalogVideoHint): rt.Result<MuxCodecH265Import, MuxError>;
export declare function initialize(self: MuxCodecH265Import, buf: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function name(self: MuxCodecH265Import): string;
export declare function demand(self: MuxCodecH265Import): TrackDemand;
export declare function track(self: MuxCodecH265Import): TrackProducer;
export declare function finish(self: MuxCodecH265Import): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxCodecH265Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecH265Import, err: Error_): rt.Result<undefined, MuxCodecH265Import>;
export declare function cut(self: MuxCodecH265Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecH265Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecH265Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function reportSkipped(self: MuxCodecH265Import, frames: rt.u64): void;
export declare function reportBitrate(self: MuxCodecH265Import, bitrate: BandwidthRate): void;
export declare function seek(self: MuxCodecH265Import, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function observeReorder(self: MuxCodecH265Import, reorder: Timestamp): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecH265Import, frames: MuxContainerFrame[]): rt.Result<undefined, MuxError>;

// MuxCodecH265Split.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCodecH265Split = rt.Handle<"MuxCodecH265Split">;
export declare function default$(): MuxCodecH265Split;
export declare function new$(): MuxCodecH265Split;
export declare function decode(self: MuxCodecH265Split, data: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function flush(self: MuxCodecH265Split, pts: Timestamp | undefined): rt.Result<MuxContainerFrame[], MuxError>;
export declare function reset(self: MuxCodecH265Split): void;
export { default$ as default, new$ as new };

// MuxCodecH265Sps.ts
import * as rt from "./rt.ts";
import type { MuxCodecH265Error } from "./MuxCodecH265Error.ts";
import type { MuxCodecH265VideoSignal } from "./MuxCodecH265VideoSignal.ts";
export type MuxCodecH265Sps = rt.Handle<"MuxCodecH265Sps">;
export declare function debug(self: MuxCodecH265Sps): string;
export declare function clone(self: MuxCodecH265Sps): MuxCodecH265Sps;
export declare function parse(nal: Uint8Array | rt.Bytes): rt.Result<MuxCodecH265Sps, MuxCodecH265Error>;
export declare function size(self: MuxCodecH265Sps): [number, number];
export declare function videoSignal(self: MuxCodecH265Sps): MuxCodecH265VideoSignal | undefined;

// MuxCodecH265VideoSignal.ts
export type MuxCodecH265VideoSignal = {
    primaries: number;
    transfer: number;
    matrix: number;
    fullRange: boolean;
};
export declare function debug(self: MuxCodecH265VideoSignal): string;
export declare function eq(a: MuxCodecH265VideoSignal, b: MuxCodecH265VideoSignal): boolean;

// MuxCodecMp3Config.ts
import * as rt from "./rt.ts";
import type { MuxCodecMp3Error } from "./MuxCodecMp3Error.ts";
export type MuxCodecMp3Config = {
    sampleRate: number;
    channelCount: number;
};
export declare function parse(data: Uint8Array | rt.Bytes): rt.Result<MuxCodecMp3Config, MuxCodecMp3Error>;

// MuxCodecMp3Error.ts
export type MuxCodecMp3Error = "HeaderTooShort" | "MissingSync" | "ReservedVersion" | "NotLayer3" | "ReservedSampleRate";
export declare function toString(self: MuxCodecMp3Error): string;
export declare function debug(self: MuxCodecMp3Error): string;

// MuxCodecMp3Import.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFinishedContainerAudioConfig } from "./MuxContainerFinishedContainerAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecMp3Import = rt.Handle<"MuxCodecMp3Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function demand(self: MuxCodecMp3Import): TrackDemand;
export declare function finish(self: MuxCodecMp3Import): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function abort(self: MuxCodecMp3Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecMp3Import, err: Error_): rt.Result<undefined, MuxCodecMp3Import>;
export declare function cut(self: MuxCodecMp3Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecMp3Import): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecMp3Import, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecMp3Import, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;

// MuxCodecOpusConfig.ts
import * as rt from "./rt.ts";
import type { MuxCodecOpusError } from "./MuxCodecOpusError.ts";
import type { MuxCodecOpusMapping } from "./MuxCodecOpusMapping.ts";
export type MuxCodecOpusConfig = rt.Handle<"MuxCodecOpusConfig">;
export declare function debug(self: MuxCodecOpusConfig): string;
export declare function clone(self: MuxCodecOpusConfig): MuxCodecOpusConfig;
export declare function eq(a: MuxCodecOpusConfig, b: MuxCodecOpusConfig): boolean;
export declare function sampleRateGet(self: MuxCodecOpusConfig): number;
export declare function sampleRateSet(self: MuxCodecOpusConfig, value: number): void;
export declare function channelCountGet(self: MuxCodecOpusConfig): number;
export declare function channelCountSet(self: MuxCodecOpusConfig, value: number): void;
export declare function preSkipGet(self: MuxCodecOpusConfig): number;
export declare function preSkipSet(self: MuxCodecOpusConfig, value: number): void;
export declare function outputGainGet(self: MuxCodecOpusConfig): number;
export declare function outputGainSet(self: MuxCodecOpusConfig, value: number): void;
export declare function mappingGet(self: MuxCodecOpusConfig): MuxCodecOpusMapping | undefined;
export declare function mappingSet(self: MuxCodecOpusConfig, value: MuxCodecOpusMapping | undefined): void;
export declare function new$(sampleRate: number, channelCount: number): MuxCodecOpusConfig;
export declare function withPreSkip(self: MuxCodecOpusConfig, preSkip: number): MuxCodecOpusConfig;
export declare function parse(buf: rt.Place<Uint8Array>): rt.Result<MuxCodecOpusConfig, MuxCodecOpusError>;
export declare function encode(self: MuxCodecOpusConfig): rt.Result<rt.Bytes, MuxCodecOpusError>;
export { new$ as new };

// MuxCodecOpusError.ts
export type MuxCodecOpusError = {
    tag: "HeadTooShort";
} | {
    tag: "InvalidSignature";
} | {
    tag: "UnsupportedVersion";
    _0: number;
} | {
    tag: "UnsupportedChannelCount";
    _0: number;
} | {
    tag: "MappingTableTooShort";
} | {
    tag: "InvalidMappingTable";
} | {
    tag: "UnsupportedMappingFamily";
    _0: number;
};
export declare function toString(self: MuxCodecOpusError): string;
export declare function debug(self: MuxCodecOpusError): string;

// MuxCodecOpusImport.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFinishedContainerAudioConfig } from "./MuxContainerFinishedContainerAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecOpusImport = rt.Handle<"MuxCodecOpusImport">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecOpusImport, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecOpusImport, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecOpusImport, MuxError>;
export declare function name(self: MuxCodecOpusImport): string;
export declare function demand(self: MuxCodecOpusImport): TrackDemand;
export declare function finish(self: MuxCodecOpusImport): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function abort(self: MuxCodecOpusImport, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecOpusImport, err: Error_): rt.Result<undefined, MuxCodecOpusImport>;
export declare function cut(self: MuxCodecOpusImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecOpusImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecOpusImport, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecOpusImport, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;

// MuxCodecOpusMapping.ts
import * as rt from "./rt.ts";
export type MuxCodecOpusMapping = rt.Opaque<"MuxCodecOpusMapping">;
export declare function debug(self: MuxCodecOpusMapping): string;
export declare function eq(a: MuxCodecOpusMapping, b: MuxCodecOpusMapping): boolean;
export declare function family(self: MuxCodecOpusMapping): number;
export declare function streams(self: MuxCodecOpusMapping): number;
export declare function coupled(self: MuxCodecOpusMapping): number;
export declare function table(self: MuxCodecOpusMapping): rt.Bytes;

// MuxCodecVp8Error.ts
export type MuxCodecVp8Error = "FrameTooShort" | "KeyframeHeaderTooShort" | "StartCodeMismatch" | "EmptyFrame";
export declare function toString(self: MuxCodecVp8Error): string;
export declare function debug(self: MuxCodecVp8Error): string;

// MuxCodecVp8Import.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecVp8Import = rt.Handle<"MuxCodecVp8Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp8Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp8Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp8Import, MuxError>;
export declare function initialize(self: MuxCodecVp8Import, buf: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecVp8Import, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxCodecVp8Import): TrackDemand;
export declare function finish(self: MuxCodecVp8Import): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxCodecVp8Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecVp8Import, err: Error_): rt.Result<undefined, MuxCodecVp8Import>;
export declare function cut(self: MuxCodecVp8Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecVp8Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecVp8Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecVp8Import, start: rt.u64): rt.Result<undefined, MuxError>;

// MuxCodecVp9Error.ts
export type MuxCodecVp9Error = "InvalidFrameMarker" | "InvalidSyncCode" | "Truncated" | "EmptyFrame";
export declare function toString(self: MuxCodecVp9Error): string;
export declare function debug(self: MuxCodecVp9Error): string;

// MuxCodecVp9Import.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecVp9Import = rt.Handle<"MuxCodecVp9Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp9Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp9Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, hint: MuxCatalogVideoHint): rt.Result<MuxCodecVp9Import, MuxError>;
export declare function initialize(self: MuxCodecVp9Import, buf: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxCodecVp9Import, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxCodecVp9Import): TrackDemand;
export declare function finish(self: MuxCodecVp9Import): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxCodecVp9Import, err: Error_): void;
export declare function abortUnwanted(self: MuxCodecVp9Import, err: Error_): rt.Result<undefined, MuxCodecVp9Import>;
export declare function cut(self: MuxCodecVp9Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecVp9Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecVp9Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecVp9Import, start: rt.u64): rt.Result<undefined, MuxError>;

// MuxContainerConsumerContainer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSyncConsumer } from "./MuxSyncConsumer.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerContainer = rt.Handle<"MuxContainerConsumerContainer">;
export declare function new$(track: TrackSubscriber, format: MuxCatalogHangContainer): MuxContainerConsumerContainer;
export declare function arrival(self: MuxContainerConsumerContainer): ClockInstant | undefined;
export declare function backlog(self: MuxContainerConsumerContainer): ClockInstant | undefined;
export declare function read(self: MuxContainerConsumerContainer): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function isSkipped(self: MuxContainerConsumerContainer, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function setDelay(self: MuxContainerConsumerContainer, delay: TrackDelay): void;
export declare function setPlayhead(self: MuxContainerConsumerContainer, playhead: MuxSyncConsumer): void;
export { new$ as new };

// MuxContainerConsumerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxSyncConsumer } from "./MuxSyncConsumer.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerFmp4Wire = rt.Handle<"MuxContainerConsumerFmp4Wire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerFmp4Wire): MuxContainerConsumerFmp4Wire;
export declare function arrival(self: MuxContainerConsumerFmp4Wire): ClockInstant | undefined;
export declare function backlog(self: MuxContainerConsumerFmp4Wire): ClockInstant | undefined;
export declare function read(self: MuxContainerConsumerFmp4Wire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxContainerFmp4Error>>;
export declare function isSkipped(self: MuxContainerConsumerFmp4Wire, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function setDelay(self: MuxContainerConsumerFmp4Wire, delay: TrackDelay): void;
export declare function setPlayhead(self: MuxContainerConsumerFmp4Wire, playhead: MuxSyncConsumer): void;
export { new$ as new };

// MuxContainerConsumerLegacyWire.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSyncConsumer } from "./MuxSyncConsumer.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerLegacyWire = rt.Handle<"MuxContainerConsumerLegacyWire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerLegacyWire): MuxContainerConsumerLegacyWire;
export declare function arrival(self: MuxContainerConsumerLegacyWire): ClockInstant | undefined;
export declare function backlog(self: MuxContainerConsumerLegacyWire): ClockInstant | undefined;
export declare function read(self: MuxContainerConsumerLegacyWire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function isSkipped(self: MuxContainerConsumerLegacyWire, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function setDelay(self: MuxContainerConsumerLegacyWire, delay: TrackDelay): void;
export declare function setPlayhead(self: MuxContainerConsumerLegacyWire, playhead: MuxSyncConsumer): void;
export { new$ as new };

// MuxContainerConsumerLocWire.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSyncConsumer } from "./MuxSyncConsumer.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerLocWire = rt.Handle<"MuxContainerConsumerLocWire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerLocWire): MuxContainerConsumerLocWire;
export declare function arrival(self: MuxContainerConsumerLocWire): ClockInstant | undefined;
export declare function backlog(self: MuxContainerConsumerLocWire): ClockInstant | undefined;
export declare function read(self: MuxContainerConsumerLocWire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function isSkipped(self: MuxContainerConsumerLocWire, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function setDelay(self: MuxContainerConsumerLocWire, delay: TrackDelay): void;
export declare function setPlayhead(self: MuxContainerConsumerLocWire, playhead: MuxSyncConsumer): void;
export { new$ as new };

// MuxContainerFinishedContainer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainer = rt.Handle<"MuxContainerFinishedContainer">;
export declare function name(self: MuxContainerFinishedContainer): string;
export declare function demand(self: MuxContainerFinishedContainer): TrackDemand;
export declare function info(self: MuxContainerFinishedContainer): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainer): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainer): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainer, err: Error_): void;

// MuxContainerFinishedContainerAudioConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainerAudioConfig = rt.Handle<"MuxContainerFinishedContainerAudioConfig">;
export declare function name(self: MuxContainerFinishedContainerAudioConfig): string;
export declare function demand(self: MuxContainerFinishedContainerAudioConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedContainerAudioConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainerAudioConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainerAudioConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainerAudioConfig, err: Error_): void;

// MuxContainerFinishedContainerBinaryConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainerBinaryConfig = rt.Handle<"MuxContainerFinishedContainerBinaryConfig">;
export declare function name(self: MuxContainerFinishedContainerBinaryConfig): string;
export declare function demand(self: MuxContainerFinishedContainerBinaryConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedContainerBinaryConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainerBinaryConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainerBinaryConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainerBinaryConfig, err: Error_): void;

// MuxContainerFinishedContainerJsonConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainerJsonConfig = rt.Handle<"MuxContainerFinishedContainerJsonConfig">;
export declare function name(self: MuxContainerFinishedContainerJsonConfig): string;
export declare function demand(self: MuxContainerFinishedContainerJsonConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedContainerJsonConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainerJsonConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainerJsonConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainerJsonConfig, err: Error_): void;

// MuxContainerFinishedContainerTextConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainerTextConfig = rt.Handle<"MuxContainerFinishedContainerTextConfig">;
export declare function name(self: MuxContainerFinishedContainerTextConfig): string;
export declare function demand(self: MuxContainerFinishedContainerTextConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedContainerTextConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainerTextConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainerTextConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainerTextConfig, err: Error_): void;

// MuxContainerFinishedContainerVideoConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedContainerVideoConfig = rt.Handle<"MuxContainerFinishedContainerVideoConfig">;
export declare function name(self: MuxContainerFinishedContainerVideoConfig): string;
export declare function demand(self: MuxContainerFinishedContainerVideoConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedContainerVideoConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedContainerVideoConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedContainerVideoConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedContainerVideoConfig, err: Error_): void;

// MuxContainerFinishedFmp4Wire.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4Wire = rt.Handle<"MuxContainerFinishedFmp4Wire">;
export declare function name(self: MuxContainerFinishedFmp4Wire): string;
export declare function demand(self: MuxContainerFinishedFmp4Wire): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4Wire): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4Wire): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4Wire): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4Wire, err: Error_): void;

// MuxContainerFinishedFmp4WireAudioConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4WireAudioConfig = rt.Handle<"MuxContainerFinishedFmp4WireAudioConfig">;
export declare function name(self: MuxContainerFinishedFmp4WireAudioConfig): string;
export declare function demand(self: MuxContainerFinishedFmp4WireAudioConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4WireAudioConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4WireAudioConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4WireAudioConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4WireAudioConfig, err: Error_): void;

// MuxContainerFinishedFmp4WireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4WireBinaryConfig = rt.Handle<"MuxContainerFinishedFmp4WireBinaryConfig">;
export declare function name(self: MuxContainerFinishedFmp4WireBinaryConfig): string;
export declare function demand(self: MuxContainerFinishedFmp4WireBinaryConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4WireBinaryConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4WireBinaryConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4WireBinaryConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4WireBinaryConfig, err: Error_): void;

// MuxContainerFinishedFmp4WireJsonConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4WireJsonConfig = rt.Handle<"MuxContainerFinishedFmp4WireJsonConfig">;
export declare function name(self: MuxContainerFinishedFmp4WireJsonConfig): string;
export declare function demand(self: MuxContainerFinishedFmp4WireJsonConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4WireJsonConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4WireJsonConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4WireJsonConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4WireJsonConfig, err: Error_): void;

// MuxContainerFinishedFmp4WireTextConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4WireTextConfig = rt.Handle<"MuxContainerFinishedFmp4WireTextConfig">;
export declare function name(self: MuxContainerFinishedFmp4WireTextConfig): string;
export declare function demand(self: MuxContainerFinishedFmp4WireTextConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4WireTextConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4WireTextConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4WireTextConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4WireTextConfig, err: Error_): void;

// MuxContainerFinishedFmp4WireVideoConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedFmp4WireVideoConfig = rt.Handle<"MuxContainerFinishedFmp4WireVideoConfig">;
export declare function name(self: MuxContainerFinishedFmp4WireVideoConfig): string;
export declare function demand(self: MuxContainerFinishedFmp4WireVideoConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedFmp4WireVideoConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedFmp4WireVideoConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedFmp4WireVideoConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedFmp4WireVideoConfig, err: Error_): void;

// MuxContainerFinishedLegacyWire.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWire = rt.Handle<"MuxContainerFinishedLegacyWire">;
export declare function name(self: MuxContainerFinishedLegacyWire): string;
export declare function demand(self: MuxContainerFinishedLegacyWire): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWire): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWire): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWire): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWire, err: Error_): void;

// MuxContainerFinishedLegacyWireAudioConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWireAudioConfig = rt.Handle<"MuxContainerFinishedLegacyWireAudioConfig">;
export declare function name(self: MuxContainerFinishedLegacyWireAudioConfig): string;
export declare function demand(self: MuxContainerFinishedLegacyWireAudioConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWireAudioConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWireAudioConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWireAudioConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWireAudioConfig, err: Error_): void;

// MuxContainerFinishedLegacyWireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWireBinaryConfig = rt.Handle<"MuxContainerFinishedLegacyWireBinaryConfig">;
export declare function name(self: MuxContainerFinishedLegacyWireBinaryConfig): string;
export declare function demand(self: MuxContainerFinishedLegacyWireBinaryConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWireBinaryConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWireBinaryConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWireBinaryConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWireBinaryConfig, err: Error_): void;

// MuxContainerFinishedLegacyWireJsonConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWireJsonConfig = rt.Handle<"MuxContainerFinishedLegacyWireJsonConfig">;
export declare function name(self: MuxContainerFinishedLegacyWireJsonConfig): string;
export declare function demand(self: MuxContainerFinishedLegacyWireJsonConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWireJsonConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWireJsonConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWireJsonConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWireJsonConfig, err: Error_): void;

// MuxContainerFinishedLegacyWireTextConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWireTextConfig = rt.Handle<"MuxContainerFinishedLegacyWireTextConfig">;
export declare function name(self: MuxContainerFinishedLegacyWireTextConfig): string;
export declare function demand(self: MuxContainerFinishedLegacyWireTextConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWireTextConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWireTextConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWireTextConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWireTextConfig, err: Error_): void;

// MuxContainerFinishedLegacyWireVideoConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLegacyWireVideoConfig = rt.Handle<"MuxContainerFinishedLegacyWireVideoConfig">;
export declare function name(self: MuxContainerFinishedLegacyWireVideoConfig): string;
export declare function demand(self: MuxContainerFinishedLegacyWireVideoConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLegacyWireVideoConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLegacyWireVideoConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLegacyWireVideoConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLegacyWireVideoConfig, err: Error_): void;

// MuxContainerFinishedLocWire.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWire = rt.Handle<"MuxContainerFinishedLocWire">;
export declare function name(self: MuxContainerFinishedLocWire): string;
export declare function demand(self: MuxContainerFinishedLocWire): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWire): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWire): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWire): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWire, err: Error_): void;

// MuxContainerFinishedLocWireAudioConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWireAudioConfig = rt.Handle<"MuxContainerFinishedLocWireAudioConfig">;
export declare function name(self: MuxContainerFinishedLocWireAudioConfig): string;
export declare function demand(self: MuxContainerFinishedLocWireAudioConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWireAudioConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWireAudioConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWireAudioConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWireAudioConfig, err: Error_): void;

// MuxContainerFinishedLocWireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWireBinaryConfig = rt.Handle<"MuxContainerFinishedLocWireBinaryConfig">;
export declare function name(self: MuxContainerFinishedLocWireBinaryConfig): string;
export declare function demand(self: MuxContainerFinishedLocWireBinaryConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWireBinaryConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWireBinaryConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWireBinaryConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWireBinaryConfig, err: Error_): void;

// MuxContainerFinishedLocWireJsonConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWireJsonConfig = rt.Handle<"MuxContainerFinishedLocWireJsonConfig">;
export declare function name(self: MuxContainerFinishedLocWireJsonConfig): string;
export declare function demand(self: MuxContainerFinishedLocWireJsonConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWireJsonConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWireJsonConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWireJsonConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWireJsonConfig, err: Error_): void;

// MuxContainerFinishedLocWireTextConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWireTextConfig = rt.Handle<"MuxContainerFinishedLocWireTextConfig">;
export declare function name(self: MuxContainerFinishedLocWireTextConfig): string;
export declare function demand(self: MuxContainerFinishedLocWireTextConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWireTextConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWireTextConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWireTextConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWireTextConfig, err: Error_): void;

// MuxContainerFinishedLocWireVideoConfig.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerFinishedLocWireVideoConfig = rt.Handle<"MuxContainerFinishedLocWireVideoConfig">;
export declare function name(self: MuxContainerFinishedLocWireVideoConfig): string;
export declare function demand(self: MuxContainerFinishedLocWireVideoConfig): TrackDemand;
export declare function info(self: MuxContainerFinishedLocWireVideoConfig): TrackInfo;
export declare function consume(self: MuxContainerFinishedLocWireVideoConfig): TrackSubscriber;
export declare function estimate(self: MuxContainerFinishedLocWireVideoConfig): MuxCatalogEstimate;
export declare function abort(self: MuxContainerFinishedLocWireVideoConfig, err: Error_): void;

// MuxContainerFlvFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFlvFinished = rt.Handle<"MuxContainerFlvFinished">;
export declare function abort(self: MuxContainerFlvFinished, err: Error_): void;

// MuxContainerFlvFinishedExt.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFlvFinishedExt = rt.Handle<"MuxContainerFlvFinishedExt">;
export declare function abort(self: MuxContainerFlvFinishedExt, err: Error_): void;

// MuxContainerFlvFinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFlvFinishedExtra = rt.Handle<"MuxContainerFlvFinishedExtra">;
export declare function abort(self: MuxContainerFlvFinishedExtra, err: Error_): void;

// MuxContainerFlvImport.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxContainerFlvFinished } from "./MuxContainerFlvFinished.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImport = rt.Handle<"MuxContainerFlvImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerFlvImport;
export declare function withContainer(self: MuxContainerFlvImport, container: HangCatalogContainer): MuxContainerFlvImport;
export declare function decode(self: MuxContainerFlvImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImport, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImport): rt.Result<MuxContainerFlvFinished, MuxError>;
export declare function abort(self: MuxContainerFlvImport, err: Error_): void;
export { new$ as new };

// MuxContainerFlvImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerFlvFinishedExt } from "./MuxContainerFlvFinishedExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImportExt = rt.Handle<"MuxContainerFlvImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerFlvImportExt;
export declare function withContainer(self: MuxContainerFlvImportExt, container: HangCatalogContainer): MuxContainerFlvImportExt;
export declare function decode(self: MuxContainerFlvImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImportExt, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImportExt): rt.Result<MuxContainerFlvFinishedExt, MuxError>;
export declare function abort(self: MuxContainerFlvImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerFlvImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFlvFinishedExtra } from "./MuxContainerFlvFinishedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImportExtra = rt.Handle<"MuxContainerFlvImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerFlvImportExtra;
export declare function withContainer(self: MuxContainerFlvImportExtra, container: HangCatalogContainer): MuxContainerFlvImportExtra;
export declare function decode(self: MuxContainerFlvImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImportExtra, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImportExtra): rt.Result<MuxContainerFlvFinishedExtra, MuxError>;
export declare function abort(self: MuxContainerFlvImportExtra, err: Error_): void;
export { new$ as new };

// MuxContainerFmp4Chunk.ts
import * as rt from "./rt.ts";
import type { MuxContainerFmp4Fragment } from "./MuxContainerFmp4Fragment.ts";
export type MuxContainerFmp4Chunk = {
    tag: "Init";
    _0: Uint8Array;
} | {
    tag: "Fragment";
    _0: MuxContainerFmp4Fragment;
};
export declare function debug(self: MuxContainerFmp4Chunk): string;
export declare function data(self: MuxContainerFmp4Chunk): rt.Bytes;
export declare function intoData(self: MuxContainerFmp4Chunk): rt.Bytes;
export declare function init(self: MuxContainerFmp4Chunk): rt.Bytes | undefined;
export declare function fragment(self: MuxContainerFmp4Chunk): MuxContainerFmp4Fragment | undefined;

// MuxContainerFmp4Error.ts
import * as rt from "./rt.ts";
export type MuxContainerFmp4Error = rt.Handle<"MuxContainerFmp4Error">;
export declare function toString(self: MuxContainerFmp4Error): string;
export declare function debug(self: MuxContainerFmp4Error): string;
export declare function clone(self: MuxContainerFmp4Error): MuxContainerFmp4Error;

// MuxContainerFmp4Finished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFmp4Finished = rt.Handle<"MuxContainerFmp4Finished">;
export declare function abort(self: MuxContainerFmp4Finished, err: Error_): void;

// MuxContainerFmp4FinishedExt.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFmp4FinishedExt = rt.Handle<"MuxContainerFmp4FinishedExt">;
export declare function abort(self: MuxContainerFmp4FinishedExt, err: Error_): void;

// MuxContainerFmp4FinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerFmp4FinishedExtra = rt.Handle<"MuxContainerFmp4FinishedExtra">;
export declare function abort(self: MuxContainerFmp4FinishedExtra, err: Error_): void;

// MuxContainerFmp4Fragment.ts
import * as rt from "./rt.ts";
export type MuxContainerFmp4Fragment = {
    data: Uint8Array;
    independent: boolean;
    duration: rt.Duration;
};
export declare function debug(self: MuxContainerFmp4Fragment): string;

// MuxContainerFmp4Import.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxContainerFmp4Finished } from "./MuxContainerFmp4Finished.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4Import = rt.Handle<"MuxContainerFmp4Import">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerFmp4Import;
export declare function cut(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4Import, select: MuxSelectBroadcast): MuxContainerFmp4Import;
export declare function withCmaf(self: MuxContainerFmp4Import, cmaf: boolean): MuxContainerFmp4Import;
export declare function decode(self: MuxContainerFmp4Import, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4Import): rt.Result<MuxContainerFmp4Finished, MuxError>;
export declare function abort(self: MuxContainerFmp4Import, err: Error_): void;
export declare function seek(self: MuxContainerFmp4Import, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function rejoin(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4ImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerFmp4FinishedExt } from "./MuxContainerFmp4FinishedExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4ImportExt = rt.Handle<"MuxContainerFmp4ImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerFmp4ImportExt;
export declare function cut(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4ImportExt, select: MuxSelectBroadcast): MuxContainerFmp4ImportExt;
export declare function withCmaf(self: MuxContainerFmp4ImportExt, cmaf: boolean): MuxContainerFmp4ImportExt;
export declare function decode(self: MuxContainerFmp4ImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4ImportExt): rt.Result<MuxContainerFmp4FinishedExt, MuxError>;
export declare function abort(self: MuxContainerFmp4ImportExt, err: Error_): void;
export declare function seek(self: MuxContainerFmp4ImportExt, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function rejoin(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4ImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerFmp4FinishedExtra } from "./MuxContainerFmp4FinishedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4ImportExtra = rt.Handle<"MuxContainerFmp4ImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerFmp4ImportExtra;
export declare function cut(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4ImportExtra, select: MuxSelectBroadcast): MuxContainerFmp4ImportExtra;
export declare function withCmaf(self: MuxContainerFmp4ImportExtra, cmaf: boolean): MuxContainerFmp4ImportExtra;
export declare function decode(self: MuxContainerFmp4ImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4ImportExtra): rt.Result<MuxContainerFmp4FinishedExtra, MuxError>;
export declare function abort(self: MuxContainerFmp4ImportExtra, err: Error_): void;
export declare function seek(self: MuxContainerFmp4ImportExtra, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function rejoin(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4Muxer.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerFmp4Muxer = rt.Handle<"MuxContainerFmp4Muxer">;
export declare function video(config: HangCatalogVideoConfig): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function audio(config: HangCatalogAudioConfig): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function timescale(self: MuxContainerFmp4Muxer): Timescale;
export declare function withTimescale(self: MuxContainerFmp4Muxer, timescale: Timescale): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function read(self: MuxContainerFmp4Muxer, group: GroupConsumer): rt.Call<rt.Result<MuxContainerFrame[], MuxError>>;
export declare function pictureSize(self: MuxContainerFmp4Muxer, keyframe: MuxContainerFrame): rt.Result<[number, number] | undefined, MuxError>;
export declare function init(self: MuxContainerFmp4Muxer): rt.Result<rt.Bytes | undefined, MuxError>;
export declare function fragment(self: MuxContainerFmp4Muxer, sequence: number, frames: MuxContainerFrame[], end: Timestamp | undefined): rt.Result<rt.Bytes, MuxError>;

// MuxContainerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
export type MuxContainerFmp4Wire = rt.Handle<"MuxContainerFmp4Wire">;
export declare function fromInit(initData: Uint8Array | rt.Bytes): rt.Result<MuxContainerFmp4Wire, MuxContainerFmp4Error>;

// MuxContainerFrame.ts
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerFrame = {
    timestamp: Timestamp;
    duration: Timestamp | undefined;
    payload: Uint8Array;
    keyframe: boolean;
};
export declare function debug(self: MuxContainerFrame): string;

// MuxContainerGroupConsumerContainer.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerGroupConsumerContainer = rt.Handle<"MuxContainerGroupConsumerContainer">;
export declare function new$(group: GroupConsumer, format: MuxCatalogHangContainer): MuxContainerGroupConsumerContainer;
export declare function start(self: MuxContainerGroupConsumerContainer): rt.u64;
export declare function read(self: MuxContainerGroupConsumerContainer): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export { new$ as new };

// MuxContainerGroupConsumerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
export type MuxContainerGroupConsumerFmp4Wire = rt.Handle<"MuxContainerGroupConsumerFmp4Wire">;
export declare function new$(group: GroupConsumer, format: MuxContainerFmp4Wire): MuxContainerGroupConsumerFmp4Wire;
export declare function start(self: MuxContainerGroupConsumerFmp4Wire): rt.u64;
export declare function read(self: MuxContainerGroupConsumerFmp4Wire): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxContainerFmp4Error>>;
export { new$ as new };

// MuxContainerGroupConsumerLegacyWire.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerGroupConsumerLegacyWire = rt.Handle<"MuxContainerGroupConsumerLegacyWire">;
export declare function new$(group: GroupConsumer, format: MuxContainerLegacyWire): MuxContainerGroupConsumerLegacyWire;
export declare function start(self: MuxContainerGroupConsumerLegacyWire): rt.u64;
export declare function read(self: MuxContainerGroupConsumerLegacyWire): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export { new$ as new };

// MuxContainerGroupConsumerLocWire.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerGroupConsumerLocWire = rt.Handle<"MuxContainerGroupConsumerLocWire">;
export declare function new$(group: GroupConsumer, format: MuxContainerLocWire): MuxContainerGroupConsumerLocWire;
export declare function start(self: MuxContainerGroupConsumerLocWire): rt.u64;
export declare function read(self: MuxContainerGroupConsumerLocWire): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export { new$ as new };

// MuxContainerGuardAudioConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioCodec } from "./HangCatalogAudioCodec.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogAudioLayout } from "./HangCatalogAudioLayout.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { Instance } from "./Instance.ts";
import type { MuxError } from "./MuxError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type MuxContainerGuardAudioConfig = rt.Handle<"MuxContainerGuardAudioConfig">;
export declare function commit(self: MuxContainerGuardAudioConfig): rt.Result<undefined, MuxError>;
export declare function broadcastGet(self: MuxContainerGuardAudioConfig): PathRelative | undefined;
export declare function broadcastSet(self: MuxContainerGuardAudioConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: MuxContainerGuardAudioConfig): string | undefined;
export declare function trackSet(self: MuxContainerGuardAudioConfig, value: string | undefined): void;
export declare function instanceGet(self: MuxContainerGuardAudioConfig): Instance | undefined;
export declare function instanceSet(self: MuxContainerGuardAudioConfig, value: Instance | undefined): void;
export declare function labelGet(self: MuxContainerGuardAudioConfig): string | undefined;
export declare function labelSet(self: MuxContainerGuardAudioConfig, value: string | undefined): void;
export declare function codecGet(self: MuxContainerGuardAudioConfig): HangCatalogAudioCodec;
export declare function codecSet(self: MuxContainerGuardAudioConfig, value: HangCatalogAudioCodec): void;
export declare function sampleRateGet(self: MuxContainerGuardAudioConfig): number;
export declare function sampleRateSet(self: MuxContainerGuardAudioConfig, value: number): void;
export declare function channelCountGet(self: MuxContainerGuardAudioConfig): number;
export declare function channelCountSet(self: MuxContainerGuardAudioConfig, value: number): void;
export declare function layoutGet(self: MuxContainerGuardAudioConfig): HangCatalogAudioLayout | undefined;
export declare function layoutSet(self: MuxContainerGuardAudioConfig, value: HangCatalogAudioLayout | undefined): void;
export declare function bitrateGet(self: MuxContainerGuardAudioConfig): rt.u64 | undefined;
export declare function bitrateSet(self: MuxContainerGuardAudioConfig, value: rt.u64 | undefined): void;
export declare function enabledGet(self: MuxContainerGuardAudioConfig): boolean;
export declare function enabledSet(self: MuxContainerGuardAudioConfig, value: boolean): void;
export declare function descriptionGet(self: MuxContainerGuardAudioConfig): rt.Bytes | undefined;
export declare function descriptionSet(self: MuxContainerGuardAudioConfig, value: Uint8Array | rt.Bytes | undefined): void;
export declare function containerGet(self: MuxContainerGuardAudioConfig): HangCatalogContainer;
export declare function containerSet(self: MuxContainerGuardAudioConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: MuxContainerGuardAudioConfig): rt.Duration | undefined;
export declare function jitterSet(self: MuxContainerGuardAudioConfig, value: rt.Duration | undefined): void;
export declare function warmupGet(self: MuxContainerGuardAudioConfig): rt.Duration | undefined;
export declare function warmupSet(self: MuxContainerGuardAudioConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxContainerGuardAudioConfig): rt.Duration | undefined;
export declare function delaySet(self: MuxContainerGuardAudioConfig, value: rt.Duration | undefined): void;
export declare function sameIdentity(self: MuxContainerGuardAudioConfig, other: HangCatalogAudioConfig): boolean;

// MuxContainerGuardBinaryConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { Instance } from "./Instance.ts";
import type { Json } from "./Json.ts";
import type { MuxError } from "./MuxError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type MuxContainerGuardBinaryConfig = rt.Handle<"MuxContainerGuardBinaryConfig">;
export declare function commit(self: MuxContainerGuardBinaryConfig): rt.Result<undefined, MuxError>;
export declare function broadcastGet(self: MuxContainerGuardBinaryConfig): PathRelative | undefined;
export declare function broadcastSet(self: MuxContainerGuardBinaryConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: MuxContainerGuardBinaryConfig): string | undefined;
export declare function trackSet(self: MuxContainerGuardBinaryConfig, value: string | undefined): void;
export declare function instanceGet(self: MuxContainerGuardBinaryConfig): Instance | undefined;
export declare function instanceSet(self: MuxContainerGuardBinaryConfig, value: Instance | undefined): void;
export declare function modeGet(self: MuxContainerGuardBinaryConfig): HangCatalogMode;
export declare function modeSet(self: MuxContainerGuardBinaryConfig, value: HangCatalogMode): void;
export declare function mimeGet(self: MuxContainerGuardBinaryConfig): string | undefined;
export declare function mimeSet(self: MuxContainerGuardBinaryConfig, value: string | undefined): void;
export declare function bitrateGet(self: MuxContainerGuardBinaryConfig): rt.u64 | undefined;
export declare function bitrateSet(self: MuxContainerGuardBinaryConfig, value: rt.u64 | undefined): void;
export declare function jitterGet(self: MuxContainerGuardBinaryConfig): rt.Duration | undefined;
export declare function jitterSet(self: MuxContainerGuardBinaryConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxContainerGuardBinaryConfig): rt.Duration | undefined;
export declare function delaySet(self: MuxContainerGuardBinaryConfig, value: rt.Duration | undefined): void;
export declare function extraGet(self: MuxContainerGuardBinaryConfig): ([string, Json])[];
export declare function extraSet(self: MuxContainerGuardBinaryConfig, value: ([string, Json])[]): void;

// MuxContainerGuardJsonConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { Instance } from "./Instance.ts";
import type { Json } from "./Json.ts";
import type { MuxError } from "./MuxError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type MuxContainerGuardJsonConfig = rt.Handle<"MuxContainerGuardJsonConfig">;
export declare function commit(self: MuxContainerGuardJsonConfig): rt.Result<undefined, MuxError>;
export declare function broadcastGet(self: MuxContainerGuardJsonConfig): PathRelative | undefined;
export declare function broadcastSet(self: MuxContainerGuardJsonConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: MuxContainerGuardJsonConfig): string | undefined;
export declare function trackSet(self: MuxContainerGuardJsonConfig, value: string | undefined): void;
export declare function instanceGet(self: MuxContainerGuardJsonConfig): Instance | undefined;
export declare function instanceSet(self: MuxContainerGuardJsonConfig, value: Instance | undefined): void;
export declare function modeGet(self: MuxContainerGuardJsonConfig): HangCatalogMode;
export declare function modeSet(self: MuxContainerGuardJsonConfig, value: HangCatalogMode): void;
export declare function schemaGet(self: MuxContainerGuardJsonConfig): string | undefined;
export declare function schemaSet(self: MuxContainerGuardJsonConfig, value: string | undefined): void;
export declare function bitrateGet(self: MuxContainerGuardJsonConfig): rt.u64 | undefined;
export declare function bitrateSet(self: MuxContainerGuardJsonConfig, value: rt.u64 | undefined): void;
export declare function jitterGet(self: MuxContainerGuardJsonConfig): rt.Duration | undefined;
export declare function jitterSet(self: MuxContainerGuardJsonConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxContainerGuardJsonConfig): rt.Duration | undefined;
export declare function delaySet(self: MuxContainerGuardJsonConfig, value: rt.Duration | undefined): void;
export declare function extraGet(self: MuxContainerGuardJsonConfig): ([string, Json])[];
export declare function extraSet(self: MuxContainerGuardJsonConfig, value: ([string, Json])[]): void;

// MuxContainerGuardTextConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { HangCatalogTextFormat } from "./HangCatalogTextFormat.ts";
import type { HangCatalogTextRole } from "./HangCatalogTextRole.ts";
import type { Instance } from "./Instance.ts";
import type { MuxError } from "./MuxError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type MuxContainerGuardTextConfig = rt.Handle<"MuxContainerGuardTextConfig">;
export declare function commit(self: MuxContainerGuardTextConfig): rt.Result<undefined, MuxError>;
export declare function broadcastGet(self: MuxContainerGuardTextConfig): PathRelative | undefined;
export declare function broadcastSet(self: MuxContainerGuardTextConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: MuxContainerGuardTextConfig): string | undefined;
export declare function trackSet(self: MuxContainerGuardTextConfig, value: string | undefined): void;
export declare function instanceGet(self: MuxContainerGuardTextConfig): Instance | undefined;
export declare function instanceSet(self: MuxContainerGuardTextConfig, value: Instance | undefined): void;
export declare function formatGet(self: MuxContainerGuardTextConfig): HangCatalogTextFormat;
export declare function formatSet(self: MuxContainerGuardTextConfig, value: HangCatalogTextFormat): void;
export declare function roleGet(self: MuxContainerGuardTextConfig): HangCatalogTextRole;
export declare function roleSet(self: MuxContainerGuardTextConfig, value: HangCatalogTextRole): void;
export declare function langGet(self: MuxContainerGuardTextConfig): string | undefined;
export declare function langSet(self: MuxContainerGuardTextConfig, value: string | undefined): void;
export declare function labelGet(self: MuxContainerGuardTextConfig): string | undefined;
export declare function labelSet(self: MuxContainerGuardTextConfig, value: string | undefined): void;
export declare function containerGet(self: MuxContainerGuardTextConfig): HangCatalogContainer;
export declare function containerSet(self: MuxContainerGuardTextConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: MuxContainerGuardTextConfig): rt.Duration | undefined;
export declare function jitterSet(self: MuxContainerGuardTextConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxContainerGuardTextConfig): rt.Duration | undefined;
export declare function delaySet(self: MuxContainerGuardTextConfig, value: rt.Duration | undefined): void;

// MuxContainerGuardUnit.ts
import * as rt from "./rt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerGuardUnit = rt.Handle<"MuxContainerGuardUnit">;
export declare function commit(self: MuxContainerGuardUnit): rt.Result<undefined, MuxError>;

// MuxContainerGuardVideoConfig.ts
import * as rt from "./rt.ts";
import type { HangCatalogColor } from "./HangCatalogColor.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { HangCatalogProjection } from "./HangCatalogProjection.ts";
import type { HangCatalogStereo } from "./HangCatalogStereo.ts";
import type { HangCatalogVideoCodec } from "./HangCatalogVideoCodec.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { Instance } from "./Instance.ts";
import type { MuxError } from "./MuxError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type MuxContainerGuardVideoConfig = rt.Handle<"MuxContainerGuardVideoConfig">;
export declare function commit(self: MuxContainerGuardVideoConfig): rt.Result<undefined, MuxError>;
export declare function broadcastGet(self: MuxContainerGuardVideoConfig): PathRelative | undefined;
export declare function broadcastSet(self: MuxContainerGuardVideoConfig, value: PathRelative | undefined): void;
export declare function trackGet(self: MuxContainerGuardVideoConfig): string | undefined;
export declare function trackSet(self: MuxContainerGuardVideoConfig, value: string | undefined): void;
export declare function instanceGet(self: MuxContainerGuardVideoConfig): Instance | undefined;
export declare function instanceSet(self: MuxContainerGuardVideoConfig, value: Instance | undefined): void;
export declare function labelGet(self: MuxContainerGuardVideoConfig): string | undefined;
export declare function labelSet(self: MuxContainerGuardVideoConfig, value: string | undefined): void;
export declare function codecGet(self: MuxContainerGuardVideoConfig): HangCatalogVideoCodec;
export declare function codecSet(self: MuxContainerGuardVideoConfig, value: HangCatalogVideoCodec): void;
export declare function descriptionGet(self: MuxContainerGuardVideoConfig): rt.Bytes | undefined;
export declare function descriptionSet(self: MuxContainerGuardVideoConfig, value: Uint8Array | rt.Bytes | undefined): void;
export declare function codedWidthGet(self: MuxContainerGuardVideoConfig): number | undefined;
export declare function codedWidthSet(self: MuxContainerGuardVideoConfig, value: number | undefined): void;
export declare function codedHeightGet(self: MuxContainerGuardVideoConfig): number | undefined;
export declare function codedHeightSet(self: MuxContainerGuardVideoConfig, value: number | undefined): void;
export declare function displayAspectWidthGet(self: MuxContainerGuardVideoConfig): number | undefined;
export declare function displayAspectWidthSet(self: MuxContainerGuardVideoConfig, value: number | undefined): void;
export declare function displayAspectHeightGet(self: MuxContainerGuardVideoConfig): number | undefined;
export declare function displayAspectHeightSet(self: MuxContainerGuardVideoConfig, value: number | undefined): void;
export declare function colorGet(self: MuxContainerGuardVideoConfig): HangCatalogColor | undefined;
export declare function colorSet(self: MuxContainerGuardVideoConfig, value: HangCatalogColor | undefined): void;
export declare function projectionGet(self: MuxContainerGuardVideoConfig): HangCatalogProjection | undefined;
export declare function projectionSet(self: MuxContainerGuardVideoConfig, value: HangCatalogProjection | undefined): void;
export declare function stereoGet(self: MuxContainerGuardVideoConfig): HangCatalogStereo | undefined;
export declare function stereoSet(self: MuxContainerGuardVideoConfig, value: HangCatalogStereo | undefined): void;
export declare function interlacedGet(self: MuxContainerGuardVideoConfig): boolean;
export declare function interlacedSet(self: MuxContainerGuardVideoConfig, value: boolean): void;
export declare function bitrateGet(self: MuxContainerGuardVideoConfig): rt.u64 | undefined;
export declare function bitrateSet(self: MuxContainerGuardVideoConfig, value: rt.u64 | undefined): void;
export declare function enabledGet(self: MuxContainerGuardVideoConfig): boolean;
export declare function enabledSet(self: MuxContainerGuardVideoConfig, value: boolean): void;
export declare function preferenceGet(self: MuxContainerGuardVideoConfig): number;
export declare function preferenceSet(self: MuxContainerGuardVideoConfig, value: number): void;
export declare function framerateGet(self: MuxContainerGuardVideoConfig): rt.f64 | undefined;
export declare function framerateSet(self: MuxContainerGuardVideoConfig, value: rt.f64 | undefined): void;
export declare function reorderGet(self: MuxContainerGuardVideoConfig): number | undefined;
export declare function reorderSet(self: MuxContainerGuardVideoConfig, value: number | undefined): void;
export declare function optimizeForLatencyGet(self: MuxContainerGuardVideoConfig): boolean | undefined;
export declare function optimizeForLatencySet(self: MuxContainerGuardVideoConfig, value: boolean | undefined): void;
export declare function containerGet(self: MuxContainerGuardVideoConfig): HangCatalogContainer;
export declare function containerSet(self: MuxContainerGuardVideoConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: MuxContainerGuardVideoConfig): rt.Duration | undefined;
export declare function jitterSet(self: MuxContainerGuardVideoConfig, value: rt.Duration | undefined): void;
export declare function delayGet(self: MuxContainerGuardVideoConfig): rt.Duration | undefined;
export declare function delaySet(self: MuxContainerGuardVideoConfig, value: rt.Duration | undefined): void;
export declare function sameIdentity(self: MuxContainerGuardVideoConfig, other: HangCatalogVideoConfig): boolean;

// MuxContainerKind.ts
export type MuxContainerKind = "Audio" | "Video" | "Data";
export declare function debug(self: MuxContainerKind): string;

// MuxContainerLegacyAudio.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerLegacyAudio = rt.Handle<"MuxContainerLegacyAudio">;
export declare function debug(self: MuxContainerLegacyAudio): string;
export declare function clone(self: MuxContainerLegacyAudio): MuxContainerLegacyAudio;
export declare function eq(a: MuxContainerLegacyAudio, b: MuxContainerLegacyAudio): boolean;
export declare function new$(config: HangCatalogAudioConfig): MuxContainerLegacyAudio | undefined;
export declare function duration(self: MuxContainerLegacyAudio, payload: Uint8Array | rt.Bytes): Timestamp | undefined;
export { new$ as new };

// MuxContainerLegacyWire.ts
import type { MuxContainerKind } from "./MuxContainerKind.ts";
export type MuxContainerLegacyWire = {
    _0: MuxContainerKind;
};

// MuxContainerLocWire.ts
import type { MuxContainerKind } from "./MuxContainerKind.ts";
export type MuxContainerLocWire = {
    _0: MuxContainerKind;
};

// MuxContainerMkvFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerMkvFinished = rt.Handle<"MuxContainerMkvFinished">;
export declare function abort(self: MuxContainerMkvFinished, err: Error_): void;

// MuxContainerMkvFinishedExt.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerMkvFinishedExt = rt.Handle<"MuxContainerMkvFinishedExt">;
export declare function abort(self: MuxContainerMkvFinishedExt, err: Error_): void;

// MuxContainerMkvFinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerMkvFinishedExtra = rt.Handle<"MuxContainerMkvFinishedExtra">;
export declare function abort(self: MuxContainerMkvFinishedExtra, err: Error_): void;

// MuxContainerMkvImport.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxContainerMkvFinished } from "./MuxContainerMkvFinished.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImport = rt.Handle<"MuxContainerMkvImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerMkvImport;
export declare function withContainer(self: MuxContainerMkvImport, container: HangCatalogContainer): MuxContainerMkvImport;
export declare function decode(self: MuxContainerMkvImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImport, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImport): rt.Result<MuxContainerMkvFinished, MuxError>;
export declare function abort(self: MuxContainerMkvImport, err: Error_): void;
export { new$ as new };

// MuxContainerMkvImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerMkvFinishedExt } from "./MuxContainerMkvFinishedExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImportExt = rt.Handle<"MuxContainerMkvImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerMkvImportExt;
export declare function withContainer(self: MuxContainerMkvImportExt, container: HangCatalogContainer): MuxContainerMkvImportExt;
export declare function decode(self: MuxContainerMkvImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImportExt, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImportExt): rt.Result<MuxContainerMkvFinishedExt, MuxError>;
export declare function abort(self: MuxContainerMkvImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerMkvImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerMkvFinishedExtra } from "./MuxContainerMkvFinishedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImportExtra = rt.Handle<"MuxContainerMkvImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerMkvImportExtra;
export declare function withContainer(self: MuxContainerMkvImportExtra, container: HangCatalogContainer): MuxContainerMkvImportExtra;
export declare function decode(self: MuxContainerMkvImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImportExtra, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImportExtra): rt.Result<MuxContainerMkvFinishedExtra, MuxError>;
export declare function abort(self: MuxContainerMkvImportExtra, err: Error_): void;
export { new$ as new };

// MuxContainerMp4Finish.ts
import * as rt from "./rt.ts";
export type MuxContainerMp4Finish = rt.Handle<"MuxContainerMp4Finish">;
export declare function debug(self: MuxContainerMp4Finish): string;
export declare function clone(self: MuxContainerMp4Finish): MuxContainerMp4Finish;

// MuxContainerPlayout.ts
import * as rt from "./rt.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerConsumerContainer } from "./MuxContainerConsumerContainer.ts";
import type { MuxContainerConsumerFmp4Wire } from "./MuxContainerConsumerFmp4Wire.ts";
import type { MuxContainerConsumerLegacyWire } from "./MuxContainerConsumerLegacyWire.ts";
import type { MuxContainerConsumerLocWire } from "./MuxContainerConsumerLocWire.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerPlayout = {
    delay: TrackDelay;
    warmup: rt.Duration;
};
export declare function debug(self: MuxContainerPlayout): string;
export declare function eq(a: MuxContainerPlayout, b: MuxContainerPlayout): boolean;
export declare function default$(): MuxContainerPlayout;
export declare function new$(delay: TrackDelay, warmup: rt.Duration): MuxContainerPlayout;
export declare function subscription(self: MuxContainerPlayout, subscription: TrackSubscription): TrackSubscription;
export declare function consumerContainer(self: MuxContainerPlayout, track: TrackSubscriber, format: MuxCatalogHangContainer): MuxContainerConsumerContainer;
export declare function consumerFmp4Wire(self: MuxContainerPlayout, track: TrackSubscriber, format: MuxContainerFmp4Wire): MuxContainerConsumerFmp4Wire;
export declare function consumerLegacyWire(self: MuxContainerPlayout, track: TrackSubscriber, format: MuxContainerLegacyWire): MuxContainerConsumerLegacyWire;
export declare function consumerLocWire(self: MuxContainerPlayout, track: TrackSubscriber, format: MuxContainerLocWire): MuxContainerConsumerLocWire;
export { default$ as default, new$ as new };

// MuxContainerProducerContainer.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerFinishedContainer } from "./MuxContainerFinishedContainer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardUnit } from "./MuxContainerGuardUnit.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainer = rt.Handle<"MuxContainerProducerContainer">;
export declare function new$(track: TrackProducer, container: MuxCatalogHangContainer): MuxContainerProducerContainer;
export declare function set(self: MuxContainerProducerContainer): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainer, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainer): rt.Result<MuxContainerGuardUnit, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainer, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainer): string;
export declare function demand(self: MuxContainerProducerContainer): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainer): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainer, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainer, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainer): boolean;
export declare function withBuffer(self: MuxContainerProducerContainer, duration: rt.Duration): MuxContainerProducerContainer;
export declare function withBandwidth(self: MuxContainerProducerContainer, allocator: BandwidthAllocator): MuxContainerProducerContainer;
export declare function reportSkipped(self: MuxContainerProducerContainer, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainer, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainer): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainer): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainer, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainer, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainer, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainer): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainer): rt.Result<MuxContainerFinishedContainer, MuxError>;
export declare function fail(self: MuxContainerProducerContainer, err: Error_): rt.Result<MuxContainerFinishedContainer, MuxError>;
export declare function abort(self: MuxContainerProducerContainer, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainer, err: Error_): rt.Result<undefined, MuxContainerProducerContainer>;
export declare function consume(self: MuxContainerProducerContainer): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainer): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainer): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainer, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainer, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainer, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainer): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainer): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainer): boolean;
export declare function largest(self: MuxContainerProducerContainer): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainer, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainer): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainer, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainer): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainer): TrackRequests;
export { new$ as new };

// MuxContainerProducerContainerAudioConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedContainerAudioConfig } from "./MuxContainerFinishedContainerAudioConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardAudioConfig } from "./MuxContainerGuardAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainerAudioConfig = rt.Handle<"MuxContainerProducerContainerAudioConfig">;
export declare function set(self: MuxContainerProducerContainerAudioConfig, config: HangCatalogAudioConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainerAudioConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainerAudioConfig): rt.Result<MuxContainerGuardAudioConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainerAudioConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainerAudioConfig): string;
export declare function demand(self: MuxContainerProducerContainerAudioConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainerAudioConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainerAudioConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainerAudioConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainerAudioConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerContainerAudioConfig, duration: rt.Duration): MuxContainerProducerContainerAudioConfig;
export declare function withBandwidth(self: MuxContainerProducerContainerAudioConfig, allocator: BandwidthAllocator): MuxContainerProducerContainerAudioConfig;
export declare function reportSkipped(self: MuxContainerProducerContainerAudioConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainerAudioConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainerAudioConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainerAudioConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainerAudioConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainerAudioConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainerAudioConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerAudioConfig): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function fail(self: MuxContainerProducerContainerAudioConfig, err: Error_): rt.Result<MuxContainerFinishedContainerAudioConfig, MuxError>;
export declare function abort(self: MuxContainerProducerContainerAudioConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainerAudioConfig, err: Error_): rt.Result<undefined, MuxContainerProducerContainerAudioConfig>;
export declare function consume(self: MuxContainerProducerContainerAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerAudioConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainerAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainerAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerAudioConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainerAudioConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainerAudioConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainerAudioConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainerAudioConfig): TrackRequests;

// MuxContainerProducerContainerBinaryConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedContainerBinaryConfig } from "./MuxContainerFinishedContainerBinaryConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardBinaryConfig } from "./MuxContainerGuardBinaryConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainerBinaryConfig = rt.Handle<"MuxContainerProducerContainerBinaryConfig">;
export declare function set(self: MuxContainerProducerContainerBinaryConfig, config: HangCatalogBinaryConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainerBinaryConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainerBinaryConfig): rt.Result<MuxContainerGuardBinaryConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainerBinaryConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainerBinaryConfig): string;
export declare function demand(self: MuxContainerProducerContainerBinaryConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainerBinaryConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainerBinaryConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainerBinaryConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainerBinaryConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerContainerBinaryConfig, duration: rt.Duration): MuxContainerProducerContainerBinaryConfig;
export declare function withBandwidth(self: MuxContainerProducerContainerBinaryConfig, allocator: BandwidthAllocator): MuxContainerProducerContainerBinaryConfig;
export declare function reportSkipped(self: MuxContainerProducerContainerBinaryConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainerBinaryConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainerBinaryConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainerBinaryConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainerBinaryConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainerBinaryConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainerBinaryConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerBinaryConfig): rt.Result<MuxContainerFinishedContainerBinaryConfig, MuxError>;
export declare function fail(self: MuxContainerProducerContainerBinaryConfig, err: Error_): rt.Result<MuxContainerFinishedContainerBinaryConfig, MuxError>;
export declare function abort(self: MuxContainerProducerContainerBinaryConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainerBinaryConfig, err: Error_): rt.Result<undefined, MuxContainerProducerContainerBinaryConfig>;
export declare function consume(self: MuxContainerProducerContainerBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerBinaryConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainerBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainerBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerBinaryConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainerBinaryConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainerBinaryConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainerBinaryConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainerBinaryConfig): TrackRequests;

// MuxContainerProducerContainerJsonConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedContainerJsonConfig } from "./MuxContainerFinishedContainerJsonConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardJsonConfig } from "./MuxContainerGuardJsonConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainerJsonConfig = rt.Handle<"MuxContainerProducerContainerJsonConfig">;
export declare function set(self: MuxContainerProducerContainerJsonConfig, config: HangCatalogJsonConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainerJsonConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainerJsonConfig): rt.Result<MuxContainerGuardJsonConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainerJsonConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainerJsonConfig): string;
export declare function demand(self: MuxContainerProducerContainerJsonConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainerJsonConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainerJsonConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainerJsonConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainerJsonConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerContainerJsonConfig, duration: rt.Duration): MuxContainerProducerContainerJsonConfig;
export declare function withBandwidth(self: MuxContainerProducerContainerJsonConfig, allocator: BandwidthAllocator): MuxContainerProducerContainerJsonConfig;
export declare function reportSkipped(self: MuxContainerProducerContainerJsonConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainerJsonConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainerJsonConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainerJsonConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainerJsonConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainerJsonConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainerJsonConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerJsonConfig): rt.Result<MuxContainerFinishedContainerJsonConfig, MuxError>;
export declare function fail(self: MuxContainerProducerContainerJsonConfig, err: Error_): rt.Result<MuxContainerFinishedContainerJsonConfig, MuxError>;
export declare function abort(self: MuxContainerProducerContainerJsonConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainerJsonConfig, err: Error_): rt.Result<undefined, MuxContainerProducerContainerJsonConfig>;
export declare function consume(self: MuxContainerProducerContainerJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerJsonConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainerJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainerJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerJsonConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainerJsonConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainerJsonConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainerJsonConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainerJsonConfig): TrackRequests;

// MuxContainerProducerContainerTextConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedContainerTextConfig } from "./MuxContainerFinishedContainerTextConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardTextConfig } from "./MuxContainerGuardTextConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainerTextConfig = rt.Handle<"MuxContainerProducerContainerTextConfig">;
export declare function set(self: MuxContainerProducerContainerTextConfig, config: HangCatalogTextConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainerTextConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainerTextConfig): rt.Result<MuxContainerGuardTextConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainerTextConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainerTextConfig): string;
export declare function demand(self: MuxContainerProducerContainerTextConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainerTextConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainerTextConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainerTextConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainerTextConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerContainerTextConfig, duration: rt.Duration): MuxContainerProducerContainerTextConfig;
export declare function withBandwidth(self: MuxContainerProducerContainerTextConfig, allocator: BandwidthAllocator): MuxContainerProducerContainerTextConfig;
export declare function reportSkipped(self: MuxContainerProducerContainerTextConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainerTextConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainerTextConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainerTextConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainerTextConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainerTextConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainerTextConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerTextConfig): rt.Result<MuxContainerFinishedContainerTextConfig, MuxError>;
export declare function fail(self: MuxContainerProducerContainerTextConfig, err: Error_): rt.Result<MuxContainerFinishedContainerTextConfig, MuxError>;
export declare function abort(self: MuxContainerProducerContainerTextConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainerTextConfig, err: Error_): rt.Result<undefined, MuxContainerProducerContainerTextConfig>;
export declare function consume(self: MuxContainerProducerContainerTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerTextConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainerTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainerTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerTextConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerTextConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainerTextConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainerTextConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainerTextConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainerTextConfig): TrackRequests;

// MuxContainerProducerContainerVideoConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedContainerVideoConfig } from "./MuxContainerFinishedContainerVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardVideoConfig } from "./MuxContainerGuardVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerContainerVideoConfig = rt.Handle<"MuxContainerProducerContainerVideoConfig">;
export declare function set(self: MuxContainerProducerContainerVideoConfig, config: HangCatalogVideoConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerContainerVideoConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerContainerVideoConfig): rt.Result<MuxContainerGuardVideoConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerContainerVideoConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerContainerVideoConfig): string;
export declare function demand(self: MuxContainerProducerContainerVideoConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerContainerVideoConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerContainerVideoConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerContainerVideoConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerContainerVideoConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerContainerVideoConfig, duration: rt.Duration): MuxContainerProducerContainerVideoConfig;
export declare function withBandwidth(self: MuxContainerProducerContainerVideoConfig, allocator: BandwidthAllocator): MuxContainerProducerContainerVideoConfig;
export declare function reportSkipped(self: MuxContainerProducerContainerVideoConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerContainerVideoConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerContainerVideoConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerContainerVideoConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerContainerVideoConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerContainerVideoConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerContainerVideoConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerVideoConfig): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function fail(self: MuxContainerProducerContainerVideoConfig, err: Error_): rt.Result<MuxContainerFinishedContainerVideoConfig, MuxError>;
export declare function abort(self: MuxContainerProducerContainerVideoConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerContainerVideoConfig, err: Error_): rt.Result<undefined, MuxContainerProducerContainerVideoConfig>;
export declare function consume(self: MuxContainerProducerContainerVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerVideoConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerContainerVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerContainerVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerVideoConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerContainerVideoConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerContainerVideoConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerContainerVideoConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerContainerVideoConfig): TrackRequests;

// MuxContainerProducerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4Wire } from "./MuxContainerFinishedFmp4Wire.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardUnit } from "./MuxContainerGuardUnit.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4Wire = rt.Handle<"MuxContainerProducerFmp4Wire">;
export declare function new$(track: TrackProducer, container: MuxContainerFmp4Wire): MuxContainerProducerFmp4Wire;
export declare function set(self: MuxContainerProducerFmp4Wire): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4Wire, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4Wire): rt.Result<MuxContainerGuardUnit, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4Wire, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4Wire): string;
export declare function demand(self: MuxContainerProducerFmp4Wire): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4Wire): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4Wire, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4Wire, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4Wire): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4Wire, duration: rt.Duration): MuxContainerProducerFmp4Wire;
export declare function withBandwidth(self: MuxContainerProducerFmp4Wire, allocator: BandwidthAllocator): MuxContainerProducerFmp4Wire;
export declare function reportSkipped(self: MuxContainerProducerFmp4Wire, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4Wire, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4Wire): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4Wire): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4Wire, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4Wire, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4Wire, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4Wire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4Wire): rt.Result<MuxContainerFinishedFmp4Wire, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4Wire, err: Error_): rt.Result<MuxContainerFinishedFmp4Wire, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4Wire, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4Wire, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4Wire>;
export declare function consume(self: MuxContainerProducerFmp4Wire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4Wire): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4Wire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4Wire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4Wire, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4Wire, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4Wire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4Wire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4Wire): boolean;
export declare function largest(self: MuxContainerProducerFmp4Wire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4Wire, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4Wire): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4Wire, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4Wire): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4Wire): TrackRequests;
export { new$ as new };

// MuxContainerProducerFmp4WireAudioConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4WireAudioConfig } from "./MuxContainerFinishedFmp4WireAudioConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardAudioConfig } from "./MuxContainerGuardAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4WireAudioConfig = rt.Handle<"MuxContainerProducerFmp4WireAudioConfig">;
export declare function set(self: MuxContainerProducerFmp4WireAudioConfig, config: HangCatalogAudioConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4WireAudioConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<MuxContainerGuardAudioConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4WireAudioConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4WireAudioConfig): string;
export declare function demand(self: MuxContainerProducerFmp4WireAudioConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4WireAudioConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4WireAudioConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4WireAudioConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4WireAudioConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4WireAudioConfig, duration: rt.Duration): MuxContainerProducerFmp4WireAudioConfig;
export declare function withBandwidth(self: MuxContainerProducerFmp4WireAudioConfig, allocator: BandwidthAllocator): MuxContainerProducerFmp4WireAudioConfig;
export declare function reportSkipped(self: MuxContainerProducerFmp4WireAudioConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4WireAudioConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4WireAudioConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4WireAudioConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4WireAudioConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4WireAudioConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4WireAudioConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<MuxContainerFinishedFmp4WireAudioConfig, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireAudioConfig, err: Error_): rt.Result<MuxContainerFinishedFmp4WireAudioConfig, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireAudioConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4WireAudioConfig, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4WireAudioConfig>;
export declare function consume(self: MuxContainerProducerFmp4WireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireAudioConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4WireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4WireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireAudioConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4WireAudioConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4WireAudioConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4WireAudioConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4WireAudioConfig): TrackRequests;

// MuxContainerProducerFmp4WireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4WireBinaryConfig } from "./MuxContainerFinishedFmp4WireBinaryConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardBinaryConfig } from "./MuxContainerGuardBinaryConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4WireBinaryConfig = rt.Handle<"MuxContainerProducerFmp4WireBinaryConfig">;
export declare function set(self: MuxContainerProducerFmp4WireBinaryConfig, config: HangCatalogBinaryConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4WireBinaryConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<MuxContainerGuardBinaryConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4WireBinaryConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4WireBinaryConfig): string;
export declare function demand(self: MuxContainerProducerFmp4WireBinaryConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4WireBinaryConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4WireBinaryConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4WireBinaryConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4WireBinaryConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4WireBinaryConfig, duration: rt.Duration): MuxContainerProducerFmp4WireBinaryConfig;
export declare function withBandwidth(self: MuxContainerProducerFmp4WireBinaryConfig, allocator: BandwidthAllocator): MuxContainerProducerFmp4WireBinaryConfig;
export declare function reportSkipped(self: MuxContainerProducerFmp4WireBinaryConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4WireBinaryConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4WireBinaryConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4WireBinaryConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4WireBinaryConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4WireBinaryConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4WireBinaryConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<MuxContainerFinishedFmp4WireBinaryConfig, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireBinaryConfig, err: Error_): rt.Result<MuxContainerFinishedFmp4WireBinaryConfig, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireBinaryConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4WireBinaryConfig, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4WireBinaryConfig>;
export declare function consume(self: MuxContainerProducerFmp4WireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireBinaryConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4WireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4WireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireBinaryConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4WireBinaryConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4WireBinaryConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4WireBinaryConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4WireBinaryConfig): TrackRequests;

// MuxContainerProducerFmp4WireJsonConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4WireJsonConfig } from "./MuxContainerFinishedFmp4WireJsonConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardJsonConfig } from "./MuxContainerGuardJsonConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4WireJsonConfig = rt.Handle<"MuxContainerProducerFmp4WireJsonConfig">;
export declare function set(self: MuxContainerProducerFmp4WireJsonConfig, config: HangCatalogJsonConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4WireJsonConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<MuxContainerGuardJsonConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4WireJsonConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4WireJsonConfig): string;
export declare function demand(self: MuxContainerProducerFmp4WireJsonConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4WireJsonConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4WireJsonConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4WireJsonConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4WireJsonConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4WireJsonConfig, duration: rt.Duration): MuxContainerProducerFmp4WireJsonConfig;
export declare function withBandwidth(self: MuxContainerProducerFmp4WireJsonConfig, allocator: BandwidthAllocator): MuxContainerProducerFmp4WireJsonConfig;
export declare function reportSkipped(self: MuxContainerProducerFmp4WireJsonConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4WireJsonConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4WireJsonConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4WireJsonConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4WireJsonConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4WireJsonConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4WireJsonConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<MuxContainerFinishedFmp4WireJsonConfig, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireJsonConfig, err: Error_): rt.Result<MuxContainerFinishedFmp4WireJsonConfig, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireJsonConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4WireJsonConfig, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4WireJsonConfig>;
export declare function consume(self: MuxContainerProducerFmp4WireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireJsonConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4WireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4WireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireJsonConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4WireJsonConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4WireJsonConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4WireJsonConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4WireJsonConfig): TrackRequests;

// MuxContainerProducerFmp4WireTextConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4WireTextConfig } from "./MuxContainerFinishedFmp4WireTextConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardTextConfig } from "./MuxContainerGuardTextConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4WireTextConfig = rt.Handle<"MuxContainerProducerFmp4WireTextConfig">;
export declare function set(self: MuxContainerProducerFmp4WireTextConfig, config: HangCatalogTextConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4WireTextConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<MuxContainerGuardTextConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4WireTextConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4WireTextConfig): string;
export declare function demand(self: MuxContainerProducerFmp4WireTextConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4WireTextConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4WireTextConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4WireTextConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4WireTextConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4WireTextConfig, duration: rt.Duration): MuxContainerProducerFmp4WireTextConfig;
export declare function withBandwidth(self: MuxContainerProducerFmp4WireTextConfig, allocator: BandwidthAllocator): MuxContainerProducerFmp4WireTextConfig;
export declare function reportSkipped(self: MuxContainerProducerFmp4WireTextConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4WireTextConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4WireTextConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4WireTextConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4WireTextConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4WireTextConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4WireTextConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<MuxContainerFinishedFmp4WireTextConfig, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireTextConfig, err: Error_): rt.Result<MuxContainerFinishedFmp4WireTextConfig, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireTextConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4WireTextConfig, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4WireTextConfig>;
export declare function consume(self: MuxContainerProducerFmp4WireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireTextConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4WireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4WireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireTextConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4WireTextConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4WireTextConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4WireTextConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4WireTextConfig): TrackRequests;

// MuxContainerProducerFmp4WireVideoConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedFmp4WireVideoConfig } from "./MuxContainerFinishedFmp4WireVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardVideoConfig } from "./MuxContainerGuardVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerFmp4WireVideoConfig = rt.Handle<"MuxContainerProducerFmp4WireVideoConfig">;
export declare function set(self: MuxContainerProducerFmp4WireVideoConfig, config: HangCatalogVideoConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerFmp4WireVideoConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<MuxContainerGuardVideoConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerFmp4WireVideoConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerFmp4WireVideoConfig): string;
export declare function demand(self: MuxContainerProducerFmp4WireVideoConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerFmp4WireVideoConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerFmp4WireVideoConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerFmp4WireVideoConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerFmp4WireVideoConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerFmp4WireVideoConfig, duration: rt.Duration): MuxContainerProducerFmp4WireVideoConfig;
export declare function withBandwidth(self: MuxContainerProducerFmp4WireVideoConfig, allocator: BandwidthAllocator): MuxContainerProducerFmp4WireVideoConfig;
export declare function reportSkipped(self: MuxContainerProducerFmp4WireVideoConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerFmp4WireVideoConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerFmp4WireVideoConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerFmp4WireVideoConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerFmp4WireVideoConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerFmp4WireVideoConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerFmp4WireVideoConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<MuxContainerFinishedFmp4WireVideoConfig, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireVideoConfig, err: Error_): rt.Result<MuxContainerFinishedFmp4WireVideoConfig, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireVideoConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerFmp4WireVideoConfig, err: Error_): rt.Result<undefined, MuxContainerProducerFmp4WireVideoConfig>;
export declare function consume(self: MuxContainerProducerFmp4WireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireVideoConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerFmp4WireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerFmp4WireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireVideoConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerFmp4WireVideoConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerFmp4WireVideoConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerFmp4WireVideoConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerFmp4WireVideoConfig): TrackRequests;

// MuxContainerProducerLegacyWire.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWire } from "./MuxContainerFinishedLegacyWire.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardUnit } from "./MuxContainerGuardUnit.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWire = rt.Handle<"MuxContainerProducerLegacyWire">;
export declare function new$(track: TrackProducer, container: MuxContainerLegacyWire): MuxContainerProducerLegacyWire;
export declare function set(self: MuxContainerProducerLegacyWire): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWire, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWire): rt.Result<MuxContainerGuardUnit, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWire, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWire): string;
export declare function demand(self: MuxContainerProducerLegacyWire): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWire): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWire, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWire, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWire): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWire, duration: rt.Duration): MuxContainerProducerLegacyWire;
export declare function withBandwidth(self: MuxContainerProducerLegacyWire, allocator: BandwidthAllocator): MuxContainerProducerLegacyWire;
export declare function reportSkipped(self: MuxContainerProducerLegacyWire, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWire, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWire): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWire): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWire, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWire, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWire, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWire): rt.Result<MuxContainerFinishedLegacyWire, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWire, err: Error_): rt.Result<MuxContainerFinishedLegacyWire, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWire, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWire, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWire>;
export declare function consume(self: MuxContainerProducerLegacyWire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWire): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWire, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWire, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWire): boolean;
export declare function largest(self: MuxContainerProducerLegacyWire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWire, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWire): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWire, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWire): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWire): TrackRequests;
export { new$ as new };

// MuxContainerProducerLegacyWireAudioConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWireAudioConfig } from "./MuxContainerFinishedLegacyWireAudioConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardAudioConfig } from "./MuxContainerGuardAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWireAudioConfig = rt.Handle<"MuxContainerProducerLegacyWireAudioConfig">;
export declare function set(self: MuxContainerProducerLegacyWireAudioConfig, config: HangCatalogAudioConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWireAudioConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<MuxContainerGuardAudioConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWireAudioConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWireAudioConfig): string;
export declare function demand(self: MuxContainerProducerLegacyWireAudioConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWireAudioConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWireAudioConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWireAudioConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWireAudioConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWireAudioConfig, duration: rt.Duration): MuxContainerProducerLegacyWireAudioConfig;
export declare function withBandwidth(self: MuxContainerProducerLegacyWireAudioConfig, allocator: BandwidthAllocator): MuxContainerProducerLegacyWireAudioConfig;
export declare function reportSkipped(self: MuxContainerProducerLegacyWireAudioConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWireAudioConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWireAudioConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWireAudioConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWireAudioConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWireAudioConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWireAudioConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<MuxContainerFinishedLegacyWireAudioConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireAudioConfig, err: Error_): rt.Result<MuxContainerFinishedLegacyWireAudioConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireAudioConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWireAudioConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWireAudioConfig>;
export declare function consume(self: MuxContainerProducerLegacyWireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireAudioConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireAudioConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWireAudioConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWireAudioConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWireAudioConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWireAudioConfig): TrackRequests;

// MuxContainerProducerLegacyWireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWireBinaryConfig } from "./MuxContainerFinishedLegacyWireBinaryConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardBinaryConfig } from "./MuxContainerGuardBinaryConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWireBinaryConfig = rt.Handle<"MuxContainerProducerLegacyWireBinaryConfig">;
export declare function set(self: MuxContainerProducerLegacyWireBinaryConfig, config: HangCatalogBinaryConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWireBinaryConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<MuxContainerGuardBinaryConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWireBinaryConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWireBinaryConfig): string;
export declare function demand(self: MuxContainerProducerLegacyWireBinaryConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWireBinaryConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWireBinaryConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWireBinaryConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWireBinaryConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWireBinaryConfig, duration: rt.Duration): MuxContainerProducerLegacyWireBinaryConfig;
export declare function withBandwidth(self: MuxContainerProducerLegacyWireBinaryConfig, allocator: BandwidthAllocator): MuxContainerProducerLegacyWireBinaryConfig;
export declare function reportSkipped(self: MuxContainerProducerLegacyWireBinaryConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWireBinaryConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWireBinaryConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWireBinaryConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWireBinaryConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWireBinaryConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWireBinaryConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<MuxContainerFinishedLegacyWireBinaryConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireBinaryConfig, err: Error_): rt.Result<MuxContainerFinishedLegacyWireBinaryConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireBinaryConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWireBinaryConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWireBinaryConfig>;
export declare function consume(self: MuxContainerProducerLegacyWireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireBinaryConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireBinaryConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWireBinaryConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWireBinaryConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWireBinaryConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWireBinaryConfig): TrackRequests;

// MuxContainerProducerLegacyWireJsonConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWireJsonConfig } from "./MuxContainerFinishedLegacyWireJsonConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardJsonConfig } from "./MuxContainerGuardJsonConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWireJsonConfig = rt.Handle<"MuxContainerProducerLegacyWireJsonConfig">;
export declare function set(self: MuxContainerProducerLegacyWireJsonConfig, config: HangCatalogJsonConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWireJsonConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<MuxContainerGuardJsonConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWireJsonConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWireJsonConfig): string;
export declare function demand(self: MuxContainerProducerLegacyWireJsonConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWireJsonConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWireJsonConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWireJsonConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWireJsonConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWireJsonConfig, duration: rt.Duration): MuxContainerProducerLegacyWireJsonConfig;
export declare function withBandwidth(self: MuxContainerProducerLegacyWireJsonConfig, allocator: BandwidthAllocator): MuxContainerProducerLegacyWireJsonConfig;
export declare function reportSkipped(self: MuxContainerProducerLegacyWireJsonConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWireJsonConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWireJsonConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWireJsonConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWireJsonConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWireJsonConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWireJsonConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<MuxContainerFinishedLegacyWireJsonConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireJsonConfig, err: Error_): rt.Result<MuxContainerFinishedLegacyWireJsonConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireJsonConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWireJsonConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWireJsonConfig>;
export declare function consume(self: MuxContainerProducerLegacyWireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireJsonConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireJsonConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWireJsonConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWireJsonConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWireJsonConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWireJsonConfig): TrackRequests;

// MuxContainerProducerLegacyWireTextConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWireTextConfig } from "./MuxContainerFinishedLegacyWireTextConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardTextConfig } from "./MuxContainerGuardTextConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWireTextConfig = rt.Handle<"MuxContainerProducerLegacyWireTextConfig">;
export declare function set(self: MuxContainerProducerLegacyWireTextConfig, config: HangCatalogTextConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWireTextConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<MuxContainerGuardTextConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWireTextConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWireTextConfig): string;
export declare function demand(self: MuxContainerProducerLegacyWireTextConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWireTextConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWireTextConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWireTextConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWireTextConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWireTextConfig, duration: rt.Duration): MuxContainerProducerLegacyWireTextConfig;
export declare function withBandwidth(self: MuxContainerProducerLegacyWireTextConfig, allocator: BandwidthAllocator): MuxContainerProducerLegacyWireTextConfig;
export declare function reportSkipped(self: MuxContainerProducerLegacyWireTextConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWireTextConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWireTextConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWireTextConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWireTextConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWireTextConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWireTextConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<MuxContainerFinishedLegacyWireTextConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireTextConfig, err: Error_): rt.Result<MuxContainerFinishedLegacyWireTextConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireTextConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWireTextConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWireTextConfig>;
export declare function consume(self: MuxContainerProducerLegacyWireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireTextConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireTextConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWireTextConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWireTextConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWireTextConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWireTextConfig): TrackRequests;

// MuxContainerProducerLegacyWireVideoConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLegacyWireVideoConfig } from "./MuxContainerFinishedLegacyWireVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardVideoConfig } from "./MuxContainerGuardVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLegacyWireVideoConfig = rt.Handle<"MuxContainerProducerLegacyWireVideoConfig">;
export declare function set(self: MuxContainerProducerLegacyWireVideoConfig, config: HangCatalogVideoConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLegacyWireVideoConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<MuxContainerGuardVideoConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLegacyWireVideoConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLegacyWireVideoConfig): string;
export declare function demand(self: MuxContainerProducerLegacyWireVideoConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLegacyWireVideoConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLegacyWireVideoConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLegacyWireVideoConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLegacyWireVideoConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLegacyWireVideoConfig, duration: rt.Duration): MuxContainerProducerLegacyWireVideoConfig;
export declare function withBandwidth(self: MuxContainerProducerLegacyWireVideoConfig, allocator: BandwidthAllocator): MuxContainerProducerLegacyWireVideoConfig;
export declare function reportSkipped(self: MuxContainerProducerLegacyWireVideoConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLegacyWireVideoConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLegacyWireVideoConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLegacyWireVideoConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLegacyWireVideoConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLegacyWireVideoConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLegacyWireVideoConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<MuxContainerFinishedLegacyWireVideoConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireVideoConfig, err: Error_): rt.Result<MuxContainerFinishedLegacyWireVideoConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireVideoConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLegacyWireVideoConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLegacyWireVideoConfig>;
export declare function consume(self: MuxContainerProducerLegacyWireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireVideoConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLegacyWireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLegacyWireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireVideoConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLegacyWireVideoConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLegacyWireVideoConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLegacyWireVideoConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLegacyWireVideoConfig): TrackRequests;

// MuxContainerProducerLocWire.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWire } from "./MuxContainerFinishedLocWire.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardUnit } from "./MuxContainerGuardUnit.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWire = rt.Handle<"MuxContainerProducerLocWire">;
export declare function new$(track: TrackProducer, container: MuxContainerLocWire): MuxContainerProducerLocWire;
export declare function set(self: MuxContainerProducerLocWire): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWire, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWire): rt.Result<MuxContainerGuardUnit, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWire, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWire): string;
export declare function demand(self: MuxContainerProducerLocWire): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWire): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWire, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWire, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWire): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWire, duration: rt.Duration): MuxContainerProducerLocWire;
export declare function withBandwidth(self: MuxContainerProducerLocWire, allocator: BandwidthAllocator): MuxContainerProducerLocWire;
export declare function reportSkipped(self: MuxContainerProducerLocWire, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWire, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWire): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWire): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWire, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWire, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWire, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWire): rt.Result<MuxContainerFinishedLocWire, MuxError>;
export declare function fail(self: MuxContainerProducerLocWire, err: Error_): rt.Result<MuxContainerFinishedLocWire, MuxError>;
export declare function abort(self: MuxContainerProducerLocWire, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWire, err: Error_): rt.Result<undefined, MuxContainerProducerLocWire>;
export declare function consume(self: MuxContainerProducerLocWire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWire): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWire, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWire, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWire): boolean;
export declare function largest(self: MuxContainerProducerLocWire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWire, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWire): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWire, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWire): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWire): TrackRequests;
export { new$ as new };

// MuxContainerProducerLocWireAudioConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWireAudioConfig } from "./MuxContainerFinishedLocWireAudioConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardAudioConfig } from "./MuxContainerGuardAudioConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWireAudioConfig = rt.Handle<"MuxContainerProducerLocWireAudioConfig">;
export declare function set(self: MuxContainerProducerLocWireAudioConfig, config: HangCatalogAudioConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWireAudioConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWireAudioConfig): rt.Result<MuxContainerGuardAudioConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWireAudioConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWireAudioConfig): string;
export declare function demand(self: MuxContainerProducerLocWireAudioConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWireAudioConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWireAudioConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWireAudioConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWireAudioConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWireAudioConfig, duration: rt.Duration): MuxContainerProducerLocWireAudioConfig;
export declare function withBandwidth(self: MuxContainerProducerLocWireAudioConfig, allocator: BandwidthAllocator): MuxContainerProducerLocWireAudioConfig;
export declare function reportSkipped(self: MuxContainerProducerLocWireAudioConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWireAudioConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWireAudioConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWireAudioConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWireAudioConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWireAudioConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWireAudioConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireAudioConfig): rt.Result<MuxContainerFinishedLocWireAudioConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireAudioConfig, err: Error_): rt.Result<MuxContainerFinishedLocWireAudioConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireAudioConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWireAudioConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLocWireAudioConfig>;
export declare function consume(self: MuxContainerProducerLocWireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireAudioConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireAudioConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWireAudioConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWireAudioConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWireAudioConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWireAudioConfig): TrackRequests;

// MuxContainerProducerLocWireBinaryConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWireBinaryConfig } from "./MuxContainerFinishedLocWireBinaryConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardBinaryConfig } from "./MuxContainerGuardBinaryConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWireBinaryConfig = rt.Handle<"MuxContainerProducerLocWireBinaryConfig">;
export declare function set(self: MuxContainerProducerLocWireBinaryConfig, config: HangCatalogBinaryConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWireBinaryConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<MuxContainerGuardBinaryConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWireBinaryConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWireBinaryConfig): string;
export declare function demand(self: MuxContainerProducerLocWireBinaryConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWireBinaryConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWireBinaryConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWireBinaryConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWireBinaryConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWireBinaryConfig, duration: rt.Duration): MuxContainerProducerLocWireBinaryConfig;
export declare function withBandwidth(self: MuxContainerProducerLocWireBinaryConfig, allocator: BandwidthAllocator): MuxContainerProducerLocWireBinaryConfig;
export declare function reportSkipped(self: MuxContainerProducerLocWireBinaryConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWireBinaryConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWireBinaryConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWireBinaryConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWireBinaryConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWireBinaryConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWireBinaryConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<MuxContainerFinishedLocWireBinaryConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireBinaryConfig, err: Error_): rt.Result<MuxContainerFinishedLocWireBinaryConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireBinaryConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWireBinaryConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLocWireBinaryConfig>;
export declare function consume(self: MuxContainerProducerLocWireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireBinaryConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireBinaryConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWireBinaryConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWireBinaryConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWireBinaryConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWireBinaryConfig): TrackRequests;

// MuxContainerProducerLocWireJsonConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWireJsonConfig } from "./MuxContainerFinishedLocWireJsonConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardJsonConfig } from "./MuxContainerGuardJsonConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWireJsonConfig = rt.Handle<"MuxContainerProducerLocWireJsonConfig">;
export declare function set(self: MuxContainerProducerLocWireJsonConfig, config: HangCatalogJsonConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWireJsonConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWireJsonConfig): rt.Result<MuxContainerGuardJsonConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWireJsonConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWireJsonConfig): string;
export declare function demand(self: MuxContainerProducerLocWireJsonConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWireJsonConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWireJsonConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWireJsonConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWireJsonConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWireJsonConfig, duration: rt.Duration): MuxContainerProducerLocWireJsonConfig;
export declare function withBandwidth(self: MuxContainerProducerLocWireJsonConfig, allocator: BandwidthAllocator): MuxContainerProducerLocWireJsonConfig;
export declare function reportSkipped(self: MuxContainerProducerLocWireJsonConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWireJsonConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWireJsonConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWireJsonConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWireJsonConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWireJsonConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWireJsonConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireJsonConfig): rt.Result<MuxContainerFinishedLocWireJsonConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireJsonConfig, err: Error_): rt.Result<MuxContainerFinishedLocWireJsonConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireJsonConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWireJsonConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLocWireJsonConfig>;
export declare function consume(self: MuxContainerProducerLocWireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireJsonConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireJsonConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWireJsonConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWireJsonConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWireJsonConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWireJsonConfig): TrackRequests;

// MuxContainerProducerLocWireTextConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWireTextConfig } from "./MuxContainerFinishedLocWireTextConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardTextConfig } from "./MuxContainerGuardTextConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWireTextConfig = rt.Handle<"MuxContainerProducerLocWireTextConfig">;
export declare function set(self: MuxContainerProducerLocWireTextConfig, config: HangCatalogTextConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWireTextConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWireTextConfig): rt.Result<MuxContainerGuardTextConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWireTextConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWireTextConfig): string;
export declare function demand(self: MuxContainerProducerLocWireTextConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWireTextConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWireTextConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWireTextConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWireTextConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWireTextConfig, duration: rt.Duration): MuxContainerProducerLocWireTextConfig;
export declare function withBandwidth(self: MuxContainerProducerLocWireTextConfig, allocator: BandwidthAllocator): MuxContainerProducerLocWireTextConfig;
export declare function reportSkipped(self: MuxContainerProducerLocWireTextConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWireTextConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWireTextConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWireTextConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWireTextConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWireTextConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWireTextConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireTextConfig): rt.Result<MuxContainerFinishedLocWireTextConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireTextConfig, err: Error_): rt.Result<MuxContainerFinishedLocWireTextConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireTextConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWireTextConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLocWireTextConfig>;
export declare function consume(self: MuxContainerProducerLocWireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireTextConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireTextConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWireTextConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWireTextConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWireTextConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWireTextConfig): TrackRequests;

// MuxContainerProducerLocWireVideoConfig.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { MuxContainerFinishedLocWireVideoConfig } from "./MuxContainerFinishedLocWireVideoConfig.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerGuardVideoConfig } from "./MuxContainerGuardVideoConfig.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type MuxContainerProducerLocWireVideoConfig = rt.Handle<"MuxContainerProducerLocWireVideoConfig">;
export declare function set(self: MuxContainerProducerLocWireVideoConfig, config: HangCatalogVideoConfig): rt.Result<undefined, MuxError>;
export declare function setBitrate(self: MuxContainerProducerLocWireVideoConfig, bitrate: rt.u64): rt.Result<undefined, MuxError>;
export declare function modify(self: MuxContainerProducerLocWireVideoConfig): rt.Result<MuxContainerGuardVideoConfig, MuxError>;
export declare function timestamp(self: MuxContainerProducerLocWireVideoConfig, hint: Timestamp | undefined): rt.Result<Timestamp, MuxError>;
export declare function name(self: MuxContainerProducerLocWireVideoConfig): string;
export declare function demand(self: MuxContainerProducerLocWireVideoConfig): TrackDemand;
export declare function estimate(self: MuxContainerProducerLocWireVideoConfig): MuxCatalogEstimate;
export declare function reorder(self: MuxContainerProducerLocWireVideoConfig, delay: Timestamp): void;
export declare function history(self: MuxContainerProducerLocWireVideoConfig, until: Timestamp): void;
export declare function needsKeyframe(self: MuxContainerProducerLocWireVideoConfig): boolean;
export declare function withBuffer(self: MuxContainerProducerLocWireVideoConfig, duration: rt.Duration): MuxContainerProducerLocWireVideoConfig;
export declare function withBandwidth(self: MuxContainerProducerLocWireVideoConfig, allocator: BandwidthAllocator): MuxContainerProducerLocWireVideoConfig;
export declare function reportSkipped(self: MuxContainerProducerLocWireVideoConfig, frames: rt.u64): void;
export declare function reportBitrate(self: MuxContainerProducerLocWireVideoConfig, bitrate: BandwidthRate): void;
export declare function track(self: MuxContainerProducerLocWireVideoConfig): TrackProducer;
export declare function liveEdge(self: MuxContainerProducerLocWireVideoConfig): Timestamp | undefined;
export declare function write(self: MuxContainerProducerLocWireVideoConfig, frame: MuxContainerFrame): rt.Result<undefined, MuxError>;
export declare function cut(self: MuxContainerProducerLocWireVideoConfig, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerProducerLocWireVideoConfig, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireVideoConfig): rt.Result<MuxContainerFinishedLocWireVideoConfig, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireVideoConfig, err: Error_): rt.Result<MuxContainerFinishedLocWireVideoConfig, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireVideoConfig, err: Error_): void;
export declare function abortUnwanted(self: MuxContainerProducerLocWireVideoConfig, err: Error_): rt.Result<undefined, MuxContainerProducerLocWireVideoConfig>;
export declare function consume(self: MuxContainerProducerLocWireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireVideoConfig, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function skipGroups(self: MuxContainerProducerLocWireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: MuxContainerProducerLocWireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireVideoConfig, other: TrackProducer): boolean;
export declare function abortHandle(self: MuxContainerProducerLocWireVideoConfig): TrackAbortHandle;
export declare function subscribe(self: MuxContainerProducerLocWireVideoConfig, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: MuxContainerProducerLocWireVideoConfig): TrackSubscription | undefined;
export declare function requests(self: MuxContainerProducerLocWireVideoConfig): TrackRequests;

// MuxContainerRead.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerRead = {
    tag: "Frame";
    group: rt.u64;
    frame: MuxContainerFrame;
} | {
    tag: "Gap";
    timestamp: Timestamp;
    duration: Timestamp;
} | {
    tag: "End";
    group: rt.u64;
    end: Timestamp;
};
export declare function debug(self: MuxContainerRead): string;

// MuxContainerReorderError.ts
import * as rt from "./rt.ts";
export type MuxContainerReorderError = {
    tag: "Depth";
    reorder: number;
    max: number;
    codec: string;
} | {
    tag: "Deepened";
    declared: number;
    reorder: number;
} | {
    tag: "Reordered";
    _0: rt.u64;
} | {
    tag: "Presentation";
} | {
    tag: "Tie";
} | {
    tag: "Undrained";
};
export declare function toString(self: MuxContainerReorderError): string;
export declare function debug(self: MuxContainerReorderError): string;
export declare function eq(a: MuxContainerReorderError, b: MuxContainerReorderError): boolean;

// MuxContainerTimestampRewind.ts
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerTimestampRewind = {
    timestamp: Timestamp;
    floor: Timestamp;
};
export declare function toString(self: MuxContainerTimestampRewind): string;
export declare function debug(self: MuxContainerTimestampRewind): string;
export declare function eq(a: MuxContainerTimestampRewind, b: MuxContainerTimestampRewind): boolean;

// MuxContainerTsDescriptor.ts
export type MuxContainerTsDescriptor = {
    tag: number;
    data: Uint8Array;
};
export declare function debug(self: MuxContainerTsDescriptor): string;
export declare function eq(a: MuxContainerTsDescriptor, b: MuxContainerTsDescriptor): boolean;

// MuxContainerTsError.ts
import * as rt from "./rt.ts";
export type MuxContainerTsError = rt.Handle<"MuxContainerTsError">;
export declare function toString(self: MuxContainerTsError): string;
export declare function debug(self: MuxContainerTsError): string;
export declare function clone(self: MuxContainerTsError): MuxContainerTsError;

// MuxContainerTsExportConsumer.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogConsumer } from "./MuxCatalogConsumer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsExport } from "./MuxContainerTsStatsExport.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsExportConsumer = rt.Handle<"MuxContainerTsExportConsumer">;
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumer, delay: rt.Duration): MuxContainerTsExportConsumer;
export declare function withMuxRate(self: MuxContainerTsExportConsumer, muxRate: rt.u64): MuxContainerTsExportConsumer;
export declare function next(self: MuxContainerTsExportConsumer): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumer): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumer): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumer, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumer): rt.Result<MuxContainerTsExportConsumer, MuxError>;
export declare function values(self: MuxContainerTsExportConsumer): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { new$ as new };

// MuxContainerTsExportConsumerExt.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogConsumerExt } from "./MuxCatalogConsumerExt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsExport } from "./MuxContainerTsStatsExport.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsExportConsumerExt = rt.Handle<"MuxContainerTsExportConsumerExt">;
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumerExt, delay: rt.Duration): MuxContainerTsExportConsumerExt;
export declare function withMuxRate(self: MuxContainerTsExportConsumerExt, muxRate: rt.u64): MuxContainerTsExportConsumerExt;
export declare function next(self: MuxContainerTsExportConsumerExt): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumerExt): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumerExt): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumerExt, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumerExt): rt.Result<MuxContainerTsExportConsumerExt, MuxError>;
export declare function values(self: MuxContainerTsExportConsumerExt): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { new$ as new };

// MuxContainerTsExportConsumerExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogConsumerExtra } from "./MuxCatalogConsumerExtra.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsExport } from "./MuxContainerTsStatsExport.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsExportConsumerExtra = rt.Handle<"MuxContainerTsExportConsumerExtra">;
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumerExtra, delay: rt.Duration): MuxContainerTsExportConsumerExtra;
export declare function withMuxRate(self: MuxContainerTsExportConsumerExtra, muxRate: rt.u64): MuxContainerTsExportConsumerExtra;
export declare function next(self: MuxContainerTsExportConsumerExtra): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumerExtra): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumerExtra): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumerExtra, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumerExtra): rt.Result<MuxContainerTsExportConsumerExtra, MuxError>;
export declare function values(self: MuxContainerTsExportConsumerExtra): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { new$ as new };

// MuxContainerTsExt.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsMpegts } from "./MuxContainerTsMpegts.ts";
export type MuxContainerTsExt = rt.Handle<"MuxContainerTsExt">;
export declare function debug(self: MuxContainerTsExt): string;
export declare function clone(self: MuxContainerTsExt): MuxContainerTsExt;
export declare function eq(a: MuxContainerTsExt, b: MuxContainerTsExt): boolean;
export declare function default$(): MuxContainerTsExt;
export declare function mpegtsGet(self: MuxContainerTsExt): MuxContainerTsMpegts;
export declare function mpegtsSet(self: MuxContainerTsExt, value: MuxContainerTsMpegts): void;
export { default$ as default };

// MuxContainerTsFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsFinished = rt.Handle<"MuxContainerTsFinished">;
export declare function stats(self: MuxContainerTsFinished): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsFinished, err: Error_): void;

// MuxContainerTsFinishedExt.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsFinishedExt = rt.Handle<"MuxContainerTsFinishedExt">;
export declare function stats(self: MuxContainerTsFinishedExt): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsFinishedExt, err: Error_): void;

// MuxContainerTsFinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsFinishedExtra = rt.Handle<"MuxContainerTsFinishedExtra">;
export declare function stats(self: MuxContainerTsFinishedExtra): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsFinishedExtra, err: Error_): void;

// MuxContainerTsFollower.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsExportConsumer } from "./MuxContainerTsExportConsumer.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsFollower = rt.Handle<"MuxContainerTsFollower">;
export declare function new$(export_: MuxContainerTsExportConsumer): rt.Result<MuxContainerTsFollower, MuxError>;
export declare function withFollow(self: MuxContainerTsFollower, follow: rt.Duration): MuxContainerTsFollower;
export declare function withStitch(self: MuxContainerTsFollower, stitch: boolean): MuxContainerTsFollower;
export declare function export$(self: MuxContainerTsFollower): MuxContainerTsExportConsumer;
export declare function next(self: MuxContainerTsFollower): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function values(self: MuxContainerTsFollower): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { export$ as export, new$ as new };

// MuxContainerTsFollowerExt.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsExportConsumerExt } from "./MuxContainerTsExportConsumerExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsFollowerExt = rt.Handle<"MuxContainerTsFollowerExt">;
export declare function new$(export_: MuxContainerTsExportConsumerExt): rt.Result<MuxContainerTsFollowerExt, MuxError>;
export declare function withFollow(self: MuxContainerTsFollowerExt, follow: rt.Duration): MuxContainerTsFollowerExt;
export declare function withStitch(self: MuxContainerTsFollowerExt, stitch: boolean): MuxContainerTsFollowerExt;
export declare function export$(self: MuxContainerTsFollowerExt): MuxContainerTsExportConsumerExt;
export declare function next(self: MuxContainerTsFollowerExt): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function values(self: MuxContainerTsFollowerExt): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { export$ as export, new$ as new };

// MuxContainerTsFollowerExtra.ts
import * as rt from "./rt.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsExportConsumerExtra } from "./MuxContainerTsExportConsumerExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsFollowerExtra = rt.Handle<"MuxContainerTsFollowerExtra">;
export declare function new$(export_: MuxContainerTsExportConsumerExtra): rt.Result<MuxContainerTsFollowerExtra, MuxError>;
export declare function withFollow(self: MuxContainerTsFollowerExtra, follow: rt.Duration): MuxContainerTsFollowerExtra;
export declare function withStitch(self: MuxContainerTsFollowerExtra, stitch: boolean): MuxContainerTsFollowerExtra;
export declare function export$(self: MuxContainerTsFollowerExtra): MuxContainerTsExportConsumerExtra;
export declare function next(self: MuxContainerTsFollowerExtra): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function values(self: MuxContainerTsFollowerExtra): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { export$ as export, new$ as new };

// MuxContainerTsFraming.ts
export type MuxContainerTsFraming = "Section";
export declare function debug(self: MuxContainerTsFraming): string;
export declare function default$(): MuxContainerTsFraming;
export { default$ as default };

// MuxContainerTsImport.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxContainerTsError } from "./MuxContainerTsError.ts";
import type { MuxContainerTsFinished } from "./MuxContainerTsFinished.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImport = rt.Handle<"MuxContainerTsImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerTsImport;
export declare function withProgram(self: MuxContainerTsImport, program: number): MuxContainerTsImport;
export declare function withContainer(self: MuxContainerTsImport, container: HangCatalogContainer): MuxContainerTsImport;
export declare function decode(self: MuxContainerTsImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxContainerTsError>;
export declare function seek(self: MuxContainerTsImport, sequence: rt.u64): rt.Result<undefined, MuxContainerTsError>;
export declare function finish(self: MuxContainerTsImport): rt.Result<MuxContainerTsFinished, MuxContainerTsError>;
export declare function stats(self: MuxContainerTsImport): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImport, err: Error_): void;
export { new$ as new };

// MuxContainerTsImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerTsError } from "./MuxContainerTsError.ts";
import type { MuxContainerTsFinishedExt } from "./MuxContainerTsFinishedExt.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImportExt = rt.Handle<"MuxContainerTsImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerTsImportExt;
export declare function withProgram(self: MuxContainerTsImportExt, program: number): MuxContainerTsImportExt;
export declare function withContainer(self: MuxContainerTsImportExt, container: HangCatalogContainer): MuxContainerTsImportExt;
export declare function decode(self: MuxContainerTsImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxContainerTsError>;
export declare function seek(self: MuxContainerTsImportExt, sequence: rt.u64): rt.Result<undefined, MuxContainerTsError>;
export declare function finish(self: MuxContainerTsImportExt): rt.Result<MuxContainerTsFinishedExt, MuxContainerTsError>;
export declare function stats(self: MuxContainerTsImportExt): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerTsImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerTsError } from "./MuxContainerTsError.ts";
import type { MuxContainerTsFinishedExtra } from "./MuxContainerTsFinishedExtra.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImportExtra = rt.Handle<"MuxContainerTsImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerTsImportExtra;
export declare function withProgram(self: MuxContainerTsImportExtra, program: number): MuxContainerTsImportExtra;
export declare function withContainer(self: MuxContainerTsImportExtra, container: HangCatalogContainer): MuxContainerTsImportExtra;
export declare function decode(self: MuxContainerTsImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxContainerTsError>;
export declare function seek(self: MuxContainerTsImportExtra, sequence: rt.u64): rt.Result<undefined, MuxContainerTsError>;
export declare function finish(self: MuxContainerTsImportExtra): rt.Result<MuxContainerTsFinishedExtra, MuxContainerTsError>;
export declare function stats(self: MuxContainerTsImportExtra): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImportExtra, err: Error_): void;
export { new$ as new };

// MuxContainerTsMpegts.ts
import * as rt from "./rt.ts";
import type { Json } from "./Json.ts";
import type { MuxContainerTsDescriptor } from "./MuxContainerTsDescriptor.ts";
import type { MuxContainerTsPassthroughTrack } from "./MuxContainerTsPassthroughTrack.ts";
import type { MuxContainerTsProgram } from "./MuxContainerTsProgram.ts";
import type { MuxContainerTsSiEntry } from "./MuxContainerTsSiEntry.ts";
import type { MuxContainerTsTrack } from "./MuxContainerTsTrack.ts";
export type MuxContainerTsMpegts = rt.Handle<"MuxContainerTsMpegts">;
export declare function debug(self: MuxContainerTsMpegts): string;
export declare function clone(self: MuxContainerTsMpegts): MuxContainerTsMpegts;
export declare function eq(a: MuxContainerTsMpegts, b: MuxContainerTsMpegts): boolean;
export declare function default$(): MuxContainerTsMpegts;
export declare function tracksGet(self: MuxContainerTsMpegts): ([string, MuxContainerTsTrack])[];
export declare function tracksSet(self: MuxContainerTsMpegts, value: ([string, MuxContainerTsTrack])[]): void;
export declare function programDescriptorsGet(self: MuxContainerTsMpegts): MuxContainerTsDescriptor[];
export declare function programDescriptorsSet(self: MuxContainerTsMpegts, value: MuxContainerTsDescriptor[]): void;
export declare function programGet(self: MuxContainerTsMpegts): MuxContainerTsProgram | undefined;
export declare function programSet(self: MuxContainerTsMpegts, value: MuxContainerTsProgram | undefined): void;
export declare function siGet(self: MuxContainerTsMpegts): ([number, ([number, MuxContainerTsSiEntry])[]])[];
export declare function siSet(self: MuxContainerTsMpegts, value: ([number, ([number, MuxContainerTsSiEntry])[]])[]): void;
export declare function muxRateGet(self: MuxContainerTsMpegts): rt.u64 | undefined;
export declare function muxRateSet(self: MuxContainerTsMpegts, value: rt.u64 | undefined): void;
export declare function passthroughGet(self: MuxContainerTsMpegts): MuxContainerTsPassthroughTrack | undefined;
export declare function passthroughSet(self: MuxContainerTsMpegts, value: MuxContainerTsPassthroughTrack | undefined): void;
export declare function deserialize(deserializer: Json): rt.Result<MuxContainerTsMpegts, string>;
export declare function isEmpty(self: MuxContainerTsMpegts): boolean;
export { default$ as default };

// MuxContainerTsPassthrough.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerTsPassthroughFinished } from "./MuxContainerTsPassthroughFinished.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthrough = rt.Handle<"MuxContainerTsPassthrough">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): rt.Result<MuxContainerTsPassthrough, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthrough, pid: number): MuxContainerTsPassthrough;
export declare function track(self: MuxContainerTsPassthrough): string;
export declare function decode(self: MuxContainerTsPassthrough, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthrough): rt.Result<MuxContainerTsPassthroughFinished, MuxError>;
export declare function abort(self: MuxContainerTsPassthrough, err: Error_): void;
export { new$ as new };

// MuxContainerTsPassthroughExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerTsPassthroughFinishedExtra } from "./MuxContainerTsPassthroughFinishedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthroughExtra = rt.Handle<"MuxContainerTsPassthroughExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): rt.Result<MuxContainerTsPassthroughExtra, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthroughExtra, pid: number): MuxContainerTsPassthroughExtra;
export declare function track(self: MuxContainerTsPassthroughExtra): string;
export declare function decode(self: MuxContainerTsPassthroughExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthroughExtra): rt.Result<MuxContainerTsPassthroughFinishedExtra, MuxError>;
export declare function abort(self: MuxContainerTsPassthroughExtra, err: Error_): void;
export { new$ as new };

// MuxContainerTsPassthroughFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerTsPassthroughFinished = rt.Handle<"MuxContainerTsPassthroughFinished">;
export declare function track(self: MuxContainerTsPassthroughFinished): string;
export declare function abort(self: MuxContainerTsPassthroughFinished, err: Error_): void;

// MuxContainerTsPassthroughFinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerTsPassthroughFinishedExtra = rt.Handle<"MuxContainerTsPassthroughFinishedExtra">;
export declare function track(self: MuxContainerTsPassthroughFinishedExtra): string;
export declare function abort(self: MuxContainerTsPassthroughFinishedExtra, err: Error_): void;

// MuxContainerTsPassthroughFinishedUnit.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxContainerTsPassthroughFinishedUnit = rt.Handle<"MuxContainerTsPassthroughFinishedUnit">;
export declare function track(self: MuxContainerTsPassthroughFinishedUnit): string;
export declare function abort(self: MuxContainerTsPassthroughFinishedUnit, err: Error_): void;

// MuxContainerTsPassthroughTrack.ts
import * as rt from "./rt.ts";
export type MuxContainerTsPassthroughTrack = rt.Handle<"MuxContainerTsPassthroughTrack">;
export declare function debug(self: MuxContainerTsPassthroughTrack): string;
export declare function clone(self: MuxContainerTsPassthroughTrack): MuxContainerTsPassthroughTrack;
export declare function eq(a: MuxContainerTsPassthroughTrack, b: MuxContainerTsPassthroughTrack): boolean;
export declare function trackGet(self: MuxContainerTsPassthroughTrack): string;
export declare function trackSet(self: MuxContainerTsPassthroughTrack, value: string): void;
export declare function randomAccessGet(self: MuxContainerTsPassthroughTrack): boolean;
export declare function randomAccessSet(self: MuxContainerTsPassthroughTrack, value: boolean): void;
export declare function new$(track: string): MuxContainerTsPassthroughTrack;
export { new$ as new };

// MuxContainerTsPassthroughUnit.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxContainerTsPassthroughFinishedUnit } from "./MuxContainerTsPassthroughFinishedUnit.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthroughUnit = rt.Handle<"MuxContainerTsPassthroughUnit">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): rt.Result<MuxContainerTsPassthroughUnit, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthroughUnit, pid: number): MuxContainerTsPassthroughUnit;
export declare function track(self: MuxContainerTsPassthroughUnit): string;
export declare function decode(self: MuxContainerTsPassthroughUnit, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthroughUnit): rt.Result<MuxContainerTsPassthroughFinishedUnit, MuxError>;
export declare function abort(self: MuxContainerTsPassthroughUnit, err: Error_): void;
export { new$ as new };

// MuxContainerTsPesPacket.ts
import * as rt from "./rt.ts";
export type MuxContainerTsPesPacket = rt.Handle<"MuxContainerTsPesPacket">;
export declare function debug(self: MuxContainerTsPesPacket): string;
export declare function clone(self: MuxContainerTsPesPacket): MuxContainerTsPesPacket;
export declare function eq(a: MuxContainerTsPesPacket, b: MuxContainerTsPesPacket): boolean;
export declare function default$(): MuxContainerTsPesPacket;
export declare function streamTypeGet(self: MuxContainerTsPesPacket): number;
export declare function streamTypeSet(self: MuxContainerTsPesPacket, value: number): void;
export declare function new$(streamType: number): MuxContainerTsPesPacket;
export { default$ as default, new$ as new };

// MuxContainerTsProgram.ts
import * as rt from "./rt.ts";
export type MuxContainerTsProgram = rt.Handle<"MuxContainerTsProgram">;
export declare function debug(self: MuxContainerTsProgram): string;
export declare function clone(self: MuxContainerTsProgram): MuxContainerTsProgram;
export declare function eq(a: MuxContainerTsProgram, b: MuxContainerTsProgram): boolean;
export declare function default$(): MuxContainerTsProgram;
export declare function transportStreamIdGet(self: MuxContainerTsProgram): number;
export declare function transportStreamIdSet(self: MuxContainerTsProgram, value: number): void;
export declare function programNumberGet(self: MuxContainerTsProgram): number;
export declare function programNumberSet(self: MuxContainerTsProgram, value: number): void;
export declare function pmtPidGet(self: MuxContainerTsProgram): number;
export declare function pmtPidSet(self: MuxContainerTsProgram, value: number): void;
export { default$ as default };

// MuxContainerTsPrograms.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxContainerTsError } from "./MuxContainerTsError.ts";
import type { MuxContainerTsProgramsFinished } from "./MuxContainerTsProgramsFinished.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Path } from "./Path.ts";
export type MuxContainerTsPrograms = rt.Handle<"MuxContainerTsPrograms">;
export declare function new$(origin: OriginProducer, name: Path, config: MuxCatalogConfig): MuxContainerTsPrograms;
export declare function decode(self: MuxContainerTsPrograms, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxContainerTsError>;
export declare function stats(self: MuxContainerTsPrograms): MuxContainerTsStatsSnapshot;
export declare function finish(self: MuxContainerTsPrograms): rt.Result<MuxContainerTsProgramsFinished, MuxContainerTsError>;
export declare function abort(self: MuxContainerTsPrograms, err: Error_): void;
export { new$ as new };

// MuxContainerTsProgramsFinished.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsProgramsFinished = rt.Handle<"MuxContainerTsProgramsFinished">;
export declare function broadcasts(self: MuxContainerTsProgramsFinished): BroadcastProducer[];
export declare function stats(self: MuxContainerTsProgramsFinished): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsProgramsFinished, err: Error_): void;

// MuxContainerTsRelease.ts
import * as rt from "./rt.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsRelease } from "./MuxContainerTsStatsRelease.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsRelease = rt.Handle<"MuxContainerTsRelease">;
export declare function new$(source: MuxSource, catalogFormat: MuxCatalogCatalogFormat, delay: rt.Duration): rt.Call<rt.Result<MuxContainerTsRelease, MuxError>>;
export declare function dropped(self: MuxContainerTsRelease): rt.u64;
export declare function drift(self: MuxContainerTsRelease): rt.f64 | undefined;
export declare function outOfTolerance(self: MuxContainerTsRelease): rt.u64;
export declare function stats(self: MuxContainerTsRelease): MuxContainerTsStatsRelease;
export declare function next(self: MuxContainerTsRelease): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function values(self: MuxContainerTsRelease): rt.Stream<rt.Result<MuxContainerFrame, MuxError>>;
export { new$ as new };

// MuxContainerTsRepublish.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { Instance } from "./Instance.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxCatalogProducerExt } from "./MuxCatalogProducerExt.ts";
import type { MuxContainerTsError } from "./MuxContainerTsError.ts";
import type { MuxContainerTsRepublishFinished } from "./MuxContainerTsRepublishFinished.ts";
import type { MuxContainerTsRepublished } from "./MuxContainerTsRepublished.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Path } from "./Path.ts";
export type MuxContainerTsRepublish = rt.Handle<"MuxContainerTsRepublish">;
export declare function new$(origin: OriginProducer, name: Path, instance: Instance, config: MuxCatalogConfig): rt.Result<MuxContainerTsRepublish, MuxContainerTsError>;
export declare function withProgram(self: MuxContainerTsRepublish, program: number): MuxContainerTsRepublish;
export declare function broadcast(self: MuxContainerTsRepublish): BroadcastProducer;
export declare function catalog(self: MuxContainerTsRepublish): MuxCatalogProducerExt;
export declare function instance(self: MuxContainerTsRepublish): Instance;
export declare function decode(self: MuxContainerTsRepublish, data: Uint8Array | rt.Bytes): rt.Result<MuxContainerTsRepublished | undefined, MuxContainerTsError>;
export declare function stats(self: MuxContainerTsRepublish): MuxContainerTsStatsSnapshot;
export declare function finish(self: MuxContainerTsRepublish): rt.Result<MuxContainerTsRepublishFinished, MuxContainerTsError>;
export declare function abort(self: MuxContainerTsRepublish, err: Error_): void;
export { new$ as new };

// MuxContainerTsRepublishFinished.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsRepublishFinished = rt.Handle<"MuxContainerTsRepublishFinished">;
export declare function broadcast(self: MuxContainerTsRepublishFinished): BroadcastProducer;
export declare function stats(self: MuxContainerTsRepublishFinished): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsRepublishFinished, err: Error_): void;

// MuxContainerTsRepublished.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
import type { MuxContainerTsStep } from "./MuxContainerTsStep.ts";
export type MuxContainerTsRepublished = rt.Handle<"MuxContainerTsRepublished">;
export declare function debug(self: MuxContainerTsRepublished): string;
export declare function clone(self: MuxContainerTsRepublished): MuxContainerTsRepublished;
export declare function stepGet(self: MuxContainerTsRepublished): MuxContainerTsStep;
export declare function stepSet(self: MuxContainerTsRepublished, value: MuxContainerTsStep): void;
export declare function statsGet(self: MuxContainerTsRepublished): MuxContainerTsStatsSnapshot;
export declare function statsSet(self: MuxContainerTsRepublished, value: MuxContainerTsStatsSnapshot): void;

// MuxContainerTsSiEntry.ts
import * as rt from "./rt.ts";
export type MuxContainerTsSiEntry = rt.Handle<"MuxContainerTsSiEntry">;
export declare function debug(self: MuxContainerTsSiEntry): string;
export declare function clone(self: MuxContainerTsSiEntry): MuxContainerTsSiEntry;
export declare function eq(a: MuxContainerTsSiEntry, b: MuxContainerTsSiEntry): boolean;
export declare function default$(): MuxContainerTsSiEntry;
export declare function trackGet(self: MuxContainerTsSiEntry): string;
export declare function trackSet(self: MuxContainerTsSiEntry, value: string): void;
export declare function intervalGet(self: MuxContainerTsSiEntry): rt.Duration | undefined;
export declare function intervalSet(self: MuxContainerTsSiEntry, value: rt.Duration | undefined): void;
export { default$ as default };

// MuxContainerTsStatsClass.ts
export type MuxContainerTsStatsClass = "Audio" | "Video" | "Data";
export declare function debug(self: MuxContainerTsStatsClass): string;
export declare function default$(): MuxContainerTsStatsClass;
export { default$ as default };

// MuxContainerTsStatsExport.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsRelease } from "./MuxContainerTsStatsRelease.ts";
import type { MuxContainerTsStatsStream } from "./MuxContainerTsStatsStream.ts";
export type MuxContainerTsStatsExport = rt.Handle<"MuxContainerTsStatsExport">;
export declare function debug(self: MuxContainerTsStatsExport): string;
export declare function clone(self: MuxContainerTsStatsExport): MuxContainerTsStatsExport;
export declare function eq(a: MuxContainerTsStatsExport, b: MuxContainerTsStatsExport): boolean;
export declare function default$(): MuxContainerTsStatsExport;
export declare function streamsGet(self: MuxContainerTsStatsExport): ([number, MuxContainerTsStatsStream])[];
export declare function streamsSet(self: MuxContainerTsStatsExport, value: ([number, MuxContainerTsStatsStream])[]): void;
export declare function releaseGet(self: MuxContainerTsStatsExport): MuxContainerTsStatsRelease;
export declare function releaseSet(self: MuxContainerTsStatsExport, value: MuxContainerTsStatsRelease): void;
export { default$ as default };

// MuxContainerTsStatsLog.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsStatsLog = rt.Handle<"MuxContainerTsStatsLog">;
export declare function default$(): MuxContainerTsStatsLog;
export declare function INTERVAL(): rt.Duration;
export declare function sample(self: MuxContainerTsStatsLog, latest: MuxContainerTsStatsSnapshot): void;
export declare function finish(self: MuxContainerTsStatsLog, latest: MuxContainerTsStatsSnapshot): void;
export { default$ as default };

// MuxContainerTsStatsRelease.ts
import * as rt from "./rt.ts";
export type MuxContainerTsStatsRelease = rt.Handle<"MuxContainerTsStatsRelease">;
export declare function debug(self: MuxContainerTsStatsRelease): string;
export declare function clone(self: MuxContainerTsStatsRelease): MuxContainerTsStatsRelease;
export declare function eq(a: MuxContainerTsStatsRelease, b: MuxContainerTsStatsRelease): boolean;
export declare function default$(): MuxContainerTsStatsRelease;
export declare function droppedGet(self: MuxContainerTsStatsRelease): rt.u64;
export declare function droppedSet(self: MuxContainerTsStatsRelease, value: rt.u64): void;
export declare function trimmedGet(self: MuxContainerTsStatsRelease): rt.u64;
export declare function trimmedSet(self: MuxContainerTsStatsRelease, value: rt.u64): void;
export declare function gapsGet(self: MuxContainerTsStatsRelease): rt.u64;
export declare function gapsSet(self: MuxContainerTsStatsRelease, value: rt.u64): void;
export declare function marginGet(self: MuxContainerTsStatsRelease): rt.i64 | undefined;
export declare function marginSet(self: MuxContainerTsStatsRelease, value: rt.i64 | undefined): void;
export declare function offsetGet(self: MuxContainerTsStatsRelease): rt.i64 | undefined;
export declare function offsetSet(self: MuxContainerTsStatsRelease, value: rt.i64 | undefined): void;
export declare function driftGet(self: MuxContainerTsStatsRelease): rt.f64 | undefined;
export declare function driftSet(self: MuxContainerTsStatsRelease, value: rt.f64 | undefined): void;
export declare function outOfToleranceGet(self: MuxContainerTsStatsRelease): rt.u64;
export declare function outOfToleranceSet(self: MuxContainerTsStatsRelease, value: rt.u64): void;
export { default$ as default };

// MuxContainerTsStatsReport.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsRelease } from "./MuxContainerTsStatsRelease.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsStatsReport = rt.Handle<"MuxContainerTsStatsReport">;
export declare function debug(self: MuxContainerTsStatsReport): string;
export declare function clone(self: MuxContainerTsStatsReport): MuxContainerTsStatsReport;
export declare function eq(a: MuxContainerTsStatsReport, b: MuxContainerTsStatsReport): boolean;
export declare function default$(): MuxContainerTsStatsReport;
export declare function mpegtsGet(self: MuxContainerTsStatsReport): MuxContainerTsStatsSnapshot | undefined;
export declare function mpegtsSet(self: MuxContainerTsStatsReport, value: MuxContainerTsStatsSnapshot | undefined): void;
export declare function releaseGet(self: MuxContainerTsStatsReport): MuxContainerTsStatsRelease | undefined;
export declare function releaseSet(self: MuxContainerTsStatsReport, value: MuxContainerTsStatsRelease | undefined): void;
export { default$ as default };

// MuxContainerTsStatsSnapshot.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsStream } from "./MuxContainerTsStatsStream.ts";
export type MuxContainerTsStatsSnapshot = rt.Handle<"MuxContainerTsStatsSnapshot">;
export declare function debug(self: MuxContainerTsStatsSnapshot): string;
export declare function clone(self: MuxContainerTsStatsSnapshot): MuxContainerTsStatsSnapshot;
export declare function eq(a: MuxContainerTsStatsSnapshot, b: MuxContainerTsStatsSnapshot): boolean;
export declare function default$(): MuxContainerTsStatsSnapshot;
export declare function streamsGet(self: MuxContainerTsStatsSnapshot): ([number, MuxContainerTsStatsStream])[];
export declare function streamsSet(self: MuxContainerTsStatsSnapshot, value: ([number, MuxContainerTsStatsStream])[]): void;
export declare function crcErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function crcErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function tsSyncLossGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function tsSyncLossSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function syncByteErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function syncByteErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function patErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function patErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function continuityCountErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function continuityCountErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function pmtErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function pmtErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function transportErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function transportErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function pcrRepetitionErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function pcrRepetitionErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function pcrDiscontinuityIndicatorErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function pcrDiscontinuityIndicatorErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function ptsErrorGet(self: MuxContainerTsStatsSnapshot): rt.u64;
export declare function ptsErrorSet(self: MuxContainerTsStatsSnapshot, value: rt.u64): void;
export declare function isEmpty(self: MuxContainerTsStatsSnapshot): boolean;
export { default$ as default };

// MuxContainerTsStatsStream.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsStatsClass } from "./MuxContainerTsStatsClass.ts";
export type MuxContainerTsStatsStream = rt.Handle<"MuxContainerTsStatsStream">;
export declare function debug(self: MuxContainerTsStatsStream): string;
export declare function clone(self: MuxContainerTsStatsStream): MuxContainerTsStatsStream;
export declare function eq(a: MuxContainerTsStatsStream, b: MuxContainerTsStatsStream): boolean;
export declare function default$(): MuxContainerTsStatsStream;
export declare function trackGet(self: MuxContainerTsStatsStream): string;
export declare function trackSet(self: MuxContainerTsStatsStream, value: string): void;
export declare function classGet(self: MuxContainerTsStatsStream): MuxContainerTsStatsClass;
export declare function classSet(self: MuxContainerTsStatsStream, value: MuxContainerTsStatsClass): void;
export declare function unitsGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function unitsSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function quietGet(self: MuxContainerTsStatsStream): rt.Duration | undefined;
export declare function quietSet(self: MuxContainerTsStatsStream, value: rt.Duration | undefined): void;
export declare function resyncsGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function resyncsSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function discardedGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function discardedSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function unconfirmedGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function unconfirmedSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function damagedGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function damagedSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function unplacedGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function unplacedSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function refusedGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function refusedSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function continuityCountErrorGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function continuityCountErrorSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function transportErrorGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function transportErrorSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export declare function ptsErrorGet(self: MuxContainerTsStatsStream): rt.u64;
export declare function ptsErrorSet(self: MuxContainerTsStatsStream, value: rt.u64): void;
export { default$ as default };

// MuxContainerTsStep.ts
import * as rt from "./rt.ts";
export type MuxContainerTsStep = {
    tag: "Rewind";
    by: rt.Duration;
    flagged: boolean;
} | {
    tag: "Leap";
    by: rt.Duration;
} | {
    tag: "Identity";
    transportStreamId: number;
    programNumber: number;
};
export declare function toString(self: MuxContainerTsStep): string;
export declare function debug(self: MuxContainerTsStep): string;
export declare function eq(a: MuxContainerTsStep, b: MuxContainerTsStep): boolean;

// MuxContainerTsTrack.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsDescriptor } from "./MuxContainerTsDescriptor.ts";
import type { MuxContainerTsPesPacket } from "./MuxContainerTsPesPacket.ts";
import type { MuxContainerTsVerbatim } from "./MuxContainerTsVerbatim.ts";
export type MuxContainerTsTrack = rt.Handle<"MuxContainerTsTrack">;
export declare function debug(self: MuxContainerTsTrack): string;
export declare function clone(self: MuxContainerTsTrack): MuxContainerTsTrack;
export declare function eq(a: MuxContainerTsTrack, b: MuxContainerTsTrack): boolean;
export declare function default$(): MuxContainerTsTrack;
export declare function pidGet(self: MuxContainerTsTrack): number;
export declare function pidSet(self: MuxContainerTsTrack, value: number): void;
export declare function descriptorsGet(self: MuxContainerTsTrack): MuxContainerTsDescriptor[];
export declare function descriptorsSet(self: MuxContainerTsTrack, value: MuxContainerTsDescriptor[]): void;
export declare function verbatimGet(self: MuxContainerTsTrack): MuxContainerTsVerbatim | undefined;
export declare function verbatimSet(self: MuxContainerTsTrack, value: MuxContainerTsVerbatim | undefined): void;
export declare function pesPacketGet(self: MuxContainerTsTrack): MuxContainerTsPesPacket | undefined;
export declare function pesPacketSet(self: MuxContainerTsTrack, value: MuxContainerTsPesPacket | undefined): void;
export declare function new$(pid: number): MuxContainerTsTrack;
export { default$ as default, new$ as new };

// MuxContainerTsVerbatim.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsFraming } from "./MuxContainerTsFraming.ts";
export type MuxContainerTsVerbatim = rt.Handle<"MuxContainerTsVerbatim">;
export declare function debug(self: MuxContainerTsVerbatim): string;
export declare function clone(self: MuxContainerTsVerbatim): MuxContainerTsVerbatim;
export declare function eq(a: MuxContainerTsVerbatim, b: MuxContainerTsVerbatim): boolean;
export declare function default$(): MuxContainerTsVerbatim;
export declare function streamTypeGet(self: MuxContainerTsVerbatim): number;
export declare function streamTypeSet(self: MuxContainerTsVerbatim, value: number): void;
export declare function framingGet(self: MuxContainerTsVerbatim): MuxContainerTsFraming;
export declare function framingSet(self: MuxContainerTsVerbatim, value: MuxContainerTsFraming): void;
export declare function new$(streamType: number, framing: MuxContainerTsFraming): MuxContainerTsVerbatim;
export { default$ as default, new$ as new };

// MuxError.ts
import * as rt from "./rt.ts";
export type MuxError = rt.Handle<"MuxError">;
export declare function toString(self: MuxError): string;
export declare function debug(self: MuxError): string;
export declare function clone(self: MuxError): MuxError;

// MuxFollowInstances.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxFollowInstances = rt.Handle<"MuxFollowInstances">;
export declare function next(self: MuxFollowInstances): rt.Call<rt.Result<BroadcastConsumer | undefined, MuxError>>;
export declare function values(self: MuxFollowInstances): rt.Stream<rt.Result<BroadcastConsumer, MuxError>>;

// MuxImportAudioFormat.ts
export type MuxImportAudioFormat = "Aac" | "Opus" | "Flac" | "Mp3";
export declare function toString(self: MuxImportAudioFormat): string;
export declare function debug(self: MuxImportAudioFormat): string;

// MuxImportAudioInit.ts
import * as rt from "./rt.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxImportAudioFormat } from "./MuxImportAudioFormat.ts";
export type MuxImportAudioInit = rt.Handle<"MuxImportAudioInit">;
export declare function debug(self: MuxImportAudioInit): string;
export declare function clone(self: MuxImportAudioInit): MuxImportAudioInit;
export declare function eq(a: MuxImportAudioInit, b: MuxImportAudioInit): boolean;
export declare function formatGet(self: MuxImportAudioInit): MuxImportAudioFormat;
export declare function formatSet(self: MuxImportAudioInit, value: MuxImportAudioFormat): void;
export declare function dataGet(self: MuxImportAudioInit): rt.Bytes;
export declare function dataSet(self: MuxImportAudioInit, value: Uint8Array | rt.Bytes): void;
export declare function labelGet(self: MuxImportAudioInit): string | undefined;
export declare function labelSet(self: MuxImportAudioInit, value: string | undefined): void;
export declare function containerGet(self: MuxImportAudioInit): HangCatalogContainer;
export declare function containerSet(self: MuxImportAudioInit, value: HangCatalogContainer): void;
export declare function bitrateGet(self: MuxImportAudioInit): rt.u64 | undefined;
export declare function bitrateSet(self: MuxImportAudioInit, value: rt.u64 | undefined): void;
export declare function new$(format: MuxImportAudioFormat, data: Uint8Array | rt.Bytes): MuxImportAudioInit;
export { new$ as new };

// MuxImportContainer.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportContainerFinished } from "./MuxImportContainerFinished.ts";
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainer = rt.Handle<"MuxImportContainer">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved, init: MuxImportContainerInit): rt.Result<MuxImportContainer, MuxError>;
export declare function decode(self: MuxImportContainer, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainer): rt.Result<MuxImportContainerFinished, MuxError>;
export declare function abort(self: MuxImportContainer, err: Error_): void;
export declare function cut(self: MuxImportContainer): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainer, start: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxImportContainerExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportContainerFinishedExt } from "./MuxImportContainerFinishedExt.ts";
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainerExt = rt.Handle<"MuxImportContainerExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt, init: MuxImportContainerInit): rt.Result<MuxImportContainerExt, MuxError>;
export declare function decode(self: MuxImportContainerExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainerExt): rt.Result<MuxImportContainerFinishedExt, MuxError>;
export declare function abort(self: MuxImportContainerExt, err: Error_): void;
export declare function cut(self: MuxImportContainerExt): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainerExt, start: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxImportContainerExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportContainerFinishedExtra } from "./MuxImportContainerFinishedExtra.ts";
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainerExtra = rt.Handle<"MuxImportContainerExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra, init: MuxImportContainerInit): rt.Result<MuxImportContainerExtra, MuxError>;
export declare function decode(self: MuxImportContainerExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainerExtra): rt.Result<MuxImportContainerFinishedExtra, MuxError>;
export declare function abort(self: MuxImportContainerExtra, err: Error_): void;
export declare function cut(self: MuxImportContainerExtra): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainerExtra, start: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxImportContainerFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxImportContainerFinished = rt.Handle<"MuxImportContainerFinished">;
export declare function abort(self: MuxImportContainerFinished, err: Error_): void;

// MuxImportContainerFinishedExt.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxImportContainerFinishedExt = rt.Handle<"MuxImportContainerFinishedExt">;
export declare function abort(self: MuxImportContainerFinishedExt, err: Error_): void;

// MuxImportContainerFinishedExtra.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxImportContainerFinishedExtra = rt.Handle<"MuxImportContainerFinishedExtra">;
export declare function abort(self: MuxImportContainerFinishedExtra, err: Error_): void;

// MuxImportContainerFormat.ts
export type MuxImportContainerFormat = "Fmp4" | "Mkv" | "Ts" | "Flv";
export declare function toString(self: MuxImportContainerFormat): string;
export declare function debug(self: MuxImportContainerFormat): string;

// MuxImportContainerInit.ts
import * as rt from "./rt.ts";
import type { MuxImportContainerFormat } from "./MuxImportContainerFormat.ts";
export type MuxImportContainerInit = rt.Handle<"MuxImportContainerInit">;
export declare function debug(self: MuxImportContainerInit): string;
export declare function clone(self: MuxImportContainerInit): MuxImportContainerInit;
export declare function eq(a: MuxImportContainerInit, b: MuxImportContainerInit): boolean;
export declare function formatGet(self: MuxImportContainerInit): MuxImportContainerFormat;
export declare function formatSet(self: MuxImportContainerInit, value: MuxImportContainerFormat): void;
export declare function dataGet(self: MuxImportContainerInit): rt.Bytes;
export declare function dataSet(self: MuxImportContainerInit, value: Uint8Array | rt.Bytes): void;
export declare function new$(format: MuxImportContainerFormat, data: Uint8Array | rt.Bytes): MuxImportContainerInit;
export { new$ as new };

// MuxImportFormat.ts
import * as rt from "./rt.ts";
import type { MuxImportAudioFormat } from "./MuxImportAudioFormat.ts";
import type { MuxImportContainerFormat } from "./MuxImportContainerFormat.ts";
import type { MuxImportKind } from "./MuxImportKind.ts";
import type { MuxImportVideoFormat } from "./MuxImportVideoFormat.ts";
export type MuxImportFormat = {
    tag: "Audio";
    _0: MuxImportAudioFormat;
} | {
    tag: "Video";
    _0: MuxImportVideoFormat;
} | {
    tag: "Container";
    _0: MuxImportContainerFormat;
};
export declare function toString(self: MuxImportFormat): string;
export declare function debug(self: MuxImportFormat): string;
export declare function eq(a: MuxImportFormat, b: MuxImportFormat): boolean;
export declare function hash(self: MuxImportFormat): rt.u64;
export declare function kind(self: MuxImportFormat): MuxImportKind;

// MuxImportKind.ts
export type MuxImportKind = "Audio" | "Video" | "Container";
export declare function debug(self: MuxImportKind): string;
export declare function name(self: MuxImportKind): string;

// MuxImportTrack.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportAudioInit } from "./MuxImportAudioInit.ts";
import type { MuxImportTrackFinished } from "./MuxImportTrackFinished.ts";
import type { MuxImportVideoInit } from "./MuxImportVideoInit.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackRequest } from "./TrackRequest.ts";
export type MuxImportTrack = rt.Handle<"MuxImportTrack">;
export declare function audioExtra(request: TrackRequest, reserved: MuxCatalogReservedExtra, init: MuxImportAudioInit): rt.Result<MuxImportTrack, MuxError>;
export declare function audioExt(request: TrackRequest, reserved: MuxCatalogReservedExt, init: MuxImportAudioInit): rt.Result<MuxImportTrack, MuxError>;
export declare function audioUnit(request: TrackRequest, reserved: MuxCatalogReserved, init: MuxImportAudioInit): rt.Result<MuxImportTrack, MuxError>;
export declare function videoExtra(request: TrackRequest, reserved: MuxCatalogReservedExtra, init: MuxImportVideoInit): rt.Result<MuxImportTrack, MuxError>;
export declare function videoExt(request: TrackRequest, reserved: MuxCatalogReservedExt, init: MuxImportVideoInit): rt.Result<MuxImportTrack, MuxError>;
export declare function videoUnit(request: TrackRequest, reserved: MuxCatalogReserved, init: MuxImportVideoInit): rt.Result<MuxImportTrack, MuxError>;
export declare function decode(self: MuxImportTrack, frame: Uint8Array | rt.Bytes, pts: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportTrack): rt.Result<MuxImportTrackFinished, MuxError>;
export declare function abort(self: MuxImportTrack, err: Error_): void;
export declare function abortUnwanted(self: MuxImportTrack, err: Error_): rt.Result<undefined, MuxImportTrack>;
export declare function cut(self: MuxImportTrack, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxImportTrack): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxImportTrack, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportTrack, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxImportTrack): TrackDemand;
export declare function name(self: MuxImportTrack): string;

// MuxImportTrackFinished.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type MuxImportTrackFinished = rt.Handle<"MuxImportTrackFinished">;
export declare function name(self: MuxImportTrackFinished): string;
export declare function abort(self: MuxImportTrackFinished, err: Error_): void;

// MuxImportTrackStream.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportTrackFinished } from "./MuxImportTrackFinished.ts";
import type { MuxImportVideoInit } from "./MuxImportVideoInit.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackRequest } from "./TrackRequest.ts";
export type MuxImportTrackStream = rt.Handle<"MuxImportTrackStream">;
export declare function videoExtra(request: TrackRequest, reserved: MuxCatalogReservedExtra, init: MuxImportVideoInit): rt.Result<MuxImportTrackStream, MuxError>;
export declare function videoExt(request: TrackRequest, reserved: MuxCatalogReservedExt, init: MuxImportVideoInit): rt.Result<MuxImportTrackStream, MuxError>;
export declare function videoUnit(request: TrackRequest, reserved: MuxCatalogReserved, init: MuxImportVideoInit): rt.Result<MuxImportTrackStream, MuxError>;
export declare function initialize(self: MuxImportTrackStream, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function decode(self: MuxImportTrackStream, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportTrackStream): rt.Result<MuxImportTrackFinished, MuxError>;
export declare function setEnabled(self: MuxImportTrackStream, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportTrackStream, err: Error_): void;
export declare function abortUnwanted(self: MuxImportTrackStream, err: Error_): rt.Result<undefined, MuxImportTrackStream>;
export declare function cut(self: MuxImportTrackStream, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportTrackStream, start: rt.u64): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxImportTrackStream): TrackDemand;
export declare function name(self: MuxImportTrackStream): string;

// MuxImportVideoFormat.ts
export type MuxImportVideoFormat = "Avc1" | "Avc3" | "Hvc1" | "Hev1" | "Av01" | "Vp8" | "Vp9";
export declare function toString(self: MuxImportVideoFormat): string;
export declare function debug(self: MuxImportVideoFormat): string;

// MuxImportVideoInit.ts
import * as rt from "./rt.ts";
import type { MuxCatalogVideoHint } from "./MuxCatalogVideoHint.ts";
import type { MuxImportVideoFormat } from "./MuxImportVideoFormat.ts";
export type MuxImportVideoInit = rt.Handle<"MuxImportVideoInit">;
export declare function debug(self: MuxImportVideoInit): string;
export declare function clone(self: MuxImportVideoInit): MuxImportVideoInit;
export declare function eq(a: MuxImportVideoInit, b: MuxImportVideoInit): boolean;
export declare function formatGet(self: MuxImportVideoInit): MuxImportVideoFormat;
export declare function formatSet(self: MuxImportVideoInit, value: MuxImportVideoFormat): void;
export declare function dataGet(self: MuxImportVideoInit): rt.Bytes;
export declare function dataSet(self: MuxImportVideoInit, value: Uint8Array | rt.Bytes): void;
export declare function labelGet(self: MuxImportVideoInit): string | undefined;
export declare function labelSet(self: MuxImportVideoInit, value: string | undefined): void;
export declare function hintGet(self: MuxImportVideoInit): MuxCatalogVideoHint;
export declare function hintSet(self: MuxImportVideoInit, value: MuxCatalogVideoHint): void;
export declare function priorityGet(self: MuxImportVideoInit): number;
export declare function prioritySet(self: MuxImportVideoInit, value: number): void;
export declare function new$(format: MuxImportVideoFormat, data: Uint8Array | rt.Bytes): MuxImportVideoInit;
export { new$ as new };

// MuxIngest.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthLimit } from "./BandwidthLimit.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
export type MuxIngest = rt.Handle<"MuxIngest">;
export declare function debug(self: MuxIngest): string;
export declare function clone(self: MuxIngest): MuxIngest;
export declare function default$(): MuxIngest;
export declare function maxAgeGet(self: MuxIngest): rt.Duration | undefined;
export declare function maxAgeSet(self: MuxIngest, value: rt.Duration | undefined): void;
export declare function bandwidthGet(self: MuxIngest): BandwidthAllocator;
export declare function bandwidthSet(self: MuxIngest, value: BandwidthAllocator): void;
export declare function lingerGet(self: MuxIngest): rt.Duration;
export declare function lingerSet(self: MuxIngest, value: rt.Duration): void;
export declare function rateGet(self: MuxIngest): BandwidthLimit | undefined;
export declare function rateSet(self: MuxIngest, value: BandwidthLimit | undefined): void;
export declare function catalog(self: MuxIngest): MuxCatalogConfig;
export { default$ as default };

// MuxJsonConfig.ts
import * as rt from "./rt.ts";
export type MuxJsonConfig = rt.Handle<"MuxJsonConfig">;
export declare function debug(self: MuxJsonConfig): string;
export declare function clone(self: MuxJsonConfig): MuxJsonConfig;
export declare function default$(): MuxJsonConfig;
export declare function schemaGet(self: MuxJsonConfig): string | undefined;
export declare function schemaSet(self: MuxJsonConfig, value: string | undefined): void;
export declare function withSchema(self: MuxJsonConfig, schema: string): MuxJsonConfig;
export { default$ as default };

// MuxJsonConsumerJson.ts
import * as rt from "./rt.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { MoqJsonTimedJson } from "./MoqJsonTimedJson.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
export type MuxJsonConsumerJson = rt.Handle<"MuxJsonConsumerJson">;
export declare function fromTrack(track: TrackConsumer, config: HangCatalogJsonConfig): rt.Call<rt.Result<MuxJsonConsumerJson, MuxError>>;
export declare function mode(self: MuxJsonConsumerJson): HangCatalogMode;
export declare function next(self: MuxJsonConsumerJson): rt.Call<rt.Result<MoqJsonTimedJson | undefined, MuxError>>;
export declare function latest(self: MuxJsonConsumerJson): rt.Call<rt.Result<MoqJsonTimedJson | undefined, MuxError>>;
export declare function values(self: MuxJsonConsumerJson): rt.Stream<rt.Result<MoqJsonTimedJson, MuxError>>;

// MuxJsonOptions.ts
import * as rt from "./rt.ts";
export type MuxJsonOptions = rt.Handle<"MuxJsonOptions">;
export declare function debug(self: MuxJsonOptions): string;
export declare function clone(self: MuxJsonOptions): MuxJsonOptions;
export declare function default$(): MuxJsonOptions;
export declare function deltasGet(self: MuxJsonOptions): boolean;
export declare function deltasSet(self: MuxJsonOptions, value: boolean): void;
export declare function withDeltas(self: MuxJsonOptions, deltas: boolean): MuxJsonOptions;
export { default$ as default };

// MuxJsonSnapshotJson.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Json } from "./Json.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxJsonSnapshotJson = rt.Handle<"MuxJsonSnapshotJson">;
export declare function name(self: MuxJsonSnapshotJson): string;
export declare function consume(self: MuxJsonSnapshotJson): TrackSubscriber;
export declare function demand(self: MuxJsonSnapshotJson): TrackDemand;
export declare function refresh(self: MuxJsonSnapshotJson): rt.Call<rt.Result<undefined, MuxError>>;
export declare function update(self: MuxJsonSnapshotJson, value: Json, timestamp: Timestamp): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxJsonSnapshotJson): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxJsonSnapshotJson, err: Error_): rt.Result<undefined, MuxError>;
export declare function abortUnwanted(self: MuxJsonSnapshotJson, err: Error_): rt.Result<undefined, MuxJsonSnapshotJson>;

// MuxJsonStreamJson.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Json } from "./Json.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxJsonStreamJson = rt.Handle<"MuxJsonStreamJson">;
export declare function name(self: MuxJsonStreamJson): string;
export declare function consume(self: MuxJsonStreamJson): TrackSubscriber;
export declare function demand(self: MuxJsonStreamJson): TrackDemand;
export declare function append(self: MuxJsonStreamJson, value: Json, timestamp: Timestamp): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxJsonStreamJson): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxJsonStreamJson, err: Error_): rt.Result<undefined, MuxError>;
export declare function abortUnwanted(self: MuxJsonStreamJson, err: Error_): rt.Result<undefined, MuxJsonStreamJson>;

// MuxLingerUnit.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
export type MuxLingerUnit = rt.Handle<"MuxLingerUnit">;
export declare function DEFAULT(): rt.Duration;
export declare function new$(broadcasts: BroadcastProducer[], duration: rt.Duration): MuxLingerUnit;
export declare function wait(self: MuxLingerUnit): rt.Call<void>;
export { new$ as new };

// MuxPacer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxPacer = rt.Handle<"MuxPacer">;
export declare function default$(): MuxPacer;
export declare function withLead(self: MuxPacer, lead: rt.Duration): MuxPacer;
export declare function withDelay(self: MuxPacer, delay: rt.Duration): MuxPacer;
export declare function pace(self: MuxPacer, ts: Timestamp, now: ClockInstant): ClockInstant;
export declare function due(self: MuxPacer, ts: Timestamp): ClockInstant | undefined;
export declare function slack(self: MuxPacer): rt.Duration;
export declare function hurry(self: MuxPacer, ts: Timestamp, now: ClockInstant): ClockInstant;
export { default$ as default };

// MuxRateControl.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxRatePolicy } from "./MuxRatePolicy.ts";
import type { MuxRateTarget } from "./MuxRateTarget.ts";
export type MuxRateControl = rt.Handle<"MuxRateControl">;
export declare function debug(self: MuxRateControl): string;
export declare function clone(self: MuxRateControl): MuxRateControl;
export declare function new$(policy: MuxRatePolicy): MuxRateControl;
export declare function target(self: MuxRateControl): MuxRateTarget;
export declare function setProbed(self: MuxRateControl, probed: boolean): void;
export declare function update(self: MuxRateControl, estimate: BandwidthRate | undefined, now: ClockInstant): MuxRateTarget | undefined;
export { new$ as new };

// MuxRateGrant.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxRatePolicy } from "./MuxRatePolicy.ts";
import type { MuxRateTarget } from "./MuxRateTarget.ts";
export type MuxRateGrant = rt.Handle<"MuxRateGrant">;
export declare function new$(grant: BandwidthConsumer, policy: MuxRatePolicy): MuxRateGrant;
export declare function assume(self: MuxRateGrant, target: MuxRateTarget): void;
export declare function policy(self: MuxRateGrant): MuxRatePolicy;
export declare function setPolicy(self: MuxRateGrant, policy: MuxRatePolicy): void;
export declare function next(self: MuxRateGrant): rt.Call<rt.Result<MuxRateTarget, Error_>>;
export declare function failed(self: MuxRateGrant): void;
export { new$ as new };

// MuxRatePolicy.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
export type MuxRatePolicy = rt.Handle<"MuxRatePolicy">;
export declare function debug(self: MuxRatePolicy): string;
export declare function clone(self: MuxRatePolicy): MuxRatePolicy;
export declare function eq(a: MuxRatePolicy, b: MuxRatePolicy): boolean;
export declare function maxGet(self: MuxRatePolicy): BandwidthRate;
export declare function maxSet(self: MuxRatePolicy, value: BandwidthRate): void;
export declare function minGet(self: MuxRatePolicy): BandwidthRate;
export declare function minSet(self: MuxRatePolicy, value: BandwidthRate): void;
export declare function lowerGet(self: MuxRatePolicy): BandwidthRate | undefined;
export declare function lowerSet(self: MuxRatePolicy, value: BandwidthRate | undefined): void;
export declare function new$(max: BandwidthRate): MuxRatePolicy;
export { new$ as new };

// MuxRateTarget.ts
import type { BandwidthRate } from "./BandwidthRate.ts";
export type MuxRateTarget = {
    tag: "Enabled";
    _0: BandwidthRate;
} | {
    tag: "Disabled";
};
export declare function debug(self: MuxRateTarget): string;
export declare function eq(a: MuxRateTarget, b: MuxRateTarget): boolean;

// MuxSelectAudio.ts
import * as rt from "./rt.ts";
import type { HangCatalogAudioCodecKind } from "./HangCatalogAudioCodecKind.ts";
export type MuxSelectAudio = rt.Handle<"MuxSelectAudio">;
export declare function debug(self: MuxSelectAudio): string;
export declare function clone(self: MuxSelectAudio): MuxSelectAudio;
export declare function default$(): MuxSelectAudio;
export declare function name(self: MuxSelectAudio, name: string): MuxSelectAudio;
export declare function codec(self: MuxSelectAudio, codec: HangCatalogAudioCodecKind): MuxSelectAudio;
export { default$ as default };

// MuxSelectBroadcast.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxSelectAudio } from "./MuxSelectAudio.ts";
import type { MuxSelectVideo } from "./MuxSelectVideo.ts";
export type MuxSelectBroadcast = rt.Handle<"MuxSelectBroadcast">;
export declare function debug(self: MuxSelectBroadcast): string;
export declare function clone(self: MuxSelectBroadcast): MuxSelectBroadcast;
export declare function default$(): MuxSelectBroadcast;
export declare function video(self: MuxSelectBroadcast, video: MuxSelectVideo): MuxSelectBroadcast;
export declare function audio(self: MuxSelectBroadcast, audio: MuxSelectAudio): MuxSelectBroadcast;
export declare function hasVideo(self: MuxSelectBroadcast): boolean;
export declare function hasAudio(self: MuxSelectBroadcast): boolean;
export declare function retainExtra(self: MuxSelectBroadcast, catalog: HangCatalogExtra): void;
export declare function retainExt(self: MuxSelectBroadcast, catalog: HangCatalogExt): void;
export declare function retainUnit(self: MuxSelectBroadcast, catalog: HangCatalog): void;
export { default$ as default };

// MuxSelectVideo.ts
import * as rt from "./rt.ts";
import type { HangCatalogVideoCodecKind } from "./HangCatalogVideoCodecKind.ts";
export type MuxSelectVideo = rt.Handle<"MuxSelectVideo">;
export declare function debug(self: MuxSelectVideo): string;
export declare function clone(self: MuxSelectVideo): MuxSelectVideo;
export declare function default$(): MuxSelectVideo;
export declare function name(self: MuxSelectVideo, name: string): MuxSelectVideo;
export declare function codec(self: MuxSelectVideo, codec: HangCatalogVideoCodecKind): MuxSelectVideo;
export { default$ as default };

// MuxSource.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { HangEchoRendition } from "./HangEchoRendition.ts";
import type { Instance } from "./Instance.ts";
import type { MuxBinding } from "./MuxBinding.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxCatalogConsumer } from "./MuxCatalogConsumer.ts";
import type { MuxCatalogConsumerExt } from "./MuxCatalogConsumerExt.ts";
import type { MuxCatalogConsumerExtra } from "./MuxCatalogConsumerExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxFollowInstances } from "./MuxFollowInstances.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { Path } from "./Path.ts";
import type { PathRelative } from "./PathRelative.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxSource = rt.Handle<"MuxSource">;
export declare function clone(self: MuxSource): MuxSource;
export declare function new$(origin: OriginConsumer, broadcast: BroadcastConsumer): MuxSource;
export declare function stats(self: MuxSource): ([string, HangEchoRendition])[];
export declare function broadcast(self: MuxSource): BroadcastConsumer;
export declare function catalogExtra(self: MuxSource, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumerExtra, MuxError>>;
export declare function catalogExt(self: MuxSource, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumerExt, MuxError>>;
export declare function catalogUnit(self: MuxSource, format: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxCatalogConsumer, MuxError>>;
export declare function resolveReference(self: MuxSource, rel: PathRelative | undefined): Path | undefined;
export declare function resolve(self: MuxSource, rel: PathRelative | undefined, instance: Instance | undefined): rt.Call<rt.Result<BroadcastConsumer, MuxError>>;
export declare function bind(self: MuxSource, rel: PathRelative | undefined, instance: Instance | undefined): rt.Result<MuxBinding, MuxError>;
export declare function subscribeTrack(self: MuxSource, rel: PathRelative | undefined, instance: Instance | undefined, name: string): rt.Call<rt.Result<TrackSubscriber, MuxError>>;
export declare function follow(self: MuxSource, rel: PathRelative | undefined, instance: Instance | undefined): rt.Result<MuxFollowInstances, MuxError>;
export { new$ as new };

// MuxSyncConsumer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxSyncSchedule } from "./MuxSyncSchedule.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxSyncConsumer = rt.Handle<"MuxSyncConsumer">;
export declare function clone(self: MuxSyncConsumer): MuxSyncConsumer;
export declare function delay(self: MuxSyncConsumer): rt.Duration;
export declare function due(self: MuxSyncConsumer, timestamp: Timestamp): ClockInstant | undefined;
export declare function position(self: MuxSyncConsumer, at: ClockInstant): rt.Duration | undefined;
export declare function schedule(self: MuxSyncConsumer): MuxSyncSchedule;
export declare function changed(self: MuxSyncConsumer, seen: MuxSyncSchedule): rt.Call<MuxSyncSchedule>;
export declare function present(self: MuxSyncConsumer, timestamps: Timestamp[], now: ClockInstant): rt.u64 | undefined;

// MuxSyncProducer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxContainerKind } from "./MuxContainerKind.ts";
import type { MuxSyncConsumer } from "./MuxSyncConsumer.ts";
import type { MuxSyncTrack } from "./MuxSyncTrack.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxSyncProducer = rt.Handle<"MuxSyncProducer">;
export declare function clone(self: MuxSyncProducer): MuxSyncProducer;
export declare function new$(delay: rt.Duration): MuxSyncProducer;
export declare function reset(self: MuxSyncProducer, delay: rt.Duration): MuxSyncProducer;
export declare function consume(self: MuxSyncProducer): MuxSyncConsumer;
export declare function track(self: MuxSyncProducer, kind: MuxContainerKind): MuxSyncTrack;
export declare function due(self: MuxSyncProducer, timestamp: Timestamp): ClockInstant | undefined;
export { new$ as new };

// MuxSyncSchedule.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxSyncSchedule = rt.Handle<"MuxSyncSchedule">;
export declare function debug(self: MuxSyncSchedule): string;
export declare function clone(self: MuxSyncSchedule): MuxSyncSchedule;
export declare function eq(a: MuxSyncSchedule, b: MuxSyncSchedule): boolean;
export declare function delay(self: MuxSyncSchedule): rt.Duration;
export declare function due(self: MuxSyncSchedule, timestamp: Timestamp): ClockInstant | undefined;

// MuxSyncTrack.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxSyncTrack = rt.Handle<"MuxSyncTrack">;
export declare function received(self: MuxSyncTrack, timestamp: Timestamp, now: ClockInstant): boolean;
export declare function reset(self: MuxSyncTrack): void;
export declare function setTarget(self: MuxSyncTrack, target: rt.Duration): boolean;
export declare function delayChanged(self: MuxSyncTrack, seen: rt.Duration): rt.Call<rt.Duration>;

// OriginClaim.ts
import * as rt from "./rt.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { Error_ } from "./Error_.ts";
import type { Instance } from "./Instance.ts";
import type { OriginCost } from "./OriginCost.ts";
import type { OriginRequest } from "./OriginRequest.ts";
export type OriginClaim = rt.Handle<"OriginClaim">;
export declare function clone(self: OriginClaim): OriginClaim;
export declare function instance(self: OriginClaim): Instance;
export declare function announce(self: OriginClaim): rt.Result<undefined, Error_>;
export declare function withdraw(self: OriginClaim): void;
export declare function isAnnounced(self: OriginClaim): boolean;
export declare function setCost(self: OriginClaim, cost: OriginCost): rt.Result<undefined, Error_>;
export declare function setAuth(self: OriginClaim, auth: AuthCredential | undefined): rt.Result<undefined, Error_>;
export declare function requested(self: OriginClaim): rt.Call<OriginRequest | undefined>;

// OriginConfig.ts
import * as rt from "./rt.ts";
import type { CachePool } from "./CachePool.ts";
export type OriginConfig = rt.Handle<"OriginConfig">;
export declare function debug(self: OriginConfig): string;
export declare function clone(self: OriginConfig): OriginConfig;
export declare function default$(): OriginConfig;
export declare function nodeGet(self: OriginConfig): rt.u64;
export declare function nodeSet(self: OriginConfig, value: rt.u64): void;
export declare function maxHopsGet(self: OriginConfig): rt.u64;
export declare function maxHopsSet(self: OriginConfig, value: rt.u64): void;
export declare function poolGet(self: OriginConfig): CachePool;
export declare function poolSet(self: OriginConfig, value: CachePool): void;
export declare function cacheDurationGet(self: OriginConfig): rt.Duration | undefined;
export declare function cacheDurationSet(self: OriginConfig, value: rt.Duration | undefined): void;
export declare function new$(node: rt.u64): OriginConfig;
export { default$ as default, new$ as new };

// OriginConsumer.ts
import * as rt from "./rt.ts";
import type { AnnounceConsumer } from "./AnnounceConsumer.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { Error_ } from "./Error_.ts";
import type { Instance } from "./Instance.ts";
import type { OriginFollowing } from "./OriginFollowing.ts";
import type { OriginRoutes } from "./OriginRoutes.ts";
import type { Path } from "./Path.ts";
import type { Patterns } from "./Patterns.ts";
import type { StatsSession } from "./StatsSession.ts";
export type OriginConsumer = rt.Handle<"OriginConsumer">;
export declare function clone(self: OriginConsumer): OriginConsumer;
export declare function node(self: OriginConsumer): rt.u64;
export declare function withHidden(self: OriginConsumer, hidden: boolean): OriginConsumer;
export declare function covering(self: OriginConsumer): OriginConsumer;
export declare function exclude(self: OriginConsumer, prefix: string): rt.Result<OriginConsumer, Error_>;
export declare function withStats(self: OriginConsumer, session: StatsSession): OriginConsumer;
export declare function announced(self: OriginConsumer): AnnounceConsumer;
export declare function routes(self: OriginConsumer): OriginRoutes;
export declare function consume(self: OriginConsumer): OriginConsumer;
export declare function follow(self: OriginConsumer, path: string, instance: Instance | undefined): rt.Result<OriginFollowing, Error_>;
export declare function scope(self: OriginConsumer, root: string, patterns: Patterns): rt.Result<OriginConsumer, Error_>;
export declare function request(self: OriginConsumer, path: string, instance: Instance | undefined): rt.Call<rt.Result<BroadcastConsumer, Error_>>;
export declare function root(self: OriginConsumer): Path;
export declare function allowed(self: OriginConsumer): Patterns;
export declare function absolute(self: OriginConsumer, path: string): rt.Result<Path, Error_>;

// OriginCost.ts
import * as rt from "./rt.ts";
export type OriginCost = rt.Opaque<"OriginCost">;
export declare function debug(self: OriginCost): string;
export declare function eq(a: OriginCost, b: OriginCost): boolean;
export declare function partialCmp(a: OriginCost, b: OriginCost): number | undefined;
export declare function cmp(a: OriginCost, b: OriginCost): number;
export declare function default$(): OriginCost;
export declare function new$(cost: rt.u64): OriginCost;
export declare function value(self: OriginCost): rt.u64;
export { default$ as default, new$ as new };

// OriginDriver.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Error_ } from "./Error_.ts";
import type { Waiter } from "./Waiter.ts";
export type OriginDriver = rt.Handle<"OriginDriver">;
export declare function poll(self: OriginDriver, now: ClockInstant, waiter: Waiter): rt.Result<ClockInstant | undefined, Error_>;

// OriginFollowing.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { Error_ } from "./Error_.ts";
export type OriginFollowing = rt.Handle<"OriginFollowing">;
export declare function available(self: OriginFollowing): rt.Call<rt.Result<BroadcastConsumer | undefined, Error_>>;

// OriginLink.ts
export type OriginLink = "Local" | "Peer" | "Upstream";
export declare function debug(self: OriginLink): string;
export declare function default$(): OriginLink;
export { default$ as default };

// OriginPriority.ts
import * as rt from "./rt.ts";
export type OriginPriority = rt.Opaque<"OriginPriority">;
export declare function debug(self: OriginPriority): string;
export declare function eq(a: OriginPriority, b: OriginPriority): boolean;
export declare function hash(self: OriginPriority): rt.u64;
export declare function partialCmp(a: OriginPriority, b: OriginPriority): number | undefined;
export declare function cmp(a: OriginPriority, b: OriginPriority): number;
export declare function default$(): OriginPriority;
export declare function new$(priority: number): OriginPriority;
export declare function value(self: OriginPriority): number;
export { default$ as default, new$ as new };

// OriginProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginClaim } from "./OriginClaim.ts";
import type { OriginConfig } from "./OriginConfig.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginDriver } from "./OriginDriver.ts";
import type { OriginLink } from "./OriginLink.ts";
import type { OriginPriority } from "./OriginPriority.ts";
import type { OriginRoute } from "./OriginRoute.ts";
import type { OriginRouteOptions } from "./OriginRouteOptions.ts";
import type { Path } from "./Path.ts";
import type { Patterns } from "./Patterns.ts";
import type { StatsSession } from "./StatsSession.ts";
export type OriginProducer = rt.Handle<"OriginProducer">;
export declare function clone(self: OriginProducer): OriginProducer;
export declare function new$(config: OriginConfig): [OriginProducer, OriginDriver];
export declare function withStats(self: OriginProducer, session: StatsSession): OriginProducer;
export declare function withLink(self: OriginProducer, link: OriginLink): OriginProducer;
export declare function withPriority(self: OriginProducer, priority: OriginPriority): OriginProducer;
export declare function withProxy(self: OriginProducer, proxy: boolean): OriginProducer;
export declare function config(self: OriginProducer): OriginConfig;
export declare function node(self: OriginProducer): rt.u64;
export declare function isOwn(self: OriginProducer, id: rt.u64): boolean;
export declare function route(self: OriginProducer, path: string, options: OriginRouteOptions): rt.Result<OriginRoute, Error_>;
export declare function claim(self: OriginProducer, prefix: string, options: OriginRouteOptions): rt.Result<OriginClaim, Error_>;
export declare function scope(self: OriginProducer, root: string, patterns: Patterns): rt.Result<OriginProducer, Error_>;
export declare function exclude(self: OriginProducer, prefix: string): rt.Result<OriginProducer, Error_>;
export declare function mount(self: OriginProducer, at: string, target: string): rt.Result<OriginProducer, Error_>;
export declare function consume(self: OriginProducer): OriginConsumer;
export declare function root(self: OriginProducer): Path;
export declare function allowed(self: OriginProducer): Patterns;
export declare function absolute(self: OriginProducer, path: string): rt.Result<Path, Error_>;
export { new$ as new };

// OriginRequest.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginRoute } from "./OriginRoute.ts";
import type { OriginRouteOptions } from "./OriginRouteOptions.ts";
import type { Path } from "./Path.ts";
export type OriginRequest = rt.Handle<"OriginRequest">;
export declare function path(self: OriginRequest): Path;
export declare function route(self: OriginRequest, options: OriginRouteOptions): rt.Result<OriginRoute, Error_>;
export declare function serveProducer(self: OriginRequest, broadcast: BroadcastProducer): void;
export declare function serveConsumer(self: OriginRequest, broadcast: BroadcastConsumer): void;
export declare function reject(self: OriginRequest, err: Error_): void;

// OriginRoute.ts
import * as rt from "./rt.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { Instance } from "./Instance.ts";
import type { OriginCost } from "./OriginCost.ts";
export type OriginRoute = rt.Handle<"OriginRoute">;
export declare function clone(self: OriginRoute): OriginRoute;
export declare function broadcast(self: OriginRoute): BroadcastProducer;
export declare function instance(self: OriginRoute): Instance;
export declare function announce(self: OriginRoute): rt.Result<undefined, Error_>;
export declare function withdraw(self: OriginRoute): void;
export declare function isAnnounced(self: OriginRoute): boolean;
export declare function setCost(self: OriginRoute, cost: OriginCost): rt.Result<undefined, Error_>;
export declare function setAuth(self: OriginRoute, auth: AuthCredential | undefined): rt.Result<undefined, Error_>;

// OriginRouteEvent.ts
import * as rt from "./rt.ts";
import type { AnnounceAnnounce } from "./AnnounceAnnounce.ts";
import type { OriginRouteKind } from "./OriginRouteKind.ts";
export type OriginRouteEvent = rt.Handle<"OriginRouteEvent">;
export declare function debug(self: OriginRouteEvent): string;
export declare function clone(self: OriginRouteEvent): OriginRouteEvent;
export declare function kindGet(self: OriginRouteEvent): OriginRouteKind;
export declare function kindSet(self: OriginRouteEvent, value: OriginRouteKind): void;
export declare function announceGet(self: OriginRouteEvent): AnnounceAnnounce;
export declare function announceSet(self: OriginRouteEvent, value: AnnounceAnnounce): void;

// OriginRouteInfo.ts
import * as rt from "./rt.ts";
import type { Instance } from "./Instance.ts";
import type { OriginCost } from "./OriginCost.ts";
export type OriginRouteInfo = rt.Handle<"OriginRouteInfo">;
export declare function debug(self: OriginRouteInfo): string;
export declare function clone(self: OriginRouteInfo): OriginRouteInfo;
export declare function eq(a: OriginRouteInfo, b: OriginRouteInfo): boolean;
export declare function instance(self: OriginRouteInfo): Instance;
export declare function cost(self: OriginRouteInfo): OriginCost;
export declare function available(self: OriginRouteInfo): boolean;

// OriginRouteKind.ts
export type OriginRouteKind = "Start" | "Update" | "End" | "Offline" | "Restart";
export declare function debug(self: OriginRouteKind): string;

// OriginRouteOptions.ts
import * as rt from "./rt.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { Instance } from "./Instance.ts";
import type { OriginCost } from "./OriginCost.ts";
export type OriginRouteOptions = rt.Handle<"OriginRouteOptions">;
export declare function debug(self: OriginRouteOptions): string;
export declare function clone(self: OriginRouteOptions): OriginRouteOptions;
export declare function default$(): OriginRouteOptions;
export declare function hidden(): OriginRouteOptions;
export declare function withCost(self: OriginRouteOptions, cost: OriginCost): OriginRouteOptions;
export declare function withAuth(self: OriginRouteOptions, auth: AuthCredential): OriginRouteOptions;
export declare function withInstance(self: OriginRouteOptions, instance: Instance, epoch: rt.u64): OriginRouteOptions;
export { default$ as default };

// OriginRoutes.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginRouteEvent } from "./OriginRouteEvent.ts";
import type { Path } from "./Path.ts";
export type OriginRoutes = rt.Handle<"OriginRoutes">;
export declare function next(self: OriginRoutes): rt.Call<OriginRouteEvent | undefined>;
export declare function tryNext(self: OriginRoutes): OriginRouteEvent | undefined;
export declare function isClosed(self: OriginRoutes): boolean;
export declare function pollSynced(self: OriginRoutes): rt.Call<void>;
export declare function root(self: OriginRoutes): Path;
export declare function absolute(self: OriginRoutes, prefix: string): rt.Result<Path, Error_>;
export declare function values(self: OriginRoutes): rt.Stream<OriginRouteEvent>;

// Park.ts
import * as rt from "./rt.ts";
import type { Waiter } from "./Waiter.ts";
export type Park = rt.Handle<"Park">;
export declare function debug(self: Park): string;
export declare function clone(self: Park): Park;
export declare function default$(): Park;
export declare function new$(waiter: Waiter): Park;
export declare function hold(self: Park, cx: () => void): Waiter;
export { default$ as default, new$ as new };

// Path.ts
import * as rt from "./rt.ts";
import type { PathError } from "./PathError.ts";
import type { PathRelative } from "./PathRelative.ts";
export type Path = rt.Handle<"Path">;
export declare function toString(self: Path): string;
export declare function debug(self: Path): string;
export declare function clone(self: Path): Path;
export declare function eq(a: Path, b: Path): boolean;
export declare function hash(self: Path): rt.u64;
export declare function partialCmp(a: Path, b: Path): number | undefined;
export declare function cmp(a: Path, b: Path): number;
export declare function default$(): Path;
export declare function new$(s: string): rt.Result<Path, PathError>;
export declare function fromStatic(s: string): Path;
export declare function hasPrefix(self: Path, prefix: Path): boolean;
export declare function stripPrefix(self: Path, prefix: Path): Path | undefined;
export declare function head(self: Path, n: rt.u64): Path | undefined;
export declare function asStr(self: Path): string;
export declare function empty(): Path;
export declare function isEmpty(self: Path): boolean;
export declare function len(self: Path): rt.u64;
export declare function join(self: Path, other: Path): rt.Result<Path, PathError>;
export declare function resolve(self: Path, rel: PathRelative): rt.Result<Path, PathError>;
export declare function tryResolve(self: Path, rel: PathRelative): Path | undefined;
export declare function relative(self: Path, base: Path): PathRelative | undefined;
export { default$ as default, new$ as new };

// PathError.ts
export type PathError = "TooLong" | "EmptySegment" | "InvalidEscape";
export declare function toString(self: PathError): string;
export declare function debug(self: PathError): string;

// PathRelative.ts
import * as rt from "./rt.ts";
import type { PathError } from "./PathError.ts";
export type PathRelative = rt.Handle<"PathRelative">;
export declare function toString(self: PathRelative): string;
export declare function debug(self: PathRelative): string;
export declare function clone(self: PathRelative): PathRelative;
export declare function eq(a: PathRelative, b: PathRelative): boolean;
export declare function hash(self: PathRelative): rt.u64;
export declare function default$(): PathRelative;
export declare function new$(s: string): rt.Result<PathRelative, PathError>;
export declare function asStr(self: PathRelative): string;
export declare function empty(): PathRelative;
export declare function isEmpty(self: PathRelative): boolean;
export declare function len(self: PathRelative): rt.u64;
export { default$ as default, new$ as new };

// Pattern.ts
import * as rt from "./rt.ts";
import type { IntersectionError } from "./IntersectionError.ts";
import type { InvalidPattern } from "./InvalidPattern.ts";
import type { Patterns } from "./Patterns.ts";
import type { Segment } from "./Segment.ts";
import type { Specificity } from "./Specificity.ts";
export type Pattern = rt.Handle<"Pattern">;
export declare function toString(self: Pattern): string;
export declare function debug(self: Pattern): string;
export declare function clone(self: Pattern): Pattern;
export declare function eq(a: Pattern, b: Pattern): boolean;
export declare function hash(self: Pattern): rt.u64;
export declare function partialCmp(a: Pattern, b: Pattern): number | undefined;
export declare function cmp(a: Pattern, b: Pattern): number;
export declare function default$(): Pattern;
export declare function new$(segments: Segment[]): rt.Result<Pattern, InvalidPattern>;
export declare function all(): Pattern;
export declare function literal(path: string): rt.Result<Pattern, InvalidPattern>;
export declare function subtree(path: string): rt.Result<Pattern, InvalidPattern>;
export declare function asStr(self: Pattern): string;
export declare function segments(self: Pattern): Segment[];
export declare function head(self: Pattern): string;
export declare function isLiteral(self: Pattern): boolean;
export declare function asPrefix(self: Pattern): string | undefined;
export declare function hasGlobstar(self: Pattern): boolean;
export declare function matches(self: Pattern, path: string): boolean;
export declare function contains(self: Pattern, other: Pattern): boolean;
export declare function overlaps(self: Pattern, other: Pattern): boolean;
export declare function specificity(self: Pattern): Specificity;
export declare function rebase(self: Pattern, root: string): Patterns;
export declare function intersect(self: Pattern, other: Pattern): rt.Result<Patterns, IntersectionError>;
export declare function captures(self: Pattern, matched: Pattern): Pattern[] | undefined;
export declare function rooted(self: Pattern, root: string): rt.Result<Pattern, InvalidPattern>;
export { default$ as default, new$ as new };

// Patterns.ts
import * as rt from "./rt.ts";
import type { IntersectionError } from "./IntersectionError.ts";
import type { InvalidPattern } from "./InvalidPattern.ts";
import type { Pattern } from "./Pattern.ts";
export type Patterns = rt.Handle<"Patterns">;
export declare function debug(self: Patterns): string;
export declare function clone(self: Patterns): Patterns;
export declare function eq(a: Patterns, b: Patterns): boolean;
export declare function hash(self: Patterns): rt.u64;
export declare function default$(): Patterns;
export declare function new$(): Patterns;
export declare function insert(self: Patterns, pattern: Pattern): boolean;
export declare function matches(self: Patterns, path: string): boolean;
export declare function contains(self: Patterns, pattern: Pattern): boolean;
export declare function covers(self: Patterns, other: Patterns): boolean;
export declare function overlaps(self: Patterns, pattern: Pattern): boolean;
export declare function intersect(self: Patterns, other: Patterns): rt.Result<Patterns, IntersectionError>;
export declare function rebase(self: Patterns, root: string): Patterns;
export declare function rooted(self: Patterns, root: string): rt.Result<Patterns, InvalidPattern>;
export declare function iter(self: Patterns): Pattern[];
export declare function len(self: Patterns): rt.u64;
export declare function isEmpty(self: Patterns): boolean;
export { default$ as default, new$ as new };

// QmuxSession.ts
import * as rt from "./rt.ts";
import type { MoqQuicConnectionMuxVersion } from "./MoqQuicConnectionMuxVersion.ts";
import type { MoqQuicLimits } from "./MoqQuicLimits.ts";
import type { RustlsPkiTypesCertificateDer } from "./RustlsPkiTypesCertificateDer.ts";
export type QmuxSession = rt.Handle<"QmuxSession">;
export declare function debug(self: QmuxSession): string;
export declare function clone(self: QmuxSession): QmuxSession;
export declare function protocol(self: QmuxSession): string | undefined;
export declare function version(self: QmuxSession): MoqQuicConnectionMuxVersion;
export declare function setLimits(self: QmuxSession, limits: MoqQuicLimits): rt.Result<undefined, string>;
export declare function peerCertificates(self: QmuxSession): RustlsPkiTypesCertificateDer[] | undefined;
export declare function serverName(self: QmuxSession): string | undefined;

// ReconnectBackoff.ts
import * as rt from "./rt.ts";
export type ReconnectBackoff = rt.Handle<"ReconnectBackoff">;
export declare function debug(self: ReconnectBackoff): string;
export declare function clone(self: ReconnectBackoff): ReconnectBackoff;
export declare function eq(a: ReconnectBackoff, b: ReconnectBackoff): boolean;
export declare function default$(): ReconnectBackoff;
export declare function initialGet(self: ReconnectBackoff): rt.Duration;
export declare function initialSet(self: ReconnectBackoff, value: rt.Duration): void;
export declare function multiplierGet(self: ReconnectBackoff): number;
export declare function multiplierSet(self: ReconnectBackoff, value: number): void;
export declare function maxGet(self: ReconnectBackoff): rt.Duration | undefined;
export declare function maxSet(self: ReconnectBackoff, value: rt.Duration | undefined): void;
export declare function healthyGet(self: ReconnectBackoff): rt.Duration;
export declare function healthySet(self: ReconnectBackoff, value: rt.Duration): void;
export declare function timeoutGet(self: ReconnectBackoff): rt.Duration | undefined;
export declare function timeoutSet(self: ReconnectBackoff, value: rt.Duration | undefined): void;
export { default$ as default };

// ReconnectConfig.ts
import * as rt from "./rt.ts";
import type { ReconnectBackoff } from "./ReconnectBackoff.ts";
import type { ReconnectHandover } from "./ReconnectHandover.ts";
import type { TokioRestart } from "./TokioRestart.ts";
export type ReconnectConfig = rt.Handle<"ReconnectConfig">;
export declare function debug(self: ReconnectConfig): string;
export declare function clone(self: ReconnectConfig): ReconnectConfig;
export declare function eq(a: ReconnectConfig, b: ReconnectConfig): boolean;
export declare function default$(): ReconnectConfig;
export declare function backoffGet(self: ReconnectConfig): ReconnectBackoff;
export declare function backoffSet(self: ReconnectConfig, value: ReconnectBackoff): void;
export declare function goawayGet(self: ReconnectConfig): ReconnectHandover;
export declare function goawaySet(self: ReconnectConfig, value: ReconnectHandover): void;
export declare function restartGet(self: ReconnectConfig): TokioRestart;
export declare function restartSet(self: ReconnectConfig, value: TokioRestart): void;
export declare function timeoutGet(self: ReconnectConfig): rt.Duration | undefined;
export declare function timeoutSet(self: ReconnectConfig, value: rt.Duration | undefined): void;
export declare function onceGet(self: ReconnectConfig): boolean;
export declare function onceSet(self: ReconnectConfig, value: boolean): void;
export declare function pinnedGet(self: ReconnectConfig): boolean;
export declare function pinnedSet(self: ReconnectConfig, value: boolean): void;
export { default$ as default };

// ReconnectConfigError.ts
export type ReconnectConfigError = "Empty" | "Healthy";
export declare function toString(self: ReconnectConfigError): string;
export declare function debug(self: ReconnectConfigError): string;

// ReconnectHandover.ts
import * as rt from "./rt.ts";
import type { ReconnectRedirect } from "./ReconnectRedirect.ts";
export type ReconnectHandover = rt.Handle<"ReconnectHandover">;
export declare function debug(self: ReconnectHandover): string;
export declare function clone(self: ReconnectHandover): ReconnectHandover;
export declare function eq(a: ReconnectHandover, b: ReconnectHandover): boolean;
export declare function default$(): ReconnectHandover;
export declare function redirectGet(self: ReconnectHandover): ReconnectRedirect;
export declare function redirectSet(self: ReconnectHandover, value: ReconnectRedirect): void;
export declare function handoverGet(self: ReconnectHandover): rt.Duration;
export declare function handoverSet(self: ReconnectHandover, value: rt.Duration): void;
export { default$ as default };

// ReconnectRedirect.ts
export type ReconnectRedirect = "Follow" | "SameHost" | "Ignore";
export declare function debug(self: ReconnectRedirect): string;
export declare function default$(): ReconnectRedirect;
export { default$ as default };

// ReconnectVerdict.ts
export type ReconnectVerdict = "Retry" | "Unusable" | "Settled";
export declare function debug(self: ReconnectVerdict): string;

// Retry.ts
export type Retry = "Same" | "Reroute" | "Never";
export declare function debug(self: Retry): string;

// RustlsCertCompressor.ts
import * as rt from "./rt.ts";
export type RustlsCertCompressor = rt.Handle<"RustlsCertCompressor">;

// RustlsCertDecompressor.ts
import * as rt from "./rt.ts";
export type RustlsCertDecompressor = rt.Handle<"RustlsCertDecompressor">;

// RustlsClientConfig.ts
import * as rt from "./rt.ts";
import type { RustlsCertCompressor } from "./RustlsCertCompressor.ts";
import type { RustlsCertDecompressor } from "./RustlsCertDecompressor.ts";
import type { RustlsClientDangerDangerousClientConfig } from "./RustlsClientDangerDangerousClientConfig.ts";
import type { RustlsClientResumption } from "./RustlsClientResumption.ts";
import type { RustlsClientTicketRequest } from "./RustlsClientTicketRequest.ts";
import type { RustlsCompressionCache } from "./RustlsCompressionCache.ts";
import type { RustlsCryptoProvider } from "./RustlsCryptoProvider.ts";
import type { RustlsKeyLog } from "./RustlsKeyLog.ts";
import type { RustlsResolvesClientCert } from "./RustlsResolvesClientCert.ts";
import type { RustlsTimeProvider } from "./RustlsTimeProvider.ts";
export type RustlsClientConfig = rt.Handle<"RustlsClientConfig">;
export declare function debug(self: RustlsClientConfig): string;
export declare function clone(self: RustlsClientConfig): RustlsClientConfig;
export declare function alpnProtocolsGet(self: RustlsClientConfig): Uint8Array[];
export declare function alpnProtocolsSet(self: RustlsClientConfig, value: Uint8Array[]): void;
export declare function checkSelectedAlpnGet(self: RustlsClientConfig): boolean;
export declare function checkSelectedAlpnSet(self: RustlsClientConfig, value: boolean): void;
export declare function resumptionGet(self: RustlsClientConfig): RustlsClientResumption;
export declare function resumptionSet(self: RustlsClientConfig, value: RustlsClientResumption): void;
export declare function maxFragmentSizeGet(self: RustlsClientConfig): rt.u64 | undefined;
export declare function maxFragmentSizeSet(self: RustlsClientConfig, value: rt.u64 | undefined): void;
export declare function clientAuthCertResolverGet(self: RustlsClientConfig): RustlsResolvesClientCert;
export declare function clientAuthCertResolverSet(self: RustlsClientConfig, value: RustlsResolvesClientCert): void;
export declare function enableSniGet(self: RustlsClientConfig): boolean;
export declare function enableSniSet(self: RustlsClientConfig, value: boolean): void;
export declare function keyLogGet(self: RustlsClientConfig): RustlsKeyLog;
export declare function keyLogSet(self: RustlsClientConfig, value: RustlsKeyLog): void;
export declare function enableSecretExtractionGet(self: RustlsClientConfig): boolean;
export declare function enableSecretExtractionSet(self: RustlsClientConfig, value: boolean): void;
export declare function enableEarlyDataGet(self: RustlsClientConfig): boolean;
export declare function enableEarlyDataSet(self: RustlsClientConfig, value: boolean): void;
export declare function requireEmsGet(self: RustlsClientConfig): boolean;
export declare function requireEmsSet(self: RustlsClientConfig, value: boolean): void;
export declare function timeProviderGet(self: RustlsClientConfig): RustlsTimeProvider;
export declare function timeProviderSet(self: RustlsClientConfig, value: RustlsTimeProvider): void;
export declare function certDecompressorsGet(self: RustlsClientConfig): RustlsCertDecompressor[];
export declare function certDecompressorsSet(self: RustlsClientConfig, value: RustlsCertDecompressor[]): void;
export declare function certCompressorsGet(self: RustlsClientConfig): RustlsCertCompressor[];
export declare function certCompressorsSet(self: RustlsClientConfig, value: RustlsCertCompressor[]): void;
export declare function certCompressionCacheGet(self: RustlsClientConfig): RustlsCompressionCache;
export declare function certCompressionCacheSet(self: RustlsClientConfig, value: RustlsCompressionCache): void;
export declare function sendTicketRequestGet(self: RustlsClientConfig): RustlsClientTicketRequest | undefined;
export declare function sendTicketRequestSet(self: RustlsClientConfig, value: RustlsClientTicketRequest | undefined): void;
export declare function fips(self: RustlsClientConfig): boolean;
export declare function cryptoProvider(self: RustlsClientConfig): RustlsCryptoProvider;
export declare function dangerous(self: RustlsClientConfig): RustlsClientDangerDangerousClientConfig;

// RustlsClientDangerDangerousClientConfig.ts
import * as rt from "./rt.ts";
import type { RustlsClientConfig } from "./RustlsClientConfig.ts";
export type RustlsClientDangerDangerousClientConfig = rt.Handle<"RustlsClientDangerDangerousClientConfig">;
export declare function debug(self: RustlsClientDangerDangerousClientConfig): string;
export declare function cfgGet(self: RustlsClientDangerDangerousClientConfig): RustlsClientConfig;

// RustlsClientResumption.ts
import * as rt from "./rt.ts";
import type { RustlsClientTls12Resumption } from "./RustlsClientTls12Resumption.ts";
export type RustlsClientResumption = rt.Handle<"RustlsClientResumption">;
export declare function debug(self: RustlsClientResumption): string;
export declare function clone(self: RustlsClientResumption): RustlsClientResumption;
export declare function default$(): RustlsClientResumption;
export declare function inMemorySessions(num: rt.u64): RustlsClientResumption;
export declare function disabled(): RustlsClientResumption;
export declare function tls12Resumption(self: RustlsClientResumption, tls12: RustlsClientTls12Resumption): RustlsClientResumption;
export { default$ as default };

// RustlsClientTicketRequest.ts
export type RustlsClientTicketRequest = {
    newSessionCount: number;
    resumptionCount: number;
};
export declare function debug(self: RustlsClientTicketRequest): string;
export declare function eq(a: RustlsClientTicketRequest, b: RustlsClientTicketRequest): boolean;

// RustlsClientTls12Resumption.ts
export type RustlsClientTls12Resumption = "Disabled" | "SessionIdOnly" | "SessionIdOrTickets";
export declare function debug(self: RustlsClientTls12Resumption): string;

// RustlsCompressionCache.ts
import * as rt from "./rt.ts";
export type RustlsCompressionCache = rt.Handle<"RustlsCompressionCache">;
export declare function clone(self: RustlsCompressionCache): RustlsCompressionCache;

// RustlsCryptoHpkeHpkePublicKey.ts
export type RustlsCryptoHpkeHpkePublicKey = {
    _0: Uint8Array;
};
export declare function debug(self: RustlsCryptoHpkeHpkePublicKey): string;

// RustlsCryptoProvider.ts
import * as rt from "./rt.ts";
export type RustlsCryptoProvider = rt.Handle<"RustlsCryptoProvider">;
export declare function clone(self: RustlsCryptoProvider): RustlsCryptoProvider;

// RustlsDistinguishedName.ts
import * as rt from "./rt.ts";
export type RustlsDistinguishedName = rt.Handle<"RustlsDistinguishedName">;
export declare function debug(self: RustlsDistinguishedName): string;
export declare function clone(self: RustlsDistinguishedName): RustlsDistinguishedName;
export declare function inSequence(bytes: Uint8Array | rt.Bytes): RustlsDistinguishedName;

// RustlsKeyLog.ts
import * as rt from "./rt.ts";
export type RustlsKeyLog = rt.Handle<"RustlsKeyLog">;
export declare function clone(self: RustlsKeyLog): RustlsKeyLog;

// RustlsPkiTypesCertificateDer.ts
import * as rt from "./rt.ts";
export type RustlsPkiTypesCertificateDer = rt.Handle<"RustlsPkiTypesCertificateDer">;
export declare function debug(self: RustlsPkiTypesCertificateDer): string;
export declare function clone(self: RustlsPkiTypesCertificateDer): RustlsPkiTypesCertificateDer;
export declare function eq(a: RustlsPkiTypesCertificateDer, b: RustlsPkiTypesCertificateDer): boolean;
export declare function hash(self: RustlsPkiTypesCertificateDer): rt.u64;
export declare function fromSlice(bytes: Uint8Array | rt.Bytes): RustlsPkiTypesCertificateDer;
export declare function intoOwned(self: RustlsPkiTypesCertificateDer): RustlsPkiTypesCertificateDer;

// RustlsPkiTypesDer.ts
import * as rt from "./rt.ts";
export type RustlsPkiTypesDer = rt.Handle<"RustlsPkiTypesDer">;
export declare function debug(self: RustlsPkiTypesDer): string;
export declare function clone(self: RustlsPkiTypesDer): RustlsPkiTypesDer;
export declare function eq(a: RustlsPkiTypesDer, b: RustlsPkiTypesDer): boolean;
export declare function hash(self: RustlsPkiTypesDer): rt.u64;
export declare function fromSlice(der: Uint8Array | rt.Bytes): RustlsPkiTypesDer;

// RustlsPkiTypesTrustAnchor.ts
import * as rt from "./rt.ts";
import type { RustlsPkiTypesDer } from "./RustlsPkiTypesDer.ts";
export type RustlsPkiTypesTrustAnchor = rt.Handle<"RustlsPkiTypesTrustAnchor">;
export declare function debug(self: RustlsPkiTypesTrustAnchor): string;
export declare function clone(self: RustlsPkiTypesTrustAnchor): RustlsPkiTypesTrustAnchor;
export declare function eq(a: RustlsPkiTypesTrustAnchor, b: RustlsPkiTypesTrustAnchor): boolean;
export declare function hash(self: RustlsPkiTypesTrustAnchor): rt.u64;
export declare function subjectGet(self: RustlsPkiTypesTrustAnchor): RustlsPkiTypesDer;
export declare function subjectSet(self: RustlsPkiTypesTrustAnchor, value: RustlsPkiTypesDer): void;
export declare function subjectPublicKeyInfoGet(self: RustlsPkiTypesTrustAnchor): RustlsPkiTypesDer;
export declare function subjectPublicKeyInfoSet(self: RustlsPkiTypesTrustAnchor, value: RustlsPkiTypesDer): void;
export declare function nameConstraintsGet(self: RustlsPkiTypesTrustAnchor): RustlsPkiTypesDer | undefined;
export declare function nameConstraintsSet(self: RustlsPkiTypesTrustAnchor, value: RustlsPkiTypesDer | undefined): void;
export declare function toOwned(self: RustlsPkiTypesTrustAnchor): RustlsPkiTypesTrustAnchor;

// RustlsProtocolVersion.ts
export type RustlsProtocolVersion = {
    tag: "SSLv2";
} | {
    tag: "SSLv3";
} | {
    tag: "TLSv1_0";
} | {
    tag: "TLSv1_1";
} | {
    tag: "TLSv1_2";
} | {
    tag: "TLSv1_3";
} | {
    tag: "DTLSv1_0";
} | {
    tag: "DTLSv1_2";
} | {
    tag: "DTLSv1_3";
} | {
    tag: "Unknown";
    _0: number;
};
export declare function debug(self: RustlsProtocolVersion): string;
export declare function eq(a: RustlsProtocolVersion, b: RustlsProtocolVersion): boolean;
export declare function toArray(self: RustlsProtocolVersion): Uint8Array;
export declare function asStr(self: RustlsProtocolVersion): string | undefined;

// RustlsResolvesClientCert.ts
import * as rt from "./rt.ts";
export type RustlsResolvesClientCert = rt.Handle<"RustlsResolvesClientCert">;
export declare function clone(self: RustlsResolvesClientCert): RustlsResolvesClientCert;

// RustlsRootCertStore.ts
import * as rt from "./rt.ts";
import type { RustlsDistinguishedName } from "./RustlsDistinguishedName.ts";
import type { RustlsPkiTypesCertificateDer } from "./RustlsPkiTypesCertificateDer.ts";
import type { RustlsPkiTypesTrustAnchor } from "./RustlsPkiTypesTrustAnchor.ts";
export type RustlsRootCertStore = rt.Handle<"RustlsRootCertStore">;
export declare function debug(self: RustlsRootCertStore): string;
export declare function clone(self: RustlsRootCertStore): RustlsRootCertStore;
export declare function rootsGet(self: RustlsRootCertStore): RustlsPkiTypesTrustAnchor[];
export declare function rootsSet(self: RustlsRootCertStore, value: RustlsPkiTypesTrustAnchor[]): void;
export declare function empty(): RustlsRootCertStore;
export declare function addParsableCertificates(self: RustlsRootCertStore, derCerts: RustlsPkiTypesCertificateDer[]): [rt.u64, rt.u64];
export declare function add(self: RustlsRootCertStore, der: RustlsPkiTypesCertificateDer): rt.Result<undefined, string>;
export declare function subjects(self: RustlsRootCertStore): RustlsDistinguishedName[];
export declare function isEmpty(self: RustlsRootCertStore): boolean;
export declare function len(self: RustlsRootCertStore): rt.u64;

// RustlsServerConfig.ts
import * as rt from "./rt.ts";
export type RustlsServerConfig = rt.Handle<"RustlsServerConfig">;
export declare function clone(self: RustlsServerConfig): RustlsServerConfig;

// RustlsTimeProvider.ts
import * as rt from "./rt.ts";
export type RustlsTimeProvider = rt.Handle<"RustlsTimeProvider">;
export declare function clone(self: RustlsTimeProvider): RustlsTimeProvider;

// Segment.ts
import * as rt from "./rt.ts";
export type Segment = {
    tag: "Literal";
    _0: string;
} | {
    tag: "Wildcard";
} | {
    tag: "Partial";
    prefix: string;
    suffix: string;
} | {
    tag: "Globstar";
};
export declare function toString(self: Segment): string;
export declare function debug(self: Segment): string;
export declare function eq(a: Segment, b: Segment): boolean;
export declare function hash(self: Segment): rt.u64;

// Server.ts
import * as rt from "./rt.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Driver } from "./Driver.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { QmuxSession } from "./QmuxSession.ts";
import type { ServerHandshake } from "./ServerHandshake.ts";
import type { Session } from "./Session.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { TransportMemorySession } from "./TransportMemorySession.ts";
import type { Versions } from "./Versions.ts";
export type Server = rt.Handle<"Server">;
export declare function clone(self: Server): Server;
export declare function default$(): Server;
export declare function new$(): Server;
export declare function withPublisherProducer(self: Server, publish: OriginProducer): Server;
export declare function withPublisherConsumer(self: Server, publish: OriginConsumer): Server;
export declare function withSubscriber(self: Server, subscribe: OriginProducer): Server;
export declare function withAllocator(self: Server, allocator: BandwidthAllocator): Server;
export declare function withStats(self: Server, stats: StatsSession): Server;
export declare function withLimits(self: Server, limits: SessionLimits): Server;
export declare function withVersions(self: Server, versions: Versions): Server;
export declare function withImplementation(self: Server, implementation: string): Server;
export declare function withTimeout(self: Server, timeout: rt.Duration | undefined): Server;
export declare function deadline(self: Server, arrived: ClockInstant): ClockInstant | undefined;
export declare function withReceiveWindow(self: Server, window: rt.u64): Server;
export declare function acceptMemorySession(self: Server, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<[Session, Driver], Error_>>;
export declare function acceptQmuxSession(self: Server, now: ClockInstant, session: QmuxSession): rt.Call<rt.Result<[Session, Driver], Error_>>;
export declare function handshakeMemorySession(self: Server, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<ServerHandshake, Error_>>;
export declare function handshakeQmuxSession(self: Server, now: ClockInstant, session: QmuxSession): rt.Call<rt.Result<ServerHandshake, Error_>>;
export { default$ as default, new$ as new };

// ServerAdmission.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ServerAdmitted } from "./ServerAdmitted.ts";
export type ServerAdmission = rt.Handle<"ServerAdmission">;
export declare function debug(self: ServerAdmission): string;
export declare function clone(self: ServerAdmission): ServerAdmission;
export declare function default$(): ServerAdmission;
export declare function enter(self: ServerAdmission): ServerAdmitted | undefined;
export declare function drain(self: ServerAdmission): void;
export declare function close(self: ServerAdmission): rt.Call<ClockInstant>;
export { default$ as default };

// ServerAdmitted.ts
import * as rt from "./rt.ts";
export type ServerAdmitted = rt.Handle<"ServerAdmitted">;
export declare function debug(self: ServerAdmitted): string;

// ServerHandshake.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { AuthRequests } from "./AuthRequests.ts";
import type { Driver } from "./Driver.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Session } from "./Session.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupRole } from "./SetupRole.ts";
import type { SetupToken } from "./SetupToken.ts";
import type { StatsSession } from "./StatsSession.ts";
export type ServerHandshake = rt.Handle<"ServerHandshake">;
export declare function path(self: ServerHandshake): string;
export declare function role(self: ServerHandshake): SetupRole | undefined;
export declare function peerNode(self: ServerHandshake): rt.u64 | undefined;
export declare function peerImplementation(self: ServerHandshake): string | undefined;
export declare function auth(self: ServerHandshake): AuthHandle;
export declare function requests(self: ServerHandshake): AuthRequests;
export declare function token(self: ServerHandshake): SetupToken | undefined;
export declare function withPublisherProducer(self: ServerHandshake, publish: OriginProducer): ServerHandshake;
export declare function withPublisherConsumer(self: ServerHandshake, publish: OriginConsumer): ServerHandshake;
export declare function withSubscriber(self: ServerHandshake, subscribe: OriginProducer): ServerHandshake;
export declare function withPeerNode(self: ServerHandshake, node: rt.u64): ServerHandshake;
export declare function withStats(self: ServerHandshake, stats: StatsSession): ServerHandshake;
export declare function withLimits(self: ServerHandshake, limits: SessionLimits): ServerHandshake;
export declare function withReceiveWindow(self: ServerHandshake, window: rt.u64): ServerHandshake;
export declare function expired(self: ServerHandshake): rt.Call<void>;
export declare function ok(self: ServerHandshake): rt.Call<rt.Result<[Session, Driver], Error_>>;
export declare function close(self: ServerHandshake, err: Error_): void;

// Session.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthViewers } from "./BandwidthViewers.ts";
import type { Error_ } from "./Error_.ts";
import type { GoawayConsumer } from "./GoawayConsumer.ts";
import type { GoawayGoaway } from "./GoawayGoaway.ts";
import type { SessionStats } from "./SessionStats.ts";
import type { Version } from "./Version.ts";
export type Session = rt.Handle<"Session">;
export declare function debug(self: Session): string;
export declare function clone(self: Session): Session;
export declare function eq(a: Session, b: Session): boolean;
export declare function version(self: Session): Version;
export declare function peerImplementation(self: Session): rt.Call<string | undefined>;
export declare function sendBandwidth(self: Session): BandwidthConsumer | undefined;
export declare function allocator(self: Session): BandwidthAllocator;
export declare function recvBandwidth(self: Session): BandwidthConsumer | undefined;
export declare function viewers(self: Session): BandwidthViewers;
export declare function stats(self: Session): SessionStats;
export declare function abort(self: Session, err: Error_): void;
export declare function close(self: Session): rt.Call<rt.Result<undefined, Error_>>;
export declare function closed(self: Session): rt.Call<Error_>;
export declare function goaway(self: Session, goaway: GoawayGoaway): rt.Result<undefined, Error_>;
export declare function draining(self: Session): GoawayConsumer;
export declare function auth(self: Session): AuthHandle;

// SessionError.ts
export type SessionError = {
    tag: "Cancel";
} | {
    tag: "Internal";
} | {
    tag: "Unauthorized";
} | {
    tag: "Expired";
} | {
    tag: "ProtocolViolation";
} | {
    tag: "TooManyRequests";
} | {
    tag: "GoawayTimeout";
} | {
    tag: "Timeout";
} | {
    tag: "Version";
} | {
    tag: "MalformedAuthToken";
} | {
    tag: "App";
    _0: number;
} | {
    tag: "Unknown";
    _0: number;
} | {
    tag: "Unsupported";
};
export declare function toString(self: SessionError): string;
export declare function debug(self: SessionError): string;
export declare function eq(a: SessionError, b: SessionError): boolean;
export declare function toCode(self: SessionError): number;
export declare function fromCode(code: number): SessionError;

// SessionLimits.ts
import * as rt from "./rt.ts";
export type SessionLimits = rt.Opaque<"SessionLimits">;
export declare function debug(self: SessionLimits): string;
export declare function eq(a: SessionLimits, b: SessionLimits): boolean;
export declare function default$(): SessionLimits;
export declare function announcesGet(self: SessionLimits): rt.u64;
export declare function announcesSet(self: SessionLimits, value: rt.u64): SessionLimits;
export { default$ as default };

// SessionStats.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
export type SessionStats = rt.Handle<"SessionStats">;
export declare function debug(self: SessionStats): string;
export declare function clone(self: SessionStats): SessionStats;
export declare function eq(a: SessionStats, b: SessionStats): boolean;
export declare function default$(): SessionStats;
export declare function rttGet(self: SessionStats): rt.Duration | undefined;
export declare function rttSet(self: SessionStats, value: rt.Duration | undefined): void;
export declare function estimatedSendRateGet(self: SessionStats): BandwidthRate | undefined;
export declare function estimatedSendRateSet(self: SessionStats, value: BandwidthRate | undefined): void;
export declare function bytesSentGet(self: SessionStats): rt.u64 | undefined;
export declare function bytesSentSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function bytesReceivedGet(self: SessionStats): rt.u64 | undefined;
export declare function bytesReceivedSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function bytesLostGet(self: SessionStats): rt.u64 | undefined;
export declare function bytesLostSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function packetsSentGet(self: SessionStats): rt.u64 | undefined;
export declare function packetsSentSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function packetsReceivedGet(self: SessionStats): rt.u64 | undefined;
export declare function packetsReceivedSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function packetsLostGet(self: SessionStats): rt.u64 | undefined;
export declare function packetsLostSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function streamsEvictedGet(self: SessionStats): rt.u64 | undefined;
export declare function streamsEvictedSet(self: SessionStats, value: rt.u64 | undefined): void;
export declare function datagramsUnroutedGet(self: SessionStats): rt.u64 | undefined;
export declare function datagramsUnroutedSet(self: SessionStats, value: rt.u64 | undefined): void;
export { default$ as default };

// SetupRole.ts
export type SetupRole = "Publisher" | "Subscriber";
export declare function toString(self: SetupRole): string;
export declare function debug(self: SetupRole): string;
export declare function asStr(self: SetupRole): string;

// SetupToken.ts
import * as rt from "./rt.ts";
export type SetupToken = {
    kind: rt.u64;
    value: Uint8Array;
};
export declare function debug(self: SetupToken): string;
export declare function eq(a: SetupToken, b: SetupToken): boolean;

// Specificity.ts
import * as rt from "./rt.ts";
export type Specificity = rt.Handle<"Specificity">;
export declare function debug(self: Specificity): string;
export declare function clone(self: Specificity): Specificity;
export declare function eq(a: Specificity, b: Specificity): boolean;
export declare function hash(self: Specificity): rt.u64;
export declare function partialCmp(a: Specificity, b: Specificity): number | undefined;
export declare function cmp(a: Specificity, b: Specificity): number;

// StatsActivity.ts
import * as rt from "./rt.ts";
export type StatsActivity = rt.Opaque<"StatsActivity">;
export declare function debug(self: StatsActivity): string;
export declare function eq(a: StatsActivity, b: StatsActivity): boolean;
export declare function default$(): StatsActivity;
export declare function idleGet(self: StatsActivity): rt.u64;
export declare function idleSet(self: StatsActivity, value: rt.u64): StatsActivity;
export declare function activeGet(self: StatsActivity): rt.u64;
export declare function activeSet(self: StatsActivity, value: rt.u64): StatsActivity;
export { default$ as default };

// StatsBill.ts
import * as rt from "./rt.ts";
export type StatsBill = rt.Handle<"StatsBill">;
export declare function debug(self: StatsBill): string;
export declare function clone(self: StatsBill): StatsBill;
export declare function default$(): StatsBill;
export declare function deliver(self: StatsBill, bytes: rt.u64): void;
export { default$ as default };

// StatsConfig.ts
import * as rt from "./rt.ts";
import type { Pattern } from "./Pattern.ts";
import type { Patterns } from "./Patterns.ts";
export type StatsConfig = rt.Handle<"StatsConfig">;
export declare function debug(self: StatsConfig): string;
export declare function clone(self: StatsConfig): StatsConfig;
export declare function default$(): StatsConfig;
export declare function excludeGet(self: StatsConfig): Patterns;
export declare function excludeSet(self: StatsConfig, value: Patterns): void;
export declare function new$(): StatsConfig;
export declare function withExclude(self: StatsConfig, pattern: Pattern): StatsConfig;
export { default$ as default, new$ as new };

// StatsContent.ts
import * as rt from "./rt.ts";
export type StatsContent = rt.Opaque<"StatsContent">;
export declare function debug(self: StatsContent): string;
export declare function eq(a: StatsContent, b: StatsContent): boolean;
export declare function default$(): StatsContent;
export declare function bytesGet(self: StatsContent): rt.u64;
export declare function bytesSet(self: StatsContent, value: rt.u64): StatsContent;
export declare function framesGet(self: StatsContent): rt.u64;
export declare function framesSet(self: StatsContent, value: rt.u64): StatsContent;
export declare function groupsGet(self: StatsContent): rt.u64;
export declare function groupsSet(self: StatsContent, value: rt.u64): StatsContent;
export { default$ as default };

// StatsFailure.ts
export type StatsFailure = "IdleTimeout" | "Transport" | "Application" | "Expired";
export declare function debug(self: StatsFailure): string;
export declare function ALL(): StatsFailure[];
export declare function asStr(self: StatsFailure): string;

// StatsFailures.ts
import * as rt from "./rt.ts";
import type { StatsFailure } from "./StatsFailure.ts";
export type StatsFailures = rt.Opaque<"StatsFailures">;
export declare function debug(self: StatsFailures): string;
export declare function eq(a: StatsFailures, b: StatsFailures): boolean;
export declare function default$(): StatsFailures;
export declare function idleTimeoutGet(self: StatsFailures): rt.u64;
export declare function idleTimeoutSet(self: StatsFailures, value: rt.u64): StatsFailures;
export declare function transportGet(self: StatsFailures): rt.u64;
export declare function transportSet(self: StatsFailures, value: rt.u64): StatsFailures;
export declare function applicationGet(self: StatsFailures): rt.u64;
export declare function applicationSet(self: StatsFailures, value: rt.u64): StatsFailures;
export declare function expiredGet(self: StatsFailures): rt.u64;
export declare function expiredSet(self: StatsFailures, value: rt.u64): StatsFailures;
export declare function count(self: StatsFailures, kind: StatsFailure): rt.u64;
export declare function add(self: StatsFailures, other: StatsFailures): StatsFailures;
export declare function regressed(self: StatsFailures, previous: StatsFailures): boolean;
export { default$ as default };

// StatsHandle.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
import type { StatsRefusal } from "./StatsRefusal.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { StatsSessionInfo } from "./StatsSessionInfo.ts";
export type StatsHandle = rt.Handle<"StatsHandle">;
export declare function clone(self: StatsHandle): StatsHandle;
export declare function default$(): StatsHandle;
export declare function refuse(self: StatsHandle, refusal: StatsRefusal): void;
export declare function session(self: StatsHandle, session: StatsSessionInfo): StatsSession;
export declare function request(self: StatsHandle, label: Path): StatsSession;
export { default$ as default };

// StatsMessage.ts
export type StatsMessage = "Session" | "Announce" | "TrackInfo" | "Subscribe" | "Fetch" | "Probe" | "Auth" | "Goaway" | "Other";
export declare function debug(self: StatsMessage): string;
export declare function asStr(self: StatsMessage): string;

// StatsPresence.ts
import * as rt from "./rt.ts";
import type { StatsFailures } from "./StatsFailures.ts";
import type { StatsRefusals } from "./StatsRefusals.ts";
import type { StatsRoutes } from "./StatsRoutes.ts";
export type StatsPresence = rt.Opaque<"StatsPresence">;
export declare function debug(self: StatsPresence): string;
export declare function eq(a: StatsPresence, b: StatsPresence): boolean;
export declare function default$(): StatsPresence;
export declare function sessionsRefusedGet(self: StatsPresence): StatsRefusals;
export declare function sessionsRefusedSet(self: StatsPresence, value: StatsRefusals): StatsPresence;
export declare function sessionsFailedGet(self: StatsPresence): StatsFailures;
export declare function sessionsFailedSet(self: StatsPresence, value: StatsFailures): StatsPresence;
export declare function requestsGet(self: StatsPresence): rt.u64;
export declare function requestsSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function sessionsStartedGet(self: StatsPresence): rt.u64;
export declare function sessionsStartedSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function sessionsEndedGet(self: StatsPresence): rt.u64;
export declare function sessionsEndedSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function sentBytesGet(self: StatsPresence): rt.u64;
export declare function sentBytesSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function receivedBytesGet(self: StatsPresence): rt.u64;
export declare function receivedBytesSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function announcesPeakGet(self: StatsPresence): rt.u64;
export declare function announcesPeakSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function subscriptionsPeakGet(self: StatsPresence): rt.u64;
export declare function subscriptionsPeakSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function wireTxGet(self: StatsPresence): rt.u64;
export declare function wireTxSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function wireRxGet(self: StatsPresence): rt.u64;
export declare function wireRxSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function routesRefusedGet(self: StatsPresence): rt.u64;
export declare function routesRefusedSet(self: StatsPresence, value: rt.u64): StatsPresence;
export declare function routesGet(self: StatsPresence): StatsRoutes;
export declare function routesSet(self: StatsPresence, value: StatsRoutes): StatsPresence;
export declare function add(self: StatsPresence, other: StatsPresence): StatsPresence;
export declare function active(self: StatsPresence): rt.u64;
export { default$ as default };

// StatsRefusal.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
import type { StatsRefusalKind } from "./StatsRefusalKind.ts";
export type StatsRefusal = rt.Handle<"StatsRefusal">;
export declare function debug(self: StatsRefusal): string;
export declare function clone(self: StatsRefusal): StatsRefusal;
export declare function labelGet(self: StatsRefusal): Path;
export declare function labelSet(self: StatsRefusal, value: Path): void;
export declare function reasonGet(self: StatsRefusal): StatsRefusalKind;
export declare function reasonSet(self: StatsRefusal, value: StatsRefusalKind): void;
export declare function new$(label: Path, reason: StatsRefusalKind): StatsRefusal;
export { new$ as new };

// StatsRefusalKind.ts
export type StatsRefusalKind = "Refused" | "Unavailable" | "Request" | "Forbidden" | "Membership" | "Expired";
export declare function debug(self: StatsRefusalKind): string;
export declare function ALL(): StatsRefusalKind[];
export declare function asStr(self: StatsRefusalKind): string;

// StatsRefusals.ts
import * as rt from "./rt.ts";
import type { StatsRefusalKind } from "./StatsRefusalKind.ts";
export type StatsRefusals = rt.Opaque<"StatsRefusals">;
export declare function debug(self: StatsRefusals): string;
export declare function eq(a: StatsRefusals, b: StatsRefusals): boolean;
export declare function default$(): StatsRefusals;
export declare function refusedGet(self: StatsRefusals): rt.u64;
export declare function refusedSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function unavailableGet(self: StatsRefusals): rt.u64;
export declare function unavailableSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function requestGet(self: StatsRefusals): rt.u64;
export declare function requestSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function forbiddenGet(self: StatsRefusals): rt.u64;
export declare function forbiddenSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function membershipGet(self: StatsRefusals): rt.u64;
export declare function membershipSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function expiredGet(self: StatsRefusals): rt.u64;
export declare function expiredSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function count(self: StatsRefusals, kind: StatsRefusalKind): rt.u64;
export declare function add(self: StatsRefusals, other: StatsRefusals): StatsRefusals;
export declare function regressed(self: StatsRefusals, previous: StatsRefusals): boolean;
export { default$ as default };

// StatsRegistry.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { StatsConfig } from "./StatsConfig.ts";
import type { StatsHandle } from "./StatsHandle.ts";
import type { StatsReport } from "./StatsReport.ts";
import type { StatsSnapshot } from "./StatsSnapshot.ts";
import type { StatsTier } from "./StatsTier.ts";
export type StatsRegistry = rt.Handle<"StatsRegistry">;
export declare function clone(self: StatsRegistry): StatsRegistry;
export declare function default$(): StatsRegistry;
export declare function new$(config: StatsConfig): StatsRegistry;
export declare function disabled(): StatsRegistry;
export declare function isEnabled(self: StatsRegistry): boolean;
export declare function tier(self: StatsRegistry, tier: StatsTier): StatsHandle;
export declare function snapshot(self: StatsRegistry): StatsSnapshot;
export declare function report(self: StatsRegistry, report: StatsReport, now: ClockInstant): void;
export { default$ as default, new$ as new };

// StatsReport.ts
import * as rt from "./rt.ts";
import type { StatsSessionEntry } from "./StatsSessionEntry.ts";
import type { StatsTrafficEntry } from "./StatsTrafficEntry.ts";
export type StatsReport = rt.Handle<"StatsReport">;
export declare function debug(self: StatsReport): string;
export declare function clone(self: StatsReport): StatsReport;
export declare function default$(): StatsReport;
export declare function trafficGet(self: StatsReport): StatsTrafficEntry[];
export declare function trafficSet(self: StatsReport, value: StatsTrafficEntry[]): void;
export declare function sessionsGet(self: StatsReport): StatsSessionEntry[];
export declare function sessionsSet(self: StatsReport, value: StatsSessionEntry[]): void;
export { default$ as default };

// StatsRole.ts
export type StatsRole = "Publisher" | "Subscriber";
export declare function debug(self: StatsRole): string;
export declare function asStr(self: StatsRole): string;

// StatsRouteClose.ts
export type StatsRouteClose = "Stalled" | "Overrun" | "Malformed" | "Role" | "Unasked" | "Regressed" | "Other";
export declare function debug(self: StatsRouteClose): string;
export declare function ALL(): StatsRouteClose[];
export declare function asStr(self: StatsRouteClose): string;

// StatsRouteCloses.ts
import * as rt from "./rt.ts";
import type { StatsRouteClose } from "./StatsRouteClose.ts";
export type StatsRouteCloses = rt.Opaque<"StatsRouteCloses">;
export declare function debug(self: StatsRouteCloses): string;
export declare function eq(a: StatsRouteCloses, b: StatsRouteCloses): boolean;
export declare function default$(): StatsRouteCloses;
export declare function stalledGet(self: StatsRouteCloses): rt.u64;
export declare function stalledSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function overrunGet(self: StatsRouteCloses): rt.u64;
export declare function overrunSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function malformedGet(self: StatsRouteCloses): rt.u64;
export declare function malformedSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function roleGet(self: StatsRouteCloses): rt.u64;
export declare function roleSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function unaskedGet(self: StatsRouteCloses): rt.u64;
export declare function unaskedSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function regressedGet(self: StatsRouteCloses): rt.u64;
export declare function regressedSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function otherGet(self: StatsRouteCloses): rt.u64;
export declare function otherSet(self: StatsRouteCloses, value: rt.u64): StatsRouteCloses;
export declare function count(self: StatsRouteCloses, kind: StatsRouteClose): rt.u64;
export declare function total(self: StatsRouteCloses): rt.u64;
export declare function add(self: StatsRouteCloses, other: StatsRouteCloses): StatsRouteCloses;
export declare function regressed(self: StatsRouteCloses, previous: StatsRouteCloses): boolean;
export { default$ as default };

// StatsRouteRefusal.ts
export type StatsRouteRefusal = "Path" | "Scope" | "Looped";
export declare function debug(self: StatsRouteRefusal): string;
export declare function ALL(): StatsRouteRefusal[];
export declare function asStr(self: StatsRouteRefusal): string;

// StatsRouteRefusals.ts
import * as rt from "./rt.ts";
import type { StatsRouteRefusal } from "./StatsRouteRefusal.ts";
export type StatsRouteRefusals = rt.Opaque<"StatsRouteRefusals">;
export declare function debug(self: StatsRouteRefusals): string;
export declare function eq(a: StatsRouteRefusals, b: StatsRouteRefusals): boolean;
export declare function default$(): StatsRouteRefusals;
export declare function pathGet(self: StatsRouteRefusals): rt.u64;
export declare function pathSet(self: StatsRouteRefusals, value: rt.u64): StatsRouteRefusals;
export declare function scopeGet(self: StatsRouteRefusals): rt.u64;
export declare function scopeSet(self: StatsRouteRefusals, value: rt.u64): StatsRouteRefusals;
export declare function loopedGet(self: StatsRouteRefusals): rt.u64;
export declare function loopedSet(self: StatsRouteRefusals, value: rt.u64): StatsRouteRefusals;
export declare function count(self: StatsRouteRefusals, kind: StatsRouteRefusal): rt.u64;
export declare function total(self: StatsRouteRefusals): rt.u64;
export declare function add(self: StatsRouteRefusals, other: StatsRouteRefusals): StatsRouteRefusals;
export declare function regressed(self: StatsRouteRefusals, previous: StatsRouteRefusals): boolean;
export { default$ as default };

// StatsRoutes.ts
import * as rt from "./rt.ts";
import type { StatsRouteCloses } from "./StatsRouteCloses.ts";
import type { StatsRouteRefusals } from "./StatsRouteRefusals.ts";
import type { StatsWithdrawals } from "./StatsWithdrawals.ts";
export type StatsRoutes = rt.Opaque<"StatsRoutes">;
export declare function debug(self: StatsRoutes): string;
export declare function eq(a: StatsRoutes, b: StatsRoutes): boolean;
export declare function default$(): StatsRoutes;
export declare function changedGet(self: StatsRoutes): rt.u64;
export declare function changedSet(self: StatsRoutes, value: rt.u64): StatsRoutes;
export declare function rewrittenGet(self: StatsRoutes): rt.u64;
export declare function rewrittenSet(self: StatsRoutes, value: rt.u64): StatsRoutes;
export declare function sentGet(self: StatsRoutes): StatsWithdrawals;
export declare function sentSet(self: StatsRoutes, value: StatsWithdrawals): StatsRoutes;
export declare function receivedGet(self: StatsRoutes): StatsWithdrawals;
export declare function receivedSet(self: StatsRoutes, value: StatsWithdrawals): StatsRoutes;
export declare function refusedGet(self: StatsRoutes): StatsRouteRefusals;
export declare function refusedSet(self: StatsRoutes, value: StatsRouteRefusals): StatsRoutes;
export declare function closedGet(self: StatsRoutes): StatsRouteCloses;
export declare function closedSet(self: StatsRoutes, value: StatsRouteCloses): StatsRoutes;
export declare function add(self: StatsRoutes, other: StatsRoutes): StatsRoutes;
export declare function regressed(self: StatsRoutes, previous: StatsRoutes): boolean;
export { default$ as default };

// StatsSession.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
import type { StatsTier } from "./StatsTier.ts";
import type { StatsWire } from "./StatsWire.ts";
import type { TransportWire } from "./TransportWire.ts";
export type StatsSession = rt.Handle<"StatsSession">;
export declare function clone(self: StatsSession): StatsSession;
export declare function default$(): StatsSession;
export declare function wire(self: StatsSession, counters: TransportWire): StatsWire;
export declare function retag(self: StatsSession, tier: StatsTier, label: Path): void;
export { default$ as default };

// StatsSessionEntry.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
import type { StatsPresence } from "./StatsPresence.ts";
import type { StatsTier } from "./StatsTier.ts";
export type StatsSessionEntry = rt.Handle<"StatsSessionEntry">;
export declare function debug(self: StatsSessionEntry): string;
export declare function clone(self: StatsSessionEntry): StatsSessionEntry;
export declare function tierGet(self: StatsSessionEntry): StatsTier;
export declare function tierSet(self: StatsSessionEntry, value: StatsTier): void;
export declare function labelGet(self: StatsSessionEntry): Path;
export declare function labelSet(self: StatsSessionEntry, value: Path): void;
export declare function presenceGet(self: StatsSessionEntry): StatsPresence;
export declare function presenceSet(self: StatsSessionEntry, value: StatsPresence): void;
export declare function closedGet(self: StatsSessionEntry): boolean;
export declare function closedSet(self: StatsSessionEntry, value: boolean): void;

// StatsSessionInfo.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
export type StatsSessionInfo = rt.Handle<"StatsSessionInfo">;
export declare function debug(self: StatsSessionInfo): string;
export declare function clone(self: StatsSessionInfo): StatsSessionInfo;
export declare function labelGet(self: StatsSessionInfo): Path;
export declare function labelSet(self: StatsSessionInfo, value: Path): void;
export declare function new$(label: Path): StatsSessionInfo;
export { new$ as new };

// StatsSnapshot.ts
import * as rt from "./rt.ts";
import type { StatsActivity } from "./StatsActivity.ts";
import type { StatsMessage } from "./StatsMessage.ts";
import type { StatsPresence } from "./StatsPresence.ts";
import type { StatsRole } from "./StatsRole.ts";
import type { StatsTier } from "./StatsTier.ts";
import type { StatsTotals } from "./StatsTotals.ts";
import type { StatsTraffic } from "./StatsTraffic.ts";
export type StatsSnapshot = rt.Handle<"StatsSnapshot">;
export declare function debug(self: StatsSnapshot): string;
export declare function clone(self: StatsSnapshot): StatsSnapshot;
export declare function eq(a: StatsSnapshot, b: StatsSnapshot): boolean;
export declare function default$(): StatsSnapshot;
export declare function control(self: StatsSnapshot): ([StatsTier, StatsMessage, StatsTotals])[];
export declare function traffic(self: StatsSnapshot): ([StatsTier, StatsRole, StatsTraffic])[];
export declare function sessions(self: StatsSnapshot): ([StatsTier, StatsPresence])[];
export declare function activity(self: StatsSnapshot): ([StatsTier, StatsActivity])[];
export { default$ as default };

// StatsTier.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
export type StatsTier = rt.Handle<"StatsTier">;
export declare function toString(self: StatsTier): string;
export declare function debug(self: StatsTier): string;
export declare function clone(self: StatsTier): StatsTier;
export declare function eq(a: StatsTier, b: StatsTier): boolean;
export declare function hash(self: StatsTier): rt.u64;
export declare function default$(): StatsTier;
export declare function new$(label: Path): StatsTier;
export declare function label(self: StatsTier): Path;
export declare function asStr(self: StatsTier): string;
export { default$ as default, new$ as new };

// StatsTotals.ts
import * as rt from "./rt.ts";
export type StatsTotals = rt.Opaque<"StatsTotals">;
export declare function debug(self: StatsTotals): string;
export declare function eq(a: StatsTotals, b: StatsTotals): boolean;
export declare function default$(): StatsTotals;
export declare function txGet(self: StatsTotals): rt.u64;
export declare function txSet(self: StatsTotals, value: rt.u64): StatsTotals;
export declare function rxGet(self: StatsTotals): rt.u64;
export declare function rxSet(self: StatsTotals, value: rt.u64): StatsTotals;
export declare function txMessagesGet(self: StatsTotals): rt.u64;
export declare function txMessagesSet(self: StatsTotals, value: rt.u64): StatsTotals;
export declare function rxMessagesGet(self: StatsTotals): rt.u64;
export declare function rxMessagesSet(self: StatsTotals, value: rt.u64): StatsTotals;
export { default$ as default };

// StatsTraffic.ts
import * as rt from "./rt.ts";
import type { StatsContent } from "./StatsContent.ts";
export type StatsTraffic = rt.Opaque<"StatsTraffic">;
export declare function debug(self: StatsTraffic): string;
export declare function eq(a: StatsTraffic, b: StatsTraffic): boolean;
export declare function default$(): StatsTraffic;
export declare function announcesStartedGet(self: StatsTraffic): rt.u64;
export declare function announcesStartedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function announcesEndedGet(self: StatsTraffic): rt.u64;
export declare function announcesEndedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function announcedBytesGet(self: StatsTraffic): rt.u64;
export declare function announcedBytesSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function broadcastsStartedGet(self: StatsTraffic): rt.u64;
export declare function broadcastsStartedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function broadcastsEndedGet(self: StatsTraffic): rt.u64;
export declare function broadcastsEndedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function subscriptionsStartedGet(self: StatsTraffic): rt.u64;
export declare function subscriptionsStartedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function subscriptionsEndedGet(self: StatsTraffic): rt.u64;
export declare function subscriptionsEndedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function fetchesGet(self: StatsTraffic): rt.u64;
export declare function fetchesSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function bytesGet(self: StatsTraffic): rt.u64;
export declare function bytesSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function framesGet(self: StatsTraffic): rt.u64;
export declare function framesSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function groupsGet(self: StatsTraffic): rt.u64;
export declare function groupsSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function datagramsGet(self: StatsTraffic): rt.u64;
export declare function datagramsSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function inflatedGet(self: StatsTraffic): rt.u64;
export declare function inflatedSet(self: StatsTraffic, value: rt.u64): StatsTraffic;
export declare function staleGet(self: StatsTraffic): StatsContent;
export declare function staleSet(self: StatsTraffic, value: StatsContent): StatsTraffic;
export declare function add(self: StatsTraffic, other: StatsTraffic): StatsTraffic;
export declare function activeBroadcasts(self: StatsTraffic): rt.u64;
export declare function activeSubscriptions(self: StatsTraffic): rt.u64;
export declare function isIdle(self: StatsTraffic): boolean;
export { default$ as default };

// StatsTrafficEntry.ts
import * as rt from "./rt.ts";
import type { Path } from "./Path.ts";
import type { StatsTier } from "./StatsTier.ts";
import type { StatsTraffic } from "./StatsTraffic.ts";
export type StatsTrafficEntry = rt.Handle<"StatsTrafficEntry">;
export declare function debug(self: StatsTrafficEntry): string;
export declare function clone(self: StatsTrafficEntry): StatsTrafficEntry;
export declare function pathGet(self: StatsTrafficEntry): Path;
export declare function pathSet(self: StatsTrafficEntry, value: Path): void;
export declare function tierGet(self: StatsTrafficEntry): StatsTier;
export declare function tierSet(self: StatsTrafficEntry, value: StatsTier): void;
export declare function publisherGet(self: StatsTrafficEntry): StatsTraffic;
export declare function publisherSet(self: StatsTrafficEntry, value: StatsTraffic): void;
export declare function subscriberGet(self: StatsTrafficEntry): StatsTraffic;
export declare function subscriberSet(self: StatsTrafficEntry, value: StatsTraffic): void;
export declare function closedGet(self: StatsTrafficEntry): boolean;
export declare function closedSet(self: StatsTrafficEntry, value: boolean): void;

// StatsWire.ts
import * as rt from "./rt.ts";
export type StatsWire = rt.Handle<"StatsWire">;
export declare function default$(): StatsWire;
export { default$ as default };

// StatsWithdrawal.ts
export type StatsWithdrawal = "Dead" | "Drain";
export declare function debug(self: StatsWithdrawal): string;
export declare function ALL(): StatsWithdrawal[];
export declare function asStr(self: StatsWithdrawal): string;

// StatsWithdrawals.ts
import * as rt from "./rt.ts";
import type { StatsWithdrawal } from "./StatsWithdrawal.ts";
export type StatsWithdrawals = rt.Opaque<"StatsWithdrawals">;
export declare function debug(self: StatsWithdrawals): string;
export declare function eq(a: StatsWithdrawals, b: StatsWithdrawals): boolean;
export declare function default$(): StatsWithdrawals;
export declare function deadGet(self: StatsWithdrawals): rt.u64;
export declare function deadSet(self: StatsWithdrawals, value: rt.u64): StatsWithdrawals;
export declare function drainGet(self: StatsWithdrawals): rt.u64;
export declare function drainSet(self: StatsWithdrawals, value: rt.u64): StatsWithdrawals;
export declare function count(self: StatsWithdrawals, kind: StatsWithdrawal): rt.u64;
export declare function total(self: StatsWithdrawals): rt.u64;
export declare function add(self: StatsWithdrawals, other: StatsWithdrawals): StatsWithdrawals;
export declare function regressed(self: StatsWithdrawals, previous: StatsWithdrawals): boolean;
export { default$ as default };

// StreamError.ts
import type { SessionError } from "./SessionError.ts";
export type StreamError = {
    tag: "Session";
    _0: SessionError;
} | {
    tag: "Internal";
} | {
    tag: "Cancel";
} | {
    tag: "DeliveryTimeout";
} | {
    tag: "ControlTimeout";
} | {
    tag: "GoingAway";
} | {
    tag: "TooFarBehind";
} | {
    tag: "MalformedTrack";
} | {
    tag: "NotFound";
} | {
    tag: "Unroutable";
} | {
    tag: "Old";
} | {
    tag: "Evicted";
} | {
    tag: "WrongSize";
} | {
    tag: "FrameTooLarge";
} | {
    tag: "GroupTooLarge";
} | {
    tag: "TimestampMismatch";
} | {
    tag: "Unauthorized";
} | {
    tag: "App";
    _0: number;
} | {
    tag: "Unknown";
    _0: number;
};
export declare function toString(self: StreamError): string;
export declare function debug(self: StreamError): string;
export declare function eq(a: StreamError, b: StreamError): boolean;
export declare function toCode(self: StreamError): number;
export declare function fromCode(code: number): StreamError;

// TimeOverflow.ts
import * as rt from "./rt.ts";
export type TimeOverflow = rt.Opaque<"TimeOverflow">;
export declare function toString(self: TimeOverflow): string;
export declare function debug(self: TimeOverflow): string;
export declare function eq(a: TimeOverflow, b: TimeOverflow): boolean;

// Timescale.ts
import * as rt from "./rt.ts";
import type { TimeOverflow } from "./TimeOverflow.ts";
export type Timescale = rt.Opaque<"Timescale">;
export declare function toString(self: Timescale): string;
export declare function debug(self: Timescale): string;
export declare function eq(a: Timescale, b: Timescale): boolean;
export declare function hash(self: Timescale): rt.u64;
export declare function partialCmp(a: Timescale, b: Timescale): number | undefined;
export declare function cmp(a: Timescale, b: Timescale): number;
export declare function SECOND(): Timescale;
export declare function MILLI(): Timescale;
export declare function MICRO(): Timescale;
export declare function NANO(): Timescale;
export declare function new$(unitsPerSecond: rt.u64): rt.Result<Timescale, TimeOverflow>;
export declare function asU64(self: Timescale): rt.u64;
export { new$ as new };

// Timestamp.ts
import * as rt from "./rt.ts";
import type { TimeOverflow } from "./TimeOverflow.ts";
import type { Timescale } from "./Timescale.ts";
export type Timestamp = rt.Opaque<"Timestamp">;
export declare function debug(self: Timestamp): string;
export declare function eq(a: Timestamp, b: Timestamp): boolean;
export declare function hash(self: Timestamp): rt.u64;
export declare function partialCmp(a: Timestamp, b: Timestamp): number | undefined;
export declare function ZERO(): Timestamp;
export declare function new$(value: rt.u64, scale: Timescale): rt.Result<Timestamp, TimeOverflow>;
export declare function unscaled(key: rt.u64): Timestamp;
export declare function fromScale(value: rt.u64, unitsPerSecond: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromSecs(seconds: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromMillis(millis: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromMicros(micros: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromNanos(nanos: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function value(self: Timestamp): rt.u64;
export declare function scale(self: Timestamp): Timescale | undefined;
export declare function isZero(self: Timestamp): boolean;
export declare function convert(self: Timestamp, newScale: Timescale): rt.Result<Timestamp, TimeOverflow>;
export declare function asScale(self: Timestamp, target: Timescale): rt.Result<rt.u128, TimeOverflow>;
export declare function asSecs(self: Timestamp): rt.Result<rt.u64, TimeOverflow>;
export declare function asMillis(self: Timestamp): rt.Result<rt.u128, TimeOverflow>;
export declare function asMicros(self: Timestamp): rt.Result<rt.u128, TimeOverflow>;
export declare function asNanos(self: Timestamp): rt.Result<rt.u128, TimeOverflow>;
export declare function checkedAdd(self: Timestamp, rhs: Timestamp): rt.Result<Timestamp, TimeOverflow>;
export declare function checkedSub(self: Timestamp, rhs: Timestamp): rt.Result<Timestamp, TimeOverflow>;
export { new$ as new };

// TokioBackoff.ts
import * as rt from "./rt.ts";
import type { MoqTokioCliBound } from "./MoqTokioCliBound.ts";
import type { TokioDuration } from "./TokioDuration.ts";
export type TokioBackoff = rt.Handle<"TokioBackoff">;
export declare function debug(self: TokioBackoff): string;
export declare function clone(self: TokioBackoff): TokioBackoff;
export declare function default$(): TokioBackoff;
export declare function initialGet(self: TokioBackoff): TokioDuration | undefined;
export declare function initialSet(self: TokioBackoff, value: TokioDuration | undefined): void;
export declare function multiplierGet(self: TokioBackoff): number;
export declare function multiplierSet(self: TokioBackoff, value: number): void;
export declare function maxGet(self: TokioBackoff): MoqTokioCliBound | undefined;
export declare function maxSet(self: TokioBackoff, value: MoqTokioCliBound | undefined): void;
export declare function healthyGet(self: TokioBackoff): TokioDuration | undefined;
export declare function healthySet(self: TokioBackoff, value: TokioDuration | undefined): void;
export declare function timeoutGet(self: TokioBackoff): MoqTokioCliBound | undefined;
export declare function timeoutSet(self: TokioBackoff, value: MoqTokioCliBound | undefined): void;
export declare function initial(self: TokioBackoff): rt.Duration;
export declare function max(self: TokioBackoff): rt.Duration | undefined;
export declare function timeout(self: TokioBackoff): rt.Duration | undefined;
export { default$ as default };

// TokioClient.ts
import * as rt from "./rt.ts";
import type { MoqTokioAddrs } from "./MoqTokioAddrs.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupToken } from "./SetupToken.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { TokioClientConfig } from "./TokioClientConfig.ts";
import type { TokioConnection } from "./TokioConnection.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioRestart } from "./TokioRestart.ts";
export type TokioClient = rt.Handle<"TokioClient">;
export declare function clone(self: TokioClient): TokioClient;
export declare function new$(config: TokioClientConfig): rt.Result<TokioClient, TokioError>;
export declare function withPublisherProducer(self: TokioClient, publish: OriginProducer): TokioClient;
export declare function withPublisherConsumer(self: TokioClient, publish: OriginConsumer): TokioClient;
export declare function withSubscriber(self: TokioClient, subscribe: OriginProducer): TokioClient;
export declare function withLimits(self: TokioClient, limits: SessionLimits): TokioClient;
export declare function withStats(self: TokioClient, stats: StatsSession): TokioClient;
export declare function withCost(self: TokioClient, cost: rt.u64): TokioClient;
export declare function withPeerNode(self: TokioClient, node: rt.u64): TokioClient;
export declare function withToken(self: TokioClient, token: SetupToken): TokioClient;
export declare function withReconnect(self: TokioClient, reconnect: boolean): TokioClient;
export declare function withRestart(self: TokioClient, restart: TokioRestart): TokioClient;
export declare function connect(self: TokioClient, addrs: MoqTokioAddrs): TokioConnection;
export declare function close(self: TokioClient): rt.Call<void>;
export declare function publish(self: TokioClient, origin: OriginConsumer): TokioConnection | undefined;
export declare function consume(self: TokioClient, origin: OriginProducer): TokioConnection | undefined;
export { new$ as new };

// TokioClientConfig.ts
import * as rt from "./rt.ts";
import type { TokioClient } from "./TokioClient.ts";
import type { TokioConnectConfig } from "./TokioConnectConfig.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioQuicConfig } from "./TokioQuicConfig.ts";
export type TokioClientConfig = rt.Handle<"TokioClientConfig">;
export declare function debug(self: TokioClientConfig): string;
export declare function clone(self: TokioClientConfig): TokioClientConfig;
export declare function default$(): TokioClientConfig;
export declare function connectGet(self: TokioClientConfig): TokioConnectConfig;
export declare function connectSet(self: TokioClientConfig, value: TokioConnectConfig): void;
export declare function quicGet(self: TokioClientConfig): TokioQuicConfig;
export declare function quicSet(self: TokioClientConfig, value: TokioQuicConfig): void;
export declare function init(self: TokioClientConfig): rt.Result<TokioClient, TokioError>;
export { default$ as default };

// TokioConnectConfig.ts
import * as rt from "./rt.ts";
import type { AuthToken } from "./AuthToken.ts";
import type { MoqTokioCliDeprecated } from "./MoqTokioCliDeprecated.ts";
import type { MoqTokioConnectResolved } from "./MoqTokioConnectResolved.ts";
import type { MoqTokioConnectionHandover } from "./MoqTokioConnectionHandover.ts";
import type { MoqTokioWebsocketConfig } from "./MoqTokioWebsocketConfig.ts";
import type { TokioBackoff } from "./TokioBackoff.ts";
import type { TokioClient } from "./TokioClient.ts";
import type { TokioDuration } from "./TokioDuration.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioQuicConfig } from "./TokioQuicConfig.ts";
import type { TokioTlsConnect } from "./TokioTlsConnect.ts";
import type { Url } from "./Url.ts";
import type { Version } from "./Version.ts";
import type { Versions } from "./Versions.ts";
export type TokioConnectConfig = rt.Handle<"TokioConnectConfig">;
export declare function debug(self: TokioConnectConfig): string;
export declare function clone(self: TokioConnectConfig): TokioConnectConfig;
export declare function default$(): TokioConnectConfig;
export declare function urlGet(self: TokioConnectConfig): Url | undefined;
export declare function urlSet(self: TokioConnectConfig, value: Url | undefined): void;
export declare function tokensGet(self: TokioConnectConfig): AuthToken[];
export declare function tokensSet(self: TokioConnectConfig, value: AuthToken[]): void;
export declare function bindGet(self: TokioConnectConfig): rt.SocketAddr | undefined;
export declare function bindSet(self: TokioConnectConfig, value: rt.SocketAddr | undefined): void;
export declare function raceGet(self: TokioConnectConfig): TokioDuration | undefined;
export declare function raceSet(self: TokioConnectConfig, value: TokioDuration | undefined): void;
export declare function resolutionDelayGet(self: TokioConnectConfig): TokioDuration | undefined;
export declare function resolutionDelaySet(self: TokioConnectConfig, value: TokioDuration | undefined): void;
export declare function timeoutGet(self: TokioConnectConfig): TokioDuration | undefined;
export declare function timeoutSet(self: TokioConnectConfig, value: TokioDuration | undefined): void;
export declare function versionGet(self: TokioConnectConfig): Version[];
export declare function versionSet(self: TokioConnectConfig, value: Version[]): void;
export declare function implementationGet(self: TokioConnectConfig): string | undefined;
export declare function implementationSet(self: TokioConnectConfig, value: string | undefined): void;
export declare function tlsGet(self: TokioConnectConfig): TokioTlsConnect;
export declare function tlsSet(self: TokioConnectConfig, value: TokioTlsConnect): void;
export declare function backoffGet(self: TokioConnectConfig): TokioBackoff;
export declare function backoffSet(self: TokioConnectConfig, value: TokioBackoff): void;
export declare function goawayGet(self: TokioConnectConfig): MoqTokioConnectionHandover;
export declare function goawaySet(self: TokioConnectConfig, value: MoqTokioConnectionHandover): void;
export declare function websocketGet(self: TokioConnectConfig): MoqTokioWebsocketConfig;
export declare function websocketSet(self: TokioConnectConfig, value: MoqTokioWebsocketConfig): void;
export declare function deprecated(self: TokioConnectConfig): MoqTokioCliDeprecated;
export declare function init(self: TokioConnectConfig, quic: TokioQuicConfig): rt.Result<TokioClient, TokioError>;
export declare function versions(self: TokioConnectConfig): Versions;
export declare function resolve(self: TokioConnectConfig): MoqTokioConnectResolved;
export { default$ as default };

// TokioConnectError.ts
export type TokioConnectError = "Unauthorized" | "Forbidden";
export declare function toString(self: TokioConnectError): string;
export declare function debug(self: TokioConnectError): string;
export declare function isAuth(self: TokioConnectError): boolean;

// TokioConnection.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { AuthTokens } from "./AuthTokens.ts";
import type { BandwidthAllocator } from "./BandwidthAllocator.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { BandwidthViewers } from "./BandwidthViewers.ts";
import type { Error_ } from "./Error_.ts";
import type { GoawayConsumer } from "./GoawayConsumer.ts";
import type { TokioConnectionMonitor } from "./TokioConnectionMonitor.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioStatus } from "./TokioStatus.ts";
import type { TokioTransport } from "./TokioTransport.ts";
import type { Version } from "./Version.ts";
export type TokioConnection = rt.Handle<"TokioConnection">;
export declare function clone(self: TokioConnection): TokioConnection;
export declare function abort(self: TokioConnection, err: Error_): void;
export declare function close(self: TokioConnection): rt.Call<rt.Result<undefined, TokioError>>;
export declare function established(self: TokioConnection): rt.Call<rt.Result<TokioConnection, TokioError>>;
export declare function status(self: TokioConnection): rt.Call<rt.Result<TokioStatus, TokioError>>;
export declare function connected(self: TokioConnection): boolean;
export declare function connects(self: TokioConnection): rt.u64;
export declare function version(self: TokioConnection): Version | undefined;
export declare function transport(self: TokioConnection): TokioTransport | undefined;
export declare function draining(self: TokioConnection): GoawayConsumer | undefined;
export declare function sendBandwidth(self: TokioConnection): BandwidthConsumer;
export declare function allocator(self: TokioConnection): BandwidthAllocator;
export declare function recvBandwidth(self: TokioConnection): BandwidthConsumer;
export declare function viewers(self: TokioConnection): BandwidthViewers;
export declare function closed(self: TokioConnection): rt.Call<rt.Result<undefined, TokioError>>;
export declare function auth(self: TokioConnection): AuthHandle;
export declare function tokens(self: TokioConnection): AuthTokens;
export declare function monitor(self: TokioConnection): TokioConnectionMonitor;

// TokioConnectionMonitor.ts
import * as rt from "./rt.ts";
import type { MoqTokioConnectionObservation } from "./MoqTokioConnectionObservation.ts";
import type { MoqTokioConnectionSnapshot } from "./MoqTokioConnectionSnapshot.ts";
import type { SessionStats } from "./SessionStats.ts";
import type { StatsPresence } from "./StatsPresence.ts";
import type { TokioError } from "./TokioError.ts";
export type TokioConnectionMonitor = rt.Handle<"TokioConnectionMonitor">;
export declare function clone(self: TokioConnectionMonitor): TokioConnectionMonitor;
export declare function state(self: TokioConnectionMonitor): MoqTokioConnectionObservation;
export declare function stateChanged(self: TokioConnectionMonitor, after: rt.u64): rt.Call<rt.Result<MoqTokioConnectionObservation, TokioError>>;
export declare function presence(self: TokioConnectionMonitor): StatsPresence;
export declare function pollPresence(self: TokioConnectionMonitor): rt.Call<rt.Result<StatsPresence, TokioError>>;
export declare function presenceChanged(self: TokioConnectionMonitor): rt.Call<rt.Result<StatsPresence, TokioError>>;
export declare function stats(self: TokioConnectionMonitor): SessionStats | undefined;
export declare function failure(self: TokioConnectionMonitor): TokioError | undefined;
export declare function snapshot(self: TokioConnectionMonitor): MoqTokioConnectionSnapshot | undefined;

// TokioDuration.ts
import * as rt from "./rt.ts";
export type TokioDuration = rt.Handle<"TokioDuration">;
export declare function toString(self: TokioDuration): string;
export declare function debug(self: TokioDuration): string;
export declare function clone(self: TokioDuration): TokioDuration;
export declare function eq(a: TokioDuration, b: TokioDuration): boolean;
export declare function hash(self: TokioDuration): rt.u64;
export declare function partialCmp(a: TokioDuration, b: TokioDuration): number | undefined;
export declare function cmp(a: TokioDuration, b: TokioDuration): number;
export declare function default$(): TokioDuration;
export { default$ as default };

// TokioError.ts
import * as rt from "./rt.ts";
import type { TokioConnectError } from "./TokioConnectError.ts";
export type TokioError = rt.Handle<"TokioError">;
export declare function toString(self: TokioError): string;
export declare function debug(self: TokioError): string;
export declare function clone(self: TokioError): TokioError;
export declare function connectError(self: TokioError): TokioConnectError | undefined;
export declare function isAuth(self: TokioError): boolean;
export declare function status(self: TokioError): number | undefined;

// TokioListenBind.ts
import * as rt from "./rt.ts";
export type TokioListenBind = {
    tag: "Addr";
    _0: rt.SocketAddr;
} | {
    tag: "Host";
    _0: string;
    _1: number;
};
export declare function toString(self: TokioListenBind): string;
export declare function debug(self: TokioListenBind): string;
export declare function eq(a: TokioListenBind, b: TokioListenBind): boolean;
export declare function resolve(self: TokioListenBind): rt.Result<rt.SocketAddr, string>;

// TokioListenConfig.ts
import * as rt from "./rt.ts";
import type { MoqTokioCliDeprecated } from "./MoqTokioCliDeprecated.ts";
import type { MoqTokioListenLoadBalancer } from "./MoqTokioListenLoadBalancer.ts";
import type { MoqTokioListenReset } from "./MoqTokioListenReset.ts";
import type { MoqTokioListenResolved } from "./MoqTokioListenResolved.ts";
import type { MoqTokioTcpConfig } from "./MoqTokioTcpConfig.ts";
import type { MoqTokioUnixConfig } from "./MoqTokioUnixConfig.ts";
import type { Server } from "./Server.ts";
import type { TokioDuration } from "./TokioDuration.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioListenBind } from "./TokioListenBind.ts";
import type { TokioQuicConfig } from "./TokioQuicConfig.ts";
import type { TokioServer } from "./TokioServer.ts";
import type { TokioTlsListen } from "./TokioTlsListen.ts";
import type { Version } from "./Version.ts";
import type { Versions } from "./Versions.ts";
export type TokioListenConfig = rt.Handle<"TokioListenConfig">;
export declare function debug(self: TokioListenConfig): string;
export declare function clone(self: TokioListenConfig): TokioListenConfig;
export declare function default$(): TokioListenConfig;
export declare function bindGet(self: TokioListenConfig): TokioListenBind | undefined;
export declare function bindSet(self: TokioListenConfig, value: TokioListenBind | undefined): void;
export declare function tcpGet(self: TokioListenConfig): MoqTokioTcpConfig;
export declare function tcpSet(self: TokioListenConfig, value: MoqTokioTcpConfig): void;
export declare function unixGet(self: TokioListenConfig): MoqTokioUnixConfig;
export declare function unixSet(self: TokioListenConfig, value: MoqTokioUnixConfig): void;
export declare function versionGet(self: TokioListenConfig): Version[];
export declare function versionSet(self: TokioListenConfig, value: Version[]): void;
export declare function implementationGet(self: TokioListenConfig): string | undefined;
export declare function implementationSet(self: TokioListenConfig, value: string | undefined): void;
export declare function timeoutGet(self: TokioListenConfig): TokioDuration | undefined;
export declare function timeoutSet(self: TokioListenConfig, value: TokioDuration | undefined): void;
export declare function tlsGet(self: TokioListenConfig): TokioTlsListen;
export declare function tlsSet(self: TokioListenConfig, value: TokioTlsListen): void;
export declare function preferredV4Get(self: TokioListenConfig): rt.SocketAddrV4 | undefined;
export declare function preferredV4Set(self: TokioListenConfig, value: rt.SocketAddrV4 | undefined): void;
export declare function preferredV6Get(self: TokioListenConfig): rt.SocketAddrV6 | undefined;
export declare function preferredV6Set(self: TokioListenConfig, value: rt.SocketAddrV6 | undefined): void;
export declare function lbGet(self: TokioListenConfig): MoqTokioListenLoadBalancer;
export declare function lbSet(self: TokioListenConfig, value: MoqTokioListenLoadBalancer): void;
export declare function resetGet(self: TokioListenConfig): MoqTokioListenReset;
export declare function resetSet(self: TokioListenConfig, value: MoqTokioListenReset): void;
export declare function deprecated(self: TokioListenConfig): MoqTokioCliDeprecated;
export declare function resolvedTimeout(self: TokioListenConfig): rt.Duration | undefined;
export declare function resolve(self: TokioListenConfig): rt.Result<MoqTokioListenResolved, TokioError>;
export declare function init(self: TokioListenConfig, quic: TokioQuicConfig): rt.Result<TokioServer, TokioError>;
export declare function initStreams(self: TokioListenConfig): rt.Result<TokioServer, TokioError>;
export declare function versions(self: TokioListenConfig): Versions;
export declare function server(self: TokioListenConfig): Server;
export declare function hasExplicitBind(self: TokioListenConfig): boolean;
export declare function servesQuic(self: TokioListenConfig): boolean;
export { default$ as default };

// TokioLog.ts
import * as rt from "./rt.ts";
import type { Level } from "./Level.ts";
import type { TokioError } from "./TokioError.ts";
export type TokioLog = rt.Handle<"TokioLog">;
export declare function debug(self: TokioLog): string;
export declare function clone(self: TokioLog): TokioLog;
export declare function default$(): TokioLog;
export declare function levelGet(self: TokioLog): Level;
export declare function levelSet(self: TokioLog, value: Level): void;
export declare function new$(level: Level): TokioLog;
export declare function level(self: TokioLog): Level | undefined;
export declare function init(self: TokioLog): rt.Result<undefined, TokioError>;
export { default$ as default, new$ as new };

// TokioQuicConfig.ts
import * as rt from "./rt.ts";
import type { MoqTokioCliDeprecated } from "./MoqTokioCliDeprecated.ts";
import type { MoqTokioQuicCongestionControl } from "./MoqTokioQuicCongestionControl.ts";
import type { MoqTokioQuicResolved } from "./MoqTokioQuicResolved.ts";
import type { TokioDuration } from "./TokioDuration.ts";
import type { TokioError } from "./TokioError.ts";
export type TokioQuicConfig = rt.Handle<"TokioQuicConfig">;
export declare function debug(self: TokioQuicConfig): string;
export declare function clone(self: TokioQuicConfig): TokioQuicConfig;
export declare function default$(): TokioQuicConfig;
export declare function maxStreamsGet(self: TokioQuicConfig): rt.u64 | undefined;
export declare function maxStreamsSet(self: TokioQuicConfig, value: rt.u64 | undefined): void;
export declare function gsoGet(self: TokioQuicConfig): boolean | undefined;
export declare function gsoSet(self: TokioQuicConfig, value: boolean | undefined): void;
export declare function idleTimeoutGet(self: TokioQuicConfig): TokioDuration | undefined;
export declare function idleTimeoutSet(self: TokioQuicConfig, value: TokioDuration | undefined): void;
export declare function keepAliveGet(self: TokioQuicConfig): TokioDuration | undefined;
export declare function keepAliveSet(self: TokioQuicConfig, value: TokioDuration | undefined): void;
export declare function mtuDiscoveryGet(self: TokioQuicConfig): boolean | undefined;
export declare function mtuDiscoverySet(self: TokioQuicConfig, value: boolean | undefined): void;
export declare function congestionControlGet(self: TokioQuicConfig): MoqTokioQuicCongestionControl | undefined;
export declare function congestionControlSet(self: TokioQuicConfig, value: MoqTokioQuicCongestionControl | undefined): void;
export declare function receiveWindowGet(self: TokioQuicConfig): rt.u64 | undefined;
export declare function receiveWindowSet(self: TokioQuicConfig, value: rt.u64 | undefined): void;
export declare function streamReceiveWindowGet(self: TokioQuicConfig): rt.u64 | undefined;
export declare function streamReceiveWindowSet(self: TokioQuicConfig, value: rt.u64 | undefined): void;
export declare function sendWindowGet(self: TokioQuicConfig): rt.u64 | undefined;
export declare function sendWindowSet(self: TokioQuicConfig, value: rt.u64 | undefined): void;
export declare function qlogGet(self: TokioQuicConfig): rt.PathBuf | undefined;
export declare function qlogSet(self: TokioQuicConfig, value: rt.PathBuf | undefined): void;
export declare function keylogGet(self: TokioQuicConfig): rt.PathBuf | undefined;
export declare function keylogSet(self: TokioQuicConfig, value: rt.PathBuf | undefined): void;
export declare function deprecated(self: TokioQuicConfig): MoqTokioCliDeprecated;
export declare function validate(self: TokioQuicConfig): rt.Result<undefined, TokioError>;
export declare function resolve(self: TokioQuicConfig): MoqTokioQuicResolved;
export { default$ as default };

// TokioRestart.ts
export type TokioRestart = "Redial" | "Handoff";
export declare function debug(self: TokioRestart): string;
export declare function default$(): TokioRestart;
export { default$ as default };

// TokioServer.ts
import * as rt from "./rt.ts";
import type { MoqTokioAcceptHealth } from "./MoqTokioAcceptHealth.ts";
import type { MoqTokioTcpTlsAcceptor } from "./MoqTokioTcpTlsAcceptor.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioServerConfig } from "./TokioServerConfig.ts";
import type { TokioServerListener } from "./TokioServerListener.ts";
import type { TokioTlsCertificates } from "./TokioTlsCertificates.ts";
export type TokioServer = rt.Handle<"TokioServer">;
export declare function new$(config: TokioServerConfig): rt.Result<TokioServer, TokioError>;
export declare function servePublish(self: TokioServer, origin: OriginConsumer): rt.Call<rt.Result<undefined, TokioError>>;
export declare function tlsAcceptor(self: TokioServer): MoqTokioTcpTlsAcceptor | undefined;
export declare function certificates(self: TokioServer): TokioTlsCertificates;
export declare function acceptHealth(self: TokioServer): MoqTokioAcceptHealth[];
export declare function bind(self: TokioServer): rt.Call<rt.Result<TokioServerListener, TokioError>>;
export declare function listen(self: TokioServer): rt.Call<rt.Result<TokioServerListener, TokioError>>;
export declare function localAddr(self: TokioServer): rt.Result<rt.SocketAddr, TokioError>;
export declare function websocketLocalAddr(self: TokioServer): rt.SocketAddr | undefined;
export { new$ as new };

// TokioServerConfig.ts
import * as rt from "./rt.ts";
import type { MoqTokioQuicDemuxDemux } from "./MoqTokioQuicDemuxDemux.ts";
import type { MoqTokioServerStreams } from "./MoqTokioServerStreams.ts";
import type { MoqTokioWebsocketListener } from "./MoqTokioWebsocketListener.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { ServerAdmission } from "./ServerAdmission.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioListenConfig } from "./TokioListenConfig.ts";
import type { TokioQuicConfig } from "./TokioQuicConfig.ts";
import type { TokioServer } from "./TokioServer.ts";
import type { TokioTlsServe } from "./TokioTlsServe.ts";
export type TokioServerConfig = rt.Handle<"TokioServerConfig">;
export declare function default$(): TokioServerConfig;
export declare function tlsGet(self: TokioServerConfig): TokioTlsServe | undefined;
export declare function tlsSet(self: TokioServerConfig, value: TokioTlsServe | undefined): void;
export declare function admissionGet(self: TokioServerConfig): ServerAdmission;
export declare function admissionSet(self: TokioServerConfig, value: ServerAdmission): void;
export declare function listenGet(self: TokioServerConfig): TokioListenConfig;
export declare function listenSet(self: TokioServerConfig, value: TokioListenConfig): void;
export declare function quicGet(self: TokioServerConfig): TokioQuicConfig;
export declare function quicSet(self: TokioServerConfig, value: TokioQuicConfig): void;
export declare function websocketGet(self: TokioServerConfig): MoqTokioWebsocketListener | undefined;
export declare function websocketSet(self: TokioServerConfig, value: MoqTokioWebsocketListener | undefined): void;
export declare function publisherGet(self: TokioServerConfig): OriginConsumer | undefined;
export declare function publisherSet(self: TokioServerConfig, value: OriginConsumer | undefined): void;
export declare function subscriberGet(self: TokioServerConfig): OriginProducer | undefined;
export declare function subscriberSet(self: TokioServerConfig, value: OriginProducer | undefined): void;
export declare function statsGet(self: TokioServerConfig): StatsSession;
export declare function statsSet(self: TokioServerConfig, value: StatsSession): void;
export declare function demuxGet(self: TokioServerConfig): MoqTokioQuicDemuxDemux;
export declare function demuxSet(self: TokioServerConfig, value: MoqTokioQuicDemuxDemux): void;
export declare function streamsGet(self: TokioServerConfig): MoqTokioServerStreams;
export declare function streamsSet(self: TokioServerConfig, value: MoqTokioServerStreams): void;
export declare function init(self: TokioServerConfig): rt.Result<TokioServer, TokioError>;
export declare function initStreams(self: TokioServerConfig): rt.Result<TokioServer, TokioError>;
export { default$ as default };

// TokioServerListener.ts
import * as rt from "./rt.ts";
import type { MoqTokioAcceptHealth } from "./MoqTokioAcceptHealth.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioServerRequest } from "./TokioServerRequest.ts";
import type { TokioTlsCertificates } from "./TokioTlsCertificates.ts";
export type TokioServerListener = rt.Handle<"TokioServerListener">;
export declare function accept(self: TokioServerListener): rt.Call<rt.Result<TokioServerRequest, TokioError>>;
export declare function close(self: TokioServerListener): rt.Call<void>;
export declare function closeStreams(self: TokioServerListener): rt.Call<void>;
export declare function localAddr(self: TokioServerListener): rt.Result<rt.SocketAddr, TokioError>;
export declare function websocketLocalAddr(self: TokioServerListener): rt.SocketAddr | undefined;
export declare function tcpLocalAddr(self: TokioServerListener): rt.SocketAddr | undefined;
export declare function certificates(self: TokioServerListener): TokioTlsCertificates;
export declare function acceptHealth(self: TokioServerListener): MoqTokioAcceptHealth[];

// TokioServerRequest.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { AuthRequests } from "./AuthRequests.ts";
import type { MoqQuicLimits } from "./MoqQuicLimits.ts";
import type { MoqTokioServerQuicLimits } from "./MoqTokioServerQuicLimits.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Session } from "./Session.ts";
import type { SessionError } from "./SessionError.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupRole } from "./SetupRole.ts";
import type { SetupToken } from "./SetupToken.ts";
import type { StatsSession } from "./StatsSession.ts";
import type { TokioError } from "./TokioError.ts";
import type { TokioTlsPeerIdentity } from "./TokioTlsPeerIdentity.ts";
import type { TokioTransport } from "./TokioTransport.ts";
import type { Url } from "./Url.ts";
export type TokioServerRequest = rt.Handle<"TokioServerRequest">;
export declare function reject(self: TokioServerRequest, code: SessionError): rt.Call<rt.Result<undefined, TokioError>>;
export declare function withPublisherProducer(self: TokioServerRequest, publish: OriginProducer): TokioServerRequest;
export declare function withPublisherConsumer(self: TokioServerRequest, publish: OriginConsumer): TokioServerRequest;
export declare function withSubscriber(self: TokioServerRequest, subscribe: OriginProducer): TokioServerRequest;
export declare function withPeerNode(self: TokioServerRequest, node: rt.u64): TokioServerRequest;
export declare function withStats(self: TokioServerRequest, stats: StatsSession): TokioServerRequest;
export declare function withLimits(self: TokioServerRequest, limits: SessionLimits): TokioServerRequest;
export declare function quicLimits(self: TokioServerRequest): MoqTokioServerQuicLimits | undefined;
export declare function withQuicLimits(self: TokioServerRequest, limits: MoqQuicLimits): rt.Result<TokioServerRequest, string>;
export declare function expired(self: TokioServerRequest): rt.Call<void>;
export declare function ok(self: TokioServerRequest): rt.Call<rt.Result<Session, TokioError>>;
export declare function transport(self: TokioServerRequest): TokioTransport;
export declare function url(self: TokioServerRequest): Url | undefined;
export declare function authority(self: TokioServerRequest): string | undefined;
export declare function path(self: TokioServerRequest): string;
export declare function query(self: TokioServerRequest): string | undefined;
export declare function remoteAddr(self: TokioServerRequest): rt.SocketAddr | undefined;
export declare function localAddr(self: TokioServerRequest): rt.SocketAddr | undefined;
export declare function serverName(self: TokioServerRequest): string | undefined;
export declare function alpn(self: TokioServerRequest): string | undefined;
export declare function role(self: TokioServerRequest): SetupRole | undefined;
export declare function auth(self: TokioServerRequest): AuthHandle;
export declare function requests(self: TokioServerRequest): AuthRequests;
export declare function peerNode(self: TokioServerRequest): rt.u64 | undefined;
export declare function peerImplementation(self: TokioServerRequest): string | undefined;
export declare function token(self: TokioServerRequest): SetupToken | undefined;
export declare function peerIdentity(self: TokioServerRequest): TokioTlsPeerIdentity | undefined;

// TokioStatus.ts
export type TokioStatus = "Connected" | "Disconnected" | "Migrating";
export declare function debug(self: TokioStatus): string;

// TokioTlsCertificates.ts
import * as rt from "./rt.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
export type TokioTlsCertificates = rt.Handle<"TokioTlsCertificates">;
export declare function debug(self: TokioTlsCertificates): string;
export declare function clone(self: TokioTlsCertificates): TokioTlsCertificates;
export declare function fromPem(chain: Uint8Array | rt.Bytes): rt.Result<TokioTlsCertificates, TokioTlsError>;
export declare function fingerprints(self: TokioTlsCertificates): string[];

// TokioTlsConnect.ts
import * as rt from "./rt.ts";
import type { MoqTokioCliDeprecated } from "./MoqTokioCliDeprecated.ts";
import type { RustlsClientConfig } from "./RustlsClientConfig.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
import type { TokioTlsIdentity } from "./TokioTlsIdentity.ts";
export type TokioTlsConnect = rt.Handle<"TokioTlsConnect">;
export declare function debug(self: TokioTlsConnect): string;
export declare function clone(self: TokioTlsConnect): TokioTlsConnect;
export declare function default$(): TokioTlsConnect;
export declare function rootGet(self: TokioTlsConnect): rt.PathBuf[];
export declare function rootSet(self: TokioTlsConnect, value: rt.PathBuf[]): void;
export declare function systemRootsGet(self: TokioTlsConnect): boolean | undefined;
export declare function systemRootsSet(self: TokioTlsConnect, value: boolean | undefined): void;
export declare function fingerprintGet(self: TokioTlsConnect): string[];
export declare function fingerprintSet(self: TokioTlsConnect, value: string[]): void;
export declare function identityGet(self: TokioTlsConnect): TokioTlsIdentity | undefined;
export declare function identitySet(self: TokioTlsConnect, value: TokioTlsIdentity | undefined): void;
export declare function certGet(self: TokioTlsConnect): rt.PathBuf | undefined;
export declare function certSet(self: TokioTlsConnect, value: rt.PathBuf | undefined): void;
export declare function keyGet(self: TokioTlsConnect): rt.PathBuf | undefined;
export declare function keySet(self: TokioTlsConnect, value: rt.PathBuf | undefined): void;
export declare function insecureGet(self: TokioTlsConnect): boolean | undefined;
export declare function insecureSet(self: TokioTlsConnect, value: boolean | undefined): void;
export declare function hostNameGet(self: TokioTlsConnect): string | undefined;
export declare function hostNameSet(self: TokioTlsConnect, value: string | undefined): void;
export declare function deprecated(self: TokioTlsConnect): MoqTokioCliDeprecated;
export declare function build(self: TokioTlsConnect): rt.Result<RustlsClientConfig, TokioTlsError>;
export { default$ as default };

// TokioTlsError.ts
import * as rt from "./rt.ts";
export type TokioTlsError = rt.Handle<"TokioTlsError">;
export declare function toString(self: TokioTlsError): string;
export declare function debug(self: TokioTlsError): string;

// TokioTlsIdentity.ts
import * as rt from "./rt.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
export type TokioTlsIdentity = rt.Handle<"TokioTlsIdentity">;
export declare function debug(self: TokioTlsIdentity): string;
export declare function clone(self: TokioTlsIdentity): TokioTlsIdentity;
export declare function generate(hostnames: string[]): rt.Result<TokioTlsIdentity, TokioTlsError>;
export declare function fingerprint(self: TokioTlsIdentity): string;

// TokioTlsKeyLog.ts
import * as rt from "./rt.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
export type TokioTlsKeyLog = rt.Handle<"TokioTlsKeyLog">;
export declare function debug(self: TokioTlsKeyLog): string;
export declare function open(path: rt.PathBuf): rt.Result<TokioTlsKeyLog, TokioTlsError>;

// TokioTlsListen.ts
import * as rt from "./rt.ts";
import type { MoqTokioCliDeprecated } from "./MoqTokioCliDeprecated.ts";
import type { RustlsRootCertStore } from "./RustlsRootCertStore.ts";
import type { RustlsServerConfig } from "./RustlsServerConfig.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
import type { TokioTlsIdentity } from "./TokioTlsIdentity.ts";
import type { TokioTlsServe } from "./TokioTlsServe.ts";
export type TokioTlsListen = rt.Handle<"TokioTlsListen">;
export declare function debug(self: TokioTlsListen): string;
export declare function clone(self: TokioTlsListen): TokioTlsListen;
export declare function default$(): TokioTlsListen;
export declare function certGet(self: TokioTlsListen): rt.PathBuf[];
export declare function certSet(self: TokioTlsListen, value: rt.PathBuf[]): void;
export declare function keyGet(self: TokioTlsListen): rt.PathBuf[];
export declare function keySet(self: TokioTlsListen, value: rt.PathBuf[]): void;
export declare function generateGet(self: TokioTlsListen): string[];
export declare function generateSet(self: TokioTlsListen, value: string[]): void;
export declare function identityGet(self: TokioTlsListen): TokioTlsIdentity | undefined;
export declare function identitySet(self: TokioTlsListen, value: TokioTlsIdentity | undefined): void;
export declare function rootGet(self: TokioTlsListen): rt.PathBuf[];
export declare function rootSet(self: TokioTlsListen, value: rt.PathBuf[]): void;
export declare function load(self: TokioTlsListen): rt.Result<TokioTlsServe, TokioTlsError>;
export declare function deprecated(self: TokioTlsListen): MoqTokioCliDeprecated;
export declare function loadRoots(self: TokioTlsListen): rt.Result<RustlsRootCertStore, TokioTlsError>;
export declare function serverConfig(self: TokioTlsListen, alpn: Uint8Array[]): rt.Result<RustlsServerConfig, TokioTlsError>;
export { default$ as default };

// TokioTlsPeerIdentity.ts
import * as rt from "./rt.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { RustlsPkiTypesCertificateDer } from "./RustlsPkiTypesCertificateDer.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
export type TokioTlsPeerIdentity = rt.Handle<"TokioTlsPeerIdentity">;
export declare function debug(self: TokioTlsPeerIdentity): string;
export declare function clone(self: TokioTlsPeerIdentity): TokioTlsPeerIdentity;
export declare function chain(self: TokioTlsPeerIdentity): RustlsPkiTypesCertificateDer[];
export declare function fingerprint(self: TokioTlsPeerIdentity): string | undefined;
export declare function name(self: TokioTlsPeerIdentity): string | undefined;
export declare function lineage(self: TokioTlsPeerIdentity): rt.Result<string[], TokioTlsError>;
export declare function issuer(self: TokioTlsPeerIdentity): string | undefined;
export declare function expiry(self: TokioTlsPeerIdentity): ClockWall | undefined;

// TokioTlsServe.ts
import * as rt from "./rt.ts";
import type { RustlsPkiTypesCertificateDer } from "./RustlsPkiTypesCertificateDer.ts";
import type { RustlsServerConfig } from "./RustlsServerConfig.ts";
import type { TokioTlsCertificates } from "./TokioTlsCertificates.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
import type { TokioTlsPeerIdentity } from "./TokioTlsPeerIdentity.ts";
export type TokioTlsServe = rt.Handle<"TokioTlsServe">;
export declare function debug(self: TokioTlsServe): string;
export declare function clone(self: TokioTlsServe): TokioTlsServe;
export declare function certificates(self: TokioTlsServe): TokioTlsCertificates;
export declare function config(self: TokioTlsServe, alpn: Uint8Array[]): rt.Result<RustlsServerConfig, TokioTlsError>;
export declare function peer(self: TokioTlsServe, chain: RustlsPkiTypesCertificateDer[]): TokioTlsPeerIdentity;

// TokioTransport.ts
export type TokioTransport = "Quic" | "WebSocket" | "Tcp" | "Unix" | "WebTransport" | "Tls";
export declare function toString(self: TokioTransport): string;
export declare function debug(self: TokioTransport): string;
export declare function asStr(self: TokioTransport): string;

// TrackAbortHandle.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type TrackAbortHandle = rt.Handle<"TrackAbortHandle">;
export declare function clone(self: TrackAbortHandle): TrackAbortHandle;
export declare function isClosed(self: TrackAbortHandle): boolean;
export declare function abort(self: TrackAbortHandle, err: Error_): void;

// TrackAnchor.ts
import * as rt from "./rt.ts";
export type TrackAnchor = {
    tag: "Relative";
    _0: rt.u64;
} | {
    tag: "Oldest";
} | {
    tag: "Time";
    _0: rt.u64;
};
export declare function debug(self: TrackAnchor): string;
export declare function eq(a: TrackAnchor, b: TrackAnchor): boolean;
export declare function hash(self: TrackAnchor): rt.u64;
export declare function partialCmp(a: TrackAnchor, b: TrackAnchor): number | undefined;
export declare function cmp(a: TrackAnchor, b: TrackAnchor): number;

// TrackCompression.ts
export type TrackCompression = "None" | "Deflate";
export declare function debug(self: TrackCompression): string;
export declare function default$(): TrackCompression;
export { default$ as default };

// TrackConsumer.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { GroupFetch } from "./GroupFetch.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackConsumer = rt.Handle<"TrackConsumer">;
export declare function clone(self: TrackConsumer): TrackConsumer;
export declare function unmetered(self: TrackConsumer): TrackConsumer;
export declare function name(self: TrackConsumer): string;
export declare function broadcast(self: TrackConsumer): BroadcastInfo;
export declare function subscribe(self: TrackConsumer, subscription: TrackSubscription | undefined): rt.Call<rt.Result<TrackSubscriber, Error_>>;
export declare function fetchGroup(self: TrackConsumer, key: rt.u64, options: GroupFetch | undefined): rt.Call<rt.Result<GroupConsumer, Error_>>;
export declare function info(self: TrackConsumer): rt.Call<rt.Result<TrackInfo, Error_>>;
export declare function largest(self: TrackConsumer): rt.Call<TrackPosition | undefined>;

// TrackControl.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackControl = rt.Handle<"TrackControl">;
export declare function clone(self: TrackControl): TrackControl;
export declare function subscription(self: TrackControl): TrackSubscription;
export declare function update(self: TrackControl, subscription: TrackSubscription): rt.Result<undefined, Error_>;
export declare function refresh(self: TrackControl): rt.Call<rt.Result<undefined, Error_>>;

// TrackDelay.ts
import * as rt from "./rt.ts";
export type TrackDelay = {
    min: rt.Duration | undefined;
    max: rt.Duration | undefined;
};
export declare function debug(self: TrackDelay): string;
export declare function eq(a: TrackDelay, b: TrackDelay): boolean;
export declare function default$(): TrackDelay;
export declare function withMin(self: TrackDelay, min: rt.Duration | undefined): TrackDelay;
export declare function withMax(self: TrackDelay, max: rt.Duration | undefined): TrackDelay;
export { default$ as default };

// TrackDemand.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type TrackDemand = rt.Handle<"TrackDemand">;
export declare function clone(self: TrackDemand): TrackDemand;
export declare function name(self: TrackDemand): string;
export declare function wanted(self: TrackDemand): boolean;
export declare function wantedChanged(self: TrackDemand): rt.Call<rt.Result<boolean, Error_>>;
export declare function refresh(self: TrackDemand): rt.Call<rt.Result<undefined, Error_>>;
export declare function closed(self: TrackDemand): rt.Call<rt.Result<undefined, Error_>>;

// TrackFill.ts
import * as rt from "./rt.ts";
export type TrackFill = rt.Handle<"TrackFill">;

// TrackFinished.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackFinished = rt.Handle<"TrackFinished">;
export declare function name(self: TrackFinished): string;
export declare function broadcast(self: TrackFinished): BroadcastInfo;
export declare function info(self: TrackFinished): TrackInfo;
export declare function requests(self: TrackFinished): TrackRequests;
export declare function createGroup(self: TrackFinished, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function abort(self: TrackFinished, err: Error_): rt.Result<undefined, Error_>;
export declare function closed(self: TrackFinished): rt.Call<rt.Result<undefined, Error_>>;
export declare function consume(self: TrackFinished): TrackConsumer;
export declare function subscribe(self: TrackFinished, subscription: TrackSubscription | undefined): TrackSubscriber;

// TrackInfo.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackCompression } from "./TrackCompression.ts";
import type { TrackRefresh } from "./TrackRefresh.ts";
export type TrackInfo = rt.Handle<"TrackInfo">;
export declare function debug(self: TrackInfo): string;
export declare function clone(self: TrackInfo): TrackInfo;
export declare function eq(a: TrackInfo, b: TrackInfo): boolean;
export declare function default$(): TrackInfo;
export declare function timescaleGet(self: TrackInfo): Timescale | undefined;
export declare function timescaleSet(self: TrackInfo, value: Timescale | undefined): void;
export declare function maxAgeGet(self: TrackInfo): rt.Duration | undefined;
export declare function maxAgeSet(self: TrackInfo, value: rt.Duration | undefined): void;
export declare function priorityGet(self: TrackInfo): number;
export declare function prioritySet(self: TrackInfo, value: number): void;
export declare function compressionGet(self: TrackInfo): TrackCompression;
export declare function compressionSet(self: TrackInfo, value: TrackCompression): void;
export declare function refreshGet(self: TrackInfo): TrackRefresh;
export declare function refreshSet(self: TrackInfo, value: TrackRefresh): void;
export declare function withTimescale(self: TrackInfo, timescale: Timescale | undefined): TrackInfo;
export declare function withMaxAge(self: TrackInfo, maxAge: rt.Duration | undefined): TrackInfo;
export declare function withPriority(self: TrackInfo, priority: number): TrackInfo;
export declare function withCompression(self: TrackInfo, compression: TrackCompression): TrackInfo;
export declare function withRefresh(self: TrackInfo, refresh: TrackRefresh): TrackInfo;
export declare function rescale(self: TrackInfo, timestamp: Timestamp): rt.Result<Timestamp, Error_>;
export declare function rescaleIn(self: TrackInfo, timestamp: Timestamp, key: rt.u64): rt.Result<Timestamp, Error_>;
export { default$ as default };

// TrackItem.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
export type TrackItem = rt.Handle<"TrackItem">;
export declare function group(self: TrackItem): GroupConsumer | undefined;

// TrackOrder.ts
export type TrackOrder = "Ascending" | "Descending";
export declare function debug(self: TrackOrder): string;
export declare function default$(): TrackOrder;
export { default$ as default };

// TrackOrdered.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackControl } from "./TrackControl.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackItem } from "./TrackItem.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackOrdered = rt.Handle<"TrackOrdered">;
export declare function info(self: TrackOrdered): TrackInfo;
export declare function name(self: TrackOrdered): string;
export declare function broadcast(self: TrackOrdered): BroadcastInfo;
export declare function nextGroup(self: TrackOrdered): rt.Call<rt.Result<TrackItem | undefined, Error_>>;
export declare function control(self: TrackOrdered): TrackControl;
export declare function refresh(self: TrackOrdered): rt.Call<rt.Result<undefined, Error_>>;
export declare function subscription(self: TrackOrdered): TrackSubscription;
export declare function update(self: TrackOrdered, subscription: TrackSubscription): rt.Result<undefined, Error_>;
export declare function finished(self: TrackOrdered): rt.Call<rt.Result<TrackPosition, Error_>>;
export declare function largest(self: TrackOrdered): rt.Call<TrackPosition | undefined>;
export declare function isSkipped(self: TrackOrdered, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function isClone(self: TrackOrdered, other: TrackOrdered): boolean;

// TrackPosition.ts
import * as rt from "./rt.ts";
export type TrackPosition = rt.Opaque<"TrackPosition">;
export declare function debug(self: TrackPosition): string;
export declare function eq(a: TrackPosition, b: TrackPosition): boolean;
export declare function hash(self: TrackPosition): rt.u64;
export declare function partialCmp(a: TrackPosition, b: TrackPosition): number | undefined;
export declare function cmp(a: TrackPosition, b: TrackPosition): number;
export declare function default$(): TrackPosition;
export declare function groupGet(self: TrackPosition): rt.u64;
export declare function groupSet(self: TrackPosition, value: rt.u64): TrackPosition;
export declare function frameGet(self: TrackPosition): rt.u64;
export declare function frameSet(self: TrackPosition, value: rt.u64): TrackPosition;
export declare function new$(group: rt.u64, frame: rt.u64): TrackPosition;
export declare function group(group: rt.u64): TrackPosition;
export { default$ as default, new$ as new };

// TrackProducer.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAbortHandle } from "./TrackAbortHandle.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackFinished } from "./TrackFinished.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackProducer = rt.Handle<"TrackProducer">;
export declare function clone(self: TrackProducer): TrackProducer;
export declare function name(self: TrackProducer): string;
export declare function broadcast(self: TrackProducer): BroadcastInfo;
export declare function info(self: TrackProducer): TrackInfo;
export declare function createGroup(self: TrackProducer, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: TrackProducer, start: Timestamp): rt.Result<GroupProducer, Error_>;
export declare function writeFrame(self: TrackProducer, timestamp: Timestamp, frame: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function finish(self: TrackProducer): rt.Result<TrackFinished, Error_>;
export declare function fail(self: TrackProducer, err: Error_): rt.Result<TrackFinished, Error_>;
export declare function finishAt(self: TrackProducer, end: TrackPosition): rt.Result<undefined, Error_>;
export declare function startAt(self: TrackProducer, key: rt.u64 | undefined): rt.Result<undefined, Error_>;
export declare function skipGroups(self: TrackProducer, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function finished(self: TrackProducer): TrackPosition | undefined;
export declare function abort(self: TrackProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function abortUnwanted(self: TrackProducer, err: Error_): rt.Result<undefined, TrackProducer>;
export declare function closed(self: TrackProducer): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: TrackProducer): boolean;
export declare function largest(self: TrackProducer): TrackPosition | undefined;
export declare function isClone(self: TrackProducer, other: TrackProducer): boolean;
export declare function abortHandle(self: TrackProducer): TrackAbortHandle;
export declare function demand(self: TrackProducer): TrackDemand;
export declare function consume(self: TrackProducer): TrackConsumer;
export declare function subscribe(self: TrackProducer, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscription(self: TrackProducer): TrackSubscription | undefined;
export declare function subscriptionChanged(self: TrackProducer): rt.Call<rt.Result<TrackSubscription | undefined, Error_>>;
export declare function requests(self: TrackProducer): TrackRequests;

// TrackRange.ts
import * as rt from "./rt.ts";
import type { TrackAnchor } from "./TrackAnchor.ts";
export type TrackRange = {
    start: TrackAnchor;
    end: TrackAnchor | undefined;
};
export declare function debug(self: TrackRange): string;
export declare function eq(a: TrackRange, b: TrackRange): boolean;
export declare function hash(self: TrackRange): rt.u64;
export declare function partialCmp(a: TrackRange, b: TrackRange): number | undefined;
export declare function cmp(a: TrackRange, b: TrackRange): number;
export declare function live(): TrackRange;
export declare function oldest(): TrackRange;

// TrackRefresh.ts
import * as rt from "./rt.ts";
export type TrackRefresh = rt.Handle<"TrackRefresh">;
export declare function debug(self: TrackRefresh): string;
export declare function clone(self: TrackRefresh): TrackRefresh;
export declare function eq(a: TrackRefresh, b: TrackRefresh): boolean;
export declare function hash(self: TrackRefresh): rt.u64;
export declare function default$(): TrackRefresh;
export declare function ratioGet(self: TrackRefresh): number | undefined;
export declare function ratioSet(self: TrackRefresh, value: number | undefined): void;
export declare function intervalGet(self: TrackRefresh): rt.Duration | undefined;
export declare function intervalSet(self: TrackRefresh, value: rt.Duration | undefined): void;
export declare function withRatio(self: TrackRefresh, ratio: number | undefined): TrackRefresh;
export declare function withInterval(self: TrackRefresh, interval: rt.Duration | undefined): TrackRefresh;
export { default$ as default };

// TrackRequest.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequests } from "./TrackRequests.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackRequest = rt.Handle<"TrackRequest">;
export declare function name(self: TrackRequest): string;
export declare function consume(self: TrackRequest): TrackConsumer;
export declare function requests(self: TrackRequest): TrackRequests;
export declare function demand(self: TrackRequest): TrackDemand;
export declare function accept(self: TrackRequest, info: TrackInfo | undefined): TrackProducer;
export declare function reject(self: TrackRequest, err: Error_): void;
export declare function subscription(self: TrackRequest): TrackSubscription | undefined;
export declare function subscriptionChanged(self: TrackRequest): rt.Call<TrackSubscription | undefined>;

// TrackRequests.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackFill } from "./TrackFill.ts";
export type TrackRequests = rt.Handle<"TrackRequests">;
export declare function clone(self: TrackRequests): TrackRequests;
export declare function name(self: TrackRequests): string;
export declare function requestedFill(self: TrackRequests): rt.Call<rt.Result<TrackFill, Error_>>;
export declare function demand(self: TrackRequests): TrackDemand;

// TrackSubscriber.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { StatsBill } from "./StatsBill.ts";
import type { TrackControl } from "./TrackControl.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackOrdered } from "./TrackOrdered.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackSubscriber = rt.Handle<"TrackSubscriber">;
export declare function bill(self: TrackSubscriber): StatsBill;
export declare function info(self: TrackSubscriber): TrackInfo;
export declare function name(self: TrackSubscriber): string;
export declare function broadcast(self: TrackSubscriber): BroadcastInfo;
export declare function control(self: TrackSubscriber): TrackControl;
export declare function recvGroup(self: TrackSubscriber): rt.Call<rt.Result<GroupConsumer | undefined, Error_>>;
export declare function ordered(self: TrackSubscriber): TrackOrdered;
export declare function isClone(self: TrackSubscriber, other: TrackSubscriber): boolean;
export declare function start(self: TrackSubscriber): rt.Call<rt.u64 | undefined>;
export declare function floor(self: TrackSubscriber): rt.u64 | undefined;
export declare function nextInRange(self: TrackSubscriber, from: rt.u64): rt.u64 | undefined;
export declare function isSkipped(self: TrackSubscriber, groups: {
    start: rt.u64;
    end: rt.u64;
}): boolean;
export declare function finished(self: TrackSubscriber): rt.Call<rt.Result<TrackPosition, Error_>>;
export declare function withRange(self: TrackSubscriber, groups: {
    start: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
    end: {
        readonly tag: "Included";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Excluded";
        readonly _0: rt.u64;
    } | {
        readonly tag: "Unbounded";
    };
}): TrackSubscriber;
export declare function subscription(self: TrackSubscriber): TrackSubscription;
export declare function update(self: TrackSubscriber, subscription: TrackSubscription): rt.Result<undefined, Error_>;
export declare function largest(self: TrackSubscriber): rt.Call<TrackPosition | undefined>;
export declare function refresh(self: TrackSubscriber): rt.Call<rt.Result<undefined, Error_>>;

// TrackSubscription.ts
import * as rt from "./rt.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackOrder } from "./TrackOrder.ts";
import type { TrackRange } from "./TrackRange.ts";
export type TrackSubscription = rt.Handle<"TrackSubscription">;
export declare function debug(self: TrackSubscription): string;
export declare function clone(self: TrackSubscription): TrackSubscription;
export declare function eq(a: TrackSubscription, b: TrackSubscription): boolean;
export declare function default$(): TrackSubscription;
export declare function priorityGet(self: TrackSubscription): number;
export declare function prioritySet(self: TrackSubscription, value: number): void;
export declare function delayGet(self: TrackSubscription): TrackDelay;
export declare function delaySet(self: TrackSubscription, value: TrackDelay): void;
export declare function rangesGet(self: TrackSubscription): TrackRange[];
export declare function rangesSet(self: TrackSubscription, value: TrackRange[]): void;
export declare function orderGet(self: TrackSubscription): TrackOrder;
export declare function orderSet(self: TrackSubscription, value: TrackOrder): void;
export declare function withPriority(self: TrackSubscription, priority: number): TrackSubscription;
export declare function withDelay(self: TrackSubscription, delay: TrackDelay): TrackSubscription;
export declare function withRanges(self: TrackSubscription, ranges: TrackRange[]): TrackSubscription;
export declare function withOrder(self: TrackSubscription, order: TrackOrder): TrackSubscription;
export { default$ as default };

// TransportAddress.ts
export type TransportAddress = "Uri" | "Setup";
export declare function debug(self: TransportAddress): string;

// TransportCloseKind.ts
export type TransportCloseKind = "Transport" | "IdleTimeout" | "Closed" | "Evicted";
export declare function debug(self: TransportCloseKind): string;
export declare function default$(): TransportCloseKind;
export { default$ as default };

// TransportLimits.ts
import * as rt from "./rt.ts";
export type TransportLimits = rt.Opaque<"TransportLimits">;
export declare function debug(self: TransportLimits): string;
export declare function eq(a: TransportLimits, b: TransportLimits): boolean;
export declare function default$(): TransportLimits;
export declare function receiveWindowGet(self: TransportLimits): rt.u64 | undefined;
export declare function receiveWindowSet(self: TransportLimits, value: rt.u64 | undefined): TransportLimits;
export { default$ as default };

// TransportLimitsError.ts
export type TransportLimitsError = {
    tag: "Unsupported";
} | {
    tag: "Refused";
    _0: string;
};
export declare function toString(self: TransportLimitsError): string;
export declare function debug(self: TransportLimitsError): string;
export declare function eq(a: TransportLimitsError, b: TransportLimitsError): boolean;

// TransportMemoryCloser.ts
import * as rt from "./rt.ts";
export type TransportMemoryCloser = rt.Handle<"TransportMemoryCloser">;
export declare function clone(self: TransportMemoryCloser): TransportMemoryCloser;
export declare function close(self: TransportMemoryCloser, code: number, reason: string): void;
export declare function isClosed(self: TransportMemoryCloser): boolean;

// TransportMemoryConfig.ts
import * as rt from "./rt.ts";
export type TransportMemoryConfig = rt.Handle<"TransportMemoryConfig">;
export declare function debug(self: TransportMemoryConfig): string;
export declare function clone(self: TransportMemoryConfig): TransportMemoryConfig;
export declare function default$(): TransportMemoryConfig;
export declare function streamBufferGet(self: TransportMemoryConfig): rt.u64;
export declare function streamBufferSet(self: TransportMemoryConfig, value: rt.u64): void;
export declare function protocolGet(self: TransportMemoryConfig): string | undefined;
export declare function protocolSet(self: TransportMemoryConfig, value: string | undefined): void;
export declare function maxConcurrentUniStreamsGet(self: TransportMemoryConfig): rt.u64 | undefined;
export declare function maxConcurrentUniStreamsSet(self: TransportMemoryConfig, value: rt.u64 | undefined): void;
export declare function maxConcurrentBiStreamsGet(self: TransportMemoryConfig): rt.u64 | undefined;
export declare function maxConcurrentBiStreamsSet(self: TransportMemoryConfig, value: rt.u64 | undefined): void;
export declare function latencyGet(self: TransportMemoryConfig): rt.Duration;
export declare function latencySet(self: TransportMemoryConfig, value: rt.Duration): void;
export { default$ as default };

// TransportMemoryError.ts
export type TransportMemoryError = {
    tag: "Session";
    code: number;
    reason: string;
} | {
    tag: "Reset";
    _0: number;
} | {
    tag: "Stopped";
    _0: number;
} | {
    tag: "Closed";
};
export declare function toString(self: TransportMemoryError): string;
export declare function debug(self: TransportMemoryError): string;

// TransportMemorySession.ts
import * as rt from "./rt.ts";
import type { TransportMemoryCloser } from "./TransportMemoryCloser.ts";
export type TransportMemorySession = rt.Handle<"TransportMemorySession">;
export declare function clone(self: TransportMemorySession): TransportMemorySession;
export declare function closer(self: TransportMemorySession): TransportMemoryCloser;

// TransportMetrics.ts
export interface TransportMetrics {
    close?(): void;
}

// TransportPollAcked.ts
export type TransportPollAcked = "Delivered" | "Unsupported" | "Sent";
export declare function debug(self: TransportPollAcked): string;

// TransportProbe.ts
import * as rt from "./rt.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
export type TransportProbe = rt.Handle<"TransportProbe">;
export declare function debug(self: TransportProbe): string;
export declare function clone(self: TransportProbe): TransportProbe;
export declare function default$(): TransportProbe;
export declare function new$(): TransportProbe;
export declare function target(self: TransportProbe): BandwidthRate;
export declare function setTarget(self: TransportProbe, rate: BandwidthRate): void;
export { default$ as default, new$ as new };

// TransportReceiveWindow.ts
import * as rt from "./rt.ts";
export type TransportReceiveWindow = rt.Opaque<"TransportReceiveWindow">;
export declare function debug(self: TransportReceiveWindow): string;
export declare function eq(a: TransportReceiveWindow, b: TransportReceiveWindow): boolean;
export declare function connectionGet(self: TransportReceiveWindow): rt.u64 | undefined;
export declare function connectionSet(self: TransportReceiveWindow, value: rt.u64 | undefined): TransportReceiveWindow;
export declare function streamGet(self: TransportReceiveWindow): rt.u64;
export declare function streamSet(self: TransportReceiveWindow, value: rt.u64): TransportReceiveWindow;
export declare function new$(connection: rt.u64 | undefined, stream: rt.u64): TransportReceiveWindow;
export { new$ as new };

// TransportWire.ts
import * as rt from "./rt.ts";
export type TransportWire = rt.Handle<"TransportWire">;
export declare function debug(self: TransportWire): string;
export declare function clone(self: TransportWire): TransportWire;
export declare function default$(): TransportWire;
export declare function new$(): TransportWire;
export declare function sentBytes(self: TransportWire): rt.u64;
export declare function receivedBytes(self: TransportWire): rt.u64;
export declare function setSentBytes(self: TransportWire, total: rt.u64): void;
export declare function setReceivedBytes(self: TransportWire, total: rt.u64): void;
export { default$ as default, new$ as new };

// Url.ts
import * as rt from "./rt.ts";
export type Url = rt.Handle<"Url">;
export declare function toString(self: Url): string;
export declare function debug(self: Url): string;
export declare function clone(self: Url): Url;
export declare function eq(a: Url, b: Url): boolean;
export declare function hash(self: Url): rt.u64;
export declare function partialCmp(a: Url, b: Url): number | undefined;
export declare function cmp(a: Url, b: Url): number;
export declare function parse(input: string): rt.Result<Url, string>;

// Version.ts
import * as rt from "./rt.ts";
export type Version = rt.Handle<"Version">;
export declare function toString(self: Version): string;
export declare function debug(self: Version): string;
export declare function clone(self: Version): Version;
export declare function eq(a: Version, b: Version): boolean;
export declare function hash(self: Version): rt.u64;
export declare function ALL(): Version[];
export declare function hasRequestPath(self: Version): boolean;
export declare function asStr(self: Version): string;
export declare function alpn(self: Version): string;
export declare function isLite(self: Version): boolean;

// Versions.ts
import * as rt from "./rt.ts";
import type { Version } from "./Version.ts";
export type Versions = rt.Handle<"Versions">;
export declare function debug(self: Versions): string;
export declare function clone(self: Versions): Versions;
export declare function default$(): Versions;
export declare function all(): Versions;
export declare function alpns(self: Versions): string[];
export declare function filter(self: Versions, other: Versions): Versions | undefined;
export declare function select(self: Versions, version: Version): Version | undefined;
export declare function contains(self: Versions, version: Version): boolean;
export declare function iter(self: Versions): Version[];
export { default$ as default };

// Waiter.ts
import * as rt from "./rt.ts";
import type { WaiterList } from "./WaiterList.ts";
export type Waiter = rt.Handle<"Waiter">;
export declare function clone(self: Waiter): Waiter;
export declare function new$(waker: () => void): Waiter;
export declare function noop(): Waiter;
export declare function register(self: Waiter, list: WaiterList): void;
export declare function waker(self: Waiter): rt.Waker;
export declare function context(self: Waiter): rt.Waker;
export declare function batch(self: Waiter, poll: () => string): string;
export declare function pollFuture(future: Promise<unknown>): rt.Call<void>;
export { new$ as new };

// WaiterList.ts
import * as rt from "./rt.ts";
import type { Waiter } from "./Waiter.ts";
export type WaiterList = rt.Handle<"WaiterList">;
export declare function debug(self: WaiterList): string;
export declare function default$(): WaiterList;
export declare function new$(): WaiterList;
export declare function register(self: WaiterList, waiter: Waiter): void;
export declare function take(self: WaiterList): WaiterList;
export declare function isEmpty(self: WaiterList): boolean;
export declare function wake(self: WaiterList): void;
export { default$ as default, new$ as new };

// index.ts
import * as rt from "./rt.ts";
import type { ClockElapsed } from "./ClockElapsed.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockInterval } from "./ClockInterval.ts";
import type { ClockRng } from "./ClockRng.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { DecodeError } from "./DecodeError.ts";
import type { Driver } from "./Driver.ts";
import type { EncodeError } from "./EncodeError.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogPriorities } from "./HangCatalogPriorities.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxCodecAnnexbError } from "./MuxCodecAnnexbError.ts";
import type { MuxContainerMp4Finish } from "./MuxContainerMp4Finish.ts";
import type { MuxError } from "./MuxError.ts";
import type { OriginDriver } from "./OriginDriver.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Path } from "./Path.ts";
import type { Timescale } from "./Timescale.ts";
import type { TokioTlsError } from "./TokioTlsError.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TransportMemoryConfig } from "./TransportMemoryConfig.ts";
import type { TransportMemorySession } from "./TransportMemorySession.ts";
import type { Waiter } from "./Waiter.ts";
export declare function load(): Promise<void>;
export declare const PATH_MAX_PARTS: rt.u64;
export declare const PATH_MAX_BYTES: rt.u64;
export declare const PATTERN_MAX_SEGMENTS: rt.u64;
export declare const PATTERN_MAX_INTERSECTIONS: rt.u64;
export declare const SETUP_TOKEN_OUT_OF_BAND: rt.u64;
export declare const TRANSPORT_MIN_RECEIVE_WINDOW: rt.u64;
export declare const TRANSPORT_SETUP_RECEIVE_WINDOW: rt.u64;
export declare const HANG_CATALOG_MAX_SAFE_INTEGER: rt.u64;
export declare function IMPLEMENTATION(): string;
export declare function ALPNS(): string[];
export declare function escape(content: string): string;
export declare function unescape(segment: string): string;
export declare function clockNow(): ClockInstant;
export declare function clockWall(): ClockWall;
export declare function clockOrigin(): [ClockInstant, ClockWall];
export declare function clockRng(): ClockRng;
export declare function clockSeed(): rt.u64;
export declare function clockHostTime(): void;
export declare function clockDrive(poll: (a0: ClockInstant, a1: Waiter) => rt.Result<ClockInstant | undefined, string>): rt.Call<string>;
export declare function clockInterval(period: rt.Duration): ClockInterval;
export declare function clockIntervalAt(start: ClockInstant, period: rt.Duration): ClockInterval;
export declare function clockSleep(duration: rt.Duration): rt.Call<void>;
export declare function clockSleepUntil(at: ClockInstant): rt.Call<void>;
export declare function clockTimeout(duration: rt.Duration, future: Promise<unknown>): rt.Call<rt.Result<undefined, ClockElapsed>>;
export declare function clockTimeoutAt(deadline: ClockInstant, future: Promise<unknown>): rt.Call<rt.Result<undefined, ClockElapsed>>;
export declare function SERVER_DEFAULT_TIMEOUT(): rt.Duration;
export declare function timeRunDriver(driver: Driver): rt.Call<Error_>;
export declare function timeRunOriginDriver(driver: OriginDriver): rt.Call<Error_>;
export declare function varintDecodeQuic(r: rt.Place<Uint8Array>): rt.Result<rt.u64, DecodeError>;
export declare function varintEncodeQuic(value: rt.u64, w: rt.Place<Uint8Array>): rt.Result<undefined, EncodeError>;
export declare function CACHE_DEFAULT_EXPIRY(): rt.Duration;
export declare function HANG_ECHO_SUFFIX(): string;
export declare function HANG_ECHO_TRACK(): string;
export declare function hangEchoIsEcho(path: Path): boolean;
export declare function HANG_CATALOG_PRIORITY(): HangCatalogPriorities;
export declare function HANG_CONTAINER_MAX_AGE(): rt.Duration;
export declare function HANG_CONTAINER_TIMESCALE(): Timescale;
export declare function hangContainerTrackInfo(priority: number): TrackInfo;
export declare function MUX_CONTAINER_DELAY_MAX(): rt.Duration;
export declare function MUX_RATE_RETRY(): rt.Duration;
export declare function tokioTlsParseFingerprint(value: string): rt.Result<Uint8Array, TokioTlsError>;
export declare function tokioOriginSpawn(): OriginProducer;
export declare function transportMemoryPair(config: TransportMemoryConfig): [TransportMemorySession, TransportMemorySession];
export declare function muxCodecAacConfig(init: Uint8Array | rt.Bytes): rt.Result<HangCatalogAudioConfig, MuxError>;
export declare function MUX_CODEC_ANNEXB_START_CODE(): rt.Bytes;
export declare function muxCodecAnnexbFromLengthPrefixed(payload: Uint8Array | rt.Bytes, lengthSize: rt.u64, prefix: Uint8Array | rt.Bytes | undefined): rt.Result<rt.Bytes, MuxCodecAnnexbError>;
export declare function muxCodecAnnexbBuildPrefix(nals: Uint8Array[]): rt.Bytes;
export declare function muxCodecAnnexbLengthPrefixedToAnnexb(data: Uint8Array | rt.Bytes, lengthSize: rt.u64, out: rt.Place<Uint8Array>): rt.Result<undefined, MuxCodecAnnexbError>;
export declare function muxCodecAnnexbAfterStartCode(b: Uint8Array | rt.Bytes): rt.Result<rt.u64 | undefined, MuxCodecAnnexbError>;
export declare function muxCodecAnnexbFindStartCode(b: Uint8Array | rt.Bytes): [rt.u64, rt.u64] | undefined;
export declare function muxCodecFlacConfig(init: Uint8Array | rt.Bytes): rt.Result<HangCatalogAudioConfig, MuxError>;
export declare function muxCodecH264Config(buf: Uint8Array | rt.Bytes): rt.Result<HangCatalogVideoConfig, MuxError>;
export declare function muxCodecH265Config(buf: Uint8Array | rt.Bytes): rt.Result<HangCatalogVideoConfig, MuxError>;
export declare function muxCodecMp3Config(init: Uint8Array | rt.Bytes): rt.Result<HangCatalogAudioConfig, MuxError>;
export declare function MUX_CODEC_OPUS_WARMUP(): rt.Duration;
export declare function muxCodecOpusPacketSamples(packet: Uint8Array | rt.Bytes): number | undefined;
export declare function muxCodecOpusConfig(init: Uint8Array | rt.Bytes): rt.Result<HangCatalogAudioConfig, MuxError>;
export declare function muxContainerMp4Repair(file: rt.Reader): rt.Result<MuxContainerMp4Finish | undefined, MuxError>;

// rt.ts
declare const brand: unique symbol;
export type U64Limbs = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "u64";
};
export type I64Limbs = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "i64";
};
export type U128Limbs = {
    readonly hi: number;
    readonly w2: number;
    readonly w1: number;
    readonly lo: number;
    readonly [brand]: "u128";
};
type Num<T extends string> = number & {
    readonly [brand]?: T;
};
export type u64 = Num<"u64"> | U64Limbs;
export type i64 = Num<"i64"> | I64Limbs;
export type u128 = Num<"u128"> | U128Limbs;
export type F32Nan = {
    readonly bits: number;
    readonly [brand]: "f32";
};
export type F64Nan = {
    readonly hi: number;
    readonly lo: number;
    readonly [brand]: "f64";
};
export type f32 = Num<"f32"> | F32Nan;
export type f64 = Num<"f64"> | F64Nan;
export type Result<T, E> = {
    readonly ok: true;
    readonly value: T;
} | {
    readonly ok: false;
    readonly error: E;
};
export declare function Ok<T>(value: T): Result<T, never>;
export declare function Err<E>(error: E): Result<never, E>;
export declare const None: unique symbol;
export type None = typeof None;
export type Place<T> = {
    v: T;
};
export interface Opaque<T extends string = string> {
    readonly [brand]: T;
}
export declare class Opaque<T extends string = string> {
    #private;
    private constructor();
}
export declare function opaqueMade(bytes: Uint8Array): Opaque;
export type Duration = {
    readonly secs: u64;
    readonly nanos: number;
};
export type PathBuf = {
    readonly bytes: Uint8Array;
};
export declare function pathFrom(text: string): PathBuf;
export declare function pathText(path: PathBuf): string;
export type Ipv4Addr = {
    readonly octets: Uint8Array;
};
export type Ipv6Addr = {
    readonly octets: Uint8Array;
};
export type IpAddr = {
    readonly tag: "V4";
    readonly _0: Ipv4Addr;
} | {
    readonly tag: "V6";
    readonly _0: Ipv6Addr;
};
export type SocketAddrV4 = {
    readonly ip: Ipv4Addr;
    readonly port: number;
};
export type SocketAddrV6 = {
    readonly ip: Ipv6Addr;
    readonly port: number;
    readonly flowinfo: number;
    readonly scopeId: number;
};
export type SocketAddr = {
    readonly tag: "V4";
    readonly _0: SocketAddrV4;
} | {
    readonly tag: "V6";
    readonly _0: SocketAddrV6;
};
export type SeekFrom = "Start" | "Current" | "End";
export interface Reader {
    read(buffer: Uint8Array): number;
    seek(offset: u64 | i64, from: SeekFrom): u64;
}
export type Call<T> = Promise<T> & {
    cancel(): void;
};
export declare class Cancelled extends Error {
    constructor();
}
export type Waker = {
    wake(): void;
    [Symbol.dispose](): void;
};
export declare function int(v: number, min: number, max: number, what: string): number;
export declare function nonZero<T extends number | u64 | i64>(v: T): T;
export declare function f32Number(v: f32): number;
export declare function f64Number(v: f64): number;
export declare function char(v: number): number;
export declare function u64Of(hi: number, lo: number): u64;
export declare function i64Of(hi: number, lo: number): i64;
export declare function u64Halves(v: u64): [number, number];
export declare function i64Halves(v: i64): [number, number];
export declare function u64Big(v: u64): bigint;
export declare function i64Big(v: i64): bigint;
export declare function u64OfBig(v: bigint): u64;
export declare function i64OfBig(v: bigint): i64;
export declare function u64ToBig(v: u64): bigint;
export declare function i64ToBig(v: i64): bigint;
export declare function u128ToBig(v: u128): bigint;
export declare function u64FromBig(v: bigint): u64;
export declare function i64FromBig(v: bigint): i64;
export declare function u128FromBig(v: bigint): u128;
export type Stream<T> = AsyncIterableIterator<T>;
export declare function values<T>(next: () => Call<unknown>, none: unknown, fallible: boolean): Stream<T>;
export declare let n: Record<string, (...args: any[]) => any>;
export declare function on(targets: readonly number[], what: string): void;
export declare function encoded(text: string): Uint8Array;
export interface Handle<T extends string = string> {
    readonly [brand]: T;
}
export declare class Handle<T extends string = string> {
    private constructor();
    [Symbol.dispose](): void;
}
export declare class Bytes {
    #private;
    private constructor();
    get length(): number;
    at(index: number): number | undefined;
    slice(start?: number, end?: number): Uint8Array<ArrayBuffer>;
    [Symbol.iterator](): IterableIterator<number>;
    [Symbol.dispose](): void;
}
export type Module = {
    readonly global: string;
    readonly module: string;
    readonly file: string;
    readonly contract: bigint;
    readonly targets: readonly number[];
};
export declare function load(m: Module): Promise<void>;
export {};
