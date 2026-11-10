/* bench/export.c's stamped frame writes, through moq's hand-written rs/moq-c: a timed
 * track, groups of 100 frames of 100 bytes, each frame stamped. */

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

int main(void) {
	int32_t origin = moq_origin_create();
	uint32_t broadcast = 0;
	int32_t publication = moq_origin_create_broadcast((uint32_t)origin, "b", 1, &broadcast);
	/* No info: a raw track on a microsecond timescale. */
	int32_t track = moq_publish_track(broadcast, "bench", 5, NULL);
	if (origin <= 0 || publication <= 0 || track <= 0) {
		return 1;
	}

	uint8_t payload[100] = {0};
	uint64_t micros = 0;

	double start = now();
	for (int g = 0; g < GROUPS; g++) {
		int32_t group = moq_publish_track_group((uint32_t)track);
		for (int f = 0; f < FRAMES; f++) {
			moq_timestamp at = {.value = micros++, .scale = 1000000};
			if (moq_publish_group_frame((uint32_t)group, payload, sizeof payload, &at) < 0) {
				return 2;
			}
		}
		moq_publish_group_finish((uint32_t)group);
	}
	double stamped = (now() - start) / (GROUPS * FRAMES);

	printf("stamped\t%f\n", stamped);
	moq_publish_close(broadcast);
	moq_publication_free((uint32_t)publication);
	moq_origin_close((uint32_t)origin);
	return 0;
}
