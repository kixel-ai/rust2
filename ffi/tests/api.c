/* The api fixture's C test: the script fixtures/api/examples/transcript.rs runs
 * natively, through the exported C ABI. tests/fixtures.rs compares the transcripts.
 *
 * The script is transcript.c, which tests/fixtures/transcript generates from the
 * transcript's; this file holds the functions it calls by the transcript's names, and
 * the checks its output does not show.
 *
 * With an argument, it instead breaks one contract of the ABI, which must abort. */

#include <math.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef __linux__
#include <sys/prctl.h>
#endif

#include "api.h"

static api_buf str(const char *s) {
	return (api_buf){.ptr = (const uint8_t *)s, .len = strlen(s)};
}

/* Print a returned buffer, then free it. */
static void print(const char *before, api_buf buf, const char *after) {
	printf("%s%.*s%s", before, (int)buf.len, (const char *)buf.ptr, after);
	api_buf_free(buf);
}

/* Drops a must-use result on purpose: before C23, GCC warns of one even cast to void. */
#define DISCARD(call) ((void)!(call))

static int wakes = 0;
static int drops = 0;

static void wake(void *data) {
	(void)data;
	wakes++;
}

static void drop(void *data) {
	(void)data;
	drops++;
}

/* A future C implements: ready on its `ready`th poll, holding the waker of the last. */
typedef struct {
	int ready;
	int polls;
	bool freed;
	api_waker waker;
} foreign;

static void foreign_drop(foreign *f) {
	if (f->waker.drop) {
		f->waker.drop(f->waker.data);
	}
	f->waker = (api_waker){0};
}

static bool foreign_poll(void *data, api_waker waker) {
	foreign *f = data;
	foreign_drop(f);
	if (++f->polls >= f->ready) {
		waker.drop(waker.data);
		return true;
	}
	f->waker = waker;
	return false;
}

static void foreign_free(void *data) {
	foreign *f = data;
	foreign_drop(f);
	f->freed = true;
}

static api_foreign_future foreign_future(foreign *f) {
	return (api_foreign_future){.poll = foreign_poll, .free = foreign_free, .data = f};
}

/* Futures C implements, which Rust polls: whether each behaved. */
static bool foreign_futures(void) {
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	uint32_t out = 0;

	/* Ready at once. */
	foreign now = {.ready = 1};
	api_after_future *after = api_after(foreign_future(&now), 4);
	bool ok = api_after_poll(after, waker, &out) == API_POLL_READY && out == 8 && now.polls == 1;
	api_after_future_free(after);
	ok = ok && now.freed;

	/* Pending once: its wake reaches the waker Rust was polled with. */
	foreign later = {.ready = 2};
	after = api_after(foreign_future(&later), 5);
	int woken = wakes;
	ok = ok && api_after_poll(after, waker, &out) == API_POLL_PENDING && later.waker.wake;
	later.waker.wake(later.waker.data);
	ok = ok && wakes == woken + 1;
	ok = ok && api_after_poll(after, waker, &out) == API_POLL_READY && out == 10 && !later.waker.wake;
	api_after_future_free(after);
	ok = ok && later.freed;

	/* Never ready: Rust gives up on it, freeing it and the waker it held once it finishes. */
	foreign never = {.ready = 1000};
	api_within_future *within = api_within(0, foreign_future(&never));
	api_Error err;
	ok = ok && api_within_poll(within, waker, &err) == API_POLL_ERROR && err.tag == API_ERROR_OVERFLOW;
	api_Error_free(&err);
	ok = ok && never.polls == 1 && never.freed && !never.waker.wake;
	api_within_future_free(within);

	/* Ready in time, by a second poll. */
	foreign twice = {.ready = 2};
	within = api_within(1, foreign_future(&twice));
	ok = ok && api_within_poll(within, waker, &err) == API_POLL_PENDING;
	ok = ok && api_within_poll(within, waker, &err) == API_POLL_READY && twice.polls == 2;
	api_within_future_free(within);
	ok = ok && twice.freed;

	/* A poll of a waiter, made of each poll's waker, of a future C implements, which the
	 * future owns and lends it pinned: freed once it finishes. */
	foreign lent = {.ready = 2};
	api_Waiter_poll_future_future *polled = api_Waiter_poll_future(foreign_future(&lent));
	ok = ok && api_Waiter_poll_future_poll(polled, waker) == API_POLL_PENDING && lent.waker.wake;
	woken = wakes;
	lent.waker.wake(lent.waker.data);
	ok = ok && wakes == woken + 1 && !lent.freed;
	ok = ok && api_Waiter_poll_future_poll(polled, waker) == API_POLL_READY && lent.polls == 2 && lent.freed;
	api_Waiter_poll_future_future_free(polled);

	/* Dropped before it finishes, it frees the future it owns. */
	foreign dropped = {.ready = 1000};
	polled = api_Waiter_poll_future(foreign_future(&dropped));
	ok = ok && api_Waiter_poll_future_poll(polled, waker) == API_POLL_PENDING;
	api_Waiter_poll_future_future_free(polled);
	return ok && dropped.freed && !dropped.waker.wake;
}

/* Futures started, made and polled once in one call: whether each behaved. */
static bool started_futures(void) {
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	uint32_t out = 0;

	/* Ready at once: the result is written, the future freed, and the handle nulled
	 * over what the caller left in it. */
	foreign now = {.ready = 1};
	api_after_future *after = (api_after_future *)&out;
	bool ok = api_after_start(foreign_future(&now), 4, &out, &after) == API_POLL_READY;
	ok = ok && out == 8 && !after && now.polls == 1 && now.freed;

	/* Pending: the handle is returned, and a poll with a waker replaces the first's,
	 * which wakes nothing. */
	foreign later = {.ready = 3};
	ok = ok && api_after_start(foreign_future(&later), 5, &out, &after) == API_POLL_PENDING && after;
	later.waker.wake(later.waker.data);
	int woken = wakes;
	ok = ok && api_after_poll(after, waker, &out) == API_POLL_PENDING && later.polls == 2;
	later.waker.wake(later.waker.data);
	ok = ok && wakes == woken + 1;
	ok = ok && api_after_poll(after, waker, &out) == API_POLL_READY && out == 10;
	api_after_future_free(after);
	ok = ok && later.freed;

	/* Pending, then freed unfinished: the future it owns is freed with it. */
	foreign dropped = {.ready = 1000};
	after = NULL;
	ok = ok && api_after_start(foreign_future(&dropped), 6, &out, &after) == API_POLL_PENDING;
	api_after_future_free(after);
	ok = ok && dropped.freed && !dropped.waker.wake;

	/* Failed at once: the error is written, and the handle nulled. */
	foreign never = {.ready = 1000};
	api_within_future *within = (api_within_future *)&out;
	api_Error err;
	ok = ok && api_within_start(0, foreign_future(&never), &err, &within) == API_POLL_ERROR;
	ok = ok && err.tag == API_ERROR_OVERFLOW && !within && never.freed;
	api_Error_free(&err);
	return ok;
}


static uint32_t spawned_task(void *data) {
	(void)data;
	return 7;
}

static void spawned_free(void *data) {
	*(bool *)data = true;
}

/* made's closure, returning a view of bytes Rust does not own. */
static api_buf made_view(void *data, uint32_t n) {
	(void)data;
	(void)n;
	return str("viewed");
}

/* drive's closure, as the transcript's: asks for a later moment, then waits for its
 * waiter, which it wakes, then stops. Rust frees it once it drops it. */
typedef struct {
	int calls;
	api_Moment start;
	bool freed;
} driver;

static bool drive_poll(void *data, api_Moment now, const api_Waiter *waiter, api_option_Moment *out, api_buf *err) {
	driver *d = data;
	d->calls++;
	if (d->calls == 1) {
		*out = (api_option_Moment){.some = true, .value = api_Moment_after(&now, 5)};
		return true;
	}
	if (d->calls == 2) {
		printf("drive %llu\n", (unsigned long long)(api_Moment_millis(&now) - api_Moment_millis(&d->start)));
		api_waker returned = api_Waiter_waker(waiter, NULL);
		returned.wake(returned.data);
		returned.drop(returned.data);
		*out = (api_option_Moment){0};
		return true;
	}
	char text[32];
	snprintf(text, sizeof text, "stopped after %d", d->calls);
	*err = api_buf_copy(str(text));
	return false;
}

/* A drive that only ever waits. */
static bool drive_idle(void *data, api_Moment now, const api_Waiter *waiter, api_option_Moment *out, api_buf *err) {
	(void)data;
	(void)now;
	(void)waiter;
	(void)err;
	*out = (api_option_Moment){0};
	return true;
}

static void drive_free(void *data) {
	driver *d = data;
	d->freed = true;
}

