/* moq's exported C API against the real library: a broadcast's track, published and
 * subscribed to in one process, with every read polled as a future, and an origin.
 * moq-pattern, kio, moq-tokio, hang, and moq-mux export into the same library, and
 * each names the others' types, the catalog is lent to C for a call, and a session's
 * config is filled in field by field.
 *
 * The local reads need no runtime: the waker counts wakes, and each poll that must wait
 * is polled again after the write that wakes it. A session over QUIC runs on moq-tokio's
 * fallback runtime, whose threads wake this one. */

#define _POSIX_C_SOURCE 200809L

#include <errno.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include "moq.h"

static int failures = 0;

#define CHECK(cond)                                                                                                    \
	do {                                                                                                               \
		if (!(cond)) {                                                                                                 \
			fprintf(stderr, "%s:%d: %s\n", __FILE__, __LINE__, #cond);                                                 \
			failures++;                                                                                                \
		}                                                                                                              \
	} while (0)

static moq_buf str(const char *s) {
	return (moq_buf){.ptr = (const uint8_t *)s, .len = strlen(s)};
}

/* Whether a returned buffer holds `s`, freeing it. */
static int is(moq_buf buf, const char *s) {
	int same = buf.len == strlen(s) && memcmp(buf.ptr, s, buf.len) == 0;
	if (!same) {
		fprintf(stderr, "got \"%.*s\", want \"%s\"\n", (int)buf.len, (const char *)buf.ptr, s);
	}
	moq_buf_free(buf);
	return same;
}

/* Moves the stats track in data into the lent catalog, in place. */
static void sets_stats(void *data, moq_hang_Catalog_Extra *catalog) {
	moq_hang_Catalog_Extra_stats_set(catalog, data, NULL);
}

/* Whether the lent catalog lists its stats track as `stats`: counts it in data. */
static void lists_stats(void *data, moq_hang_Catalog_Extra *catalog) {
	moq_hang_catalog_Stats *stats = moq_hang_Catalog_Extra_stats_get(catalog, NULL);
	moq_hang_catalog_Stats *want = moq_hang_catalog_Stats_new(str("stats"));
	if (stats != NULL && moq_hang_catalog_Stats_eq(stats, want, NULL)) {
		(*(int *)data)++;
	}
	moq_hang_catalog_Stats_free(want);
	moq_hang_catalog_Stats_free(stats);
}

/* Whether `buf` holds `s` anywhere. */
static int contains(moq_buf buf, const char *s) {
	size_t len = strlen(s);
	for (size_t i = 0; i + len <= buf.len; i++) {
		if (memcmp(buf.ptr + i, s, len) == 0) {
			return 1;
		}
	}
	return 0;
}

/* Sets the lent catalog's `app` section to the JSON document in data, in place. */
static void sets_app(void *data, moq_hang_Catalog_Extra *catalog) {
	moq_mux_catalog_hang_Extra *sections = moq_hang_Catalog_Extra_ext_get(catalog, NULL);
	moq_mux_Error *err = NULL;
	CHECK(moq_mux_catalog_hang_Extra_set(sections, str("app"), data, &err, NULL));
	moq_hang_Catalog_Extra_ext_set(catalog, sections, NULL);
	moq_mux_catalog_hang_Extra_free(sections);
}

/* What C reads back from the lent catalog: its JSON, and its `app` section as text. */
struct readback {
	moq_buf json;
	moq_buf app;
};

static void reads_back(void *data, moq_hang_Catalog_Extra *catalog) {
	struct readback *back = data;
	moq_hang_Error *hang_err = NULL;
	CHECK(moq_hang_Catalog_Extra_to_json(catalog, &back->json, &hang_err, NULL));
	moq_mux_catalog_hang_Extra *sections = moq_hang_Catalog_Extra_ext_get(catalog, NULL);
	moq_Json *app = moq_mux_catalog_hang_Extra_get(sections, str("app"), NULL);
	if (app != NULL) {
		back->app = moq_Json_to_text(app, NULL);
		moq_Json_free(app);
	}
	moq_mux_catalog_hang_Extra_free(sections);
}

static int wakes = 0;

static void wake(void *data) {
	(void)data;
	wakes++;
}

/* A session's futures complete on moq-tokio's runtime threads, so its waker signals this
 * thread, which waits for a wake after a poll finds a future pending. */
static pthread_mutex_t woken_lock = PTHREAD_MUTEX_INITIALIZER;
static pthread_cond_t woken_cond = PTHREAD_COND_INITIALIZER;
static unsigned long woken = 0;

static void wake_session(void *data) {
	(void)data;
	pthread_mutex_lock(&woken_lock);
	woken++;
	pthread_cond_broadcast(&woken_cond);
	pthread_mutex_unlock(&woken_lock);
}

static const moq_waker session_waker = {.wake = wake_session};

static unsigned long wakes_so_far(void) {
	pthread_mutex_lock(&woken_lock);
	unsigned long n = woken;
	pthread_mutex_unlock(&woken_lock);
	return n;
}

/* Waits for a wake after `seen`, failing the run rather than hanging when none comes. */
static void wait_wake(unsigned long seen) {
	struct timespec deadline;
	clock_gettime(CLOCK_REALTIME, &deadline);
	deadline.tv_sec += 30;
	pthread_mutex_lock(&woken_lock);
	while (woken == seen) {
		if (pthread_cond_timedwait(&woken_cond, &woken_lock, &deadline) == ETIMEDOUT) {
			fprintf(stderr, "no wake in 30 seconds\n");
			abort();
		}
	}
	pthread_mutex_unlock(&woken_lock);
}

/* The server's accept loop, which must keep running while the client waits on it. */
static moq_tokio_Server_serve_publish_future *serving = NULL;

/* Polls `poll` until it is ready, and the server's accept loop alongside it. */
#define AWAIT(poll)                                                                                                    \
	for (;;) {                                                                                                         \
		unsigned long seen = wakes_so_far();                                                                           \
		moq_tokio_Error *serve_err = NULL;                                                                             \
		if (moq_tokio_Server_serve_publish_poll(serving, session_waker, &serve_err) == MOQ_POLL_READY) {             \
			fprintf(stderr, "%s:%d: the server stopped\n", __FILE__, __LINE__);                                        \
			abort();                                                                                                   \
		}                                                                                                              \
		if ((poll) == MOQ_POLL_READY) {                                                                                \
			break;                                                                                                     \
		}                                                                                                              \
		wait_wake(seen);                                                                                               \
	}

int main(void) {
	moq_waker waker = {.wake = wake};
	moq_Error *err = NULL;
	CHECK(moq_contract() == MOQ_CONTRACT);

	moq_broadcast_Info *info = moq_broadcast_Info_new();
	moq_broadcast_Producer *broadcast = moq_broadcast_Info_produce(info, NULL);
	moq_broadcast_Info_free(info);

	moq_track_Producer *track = NULL;
	CHECK(moq_broadcast_Producer_create_track(broadcast, str("video"), NULL, &track, &err, NULL));
	CHECK(is(moq_track_Producer_name(track, NULL), "video"));

	/* A second track of the same name is an error, printed as Rust prints it. */
	moq_track_Producer *duplicate = NULL;
	CHECK(!moq_broadcast_Producer_create_track(broadcast, str("video"), NULL, &duplicate, &err, NULL));
	CHECK(err != NULL && is(moq_Error_to_string(err, NULL), "duplicate"));
	moq_Error_free(err);

	moq_track_Consumer *consumer = moq_track_Producer_consume(track, NULL);
	moq_track_Consumer_subscribe_future *subscribing = moq_track_Consumer_subscribe(consumer, NULL, NULL);
	moq_track_Subscriber *subscriber = NULL;
	CHECK(moq_track_Consumer_subscribe_poll(subscribing, waker, &subscriber, &err) == MOQ_POLL_READY);
	moq_track_Consumer_subscribe_future_free(subscribing);

	/* A group's arrival wakes the subscriber's pending receive. */
	moq_track_Subscriber_recv_group_future *receiving = moq_track_Subscriber_recv_group(subscriber, NULL);
	moq_group_Consumer *group = NULL;
	CHECK(moq_track_Subscriber_recv_group_poll(receiving, waker, &group, &err) == MOQ_POLL_PENDING);
	moq_group_Producer *writer = NULL;
	CHECK(moq_track_Producer_append_group(track, &writer, &err, NULL));
	CHECK(wakes == 1);
	CHECK(moq_track_Subscriber_recv_group_poll(receiving, waker, &group, &err) == MOQ_POLL_READY);
	moq_track_Subscriber_recv_group_future_free(receiving);
	CHECK(group != NULL && moq_group_Consumer_index(group, NULL) == 0);

	/* So does a frame's. */
	moq_group_Consumer_read_frame_future *reading = moq_group_Consumer_read_frame(group, NULL);
	moq_frame_Frame *frame = NULL;
	CHECK(moq_group_Consumer_read_frame_poll(reading, waker, &frame, &err) == MOQ_POLL_PENDING);
	CHECK(moq_group_Producer_write_frame(writer, (moq_option_Timestamp){0}, str("hello"), &err, NULL));
	CHECK(wakes == 2);
	CHECK(moq_group_Consumer_read_frame_poll(reading, waker, &frame, &err) == MOQ_POLL_READY);
	moq_group_Consumer_read_frame_future_free(reading);
	CHECK(frame != NULL && is(moq_frame_Frame_payload_get(frame, NULL), "hello"));
	CHECK(!moq_frame_Frame_timestamp_get(frame, NULL).some);
	/* When it arrived: kio's clock's `Instant`, from kio's own export. */
	moq_clock_Instant arrived = moq_frame_Frame_arrived(frame, NULL);
	moq_clock_Instant now = moq_clock_now();
	CHECK(moq_clock_Instant_checked_duration_since(&now, arrived).some);
	moq_frame_Frame_free(frame);

	/* A timestamp is a `Copy` value C copies, which allocates nothing. The track is
	 * untimed, so a timestamped frame is refused. */
	moq_Timestamp at;
	moq_TimeOverflow overflow;
	CHECK(moq_Timestamp_from_millis(1500, &at, &overflow));
	CHECK(moq_Timestamp_as_secs(&at) == 1);
	CHECK(!moq_group_Producer_write_frame(writer, (moq_option_Timestamp){.some = true, .value = at}, str("late"), &err, NULL));
	CHECK(err != NULL && is(moq_Error_to_string(err, NULL), "frame timestamp doesn't fit its track"));
	moq_Error_free(err);
	CHECK(moq_Timestamp_as_secs(&at) == 1);

	/* The last frame, bytes that are not UTF-8, then the group's end, which reads as None. */
	CHECK(moq_group_Producer_write_frame(writer, (moq_option_Timestamp){0}, str("\xffworld"), &err, NULL));
	CHECK(moq_group_Producer_finish(writer, &err, NULL));
	reading = moq_group_Consumer_read_frame(group, NULL);
	CHECK(moq_group_Consumer_read_frame_poll(reading, waker, &frame, &err) == MOQ_POLL_READY);
	moq_group_Consumer_read_frame_future_free(reading);
	CHECK(frame != NULL && is(moq_frame_Frame_payload_get(frame, NULL), "\xffworld"));
	moq_frame_Frame_free(frame);
	reading = moq_group_Consumer_read_frame(group, NULL);
	frame = (moq_frame_Frame *)1;
	CHECK(moq_group_Consumer_read_frame_poll(reading, waker, &frame, &err) == MOQ_POLL_READY);
	CHECK(frame == NULL);
	moq_group_Consumer_read_frame_future_free(reading);

	moq_group_Consumer_free(group);
	moq_group_Producer_free(writer);
	moq_track_Subscriber_free(subscriber);
	moq_track_Consumer_free(consumer);
	moq_track_Producer_free(track);
	moq_broadcast_Producer_free(broadcast);

	/* An origin and the driver that runs it, a tuple returned through two out pointers.
	 * Its root, which it lends, is returned as a new handle to a clone. */
	moq_origin_Config *config = moq_origin_Config_default();
	moq_origin_Producer *origin = NULL;
	moq_origin_Driver *driver = NULL;
	moq_origin_Producer_new(config, &origin, &driver, NULL);
	moq_origin_Config_free(config);
	CHECK(origin != NULL && driver != NULL);
	moq_Path *root = moq_origin_Producer_root(origin, NULL);
	CHECK(is(moq_Path_as_str(root, NULL), ""));
	moq_Path_free(root);

	/* Scoped beneath a root to patterns, which moq-pattern's own export builds. */
	moq_Pattern *alice = NULL;
	moq_InvalidPattern invalid;
	CHECK(moq_Pattern_subtree(str("alice"), &alice, &invalid));
	moq_Patterns *patterns = moq_Patterns_new();
	CHECK(moq_Patterns_insert(patterns, alice, NULL));
	moq_Pattern_free(alice);
	moq_origin_Producer *scoped = NULL;
	CHECK(moq_origin_Producer_scope(origin, str("room"), patterns, &scoped, &err, NULL));
	moq_Patterns_free(patterns);
	root = moq_origin_Producer_root(scoped, NULL);
	CHECK(is(moq_Path_as_str(root, NULL), "room"));
	moq_Path_free(root);
	moq_Path *absolute = NULL;
	CHECK(moq_origin_Producer_absolute(scoped, str("alice"), &absolute, &err, NULL));
	CHECK(is(moq_Path_as_str(absolute, NULL), "room/alice"));
	moq_Path_free(absolute);
	moq_Patterns *allowed = moq_origin_Producer_allowed(scoped, NULL);
	CHECK(moq_Patterns_matches(allowed, str("alice/cam"), NULL));
	CHECK(!moq_Patterns_matches(allowed, str("bob"), NULL));
	moq_Patterns_free(allowed);
	moq_origin_Consumer *reader = moq_origin_Producer_consume(scoped, NULL);
	CHECK(moq_origin_Consumer_absolute(reader, str("bob"), &absolute, &err, NULL));
	CHECK(is(moq_Path_as_str(absolute, NULL), "room/bob"));
	moq_Path_free(absolute);
	moq_origin_Consumer_free(reader);
	moq_origin_Producer_free(scoped);

	/* A broadcast the origin creates is stamped with its path, which its info lends. */
	moq_origin_Publication *publication = NULL;
	CHECK(moq_origin_Producer_create_broadcast(origin, str("room/alice"), &publication, &err, NULL));
	moq_broadcast_Producer *created = moq_origin_Publication_broadcast(publication);
	moq_origin_Publication_free(publication);
	moq_broadcast_Info *stamped = moq_broadcast_Producer_info(created, NULL);
	moq_Path *path = moq_broadcast_Info_path_get(stamped, NULL);
	CHECK(is(moq_Path_as_str(path, NULL), "room/alice"));

	/* Paths own their segments, so C builds them, and relative ones, from a string. */
	moq_Path *cam = NULL;
	moq_path_Error path_error;
	CHECK(!moq_Path_new(str("/cam/"), &cam, &path_error) && path_error == MOQ_PATH_ERROR_EMPTY_SEGMENT);
	CHECK(moq_Path_new(str("cam"), &cam, &path_error));
	CHECK(is(moq_Path_as_str(cam, NULL), "cam"));
	moq_Path *joined = NULL;
	CHECK(moq_Path_join(path, cam, &joined, &path_error, NULL));
	CHECK(is(moq_Path_as_str(joined, NULL), "room/alice/cam"));
	moq_Path *stripped = moq_Path_strip_prefix(joined, path, NULL);
	CHECK(stripped != NULL && moq_Path_eq(stripped, cam, NULL));
	moq_Path_free(stripped);
	CHECK(moq_Path_strip_prefix(path, joined, NULL) == NULL);
	moq_path_Relative *up = NULL;
	CHECK(moq_path_Relative_new(str("../bob"), &up, &path_error));
	moq_Path *resolved = NULL;
	CHECK(moq_Path_resolve(joined, up, &resolved, &path_error, NULL));
	CHECK(is(moq_Path_as_str(resolved, NULL), "room/bob"));
	moq_Path_free(resolved);
	moq_path_Relative_free(up);
	moq_Path_free(joined);
	moq_Path_free(cam);
	moq_Path_free(path);
	moq_broadcast_Info_free(stamped);
	moq_broadcast_Producer_free(created);

	/* The driver, polled as a language's event loop would: at the time, and with a waiter
	 * made from the waker and held across polls, since what a poll registers lives only
	 * as long as its waiter. It runs until every owner of the origin is gone. */
	moq_Waiter *waiter = moq_Waiter_new(waker);
	now = moq_clock_now();
	moq_option_clock_Instant deadline;
	CHECK(moq_origin_Driver_poll(driver, now, waiter, &deadline, &err, NULL));
	/* A client publishes the origin through moq-net's generic `with_publisher`, at each
	 * type that implements its bound, by reference, so C keeps its handle. */
	moq_Client *client = moq_Client_new();
	moq_Client *publishing = moq_Client_with_publisher_Producer(client, origin, NULL);
	CHECK(publishing != NULL);
	moq_Client_free(publishing);
	moq_Client_free(client);
	int before = wakes;
	moq_origin_Producer_free(origin);
	/* The last owner's drop wakes the driver, which then finishes, cleanly. */
	CHECK(wakes > before);
	now = moq_clock_now();
	deadline = (moq_option_clock_Instant){0};
	err = NULL;
	CHECK(!moq_origin_Driver_poll(driver, now, waiter, &deadline, &err, NULL));
	CHECK(!deadline.some && err != NULL && is(moq_Error_to_string(err, NULL), "closed"));
	moq_Error_free(err);
	err = NULL;
	moq_Waiter_free(waiter);
	moq_origin_Driver_free(driver);

	/* moq-mux reads an OpusHead into hang's catalog config: 44.1 kHz in, stereo, which
	 * the catalog lists at the decoder's 48 kHz, keeping the head as its description. */
	static const uint8_t head[] = {'O', 'p', 'u', 's', 'H', 'e', 'a', 'd', 1, 2, 0x38, 0x01, 0x44, 0xac, 0, 0, 0, 0, 0};
	moq_hang_catalog_AudioConfig *audio = NULL;
	moq_mux_Error *mux_err = NULL;
	CHECK(moq_mux_codec_opus_config((moq_buf){.ptr = head, .len = sizeof head}, &audio, &mux_err));
	CHECK(moq_hang_catalog_AudioConfig_sample_rate_get(audio, NULL) == 48000);
	CHECK(moq_hang_catalog_AudioConfig_channel_count_get(audio, NULL) == 2);
	moq_hang_catalog_AudioCodec codec = moq_hang_catalog_AudioConfig_codec_get(audio, NULL);
	CHECK(codec.tag == MOQ_HANG_CATALOG_AUDIOCODEC_OPUS);
	moq_hang_catalog_AudioCodec_free(&codec);
	moq_buf description = moq_hang_catalog_AudioConfig_description_get(audio, NULL);
	CHECK(description.len == sizeof head && memcmp(description.ptr, head, sizeof head) == 0);
	moq_buf_free(description);
	moq_hang_catalog_AudioConfig_free(audio);
	/* A truncated head is moq-mux's error. */
	CHECK(!moq_mux_codec_opus_config((moq_buf){.ptr = head, .len = 8}, &audio, &mux_err));
	CHECK(mux_err != NULL);
	moq_buf message = moq_mux_Error_to_string(mux_err);
	CHECK(message.len > 0);
	moq_buf_free(message);
	moq_mux_Error_free(mux_err);
	mux_err = NULL;

	/* A subscription's ranges, from any iterable of them: an array, copied in. */
	moq_track_Range ranges[] = {
	    moq_track_Range_live(),
	    moq_track_Range_groups((moq_bounds_u64){.start = {.kind = MOQ_BOUND_INCLUDED, .value = 2},
	                                            .end = {.kind = MOQ_BOUND_EXCLUDED, .value = 6}}),
	};
	moq_track_Subscription *unranged = moq_track_Subscription_default();
	moq_track_Subscription *ranged =
	    moq_track_Subscription_with_ranges(unranged, (moq_vec_track_Range){.ptr = ranges, .len = 2}, NULL);
	moq_track_Subscription_free(unranged);
	moq_vec_track_Range got = moq_track_Subscription_ranges_get(ranged, NULL);
	CHECK(got.len == 2 && moq_track_Range_eq(&got.ptr[0], &ranges[0]));
	CHECK(got.len == 2 && got.ptr[1].start.tag == MOQ_TRACK_ANCHOR_ABSOLUTE && got.ptr[1].start.absolute._0 == 2);
	CHECK(got.len == 2 && got.ptr[1].end.some && got.ptr[1].end.value.absolute._0 == 6);
	moq_vec_track_Range_free(got);
	moq_track_Range_free(&ranges[0]);
	moq_track_Range_free(&ranges[1]);
	moq_track_Subscription_free(ranged);

	/* A catalog at moq-mux's untyped extension, from the config C builds, whose generic
	 * `is_claimed` is named by the last segment of its argument, and whose `mutate` lends
	 * the catalog to C, which sets its fields in place. */
	moq_broadcast_Info *listed = moq_broadcast_Info_new();
	moq_broadcast_Producer *publisher = moq_broadcast_Info_produce(listed, NULL);
	moq_broadcast_Info_free(listed);
	moq_mux_catalog_Config *catalog_config = moq_mux_catalog_Config_default();
	moq_hang_Catalog_Extra *hang = moq_hang_Catalog_Extra_default();
	moq_mux_catalog_Config_Extra *extra = moq_mux_catalog_Config_with_catalog_Extra(catalog_config, hang, NULL);
	moq_mux_catalog_Config_free(catalog_config);
	moq_hang_Catalog_Extra_free(hang);
	moq_mux_catalog_Producer_Extra *catalog = NULL;
	CHECK(moq_mux_catalog_Producer_Extra_new(publisher, extra, &catalog, &mux_err, NULL));
	moq_mux_catalog_Config_Extra_free(extra);
	CHECK(!moq_mux_catalog_Producer_Extra_is_claimed_BinaryConfig(catalog, str("state"), NULL));
	/* A field of the lent catalog, set in place: the next call reads it. */
	moq_hang_catalog_Stats *stats = moq_hang_catalog_Stats_new(str("stats"));
	CHECK(moq_mux_catalog_Producer_Extra_mutate(catalog, sets_stats, stats, &mux_err, NULL));
	moq_hang_catalog_Stats_free(stats);
	int listed_stats = 0;
	CHECK(moq_mux_catalog_Producer_Extra_mutate(catalog, lists_stats, &listed_stats, &mux_err, NULL));
	CHECK(listed_stats == 1);
	/* A JSON track the catalog lists, published through moq-mux's generic `json_snapshot`
	 * at a JSON document, and a section C parses from text, set in the lent catalog and
	 * read back as text. Its track has a timescale, since every write is stamped. */
	moq_track_Info *status_info =
		moq_track_Info_with_timescale(moq_track_Info_default(), (moq_option_Timescale){.some = true, .value = moq_Timescale_MILLI()}, NULL);
	moq_track_Producer *status = NULL;
	CHECK(moq_broadcast_Producer_create_track(publisher, str("status"), status_info, &status, &err, NULL));
	moq_mux_json_Config *json_config = moq_mux_json_Config_default();
	moq_mux_json_Options *json_options = moq_mux_json_Options_default();
	moq_mux_json_Snapshot_Json *status_snapshot = NULL;
	CHECK(moq_mux_catalog_Producer_Extra_json_snapshot_Config(catalog, status, json_config, json_options, &status_snapshot,
	                                                      &mux_err, NULL));
	moq_mux_json_Config_free(json_config);
	moq_mux_json_Options_free(json_options);
	moq_track_Producer_free(status);
	CHECK(is(moq_mux_json_Snapshot_Json_name(status_snapshot, NULL), "status"));
	CHECK(moq_mux_catalog_Producer_Extra_is_claimed_JsonConfig(catalog, str("status"), NULL));
	/* A value C parses is published as the track's latest, which a subscriber reads back
	 * as JSON text. */
	moq_Json *latest = NULL;
	moq_buf json_err;
	CHECK(moq_Json_parse(str("{\"on\": true}"), &latest, &json_err));
	CHECK(moq_mux_json_Snapshot_Json_update(status_snapshot, latest, moq_Timestamp_now(), &mux_err, NULL));
	moq_Json_free(latest);
	moq_track_Subscriber *status_subscriber = moq_mux_json_Snapshot_Json_consume(status_snapshot, NULL);
	moq_track_Subscriber_recv_group_future *status_receiving = moq_track_Subscriber_recv_group(status_subscriber, NULL);
	moq_group_Consumer *status_group = NULL;
	CHECK(moq_track_Subscriber_recv_group_poll(status_receiving, waker, &status_group, &err) == MOQ_POLL_READY);
	moq_track_Subscriber_recv_group_future_free(status_receiving);
	moq_frame_Frame *status_frame = NULL;
	if (status_group != NULL) {
		moq_group_Consumer_read_frame_future *status_reading = moq_group_Consumer_read_frame(status_group, NULL);
		CHECK(moq_group_Consumer_read_frame_poll(status_reading, waker, &status_frame, &err) == MOQ_POLL_READY);
		moq_group_Consumer_read_frame_future_free(status_reading);
		moq_group_Consumer_free(status_group);
	}
	CHECK(status_frame != NULL && is(moq_frame_Frame_payload_get(status_frame, NULL), "{\"on\":true}"));
	moq_frame_Frame_free(status_frame);
	moq_track_Subscriber_free(status_subscriber);
	moq_Json *app = NULL;
	CHECK(moq_Json_parse(str("{\"v\": [1, true]}"), &app, &json_err));
	CHECK(moq_mux_catalog_Producer_Extra_mutate(catalog, sets_app, app, &mux_err, NULL));
	moq_Json_free(app);
	struct readback back = {0};
	CHECK(moq_mux_catalog_Producer_Extra_mutate(catalog, reads_back, &back, &mux_err, NULL));
	CHECK(contains(back.json, "\"status\""));
	moq_buf_free(back.json);
	CHECK(is(back.app, "{\"v\":[1,true]}"));
	/* Malformed text fails to parse, and a reserved section is moq-mux's error. */
	CHECK(!moq_Json_parse(str("{\"v\":"), &app, &json_err));
	CHECK(is(json_err, "EOF while parsing a value at line 1 column 5"));
	moq_mux_catalog_hang_Extra *sections = moq_mux_catalog_hang_Extra_default();
	CHECK(moq_Json_parse(str("1"), &app, &json_err));
	CHECK(!moq_mux_catalog_hang_Extra_set(sections, str("json"), app, &mux_err, NULL));
	CHECK(mux_err != NULL && is(moq_mux_Error_to_string(mux_err), "reserved catalog section: json"));
	moq_mux_Error_free(mux_err);
	mux_err = NULL;
	moq_Json_free(app);
	moq_mux_catalog_hang_Extra_free(sections);
	moq_mux_json_Snapshot_Json_free(status_snapshot);
	moq_mux_catalog_Producer_Extra_free(catalog);
	moq_broadcast_Producer_free(publisher);

	/* Outside any tokio runtime, as C is, moq-tokio runs on the fallback it starts. */
	moq_origin_Producer *spawned = moq_tokio_origin_spawn();
	moq_origin_Producer_free(spawned);
	moq_tokio_connect_Config *client_config = moq_tokio_connect_Config_default();
	moq_tokio_quic_Config *quic = moq_tokio_quic_Config_default();
	moq_tokio_Client *tokio_client = NULL;
	moq_tokio_Error *tokio_err = NULL;
	CHECK(moq_tokio_connect_Config_init(client_config, quic, &tokio_client, &tokio_err, NULL));
	moq_tokio_connect_Config_free(client_config);
	moq_tokio_quic_Config_free(quic);
	moq_tokio_Client_free(tokio_client);

	/* moq-tokio's terminal connect errors, which moq-net's `Error` shares no names with. */
	moq_tokio_ConnectError unauthorized = MOQ_TOKIO_CONNECTERROR_UNAUTHORIZED;
	CHECK(moq_tokio_ConnectError_is_auth(&unauthorized));
	CHECK(is(moq_tokio_ConnectError_to_string(&unauthorized), "unauthorized"));

	/* A session's config, filled in as Rust assigns it, and a nested field by getting a copy,
	 * editing it, and setting it back. */
	moq_tokio_connect_Config *connect = moq_tokio_connect_Config_default();
	moq_tokio_tls_Connect *tls = moq_tokio_connect_Config_tls_get(connect, NULL);
	moq_tokio_tls_Connect_insecure_set(tls, (moq_option_bool){.some = true, .value = true}, NULL);
	moq_tokio_tls_Connect_host_name_set(tls, str("relay.example"), NULL);
	/* A `Vec<String>`, an array of buffers C keeps. */
	moq_buf pins[] = {str("ab:cd")};
	moq_tokio_tls_Connect_fingerprint_set(tls, (moq_vec_string){.ptr = pins, .len = 1}, NULL);
	moq_tokio_connect_Config_tls_set(connect, tls, NULL);
	moq_tokio_tls_Connect_free(tls);
	moq_tokio_connect_Config_implementation_set(connect, str("moq.c"), NULL);
	/* A URL, parsed from text, failing with why, and moved in. */
	moq_Url *url = NULL;
	moq_buf parse_err;
	CHECK(!moq_Url_parse(str("relay.example"), &url, &parse_err));
	CHECK(is(parse_err, "relative URL without a base"));
	CHECK(moq_Url_parse(str("https://relay.example:4443/demo"), &url, &parse_err));
	moq_tokio_connect_Config_url_set(connect, url, NULL);
	moq_Url_free(url);
	tls = moq_tokio_connect_Config_tls_get(connect, NULL);
	moq_option_bool insecure = moq_tokio_tls_Connect_insecure_get(tls, NULL);
	CHECK(insecure.some && insecure.value);
	CHECK(is(moq_tokio_tls_Connect_host_name_get(tls, NULL), "relay.example"));
	/* TLS files, as OS paths: none by default. */
	moq_vec_path roots = moq_tokio_tls_Connect_root_get(tls, NULL);
	moq_vec_string fingerprints = moq_tokio_tls_Connect_fingerprint_get(tls, NULL);
	CHECK(roots.ptr != NULL && roots.len == 0);
	CHECK(fingerprints.len == 1 && fingerprints.ptr[0].len == 5 && memcmp(fingerprints.ptr[0].ptr, "ab:cd", 5) == 0);
	moq_vec_path_free(roots);
	moq_vec_string_free(fingerprints);
	moq_tokio_tls_Connect_free(tls);
	CHECK(is(moq_tokio_connect_Config_implementation_get(connect, NULL), "moq.c"));
	url = moq_tokio_connect_Config_url_get(connect, NULL);
	CHECK(url != NULL && is(moq_Url_to_string(url, NULL), "https://relay.example:4443/demo"));
	/* The config's URL, dialed with no tokens. */
	moq_auth_Tokens *tokens = moq_auth_Tokens_new((moq_vec_auth_Token){0}, NULL);
	moq_Versions *versions = moq_Versions_default();
	moq_auth_Dial *dial = moq_auth_Tokens_dial(tokens, url, versions, NULL);
	moq_Url *dialed = moq_auth_Dial_url_get(dial, NULL);
	CHECK(moq_Url_eq(dialed, url, NULL));
	moq_Url_free(dialed);
	moq_auth_Dial_free(dial);
	moq_auth_Tokens_free(tokens);
	moq_Url_free(url);
	moq_tokio_connect_Config_free(connect);

	/* A backoff's fields are settings left unset, and the methods of their names resolve them. */
	moq_tokio_Backoff *backoff = moq_tokio_Backoff_default();
	CHECK(!moq_tokio_Backoff_initial_get(backoff, NULL).some);
	moq_duration initial = moq_tokio_Backoff_initial(backoff, NULL);
	CHECK(initial.secs == 1 && initial.nanos == 0);
	moq_tokio_Backoff_free(backoff);

	/* A `Vec<String>`, each element a buffer the array frees. */
	moq_vec_string alpns = moq_Versions_alpns(versions, NULL);
	CHECK(alpns.len > 0);
	for (size_t i = 0; i < alpns.len; i++) {
		CHECK(alpns.ptr[i].len > 0);
	}
	moq_vec_string_free(alpns);
	moq_Versions_free(versions);
	/* A socket address, crossing in a listener's bind, set on its config and read back. */
	moq_tokio_listen_Bind bind = {.tag = MOQ_TOKIO_LISTEN_BIND_ADDR, .addr = {._0 = {.ip = {[15] = 1}, .port = 4443, .v6 = true}}};
	CHECK(is(moq_tokio_listen_Bind_to_string(&bind), "[::1]:4443"));
	moq_tokio_listen_Bind v4 = {.tag = MOQ_TOKIO_LISTEN_BIND_ADDR, .addr = {._0 = {.ip = {127, 0, 0, 1}, .port = 0}}};
	CHECK(is(moq_tokio_listen_Bind_to_string(&v4), "127.0.0.1:0"));
	CHECK(!moq_tokio_listen_Bind_eq(&bind, &v4));
	moq_tokio_listen_Config *listen = moq_tokio_listen_Config_default();
	moq_tokio_listen_Config_bind_set(listen, (moq_option_tokio_listen_Bind){.some = true, .value = bind}, NULL);
	moq_option_tokio_listen_Bind bound = moq_tokio_listen_Config_bind_get(listen, NULL);
	CHECK(bound.some && moq_tokio_listen_Bind_eq(&bound.value, &bind));
	moq_tokio_listen_Bind_free(&bound.value);
	/* The preferred address of each family, which takes only that family. */
	moq_socket_addr preferred = {.ip = {10, 0, 0, 1}, .port = 4443};
	moq_tokio_listen_Config_preferred_v4_set(listen, (moq_option_socket_addr){.some = true, .value = preferred}, NULL);
	moq_option_socket_addr preferred_v4 = moq_tokio_listen_Config_preferred_v4_get(listen, NULL);
	CHECK(preferred_v4.some && !preferred_v4.value.v6 && preferred_v4.value.ip[0] == 10 && preferred_v4.value.port == 4443);
	CHECK(!moq_tokio_listen_Config_preferred_v6_get(listen, NULL).some);
	moq_tokio_listen_Config_free(listen);
	/* A constant slice, as an array of clones. */
	moq_vec_string alpns_all = moq_ALPNS();
	CHECK(alpns_all.len > 0);
	moq_vec_string_free(alpns_all);

	/* A session from C: a server binds QUIC on port 0, reads back the port it got, and
	 * serves an origin with one call, and a client dials it with one call and reads a frame
	 * back. Neither has a runtime of C's, so moq-tokio's fallback runs both. */
	moq_origin_Producer *served = moq_tokio_origin_spawn();
	moq_origin_Publication *live = NULL;
	CHECK(moq_origin_Producer_create_broadcast(served, str("live"), &live, &err, NULL));
	moq_origin_Route *route = moq_origin_Route_default();
	CHECK(moq_origin_Publication_announce(live, route, &err, NULL));
	moq_origin_Route_free(route);
	moq_broadcast_Producer *live_broadcast = moq_origin_Publication_broadcast(live);
	moq_track_Producer *live_track = NULL;
	CHECK(moq_broadcast_Producer_create_track(live_broadcast, str("video"), NULL, &live_track, &err, NULL));
	moq_group_Producer *live_group = NULL;
	CHECK(moq_track_Producer_append_group(live_track, &live_group, &err, NULL));
	CHECK(moq_group_Producer_write_frame(live_group, (moq_option_Timestamp){0}, str("hello"), &err, NULL));
	CHECK(moq_group_Producer_finish(live_group, &err, NULL));
	moq_group_Producer_free(live_group);

	moq_tokio_server_Config *server_config = moq_tokio_server_Config_default();
	moq_tokio_listen_Config *listen_config = moq_tokio_server_Config_listen_get(server_config, NULL);
	moq_tokio_listen_Bind any = {.tag = MOQ_TOKIO_LISTEN_BIND_ADDR, .addr = {._0 = {.port = 0, .v6 = true}}};
	moq_tokio_listen_Config_bind_set(listen_config, (moq_option_tokio_listen_Bind){.some = true, .value = any}, NULL);
	moq_tokio_tls_Listen *tls_listen = moq_tokio_tls_Listen_default();
	moq_buf names[] = {str("localhost")};
	moq_tokio_tls_Listen_generate_set(tls_listen, (moq_vec_string){.ptr = names, .len = 1}, NULL);
	moq_tokio_listen_Config_tls_set(listen_config, tls_listen, NULL);
	moq_tokio_tls_Listen_free(tls_listen);
	moq_tokio_server_Config_listen_set(server_config, listen_config, NULL);
	moq_tokio_listen_Config_free(listen_config);
	moq_tokio_Server *server = NULL;
	CHECK(moq_tokio_server_Config_init(server_config, &server, &tokio_err, NULL));
	moq_tokio_server_Config_free(server_config);
	moq_socket_addr local;
	CHECK(moq_tokio_Server_local_addr(server, &local, &tokio_err, NULL));
	CHECK(local.v6 && local.port != 0);
	moq_origin_Consumer *published = moq_origin_Producer_consume(served, NULL);
	serving = moq_tokio_Server_serve_publish(server, published, NULL);
	moq_tokio_Server_free(server);
	moq_origin_Consumer_free(published);

	char address[32];
	snprintf(address, sizeof address, "moqt://localhost:%u", (unsigned)local.port);
	moq_tokio_connect_Config *dialing = moq_tokio_connect_Config_default();
	CHECK(moq_Url_parse(str(address), &url, &parse_err));
	moq_tokio_connect_Config_url_set(dialing, url, NULL);
	moq_Url_free(url);
	tls = moq_tokio_connect_Config_tls_get(dialing, NULL);
	moq_tokio_tls_Connect_insecure_set(tls, (moq_option_bool){.some = true, .value = true}, NULL);
	moq_tokio_connect_Config_tls_set(dialing, tls, NULL);
	moq_tokio_tls_Connect_free(tls);
	moq_tokio_client_Config *session_client = moq_tokio_client_Config_default();
	moq_tokio_client_Config_connect_set(session_client, dialing, NULL);
	moq_tokio_connect_Config_free(dialing);
	moq_tokio_Client *dialer = NULL;
	CHECK(moq_tokio_client_Config_init(session_client, &dialer, &tokio_err, NULL));
	moq_tokio_client_Config_free(session_client);
	moq_tokio_Client *dialer_copy = moq_tokio_Client_clone(dialer, NULL);
	moq_tokio_Client *once = moq_tokio_Client_with_reconnect(dialer_copy, false, NULL);
	moq_tokio_Client_free(dialer_copy);
	moq_origin_Producer *received = moq_tokio_origin_spawn();
	moq_origin_Consumer *remote = moq_origin_Producer_consume(received, NULL);
	moq_tokio_Connection *connection = moq_tokio_Client_consume(once, received, NULL);
	moq_tokio_Client_free(once);
	moq_origin_Producer_free(received);
	CHECK(connection != NULL);

	moq_tokio_Connection_established_future *establishing = moq_tokio_Connection_established(connection, NULL);
	moq_tokio_Connection_free(connection);
	moq_tokio_Connection *established = NULL;
	AWAIT(moq_tokio_Connection_established_poll(establishing, session_waker, &established, &tokio_err));
	moq_tokio_Connection_established_future_free(establishing);
	CHECK(established != NULL);

	moq_origin_Consumer_request_future *requesting = moq_origin_Consumer_request(remote, str("live"), NULL, NULL);
	moq_broadcast_Consumer *remote_broadcast = NULL;
	AWAIT(moq_origin_Consumer_request_poll(requesting, session_waker, &remote_broadcast, &err));
	moq_origin_Consumer_request_future_free(requesting);
	CHECK(remote_broadcast != NULL);
	moq_track_Consumer *remote_track = NULL;
	CHECK(moq_broadcast_Consumer_track(remote_broadcast, str("video"), &remote_track, &err, NULL));
	subscribing = moq_track_Consumer_subscribe(remote_track, NULL, NULL);
	AWAIT(moq_track_Consumer_subscribe_poll(subscribing, session_waker, &subscriber, &err));
	moq_track_Consumer_subscribe_future_free(subscribing);
	CHECK(subscriber != NULL);
	receiving = moq_track_Subscriber_recv_group(subscriber, NULL);
	group = NULL;
	AWAIT(moq_track_Subscriber_recv_group_poll(receiving, session_waker, &group, &err));
	moq_track_Subscriber_recv_group_future_free(receiving);
	CHECK(group != NULL);
	reading = moq_group_Consumer_read_frame(group, NULL);
	frame = NULL;
	AWAIT(moq_group_Consumer_read_frame_poll(reading, session_waker, &frame, &err));
	moq_group_Consumer_read_frame_future_free(reading);
	CHECK(frame != NULL && is(moq_frame_Frame_payload_get(frame, NULL), "hello"));
	moq_frame_Frame_free(frame);
	moq_group_Consumer_free(group);
	moq_track_Subscriber_free(subscriber);
	moq_track_Consumer_free(remote_track);
	moq_broadcast_Consumer_free(remote_broadcast);

	moq_tokio_Connection_close_future *closing = moq_tokio_Connection_close(established, NULL);
	moq_tokio_Connection_free(established);
	AWAIT(moq_tokio_Connection_close_poll(closing, session_waker, &tokio_err));
	moq_tokio_Connection_close_future_free(closing);
	moq_tokio_Client_close_future *client_closing = moq_tokio_Client_close(dialer, NULL);
	moq_tokio_Client_free(dialer);
	AWAIT(moq_tokio_Client_close_poll(client_closing, session_waker));
	moq_tokio_Client_close_future_free(client_closing);
	moq_tokio_Server_serve_publish_future_free(serving);
	moq_origin_Consumer_free(remote);
	moq_track_Producer_free(live_track);
	moq_broadcast_Producer_free(live_broadcast);
	moq_origin_Publication_free(live);
	moq_origin_Producer_free(served);
	CHECK(tokio_err == NULL);

	if (failures == 0) {
		printf("ok\n");
	}
	return failures != 0;
}
