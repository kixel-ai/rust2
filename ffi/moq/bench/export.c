/* Every case bench/ffi/README.md lists, through the exported C ABI: the floor every other
 * language's binding builds on. bench/export.rs times the same in Rust, and
 * bench/export.cpp through the C++ header. Futures are polled by hand with a null waker.
 * Each case prints its wall time and the process's CPU time, or under CodSpeed, runs once
 * for its CPU simulation to count. */

#define _POSIX_C_SOURCE 200112L

#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "codspeed.h"
#include "moq.h"

/* A tenth under CodSpeed, whose simulation counts every call exactly. */
static int GROUPS = 10000;
#define FRAMES 100
#define N (GROUPS * FRAMES)
#define THREADS 4

static const size_t sizes[] = {100, 1024, 16384, 65536, 1048576};

static double clock_ns(clockid_t clock) {
	struct timespec ts;
	clock_gettime(clock, &ts);
	return (double)ts.tv_sec * 1e9 + (double)ts.tv_nsec;
}

/* Wall and CPU nanoseconds: a point, an elapsed span, or a span per call. */
typedef struct {
	double wall, cpu;
} span;

static span now(void) {
	return (span){clock_ns(CLOCK_MONOTONIC), clock_ns(CLOCK_PROCESS_CPUTIME_ID)};
}

/* Nanoseconds per call since `start`, over `n` calls. */
static span per(span start, double n) {
	span end = now();
	return (span){(end.wall - start.wall) / n, (end.cpu - start.cpu) / n};
}

/* Whether CodSpeed counts this run, which then skips the warm-up and the threaded cases. */
static bool measured;

/* A case's start: now, and CodSpeed counting from here. */
static span begin(void) {
	span start = now();
	codspeed_start();
	return start;
}

/* The case `name`'s nanoseconds per call since `start`, over `n` calls. */
static span end(span start, double n, const char *name) {
	codspeed_stop(name);
	return per(start, n);
}

static moq_buf str(const char *s) {
	return (moq_buf){.ptr = (const uint8_t *)s, .len = strlen(s)};
}

static void fail(const char *what) {
	fprintf(stderr, "bench: %s\n", what);
	exit(1);
}

static const moq_waker waker = {0};

/* What the loops sum, kept so the compiler cannot drop the calls. */
static volatile unsigned sink;

static moq_group_Consumer *recv_group(moq_track_Subscriber *subscriber) {
	moq_track_Subscriber_recv_group_future *receiving = moq_track_Subscriber_recv_group(subscriber, NULL);
	moq_group_Consumer *reader = NULL;
	moq_Error *err = NULL;
	if (moq_track_Subscriber_recv_group_poll(receiving, waker, &reader, &err) != MOQ_POLL_READY || reader == NULL) {
		fail("recv_group");
	}
	moq_track_Subscriber_recv_group_future_free(receiving);
	return reader;
}

/* A read ready at once: made and polled in one call, with no handle. */
static moq_frame_Frame *read_frame(moq_group_Consumer *reader) {
	moq_group_Consumer_read_frame_future *reading = NULL;
	moq_frame_Frame *read = NULL;
	moq_Error *err = NULL;
	if (moq_group_Consumer_read_frame_start(reader, &read, &err, &reading, NULL) != MOQ_POLL_READY || read == NULL) {
		fail("read_frame");
	}
	return read;
}

static moq_group_Producer *append_group(moq_track_Producer *track) {
	moq_group_Producer *group = NULL;
	moq_Error *err = NULL;
	if (!moq_track_Producer_append_group(track, &group, &err, NULL)) {
		fail("append_group");
	}
	return group;
}

static void write_frame(moq_group_Producer *group, moq_Timestamp at, moq_buf frame) {
	moq_Error *err = NULL;
	if (!moq_group_Producer_write_frame(group, (moq_option_Timestamp){.some = true, .value = at}, frame, &err, NULL)) {
		fail("write_frame");
	}
}

static void finish(moq_group_Producer *group) {
	moq_Error *err = NULL;
	if (!moq_group_Producer_finish(group, &err, NULL)) {
		fail("finish");
	}
	moq_group_Producer_free(group);
}

/* Groups of 100 frames of `size` bytes written and each read back, its payload's first byte
 * touched, in about 64 MiB and at least 300 frames: megabytes a wall and a CPU second. */
static span throughput(moq_track_Producer *track, moq_track_Subscriber *subscriber, moq_Timestamp at, size_t size, unsigned *sum) {
	size_t groups = (64u << 20) / (size * FRAMES);
	groups = groups < 3 ? 3 : groups > 2000 ? 2000 : groups;
	uint8_t *payload = calloc(size, 1);
	moq_buf frame = {.ptr = payload, .len = size};
	char name[32];
	snprintf(name, sizeof name, "throughput[%zu]", size);
	span start = begin();
	for (size_t g = 0; g < groups; g++) {
		moq_group_Producer *group = append_group(track);
		moq_group_Consumer *reader = recv_group(subscriber);
		for (int f = 0; f < FRAMES; f++) {
			write_frame(group, at, frame);
			moq_frame_Frame *read = read_frame(reader);
			moq_buf bytes = moq_frame_Frame_payload_get(read, NULL);
			*sum += bytes.ptr[0];
			moq_buf_free(bytes);
			moq_frame_Frame_free(read);
		}
		finish(group);
		moq_group_Consumer_free(reader);
	}
	span ns = end(start, (double)(groups * FRAMES * size), name);
	free(payload);
	return (span){1e3 / ns.wall, 1e3 / ns.cpu};
}