/* A reader of bytes in memory. */
typedef struct {
	const char *bytes;
	size_t len;
	size_t at;
} memory;

static bool memory_read(void *data, uint8_t *buf, size_t len, size_t *n, api_buf *err) {
	memory *m = data;
	(void)err;
	size_t left = m->len - m->at;
	*n = len < left ? len : left;
	if (*n > 0) {
		memcpy(buf, m->bytes + m->at, *n);
	}
	m->at += *n;
	return true;
}

static bool memory_seek(void *data, int whence, int64_t offset, uint64_t *position, api_buf *err) {
	memory *m = data;
	if (whence == SEEK_SET) {
		/* Anywhere from the start, as Rust's `Cursor` lands. */
		m->at = (size_t)(uint64_t)offset;
		*position = m->at;
		return true;
	}
	int64_t base = whence == SEEK_CUR ? (int64_t)m->at : (int64_t)m->len;
	if (base + offset < 0) {
		*err = api_buf_copy(str("before the start"));
		return false;
	}
	m->at = (size_t)(base + offset);
	*position = m->at;
	return true;
}

/* A reader whose reads fail, as the transcript's: each seek lands at 0. */
static bool broken_read(void *data, uint8_t *buf, size_t len, size_t *n, api_buf *err) {
	(void)data;
	(void)buf;
	(void)len;
	(void)n;
	*err = api_buf_copy(str("broken"));
	return false;
}

static bool broken_seek(void *data, int whence, int64_t offset, uint64_t *position, api_buf *err) {
	(void)data;
	(void)whence;
	(void)offset;
	(void)err;
	*position = 0;
	return true;
}

/* The payload sent last, ready at the first poll. */
static api_buf received(api_Channel *channel) {
	api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
	api_buf payload = {0};
	bool ready = api_Channel_recv_poll(recv, (api_waker){0}, &payload) == API_POLL_READY;
	api_Channel_recv_future_free(recv);
	return ready ? payload : (api_buf){0};
}

/* A payload Rust returned goes back to Rust in place: taken whole, taken narrowed to a
 * range of it, shared by a second owner while kept, and carried in a record. Its bytes
 * stay where Rust put them, and drop once, with its last owner. */
static bool forwarding(void) {
	uint32_t before = api_dropped();
	api_Channel *channel = api_Channel_new();
	api_buf payload = api_token_bytes(3, 64);
	const uint8_t *at = payload.ptr;
	api_Channel_send(channel, payload, NULL);
	payload = received(channel);
	bool same = payload.ptr == at && payload.len == 64;
	api_Channel_send(channel, (api_buf){.ptr = payload.ptr + 1, .len = 62, .owner = payload.owner}, NULL);
	payload = received(channel);
	same = same && payload.ptr == at + 1 && payload.len == 62;
	api_Channel_send(channel, api_buf_clone(payload), NULL);
	api_buf shared = received(channel);
	same = same && shared.ptr == payload.ptr && shared.len == 62;
	api_buf_free(payload);
	api_Packet packet = api_next_packet((api_Packet){.seq = 1, .payload = shared});
	same = same && packet.payload.ptr == at + 1 && api_dropped() == before;
	api_Packet_free(&packet);
	api_Channel_free(channel);
	return same && api_dropped() == before + 1;
}

static void tail(api_foreign_reader reader) {
	api_buf bytes;
	uint64_t at;
	api_buf err;
	if (api_tail(reader, 5, &bytes, &at, &err)) {
		print("tail ", bytes, "");
		printf(" %llu\n", (unsigned long long)at);
	} else {
		print("tail error ", err, "\n");
	}
}


/* The Log callback: keeps even levels. */
static bool log_line(void *data, uint8_t level, api_buf line) {
	(void)data;
	printf("log %u %.*s\n", level, (int)line.len, (const char *)line.ptr);
	return level % 2 == 0;
}

static void log_free(void *data) {
	(void)data;
	printf("freed\n");
}


/* Breaks a lent handle's contract: frees it, or moves its value out. */
static bool free_lent(void *data, api_Counter *counter) {
	(void)data;
	api_Counter_free(counter);
	return false;
}

static bool move_lent(void *data, api_Counter *counter) {
	(void)data;
	api_Label label = api_Counter_into_label(counter, (api_option_Point){0}, NULL);
	api_Label_free(&label);
	return false;
}


/* The transcript's own functions, which the generated script calls by the names
 * transcript.rs gives them, defined after it, beside the helpers it defines. */
static api_Log Print(void);
static void channel(void);
static void drive(void);
static void readers(void);
static void streams(void);
static void local_later(const api_Local *local);
static api_buf hex(api_buf bytes);
static api_buf escaped(api_buf path);
static api_buf path_hex(api_buf path);
static api_buf seconds(api_duration d);
static uint64_t millis(api_duration d);
static uint32_t code(uint32_t c);
static api_vec_tls_Codec reversed(api_vec_tls_Codec list);
static api_vec_u64 f64_bits(api_vec_f64 floats);
static api_buf socket_hex(api_socket_addr addr);
static api_buf ip_hex(api_ip_addr ip);
static void numbers(api_buf what, api_Numbers n);
static api_buf unspelled(void);

#include "transcript.c"

/* Each string a function of the transcript's returns is the script's to free. */

static api_buf copied(const char *text, size_t len) {
	return api_buf_copy((api_buf){.ptr = (const uint8_t *)text, .len = len});
}

/* Print's Log: keeps even levels, and says when Rust frees it. */
static api_Log Print(void) {
	static const api_Log_vtable vtable = {.free = log_free, .log = log_line};
	return (api_Log){.vtable = &vtable};
}

