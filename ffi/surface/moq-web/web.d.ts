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

// AnnounceEvent.ts
import * as rt from "./rt.ts";
export type AnnounceEvent = rt.Handle<"AnnounceEvent">;
export declare function debug(self: AnnounceEvent): string;
export declare function clone(self: AnnounceEvent): AnnounceEvent;

// AuthAnswer.ts
export type AuthAnswer = "Granted" | "Requests";
export declare function debug(self: AuthAnswer): string;

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
import type { AuthAnswer } from "./AuthAnswer.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { AuthWatch } from "./AuthWatch.ts";
import type { Error_ } from "./Error_.ts";
export type AuthHandle = rt.Handle<"AuthHandle">;
export declare function clone(self: AuthHandle): AuthHandle;
export declare function default$(): AuthHandle;
export declare function new$(): AuthHandle;
export declare function grant(self: AuthHandle): AuthWatch;
export declare function add(self: AuthHandle, token: Uint8Array | rt.Bytes): rt.Call<rt.Result<[AuthCredential, AuthAnswer], Error_>>;
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
import type { AuthRequestKind } from "./AuthRequestKind.ts";
import type { Error_ } from "./Error_.ts";
import type { Path } from "./Path.ts";
import type { SessionError } from "./SessionError.ts";
export type AuthRequest = rt.Handle<"AuthRequest">;
export declare function token(self: AuthRequest): rt.Bytes;
export declare function kind(self: AuthRequest): AuthRequestKind | undefined;
export declare function path(self: AuthRequest): Path | undefined;
export declare function closed(self: AuthRequest): rt.Call<Error_>;
export declare function accept(self: AuthRequest, grant: AuthGrant): AuthIssued;
export declare function reject(self: AuthRequest, code: SessionError, reason: string): void;

// AuthRequestKind.ts
export type AuthRequestKind = "Subscribe" | "PublishNamespace" | "Discover" | "Publish";
export declare function debug(self: AuthRequestKind): string;

// AuthRequests.ts
import * as rt from "./rt.ts";
import type { AuthRequest } from "./AuthRequest.ts";
export type AuthRequests = rt.Handle<"AuthRequests">;
export declare function next(self: AuthRequests): rt.Call<AuthRequest | undefined>;

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
export declare function default$(): BandwidthRate;
export declare function ZERO(): BandwidthRate;
export declare function fromBps(bps: rt.u64): BandwidthRate;
export declare function fromKbps(kbps: rt.u64): BandwidthRate;
export declare function fromMbps(mbps: rt.u64): BandwidthRate;
export declare function asBps(self: BandwidthRate): rt.u64;
export declare function scaled(self: BandwidthRate, factor: number): BandwidthRate;
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
export declare function cacheDurationGet(self: BroadcastInfo): rt.Duration;
export declare function cacheDurationSet(self: BroadcastInfo, value: rt.Duration): void;
export declare function pathGet(self: BroadcastInfo): Path;
export declare function pathSet(self: BroadcastInfo, value: Path): void;
export declare function instanceGet(self: BroadcastInfo): Instance | undefined;
export declare function instanceSet(self: BroadcastInfo, value: Instance | undefined): void;
export declare function new$(): BroadcastInfo;
export declare function produce(self: BroadcastInfo): BroadcastProducer;
export { default$ as default, new$ as new };

// BroadcastLister.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { TrackInfo } from "./TrackInfo.ts";
export type BroadcastLister = rt.Handle<"BroadcastLister">;
export declare function pollWanted(self: BroadcastLister): rt.Call<rt.Result<undefined, Error_>>;
export declare function pollUnwanted(self: BroadcastLister): rt.Call<rt.Result<undefined, Error_>>;
export declare function add(self: BroadcastLister, name: string, info: TrackInfo): rt.Result<undefined, Error_>;
export declare function remove(self: BroadcastLister, name: string): rt.Result<undefined, Error_>;
export declare function finish(self: BroadcastLister, name: string): rt.Result<undefined, Error_>;
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
import type { Hop } from "./Hop.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginCost } from "./OriginCost.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Session } from "./Session.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupExtensions } from "./SetupExtensions.ts";
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
export declare function withOrigin(self: Client, origin: OriginProducer): Client;
export declare function withLimits(self: Client, limits: SessionLimits): Client;
export declare function withAllocator(self: Client, allocator: BandwidthAllocator): Client;
export declare function withViewers(self: Client, viewers: BandwidthViewers): Client;
export declare function withVersions(self: Client, versions: Versions): Client;
export declare function withUrl(self: Client, url: Url): Client;
export declare function withToken(self: Client, token: SetupToken): Client;
export declare function withAuth(self: Client, auth: AuthHandle): Client;
export declare function withExtensions(self: Client, extensions: SetupExtensions): Client;
export declare function withImplementation(self: Client, implementation: string): Client;
export declare function withCost(self: Client, cost: OriginCost): Client;
export declare function withPeerHop(self: Client, hop: Hop): Client;
export declare function connectSession(self: Client, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<[Session, Driver], Error_>>;
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
export declare function ZERO(): ClockInstant;
export declare function now(): ClockInstant;
export declare function elapsed(self: ClockInstant): rt.Duration;
export declare function durationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration;
export declare function checkedDurationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration | undefined;
export declare function saturatingDurationSince(self: ClockInstant, earlier: ClockInstant): rt.Duration;
export declare function checkedAdd(self: ClockInstant, duration: rt.Duration): ClockInstant | undefined;
export declare function checkedSub(self: ClockInstant, duration: rt.Duration): ClockInstant | undefined;
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
export declare function UNIX_EPOCH(): ClockWall;
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

// Datagram.ts
import * as rt from "./rt.ts";
import type { Timestamp } from "./Timestamp.ts";
export type Datagram = {
    sequence: rt.u64;
    timestamp: Timestamp | undefined;
    payload: Uint8Array;
};
export declare function debug(self: Datagram): string;
export declare function eq(a: Datagram, b: Datagram): boolean;

// DatagramConsumer.ts
import * as rt from "./rt.ts";
import type { Datagram } from "./Datagram.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { Error_ } from "./Error_.ts";
export type DatagramConsumer = rt.Handle<"DatagramConsumer">;
export declare function clone(self: DatagramConsumer): DatagramConsumer;
export declare function name(self: DatagramConsumer): string;
export declare function info(self: DatagramConsumer): rt.Call<rt.Result<DatagramInfo, Error_>>;
export declare function recv(self: DatagramConsumer): rt.Call<rt.Result<Datagram | undefined, Error_>>;
export declare function priority(self: DatagramConsumer): number;
export declare function setPriority(self: DatagramConsumer, priority: number): void;
export declare function isClone(self: DatagramConsumer, other: DatagramConsumer): boolean;

// DatagramInfo.ts
import * as rt from "./rt.ts";
import type { DatagramProducer } from "./DatagramProducer.ts";
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
export declare function rescale(self: DatagramInfo, timestamp: Timestamp | undefined): rt.Result<Timestamp | undefined, Error_>;
export declare function produce(self: DatagramInfo, name: string): DatagramProducer;
export { default$ as default };

// DatagramProducer.ts
import * as rt from "./rt.ts";
import type { Datagram } from "./Datagram.ts";
import type { DatagramConsumer } from "./DatagramConsumer.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { Timestamp } from "./Timestamp.ts";
export type DatagramProducer = rt.Handle<"DatagramProducer">;
export declare function clone(self: DatagramProducer): DatagramProducer;
export declare function name(self: DatagramProducer): string;
export declare function info(self: DatagramProducer): DatagramInfo;
export declare function append(self: DatagramProducer, timestamp: Timestamp | undefined, payload: Uint8Array | rt.Bytes): rt.Result<rt.u64, Error_>;
export declare function insert(self: DatagramProducer, datagram: Datagram): rt.Result<undefined, Error_>;
export declare function consume(self: DatagramProducer): DatagramConsumer;
export declare function finish(self: DatagramProducer): void;
export declare function abort(self: DatagramProducer, err: Error_): void;
export declare function priority(self: DatagramProducer): number | undefined;
export declare function pollPriority(self: DatagramProducer, last: number | undefined): rt.Call<number | undefined>;
export declare function isClone(self: DatagramProducer, other: DatagramProducer): boolean;

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

// FrameBuffer.ts
import * as rt from "./rt.ts";
import type { FrameFrame } from "./FrameFrame.ts";
export type FrameBuffer = rt.Handle<"FrameBuffer">;
export declare function debug(self: FrameBuffer): string;
export declare function default$(): FrameBuffer;
export declare function new$(): FrameBuffer;
export declare function capacity(self: FrameBuffer): number;
export declare function len(self: FrameBuffer): number;
export declare function isEmpty(self: FrameBuffer): boolean;
export declare function filled(self: FrameBuffer): FrameFrame[];
export declare function push(self: FrameBuffer, frame: FrameFrame): rt.Result<undefined, FrameFrame>;
export declare function clear(self: FrameBuffer): void;
export { default$ as default, new$ as new };

// FrameConsumer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameConsumer = rt.Handle<"FrameConsumer">;
export declare function clone(self: FrameConsumer): FrameConsumer;
export declare function readChunk(self: FrameConsumer): rt.Call<rt.Result<rt.Bytes | undefined, Error_>>;
export declare function readAll(self: FrameConsumer): rt.Call<rt.Result<rt.Bytes, Error_>>;
export declare function sizeGet(self: FrameConsumer): rt.u64;
export declare function timestampGet(self: FrameConsumer): Timestamp | undefined;

// FrameFrame.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameFrame = rt.Handle<"FrameFrame">;
export declare function debug(self: FrameFrame): string;
export declare function clone(self: FrameFrame): FrameFrame;
export declare function timestampGet(self: FrameFrame): Timestamp | undefined;
export declare function timestampSet(self: FrameFrame, value: Timestamp | undefined): void;
export declare function payloadGet(self: FrameFrame): rt.Bytes;
export declare function payloadSet(self: FrameFrame, value: Uint8Array | rt.Bytes): void;
export declare function new$(timestamp: Timestamp | undefined, payload: Uint8Array | rt.Bytes): FrameFrame;
export declare function arrived(self: FrameFrame): ClockInstant;
export { new$ as new };

// FrameInfo.ts
import * as rt from "./rt.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameInfo = {
    size: rt.u64;
    timestamp: Timestamp | undefined;
};
export declare function debug(self: FrameInfo): string;

// FrameProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { Timestamp } from "./Timestamp.ts";
export type FrameProducer = rt.Handle<"FrameProducer">;
export declare function group(self: FrameProducer): GroupProducer;
export declare function remaining(self: FrameProducer): number;
export declare function write(self: FrameProducer, chunk: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function finish(self: FrameProducer): rt.Result<GroupProducer, Error_>;
export declare function abort(self: FrameProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function sizeGet(self: FrameProducer): rt.u64;
export declare function timestampGet(self: FrameProducer): Timestamp | undefined;

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
export declare function withTimeout(self: GoawayGoaway, timeout: rt.Duration): GoawayGoaway;
export declare function uri(self: GoawayGoaway): string;
export declare function timeout(self: GoawayGoaway): rt.Duration | undefined;
export { default$ as default, new$ as new };

// GoawayProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GoawayGoaway } from "./GoawayGoaway.ts";
export type GoawayProducer = rt.Handle<"GoawayProducer">;
export declare function send(self: GoawayProducer, goaway: GoawayGoaway): rt.Result<undefined, Error_>;

// GroupConsumer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Error_ } from "./Error_.ts";
import type { FrameBuffer } from "./FrameBuffer.ts";
import type { FrameConsumer } from "./FrameConsumer.ts";
import type { FrameFrame } from "./FrameFrame.ts";
import type { Timescale } from "./Timescale.ts";
export type GroupConsumer = rt.Handle<"GroupConsumer">;
export declare function clone(self: GroupConsumer): GroupConsumer;
export declare function keepAlive(self: GroupConsumer): void;
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
export declare function frameCount(self: GroupConsumer): number;
export declare function nextFrame(self: GroupConsumer): rt.Call<rt.Result<FrameConsumer | undefined, Error_>>;
export declare function readFrame(self: GroupConsumer): rt.Call<rt.Result<FrameFrame | undefined, Error_>>;
export declare function pollReadFrames8(self: GroupConsumer, out: FrameBuffer): rt.Call<rt.Result<number, Error_>>;
export declare function readFrames8(self: GroupConsumer, out: FrameBuffer): rt.Call<rt.Result<FrameFrame[], Error_>>;
export declare function finished(self: GroupConsumer): rt.Call<rt.Result<rt.u64, Error_>>;
export declare function sequenceGet(self: GroupConsumer): rt.u64;

// GroupDemand.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
export type GroupDemand = rt.Handle<"GroupDemand">;
export declare function clone(self: GroupDemand): GroupDemand;
export declare function sequence(self: GroupDemand): rt.u64;
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

// GroupInfo.ts
import * as rt from "./rt.ts";
export type GroupInfo = {
    sequence: rt.u64;
};
export declare function debug(self: GroupInfo): string;
export declare function eq(a: GroupInfo, b: GroupInfo): boolean;
export declare function hash(self: GroupInfo): rt.u64;

// GroupProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { FrameBuffer } from "./FrameBuffer.ts";
import type { FrameInfo } from "./FrameInfo.ts";
import type { FrameProducer } from "./FrameProducer.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { GroupDemand } from "./GroupDemand.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type GroupProducer = rt.Handle<"GroupProducer">;
export declare function timescale(self: GroupProducer): Timescale | undefined;
export declare function startAt(self: GroupProducer, index: rt.u64): rt.Result<undefined, Error_>;
export declare function writeFrame(self: GroupProducer, timestamp: Timestamp | undefined, data: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function writeFrames8(self: GroupProducer, frames: FrameBuffer): rt.Result<undefined, Error_>;
export declare function createFrame(self: GroupProducer, frame: FrameInfo): rt.Result<FrameProducer, Error_>;
export declare function finish(self: GroupProducer): rt.Result<undefined, Error_>;
export declare function abort(self: GroupProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function closed(self: GroupProducer): rt.Call<rt.Result<undefined, Error_>>;
export declare function ready(self: GroupProducer, frame: FrameInfo): rt.Call<void>;
export declare function frameCount(self: GroupProducer): number;
export declare function consume(self: GroupProducer): GroupConsumer;
export declare function demand(self: GroupProducer): GroupDemand;
export declare function sequenceGet(self: GroupProducer): rt.u64;

// HangCatalog.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogConsumer } from "./HangCatalogConsumer.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
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
export declare function mediaGet(self: HangCatalog): HangCatalogMedia;
export declare function mediaSet(self: HangCatalog, value: HangCatalogMedia): void;
export declare function textGet(self: HangCatalog): HangCatalogText;
export declare function textSet(self: HangCatalog, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalog): HangCatalogJson;
export declare function jsonSet(self: HangCatalog, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalog): HangCatalogBinary;
export declare function binarySet(self: HangCatalog, value: HangCatalogBinary): void;
export declare function m2tsGet(self: HangCatalog): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: HangCatalog, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: HangCatalog): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalog, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalog): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalog, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: HangCatalog): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: HangCatalog, value: ([string, HangCatalogContribution])[]): void;
export declare function DEFAULT_NAME(): string;
export declare function defaultTrackInfo(): TrackInfo;
export declare function defaultSubscription(): TrackSubscription;
export declare function checkRenditions(self: HangCatalog): rt.Result<undefined, HangError>;
export declare function media(self: HangCatalog): HangCatalog;
export declare function subscribe(broadcast: BroadcastConsumer): rt.Call<rt.Result<HangCatalogConsumer, HangError>>;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalog, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalog, HangError>;
export declare function toJson(self: HangCatalog): rt.Result<string, HangError>;
export declare function toJsonPretty(self: HangCatalog): rt.Result<string, HangError>;
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
export type HangCatalogChromaticity = {
    x: number;
    y: number;
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
import type { HangCatalogContentLight } from "./HangCatalogContentLight.ts";
import type { HangCatalogMastering } from "./HangCatalogMastering.ts";
import type { HangCatalogRange } from "./HangCatalogRange.ts";
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
export declare function rangeGet(self: HangCatalogColor): HangCatalogRange | undefined;
export declare function rangeSet(self: HangCatalogColor, value: HangCatalogRange | undefined): void;
export declare function masteringGet(self: HangCatalogColor): HangCatalogMastering | undefined;
export declare function masteringSet(self: HangCatalogColor, value: HangCatalogMastering | undefined): void;
export declare function contentLightGet(self: HangCatalogColor): HangCatalogContentLight | undefined;
export declare function contentLightSet(self: HangCatalogColor, value: HangCatalogContentLight | undefined): void;
export { default$ as default };

// HangCatalogConsumer.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangError } from "./HangError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type HangCatalogConsumer = rt.Handle<"HangCatalogConsumer">;
export declare function new$(track: TrackSubscriber): HangCatalogConsumer;
export declare function next(self: HangCatalogConsumer): rt.Call<rt.Result<HangCatalog | undefined, HangError>>;
export { new$ as new };

// HangCatalogConsumerExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangError } from "./HangError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type HangCatalogConsumerExt = rt.Handle<"HangCatalogConsumerExt">;
export declare function new$(track: TrackSubscriber): HangCatalogConsumerExt;
export declare function next(self: HangCatalogConsumerExt): rt.Call<rt.Result<HangCatalogExt | undefined, HangError>>;
export { new$ as new };

// HangCatalogConsumerExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { HangError } from "./HangError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type HangCatalogConsumerExtra = rt.Handle<"HangCatalogConsumerExtra">;
export declare function new$(track: TrackSubscriber): HangCatalogConsumerExtra;
export declare function next(self: HangCatalogConsumerExtra): rt.Call<rt.Result<HangCatalogExtra | undefined, HangError>>;
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

// HangCatalogContribution.ts
import * as rt from "./rt.ts";
import type { Instance } from "./Instance.ts";
import type { PathRelative } from "./PathRelative.ts";
export type HangCatalogContribution = rt.Handle<"HangCatalogContribution">;
export declare function debug(self: HangCatalogContribution): string;
export declare function clone(self: HangCatalogContribution): HangCatalogContribution;
export declare function eq(a: HangCatalogContribution, b: HangCatalogContribution): boolean;
export declare function broadcastGet(self: HangCatalogContribution): PathRelative;
export declare function broadcastSet(self: HangCatalogContribution, value: PathRelative): void;
export declare function instanceGet(self: HangCatalogContribution): Instance | undefined;
export declare function instanceSet(self: HangCatalogContribution, value: Instance | undefined): void;
export declare function new$(broadcast: PathRelative): HangCatalogContribution;
export { new$ as new };

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
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogConsumerExt } from "./HangCatalogConsumerExt.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
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
export declare function mediaGet(self: HangCatalogExt): HangCatalogMedia;
export declare function mediaSet(self: HangCatalogExt, value: HangCatalogMedia): void;
export declare function textGet(self: HangCatalogExt): HangCatalogText;
export declare function textSet(self: HangCatalogExt, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalogExt): HangCatalogJson;
export declare function jsonSet(self: HangCatalogExt, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalogExt): HangCatalogBinary;
export declare function binarySet(self: HangCatalogExt, value: HangCatalogBinary): void;
export declare function m2tsGet(self: HangCatalogExt): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: HangCatalogExt, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: HangCatalogExt): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalogExt, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalogExt): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalogExt, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: HangCatalogExt): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: HangCatalogExt, value: ([string, HangCatalogContribution])[]): void;
export declare function extGet(self: HangCatalogExt): MuxContainerTsExt;
export declare function extSet(self: HangCatalogExt, value: MuxContainerTsExt): void;
export declare function checkRenditions(self: HangCatalogExt): rt.Result<undefined, HangError>;
export declare function media(self: HangCatalogExt): HangCatalog;
export declare function subscribe(broadcast: BroadcastConsumer): rt.Call<rt.Result<HangCatalogConsumerExt, HangError>>;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalogExt, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalogExt, HangError>;
export declare function toJson(self: HangCatalogExt): rt.Result<string, HangError>;
export declare function toJsonPretty(self: HangCatalogExt): rt.Result<string, HangError>;
export declare function toVec(self: HangCatalogExt): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: HangCatalogExt, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;
export { default$ as default };

// HangCatalogExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogConsumerExtra } from "./HangCatalogConsumerExtra.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
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
export declare function mediaGet(self: HangCatalogExtra): HangCatalogMedia;
export declare function mediaSet(self: HangCatalogExtra, value: HangCatalogMedia): void;
export declare function textGet(self: HangCatalogExtra): HangCatalogText;
export declare function textSet(self: HangCatalogExtra, value: HangCatalogText): void;
export declare function jsonGet(self: HangCatalogExtra): HangCatalogJson;
export declare function jsonSet(self: HangCatalogExtra, value: HangCatalogJson): void;
export declare function binaryGet(self: HangCatalogExtra): HangCatalogBinary;
export declare function binarySet(self: HangCatalogExtra, value: HangCatalogBinary): void;
export declare function m2tsGet(self: HangCatalogExtra): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: HangCatalogExtra, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: HangCatalogExtra): HangCatalogStats | undefined;
export declare function statsSet(self: HangCatalogExtra, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: HangCatalogExtra): HangCatalogEcho | undefined;
export declare function echoSet(self: HangCatalogExtra, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: HangCatalogExtra): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: HangCatalogExtra, value: ([string, HangCatalogContribution])[]): void;
export declare function extGet(self: HangCatalogExtra): MuxCatalogHangExtra;
export declare function extSet(self: HangCatalogExtra, value: MuxCatalogHangExtra): void;
export declare function checkRenditions(self: HangCatalogExtra): rt.Result<undefined, HangError>;
export declare function media(self: HangCatalogExtra): HangCatalog;
export declare function subscribe(broadcast: BroadcastConsumer): rt.Call<rt.Result<HangCatalogConsumerExtra, HangError>>;
export declare function fromSlice(v: Uint8Array | rt.Bytes): rt.Result<HangCatalogExtra, HangError>;
export declare function fromReader(reader: Uint8Array | rt.Bytes): rt.Result<HangCatalogExtra, HangError>;
export declare function toJson(self: HangCatalogExtra): rt.Result<string, HangError>;
export declare function toJsonPretty(self: HangCatalogExtra): rt.Result<string, HangError>;
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

// HangCatalogJoin.ts
import * as rt from "./rt.ts";
export type HangCatalogJoin = rt.Handle<"HangCatalogJoin">;
export declare function debug(self: HangCatalogJoin): string;
export declare function clone(self: HangCatalogJoin): HangCatalogJoin;
export declare function eq(a: HangCatalogJoin, b: HangCatalogJoin): boolean;
export declare function intervalGet(self: HangCatalogJoin): rt.Duration;
export declare function intervalSet(self: HangCatalogJoin, value: rt.Duration): void;
export declare function new$(interval: rt.Duration): HangCatalogJoin | undefined;
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

// HangCatalogM2ts.ts
import * as rt from "./rt.ts";
export type HangCatalogM2ts = rt.Handle<"HangCatalogM2ts">;
export declare function debug(self: HangCatalogM2ts): string;
export declare function clone(self: HangCatalogM2ts): HangCatalogM2ts;
export declare function eq(a: HangCatalogM2ts, b: HangCatalogM2ts): boolean;
export declare function trackGet(self: HangCatalogM2ts): string;
export declare function trackSet(self: HangCatalogM2ts, value: string): void;
export declare function randomAccessGet(self: HangCatalogM2ts): boolean;
export declare function randomAccessSet(self: HangCatalogM2ts, value: boolean): void;
export declare function muxRateGet(self: HangCatalogM2ts): rt.u64 | undefined;
export declare function muxRateSet(self: HangCatalogM2ts, value: rt.u64 | undefined): void;
export declare function new$(track: string): HangCatalogM2ts;
export { new$ as new };

// HangCatalogMastering.ts
import type { HangCatalogChromaticity } from "./HangCatalogChromaticity.ts";
export type HangCatalogMastering = {
    red: HangCatalogChromaticity;
    green: HangCatalogChromaticity;
    blue: HangCatalogChromaticity;
    white: HangCatalogChromaticity;
    maxLuminance: number;
    minLuminance: number;
};
export declare function debug(self: HangCatalogMastering): string;
export declare function eq(a: HangCatalogMastering, b: HangCatalogMastering): boolean;

// HangCatalogMedia.ts
import * as rt from "./rt.ts";
import type { HangCatalogClock } from "./HangCatalogClock.ts";
import type { HangCatalogJoin } from "./HangCatalogJoin.ts";
export type HangCatalogMedia = rt.Handle<"HangCatalogMedia">;
export declare function debug(self: HangCatalogMedia): string;
export declare function clone(self: HangCatalogMedia): HangCatalogMedia;
export declare function eq(a: HangCatalogMedia, b: HangCatalogMedia): boolean;
export declare function default$(): HangCatalogMedia;
export declare function clockGet(self: HangCatalogMedia): HangCatalogClock | undefined;
export declare function clockSet(self: HangCatalogMedia, value: HangCatalogClock | undefined): void;
export declare function joinGet(self: HangCatalogMedia): HangCatalogJoin | undefined;
export declare function joinSet(self: HangCatalogMedia, value: HangCatalogJoin | undefined): void;
export declare function isEmpty(self: HangCatalogMedia): boolean;
export declare function interval(self: HangCatalogMedia): rt.Duration | undefined;
export { default$ as default };

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

// HangCatalogRange.ts
export type HangCatalogRange = "Limited" | "Full";
export declare function debug(self: HangCatalogRange): string;

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
export declare function rotationGet(self: HangCatalogVideo): number | undefined;
export declare function rotationSet(self: HangCatalogVideo, value: number | undefined): void;
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
export declare function framerateGet(self: HangCatalogVideoConfig): number | undefined;
export declare function framerateSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function reorderGet(self: HangCatalogVideoConfig): number | undefined;
export declare function reorderSet(self: HangCatalogVideoConfig, value: number | undefined): void;
export declare function optimizeForLatencyGet(self: HangCatalogVideoConfig): boolean | undefined;
export declare function optimizeForLatencySet(self: HangCatalogVideoConfig, value: boolean | undefined): void;
export declare function containerGet(self: HangCatalogVideoConfig): HangCatalogContainer;
export declare function containerSet(self: HangCatalogVideoConfig, value: HangCatalogContainer): void;
export declare function jitterGet(self: HangCatalogVideoConfig): rt.Duration | undefined;
export declare function jitterSet(self: HangCatalogVideoConfig, value: rt.Duration | undefined): void;
export declare function warmupGet(self: HangCatalogVideoConfig): rt.Duration | undefined;
export declare function warmupSet(self: HangCatalogVideoConfig, value: rt.Duration | undefined): void;
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
export declare function rotationGet(self: HangCatalogVideoProperties): number | undefined;
export declare function rotationSet(self: HangCatalogVideoProperties, value: number | undefined): void;
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
export declare function latencyGet(self: HangEchoRendition): rt.Duration | undefined;
export declare function latencySet(self: HangEchoRendition, value: rt.Duration | undefined): void;
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

// HangEchoSnapshotString.ts
import * as rt from "./rt.ts";
import type { HangEchoRendition } from "./HangEchoRendition.ts";
import type { HangStatsTransport } from "./HangStatsTransport.ts";
export type HangEchoSnapshotString = rt.Handle<"HangEchoSnapshotString">;
export declare function debug(self: HangEchoSnapshotString): string;
export declare function clone(self: HangEchoSnapshotString): HangEchoSnapshotString;
export declare function eq(a: HangEchoSnapshotString, b: HangEchoSnapshotString): boolean;
export declare function default$(): HangEchoSnapshotString;
export declare function transportGet(self: HangEchoSnapshotString): HangStatsTransport;
export declare function transportSet(self: HangEchoSnapshotString, value: HangStatsTransport): void;
export declare function renditionsGet(self: HangEchoSnapshotString): ([string, HangEchoRendition])[];
export declare function renditionsSet(self: HangEchoSnapshotString, value: ([string, HangEchoRendition])[]): void;
export declare function extGet(self: HangEchoSnapshotString): string;
export declare function extSet(self: HangEchoSnapshotString, value: string): void;
export { default$ as default };

// HangError.ts
import * as rt from "./rt.ts";
export type HangError = rt.Handle<"HangError">;
export declare function toString(self: HangError): string;
export declare function debug(self: HangError): string;
export declare function clone(self: HangError): HangError;

// HangHealthConnection.ts
import * as rt from "./rt.ts";
import type { HangHealthEvidence } from "./HangHealthEvidence.ts";
import type { HangHealthVerdict } from "./HangHealthVerdict.ts";
export type HangHealthConnection = rt.Handle<"HangHealthConnection">;
export declare function debug(self: HangHealthConnection): string;
export declare function clone(self: HangHealthConnection): HangHealthConnection;
export declare function eq(a: HangHealthConnection, b: HangHealthConnection): boolean;
export declare function default$(): HangHealthConnection;
export declare function verdictGet(self: HangHealthConnection): HangHealthVerdict;
export declare function verdictSet(self: HangHealthConnection, value: HangHealthVerdict): void;
export declare function evidenceGet(self: HangHealthConnection): HangHealthEvidence[];
export declare function evidenceSet(self: HangHealthConnection, value: HangHealthEvidence[]): void;
export declare function staleGet(self: HangHealthConnection): boolean;
export declare function staleSet(self: HangHealthConnection, value: boolean): void;
export declare function lostBytesGet(self: HangHealthConnection): rt.u64 | undefined;
export declare function lostBytesSet(self: HangHealthConnection, value: rt.u64 | undefined): void;
export declare function lostPacketsGet(self: HangHealthConnection): rt.u64 | undefined;
export declare function lostPacketsSet(self: HangHealthConnection, value: rt.u64 | undefined): void;
export { default$ as default };

// HangHealthEvidence.ts
import * as rt from "./rt.ts";
import type { HangHealthInput } from "./HangHealthInput.ts";
import type { HangHealthVerdict } from "./HangHealthVerdict.ts";
export type HangHealthEvidence = rt.Handle<"HangHealthEvidence">;
export declare function debug(self: HangHealthEvidence): string;
export declare function clone(self: HangHealthEvidence): HangHealthEvidence;
export declare function eq(a: HangHealthEvidence, b: HangHealthEvidence): boolean;
export declare function inputGet(self: HangHealthEvidence): HangHealthInput;
export declare function inputSet(self: HangHealthEvidence, value: HangHealthInput): void;
export declare function valueGet(self: HangHealthEvidence): number | undefined;
export declare function valueSet(self: HangHealthEvidence, value: number | undefined): void;
export declare function verdictGet(self: HangHealthEvidence): HangHealthVerdict;
export declare function verdictSet(self: HangHealthEvidence, value: HangHealthVerdict): void;

// HangHealthInput.ts
export type HangHealthInput = "Rtt" | "Loss" | "Headroom" | "Skipped" | "Late" | "Errors" | "Stalled";
export declare function debug(self: HangHealthInput): string;

// HangHealthLimitsDuration.ts
import * as rt from "./rt.ts";
export type HangHealthLimitsDuration = {
    degraded: rt.Duration;
    unhealthy: rt.Duration;
};
export declare function debug(self: HangHealthLimitsDuration): string;
export declare function eq(a: HangHealthLimitsDuration, b: HangHealthLimitsDuration): boolean;

// HangHealthLimitsF64.ts
export type HangHealthLimitsF64 = {
    degraded: number;
    unhealthy: number;
};
export declare function debug(self: HangHealthLimitsF64): string;
export declare function eq(a: HangHealthLimitsF64, b: HangHealthLimitsF64): boolean;

// HangHealthObserver.ts
import * as rt from "./rt.ts";
import type { Instance } from "./Instance.ts";
import type { Path } from "./Path.ts";
export type HangHealthObserver = rt.Handle<"HangHealthObserver">;
export declare function debug(self: HangHealthObserver): string;
export declare function clone(self: HangHealthObserver): HangHealthObserver;
export declare function eq(a: HangHealthObserver, b: HangHealthObserver): boolean;
export declare function pathGet(self: HangHealthObserver): Path;
export declare function pathSet(self: HangHealthObserver, value: Path): void;
export declare function instanceGet(self: HangHealthObserver): Instance | undefined;
export declare function instanceSet(self: HangHealthObserver, value: Instance | undefined): void;
export declare function new$(path: Path, instance: Instance | undefined): HangHealthObserver;
export { new$ as new };

// HangHealthPublisherThresholds.ts
import * as rt from "./rt.ts";
import type { HangHealthLimitsF64 } from "./HangHealthLimitsF64.ts";
export type HangHealthPublisherThresholds = rt.Handle<"HangHealthPublisherThresholds">;
export declare function debug(self: HangHealthPublisherThresholds): string;
export declare function clone(self: HangHealthPublisherThresholds): HangHealthPublisherThresholds;
export declare function eq(a: HangHealthPublisherThresholds, b: HangHealthPublisherThresholds): boolean;
export declare function skippedGet(self: HangHealthPublisherThresholds): HangHealthLimitsF64;
export declare function skippedSet(self: HangHealthPublisherThresholds, value: HangHealthLimitsF64): void;

// HangHealthRendition.ts
import * as rt from "./rt.ts";
import type { HangHealthEvidence } from "./HangHealthEvidence.ts";
import type { HangHealthVerdict } from "./HangHealthVerdict.ts";
export type HangHealthRendition = rt.Handle<"HangHealthRendition">;
export declare function debug(self: HangHealthRendition): string;
export declare function clone(self: HangHealthRendition): HangHealthRendition;
export declare function eq(a: HangHealthRendition, b: HangHealthRendition): boolean;
export declare function default$(): HangHealthRendition;
export declare function verdictGet(self: HangHealthRendition): HangHealthVerdict;
export declare function verdictSet(self: HangHealthRendition, value: HangHealthVerdict): void;
export declare function evidenceGet(self: HangHealthRendition): HangHealthEvidence[];
export declare function evidenceSet(self: HangHealthRendition, value: HangHealthEvidence[]): void;
export declare function framesGet(self: HangHealthRendition): rt.u64 | undefined;
export declare function framesSet(self: HangHealthRendition, value: rt.u64 | undefined): void;
export declare function bytesGet(self: HangHealthRendition): rt.u64 | undefined;
export declare function bytesSet(self: HangHealthRendition, value: rt.u64 | undefined): void;
export declare function bitrateGet(self: HangHealthRendition): rt.u64 | undefined;
export declare function bitrateSet(self: HangHealthRendition, value: rt.u64 | undefined): void;
export { default$ as default };

// HangHealthReportEchoSnapshotString.ts
import * as rt from "./rt.ts";
import type { HangEchoSnapshotString } from "./HangEchoSnapshotString.ts";
import type { HangHealthObserver } from "./HangHealthObserver.ts";
export type HangHealthReportEchoSnapshotString = rt.Handle<"HangHealthReportEchoSnapshotString">;
export declare function debug(self: HangHealthReportEchoSnapshotString): string;
export declare function clone(self: HangHealthReportEchoSnapshotString): HangHealthReportEchoSnapshotString;
export declare function eq(a: HangHealthReportEchoSnapshotString, b: HangHealthReportEchoSnapshotString): boolean;
export declare function observerGet(self: HangHealthReportEchoSnapshotString): HangHealthObserver;
export declare function observerSet(self: HangHealthReportEchoSnapshotString, value: HangHealthObserver): void;
export declare function snapshotGet(self: HangHealthReportEchoSnapshotString): HangEchoSnapshotString;
export declare function snapshotSet(self: HangHealthReportEchoSnapshotString, value: HangEchoSnapshotString): void;
export declare function new$(observer: HangHealthObserver, snapshot: HangEchoSnapshotString): HangHealthReportEchoSnapshotString;
export { new$ as new };

// HangHealthReportStatsSnapshotString.ts
import * as rt from "./rt.ts";
import type { HangHealthObserver } from "./HangHealthObserver.ts";
import type { HangStatsSnapshotString } from "./HangStatsSnapshotString.ts";
export type HangHealthReportStatsSnapshotString = rt.Handle<"HangHealthReportStatsSnapshotString">;
export declare function debug(self: HangHealthReportStatsSnapshotString): string;
export declare function clone(self: HangHealthReportStatsSnapshotString): HangHealthReportStatsSnapshotString;
export declare function eq(a: HangHealthReportStatsSnapshotString, b: HangHealthReportStatsSnapshotString): boolean;
export declare function observerGet(self: HangHealthReportStatsSnapshotString): HangHealthObserver;
export declare function observerSet(self: HangHealthReportStatsSnapshotString, value: HangHealthObserver): void;
export declare function snapshotGet(self: HangHealthReportStatsSnapshotString): HangStatsSnapshotString;
export declare function snapshotSet(self: HangHealthReportStatsSnapshotString, value: HangStatsSnapshotString): void;
export declare function new$(observer: HangHealthObserver, snapshot: HangStatsSnapshotString): HangHealthReportStatsSnapshotString;
export { new$ as new };

// HangHealthReset.ts
export type HangHealthReset = "Decrease" | "Replaced";
export declare function debug(self: HangHealthReset): string;

// HangHealthRole.ts
export type HangHealthRole = "Publisher" | "Viewer";
export declare function debug(self: HangHealthRole): string;

// HangHealthThresholds.ts
import * as rt from "./rt.ts";
import type { HangHealthPublisherThresholds } from "./HangHealthPublisherThresholds.ts";
import type { HangHealthTransportThresholds } from "./HangHealthTransportThresholds.ts";
import type { HangHealthViewerThresholds } from "./HangHealthViewerThresholds.ts";
export type HangHealthThresholds = rt.Handle<"HangHealthThresholds">;
export declare function debug(self: HangHealthThresholds): string;
export declare function clone(self: HangHealthThresholds): HangHealthThresholds;
export declare function eq(a: HangHealthThresholds, b: HangHealthThresholds): boolean;
export declare function default$(): HangHealthThresholds;
export declare function staleGet(self: HangHealthThresholds): rt.Duration;
export declare function staleSet(self: HangHealthThresholds, value: rt.Duration): void;
export declare function transportGet(self: HangHealthThresholds): HangHealthTransportThresholds;
export declare function transportSet(self: HangHealthThresholds, value: HangHealthTransportThresholds): void;
export declare function publisherGet(self: HangHealthThresholds): HangHealthPublisherThresholds;
export declare function publisherSet(self: HangHealthThresholds, value: HangHealthPublisherThresholds): void;
export declare function viewerGet(self: HangHealthThresholds): HangHealthViewerThresholds;
export declare function viewerSet(self: HangHealthThresholds, value: HangHealthViewerThresholds): void;
export { default$ as default };

// HangHealthTransportThresholds.ts
import * as rt from "./rt.ts";
import type { HangHealthLimitsDuration } from "./HangHealthLimitsDuration.ts";
import type { HangHealthLimitsF64 } from "./HangHealthLimitsF64.ts";
export type HangHealthTransportThresholds = rt.Handle<"HangHealthTransportThresholds">;
export declare function debug(self: HangHealthTransportThresholds): string;
export declare function clone(self: HangHealthTransportThresholds): HangHealthTransportThresholds;
export declare function eq(a: HangHealthTransportThresholds, b: HangHealthTransportThresholds): boolean;
export declare function rttGet(self: HangHealthTransportThresholds): HangHealthLimitsDuration;
export declare function rttSet(self: HangHealthTransportThresholds, value: HangHealthLimitsDuration): void;
export declare function lossGet(self: HangHealthTransportThresholds): HangHealthLimitsF64;
export declare function lossSet(self: HangHealthTransportThresholds, value: HangHealthLimitsF64): void;
export declare function headroomGet(self: HangHealthTransportThresholds): HangHealthLimitsF64;
export declare function headroomSet(self: HangHealthTransportThresholds, value: HangHealthLimitsF64): void;

// HangHealthVerdict.ts
export type HangHealthVerdict = "Unknown" | "Healthy" | "Degraded" | "Unhealthy";
export declare function debug(self: HangHealthVerdict): string;
export declare function default$(): HangHealthVerdict;
export { default$ as default };

// HangHealthViewerThresholds.ts
import * as rt from "./rt.ts";
import type { HangHealthLimitsF64 } from "./HangHealthLimitsF64.ts";
export type HangHealthViewerThresholds = rt.Handle<"HangHealthViewerThresholds">;
export declare function debug(self: HangHealthViewerThresholds): string;
export declare function clone(self: HangHealthViewerThresholds): HangHealthViewerThresholds;
export declare function eq(a: HangHealthViewerThresholds, b: HangHealthViewerThresholds): boolean;
export declare function lateGet(self: HangHealthViewerThresholds): HangHealthLimitsF64;
export declare function lateSet(self: HangHealthViewerThresholds, value: HangHealthLimitsF64): void;
export declare function errorsGet(self: HangHealthViewerThresholds): HangHealthLimitsF64;
export declare function errorsSet(self: HangHealthViewerThresholds, value: HangHealthLimitsF64): void;
export declare function stalledGet(self: HangHealthViewerThresholds): HangHealthLimitsF64;
export declare function stalledSet(self: HangHealthViewerThresholds, value: HangHealthLimitsF64): void;

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

// HangStatsSnapshotString.ts
import * as rt from "./rt.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { HangStatsTransport } from "./HangStatsTransport.ts";
export type HangStatsSnapshotString = rt.Handle<"HangStatsSnapshotString">;
export declare function debug(self: HangStatsSnapshotString): string;
export declare function clone(self: HangStatsSnapshotString): HangStatsSnapshotString;
export declare function eq(a: HangStatsSnapshotString, b: HangStatsSnapshotString): boolean;
export declare function default$(): HangStatsSnapshotString;
export declare function transportGet(self: HangStatsSnapshotString): HangStatsTransport;
export declare function transportSet(self: HangStatsSnapshotString, value: HangStatsTransport): void;
export declare function renditionsGet(self: HangStatsSnapshotString): ([string, HangStatsRendition])[];
export declare function renditionsSet(self: HangStatsSnapshotString, value: ([string, HangStatsRendition])[]): void;
export declare function extGet(self: HangStatsSnapshotString): string;
export declare function extSet(self: HangStatsSnapshotString, value: string): void;
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

// Hop.ts
import * as rt from "./rt.ts";
import type { InvalidHop } from "./InvalidHop.ts";
export type Hop = rt.Opaque<"Hop">;
export declare function toString(self: Hop): string;
export declare function debug(self: Hop): string;
export declare function eq(a: Hop, b: Hop): boolean;
export declare function hash(self: Hop): rt.u64;
export declare function UNKNOWN(): Hop;
export declare function new$(id: rt.u64): rt.Result<Hop, InvalidHop>;
export declare function random(): Hop;
export declare function id(self: Hop): rt.u64;
export { new$ as new };

// Hops.ts
import * as rt from "./rt.ts";
import type { Hop } from "./Hop.ts";
import type { InvalidHop } from "./InvalidHop.ts";
export type Hops = rt.Handle<"Hops">;
export declare function debug(self: Hops): string;
export declare function clone(self: Hops): Hops;
export declare function eq(a: Hops, b: Hops): boolean;
export declare function default$(): Hops;
export declare function new$(): Hops;
export declare function push(self: Hops, hop: Hop): rt.Result<undefined, InvalidHop>;
export declare function contains(self: Hops, hop: Hop): boolean;
export declare function len(self: Hops): number;
export declare function isEmpty(self: Hops): boolean;
export declare function iter(self: Hops): Hop[];
export declare function asSlice(self: Hops): Hop[];
export { default$ as default, new$ as new };

// Instance.ts
import * as rt from "./rt.ts";
export type Instance = rt.Handle<"Instance">;
export declare function toString(self: Instance): string;
export declare function debug(self: Instance): string;
export declare function clone(self: Instance): Instance;
export declare function eq(a: Instance, b: Instance): boolean;
export declare function hash(self: Instance): rt.u64;
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

// IntersectionError.ts
export type IntersectionError = "TooManyPatterns";
export declare function toString(self: IntersectionError): string;
export declare function debug(self: IntersectionError): string;

// InvalidHop.ts
export type InvalidHop = "Range" | "TooMany" | "Duplicate";
export declare function toString(self: InvalidHop): string;
export declare function debug(self: InvalidHop): string;

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

// Mp4AtomAc3.ts
import type { Mp4AtomAc3SpecificBox } from "./Mp4AtomAc3SpecificBox.ts";
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
export type Mp4AtomAc3 = {
    audio: Mp4AtomAudio;
    dac3: Mp4AtomAc3SpecificBox;
};
export declare function debug(self: Mp4AtomAc3): string;
export declare function eq(a: Mp4AtomAc3, b: Mp4AtomAc3): boolean;

// Mp4AtomAc3SpecificBox.ts
export type Mp4AtomAc3SpecificBox = {
    fscod: number;
    bsid: number;
    bsmod: number;
    acmod: number;
    lfeon: boolean;
    bitRateCode: number;
};
export declare function debug(self: Mp4AtomAc3SpecificBox): string;
export declare function eq(a: Mp4AtomAc3SpecificBox, b: Mp4AtomAc3SpecificBox): boolean;

// Mp4AtomAinf.ts
export type Mp4AtomAinf = {
    hidden: boolean;
    profileVersion: number;
    apid: string;
};
export declare function debug(self: Mp4AtomAinf): string;
export declare function eq(a: Mp4AtomAinf, b: Mp4AtomAinf): boolean;
export declare function default$(): Mp4AtomAinf;
export { default$ as default };

// Mp4AtomAmrSampleEntry.ts
export type Mp4AtomAmrSampleEntry = {
    dataReferenceIndex: number;
    timescale: number;
};
export declare function debug(self: Mp4AtomAmrSampleEntry): string;
export declare function eq(a: Mp4AtomAmrSampleEntry, b: Mp4AtomAmrSampleEntry): boolean;

// Mp4AtomAny.ts
import * as rt from "./rt.ts";
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomAny = rt.Handle<"Mp4AtomAny">;
export declare function debug(self: Mp4AtomAny): string;
export declare function clone(self: Mp4AtomAny): Mp4AtomAny;
export declare function eq(a: Mp4AtomAny, b: Mp4AtomAny): boolean;
export declare function kind(self: Mp4AtomAny): Mp4AtomFourCc;

// Mp4AtomAnySampleGroupEntry.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomAnySampleGroupEntry = {
    tag: "DirectReferenceSampleList";
    _0: number;
    _1: Uint32Array;
} | {
    tag: "UnknownGroupingType";
    _0: Mp4AtomFourCc;
    _1: Uint8Array;
};
export declare function debug(self: Mp4AtomAnySampleGroupEntry): string;
export declare function eq(a: Mp4AtomAnySampleGroupEntry, b: Mp4AtomAnySampleGroupEntry): boolean;

// Mp4AtomAudio.ts
import type { Mp4AtomFixedPointU16 } from "./Mp4AtomFixedPointU16.ts";
export type Mp4AtomAudio = {
    dataReferenceIndex: number;
    channelCount: number;
    sampleSize: number;
    sampleRate: Mp4AtomFixedPointU16;
};
export declare function debug(self: Mp4AtomAudio): string;
export declare function eq(a: Mp4AtomAudio, b: Mp4AtomAudio): boolean;

// Mp4AtomAudioChannelPosition.ts
export type Mp4AtomAudioChannelPosition = "FrontLeft" | "FrontRight" | "FrontCenter" | "Lfe1" | "LeftSurround" | "RightSurround" | "FrontLeftOfCenter" | "FrontRightOfCenter" | "RearLeft" | "RearRight" | "RearCenter" | "SurroundLeft" | "SurroundRight" | "SideLeft" | "SideRight" | "FrontLeftWide" | "FrontRightWide" | "TopFrontLeft" | "TopFrontRight" | "TopFrontCenter" | "TopRearLeft" | "TopRearRight" | "TopRearCenter" | "TopSideLeft" | "TopSideRight" | "TopCenter" | "Lfe2" | "BottomFrontLeft" | "BottomFrontRight" | "BottomFrontCenter" | "TopSurroundLeft" | "TopSurroundRight" | "Lfe3" | "Leos" | "Reos" | "Hwbcal" | "Hwbcar" | "Lbs" | "Rbs" | "LeftEar" | "RightEar" | "Unknown";
export declare function debug(self: Mp4AtomAudioChannelPosition): string;
export declare function fromU8(value: number): Mp4AtomAudioChannelPosition | undefined;

// Mp4AtomAuxInfo.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomAuxInfo = {
    auxInfoType: Mp4AtomFourCc;
    auxInfoTypeParameter: number;
};
export declare function debug(self: Mp4AtomAuxInfo): string;
export declare function eq(a: Mp4AtomAuxInfo, b: Mp4AtomAuxInfo): boolean;

// Mp4AtomAuxc.ts
export type Mp4AtomAuxc = {
    auxType: string;
    auxSubtype: Uint8Array;
};
export declare function debug(self: Mp4AtomAuxc): string;
export declare function eq(a: Mp4AtomAuxc, b: Mp4AtomAuxc): boolean;
export declare function default$(): Mp4AtomAuxc;
export { default$ as default };

// Mp4AtomAv01.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAv1c } from "./Mp4AtomAv1c.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomCcst } from "./Mp4AtomCcst.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomAv01 = rt.Handle<"Mp4AtomAv01">;
export declare function debug(self: Mp4AtomAv01): string;
export declare function clone(self: Mp4AtomAv01): Mp4AtomAv01;
export declare function eq(a: Mp4AtomAv01, b: Mp4AtomAv01): boolean;
export declare function default$(): Mp4AtomAv01;
export declare function visualGet(self: Mp4AtomAv01): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomAv01, value: Mp4AtomVisual): void;
export declare function av1cGet(self: Mp4AtomAv01): Mp4AtomAv1c;
export declare function av1cSet(self: Mp4AtomAv01, value: Mp4AtomAv1c): void;
export declare function btrtGet(self: Mp4AtomAv01): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomAv01, value: Mp4AtomBtrt | undefined): void;
export declare function ccstGet(self: Mp4AtomAv01): Mp4AtomCcst | undefined;
export declare function ccstSet(self: Mp4AtomAv01, value: Mp4AtomCcst | undefined): void;
export declare function colrGet(self: Mp4AtomAv01): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomAv01, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomAv01): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomAv01, value: Mp4AtomPasp | undefined): void;
export declare function taicGet(self: Mp4AtomAv01): Mp4AtomTaic | undefined;
export declare function taicSet(self: Mp4AtomAv01, value: Mp4AtomTaic | undefined): void;
export declare function spatialGet(self: Mp4AtomAv01): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomAv01, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomAv1c.ts
export type Mp4AtomAv1c = {
    seqProfile: number;
    seqLevelIdx0: number;
    seqTier0: boolean;
    highBitdepth: boolean;
    twelveBit: boolean;
    monochrome: boolean;
    chromaSubsamplingX: boolean;
    chromaSubsamplingY: boolean;
    chromaSamplePosition: number;
    initialPresentationDelay: number | undefined;
    configObus: Uint8Array;
};
export declare function debug(self: Mp4AtomAv1c): string;
export declare function eq(a: Mp4AtomAv1c, b: Mp4AtomAv1c): boolean;
export declare function default$(): Mp4AtomAv1c;
export { default$ as default };

// Mp4AtomAvc1.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAvcc } from "./Mp4AtomAvcc.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomAvc1 = rt.Handle<"Mp4AtomAvc1">;
export declare function debug(self: Mp4AtomAvc1): string;
export declare function clone(self: Mp4AtomAvc1): Mp4AtomAvc1;
export declare function eq(a: Mp4AtomAvc1, b: Mp4AtomAvc1): boolean;
export declare function default$(): Mp4AtomAvc1;
export declare function visualGet(self: Mp4AtomAvc1): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomAvc1, value: Mp4AtomVisual): void;
export declare function avccGet(self: Mp4AtomAvc1): Mp4AtomAvcc;
export declare function avccSet(self: Mp4AtomAvc1, value: Mp4AtomAvcc): void;
export declare function btrtGet(self: Mp4AtomAvc1): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomAvc1, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomAvc1): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomAvc1, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomAvc1): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomAvc1, value: Mp4AtomPasp | undefined): void;
export declare function taicGet(self: Mp4AtomAvc1): Mp4AtomTaic | undefined;
export declare function taicSet(self: Mp4AtomAvc1, value: Mp4AtomTaic | undefined): void;
export declare function fielGet(self: Mp4AtomAvc1): Mp4AtomFiel | undefined;
export declare function fielSet(self: Mp4AtomAvc1, value: Mp4AtomFiel | undefined): void;
export declare function spatialGet(self: Mp4AtomAvc1): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomAvc1, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomAvc3.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAvcc } from "./Mp4AtomAvcc.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomAvc3 = rt.Handle<"Mp4AtomAvc3">;
export declare function debug(self: Mp4AtomAvc3): string;
export declare function clone(self: Mp4AtomAvc3): Mp4AtomAvc3;
export declare function eq(a: Mp4AtomAvc3, b: Mp4AtomAvc3): boolean;
export declare function default$(): Mp4AtomAvc3;
export declare function visualGet(self: Mp4AtomAvc3): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomAvc3, value: Mp4AtomVisual): void;
export declare function avccGet(self: Mp4AtomAvc3): Mp4AtomAvcc;
export declare function avccSet(self: Mp4AtomAvc3, value: Mp4AtomAvcc): void;
export declare function btrtGet(self: Mp4AtomAvc3): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomAvc3, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomAvc3): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomAvc3, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomAvc3): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomAvc3, value: Mp4AtomPasp | undefined): void;
export declare function taicGet(self: Mp4AtomAvc3): Mp4AtomTaic | undefined;
export declare function taicSet(self: Mp4AtomAvc3, value: Mp4AtomTaic | undefined): void;
export declare function fielGet(self: Mp4AtomAvc3): Mp4AtomFiel | undefined;
export declare function fielSet(self: Mp4AtomAvc3, value: Mp4AtomFiel | undefined): void;
export declare function spatialGet(self: Mp4AtomAvc3): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomAvc3, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomAvcc.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAvccExt } from "./Mp4AtomAvccExt.ts";
export type Mp4AtomAvcc = {
    configurationVersion: number;
    avcProfileIndication: number;
    profileCompatibility: number;
    avcLevelIndication: number;
    lengthSize: number;
    sequenceParameterSets: Uint8Array[];
    pictureParameterSets: Uint8Array[];
    ext: Mp4AtomAvccExt | undefined;
};
export declare function debug(self: Mp4AtomAvcc): string;
export declare function eq(a: Mp4AtomAvcc, b: Mp4AtomAvcc): boolean;
export declare function default$(): Mp4AtomAvcc;
export declare function new$(sps: Uint8Array | rt.Bytes, pps: Uint8Array | rt.Bytes): rt.Result<Mp4AtomAvcc, string>;
export { default$ as default, new$ as new };

// Mp4AtomAvccExt.ts
export type Mp4AtomAvccExt = {
    chromaFormat: number;
    bitDepthLuma: number;
    bitDepthChroma: number;
    sequenceParameterSetsExt: Uint8Array[];
};
export declare function debug(self: Mp4AtomAvccExt): string;
export declare function eq(a: Mp4AtomAvccExt, b: Mp4AtomAvccExt): boolean;
export declare function default$(): Mp4AtomAvccExt;
export { default$ as default };

// Mp4AtomBlin.ts
export type Mp4AtomBlin = {
    baselineValue: number;
};
export declare function debug(self: Mp4AtomBlin): string;
export declare function eq(a: Mp4AtomBlin, b: Mp4AtomBlin): boolean;
export declare function default$(): Mp4AtomBlin;
export { default$ as default };

// Mp4AtomBtrt.ts
import * as rt from "./rt.ts";
export type Mp4AtomBtrt = {
    bufferSizeDb: number;
    maxBitrate: number;
    avgBitrate: number;
};
export declare function debug(self: Mp4AtomBtrt): string;
export declare function eq(a: Mp4AtomBtrt, b: Mp4AtomBtrt): boolean;
export declare function default$(): Mp4AtomBtrt;
export declare function new$(bufferSizeDb: number, maxBitrate: number, avgBitrate: number): rt.Result<Mp4AtomBtrt, string>;
export { default$ as default, new$ as new };

// Mp4AtomC2pa.ts
export type Mp4AtomC2pa = {
    boxPurpose: string;
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomC2pa): string;
export declare function eq(a: Mp4AtomC2pa, b: Mp4AtomC2pa): boolean;

// Mp4AtomCamm.ts
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
export type Mp4AtomCamm = {
    metadata: Mp4AtomMetaData;
};
export declare function debug(self: Mp4AtomCamm): string;
export declare function eq(a: Mp4AtomCamm, b: Mp4AtomCamm): boolean;

// Mp4AtomCams.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomBlin } from "./Mp4AtomBlin.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
export type Mp4AtomCams = rt.Handle<"Mp4AtomCams">;
export declare function debug(self: Mp4AtomCams): string;
export declare function clone(self: Mp4AtomCams): Mp4AtomCams;
export declare function eq(a: Mp4AtomCams, b: Mp4AtomCams): boolean;
export declare function default$(): Mp4AtomCams;
export declare function mustGet(self: Mp4AtomCams): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomCams, value: Mp4AtomMust | undefined): void;
export declare function blinGet(self: Mp4AtomCams): Mp4AtomBlin | undefined;
export declare function blinSet(self: Mp4AtomCams, value: Mp4AtomBlin | undefined): void;
export declare function unknownGet(self: Mp4AtomCams): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomCams, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomCbmp.ts
export type Mp4AtomCbmp = {
    layout: number;
    padding: number;
};
export declare function debug(self: Mp4AtomCbmp): string;
export declare function eq(a: Mp4AtomCbmp, b: Mp4AtomCbmp): boolean;
export declare function default$(): Mp4AtomCbmp;
export { default$ as default };

// Mp4AtomCcst.ts
export type Mp4AtomCcst = {
    allRefPicsIntra: boolean;
    intraPredUsed: boolean;
    maxRefPerPic: number;
};
export declare function debug(self: Mp4AtomCcst): string;
export declare function eq(a: Mp4AtomCcst, b: Mp4AtomCcst): boolean;
export declare function default$(): Mp4AtomCcst;
export { default$ as default };

// Mp4AtomChannelStructure.ts
import * as rt from "./rt.ts";
import type { Mp4AtomSpeakerPosition } from "./Mp4AtomSpeakerPosition.ts";
export type Mp4AtomChannelStructure = {
    tag: "ExplicitPositions";
    positions: Mp4AtomSpeakerPosition[];
} | {
    tag: "DefinedLayout";
    layout: number;
    omittedChannelsMap: rt.u64 | undefined;
    channelOrderDefinition: number | undefined;
};
export declare function debug(self: Mp4AtomChannelStructure): string;
export declare function eq(a: Mp4AtomChannelStructure, b: Mp4AtomChannelStructure): boolean;

// Mp4AtomChnl.ts
import * as rt from "./rt.ts";
import type { Mp4AtomChannelStructure } from "./Mp4AtomChannelStructure.ts";
export type Mp4AtomChnl = {
    channelStructure: Mp4AtomChannelStructure | undefined;
    objectCount: number | undefined;
    formatOrdering: number | undefined;
    baseChannelCount: number | undefined;
};
export declare function debug(self: Mp4AtomChnl): string;
export declare function eq(a: Mp4AtomChnl, b: Mp4AtomChnl): boolean;
export declare function decodeBodyWithChannelCount(buf: rt.Place<Uint8Array>, channelCount: number): rt.Result<Mp4AtomChnl, string>;

// Mp4AtomChpl.ts
import type { Mp4AtomChplEntry } from "./Mp4AtomChplEntry.ts";
export type Mp4AtomChpl = {
    entries: Mp4AtomChplEntry[];
};
export declare function debug(self: Mp4AtomChpl): string;
export declare function eq(a: Mp4AtomChpl, b: Mp4AtomChpl): boolean;
export declare function default$(): Mp4AtomChpl;
export { default$ as default };

// Mp4AtomChplEntry.ts
import * as rt from "./rt.ts";
export type Mp4AtomChplEntry = {
    start: rt.u64;
    title: string;
};
export declare function debug(self: Mp4AtomChplEntry): string;
export declare function eq(a: Mp4AtomChplEntry, b: Mp4AtomChplEntry): boolean;

// Mp4AtomClap.ts
export type Mp4AtomClap = {
    cleanApertureWidthN: number;
    cleanApertureWidthD: number;
    cleanApertureHeightN: number;
    cleanApertureHeightD: number;
    horizOffN: number;
    horizOffD: number;
    vertOffN: number;
    vertOffD: number;
};
export declare function debug(self: Mp4AtomClap): string;
export declare function eq(a: Mp4AtomClap, b: Mp4AtomClap): boolean;
export declare function default$(): Mp4AtomClap;
export { default$ as default };

// Mp4AtomClockType.ts
export type Mp4AtomClockType = "Unknown" | "DoesNotSync" | "CanSync" | "Reserved";
export declare function debug(self: Mp4AtomClockType): string;
export declare function default$(): Mp4AtomClockType;
export { default$ as default };

// Mp4AtomCmfy.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomDadj } from "./Mp4AtomDadj.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
export type Mp4AtomCmfy = rt.Handle<"Mp4AtomCmfy">;
export declare function debug(self: Mp4AtomCmfy): string;
export declare function clone(self: Mp4AtomCmfy): Mp4AtomCmfy;
export declare function eq(a: Mp4AtomCmfy, b: Mp4AtomCmfy): boolean;
export declare function default$(): Mp4AtomCmfy;
export declare function mustGet(self: Mp4AtomCmfy): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomCmfy, value: Mp4AtomMust | undefined): void;
export declare function dadjGet(self: Mp4AtomCmfy): Mp4AtomDadj | undefined;
export declare function dadjSet(self: Mp4AtomCmfy, value: Mp4AtomDadj | undefined): void;
export declare function unknownGet(self: Mp4AtomCmfy): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomCmfy, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomCmpd.ts
import type { Mp4AtomComponent } from "./Mp4AtomComponent.ts";
export type Mp4AtomCmpd = {
    components: Mp4AtomComponent[];
};
export declare function debug(self: Mp4AtomCmpd): string;
export declare function eq(a: Mp4AtomCmpd, b: Mp4AtomCmpd): boolean;
export declare function default$(): Mp4AtomCmpd;
export { default$ as default };

// Mp4AtomCo64.ts
export type Mp4AtomCo64 = {
    entries: BigUint64Array;
};
export declare function debug(self: Mp4AtomCo64): string;
export declare function eq(a: Mp4AtomCo64, b: Mp4AtomCo64): boolean;
export declare function default$(): Mp4AtomCo64;
export { default$ as default };

// Mp4AtomCodec.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomCodec = rt.Handle<"Mp4AtomCodec">;
export declare function debug(self: Mp4AtomCodec): string;
export declare function clone(self: Mp4AtomCodec): Mp4AtomCodec;
export declare function eq(a: Mp4AtomCodec, b: Mp4AtomCodec): boolean;
export declare function visual(self: Mp4AtomCodec): Mp4AtomVisual | undefined;
export declare function audio(self: Mp4AtomCodec): Mp4AtomAudio | undefined;
export declare function metadata(self: Mp4AtomCodec): Mp4AtomMetaData | undefined;

// Mp4AtomColr.ts
import * as rt from "./rt.ts";
export type Mp4AtomColr = {
    tag: "Nclx";
    colourPrimaries: number;
    transferCharacteristics: number;
    matrixCoefficients: number;
    fullRangeFlag: boolean;
} | {
    tag: "Nclc";
    colourPrimaries: number;
    transferCharacteristics: number;
    matrixCoefficients: number;
} | {
    tag: "Ricc";
    profile: Uint8Array;
} | {
    tag: "Prof";
    profile: Uint8Array;
};
export declare function debug(self: Mp4AtomColr): string;
export declare function eq(a: Mp4AtomColr, b: Mp4AtomColr): boolean;
export declare function default$(): Mp4AtomColr;
export declare function new$(colourPrimaries: number, transferCharacteristics: number, matrixCoefficients: number, fullRangeFlag: boolean): rt.Result<Mp4AtomColr, string>;
export { default$ as default, new$ as new };

// Mp4AtomComponent.ts
export type Mp4AtomComponent = {
    componentType: number;
    componentTypeUri: string | undefined;
};
export declare function debug(self: Mp4AtomComponent): string;
export declare function eq(a: Mp4AtomComponent, b: Mp4AtomComponent): boolean;

// Mp4AtomCompressor.ts
import * as rt from "./rt.ts";
export type Mp4AtomCompressor = rt.Handle<"Mp4AtomCompressor">;
export declare function debug(self: Mp4AtomCompressor): string;
export declare function clone(self: Mp4AtomCompressor): Mp4AtomCompressor;
export declare function eq(a: Mp4AtomCompressor, b: Mp4AtomCompressor): boolean;
export declare function default$(): Mp4AtomCompressor;
export { default$ as default };

// Mp4AtomCopyright.ts
export type Mp4AtomCopyright = {
    countryIndicator: number;
    languageIndicator: number;
    text: string;
};
export declare function debug(self: Mp4AtomCopyright): string;
export declare function eq(a: Mp4AtomCopyright, b: Mp4AtomCopyright): boolean;

// Mp4AtomCovr.ts
export type Mp4AtomCovr = {
    _0: Uint8Array;
};
export declare function debug(self: Mp4AtomCovr): string;
export declare function eq(a: Mp4AtomCovr, b: Mp4AtomCovr): boolean;

// Mp4AtomCprt.ts
export type Mp4AtomCprt = {
    language: string;
    notice: string;
};
export declare function debug(self: Mp4AtomCprt): string;
export declare function eq(a: Mp4AtomCprt, b: Mp4AtomCprt): boolean;
export declare function default$(): Mp4AtomCprt;
export { default$ as default };

// Mp4AtomCslg.ts
import * as rt from "./rt.ts";
export type Mp4AtomCslg = {
    compositionToDtsShift: rt.i64;
    leastDecodeToDisplayShift: rt.i64;
    greatestDecodeToDisplayDelta: rt.i64;
    compositionStartTime: rt.i64;
    compositionEndTime: rt.i64;
};
export declare function debug(self: Mp4AtomCslg): string;
export declare function eq(a: Mp4AtomCslg, b: Mp4AtomCslg): boolean;
export declare function default$(): Mp4AtomCslg;
export { default$ as default };

// Mp4AtomCtts.ts
import type { Mp4AtomCttsEntry } from "./Mp4AtomCttsEntry.ts";
export type Mp4AtomCtts = {
    entries: Mp4AtomCttsEntry[];
};
export declare function debug(self: Mp4AtomCtts): string;
export declare function eq(a: Mp4AtomCtts, b: Mp4AtomCtts): boolean;
export declare function default$(): Mp4AtomCtts;
export { default$ as default };

// Mp4AtomCttsEntry.ts
import * as rt from "./rt.ts";
export type Mp4AtomCttsEntry = {
    sampleCount: number;
    sampleOffset: rt.i64;
};
export declare function debug(self: Mp4AtomCttsEntry): string;
export declare function eq(a: Mp4AtomCttsEntry, b: Mp4AtomCttsEntry): boolean;
export declare function default$(): Mp4AtomCttsEntry;
export { default$ as default };

// Mp4AtomDadj.ts
export type Mp4AtomDadj = {
    disparityAdjustment: number;
};
export declare function debug(self: Mp4AtomDadj): string;
export declare function eq(a: Mp4AtomDadj, b: Mp4AtomDadj): boolean;
export declare function default$(): Mp4AtomDadj;
export { default$ as default };

// Mp4AtomDamr.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomDamr = {
    vendor: Mp4AtomFourCc;
    decoderVersion: number;
    modeSet: number;
    modeChangePeriod: number;
    framesPerSample: number;
};
export declare function debug(self: Mp4AtomDamr): string;
export declare function eq(a: Mp4AtomDamr, b: Mp4AtomDamr): boolean;

// Mp4AtomDataType.ts
export type Mp4AtomDataType = {
    tag: "WellKnown";
    _0: number;
} | {
    tag: "Custom";
    _0: string;
} | {
    tag: "Unknown";
    _0: number;
    _1: Uint8Array;
};
export declare function debug(self: Mp4AtomDataType): string;
export declare function eq(a: Mp4AtomDataType, b: Mp4AtomDataType): boolean;

// Mp4AtomDesc.ts
export type Mp4AtomDesc = {
    countryIndicator: number;
    languageIndicator: number;
    text: string;
};
export declare function debug(self: Mp4AtomDesc): string;
export declare function eq(a: Mp4AtomDesc, b: Mp4AtomDesc): boolean;

// Mp4AtomDfla.ts
import type { Mp4AtomFlacMetadataBlock } from "./Mp4AtomFlacMetadataBlock.ts";
export type Mp4AtomDfla = {
    blocks: Mp4AtomFlacMetadataBlock[];
};
export declare function debug(self: Mp4AtomDfla): string;
export declare function eq(a: Mp4AtomDfla, b: Mp4AtomDfla): boolean;

// Mp4AtomDinf.ts
import type { Mp4AtomDref } from "./Mp4AtomDref.ts";
export type Mp4AtomDinf = {
    dref: Mp4AtomDref;
};
export declare function debug(self: Mp4AtomDinf): string;
export declare function eq(a: Mp4AtomDinf, b: Mp4AtomDinf): boolean;
export declare function default$(): Mp4AtomDinf;
export { default$ as default };

// Mp4AtomDisp.ts
export type Mp4AtomDisp = {
    disparityShiftIn16thPel: number;
};
export declare function debug(self: Mp4AtomDisp): string;
export declare function eq(a: Mp4AtomDisp, b: Mp4AtomDisp): boolean;
export declare function default$(): Mp4AtomDisp;
export { default$ as default };

// Mp4AtomDops.ts
import type { Mp4AtomOpusChannelMapping } from "./Mp4AtomOpusChannelMapping.ts";
export type Mp4AtomDops = {
    outputChannelCount: number;
    preSkip: number;
    inputSampleRate: number;
    outputGain: number;
    channelMapping: Mp4AtomOpusChannelMapping | undefined;
};
export declare function debug(self: Mp4AtomDops): string;
export declare function eq(a: Mp4AtomDops, b: Mp4AtomDops): boolean;

// Mp4AtomDref.ts
import type { Mp4AtomDrefEntry } from "./Mp4AtomDrefEntry.ts";
export type Mp4AtomDref = {
    entries: Mp4AtomDrefEntry[];
};
export declare function debug(self: Mp4AtomDref): string;
export declare function eq(a: Mp4AtomDref, b: Mp4AtomDref): boolean;
export declare function default$(): Mp4AtomDref;
export { default$ as default };

// Mp4AtomDrefEntry.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomUrl } from "./Mp4AtomUrl.ts";
import type { Mp4AtomUrn } from "./Mp4AtomUrn.ts";
export type Mp4AtomDrefEntry = {
    tag: "Url";
    _0: Mp4AtomUrl;
} | {
    tag: "Urn";
    _0: Mp4AtomUrn;
} | {
    tag: "Unknown";
    _0: Mp4AtomFourCc;
    _1: Uint8Array;
};
export declare function debug(self: Mp4AtomDrefEntry): string;
export declare function eq(a: Mp4AtomDrefEntry, b: Mp4AtomDrefEntry): boolean;
export declare function kind(self: Mp4AtomDrefEntry): Mp4AtomFourCc;

// Mp4AtomDtyp.ts
import type { Mp4AtomDataType } from "./Mp4AtomDataType.ts";
export type Mp4AtomDtyp = {
    dataType: Mp4AtomDataType;
};
export declare function debug(self: Mp4AtomDtyp): string;
export declare function eq(a: Mp4AtomDtyp, b: Mp4AtomDtyp): boolean;

// Mp4AtomEac3.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomEc3SpecificBox } from "./Mp4AtomEc3SpecificBox.ts";
export type Mp4AtomEac3 = {
    audio: Mp4AtomAudio;
    dec3: Mp4AtomEc3SpecificBox;
};
export declare function debug(self: Mp4AtomEac3): string;
export declare function eq(a: Mp4AtomEac3, b: Mp4AtomEac3): boolean;

// Mp4AtomEc3IndependentSubstream.ts
export type Mp4AtomEc3IndependentSubstream = {
    fscod: number;
    bsid: number;
    asvc: boolean;
    bsmod: number;
    acmod: number;
    lfeon: boolean;
    numDepSub: number;
    chanLoc: number | undefined;
};
export declare function debug(self: Mp4AtomEc3IndependentSubstream): string;
export declare function eq(a: Mp4AtomEc3IndependentSubstream, b: Mp4AtomEc3IndependentSubstream): boolean;

// Mp4AtomEc3SpecificBox.ts
import type { Mp4AtomEc3IndependentSubstream } from "./Mp4AtomEc3IndependentSubstream.ts";
export type Mp4AtomEc3SpecificBox = {
    dataRate: number;
    substreams: Mp4AtomEc3IndependentSubstream[];
};
export declare function debug(self: Mp4AtomEc3SpecificBox): string;
export declare function eq(a: Mp4AtomEc3SpecificBox, b: Mp4AtomEc3SpecificBox): boolean;

// Mp4AtomEdts.ts
import type { Mp4AtomElst } from "./Mp4AtomElst.ts";
export type Mp4AtomEdts = {
    elst: Mp4AtomElst | undefined;
};
export declare function debug(self: Mp4AtomEdts): string;
export declare function eq(a: Mp4AtomEdts, b: Mp4AtomEdts): boolean;
export declare function default$(): Mp4AtomEdts;
export { default$ as default };

// Mp4AtomElst.ts
import type { Mp4AtomElstEntry } from "./Mp4AtomElstEntry.ts";
export type Mp4AtomElst = {
    entries: Mp4AtomElstEntry[];
};
export declare function debug(self: Mp4AtomElst): string;
export declare function eq(a: Mp4AtomElst, b: Mp4AtomElst): boolean;
export declare function default$(): Mp4AtomElst;
export { default$ as default };

// Mp4AtomElstEntry.ts
import * as rt from "./rt.ts";
import type { Mp4AtomFixedPointI16 } from "./Mp4AtomFixedPointI16.ts";
export type Mp4AtomElstEntry = {
    segmentDuration: rt.u64;
    mediaTime: rt.u64 | undefined;
    mediaRate: Mp4AtomFixedPointI16;
};
export declare function debug(self: Mp4AtomElstEntry): string;
export declare function eq(a: Mp4AtomElstEntry, b: Mp4AtomElstEntry): boolean;
export declare function default$(): Mp4AtomElstEntry;
export { default$ as default };

// Mp4AtomEmsg.ts
import type { Mp4AtomEmsgTimestamp } from "./Mp4AtomEmsgTimestamp.ts";
export type Mp4AtomEmsg = {
    timescale: number;
    presentationTime: Mp4AtomEmsgTimestamp;
    eventDuration: number;
    id: number;
    schemeIdUri: string;
    value: string;
    messageData: Uint8Array;
};
export declare function debug(self: Mp4AtomEmsg): string;
export declare function eq(a: Mp4AtomEmsg, b: Mp4AtomEmsg): boolean;

// Mp4AtomEmsgTimestamp.ts
import * as rt from "./rt.ts";
export type Mp4AtomEmsgTimestamp = {
    tag: "Relative";
    _0: number;
} | {
    tag: "Absolute";
    _0: rt.u64;
};
export declare function debug(self: Mp4AtomEmsgTimestamp): string;
export declare function eq(a: Mp4AtomEmsgTimestamp, b: Mp4AtomEmsgTimestamp): boolean;

// Mp4AtomEqui.ts
export type Mp4AtomEqui = {
    projectionBoundsTop: number;
    projectionBoundsBottom: number;
    projectionBoundsLeft: number;
    projectionBoundsRight: number;
};
export declare function debug(self: Mp4AtomEqui): string;
export declare function eq(a: Mp4AtomEqui, b: Mp4AtomEqui): boolean;
export declare function default$(): Mp4AtomEqui;
export { default$ as default };

// Mp4AtomEsds.ts
import type { Mp4AtomEsdsEsDescriptor } from "./Mp4AtomEsdsEsDescriptor.ts";
export type Mp4AtomEsds = {
    esDesc: Mp4AtomEsdsEsDescriptor;
};
export declare function debug(self: Mp4AtomEsds): string;
export declare function eq(a: Mp4AtomEsds, b: Mp4AtomEsds): boolean;
export declare function default$(): Mp4AtomEsds;
export { default$ as default };

// Mp4AtomEsdsDecoderConfig.ts
import type { Mp4AtomEsdsDecoderSpecific } from "./Mp4AtomEsdsDecoderSpecific.ts";
import type { Mp4AtomU24 } from "./Mp4AtomU24.ts";
export type Mp4AtomEsdsDecoderConfig = {
    objectTypeIndication: number;
    streamType: number;
    upStream: number;
    bufferSizeDb: Mp4AtomU24;
    maxBitrate: number;
    avgBitrate: number;
    decSpecific: Mp4AtomEsdsDecoderSpecific | undefined;
};
export declare function debug(self: Mp4AtomEsdsDecoderConfig): string;
export declare function eq(a: Mp4AtomEsdsDecoderConfig, b: Mp4AtomEsdsDecoderConfig): boolean;
export declare function default$(): Mp4AtomEsdsDecoderConfig;
export { default$ as default };

// Mp4AtomEsdsDecoderSpecific.ts
export type Mp4AtomEsdsDecoderSpecific = {
    profile: number;
    freqIndex: number;
    chanConf: number;
    raw: Uint8Array;
};
export declare function debug(self: Mp4AtomEsdsDecoderSpecific): string;
export declare function eq(a: Mp4AtomEsdsDecoderSpecific, b: Mp4AtomEsdsDecoderSpecific): boolean;
export declare function default$(): Mp4AtomEsdsDecoderSpecific;
export { default$ as default };

// Mp4AtomEsdsEsDescriptor.ts
import type { Mp4AtomEsdsDecoderConfig } from "./Mp4AtomEsdsDecoderConfig.ts";
import type { Mp4AtomEsdsSlConfig } from "./Mp4AtomEsdsSlConfig.ts";
export type Mp4AtomEsdsEsDescriptor = {
    esId: number;
    dependsOnEsId: number | undefined;
    url: Uint8Array | undefined;
    ocrEsId: number | undefined;
    streamPriority: number;
    decConfig: Mp4AtomEsdsDecoderConfig;
    slConfig: Mp4AtomEsdsSlConfig;
};
export declare function debug(self: Mp4AtomEsdsEsDescriptor): string;
export declare function eq(a: Mp4AtomEsdsEsDescriptor, b: Mp4AtomEsdsEsDescriptor): boolean;
export declare function default$(): Mp4AtomEsdsEsDescriptor;
export { default$ as default };

// Mp4AtomEsdsSlConfig.ts
import * as rt from "./rt.ts";
export type Mp4AtomEsdsSlConfig = rt.Opaque<"Mp4AtomEsdsSlConfig">;
export declare function debug(self: Mp4AtomEsdsSlConfig): string;
export declare function eq(a: Mp4AtomEsdsSlConfig, b: Mp4AtomEsdsSlConfig): boolean;
export declare function default$(): Mp4AtomEsdsSlConfig;
export { default$ as default };

// Mp4AtomEvte.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
import type { Mp4AtomSilb } from "./Mp4AtomSilb.ts";
export type Mp4AtomEvte = {
    metadata: Mp4AtomMetaData;
    btrt: Mp4AtomBtrt | undefined;
    silb: Mp4AtomSilb | undefined;
};
export declare function debug(self: Mp4AtomEvte): string;
export declare function eq(a: Mp4AtomEvte, b: Mp4AtomEvte): boolean;

// Mp4AtomExplicitSpeakerPosition.ts
export type Mp4AtomExplicitSpeakerPosition = {
    azimuth: number;
    elevation: number;
};
export declare function debug(self: Mp4AtomExplicitSpeakerPosition): string;
export declare function eq(a: Mp4AtomExplicitSpeakerPosition, b: Mp4AtomExplicitSpeakerPosition): boolean;

// Mp4AtomExtendedType.ts
import * as rt from "./rt.ts";
export type Mp4AtomExtendedType = rt.Opaque<"Mp4AtomExtendedType">;
export declare function toString(self: Mp4AtomExtendedType): string;
export declare function debug(self: Mp4AtomExtendedType): string;
export declare function eq(a: Mp4AtomExtendedType, b: Mp4AtomExtendedType): boolean;
export declare function default$(): Mp4AtomExtendedType;
export declare function new$(value: Uint8Array): Mp4AtomExtendedType;
export { default$ as default, new$ as new };

// Mp4AtomEyes.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomCams } from "./Mp4AtomCams.ts";
import type { Mp4AtomCmfy } from "./Mp4AtomCmfy.ts";
import type { Mp4AtomHero } from "./Mp4AtomHero.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
import type { Mp4AtomStri } from "./Mp4AtomStri.ts";
export type Mp4AtomEyes = rt.Handle<"Mp4AtomEyes">;
export declare function debug(self: Mp4AtomEyes): string;
export declare function clone(self: Mp4AtomEyes): Mp4AtomEyes;
export declare function eq(a: Mp4AtomEyes, b: Mp4AtomEyes): boolean;
export declare function default$(): Mp4AtomEyes;
export declare function mustGet(self: Mp4AtomEyes): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomEyes, value: Mp4AtomMust | undefined): void;
export declare function striGet(self: Mp4AtomEyes): Mp4AtomStri | undefined;
export declare function striSet(self: Mp4AtomEyes, value: Mp4AtomStri | undefined): void;
export declare function heroGet(self: Mp4AtomEyes): Mp4AtomHero | undefined;
export declare function heroSet(self: Mp4AtomEyes, value: Mp4AtomHero | undefined): void;
export declare function camsGet(self: Mp4AtomEyes): Mp4AtomCams | undefined;
export declare function camsSet(self: Mp4AtomEyes, value: Mp4AtomCams | undefined): void;
export declare function cmfyGet(self: Mp4AtomEyes): Mp4AtomCmfy | undefined;
export declare function cmfySet(self: Mp4AtomEyes, value: Mp4AtomCmfy | undefined): void;
export declare function unknownGet(self: Mp4AtomEyes): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomEyes, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomFiel.ts
import * as rt from "./rt.ts";
export type Mp4AtomFiel = {
    fieldCount: number;
    fieldOrder: number;
};
export declare function debug(self: Mp4AtomFiel): string;
export declare function eq(a: Mp4AtomFiel, b: Mp4AtomFiel): boolean;
export declare function default$(): Mp4AtomFiel;
export declare function new$(fieldCount: number, fieldOrder: number): rt.Result<Mp4AtomFiel, string>;
export { default$ as default, new$ as new };

// Mp4AtomFixedPointI16.ts
import * as rt from "./rt.ts";
export type Mp4AtomFixedPointI16 = rt.Opaque<"Mp4AtomFixedPointI16">;
export declare function debug(self: Mp4AtomFixedPointI16): string;
export declare function eq(a: Mp4AtomFixedPointI16, b: Mp4AtomFixedPointI16): boolean;
export declare function default$(): Mp4AtomFixedPointI16;
export declare function new$(int: number, dec: number): Mp4AtomFixedPointI16;
export declare function integer(self: Mp4AtomFixedPointI16): number;
export declare function decimal(self: Mp4AtomFixedPointI16): number;
export { default$ as default, new$ as new };

// Mp4AtomFixedPointI8.ts
import * as rt from "./rt.ts";
export type Mp4AtomFixedPointI8 = rt.Opaque<"Mp4AtomFixedPointI8">;
export declare function debug(self: Mp4AtomFixedPointI8): string;
export declare function eq(a: Mp4AtomFixedPointI8, b: Mp4AtomFixedPointI8): boolean;
export declare function default$(): Mp4AtomFixedPointI8;
export declare function new$(int: number, dec: number): Mp4AtomFixedPointI8;
export declare function integer(self: Mp4AtomFixedPointI8): number;
export declare function decimal(self: Mp4AtomFixedPointI8): number;
export { default$ as default, new$ as new };

// Mp4AtomFixedPointU16.ts
import * as rt from "./rt.ts";
export type Mp4AtomFixedPointU16 = rt.Opaque<"Mp4AtomFixedPointU16">;
export declare function debug(self: Mp4AtomFixedPointU16): string;
export declare function eq(a: Mp4AtomFixedPointU16, b: Mp4AtomFixedPointU16): boolean;
export declare function default$(): Mp4AtomFixedPointU16;
export declare function new$(int: number, dec: number): Mp4AtomFixedPointU16;
export declare function integer(self: Mp4AtomFixedPointU16): number;
export declare function decimal(self: Mp4AtomFixedPointU16): number;
export { default$ as default, new$ as new };

// Mp4AtomFixedPointU8.ts
import * as rt from "./rt.ts";
export type Mp4AtomFixedPointU8 = rt.Opaque<"Mp4AtomFixedPointU8">;
export declare function debug(self: Mp4AtomFixedPointU8): string;
export declare function eq(a: Mp4AtomFixedPointU8, b: Mp4AtomFixedPointU8): boolean;
export declare function default$(): Mp4AtomFixedPointU8;
export declare function new$(int: number, dec: number): Mp4AtomFixedPointU8;
export declare function integer(self: Mp4AtomFixedPointU8): number;
export declare function decimal(self: Mp4AtomFixedPointU8): number;
export { default$ as default, new$ as new };

// Mp4AtomFl32.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomFl32 = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomFl32): string;
export declare function eq(a: Mp4AtomFl32, b: Mp4AtomFl32): boolean;
export declare function format(self: Mp4AtomFl32): Mp4AtomPcmFormat | undefined;

// Mp4AtomFl64.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomFl64 = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomFl64): string;
export declare function eq(a: Mp4AtomFl64, b: Mp4AtomFl64): boolean;
export declare function format(self: Mp4AtomFl64): Mp4AtomPcmFormat | undefined;

// Mp4AtomFlac.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomDfla } from "./Mp4AtomDfla.ts";
export type Mp4AtomFlac = {
    audio: Mp4AtomAudio;
    dfla: Mp4AtomDfla;
};
export declare function debug(self: Mp4AtomFlac): string;
export declare function eq(a: Mp4AtomFlac, b: Mp4AtomFlac): boolean;

// Mp4AtomFlacMetadataBlock.ts
import * as rt from "./rt.ts";
import type { Mp4AtomU24 } from "./Mp4AtomU24.ts";
export type Mp4AtomFlacMetadataBlock = {
    tag: "StreamInfo";
    minimumBlockSize: number;
    maximumBlockSize: number;
    minimumFrameSize: Mp4AtomU24;
    maximumFrameSize: Mp4AtomU24;
    sampleRate: number;
    numChannelsMinusOne: number;
    bitsPerSampleMinusOne: number;
    numberOfInterchannelSamples: rt.u64;
    md5Checksum: Uint8Array;
} | {
    tag: "Padding";
} | {
    tag: "Application";
} | {
    tag: "SeekTable";
} | {
    tag: "VorbisComment";
    vendorString: string;
    comments: string[];
} | {
    tag: "CueSheet";
} | {
    tag: "Picture";
} | {
    tag: "Reserved";
} | {
    tag: "Forbidden";
};
export declare function debug(self: Mp4AtomFlacMetadataBlock): string;
export declare function eq(a: Mp4AtomFlacMetadataBlock, b: Mp4AtomFlacMetadataBlock): boolean;

// Mp4AtomFontEntry.ts
export type Mp4AtomFontEntry = {
    fontId: number;
    font: string;
};
export declare function debug(self: Mp4AtomFontEntry): string;
export declare function eq(a: Mp4AtomFontEntry, b: Mp4AtomFontEntry): boolean;
export declare function default$(): Mp4AtomFontEntry;
export { default$ as default };

// Mp4AtomFourCc.ts
import * as rt from "./rt.ts";
export type Mp4AtomFourCc = rt.Opaque<"Mp4AtomFourCc">;
export declare function toString(self: Mp4AtomFourCc): string;
export declare function debug(self: Mp4AtomFourCc): string;
export declare function eq(a: Mp4AtomFourCc, b: Mp4AtomFourCc): boolean;
export declare function hash(self: Mp4AtomFourCc): rt.u64;
export declare function new$(value: Uint8Array): Mp4AtomFourCc;
export { new$ as new };

// Mp4AtomFpcm.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomFpcm = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomFpcm): string;
export declare function eq(a: Mp4AtomFpcm, b: Mp4AtomFpcm): boolean;
export declare function format(self: Mp4AtomFpcm): Mp4AtomPcmFormat | undefined;

// Mp4AtomFragmentInfo.ts
import * as rt from "./rt.ts";
export type Mp4AtomFragmentInfo = {
    time: rt.u64;
    moofOffset: rt.u64;
    trafNumber: number;
    trunNumber: number;
    sampleDelta: number;
};
export declare function debug(self: Mp4AtomFragmentInfo): string;
export declare function eq(a: Mp4AtomFragmentInfo, b: Mp4AtomFragmentInfo): boolean;
export declare function default$(): Mp4AtomFragmentInfo;
export { default$ as default };

// Mp4AtomFree.ts
import type { Mp4AtomZeroed } from "./Mp4AtomZeroed.ts";
export type Mp4AtomFree = {
    zeroed: Mp4AtomZeroed;
};
export declare function debug(self: Mp4AtomFree): string;
export declare function eq(a: Mp4AtomFree, b: Mp4AtomFree): boolean;

// Mp4AtomFtab.ts
import type { Mp4AtomFontEntry } from "./Mp4AtomFontEntry.ts";
export type Mp4AtomFtab = {
    fontEntries: Mp4AtomFontEntry[];
};
export declare function debug(self: Mp4AtomFtab): string;
export declare function eq(a: Mp4AtomFtab, b: Mp4AtomFtab): boolean;
export declare function default$(): Mp4AtomFtab;
export { default$ as default };

// Mp4AtomFtyp.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomFtyp = {
    majorBrand: Mp4AtomFourCc;
    minorVersion: number;
    compatibleBrands: Mp4AtomFourCc[];
};
export declare function debug(self: Mp4AtomFtyp): string;
export declare function eq(a: Mp4AtomFtyp, b: Mp4AtomFtyp): boolean;

// Mp4AtomHdlr.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomHdlr = {
    handler: Mp4AtomFourCc;
    name: string;
};
export declare function debug(self: Mp4AtomHdlr): string;
export declare function eq(a: Mp4AtomHdlr, b: Mp4AtomHdlr): boolean;
export declare function default$(): Mp4AtomHdlr;
export { default$ as default };

// Mp4AtomHero.ts
export type Mp4AtomHero = {
    heroEyeIndicator: number;
};
export declare function debug(self: Mp4AtomHero): string;
export declare function eq(a: Mp4AtomHero, b: Mp4AtomHero): boolean;
export declare function default$(): Mp4AtomHero;
export { default$ as default };

// Mp4AtomHev1.ts
import * as rt from "./rt.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomHvcc } from "./Mp4AtomHvcc.ts";
import type { Mp4AtomLhvc } from "./Mp4AtomLhvc.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomHev1 = rt.Handle<"Mp4AtomHev1">;
export declare function debug(self: Mp4AtomHev1): string;
export declare function clone(self: Mp4AtomHev1): Mp4AtomHev1;
export declare function eq(a: Mp4AtomHev1, b: Mp4AtomHev1): boolean;
export declare function default$(): Mp4AtomHev1;
export declare function visualGet(self: Mp4AtomHev1): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomHev1, value: Mp4AtomVisual): void;
export declare function hvccGet(self: Mp4AtomHev1): Mp4AtomHvcc;
export declare function hvccSet(self: Mp4AtomHev1, value: Mp4AtomHvcc): void;
export declare function lhvcGet(self: Mp4AtomHev1): Mp4AtomLhvc | undefined;
export declare function lhvcSet(self: Mp4AtomHev1, value: Mp4AtomLhvc | undefined): void;
export declare function btrtGet(self: Mp4AtomHev1): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomHev1, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomHev1): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomHev1, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomHev1): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomHev1, value: Mp4AtomPasp | undefined): void;
export declare function taicGet(self: Mp4AtomHev1): Mp4AtomTaic | undefined;
export declare function taicSet(self: Mp4AtomHev1, value: Mp4AtomTaic | undefined): void;
export declare function fielGet(self: Mp4AtomHev1): Mp4AtomFiel | undefined;
export declare function fielSet(self: Mp4AtomHev1, value: Mp4AtomFiel | undefined): void;
export declare function spatialGet(self: Mp4AtomHev1): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomHev1, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomHfov.ts
export type Mp4AtomHfov = {
    fieldOfView: number;
};
export declare function debug(self: Mp4AtomHfov): string;
export declare function eq(a: Mp4AtomHfov, b: Mp4AtomHfov): boolean;
export declare function default$(): Mp4AtomHfov;
export { default$ as default };

// Mp4AtomHmhd.ts
export type Mp4AtomHmhd = {
    maxPduSize: number;
    avgPduSize: number;
    maxBitrate: number;
    avgBitrate: number;
};
export declare function debug(self: Mp4AtomHmhd): string;
export declare function eq(a: Mp4AtomHmhd, b: Mp4AtomHmhd): boolean;
export declare function default$(): Mp4AtomHmhd;
export { default$ as default };

// Mp4AtomHvc1.ts
import * as rt from "./rt.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomCcst } from "./Mp4AtomCcst.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomHvcc } from "./Mp4AtomHvcc.ts";
import type { Mp4AtomLhvc } from "./Mp4AtomLhvc.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomHvc1 = rt.Handle<"Mp4AtomHvc1">;
export declare function debug(self: Mp4AtomHvc1): string;
export declare function clone(self: Mp4AtomHvc1): Mp4AtomHvc1;
export declare function eq(a: Mp4AtomHvc1, b: Mp4AtomHvc1): boolean;
export declare function default$(): Mp4AtomHvc1;
export declare function visualGet(self: Mp4AtomHvc1): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomHvc1, value: Mp4AtomVisual): void;
export declare function hvccGet(self: Mp4AtomHvc1): Mp4AtomHvcc;
export declare function hvccSet(self: Mp4AtomHvc1, value: Mp4AtomHvcc): void;
export declare function lhvcGet(self: Mp4AtomHvc1): Mp4AtomLhvc | undefined;
export declare function lhvcSet(self: Mp4AtomHvc1, value: Mp4AtomLhvc | undefined): void;
export declare function btrtGet(self: Mp4AtomHvc1): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomHvc1, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomHvc1): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomHvc1, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomHvc1): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomHvc1, value: Mp4AtomPasp | undefined): void;
export declare function taicGet(self: Mp4AtomHvc1): Mp4AtomTaic | undefined;
export declare function taicSet(self: Mp4AtomHvc1, value: Mp4AtomTaic | undefined): void;
export declare function fielGet(self: Mp4AtomHvc1): Mp4AtomFiel | undefined;
export declare function fielSet(self: Mp4AtomHvc1, value: Mp4AtomFiel | undefined): void;
export declare function ccstGet(self: Mp4AtomHvc1): Mp4AtomCcst | undefined;
export declare function ccstSet(self: Mp4AtomHvc1, value: Mp4AtomCcst | undefined): void;
export declare function spatialGet(self: Mp4AtomHvc1): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomHvc1, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomHvcCArray.ts
export type Mp4AtomHvcCArray = {
    completeness: boolean;
    nalUnitType: number;
    nalus: Uint8Array[];
};
export declare function debug(self: Mp4AtomHvcCArray): string;
export declare function eq(a: Mp4AtomHvcCArray, b: Mp4AtomHvcCArray): boolean;
export declare function default$(): Mp4AtomHvcCArray;
export { default$ as default };

// Mp4AtomHvcc.ts
import type { Mp4AtomHvcCArray } from "./Mp4AtomHvcCArray.ts";
export type Mp4AtomHvcc = {
    configurationVersion: number;
    generalProfileSpace: number;
    generalTierFlag: boolean;
    generalProfileIdc: number;
    generalProfileCompatibilityFlags: Uint8Array;
    generalConstraintIndicatorFlags: Uint8Array;
    generalLevelIdc: number;
    minSpatialSegmentationIdc: number;
    parallelismType: number;
    chromaFormatIdc: number;
    bitDepthLumaMinus8: number;
    bitDepthChromaMinus8: number;
    avgFrameRate: number;
    constantFrameRate: number;
    numTemporalLayers: number;
    temporalIdNested: boolean;
    lengthSizeMinusOne: number;
    arrays: Mp4AtomHvcCArray[];
};
export declare function debug(self: Mp4AtomHvcc): string;
export declare function eq(a: Mp4AtomHvcc, b: Mp4AtomHvcc): boolean;
export declare function default$(): Mp4AtomHvcc;
export declare function new$(): Mp4AtomHvcc;
export { default$ as default, new$ as new };

// Mp4AtomIdat.ts
export type Mp4AtomIdat = {
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomIdat): string;
export declare function eq(a: Mp4AtomIdat, b: Mp4AtomIdat): boolean;

// Mp4AtomIinf.ts
import type { Mp4AtomItemInfoEntry } from "./Mp4AtomItemInfoEntry.ts";
export type Mp4AtomIinf = {
    itemInfos: Mp4AtomItemInfoEntry[];
};
export declare function debug(self: Mp4AtomIinf): string;
export declare function eq(a: Mp4AtomIinf, b: Mp4AtomIinf): boolean;

// Mp4AtomIloc.ts
import type { Mp4AtomItemLocation } from "./Mp4AtomItemLocation.ts";
export type Mp4AtomIloc = {
    itemLocations: Mp4AtomItemLocation[];
};
export declare function debug(self: Mp4AtomIloc): string;
export declare function eq(a: Mp4AtomIloc, b: Mp4AtomIloc): boolean;

// Mp4AtomIlst.ts
import type { Mp4AtomCopyright } from "./Mp4AtomCopyright.ts";
import type { Mp4AtomCovr } from "./Mp4AtomCovr.ts";
import type { Mp4AtomDesc } from "./Mp4AtomDesc.ts";
import type { Mp4AtomIlstData } from "./Mp4AtomIlstData.ts";
import type { Mp4AtomName } from "./Mp4AtomName.ts";
import type { Mp4AtomTool } from "./Mp4AtomTool.ts";
import type { Mp4AtomYear } from "./Mp4AtomYear.ts";
export type Mp4AtomIlst = {
    name: Mp4AtomName | undefined;
    year: Mp4AtomYear | undefined;
    covr: Mp4AtomCovr | undefined;
    desc: Mp4AtomDesc | undefined;
    ctoo: Mp4AtomTool | undefined;
    cprt: Mp4AtomCopyright | undefined;
    mdta: ([number, Mp4AtomIlstData])[];
};
export declare function debug(self: Mp4AtomIlst): string;
export declare function eq(a: Mp4AtomIlst, b: Mp4AtomIlst): boolean;
export declare function default$(): Mp4AtomIlst;
export { default$ as default };

// Mp4AtomIlstData.ts
import type { Mp4AtomIlstDataValue } from "./Mp4AtomIlstDataValue.ts";
export type Mp4AtomIlstData = {
    countryIndicator: number;
    languageIndicator: number;
    value: Mp4AtomIlstDataValue;
};
export declare function debug(self: Mp4AtomIlstData): string;
export declare function eq(a: Mp4AtomIlstData, b: Mp4AtomIlstData): boolean;

// Mp4AtomIlstDataValue.ts
import * as rt from "./rt.ts";
export type Mp4AtomIlstDataValue = {
    tag: "Reserved";
    _0: Uint8Array;
} | {
    tag: "Utf8";
    _0: string;
} | {
    tag: "Utf16";
    _0: string;
} | {
    tag: "Jpeg";
    _0: Uint8Array;
} | {
    tag: "Png";
    _0: Uint8Array;
} | {
    tag: "Bmp";
    _0: Uint8Array;
} | {
    tag: "BeSignedInt";
    _0: rt.i64;
} | {
    tag: "BeUnsignedInt";
    _0: rt.u64;
} | {
    tag: "BeFloat32";
    _0: number;
} | {
    tag: "BeFloat64";
    _0: number;
} | {
    tag: "Unknown";
    _0: number;
    _1: Uint8Array;
};
export declare function debug(self: Mp4AtomIlstDataValue): string;
export declare function eq(a: Mp4AtomIlstDataValue, b: Mp4AtomIlstDataValue): boolean;

// Mp4AtomImir.ts
export type Mp4AtomImir = {
    axis: number;
};
export declare function debug(self: Mp4AtomImir): string;
export declare function eq(a: Mp4AtomImir, b: Mp4AtomImir): boolean;
export declare function default$(): Mp4AtomImir;
export { default$ as default };

// Mp4AtomIn24.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomIn24 = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomIn24): string;
export declare function eq(a: Mp4AtomIn24, b: Mp4AtomIn24): boolean;
export declare function format(self: Mp4AtomIn24): Mp4AtomPcmFormat | undefined;

// Mp4AtomIn32.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomIn32 = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomIn32): string;
export declare function eq(a: Mp4AtomIn32, b: Mp4AtomIn32): boolean;
export declare function format(self: Mp4AtomIn32): Mp4AtomPcmFormat | undefined;

// Mp4AtomIpcm.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomIpcm = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomIpcm): string;
export declare function eq(a: Mp4AtomIpcm, b: Mp4AtomIpcm): boolean;
export declare function format(self: Mp4AtomIpcm): Mp4AtomPcmFormat | undefined;

// Mp4AtomIpco.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
export type Mp4AtomIpco = rt.Handle<"Mp4AtomIpco">;
export declare function debug(self: Mp4AtomIpco): string;
export declare function clone(self: Mp4AtomIpco): Mp4AtomIpco;
export declare function eq(a: Mp4AtomIpco, b: Mp4AtomIpco): boolean;
export declare function default$(): Mp4AtomIpco;
export declare function propertiesGet(self: Mp4AtomIpco): Mp4AtomAny[];
export declare function propertiesSet(self: Mp4AtomIpco, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomIpma.ts
import type { Mp4AtomPropertyAssociations } from "./Mp4AtomPropertyAssociations.ts";
export type Mp4AtomIpma = {
    itemProperties: Mp4AtomPropertyAssociations[];
};
export declare function debug(self: Mp4AtomIpma): string;
export declare function eq(a: Mp4AtomIpma, b: Mp4AtomIpma): boolean;
export declare function default$(): Mp4AtomIpma;
export { default$ as default };

// Mp4AtomIprp.ts
import * as rt from "./rt.ts";
import type { Mp4AtomIpco } from "./Mp4AtomIpco.ts";
import type { Mp4AtomIpma } from "./Mp4AtomIpma.ts";
export type Mp4AtomIprp = rt.Handle<"Mp4AtomIprp">;
export declare function debug(self: Mp4AtomIprp): string;
export declare function clone(self: Mp4AtomIprp): Mp4AtomIprp;
export declare function eq(a: Mp4AtomIprp, b: Mp4AtomIprp): boolean;
export declare function default$(): Mp4AtomIprp;
export declare function ipcoGet(self: Mp4AtomIprp): Mp4AtomIpco;
export declare function ipcoSet(self: Mp4AtomIprp, value: Mp4AtomIpco): void;
export declare function ipmaGet(self: Mp4AtomIprp): Mp4AtomIpma[];
export declare function ipmaSet(self: Mp4AtomIprp, value: Mp4AtomIpma[]): void;
export { default$ as default };

// Mp4AtomIref.ts
import type { Mp4AtomReference } from "./Mp4AtomReference.ts";
export type Mp4AtomIref = {
    references: Mp4AtomReference[];
};
export declare function debug(self: Mp4AtomIref): string;
export declare function eq(a: Mp4AtomIref, b: Mp4AtomIref): boolean;

// Mp4AtomIrot.ts
export type Mp4AtomIrot = {
    angle: number;
};
export declare function debug(self: Mp4AtomIrot): string;
export declare function eq(a: Mp4AtomIrot, b: Mp4AtomIrot): boolean;
export declare function default$(): Mp4AtomIrot;
export { default$ as default };

// Mp4AtomIscl.ts
export type Mp4AtomIscl = {
    targetWidthNumerator: number;
    targetWidthDenominator: number;
    targetHeightNumerator: number;
    targetHeightDenominator: number;
};
export declare function debug(self: Mp4AtomIscl): string;
export declare function eq(a: Mp4AtomIscl, b: Mp4AtomIscl): boolean;
export declare function default$(): Mp4AtomIscl;
export { default$ as default };

// Mp4AtomIspe.ts
export type Mp4AtomIspe = {
    width: number;
    height: number;
};
export declare function debug(self: Mp4AtomIspe): string;
export declare function eq(a: Mp4AtomIspe, b: Mp4AtomIspe): boolean;
export declare function default$(): Mp4AtomIspe;
export { default$ as default };

// Mp4AtomItemInfoEntry.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomItemInfoEntry = {
    itemId: number;
    itemProtectionIndex: number;
    itemType: Mp4AtomFourCc | undefined;
    itemName: string;
    contentType: string | undefined;
    contentEncoding: string | undefined;
    itemUriType: string | undefined;
    itemNotInPresentation: boolean;
};
export declare function debug(self: Mp4AtomItemInfoEntry): string;
export declare function eq(a: Mp4AtomItemInfoEntry, b: Mp4AtomItemInfoEntry): boolean;

// Mp4AtomItemLocation.ts
import * as rt from "./rt.ts";
import type { Mp4AtomItemLocationExtent } from "./Mp4AtomItemLocationExtent.ts";
export type Mp4AtomItemLocation = {
    itemId: number;
    constructionMethod: number;
    dataReferenceIndex: number;
    baseOffset: rt.u64;
    extents: Mp4AtomItemLocationExtent[];
};
export declare function debug(self: Mp4AtomItemLocation): string;
export declare function eq(a: Mp4AtomItemLocation, b: Mp4AtomItemLocation): boolean;

// Mp4AtomItemLocationExtent.ts
import * as rt from "./rt.ts";
export type Mp4AtomItemLocationExtent = {
    itemReferenceIndex: rt.u64;
    offset: rt.u64;
    length: rt.u64;
};
export declare function debug(self: Mp4AtomItemLocationExtent): string;
export declare function eq(a: Mp4AtomItemLocationExtent, b: Mp4AtomItemLocationExtent): boolean;

// Mp4AtomJpeg.ts
import * as rt from "./rt.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
export type Mp4AtomJpeg = rt.Handle<"Mp4AtomJpeg">;
export declare function debug(self: Mp4AtomJpeg): string;
export declare function clone(self: Mp4AtomJpeg): Mp4AtomJpeg;
export declare function eq(a: Mp4AtomJpeg, b: Mp4AtomJpeg): boolean;
export declare function default$(): Mp4AtomJpeg;
export declare function visualGet(self: Mp4AtomJpeg): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomJpeg, value: Mp4AtomVisual): void;
export declare function btrtGet(self: Mp4AtomJpeg): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomJpeg, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomJpeg): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomJpeg, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomJpeg): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomJpeg, value: Mp4AtomPasp | undefined): void;
export declare function fielGet(self: Mp4AtomJpeg): Mp4AtomFiel | undefined;
export declare function fielSet(self: Mp4AtomJpeg, value: Mp4AtomFiel | undefined): void;
export declare function spatialGet(self: Mp4AtomJpeg): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomJpeg, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomKeyEntry.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomKeyEntry = {
    keyNamespace: Mp4AtomFourCc;
    keyValue: string;
};
export declare function debug(self: Mp4AtomKeyEntry): string;
export declare function eq(a: Mp4AtomKeyEntry, b: Mp4AtomKeyEntry): boolean;

// Mp4AtomKeyd.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomKeyd = {
    keyNamespace: Mp4AtomFourCc;
    keyValue: Uint8Array;
};
export declare function debug(self: Mp4AtomKeyd): string;
export declare function eq(a: Mp4AtomKeyd, b: Mp4AtomKeyd): boolean;

// Mp4AtomKeys.ts
import type { Mp4AtomKeyEntry } from "./Mp4AtomKeyEntry.ts";
export type Mp4AtomKeys = {
    entries: Mp4AtomKeyEntry[];
};
export declare function debug(self: Mp4AtomKeys): string;
export declare function eq(a: Mp4AtomKeys, b: Mp4AtomKeys): boolean;
export declare function default$(): Mp4AtomKeys;
export { default$ as default };

// Mp4AtomKind.ts
export type Mp4AtomKind = {
    schemeUri: string;
    value: string;
};
export declare function debug(self: Mp4AtomKind): string;
export declare function eq(a: Mp4AtomKind, b: Mp4AtomKind): boolean;
export declare function default$(): Mp4AtomKind;
export { default$ as default };

// Mp4AtomLhvc.ts
import type { Mp4AtomHvcCArray } from "./Mp4AtomHvcCArray.ts";
export type Mp4AtomLhvc = {
    configurationVersion: number;
    minSpatialSegmentationIdc: number;
    parallelismType: number;
    numTemporalLayers: number;
    temporalIdNested: boolean;
    lengthSizeMinusOne: number;
    arrays: Mp4AtomHvcCArray[];
};
export declare function debug(self: Mp4AtomLhvc): string;
export declare function eq(a: Mp4AtomLhvc, b: Mp4AtomLhvc): boolean;
export declare function default$(): Mp4AtomLhvc;
export { default$ as default };

// Mp4AtomLoca.ts
export type Mp4AtomLoca = {
    locale: string;
};
export declare function debug(self: Mp4AtomLoca): string;
export declare function eq(a: Mp4AtomLoca, b: Mp4AtomLoca): boolean;

// Mp4AtomLpcm.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomLpcm = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomLpcm): string;
export declare function eq(a: Mp4AtomLpcm, b: Mp4AtomLpcm): boolean;
export declare function format(self: Mp4AtomLpcm): Mp4AtomPcmFormat | undefined;

// Mp4AtomMatrix.ts
export type Mp4AtomMatrix = {
    a: number;
    b: number;
    u: number;
    c: number;
    d: number;
    v: number;
    x: number;
    y: number;
    w: number;
};
export declare function toString(self: Mp4AtomMatrix): string;
export declare function debug(self: Mp4AtomMatrix): string;
export declare function eq(a: Mp4AtomMatrix, b: Mp4AtomMatrix): boolean;
export declare function default$(): Mp4AtomMatrix;
export { default$ as default };

// Mp4AtomMdat.ts
export type Mp4AtomMdat = {
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomMdat): string;
export declare function eq(a: Mp4AtomMdat, b: Mp4AtomMdat): boolean;

// Mp4AtomMdhd.ts
import * as rt from "./rt.ts";
export type Mp4AtomMdhd = {
    creationTime: rt.u64;
    modificationTime: rt.u64;
    timescale: number;
    duration: rt.u64;
    language: string;
};
export declare function debug(self: Mp4AtomMdhd): string;
export declare function eq(a: Mp4AtomMdhd, b: Mp4AtomMdhd): boolean;
export declare function default$(): Mp4AtomMdhd;
export { default$ as default };

// Mp4AtomMdia.ts
import * as rt from "./rt.ts";
import type { Mp4AtomHdlr } from "./Mp4AtomHdlr.ts";
import type { Mp4AtomMdhd } from "./Mp4AtomMdhd.ts";
import type { Mp4AtomMinf } from "./Mp4AtomMinf.ts";
export type Mp4AtomMdia = rt.Handle<"Mp4AtomMdia">;
export declare function debug(self: Mp4AtomMdia): string;
export declare function clone(self: Mp4AtomMdia): Mp4AtomMdia;
export declare function eq(a: Mp4AtomMdia, b: Mp4AtomMdia): boolean;
export declare function default$(): Mp4AtomMdia;
export declare function mdhdGet(self: Mp4AtomMdia): Mp4AtomMdhd;
export declare function mdhdSet(self: Mp4AtomMdia, value: Mp4AtomMdhd): void;
export declare function hdlrGet(self: Mp4AtomMdia): Mp4AtomHdlr;
export declare function hdlrSet(self: Mp4AtomMdia, value: Mp4AtomHdlr): void;
export declare function minfGet(self: Mp4AtomMdia): Mp4AtomMinf;
export declare function minfSet(self: Mp4AtomMdia, value: Mp4AtomMinf): void;
export { default$ as default };

// Mp4AtomMebx.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomMebxKey } from "./Mp4AtomMebxKey.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
export type Mp4AtomMebx = {
    metadata: Mp4AtomMetaData;
    keys: Mp4AtomMebxKey[];
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomMebx): string;
export declare function eq(a: Mp4AtomMebx, b: Mp4AtomMebx): boolean;

// Mp4AtomMebxKey.ts
import type { Mp4AtomDtyp } from "./Mp4AtomDtyp.ts";
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomKeyd } from "./Mp4AtomKeyd.ts";
import type { Mp4AtomLoca } from "./Mp4AtomLoca.ts";
import type { Mp4AtomSetu } from "./Mp4AtomSetu.ts";
export type Mp4AtomMebxKey = {
    localKeyId: Mp4AtomFourCc;
    keyd: Mp4AtomKeyd;
    dtyp: Mp4AtomDtyp | undefined;
    loca: Mp4AtomLoca | undefined;
    setu: Mp4AtomSetu | undefined;
};
export declare function debug(self: Mp4AtomMebxKey): string;
export declare function eq(a: Mp4AtomMebxKey, b: Mp4AtomMebxKey): boolean;

// Mp4AtomMehd.ts
import * as rt from "./rt.ts";
export type Mp4AtomMehd = {
    fragmentDuration: rt.u64;
};
export declare function debug(self: Mp4AtomMehd): string;
export declare function eq(a: Mp4AtomMehd, b: Mp4AtomMehd): boolean;
export declare function default$(): Mp4AtomMehd;
export { default$ as default };

// Mp4AtomMeta.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomDinf } from "./Mp4AtomDinf.ts";
import type { Mp4AtomHdlr } from "./Mp4AtomHdlr.ts";
import type { Mp4AtomIdat } from "./Mp4AtomIdat.ts";
import type { Mp4AtomIinf } from "./Mp4AtomIinf.ts";
import type { Mp4AtomIloc } from "./Mp4AtomIloc.ts";
import type { Mp4AtomIlst } from "./Mp4AtomIlst.ts";
import type { Mp4AtomIprp } from "./Mp4AtomIprp.ts";
import type { Mp4AtomIref } from "./Mp4AtomIref.ts";
import type { Mp4AtomKeys } from "./Mp4AtomKeys.ts";
import type { Mp4AtomPitm } from "./Mp4AtomPitm.ts";
export type Mp4AtomMeta = rt.Handle<"Mp4AtomMeta">;
export declare function debug(self: Mp4AtomMeta): string;
export declare function clone(self: Mp4AtomMeta): Mp4AtomMeta;
export declare function eq(a: Mp4AtomMeta, b: Mp4AtomMeta): boolean;
export declare function hdlrGet(self: Mp4AtomMeta): Mp4AtomHdlr;
export declare function hdlrSet(self: Mp4AtomMeta, value: Mp4AtomHdlr): void;
export declare function itemsGet(self: Mp4AtomMeta): Mp4AtomAny[];
export declare function itemsSet(self: Mp4AtomMeta, value: Mp4AtomAny[]): void;
export declare function getPitm(self: Mp4AtomMeta): Mp4AtomPitm | undefined;
export declare function getDinf(self: Mp4AtomMeta): Mp4AtomDinf | undefined;
export declare function getIloc(self: Mp4AtomMeta): Mp4AtomIloc | undefined;
export declare function getIinf(self: Mp4AtomMeta): Mp4AtomIinf | undefined;
export declare function getIprp(self: Mp4AtomMeta): Mp4AtomIprp | undefined;
export declare function getIref(self: Mp4AtomMeta): Mp4AtomIref | undefined;
export declare function getIdat(self: Mp4AtomMeta): Mp4AtomIdat | undefined;
export declare function getIlst(self: Mp4AtomMeta): Mp4AtomIlst | undefined;
export declare function getKeys(self: Mp4AtomMeta): Mp4AtomKeys | undefined;
export declare function getMutPitm(self: Mp4AtomMeta): Mp4AtomPitm | undefined;
export declare function getMutDinf(self: Mp4AtomMeta): Mp4AtomDinf | undefined;
export declare function getMutIloc(self: Mp4AtomMeta): Mp4AtomIloc | undefined;
export declare function getMutIinf(self: Mp4AtomMeta): Mp4AtomIinf | undefined;
export declare function getMutIprp(self: Mp4AtomMeta): Mp4AtomIprp | undefined;
export declare function getMutIref(self: Mp4AtomMeta): Mp4AtomIref | undefined;
export declare function getMutIdat(self: Mp4AtomMeta): Mp4AtomIdat | undefined;
export declare function getMutIlst(self: Mp4AtomMeta): Mp4AtomIlst | undefined;
export declare function getMutKeys(self: Mp4AtomMeta): Mp4AtomKeys | undefined;
export declare function pushPitm(self: Mp4AtomMeta, atom: Mp4AtomPitm): void;
export declare function pushDinf(self: Mp4AtomMeta, atom: Mp4AtomDinf): void;
export declare function pushIloc(self: Mp4AtomMeta, atom: Mp4AtomIloc): void;
export declare function pushIinf(self: Mp4AtomMeta, atom: Mp4AtomIinf): void;
export declare function pushIprp(self: Mp4AtomMeta, atom: Mp4AtomIprp): void;
export declare function pushIref(self: Mp4AtomMeta, atom: Mp4AtomIref): void;
export declare function pushIdat(self: Mp4AtomMeta, atom: Mp4AtomIdat): void;
export declare function pushIlst(self: Mp4AtomMeta, atom: Mp4AtomIlst): void;
export declare function pushKeys(self: Mp4AtomMeta, atom: Mp4AtomKeys): void;
export declare function removePitm(self: Mp4AtomMeta): Mp4AtomPitm | undefined;
export declare function removeDinf(self: Mp4AtomMeta): Mp4AtomDinf | undefined;
export declare function removeIloc(self: Mp4AtomMeta): Mp4AtomIloc | undefined;
export declare function removeIinf(self: Mp4AtomMeta): Mp4AtomIinf | undefined;
export declare function removeIprp(self: Mp4AtomMeta): Mp4AtomIprp | undefined;
export declare function removeIref(self: Mp4AtomMeta): Mp4AtomIref | undefined;
export declare function removeIdat(self: Mp4AtomMeta): Mp4AtomIdat | undefined;
export declare function removeIlst(self: Mp4AtomMeta): Mp4AtomIlst | undefined;
export declare function removeKeys(self: Mp4AtomMeta): Mp4AtomKeys | undefined;

// Mp4AtomMetaData.ts
export type Mp4AtomMetaData = {
    dataReferenceIndex: number;
};
export declare function debug(self: Mp4AtomMetaData): string;
export declare function eq(a: Mp4AtomMetaData, b: Mp4AtomMetaData): boolean;

// Mp4AtomMett.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
import type { Mp4AtomTxtC } from "./Mp4AtomTxtC.ts";
export type Mp4AtomMett = {
    metadata: Mp4AtomMetaData;
    contentEncoding: string;
    mimeFormat: string;
    btrt: Mp4AtomBtrt | undefined;
    txtc: Mp4AtomTxtC | undefined;
};
export declare function debug(self: Mp4AtomMett): string;
export declare function eq(a: Mp4AtomMett, b: Mp4AtomMett): boolean;

// Mp4AtomMetx.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
export type Mp4AtomMetx = {
    metadata: Mp4AtomMetaData;
    contentEncoding: string;
    namespace: string;
    schemaLocation: string;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomMetx): string;
export declare function eq(a: Mp4AtomMetx, b: Mp4AtomMetx): boolean;

// Mp4AtomMfhd.ts
export type Mp4AtomMfhd = {
    sequenceNumber: number;
};
export declare function debug(self: Mp4AtomMfhd): string;
export declare function eq(a: Mp4AtomMfhd, b: Mp4AtomMfhd): boolean;
export declare function default$(): Mp4AtomMfhd;
export { default$ as default };

// Mp4AtomMfra.ts
import type { Mp4AtomMfro } from "./Mp4AtomMfro.ts";
import type { Mp4AtomTfra } from "./Mp4AtomTfra.ts";
export type Mp4AtomMfra = {
    tfra: Mp4AtomTfra[];
    mfro: Mp4AtomMfro;
};
export declare function debug(self: Mp4AtomMfra): string;
export declare function eq(a: Mp4AtomMfra, b: Mp4AtomMfra): boolean;
export declare function default$(): Mp4AtomMfra;
export { default$ as default };

// Mp4AtomMfro.ts
export type Mp4AtomMfro = {
    parentSize: number;
};
export declare function debug(self: Mp4AtomMfro): string;
export declare function eq(a: Mp4AtomMfro, b: Mp4AtomMfro): boolean;
export declare function default$(): Mp4AtomMfro;
export { default$ as default };

// Mp4AtomMinf.ts
import * as rt from "./rt.ts";
import type { Mp4AtomDinf } from "./Mp4AtomDinf.ts";
import type { Mp4AtomHmhd } from "./Mp4AtomHmhd.ts";
import type { Mp4AtomNmhd } from "./Mp4AtomNmhd.ts";
import type { Mp4AtomSmhd } from "./Mp4AtomSmhd.ts";
import type { Mp4AtomStbl } from "./Mp4AtomStbl.ts";
import type { Mp4AtomSthd } from "./Mp4AtomSthd.ts";
import type { Mp4AtomVmhd } from "./Mp4AtomVmhd.ts";
export type Mp4AtomMinf = rt.Handle<"Mp4AtomMinf">;
export declare function debug(self: Mp4AtomMinf): string;
export declare function clone(self: Mp4AtomMinf): Mp4AtomMinf;
export declare function eq(a: Mp4AtomMinf, b: Mp4AtomMinf): boolean;
export declare function default$(): Mp4AtomMinf;
export declare function vmhdGet(self: Mp4AtomMinf): Mp4AtomVmhd | undefined;
export declare function vmhdSet(self: Mp4AtomMinf, value: Mp4AtomVmhd | undefined): void;
export declare function smhdGet(self: Mp4AtomMinf): Mp4AtomSmhd | undefined;
export declare function smhdSet(self: Mp4AtomMinf, value: Mp4AtomSmhd | undefined): void;
export declare function nmhdGet(self: Mp4AtomMinf): Mp4AtomNmhd | undefined;
export declare function nmhdSet(self: Mp4AtomMinf, value: Mp4AtomNmhd | undefined): void;
export declare function sthdGet(self: Mp4AtomMinf): Mp4AtomSthd | undefined;
export declare function sthdSet(self: Mp4AtomMinf, value: Mp4AtomSthd | undefined): void;
export declare function hmhdGet(self: Mp4AtomMinf): Mp4AtomHmhd | undefined;
export declare function hmhdSet(self: Mp4AtomMinf, value: Mp4AtomHmhd | undefined): void;
export declare function dinfGet(self: Mp4AtomMinf): Mp4AtomDinf;
export declare function dinfSet(self: Mp4AtomMinf, value: Mp4AtomDinf): void;
export declare function stblGet(self: Mp4AtomMinf): Mp4AtomStbl;
export declare function stblSet(self: Mp4AtomMinf, value: Mp4AtomStbl): void;
export { default$ as default };

// Mp4AtomMoof.ts
import * as rt from "./rt.ts";
import type { Mp4AtomMfhd } from "./Mp4AtomMfhd.ts";
import type { Mp4AtomTraf } from "./Mp4AtomTraf.ts";
export type Mp4AtomMoof = rt.Handle<"Mp4AtomMoof">;
export declare function debug(self: Mp4AtomMoof): string;
export declare function clone(self: Mp4AtomMoof): Mp4AtomMoof;
export declare function eq(a: Mp4AtomMoof, b: Mp4AtomMoof): boolean;
export declare function default$(): Mp4AtomMoof;
export declare function mfhdGet(self: Mp4AtomMoof): Mp4AtomMfhd;
export declare function mfhdSet(self: Mp4AtomMoof, value: Mp4AtomMfhd): void;
export declare function trafGet(self: Mp4AtomMoof): Mp4AtomTraf[];
export declare function trafSet(self: Mp4AtomMoof, value: Mp4AtomTraf[]): void;
export { default$ as default };

// Mp4AtomMoov.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAinf } from "./Mp4AtomAinf.ts";
import type { Mp4AtomMeta } from "./Mp4AtomMeta.ts";
import type { Mp4AtomMvex } from "./Mp4AtomMvex.ts";
import type { Mp4AtomMvhd } from "./Mp4AtomMvhd.ts";
import type { Mp4AtomTrak } from "./Mp4AtomTrak.ts";
import type { Mp4AtomUdta } from "./Mp4AtomUdta.ts";
export type Mp4AtomMoov = rt.Handle<"Mp4AtomMoov">;
export declare function debug(self: Mp4AtomMoov): string;
export declare function clone(self: Mp4AtomMoov): Mp4AtomMoov;
export declare function eq(a: Mp4AtomMoov, b: Mp4AtomMoov): boolean;
export declare function default$(): Mp4AtomMoov;
export declare function mvhdGet(self: Mp4AtomMoov): Mp4AtomMvhd;
export declare function mvhdSet(self: Mp4AtomMoov, value: Mp4AtomMvhd): void;
export declare function metaGet(self: Mp4AtomMoov): Mp4AtomMeta | undefined;
export declare function metaSet(self: Mp4AtomMoov, value: Mp4AtomMeta | undefined): void;
export declare function mvexGet(self: Mp4AtomMoov): Mp4AtomMvex | undefined;
export declare function mvexSet(self: Mp4AtomMoov, value: Mp4AtomMvex | undefined): void;
export declare function trakGet(self: Mp4AtomMoov): Mp4AtomTrak[];
export declare function trakSet(self: Mp4AtomMoov, value: Mp4AtomTrak[]): void;
export declare function udtaGet(self: Mp4AtomMoov): Mp4AtomUdta | undefined;
export declare function udtaSet(self: Mp4AtomMoov, value: Mp4AtomUdta | undefined): void;
export declare function ainfGet(self: Mp4AtomMoov): Mp4AtomAinf | undefined;
export declare function ainfSet(self: Mp4AtomMoov, value: Mp4AtomAinf | undefined): void;
export { default$ as default };

// Mp4AtomMp3.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
export type Mp4AtomMp3 = {
    audio: Mp4AtomAudio;
};
export declare function debug(self: Mp4AtomMp3): string;
export declare function eq(a: Mp4AtomMp3, b: Mp4AtomMp3): boolean;

// Mp4AtomMp4a.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomEsds } from "./Mp4AtomEsds.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
export type Mp4AtomMp4a = {
    audio: Mp4AtomAudio;
    esds: Mp4AtomEsds;
    btrt: Mp4AtomBtrt | undefined;
    taic: Mp4AtomTaic | undefined;
};
export declare function debug(self: Mp4AtomMp4a): string;
export declare function eq(a: Mp4AtomMp4a, b: Mp4AtomMp4a): boolean;

// Mp4AtomMshp.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomMshp = {
    crc: number;
    encodingFourCc: Mp4AtomFourCc;
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomMshp): string;
export declare function eq(a: Mp4AtomMshp, b: Mp4AtomMshp): boolean;

// Mp4AtomMust.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomMust = {
    requiredBoxTypes: Mp4AtomFourCc[];
};
export declare function debug(self: Mp4AtomMust): string;
export declare function eq(a: Mp4AtomMust, b: Mp4AtomMust): boolean;
export declare function default$(): Mp4AtomMust;
export { default$ as default };

// Mp4AtomMvex.ts
import type { Mp4AtomMehd } from "./Mp4AtomMehd.ts";
import type { Mp4AtomTrex } from "./Mp4AtomTrex.ts";
export type Mp4AtomMvex = {
    mehd: Mp4AtomMehd | undefined;
    trex: Mp4AtomTrex[];
};
export declare function debug(self: Mp4AtomMvex): string;
export declare function eq(a: Mp4AtomMvex, b: Mp4AtomMvex): boolean;
export declare function default$(): Mp4AtomMvex;
export { default$ as default };

// Mp4AtomMvhd.ts
import * as rt from "./rt.ts";
import type { Mp4AtomFixedPointU16 } from "./Mp4AtomFixedPointU16.ts";
import type { Mp4AtomFixedPointU8 } from "./Mp4AtomFixedPointU8.ts";
import type { Mp4AtomMatrix } from "./Mp4AtomMatrix.ts";
export type Mp4AtomMvhd = {
    creationTime: rt.u64;
    modificationTime: rt.u64;
    timescale: number;
    duration: rt.u64;
    rate: Mp4AtomFixedPointU16;
    volume: Mp4AtomFixedPointU8;
    matrix: Mp4AtomMatrix;
    nextTrackId: number;
};
export declare function debug(self: Mp4AtomMvhd): string;
export declare function eq(a: Mp4AtomMvhd, b: Mp4AtomMvhd): boolean;
export declare function default$(): Mp4AtomMvhd;
export { default$ as default };

// Mp4AtomName.ts
export type Mp4AtomName = {
    _0: string;
};
export declare function debug(self: Mp4AtomName): string;
export declare function eq(a: Mp4AtomName, b: Mp4AtomName): boolean;

// Mp4AtomNmhd.ts
import * as rt from "./rt.ts";
export type Mp4AtomNmhd = rt.Handle<"Mp4AtomNmhd">;
export declare function debug(self: Mp4AtomNmhd): string;
export declare function clone(self: Mp4AtomNmhd): Mp4AtomNmhd;
export declare function eq(a: Mp4AtomNmhd, b: Mp4AtomNmhd): boolean;
export declare function default$(): Mp4AtomNmhd;
export { default$ as default };

// Mp4AtomOpus.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomDops } from "./Mp4AtomDops.ts";
export type Mp4AtomOpus = {
    audio: Mp4AtomAudio;
    dops: Mp4AtomDops;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomOpus): string;
export declare function eq(a: Mp4AtomOpus, b: Mp4AtomOpus): boolean;

// Mp4AtomOpusChannelMapping.ts
export type Mp4AtomOpusChannelMapping = {
    family: number;
    streamCount: number;
    coupledCount: number;
    channelMapping: Uint8Array;
};
export declare function debug(self: Mp4AtomOpusChannelMapping): string;
export declare function eq(a: Mp4AtomOpusChannelMapping, b: Mp4AtomOpusChannelMapping): boolean;

// Mp4AtomPack.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
import type { Mp4AtomPkin } from "./Mp4AtomPkin.ts";
export type Mp4AtomPack = rt.Handle<"Mp4AtomPack">;
export declare function debug(self: Mp4AtomPack): string;
export declare function clone(self: Mp4AtomPack): Mp4AtomPack;
export declare function eq(a: Mp4AtomPack, b: Mp4AtomPack): boolean;
export declare function default$(): Mp4AtomPack;
export declare function mustGet(self: Mp4AtomPack): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomPack, value: Mp4AtomMust | undefined): void;
export declare function pkinGet(self: Mp4AtomPack): Mp4AtomPkin | undefined;
export declare function pkinSet(self: Mp4AtomPack, value: Mp4AtomPkin | undefined): void;
export declare function unknownGet(self: Mp4AtomPack): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomPack, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomPasp.ts
import * as rt from "./rt.ts";
export type Mp4AtomPasp = {
    hSpacing: number;
    vSpacing: number;
};
export declare function debug(self: Mp4AtomPasp): string;
export declare function eq(a: Mp4AtomPasp, b: Mp4AtomPasp): boolean;
export declare function default$(): Mp4AtomPasp;
export declare function new$(hSpacing: number, vSpacing: number): rt.Result<Mp4AtomPasp, string>;
export { default$ as default, new$ as new };

// Mp4AtomPcmC.ts
export type Mp4AtomPcmC = {
    bigEndian: boolean;
    sampleSize: number;
};
export declare function debug(self: Mp4AtomPcmC): string;
export declare function eq(a: Mp4AtomPcmC, b: Mp4AtomPcmC): boolean;

// Mp4AtomPcmFormat.ts
export type Mp4AtomPcmFormat = {
    bigEndian: boolean;
    sampleSize: number;
    float: boolean;
};
export declare function debug(self: Mp4AtomPcmFormat): string;
export declare function eq(a: Mp4AtomPcmFormat, b: Mp4AtomPcmFormat): boolean;

// Mp4AtomPitm.ts
export type Mp4AtomPitm = {
    itemId: number;
};
export declare function debug(self: Mp4AtomPitm): string;
export declare function eq(a: Mp4AtomPitm, b: Mp4AtomPitm): boolean;
export declare function default$(): Mp4AtomPitm;
export { default$ as default };

// Mp4AtomPixi.ts
export type Mp4AtomPixi = {
    bitsPerChannel: Uint8Array;
};
export declare function debug(self: Mp4AtomPixi): string;
export declare function eq(a: Mp4AtomPixi, b: Mp4AtomPixi): boolean;
export declare function default$(): Mp4AtomPixi;
export { default$ as default };

// Mp4AtomPkin.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomPkin = {
    viewPackingKind: Mp4AtomFourCc;
};
export declare function debug(self: Mp4AtomPkin): string;
export declare function eq(a: Mp4AtomPkin, b: Mp4AtomPkin): boolean;
export declare function SIDE_BY_SIDE(): Mp4AtomFourCc;
export declare function OVER_UNDER(): Mp4AtomFourCc;

// Mp4AtomPlainText.ts
export type Mp4AtomPlainText = {
    dataReferenceIndex: number;
};
export declare function debug(self: Mp4AtomPlainText): string;
export declare function eq(a: Mp4AtomPlainText, b: Mp4AtomPlainText): boolean;

// Mp4AtomPrft.ts
import * as rt from "./rt.ts";
import type { Mp4AtomReferenceTime } from "./Mp4AtomReferenceTime.ts";
export type Mp4AtomPrft = {
    referenceTrackId: number;
    ntpTimestamp: rt.u64;
    mediaTime: rt.u64;
    utcTimeSemantics: Mp4AtomReferenceTime;
};
export declare function debug(self: Mp4AtomPrft): string;
export declare function eq(a: Mp4AtomPrft, b: Mp4AtomPrft): boolean;
export declare function default$(): Mp4AtomPrft;
export { default$ as default };

// Mp4AtomPrhd.ts
export type Mp4AtomPrhd = {
    poseYawDegrees: number;
    posePitchDegrees: number;
    poseRollDegrees: number;
};
export declare function debug(self: Mp4AtomPrhd): string;
export declare function eq(a: Mp4AtomPrhd, b: Mp4AtomPrhd): boolean;
export declare function default$(): Mp4AtomPrhd;
export { default$ as default };

// Mp4AtomPrji.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomPrji = {
    projectionKind: Mp4AtomFourCc;
};
export declare function debug(self: Mp4AtomPrji): string;
export declare function eq(a: Mp4AtomPrji, b: Mp4AtomPrji): boolean;
export declare function RECTILINEAR(): Mp4AtomFourCc;
export declare function EQUIRECTANGULAR(): Mp4AtomFourCc;
export declare function HALF_EQUIRECTANGULAR(): Mp4AtomFourCc;
export declare function FISHEYE(): Mp4AtomFourCc;
export declare function PARAMETRIC_IMMERSIVE(): Mp4AtomFourCc;

// Mp4AtomProj.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomPrhd } from "./Mp4AtomPrhd.ts";
import type { Mp4AtomProjKind } from "./Mp4AtomProjKind.ts";
export type Mp4AtomProj = rt.Handle<"Mp4AtomProj">;
export declare function debug(self: Mp4AtomProj): string;
export declare function clone(self: Mp4AtomProj): Mp4AtomProj;
export declare function eq(a: Mp4AtomProj, b: Mp4AtomProj): boolean;
export declare function default$(): Mp4AtomProj;
export declare function prhdGet(self: Mp4AtomProj): Mp4AtomPrhd | undefined;
export declare function prhdSet(self: Mp4AtomProj, value: Mp4AtomPrhd | undefined): void;
export declare function kindGet(self: Mp4AtomProj): Mp4AtomProjKind | undefined;
export declare function kindSet(self: Mp4AtomProj, value: Mp4AtomProjKind | undefined): void;
export declare function unknownGet(self: Mp4AtomProj): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomProj, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomProjKind.ts
import type { Mp4AtomCbmp } from "./Mp4AtomCbmp.ts";
import type { Mp4AtomEqui } from "./Mp4AtomEqui.ts";
import type { Mp4AtomMshp } from "./Mp4AtomMshp.ts";
export type Mp4AtomProjKind = {
    tag: "Equi";
    _0: Mp4AtomEqui;
} | {
    tag: "Cbmp";
    _0: Mp4AtomCbmp;
} | {
    tag: "Mshp";
    _0: Mp4AtomMshp;
};
export declare function debug(self: Mp4AtomProjKind): string;
export declare function eq(a: Mp4AtomProjKind, b: Mp4AtomProjKind): boolean;

// Mp4AtomPropertyAssociation.ts
export type Mp4AtomPropertyAssociation = {
    essential: boolean;
    propertyIndex: number;
};
export declare function debug(self: Mp4AtomPropertyAssociation): string;
export declare function eq(a: Mp4AtomPropertyAssociation, b: Mp4AtomPropertyAssociation): boolean;
export declare function default$(): Mp4AtomPropertyAssociation;
export { default$ as default };

// Mp4AtomPropertyAssociations.ts
import type { Mp4AtomPropertyAssociation } from "./Mp4AtomPropertyAssociation.ts";
export type Mp4AtomPropertyAssociations = {
    itemId: number;
    associations: Mp4AtomPropertyAssociation[];
};
export declare function debug(self: Mp4AtomPropertyAssociations): string;
export declare function eq(a: Mp4AtomPropertyAssociations, b: Mp4AtomPropertyAssociations): boolean;
export declare function default$(): Mp4AtomPropertyAssociations;
export { default$ as default };

// Mp4AtomReference.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomReference = {
    referenceType: Mp4AtomFourCc;
    fromItemId: number;
    toItemIds: Uint32Array;
};
export declare function debug(self: Mp4AtomReference): string;
export declare function eq(a: Mp4AtomReference, b: Mp4AtomReference): boolean;

// Mp4AtomReferenceTime.ts
export type Mp4AtomReferenceTime = "Input" | "Output" | "Finalised" | "Written" | "Consistent" | "RealTime";
export declare function debug(self: Mp4AtomReferenceTime): string;
export declare function default$(): Mp4AtomReferenceTime;
export { default$ as default };

// Mp4AtomRgbColor.ts
export type Mp4AtomRgbColor = {
    red: number;
    green: number;
    blue: number;
};
export declare function debug(self: Mp4AtomRgbColor): string;
export declare function eq(a: Mp4AtomRgbColor, b: Mp4AtomRgbColor): boolean;
export declare function default$(): Mp4AtomRgbColor;
export { default$ as default };

// Mp4AtomRgbaColor.ts
export type Mp4AtomRgbaColor = {
    red: number;
    green: number;
    blue: number;
    alpha: number;
};
export declare function debug(self: Mp4AtomRgbaColor): string;
export declare function eq(a: Mp4AtomRgbaColor, b: Mp4AtomRgbaColor): boolean;
export declare function default$(): Mp4AtomRgbaColor;
export { default$ as default };

// Mp4AtomRref.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomRref = {
    referenceTypes: Mp4AtomFourCc[];
};
export declare function debug(self: Mp4AtomRref): string;
export declare function eq(a: Mp4AtomRref, b: Mp4AtomRref): boolean;
export declare function default$(): Mp4AtomRref;
export { default$ as default };

// Mp4AtomRtng.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomRtng = {
    entity: Mp4AtomFourCc;
    criteria: Mp4AtomFourCc;
    language: string;
    ratingInfo: string;
};
export declare function debug(self: Mp4AtomRtng): string;
export declare function eq(a: Mp4AtomRtng, b: Mp4AtomRtng): boolean;

// Mp4AtomS16l.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomS16l = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomS16l): string;
export declare function eq(a: Mp4AtomS16l, b: Mp4AtomS16l): boolean;
export declare function format(self: Mp4AtomS16l): Mp4AtomPcmFormat | undefined;

// Mp4AtomSaio.ts
import type { Mp4AtomAuxInfo } from "./Mp4AtomAuxInfo.ts";
export type Mp4AtomSaio = {
    auxInfo: Mp4AtomAuxInfo | undefined;
    offsets: BigUint64Array;
};
export declare function debug(self: Mp4AtomSaio): string;
export declare function eq(a: Mp4AtomSaio, b: Mp4AtomSaio): boolean;
export declare function default$(): Mp4AtomSaio;
export { default$ as default };

// Mp4AtomSaiz.ts
import type { Mp4AtomAuxInfo } from "./Mp4AtomAuxInfo.ts";
export type Mp4AtomSaiz = {
    auxInfo: Mp4AtomAuxInfo | undefined;
    defaultSampleInfoSize: number;
    sampleCount: number;
    sampleInfoSize: Uint8Array;
};
export declare function debug(self: Mp4AtomSaiz): string;
export declare function eq(a: Mp4AtomSaiz, b: Mp4AtomSaiz): boolean;
export declare function default$(): Mp4AtomSaiz;
export { default$ as default };

// Mp4AtomSamr.ts
import type { Mp4AtomAmrSampleEntry } from "./Mp4AtomAmrSampleEntry.ts";
import type { Mp4AtomDamr } from "./Mp4AtomDamr.ts";
export type Mp4AtomSamr = {
    amrsampleentry: Mp4AtomAmrSampleEntry;
    damr: Mp4AtomDamr;
};
export declare function debug(self: Mp4AtomSamr): string;
export declare function eq(a: Mp4AtomSamr, b: Mp4AtomSamr): boolean;

// Mp4AtomSbgp.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomSbgpEntry } from "./Mp4AtomSbgpEntry.ts";
export type Mp4AtomSbgp = {
    groupingType: Mp4AtomFourCc;
    groupingTypeParameter: number | undefined;
    entries: Mp4AtomSbgpEntry[];
};
export declare function debug(self: Mp4AtomSbgp): string;
export declare function eq(a: Mp4AtomSbgp, b: Mp4AtomSbgp): boolean;

// Mp4AtomSbgpEntry.ts
export type Mp4AtomSbgpEntry = {
    sampleCount: number;
    groupDescriptionIndex: number;
};
export declare function debug(self: Mp4AtomSbgpEntry): string;
export declare function eq(a: Mp4AtomSbgpEntry, b: Mp4AtomSbgpEntry): boolean;

// Mp4AtomSchemeIdEntry.ts
export type Mp4AtomSchemeIdEntry = {
    schemeIdUri: string;
    value: string;
    atleastOnce: boolean;
};
export declare function debug(self: Mp4AtomSchemeIdEntry): string;
export declare function eq(a: Mp4AtomSchemeIdEntry, b: Mp4AtomSchemeIdEntry): boolean;

// Mp4AtomSegmentReference.ts
export type Mp4AtomSegmentReference = {
    referenceType: boolean;
    referencedSize: number;
    subsegmentDuration: number;
    startsWithSap: boolean;
    sapType: number;
    sapDeltaTime: number;
};
export declare function debug(self: Mp4AtomSegmentReference): string;
export declare function eq(a: Mp4AtomSegmentReference, b: Mp4AtomSegmentReference): boolean;

// Mp4AtomSenc.ts
import type { Mp4AtomSencBoxVersion } from "./Mp4AtomSencBoxVersion.ts";
export type Mp4AtomSenc = {
    version: Mp4AtomSencBoxVersion;
    useSubsamples: boolean;
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomSenc): string;
export declare function eq(a: Mp4AtomSenc, b: Mp4AtomSenc): boolean;
export declare function default$(): Mp4AtomSenc;
export { default$ as default };

// Mp4AtomSencBoxVersion.ts
export type Mp4AtomSencBoxVersion = "V0" | "V1" | "V2";
export declare function debug(self: Mp4AtomSencBoxVersion): string;
export declare function default$(): Mp4AtomSencBoxVersion;
export { default$ as default };

// Mp4AtomSetu.ts
export type Mp4AtomSetu = {
    data: Uint8Array;
};
export declare function debug(self: Mp4AtomSetu): string;
export declare function eq(a: Mp4AtomSetu, b: Mp4AtomSetu): boolean;

// Mp4AtomSgpd.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomSgpdEntry } from "./Mp4AtomSgpdEntry.ts";
export type Mp4AtomSgpd = {
    groupingType: Mp4AtomFourCc;
    defaultLength: number | undefined;
    defaultGroupDescriptionIndex: number | undefined;
    staticGroupDescription: boolean;
    staticMapping: boolean;
    essential: boolean;
    entries: Mp4AtomSgpdEntry[];
};
export declare function debug(self: Mp4AtomSgpd): string;
export declare function eq(a: Mp4AtomSgpd, b: Mp4AtomSgpd): boolean;

// Mp4AtomSgpdEntry.ts
import type { Mp4AtomAnySampleGroupEntry } from "./Mp4AtomAnySampleGroupEntry.ts";
export type Mp4AtomSgpdEntry = {
    descriptionLength: number | undefined;
    entry: Mp4AtomAnySampleGroupEntry;
};
export declare function debug(self: Mp4AtomSgpdEntry): string;
export declare function eq(a: Mp4AtomSgpdEntry, b: Mp4AtomSgpdEntry): boolean;

// Mp4AtomSidx.ts
import * as rt from "./rt.ts";
import type { Mp4AtomSegmentReference } from "./Mp4AtomSegmentReference.ts";
export type Mp4AtomSidx = {
    referenceId: number;
    timescale: number;
    earliestPresentationTime: rt.u64;
    firstOffset: rt.u64;
    references: Mp4AtomSegmentReference[];
};
export declare function debug(self: Mp4AtomSidx): string;
export declare function eq(a: Mp4AtomSidx, b: Mp4AtomSidx): boolean;
export declare function default$(): Mp4AtomSidx;
export { default$ as default };

// Mp4AtomSilb.ts
import type { Mp4AtomSchemeIdEntry } from "./Mp4AtomSchemeIdEntry.ts";
export type Mp4AtomSilb = {
    schemes: Mp4AtomSchemeIdEntry[];
    otherSchemes: boolean;
};
export declare function debug(self: Mp4AtomSilb): string;
export declare function eq(a: Mp4AtomSilb, b: Mp4AtomSilb): boolean;

// Mp4AtomSkip.ts
import type { Mp4AtomZeroed } from "./Mp4AtomZeroed.ts";
export type Mp4AtomSkip = {
    zeroed: Mp4AtomZeroed;
};
export declare function debug(self: Mp4AtomSkip): string;
export declare function eq(a: Mp4AtomSkip, b: Mp4AtomSkip): boolean;
export declare function default$(): Mp4AtomSkip;
export { default$ as default };

// Mp4AtomSmhd.ts
import type { Mp4AtomFixedPointI8 } from "./Mp4AtomFixedPointI8.ts";
export type Mp4AtomSmhd = {
    balance: Mp4AtomFixedPointI8;
};
export declare function debug(self: Mp4AtomSmhd): string;
export declare function eq(a: Mp4AtomSmhd, b: Mp4AtomSmhd): boolean;
export declare function default$(): Mp4AtomSmhd;
export { default$ as default };

// Mp4AtomSowt.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomSowt = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomSowt): string;
export declare function eq(a: Mp4AtomSowt, b: Mp4AtomSowt): boolean;
export declare function format(self: Mp4AtomSowt): Mp4AtomPcmFormat | undefined;

// Mp4AtomSpatial.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomHfov } from "./Mp4AtomHfov.ts";
import type { Mp4AtomSt3d } from "./Mp4AtomSt3d.ts";
import type { Mp4AtomSv3d } from "./Mp4AtomSv3d.ts";
import type { Mp4AtomVexu } from "./Mp4AtomVexu.ts";
export type Mp4AtomSpatial = rt.Handle<"Mp4AtomSpatial">;
export declare function debug(self: Mp4AtomSpatial): string;
export declare function clone(self: Mp4AtomSpatial): Mp4AtomSpatial;
export declare function eq(a: Mp4AtomSpatial, b: Mp4AtomSpatial): boolean;
export declare function default$(): Mp4AtomSpatial;
export declare function st3dGet(self: Mp4AtomSpatial): Mp4AtomSt3d | undefined;
export declare function st3dSet(self: Mp4AtomSpatial, value: Mp4AtomSt3d | undefined): void;
export declare function sv3dGet(self: Mp4AtomSpatial): Mp4AtomSv3d | undefined;
export declare function sv3dSet(self: Mp4AtomSpatial, value: Mp4AtomSv3d | undefined): void;
export declare function vexuGet(self: Mp4AtomSpatial): Mp4AtomVexu | undefined;
export declare function vexuSet(self: Mp4AtomSpatial, value: Mp4AtomVexu | undefined): void;
export declare function hfovGet(self: Mp4AtomSpatial): Mp4AtomHfov | undefined;
export declare function hfovSet(self: Mp4AtomSpatial, value: Mp4AtomHfov | undefined): void;
export declare function unknownGet(self: Mp4AtomSpatial): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomSpatial, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomSpeakerPosition.ts
import type { Mp4AtomAudioChannelPosition } from "./Mp4AtomAudioChannelPosition.ts";
import type { Mp4AtomExplicitSpeakerPosition } from "./Mp4AtomExplicitSpeakerPosition.ts";
export type Mp4AtomSpeakerPosition = {
    tag: "Standard";
    _0: Mp4AtomAudioChannelPosition;
} | {
    tag: "Explicit";
    _0: Mp4AtomExplicitSpeakerPosition;
};
export declare function debug(self: Mp4AtomSpeakerPosition): string;
export declare function eq(a: Mp4AtomSpeakerPosition, b: Mp4AtomSpeakerPosition): boolean;

// Mp4AtomSt3d.ts
export type Mp4AtomSt3d = {
    stereoMode: number;
};
export declare function debug(self: Mp4AtomSt3d): string;
export declare function eq(a: Mp4AtomSt3d, b: Mp4AtomSt3d): boolean;
export declare function default$(): Mp4AtomSt3d;
export { default$ as default };

// Mp4AtomStbl.ts
import * as rt from "./rt.ts";
import type { Mp4AtomCo64 } from "./Mp4AtomCo64.ts";
import type { Mp4AtomCslg } from "./Mp4AtomCslg.ts";
import type { Mp4AtomCtts } from "./Mp4AtomCtts.ts";
import type { Mp4AtomSaio } from "./Mp4AtomSaio.ts";
import type { Mp4AtomSaiz } from "./Mp4AtomSaiz.ts";
import type { Mp4AtomSbgp } from "./Mp4AtomSbgp.ts";
import type { Mp4AtomSgpd } from "./Mp4AtomSgpd.ts";
import type { Mp4AtomStco } from "./Mp4AtomStco.ts";
import type { Mp4AtomStsc } from "./Mp4AtomStsc.ts";
import type { Mp4AtomStsd } from "./Mp4AtomStsd.ts";
import type { Mp4AtomStss } from "./Mp4AtomStss.ts";
import type { Mp4AtomStsz } from "./Mp4AtomStsz.ts";
import type { Mp4AtomStts } from "./Mp4AtomStts.ts";
import type { Mp4AtomSubs } from "./Mp4AtomSubs.ts";
export type Mp4AtomStbl = rt.Handle<"Mp4AtomStbl">;
export declare function debug(self: Mp4AtomStbl): string;
export declare function clone(self: Mp4AtomStbl): Mp4AtomStbl;
export declare function eq(a: Mp4AtomStbl, b: Mp4AtomStbl): boolean;
export declare function default$(): Mp4AtomStbl;
export declare function stsdGet(self: Mp4AtomStbl): Mp4AtomStsd;
export declare function stsdSet(self: Mp4AtomStbl, value: Mp4AtomStsd): void;
export declare function sttsGet(self: Mp4AtomStbl): Mp4AtomStts;
export declare function sttsSet(self: Mp4AtomStbl, value: Mp4AtomStts): void;
export declare function cttsGet(self: Mp4AtomStbl): Mp4AtomCtts | undefined;
export declare function cttsSet(self: Mp4AtomStbl, value: Mp4AtomCtts | undefined): void;
export declare function stssGet(self: Mp4AtomStbl): Mp4AtomStss | undefined;
export declare function stssSet(self: Mp4AtomStbl, value: Mp4AtomStss | undefined): void;
export declare function stscGet(self: Mp4AtomStbl): Mp4AtomStsc;
export declare function stscSet(self: Mp4AtomStbl, value: Mp4AtomStsc): void;
export declare function stszGet(self: Mp4AtomStbl): Mp4AtomStsz;
export declare function stszSet(self: Mp4AtomStbl, value: Mp4AtomStsz): void;
export declare function stcoGet(self: Mp4AtomStbl): Mp4AtomStco | undefined;
export declare function stcoSet(self: Mp4AtomStbl, value: Mp4AtomStco | undefined): void;
export declare function co64Get(self: Mp4AtomStbl): Mp4AtomCo64 | undefined;
export declare function co64Set(self: Mp4AtomStbl, value: Mp4AtomCo64 | undefined): void;
export declare function sbgpGet(self: Mp4AtomStbl): Mp4AtomSbgp[];
export declare function sbgpSet(self: Mp4AtomStbl, value: Mp4AtomSbgp[]): void;
export declare function sgpdGet(self: Mp4AtomStbl): Mp4AtomSgpd[];
export declare function sgpdSet(self: Mp4AtomStbl, value: Mp4AtomSgpd[]): void;
export declare function subsGet(self: Mp4AtomStbl): Mp4AtomSubs[];
export declare function subsSet(self: Mp4AtomStbl, value: Mp4AtomSubs[]): void;
export declare function saizGet(self: Mp4AtomStbl): Mp4AtomSaiz[];
export declare function saizSet(self: Mp4AtomStbl, value: Mp4AtomSaiz[]): void;
export declare function saioGet(self: Mp4AtomStbl): Mp4AtomSaio[];
export declare function saioSet(self: Mp4AtomStbl, value: Mp4AtomSaio[]): void;
export declare function cslgGet(self: Mp4AtomStbl): Mp4AtomCslg | undefined;
export declare function cslgSet(self: Mp4AtomStbl, value: Mp4AtomCslg | undefined): void;
export { default$ as default };

// Mp4AtomStco.ts
export type Mp4AtomStco = {
    entries: Uint32Array;
};
export declare function debug(self: Mp4AtomStco): string;
export declare function eq(a: Mp4AtomStco, b: Mp4AtomStco): boolean;
export declare function default$(): Mp4AtomStco;
export { default$ as default };

// Mp4AtomSthd.ts
import * as rt from "./rt.ts";
export type Mp4AtomSthd = rt.Handle<"Mp4AtomSthd">;
export declare function debug(self: Mp4AtomSthd): string;
export declare function clone(self: Mp4AtomSthd): Mp4AtomSthd;
export declare function eq(a: Mp4AtomSthd, b: Mp4AtomSthd): boolean;
export declare function default$(): Mp4AtomSthd;
export { default$ as default };

// Mp4AtomStri.ts
export type Mp4AtomStri = {
    eyeViewsReversed: boolean;
    hasAdditionalViews: boolean;
    hasRightEyeView: boolean;
    hasLeftEyeView: boolean;
};
export declare function debug(self: Mp4AtomStri): string;
export declare function eq(a: Mp4AtomStri, b: Mp4AtomStri): boolean;
export declare function default$(): Mp4AtomStri;
export { default$ as default };

// Mp4AtomStsc.ts
import type { Mp4AtomStscEntry } from "./Mp4AtomStscEntry.ts";
export type Mp4AtomStsc = {
    entries: Mp4AtomStscEntry[];
};
export declare function debug(self: Mp4AtomStsc): string;
export declare function eq(a: Mp4AtomStsc, b: Mp4AtomStsc): boolean;
export declare function default$(): Mp4AtomStsc;
export { default$ as default };

// Mp4AtomStscEntry.ts
export type Mp4AtomStscEntry = {
    firstChunk: number;
    samplesPerChunk: number;
    sampleDescriptionIndex: number;
};
export declare function debug(self: Mp4AtomStscEntry): string;
export declare function eq(a: Mp4AtomStscEntry, b: Mp4AtomStscEntry): boolean;
export declare function default$(): Mp4AtomStscEntry;
export { default$ as default };

// Mp4AtomStsd.ts
import * as rt from "./rt.ts";
import type { Mp4AtomCodec } from "./Mp4AtomCodec.ts";
export type Mp4AtomStsd = rt.Handle<"Mp4AtomStsd">;
export declare function debug(self: Mp4AtomStsd): string;
export declare function clone(self: Mp4AtomStsd): Mp4AtomStsd;
export declare function eq(a: Mp4AtomStsd, b: Mp4AtomStsd): boolean;
export declare function default$(): Mp4AtomStsd;
export declare function codecsGet(self: Mp4AtomStsd): Mp4AtomCodec[];
export declare function codecsSet(self: Mp4AtomStsd, value: Mp4AtomCodec[]): void;
export { default$ as default };

// Mp4AtomStss.ts
export type Mp4AtomStss = {
    entries: Uint32Array;
};
export declare function debug(self: Mp4AtomStss): string;
export declare function eq(a: Mp4AtomStss, b: Mp4AtomStss): boolean;
export declare function default$(): Mp4AtomStss;
export { default$ as default };

// Mp4AtomStsz.ts
import type { Mp4AtomStszSamples } from "./Mp4AtomStszSamples.ts";
export type Mp4AtomStsz = {
    samples: Mp4AtomStszSamples;
};
export declare function debug(self: Mp4AtomStsz): string;
export declare function eq(a: Mp4AtomStsz, b: Mp4AtomStsz): boolean;
export declare function default$(): Mp4AtomStsz;
export { default$ as default };

// Mp4AtomStszSamples.ts
export type Mp4AtomStszSamples = {
    tag: "Identical";
    count: number;
    size: number;
} | {
    tag: "Different";
    sizes: Uint32Array;
};
export declare function debug(self: Mp4AtomStszSamples): string;
export declare function eq(a: Mp4AtomStszSamples, b: Mp4AtomStszSamples): boolean;
export declare function default$(): Mp4AtomStszSamples;
export { default$ as default };

// Mp4AtomStts.ts
import type { Mp4AtomSttsEntry } from "./Mp4AtomSttsEntry.ts";
export type Mp4AtomStts = {
    entries: Mp4AtomSttsEntry[];
};
export declare function debug(self: Mp4AtomStts): string;
export declare function eq(a: Mp4AtomStts, b: Mp4AtomStts): boolean;
export declare function default$(): Mp4AtomStts;
export { default$ as default };

// Mp4AtomSttsEntry.ts
export type Mp4AtomSttsEntry = {
    sampleCount: number;
    sampleDelta: number;
};
export declare function debug(self: Mp4AtomSttsEntry): string;
export declare function eq(a: Mp4AtomSttsEntry, b: Mp4AtomSttsEntry): boolean;
export declare function default$(): Mp4AtomSttsEntry;
export { default$ as default };

// Mp4AtomStyp.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomStyp = {
    majorBrand: Mp4AtomFourCc;
    minorVersion: number;
    compatibleBrands: Mp4AtomFourCc[];
};
export declare function debug(self: Mp4AtomStyp): string;
export declare function eq(a: Mp4AtomStyp, b: Mp4AtomStyp): boolean;

// Mp4AtomSubs.ts
import type { Mp4AtomSubsEntry } from "./Mp4AtomSubsEntry.ts";
export type Mp4AtomSubs = {
    flags: Uint8Array;
    entries: Mp4AtomSubsEntry[];
};
export declare function debug(self: Mp4AtomSubs): string;
export declare function eq(a: Mp4AtomSubs, b: Mp4AtomSubs): boolean;
export declare function default$(): Mp4AtomSubs;
export { default$ as default };

// Mp4AtomSubsEntry.ts
import type { Mp4AtomSubsSubsample } from "./Mp4AtomSubsSubsample.ts";
export type Mp4AtomSubsEntry = {
    sampleDelta: number;
    subsamples: Mp4AtomSubsSubsample[];
};
export declare function debug(self: Mp4AtomSubsEntry): string;
export declare function eq(a: Mp4AtomSubsEntry, b: Mp4AtomSubsEntry): boolean;
export declare function default$(): Mp4AtomSubsEntry;
export { default$ as default };

// Mp4AtomSubsSubsample.ts
import type { Mp4AtomSubsSubsampleSize } from "./Mp4AtomSubsSubsampleSize.ts";
export type Mp4AtomSubsSubsample = {
    size: Mp4AtomSubsSubsampleSize;
    priority: number;
    discardable: boolean;
    codecSpecificParameters: Uint8Array;
};
export declare function debug(self: Mp4AtomSubsSubsample): string;
export declare function eq(a: Mp4AtomSubsSubsample, b: Mp4AtomSubsSubsample): boolean;
export declare function default$(): Mp4AtomSubsSubsample;
export { default$ as default };

// Mp4AtomSubsSubsampleSize.ts
export type Mp4AtomSubsSubsampleSize = {
    tag: "U16";
    _0: number;
} | {
    tag: "U32";
    _0: number;
};
export declare function debug(self: Mp4AtomSubsSubsampleSize): string;
export declare function eq(a: Mp4AtomSubsSubsampleSize, b: Mp4AtomSubsSubsampleSize): boolean;
export declare function default$(): Mp4AtomSubsSubsampleSize;
export declare function value(self: Mp4AtomSubsSubsampleSize): number;
export { default$ as default };

// Mp4AtomSv3d.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomProj } from "./Mp4AtomProj.ts";
import type { Mp4AtomSvhd } from "./Mp4AtomSvhd.ts";
export type Mp4AtomSv3d = rt.Handle<"Mp4AtomSv3d">;
export declare function debug(self: Mp4AtomSv3d): string;
export declare function clone(self: Mp4AtomSv3d): Mp4AtomSv3d;
export declare function eq(a: Mp4AtomSv3d, b: Mp4AtomSv3d): boolean;
export declare function default$(): Mp4AtomSv3d;
export declare function svhdGet(self: Mp4AtomSv3d): Mp4AtomSvhd | undefined;
export declare function svhdSet(self: Mp4AtomSv3d, value: Mp4AtomSvhd | undefined): void;
export declare function projGet(self: Mp4AtomSv3d): Mp4AtomProj | undefined;
export declare function projSet(self: Mp4AtomSv3d, value: Mp4AtomProj | undefined): void;
export declare function unknownGet(self: Mp4AtomSv3d): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomSv3d, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomSvhd.ts
export type Mp4AtomSvhd = {
    metadataSource: string;
};
export declare function debug(self: Mp4AtomSvhd): string;
export declare function eq(a: Mp4AtomSvhd, b: Mp4AtomSvhd): boolean;
export declare function default$(): Mp4AtomSvhd;
export { default$ as default };

// Mp4AtomTaic.ts
import * as rt from "./rt.ts";
import type { Mp4AtomClockType } from "./Mp4AtomClockType.ts";
export type Mp4AtomTaic = {
    timeUncertainty: rt.u64;
    clockResolution: number;
    clockDriftRate: number;
    clockType: Mp4AtomClockType;
};
export declare function debug(self: Mp4AtomTaic): string;
export declare function eq(a: Mp4AtomTaic, b: Mp4AtomTaic): boolean;
export declare function default$(): Mp4AtomTaic;
export declare function new$(timeUncertainty: rt.u64, clockResolution: number, clockDriftRate: number, clockType: Mp4AtomClockType): rt.Result<Mp4AtomTaic, string>;
export { default$ as default, new$ as new };

// Mp4AtomTfdt.ts
import * as rt from "./rt.ts";
export type Mp4AtomTfdt = {
    baseMediaDecodeTime: rt.u64;
};
export declare function debug(self: Mp4AtomTfdt): string;
export declare function eq(a: Mp4AtomTfdt, b: Mp4AtomTfdt): boolean;
export declare function default$(): Mp4AtomTfdt;
export { default$ as default };

// Mp4AtomTfhd.ts
import * as rt from "./rt.ts";
export type Mp4AtomTfhd = {
    trackId: number;
    baseDataOffset: rt.u64 | undefined;
    sampleDescriptionIndex: number | undefined;
    defaultSampleDuration: number | undefined;
    defaultSampleSize: number | undefined;
    defaultSampleFlags: number | undefined;
    durationIsEmpty: boolean;
    defaultBaseIsMoof: boolean;
};
export declare function debug(self: Mp4AtomTfhd): string;
export declare function eq(a: Mp4AtomTfhd, b: Mp4AtomTfhd): boolean;
export declare function default$(): Mp4AtomTfhd;
export { default$ as default };

// Mp4AtomTfra.ts
import type { Mp4AtomFragmentInfo } from "./Mp4AtomFragmentInfo.ts";
export type Mp4AtomTfra = {
    trackId: number;
    entries: Mp4AtomFragmentInfo[];
};
export declare function debug(self: Mp4AtomTfra): string;
export declare function eq(a: Mp4AtomTfra, b: Mp4AtomTfra): boolean;
export declare function default$(): Mp4AtomTfra;
export { default$ as default };

// Mp4AtomTkhd.ts
import * as rt from "./rt.ts";
import type { Mp4AtomFixedPointU16 } from "./Mp4AtomFixedPointU16.ts";
import type { Mp4AtomFixedPointU8 } from "./Mp4AtomFixedPointU8.ts";
import type { Mp4AtomMatrix } from "./Mp4AtomMatrix.ts";
export type Mp4AtomTkhd = {
    creationTime: rt.u64;
    modificationTime: rt.u64;
    trackId: number;
    duration: rt.u64;
    layer: number;
    alternateGroup: number;
    enabled: boolean;
    inMovie: boolean;
    sizeIsAspectRatio: boolean;
    volume: Mp4AtomFixedPointU8;
    matrix: Mp4AtomMatrix;
    width: Mp4AtomFixedPointU16;
    height: Mp4AtomFixedPointU16;
};
export declare function debug(self: Mp4AtomTkhd): string;
export declare function eq(a: Mp4AtomTkhd, b: Mp4AtomTkhd): boolean;
export declare function default$(): Mp4AtomTkhd;
export { default$ as default };

// Mp4AtomTool.ts
import * as rt from "./rt.ts";
export type Mp4AtomTool = {
    countryIndicator: number;
    languageIndicator: number;
    text: string;
};
export declare function debug(self: Mp4AtomTool): string;
export declare function eq(a: Mp4AtomTool, b: Mp4AtomTool): boolean;
export declare function new$(countryIndicator: number, languageIndicator: number, text: string): rt.Result<Mp4AtomTool, string>;
export { new$ as new };

// Mp4AtomTrackReferenceTypeBox.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomTrackReferenceTypeBox = {
    referenceType: Mp4AtomFourCc;
    trackIds: Uint32Array;
};
export declare function debug(self: Mp4AtomTrackReferenceTypeBox): string;
export declare function eq(a: Mp4AtomTrackReferenceTypeBox, b: Mp4AtomTrackReferenceTypeBox): boolean;

// Mp4AtomTraf.ts
import * as rt from "./rt.ts";
import type { Mp4AtomMeta } from "./Mp4AtomMeta.ts";
import type { Mp4AtomSaio } from "./Mp4AtomSaio.ts";
import type { Mp4AtomSaiz } from "./Mp4AtomSaiz.ts";
import type { Mp4AtomSbgp } from "./Mp4AtomSbgp.ts";
import type { Mp4AtomSenc } from "./Mp4AtomSenc.ts";
import type { Mp4AtomSgpd } from "./Mp4AtomSgpd.ts";
import type { Mp4AtomSubs } from "./Mp4AtomSubs.ts";
import type { Mp4AtomTfdt } from "./Mp4AtomTfdt.ts";
import type { Mp4AtomTfhd } from "./Mp4AtomTfhd.ts";
import type { Mp4AtomTrun } from "./Mp4AtomTrun.ts";
import type { Mp4AtomUdta } from "./Mp4AtomUdta.ts";
export type Mp4AtomTraf = rt.Handle<"Mp4AtomTraf">;
export declare function debug(self: Mp4AtomTraf): string;
export declare function clone(self: Mp4AtomTraf): Mp4AtomTraf;
export declare function eq(a: Mp4AtomTraf, b: Mp4AtomTraf): boolean;
export declare function default$(): Mp4AtomTraf;
export declare function tfhdGet(self: Mp4AtomTraf): Mp4AtomTfhd;
export declare function tfhdSet(self: Mp4AtomTraf, value: Mp4AtomTfhd): void;
export declare function tfdtGet(self: Mp4AtomTraf): Mp4AtomTfdt | undefined;
export declare function tfdtSet(self: Mp4AtomTraf, value: Mp4AtomTfdt | undefined): void;
export declare function trunGet(self: Mp4AtomTraf): Mp4AtomTrun[];
export declare function trunSet(self: Mp4AtomTraf, value: Mp4AtomTrun[]): void;
export declare function sbgpGet(self: Mp4AtomTraf): Mp4AtomSbgp[];
export declare function sbgpSet(self: Mp4AtomTraf, value: Mp4AtomSbgp[]): void;
export declare function sgpdGet(self: Mp4AtomTraf): Mp4AtomSgpd[];
export declare function sgpdSet(self: Mp4AtomTraf, value: Mp4AtomSgpd[]): void;
export declare function subsGet(self: Mp4AtomTraf): Mp4AtomSubs[];
export declare function subsSet(self: Mp4AtomTraf, value: Mp4AtomSubs[]): void;
export declare function saizGet(self: Mp4AtomTraf): Mp4AtomSaiz[];
export declare function saizSet(self: Mp4AtomTraf, value: Mp4AtomSaiz[]): void;
export declare function saioGet(self: Mp4AtomTraf): Mp4AtomSaio[];
export declare function saioSet(self: Mp4AtomTraf, value: Mp4AtomSaio[]): void;
export declare function metaGet(self: Mp4AtomTraf): Mp4AtomMeta | undefined;
export declare function metaSet(self: Mp4AtomTraf, value: Mp4AtomMeta | undefined): void;
export declare function sencGet(self: Mp4AtomTraf): Mp4AtomSenc | undefined;
export declare function sencSet(self: Mp4AtomTraf, value: Mp4AtomSenc | undefined): void;
export declare function udtaGet(self: Mp4AtomTraf): Mp4AtomUdta | undefined;
export declare function udtaSet(self: Mp4AtomTraf, value: Mp4AtomUdta | undefined): void;
export { default$ as default };

// Mp4AtomTrak.ts
import * as rt from "./rt.ts";
import type { Mp4AtomEdts } from "./Mp4AtomEdts.ts";
import type { Mp4AtomMdia } from "./Mp4AtomMdia.ts";
import type { Mp4AtomMeta } from "./Mp4AtomMeta.ts";
import type { Mp4AtomSenc } from "./Mp4AtomSenc.ts";
import type { Mp4AtomTkhd } from "./Mp4AtomTkhd.ts";
import type { Mp4AtomTref } from "./Mp4AtomTref.ts";
import type { Mp4AtomUdta } from "./Mp4AtomUdta.ts";
export type Mp4AtomTrak = rt.Handle<"Mp4AtomTrak">;
export declare function debug(self: Mp4AtomTrak): string;
export declare function clone(self: Mp4AtomTrak): Mp4AtomTrak;
export declare function eq(a: Mp4AtomTrak, b: Mp4AtomTrak): boolean;
export declare function default$(): Mp4AtomTrak;
export declare function tkhdGet(self: Mp4AtomTrak): Mp4AtomTkhd;
export declare function tkhdSet(self: Mp4AtomTrak, value: Mp4AtomTkhd): void;
export declare function edtsGet(self: Mp4AtomTrak): Mp4AtomEdts | undefined;
export declare function edtsSet(self: Mp4AtomTrak, value: Mp4AtomEdts | undefined): void;
export declare function metaGet(self: Mp4AtomTrak): Mp4AtomMeta | undefined;
export declare function metaSet(self: Mp4AtomTrak, value: Mp4AtomMeta | undefined): void;
export declare function mdiaGet(self: Mp4AtomTrak): Mp4AtomMdia;
export declare function mdiaSet(self: Mp4AtomTrak, value: Mp4AtomMdia): void;
export declare function sencGet(self: Mp4AtomTrak): Mp4AtomSenc | undefined;
export declare function sencSet(self: Mp4AtomTrak, value: Mp4AtomSenc | undefined): void;
export declare function trefGet(self: Mp4AtomTrak): Mp4AtomTref | undefined;
export declare function trefSet(self: Mp4AtomTrak, value: Mp4AtomTref | undefined): void;
export declare function udtaGet(self: Mp4AtomTrak): Mp4AtomUdta | undefined;
export declare function udtaSet(self: Mp4AtomTrak, value: Mp4AtomUdta | undefined): void;
export { default$ as default };

// Mp4AtomTref.ts
import type { Mp4AtomTrackReferenceTypeBox } from "./Mp4AtomTrackReferenceTypeBox.ts";
export type Mp4AtomTref = {
    trackReferenceTypeBoxes: Mp4AtomTrackReferenceTypeBox[];
};
export declare function debug(self: Mp4AtomTref): string;
export declare function eq(a: Mp4AtomTref, b: Mp4AtomTref): boolean;
export declare function default$(): Mp4AtomTref;
export { default$ as default };

// Mp4AtomTrex.ts
export type Mp4AtomTrex = {
    trackId: number;
    defaultSampleDescriptionIndex: number;
    defaultSampleDuration: number;
    defaultSampleSize: number;
    defaultSampleFlags: number;
};
export declare function debug(self: Mp4AtomTrex): string;
export declare function eq(a: Mp4AtomTrex, b: Mp4AtomTrex): boolean;
export declare function default$(): Mp4AtomTrex;
export { default$ as default };

// Mp4AtomTrun.ts
import type { Mp4AtomTrunSamples } from "./Mp4AtomTrunSamples.ts";
export type Mp4AtomTrun = {
    dataOffset: number | undefined;
    samples: Mp4AtomTrunSamples;
};
export declare function debug(self: Mp4AtomTrun): string;
export declare function eq(a: Mp4AtomTrun, b: Mp4AtomTrun): boolean;
export declare function default$(): Mp4AtomTrun;
export { default$ as default };

// Mp4AtomTrunEntry.ts
import * as rt from "./rt.ts";
export type Mp4AtomTrunEntry = {
    duration: number | undefined;
    size: number | undefined;
    flags: number | undefined;
    cts: rt.i64 | undefined;
};
export declare function debug(self: Mp4AtomTrunEntry): string;
export declare function eq(a: Mp4AtomTrunEntry, b: Mp4AtomTrunEntry): boolean;
export declare function default$(): Mp4AtomTrunEntry;
export { default$ as default };

// Mp4AtomTrunSamples.ts
import type { Mp4AtomTrunEntry } from "./Mp4AtomTrunEntry.ts";
export type Mp4AtomTrunSamples = {
    tag: "Inherited";
    count: number;
    firstFlags: number | undefined;
} | {
    tag: "Explicit";
    _0: Mp4AtomTrunEntry[];
};
export declare function debug(self: Mp4AtomTrunSamples): string;
export declare function eq(a: Mp4AtomTrunSamples, b: Mp4AtomTrunSamples): boolean;
export declare function default$(): Mp4AtomTrunSamples;
export declare function len(self: Mp4AtomTrunSamples): number;
export declare function isEmpty(self: Mp4AtomTrunSamples): boolean;
export declare function get(self: Mp4AtomTrunSamples, index: number): Mp4AtomTrunEntry | undefined;
export { default$ as default };

// Mp4AtomTwos.ts
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
export type Mp4AtomTwos = {
    audio: Mp4AtomAudio;
    pcmc: Mp4AtomPcmC | undefined;
    chnl: Mp4AtomChnl | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomTwos): string;
export declare function eq(a: Mp4AtomTwos, b: Mp4AtomTwos): boolean;
export declare function format(self: Mp4AtomTwos): Mp4AtomPcmFormat | undefined;

// Mp4AtomTx3g.ts
import * as rt from "./rt.ts";
import type { Mp4AtomDisp } from "./Mp4AtomDisp.ts";
import type { Mp4AtomFtab } from "./Mp4AtomFtab.ts";
import type { Mp4AtomRgbaColor } from "./Mp4AtomRgbaColor.ts";
export type Mp4AtomTx3g = rt.Handle<"Mp4AtomTx3g">;
export declare function debug(self: Mp4AtomTx3g): string;
export declare function clone(self: Mp4AtomTx3g): Mp4AtomTx3g;
export declare function eq(a: Mp4AtomTx3g, b: Mp4AtomTx3g): boolean;
export declare function default$(): Mp4AtomTx3g;
export declare function dataReferenceIndexGet(self: Mp4AtomTx3g): number;
export declare function dataReferenceIndexSet(self: Mp4AtomTx3g, value: number): void;
export declare function displayFlagsGet(self: Mp4AtomTx3g): number;
export declare function displayFlagsSet(self: Mp4AtomTx3g, value: number): void;
export declare function horizontalJustificationGet(self: Mp4AtomTx3g): number;
export declare function horizontalJustificationSet(self: Mp4AtomTx3g, value: number): void;
export declare function verticalJustificationGet(self: Mp4AtomTx3g): number;
export declare function verticalJustificationSet(self: Mp4AtomTx3g, value: number): void;
export declare function bgColorRgbaGet(self: Mp4AtomTx3g): Mp4AtomRgbaColor;
export declare function bgColorRgbaSet(self: Mp4AtomTx3g, value: Mp4AtomRgbaColor): void;
export declare function styleRecordGet(self: Mp4AtomTx3g): Uint8Array;
export declare function styleRecordSet(self: Mp4AtomTx3g, value: Uint8Array): void;
export declare function ftabGet(self: Mp4AtomTx3g): Mp4AtomFtab | undefined;
export declare function ftabSet(self: Mp4AtomTx3g, value: Mp4AtomFtab | undefined): void;
export declare function dispGet(self: Mp4AtomTx3g): Mp4AtomDisp | undefined;
export declare function dispSet(self: Mp4AtomTx3g, value: Mp4AtomDisp | undefined): void;
export { default$ as default };

// Mp4AtomTxtC.ts
export type Mp4AtomTxtC = {
    textConfig: string;
};
export declare function debug(self: Mp4AtomTxtC): string;
export declare function eq(a: Mp4AtomTxtC, b: Mp4AtomTxtC): boolean;

// Mp4AtomU24.ts
import * as rt from "./rt.ts";
export type Mp4AtomU24 = rt.Opaque<"Mp4AtomU24">;
export declare function debug(self: Mp4AtomU24): string;
export declare function eq(a: Mp4AtomU24, b: Mp4AtomU24): boolean;
export declare function default$(): Mp4AtomU24;
export { default$ as default };

// Mp4AtomUdta.ts
import * as rt from "./rt.ts";
import type { Mp4AtomChpl } from "./Mp4AtomChpl.ts";
import type { Mp4AtomCprt } from "./Mp4AtomCprt.ts";
import type { Mp4AtomKind } from "./Mp4AtomKind.ts";
import type { Mp4AtomMeta } from "./Mp4AtomMeta.ts";
import type { Mp4AtomRtng } from "./Mp4AtomRtng.ts";
export type Mp4AtomUdta = rt.Handle<"Mp4AtomUdta">;
export declare function debug(self: Mp4AtomUdta): string;
export declare function clone(self: Mp4AtomUdta): Mp4AtomUdta;
export declare function eq(a: Mp4AtomUdta, b: Mp4AtomUdta): boolean;
export declare function default$(): Mp4AtomUdta;
export declare function chplGet(self: Mp4AtomUdta): Mp4AtomChpl | undefined;
export declare function chplSet(self: Mp4AtomUdta, value: Mp4AtomChpl | undefined): void;
export declare function cprtGet(self: Mp4AtomUdta): Mp4AtomCprt[];
export declare function cprtSet(self: Mp4AtomUdta, value: Mp4AtomCprt[]): void;
export declare function kindGet(self: Mp4AtomUdta): Mp4AtomKind[];
export declare function kindSet(self: Mp4AtomUdta, value: Mp4AtomKind[]): void;
export declare function metaGet(self: Mp4AtomUdta): Mp4AtomMeta | undefined;
export declare function metaSet(self: Mp4AtomUdta, value: Mp4AtomMeta | undefined): void;
export declare function rtngGet(self: Mp4AtomUdta): Mp4AtomRtng[];
export declare function rtngSet(self: Mp4AtomUdta, value: Mp4AtomRtng[]): void;
export { default$ as default };

// Mp4AtomUncC.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomUncompressedComponent } from "./Mp4AtomUncompressedComponent.ts";
export type Mp4AtomUncC = {
    tag: "V1";
    profile: Mp4AtomFourCc;
} | {
    tag: "V0";
    profile: Mp4AtomFourCc;
    components: Mp4AtomUncompressedComponent[];
    samplingType: number;
    interleaveType: number;
    blockSize: number;
    componentsLittleEndian: boolean;
    blockPadLsb: boolean;
    blockLittleEndian: boolean;
    blockReversed: boolean;
    padUnknown: boolean;
    pixelSize: number;
    rowAlignSize: number;
    tileAlignSize: number;
    numTileColsMinusOne: number;
    numTileRowsMinusOne: number;
};
export declare function debug(self: Mp4AtomUncC): string;
export declare function eq(a: Mp4AtomUncC, b: Mp4AtomUncC): boolean;

// Mp4AtomUncompressedComponent.ts
export type Mp4AtomUncompressedComponent = {
    componentIndex: number;
    componentBitDepthMinusOne: number;
    componentFormat: number;
    componentAlignSize: number;
};
export declare function debug(self: Mp4AtomUncompressedComponent): string;
export declare function eq(a: Mp4AtomUncompressedComponent, b: Mp4AtomUncompressedComponent): boolean;

// Mp4AtomUri.ts
export type Mp4AtomUri = {
    theUri: string;
};
export declare function debug(self: Mp4AtomUri): string;
export declare function eq(a: Mp4AtomUri, b: Mp4AtomUri): boolean;

// Mp4AtomUriI.ts
export type Mp4AtomUriI = {
    uriInitializationData: Uint8Array;
};
export declare function debug(self: Mp4AtomUriI): string;
export declare function eq(a: Mp4AtomUriI, b: Mp4AtomUriI): boolean;

// Mp4AtomUrim.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
import type { Mp4AtomUri } from "./Mp4AtomUri.ts";
import type { Mp4AtomUriI } from "./Mp4AtomUriI.ts";
export type Mp4AtomUrim = {
    metadata: Mp4AtomMetaData;
    theLabel: Mp4AtomUri;
    init: Mp4AtomUriI | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomUrim): string;
export declare function eq(a: Mp4AtomUrim, b: Mp4AtomUrim): boolean;

// Mp4AtomUrl.ts
export type Mp4AtomUrl = {
    location: string;
};
export declare function debug(self: Mp4AtomUrl): string;
export declare function eq(a: Mp4AtomUrl, b: Mp4AtomUrl): boolean;
export declare function default$(): Mp4AtomUrl;
export { default$ as default };

// Mp4AtomUrn.ts
export type Mp4AtomUrn = {
    name: string;
    location: string;
};
export declare function debug(self: Mp4AtomUrn): string;
export declare function eq(a: Mp4AtomUrn, b: Mp4AtomUrn): boolean;
export declare function default$(): Mp4AtomUrn;
export { default$ as default };

// Mp4AtomUuid.ts
import type { Mp4AtomC2pa } from "./Mp4AtomC2pa.ts";
import type { Mp4AtomExtendedType } from "./Mp4AtomExtendedType.ts";
export type Mp4AtomUuid = {
    tag: "C2pa";
    _0: Mp4AtomC2pa;
} | {
    tag: "Unknown";
    _0: Mp4AtomExtendedType;
    _1: Uint8Array;
};
export declare function debug(self: Mp4AtomUuid): string;
export declare function eq(a: Mp4AtomUuid, b: Mp4AtomUuid): boolean;

// Mp4AtomVexu.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomEyes } from "./Mp4AtomEyes.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
import type { Mp4AtomPack } from "./Mp4AtomPack.ts";
import type { Mp4AtomVexuProj } from "./Mp4AtomVexuProj.ts";
export type Mp4AtomVexu = rt.Handle<"Mp4AtomVexu">;
export declare function debug(self: Mp4AtomVexu): string;
export declare function clone(self: Mp4AtomVexu): Mp4AtomVexu;
export declare function eq(a: Mp4AtomVexu, b: Mp4AtomVexu): boolean;
export declare function default$(): Mp4AtomVexu;
export declare function mustGet(self: Mp4AtomVexu): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomVexu, value: Mp4AtomMust | undefined): void;
export declare function projGet(self: Mp4AtomVexu): Mp4AtomVexuProj | undefined;
export declare function projSet(self: Mp4AtomVexu, value: Mp4AtomVexuProj | undefined): void;
export declare function eyesGet(self: Mp4AtomVexu): Mp4AtomEyes | undefined;
export declare function eyesSet(self: Mp4AtomVexu, value: Mp4AtomEyes | undefined): void;
export declare function packGet(self: Mp4AtomVexu): Mp4AtomPack | undefined;
export declare function packSet(self: Mp4AtomVexu, value: Mp4AtomPack | undefined): void;
export declare function unknownGet(self: Mp4AtomVexu): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomVexu, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomVexuProj.ts
import * as rt from "./rt.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
import type { Mp4AtomPrji } from "./Mp4AtomPrji.ts";
import type { Mp4AtomVexuProjKind } from "./Mp4AtomVexuProjKind.ts";
export type Mp4AtomVexuProj = rt.Handle<"Mp4AtomVexuProj">;
export declare function debug(self: Mp4AtomVexuProj): string;
export declare function clone(self: Mp4AtomVexuProj): Mp4AtomVexuProj;
export declare function eq(a: Mp4AtomVexuProj, b: Mp4AtomVexuProj): boolean;
export declare function default$(): Mp4AtomVexuProj;
export declare function mustGet(self: Mp4AtomVexuProj): Mp4AtomMust | undefined;
export declare function mustSet(self: Mp4AtomVexuProj, value: Mp4AtomMust | undefined): void;
export declare function prjiGet(self: Mp4AtomVexuProj): Mp4AtomPrji | undefined;
export declare function prjiSet(self: Mp4AtomVexuProj, value: Mp4AtomPrji | undefined): void;
export declare function kindGet(self: Mp4AtomVexuProj): Mp4AtomVexuProjKind | undefined;
export declare function kindSet(self: Mp4AtomVexuProj, value: Mp4AtomVexuProjKind | undefined): void;
export declare function unknownGet(self: Mp4AtomVexuProj): Mp4AtomAny[];
export declare function unknownSet(self: Mp4AtomVexuProj, value: Mp4AtomAny[]): void;
export { default$ as default };

// Mp4AtomVexuProjKind.ts
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
export type Mp4AtomVexuProjKind = "Rectilinear" | "Equirectangular" | "HalfEquirectangular" | "Fisheye" | "ParametricImmersive";
export declare function debug(self: Mp4AtomVexuProjKind): string;
export declare function kind(self: Mp4AtomVexuProjKind): Mp4AtomFourCc;

// Mp4AtomVisual.ts
import * as rt from "./rt.ts";
import type { Mp4AtomCompressor } from "./Mp4AtomCompressor.ts";
import type { Mp4AtomFixedPointU16 } from "./Mp4AtomFixedPointU16.ts";
export type Mp4AtomVisual = rt.Handle<"Mp4AtomVisual">;
export declare function debug(self: Mp4AtomVisual): string;
export declare function clone(self: Mp4AtomVisual): Mp4AtomVisual;
export declare function eq(a: Mp4AtomVisual, b: Mp4AtomVisual): boolean;
export declare function default$(): Mp4AtomVisual;
export declare function dataReferenceIndexGet(self: Mp4AtomVisual): number;
export declare function dataReferenceIndexSet(self: Mp4AtomVisual, value: number): void;
export declare function widthGet(self: Mp4AtomVisual): number;
export declare function widthSet(self: Mp4AtomVisual, value: number): void;
export declare function heightGet(self: Mp4AtomVisual): number;
export declare function heightSet(self: Mp4AtomVisual, value: number): void;
export declare function horizresolutionGet(self: Mp4AtomVisual): Mp4AtomFixedPointU16;
export declare function horizresolutionSet(self: Mp4AtomVisual, value: Mp4AtomFixedPointU16): void;
export declare function vertresolutionGet(self: Mp4AtomVisual): Mp4AtomFixedPointU16;
export declare function vertresolutionSet(self: Mp4AtomVisual, value: Mp4AtomFixedPointU16): void;
export declare function frameCountGet(self: Mp4AtomVisual): number;
export declare function frameCountSet(self: Mp4AtomVisual, value: number): void;
export declare function compressorGet(self: Mp4AtomVisual): Mp4AtomCompressor;
export declare function compressorSet(self: Mp4AtomVisual, value: Mp4AtomCompressor): void;
export declare function depthGet(self: Mp4AtomVisual): number;
export declare function depthSet(self: Mp4AtomVisual, value: number): void;
export { default$ as default };

// Mp4AtomVlab.ts
export type Mp4AtomVlab = {
    sourceLabel: string;
};
export declare function debug(self: Mp4AtomVlab): string;
export declare function eq(a: Mp4AtomVlab, b: Mp4AtomVlab): boolean;

// Mp4AtomVmhd.ts
import type { Mp4AtomRgbColor } from "./Mp4AtomRgbColor.ts";
export type Mp4AtomVmhd = {
    graphicsMode: number;
    opColor: Mp4AtomRgbColor;
};
export declare function debug(self: Mp4AtomVmhd): string;
export declare function eq(a: Mp4AtomVmhd, b: Mp4AtomVmhd): boolean;
export declare function default$(): Mp4AtomVmhd;
export { default$ as default };

// Mp4AtomVp08.ts
import * as rt from "./rt.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
import type { Mp4AtomVpcC } from "./Mp4AtomVpcC.ts";
export type Mp4AtomVp08 = rt.Handle<"Mp4AtomVp08">;
export declare function debug(self: Mp4AtomVp08): string;
export declare function clone(self: Mp4AtomVp08): Mp4AtomVp08;
export declare function eq(a: Mp4AtomVp08, b: Mp4AtomVp08): boolean;
export declare function default$(): Mp4AtomVp08;
export declare function visualGet(self: Mp4AtomVp08): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomVp08, value: Mp4AtomVisual): void;
export declare function vpccGet(self: Mp4AtomVp08): Mp4AtomVpcC;
export declare function vpccSet(self: Mp4AtomVp08, value: Mp4AtomVpcC): void;
export declare function btrtGet(self: Mp4AtomVp08): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomVp08, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomVp08): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomVp08, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomVp08): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomVp08, value: Mp4AtomPasp | undefined): void;
export declare function spatialGet(self: Mp4AtomVp08): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomVp08, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomVp09.ts
import * as rt from "./rt.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomSpatial } from "./Mp4AtomSpatial.ts";
import type { Mp4AtomVisual } from "./Mp4AtomVisual.ts";
import type { Mp4AtomVpcC } from "./Mp4AtomVpcC.ts";
export type Mp4AtomVp09 = rt.Handle<"Mp4AtomVp09">;
export declare function debug(self: Mp4AtomVp09): string;
export declare function clone(self: Mp4AtomVp09): Mp4AtomVp09;
export declare function eq(a: Mp4AtomVp09, b: Mp4AtomVp09): boolean;
export declare function default$(): Mp4AtomVp09;
export declare function visualGet(self: Mp4AtomVp09): Mp4AtomVisual;
export declare function visualSet(self: Mp4AtomVp09, value: Mp4AtomVisual): void;
export declare function vpccGet(self: Mp4AtomVp09): Mp4AtomVpcC;
export declare function vpccSet(self: Mp4AtomVp09, value: Mp4AtomVpcC): void;
export declare function btrtGet(self: Mp4AtomVp09): Mp4AtomBtrt | undefined;
export declare function btrtSet(self: Mp4AtomVp09, value: Mp4AtomBtrt | undefined): void;
export declare function colrGet(self: Mp4AtomVp09): Mp4AtomColr | undefined;
export declare function colrSet(self: Mp4AtomVp09, value: Mp4AtomColr | undefined): void;
export declare function paspGet(self: Mp4AtomVp09): Mp4AtomPasp | undefined;
export declare function paspSet(self: Mp4AtomVp09, value: Mp4AtomPasp | undefined): void;
export declare function spatialGet(self: Mp4AtomVp09): Mp4AtomSpatial | undefined;
export declare function spatialSet(self: Mp4AtomVp09, value: Mp4AtomSpatial | undefined): void;
export { default$ as default };

// Mp4AtomVpcC.ts
export type Mp4AtomVpcC = {
    profile: number;
    level: number;
    bitDepth: number;
    chromaSubsampling: number;
    videoFullRangeFlag: boolean;
    colorPrimaries: number;
    transferCharacteristics: number;
    matrixCoefficients: number;
    codecInitializationData: Uint8Array;
};
export declare function debug(self: Mp4AtomVpcC): string;
export declare function eq(a: Mp4AtomVpcC, b: Mp4AtomVpcC): boolean;
export declare function default$(): Mp4AtomVpcC;
export { default$ as default };

// Mp4AtomVttC.ts
export type Mp4AtomVttC = {
    config: string;
};
export declare function debug(self: Mp4AtomVttC): string;
export declare function eq(a: Mp4AtomVttC, b: Mp4AtomVttC): boolean;

// Mp4AtomWvtt.ts
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomPlainText } from "./Mp4AtomPlainText.ts";
import type { Mp4AtomVlab } from "./Mp4AtomVlab.ts";
import type { Mp4AtomVttC } from "./Mp4AtomVttC.ts";
export type Mp4AtomWvtt = {
    plaintext: Mp4AtomPlainText;
    config: Mp4AtomVttC;
    label: Mp4AtomVlab | undefined;
    btrt: Mp4AtomBtrt | undefined;
};
export declare function debug(self: Mp4AtomWvtt): string;
export declare function eq(a: Mp4AtomWvtt, b: Mp4AtomWvtt): boolean;

// Mp4AtomYear.ts
export type Mp4AtomYear = {
    _0: string;
};
export declare function debug(self: Mp4AtomYear): string;
export declare function eq(a: Mp4AtomYear, b: Mp4AtomYear): boolean;

// Mp4AtomZeroed.ts
export type Mp4AtomZeroed = {
    size: number;
};
export declare function debug(self: Mp4AtomZeroed): string;
export declare function eq(a: Mp4AtomZeroed, b: Mp4AtomZeroed): boolean;
export declare function default$(): Mp4AtomZeroed;
export declare function new$(size: number): Mp4AtomZeroed;
export { default$ as default, new$ as new };

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
export declare function framerateGet(self: MuxCatalogEstimate): number | undefined;
export declare function framerateSet(self: MuxCatalogEstimate, value: number | undefined): void;
export declare function withJitter(self: MuxCatalogEstimate, jitter: rt.Duration | undefined): MuxCatalogEstimate;
export declare function withBitrate(self: MuxCatalogEstimate, bitrate: rt.u64 | undefined): MuxCatalogEstimate;
export declare function withDelay(self: MuxCatalogEstimate, delay: rt.Duration | undefined): MuxCatalogEstimate;
export declare function withFramerate(self: MuxCatalogEstimate, framerate: number | undefined): MuxCatalogEstimate;
export { default$ as default };

// MuxCatalogEstimator.ts
import * as rt from "./rt.ts";
import type { MuxCatalogEstimate } from "./MuxCatalogEstimate.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxCatalogEstimator = rt.Handle<"MuxCatalogEstimator">;
export declare function default$(): MuxCatalogEstimator;
export declare function new$(): MuxCatalogEstimator;
export declare function write(self: MuxCatalogEstimator, timestamp: Timestamp, bytes: number): void;
export declare function cut(self: MuxCatalogEstimator, end: Timestamp | undefined): void;
export declare function discontinuity(self: MuxCatalogEstimator): void;
export declare function reorder(self: MuxCatalogEstimator, delay: Timestamp): void;
export declare function estimate(self: MuxCatalogEstimator): MuxCatalogEstimate;
export { default$ as default, new$ as new };

// MuxCatalogGuard.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
import type { HangCatalogVideo } from "./HangCatalogVideo.ts";
import type { HangError } from "./HangError.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxCatalogGuard = rt.Handle<"MuxCatalogGuard">;
export declare function commit(self: MuxCatalogGuard): rt.Result<undefined, MuxError>;
export declare function videoGet(self: MuxCatalogGuard): HangCatalogVideo;
export declare function videoSet(self: MuxCatalogGuard, value: HangCatalogVideo): void;
export declare function audioGet(self: MuxCatalogGuard): HangCatalogAudio;
export declare function audioSet(self: MuxCatalogGuard, value: HangCatalogAudio): void;
export declare function mediaGet(self: MuxCatalogGuard): HangCatalogMedia;
export declare function mediaSet(self: MuxCatalogGuard, value: HangCatalogMedia): void;
export declare function textGet(self: MuxCatalogGuard): HangCatalogText;
export declare function textSet(self: MuxCatalogGuard, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuard): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuard, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuard): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuard, value: HangCatalogBinary): void;
export declare function m2tsGet(self: MuxCatalogGuard): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: MuxCatalogGuard, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: MuxCatalogGuard): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuard, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuard): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuard, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: MuxCatalogGuard): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: MuxCatalogGuard, value: ([string, HangCatalogContribution])[]): void;
export declare function checkRenditions(self: MuxCatalogGuard): rt.Result<undefined, HangError>;
export declare function media(self: MuxCatalogGuard): HangCatalog;
export declare function toJson(self: MuxCatalogGuard): rt.Result<string, HangError>;
export declare function toJsonPretty(self: MuxCatalogGuard): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuard): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuard, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogGuardExt.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
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
export declare function mediaGet(self: MuxCatalogGuardExt): HangCatalogMedia;
export declare function mediaSet(self: MuxCatalogGuardExt, value: HangCatalogMedia): void;
export declare function textGet(self: MuxCatalogGuardExt): HangCatalogText;
export declare function textSet(self: MuxCatalogGuardExt, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuardExt): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuardExt, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuardExt): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuardExt, value: HangCatalogBinary): void;
export declare function m2tsGet(self: MuxCatalogGuardExt): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: MuxCatalogGuardExt, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: MuxCatalogGuardExt): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuardExt, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuardExt): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuardExt, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: MuxCatalogGuardExt): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: MuxCatalogGuardExt, value: ([string, HangCatalogContribution])[]): void;
export declare function extGet(self: MuxCatalogGuardExt): MuxContainerTsExt;
export declare function extSet(self: MuxCatalogGuardExt, value: MuxContainerTsExt): void;
export declare function checkRenditions(self: MuxCatalogGuardExt): rt.Result<undefined, HangError>;
export declare function media(self: MuxCatalogGuardExt): HangCatalog;
export declare function toJson(self: MuxCatalogGuardExt): rt.Result<string, HangError>;
export declare function toJsonPretty(self: MuxCatalogGuardExt): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuardExt): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuardExt, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogGuardExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAudio } from "./HangCatalogAudio.ts";
import type { HangCatalogBinary } from "./HangCatalogBinary.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogEcho } from "./HangCatalogEcho.ts";
import type { HangCatalogJson } from "./HangCatalogJson.ts";
import type { HangCatalogM2ts } from "./HangCatalogM2ts.ts";
import type { HangCatalogMedia } from "./HangCatalogMedia.ts";
import type { HangCatalogStats } from "./HangCatalogStats.ts";
import type { HangCatalogText } from "./HangCatalogText.ts";
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
export declare function mediaGet(self: MuxCatalogGuardExtra): HangCatalogMedia;
export declare function mediaSet(self: MuxCatalogGuardExtra, value: HangCatalogMedia): void;
export declare function textGet(self: MuxCatalogGuardExtra): HangCatalogText;
export declare function textSet(self: MuxCatalogGuardExtra, value: HangCatalogText): void;
export declare function jsonGet(self: MuxCatalogGuardExtra): HangCatalogJson;
export declare function jsonSet(self: MuxCatalogGuardExtra, value: HangCatalogJson): void;
export declare function binaryGet(self: MuxCatalogGuardExtra): HangCatalogBinary;
export declare function binarySet(self: MuxCatalogGuardExtra, value: HangCatalogBinary): void;
export declare function m2tsGet(self: MuxCatalogGuardExtra): HangCatalogM2ts | undefined;
export declare function m2tsSet(self: MuxCatalogGuardExtra, value: HangCatalogM2ts | undefined): void;
export declare function statsGet(self: MuxCatalogGuardExtra): HangCatalogStats | undefined;
export declare function statsSet(self: MuxCatalogGuardExtra, value: HangCatalogStats | undefined): void;
export declare function echoGet(self: MuxCatalogGuardExtra): HangCatalogEcho | undefined;
export declare function echoSet(self: MuxCatalogGuardExtra, value: HangCatalogEcho | undefined): void;
export declare function contributionsGet(self: MuxCatalogGuardExtra): ([string, HangCatalogContribution])[];
export declare function contributionsSet(self: MuxCatalogGuardExtra, value: ([string, HangCatalogContribution])[]): void;
export declare function extGet(self: MuxCatalogGuardExtra): MuxCatalogHangExtra;
export declare function extSet(self: MuxCatalogGuardExtra, value: MuxCatalogHangExtra): void;
export declare function checkRenditions(self: MuxCatalogGuardExtra): rt.Result<undefined, HangError>;
export declare function media(self: MuxCatalogGuardExtra): HangCatalog;
export declare function toJson(self: MuxCatalogGuardExtra): rt.Result<string, HangError>;
export declare function toJsonPretty(self: MuxCatalogGuardExtra): rt.Result<string, HangError>;
export declare function toVec(self: MuxCatalogGuardExtra): rt.Result<rt.Bytes, HangError>;
export declare function toWriter(self: MuxCatalogGuardExtra, writer: Uint8Array | rt.Bytes): rt.Result<undefined, HangError>;

// MuxCatalogHangConsumer.ts
import * as rt from "./rt.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogHangConsumer = rt.Handle<"MuxCatalogHangConsumer">;
export declare function new$(track: TrackSubscriber): MuxCatalogHangConsumer;
export declare function next(self: MuxCatalogHangConsumer): rt.Call<rt.Result<HangCatalog | undefined, MuxError>>;
export { new$ as new };

// MuxCatalogHangConsumerExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogHangConsumerExt = rt.Handle<"MuxCatalogHangConsumerExt">;
export declare function new$(track: TrackSubscriber): MuxCatalogHangConsumerExt;
export declare function next(self: MuxCatalogHangConsumerExt): rt.Call<rt.Result<HangCatalogExt | undefined, MuxError>>;
export { new$ as new };

// MuxCatalogHangConsumerExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogHangConsumerExtra = rt.Handle<"MuxCatalogHangConsumerExtra">;
export declare function new$(track: TrackSubscriber): MuxCatalogHangConsumerExtra;
export declare function next(self: MuxCatalogHangConsumerExtra): rt.Call<rt.Result<HangCatalogExtra | undefined, MuxError>>;
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
export declare function len(self: MuxCatalogHangExtra): number;
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
export { new$ as new };

// MuxCatalogMsfConsumerExt.ts
import * as rt from "./rt.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogMsfConsumerExt = rt.Handle<"MuxCatalogMsfConsumerExt">;
export declare function new$(track: TrackSubscriber): MuxCatalogMsfConsumerExt;
export declare function next(self: MuxCatalogMsfConsumerExt): rt.Call<rt.Result<HangCatalogExt | undefined, MuxError>>;
export { new$ as new };

// MuxCatalogMsfConsumerExtra.ts
import * as rt from "./rt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxCatalogMsfConsumerExtra = rt.Handle<"MuxCatalogMsfConsumerExtra">;
export declare function new$(track: TrackSubscriber): MuxCatalogMsfConsumerExtra;
export declare function next(self: MuxCatalogMsfConsumerExtra): rt.Call<rt.Result<HangCatalogExtra | undefined, MuxError>>;
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
export declare function framerateGet(self: MuxCatalogVideoHint): number | undefined;
export declare function framerateSet(self: MuxCatalogVideoHint, value: number | undefined): void;
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
export declare function toConfig(self: MuxCatalogVideoHint): HangCatalogVideoConfig | undefined;
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
export type MuxCodecAacAdtsError = "Truncated" | "Syncword" | "SampleRate" | "FrameLength" | "PacketLength";
export declare function toString(self: MuxCodecAacAdtsError): string;
export declare function debug(self: MuxCodecAacAdtsError): string;

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
export type MuxCodecAacConfig = {
    profile: number;
    sampleRate: number;
    channelCount: number;
};
export declare function parse(buf: rt.Place<Uint8Array>): rt.Result<MuxCodecAacConfig, MuxCodecAacError>;
export declare function encode(self: MuxCodecAacConfig): rt.Result<rt.Bytes, MuxCodecAacError>;

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
export declare function finish(self: MuxCodecAacImport): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecAacImport, err: Error_): void;
export declare function cut(self: MuxCodecAacImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecAacImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecAacImport, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
export declare function finish(self: MuxCodecAv1Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecAv1Import, err: Error_): void;
export declare function cut(self: MuxCodecAv1Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecAv1Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecAv1Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecAv1Import, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecFlacImport = rt.Handle<"MuxCodecFlacImport">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecFlacImport, MuxError>;
export declare function demand(self: MuxCodecFlacImport): TrackDemand;
export declare function finish(self: MuxCodecFlacImport): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecFlacImport, err: Error_): void;
export declare function cut(self: MuxCodecFlacImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecFlacImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecFlacImport, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
export declare function lengthSizeGet(self: MuxCodecH264Avcc): number;
export declare function lengthSizeSet(self: MuxCodecH264Avcc, value: number): void;
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
export declare function finish(self: MuxCodecH264Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecH264Import, err: Error_): void;
export declare function cut(self: MuxCodecH264Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecH264Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecH264Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function reportSkipped(self: MuxCodecH264Import, frames: rt.u64): void;
export declare function reportBitrate(self: MuxCodecH264Import, bitrate: BandwidthRate): void;
export declare function seek(self: MuxCodecH264Import, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
export declare function lengthSizeGet(self: MuxCodecH265Hvcc): number;
export declare function lengthSizeSet(self: MuxCodecH265Hvcc, value: number): void;
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
export declare function finish(self: MuxCodecH265Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecH265Import, err: Error_): void;
export declare function cut(self: MuxCodecH265Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecH265Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecH265Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function reportSkipped(self: MuxCodecH265Import, frames: rt.u64): void;
export declare function reportBitrate(self: MuxCodecH265Import, bitrate: BandwidthRate): void;
export declare function seek(self: MuxCodecH265Import, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
import type { MuxError } from "./MuxError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackDemand } from "./TrackDemand.ts";
import type { TrackProducer } from "./TrackProducer.ts";
export type MuxCodecMp3Import = rt.Handle<"MuxCodecMp3Import">;
export declare function newExtra(track: TrackProducer, reserved: MuxCatalogReservedExtra, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function newExt(track: TrackProducer, reserved: MuxCatalogReservedExt, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function newUnit(track: TrackProducer, reserved: MuxCatalogReserved, config: HangCatalogAudioConfig): rt.Result<MuxCodecMp3Import, MuxError>;
export declare function demand(self: MuxCodecMp3Import): TrackDemand;
export declare function finish(self: MuxCodecMp3Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecMp3Import, err: Error_): void;
export declare function cut(self: MuxCodecMp3Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecMp3Import): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecMp3Import, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
export declare function finish(self: MuxCodecOpusImport): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecOpusImport, err: Error_): void;
export declare function cut(self: MuxCodecOpusImport, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecOpusImport): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecOpusImport, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
export declare function finish(self: MuxCodecVp8Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecVp8Import, err: Error_): void;
export declare function cut(self: MuxCodecVp8Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecVp8Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecVp8Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecVp8Import, sequence: rt.u64): rt.Result<undefined, MuxError>;

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
export declare function finish(self: MuxCodecVp9Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxCodecVp9Import, err: Error_): void;
export declare function cut(self: MuxCodecVp9Import, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxCodecVp9Import): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxCodecVp9Import, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxCodecVp9Import, sequence: rt.u64): rt.Result<undefined, MuxError>;

// MuxContainerConsumerContainer.ts
import * as rt from "./rt.ts";
import type { MuxCatalogHangContainer } from "./MuxCatalogHangContainer.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerContainer = rt.Handle<"MuxContainerConsumerContainer">;
export declare function new$(track: TrackSubscriber, format: MuxCatalogHangContainer): MuxContainerConsumerContainer;
export declare function withWarmup(self: MuxContainerConsumerContainer, warmup: rt.Duration): MuxContainerConsumerContainer;
export declare function withDelay(self: MuxContainerConsumerContainer, delay: TrackDelay): MuxContainerConsumerContainer;
export declare function read(self: MuxContainerConsumerContainer): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function setDelay(self: MuxContainerConsumerContainer, delay: TrackDelay): void;
export declare function setMaxBuffer(self: MuxContainerConsumerContainer, bytes: number): void;
export { new$ as new };

// MuxContainerConsumerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerFmp4Wire = rt.Handle<"MuxContainerConsumerFmp4Wire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerFmp4Wire): MuxContainerConsumerFmp4Wire;
export declare function withWarmup(self: MuxContainerConsumerFmp4Wire, warmup: rt.Duration): MuxContainerConsumerFmp4Wire;
export declare function withDelay(self: MuxContainerConsumerFmp4Wire, delay: TrackDelay): MuxContainerConsumerFmp4Wire;
export declare function read(self: MuxContainerConsumerFmp4Wire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxContainerFmp4Error>>;
export declare function setDelay(self: MuxContainerConsumerFmp4Wire, delay: TrackDelay): void;
export declare function setMaxBuffer(self: MuxContainerConsumerFmp4Wire, bytes: number): void;
export { new$ as new };

// MuxContainerConsumerLegacyWire.ts
import * as rt from "./rt.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerLegacyWire = rt.Handle<"MuxContainerConsumerLegacyWire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerLegacyWire): MuxContainerConsumerLegacyWire;
export declare function withWarmup(self: MuxContainerConsumerLegacyWire, warmup: rt.Duration): MuxContainerConsumerLegacyWire;
export declare function withDelay(self: MuxContainerConsumerLegacyWire, delay: TrackDelay): MuxContainerConsumerLegacyWire;
export declare function read(self: MuxContainerConsumerLegacyWire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function setDelay(self: MuxContainerConsumerLegacyWire, delay: TrackDelay): void;
export declare function setMaxBuffer(self: MuxContainerConsumerLegacyWire, bytes: number): void;
export { new$ as new };

// MuxContainerConsumerLocWire.ts
import * as rt from "./rt.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxError } from "./MuxError.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxContainerConsumerLocWire = rt.Handle<"MuxContainerConsumerLocWire">;
export declare function new$(track: TrackSubscriber, format: MuxContainerLocWire): MuxContainerConsumerLocWire;
export declare function withWarmup(self: MuxContainerConsumerLocWire, warmup: rt.Duration): MuxContainerConsumerLocWire;
export declare function withDelay(self: MuxContainerConsumerLocWire, delay: TrackDelay): MuxContainerConsumerLocWire;
export declare function read(self: MuxContainerConsumerLocWire): rt.Call<rt.Result<MuxContainerRead | undefined, MuxError>>;
export declare function setDelay(self: MuxContainerConsumerLocWire, delay: TrackDelay): void;
export declare function setMaxBuffer(self: MuxContainerConsumerLocWire, bytes: number): void;
export { new$ as new };

// MuxContainerFlvImport.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImport = rt.Handle<"MuxContainerFlvImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerFlvImport;
export declare function withContainer(self: MuxContainerFlvImport, container: HangCatalogContainer): MuxContainerFlvImport;
export declare function decode(self: MuxContainerFlvImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImport, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImport): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerFlvImport, err: Error_): void;
export { new$ as new };

// MuxContainerFlvImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImportExt = rt.Handle<"MuxContainerFlvImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerFlvImportExt;
export declare function withContainer(self: MuxContainerFlvImportExt, container: HangCatalogContainer): MuxContainerFlvImportExt;
export declare function decode(self: MuxContainerFlvImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImportExt, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImportExt): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerFlvImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerFlvImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFlvImportExtra = rt.Handle<"MuxContainerFlvImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerFlvImportExtra;
export declare function withContainer(self: MuxContainerFlvImportExtra, container: HangCatalogContainer): MuxContainerFlvImportExtra;
export declare function decode(self: MuxContainerFlvImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerFlvImportExtra, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFlvImportExtra): rt.Result<undefined, MuxError>;
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

// MuxContainerFmp4Fragment.ts
import * as rt from "./rt.ts";
export type MuxContainerFmp4Fragment = {
    data: Uint8Array;
    independent: boolean;
    duration: rt.Duration;
};
export declare function debug(self: MuxContainerFmp4Fragment): string;

// MuxContainerFmp4FragmentConfig.ts
import * as rt from "./rt.ts";
import type { MuxContainerFmp4FragmentMissingDuration } from "./MuxContainerFmp4FragmentMissingDuration.ts";
export type MuxContainerFmp4FragmentConfig = rt.Opaque<"MuxContainerFmp4FragmentConfig">;
export declare function debug(self: MuxContainerFmp4FragmentConfig): string;
export declare function default$(): MuxContainerFmp4FragmentConfig;
export declare function missingDurationGet(self: MuxContainerFmp4FragmentConfig): MuxContainerFmp4FragmentMissingDuration;
export declare function missingDurationSet(self: MuxContainerFmp4FragmentConfig, value: MuxContainerFmp4FragmentMissingDuration): MuxContainerFmp4FragmentConfig;
export { default$ as default };

// MuxContainerFmp4FragmentMissingDuration.ts
export type MuxContainerFmp4FragmentMissingDuration = "Reject" | "InferFromPresentationTime";
export declare function debug(self: MuxContainerFmp4FragmentMissingDuration): string;
export declare function default$(): MuxContainerFmp4FragmentMissingDuration;
export { default$ as default };

// MuxContainerFmp4Fragmenter.ts
import * as rt from "./rt.ts";
import type { MuxContainerFmp4Fragment } from "./MuxContainerFmp4Fragment.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerFmp4Fragmenter = rt.Handle<"MuxContainerFmp4Fragmenter">;
export declare function push(self: MuxContainerFmp4Fragmenter, frame: MuxContainerFrame): rt.Result<MuxContainerFmp4Fragment[], MuxError>;
export declare function pushGroup(self: MuxContainerFmp4Fragmenter, frame: MuxContainerFrame): rt.Result<MuxContainerFmp4Fragment[], MuxError>;
export declare function flush(self: MuxContainerFmp4Fragmenter): rt.Result<MuxContainerFmp4Fragment | undefined, MuxError>;

// MuxContainerFmp4Import.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4Import = rt.Handle<"MuxContainerFmp4Import">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerFmp4Import;
export declare function cut(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4Import, select: MuxSelectBroadcast): MuxContainerFmp4Import;
export declare function withPassthrough(self: MuxContainerFmp4Import, passthrough: boolean): MuxContainerFmp4Import;
export declare function decode(self: MuxContainerFmp4Import, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerFmp4Import, err: Error_): void;
export declare function seek(self: MuxContainerFmp4Import, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4Import): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4ImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4ImportExt = rt.Handle<"MuxContainerFmp4ImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerFmp4ImportExt;
export declare function cut(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4ImportExt, select: MuxSelectBroadcast): MuxContainerFmp4ImportExt;
export declare function withPassthrough(self: MuxContainerFmp4ImportExt, passthrough: boolean): MuxContainerFmp4ImportExt;
export declare function decode(self: MuxContainerFmp4ImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerFmp4ImportExt, err: Error_): void;
export declare function seek(self: MuxContainerFmp4ImportExt, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4ImportExt): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4ImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSelectBroadcast } from "./MuxSelectBroadcast.ts";
export type MuxContainerFmp4ImportExtra = rt.Handle<"MuxContainerFmp4ImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerFmp4ImportExtra;
export declare function cut(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export declare function withSelect(self: MuxContainerFmp4ImportExtra, select: MuxSelectBroadcast): MuxContainerFmp4ImportExtra;
export declare function withPassthrough(self: MuxContainerFmp4ImportExtra, passthrough: boolean): MuxContainerFmp4ImportExtra;
export declare function decode(self: MuxContainerFmp4ImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerFmp4ImportExtra, err: Error_): void;
export declare function seek(self: MuxContainerFmp4ImportExtra, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerFmp4ImportExtra): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxContainerFmp4Muxer.ts
import * as rt from "./rt.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { MuxContainerFmp4FragmentConfig } from "./MuxContainerFmp4FragmentConfig.ts";
import type { MuxContainerFmp4Fragmenter } from "./MuxContainerFmp4Fragmenter.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxContainerFmp4Muxer = rt.Handle<"MuxContainerFmp4Muxer">;
export declare function video(config: HangCatalogVideoConfig): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function audio(config: HangCatalogAudioConfig): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function timescale(self: MuxContainerFmp4Muxer): Timescale;
export declare function withTimescale(self: MuxContainerFmp4Muxer, timescale: Timescale): rt.Result<MuxContainerFmp4Muxer, MuxError>;
export declare function withFramerate(self: MuxContainerFmp4Muxer, framerate: number | undefined): MuxContainerFmp4Muxer;
export declare function read(self: MuxContainerFmp4Muxer, group: GroupConsumer): rt.Call<rt.Result<MuxContainerFrame[], MuxError>>;
export declare function init(self: MuxContainerFmp4Muxer): rt.Result<rt.Bytes | undefined, MuxError>;
export declare function fragment(self: MuxContainerFmp4Muxer, sequence: number, frames: MuxContainerFrame[], end: Timestamp | undefined): rt.Result<rt.Bytes, MuxError>;
export declare function fragmenter(self: MuxContainerFmp4Muxer, config: MuxContainerFmp4FragmentConfig): MuxContainerFmp4Fragmenter;

// MuxContainerFmp4Wire.ts
import * as rt from "./rt.ts";
import type { Mp4AtomTrak } from "./Mp4AtomTrak.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
export type MuxContainerFmp4Wire = rt.Handle<"MuxContainerFmp4Wire">;
export declare function new$(trak: Mp4AtomTrak): rt.Result<MuxContainerFmp4Wire, MuxContainerFmp4Error>;
export declare function fromInit(initData: Uint8Array | rt.Bytes): rt.Result<MuxContainerFmp4Wire, MuxContainerFmp4Error>;
export declare function trak(self: MuxContainerFmp4Wire): Mp4AtomTrak;
export { new$ as new };

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
export declare function sequence(self: MuxContainerGroupConsumerContainer): rt.u64;
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
export declare function sequence(self: MuxContainerGroupConsumerFmp4Wire): rt.u64;
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
export declare function sequence(self: MuxContainerGroupConsumerLegacyWire): rt.u64;
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
export declare function sequence(self: MuxContainerGroupConsumerLocWire): rt.u64;
export declare function read(self: MuxContainerGroupConsumerLocWire): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export { new$ as new };

// MuxContainerKind.ts
export type MuxContainerKind = "Audio" | "Video" | "Data";
export declare function debug(self: MuxContainerKind): string;

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

// MuxContainerMkvImport.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImport = rt.Handle<"MuxContainerMkvImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerMkvImport;
export declare function withContainer(self: MuxContainerMkvImport, container: HangCatalogContainer): MuxContainerMkvImport;
export declare function decode(self: MuxContainerMkvImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImport, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImport): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerMkvImport, err: Error_): void;
export { new$ as new };

// MuxContainerMkvImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImportExt = rt.Handle<"MuxContainerMkvImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerMkvImportExt;
export declare function withContainer(self: MuxContainerMkvImportExt, container: HangCatalogContainer): MuxContainerMkvImportExt;
export declare function decode(self: MuxContainerMkvImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImportExt, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImportExt): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerMkvImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerMkvImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerMkvImportExtra = rt.Handle<"MuxContainerMkvImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerMkvImportExtra;
export declare function withContainer(self: MuxContainerMkvImportExtra, container: HangCatalogContainer): MuxContainerMkvImportExtra;
export declare function decode(self: MuxContainerMkvImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxContainerMkvImportExtra, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerMkvImportExtra): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerMkvImportExtra, err: Error_): void;
export { new$ as new };

// MuxContainerMp4Finish.ts
import * as rt from "./rt.ts";
export type MuxContainerMp4Finish = {
    moovOffset: rt.u64;
    moov: Uint8Array;
    mdatOffset: rt.u64;
    mdat: Uint8Array;
};
export declare function debug(self: MuxContainerMp4Finish): string;

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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainer, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainer): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainer): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainer, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainer, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainer): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainer): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainer): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainer, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainer): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainer, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainer): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainer): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainer): boolean;
export declare function largest(self: MuxContainerProducerContainer): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainer, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainerAudioConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerAudioConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainerAudioConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainerAudioConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainerAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerAudioConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainerAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainerAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerAudioConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainerBinaryConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerBinaryConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainerBinaryConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainerBinaryConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainerBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerBinaryConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainerBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainerBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerBinaryConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainerJsonConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerJsonConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainerJsonConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainerJsonConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainerJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerJsonConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainerJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainerJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerJsonConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainerTextConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerTextConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainerTextConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainerTextConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainerTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerTextConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainerTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainerTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerTextConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerTextConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerContainerVideoConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerContainerVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerContainerVideoConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerContainerVideoConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerContainerVideoConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerContainerVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerContainerVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerContainerVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerContainerVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerContainerVideoConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerContainerVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerContainerVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerContainerVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerContainerVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerContainerVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerContainerVideoConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFmp4Wire } from "./MuxContainerFmp4Wire.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4Wire, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4Wire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4Wire): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4Wire, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4Wire, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4Wire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4Wire): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4Wire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4Wire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4Wire): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4Wire, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4Wire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4Wire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4Wire): boolean;
export declare function largest(self: MuxContainerProducerFmp4Wire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4Wire, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4WireAudioConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireAudioConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireAudioConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4WireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireAudioConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4WireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4WireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireAudioConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4WireBinaryConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireBinaryConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireBinaryConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4WireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4WireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4WireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireBinaryConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4WireJsonConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireJsonConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireJsonConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4WireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireJsonConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4WireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4WireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireJsonConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4WireTextConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireTextConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireTextConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4WireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireTextConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4WireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4WireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireTextConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerFmp4WireVideoConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerFmp4WireVideoConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerFmp4WireVideoConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerFmp4WireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerFmp4WireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerFmp4WireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerFmp4WireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerFmp4WireVideoConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerFmp4WireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerFmp4WireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerFmp4WireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerFmp4WireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerFmp4WireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerFmp4WireVideoConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWire, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWire): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWire, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWire, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWire): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWire): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWire, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWire): boolean;
export declare function largest(self: MuxContainerProducerLegacyWire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWire, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWireAudioConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireAudioConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireAudioConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireAudioConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireAudioConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWireBinaryConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireBinaryConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireBinaryConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireBinaryConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWireJsonConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireJsonConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireJsonConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireJsonConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireJsonConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWireTextConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireTextConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireTextConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireTextConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireTextConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLegacyWireVideoConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLegacyWireVideoConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLegacyWireVideoConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLegacyWireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLegacyWireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLegacyWireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLegacyWireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLegacyWireVideoConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLegacyWireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLegacyWireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLegacyWireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLegacyWireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerLegacyWireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLegacyWireVideoConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWire, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWire): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWire): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWire, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWire, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWire): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWire): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWire): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWire, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWire): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWire, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWire): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWire): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWire): boolean;
export declare function largest(self: MuxContainerProducerLocWire): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWire, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWireAudioConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireAudioConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireAudioConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireAudioConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWireAudioConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireAudioConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireAudioConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireAudioConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireAudioConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWireAudioConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWireAudioConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireAudioConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireAudioConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireAudioConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireAudioConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWireBinaryConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireBinaryConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireBinaryConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWireBinaryConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireBinaryConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireBinaryConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireBinaryConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireBinaryConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWireBinaryConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWireBinaryConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireBinaryConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireBinaryConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireBinaryConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireBinaryConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWireJsonConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireJsonConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireJsonConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireJsonConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWireJsonConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireJsonConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireJsonConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireJsonConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireJsonConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWireJsonConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWireJsonConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireJsonConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireJsonConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireJsonConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireJsonConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWireTextConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireTextConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireTextConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireTextConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireTextConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWireTextConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireTextConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireTextConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireTextConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireTextConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWireTextConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWireTextConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireTextConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireTextConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireTextConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireTextConfig, other: TrackProducer): boolean;
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
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxError } from "./MuxError.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function seek(self: MuxContainerProducerLocWireVideoConfig, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxContainerProducerLocWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerProducerLocWireVideoConfig): rt.Result<undefined, MuxError>;
export declare function fail(self: MuxContainerProducerLocWireVideoConfig, err: Error_): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerProducerLocWireVideoConfig, err: Error_): void;
export declare function consume(self: MuxContainerProducerLocWireVideoConfig): TrackSubscriber;
export declare function broadcast(self: MuxContainerProducerLocWireVideoConfig): BroadcastInfo;
export declare function info(self: MuxContainerProducerLocWireVideoConfig): TrackInfo;
export declare function createGroup(self: MuxContainerProducerLocWireVideoConfig, group: GroupInfo): rt.Result<GroupProducer, Error_>;
export declare function appendGroup(self: MuxContainerProducerLocWireVideoConfig): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: MuxContainerProducerLocWireVideoConfig, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: MuxContainerProducerLocWireVideoConfig): TrackPosition | undefined;
export declare function closed(self: MuxContainerProducerLocWireVideoConfig): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: MuxContainerProducerLocWireVideoConfig): boolean;
export declare function largest(self: MuxContainerProducerLocWireVideoConfig): TrackPosition | undefined;
export declare function isClone(self: MuxContainerProducerLocWireVideoConfig, other: TrackProducer): boolean;
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

// MuxContainerTsExportConsumer.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { MuxCatalogConsumer } from "./MuxCatalogConsumer.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsExport } from "./MuxContainerTsStatsExport.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsExportConsumer = rt.Handle<"MuxContainerTsExportConsumer">;
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumer): MuxContainerTsExportConsumer;
export declare function withMuxRate(self: MuxContainerTsExportConsumer, muxRate: rt.u64): MuxContainerTsExportConsumer;
export declare function withDelay(self: MuxContainerTsExportConsumer, delay: rt.Duration): MuxContainerTsExportConsumer;
export declare function next(self: MuxContainerTsExportConsumer): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumer): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumer): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumer, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumer): rt.Result<MuxContainerTsExportConsumer, MuxError>;
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
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumerExt): MuxContainerTsExportConsumerExt;
export declare function withMuxRate(self: MuxContainerTsExportConsumerExt, muxRate: rt.u64): MuxContainerTsExportConsumerExt;
export declare function withDelay(self: MuxContainerTsExportConsumerExt, delay: rt.Duration): MuxContainerTsExportConsumerExt;
export declare function next(self: MuxContainerTsExportConsumerExt): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumerExt): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumerExt): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumerExt, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumerExt): rt.Result<MuxContainerTsExportConsumerExt, MuxError>;
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
export declare function new$(source: MuxSource, catalog: MuxCatalogConsumerExtra): MuxContainerTsExportConsumerExtra;
export declare function withMuxRate(self: MuxContainerTsExportConsumerExtra, muxRate: rt.u64): MuxContainerTsExportConsumerExtra;
export declare function withDelay(self: MuxContainerTsExportConsumerExtra, delay: rt.Duration): MuxContainerTsExportConsumerExtra;
export declare function next(self: MuxContainerTsExportConsumerExtra): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export declare function stats(self: MuxContainerTsExportConsumerExtra): MuxContainerTsStatsExport;
export declare function discontinuity(self: MuxContainerTsExportConsumerExtra): rt.u64;
export declare function follow(self: MuxContainerTsExportConsumerExtra, broadcast: BroadcastConsumer, catalog: MuxCatalogConsumerExtra): rt.Result<MuxContainerTsExportConsumerExtra, MuxError>;
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
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImport = rt.Handle<"MuxContainerTsImport">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): MuxContainerTsImport;
export declare function withProgram(self: MuxContainerTsImport, program: number): MuxContainerTsImport;
export declare function withContainer(self: MuxContainerTsImport, container: HangCatalogContainer): MuxContainerTsImport;
export declare function decode(self: MuxContainerTsImport, data: Uint8Array | rt.Bytes): rt.Result<undefined, string>;
export declare function seek(self: MuxContainerTsImport, sequence: rt.u64): rt.Result<undefined, string>;
export declare function finish(self: MuxContainerTsImport): rt.Result<undefined, string>;
export declare function stats(self: MuxContainerTsImport): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImport, err: Error_): void;
export { new$ as new };

// MuxContainerTsImportExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImportExt = rt.Handle<"MuxContainerTsImportExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): MuxContainerTsImportExt;
export declare function withProgram(self: MuxContainerTsImportExt, program: number): MuxContainerTsImportExt;
export declare function withContainer(self: MuxContainerTsImportExt, container: HangCatalogContainer): MuxContainerTsImportExt;
export declare function decode(self: MuxContainerTsImportExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, string>;
export declare function seek(self: MuxContainerTsImportExt, sequence: rt.u64): rt.Result<undefined, string>;
export declare function finish(self: MuxContainerTsImportExt): rt.Result<undefined, string>;
export declare function stats(self: MuxContainerTsImportExt): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImportExt, err: Error_): void;
export { new$ as new };

// MuxContainerTsImportExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { HangCatalogContainer } from "./HangCatalogContainer.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
export type MuxContainerTsImportExtra = rt.Handle<"MuxContainerTsImportExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): MuxContainerTsImportExtra;
export declare function withProgram(self: MuxContainerTsImportExtra, program: number): MuxContainerTsImportExtra;
export declare function withContainer(self: MuxContainerTsImportExtra, container: HangCatalogContainer): MuxContainerTsImportExtra;
export declare function decode(self: MuxContainerTsImportExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, string>;
export declare function seek(self: MuxContainerTsImportExtra, sequence: rt.u64): rt.Result<undefined, string>;
export declare function finish(self: MuxContainerTsImportExtra): rt.Result<undefined, string>;
export declare function stats(self: MuxContainerTsImportExtra): MuxContainerTsStatsSnapshot;
export declare function abort(self: MuxContainerTsImportExtra, err: Error_): void;
export { new$ as new };

// MuxContainerTsMpegts.ts
import * as rt from "./rt.ts";
import type { MuxContainerTsDescriptor } from "./MuxContainerTsDescriptor.ts";
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
export declare function isEmpty(self: MuxContainerTsMpegts): boolean;
export { default$ as default };

// MuxContainerTsPassthrough.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthrough = rt.Handle<"MuxContainerTsPassthrough">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved): rt.Result<MuxContainerTsPassthrough, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthrough, pid: number): MuxContainerTsPassthrough;
export declare function track(self: MuxContainerTsPassthrough): string;
export declare function decode(self: MuxContainerTsPassthrough, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthrough): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerTsPassthrough, err: Error_): void;
export { new$ as new };

// MuxContainerTsPassthroughExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthroughExt = rt.Handle<"MuxContainerTsPassthroughExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt): rt.Result<MuxContainerTsPassthroughExt, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthroughExt, pid: number): MuxContainerTsPassthroughExt;
export declare function track(self: MuxContainerTsPassthroughExt): string;
export declare function decode(self: MuxContainerTsPassthroughExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthroughExt): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerTsPassthroughExt, err: Error_): void;
export { new$ as new };

// MuxContainerTsPassthroughExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
export type MuxContainerTsPassthroughExtra = rt.Handle<"MuxContainerTsPassthroughExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra): rt.Result<MuxContainerTsPassthroughExtra, MuxError>;
export declare function withPcrPid(self: MuxContainerTsPassthroughExtra, pid: number): MuxContainerTsPassthroughExtra;
export declare function track(self: MuxContainerTsPassthroughExtra): string;
export declare function decode(self: MuxContainerTsPassthroughExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxContainerTsPassthroughExtra): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxContainerTsPassthroughExtra, err: Error_): void;
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
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogConfig } from "./MuxCatalogConfig.ts";
import type { MuxContainerTsStatsSnapshot } from "./MuxContainerTsStatsSnapshot.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { Path } from "./Path.ts";
export type MuxContainerTsPrograms = rt.Handle<"MuxContainerTsPrograms">;
export declare function new$(origin: OriginProducer, name: Path, config: MuxCatalogConfig): MuxContainerTsPrograms;
export declare function decode(self: MuxContainerTsPrograms, data: Uint8Array | rt.Bytes): rt.Result<undefined, string>;
export declare function stats(self: MuxContainerTsPrograms): MuxContainerTsStatsSnapshot;
export declare function finish(self: MuxContainerTsPrograms): rt.Result<BroadcastProducer[], string>;
export declare function abort(self: MuxContainerTsPrograms, err: Error_): void;
export { new$ as new };

// MuxContainerTsRelease.ts
import * as rt from "./rt.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerTsStatsRelease } from "./MuxContainerTsStatsRelease.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxSource } from "./MuxSource.ts";
export type MuxContainerTsRelease = rt.Handle<"MuxContainerTsRelease">;
export declare function new$(source: MuxSource, catalogFormat: MuxCatalogCatalogFormat): rt.Call<rt.Result<MuxContainerTsRelease, MuxError>>;
export declare function withDelay(self: MuxContainerTsRelease, delay: rt.Duration): MuxContainerTsRelease;
export declare function dropped(self: MuxContainerTsRelease): rt.u64;
export declare function drift(self: MuxContainerTsRelease): number | undefined;
export declare function outOfTolerance(self: MuxContainerTsRelease): rt.u64;
export declare function stats(self: MuxContainerTsRelease): MuxContainerTsStatsRelease;
export declare function next(self: MuxContainerTsRelease): rt.Call<rt.Result<MuxContainerFrame | undefined, MuxError>>;
export { new$ as new };

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
export declare function driftGet(self: MuxContainerTsStatsRelease): number | undefined;
export declare function driftSet(self: MuxContainerTsStatsRelease, value: number | undefined): void;
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

// MuxFollowEvent.ts
import * as rt from "./rt.ts";
export type MuxFollowEvent = rt.Handle<"MuxFollowEvent">;

// MuxFollowInstances.ts
import * as rt from "./rt.ts";
import type { Instance } from "./Instance.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxFollowEvent } from "./MuxFollowEvent.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
export type MuxFollowInstances = rt.Handle<"MuxFollowInstances">;
export declare function new$(origin: OriginConsumer, path: string, instance: Instance | undefined): rt.Result<MuxFollowInstances, MuxError>;
export declare function next(self: MuxFollowInstances): rt.Call<rt.Result<MuxFollowEvent, MuxError>>;
export { new$ as new };

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
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainer = rt.Handle<"MuxImportContainer">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReserved, init: MuxImportContainerInit): rt.Result<MuxImportContainer, MuxError>;
export declare function decode(self: MuxImportContainer, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainer): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportContainer, err: Error_): void;
export declare function cut(self: MuxImportContainer): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainer, sequence: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxImportContainerExt.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainerExt = rt.Handle<"MuxImportContainerExt">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExt, init: MuxImportContainerInit): rt.Result<MuxImportContainerExt, MuxError>;
export declare function decode(self: MuxImportContainerExt, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainerExt): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportContainerExt, err: Error_): void;
export declare function cut(self: MuxImportContainerExt): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainerExt, sequence: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

// MuxImportContainerExtra.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxImportContainerInit } from "./MuxImportContainerInit.ts";
export type MuxImportContainerExtra = rt.Handle<"MuxImportContainerExtra">;
export declare function new$(broadcast: BroadcastProducer, reserved: MuxCatalogReservedExtra, init: MuxImportContainerInit): rt.Result<MuxImportContainerExtra, MuxError>;
export declare function decode(self: MuxImportContainerExtra, data: Uint8Array | rt.Bytes): rt.Result<undefined, MuxError>;
export declare function finish(self: MuxImportContainerExtra): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportContainerExtra, err: Error_): void;
export declare function cut(self: MuxImportContainerExtra): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportContainerExtra, sequence: rt.u64): rt.Result<undefined, MuxError>;
export { new$ as new };

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
export declare function finish(self: MuxImportTrack): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportTrack, err: Error_): void;
export declare function cut(self: MuxImportTrack, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function discontinuity(self: MuxImportTrack): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxImportTrack, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportTrack, sequence: rt.u64): rt.Result<undefined, MuxError>;
export declare function demand(self: MuxImportTrack): TrackDemand;
export declare function name(self: MuxImportTrack): string;

// MuxImportTrackStream.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxCatalogReserved } from "./MuxCatalogReserved.ts";
import type { MuxCatalogReservedExt } from "./MuxCatalogReservedExt.ts";
import type { MuxCatalogReservedExtra } from "./MuxCatalogReservedExtra.ts";
import type { MuxError } from "./MuxError.ts";
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
export declare function finish(self: MuxImportTrackStream): rt.Result<undefined, MuxError>;
export declare function setEnabled(self: MuxImportTrackStream, enabled: boolean): rt.Result<undefined, MuxError>;
export declare function abort(self: MuxImportTrackStream, err: Error_): void;
export declare function cut(self: MuxImportTrackStream, end: Timestamp | undefined): rt.Result<undefined, MuxError>;
export declare function seek(self: MuxImportTrackStream, sequence: rt.u64): rt.Result<undefined, MuxError>;
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
import type { MuxError } from "./MuxError.ts";
import type { TimedJson } from "./TimedJson.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
export type MuxJsonConsumerJson = rt.Handle<"MuxJsonConsumerJson">;
export declare function fromTrack(track: TrackSubscriber, config: HangCatalogJsonConfig): rt.Result<MuxJsonConsumerJson, MuxError>;
export declare function mode(self: MuxJsonConsumerJson): HangCatalogMode;
export declare function next(self: MuxJsonConsumerJson): rt.Call<rt.Result<TimedJson | undefined, MuxError>>;
export declare function latest(self: MuxJsonConsumerJson): rt.Call<rt.Result<TimedJson | undefined, MuxError>>;

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

// MuxRateFollower.ts
import * as rt from "./rt.ts";
import type { BandwidthConsumer } from "./BandwidthConsumer.ts";
import type { Error_ } from "./Error_.ts";
import type { MuxRatePolicy } from "./MuxRatePolicy.ts";
import type { MuxRateTarget } from "./MuxRateTarget.ts";
export type MuxRateFollower = rt.Handle<"MuxRateFollower">;
export declare function new$(grant: BandwidthConsumer, policy: MuxRatePolicy): MuxRateFollower;
export declare function policy(self: MuxRateFollower): MuxRatePolicy;
export declare function setPolicy(self: MuxRateFollower, policy: MuxRatePolicy): void;
export declare function next(self: MuxRateFollower): rt.Call<rt.Result<MuxRateTarget, Error_>>;
export declare function failed(self: MuxRateFollower): void;
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
import type { MuxSelectSelected } from "./MuxSelectSelected.ts";
import type { MuxSelectVideo } from "./MuxSelectVideo.ts";
export type MuxSelectBroadcast = rt.Handle<"MuxSelectBroadcast">;
export declare function debug(self: MuxSelectBroadcast): string;
export declare function clone(self: MuxSelectBroadcast): MuxSelectBroadcast;
export declare function default$(): MuxSelectBroadcast;
export declare function video(self: MuxSelectBroadcast, video: MuxSelectVideo): MuxSelectBroadcast;
export declare function audio(self: MuxSelectBroadcast, audio: MuxSelectAudio): MuxSelectBroadcast;
export declare function hasVideo(self: MuxSelectBroadcast): boolean;
export declare function hasAudio(self: MuxSelectBroadcast): boolean;
export declare function retainExtra(self: MuxSelectBroadcast, catalog: HangCatalogExtra, selected: MuxSelectSelected): void;
export declare function retainExt(self: MuxSelectBroadcast, catalog: HangCatalogExt, selected: MuxSelectSelected): void;
export declare function retainUnit(self: MuxSelectBroadcast, catalog: HangCatalog, selected: MuxSelectSelected): void;
export { default$ as default };

// MuxSelectSelected.ts
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
export type MuxSelectSelected = {
    video: string[];
    audio: string[];
};
export declare function debug(self: MuxSelectSelected): string;
export declare function eq(a: MuxSelectSelected, b: MuxSelectSelected): boolean;
export declare function default$(): MuxSelectSelected;
export declare function ofExtra(catalog: HangCatalogExtra): MuxSelectSelected;
export declare function ofExt(catalog: HangCatalogExt): MuxSelectSelected;
export declare function ofUnit(catalog: HangCatalog): MuxSelectSelected;
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

// MuxTimelineAudio.ts
import * as rt from "./rt.ts";
export type MuxTimelineAudio = {
    end: rt.Duration;
    buffered: rt.Duration;
    target: rt.Duration;
};
export declare function debug(self: MuxTimelineAudio): string;

// MuxTimelineConsumer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxTimelineConsumer = rt.Handle<"MuxTimelineConsumer">;
export declare function clone(self: MuxTimelineConsumer): MuxTimelineConsumer;
export declare function playhead(self: MuxTimelineConsumer, now: ClockInstant): rt.Duration | undefined;
export declare function target(self: MuxTimelineConsumer): rt.Duration | undefined;
export declare function due(self: MuxTimelineConsumer, timestamp: Timestamp): ClockInstant | undefined;
export declare function present(self: MuxTimelineConsumer, timestamps: Timestamp[], now: ClockInstant): number | undefined;

// MuxTimelineError.ts
export type MuxTimelineError = "Overflow" | "BeforeOrigin" | "DelayOverflow";
export declare function toString(self: MuxTimelineError): string;
export declare function debug(self: MuxTimelineError): string;

// MuxTimelineKind.ts
export type MuxTimelineKind = "Video" | "Audio";
export declare function debug(self: MuxTimelineKind): string;

// MuxTimelineProducer.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxTimelineAudio } from "./MuxTimelineAudio.ts";
import type { MuxTimelineConsumer } from "./MuxTimelineConsumer.ts";
import type { MuxTimelineKind } from "./MuxTimelineKind.ts";
import type { MuxTimelineTrack } from "./MuxTimelineTrack.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxTimelineProducer = rt.Handle<"MuxTimelineProducer">;
export declare function clone(self: MuxTimelineProducer): MuxTimelineProducer;
export declare function new$(delay: rt.Duration): MuxTimelineProducer;
export declare function reset(self: MuxTimelineProducer, delay: rt.Duration): MuxTimelineProducer;
export declare function consume(self: MuxTimelineProducer): MuxTimelineConsumer;
export declare function track(self: MuxTimelineProducer, kind: MuxTimelineKind): MuxTimelineTrack;
export declare function video(self: MuxTimelineProducer, timestamp: Timestamp, now: ClockInstant): boolean;
export declare function videoRestarted(self: MuxTimelineProducer): void;
export declare function audio(self: MuxTimelineProducer, audio: MuxTimelineAudio, now: ClockInstant): boolean;
export declare function audioRestarted(self: MuxTimelineProducer): void;
export declare function audioStopped(self: MuxTimelineProducer): void;
export declare function due(self: MuxTimelineProducer, timestamp: Timestamp): ClockInstant | undefined;
export { new$ as new };

// MuxTimelineTrack.ts
import * as rt from "./rt.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { MuxTimelineError } from "./MuxTimelineError.ts";
import type { MuxTimelineTransition } from "./MuxTimelineTransition.ts";
import type { Timestamp } from "./Timestamp.ts";
export type MuxTimelineTrack = rt.Handle<"MuxTimelineTrack">;
export declare function retire(self: MuxTimelineTrack, drained: ClockInstant | undefined): void;
export declare function map(self: MuxTimelineTrack, source: Timestamp): rt.Result<Timestamp, MuxTimelineError>;
export declare function declare(self: MuxTimelineTrack): MuxTimelineTransition;
export declare function end(self: MuxTimelineTrack, source: Timestamp): rt.Result<undefined, MuxTimelineError>;
export declare function plan(self: MuxTimelineTrack, transition: MuxTimelineTransition, source: Timestamp, delay: rt.Duration | undefined): void;
export declare function gap(self: MuxTimelineTrack): void;
export declare function abandon(self: MuxTimelineTrack, transition: MuxTimelineTransition): void;

// MuxTimelineTransition.ts
import * as rt from "./rt.ts";
export type MuxTimelineTransition = rt.Opaque<"MuxTimelineTransition">;
export declare function debug(self: MuxTimelineTransition): string;
export declare function eq(a: MuxTimelineTransition, b: MuxTimelineTransition): boolean;

// OriginConfig.ts
import * as rt from "./rt.ts";
import type { CachePool } from "./CachePool.ts";
import type { Hop } from "./Hop.ts";
import type { OriginHopLimit } from "./OriginHopLimit.ts";
export type OriginConfig = rt.Handle<"OriginConfig">;
export declare function debug(self: OriginConfig): string;
export declare function clone(self: OriginConfig): OriginConfig;
export declare function default$(): OriginConfig;
export declare function hopGet(self: OriginConfig): Hop;
export declare function hopSet(self: OriginConfig, value: Hop): void;
export declare function maxHopsGet(self: OriginConfig): OriginHopLimit;
export declare function maxHopsSet(self: OriginConfig, value: OriginHopLimit): void;
export declare function poolGet(self: OriginConfig): CachePool;
export declare function poolSet(self: OriginConfig, value: CachePool): void;
export declare function cacheDurationGet(self: OriginConfig): rt.Duration;
export declare function cacheDurationSet(self: OriginConfig, value: rt.Duration): void;
export declare function updateHoldGet(self: OriginConfig): rt.Duration;
export declare function updateHoldSet(self: OriginConfig, value: rt.Duration): void;
export declare function new$(hop: Hop): OriginConfig;
export { default$ as default, new$ as new };

// OriginConsumer.ts
import * as rt from "./rt.ts";
import type { AnnounceConsumer } from "./AnnounceConsumer.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { Error_ } from "./Error_.ts";
import type { Hop } from "./Hop.ts";
import type { Instance } from "./Instance.ts";
import type { OriginFollowing } from "./OriginFollowing.ts";
import type { Path } from "./Path.ts";
import type { Patterns } from "./Patterns.ts";
import type { StatsSession } from "./StatsSession.ts";
export type OriginConsumer = rt.Handle<"OriginConsumer">;
export declare function clone(self: OriginConsumer): OriginConsumer;
export declare function hop(self: OriginConsumer): Hop;
export declare function local(self: OriginConsumer): OriginConsumer;
export declare function withHidden(self: OriginConsumer, hidden: boolean): OriginConsumer;
export declare function covering(self: OriginConsumer): OriginConsumer;
export declare function exclude(self: OriginConsumer, prefix: string): rt.Result<OriginConsumer, Error_>;
export declare function withStats(self: OriginConsumer, session: StatsSession): OriginConsumer;
export declare function announced(self: OriginConsumer): AnnounceConsumer;
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
export declare function default$(): OriginCost;
export declare function new$(cost: rt.u64): OriginCost;
export declare function value(self: OriginCost): rt.u64;
export declare function MAX(): OriginCost;
export declare function DRAIN(): OriginCost;
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
export declare function available(self: OriginFollowing): rt.Call<rt.Result<BroadcastConsumer, Error_>>;

// OriginHopLimit.ts
import * as rt from "./rt.ts";
export type OriginHopLimit = rt.Opaque<"OriginHopLimit">;
export declare function debug(self: OriginHopLimit): string;
export declare function eq(a: OriginHopLimit, b: OriginHopLimit): boolean;
export declare function hash(self: OriginHopLimit): rt.u64;
export declare function default$(): OriginHopLimit;
export declare function new$(max: number): OriginHopLimit | undefined;
export declare function get(self: OriginHopLimit): number;
export { default$ as default, new$ as new };

// OriginLink.ts
export type OriginLink = "Local" | "Peer" | "Upstream";
export declare function debug(self: OriginLink): string;
export declare function default$(): OriginLink;
export { default$ as default };

// OriginPreference.ts
import * as rt from "./rt.ts";
export type OriginPreference = rt.Opaque<"OriginPreference">;
export declare function debug(self: OriginPreference): string;
export declare function eq(a: OriginPreference, b: OriginPreference): boolean;
export declare function hash(self: OriginPreference): rt.u64;
export declare function default$(): OriginPreference;
export declare function new$(preference: number): OriginPreference;
export declare function value(self: OriginPreference): number;
export { default$ as default, new$ as new };

// OriginPrepared.ts
import * as rt from "./rt.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginPublication } from "./OriginPublication.ts";
export type OriginPrepared = rt.Handle<"OriginPrepared">;
export declare function broadcast(self: OriginPrepared): BroadcastProducer;
export declare function publish(self: OriginPrepared): rt.Result<OriginPublication, Error_>;

// OriginProducer.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Hop } from "./Hop.ts";
import type { OriginConfig } from "./OriginConfig.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginDriver } from "./OriginDriver.ts";
import type { OriginLink } from "./OriginLink.ts";
import type { OriginPreference } from "./OriginPreference.ts";
import type { OriginPublication } from "./OriginPublication.ts";
import type { OriginRequests } from "./OriginRequests.ts";
import type { OriginRoute } from "./OriginRoute.ts";
import type { Path } from "./Path.ts";
import type { Patterns } from "./Patterns.ts";
import type { StatsSession } from "./StatsSession.ts";
export type OriginProducer = rt.Handle<"OriginProducer">;
export declare function clone(self: OriginProducer): OriginProducer;
export declare function new$(config: OriginConfig): [OriginProducer, OriginDriver];
export declare function withStats(self: OriginProducer, session: StatsSession): OriginProducer;
export declare function withLink(self: OriginProducer, link: OriginLink): OriginProducer;
export declare function withPreference(self: OriginProducer, preference: OriginPreference): OriginProducer;
export declare function config(self: OriginProducer): OriginConfig;
export declare function hop(self: OriginProducer): Hop;
export declare function createBroadcast(self: OriginProducer, path: string): rt.Result<OriginPublication, Error_>;
export declare function publish(self: OriginProducer, path: string, route: OriginRoute): rt.Result<OriginPublication, Error_>;
export declare function requests(self: OriginProducer, prefix: string): rt.Result<OriginRequests, Error_>;
export declare function scope(self: OriginProducer, root: string, patterns: Patterns): rt.Result<OriginProducer, Error_>;
export declare function mount(self: OriginProducer, at: string, target: string): rt.Result<OriginProducer, Error_>;
export declare function consume(self: OriginProducer): OriginConsumer;
export declare function root(self: OriginProducer): Path;
export declare function allowed(self: OriginProducer): Patterns;
export declare function absolute(self: OriginProducer, path: string): rt.Result<Path, Error_>;
export { new$ as new };

// OriginPublication.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastDemand } from "./BroadcastDemand.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { BroadcastRequests } from "./BroadcastRequests.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { DatagramProducer } from "./DatagramProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginRoute } from "./OriginRoute.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackProducer } from "./TrackProducer.ts";
import type { TrackRequest } from "./TrackRequest.ts";
export type OriginPublication = rt.Handle<"OriginPublication">;
export declare function clone(self: OriginPublication): OriginPublication;
export declare function broadcast(self: OriginPublication): BroadcastProducer;
export declare function announce(self: OriginPublication, route: OriginRoute): rt.Result<undefined, Error_>;
export declare function unannounce(self: OriginPublication): void;
export declare function route(self: OriginPublication): OriginRoute | undefined;
export declare function info(self: OriginPublication): BroadcastInfo;
export declare function demand(self: OriginPublication): BroadcastDemand;
export declare function createTrack(self: OriginPublication, name: string, info: TrackInfo | undefined): rt.Result<TrackProducer, Error_>;
export declare function createDatagrams(self: OriginPublication, name: string, info: DatagramInfo): rt.Result<DatagramProducer, Error_>;
export declare function reserveTrack(self: OriginPublication, name: string): rt.Result<TrackRequest, Error_>;
export declare function uniqueTrack(self: OriginPublication, suffix: string, info: TrackInfo | undefined): rt.Result<TrackProducer, Error_>;
export declare function uniqueName(self: OriginPublication, suffix: string): string;
export declare function requests(self: OriginPublication): BroadcastRequests;
export declare function consume(self: OriginPublication): BroadcastConsumer;
export declare function close(self: OriginPublication): void;
export declare function isClone(self: OriginPublication, other: BroadcastProducer): boolean;

// OriginRequest.ts
import * as rt from "./rt.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginPrepared } from "./OriginPrepared.ts";
import type { Path } from "./Path.ts";
export type OriginRequest = rt.Handle<"OriginRequest">;
export declare function path(self: OriginRequest): Path;
export declare function prepare(self: OriginRequest): rt.Result<OriginPrepared, Error_>;
export declare function serveProducer(self: OriginRequest, broadcast: BroadcastProducer): void;
export declare function serveConsumer(self: OriginRequest, broadcast: BroadcastConsumer): void;
export declare function reject(self: OriginRequest, err: Error_): void;

// OriginRequests.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginRequest } from "./OriginRequest.ts";
import type { OriginRoute } from "./OriginRoute.ts";
export type OriginRequests = rt.Handle<"OriginRequests">;
export declare function route(self: OriginRequests): OriginRoute;
export declare function requestedBroadcast(self: OriginRequests): rt.Call<rt.Result<OriginRequest, Error_>>;

// OriginRoute.ts
import * as rt from "./rt.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { Hops } from "./Hops.ts";
import type { Instance } from "./Instance.ts";
import type { OriginCost } from "./OriginCost.ts";
import type { OriginSource } from "./OriginSource.ts";
export type OriginRoute = rt.Handle<"OriginRoute">;
export declare function debug(self: OriginRoute): string;
export declare function clone(self: OriginRoute): OriginRoute;
export declare function eq(a: OriginRoute, b: OriginRoute): boolean;
export declare function default$(): OriginRoute;
export declare function instanceGet(self: OriginRoute): Instance | undefined;
export declare function instanceSet(self: OriginRoute, value: Instance | undefined): void;
export declare function hopsGet(self: OriginRoute): Hops;
export declare function hopsSet(self: OriginRoute, value: Hops): void;
export declare function costGet(self: OriginRoute): OriginCost;
export declare function costSet(self: OriginRoute, value: OriginCost): void;
export declare function authGet(self: OriginRoute): AuthCredential | undefined;
export declare function authSet(self: OriginRoute, value: AuthCredential | undefined): void;
export declare function withInstance(self: OriginRoute, instance: Instance, epoch: rt.u64): OriginRoute;
export declare function withHops(self: OriginRoute, hops: Hops): OriginRoute;
export declare function withCost(self: OriginRoute, cost: OriginCost): OriginRoute;
export declare function withAuth(self: OriginRoute, auth: AuthCredential): OriginRoute;
export declare function isAnonymous(self: OriginRoute): boolean;
export declare function source(self: OriginRoute): OriginSource;
export { default$ as default };

// OriginSource.ts
import * as rt from "./rt.ts";
import type { Hop } from "./Hop.ts";
export type OriginSource = {
    tag: "Local";
} | {
    tag: "Peer";
    _0: Hop;
};
export declare function debug(self: OriginSource): string;
export declare function eq(a: OriginSource, b: OriginSource): boolean;
export declare function hash(self: OriginSource): rt.u64;
export declare function default$(): OriginSource;
export { default$ as default };

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
export declare function default$(): Path;
export declare function new$(s: string): rt.Result<Path, PathError>;
export declare function fromStatic(s: string): Path;
export declare function hasPrefix(self: Path, prefix: Path): boolean;
export declare function stripPrefix(self: Path, prefix: Path): Path | undefined;
export declare function head(self: Path, n: number): Path | undefined;
export declare function parts(self: Path): string[];
export declare function asStr(self: Path): string;
export declare function empty(): Path;
export declare function isEmpty(self: Path): boolean;
export declare function len(self: Path): number;
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
export declare function len(self: PathRelative): number;
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
export declare function len(self: Patterns): number;
export declare function isEmpty(self: Patterns): boolean;
export { default$ as default, new$ as new };

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
import type { ClockInstant } from "./ClockInstant.ts";
import type { Driver } from "./Driver.ts";
import type { Error_ } from "./Error_.ts";
import type { OriginConsumer } from "./OriginConsumer.ts";
import type { OriginProducer } from "./OriginProducer.ts";
import type { ServerHandshake } from "./ServerHandshake.ts";
import type { Session } from "./Session.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupExtensions } from "./SetupExtensions.ts";
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
export declare function withStats(self: Server, stats: StatsSession): Server;
export declare function withLimits(self: Server, limits: SessionLimits): Server;
export declare function withVersions(self: Server, versions: Versions): Server;
export declare function withExtensions(self: Server, extensions: SetupExtensions): Server;
export declare function withImplementation(self: Server, implementation: string): Server;
export declare function acceptSession(self: Server, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<[Session, Driver], Error_>>;
export declare function acceptRequestSession(self: Server, now: ClockInstant, session: TransportMemorySession): rt.Call<rt.Result<ServerHandshake, Error_>>;
export { default$ as default, new$ as new };

// ServerActive.ts
import * as rt from "./rt.ts";
export type ServerActive = rt.Handle<"ServerActive">;
export declare function debug(self: ServerActive): string;

// ServerActivity.ts
import * as rt from "./rt.ts";
import type { ServerActive } from "./ServerActive.ts";
export type ServerActivity = rt.Handle<"ServerActivity">;
export declare function debug(self: ServerActivity): string;
export declare function clone(self: ServerActivity): ServerActivity;
export declare function default$(): ServerActivity;
export declare function enter(self: ServerActivity): ServerActive | undefined;
export declare function close(self: ServerActivity): rt.Call<void>;
export { default$ as default };

// ServerHandshake.ts
import * as rt from "./rt.ts";
import type { AuthHandle } from "./AuthHandle.ts";
import type { AuthRequests } from "./AuthRequests.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { Driver } from "./Driver.ts";
import type { Error_ } from "./Error_.ts";
import type { Hop } from "./Hop.ts";
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
export declare function peerHop(self: ServerHandshake): Hop | undefined;
export declare function peerImplementation(self: ServerHandshake): string | undefined;
export declare function auth(self: ServerHandshake): AuthHandle;
export declare function requests(self: ServerHandshake): AuthRequests;
export declare function token(self: ServerHandshake): SetupToken | undefined;
export declare function withPublisherProducer(self: ServerHandshake, publish: OriginProducer): ServerHandshake;
export declare function withPublisherConsumer(self: ServerHandshake, publish: OriginConsumer): ServerHandshake;
export declare function withSubscriber(self: ServerHandshake, subscribe: OriginProducer): ServerHandshake;
export declare function withPeerHop(self: ServerHandshake, hop: Hop): ServerHandshake;
export declare function withStats(self: ServerHandshake, stats: StatsSession): ServerHandshake;
export declare function withLimits(self: ServerHandshake, limits: SessionLimits): ServerHandshake;
export declare function withDeadline(self: ServerHandshake, at: ClockInstant): ServerHandshake;
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
import type { GoawayProducer } from "./GoawayProducer.ts";
import type { SessionStats } from "./SessionStats.ts";
import type { Version } from "./Version.ts";
export type Session = rt.Handle<"Session">;
export declare function clone(self: Session): Session;
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
export declare function drain(self: Session): GoawayProducer;
export declare function draining(self: Session): GoawayConsumer;
export declare function idle(self: Session): rt.Call<void>;
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
    tag: "KeyValueFormatting";
} | {
    tag: "TooManyRequests";
} | {
    tag: "GoawayTimeout";
} | {
    tag: "Timeout";
} | {
    tag: "AuthTokenCacheOverflow";
} | {
    tag: "Version";
} | {
    tag: "MalformedAuthToken";
} | {
    tag: "UnknownAuthTokenAlias";
} | {
    tag: "TooManyRequestUpdates";
} | {
    tag: "App";
    _0: number;
} | {
    tag: "Unknown";
    _0: number;
} | {
    tag: "Unsupported";
} | {
    tag: "InvalidRequestId";
} | {
    tag: "InvalidPath";
} | {
    tag: "InvalidAuthority";
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
export declare function announcesGet(self: SessionLimits): number;
export declare function announcesSet(self: SessionLimits, value: number): SessionLimits;
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
export { default$ as default };

// SetupExtensions.ts
import * as rt from "./rt.ts";
export type SetupExtensions = rt.Opaque<"SetupExtensions">;
export declare function debug(self: SetupExtensions): string;
export declare function eq(a: SetupExtensions, b: SetupExtensions): boolean;
export declare function default$(): SetupExtensions;
export declare function authGet(self: SetupExtensions): boolean;
export declare function authSet(self: SetupExtensions, value: boolean): SetupExtensions;
export declare function solicitGet(self: SetupExtensions): boolean;
export declare function solicitSet(self: SetupExtensions, value: boolean): SetupExtensions;
export declare function ietfGet(self: SetupExtensions): boolean;
export declare function ietfSet(self: SetupExtensions, value: boolean): SetupExtensions;
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
import type { StatsRefusalKind } from "./StatsRefusalKind.ts";
import type { StatsSession } from "./StatsSession.ts";
export type StatsHandle = rt.Handle<"StatsHandle">;
export declare function clone(self: StatsHandle): StatsHandle;
export declare function default$(): StatsHandle;
export declare function refuse(self: StatsHandle, label: Path, reason: StatsRefusalKind): void;
export declare function session(self: StatsHandle, label: Path): StatsSession;
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
export declare function add(self: StatsPresence, other: StatsPresence): StatsPresence;
export declare function active(self: StatsPresence): rt.u64;
export { default$ as default };

// StatsRefusalKind.ts
export type StatsRefusalKind = "Refused" | "Unavailable" | "Request" | "Forbidden" | "Lan" | "Expired";
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
export declare function lanGet(self: StatsRefusals): rt.u64;
export declare function lanSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function expiredGet(self: StatsRefusals): rt.u64;
export declare function expiredSet(self: StatsRefusals, value: rt.u64): StatsRefusals;
export declare function count(self: StatsRefusals, kind: StatsRefusalKind): rt.u64;
export declare function add(self: StatsRefusals, other: StatsRefusals): StatsRefusals;
export declare function regressed(self: StatsRefusals, previous: StatsRefusals): boolean;
export { default$ as default };

// StatsRegistry.ts
import * as rt from "./rt.ts";
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
export declare function report(self: StatsRegistry, report: StatsReport): void;
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

// StatsSnapshot.ts
import * as rt from "./rt.ts";
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

// TimedJson.ts
import * as rt from "./rt.ts";
import type { Json } from "./Json.ts";
import type { Timestamp } from "./Timestamp.ts";
export type TimedJson = rt.Handle<"TimedJson">;
export declare function debug(self: TimedJson): string;
export declare function clone(self: TimedJson): TimedJson;
export declare function valueGet(self: TimedJson): Json;
export declare function valueSet(self: TimedJson, value: Json): void;
export declare function atGet(self: TimedJson): Timestamp | undefined;
export declare function atSet(self: TimedJson, value: Timestamp | undefined): void;
export declare function at(self: TimedJson, at: Timestamp): TimedJson;

// Timescale.ts
import * as rt from "./rt.ts";
import type { TimeOverflow } from "./TimeOverflow.ts";
export type Timescale = rt.Opaque<"Timescale">;
export declare function toString(self: Timescale): string;
export declare function debug(self: Timescale): string;
export declare function eq(a: Timescale, b: Timescale): boolean;
export declare function hash(self: Timescale): rt.u64;
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
export declare function ZERO(): Timestamp;
export declare function new$(value: rt.u64, scale: Timescale): rt.Result<Timestamp, TimeOverflow>;
export declare function fromScale(value: rt.u64, unitsPerSecond: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromSecs(seconds: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromMillis(millis: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromMicros(micros: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function fromNanos(nanos: rt.u64): rt.Result<Timestamp, TimeOverflow>;
export declare function value(self: Timestamp): rt.u64;
export declare function scale(self: Timestamp): Timescale;
export declare function isZero(self: Timestamp): boolean;
export declare function convert(self: Timestamp, newScale: Timescale): rt.Result<Timestamp, TimeOverflow>;
export declare function asScale(self: Timestamp, target: Timescale): rt.u128;
export declare function asSecs(self: Timestamp): rt.u64;
export declare function asMillis(self: Timestamp): rt.u128;
export declare function asMicros(self: Timestamp): rt.u128;
export declare function asNanos(self: Timestamp): rt.u128;
export declare function checkedAdd(self: Timestamp, rhs: Timestamp): rt.Result<Timestamp, TimeOverflow>;
export declare function checkedSub(self: Timestamp, rhs: Timestamp): rt.Result<Timestamp, TimeOverflow>;
export declare function now(): Timestamp;
export { new$ as new };

// TrackAnchor.ts
import * as rt from "./rt.ts";
import type { Timestamp } from "./Timestamp.ts";
export type TrackAnchor = {
    tag: "Absolute";
    _0: rt.u64;
} | {
    tag: "Relative";
    _0: rt.u64;
} | {
    tag: "Time";
    _0: Timestamp;
};
export declare function debug(self: TrackAnchor): string;
export declare function eq(a: TrackAnchor, b: TrackAnchor): boolean;
export declare function hash(self: TrackAnchor): rt.u64;

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
export declare function name(self: TrackConsumer): string;
export declare function broadcast(self: TrackConsumer): BroadcastInfo;
export declare function subscribe(self: TrackConsumer, subscription: TrackSubscription | undefined): rt.Call<rt.Result<TrackSubscriber, Error_>>;
export declare function fetchGroup(self: TrackConsumer, sequence: rt.u64, options: GroupFetch | undefined): rt.Call<rt.Result<GroupConsumer, Error_>>;
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
export declare function wanted(self: TrackDemand): rt.u64 | undefined;
export declare function wantedChanged(self: TrackDemand): rt.Call<rt.Result<rt.u64 | undefined, Error_>>;
export declare function closed(self: TrackDemand): rt.Call<rt.Result<undefined, Error_>>;

// TrackFill.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackOrder } from "./TrackOrder.ts";
import type { TrackPosition } from "./TrackPosition.ts";
export type TrackFill = rt.Handle<"TrackFill">;
export declare function clone(self: TrackFill): TrackFill;
export declare function start(self: TrackFill): TrackPosition;
export declare function frameStart(self: TrackFill, sequence: rt.u64): rt.u64;
export declare function end(self: TrackFill): rt.u64;
export declare function time(self: TrackFill): Timestamp | undefined;
export declare function priority(self: TrackFill): number;
export declare function maxDelay(self: TrackFill): rt.Duration | undefined;
export declare function order(self: TrackFill): TrackOrder;
export declare function accept(self: TrackFill, sequence: rt.u64, info: TrackInfo | undefined): rt.Result<GroupProducer, Error_>;
export declare function dropGroups(self: TrackFill, groups: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function refuseGroups(self: TrackFill, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: Error_): void;
export declare function isWanted(self: TrackFill, sequence: rt.u64): boolean;
export declare function unwanted(self: TrackFill, sequence: rt.u64): rt.Call<void>;
export declare function wanted(self: TrackFill): rt.u64 | undefined;
export declare function wantedChanged(self: TrackFill): rt.Call<rt.u64 | undefined>;

// TrackInfo.ts
import * as rt from "./rt.ts";
import type { Error_ } from "./Error_.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
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
export declare function compressGet(self: TrackInfo): boolean;
export declare function compressSet(self: TrackInfo, value: boolean): void;
export declare function refreshGet(self: TrackInfo): number | undefined;
export declare function refreshSet(self: TrackInfo, value: number | undefined): void;
export declare function withTimescale(self: TrackInfo, timescale: Timescale | undefined): TrackInfo;
export declare function withMaxAge(self: TrackInfo, maxAge: rt.Duration | undefined): TrackInfo;
export declare function withPriority(self: TrackInfo, priority: number): TrackInfo;
export declare function withCompress(self: TrackInfo, compress: boolean): TrackInfo;
export declare function withRefresh(self: TrackInfo, refresh: number | undefined): TrackInfo;
export declare function rescale(self: TrackInfo, timestamp: Timestamp | undefined): rt.Result<Timestamp | undefined, Error_>;
export { default$ as default };

// TrackOrder.ts
export type TrackOrder = "Ascending" | "Descending";
export declare function debug(self: TrackOrder): string;
export declare function default$(): TrackOrder;
export { default$ as default };

// TrackOrdered.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { TrackControl } from "./TrackControl.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackOrdered = rt.Handle<"TrackOrdered">;
export declare function info(self: TrackOrdered): TrackInfo;
export declare function name(self: TrackOrdered): string;
export declare function broadcast(self: TrackOrdered): BroadcastInfo;
export declare function nextGroup(self: TrackOrdered): rt.Call<rt.Result<GroupConsumer | undefined, Error_>>;
export declare function withRange(self: TrackOrdered, groups: {
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
}): TrackOrdered;
export declare function control(self: TrackOrdered): TrackControl;
export declare function subscription(self: TrackOrdered): TrackSubscription;
export declare function update(self: TrackOrdered, subscription: TrackSubscription): rt.Result<undefined, Error_>;
export declare function finished(self: TrackOrdered): rt.Call<rt.Result<TrackPosition, Error_>>;
export declare function largest(self: TrackOrdered): rt.Call<TrackPosition | undefined>;
export declare function isClone(self: TrackOrdered, other: TrackOrdered): boolean;

// TrackPosition.ts
import * as rt from "./rt.ts";
export type TrackPosition = {
    group: rt.u64;
    frame: rt.u64;
};
export declare function debug(self: TrackPosition): string;
export declare function eq(a: TrackPosition, b: TrackPosition): boolean;
export declare function hash(self: TrackPosition): rt.u64;
export declare function default$(): TrackPosition;
export declare function group(group: rt.u64): TrackPosition;
export declare function after(group: rt.u64, frame: rt.u64): TrackPosition | undefined;
export declare function afterGroup(group: rt.u64): TrackPosition | undefined;
export declare function groupEnd(self: TrackPosition): {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
};
export { default$ as default };

// TrackProducer.ts
import * as rt from "./rt.ts";
import type { BroadcastInfo } from "./BroadcastInfo.ts";
import type { Error_ } from "./Error_.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { GroupProducer } from "./GroupProducer.ts";
import type { StreamError } from "./StreamError.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackConsumer } from "./TrackConsumer.ts";
import type { TrackDemand } from "./TrackDemand.ts";
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
export declare function appendGroup(self: TrackProducer): rt.Result<GroupProducer, Error_>;
export declare function writeFrame(self: TrackProducer, timestamp: Timestamp | undefined, frame: Uint8Array | rt.Bytes): rt.Result<undefined, Error_>;
export declare function finish(self: TrackProducer): rt.Result<undefined, Error_>;
export declare function finishAt(self: TrackProducer, end: TrackPosition, error: Error_ | undefined): rt.Result<undefined, Error_>;
export declare function startAt(self: TrackProducer, sequence: rt.u64 | undefined): rt.Result<undefined, Error_>;
export declare function dropGroups(self: TrackProducer, groups: {
    start: rt.u64;
    end: rt.u64;
}, err: StreamError): void;
export declare function finished(self: TrackProducer): TrackPosition | undefined;
export declare function abort(self: TrackProducer, err: Error_): rt.Result<undefined, Error_>;
export declare function abortUnwanted(self: TrackProducer, err: Error_): rt.Result<undefined, TrackProducer>;
export declare function closed(self: TrackProducer): rt.Call<rt.Result<undefined, Error_>>;
export declare function isClosed(self: TrackProducer): boolean;
export declare function largest(self: TrackProducer): TrackPosition | undefined;
export declare function isClone(self: TrackProducer, other: TrackProducer): boolean;
export declare function demand(self: TrackProducer): TrackDemand;
export declare function consume(self: TrackProducer): TrackConsumer;
export declare function subscribe(self: TrackProducer, subscription: TrackSubscription | undefined): TrackSubscriber;
export declare function subscriptionChanged(self: TrackProducer): rt.Call<rt.Result<TrackSubscription | undefined, Error_>>;
export declare function subscription(self: TrackProducer): TrackSubscription | undefined;
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
export declare function live(): TrackRange;
export declare function oldest(): TrackRange;
export declare function groups(groups: {
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
}): TrackRange;

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
import type { TrackControl } from "./TrackControl.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackOrdered } from "./TrackOrdered.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
export type TrackSubscriber = rt.Handle<"TrackSubscriber">;
export declare function info(self: TrackSubscriber): TrackInfo;
export declare function name(self: TrackSubscriber): string;
export declare function broadcast(self: TrackSubscriber): BroadcastInfo;
export declare function control(self: TrackSubscriber): TrackControl;
export declare function recvGroup(self: TrackSubscriber): rt.Call<rt.Result<GroupConsumer | undefined, Error_>>;
export declare function ordered(self: TrackSubscriber): TrackOrdered;
export declare function isClone(self: TrackSubscriber, other: TrackSubscriber): boolean;
export declare function floor(self: TrackSubscriber): rt.u64 | undefined;
export declare function nextInRange(self: TrackSubscriber, from: rt.u64): rt.u64 | undefined;
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
export declare function withRange(self: TrackSubscription, groups: {
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
}): TrackSubscription;
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

// TransportMemoryConfig.ts
import * as rt from "./rt.ts";
export type TransportMemoryConfig = rt.Handle<"TransportMemoryConfig">;
export declare function debug(self: TransportMemoryConfig): string;
export declare function clone(self: TransportMemoryConfig): TransportMemoryConfig;
export declare function default$(): TransportMemoryConfig;
export declare function streamBufferGet(self: TransportMemoryConfig): number;
export declare function streamBufferSet(self: TransportMemoryConfig, value: number): void;
export declare function protocolGet(self: TransportMemoryConfig): string | undefined;
export declare function protocolSet(self: TransportMemoryConfig, value: string | undefined): void;
export declare function maxConcurrentUniStreamsGet(self: TransportMemoryConfig): number | undefined;
export declare function maxConcurrentUniStreamsSet(self: TransportMemoryConfig, value: number | undefined): void;
export declare function maxConcurrentBiStreamsGet(self: TransportMemoryConfig): number | undefined;
export declare function maxConcurrentBiStreamsSet(self: TransportMemoryConfig, value: number | undefined): void;
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
export type TransportMemorySession = rt.Handle<"TransportMemorySession">;
export declare function clone(self: TransportMemorySession): TransportMemorySession;

// TransportMetrics.ts
export interface TransportMetrics {
    close?(): void;
}

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
export declare function connectionGet(self: TransportReceiveWindow): rt.u64;
export declare function connectionSet(self: TransportReceiveWindow, value: rt.u64): TransportReceiveWindow;
export declare function streamGet(self: TransportReceiveWindow): rt.u64;
export declare function streamSet(self: TransportReceiveWindow, value: rt.u64): TransportReceiveWindow;
export declare function new$(connection: rt.u64, stream: rt.u64): TransportReceiveWindow;
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
export declare function parse(input: string): rt.Result<Url, string>;

// Version.ts
import * as rt from "./rt.ts";
export type Version = rt.Handle<"Version">;
export declare function toString(self: Version): string;
export declare function debug(self: Version): string;
export declare function clone(self: Version): Version;
export declare function eq(a: Version, b: Version): boolean;
export declare function hash(self: Version): rt.u64;
export declare function hasSetupToken(self: Version): boolean;
export declare function asStr(self: Version): string;
export declare function fromCode(code: rt.u64): Version | undefined;
export declare function code(self: Version): rt.u64;
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

// glue.ts
import * as rt from "./rt.ts";
import type { AnnounceEvent } from "./AnnounceEvent.ts";
import type { AuthAnswer } from "./AuthAnswer.ts";
import type { AuthCredential } from "./AuthCredential.ts";
import type { AuthGrant } from "./AuthGrant.ts";
import type { AuthRequest } from "./AuthRequest.ts";
import type { AuthRequestKind } from "./AuthRequestKind.ts";
import type { BandwidthRate } from "./BandwidthRate.ts";
import type { BroadcastConsumer } from "./BroadcastConsumer.ts";
import type { BroadcastProducer } from "./BroadcastProducer.ts";
import type { BroadcastTracksEvent } from "./BroadcastTracksEvent.ts";
import type { ClockElapsed } from "./ClockElapsed.ts";
import type { ClockInstant } from "./ClockInstant.ts";
import type { ClockMissedTickBehavior } from "./ClockMissedTickBehavior.ts";
import type { ClockWall } from "./ClockWall.ts";
import type { Datagram } from "./Datagram.ts";
import type { DatagramInfo } from "./DatagramInfo.ts";
import type { DatagramRequest } from "./DatagramRequest.ts";
import type { Driver } from "./Driver.ts";
import type { EncodeError } from "./EncodeError.ts";
import type { Error_ } from "./Error_.ts";
import type { FrameConsumer } from "./FrameConsumer.ts";
import type { FrameFrame } from "./FrameFrame.ts";
import type { FrameInfo } from "./FrameInfo.ts";
import type { GoawayGoaway } from "./GoawayGoaway.ts";
import type { GroupConsumer } from "./GroupConsumer.ts";
import type { GroupInfo } from "./GroupInfo.ts";
import type { HangCatalog } from "./HangCatalog.ts";
import type { HangCatalogAac } from "./HangCatalogAac.ts";
import type { HangCatalogAudioCodec } from "./HangCatalogAudioCodec.ts";
import type { HangCatalogAudioCodecKind } from "./HangCatalogAudioCodecKind.ts";
import type { HangCatalogAudioConfig } from "./HangCatalogAudioConfig.ts";
import type { HangCatalogAudioLayout } from "./HangCatalogAudioLayout.ts";
import type { HangCatalogAv1 } from "./HangCatalogAv1.ts";
import type { HangCatalogBinaryConfig } from "./HangCatalogBinaryConfig.ts";
import type { HangCatalogChromaticity } from "./HangCatalogChromaticity.ts";
import type { HangCatalogClock } from "./HangCatalogClock.ts";
import type { HangCatalogConsumer } from "./HangCatalogConsumer.ts";
import type { HangCatalogConsumerExt } from "./HangCatalogConsumerExt.ts";
import type { HangCatalogConsumerExtra } from "./HangCatalogConsumerExtra.ts";
import type { HangCatalogContentLight } from "./HangCatalogContentLight.ts";
import type { HangCatalogContribution } from "./HangCatalogContribution.ts";
import type { HangCatalogDisplay } from "./HangCatalogDisplay.ts";
import type { HangCatalogExt } from "./HangCatalogExt.ts";
import type { HangCatalogExtra } from "./HangCatalogExtra.ts";
import type { HangCatalogH264 } from "./HangCatalogH264.ts";
import type { HangCatalogH265 } from "./HangCatalogH265.ts";
import type { HangCatalogJsonConfig } from "./HangCatalogJsonConfig.ts";
import type { HangCatalogMastering } from "./HangCatalogMastering.ts";
import type { HangCatalogMode } from "./HangCatalogMode.ts";
import type { HangCatalogPriorities } from "./HangCatalogPriorities.ts";
import type { HangCatalogRange } from "./HangCatalogRange.ts";
import type { HangCatalogStereo } from "./HangCatalogStereo.ts";
import type { HangCatalogTextConfig } from "./HangCatalogTextConfig.ts";
import type { HangCatalogTextFormat } from "./HangCatalogTextFormat.ts";
import type { HangCatalogTextRole } from "./HangCatalogTextRole.ts";
import type { HangCatalogVideoCodec } from "./HangCatalogVideoCodec.ts";
import type { HangCatalogVideoCodecKind } from "./HangCatalogVideoCodecKind.ts";
import type { HangCatalogVideoConfig } from "./HangCatalogVideoConfig.ts";
import type { HangCatalogVp9 } from "./HangCatalogVp9.ts";
import type { HangContainerFrame } from "./HangContainerFrame.ts";
import type { HangEchoRendition } from "./HangEchoRendition.ts";
import type { HangError } from "./HangError.ts";
import type { HangHealthEvidence } from "./HangHealthEvidence.ts";
import type { HangHealthInput } from "./HangHealthInput.ts";
import type { HangHealthLimitsDuration } from "./HangHealthLimitsDuration.ts";
import type { HangHealthLimitsF64 } from "./HangHealthLimitsF64.ts";
import type { HangHealthReset } from "./HangHealthReset.ts";
import type { HangHealthRole } from "./HangHealthRole.ts";
import type { HangHealthVerdict } from "./HangHealthVerdict.ts";
import type { HangStatsRendition } from "./HangStatsRendition.ts";
import type { Hop } from "./Hop.ts";
import type { IntersectionError } from "./IntersectionError.ts";
import type { InvalidHop } from "./InvalidHop.ts";
import type { InvalidPattern } from "./InvalidPattern.ts";
import type { Json } from "./Json.ts";
import type { Mp4AtomAc3 } from "./Mp4AtomAc3.ts";
import type { Mp4AtomAc3SpecificBox } from "./Mp4AtomAc3SpecificBox.ts";
import type { Mp4AtomAinf } from "./Mp4AtomAinf.ts";
import type { Mp4AtomAmrSampleEntry } from "./Mp4AtomAmrSampleEntry.ts";
import type { Mp4AtomAny } from "./Mp4AtomAny.ts";
import type { Mp4AtomAnySampleGroupEntry } from "./Mp4AtomAnySampleGroupEntry.ts";
import type { Mp4AtomAudio } from "./Mp4AtomAudio.ts";
import type { Mp4AtomAudioChannelPosition } from "./Mp4AtomAudioChannelPosition.ts";
import type { Mp4AtomAuxInfo } from "./Mp4AtomAuxInfo.ts";
import type { Mp4AtomAuxc } from "./Mp4AtomAuxc.ts";
import type { Mp4AtomAv1c } from "./Mp4AtomAv1c.ts";
import type { Mp4AtomAvcc } from "./Mp4AtomAvcc.ts";
import type { Mp4AtomAvccExt } from "./Mp4AtomAvccExt.ts";
import type { Mp4AtomBlin } from "./Mp4AtomBlin.ts";
import type { Mp4AtomBtrt } from "./Mp4AtomBtrt.ts";
import type { Mp4AtomC2pa } from "./Mp4AtomC2pa.ts";
import type { Mp4AtomCamm } from "./Mp4AtomCamm.ts";
import type { Mp4AtomCbmp } from "./Mp4AtomCbmp.ts";
import type { Mp4AtomCcst } from "./Mp4AtomCcst.ts";
import type { Mp4AtomChannelStructure } from "./Mp4AtomChannelStructure.ts";
import type { Mp4AtomChnl } from "./Mp4AtomChnl.ts";
import type { Mp4AtomChpl } from "./Mp4AtomChpl.ts";
import type { Mp4AtomChplEntry } from "./Mp4AtomChplEntry.ts";
import type { Mp4AtomClap } from "./Mp4AtomClap.ts";
import type { Mp4AtomClockType } from "./Mp4AtomClockType.ts";
import type { Mp4AtomCmpd } from "./Mp4AtomCmpd.ts";
import type { Mp4AtomCo64 } from "./Mp4AtomCo64.ts";
import type { Mp4AtomCodec } from "./Mp4AtomCodec.ts";
import type { Mp4AtomColr } from "./Mp4AtomColr.ts";
import type { Mp4AtomComponent } from "./Mp4AtomComponent.ts";
import type { Mp4AtomCopyright } from "./Mp4AtomCopyright.ts";
import type { Mp4AtomCovr } from "./Mp4AtomCovr.ts";
import type { Mp4AtomCprt } from "./Mp4AtomCprt.ts";
import type { Mp4AtomCslg } from "./Mp4AtomCslg.ts";
import type { Mp4AtomCtts } from "./Mp4AtomCtts.ts";
import type { Mp4AtomCttsEntry } from "./Mp4AtomCttsEntry.ts";
import type { Mp4AtomDadj } from "./Mp4AtomDadj.ts";
import type { Mp4AtomDamr } from "./Mp4AtomDamr.ts";
import type { Mp4AtomDataType } from "./Mp4AtomDataType.ts";
import type { Mp4AtomDesc } from "./Mp4AtomDesc.ts";
import type { Mp4AtomDfla } from "./Mp4AtomDfla.ts";
import type { Mp4AtomDinf } from "./Mp4AtomDinf.ts";
import type { Mp4AtomDisp } from "./Mp4AtomDisp.ts";
import type { Mp4AtomDops } from "./Mp4AtomDops.ts";
import type { Mp4AtomDref } from "./Mp4AtomDref.ts";
import type { Mp4AtomDrefEntry } from "./Mp4AtomDrefEntry.ts";
import type { Mp4AtomDtyp } from "./Mp4AtomDtyp.ts";
import type { Mp4AtomEac3 } from "./Mp4AtomEac3.ts";
import type { Mp4AtomEc3IndependentSubstream } from "./Mp4AtomEc3IndependentSubstream.ts";
import type { Mp4AtomEc3SpecificBox } from "./Mp4AtomEc3SpecificBox.ts";
import type { Mp4AtomEdts } from "./Mp4AtomEdts.ts";
import type { Mp4AtomElst } from "./Mp4AtomElst.ts";
import type { Mp4AtomElstEntry } from "./Mp4AtomElstEntry.ts";
import type { Mp4AtomEmsg } from "./Mp4AtomEmsg.ts";
import type { Mp4AtomEmsgTimestamp } from "./Mp4AtomEmsgTimestamp.ts";
import type { Mp4AtomEqui } from "./Mp4AtomEqui.ts";
import type { Mp4AtomEsds } from "./Mp4AtomEsds.ts";
import type { Mp4AtomEsdsDecoderConfig } from "./Mp4AtomEsdsDecoderConfig.ts";
import type { Mp4AtomEsdsDecoderSpecific } from "./Mp4AtomEsdsDecoderSpecific.ts";
import type { Mp4AtomEsdsEsDescriptor } from "./Mp4AtomEsdsEsDescriptor.ts";
import type { Mp4AtomEsdsSlConfig } from "./Mp4AtomEsdsSlConfig.ts";
import type { Mp4AtomEvte } from "./Mp4AtomEvte.ts";
import type { Mp4AtomExplicitSpeakerPosition } from "./Mp4AtomExplicitSpeakerPosition.ts";
import type { Mp4AtomExtendedType } from "./Mp4AtomExtendedType.ts";
import type { Mp4AtomFiel } from "./Mp4AtomFiel.ts";
import type { Mp4AtomFixedPointI16 } from "./Mp4AtomFixedPointI16.ts";
import type { Mp4AtomFixedPointI8 } from "./Mp4AtomFixedPointI8.ts";
import type { Mp4AtomFixedPointU16 } from "./Mp4AtomFixedPointU16.ts";
import type { Mp4AtomFixedPointU8 } from "./Mp4AtomFixedPointU8.ts";
import type { Mp4AtomFl32 } from "./Mp4AtomFl32.ts";
import type { Mp4AtomFl64 } from "./Mp4AtomFl64.ts";
import type { Mp4AtomFlac } from "./Mp4AtomFlac.ts";
import type { Mp4AtomFlacMetadataBlock } from "./Mp4AtomFlacMetadataBlock.ts";
import type { Mp4AtomFontEntry } from "./Mp4AtomFontEntry.ts";
import type { Mp4AtomFourCc } from "./Mp4AtomFourCc.ts";
import type { Mp4AtomFpcm } from "./Mp4AtomFpcm.ts";
import type { Mp4AtomFragmentInfo } from "./Mp4AtomFragmentInfo.ts";
import type { Mp4AtomFree } from "./Mp4AtomFree.ts";
import type { Mp4AtomFtab } from "./Mp4AtomFtab.ts";
import type { Mp4AtomFtyp } from "./Mp4AtomFtyp.ts";
import type { Mp4AtomHdlr } from "./Mp4AtomHdlr.ts";
import type { Mp4AtomHero } from "./Mp4AtomHero.ts";
import type { Mp4AtomHfov } from "./Mp4AtomHfov.ts";
import type { Mp4AtomHmhd } from "./Mp4AtomHmhd.ts";
import type { Mp4AtomHvcCArray } from "./Mp4AtomHvcCArray.ts";
import type { Mp4AtomHvcc } from "./Mp4AtomHvcc.ts";
import type { Mp4AtomIdat } from "./Mp4AtomIdat.ts";
import type { Mp4AtomIinf } from "./Mp4AtomIinf.ts";
import type { Mp4AtomIloc } from "./Mp4AtomIloc.ts";
import type { Mp4AtomIlst } from "./Mp4AtomIlst.ts";
import type { Mp4AtomIlstData } from "./Mp4AtomIlstData.ts";
import type { Mp4AtomIlstDataValue } from "./Mp4AtomIlstDataValue.ts";
import type { Mp4AtomImir } from "./Mp4AtomImir.ts";
import type { Mp4AtomIn24 } from "./Mp4AtomIn24.ts";
import type { Mp4AtomIn32 } from "./Mp4AtomIn32.ts";
import type { Mp4AtomIpcm } from "./Mp4AtomIpcm.ts";
import type { Mp4AtomIpma } from "./Mp4AtomIpma.ts";
import type { Mp4AtomIref } from "./Mp4AtomIref.ts";
import type { Mp4AtomIrot } from "./Mp4AtomIrot.ts";
import type { Mp4AtomIscl } from "./Mp4AtomIscl.ts";
import type { Mp4AtomIspe } from "./Mp4AtomIspe.ts";
import type { Mp4AtomItemInfoEntry } from "./Mp4AtomItemInfoEntry.ts";
import type { Mp4AtomItemLocation } from "./Mp4AtomItemLocation.ts";
import type { Mp4AtomItemLocationExtent } from "./Mp4AtomItemLocationExtent.ts";
import type { Mp4AtomKeyEntry } from "./Mp4AtomKeyEntry.ts";
import type { Mp4AtomKeyd } from "./Mp4AtomKeyd.ts";
import type { Mp4AtomKeys } from "./Mp4AtomKeys.ts";
import type { Mp4AtomKind } from "./Mp4AtomKind.ts";
import type { Mp4AtomLhvc } from "./Mp4AtomLhvc.ts";
import type { Mp4AtomLoca } from "./Mp4AtomLoca.ts";
import type { Mp4AtomLpcm } from "./Mp4AtomLpcm.ts";
import type { Mp4AtomMatrix } from "./Mp4AtomMatrix.ts";
import type { Mp4AtomMdat } from "./Mp4AtomMdat.ts";
import type { Mp4AtomMdhd } from "./Mp4AtomMdhd.ts";
import type { Mp4AtomMebx } from "./Mp4AtomMebx.ts";
import type { Mp4AtomMebxKey } from "./Mp4AtomMebxKey.ts";
import type { Mp4AtomMehd } from "./Mp4AtomMehd.ts";
import type { Mp4AtomMetaData } from "./Mp4AtomMetaData.ts";
import type { Mp4AtomMett } from "./Mp4AtomMett.ts";
import type { Mp4AtomMetx } from "./Mp4AtomMetx.ts";
import type { Mp4AtomMfhd } from "./Mp4AtomMfhd.ts";
import type { Mp4AtomMfra } from "./Mp4AtomMfra.ts";
import type { Mp4AtomMfro } from "./Mp4AtomMfro.ts";
import type { Mp4AtomMp3 } from "./Mp4AtomMp3.ts";
import type { Mp4AtomMp4a } from "./Mp4AtomMp4a.ts";
import type { Mp4AtomMshp } from "./Mp4AtomMshp.ts";
import type { Mp4AtomMust } from "./Mp4AtomMust.ts";
import type { Mp4AtomMvex } from "./Mp4AtomMvex.ts";
import type { Mp4AtomMvhd } from "./Mp4AtomMvhd.ts";
import type { Mp4AtomName } from "./Mp4AtomName.ts";
import type { Mp4AtomOpus } from "./Mp4AtomOpus.ts";
import type { Mp4AtomOpusChannelMapping } from "./Mp4AtomOpusChannelMapping.ts";
import type { Mp4AtomPasp } from "./Mp4AtomPasp.ts";
import type { Mp4AtomPcmC } from "./Mp4AtomPcmC.ts";
import type { Mp4AtomPcmFormat } from "./Mp4AtomPcmFormat.ts";
import type { Mp4AtomPitm } from "./Mp4AtomPitm.ts";
import type { Mp4AtomPixi } from "./Mp4AtomPixi.ts";
import type { Mp4AtomPkin } from "./Mp4AtomPkin.ts";
import type { Mp4AtomPlainText } from "./Mp4AtomPlainText.ts";
import type { Mp4AtomPrft } from "./Mp4AtomPrft.ts";
import type { Mp4AtomPrhd } from "./Mp4AtomPrhd.ts";
import type { Mp4AtomPrji } from "./Mp4AtomPrji.ts";
import type { Mp4AtomProjKind } from "./Mp4AtomProjKind.ts";
import type { Mp4AtomPropertyAssociation } from "./Mp4AtomPropertyAssociation.ts";
import type { Mp4AtomPropertyAssociations } from "./Mp4AtomPropertyAssociations.ts";
import type { Mp4AtomReference } from "./Mp4AtomReference.ts";
import type { Mp4AtomReferenceTime } from "./Mp4AtomReferenceTime.ts";
import type { Mp4AtomRgbColor } from "./Mp4AtomRgbColor.ts";
import type { Mp4AtomRgbaColor } from "./Mp4AtomRgbaColor.ts";
import type { Mp4AtomRref } from "./Mp4AtomRref.ts";
import type { Mp4AtomRtng } from "./Mp4AtomRtng.ts";
import type { Mp4AtomS16l } from "./Mp4AtomS16l.ts";
import type { Mp4AtomSaio } from "./Mp4AtomSaio.ts";
import type { Mp4AtomSaiz } from "./Mp4AtomSaiz.ts";
import type { Mp4AtomSamr } from "./Mp4AtomSamr.ts";
import type { Mp4AtomSbgp } from "./Mp4AtomSbgp.ts";
import type { Mp4AtomSbgpEntry } from "./Mp4AtomSbgpEntry.ts";
import type { Mp4AtomSchemeIdEntry } from "./Mp4AtomSchemeIdEntry.ts";
import type { Mp4AtomSegmentReference } from "./Mp4AtomSegmentReference.ts";
import type { Mp4AtomSenc } from "./Mp4AtomSenc.ts";
import type { Mp4AtomSencBoxVersion } from "./Mp4AtomSencBoxVersion.ts";
import type { Mp4AtomSetu } from "./Mp4AtomSetu.ts";
import type { Mp4AtomSgpd } from "./Mp4AtomSgpd.ts";
import type { Mp4AtomSgpdEntry } from "./Mp4AtomSgpdEntry.ts";
import type { Mp4AtomSidx } from "./Mp4AtomSidx.ts";
import type { Mp4AtomSilb } from "./Mp4AtomSilb.ts";
import type { Mp4AtomSkip } from "./Mp4AtomSkip.ts";
import type { Mp4AtomSmhd } from "./Mp4AtomSmhd.ts";
import type { Mp4AtomSowt } from "./Mp4AtomSowt.ts";
import type { Mp4AtomSpeakerPosition } from "./Mp4AtomSpeakerPosition.ts";
import type { Mp4AtomSt3d } from "./Mp4AtomSt3d.ts";
import type { Mp4AtomStco } from "./Mp4AtomStco.ts";
import type { Mp4AtomStri } from "./Mp4AtomStri.ts";
import type { Mp4AtomStsc } from "./Mp4AtomStsc.ts";
import type { Mp4AtomStscEntry } from "./Mp4AtomStscEntry.ts";
import type { Mp4AtomStss } from "./Mp4AtomStss.ts";
import type { Mp4AtomStsz } from "./Mp4AtomStsz.ts";
import type { Mp4AtomStszSamples } from "./Mp4AtomStszSamples.ts";
import type { Mp4AtomStts } from "./Mp4AtomStts.ts";
import type { Mp4AtomSttsEntry } from "./Mp4AtomSttsEntry.ts";
import type { Mp4AtomStyp } from "./Mp4AtomStyp.ts";
import type { Mp4AtomSubs } from "./Mp4AtomSubs.ts";
import type { Mp4AtomSubsEntry } from "./Mp4AtomSubsEntry.ts";
import type { Mp4AtomSubsSubsample } from "./Mp4AtomSubsSubsample.ts";
import type { Mp4AtomSubsSubsampleSize } from "./Mp4AtomSubsSubsampleSize.ts";
import type { Mp4AtomSvhd } from "./Mp4AtomSvhd.ts";
import type { Mp4AtomTaic } from "./Mp4AtomTaic.ts";
import type { Mp4AtomTfdt } from "./Mp4AtomTfdt.ts";
import type { Mp4AtomTfhd } from "./Mp4AtomTfhd.ts";
import type { Mp4AtomTfra } from "./Mp4AtomTfra.ts";
import type { Mp4AtomTkhd } from "./Mp4AtomTkhd.ts";
import type { Mp4AtomTool } from "./Mp4AtomTool.ts";
import type { Mp4AtomTrackReferenceTypeBox } from "./Mp4AtomTrackReferenceTypeBox.ts";
import type { Mp4AtomTraf } from "./Mp4AtomTraf.ts";
import type { Mp4AtomTrak } from "./Mp4AtomTrak.ts";
import type { Mp4AtomTref } from "./Mp4AtomTref.ts";
import type { Mp4AtomTrex } from "./Mp4AtomTrex.ts";
import type { Mp4AtomTrun } from "./Mp4AtomTrun.ts";
import type { Mp4AtomTrunEntry } from "./Mp4AtomTrunEntry.ts";
import type { Mp4AtomTrunSamples } from "./Mp4AtomTrunSamples.ts";
import type { Mp4AtomTwos } from "./Mp4AtomTwos.ts";
import type { Mp4AtomTxtC } from "./Mp4AtomTxtC.ts";
import type { Mp4AtomU24 } from "./Mp4AtomU24.ts";
import type { Mp4AtomUncC } from "./Mp4AtomUncC.ts";
import type { Mp4AtomUncompressedComponent } from "./Mp4AtomUncompressedComponent.ts";
import type { Mp4AtomUri } from "./Mp4AtomUri.ts";
import type { Mp4AtomUriI } from "./Mp4AtomUriI.ts";
import type { Mp4AtomUrim } from "./Mp4AtomUrim.ts";
import type { Mp4AtomUrl } from "./Mp4AtomUrl.ts";
import type { Mp4AtomUrn } from "./Mp4AtomUrn.ts";
import type { Mp4AtomUuid } from "./Mp4AtomUuid.ts";
import type { Mp4AtomVexuProjKind } from "./Mp4AtomVexuProjKind.ts";
import type { Mp4AtomVlab } from "./Mp4AtomVlab.ts";
import type { Mp4AtomVmhd } from "./Mp4AtomVmhd.ts";
import type { Mp4AtomVpcC } from "./Mp4AtomVpcC.ts";
import type { Mp4AtomVttC } from "./Mp4AtomVttC.ts";
import type { Mp4AtomWvtt } from "./Mp4AtomWvtt.ts";
import type { Mp4AtomYear } from "./Mp4AtomYear.ts";
import type { Mp4AtomZeroed } from "./Mp4AtomZeroed.ts";
import type { MuxCatalogCatalogFormat } from "./MuxCatalogCatalogFormat.ts";
import type { MuxCatalogConsumer } from "./MuxCatalogConsumer.ts";
import type { MuxCatalogConsumerExt } from "./MuxCatalogConsumerExt.ts";
import type { MuxCatalogConsumerExtra } from "./MuxCatalogConsumerExtra.ts";
import type { MuxCatalogMsfError } from "./MuxCatalogMsfError.ts";
import type { MuxCodecAacAdtsError } from "./MuxCodecAacAdtsError.ts";
import type { MuxCodecAacConfig } from "./MuxCodecAacConfig.ts";
import type { MuxCodecFlacConfig } from "./MuxCodecFlacConfig.ts";
import type { MuxCodecFlacError } from "./MuxCodecFlacError.ts";
import type { MuxCodecH264Sps } from "./MuxCodecH264Sps.ts";
import type { MuxCodecMp3Config } from "./MuxCodecMp3Config.ts";
import type { MuxCodecMp3Error } from "./MuxCodecMp3Error.ts";
import type { MuxCodecOpusError } from "./MuxCodecOpusError.ts";
import type { MuxCodecOpusMapping } from "./MuxCodecOpusMapping.ts";
import type { MuxCodecVp8Error } from "./MuxCodecVp8Error.ts";
import type { MuxCodecVp9Error } from "./MuxCodecVp9Error.ts";
import type { MuxContainerFmp4Chunk } from "./MuxContainerFmp4Chunk.ts";
import type { MuxContainerFmp4Error } from "./MuxContainerFmp4Error.ts";
import type { MuxContainerFmp4Fragment } from "./MuxContainerFmp4Fragment.ts";
import type { MuxContainerFmp4FragmentConfig } from "./MuxContainerFmp4FragmentConfig.ts";
import type { MuxContainerFmp4FragmentMissingDuration } from "./MuxContainerFmp4FragmentMissingDuration.ts";
import type { MuxContainerFrame } from "./MuxContainerFrame.ts";
import type { MuxContainerKind } from "./MuxContainerKind.ts";
import type { MuxContainerLegacyWire } from "./MuxContainerLegacyWire.ts";
import type { MuxContainerLocWire } from "./MuxContainerLocWire.ts";
import type { MuxContainerMp4Finish } from "./MuxContainerMp4Finish.ts";
import type { MuxContainerRead } from "./MuxContainerRead.ts";
import type { MuxContainerTimestampRewind } from "./MuxContainerTimestampRewind.ts";
import type { MuxContainerTsDescriptor } from "./MuxContainerTsDescriptor.ts";
import type { MuxContainerTsFraming } from "./MuxContainerTsFraming.ts";
import type { MuxContainerTsRelease } from "./MuxContainerTsRelease.ts";
import type { MuxContainerTsSiEntry } from "./MuxContainerTsSiEntry.ts";
import type { MuxContainerTsStatsClass } from "./MuxContainerTsStatsClass.ts";
import type { MuxContainerTsStatsStream } from "./MuxContainerTsStatsStream.ts";
import type { MuxContainerTsStep } from "./MuxContainerTsStep.ts";
import type { MuxContainerTsTrack } from "./MuxContainerTsTrack.ts";
import type { MuxError } from "./MuxError.ts";
import type { MuxFollowEvent } from "./MuxFollowEvent.ts";
import type { MuxImportAudioFormat } from "./MuxImportAudioFormat.ts";
import type { MuxImportContainerFormat } from "./MuxImportContainerFormat.ts";
import type { MuxImportFormat } from "./MuxImportFormat.ts";
import type { MuxImportKind } from "./MuxImportKind.ts";
import type { MuxImportVideoFormat } from "./MuxImportVideoFormat.ts";
import type { MuxRateTarget } from "./MuxRateTarget.ts";
import type { MuxSelectSelected } from "./MuxSelectSelected.ts";
import type { MuxTimelineAudio } from "./MuxTimelineAudio.ts";
import type { MuxTimelineError } from "./MuxTimelineError.ts";
import type { MuxTimelineKind } from "./MuxTimelineKind.ts";
import type { MuxTimelineTransition } from "./MuxTimelineTransition.ts";
import type { OriginCost } from "./OriginCost.ts";
import type { OriginHopLimit } from "./OriginHopLimit.ts";
import type { OriginLink } from "./OriginLink.ts";
import type { OriginPreference } from "./OriginPreference.ts";
import type { OriginRequest } from "./OriginRequest.ts";
import type { OriginSource } from "./OriginSource.ts";
import type { PathError } from "./PathError.ts";
import type { Pattern } from "./Pattern.ts";
import type { Segment } from "./Segment.ts";
import type { ServerHandshake } from "./ServerHandshake.ts";
import type { Session } from "./Session.ts";
import type { SessionError } from "./SessionError.ts";
import type { SessionLimits } from "./SessionLimits.ts";
import type { SetupExtensions } from "./SetupExtensions.ts";
import type { SetupRole } from "./SetupRole.ts";
import type { SetupToken } from "./SetupToken.ts";
import type { StatsContent } from "./StatsContent.ts";
import type { StatsFailure } from "./StatsFailure.ts";
import type { StatsFailures } from "./StatsFailures.ts";
import type { StatsMessage } from "./StatsMessage.ts";
import type { StatsPresence } from "./StatsPresence.ts";
import type { StatsRefusalKind } from "./StatsRefusalKind.ts";
import type { StatsRefusals } from "./StatsRefusals.ts";
import type { StatsRole } from "./StatsRole.ts";
import type { StatsSessionEntry } from "./StatsSessionEntry.ts";
import type { StatsTier } from "./StatsTier.ts";
import type { StatsTotals } from "./StatsTotals.ts";
import type { StatsTraffic } from "./StatsTraffic.ts";
import type { StatsTrafficEntry } from "./StatsTrafficEntry.ts";
import type { StreamError } from "./StreamError.ts";
import type { TimeOverflow } from "./TimeOverflow.ts";
import type { TimedJson } from "./TimedJson.ts";
import type { Timescale } from "./Timescale.ts";
import type { Timestamp } from "./Timestamp.ts";
import type { TrackAnchor } from "./TrackAnchor.ts";
import type { TrackDelay } from "./TrackDelay.ts";
import type { TrackFill } from "./TrackFill.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TrackOrder } from "./TrackOrder.ts";
import type { TrackPosition } from "./TrackPosition.ts";
import type { TrackRange } from "./TrackRange.ts";
import type { TrackRequest } from "./TrackRequest.ts";
import type { TrackSubscriber } from "./TrackSubscriber.ts";
import type { TrackSubscription } from "./TrackSubscription.ts";
import type { TransportAddress } from "./TransportAddress.ts";
import type { TransportCloseKind } from "./TransportCloseKind.ts";
import type { TransportMemoryError } from "./TransportMemoryError.ts";
import type { TransportReceiveWindow } from "./TransportReceiveWindow.ts";
import type { Version } from "./Version.ts";
import type { Waiter } from "./Waiter.ts";
export declare function e_EncodeError(v: EncodeError): number;
export declare function e_IntersectionError(v: IntersectionError): number;
export declare function e_InvalidHop(v: InvalidHop): number;
export declare function e_auth_Answer(v: AuthAnswer): number;
export declare function e_auth_RequestKind(v: AuthRequestKind): number;
export declare function e_clock_MissedTickBehavior(v: ClockMissedTickBehavior): number;
export declare function e_hang_catalog_AudioCodecKind(v: HangCatalogAudioCodecKind): number;
export declare function e_hang_catalog_Range(v: HangCatalogRange): number;
export declare function e_hang_catalog_VideoCodecKind(v: HangCatalogVideoCodecKind): number;
export declare function e_hang_health_Input(v: HangHealthInput): number;
export declare function e_hang_health_Reset(v: HangHealthReset): number;
export declare function e_hang_health_Role(v: HangHealthRole): number;
export declare function e_hang_health_Verdict(v: HangHealthVerdict): number;
export declare function e_mp4_atom_AudioChannelPosition(v: Mp4AtomAudioChannelPosition): number;
export declare function e_mp4_atom_ClockType(v: Mp4AtomClockType): number;
export declare function e_mp4_atom_ReferenceTime(v: Mp4AtomReferenceTime): number;
export declare function e_mp4_atom_SencBoxVersion(v: Mp4AtomSencBoxVersion): number;
export declare function e_mp4_atom_VexuProjKind(v: Mp4AtomVexuProjKind): number;
export declare function e_mux_catalog_CatalogFormat(v: MuxCatalogCatalogFormat): number;
export declare function e_mux_codec_aac_adts_Error(v: MuxCodecAacAdtsError): number;
export declare function e_mux_codec_flac_Error(v: MuxCodecFlacError): number;
export declare function e_mux_codec_mp3_Error(v: MuxCodecMp3Error): number;
export declare function e_mux_codec_vp8_Error(v: MuxCodecVp8Error): number;
export declare function e_mux_codec_vp9_Error(v: MuxCodecVp9Error): number;
export declare function e_mux_container_Kind(v: MuxContainerKind): number;
export declare function e_mux_container_fmp4_fragment_MissingDuration(v: MuxContainerFmp4FragmentMissingDuration): number;
export declare function e_mux_container_ts_Framing(v: MuxContainerTsFraming): number;
export declare function e_mux_container_ts_stats_Class(v: MuxContainerTsStatsClass): number;
export declare function e_mux_import_AudioFormat(v: MuxImportAudioFormat): number;
export declare function e_mux_import_ContainerFormat(v: MuxImportContainerFormat): number;
export declare function e_mux_import_Kind(v: MuxImportKind): number;
export declare function e_mux_import_VideoFormat(v: MuxImportVideoFormat): number;
export declare function e_mux_timeline_Error(v: MuxTimelineError): number;
export declare function e_mux_timeline_Kind(v: MuxTimelineKind): number;
export declare function e_origin_Link(v: OriginLink): number;
export declare function e_path_Error(v: PathError): number;
export declare function e_setup_Role(v: SetupRole): number;
export declare function e_stats_Failure(v: StatsFailure): number;
export declare function e_stats_Message(v: StatsMessage): number;
export declare function e_stats_RefusalKind(v: StatsRefusalKind): number;
export declare function e_stats_Role(v: StatsRole): number;
export declare function e_track_Order(v: TrackOrder): number;
export declare function e_transport_Address(v: TransportAddress): number;
export declare function e_transport_CloseKind(v: TransportCloseKind): number;
export declare function f0(): number;
export declare function f1(): number;
export declare function f2(): number;
export declare function f3(): number;
export declare function f4(): number;
export declare function f_EncodeError(c: number): EncodeError;
export declare function f_IntersectionError(c: number): IntersectionError;
export declare function f_InvalidHop(c: number): InvalidHop;
export declare function f_auth_Answer(c: number): AuthAnswer;
export declare function f_auth_RequestKind(c: number): AuthRequestKind;
export declare function f_clock_MissedTickBehavior(c: number): ClockMissedTickBehavior;
export declare function f_hang_catalog_AudioCodecKind(c: number): HangCatalogAudioCodecKind;
export declare function f_hang_catalog_Range(c: number): HangCatalogRange;
export declare function f_hang_catalog_VideoCodecKind(c: number): HangCatalogVideoCodecKind;
export declare function f_hang_health_Input(c: number): HangHealthInput;
export declare function f_hang_health_Verdict(c: number): HangHealthVerdict;
export declare function f_mp4_atom_AudioChannelPosition(c: number): Mp4AtomAudioChannelPosition;
export declare function f_mp4_atom_ClockType(c: number): Mp4AtomClockType;
export declare function f_mp4_atom_ReferenceTime(c: number): Mp4AtomReferenceTime;
export declare function f_mp4_atom_SencBoxVersion(c: number): Mp4AtomSencBoxVersion;
export declare function f_mp4_atom_VexuProjKind(c: number): Mp4AtomVexuProjKind;
export declare function f_mux_catalog_CatalogFormat(c: number): MuxCatalogCatalogFormat;
export declare function f_mux_codec_aac_adts_Error(c: number): MuxCodecAacAdtsError;
export declare function f_mux_codec_flac_Error(c: number): MuxCodecFlacError;
export declare function f_mux_codec_mp3_Error(c: number): MuxCodecMp3Error;
export declare function f_mux_container_fmp4_fragment_MissingDuration(c: number): MuxContainerFmp4FragmentMissingDuration;
export declare function f_mux_container_ts_Framing(c: number): MuxContainerTsFraming;
export declare function f_mux_container_ts_stats_Class(c: number): MuxContainerTsStatsClass;
export declare function f_mux_import_AudioFormat(c: number): MuxImportAudioFormat;
export declare function f_mux_import_ContainerFormat(c: number): MuxImportContainerFormat;
export declare function f_mux_import_Kind(c: number): MuxImportKind;
export declare function f_mux_import_VideoFormat(c: number): MuxImportVideoFormat;
export declare function f_mux_timeline_Error(c: number): MuxTimelineError;
export declare function f_origin_Link(c: number): OriginLink;
export declare function f_path_Error(c: number): PathError;
export declare function f_setup_Role(c: number): SetupRole;
export declare function f_stats_Failure(c: number): StatsFailure;
export declare function f_stats_Message(c: number): StatsMessage;
export declare function f_stats_RefusalKind(c: number): StatsRefusalKind;
export declare function f_stats_Role(c: number): StatsRole;
export declare function f_track_Order(c: number): TrackOrder;
export declare function f_transport_CloseKind(c: number): TransportCloseKind;
export declare const k_Client: rt.Kind;
export declare const k_DecodeError: rt.Kind;
export declare const k_Driver: rt.Kind;
export declare const k_Error: rt.Kind;
export declare const k_Hops: rt.Kind;
export declare const k_Instance: rt.Kind;
export declare const k_Instant: rt.Kind;
export declare const k_Json: rt.Kind;
export declare const k_Park: rt.Kind;
export declare const k_Path: rt.Kind;
export declare const k_Pattern: rt.Kind;
export declare const k_Patterns: rt.Kind;
export declare const k_Server: rt.Kind;
export declare const k_Session: rt.Kind;
export declare const k_Specificity: rt.Kind;
export declare const k_Timed_Json: rt.Kind;
export declare const k_Url: rt.Kind;
export declare const k_Version: rt.Kind;
export declare const k_Versions: rt.Kind;
export declare const k_Waiter: rt.Kind;
export declare const k_WaiterList: rt.Kind;
export declare const k_announce_Consumer: rt.Kind;
export declare const k_announce_Event: rt.Kind;
export declare const k_auth_Credential: rt.Kind;
export declare const k_auth_Grant: rt.Kind;
export declare const k_auth_Handle: rt.Kind;
export declare const k_auth_Issued: rt.Kind;
export declare const k_auth_Request: rt.Kind;
export declare const k_auth_Requests: rt.Kind;
export declare const k_auth_Watch: rt.Kind;
export declare const k_bandwidth_Allocator: rt.Kind;
export declare const k_bandwidth_Consumer: rt.Kind;
export declare const k_bandwidth_Demand: rt.Kind;
export declare const k_bandwidth_Producer: rt.Kind;
export declare const k_bandwidth_Reservation: rt.Kind;
export declare const k_bandwidth_Viewer: rt.Kind;
export declare const k_bandwidth_Viewers: rt.Kind;
export declare const k_bandwidth_Weak: rt.Kind;
export declare const k_broadcast_Consumer: rt.Kind;
export declare const k_broadcast_DatagramRequests: rt.Kind;
export declare const k_broadcast_Demand: rt.Kind;
export declare const k_broadcast_Info: rt.Kind;
export declare const k_broadcast_Lister: rt.Kind;
export declare const k_broadcast_Producer: rt.Kind;
export declare const k_broadcast_Requests: rt.Kind;
export declare const k_broadcast_Tracks: rt.Kind;
export declare const k_broadcast_TracksEvent: rt.Kind;
export declare const k_cache_Config: rt.Kind;
export declare const k_cache_Pool: rt.Kind;
export declare const k_cache_PoolWeak: rt.Kind;
export declare const k_clock_Clock: rt.Kind;
export declare const k_clock_Deadline: rt.Kind;
export declare const k_clock_Deadlines: rt.Kind;
export declare const k_clock_Entered: rt.Kind;
export declare const k_clock_Interval: rt.Kind;
export declare const k_clock_Manual: rt.Kind;
export declare const k_clock_Rng: rt.Kind;
export declare const k_clock_WallMap: rt.Kind;
export declare const k_coop_Budget: rt.Kind;
export declare const k_datagram_Consumer: rt.Kind;
export declare const k_datagram_Info: rt.Kind;
export declare const k_datagram_Producer: rt.Kind;
export declare const k_datagram_Request: rt.Kind;
export declare const k_frame_Buffer: rt.Kind;
export declare const k_frame_Consumer: rt.Kind;
export declare const k_frame_Frame: rt.Kind;
export declare const k_frame_Producer: rt.Kind;
export declare const k_goaway_Consumer: rt.Kind;
export declare const k_goaway_Goaway: rt.Kind;
export declare const k_goaway_Producer: rt.Kind;
export declare const k_group_Consumer: rt.Kind;
export declare const k_group_Demand: rt.Kind;
export declare const k_group_Fetch: rt.Kind;
export declare const k_group_Producer: rt.Kind;
export declare const k_hang_Catalog: rt.Kind;
export declare const k_hang_Catalog_Ext: rt.Kind;
export declare const k_hang_Catalog_Extra: rt.Kind;
export declare const k_hang_Error: rt.Kind;
export declare const k_hang_catalog_Audio: rt.Kind;
export declare const k_hang_catalog_AudioConfig: rt.Kind;
export declare const k_hang_catalog_Binary: rt.Kind;
export declare const k_hang_catalog_BinaryConfig: rt.Kind;
export declare const k_hang_catalog_Color: rt.Kind;
export declare const k_hang_catalog_Consumer: rt.Kind;
export declare const k_hang_catalog_Consumer_Ext: rt.Kind;
export declare const k_hang_catalog_Consumer_Extra: rt.Kind;
export declare const k_hang_catalog_Container: rt.Kind;
export declare const k_hang_catalog_Contribution: rt.Kind;
export declare const k_hang_catalog_Echo: rt.Kind;
export declare const k_hang_catalog_Join: rt.Kind;
export declare const k_hang_catalog_Json: rt.Kind;
export declare const k_hang_catalog_JsonConfig: rt.Kind;
export declare const k_hang_catalog_M2ts: rt.Kind;
export declare const k_hang_catalog_Media: rt.Kind;
export declare const k_hang_catalog_Projection: rt.Kind;
export declare const k_hang_catalog_Stats: rt.Kind;
export declare const k_hang_catalog_Text: rt.Kind;
export declare const k_hang_catalog_TextConfig: rt.Kind;
export declare const k_hang_catalog_Video: rt.Kind;
export declare const k_hang_catalog_VideoConfig: rt.Kind;
export declare const k_hang_catalog_VideoProperties: rt.Kind;
export declare const k_hang_echo_Arrival: rt.Kind;
export declare const k_hang_echo_Rendition: rt.Kind;
export declare const k_hang_echo_Snapshot: rt.Kind;
export declare const k_hang_echo_Snapshot_string: rt.Kind;
export declare const k_hang_health_Connection: rt.Kind;
export declare const k_hang_health_Evidence: rt.Kind;
export declare const k_hang_health_Observer: rt.Kind;
export declare const k_hang_health_PublisherThresholds: rt.Kind;
export declare const k_hang_health_Rendition: rt.Kind;
export declare const k_hang_health_Report_echo_Snapshot_string: rt.Kind;
export declare const k_hang_health_Report_stats_Snapshot_string: rt.Kind;
export declare const k_hang_health_Thresholds: rt.Kind;
export declare const k_hang_health_TransportThresholds: rt.Kind;
export declare const k_hang_health_ViewerThresholds: rt.Kind;
export declare const k_hang_stats_Rendition: rt.Kind;
export declare const k_hang_stats_Snapshot: rt.Kind;
export declare const k_hang_stats_Snapshot_string: rt.Kind;
export declare const k_hang_stats_Transport: rt.Kind;
export declare const k_mp4_atom_Any: rt.Kind;
export declare const k_mp4_atom_Av01: rt.Kind;
export declare const k_mp4_atom_Avc1: rt.Kind;
export declare const k_mp4_atom_Avc3: rt.Kind;
export declare const k_mp4_atom_Cams: rt.Kind;
export declare const k_mp4_atom_Cmfy: rt.Kind;
export declare const k_mp4_atom_Codec: rt.Kind;
export declare const k_mp4_atom_Compressor: rt.Kind;
export declare const k_mp4_atom_Eyes: rt.Kind;
export declare const k_mp4_atom_Hev1: rt.Kind;
export declare const k_mp4_atom_Hvc1: rt.Kind;
export declare const k_mp4_atom_Ipco: rt.Kind;
export declare const k_mp4_atom_Iprp: rt.Kind;
export declare const k_mp4_atom_Jpeg: rt.Kind;
export declare const k_mp4_atom_Mdia: rt.Kind;
export declare const k_mp4_atom_Meta: rt.Kind;
export declare const k_mp4_atom_Minf: rt.Kind;
export declare const k_mp4_atom_Moof: rt.Kind;
export declare const k_mp4_atom_Moov: rt.Kind;
export declare const k_mp4_atom_Nmhd: rt.Kind;
export declare const k_mp4_atom_Pack: rt.Kind;
export declare const k_mp4_atom_Proj: rt.Kind;
export declare const k_mp4_atom_Spatial: rt.Kind;
export declare const k_mp4_atom_Stbl: rt.Kind;
export declare const k_mp4_atom_Sthd: rt.Kind;
export declare const k_mp4_atom_Stsd: rt.Kind;
export declare const k_mp4_atom_Sv3d: rt.Kind;
export declare const k_mp4_atom_Traf: rt.Kind;
export declare const k_mp4_atom_Trak: rt.Kind;
export declare const k_mp4_atom_Tx3g: rt.Kind;
export declare const k_mp4_atom_Udta: rt.Kind;
export declare const k_mp4_atom_Vexu: rt.Kind;
export declare const k_mp4_atom_VexuProj: rt.Kind;
export declare const k_mp4_atom_Visual: rt.Kind;
export declare const k_mp4_atom_Vp08: rt.Kind;
export declare const k_mp4_atom_Vp09: rt.Kind;
export declare const k_mux_Binding: rt.Kind;
export declare const k_mux_Clock: rt.Kind;
export declare const k_mux_Error: rt.Kind;
export declare const k_mux_Linger_unit: rt.Kind;
export declare const k_mux_Pacer: rt.Kind;
export declare const k_mux_Source: rt.Kind;
export declare const k_mux_catalog_Config: rt.Kind;
export declare const k_mux_catalog_Config_Ext: rt.Kind;
export declare const k_mux_catalog_Config_Extra: rt.Kind;
export declare const k_mux_catalog_Consumer: rt.Kind;
export declare const k_mux_catalog_Consumer_Ext: rt.Kind;
export declare const k_mux_catalog_Consumer_Extra: rt.Kind;
export declare const k_mux_catalog_Estimate: rt.Kind;
export declare const k_mux_catalog_Estimator: rt.Kind;
export declare const k_mux_catalog_Guard: rt.Kind;
export declare const k_mux_catalog_Guard_Ext: rt.Kind;
export declare const k_mux_catalog_Guard_Extra: rt.Kind;
export declare const k_mux_catalog_Producer: rt.Kind;
export declare const k_mux_catalog_Producer_Ext: rt.Kind;
export declare const k_mux_catalog_Producer_Extra: rt.Kind;
export declare const k_mux_catalog_Reserved: rt.Kind;
export declare const k_mux_catalog_Reserved_Ext: rt.Kind;
export declare const k_mux_catalog_Reserved_Extra: rt.Kind;
export declare const k_mux_catalog_Timebase: rt.Kind;
export declare const k_mux_catalog_Timebase_Ext: rt.Kind;
export declare const k_mux_catalog_Timebase_Extra: rt.Kind;
export declare const k_mux_catalog_VideoHint: rt.Kind;
export declare const k_mux_catalog_hang_Consumer: rt.Kind;
export declare const k_mux_catalog_hang_Consumer_Ext: rt.Kind;
export declare const k_mux_catalog_hang_Consumer_Extra: rt.Kind;
export declare const k_mux_catalog_hang_Container: rt.Kind;
export declare const k_mux_catalog_hang_Extra: rt.Kind;
export declare const k_mux_catalog_msf_Consumer: rt.Kind;
export declare const k_mux_catalog_msf_Consumer_Ext: rt.Kind;
export declare const k_mux_catalog_msf_Consumer_Extra: rt.Kind;
export declare const k_mux_codec_aac_Error: rt.Kind;
export declare const k_mux_codec_aac_Import: rt.Kind;
export declare const k_mux_codec_aac_adts_Header: rt.Kind;
export declare const k_mux_codec_annexb_Error: rt.Kind;
export declare const k_mux_codec_av1_Import: rt.Kind;
export declare const k_mux_codec_av1_Split: rt.Kind;
export declare const k_mux_codec_flac_Import: rt.Kind;
export declare const k_mux_codec_h264_Avc1: rt.Kind;
export declare const k_mux_codec_h264_Avcc: rt.Kind;
export declare const k_mux_codec_h264_Error: rt.Kind;
export declare const k_mux_codec_h264_Import: rt.Kind;
export declare const k_mux_codec_h264_Split: rt.Kind;
export declare const k_mux_codec_h265_Error: rt.Kind;
export declare const k_mux_codec_h265_Hvc1: rt.Kind;
export declare const k_mux_codec_h265_Hvcc: rt.Kind;
export declare const k_mux_codec_h265_Import: rt.Kind;
export declare const k_mux_codec_h265_Split: rt.Kind;
export declare const k_mux_codec_mp3_Import: rt.Kind;
export declare const k_mux_codec_opus_Config: rt.Kind;
export declare const k_mux_codec_opus_Import: rt.Kind;
export declare const k_mux_codec_vp8_Import: rt.Kind;
export declare const k_mux_codec_vp9_Import: rt.Kind;
export declare const k_mux_container_Consumer_Container: rt.Kind;
export declare const k_mux_container_Consumer_fmp4_Wire: rt.Kind;
export declare const k_mux_container_Consumer_legacy_Wire: rt.Kind;
export declare const k_mux_container_Consumer_loc_Wire: rt.Kind;
export declare const k_mux_container_GroupConsumer_Container: rt.Kind;
export declare const k_mux_container_GroupConsumer_fmp4_Wire: rt.Kind;
export declare const k_mux_container_GroupConsumer_legacy_Wire: rt.Kind;
export declare const k_mux_container_GroupConsumer_loc_Wire: rt.Kind;
export declare const k_mux_container_Producer_Container: rt.Kind;
export declare const k_mux_container_Producer_Container_AudioConfig: rt.Kind;
export declare const k_mux_container_Producer_Container_BinaryConfig: rt.Kind;
export declare const k_mux_container_Producer_Container_JsonConfig: rt.Kind;
export declare const k_mux_container_Producer_Container_TextConfig: rt.Kind;
export declare const k_mux_container_Producer_Container_VideoConfig: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire_AudioConfig: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire_BinaryConfig: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire_JsonConfig: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire_TextConfig: rt.Kind;
export declare const k_mux_container_Producer_fmp4_Wire_VideoConfig: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire_AudioConfig: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire_BinaryConfig: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire_JsonConfig: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire_TextConfig: rt.Kind;
export declare const k_mux_container_Producer_legacy_Wire_VideoConfig: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire_AudioConfig: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire_BinaryConfig: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire_JsonConfig: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire_TextConfig: rt.Kind;
export declare const k_mux_container_Producer_loc_Wire_VideoConfig: rt.Kind;
export declare const k_mux_container_flv_Import: rt.Kind;
export declare const k_mux_container_flv_Import_Ext: rt.Kind;
export declare const k_mux_container_flv_Import_Extra: rt.Kind;
export declare const k_mux_container_fmp4_Error: rt.Kind;
export declare const k_mux_container_fmp4_Fragmenter: rt.Kind;
export declare const k_mux_container_fmp4_Import: rt.Kind;
export declare const k_mux_container_fmp4_Import_Ext: rt.Kind;
export declare const k_mux_container_fmp4_Import_Extra: rt.Kind;
export declare const k_mux_container_fmp4_Muxer: rt.Kind;
export declare const k_mux_container_fmp4_Wire: rt.Kind;
export declare const k_mux_container_mkv_Import: rt.Kind;
export declare const k_mux_container_mkv_Import_Ext: rt.Kind;
export declare const k_mux_container_mkv_Import_Extra: rt.Kind;
export declare const k_mux_container_ts_Export_Consumer: rt.Kind;
export declare const k_mux_container_ts_Export_Consumer_Ext: rt.Kind;
export declare const k_mux_container_ts_Export_Consumer_Extra: rt.Kind;
export declare const k_mux_container_ts_Ext: rt.Kind;
export declare const k_mux_container_ts_Follower: rt.Kind;
export declare const k_mux_container_ts_Follower_Ext: rt.Kind;
export declare const k_mux_container_ts_Follower_Extra: rt.Kind;
export declare const k_mux_container_ts_Import: rt.Kind;
export declare const k_mux_container_ts_Import_Ext: rt.Kind;
export declare const k_mux_container_ts_Import_Extra: rt.Kind;
export declare const k_mux_container_ts_Mpegts: rt.Kind;
export declare const k_mux_container_ts_Passthrough: rt.Kind;
export declare const k_mux_container_ts_Passthrough_Ext: rt.Kind;
export declare const k_mux_container_ts_Passthrough_Extra: rt.Kind;
export declare const k_mux_container_ts_PesPacket: rt.Kind;
export declare const k_mux_container_ts_Program: rt.Kind;
export declare const k_mux_container_ts_Programs: rt.Kind;
export declare const k_mux_container_ts_Release: rt.Kind;
export declare const k_mux_container_ts_SiEntry: rt.Kind;
export declare const k_mux_container_ts_Track: rt.Kind;
export declare const k_mux_container_ts_Verbatim: rt.Kind;
export declare const k_mux_container_ts_stats_Export: rt.Kind;
export declare const k_mux_container_ts_stats_Log: rt.Kind;
export declare const k_mux_container_ts_stats_Release: rt.Kind;
export declare const k_mux_container_ts_stats_Report: rt.Kind;
export declare const k_mux_container_ts_stats_Snapshot: rt.Kind;
export declare const k_mux_container_ts_stats_Stream: rt.Kind;
export declare const k_mux_follow_Event: rt.Kind;
export declare const k_mux_follow_Instances: rt.Kind;
export declare const k_mux_import_AudioInit: rt.Kind;
export declare const k_mux_import_Container: rt.Kind;
export declare const k_mux_import_ContainerInit: rt.Kind;
export declare const k_mux_import_Container_Ext: rt.Kind;
export declare const k_mux_import_Container_Extra: rt.Kind;
export declare const k_mux_import_Track: rt.Kind;
export declare const k_mux_import_TrackStream: rt.Kind;
export declare const k_mux_import_VideoInit: rt.Kind;
export declare const k_mux_json_Config: rt.Kind;
export declare const k_mux_json_Consumer_Json: rt.Kind;
export declare const k_mux_json_Options: rt.Kind;
export declare const k_mux_json_Snapshot_Json: rt.Kind;
export declare const k_mux_json_Stream_Json: rt.Kind;
export declare const k_mux_rate_Control: rt.Kind;
export declare const k_mux_rate_Follower: rt.Kind;
export declare const k_mux_rate_Policy: rt.Kind;
export declare const k_mux_select_Audio: rt.Kind;
export declare const k_mux_select_Broadcast: rt.Kind;
export declare const k_mux_select_Video: rt.Kind;
export declare const k_mux_timeline_Consumer: rt.Kind;
export declare const k_mux_timeline_Producer: rt.Kind;
export declare const k_mux_timeline_Track: rt.Kind;
export declare const k_origin_Config: rt.Kind;
export declare const k_origin_Consumer: rt.Kind;
export declare const k_origin_Driver: rt.Kind;
export declare const k_origin_Following: rt.Kind;
export declare const k_origin_Prepared: rt.Kind;
export declare const k_origin_Producer: rt.Kind;
export declare const k_origin_Publication: rt.Kind;
export declare const k_origin_Request: rt.Kind;
export declare const k_origin_Requests: rt.Kind;
export declare const k_origin_Route: rt.Kind;
export declare const k_path_Relative: rt.Kind;
export declare const k_server_Active: rt.Kind;
export declare const k_server_Activity: rt.Kind;
export declare const k_server_Handshake: rt.Kind;
export declare const k_session_Stats: rt.Kind;
export declare const k_stats_Config: rt.Kind;
export declare const k_stats_Handle: rt.Kind;
export declare const k_stats_Registry: rt.Kind;
export declare const k_stats_Report: rt.Kind;
export declare const k_stats_Session: rt.Kind;
export declare const k_stats_SessionEntry: rt.Kind;
export declare const k_stats_Snapshot: rt.Kind;
export declare const k_stats_Tier: rt.Kind;
export declare const k_stats_TrafficEntry: rt.Kind;
export declare const k_stats_Wire: rt.Kind;
export declare const k_track_Consumer: rt.Kind;
export declare const k_track_Control: rt.Kind;
export declare const k_track_Demand: rt.Kind;
export declare const k_track_Fill: rt.Kind;
export declare const k_track_Info: rt.Kind;
export declare const k_track_Ordered: rt.Kind;
export declare const k_track_Producer: rt.Kind;
export declare const k_track_Request: rt.Kind;
export declare const k_track_Requests: rt.Kind;
export declare const k_track_Subscriber: rt.Kind;
export declare const k_track_Subscription: rt.Kind;
export declare const k_transport_Probe: rt.Kind;
export declare const k_transport_Wire: rt.Kind;
export declare const k_transport_memory_Config: rt.Kind;
export declare const k_transport_memory_Session: rt.Kind;
export declare function p_Client_connect_Session(f: number, w$: number): [boolean, rt.Result<[Session, Driver], Error_>];
export declare function p_Server_accept_Session(f: number, w$: number): [boolean, rt.Result<[Session, Driver], Error_>];
export declare function p_Server_accept_request_Session(f: number, w$: number): [boolean, rt.Result<ServerHandshake, Error_>];
export declare function p_Session_close(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_Session_closed(f: number, w$: number): [boolean, Error_];
export declare function p_Session_idle(f: number, w$: number): [boolean, void];
export declare function p_Session_peer_implementation(f: number, w$: number): [boolean, string | undefined];
export declare function p_Waiter_poll_future(f: number, w$: number): [boolean, void];
export declare function p_announce_Consumer_next(f: number, w$: number): [boolean, AnnounceEvent | undefined];
export declare function p_announce_Consumer_poll_synced(f: number, w$: number): [boolean, void];
export declare function p_auth_Credential_closed(f: number, w$: number): [boolean, Error_];
export declare function p_auth_Handle_add(f: number, w$: number): [boolean, rt.Result<[AuthCredential, AuthAnswer], Error_>];
export declare function p_auth_Issued_closed(f: number, w$: number): [boolean, Error_];
export declare function p_auth_Request_closed(f: number, w$: number): [boolean, Error_];
export declare function p_auth_Requests_next(f: number, w$: number): [boolean, AuthRequest | undefined];
export declare function p_auth_Watch_changed(f: number, w$: number): [boolean, rt.Result<AuthGrant | undefined, Error_>];
export declare function p_bandwidth_Consumer_changed(f: number, w$: number): [boolean, rt.Result<BandwidthRate | undefined, Error_>];
export declare function p_bandwidth_Demand_wanted_changed(f: number, w$: number): [boolean, rt.Result<boolean, Error_>];
export declare function p_broadcast_Consumer_closed(f: number, w$: number): [boolean, void];
export declare function p_broadcast_DatagramRequests_requested(f: number, w$: number): [boolean, rt.Result<DatagramRequest, Error_>];
export declare function p_broadcast_Demand_wanted_changed(f: number, w$: number): [boolean, rt.Result<boolean, Error_>];
export declare function p_broadcast_Lister_poll_unwanted(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_broadcast_Lister_poll_wanted(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_broadcast_Requests_closed(f: number, w$: number): [boolean, void];
export declare function p_broadcast_Requests_requested_track(f: number, w$: number): [boolean, rt.Result<TrackRequest, Error_>];
export declare function p_broadcast_Tracks_next(f: number, w$: number): [boolean, rt.Result<BroadcastTracksEvent | undefined, Error_>];
export declare function p_clock_Deadline_poll(f: number, w$: number): [boolean, void];
export declare function p_clock_Interval_tick(f: number, w$: number): [boolean, ClockInstant];
export declare function p_clock_drive(f: number, w$: number): [boolean, string];
export declare function p_clock_sleep(f: number, w$: number): [boolean, void];
export declare function p_clock_sleep_until(f: number, w$: number): [boolean, void];
export declare function p_clock_timeout(f: number, w$: number): [boolean, rt.Result<undefined, ClockElapsed>];
export declare function p_clock_timeout_at(f: number, w$: number): [boolean, rt.Result<undefined, ClockElapsed>];
export declare function p_coop_Budget_poll_yield(f: number, w$: number): [boolean, void];
export declare function p_datagram_Consumer_info(f: number, w$: number): [boolean, rt.Result<DatagramInfo, Error_>];
export declare function p_datagram_Consumer_recv(f: number, w$: number): [boolean, rt.Result<Datagram | undefined, Error_>];
export declare function p_datagram_Producer_poll_priority(f: number, w$: number): [boolean, number | undefined];
export declare function p_frame_Consumer_read_all(f: number, w$: number): [boolean, rt.Result<rt.Bytes, Error_>];
export declare function p_frame_Consumer_read_chunk(f: number, w$: number): [boolean, rt.Result<rt.Bytes | undefined, Error_>];
export declare function p_goaway_Consumer_poll(f: number, w$: number): [boolean, rt.Result<GoawayGoaway, Error_>];
export declare function p_group_Consumer_finished(f: number, w$: number): [boolean, rt.Result<rt.u64, Error_>];
export declare function p_group_Consumer_next_frame(f: number, w$: number): [boolean, rt.Result<FrameConsumer | undefined, Error_>];
export declare function p_group_Consumer_poll_read_frames_8(f: number, w$: number): [boolean, rt.Result<number, Error_>];
export declare function p_group_Consumer_read_frame(f: number, w$: number): [boolean, rt.Result<FrameFrame | undefined, Error_>];
export declare function p_group_Consumer_read_frames_8(f: number, w$: number): [boolean, rt.Result<FrameFrame[], Error_>];
export declare function p_group_Demand_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_group_Demand_wanted_changed(f: number, w$: number): [boolean, rt.Result<boolean, Error_>];
export declare function p_group_Producer_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_group_Producer_ready(f: number, w$: number): [boolean, void];
export declare function p_hang_Catalog_Ext_subscribe(f: number, w$: number): [boolean, rt.Result<HangCatalogConsumerExt, HangError>];
export declare function p_hang_Catalog_Extra_subscribe(f: number, w$: number): [boolean, rt.Result<HangCatalogConsumerExtra, HangError>];
export declare function p_hang_Catalog_subscribe(f: number, w$: number): [boolean, rt.Result<HangCatalogConsumer, HangError>];
export declare function p_hang_catalog_Consumer_Ext_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExt | undefined, HangError>];
export declare function p_hang_catalog_Consumer_Extra_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExtra | undefined, HangError>];
export declare function p_hang_catalog_Consumer_next(f: number, w$: number): [boolean, rt.Result<HangCatalog | undefined, HangError>];
export declare function p_mux_Binding_broadcast(f: number, w$: number): [boolean, rt.Result<BroadcastConsumer, MuxError>];
export declare function p_mux_Linger_unit_wait(f: number, w$: number): [boolean, void];
export declare function p_mux_Source_catalog_Ext(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumerExt, MuxError>];
export declare function p_mux_Source_catalog_Extra(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumerExtra, MuxError>];
export declare function p_mux_Source_catalog_unit(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumer, MuxError>];
export declare function p_mux_Source_resolve(f: number, w$: number): [boolean, rt.Result<BroadcastConsumer, MuxError>];
export declare function p_mux_Source_subscribe_track(f: number, w$: number): [boolean, rt.Result<TrackSubscriber, MuxError>];
export declare function p_mux_catalog_Consumer_Ext_new(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumerExt, MuxError>];
export declare function p_mux_catalog_Consumer_Extra_new(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumerExtra, MuxError>];
export declare function p_mux_catalog_Consumer_new(f: number, w$: number): [boolean, rt.Result<MuxCatalogConsumer, MuxError>];
export declare function p_mux_catalog_hang_Consumer_Ext_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExt | undefined, MuxError>];
export declare function p_mux_catalog_hang_Consumer_Extra_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExtra | undefined, MuxError>];
export declare function p_mux_catalog_hang_Consumer_next(f: number, w$: number): [boolean, rt.Result<HangCatalog | undefined, MuxError>];
export declare function p_mux_catalog_msf_Consumer_Ext_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExt | undefined, MuxError>];
export declare function p_mux_catalog_msf_Consumer_Extra_next(f: number, w$: number): [boolean, rt.Result<HangCatalogExtra | undefined, MuxError>];
export declare function p_mux_catalog_msf_Consumer_next(f: number, w$: number): [boolean, rt.Result<HangCatalog | undefined, MuxError>];
export declare function p_mux_container_Consumer_Container_read(f: number, w$: number): [boolean, rt.Result<MuxContainerRead | undefined, MuxError>];
export declare function p_mux_container_Consumer_fmp4_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerRead | undefined, MuxContainerFmp4Error>];
export declare function p_mux_container_Consumer_legacy_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerRead | undefined, MuxError>];
export declare function p_mux_container_Consumer_loc_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerRead | undefined, MuxError>];
export declare function p_mux_container_GroupConsumer_Container_read(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_GroupConsumer_fmp4_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxContainerFmp4Error>];
export declare function p_mux_container_GroupConsumer_legacy_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_GroupConsumer_loc_Wire_read(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_Producer_Container_AudioConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_Container_BinaryConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_Container_JsonConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_Container_TextConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_Container_VideoConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_Container_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_AudioConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_BinaryConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_JsonConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_TextConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_VideoConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_fmp4_Wire_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_AudioConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_BinaryConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_JsonConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_TextConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_VideoConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_legacy_Wire_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_AudioConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_BinaryConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_JsonConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_TextConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_VideoConfig_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_Producer_loc_Wire_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_mux_container_fmp4_Muxer_read(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame[], MuxError>];
export declare function p_mux_container_ts_Export_Consumer_Ext_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Export_Consumer_Extra_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Export_Consumer_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Follower_Ext_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Follower_Extra_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Follower_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_container_ts_Release_new(f: number, w$: number): [boolean, rt.Result<MuxContainerTsRelease, MuxError>];
export declare function p_mux_container_ts_Release_next(f: number, w$: number): [boolean, rt.Result<MuxContainerFrame | undefined, MuxError>];
export declare function p_mux_follow_Instances_next(f: number, w$: number): [boolean, rt.Result<MuxFollowEvent, MuxError>];
export declare function p_mux_json_Consumer_Json_latest(f: number, w$: number): [boolean, rt.Result<TimedJson | undefined, MuxError>];
export declare function p_mux_json_Consumer_Json_next(f: number, w$: number): [boolean, rt.Result<TimedJson | undefined, MuxError>];
export declare function p_mux_json_Snapshot_Json_refresh(f: number, w$: number): [boolean, rt.Result<undefined, MuxError>];
export declare function p_mux_rate_Follower_next(f: number, w$: number): [boolean, rt.Result<MuxRateTarget, Error_>];
export declare function p_origin_Consumer_request(f: number, w$: number): [boolean, rt.Result<BroadcastConsumer, Error_>];
export declare function p_origin_Following_available(f: number, w$: number): [boolean, rt.Result<BroadcastConsumer, Error_>];
export declare function p_origin_Requests_requested_broadcast(f: number, w$: number): [boolean, rt.Result<OriginRequest, Error_>];
export declare function p_server_Activity_close(f: number, w$: number): [boolean, void];
export declare function p_server_Handshake_ok(f: number, w$: number): [boolean, rt.Result<[Session, Driver], Error_>];
export declare function p_time_run_Driver(f: number, w$: number): [boolean, Error_];
export declare function p_time_run_origin_Driver(f: number, w$: number): [boolean, Error_];
export declare function p_track_Consumer_fetch_group(f: number, w$: number): [boolean, rt.Result<GroupConsumer, Error_>];
export declare function p_track_Consumer_info(f: number, w$: number): [boolean, rt.Result<TrackInfo, Error_>];
export declare function p_track_Consumer_largest(f: number, w$: number): [boolean, TrackPosition | undefined];
export declare function p_track_Consumer_subscribe(f: number, w$: number): [boolean, rt.Result<TrackSubscriber, Error_>];
export declare function p_track_Demand_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_track_Demand_wanted_changed(f: number, w$: number): [boolean, rt.Result<rt.u64 | undefined, Error_>];
export declare function p_track_Fill_unwanted(f: number, w$: number): [boolean, void];
export declare function p_track_Fill_wanted_changed(f: number, w$: number): [boolean, rt.u64 | undefined];
export declare function p_track_Ordered_finished(f: number, w$: number): [boolean, rt.Result<TrackPosition, Error_>];
export declare function p_track_Ordered_largest(f: number, w$: number): [boolean, TrackPosition | undefined];
export declare function p_track_Ordered_next_group(f: number, w$: number): [boolean, rt.Result<GroupConsumer | undefined, Error_>];
export declare function p_track_Producer_closed(f: number, w$: number): [boolean, rt.Result<undefined, Error_>];
export declare function p_track_Producer_subscription_changed(f: number, w$: number): [boolean, rt.Result<TrackSubscription | undefined, Error_>];
export declare function p_track_Request_subscription_changed(f: number, w$: number): [boolean, TrackSubscription | undefined];
export declare function p_track_Requests_requested_fill(f: number, w$: number): [boolean, rt.Result<TrackFill, Error_>];
export declare function p_track_Subscriber_finished(f: number, w$: number): [boolean, rt.Result<TrackPosition, Error_>];
export declare function p_track_Subscriber_largest(f: number, w$: number): [boolean, TrackPosition | undefined];
export declare function p_track_Subscriber_recv_group(f: number, w$: number): [boolean, rt.Result<GroupConsumer | undefined, Error_>];
export declare function r_Datagram(p: number, o: boolean): Datagram;
export declare function r_Hop(p: number, o: boolean): Hop;
export declare function r_InvalidPattern(p: number, o: boolean): InvalidPattern;
export declare function r_Segment(p: number, o: boolean): Segment;
export declare function r_SessionError(p: number, o: boolean): SessionError;
export declare function r_StreamError(p: number, o: boolean): StreamError;
export declare function r_TimeOverflow(p: number, o: boolean): TimeOverflow;
export declare function r_Timescale(p: number, o: boolean): Timescale;
export declare function r_Timestamp(p: number, o: boolean): Timestamp;
export declare function r_bandwidth_Rate(p: number, o: boolean): BandwidthRate;
export declare function r_bound_u64(p: number, o: boolean): {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
};
export declare function r_clock_Elapsed(p: number, o: boolean): ClockElapsed;
export declare function r_clock_Instant(p: number, o: boolean): ClockInstant;
export declare function r_clock_Wall(p: number, o: boolean): ClockWall;
export declare function r_hang_catalog_AAC(p: number, o: boolean): HangCatalogAac;
export declare function r_hang_catalog_AV1(p: number, o: boolean): HangCatalogAv1;
export declare function r_hang_catalog_AudioCodec(p: number, o: boolean): HangCatalogAudioCodec;
export declare function r_hang_catalog_AudioLayout(p: number, o: boolean): HangCatalogAudioLayout;
export declare function r_hang_catalog_Chromaticity(p: number, o: boolean): HangCatalogChromaticity;
export declare function r_hang_catalog_Clock(p: number, o: boolean): HangCatalogClock;
export declare function r_hang_catalog_ContentLight(p: number, o: boolean): HangCatalogContentLight;
export declare function r_hang_catalog_Display(p: number, o: boolean): HangCatalogDisplay;
export declare function r_hang_catalog_H264(p: number, o: boolean): HangCatalogH264;
export declare function r_hang_catalog_H265(p: number, o: boolean): HangCatalogH265;
export declare function r_hang_catalog_Mastering(p: number, o: boolean): HangCatalogMastering;
export declare function r_hang_catalog_Mode(p: number, o: boolean): HangCatalogMode;
export declare function r_hang_catalog_Priorities(p: number, o: boolean): HangCatalogPriorities;
export declare function r_hang_catalog_Stereo(p: number, o: boolean): HangCatalogStereo;
export declare function r_hang_catalog_TextFormat(p: number, o: boolean): HangCatalogTextFormat;
export declare function r_hang_catalog_TextRole(p: number, o: boolean): HangCatalogTextRole;
export declare function r_hang_catalog_VP9(p: number, o: boolean): HangCatalogVp9;
export declare function r_hang_catalog_VideoCodec(p: number, o: boolean): HangCatalogVideoCodec;
export declare function r_hang_container_Frame(p: number, o: boolean): HangContainerFrame;
export declare function r_hang_health_Limits_duration(p: number, o: boolean): HangHealthLimitsDuration;
export declare function r_hang_health_Limits_f64(p: number, o: boolean): HangHealthLimitsF64;
export declare function r_mp4_atom_Ainf(p: number, o: boolean): Mp4AtomAinf;
export declare function r_mp4_atom_AnySampleGroupEntry(p: number, o: boolean): Mp4AtomAnySampleGroupEntry;
export declare function r_mp4_atom_Audio(p: number, o: boolean): Mp4AtomAudio;
export declare function r_mp4_atom_AuxInfo(p: number, o: boolean): Mp4AtomAuxInfo;
export declare function r_mp4_atom_Auxc(p: number, o: boolean): Mp4AtomAuxc;
export declare function r_mp4_atom_Av1c(p: number, o: boolean): Mp4AtomAv1c;
export declare function r_mp4_atom_Avcc(p: number, o: boolean): Mp4AtomAvcc;
export declare function r_mp4_atom_AvccExt(p: number, o: boolean): Mp4AtomAvccExt;
export declare function r_mp4_atom_Blin(p: number, o: boolean): Mp4AtomBlin;
export declare function r_mp4_atom_Btrt(p: number, o: boolean): Mp4AtomBtrt;
export declare function r_mp4_atom_Cbmp(p: number, o: boolean): Mp4AtomCbmp;
export declare function r_mp4_atom_Ccst(p: number, o: boolean): Mp4AtomCcst;
export declare function r_mp4_atom_ChannelStructure(p: number, o: boolean): Mp4AtomChannelStructure;
export declare function r_mp4_atom_Chnl(p: number, o: boolean): Mp4AtomChnl;
export declare function r_mp4_atom_Chpl(p: number, o: boolean): Mp4AtomChpl;
export declare function r_mp4_atom_ChplEntry(p: number, o: boolean): Mp4AtomChplEntry;
export declare function r_mp4_atom_Clap(p: number, o: boolean): Mp4AtomClap;
export declare function r_mp4_atom_Cmpd(p: number, o: boolean): Mp4AtomCmpd;
export declare function r_mp4_atom_Co64(p: number, o: boolean): Mp4AtomCo64;
export declare function r_mp4_atom_Colr(p: number, o: boolean): Mp4AtomColr;
export declare function r_mp4_atom_Component(p: number, o: boolean): Mp4AtomComponent;
export declare function r_mp4_atom_Copyright(p: number, o: boolean): Mp4AtomCopyright;
export declare function r_mp4_atom_Covr(p: number, o: boolean): Mp4AtomCovr;
export declare function r_mp4_atom_Cprt(p: number, o: boolean): Mp4AtomCprt;
export declare function r_mp4_atom_Cslg(p: number, o: boolean): Mp4AtomCslg;
export declare function r_mp4_atom_Ctts(p: number, o: boolean): Mp4AtomCtts;
export declare function r_mp4_atom_CttsEntry(p: number, o: boolean): Mp4AtomCttsEntry;
export declare function r_mp4_atom_Dadj(p: number, o: boolean): Mp4AtomDadj;
export declare function r_mp4_atom_Desc(p: number, o: boolean): Mp4AtomDesc;
export declare function r_mp4_atom_Dinf(p: number, o: boolean): Mp4AtomDinf;
export declare function r_mp4_atom_Disp(p: number, o: boolean): Mp4AtomDisp;
export declare function r_mp4_atom_Dref(p: number, o: boolean): Mp4AtomDref;
export declare function r_mp4_atom_DrefEntry(p: number, o: boolean): Mp4AtomDrefEntry;
export declare function r_mp4_atom_Edts(p: number, o: boolean): Mp4AtomEdts;
export declare function r_mp4_atom_Elst(p: number, o: boolean): Mp4AtomElst;
export declare function r_mp4_atom_ElstEntry(p: number, o: boolean): Mp4AtomElstEntry;
export declare function r_mp4_atom_Equi(p: number, o: boolean): Mp4AtomEqui;
export declare function r_mp4_atom_Esds(p: number, o: boolean): Mp4AtomEsds;
export declare function r_mp4_atom_ExplicitSpeakerPosition(p: number, o: boolean): Mp4AtomExplicitSpeakerPosition;
export declare function r_mp4_atom_ExtendedType(p: number, o: boolean): Mp4AtomExtendedType;
export declare function r_mp4_atom_Fiel(p: number, o: boolean): Mp4AtomFiel;
export declare function r_mp4_atom_FixedPoint_i16(p: number, o: boolean): Mp4AtomFixedPointI16;
export declare function r_mp4_atom_FixedPoint_i8(p: number, o: boolean): Mp4AtomFixedPointI8;
export declare function r_mp4_atom_FixedPoint_u16(p: number, o: boolean): Mp4AtomFixedPointU16;
export declare function r_mp4_atom_FixedPoint_u8(p: number, o: boolean): Mp4AtomFixedPointU8;
export declare function r_mp4_atom_FontEntry(p: number, o: boolean): Mp4AtomFontEntry;
export declare function r_mp4_atom_FourCC(p: number, o: boolean): Mp4AtomFourCc;
export declare function r_mp4_atom_FragmentInfo(p: number, o: boolean): Mp4AtomFragmentInfo;
export declare function r_mp4_atom_Ftab(p: number, o: boolean): Mp4AtomFtab;
export declare function r_mp4_atom_Hdlr(p: number, o: boolean): Mp4AtomHdlr;
export declare function r_mp4_atom_Hero(p: number, o: boolean): Mp4AtomHero;
export declare function r_mp4_atom_Hfov(p: number, o: boolean): Mp4AtomHfov;
export declare function r_mp4_atom_Hmhd(p: number, o: boolean): Mp4AtomHmhd;
export declare function r_mp4_atom_HvcCArray(p: number, o: boolean): Mp4AtomHvcCArray;
export declare function r_mp4_atom_Hvcc(p: number, o: boolean): Mp4AtomHvcc;
export declare function r_mp4_atom_Idat(p: number, o: boolean): Mp4AtomIdat;
export declare function r_mp4_atom_Iinf(p: number, o: boolean): Mp4AtomIinf;
export declare function r_mp4_atom_Iloc(p: number, o: boolean): Mp4AtomIloc;
export declare function r_mp4_atom_Ilst(p: number, o: boolean): Mp4AtomIlst;
export declare function r_mp4_atom_IlstData(p: number, o: boolean): Mp4AtomIlstData;
export declare function r_mp4_atom_IlstDataValue(p: number, o: boolean): Mp4AtomIlstDataValue;
export declare function r_mp4_atom_Imir(p: number, o: boolean): Mp4AtomImir;
export declare function r_mp4_atom_Ipma(p: number, o: boolean): Mp4AtomIpma;
export declare function r_mp4_atom_Iref(p: number, o: boolean): Mp4AtomIref;
export declare function r_mp4_atom_Irot(p: number, o: boolean): Mp4AtomIrot;
export declare function r_mp4_atom_Iscl(p: number, o: boolean): Mp4AtomIscl;
export declare function r_mp4_atom_Ispe(p: number, o: boolean): Mp4AtomIspe;
export declare function r_mp4_atom_ItemInfoEntry(p: number, o: boolean): Mp4AtomItemInfoEntry;
export declare function r_mp4_atom_ItemLocation(p: number, o: boolean): Mp4AtomItemLocation;
export declare function r_mp4_atom_ItemLocationExtent(p: number, o: boolean): Mp4AtomItemLocationExtent;
export declare function r_mp4_atom_KeyEntry(p: number, o: boolean): Mp4AtomKeyEntry;
export declare function r_mp4_atom_Keys(p: number, o: boolean): Mp4AtomKeys;
export declare function r_mp4_atom_Kind(p: number, o: boolean): Mp4AtomKind;
export declare function r_mp4_atom_Lhvc(p: number, o: boolean): Mp4AtomLhvc;
export declare function r_mp4_atom_Matrix(p: number, o: boolean): Mp4AtomMatrix;
export declare function r_mp4_atom_Mdhd(p: number, o: boolean): Mp4AtomMdhd;
export declare function r_mp4_atom_Mehd(p: number, o: boolean): Mp4AtomMehd;
export declare function r_mp4_atom_MetaData(p: number, o: boolean): Mp4AtomMetaData;
export declare function r_mp4_atom_Mfhd(p: number, o: boolean): Mp4AtomMfhd;
export declare function r_mp4_atom_Mfra(p: number, o: boolean): Mp4AtomMfra;
export declare function r_mp4_atom_Mfro(p: number, o: boolean): Mp4AtomMfro;
export declare function r_mp4_atom_Mshp(p: number, o: boolean): Mp4AtomMshp;
export declare function r_mp4_atom_Must(p: number, o: boolean): Mp4AtomMust;
export declare function r_mp4_atom_Mvex(p: number, o: boolean): Mp4AtomMvex;
export declare function r_mp4_atom_Mvhd(p: number, o: boolean): Mp4AtomMvhd;
export declare function r_mp4_atom_Name(p: number, o: boolean): Mp4AtomName;
export declare function r_mp4_atom_Pasp(p: number, o: boolean): Mp4AtomPasp;
export declare function r_mp4_atom_PcmFormat(p: number, o: boolean): Mp4AtomPcmFormat;
export declare function r_mp4_atom_Pitm(p: number, o: boolean): Mp4AtomPitm;
export declare function r_mp4_atom_Pixi(p: number, o: boolean): Mp4AtomPixi;
export declare function r_mp4_atom_Pkin(p: number, o: boolean): Mp4AtomPkin;
export declare function r_mp4_atom_Prft(p: number, o: boolean): Mp4AtomPrft;
export declare function r_mp4_atom_Prhd(p: number, o: boolean): Mp4AtomPrhd;
export declare function r_mp4_atom_Prji(p: number, o: boolean): Mp4AtomPrji;
export declare function r_mp4_atom_ProjKind(p: number, o: boolean): Mp4AtomProjKind;
export declare function r_mp4_atom_PropertyAssociation(p: number, o: boolean): Mp4AtomPropertyAssociation;
export declare function r_mp4_atom_PropertyAssociations(p: number, o: boolean): Mp4AtomPropertyAssociations;
export declare function r_mp4_atom_Reference(p: number, o: boolean): Mp4AtomReference;
export declare function r_mp4_atom_RgbColor(p: number, o: boolean): Mp4AtomRgbColor;
export declare function r_mp4_atom_RgbaColor(p: number, o: boolean): Mp4AtomRgbaColor;
export declare function r_mp4_atom_Rref(p: number, o: boolean): Mp4AtomRref;
export declare function r_mp4_atom_Rtng(p: number, o: boolean): Mp4AtomRtng;
export declare function r_mp4_atom_Saio(p: number, o: boolean): Mp4AtomSaio;
export declare function r_mp4_atom_Saiz(p: number, o: boolean): Mp4AtomSaiz;
export declare function r_mp4_atom_Sbgp(p: number, o: boolean): Mp4AtomSbgp;
export declare function r_mp4_atom_SbgpEntry(p: number, o: boolean): Mp4AtomSbgpEntry;
export declare function r_mp4_atom_SegmentReference(p: number, o: boolean): Mp4AtomSegmentReference;
export declare function r_mp4_atom_Senc(p: number, o: boolean): Mp4AtomSenc;
export declare function r_mp4_atom_Sgpd(p: number, o: boolean): Mp4AtomSgpd;
export declare function r_mp4_atom_SgpdEntry(p: number, o: boolean): Mp4AtomSgpdEntry;
export declare function r_mp4_atom_Sidx(p: number, o: boolean): Mp4AtomSidx;
export declare function r_mp4_atom_Skip(p: number, o: boolean): Mp4AtomSkip;
export declare function r_mp4_atom_Smhd(p: number, o: boolean): Mp4AtomSmhd;
export declare function r_mp4_atom_SpeakerPosition(p: number, o: boolean): Mp4AtomSpeakerPosition;
export declare function r_mp4_atom_St3d(p: number, o: boolean): Mp4AtomSt3d;
export declare function r_mp4_atom_Stco(p: number, o: boolean): Mp4AtomStco;
export declare function r_mp4_atom_Stri(p: number, o: boolean): Mp4AtomStri;
export declare function r_mp4_atom_Stsc(p: number, o: boolean): Mp4AtomStsc;
export declare function r_mp4_atom_StscEntry(p: number, o: boolean): Mp4AtomStscEntry;
export declare function r_mp4_atom_Stss(p: number, o: boolean): Mp4AtomStss;
export declare function r_mp4_atom_Stsz(p: number, o: boolean): Mp4AtomStsz;
export declare function r_mp4_atom_StszSamples(p: number, o: boolean): Mp4AtomStszSamples;
export declare function r_mp4_atom_Stts(p: number, o: boolean): Mp4AtomStts;
export declare function r_mp4_atom_SttsEntry(p: number, o: boolean): Mp4AtomSttsEntry;
export declare function r_mp4_atom_Subs(p: number, o: boolean): Mp4AtomSubs;
export declare function r_mp4_atom_SubsEntry(p: number, o: boolean): Mp4AtomSubsEntry;
export declare function r_mp4_atom_SubsSubsample(p: number, o: boolean): Mp4AtomSubsSubsample;
export declare function r_mp4_atom_SubsSubsampleSize(p: number, o: boolean): Mp4AtomSubsSubsampleSize;
export declare function r_mp4_atom_Svhd(p: number, o: boolean): Mp4AtomSvhd;
export declare function r_mp4_atom_Taic(p: number, o: boolean): Mp4AtomTaic;
export declare function r_mp4_atom_Tfdt(p: number, o: boolean): Mp4AtomTfdt;
export declare function r_mp4_atom_Tfhd(p: number, o: boolean): Mp4AtomTfhd;
export declare function r_mp4_atom_Tfra(p: number, o: boolean): Mp4AtomTfra;
export declare function r_mp4_atom_Tkhd(p: number, o: boolean): Mp4AtomTkhd;
export declare function r_mp4_atom_Tool(p: number, o: boolean): Mp4AtomTool;
export declare function r_mp4_atom_TrackReferenceTypeBox(p: number, o: boolean): Mp4AtomTrackReferenceTypeBox;
export declare function r_mp4_atom_Tref(p: number, o: boolean): Mp4AtomTref;
export declare function r_mp4_atom_Trex(p: number, o: boolean): Mp4AtomTrex;
export declare function r_mp4_atom_Trun(p: number, o: boolean): Mp4AtomTrun;
export declare function r_mp4_atom_TrunEntry(p: number, o: boolean): Mp4AtomTrunEntry;
export declare function r_mp4_atom_TrunSamples(p: number, o: boolean): Mp4AtomTrunSamples;
export declare function r_mp4_atom_Url(p: number, o: boolean): Mp4AtomUrl;
export declare function r_mp4_atom_Urn(p: number, o: boolean): Mp4AtomUrn;
export declare function r_mp4_atom_Vmhd(p: number, o: boolean): Mp4AtomVmhd;
export declare function r_mp4_atom_VpcC(p: number, o: boolean): Mp4AtomVpcC;
export declare function r_mp4_atom_Year(p: number, o: boolean): Mp4AtomYear;
export declare function r_mp4_atom_Zeroed(p: number, o: boolean): Mp4AtomZeroed;
export declare function r_mp4_atom_esds_DecoderConfig(p: number, o: boolean): Mp4AtomEsdsDecoderConfig;
export declare function r_mp4_atom_esds_DecoderSpecific(p: number, o: boolean): Mp4AtomEsdsDecoderSpecific;
export declare function r_mp4_atom_esds_EsDescriptor(p: number, o: boolean): Mp4AtomEsdsEsDescriptor;
export declare function r_mp4_atom_esds_SLConfig(p: number, o: boolean): Mp4AtomEsdsSlConfig;
export declare function r_mp4_atom_u24(p: number, o: boolean): Mp4AtomU24;
export declare function r_mux_codec_aac_Config(p: number, o: boolean): MuxCodecAacConfig;
export declare function r_mux_codec_flac_Config(p: number, o: boolean): MuxCodecFlacConfig;
export declare function r_mux_codec_h264_Sps(p: number, o: boolean): MuxCodecH264Sps;
export declare function r_mux_codec_mp3_Config(p: number, o: boolean): MuxCodecMp3Config;
export declare function r_mux_codec_opus_Error(p: number, o: boolean): MuxCodecOpusError;
export declare function r_mux_codec_opus_Mapping(p: number, o: boolean): MuxCodecOpusMapping;
export declare function r_mux_container_Frame(p: number, o: boolean): MuxContainerFrame;
export declare function r_mux_container_Read(p: number, o: boolean): MuxContainerRead;
export declare function r_mux_container_fmp4_Fragment(p: number, o: boolean): MuxContainerFmp4Fragment;
export declare function r_mux_container_fmp4_fragment_Config(p: number, o: boolean): MuxContainerFmp4FragmentConfig;
export declare function r_mux_container_mp4_Finish(p: number, o: boolean): MuxContainerMp4Finish;
export declare function r_mux_container_ts_Descriptor(p: number, o: boolean): MuxContainerTsDescriptor;
export declare function r_mux_rate_Target(p: number, o: boolean): MuxRateTarget;
export declare function r_mux_select_Selected(p: number, o: boolean): MuxSelectSelected;
export declare function r_mux_timeline_Transition(p: number, o: boolean): MuxTimelineTransition;
export declare function r_option_Datagram(p: number, o: boolean): Datagram | undefined;
export declare function r_option_Hop(p: number, o: boolean): Hop | undefined;
export declare function r_option_SessionError(p: number, o: boolean): SessionError | undefined;
export declare function r_option_StreamError(p: number, o: boolean): StreamError | undefined;
export declare function r_option_Timescale(p: number, o: boolean): Timescale | undefined;
export declare function r_option_Timestamp(p: number, o: boolean): Timestamp | undefined;
export declare function r_option_auth_RequestKind(p: number, o: boolean): AuthRequestKind | undefined;
export declare function r_option_bandwidth_Rate(p: number, o: boolean): BandwidthRate | undefined;
export declare function r_option_bool(p: number, o: boolean): boolean | undefined;
export declare function r_option_clock_Instant(p: number, o: boolean): ClockInstant | undefined;
export declare function r_option_clock_Wall(p: number, o: boolean): ClockWall | undefined;
export declare function r_option_duration(p: number, o: boolean): rt.Duration | undefined;
export declare function r_option_f64(p: number, o: boolean): number | undefined;
export declare function r_option_hang_catalog_AudioLayout(p: number, o: boolean): HangCatalogAudioLayout | undefined;
export declare function r_option_hang_catalog_Clock(p: number, o: boolean): HangCatalogClock | undefined;
export declare function r_option_hang_catalog_ContentLight(p: number, o: boolean): HangCatalogContentLight | undefined;
export declare function r_option_hang_catalog_Display(p: number, o: boolean): HangCatalogDisplay | undefined;
export declare function r_option_hang_catalog_Mastering(p: number, o: boolean): HangCatalogMastering | undefined;
export declare function r_option_hang_catalog_Range(p: number, o: boolean): HangCatalogRange | undefined;
export declare function r_option_hang_catalog_Stereo(p: number, o: boolean): HangCatalogStereo | undefined;
export declare function r_option_hang_catalog_VideoCodec(p: number, o: boolean): HangCatalogVideoCodec | undefined;
export declare function r_option_i32(p: number, o: boolean): number | undefined;
export declare function r_option_i64(p: number, o: boolean): rt.i64 | undefined;
export declare function r_option_i8(p: number, o: boolean): number | undefined;
export declare function r_option_mp4_atom_Ainf(p: number, o: boolean): Mp4AtomAinf | undefined;
export declare function r_option_mp4_atom_Audio(p: number, o: boolean): Mp4AtomAudio | undefined;
export declare function r_option_mp4_atom_AudioChannelPosition(p: number, o: boolean): Mp4AtomAudioChannelPosition | undefined;
export declare function r_option_mp4_atom_AuxInfo(p: number, o: boolean): Mp4AtomAuxInfo | undefined;
export declare function r_option_mp4_atom_AvccExt(p: number, o: boolean): Mp4AtomAvccExt | undefined;
export declare function r_option_mp4_atom_Blin(p: number, o: boolean): Mp4AtomBlin | undefined;
export declare function r_option_mp4_atom_Btrt(p: number, o: boolean): Mp4AtomBtrt | undefined;
export declare function r_option_mp4_atom_Ccst(p: number, o: boolean): Mp4AtomCcst | undefined;
export declare function r_option_mp4_atom_ChannelStructure(p: number, o: boolean): Mp4AtomChannelStructure | undefined;
export declare function r_option_mp4_atom_Chpl(p: number, o: boolean): Mp4AtomChpl | undefined;
export declare function r_option_mp4_atom_Co64(p: number, o: boolean): Mp4AtomCo64 | undefined;
export declare function r_option_mp4_atom_Colr(p: number, o: boolean): Mp4AtomColr | undefined;
export declare function r_option_mp4_atom_Copyright(p: number, o: boolean): Mp4AtomCopyright | undefined;
export declare function r_option_mp4_atom_Covr(p: number, o: boolean): Mp4AtomCovr | undefined;
export declare function r_option_mp4_atom_Cslg(p: number, o: boolean): Mp4AtomCslg | undefined;
export declare function r_option_mp4_atom_Ctts(p: number, o: boolean): Mp4AtomCtts | undefined;
export declare function r_option_mp4_atom_Dadj(p: number, o: boolean): Mp4AtomDadj | undefined;
export declare function r_option_mp4_atom_Desc(p: number, o: boolean): Mp4AtomDesc | undefined;
export declare function r_option_mp4_atom_Dinf(p: number, o: boolean): Mp4AtomDinf | undefined;
export declare function r_option_mp4_atom_Disp(p: number, o: boolean): Mp4AtomDisp | undefined;
export declare function r_option_mp4_atom_Edts(p: number, o: boolean): Mp4AtomEdts | undefined;
export declare function r_option_mp4_atom_Elst(p: number, o: boolean): Mp4AtomElst | undefined;
export declare function r_option_mp4_atom_Fiel(p: number, o: boolean): Mp4AtomFiel | undefined;
export declare function r_option_mp4_atom_FourCC(p: number, o: boolean): Mp4AtomFourCc | undefined;
export declare function r_option_mp4_atom_Ftab(p: number, o: boolean): Mp4AtomFtab | undefined;
export declare function r_option_mp4_atom_Hero(p: number, o: boolean): Mp4AtomHero | undefined;
export declare function r_option_mp4_atom_Hfov(p: number, o: boolean): Mp4AtomHfov | undefined;
export declare function r_option_mp4_atom_Hmhd(p: number, o: boolean): Mp4AtomHmhd | undefined;
export declare function r_option_mp4_atom_Idat(p: number, o: boolean): Mp4AtomIdat | undefined;
export declare function r_option_mp4_atom_Iinf(p: number, o: boolean): Mp4AtomIinf | undefined;
export declare function r_option_mp4_atom_Iloc(p: number, o: boolean): Mp4AtomIloc | undefined;
export declare function r_option_mp4_atom_Ilst(p: number, o: boolean): Mp4AtomIlst | undefined;
export declare function r_option_mp4_atom_Iref(p: number, o: boolean): Mp4AtomIref | undefined;
export declare function r_option_mp4_atom_Keys(p: number, o: boolean): Mp4AtomKeys | undefined;
export declare function r_option_mp4_atom_Lhvc(p: number, o: boolean): Mp4AtomLhvc | undefined;
export declare function r_option_mp4_atom_Mehd(p: number, o: boolean): Mp4AtomMehd | undefined;
export declare function r_option_mp4_atom_MetaData(p: number, o: boolean): Mp4AtomMetaData | undefined;
export declare function r_option_mp4_atom_Must(p: number, o: boolean): Mp4AtomMust | undefined;
export declare function r_option_mp4_atom_Mvex(p: number, o: boolean): Mp4AtomMvex | undefined;
export declare function r_option_mp4_atom_Name(p: number, o: boolean): Mp4AtomName | undefined;
export declare function r_option_mp4_atom_Pasp(p: number, o: boolean): Mp4AtomPasp | undefined;
export declare function r_option_mp4_atom_PcmFormat(p: number, o: boolean): Mp4AtomPcmFormat | undefined;
export declare function r_option_mp4_atom_Pitm(p: number, o: boolean): Mp4AtomPitm | undefined;
export declare function r_option_mp4_atom_Pkin(p: number, o: boolean): Mp4AtomPkin | undefined;
export declare function r_option_mp4_atom_Prhd(p: number, o: boolean): Mp4AtomPrhd | undefined;
export declare function r_option_mp4_atom_Prji(p: number, o: boolean): Mp4AtomPrji | undefined;
export declare function r_option_mp4_atom_ProjKind(p: number, o: boolean): Mp4AtomProjKind | undefined;
export declare function r_option_mp4_atom_Senc(p: number, o: boolean): Mp4AtomSenc | undefined;
export declare function r_option_mp4_atom_Smhd(p: number, o: boolean): Mp4AtomSmhd | undefined;
export declare function r_option_mp4_atom_St3d(p: number, o: boolean): Mp4AtomSt3d | undefined;
export declare function r_option_mp4_atom_Stco(p: number, o: boolean): Mp4AtomStco | undefined;
export declare function r_option_mp4_atom_Stri(p: number, o: boolean): Mp4AtomStri | undefined;
export declare function r_option_mp4_atom_Stss(p: number, o: boolean): Mp4AtomStss | undefined;
export declare function r_option_mp4_atom_Svhd(p: number, o: boolean): Mp4AtomSvhd | undefined;
export declare function r_option_mp4_atom_Taic(p: number, o: boolean): Mp4AtomTaic | undefined;
export declare function r_option_mp4_atom_Tfdt(p: number, o: boolean): Mp4AtomTfdt | undefined;
export declare function r_option_mp4_atom_Tool(p: number, o: boolean): Mp4AtomTool | undefined;
export declare function r_option_mp4_atom_Tref(p: number, o: boolean): Mp4AtomTref | undefined;
export declare function r_option_mp4_atom_TrunEntry(p: number, o: boolean): Mp4AtomTrunEntry | undefined;
export declare function r_option_mp4_atom_VexuProjKind(p: number, o: boolean): Mp4AtomVexuProjKind | undefined;
export declare function r_option_mp4_atom_Vmhd(p: number, o: boolean): Mp4AtomVmhd | undefined;
export declare function r_option_mp4_atom_Year(p: number, o: boolean): Mp4AtomYear | undefined;
export declare function r_option_mp4_atom_esds_DecoderSpecific(p: number, o: boolean): Mp4AtomEsdsDecoderSpecific | undefined;
export declare function r_option_mux_catalog_CatalogFormat(p: number, o: boolean): MuxCatalogCatalogFormat | undefined;
export declare function r_option_mux_codec_opus_Mapping(p: number, o: boolean): MuxCodecOpusMapping | undefined;
export declare function r_option_mux_container_Frame(p: number, o: boolean): MuxContainerFrame | undefined;
export declare function r_option_mux_container_Read(p: number, o: boolean): MuxContainerRead | undefined;
export declare function r_option_mux_container_fmp4_Fragment(p: number, o: boolean): MuxContainerFmp4Fragment | undefined;
export declare function r_option_mux_container_mp4_Finish(p: number, o: boolean): MuxContainerMp4Finish | undefined;
export declare function r_option_mux_rate_Target(p: number, o: boolean): MuxRateTarget | undefined;
export declare function r_option_origin_HopLimit(p: number, o: boolean): OriginHopLimit | undefined;
export declare function r_option_setup_Role(p: number, o: boolean): SetupRole | undefined;
export declare function r_option_setup_Token(p: number, o: boolean): SetupToken | undefined;
export declare function r_option_track_Anchor(p: number, o: boolean): TrackAnchor | undefined;
export declare function r_option_track_Position(p: number, o: boolean): TrackPosition | undefined;
export declare function r_option_tuple_usize_usize(p: number, o: boolean): [number, number] | undefined;
export declare function r_option_u16(p: number, o: boolean): number | undefined;
export declare function r_option_u32(p: number, o: boolean): number | undefined;
export declare function r_option_u64(p: number, o: boolean): rt.u64 | undefined;
export declare function r_option_u8(p: number, o: boolean): number | undefined;
export declare function r_option_usize(p: number, o: boolean): number | undefined;
export declare function r_origin_Cost(p: number, o: boolean): OriginCost;
export declare function r_origin_HopLimit(p: number, o: boolean): OriginHopLimit;
export declare function r_origin_Preference(p: number, o: boolean): OriginPreference;
export declare function r_origin_Source(p: number, o: boolean): OriginSource;
export declare function r_session_Limits(p: number, o: boolean): SessionLimits;
export declare function r_setup_Extensions(p: number, o: boolean): SetupExtensions;
export declare function r_setup_Token(p: number, o: boolean): SetupToken;
export declare function r_stats_Content(p: number, o: boolean): StatsContent;
export declare function r_stats_Failures(p: number, o: boolean): StatsFailures;
export declare function r_stats_Presence(p: number, o: boolean): StatsPresence;
export declare function r_stats_Refusals(p: number, o: boolean): StatsRefusals;
export declare function r_stats_Totals(p: number, o: boolean): StatsTotals;
export declare function r_stats_Traffic(p: number, o: boolean): StatsTraffic;
export declare function r_track_Anchor(p: number, o: boolean): TrackAnchor;
export declare function r_track_Delay(p: number, o: boolean): TrackDelay;
export declare function r_track_Position(p: number, o: boolean): TrackPosition;
export declare function r_track_Range(p: number, o: boolean): TrackRange;
export declare function r_transport_ReceiveWindow(p: number, o: boolean): TransportReceiveWindow;
export declare function r_tuple_stats_Tier_stats_Message_stats_Totals(p: number, o: boolean): [StatsTier, StatsMessage, StatsTotals];
export declare function r_tuple_stats_Tier_stats_Presence(p: number, o: boolean): [StatsTier, StatsPresence];
export declare function r_tuple_stats_Tier_stats_Role_stats_Traffic(p: number, o: boolean): [StatsTier, StatsRole, StatsTraffic];
export declare function r_tuple_string_Json(p: number, o: boolean): [string, Json];
export declare function r_tuple_string_Json_2(p: number, o: boolean): [string, Json];
export declare function r_tuple_string_hang_catalog_AudioConfig(p: number, o: boolean): [string, HangCatalogAudioConfig];
export declare function r_tuple_string_hang_catalog_AudioConfig_2(p: number, o: boolean): [string, HangCatalogAudioConfig];
export declare function r_tuple_string_hang_catalog_BinaryConfig(p: number, o: boolean): [string, HangCatalogBinaryConfig];
export declare function r_tuple_string_hang_catalog_Contribution(p: number, o: boolean): [string, HangCatalogContribution];
export declare function r_tuple_string_hang_catalog_JsonConfig(p: number, o: boolean): [string, HangCatalogJsonConfig];
export declare function r_tuple_string_hang_catalog_TextConfig(p: number, o: boolean): [string, HangCatalogTextConfig];
export declare function r_tuple_string_hang_catalog_VideoConfig(p: number, o: boolean): [string, HangCatalogVideoConfig];
export declare function r_tuple_string_hang_catalog_VideoConfig_2(p: number, o: boolean): [string, HangCatalogVideoConfig];
export declare function r_tuple_string_hang_echo_Rendition(p: number, o: boolean): [string, HangEchoRendition];
export declare function r_tuple_string_hang_stats_Rendition(p: number, o: boolean): [string, HangStatsRendition];
export declare function r_tuple_string_mux_container_ts_Track(p: number, o: boolean): [string, MuxContainerTsTrack];
export declare function r_tuple_u16_mux_container_ts_stats_Stream(p: number, o: boolean): [number, MuxContainerTsStatsStream];
export declare function r_tuple_u16_vec_tuple_u8_mux_container_ts_SiEntry(p: number, o: boolean): [number, ([number, MuxContainerTsSiEntry])[]];
export declare function r_tuple_u32_mp4_atom_IlstData(p: number, o: boolean): [number, Mp4AtomIlstData];
export declare function r_tuple_u8_mux_container_ts_SiEntry(p: number, o: boolean): [number, MuxContainerTsSiEntry];
export declare function r_tuple_usize_usize(p: number, o: boolean): [number, number];
export declare function r_vec_Hop(p: number, o: boolean): Hop[];
export declare function r_vec_Hop_2(p: number, o: boolean): Hop[];
export declare function r_vec_Pattern(p: number, o: boolean): Pattern[];
export declare function r_vec_Pattern_2(p: number, o: boolean): Pattern[];
export declare function r_vec_Segment(p: number, o: boolean): Segment[];
export declare function r_vec_Version(p: number, o: boolean): Version[];
export declare function r_vec_broadcast_Producer(p: number, o: boolean): BroadcastProducer[];
export declare function r_vec_bytes(p: number, o: boolean): Uint8Array[];
export declare function r_vec_bytes_2(p: number, o: boolean): Uint8Array[];
export declare function r_vec_frame_Frame(p: number, o: boolean): FrameFrame[];
export declare function r_vec_hang_health_Evidence(p: number, o: boolean): HangHealthEvidence[];
export declare function r_vec_mp4_atom_Any(p: number, o: boolean): Mp4AtomAny[];
export declare function r_vec_mp4_atom_ChplEntry(p: number, o: boolean): Mp4AtomChplEntry[];
export declare function r_vec_mp4_atom_Codec(p: number, o: boolean): Mp4AtomCodec[];
export declare function r_vec_mp4_atom_Component(p: number, o: boolean): Mp4AtomComponent[];
export declare function r_vec_mp4_atom_Cprt(p: number, o: boolean): Mp4AtomCprt[];
export declare function r_vec_mp4_atom_CttsEntry(p: number, o: boolean): Mp4AtomCttsEntry[];
export declare function r_vec_mp4_atom_DrefEntry(p: number, o: boolean): Mp4AtomDrefEntry[];
export declare function r_vec_mp4_atom_ElstEntry(p: number, o: boolean): Mp4AtomElstEntry[];
export declare function r_vec_mp4_atom_FontEntry(p: number, o: boolean): Mp4AtomFontEntry[];
export declare function r_vec_mp4_atom_FourCC(p: number, o: boolean): Mp4AtomFourCc[];
export declare function r_vec_mp4_atom_FragmentInfo(p: number, o: boolean): Mp4AtomFragmentInfo[];
export declare function r_vec_mp4_atom_HvcCArray(p: number, o: boolean): Mp4AtomHvcCArray[];
export declare function r_vec_mp4_atom_Ipma(p: number, o: boolean): Mp4AtomIpma[];
export declare function r_vec_mp4_atom_ItemInfoEntry(p: number, o: boolean): Mp4AtomItemInfoEntry[];
export declare function r_vec_mp4_atom_ItemLocation(p: number, o: boolean): Mp4AtomItemLocation[];
export declare function r_vec_mp4_atom_ItemLocationExtent(p: number, o: boolean): Mp4AtomItemLocationExtent[];
export declare function r_vec_mp4_atom_KeyEntry(p: number, o: boolean): Mp4AtomKeyEntry[];
export declare function r_vec_mp4_atom_Kind(p: number, o: boolean): Mp4AtomKind[];
export declare function r_vec_mp4_atom_PropertyAssociation(p: number, o: boolean): Mp4AtomPropertyAssociation[];
export declare function r_vec_mp4_atom_PropertyAssociations(p: number, o: boolean): Mp4AtomPropertyAssociations[];
export declare function r_vec_mp4_atom_Reference(p: number, o: boolean): Mp4AtomReference[];
export declare function r_vec_mp4_atom_Rtng(p: number, o: boolean): Mp4AtomRtng[];
export declare function r_vec_mp4_atom_Saio(p: number, o: boolean): Mp4AtomSaio[];
export declare function r_vec_mp4_atom_Saiz(p: number, o: boolean): Mp4AtomSaiz[];
export declare function r_vec_mp4_atom_Sbgp(p: number, o: boolean): Mp4AtomSbgp[];
export declare function r_vec_mp4_atom_SbgpEntry(p: number, o: boolean): Mp4AtomSbgpEntry[];
export declare function r_vec_mp4_atom_SegmentReference(p: number, o: boolean): Mp4AtomSegmentReference[];
export declare function r_vec_mp4_atom_Sgpd(p: number, o: boolean): Mp4AtomSgpd[];
export declare function r_vec_mp4_atom_SgpdEntry(p: number, o: boolean): Mp4AtomSgpdEntry[];
export declare function r_vec_mp4_atom_SpeakerPosition(p: number, o: boolean): Mp4AtomSpeakerPosition[];
export declare function r_vec_mp4_atom_StscEntry(p: number, o: boolean): Mp4AtomStscEntry[];
export declare function r_vec_mp4_atom_SttsEntry(p: number, o: boolean): Mp4AtomSttsEntry[];
export declare function r_vec_mp4_atom_Subs(p: number, o: boolean): Mp4AtomSubs[];
export declare function r_vec_mp4_atom_SubsEntry(p: number, o: boolean): Mp4AtomSubsEntry[];
export declare function r_vec_mp4_atom_SubsSubsample(p: number, o: boolean): Mp4AtomSubsSubsample[];
export declare function r_vec_mp4_atom_Tfra(p: number, o: boolean): Mp4AtomTfra[];
export declare function r_vec_mp4_atom_TrackReferenceTypeBox(p: number, o: boolean): Mp4AtomTrackReferenceTypeBox[];
export declare function r_vec_mp4_atom_Traf(p: number, o: boolean): Mp4AtomTraf[];
export declare function r_vec_mp4_atom_Trak(p: number, o: boolean): Mp4AtomTrak[];
export declare function r_vec_mp4_atom_Trex(p: number, o: boolean): Mp4AtomTrex[];
export declare function r_vec_mp4_atom_Trun(p: number, o: boolean): Mp4AtomTrun[];
export declare function r_vec_mp4_atom_TrunEntry(p: number, o: boolean): Mp4AtomTrunEntry[];
export declare function r_vec_mux_container_Frame(p: number, o: boolean): MuxContainerFrame[];
export declare function r_vec_mux_container_fmp4_Fragment(p: number, o: boolean): MuxContainerFmp4Fragment[];
export declare function r_vec_mux_container_ts_Descriptor(p: number, o: boolean): MuxContainerTsDescriptor[];
export declare function r_vec_stats_Failure(p: number, o: boolean): StatsFailure[];
export declare function r_vec_stats_RefusalKind(p: number, o: boolean): StatsRefusalKind[];
export declare function r_vec_stats_SessionEntry(p: number, o: boolean): StatsSessionEntry[];
export declare function r_vec_stats_TrafficEntry(p: number, o: boolean): StatsTrafficEntry[];
export declare function r_vec_string(p: number, o: boolean): string[];
export declare function r_vec_string_2(p: number, o: boolean): string[];
export declare function r_vec_string_3(p: number, o: boolean): string[];
export declare function r_vec_track_Range(p: number, o: boolean): TrackRange[];
export declare function r_vec_tuple_stats_Tier_stats_Message_stats_Totals(p: number, o: boolean): ([StatsTier, StatsMessage, StatsTotals])[];
export declare function r_vec_tuple_stats_Tier_stats_Presence(p: number, o: boolean): ([StatsTier, StatsPresence])[];
export declare function r_vec_tuple_stats_Tier_stats_Role_stats_Traffic(p: number, o: boolean): ([StatsTier, StatsRole, StatsTraffic])[];
export declare function r_vec_tuple_string_Json(p: number, o: boolean): ([string, Json])[];
export declare function r_vec_tuple_string_Json_2(p: number, o: boolean): ([string, Json])[];
export declare function r_vec_tuple_string_hang_catalog_AudioConfig(p: number, o: boolean): ([string, HangCatalogAudioConfig])[];
export declare function r_vec_tuple_string_hang_catalog_AudioConfig_2(p: number, o: boolean): ([string, HangCatalogAudioConfig])[];
export declare function r_vec_tuple_string_hang_catalog_BinaryConfig(p: number, o: boolean): ([string, HangCatalogBinaryConfig])[];
export declare function r_vec_tuple_string_hang_catalog_Contribution(p: number, o: boolean): ([string, HangCatalogContribution])[];
export declare function r_vec_tuple_string_hang_catalog_JsonConfig(p: number, o: boolean): ([string, HangCatalogJsonConfig])[];
export declare function r_vec_tuple_string_hang_catalog_TextConfig(p: number, o: boolean): ([string, HangCatalogTextConfig])[];
export declare function r_vec_tuple_string_hang_catalog_VideoConfig(p: number, o: boolean): ([string, HangCatalogVideoConfig])[];
export declare function r_vec_tuple_string_hang_catalog_VideoConfig_2(p: number, o: boolean): ([string, HangCatalogVideoConfig])[];
export declare function r_vec_tuple_string_hang_echo_Rendition(p: number, o: boolean): ([string, HangEchoRendition])[];
export declare function r_vec_tuple_string_hang_stats_Rendition(p: number, o: boolean): ([string, HangStatsRendition])[];
export declare function r_vec_tuple_string_mux_container_ts_Track(p: number, o: boolean): ([string, MuxContainerTsTrack])[];
export declare function r_vec_tuple_u16_mux_container_ts_stats_Stream(p: number, o: boolean): ([number, MuxContainerTsStatsStream])[];
export declare function r_vec_tuple_u16_vec_tuple_u8_mux_container_ts_SiEntry(p: number, o: boolean): ([number, ([number, MuxContainerTsSiEntry])[]])[];
export declare function r_vec_tuple_u32_mp4_atom_IlstData(p: number, o: boolean): ([number, Mp4AtomIlstData])[];
export declare function r_vec_tuple_u8_mux_container_ts_SiEntry(p: number, o: boolean): ([number, MuxContainerTsSiEntry])[];
export declare function r_vec_u32(p: number, o: boolean): Uint32Array;
export declare function r_vec_u64(p: number, o: boolean): BigUint64Array;
export declare function t0(f: () => string, r$: number): void;
export declare function t1(f: (a0: HangCatalog) => void, p0: number): void;
export declare function t2(f: (a0: HangCatalogExtra) => void, p0: number): void;
export declare function t3(f: (a0: HangCatalogExt) => void, p0: number): void;
export declare function t4(f: (a0: ClockInstant, a1: Waiter) => rt.Result<ClockInstant | undefined, string>, p0: bigint, p1: number, p2: number, p3: number): number;
export declare function t_clock_Instant(p: number, v: ClockInstant): void;
export declare function t_option_clock_Instant(p: number, v: ClockInstant | undefined): void;
export declare function w_Datagram(p: number, v: Datagram): void;
export declare function w_Hop(p: number, v: Hop): void;
export declare function w_InvalidPattern(p: number, v: InvalidPattern): void;
export declare function w_Segment(p: number, v: Segment): void;
export declare function w_SessionError(p: number, v: SessionError): void;
export declare function w_StreamError(p: number, v: StreamError): void;
export declare function w_TimeOverflow(p: number, v: TimeOverflow): void;
export declare function w_Timescale(p: number, v: Timescale): void;
export declare function w_Timestamp(p: number, v: Timestamp): void;
export declare function w_bandwidth_Rate(p: number, v: BandwidthRate): void;
export declare function w_bound_u64(p: number, v: {
    readonly tag: "Included";
    readonly _0: rt.u64;
} | {
    readonly tag: "Excluded";
    readonly _0: rt.u64;
} | {
    readonly tag: "Unbounded";
}): void;
export declare function w_bounds_u64(p: number, v: {
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
}): void;
export declare function w_clock_Elapsed(p: number, v: ClockElapsed): void;
export declare function w_clock_Instant(p: number, v: ClockInstant): void;
export declare function w_clock_Wall(p: number, v: ClockWall): void;
export declare function w_frame_Info(p: number, v: FrameInfo): void;
export declare function w_group_Info(p: number, v: GroupInfo): void;
export declare function w_hang_catalog_AAC(p: number, v: HangCatalogAac): void;
export declare function w_hang_catalog_AV1(p: number, v: HangCatalogAv1): void;
export declare function w_hang_catalog_AudioCodec(p: number, v: HangCatalogAudioCodec): void;
export declare function w_hang_catalog_AudioLayout(p: number, v: HangCatalogAudioLayout): void;
export declare function w_hang_catalog_Chromaticity(p: number, v: HangCatalogChromaticity): void;
export declare function w_hang_catalog_Clock(p: number, v: HangCatalogClock): void;
export declare function w_hang_catalog_ContentLight(p: number, v: HangCatalogContentLight): void;
export declare function w_hang_catalog_Display(p: number, v: HangCatalogDisplay): void;
export declare function w_hang_catalog_H264(p: number, v: HangCatalogH264): void;
export declare function w_hang_catalog_H265(p: number, v: HangCatalogH265): void;
export declare function w_hang_catalog_Mastering(p: number, v: HangCatalogMastering): void;
export declare function w_hang_catalog_Mode(p: number, v: HangCatalogMode): void;
export declare function w_hang_catalog_Priorities(p: number, v: HangCatalogPriorities): void;
export declare function w_hang_catalog_Stereo(p: number, v: HangCatalogStereo): void;
export declare function w_hang_catalog_TextFormat(p: number, v: HangCatalogTextFormat): void;
export declare function w_hang_catalog_TextRole(p: number, v: HangCatalogTextRole): void;
export declare function w_hang_catalog_VP9(p: number, v: HangCatalogVp9): void;
export declare function w_hang_catalog_VideoCodec(p: number, v: HangCatalogVideoCodec): void;
export declare function w_hang_container_Frame(p: number, v: HangContainerFrame): void;
export declare function w_hang_health_Limits_duration(p: number, v: HangHealthLimitsDuration): void;
export declare function w_hang_health_Limits_f64(p: number, v: HangHealthLimitsF64): void;
export declare function w_mp4_atom_Ac3(p: number, v: Mp4AtomAc3): void;
export declare function w_mp4_atom_Ac3SpecificBox(p: number, v: Mp4AtomAc3SpecificBox): void;
export declare function w_mp4_atom_Ainf(p: number, v: Mp4AtomAinf): void;
export declare function w_mp4_atom_AmrSampleEntry(p: number, v: Mp4AtomAmrSampleEntry): void;
export declare function w_mp4_atom_AnySampleGroupEntry(p: number, v: Mp4AtomAnySampleGroupEntry): void;
export declare function w_mp4_atom_Audio(p: number, v: Mp4AtomAudio): void;
export declare function w_mp4_atom_AuxInfo(p: number, v: Mp4AtomAuxInfo): void;
export declare function w_mp4_atom_Auxc(p: number, v: Mp4AtomAuxc): void;
export declare function w_mp4_atom_Av1c(p: number, v: Mp4AtomAv1c): void;
export declare function w_mp4_atom_Avcc(p: number, v: Mp4AtomAvcc): void;
export declare function w_mp4_atom_AvccExt(p: number, v: Mp4AtomAvccExt): void;
export declare function w_mp4_atom_Blin(p: number, v: Mp4AtomBlin): void;
export declare function w_mp4_atom_Btrt(p: number, v: Mp4AtomBtrt): void;
export declare function w_mp4_atom_C2pa(p: number, v: Mp4AtomC2pa): void;
export declare function w_mp4_atom_Camm(p: number, v: Mp4AtomCamm): void;
export declare function w_mp4_atom_Cbmp(p: number, v: Mp4AtomCbmp): void;
export declare function w_mp4_atom_Ccst(p: number, v: Mp4AtomCcst): void;
export declare function w_mp4_atom_ChannelStructure(p: number, v: Mp4AtomChannelStructure): void;
export declare function w_mp4_atom_Chnl(p: number, v: Mp4AtomChnl): void;
export declare function w_mp4_atom_Chpl(p: number, v: Mp4AtomChpl): void;
export declare function w_mp4_atom_ChplEntry(p: number, v: Mp4AtomChplEntry): void;
export declare function w_mp4_atom_Clap(p: number, v: Mp4AtomClap): void;
export declare function w_mp4_atom_Cmpd(p: number, v: Mp4AtomCmpd): void;
export declare function w_mp4_atom_Co64(p: number, v: Mp4AtomCo64): void;
export declare function w_mp4_atom_Colr(p: number, v: Mp4AtomColr): void;
export declare function w_mp4_atom_Component(p: number, v: Mp4AtomComponent): void;
export declare function w_mp4_atom_Copyright(p: number, v: Mp4AtomCopyright): void;
export declare function w_mp4_atom_Covr(p: number, v: Mp4AtomCovr): void;
export declare function w_mp4_atom_Cprt(p: number, v: Mp4AtomCprt): void;
export declare function w_mp4_atom_Cslg(p: number, v: Mp4AtomCslg): void;
export declare function w_mp4_atom_Ctts(p: number, v: Mp4AtomCtts): void;
export declare function w_mp4_atom_CttsEntry(p: number, v: Mp4AtomCttsEntry): void;
export declare function w_mp4_atom_Dadj(p: number, v: Mp4AtomDadj): void;
export declare function w_mp4_atom_Damr(p: number, v: Mp4AtomDamr): void;
export declare function w_mp4_atom_DataType(p: number, v: Mp4AtomDataType): void;
export declare function w_mp4_atom_Desc(p: number, v: Mp4AtomDesc): void;
export declare function w_mp4_atom_Dfla(p: number, v: Mp4AtomDfla): void;
export declare function w_mp4_atom_Dinf(p: number, v: Mp4AtomDinf): void;
export declare function w_mp4_atom_Disp(p: number, v: Mp4AtomDisp): void;
export declare function w_mp4_atom_Dops(p: number, v: Mp4AtomDops): void;
export declare function w_mp4_atom_Dref(p: number, v: Mp4AtomDref): void;
export declare function w_mp4_atom_DrefEntry(p: number, v: Mp4AtomDrefEntry): void;
export declare function w_mp4_atom_Dtyp(p: number, v: Mp4AtomDtyp): void;
export declare function w_mp4_atom_Eac3(p: number, v: Mp4AtomEac3): void;
export declare function w_mp4_atom_Ec3IndependentSubstream(p: number, v: Mp4AtomEc3IndependentSubstream): void;
export declare function w_mp4_atom_Ec3SpecificBox(p: number, v: Mp4AtomEc3SpecificBox): void;
export declare function w_mp4_atom_Edts(p: number, v: Mp4AtomEdts): void;
export declare function w_mp4_atom_Elst(p: number, v: Mp4AtomElst): void;
export declare function w_mp4_atom_ElstEntry(p: number, v: Mp4AtomElstEntry): void;
export declare function w_mp4_atom_Emsg(p: number, v: Mp4AtomEmsg): void;
export declare function w_mp4_atom_EmsgTimestamp(p: number, v: Mp4AtomEmsgTimestamp): void;
export declare function w_mp4_atom_Equi(p: number, v: Mp4AtomEqui): void;
export declare function w_mp4_atom_Esds(p: number, v: Mp4AtomEsds): void;
export declare function w_mp4_atom_Evte(p: number, v: Mp4AtomEvte): void;
export declare function w_mp4_atom_ExplicitSpeakerPosition(p: number, v: Mp4AtomExplicitSpeakerPosition): void;
export declare function w_mp4_atom_ExtendedType(p: number, v: Mp4AtomExtendedType): void;
export declare function w_mp4_atom_Fiel(p: number, v: Mp4AtomFiel): void;
export declare function w_mp4_atom_FixedPoint_i16(p: number, v: Mp4AtomFixedPointI16): void;
export declare function w_mp4_atom_FixedPoint_i8(p: number, v: Mp4AtomFixedPointI8): void;
export declare function w_mp4_atom_FixedPoint_u16(p: number, v: Mp4AtomFixedPointU16): void;
export declare function w_mp4_atom_FixedPoint_u8(p: number, v: Mp4AtomFixedPointU8): void;
export declare function w_mp4_atom_Fl32(p: number, v: Mp4AtomFl32): void;
export declare function w_mp4_atom_Fl64(p: number, v: Mp4AtomFl64): void;
export declare function w_mp4_atom_Flac(p: number, v: Mp4AtomFlac): void;
export declare function w_mp4_atom_FlacMetadataBlock(p: number, v: Mp4AtomFlacMetadataBlock): void;
export declare function w_mp4_atom_FontEntry(p: number, v: Mp4AtomFontEntry): void;
export declare function w_mp4_atom_FourCC(p: number, v: Mp4AtomFourCc): void;
export declare function w_mp4_atom_Fpcm(p: number, v: Mp4AtomFpcm): void;
export declare function w_mp4_atom_FragmentInfo(p: number, v: Mp4AtomFragmentInfo): void;
export declare function w_mp4_atom_Free(p: number, v: Mp4AtomFree): void;
export declare function w_mp4_atom_Ftab(p: number, v: Mp4AtomFtab): void;
export declare function w_mp4_atom_Ftyp(p: number, v: Mp4AtomFtyp): void;
export declare function w_mp4_atom_Hdlr(p: number, v: Mp4AtomHdlr): void;
export declare function w_mp4_atom_Hero(p: number, v: Mp4AtomHero): void;
export declare function w_mp4_atom_Hfov(p: number, v: Mp4AtomHfov): void;
export declare function w_mp4_atom_Hmhd(p: number, v: Mp4AtomHmhd): void;
export declare function w_mp4_atom_HvcCArray(p: number, v: Mp4AtomHvcCArray): void;
export declare function w_mp4_atom_Hvcc(p: number, v: Mp4AtomHvcc): void;
export declare function w_mp4_atom_Idat(p: number, v: Mp4AtomIdat): void;
export declare function w_mp4_atom_Iinf(p: number, v: Mp4AtomIinf): void;
export declare function w_mp4_atom_Iloc(p: number, v: Mp4AtomIloc): void;
export declare function w_mp4_atom_Ilst(p: number, v: Mp4AtomIlst): void;
export declare function w_mp4_atom_IlstData(p: number, v: Mp4AtomIlstData): void;
export declare function w_mp4_atom_IlstDataValue(p: number, v: Mp4AtomIlstDataValue): void;
export declare function w_mp4_atom_Imir(p: number, v: Mp4AtomImir): void;
export declare function w_mp4_atom_In24(p: number, v: Mp4AtomIn24): void;
export declare function w_mp4_atom_In32(p: number, v: Mp4AtomIn32): void;
export declare function w_mp4_atom_Ipcm(p: number, v: Mp4AtomIpcm): void;
export declare function w_mp4_atom_Ipma(p: number, v: Mp4AtomIpma): void;
export declare function w_mp4_atom_Iref(p: number, v: Mp4AtomIref): void;
export declare function w_mp4_atom_Irot(p: number, v: Mp4AtomIrot): void;
export declare function w_mp4_atom_Iscl(p: number, v: Mp4AtomIscl): void;
export declare function w_mp4_atom_Ispe(p: number, v: Mp4AtomIspe): void;
export declare function w_mp4_atom_ItemInfoEntry(p: number, v: Mp4AtomItemInfoEntry): void;
export declare function w_mp4_atom_ItemLocation(p: number, v: Mp4AtomItemLocation): void;
export declare function w_mp4_atom_ItemLocationExtent(p: number, v: Mp4AtomItemLocationExtent): void;
export declare function w_mp4_atom_KeyEntry(p: number, v: Mp4AtomKeyEntry): void;
export declare function w_mp4_atom_Keyd(p: number, v: Mp4AtomKeyd): void;
export declare function w_mp4_atom_Keys(p: number, v: Mp4AtomKeys): void;
export declare function w_mp4_atom_Kind(p: number, v: Mp4AtomKind): void;
export declare function w_mp4_atom_Lhvc(p: number, v: Mp4AtomLhvc): void;
export declare function w_mp4_atom_Loca(p: number, v: Mp4AtomLoca): void;
export declare function w_mp4_atom_Lpcm(p: number, v: Mp4AtomLpcm): void;
export declare function w_mp4_atom_Matrix(p: number, v: Mp4AtomMatrix): void;
export declare function w_mp4_atom_Mdat(p: number, v: Mp4AtomMdat): void;
export declare function w_mp4_atom_Mdhd(p: number, v: Mp4AtomMdhd): void;
export declare function w_mp4_atom_Mebx(p: number, v: Mp4AtomMebx): void;
export declare function w_mp4_atom_MebxKey(p: number, v: Mp4AtomMebxKey): void;
export declare function w_mp4_atom_Mehd(p: number, v: Mp4AtomMehd): void;
export declare function w_mp4_atom_MetaData(p: number, v: Mp4AtomMetaData): void;
export declare function w_mp4_atom_Mett(p: number, v: Mp4AtomMett): void;
export declare function w_mp4_atom_Metx(p: number, v: Mp4AtomMetx): void;
export declare function w_mp4_atom_Mfhd(p: number, v: Mp4AtomMfhd): void;
export declare function w_mp4_atom_Mfra(p: number, v: Mp4AtomMfra): void;
export declare function w_mp4_atom_Mfro(p: number, v: Mp4AtomMfro): void;
export declare function w_mp4_atom_Mp3(p: number, v: Mp4AtomMp3): void;
export declare function w_mp4_atom_Mp4a(p: number, v: Mp4AtomMp4a): void;
export declare function w_mp4_atom_Mshp(p: number, v: Mp4AtomMshp): void;
export declare function w_mp4_atom_Must(p: number, v: Mp4AtomMust): void;
export declare function w_mp4_atom_Mvex(p: number, v: Mp4AtomMvex): void;
export declare function w_mp4_atom_Mvhd(p: number, v: Mp4AtomMvhd): void;
export declare function w_mp4_atom_Name(p: number, v: Mp4AtomName): void;
export declare function w_mp4_atom_Opus(p: number, v: Mp4AtomOpus): void;
export declare function w_mp4_atom_OpusChannelMapping(p: number, v: Mp4AtomOpusChannelMapping): void;
export declare function w_mp4_atom_Pasp(p: number, v: Mp4AtomPasp): void;
export declare function w_mp4_atom_PcmC(p: number, v: Mp4AtomPcmC): void;
export declare function w_mp4_atom_PcmFormat(p: number, v: Mp4AtomPcmFormat): void;
export declare function w_mp4_atom_Pitm(p: number, v: Mp4AtomPitm): void;
export declare function w_mp4_atom_Pixi(p: number, v: Mp4AtomPixi): void;
export declare function w_mp4_atom_Pkin(p: number, v: Mp4AtomPkin): void;
export declare function w_mp4_atom_PlainText(p: number, v: Mp4AtomPlainText): void;
export declare function w_mp4_atom_Prft(p: number, v: Mp4AtomPrft): void;
export declare function w_mp4_atom_Prhd(p: number, v: Mp4AtomPrhd): void;
export declare function w_mp4_atom_Prji(p: number, v: Mp4AtomPrji): void;
export declare function w_mp4_atom_ProjKind(p: number, v: Mp4AtomProjKind): void;
export declare function w_mp4_atom_PropertyAssociation(p: number, v: Mp4AtomPropertyAssociation): void;
export declare function w_mp4_atom_PropertyAssociations(p: number, v: Mp4AtomPropertyAssociations): void;
export declare function w_mp4_atom_Reference(p: number, v: Mp4AtomReference): void;
export declare function w_mp4_atom_RgbColor(p: number, v: Mp4AtomRgbColor): void;
export declare function w_mp4_atom_RgbaColor(p: number, v: Mp4AtomRgbaColor): void;
export declare function w_mp4_atom_Rref(p: number, v: Mp4AtomRref): void;
export declare function w_mp4_atom_Rtng(p: number, v: Mp4AtomRtng): void;
export declare function w_mp4_atom_S16l(p: number, v: Mp4AtomS16l): void;
export declare function w_mp4_atom_Saio(p: number, v: Mp4AtomSaio): void;
export declare function w_mp4_atom_Saiz(p: number, v: Mp4AtomSaiz): void;
export declare function w_mp4_atom_Samr(p: number, v: Mp4AtomSamr): void;
export declare function w_mp4_atom_Sbgp(p: number, v: Mp4AtomSbgp): void;
export declare function w_mp4_atom_SbgpEntry(p: number, v: Mp4AtomSbgpEntry): void;
export declare function w_mp4_atom_SchemeIdEntry(p: number, v: Mp4AtomSchemeIdEntry): void;
export declare function w_mp4_atom_SegmentReference(p: number, v: Mp4AtomSegmentReference): void;
export declare function w_mp4_atom_Senc(p: number, v: Mp4AtomSenc): void;
export declare function w_mp4_atom_Setu(p: number, v: Mp4AtomSetu): void;
export declare function w_mp4_atom_Sgpd(p: number, v: Mp4AtomSgpd): void;
export declare function w_mp4_atom_SgpdEntry(p: number, v: Mp4AtomSgpdEntry): void;
export declare function w_mp4_atom_Sidx(p: number, v: Mp4AtomSidx): void;
export declare function w_mp4_atom_Silb(p: number, v: Mp4AtomSilb): void;
export declare function w_mp4_atom_Skip(p: number, v: Mp4AtomSkip): void;
export declare function w_mp4_atom_Smhd(p: number, v: Mp4AtomSmhd): void;
export declare function w_mp4_atom_Sowt(p: number, v: Mp4AtomSowt): void;
export declare function w_mp4_atom_SpeakerPosition(p: number, v: Mp4AtomSpeakerPosition): void;
export declare function w_mp4_atom_St3d(p: number, v: Mp4AtomSt3d): void;
export declare function w_mp4_atom_Stco(p: number, v: Mp4AtomStco): void;
export declare function w_mp4_atom_Stri(p: number, v: Mp4AtomStri): void;
export declare function w_mp4_atom_Stsc(p: number, v: Mp4AtomStsc): void;
export declare function w_mp4_atom_StscEntry(p: number, v: Mp4AtomStscEntry): void;
export declare function w_mp4_atom_Stss(p: number, v: Mp4AtomStss): void;
export declare function w_mp4_atom_Stsz(p: number, v: Mp4AtomStsz): void;
export declare function w_mp4_atom_StszSamples(p: number, v: Mp4AtomStszSamples): void;
export declare function w_mp4_atom_Stts(p: number, v: Mp4AtomStts): void;
export declare function w_mp4_atom_SttsEntry(p: number, v: Mp4AtomSttsEntry): void;
export declare function w_mp4_atom_Styp(p: number, v: Mp4AtomStyp): void;
export declare function w_mp4_atom_Subs(p: number, v: Mp4AtomSubs): void;
export declare function w_mp4_atom_SubsEntry(p: number, v: Mp4AtomSubsEntry): void;
export declare function w_mp4_atom_SubsSubsample(p: number, v: Mp4AtomSubsSubsample): void;
export declare function w_mp4_atom_SubsSubsampleSize(p: number, v: Mp4AtomSubsSubsampleSize): void;
export declare function w_mp4_atom_Svhd(p: number, v: Mp4AtomSvhd): void;
export declare function w_mp4_atom_Taic(p: number, v: Mp4AtomTaic): void;
export declare function w_mp4_atom_Tfdt(p: number, v: Mp4AtomTfdt): void;
export declare function w_mp4_atom_Tfhd(p: number, v: Mp4AtomTfhd): void;
export declare function w_mp4_atom_Tfra(p: number, v: Mp4AtomTfra): void;
export declare function w_mp4_atom_Tkhd(p: number, v: Mp4AtomTkhd): void;
export declare function w_mp4_atom_Tool(p: number, v: Mp4AtomTool): void;
export declare function w_mp4_atom_TrackReferenceTypeBox(p: number, v: Mp4AtomTrackReferenceTypeBox): void;
export declare function w_mp4_atom_Tref(p: number, v: Mp4AtomTref): void;
export declare function w_mp4_atom_Trex(p: number, v: Mp4AtomTrex): void;
export declare function w_mp4_atom_Trun(p: number, v: Mp4AtomTrun): void;
export declare function w_mp4_atom_TrunEntry(p: number, v: Mp4AtomTrunEntry): void;
export declare function w_mp4_atom_TrunSamples(p: number, v: Mp4AtomTrunSamples): void;
export declare function w_mp4_atom_Twos(p: number, v: Mp4AtomTwos): void;
export declare function w_mp4_atom_TxtC(p: number, v: Mp4AtomTxtC): void;
export declare function w_mp4_atom_UncC(p: number, v: Mp4AtomUncC): void;
export declare function w_mp4_atom_UncompressedComponent(p: number, v: Mp4AtomUncompressedComponent): void;
export declare function w_mp4_atom_Uri(p: number, v: Mp4AtomUri): void;
export declare function w_mp4_atom_UriI(p: number, v: Mp4AtomUriI): void;
export declare function w_mp4_atom_Urim(p: number, v: Mp4AtomUrim): void;
export declare function w_mp4_atom_Url(p: number, v: Mp4AtomUrl): void;
export declare function w_mp4_atom_Urn(p: number, v: Mp4AtomUrn): void;
export declare function w_mp4_atom_Uuid(p: number, v: Mp4AtomUuid): void;
export declare function w_mp4_atom_Vlab(p: number, v: Mp4AtomVlab): void;
export declare function w_mp4_atom_Vmhd(p: number, v: Mp4AtomVmhd): void;
export declare function w_mp4_atom_VpcC(p: number, v: Mp4AtomVpcC): void;
export declare function w_mp4_atom_VttC(p: number, v: Mp4AtomVttC): void;
export declare function w_mp4_atom_Wvtt(p: number, v: Mp4AtomWvtt): void;
export declare function w_mp4_atom_Year(p: number, v: Mp4AtomYear): void;
export declare function w_mp4_atom_Zeroed(p: number, v: Mp4AtomZeroed): void;
export declare function w_mp4_atom_esds_DecoderConfig(p: number, v: Mp4AtomEsdsDecoderConfig): void;
export declare function w_mp4_atom_esds_DecoderSpecific(p: number, v: Mp4AtomEsdsDecoderSpecific): void;
export declare function w_mp4_atom_esds_EsDescriptor(p: number, v: Mp4AtomEsdsEsDescriptor): void;
export declare function w_mp4_atom_esds_SLConfig(p: number, v: Mp4AtomEsdsSlConfig): void;
export declare function w_mp4_atom_u24(p: number, v: Mp4AtomU24): void;
export declare function w_mux_catalog_msf_Error(p: number, v: MuxCatalogMsfError): void;
export declare function w_mux_codec_aac_Config(p: number, v: MuxCodecAacConfig): void;
export declare function w_mux_codec_flac_Config(p: number, v: MuxCodecFlacConfig): void;
export declare function w_mux_codec_h264_Sps(p: number, v: MuxCodecH264Sps): void;
export declare function w_mux_codec_opus_Error(p: number, v: MuxCodecOpusError): void;
export declare function w_mux_codec_opus_Mapping(p: number, v: MuxCodecOpusMapping): void;
export declare function w_mux_container_Frame(p: number, v: MuxContainerFrame): void;
export declare function w_mux_container_Read(p: number, v: MuxContainerRead): void;
export declare function w_mux_container_TimestampRewind(p: number, v: MuxContainerTimestampRewind): void;
export declare function w_mux_container_fmp4_Chunk(p: number, v: MuxContainerFmp4Chunk): void;
export declare function w_mux_container_fmp4_Fragment(p: number, v: MuxContainerFmp4Fragment): void;
export declare function w_mux_container_fmp4_fragment_Config(p: number, v: MuxContainerFmp4FragmentConfig): void;
export declare function w_mux_container_legacy_Wire(p: number, v: MuxContainerLegacyWire): void;
export declare function w_mux_container_loc_Wire(p: number, v: MuxContainerLocWire): void;
export declare function w_mux_container_mp4_Finish(p: number, v: MuxContainerMp4Finish): void;
export declare function w_mux_container_ts_Descriptor(p: number, v: MuxContainerTsDescriptor): void;
export declare function w_mux_container_ts_Step(p: number, v: MuxContainerTsStep): void;
export declare function w_mux_import_Format(p: number, v: MuxImportFormat): void;
export declare function w_mux_rate_Target(p: number, v: MuxRateTarget): void;
export declare function w_mux_select_Selected(p: number, v: MuxSelectSelected): void;
export declare function w_mux_timeline_Audio(p: number, v: MuxTimelineAudio): void;
export declare function w_mux_timeline_Transition(p: number, v: MuxTimelineTransition): void;
export declare function w_option_Timescale(p: number, v: Timescale | undefined): void;
export declare function w_option_Timestamp(p: number, v: Timestamp | undefined): void;
export declare function w_option_bandwidth_Rate(p: number, v: BandwidthRate | undefined): void;
export declare function w_option_bool(p: number, v: boolean | undefined): void;
export declare function w_option_clock_Instant(p: number, v: ClockInstant | undefined): void;
export declare function w_option_duration(p: number, v: rt.Duration | undefined): void;
export declare function w_option_f64(p: number, v: number | undefined): void;
export declare function w_option_hang_catalog_AudioLayout(p: number, v: HangCatalogAudioLayout | undefined): void;
export declare function w_option_hang_catalog_Clock(p: number, v: HangCatalogClock | undefined): void;
export declare function w_option_hang_catalog_ContentLight(p: number, v: HangCatalogContentLight | undefined): void;
export declare function w_option_hang_catalog_Display(p: number, v: HangCatalogDisplay | undefined): void;
export declare function w_option_hang_catalog_Mastering(p: number, v: HangCatalogMastering | undefined): void;
export declare function w_option_hang_catalog_Range(p: number, v: HangCatalogRange | undefined): void;
export declare function w_option_hang_catalog_Stereo(p: number, v: HangCatalogStereo | undefined): void;
export declare function w_option_hang_catalog_VideoCodec(p: number, v: HangCatalogVideoCodec | undefined): void;
export declare function w_option_i32(p: number, v: number | undefined): void;
export declare function w_option_i64(p: number, v: rt.i64 | undefined): void;
export declare function w_option_i8(p: number, v: number | undefined): void;
export declare function w_option_mp4_atom_Ainf(p: number, v: Mp4AtomAinf | undefined): void;
export declare function w_option_mp4_atom_AuxInfo(p: number, v: Mp4AtomAuxInfo | undefined): void;
export declare function w_option_mp4_atom_AvccExt(p: number, v: Mp4AtomAvccExt | undefined): void;
export declare function w_option_mp4_atom_Blin(p: number, v: Mp4AtomBlin | undefined): void;
export declare function w_option_mp4_atom_Btrt(p: number, v: Mp4AtomBtrt | undefined): void;
export declare function w_option_mp4_atom_Ccst(p: number, v: Mp4AtomCcst | undefined): void;
export declare function w_option_mp4_atom_ChannelStructure(p: number, v: Mp4AtomChannelStructure | undefined): void;
export declare function w_option_mp4_atom_Chnl(p: number, v: Mp4AtomChnl | undefined): void;
export declare function w_option_mp4_atom_Chpl(p: number, v: Mp4AtomChpl | undefined): void;
export declare function w_option_mp4_atom_Co64(p: number, v: Mp4AtomCo64 | undefined): void;
export declare function w_option_mp4_atom_Colr(p: number, v: Mp4AtomColr | undefined): void;
export declare function w_option_mp4_atom_Copyright(p: number, v: Mp4AtomCopyright | undefined): void;
export declare function w_option_mp4_atom_Covr(p: number, v: Mp4AtomCovr | undefined): void;
export declare function w_option_mp4_atom_Cslg(p: number, v: Mp4AtomCslg | undefined): void;
export declare function w_option_mp4_atom_Ctts(p: number, v: Mp4AtomCtts | undefined): void;
export declare function w_option_mp4_atom_Dadj(p: number, v: Mp4AtomDadj | undefined): void;
export declare function w_option_mp4_atom_Desc(p: number, v: Mp4AtomDesc | undefined): void;
export declare function w_option_mp4_atom_Disp(p: number, v: Mp4AtomDisp | undefined): void;
export declare function w_option_mp4_atom_Dtyp(p: number, v: Mp4AtomDtyp | undefined): void;
export declare function w_option_mp4_atom_Edts(p: number, v: Mp4AtomEdts | undefined): void;
export declare function w_option_mp4_atom_Elst(p: number, v: Mp4AtomElst | undefined): void;
export declare function w_option_mp4_atom_Fiel(p: number, v: Mp4AtomFiel | undefined): void;
export declare function w_option_mp4_atom_FourCC(p: number, v: Mp4AtomFourCc | undefined): void;
export declare function w_option_mp4_atom_Ftab(p: number, v: Mp4AtomFtab | undefined): void;
export declare function w_option_mp4_atom_Hero(p: number, v: Mp4AtomHero | undefined): void;
export declare function w_option_mp4_atom_Hfov(p: number, v: Mp4AtomHfov | undefined): void;
export declare function w_option_mp4_atom_Hmhd(p: number, v: Mp4AtomHmhd | undefined): void;
export declare function w_option_mp4_atom_Lhvc(p: number, v: Mp4AtomLhvc | undefined): void;
export declare function w_option_mp4_atom_Loca(p: number, v: Mp4AtomLoca | undefined): void;
export declare function w_option_mp4_atom_Mehd(p: number, v: Mp4AtomMehd | undefined): void;
export declare function w_option_mp4_atom_Must(p: number, v: Mp4AtomMust | undefined): void;
export declare function w_option_mp4_atom_Mvex(p: number, v: Mp4AtomMvex | undefined): void;
export declare function w_option_mp4_atom_Name(p: number, v: Mp4AtomName | undefined): void;
export declare function w_option_mp4_atom_OpusChannelMapping(p: number, v: Mp4AtomOpusChannelMapping | undefined): void;
export declare function w_option_mp4_atom_Pasp(p: number, v: Mp4AtomPasp | undefined): void;
export declare function w_option_mp4_atom_PcmC(p: number, v: Mp4AtomPcmC | undefined): void;
export declare function w_option_mp4_atom_Pkin(p: number, v: Mp4AtomPkin | undefined): void;
export declare function w_option_mp4_atom_Prhd(p: number, v: Mp4AtomPrhd | undefined): void;
export declare function w_option_mp4_atom_Prji(p: number, v: Mp4AtomPrji | undefined): void;
export declare function w_option_mp4_atom_ProjKind(p: number, v: Mp4AtomProjKind | undefined): void;
export declare function w_option_mp4_atom_Senc(p: number, v: Mp4AtomSenc | undefined): void;
export declare function w_option_mp4_atom_Setu(p: number, v: Mp4AtomSetu | undefined): void;
export declare function w_option_mp4_atom_Silb(p: number, v: Mp4AtomSilb | undefined): void;
export declare function w_option_mp4_atom_Smhd(p: number, v: Mp4AtomSmhd | undefined): void;
export declare function w_option_mp4_atom_St3d(p: number, v: Mp4AtomSt3d | undefined): void;
export declare function w_option_mp4_atom_Stco(p: number, v: Mp4AtomStco | undefined): void;
export declare function w_option_mp4_atom_Stri(p: number, v: Mp4AtomStri | undefined): void;
export declare function w_option_mp4_atom_Stss(p: number, v: Mp4AtomStss | undefined): void;
export declare function w_option_mp4_atom_Svhd(p: number, v: Mp4AtomSvhd | undefined): void;
export declare function w_option_mp4_atom_Taic(p: number, v: Mp4AtomTaic | undefined): void;
export declare function w_option_mp4_atom_Tfdt(p: number, v: Mp4AtomTfdt | undefined): void;
export declare function w_option_mp4_atom_Tool(p: number, v: Mp4AtomTool | undefined): void;
export declare function w_option_mp4_atom_Tref(p: number, v: Mp4AtomTref | undefined): void;
export declare function w_option_mp4_atom_TxtC(p: number, v: Mp4AtomTxtC | undefined): void;
export declare function w_option_mp4_atom_UriI(p: number, v: Mp4AtomUriI | undefined): void;
export declare function w_option_mp4_atom_VexuProjKind(p: number, v: Mp4AtomVexuProjKind | undefined): void;
export declare function w_option_mp4_atom_Vlab(p: number, v: Mp4AtomVlab | undefined): void;
export declare function w_option_mp4_atom_Vmhd(p: number, v: Mp4AtomVmhd | undefined): void;
export declare function w_option_mp4_atom_Year(p: number, v: Mp4AtomYear | undefined): void;
export declare function w_option_mp4_atom_esds_DecoderSpecific(p: number, v: Mp4AtomEsdsDecoderSpecific | undefined): void;
export declare function w_option_mux_codec_opus_Mapping(p: number, v: MuxCodecOpusMapping | undefined): void;
export declare function w_option_track_Anchor(p: number, v: TrackAnchor | undefined): void;
export declare function w_option_u16(p: number, v: number | undefined): void;
export declare function w_option_u32(p: number, v: number | undefined): void;
export declare function w_option_u64(p: number, v: rt.u64 | undefined): void;
export declare function w_option_u8(p: number, v: number | undefined): void;
export declare function w_option_usize(p: number, v: number | undefined): void;
export declare function w_origin_Cost(p: number, v: OriginCost): void;
export declare function w_origin_HopLimit(p: number, v: OriginHopLimit): void;
export declare function w_origin_Preference(p: number, v: OriginPreference): void;
export declare function w_origin_Source(p: number, v: OriginSource): void;
export declare function w_range_u64(p: number, v: {
    start: rt.u64;
    end: rt.u64;
}): void;
export declare function w_session_Limits(p: number, v: SessionLimits): void;
export declare function w_setup_Extensions(p: number, v: SetupExtensions): void;
export declare function w_setup_Token(p: number, v: SetupToken): void;
export declare function w_stats_Content(p: number, v: StatsContent): void;
export declare function w_stats_Failures(p: number, v: StatsFailures): void;
export declare function w_stats_Presence(p: number, v: StatsPresence): void;
export declare function w_stats_Refusals(p: number, v: StatsRefusals): void;
export declare function w_stats_Totals(p: number, v: StatsTotals): void;
export declare function w_stats_Traffic(p: number, v: StatsTraffic): void;
export declare function w_track_Anchor(p: number, v: TrackAnchor): void;
export declare function w_track_Delay(p: number, v: TrackDelay): void;
export declare function w_track_Position(p: number, v: TrackPosition): void;
export declare function w_track_Range(p: number, v: TrackRange): void;
export declare function w_transport_ReceiveWindow(p: number, v: TransportReceiveWindow): void;
export declare function w_transport_memory_Error(p: number, v: TransportMemoryError): void;
export declare function w_tuple_string_Json(p: number, v: [string, Json]): void;
export declare function w_tuple_string_hang_catalog_AudioConfig(p: number, v: [string, HangCatalogAudioConfig]): void;
export declare function w_tuple_string_hang_catalog_BinaryConfig(p: number, v: [string, HangCatalogBinaryConfig]): void;
export declare function w_tuple_string_hang_catalog_Contribution(p: number, v: [string, HangCatalogContribution]): void;
export declare function w_tuple_string_hang_catalog_JsonConfig(p: number, v: [string, HangCatalogJsonConfig]): void;
export declare function w_tuple_string_hang_catalog_TextConfig(p: number, v: [string, HangCatalogTextConfig]): void;
export declare function w_tuple_string_hang_catalog_VideoConfig(p: number, v: [string, HangCatalogVideoConfig]): void;
export declare function w_tuple_string_hang_echo_Rendition(p: number, v: [string, HangEchoRendition]): void;
export declare function w_tuple_string_hang_stats_Rendition(p: number, v: [string, HangStatsRendition]): void;
export declare function w_tuple_string_mux_container_ts_Track(p: number, v: [string, MuxContainerTsTrack]): void;
export declare function w_tuple_u16_mux_container_ts_stats_Stream(p: number, v: [number, MuxContainerTsStatsStream]): void;
export declare function w_tuple_u16_vec_tuple_u8_mux_container_ts_SiEntry(p: number, v: [number, ([number, MuxContainerTsSiEntry])[]]): void;
export declare function w_tuple_u32_mp4_atom_IlstData(p: number, v: [number, Mp4AtomIlstData]): void;
export declare function w_tuple_u8_mux_container_ts_SiEntry(p: number, v: [number, MuxContainerTsSiEntry]): void;
export declare function w_vec_Segment(p: number, v: Segment[] | undefined): void;
export declare function w_vec_Timestamp(p: number, v: Timestamp[] | undefined): void;
export declare function w_vec_broadcast_Producer(p: number, v: BroadcastProducer[] | undefined): void;
export declare function w_vec_bytes(p: number, v: Uint8Array[] | undefined): void;
export declare function w_vec_bytes_2(p: number, v: Uint8Array[] | undefined): void;
export declare function w_vec_bytes_3(p: number, v: Uint8Array[] | undefined): void;
export declare function w_vec_hang_health_Evidence(p: number, v: HangHealthEvidence[] | undefined): void;
export declare function w_vec_mp4_atom_Any(p: number, v: Mp4AtomAny[] | undefined): void;
export declare function w_vec_mp4_atom_ChplEntry(p: number, v: Mp4AtomChplEntry[] | undefined): void;
export declare function w_vec_mp4_atom_Codec(p: number, v: Mp4AtomCodec[] | undefined): void;
export declare function w_vec_mp4_atom_Component(p: number, v: Mp4AtomComponent[] | undefined): void;
export declare function w_vec_mp4_atom_Cprt(p: number, v: Mp4AtomCprt[] | undefined): void;
export declare function w_vec_mp4_atom_CttsEntry(p: number, v: Mp4AtomCttsEntry[] | undefined): void;
export declare function w_vec_mp4_atom_DrefEntry(p: number, v: Mp4AtomDrefEntry[] | undefined): void;
export declare function w_vec_mp4_atom_Ec3IndependentSubstream(p: number, v: Mp4AtomEc3IndependentSubstream[] | undefined): void;
export declare function w_vec_mp4_atom_ElstEntry(p: number, v: Mp4AtomElstEntry[] | undefined): void;
export declare function w_vec_mp4_atom_FlacMetadataBlock(p: number, v: Mp4AtomFlacMetadataBlock[] | undefined): void;
export declare function w_vec_mp4_atom_FontEntry(p: number, v: Mp4AtomFontEntry[] | undefined): void;
export declare function w_vec_mp4_atom_FourCC(p: number, v: Mp4AtomFourCc[] | undefined): void;
export declare function w_vec_mp4_atom_FragmentInfo(p: number, v: Mp4AtomFragmentInfo[] | undefined): void;
export declare function w_vec_mp4_atom_HvcCArray(p: number, v: Mp4AtomHvcCArray[] | undefined): void;
export declare function w_vec_mp4_atom_Ipma(p: number, v: Mp4AtomIpma[] | undefined): void;
export declare function w_vec_mp4_atom_ItemInfoEntry(p: number, v: Mp4AtomItemInfoEntry[] | undefined): void;
export declare function w_vec_mp4_atom_ItemLocation(p: number, v: Mp4AtomItemLocation[] | undefined): void;
export declare function w_vec_mp4_atom_ItemLocationExtent(p: number, v: Mp4AtomItemLocationExtent[] | undefined): void;
export declare function w_vec_mp4_atom_KeyEntry(p: number, v: Mp4AtomKeyEntry[] | undefined): void;
export declare function w_vec_mp4_atom_Kind(p: number, v: Mp4AtomKind[] | undefined): void;
export declare function w_vec_mp4_atom_MebxKey(p: number, v: Mp4AtomMebxKey[] | undefined): void;
export declare function w_vec_mp4_atom_PropertyAssociation(p: number, v: Mp4AtomPropertyAssociation[] | undefined): void;
export declare function w_vec_mp4_atom_PropertyAssociations(p: number, v: Mp4AtomPropertyAssociations[] | undefined): void;
export declare function w_vec_mp4_atom_Reference(p: number, v: Mp4AtomReference[] | undefined): void;
export declare function w_vec_mp4_atom_Rtng(p: number, v: Mp4AtomRtng[] | undefined): void;
export declare function w_vec_mp4_atom_Saio(p: number, v: Mp4AtomSaio[] | undefined): void;
export declare function w_vec_mp4_atom_Saiz(p: number, v: Mp4AtomSaiz[] | undefined): void;
export declare function w_vec_mp4_atom_Sbgp(p: number, v: Mp4AtomSbgp[] | undefined): void;
export declare function w_vec_mp4_atom_SbgpEntry(p: number, v: Mp4AtomSbgpEntry[] | undefined): void;
export declare function w_vec_mp4_atom_SchemeIdEntry(p: number, v: Mp4AtomSchemeIdEntry[] | undefined): void;
export declare function w_vec_mp4_atom_SegmentReference(p: number, v: Mp4AtomSegmentReference[] | undefined): void;
export declare function w_vec_mp4_atom_Sgpd(p: number, v: Mp4AtomSgpd[] | undefined): void;
export declare function w_vec_mp4_atom_SgpdEntry(p: number, v: Mp4AtomSgpdEntry[] | undefined): void;
export declare function w_vec_mp4_atom_SpeakerPosition(p: number, v: Mp4AtomSpeakerPosition[] | undefined): void;
export declare function w_vec_mp4_atom_StscEntry(p: number, v: Mp4AtomStscEntry[] | undefined): void;
export declare function w_vec_mp4_atom_SttsEntry(p: number, v: Mp4AtomSttsEntry[] | undefined): void;
export declare function w_vec_mp4_atom_Subs(p: number, v: Mp4AtomSubs[] | undefined): void;
export declare function w_vec_mp4_atom_SubsEntry(p: number, v: Mp4AtomSubsEntry[] | undefined): void;
export declare function w_vec_mp4_atom_SubsSubsample(p: number, v: Mp4AtomSubsSubsample[] | undefined): void;
export declare function w_vec_mp4_atom_Tfra(p: number, v: Mp4AtomTfra[] | undefined): void;
export declare function w_vec_mp4_atom_TrackReferenceTypeBox(p: number, v: Mp4AtomTrackReferenceTypeBox[] | undefined): void;
export declare function w_vec_mp4_atom_Traf(p: number, v: Mp4AtomTraf[] | undefined): void;
export declare function w_vec_mp4_atom_Trak(p: number, v: Mp4AtomTrak[] | undefined): void;
export declare function w_vec_mp4_atom_Trex(p: number, v: Mp4AtomTrex[] | undefined): void;
export declare function w_vec_mp4_atom_Trun(p: number, v: Mp4AtomTrun[] | undefined): void;
export declare function w_vec_mp4_atom_TrunEntry(p: number, v: Mp4AtomTrunEntry[] | undefined): void;
export declare function w_vec_mp4_atom_UncompressedComponent(p: number, v: Mp4AtomUncompressedComponent[] | undefined): void;
export declare function w_vec_mux_container_Frame(p: number, v: MuxContainerFrame[] | undefined): void;
export declare function w_vec_mux_container_Frame_2(p: number, v: MuxContainerFrame[] | undefined): void;
export declare function w_vec_mux_container_ts_Descriptor(p: number, v: MuxContainerTsDescriptor[] | undefined): void;
export declare function w_vec_stats_SessionEntry(p: number, v: StatsSessionEntry[] | undefined): void;
export declare function w_vec_stats_TrafficEntry(p: number, v: StatsTrafficEntry[] | undefined): void;
export declare function w_vec_string(p: number, v: string[] | undefined): void;
export declare function w_vec_string_2(p: number, v: string[] | undefined): void;
export declare function w_vec_track_Range(p: number, v: TrackRange[] | undefined): void;
export declare function w_vec_tuple_string_Json(p: number, v: ([string, Json])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_AudioConfig(p: number, v: ([string, HangCatalogAudioConfig])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_BinaryConfig(p: number, v: ([string, HangCatalogBinaryConfig])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_Contribution(p: number, v: ([string, HangCatalogContribution])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_JsonConfig(p: number, v: ([string, HangCatalogJsonConfig])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_TextConfig(p: number, v: ([string, HangCatalogTextConfig])[] | undefined): void;
export declare function w_vec_tuple_string_hang_catalog_VideoConfig(p: number, v: ([string, HangCatalogVideoConfig])[] | undefined): void;
export declare function w_vec_tuple_string_hang_echo_Rendition(p: number, v: ([string, HangEchoRendition])[] | undefined): void;
export declare function w_vec_tuple_string_hang_stats_Rendition(p: number, v: ([string, HangStatsRendition])[] | undefined): void;
export declare function w_vec_tuple_string_mux_container_ts_Track(p: number, v: ([string, MuxContainerTsTrack])[] | undefined): void;
export declare function w_vec_tuple_u16_mux_container_ts_stats_Stream(p: number, v: ([number, MuxContainerTsStatsStream])[] | undefined): void;
export declare function w_vec_tuple_u16_vec_tuple_u8_mux_container_ts_SiEntry(p: number, v: ([number, ([number, MuxContainerTsSiEntry])[]])[] | undefined): void;
export declare function w_vec_tuple_u32_mp4_atom_IlstData(p: number, v: ([number, Mp4AtomIlstData])[] | undefined): void;
export declare function w_vec_tuple_u8_mux_container_ts_SiEntry(p: number, v: ([number, MuxContainerTsSiEntry])[] | undefined): void;
export declare function w_vec_u32(p: number, v: Uint32Array | undefined): void;
export declare function w_vec_u64(p: number, v: BigUint64Array | undefined): void;
export declare const imports: {
    moq_web_i_fn_0: (r$: number, data: number) => void;
    moq_web_i_fn_1: (data: number, p0: number) => void;
    moq_web_i_fn_2: (data: number, p0: number) => void;
    moq_web_i_fn_3: (data: number, p0: number) => void;
    moq_web_i_fn_4: (data: number, p0: bigint, p1: number, p2: number, p3: number) => number;
};

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
import type { Path } from "./Path.ts";
import type { Timescale } from "./Timescale.ts";
import type { TrackInfo } from "./TrackInfo.ts";
import type { TransportMemoryConfig } from "./TransportMemoryConfig.ts";
import type { TransportMemorySession } from "./TransportMemorySession.ts";
import type { Waiter } from "./Waiter.ts";
export declare function load(source: rt.Source): Promise<void>;
export declare const PATH_MAX_PARTS: number;
export declare const PATH_MAX_BYTES: number;
export declare const PATTERN_MAX_SEGMENTS: number;
export declare const PATTERN_MAX_INTERSECTIONS: number;
export declare const SETUP_TOKEN_OUT_OF_BAND: rt.u64;
export declare const TRANSPORT_MIN_RECEIVE_WINDOW: rt.u64;
export declare const TRANSPORT_SETUP_RECEIVE_WINDOW: rt.u64;
export declare const GROUP_MAX_CACHE_BYTES: rt.u64;
export declare const HANG_CATALOG_MOQ_EPOCH_UNIX_MILLIS: rt.u64;
export declare const HANG_CATALOG_MAX_SAFE_INTEGER: rt.u64;
export declare const MUX_CONTAINER_DEFAULT_MAX_BUFFER: number;
export declare const MUX_CONTAINER_FMP4_MAX_HELD_BYTES: number;
export declare const MUX_CONTAINER_FMP4_MAX_METADATA_BYTES: number;
export declare const MUX_CONTAINER_FMP4_MAX_FRAGMENT_SAMPLES: rt.u64;
export declare const MUX_CONTAINER_FMP4_MAX_TRACKS: number;
export declare const MP4_ATOM_ST3D_MONO: number;
export declare const MP4_ATOM_ST3D_TOP_BOTTOM: number;
export declare const MP4_ATOM_ST3D_LEFT_RIGHT: number;
export declare const MP4_ATOM_ST3D_STEREO_CUSTOM: number;
export declare const MP4_ATOM_ST3D_RIGHT_LEFT: number;
export declare const MP4_ATOM_HFOV_DENOMINATOR: number;
export declare const MP4_ATOM_ESDS_ES_DESCRIPTOR_TAG: number;
export declare const MP4_ATOM_ESDS_DECODER_CONFIG_TAG: number;
export declare const MP4_ATOM_ESDS_SL_CONFIG_TAG: number;
export declare const MP4_ATOM_U24_MAX: number;
export declare const MP4_ATOM_HERO_NONE: number;
export declare const MP4_ATOM_HERO_LEFT: number;
export declare const MP4_ATOM_HERO_RIGHT: number;
export declare const MP4_ATOM_ESDS_DECODER_SPECIFIC_TAG: number;
export declare const MP4_ATOM_DADJ_DENOMINATOR: number;
export declare function IMPLEMENTATION(): string;
export declare function ALPNS(): string[];
export declare function clockNow(): ClockInstant;
export declare function clockWall(): ClockWall;
export declare function clockOrigin(): [ClockInstant, ClockWall];
export declare function clockRng(): ClockRng;
export declare function clockSeed(): rt.u64;
export declare function clockDrive(poll: (a0: ClockInstant, a1: Waiter) => rt.Result<ClockInstant | undefined, string>): rt.Call<string>;
export declare function clockInterval(period: rt.Duration): ClockInterval;
export declare function clockIntervalAt(start: ClockInstant, period: rt.Duration): ClockInterval;
export declare function clockSleep(duration: rt.Duration): rt.Call<void>;
export declare function clockSleepUntil(at: ClockInstant): rt.Call<void>;
export declare function clockTimeout(duration: rt.Duration, future: Promise<unknown>): rt.Call<rt.Result<undefined, ClockElapsed>>;
export declare function clockTimeoutAt(deadline: ClockInstant, future: Promise<unknown>): rt.Call<rt.Result<undefined, ClockElapsed>>;
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
export declare function transportMemoryPair(config: TransportMemoryConfig): [TransportMemorySession, TransportMemorySession];
export declare function muxCodecAacConfig(init: Uint8Array | rt.Bytes): rt.Result<HangCatalogAudioConfig, MuxError>;
export declare function MUX_CODEC_ANNEXB_START_CODE(): rt.Bytes;
export declare function muxCodecAnnexbFromLengthPrefixed(payload: Uint8Array | rt.Bytes, lengthSize: number, prefix: Uint8Array | rt.Bytes | undefined): rt.Result<rt.Bytes, MuxCodecAnnexbError>;
export declare function muxCodecAnnexbBuildPrefix(nals: Uint8Array[]): rt.Bytes;
export declare function muxCodecAnnexbLengthPrefixedToAnnexb(data: Uint8Array | rt.Bytes, lengthSize: number, out: rt.Place<Uint8Array>): rt.Result<undefined, string>;
export declare function muxCodecAnnexbAfterStartCode(b: Uint8Array | rt.Bytes): rt.Result<number | undefined, MuxCodecAnnexbError>;
export declare function muxCodecAnnexbFindStartCode(b: Uint8Array | rt.Bytes): [number, number] | undefined;
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
export declare class Opaque<T extends string = string> {
    #private;
    readonly [brand]: T;
    private constructor();
}
export declare function opaqueMade(bytes: Uint8Array): Opaque;
export declare function opaquePut(v: Opaque, n: number, p: number): void;
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
type Exports = Record<string, any>;
export declare let w: Exports;
export declare function dv(): DataView;
export declare function mem(): Uint8Array<ArrayBuffer>;
export declare const s: {
    d: number;
};
export declare function abort(message: string): never;
export declare function enter(): number;
export declare function leave(mark: number): void;
export declare function alloc(size: number, align: number): number;
export declare function status(p: number, what: string): void;
export type Kind = {
    readonly name: string;
    readonly free: string;
    readonly drop?: string;
};
export declare class Handle<T extends string = string> {
    readonly [brand]: T;
    readonly p: number;
    readonly k: Kind;
    h: number;
    closed: boolean;
    readonly owned: boolean;
    readonly checked: boolean;
    constructor(p: number, k: Kind, owned: boolean);
    hold(): number;
    release(): void;
    free(): void;
    [Symbol.dispose](): void;
}
export declare function hold(handle: Handle): number;
export declare function holdOr(handle: Handle | undefined): number;
export declare function lentHandle<T extends string>(p: number, k: Kind): Handle<T>;
export declare function int(v: number, min: number, max: number, what: string): number;
export declare function nonZero<T extends number | u64 | i64>(v: T): T;
export declare function f32(v: number): number;
export declare function char(v: number): number;
export declare function u64Of(hi: number, lo: number): u64;
export declare function i64Of(hi: number, lo: number): i64;
export declare function u64Halves(v: u64): [number, number];
export declare function i64Halves(v: i64): [number, number];
export declare function u64Get(p: number): u64;
export declare function i64Get(p: number): i64;
export declare function u64Put(p: number, v: u64): void;
export declare function i64Put(p: number, v: i64): void;
export declare function u64Big(v: u64): bigint;
export declare function i64Big(v: i64): bigint;
export declare function u64OfBig(v: bigint): u64;
export declare function i64OfBig(v: bigint): i64;
export declare function u128Get(p: number): u128;
export declare function u128Put(p: number, v: u128): void;
export declare function u64ToBig(v: u64): bigint;
export declare function i64ToBig(v: i64): bigint;
export declare function u128ToBig(v: u128): bigint;
export declare function u64FromBig(v: bigint): u64;
export declare function i64FromBig(v: bigint): i64;
export declare function u128FromBig(v: bigint): u128;
export declare function encoded(text: string): Uint8Array;
export declare function putStr(dst: number, text: string): void;
export declare function putBytes(dst: number, data: Uint8Array | Bytes): void;
export declare function shared(dst: number, data: Uint8Array | Bytes | undefined): void;
export declare function putNone(dst: number): void;
export declare function isNone(p: number): boolean;
export declare function bytesLent(p: number): Uint8Array;
export declare function strLent(p: number): string;
export declare function bytes_(p: number): Uint8Array;
export declare class Bytes {
    #private;
    private constructor();
    get length(): number;
    at(index: number): number | undefined;
    slice(start?: number, end?: number): Uint8Array<ArrayBuffer>;
    [Symbol.iterator](): IterableIterator<number>;
    [Symbol.dispose](): void;
}
export declare function returnedBytes(p: number): Bytes;
export declare function str(p: number): string;
export declare function copied(dst: number, text: string, message?: boolean): void;
export declare function copiedBytes(dst: number, data: Uint8Array): void;
export declare function array(data: Uint8Array, n: number): Uint8Array;
export declare function pathBytes(path: PathBuf): Uint8Array;
export declare function durationGet(p: number): Duration;
export declare function durationPut(p: number, v: Duration): void;
export declare function ipv4Get(p: number): Ipv4Addr;
export declare function ipv4Put(p: number, v: Ipv4Addr): void;
export declare function ipv6Get(p: number): Ipv6Addr;
export declare function ipv6Put(p: number, v: Ipv6Addr): void;
export declare function ipGet(p: number): IpAddr;
export declare function ipPut(p: number, v: IpAddr): void;
export declare function socketV4Get(p: number): SocketAddrV4;
export declare function socketV4Put(p: number, v: SocketAddrV4): void;
export declare function socketV6Get(p: number): SocketAddrV6;
export declare function socketV6Put(p: number, v: SocketAddrV6): void;
export declare function socketGet(p: number): SocketAddr;
export declare function socketPut(p: number, v: SocketAddr): void;
export declare function give(value: unknown, drop?: () => void): number;
export declare function lend(value: unknown): number;
export declare function find<T>(t: number): T;
export declare const fn: {
    wake: number;
    wakerDrop: number;
    free: number;
    poll: number;
    read: number;
    seek: number;
};
export declare function out<T>(what: string, f: () => T): T;
export declare function wakerOf(dst: number, wake: () => void): void;
export declare function wakerGet(p: number): Waker;
export declare function futureOf(dst: number, promise: Promise<unknown>): void;
export declare function readerOf(dst: number, reader: Reader): void;
export declare function pending<T>(f: number, poll: (f: number, waker: number) => [boolean, T], free: string): Call<T>;
export declare function ready<T>(value: T): Call<T>;
export declare function failed<T>(error: unknown): Call<T>;
export type Source = WebAssembly.Module | BufferSource | Response | PromiseLike<Response>;
export type Glue = {
    readonly prefix: string;
    readonly space: string;
    readonly file: string;
    readonly contract: bigint;
    readonly target: number;
    readonly imports: Record<string, (...args: never[]) => unknown>;
};
export declare function load(source: Source, glue: Glue): Promise<void>;
export {};
