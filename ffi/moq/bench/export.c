/* The frame write path through the exported C ABI: a timed track, groups of 100 frames of
 * 100 bytes, each frame stamped. bench/moq-c.c times the same through moq's rs/moq-c, and
 * bench/export.cpp through the C++ header. Then the read path, each read a future, and a
 * frame's payload read. */

#define _POSIX_C_SOURCE 199309L

#include <stdio.h>
#include <string.h>
#include <time.h>

#include "moq.h"

#define GROUPS 10000
#define FRAMES 100

static double now(void) {
	struct timespec ts;
	clock_gettime(CLOCK_MONOTONIC, &ts);
	return (double)ts.tv_sec * 1e9 + (double)ts.tv_nsec;
}

static moq_buf str(const char *s) {
	return (moq_buf){.ptr = (const uint8_t *)s, .len = strlen(s)};
}

int main(void) {
	moq_broadcast_Info *binfo = moq_broadcast_Info_new();
	moq_broadcast_Producer *broadcast = moq_broadcast_Info_produce(binfo);
	moq_broadcast_Info_free(binfo);
	moq_Timescale *micro = moq_Timescale_MICRO();
	/* Builders take the handle's value, leaving the handle to free. */
	moq_track_Info *defaults = moq_track_Info_default();
	moq_track_Info *info = moq_track_Info_with_timescale(defaults, micro);
	moq_track_Info_free(defaults);
	moq_track_Producer *track = NULL;
	moq_Error *err = NULL;
	if (!moq_broadcast_Producer_create_track(broadcast, str("bench"), info, &track, &err)) {
		return 1;
	}
	moq_track_Info_free(info);

	uint8_t payload[100] = {0};
	moq_buf frame = {.ptr = payload, .len = sizeof payload};
	uint64_t micros = 0;

	/* Each frame stamped through a fresh timestamp handle, as a caller holding a u64 must. */
	double start = now();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = NULL;
		moq_track_Producer_append_group(track, &group, &err);
		for (int f = 0; f < FRAMES; f++) {
			moq_Timestamp *at = NULL;
			moq_TimeOverflow *overflow = NULL;
			moq_Timestamp_from_micros(micros++, &at, &overflow);
			if (!moq_group_Producer_write_frame(group, at, frame, &err)) {
				return 2;
			}
			moq_Timestamp_free(at);
		}
		moq_group_Producer_finish(group, &err);
		moq_group_Producer_free(group);
	}
	double stamped = (now() - start) / (GROUPS * FRAMES);

	/* The same, reusing one timestamp handle: what the handle costs. */
	moq_Timestamp *at = NULL;
	moq_TimeOverflow *overflow = NULL;
	moq_Timestamp_from_micros(micros, &at, &overflow);
	start = now();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = NULL;
		moq_track_Producer_append_group(track, &group, &err);
		for (int f = 0; f < FRAMES; f++) {
			if (!moq_group_Producer_write_frame(group, at, frame, &err)) {
				return 3;
			}
		}
		moq_group_Producer_finish(group, &err);
		moq_group_Producer_free(group);
	}
	double reused = (now() - start) / (GROUPS * FRAMES);

	/* The floor: a borrow of a handle and a bool back. */
	start = now();
	int closed = 0;
	for (int i = 0; i < GROUPS * FRAMES; i++) {
		closed += moq_track_Producer_is_closed(track);
	}
	double call = (now() - start) / (GROUPS * FRAMES);

	/* Reads: each frame written, then read through a new future polled once, ready. */
	moq_waker waker = {0};
	moq_track_Consumer *consumer = moq_track_Producer_consume(track);
	moq_track_Consumer_subscribe_future *subscribing = moq_track_Consumer_subscribe(consumer, NULL);
	moq_track_Subscriber *subscriber = NULL;
	if (moq_track_Consumer_subscribe_poll(subscribing, waker, &subscriber, &err) != MOQ_POLL_READY) {
		return 4;
	}
	moq_track_Consumer_subscribe_future_free(subscribing);
	moq_frame_Frame *last = NULL;
	start = now();
	for (int g = 0; g < GROUPS; g++) {
		moq_group_Producer *group = NULL;
		moq_track_Producer_append_group(track, &group, &err);
		moq_track_Subscriber_recv_group_future *receiving = moq_track_Subscriber_recv_group(subscriber);
		moq_group_Consumer *reader = NULL;
		if (moq_track_Subscriber_recv_group_poll(receiving, waker, &reader, &err) != MOQ_POLL_READY || reader == NULL) {
			return 5;
		}
		moq_track_Subscriber_recv_group_future_free(receiving);
		for (int f = 0; f < FRAMES; f++) {
			if (!moq_group_Producer_write_frame(group, at, frame, &err)) {
				return 6;
			}
			moq_group_Consumer_read_frame_future *reading = moq_group_Consumer_read_frame(reader);
			moq_frame_Frame *read = NULL;
			if (moq_group_Consumer_read_frame_poll(reading, waker, &read, &err) != MOQ_POLL_READY || read == NULL) {
				return 7;
			}
			moq_group_Consumer_read_frame_future_free(reading);
			if (g == GROUPS - 1 && f == FRAMES - 1) {
				last = read;
			} else {
				moq_frame_Frame_free(read);
			}
		}
		moq_group_Producer_finish(group, &err);
		moq_group_Producer_free(group);
		moq_group_Consumer_free(reader);
	}
	double read = (now() - start) / (GROUPS * FRAMES);

	/* Payload reads: the last frame's 100 bytes, owned without a copy, then freed. */
	start = now();
	for (int i = 0; i < GROUPS * FRAMES; i++) {
		moq_buf bytes = moq_frame_Frame_payload_get(last);
		closed += bytes.ptr[0];
		moq_buf_free(bytes);
	}
	double lent = (now() - start) / (GROUPS * FRAMES);
	moq_frame_Frame_free(last);
	moq_Timestamp_free(at);
	moq_track_Subscriber_free(subscriber);
	moq_track_Consumer_free(consumer);

	printf("export: %.0f ns per stamped 100 B frame, %.0f reusing the timestamp, %.1f ns per handle call, %.0f ns per frame "
		   "written and read, %.0f ns per payload read%s\n",
		   stamped, reused, call, read, lent, closed ? "!" : "");
	moq_track_Producer_free(track);
	moq_Timescale_free(micro);
	moq_broadcast_Producer_free(broadcast);
	return 0;
}