/* A channel's futures, polled by hand. */
static void channel(void) {
	api_Channel *channel = api_Channel_new();
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	api_Error err;
	api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
	api_buf payload;
	printf("recv pending %s\n", api_Channel_recv_poll(recv, waker, &payload) == API_POLL_PENDING ? "true" : "false");
	api_Channel_send(channel, str("hi"), NULL);
	printf("woken %d\n", wakes);
	if (api_Channel_recv_poll(recv, waker, &payload) == API_POLL_READY) {
		print("recv ", payload, "\n");
	}
	api_Channel_recv_future_free(recv);
	api_Channel_send(channel, str(""), NULL);
	api_Channel_recv_nonempty_future *nonempty = api_Channel_recv_nonempty(channel, NULL);
	switch (api_Channel_recv_nonempty_poll(nonempty, waker, &payload, &err)) {
	case API_POLL_READY:
		printf("nonempty %zu\n", payload.len);
		api_buf_free(payload);
		break;
	case API_POLL_ERROR:
		print("nonempty err ", api_Error_to_string(&err), "\n");
		api_Error_free(&err);
		break;
	default:
		printf("nonempty pending\n");
	}
	api_Channel_recv_nonempty_future_free(nonempty);
	api_Batch *fill4 = api_Batch_new();
	api_Batch_2 *fill2 = api_pair();
	api_fill_4_future *filling4 = api_fill_4(fill4, 9, NULL);
	api_fill_2_future *filling2 = api_fill_2(fill2, 1, NULL);
	api_vec_u32 filled4, filled2;
	if (api_fill_4_poll(filling4, waker, &filled4) == API_POLL_READY && api_fill_2_poll(filling2, waker, &filled2) == API_POLL_READY) {
		printf("fill");
		for (size_t i = 0; i < filled4.len; i++) {
			printf(" %u", (unsigned)filled4.ptr[i]);
		}
		for (size_t i = 0; i < filled2.len; i++) {
			printf(" %u", (unsigned)filled2.ptr[i]);
		}
		printf("\n");
		api_vec_u32_free(filled4);
		api_vec_u32_free(filled2);
	}
	api_fill_4_future_free(filling4);
	api_fill_2_future_free(filling2);
	api_Batch_2_free(fill2);
	api_Batch_free(fill4);

	/* The send happens at the call: the future is dropped unpolled. */
	api_Channel_send_later_future_free(api_Channel_send_later(channel, str("eager"), NULL));
	recv = api_Channel_recv(channel, NULL);
	if (api_Channel_recv_poll(recv, waker, &payload) == API_POLL_READY) {
		print("recv ", payload, "\n");
	}
	api_Channel_recv_future_free(recv);
	/* The send happens at the call, and the future borrows the channel and the tag. */
	api_Channel_tagged_future *tagging = api_Channel_tagged(channel, str("tag"), NULL);
	api_buf tagged_text;
	if (api_Channel_tagged_poll(tagging, waker, &tagged_text) == API_POLL_READY) {
		print("tagged ", tagged_text, "\n");
	}
	api_Channel_tagged_future_free(tagging);
	/* So does a future borrowing an optional slice, which the export owns while it is pending. */
	const uint32_t items[] = {1, 2, 3};
	api_Channel_itemized_future *itemizing = api_Channel_itemized(channel, (api_vec_u32){.ptr = items, .len = 3}, NULL);
	size_t itemized;
	if (api_Channel_itemized_poll(itemizing, waker, &itemized) == API_POLL_READY) {
		printf("itemized %zu\n", itemized);
	}
	api_Channel_itemized_future_free(itemizing);
	/* Another crate's future borrowing its bytes, which the export owns while it is pending. */
	api_byte_len_future *measuring = api_byte_len(str("abc"));
	size_t byte_len;
	if (api_byte_len_poll(measuring, waker, &byte_len) == API_POLL_READY) {
		printf("byte_len %zu\n", byte_len);
	}
	api_byte_len_future_free(measuring);
	api_Channel_send(channel, str("sized"), NULL);
	api_Channel_recv_sized_future *sized = api_Channel_recv_sized(channel, NULL);
	size_t len;
	if (api_Channel_recv_sized_poll(sized, waker, &payload, &len) == API_POLL_READY) {
		print("sized ", payload, " ");
		printf("%zu\n", len);
	}
	api_Channel_recv_sized_future_free(sized);
	/* The waker moves into Rust, which drops it once it wakes. */
	api_Channel_notify(channel, waker, NULL);
	api_Channel_send(channel, str("rung"), NULL);
	printf("woken %d\n", wakes);
	/* A poll with no async twin is a future: it takes what the last send left. */
	api_Channel_poll_take_future *take = api_Channel_poll_take(channel, NULL);
	if (api_Channel_poll_take_poll(take, waker, &payload) == API_POLL_READY) {
		print("take ", payload, "\n");
	}
	api_Channel_poll_take_future_free(take);
	api_Channel_send(channel, str("measured"), NULL);
	api_Channel_poll_measure_future *measure = api_Channel_poll_measure(channel, 2, NULL);
	if (api_Channel_poll_measure_poll(measure, waker, &len, &err) == API_POLL_READY) {
		printf("measure %zu\n", len);
	}
	api_Channel_poll_measure_future_free(measure);
	/* A context is made of the waker, which Rust keeps until the send wakes it. */
	api_Channel_listen(channel, waker, NULL);
	api_Channel_send(channel, str("heard"), NULL);
	printf("woken %d\n", wakes);
	/* A returned waker is the caller's: it wakes the waiter's task until dropped. */
	api_Waiter *waiter = api_Waiter_new(waker);
	api_waker returned = api_Waiter_waker(waiter, NULL);
	returned.wake(returned.data);
	returned.drop(returned.data);
	returned = api_Waiter_context(waiter, NULL);
	returned.wake(returned.data);
	printf("woken %d\n", wakes);

	/* Passed back, it moves into Rust as any waker does. */
	int woken = wakes;
	api_Channel_notify(channel, returned, NULL);
	api_Waiter_free(waiter);
	api_Channel_send(channel, str("again"), NULL);
	if (wakes != woken + 1) {
		tr_fail("a waker passed back did not wake");
	}
	api_Channel_clear(channel, NULL);
	/* The waiter a pending poll registers lives until the next poll, so the send wakes it. */
	take = api_Channel_poll_take(channel, NULL);
	if (api_Channel_poll_take_poll(take, waker, &payload) != API_POLL_PENDING) {
		tr_fail("a poll_take of an empty channel was ready");
	}
	woken = wakes;
	api_Channel_send(channel, str("later"), NULL);
	if (wakes != woken + 1 || api_Channel_poll_take_poll(take, waker, &payload) != API_POLL_READY) {
		tr_fail("a pending poll_take's waiter did not wake");
	}
	api_buf_free(payload);
	api_Channel_poll_take_future_free(take);
	api_Channel_free(channel);
	/* Each future dropped its waker once it finished, and Rust the one it was given. */
	if (drops != 15) {
		tr_fail("a waker was not dropped");
	}
}

/* A kept closure: called again at the instant it asks for, then once its waiter wakes. */
static void drive(void) {
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	api_Moment start = api_Moment_new(0, 1000).value;
	driver d = {.start = start};
	api_drive_future *drive = api_drive(start, drive_poll, &d, drive_free);
	api_buf stopped;
	int woken = wakes;
	while (api_drive_poll(drive, waker, &stopped) == API_POLL_PENDING) {
	}
	print("drive ", stopped, "\n");
	if (!d.freed || wakes != woken + 1) {
		tr_fail("drive kept its closure, or its waiter did not wake");
	}
	api_drive_future_free(drive);
}

/* Readers C implements: one over bytes, one whose reads fail, and a seek from the start
 * past INT64_MAX, which reaches the reader whole, as `Cursor` takes one. */
static void readers(void) {
	memory hello = {.bytes = "hello world", .len = 11};
	tail((api_foreign_reader){.read = memory_read, .seek = memory_seek, .data = &hello});
	tail((api_foreign_reader){.read = broken_read, .seek = broken_seek, .data = NULL});
	memory empty = {.bytes = "", .len = 0};
	uint64_t landed = 0;
	api_buf landed_err;
	if (api_seek_to((api_foreign_reader){.read = memory_read, .seek = memory_seek, .data = &empty}, UINT64_MAX, &landed,
			&landed_err)) {
		printf("sought %llu\n", (unsigned long long)landed);
	} else {
		print("sought error ", landed_err, "\n");
	}
}

/* A countdown's next count, polled until ready: false once it ends. */
static bool countdown_next(api_Countdown *countdown, uint32_t *count) {
	api_Countdown_next_future *next = api_Countdown_next(countdown, NULL);
	api_option_u32 out;
	while (api_Countdown_next_poll(next, (api_waker){0}, &out) == API_POLL_PENDING) {
	}
	api_Countdown_next_future_free(next);
	*count = out.value;
	return out.some;
}

/* Streams, each next polled in turn, as C has no iteration of its own. */
static void streams(void) {
	uint32_t count;
	api_Countdown *countdown = api_Countdown_new(3);
	printf("countdown");
	while (countdown_next(countdown, &count)) {
		printf(" %u", (unsigned)count);
	}
	printf("\n");
	api_Countdown_free(countdown);
	countdown = api_Countdown_new(5);
	while (countdown_next(countdown, &count) && count != 4) {
	}
	countdown_next(countdown, &count);
	printf("countdown after break %u\n", (unsigned)count);
	api_Countdown_free(countdown);
	api_Channel *channel = api_Channel_new();
	api_Feed *feed = api_Channel_feed(channel, NULL);
	const char *firsts[2] = {"a", "c"};
	for (int i = 0; i < 2; i++) {
		api_Channel_send(channel, str(firsts[i]), NULL);
		for (;;) {
			api_Feed_next_future *next = api_Feed_next(feed, NULL);
			api_buf payload;
			api_Error err;
			api_poll poll;
			while ((poll = api_Feed_next_poll(next, (api_waker){0}, &payload, &err)) == API_POLL_PENDING) {
			}
			api_Feed_next_future_free(next);
			if (poll == API_POLL_ERROR) {
				print("feed error ", api_Error_to_string(&err), "\n");
				api_Error_free(&err);
				break;
			}
			if (payload.ptr == NULL) {
				printf("feed end\n");
				break;
			}
			char first = payload.len == 1 ? (char)payload.ptr[0] : 0;
			print("feed ", payload, "\n");
			api_Channel_send(channel, str(first == 'a' ? "b" : first == 'b' ? "!" : ""), NULL);
		}
	}
	api_Feed_free(feed);
	api_Channel_free(channel);
}

/* Futures of a value that is not Send, polled on this thread until each is ready. */
static void local_later(const api_Local *local) {
	api_status status = 0;
	const api_Local *froms[2] = {local, NULL};
	const char *labels[2] = {"local later", "local later fresh"};
	for (int i = 0; i < 2; i++) {
		api_local_or_new_later_future *later = api_local_or_new_later(froms[i], &status);
		api_Local *made = NULL;
		while (status == 0 && api_local_or_new_later_poll(later, (api_waker){0}, &made, &status) == API_POLL_PENDING) {
		}
		printf("%s %u\n", labels[i], made ? api_Local_bump(made, &status) : 0);
		api_Local_free(made);
		api_local_or_new_later_future_free(later);
	}
	if (status != 0) {
		tr_fail("a local's future failed");
	}
}

/* Bytes in hex, two digits each. */
static api_buf hex(api_buf bytes) {
	char text[256];
	size_t n = 0;
	for (size_t i = 0; i < bytes.len && n + 3 <= sizeof text; i++) {
		n += (size_t)snprintf(text + n, sizeof text - n, "%02x", bytes.ptr[i]);
	}
	return copied(text, n);
}

