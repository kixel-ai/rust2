/* bench/export.c's stamped frame writes, through moq's hand-written rs/moq-c: a timed
 * track, groups of 100 frames of 100 bytes, each frame stamped, in wall and CPU time, or
 * under CodSpeed, once for its CPU simulation to count. */

#define _POSIX_C_SOURCE 199309L

#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

#include "codspeed.h"
#include "moq.h"

#define GROUPS 10000
#define FRAMES 100

static double clock_ns(clockid_t clock) {
	struct timespec ts;
	clock_gettime(clock, &ts);
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

	/* Two rounds, printing the second, as bench/export.c does; under CodSpeed, one. */
	bool measured = codspeed_init("ffi/moq/bench/moq-c.c::moq-c");
	uint8_t payload[100] = {0};
	uint64_t micros = 0;
	for (int round = measured; round < 2; round++) {
		double wall = clock_ns(CLOCK_MONOTONIC);
		double cpu = clock_ns(CLOCK_PROCESS_CPUTIME_ID);
		codspeed_start();
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
		codspeed_stop("stamped");
		wall = (clock_ns(CLOCK_MONOTONIC) - wall) / (GROUPS * FRAMES);
		cpu = (clock_ns(CLOCK_PROCESS_CPUTIME_ID) - cpu) / (GROUPS * FRAMES);

		if (round == 1 && !measured) {
			printf("stamped\t%f\t%f\n", wall, cpu);
		}
	}
	moq_publish_close(broadcast);
	moq_publication_free((uint32_t)publication);
	moq_origin_close((uint32_t)origin);
	return 0;
}
