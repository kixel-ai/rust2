/* A Node-API addon over moq.h with the calls bench/ffi's cases make: what a generator
 * would emit for them, as thin as node:ffi's route in napi.ts gets. A handle crosses as
 * its address in a number, which every 64-bit platform's 48-bit user space keeps exact; a
 * failed check throws, naming the call. A wake on the loop's thread calls JavaScript's
 * wake at once, which queues the poll on a microtask; one from another thread goes through
 * a threadsafe function to the loop. */

#define NAPI_VERSION 9
#include <node_api.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "moq.h"

static napi_env main_env;
static pthread_t main_thread;
static napi_ref wake_fn;
static napi_threadsafe_function wake_tsfn;

/* Scratch the JavaScript views, as nf.c's: opaque values at AT, a copy of up to 64 bytes at
 * COPY, and a lent buffer's address and owner as doubles at LENT. */
static uint8_t *scratch;
enum { AT = 0, COPY = 64, LENT = 128 };

#define CHECK(call)                                                                                                   \
	do {                                                                                                          \
		if ((call) != napi_ok) {                                                                              \
			napi_throw_error(env, NULL, #call);                                                           \
			return NULL;                                                                                  \
		}                                                                                                     \
	} while (0)

static void *ptr_arg(napi_env env, napi_value v) {
	double d;
	napi_get_value_double(env, v, &d);
	return (void *)(uintptr_t)d;
}

static napi_value num(napi_env env, double d) {
	napi_value out;
	napi_create_double(env, d, &out);
	return out;
}

static napi_value ptr(napi_env env, const void *p) { return num(env, (double)(uintptr_t)p); }

static napi_value boolean(napi_env env, bool b) {
	napi_value out;
	napi_get_boolean(env, b, &out);
	return out;
}

static napi_value undef(napi_env env) {
	napi_value out;
	napi_get_undefined(env, &out);
	return out;
}

/* Throws the failed check's message if `status` holds one. */
static bool failed(napi_env env, const char *fn, moq_status status) {
	if (status == 0) return false;
	char msg[256];
	snprintf(msg, sizeof msg, "%s: %s", fn, moq_status_message(status));
	napi_throw_error(env, NULL, msg);
	return true;
}

static napi_value err_or(napi_env env, const char *fn, moq_Error *err) {
	if (err != NULL) moq_Error_free(err);
	napi_throw_error(env, NULL, fn);
	return NULL;
}

#define ARGS(n)                                                                                                       \
	size_t argc = n;                                                                                              \
	napi_value argv[n > 0 ? n : 1];                                                                               \
	napi_get_cb_info(env, cb, &argc, argv, NULL, NULL)

static napi_value is_closed(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	bool r = moq_track_Producer_is_closed(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Producer_is_closed", st)) return NULL;
	return boolean(env, r);
}

static napi_value demand_start(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	uint64_t r = moq_group_Demand_start(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_group_Demand_start", st)) return NULL;
	if (r <= (1ull << 53)) return num(env, (double)r);
	napi_value out;
	napi_create_bigint_uint64(env, r, &out);
	return out;
}

static napi_value largest(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	moq_option_track_Position r = moq_track_Producer_largest(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Producer_largest", st)) return NULL;
	memcpy(scratch + AT, &r.value, sizeof r.value);
	return boolean(env, r.some);
}

static napi_value info(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	moq_track_Info *r = moq_track_Producer_info(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Producer_info", st)) return NULL;
	return ptr(env, r);
}

static napi_value priority(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	int8_t r = moq_track_Info_priority_get(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Info_priority_get", st)) return NULL;
	return num(env, r);
}

static napi_value demand(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	moq_group_Demand *r = moq_group_Producer_demand(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_group_Producer_demand", st)) return NULL;
	return ptr(env, r);
}

static napi_value from_micros(napi_env env, napi_callback_info cb) {
	ARGS(1);
	int64_t micros;
	napi_get_value_int64(env, argv[0], &micros);
	moq_TimeOverflow overflow;
	if (!moq_Timestamp_from_micros((uint64_t)micros, (moq_Timestamp *)(scratch + AT), &overflow)) return err_or(env, "from_micros", NULL);
	return undef(env);
}

static napi_value append_group(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_Timestamp at;
	memcpy(&at, scratch + AT, sizeof at);
	moq_group_Producer *out = NULL;
	moq_Error *err = NULL;
	moq_status st = 0;
	bool ok = moq_track_Producer_append_group(ptr_arg(env, argv[0]), at, &out, &err, &st);
	if (failed(env, "moq_track_Producer_append_group", st)) return NULL;
	if (!ok) return err_or(env, "append_group", err);
	return ptr(env, out);
}

static napi_value write_frame(napi_env env, napi_callback_info cb) {
	ARGS(2);
	moq_Timestamp at;
	memcpy(&at, scratch + AT, sizeof at);
	void *data;
	size_t len;
	napi_get_typedarray_info(env, argv[1], NULL, &len, &data, NULL, NULL);
	moq_Error *err = NULL;
	moq_status st = 0;
	bool ok = moq_group_Producer_write_frame(ptr_arg(env, argv[0]), at, (moq_buf){.ptr = data, .len = len}, &err, &st);
	if (failed(env, "moq_group_Producer_write_frame", st)) return NULL;
	if (!ok) return err_or(env, "write_frame", err);
	return undef(env);
}

static napi_value finish(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_group_Finished *out = NULL;
	moq_Error *err = NULL;
	moq_status st = 0;
	bool ok = moq_group_Producer_finish(ptr_arg(env, argv[0]), &out, &err, &st);
	if (failed(env, "moq_group_Producer_finish", st)) return NULL;
	if (!ok) return err_or(env, "finish", err);
	return ptr(env, out);
}

static napi_value create_track(napi_env env, napi_callback_info cb) {
	ARGS(2);
	char name[256];
	size_t len;
	napi_get_value_string_utf8(env, argv[1], name, sizeof name, &len);
	moq_option_Timescale micro = {.some = true, .value = moq_Timescale_MICRO()};
	moq_track_Info *defaults = moq_track_Info_default();
	moq_track_Info *info = moq_track_Info_with_timescale(defaults, micro, NULL);
	moq_track_Info_free(defaults);
	moq_track_Producer *out = NULL;
	moq_Error *err = NULL;
	moq_status st = 0;
	bool ok = moq_broadcast_Producer_create_track(ptr_arg(env, argv[0]), (moq_buf){.ptr = (uint8_t *)name, .len = len}, info, &out, &err, &st);
	moq_track_Info_free(info);
	if (failed(env, "moq_broadcast_Producer_create_track", st)) return NULL;
	if (!ok) return err_or(env, "create_track", err);
	return ptr(env, out);
}

static napi_value broadcast(napi_env env, napi_callback_info cb) {
	moq_broadcast_Info *info = moq_broadcast_Info_new();
	moq_broadcast_Producer *r = moq_broadcast_Info_produce(info, NULL);
	moq_broadcast_Info_free(info);
	return ptr(env, r);
}

static napi_value consume(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	moq_track_Consumer *r = moq_track_Producer_consume(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Producer_consume", st)) return NULL;
	return ptr(env, r);
}

/* Bytes Rust returned: up to 64 copied to COPY and freed, as the web binding holds them,
 * and past it lent, its address and owner at LENT, read through `byte_at` and `buf_copy`
 * until `buf_free`. Returns their length. */
static napi_value payload(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	moq_buf buf = moq_frame_Frame_payload_get(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_frame_Frame_payload_get", st)) return NULL;
	if (buf.len <= 64) {
		memcpy(scratch + COPY, buf.ptr, buf.len);
		moq_buf_free(buf);
	} else {
		double lent[2] = {(double)(uintptr_t)buf.ptr, (double)(uintptr_t)buf.owner};
		memcpy(scratch + LENT, lent, sizeof lent);
	}
	return num(env, (double)buf.len);
}

static napi_value byte_at(napi_env env, napi_callback_info cb) {
	ARGS(2);
	uint32_t i;
	napi_get_value_uint32(env, argv[1], &i);
	return num(env, ((const uint8_t *)ptr_arg(env, argv[0]))[i]);
}

/* Copies a lent buffer's bytes into a typed array of its length. */
static napi_value buf_copy(napi_env env, napi_callback_info cb) {
	ARGS(2);
	void *data;
	size_t len;
	napi_get_typedarray_info(env, argv[1], NULL, &len, &data, NULL, NULL);
	memcpy(data, ptr_arg(env, argv[0]), len);
	return undef(env);
}

static napi_value buf_free(napi_env env, napi_callback_info cb) {
	ARGS(3);
	double len;
	napi_get_value_double(env, argv[1], &len);
	moq_buf_free((moq_buf){.ptr = ptr_arg(env, argv[0]), .len = (size_t)len, .owner = ptr_arg(env, argv[2])});
	return undef(env);
}

static void wake(void *data) {
	if (pthread_equal(pthread_self(), main_thread)) {
		napi_env env = main_env;
		napi_value fn, global, arg;
		napi_get_reference_value(env, wake_fn, &fn);
		napi_get_global(env, &global);
		napi_create_double(env, (double)(uintptr_t)data, &arg);
		napi_call_function(env, global, fn, 1, &arg, NULL);
	} else {
		napi_call_threadsafe_function(wake_tsfn, data, napi_tsfn_nonblocking);
	}
}

static void wake_on_loop(napi_env env, napi_value fn, void *context, void *data) {
	napi_value global, arg;
	napi_get_global(env, &global);
	napi_create_double(env, (double)(uintptr_t)data, &arg);
	napi_call_function(env, global, fn, 1, &arg, NULL);
}

static void drop_waker(void *data) {}

static moq_waker waker_arg(napi_env env, napi_value v) {
	return (moq_waker){.wake = wake, .drop = drop_waker, .data = ptr_arg(env, v)};
}

/* A ready result's address, or a pending future's negated. */
static napi_value read_frame(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_group_Consumer_read_frame_future *future = NULL;
	moq_frame_Frame *out = NULL;
	moq_Error *err = NULL;
	moq_status st = 0;
	moq_poll p = moq_group_Consumer_read_frame_start(ptr_arg(env, argv[0]), &out, &err, &future, &st);
	if (failed(env, "moq_group_Consumer_read_frame_start", st)) return NULL;
	if (p == MOQ_POLL_ERROR || (p == MOQ_POLL_READY && out == NULL)) return err_or(env, "read_frame", err);
	return p == MOQ_POLL_READY ? ptr(env, out) : num(env, -(double)(uintptr_t)future);
}

#define POLL(name, future_t, out_t, poll_fn, free_fn)                                                                 \
	static napi_value name(napi_env env, napi_callback_info cb) {                                                 \
		ARGS(2);                                                                                              \
		future_t *future = ptr_arg(env, argv[0]);                                                             \
		out_t *out = NULL;                                                                                    \
		moq_Error *err = NULL;                                                                                \
		moq_poll p = poll_fn(future, waker_arg(env, argv[1]), &out, &err);                                    \
		if (p == MOQ_POLL_PENDING) return num(env, 0);                                                        \
		free_fn(future);                                                                                      \
		if (p == MOQ_POLL_ERROR || out == NULL) return err_or(env, #poll_fn, err);                            \
		return ptr(env, out);                                                                                 \
	}

POLL(read_frame_poll, moq_group_Consumer_read_frame_future, moq_frame_Frame, moq_group_Consumer_read_frame_poll, moq_group_Consumer_read_frame_future_free)
POLL(recv_group_poll, moq_track_Subscriber_recv_group_future, moq_group_Consumer, moq_track_Subscriber_recv_group_poll, moq_track_Subscriber_recv_group_future_free)
POLL(subscribe_poll, moq_track_Consumer_subscribe_future, moq_track_Subscriber, moq_track_Consumer_subscribe_poll, moq_track_Consumer_subscribe_future_free)

static napi_value recv_group(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	void *r = moq_track_Subscriber_recv_group(ptr_arg(env, argv[0]), &st);
	if (failed(env, "moq_track_Subscriber_recv_group", st)) return NULL;
	return ptr(env, r);
}

static napi_value subscribe(napi_env env, napi_callback_info cb) {
	ARGS(1);
	moq_status st = 0;
	void *r = moq_track_Consumer_subscribe(ptr_arg(env, argv[0]), NULL, &st);
	if (failed(env, "moq_track_Consumer_subscribe", st)) return NULL;
	return ptr(env, r);
}

#define FREE(name, fn)                                                                                                \
	static napi_value name(napi_env env, napi_callback_info cb) {                                                 \
		ARGS(1);                                                                                              \
		fn(ptr_arg(env, argv[0]));                                                                            \
		return NULL;                                                                                          \
	}

FREE(broadcast_free, moq_broadcast_Producer_free)
FREE(track_free, moq_track_Producer_free)
FREE(consumer_free, moq_track_Consumer_free)
FREE(subscriber_free, moq_track_Subscriber_free)
FREE(info_free, moq_track_Info_free)
FREE(demand_free, moq_group_Demand_free)
FREE(group_free, moq_group_Producer_free)
FREE(finished_free, moq_group_Finished_free)
FREE(reader_free, moq_group_Consumer_free)
FREE(frame_free, moq_frame_Frame_free)

static napi_value set_scratch(napi_env env, napi_callback_info cb) {
	ARGS(1);
	void *data;
	CHECK(napi_get_typedarray_info(env, argv[0], NULL, NULL, &data, NULL, NULL));
	scratch = data;
	return undef(env);
}

static napi_value set_wake(napi_env env, napi_callback_info cb) {
	ARGS(1);
	CHECK(napi_create_reference(env, argv[0], 1, &wake_fn));
	napi_value name;
	napi_create_string_utf8(env, "wake", NAPI_AUTO_LENGTH, &name);
	CHECK(napi_create_threadsafe_function(env, argv[0], NULL, name, 0, 1, NULL, NULL, NULL, wake_on_loop, &wake_tsfn));
	/* A wake pending from another thread need not keep the process alive. */
	CHECK(napi_unref_threadsafe_function(env, wake_tsfn));
	return undef(env);
}

NAPI_MODULE_INIT() {
	main_env = env;
	main_thread = pthread_self();
	if (moq_contract() == 0) return NULL;
#define F(name) {#name, 0, name, 0, 0, 0, napi_default, 0}
	napi_property_descriptor d[] = {
		F(is_closed), F(demand_start), F(largest), F(info), F(priority), F(demand), F(from_micros),
		F(append_group), F(write_frame), F(finish), F(create_track), F(broadcast), F(consume),
		F(payload), F(byte_at), F(buf_copy), F(buf_free), F(set_scratch), F(read_frame), F(read_frame_poll), F(recv_group), F(recv_group_poll),
		F(subscribe), F(subscribe_poll), F(broadcast_free), F(track_free), F(consumer_free),
		F(subscriber_free), F(info_free), F(demand_free), F(group_free), F(finished_free), F(reader_free),
		F(frame_free), F(set_wake),
	};
	napi_define_properties(env, exports, sizeof d / sizeof d[0], d);
	return exports;
}