/* A path's bytes as `escape_ascii` prints them. */
static api_buf escaped(api_buf path) {
	tr_text t = {0};
	for (size_t i = 0; i < path.len; i++) {
		uint8_t b = path.ptr[i];
		switch (b) {
		case '\t':
			tr_str(&t, "\\t");
			break;
		case '\r':
			tr_str(&t, "\\r");
			break;
		case '\n':
			tr_str(&t, "\\n");
			break;
		case '\\':
		case '\'':
		case '"':
			tr_fmt(&t, "\\%c", b);
			break;
		default:
			if (b >= 0x20 && b < 0x7f) {
				tr_put(&t, (const char *)&b, 1);
			} else {
				tr_fmt(&t, "\\x%02x", b);
			}
		}
	}
	return tr_done(&t);
}

static api_buf path_hex(api_buf path) {
	return hex(path);
}

/* A duration as its seconds and nine digits of nanoseconds. */
static api_buf seconds(api_duration d) {
	char text[64];
	int n = snprintf(text, sizeof text, "%llu.%09u", (unsigned long long)d.secs, (unsigned)d.nanos);
	return copied(text, (size_t)n);
}

/* A duration in whole milliseconds. */
static uint64_t millis(api_duration d) {
	return d.secs * 1000 + d.nanos / 1000000;
}

/* A char's code point. */
static uint32_t code(uint32_t c) {
	return c;
}

/* The list reversed in place, a view of it: the script frees the list it passed. */
static api_vec_tls_Codec reversed(api_vec_tls_Codec list) {
	api_tls_Codec **items = (api_tls_Codec **)list.ptr;
	for (size_t i = 0; i < list.len / 2; i++) {
		api_tls_Codec *swap = items[i];
		items[i] = items[list.len - 1 - i];
		items[list.len - 1 - i] = swap;
	}
	return (api_vec_tls_Codec){.ptr = list.ptr, .len = list.len};
}

/* Each float's bits, a view of storage of its own. */
static api_vec_u64 f64_bits(api_vec_f64 floats) {
	static uint64_t bits[64];
	if (floats.len > sizeof bits / sizeof bits[0]) {
		tr_fail("f64_bits of too many floats");
	}
	for (size_t i = 0; i < floats.len; i++) {
		memcpy(&bits[i], &floats.ptr[i], sizeof bits[i]);
	}
	return (api_vec_u64){.ptr = bits, .len = floats.len};
}

static void put_hex(tr_text *t, const uint8_t *bytes, size_t len) {
	for (size_t i = 0; i < len; i++) {
		tr_fmt(t, "%02x", bytes[i]);
	}
}

/* A socket address as its IP's bytes in hex, its port, flow label, and scope, each
 * IPv6's alone, apart by colons. */
static api_buf socket_hex(api_socket_addr addr) {
	tr_text t = {0};
	if (!addr.v6) {
		put_hex(&t, addr.addr.v4.ip.octets, 4);
		tr_fmt(&t, ":%u", addr.addr.v4.port);
	} else {
		put_hex(&t, addr.addr.v6.ip.octets, 16);
		tr_fmt(&t, ":%u:%u:%u", addr.addr.v6.port, addr.addr.v6.flowinfo, addr.addr.v6.scope_id);
	}
	return tr_done(&t);
}

/* An address's bytes in hex, so the length shows its family. */
static api_buf ip_hex(api_ip_addr ip) {
	tr_text t = {0};
	if (ip.v6) {
		put_hex(&t, ip.ip.v6.octets, 16);
	} else {
		put_hex(&t, ip.ip.v4.octets, 4);
	}
	return tr_done(&t);
}

static unsigned long long bits32(float f) {
	uint32_t bits;
	memcpy(&bits, &f, sizeof bits);
	return bits;
}

static unsigned long long bits64(double f) {
	uint64_t bits;
	memcpy(&bits, &f, sizeof bits);
	return bits;
}

/* Print each of `numbers`' lists, its floats as their bits. */
#define PRINT_LIST(list, fmt, cast)                                           \
	do {                                                                  \
		printf("[");                                                  \
		for (size_t i_ = 0; i_ < (list).len; i_++) {                  \
			printf(i_ ? ", " fmt : fmt, cast((list).ptr[i_]));    \
		}                                                             \
		printf("]");                                                  \
	} while (0)

static void numbers(api_buf what_buf, api_Numbers n) {
	int w = (int)what_buf.len;
	const char *what = (const char *)what_buf.ptr;
	printf("%.*s u16 ", w, what);
	PRINT_LIST(n.u16s, "%u", (unsigned));
	printf("\n%.*s u32 ", w, what);
	PRINT_LIST(n.u32s, "%u", (unsigned));
	printf("\n%.*s u64 ", w, what);
	PRINT_LIST(n.u64s, "%llu", (unsigned long long));
	printf("\n%.*s i8 ", w, what);
	PRINT_LIST(n.i8s, "%d", (int));
	printf("\n%.*s i16 ", w, what);
	PRINT_LIST(n.i16s, "%d", (int));
	printf("\n%.*s i32 ", w, what);
	PRINT_LIST(n.i32s, "%d", (int));
	printf("\n%.*s i64 ", w, what);
	PRINT_LIST(n.i64s, "%lld", (long long));
	printf("\n%.*s f32 ", w, what);
	PRINT_LIST(n.f32s, "%llu", bits32);
	printf("\n%.*s f64 ", w, what);
	PRINT_LIST(n.f64s, "%llu", bits64);
	printf("\n");
	if (n.some.ptr) {
		printf("%.*s some Some(", w, what);
		PRINT_LIST(n.some, "%d", (int));
		printf(")\n");
	} else {
		printf("%.*s some None\n", w, what);
	}
}

/* A path no Rust string spells: bytes that are not UTF-8. */
static api_buf unspelled(void) {
	return api_stray();
}


/* What the transcript's output does not show: each check of the ABI's contract beyond it. */
static void checks(void) {
	/* A name takes a copy: the caller's buffer is its own again. */
	api_Name *name = api_Name_owned(str("a"));
	char text[] = "c";
	api_Name_rename(name, str(text), NULL);
	text[0] = 'x';
	api_buf renamed = api_Name_as_str(name, NULL);
	if (renamed.len != 1 || renamed.ptr[0] != 'c') {
		tr_fail("a rename kept the caller's buffer");
	}
	api_buf_free(renamed);
	api_Name_free(name);

	/* A path's bytes as the OS holds them, which need not be UTF-8. A second owner of the
	 * buffer goes to Rust, which takes it, so stray stays ours. */
	api_buf stray = api_stray();
	api_buf in_stray = api_file(api_buf_clone(stray), str("f"));
	if (in_stray.len != 3 || memcmp(in_stray.ptr, "\xff/f", 3) != 0) {
		tr_fail("a cloned buffer passed to Rust was not its own");
	}
	api_buf_free(stray);
	api_buf_free(in_stray);

	api_Key *key = api_Key_new(str("a"));
	if (api_Key_hash(key) != api_Key_hash_of(key)) {
		tr_fail("api_Key_hash differs from Rust's own hash");
	}
	api_Key_free(key);

	/* A returned `Vec<&Token>` holds clones, which its free drops. */
	api_Token *twice = api_Token_new(9);
	uint32_t before = api_dropped();
	api_vec_Token tokens = api_Token_twice(twice, NULL);
	if (tokens.len != 2 || api_Token_id_get(tokens.ptr[1], NULL) != 9) {
		tr_fail("api_Token_twice");
	}
	api_vec_Token_free(tokens);
	if (api_dropped() != before + 2) {
		tr_fail("a returned vec's free did not drop its clones");
	}
	api_Token_free(twice);

	/* A kept closure, called on another thread after the call that took it returned. */
	bool spawned_freed = false;
	api_Spawned *spawned = api_Spawned_new(spawned_task, &spawned_freed, spawned_free);
	uint32_t joined = api_Spawned_join(spawned, NULL);
	api_Spawned_free(spawned);
	if (joined != 7 || !spawned_freed) {
		tr_fail("a spawned closure");
	}

	/* Dropped before it finishes, a drive frees the closure it kept. */
	api_waker waker = {.wake = wake, .drop = drop, .data = NULL};
	api_Moment start = api_Moment_new(0, 1000).value;
	driver idle = {.start = start};
	api_drive_future *drive = api_drive(start, drive_idle, &idle, drive_free);
	api_buf stopped;
	if (api_drive_poll(drive, waker, &stopped) != API_POLL_PENDING || idle.freed) {
		tr_fail("an idle drive");
	}
	api_drive_future_free(drive);
	if (!idle.freed) {
		tr_fail("a dropped drive kept its closure");
	}

	if (!foreign_futures()) {
		tr_fail("foreign futures");
	}
	if (!forwarding()) {
		tr_fail("forwarding");
	}
	if (!started_futures()) {
		tr_fail("started futures");
	}
}