/* `concurrent`'s start: each thread counts itself ready, then waits for go. macOS has no
 * pthread barrier. */
typedef struct {
	atomic_int ready;
	atomic_bool go;
} gate;

/* One of `concurrent`'s threads: N calls on its handle once every thread is ready. */
typedef struct {
	gate *start;
	const void *handle;
	int getter;
	unsigned sum;
} worker;

static void *work(void *arg) {
	worker *w = arg;
	unsigned sum = 0;
	atomic_fetch_add(&w->start->ready, 1);
	while (!atomic_load(&w->start->go)) {
	}
	if (w->getter) {
		for (int i = 0; i < N; i++) {
			sum += (unsigned)moq_group_Demand_sequence(w->handle, NULL);
		}
	} else {
		for (int i = 0; i < N; i++) {
			sum += moq_track_Producer_is_closed(w->handle, NULL);
		}
	}
	w->sum = sum;
	return NULL;
}

/* N calls on each of THREADS threads at once, thread i on handles[i]: wall nanoseconds per
 * call on one thread, and CPU nanoseconds per call. */
static span concurrent(const void *const *handles, int getter, unsigned *sum) {
	gate start = {0};
	worker workers[THREADS];
	pthread_t threads[THREADS];
	for (int i = 0; i < THREADS; i++) {
		workers[i] = (worker){.start = &start, .handle = handles[i], .getter = getter};
		if (pthread_create(&threads[i], NULL, work, &workers[i]) != 0) {
			fail("pthread_create");
		}
	}
	while (atomic_load(&start.ready) < THREADS) {
	}
	span begin = now();
	atomic_store(&start.go, true);
	for (int i = 0; i < THREADS; i++) {
		pthread_join(threads[i], NULL);
		*sum += workers[i].sum;
	}
	span end = now();
	return (span){(end.wall - begin.wall) / N, (end.cpu - begin.cpu) / ((double)N * THREADS)};
}