/* Each breaks a contract, which must abort with a message naming the function. */
/* A counter a closure was lent, which it fails to move out, then reads as it was. */
static bool move_lent_status(void *data, api_Counter *counter) {
	api_status *status = data;
	/* The refused call returns a zeroed label, which holds nothing to free. */
	api_Label refused = api_Counter_into_label(counter, (api_option_Point){0}, status);
	(void)refused;
	return *status == API_STATUS_LENT && api_Counter_get(counter, NULL) == 0;
}

/* Each check a call fails writes its status through a non-null one, and changes nothing. */
/* What a second thread does with the main thread's local and future: each use fails as
 * not Send, and each free queues the drop back to the main thread. */
typedef struct {
	api_Local *local;
	api_local_later_future *later;
	api_Local_poll_count_future *count;
	api_status bumped;
	api_status polled;
	api_status counted;
	api_poll poll;
} elsewhere;

static void *use_elsewhere(void *data) {
	elsewhere *e = data;
	api_Local_bump(e->local, &e->bumped);
	uint32_t out = 0;
	e->poll = api_local_later_poll(e->later, (api_waker){0}, &out, &e->polled);
	DISCARD(api_Local_poll_count_poll(e->count, (api_waker){0}, &out, &e->counted));
	api_Local_free(e->local);
	api_local_later_future_free(e->later);
	api_Local_poll_count_future_free(e->count);
	return NULL;
}

/* A local a second thread makes, which it keeps until told it was freed: then it exits,
 * which drops it there. */
typedef struct {
	api_Local *local;
	/* Whether it also makes a future borrowing the local, which then holds it last. */
	bool later_too;
	api_Local_later_future *later;
	atomic_bool made;
	atomic_bool freed;
} kept;

static void *keep_local(void *data) {
	kept *k = data;
	k->local = api_Local_new();
	if (k->later_too) {
		api_status status = 0;
		k->later = api_Local_later(k->local, &status);
	}
	atomic_store(&k->made, true);
	while (!atomic_load(&k->freed)) {
	}
	return NULL;
}

/* A local a second thread bumps, writing why it could not. */
typedef struct {
	api_Local *local;
	uint32_t count;
	api_status status;
} bumped;

static void *bump_local(void *data) {
	bumped *b = data;
	b->status = 0;
	b->count = api_Local_bump(b->local, &b->status);
	return NULL;
}

/* A future making a local, which a second thread polls until it finishes or fails. */
typedef struct {
	api_local_or_new_later_future *future;
	api_Local *out;
	api_poll poll;
	api_status status;
} polled_later;

static void *poll_later(void *data) {
	polled_later *p = data;
	p->status = 0;
	do {
		p->poll = api_local_or_new_later_poll(p->future, (api_waker){0}, &p->out, &p->status);
	} while (p->poll == API_POLL_PENDING && p->status == 0);
	return NULL;
}

/* Two locals on two homes, each visit calling into the other's. */
typedef struct {
	api_Local *a;
	api_Local *b;
	api_status status;
} homes;

static uint32_t bump_a(void *data) {
	homes *h = data;
	return api_Local_bump(h->a, &h->status);
}

static uint32_t visit_b(void *data) {
	homes *h = data;
	return api_Local_visit(h->b, bump_a, h, &h->status);
}

static void *make_local(void *data) {
	*(api_Local **)data = api_Local_new();
	return NULL;
}

/* A value that is not Send stays on the thread that made it. */
static int not_send(void) {
	int failed = 0;
	api_status status = 0;
	api_Local *local = api_Local_new();
	api_Local *shared = api_Local_share(local, &status);
	api_Local_bump(local, &status);
	if (api_Local_bump(shared, &status) != 2 || status != 0 || !api_Local_eq(local, shared, &status) ||
		api_Local_hash(local, &status) != api_Local_hash(shared, &status) || status != 0) {
		fprintf(stderr, "a shared local counted apart\n");
		failed = 1;
	}
	api_local_later_future *later = api_local_later(3);
	uint32_t out = 0;
	if (api_local_later_poll(later, (api_waker){0}, &out, &status) != API_POLL_PENDING || status != 0) {
		fprintf(stderr, "a local future finished at once\n");
		failed = 1;
	}
	uint32_t before = api_locals_dropped();
	elsewhere e = {.local = local, .later = later, .count = api_Local_poll_count(local, &status)};
	pthread_t thread;
	pthread_create(&thread, NULL, use_elsewhere, &e);
	pthread_join(thread, NULL);
	if (e.bumped != API_STATUS_NOT_SEND || e.polled != API_STATUS_NOT_SEND || e.counted != API_STATUS_NOT_SEND ||
		e.poll != API_POLL_PENDING ||
		strcmp(api_status_message(e.bumped), "a value that is not Send is used on another thread than the one that made it") != 0) {
		fprintf(stderr, "a local used elsewhere wrote %d and %d\n", e.bumped, e.polled);
		failed = 1;
	}
	/* Freed elsewhere, it drops here at this thread's next check. */
	if (api_locals_dropped() != before) {
		fprintf(stderr, "a local freed elsewhere dropped there\n");
		failed = 1;
	}
	if (api_Local_bump(shared, &status) != 3 || status != 0 || api_locals_dropped() != before + 1) {
		fprintf(stderr, "a local freed elsewhere never dropped on its thread\n");
		failed = 1;
	}
	api_Local_free(shared);

	/* Freed while its thread runs, it drops there as the thread exits. */
	kept k = {0};
	before = api_locals_dropped();
	pthread_create(&thread, NULL, keep_local, &k);
	while (!atomic_load(&k.made)) {
	}
	api_Local_free(k.local);
	atomic_store(&k.freed, true);
	pthread_join(thread, NULL);
	if (api_locals_dropped() != before + 1) {
		fprintf(stderr, "a local freed elsewhere never dropped as its thread exited\n");
		failed = 1;
	}

	/* A future freed elsewhere that borrows a local freed before it: dropped as the thread
	 * exits, the future lets go of the local, whose drop runs there too. */
	kept l = {.later_too = true};
	before = api_locals_dropped();
	pthread_create(&thread, NULL, keep_local, &l);
	while (!atomic_load(&l.made)) {
	}
	api_Local_free(l.local);
	api_Local_later_future_free(l.later);
	atomic_store(&l.freed, true);
	pthread_join(thread, NULL);
	if (api_locals_dropped() != before + 1) {
		fprintf(stderr, "a local a future freed elsewhere borrowed never dropped as its thread exited\n");
		failed = 1;
	}

	/* Made on a home, a thread Rust owns, it stays there, and each call runs there. */
	api_Local *homed = api_Local_new_homed();
	status = 0;
	if (api_Local_bump(homed, &status) != 1 || !api_Local_on_own_thread(homed, &status) || status != 0) {
		fprintf(stderr, "a homed local ran here, writing %d\n", status);
		failed = 1;
	}
	api_local_later_future *homed_later = NULL;
	api_poll poll = api_local_later_start_homed(4, &out, &homed_later);
	while (poll == API_POLL_PENDING) {
		poll = api_local_later_poll(homed_later, (api_waker){0}, &out, &status);
	}
	if (out != 8 || status != 0) {
		fprintf(stderr, "a homed future returned %u, writing %d\n", out, status);
		failed = 1;
	}
	if (homed_later != NULL) {
		api_local_later_future_free(homed_later);
	}
	/* A maker whose optional handle is absent runs on a home, as one taking none does, so
	 * any thread may use what it makes; with the handle present, it runs where that stays. */
	uint32_t before_made = api_locals_dropped();
	bumped b = {.local = api_local_or_new_homed(NULL, &status)};
	pthread_create(&thread, NULL, bump_local, &b);
	pthread_join(thread, NULL);
	if (status != 0 || b.status != 0 || b.count != 1) {
		fprintf(stderr, "a local made with no handle stayed here, writing %d and %d\n", status, b.status);
		failed = 1;
	}
	api_Local *mine = api_Local_new();
	bumped m = {.local = api_local_or_new_homed(mine, &status)};
	pthread_create(&thread, NULL, bump_local, &m);
	pthread_join(thread, NULL);
	if (status != 0 || m.status != API_STATUS_NOT_SEND || api_Local_bump(mine, &status) != 1 || status != 0) {
		fprintf(stderr, "a local made from one here left it, writing %d and %d\n", status, m.status);
		failed = 1;
	}
	api_Local_free(m.local);
	api_Local_free(mine);
	api_Local_free(b.local);
	while (api_locals_dropped() != before_made + 3) {
	}
	/* So does such a future, which a second thread then polls: on its home with the
	 * handle absent, and failing its check with it present, as it stays here. */
	before_made = api_locals_dropped();
	polled_later none = {0};
	if (api_local_or_new_later_start_homed(NULL, &none.out, &none.future, &status) != API_POLL_PENDING ||
		status != 0) {
		fprintf(stderr, "a future made with no handle finished at once, writing %d\n", status);
		failed = 1;
	}
	pthread_create(&thread, NULL, poll_later, &none);
	pthread_join(thread, NULL);
	if (none.poll != API_POLL_READY || none.status != 0 || api_Local_bump(none.out, &status) != 1 || status != 0) {
		fprintf(stderr, "a future made with no handle left its home, writing %d and %d\n", none.status, status);
		failed = 1;
	}
	api_Local *here = api_Local_new();
	polled_later some = {0};
	if (api_local_or_new_later_start_homed(here, &some.out, &some.future, &status) != API_POLL_PENDING ||
		status != 0) {
		fprintf(stderr, "a future made from a local here finished at once, writing %d\n", status);
		failed = 1;
	}
	pthread_create(&thread, NULL, poll_later, &some);
	pthread_join(thread, NULL);
	if (some.status != API_STATUS_NOT_SEND) {
		fprintf(stderr, "a future made from a local here was polled elsewhere, writing %d\n", some.status);
		failed = 1;
	}
	poll_later(&some);
	if (some.poll != API_POLL_READY || some.status != 0 || api_Local_bump(some.out, &status) != 1 ||
		api_Local_bump(here, &status) != 2 || status != 0) {
		fprintf(stderr, "a future made from a local here did not share it, writing %d and %d\n", some.status, status);
		failed = 1;
	}
	api_local_or_new_later_future_free(none.future);
	api_local_or_new_later_future_free(some.future);
	api_Local_free(none.out);
	api_Local_free(some.out);
	api_Local_free(here);
	while (api_locals_dropped() != before_made + 3) {
	}
	/* A call on one home's value whose callback calls on another's, which calls back on
	 * the first's: the first home runs that while it waits on the second. */
	homes h = {.a = api_Local_new_homed(), .b = api_Local_new_homed()};
	if (api_Local_visit(h.a, visit_b, &h, &status) != 1 || status != 0 || h.status != 0) {
		fprintf(stderr, "two homes calling each other wrote %d and %d\n", status, h.status);
		failed = 1;
	}
	before_made = api_locals_dropped();
	api_Local_free(h.a);
	api_Local_free(h.b);
	while (api_locals_dropped() != before_made + 2) {
	}
	/* A home whose value was closed, then freed elsewhere, goes idle, so a later value made
	 * from none lives there: one of the next few, as the home sees it in its own time. */
	api_Local *closed = api_Local_new_homed();
	uint64_t closed_on = api_Local_thread_number(closed, &status);
	api_Local_drop(closed, &status);
	api_Local_free(closed);
	api_Local *made[100];
	size_t count = 0;
	bool reused = false;
	while (count < 100 && !reused) {
		made[count] = api_Local_new_homed();
		reused = api_Local_thread_number(made[count], &status) == closed_on;
		count++;
		struct timespec start, now;
		timespec_get(&start, TIME_UTC);
		do {
			timespec_get(&now, TIME_UTC);
		} while (!reused && (now.tv_sec - start.tv_sec) * 1000000000L + (now.tv_nsec - start.tv_nsec) < 20000000L);
	}
	if (!reused || status != 0) {
		fprintf(stderr, "a home whose value was closed never went idle, writing %d\n", status);
		failed = 1;
	}
	before_made = api_locals_dropped();
	for (size_t i = 0; i < count; i++) {
		api_Local_free(made[i]);
	}
	while (api_locals_dropped() != before_made + count) {
	}
	/* A poll through a waiter that is not Send keeps it, so its future is not Send either. */
	api_LocalWaiter_poll_ready_future *waited = NULL;
	uint32_t ready = 0;
	if (api_LocalWaiter_poll_ready_start(2, &ready, &waited) != API_POLL_READY || ready != 2) {
		fprintf(stderr, "a waiter's own poll returned %u\n", ready);
		failed = 1;
	}
	api_poll_tripled_future *tripled = NULL;
	if (api_poll_tripled_start(3, &ready, &tripled) != API_POLL_READY || ready != 9) {
		fprintf(stderr, "a poll through a waiter returned %u\n", ready);
		failed = 1;
	}
	/* Freed here, it drops on its home, which waits for work. */
	uint32_t before_homed = api_locals_dropped();
	api_Local_free(homed);
	while (api_locals_dropped() != before_homed + 1) {
	}
	return failed;
}

/* A local freed once its thread exited leaks, saying so, since nothing may drop it. */
static int orphan(void) {
	uint32_t before = api_locals_dropped();
	api_Local *local = NULL;
	pthread_t thread;
	pthread_create(&thread, NULL, make_local, &local);
	pthread_join(thread, NULL);
	api_Local_free(local);
	return api_locals_dropped() != before;
}

static int statuses(void) {
	int failed = 0;
	api_status status = 0;
	api_Channel *channel = api_Channel_new();
	api_Channel_send(channel, str("x"), NULL);
	api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
	api_Channel_clear(channel, &status);
	if (status != API_STATUS_BORROWED || strcmp(api_status_message(status), "a handle is already borrowed") != 0) {
		fprintf(stderr, "a clear during a recv wrote %d\n", status);
		failed = 1;
	}
	/* The recv still has what was sent, and once it ends the clear passes. */
	api_waker waker = {0};
	api_buf payload;
	if (api_Channel_recv_poll(recv, waker, &payload) != API_POLL_READY || payload.len != 1) {
		fprintf(stderr, "a failed clear changed the channel\n");
		failed = 1;
	}
	api_buf_free(payload);
	api_Channel_recv_future_free(recv);
	status = 0;
	api_Channel_clear(channel, &status);
	if (status != 0) {
		fprintf(stderr, "a clear after the recv wrote %d\n", status);
		failed = 1;
	}
	api_Channel_free(channel);

	/* A drop under a running call is refused; once it ends the drop takes the value, which
	 * a later call then finds moved, and free frees only the handle. */
	channel = api_Channel_new();
	api_Channel_send(channel, str("y"), NULL);
	recv = api_Channel_recv(channel, NULL);
	status = 0;
	api_Channel_drop(channel, &status);
	if (status != API_STATUS_BORROWED) {
		fprintf(stderr, "a drop during a recv wrote %d\n", status);
		failed = 1;
	}
	if (api_Channel_recv_poll(recv, waker, &payload) != API_POLL_READY || payload.len != 1) {
		fprintf(stderr, "a refused drop changed the channel\n");
		failed = 1;
	}
	api_buf_free(payload);
	api_Channel_recv_future_free(recv);
	status = 0;
	api_Channel_drop(channel, &status);
	api_Channel_drop(channel, &status);
	if (status != 0) {
		fprintf(stderr, "a drop after the recv wrote %d\n", status);
		failed = 1;
	}
	api_Channel_clear(channel, &status);
	if (status != API_STATUS_MOVED) {
		fprintf(stderr, "a clear after a drop wrote %d\n", status);
		failed = 1;
	}
	api_Channel_free(channel);

	/* A draft holds its ledger's exclusive borrow until it drops: any other use of the
	 * ledger fails meanwhile, as Rust's borrow checker refuses it, and a commit, a close,
	 * or a free of the draft gives it back. */
	{
		api_Ledger *ledger = api_Ledger_new();
		api_Draft *draft;
		api_Error err;
		DISCARD(api_Ledger_edit(ledger, &draft, &err, NULL));
		status = 0;
		api_Ledger_published(ledger, &status);
		if (status != API_STATUS_EXCLUSIVE) {
			fprintf(stderr, "a read of a lent ledger wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		api_Draft *second;
		if (api_Ledger_edit(ledger, &second, &err, &status) || status != API_STATUS_BORROWED) {
			fprintf(stderr, "a second draft of a lent ledger wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		api_Ledger_drop(ledger, &status);
		if (status != API_STATUS_BORROWED) {
			fprintf(stderr, "a drop of a lent ledger wrote %d\n", status);
			failed = 1;
		}
		/* A draft that must outlive the program, which a lent one never does. */
		status = 0;
		if (api_lasting(draft, &status) || status != API_STATUS_LENT) {
			fprintf(stderr, "a lent draft passed as lasting wrote %d\n", status);
			failed = 1;
		}
		api_Draft_write(draft, str("x"), NULL);
		status = 0;
		if (api_Draft_commit(draft, &status) != 1 || status != 0) {
			fprintf(stderr, "a commit wrote %d\n", status);
			failed = 1;
		}
		/* The commit moved the draft out and gave the borrow back. */
		status = 0;
		api_Draft_lines(draft, &status);
		if (status != API_STATUS_MOVED) {
			fprintf(stderr, "a committed draft wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		if (api_Ledger_published(ledger, &status) != 1 || status != 0) {
			fprintf(stderr, "a ledger after a commit wrote %d\n", status);
			failed = 1;
		}
		api_Draft_free(draft);
		/* A close gives it back too, publishing what the draft marked. */
		DISCARD(api_Ledger_edit(ledger, &draft, &err, NULL));
		api_Draft_write(draft, str("y"), NULL);
		status = 0;
		api_Draft_drop(draft, &status);
		if (status != 0 || api_Ledger_published(ledger, &status) != 2 || status != 0) {
			fprintf(stderr, "a ledger after a close wrote %d\n", status);
			failed = 1;
		}
		api_Draft_free(draft);
		/* The ledger freed while lent drops once the draft does, which still works. */
		DISCARD(api_Ledger_edit(ledger, &draft, &err, NULL));
		api_Ledger_free(ledger);
		api_Draft_write(draft, str("z"), NULL);
		status = 0;
		if (api_Draft_commit(draft, &status) != 3 || status != 0) {
			fprintf(stderr, "a draft of a freed ledger wrote %d\n", status);
			failed = 1;
		}
		api_Draft_free(draft);
		/* Clones of a receipt share its borrow, which the last one to drop gives back. */
		ledger = api_Ledger_new();
		api_Receipt *receipt = api_Ledger_receipt(ledger, NULL);
		api_Receipt *copy = api_Receipt_clone(receipt, NULL);
		api_Receipt_free(receipt);
		status = 0;
		api_Ledger_seal(ledger, &status);
		if (status != API_STATUS_BORROWED || api_Receipt_count(copy, NULL) != 0) {
			fprintf(stderr, "a ledger a cloned receipt borrows wrote %d\n", status);
			failed = 1;
		}
		api_Receipt_free(copy);
		status = 0;
		api_Ledger_seal(ledger, &status);
		if (status != 0) {
			fprintf(stderr, "a ledger after its receipts wrote %d\n", status);
			failed = 1;
		}
		/* A receipt whose bound lets it live as long as the program, and a field of a
		 * slip's clone, which would carry the ledger's borrow out, each fail as lent. */
		receipt = api_Ledger_receipt(ledger, NULL);
		status = 0;
		api_stored(receipt, &status);
		if (status != API_STATUS_LENT || api_Receipt_count(receipt, NULL) != 0) {
			fprintf(stderr, "a lent receipt stored wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		api_erased(receipt, &status);
		if (status != API_STATUS_LENT) {
			fprintf(stderr, "a lent receipt erased wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		api_summed_up_Receipt(receipt, &status);
		if (status != API_STATUS_LENT) {
			fprintf(stderr, "a lent receipt summed up wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		if (api_tallied(receipt, &status) != 0 || status != 0) {
			fprintf(stderr, "a lent receipt tallied wrote %d\n", status);
			failed = 1;
		}
		status = 0;
		api_Ledger_seal(ledger, &status);
		if (status != 0) {
			fprintf(stderr, "a ledger after its receipt moved into a call wrote %d\n", status);
			failed = 1;
		}
		api_Receipt_free(receipt);
		/* A lend bounded on its own borrow by a trait every lifetime meets borrows the
		 * ledger as any lend does. */
		receipt = api_Ledger_noted(ledger, NULL);
		status = 0;
		api_Ledger_seal(ledger, &status);
		if (status != API_STATUS_BORROWED || api_Receipt_count(receipt, NULL) != 0) {
			fprintf(stderr, "a ledger a noted receipt borrows wrote %d\n", status);
			failed = 1;
		}
		api_Receipt_free(receipt);
		api_Slip *slip = api_Ledger_slip(ledger, NULL);
		api_Slip *twin = api_Slip_clone(slip, NULL);
		api_Slip_free(slip);
		status = 0;
		DISCARD(api_Slip_receipt_get(twin, &status));
		if (status != API_STATUS_LENT || api_Slip_count(twin, NULL) != 0) {
			fprintf(stderr, "a lent slip's receipt wrote %d\n", status);
			failed = 1;
		}
		/* A field that borrows only what outlives the ledger reads as ever. */
		status = 0;
		api_Receipt *global = api_Slip_global_get(twin, &status);
		if (status != 0 || api_Receipt_count(global, NULL) != 7) {
			fprintf(stderr, "a lent slip's global receipt wrote %d\n", status);
			failed = 1;
		}
		api_Receipt_free(global);
		api_Slip_free(twin);
		status = 0;
		api_Ledger_seal(ledger, &status);
		if (status != 0) {
			fprintf(stderr, "a ledger after its slips wrote %d\n", status);
			failed = 1;
		}
		api_Ledger_free(ledger);
		/* Only `Pad<u8>`'s own `describe` hides its sheet's. */
		api_Pad_u8 *small = api_Pad_u8_small();
		api_Pad_u16 *wide = api_Pad_u16_wide();
		api_buf padded = api_Pad_u8_describe(small);
		api_buf sheeted = api_Pad_u16_describe(wide);
		if (padded.len != 5 || memcmp(padded.ptr, "pad 1", 5) != 0 || sheeted.len != 6 ||
		    memcmp(sheeted.ptr, "sheet ", 6) != 0 || api_Pad_u16_tag(wide) != 2) {
			fprintf(stderr, "pads describe as %.*s and %.*s\n", (int)padded.len, (const char *)padded.ptr,
			        (int)sheeted.len, (const char *)sheeted.ptr);
			failed = 1;
		}
		api_buf_free(padded);
		api_buf_free(sheeted);
		/* `Pad<u16>`'s own `title`, whose path the config shares with its sheet's field. */
		if (api_Pad_u16_title(wide) != 2) {
			fprintf(stderr, "a pad's title is %u\n", api_Pad_u16_title(wide));
			failed = 1;
		}
		/* A sheet behind a layer that does not export, whose `lines` it reaches. */
		api_Layered *layered = api_Layered_new();
		if (api_Layered_lines(layered) != 0) {
			fprintf(stderr, "a layered sheet has lines\n");
			failed = 1;
		}
		api_Layered_free(layered);
		/* A record's fields, reached through `Deref` and written through `DerefMut`. */
		api_Marker *marker = api_Marker_new(str("m"), 3, 5);
		api_Marker_y_set(marker, 7, NULL);
		if (api_Marker_x_get(marker, NULL) != 3 || api_Marker_y_get(marker, NULL) != 7) {
			fprintf(stderr, "a marker's point is %d, %d\n", api_Marker_x_get(marker, NULL), api_Marker_y_get(marker, NULL));
			failed = 1;
		}
		api_Marker_free(marker);
		api_Pad_u8_free(small);
		api_Pad_u16_free(wide);
	}

	status = 0;
	api_Meters *meters = api_Meters_new();
	api_Meter *first = api_Meters_first(meters, NULL);
	api_Meters_clear(meters, NULL);
	if (api_Meter_count(first, &status) != 0 || status != API_STATUS_OUTLIVED) {
		fprintf(stderr, "a meter that outlived its meters wrote %d\n", status);
		failed = 1;
	}
	api_Meter_free(first);
	api_Meters_free(meters);

	/* A counter moved in beside a dropped mark, alone or held in a record or an enum, is
	 * refused before the move, so the counter is still there; once every mark is live,
	 * the call moves it. */
	api_Counter *admitted = api_Counter_new(str("a"), 5);
	api_Mark *mark = api_Mark_of(3);
	api_Mark *dropped = api_Mark_of(4);
	api_Mark_drop(dropped, NULL);
	api_Choice picked = {.tag = API_CHOICE_PICKED, .picked = {._0 = mark}};
	api_Choice empty = {.tag = API_CHOICE_EMPTY};
	api_tuple_Counter_Mark_Seat_Choice entries[] = {
		{admitted, dropped, {mark, 1}, empty},
		{admitted, mark, {dropped, 1}, empty},
		{admitted, mark, {mark, 1}, {.tag = API_CHOICE_PICKED, .picked = {._0 = dropped}}},
	};
	for (size_t i = 0; i < sizeof(entries) / sizeof(entries[0]); i++) {
		status = 0;
		api_admit(entries[i], &status);
		uint64_t still = api_Counter_get(admitted, NULL);
		if (status != API_STATUS_MOVED || still != 5) {
			fprintf(stderr, "admitting beside a dropped mark %zu wrote %d, leaving %llu\n", i, status, (unsigned long long)still);
			failed = 1;
		}
	}
	status = 0;
	uint64_t sum = api_admit((api_tuple_Counter_Mark_Seat_Choice){admitted, mark, {mark, 1}, picked}, &status);
	if (status != 0 || sum != 14) {
		fprintf(stderr, "admitting wrote %d, summing %llu\n", status, (unsigned long long)sum);
		failed = 1;
	}
	api_Counter_get(admitted, &status);
	if (status != API_STATUS_MOVED) {
		fprintf(stderr, "a counter admitted wrote %d\n", status);
		failed = 1;
	}
	/* A record's method lifts the handle its receiver holds, failing as a call does. */
	status = 0;
	api_Seat seat = {dropped, 2};
	api_Seat_count(&seat, &status);
	if (status != API_STATUS_MOVED) {
		fprintf(stderr, "a seat's count with a dropped mark wrote %d\n", status);
		failed = 1;
	}
	/* A value's `&mut self` method refused over a dropped mark leaves the value as it was. */
	status = 0;
	api_Score refused = api_Score_of(2);
	api_Score_note(&refused, dropped, &status);
	if (status != API_STATUS_MOVED || api_Score_count(&refused) != 2) {
		fprintf(stderr, "a score noting a dropped mark wrote %d, leaving %llu\n", status, (unsigned long long)api_Score_count(&refused));
		failed = 1;
	}
	status = 0;
	if (api_choose(empty, &status) != 0 || api_choose(picked, &status) != 3 || status != 0) {
		fprintf(stderr, "choosing wrote %d\n", status);
		failed = 1;
	}
	api_Counter_free(admitted);
	api_Mark_free(mark);
	api_Mark_free(dropped);

	/* A refused call frees each buffer it was given: beside the failing handle, in a record
	 * lent, and beside a move, which it leaves undone. Each case gives three. */
	api_Mark *live = api_Mark_of(2);
	api_Mark *gone = api_Mark_of(2);
	api_Mark_drop(gone, NULL);
	api_Counter *weight = api_Counter_new(str("w"), 1);
	api_Counter *dropped_weight = api_Counter_new(str("d"), 1);
	api_Counter_drop(dropped_weight, NULL);
	struct {
		api_Mark *marked, *lent;
		api_Counter *moved;
	} hefts[] = {{gone, live, weight}, {live, gone, weight}, {live, live, dropped_weight}};
	for (size_t i = 0; i < sizeof(hefts) / sizeof(hefts[0]); i++) {
		uint32_t before = api_dropped();
		status = 0;
		api_heft((api_tuple_Mark_bytes){hefts[i].marked, api_token_bytes(1, 2)},
			(api_Parcel){.mark = hefts[i].lent, .payload = api_token_bytes(2, 3)},
			(api_tuple_Counter_bytes){hefts[i].moved, api_token_bytes(3, 4)}, &status);
		if (status != API_STATUS_MOVED || api_dropped() != before + 3 || api_Counter_get(weight, NULL) != 1) {
			fprintf(stderr, "a refused heft %zu wrote %d, freeing %u buffers\n", i, status, api_dropped() - before);
			failed = 1;
		}
	}
	uint32_t before_heft = api_dropped();
	status = 0;
	uint64_t heft = api_heft((api_tuple_Mark_bytes){live, api_token_bytes(1, 2)},
		(api_Parcel){.mark = live, .payload = api_token_bytes(2, 3)},
		(api_tuple_Counter_bytes){weight, api_token_bytes(3, 4)}, &status);
	if (status != 0 || heft != 14 || api_dropped() != before_heft + 3) {
		fprintf(stderr, "a heft wrote %d, weighing %llu\n", status, (unsigned long long)heft);
		failed = 1;
	}
	api_Counter_free(dropped_weight);
	api_Counter_free(weight);
	api_Mark_free(gone);
	api_Mark_free(live);

	status = 0;
	api_Counter *kept = api_Counter_new(str("c"), 0);
	api_Registry *registry = api_Registry_new(kept, NULL);
	api_Counter_free(kept);
	if (!api_Registry_edit(registry, move_lent_status, &status, NULL)) {
		fprintf(stderr, "a lent counter moved out wrote %d\n", status);
		failed = 1;
	}
	api_Registry_free(registry);
	return failed;
}

static int broken(const char *which) {
#ifdef __linux__
	/* Non-dumpable, the expected abort writes no core and never starts the host's crash
	 * handler. */
	if (prctl(PR_SET_DUMPABLE, 0, 0, 0, 0) != 0) {
		perror("prctl");
		return 1;
	}
#endif
	if (strcmp(which, "statuses") == 0) {
		return statuses();
	}
	if (strcmp(which, "threads") == 0) {
		return not_send();
	}
	if (strcmp(which, "orphan") == 0) {
		return orphan();
	}
	if (strcmp(which, "moved") == 0) {
		api_Counter *c = api_Counter_new(str("c"), 0);
		api_Label label = api_Counter_into_label(c, (api_option_Point){0}, NULL);
		api_Label_free(&label);
		api_Counter_get(c, NULL);
	} else if (strcmp(which, "borrowed") == 0) {
		api_Channel *channel = api_Channel_new();
		api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
		(void)recv;
		api_Channel_clear(channel, NULL);
	} else if (strcmp(which, "utf8") == 0) {
		api_buf bad = {.ptr = (const uint8_t *)"\xff", .len = 1};
		api_buf_free(api_greet(bad, API_SHAPE_CIRCLE));
	} else if (strcmp(which, "finished") == 0) {
		api_Channel *channel = api_Channel_new();
		api_Channel_send(channel, str("x"), NULL);
		api_Channel_recv_future *recv = api_Channel_recv(channel, NULL);
		api_waker waker = {0};
		api_buf payload;
		DISCARD(api_Channel_recv_poll(recv, waker, &payload));
		api_buf_free(payload);
		DISCARD(api_Channel_recv_poll(recv, waker, &payload));
	} else if (strcmp(which, "enum") == 0) {
		api_buf_free(api_greet(str("x"), (api_Shape)2));
	} else if (strcmp(which, "lent") == 0 || strcmp(which, "lent moved") == 0) {
		api_Registry *registry = api_Registry_new(api_Counter_new(str("c"), 0), NULL);
		api_Registry_edit(registry, strcmp(which, "lent") == 0 ? free_lent : move_lent, NULL, NULL);
	} else if (strcmp(which, "outlived") == 0) {
		api_Meters *meters = api_Meters_new();
		api_Meter *first = api_Meters_first(meters, NULL);
		api_Meters_clear(meters, NULL);
		api_Meter_count(first, NULL);
	} else if (strcmp(which, "lent shared") == 0) {
		api_Meters *meters = api_Meters_new();
		api_Meter_reset(api_Meters_first(meters, NULL), NULL);
	} else if (strcmp(which, "null") == 0) {
		api_Registry *registry = api_Registry_new(api_Counter_new(str("c"), 0), NULL);
		api_Registry_edit(registry, NULL, NULL, NULL);
	} else if (strcmp(which, "returned") == 0) {
		api_buf_free(api_made(1, made_view, NULL));
	} else if (strcmp(which, "reader") == 0) {
		api_buf bytes;
		uint64_t at;
		api_buf err;
		DISCARD(api_tail((api_foreign_reader){0}, 1, &bytes, &at, &err));
	} else if (strcmp(which, "bound") == 0) {
		api_count((api_bounds_u64){.start = {.kind = (api_bound)3}});
	} else if (strcmp(which, "future") == 0) {
		/* A future with no poll. */
		DISCARD(api_after((api_foreign_future){0}, 1));
	} else if (strcmp(which, "started") == 0) {
		/* A future with no poll, through a start, which the abort names. */
		uint32_t out;
		api_after_future *after;
		DISCARD(api_after_start((api_foreign_future){0}, 1, &out, &after));
	} else if (strcmp(which, "start") == 0) {
		/* A start with nowhere to put the handle of a future that pends. */
		uint32_t out;
		DISCARD(api_yielded_start(1, &out, NULL));
	} else if (strcmp(which, "array") == 0) {
		api_vec_u16 ports = api_ports((api_vec_socket_addr){0});
		ports.len = 1;
		api_vec_u16_free(ports);
	} else if (strcmp(which, "target") == 0) {
		/* The header serves wasm32 too, whose items the host's library lacks. */
		api_wasm_only();
	} else if (strcmp(which, "variant") == 0) {
		api_pointer_bits((api_Platform){.tag = API_PLATFORM_WASM, .wasm = {.bits = 32}});
	} else if (strcmp(which, "plain variant") == 0) {
		api_is_family(API_FAMILY_WASM);
	} else if (strcmp(which, "zero") == 0) {
		api_next_hop((api_option_non_zero_u64){.some = true, .value = 0});
	}
	return 0;
}

int main(int argc, char **argv) {
	if (api_contract() != API_CONTRACT) {
		fprintf(stderr, "api.h and the library are from different builds\n");
		return 1;
	}
	if (argc > 1) {
		return broken(argv[1]);
	}
	script();
	checks();
	return 0;
}