static void round_(int print) {
	moq_broadcast_Info *binfo = moq_broadcast_Info_new();
	moq_broadcast_Producer *broadcast = moq_broadcast_Info_produce(binfo, NULL);
	moq_broadcast_Info_free(binfo);
	moq_option_Timescale micro = {.some = true, .value = moq_Timescale_MICRO()};
	/* Builders take the handle's value, leaving the handle to free. */
	moq_track_Info *defaults = moq_track_Info_default();
	moq_track_Info *info = moq_track_Info_with_timescale(defaults, micro, NULL);
	moq_track_Info_free(defaults);
	moq_track_Producer *track = NULL;
	moq_Error *err = NULL;
	if (!moq_broadcast_Producer_create_track(broadcast, str("bench"), info, &track, &err, NULL)) {
		fail("create_track");
	}
	moq_track_Info_free(info);

	uint8_t payload[100] = {0};
	moq_buf frame = {.ptr = payload, .len = sizeof payload};
	unsigned sum = 0;
	span start;

	start = begin();
	for (int i = 0; i < N; i++) {
		sum += moq_track_Producer_is_closed(track, NULL);
	}
	span call = end(start, N, "call");

	moq_group_Producer *demanded = append_group(track);
	moq_group_Demand *demand = moq_group_Producer_demand(demanded, NULL);
	start = begin();
	for (int i = 0; i < N; i++) {
		sum += (unsigned)moq_group_Demand_sequence(demand, NULL);
	}
	span getter = end(start, N, "getter");

	/* The handle call and the getter from THREADS threads: on one shared handle, then on a
	 * clone each, a handle of its own to the same value. CodSpeed's simulation runs one
	 * thread at a time, which contends for nothing, so it skips them. */
	span shared_call = {0}, own_call = {0}, shared_getter = {0}, own_getter = {0};
	if (!measured) {
		const void *shared_track[THREADS], *shared_demand[THREADS], *own_track[THREADS], *own_demand[THREADS];
		for (int i = 0; i < THREADS; i++) {
			shared_track[i] = track;
			shared_demand[i] = demand;
			own_track[i] = moq_track_Producer_clone(track, NULL);
			own_demand[i] = moq_group_Demand_clone(demand, NULL);
		}
		shared_call = concurrent(shared_track, 0, &sum);
		own_call = concurrent(own_track, 0, &sum);
		shared_getter = concurrent(shared_demand, 1, &sum);
		own_getter = concurrent(own_demand, 1, &sum);
		for (int i = 0; i < THREADS; i++) {
			moq_track_Producer_free((moq_track_Producer *)own_track[i]);
			moq_group_Demand_free((moq_group_Demand *)own_demand[i]);
		}
	}
	moq_group_Demand_free(demand);
	finish(demanded);

	start = begin();
	for (int i = 0; i < N; i++) {
		sum += moq_track_Producer_largest(track, NULL).some;
	}
	span record = end(start, N, "record");

	start = begin();
	for (int i = 0; i < N; i++) {
		moq_track_Info_free(moq_track_Producer_info(track, NULL));
	}
	span object = end(start, N, "object");

	/* Each frame stamped through a fresh timestamp, as a caller holding a u64 must. */
	uint64_t micros = 0;
	start = begin();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = append_group(track);
		for (int f = 0; f < FRAMES; f++) {
			moq_Timestamp at;
			moq_TimeOverflow overflow;
			if (!moq_Timestamp_from_micros(micros++, &at, &overflow)) {
				fail("from_micros");
			}
			write_frame(group, at, frame);
		}
		finish(group);
	}
	span stamped = end(start, N, "stamped");

	moq_Timestamp at;
	moq_TimeOverflow overflow;
	if (!moq_Timestamp_from_micros(micros, &at, &overflow)) {
		fail("from_micros");
	}
	start = begin();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = append_group(track);
		for (int f = 0; f < FRAMES; f++) {
			write_frame(group, at, frame);
		}
		finish(group);
	}
	span write = end(start, N, "write");

	moq_track_Consumer *consumer = moq_track_Producer_consume(track, NULL);
	moq_track_Consumer_subscribe_future *subscribing = moq_track_Consumer_subscribe(consumer, NULL, NULL);
	moq_track_Subscriber *subscriber = NULL;
	if (moq_track_Consumer_subscribe_poll(subscribing, waker, &subscriber, &err) != MOQ_POLL_READY) {
		fail("subscribe");
	}
	moq_track_Consumer_subscribe_future_free(subscribing);
	moq_track_Consumer_free(consumer);

	/* Each frame written, then read through a new future ready at its first poll. */
	moq_frame_Frame *last = NULL;
	start = begin();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = append_group(track);
		moq_group_Consumer *reader = recv_group(subscriber);
		for (int f = 0; f < FRAMES; f++) {
			write_frame(group, at, frame);
			if (last != NULL) {
				moq_frame_Frame_free(last);
			}
			last = read_frame(reader);
		}
		finish(group);
		moq_group_Consumer_free(reader);
	}
	span ready = end(start, N, "ready");

	/* Each read polled before its frame is written, so it pends, then polled again. */
	start = begin();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = append_group(track);
		moq_group_Consumer *reader = recv_group(subscriber);
		for (int f = 0; f < FRAMES; f++) {
			moq_group_Consumer_read_frame_future *reading = moq_group_Consumer_read_frame(reader, NULL);
			moq_frame_Frame *read = NULL;
			if (moq_group_Consumer_read_frame_poll(reading, waker, &read, &err) != MOQ_POLL_PENDING) {
				fail("woken read_frame ready");
			}
			write_frame(group, at, frame);
			if (moq_group_Consumer_read_frame_poll(reading, waker, &read, &err) != MOQ_POLL_READY || read == NULL) {
				fail("woken read_frame");
			}
			moq_group_Consumer_read_frame_future_free(reading);
			moq_frame_Frame_free(read);
		}
		finish(group);
		moq_group_Consumer_free(reader);
	}
	span woken = end(start, N, "woken");

	/* The last frame's 100 bytes, owned without a copy, then freed. */
	start = begin();
	for (int i = 0; i < N; i++) {
		moq_buf bytes = moq_frame_Frame_payload_get(last, NULL);
		sum += bytes.ptr[0];
		moq_buf_free(bytes);
	}
	span payload_ = end(start, N, "payload");
	moq_frame_Frame_free(last);

	span mbps[sizeof sizes / sizeof sizes[0]];
	for (size_t i = 0; i < sizeof sizes / sizeof sizes[0]; i++) {
		mbps[i] = throughput(track, subscriber, at, sizes[i], &sum);
	}

	sink = sum;
	moq_track_Subscriber_free(subscriber);
	moq_track_Producer_free(track);
	moq_broadcast_Producer_free(broadcast);

	if (print) {
		const char *names[] = {"call", "getter", "shared:call", "own:call", "shared:getter", "own:getter", "record", "object", "stamped", "write", "ready", "woken", "payload"};
		span values[] = {call, getter, shared_call, own_call, shared_getter, own_getter, record, object, stamped, write, ready, woken, payload_};
		for (size_t i = 0; i < sizeof names / sizeof names[0]; i++) {
			printf("%s\t%f\t%f\n", names[i], values[i].wall, values[i].cpu);
		}
		for (size_t i = 0; i < sizeof sizes / sizeof sizes[0]; i++) {
			printf("mbps:%zu\t%f\t%f\n", sizes[i], mbps[i].wall, mbps[i].cpu);
		}
	}
}

int main(void) {
	/* Under CodSpeed, one round, which prints nothing: its simulation counts each case. */
	measured = codspeed_init("ffi/moq/bench/export.c::c");
	if (measured) {
		GROUPS /= 10;
	} else {
		round_(0);
	}
	round_(!measured);
	return 0;
}
